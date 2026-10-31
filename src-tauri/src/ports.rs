//! Core I/O ports. Collectors interact with the outside world only through these ports.
//! Tests inject recorded real output from fixtures/ and a fake clock.

use std::io;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Filesystem capacity in bytes, as reported by statvfs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Space {
    pub total: u64,
    /// Free for any process, including blocks reserved for root.
    pub free: u64,
    /// Free for the user: the available value reported by df.
    pub available: u64,
}

/// Filesystem relative to a root, using absolute paths such as /proc/stat.
pub trait Files: Send {
    fn read(&self, path: &str) -> io::Result<String>;
    /// Directory entry names.
    fn list(&self, dir: &str) -> io::Result<Vec<String>>;
    fn space(&self, mount_point: &str) -> io::Result<Space>;
}

/// Real filesystem under dir (/ in production).
pub struct RootFilesystem {
    dir: PathBuf,
}

impl RootFilesystem {
    pub fn system() -> Self {
        RootFilesystem::at("/")
    }

    pub fn at(dir: impl Into<PathBuf>) -> Self {
        RootFilesystem { dir: dir.into() }
    }

    fn path(&self, path: &str) -> PathBuf {
        self.dir.join(path.trim_start_matches('/'))
    }
}

impl Files for RootFilesystem {
    fn read(&self, path: &str) -> io::Result<String> {
        std::fs::read_to_string(self.path(path))
    }

    fn list(&self, dir: &str) -> io::Result<Vec<String>> {
        std::fs::read_dir(self.path(dir))?
            .map(|e| Ok(e?.file_name().to_string_lossy().into_owned()))
            .collect()
    }

    fn space(&self, mount_point: &str) -> io::Result<Space> {
        let s = rustix::fs::statvfs(self.path(mount_point))?;
        Ok(Space {
            total: s.f_blocks * s.f_frsize,
            free: s.f_bfree * s.f_frsize,
            available: s.f_bavail * s.f_frsize,
        })
    }
}

/// Current time in epoch milliseconds.
pub trait Clock: Send {
    fn now(&self) -> u64;
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_millis() as u64)
    }
}

#[cfg(test)]
pub mod fakes {
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Arc, Mutex};

    use std::path::Path;

    use super::*;

    pub fn fixtures() -> &'static Path {
        Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures"))
    }

    /// Fake root: a fixture copied into a temporary directory that tests may modify,
    /// with statvfs values supplied from a table.
    #[derive(Clone)]
    pub struct FakeRoot {
        pub dir: Arc<tempfile::TempDir>,
        spaces: Arc<Mutex<HashMap<String, Space>>>,
    }

    impl FakeRoot {
        /// Copy fixtures/<scenario> into a new root.
        pub fn from(scenario: &str) -> Self {
            let dir = tempfile::tempdir().unwrap();
            copy(&fixtures().join(scenario), dir.path());
            FakeRoot {
                dir: Arc::new(dir),
                spaces: Default::default(),
            }
        }

        pub fn write(&self, path: &str, content: &str) {
            let destination = self.dir.path().join(path.trim_start_matches('/'));
            std::fs::create_dir_all(destination.parent().unwrap()).unwrap();
            std::fs::write(destination, content).unwrap();
        }

        pub fn remove(&self, path: &str) {
            let _ = std::fs::remove_dir_all(self.dir.path().join(path.trim_start_matches('/')));
        }

        pub fn set_space(&self, mount: &str, space: Space) {
            self.spaces.lock().unwrap().insert(mount.into(), space);
        }
    }

    impl Files for FakeRoot {
        fn read(&self, path: &str) -> io::Result<String> {
            RootFilesystem::at(self.dir.path()).read(path)
        }

        fn list(&self, dir: &str) -> io::Result<Vec<String>> {
            RootFilesystem::at(self.dir.path()).list(dir)
        }

        fn space(&self, mount: &str) -> io::Result<Space> {
            self.spaces
                .lock()
                .unwrap()
                .get(mount)
                .copied()
                .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))
        }
    }

    fn copy(source: &Path, destination: &Path) {
        std::fs::create_dir_all(destination).unwrap();
        for entry in std::fs::read_dir(source).unwrap() {
            let entry = entry.unwrap();
            let target = destination.join(entry.file_name());
            if entry.file_type().unwrap().is_dir() {
                copy(&entry.path(), &target);
            } else {
                std::fs::copy(entry.path(), target).unwrap();
            }
        }
    }

    #[derive(Clone, Default)]
    pub struct FakeClock(Arc<AtomicU64>);

    impl FakeClock {
        pub fn at(ms: u64) -> Self {
            FakeClock(Arc::new(AtomicU64::new(ms)))
        }

        pub fn advance(&self, ms: u64) {
            self.0.fetch_add(ms, Ordering::SeqCst);
        }
    }

    impl Clock for FakeClock {
        fn now(&self) -> u64 {
            self.0.load(Ordering::SeqCst)
        }
    }
}

/// Closed event set: this port accepts neither sensitive content nor free text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Notification {
    NetworkAtRisk,
}
pub trait Notifier: Send {
    fn notify(&self, event: Notification);
}
pub struct DesktopNotifier;
impl Notifier for DesktopNotifier {
    fn notify(&self, _: Notification) {
        let Ok(bus) = zbus::blocking::Connection::session() else {
            return;
        };
        let Ok(proxy) = zbus::blocking::Proxy::new(
            &bus,
            "org.freedesktop.Notifications",
            "/org/freedesktop/Notifications",
            "org.freedesktop.Notifications",
        ) else {
            return;
        };
        let _: zbus::Result<u32> = proxy.call(
            "Notify",
            &(
                "kvojps-monitor",
                0u32,
                "",
                "Segurança da rede: Risco",
                "A conexão apresenta sinais de risco. Consulte o Módulo Rede.",
                Vec::<String>::new(),
                std::collections::HashMap::<String, zbus::zvariant::Value>::new(),
                -1i32,
            ),
        );
    }
}

/// Short-lived processes. Output and errors are never logged.
pub trait Processes: Send {
    fn execute(&self, program: &str, args: &[&str]) -> Result<String, ()>;
}
pub struct SystemProcesses;
impl Processes for SystemProcesses {
    fn execute(&self, program: &str, args: &[&str]) -> Result<String, ()> {
        let output = std::process::Command::new(program)
            .args(args)
            .output()
            .map_err(|_| ())?;
        if !output.status.success() {
            return Err(());
        }
        String::from_utf8(output.stdout).map_err(|_| ())
    }
}
/// GitHub GraphQL endpoint only; borrow credentials for one refresh.
pub trait Http: Send {
    fn graphql(&self, credential: &str, body: &serde_json::Value) -> Result<serde_json::Value, ()>;
}
pub struct GithubHttp;
impl Http for GithubHttp {
    fn graphql(&self, credential: &str, body: &serde_json::Value) -> Result<serde_json::Value, ()> {
        reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|_| ())?
            .post("https://api.github.com/graphql")
            .header("User-Agent", "kvojps-monitor")
            .bearer_auth(credential)
            .json(body)
            .send()
            .map_err(|_| ())?
            .error_for_status()
            .map_err(|_| ())?
            .json()
            .map_err(|_| ())
    }
}
