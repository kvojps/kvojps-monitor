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
    /// Query a local SQLite database without creating or modifying it.
    fn read_sqlite(&self, _path: &str, _query: &str) -> io::Result<Vec<serde_json::Value>> {
        Err(io::ErrorKind::Unsupported.into())
    }
    fn read_link(&self, _path: &str) -> io::Result<PathBuf> {
        Err(io::ErrorKind::Unsupported.into())
    }
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

    fn read_sqlite(&self, path: &str, query: &str) -> io::Result<Vec<serde_json::Value>> {
        use rusqlite::{types::ValueRef, Connection, OpenFlags};
        let read = || -> rusqlite::Result<Vec<serde_json::Value>> {
            let connection = Connection::open_with_flags(
                self.path(path),
                OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
            )?;
            let mut statement = connection.prepare(query)?;
            let names: Vec<String> = statement
                .column_names()
                .iter()
                .map(|name| (*name).to_owned())
                .collect();
            let rows = statement
                .query_map([], |row| {
                    let mut object = serde_json::Map::new();
                    for (index, name) in names.iter().enumerate() {
                        let value = match row.get_ref(index)? {
                            ValueRef::Null => serde_json::Value::Null,
                            ValueRef::Integer(value) => value.into(),
                            ValueRef::Text(value) => {
                                String::from_utf8_lossy(value).into_owned().into()
                            }
                            _ => return Err(rusqlite::Error::InvalidQuery),
                        };
                        object.insert(name.clone(), value);
                    }
                    Ok(serde_json::Value::Object(object))
                })?
                .collect();
            rows
        };
        read().map_err(|_| io::Error::other("SQLite read failed"))
    }

    fn read_link(&self, path: &str) -> io::Result<PathBuf> {
        std::fs::read_link(self.path(path))
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

        fn read_sqlite(&self, path: &str, query: &str) -> io::Result<Vec<serde_json::Value>> {
            RootFilesystem::at(self.dir.path()).read_sqlite(path, query)
        }

        fn read_link(&self, path: &str) -> io::Result<PathBuf> {
            RootFilesystem::at(self.dir.path()).read_link(path)
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
/// Transport failures contain metadata only, never response bodies or credentials.
#[derive(Debug, Clone, Copy, Default)]
pub struct HttpError {
    pub status: Option<u16>,
    pub retry_after_ms: u64,
    pub retry_after_at: Option<u64>,
}
impl HttpError {
    /// Decode Retry-After without reading a clock. The collector compares absolute
    /// HTTP dates with its injected clock after the request finishes.
    pub fn response(status: u16, retry_after: Option<&str>) -> Self {
        let mut error = Self {
            status: Some(status),
            ..Self::default()
        };
        if let Some(header) = retry_after {
            if let Ok(seconds) = header.trim().parse::<u64>() {
                error.retry_after_ms = seconds.saturating_mul(1000);
            } else {
                error.retry_after_at = chrono::DateTime::parse_from_rfc2822(header)
                    .ok()
                    .map(|date| date.timestamp_millis().max(0) as u64);
            }
        }
        error
    }
}
impl From<()> for HttpError {
    fn from(_: ()) -> Self {
        Self::default()
    }
}
/// GitHub GraphQL endpoint only; borrow credentials for one refresh.
#[derive(serde::Deserialize)]
pub struct GithubValidationResponse {
    pub scopes: Option<String>,
    pub body: serde_json::Value,
}
pub trait Http: Send {
    fn github_validation(&self, _credential: &str) -> Result<GithubValidationResponse, HttpError> {
        Err(HttpError::default())
    }
    fn graphql(
        &self,
        credential: &str,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, HttpError>;
}
pub struct GithubHttp;
impl Http for GithubHttp {
    fn github_validation(&self, credential: &str) -> Result<GithubValidationResponse, HttpError> {
        let response = github_request(
            credential,
            &serde_json::json!({"query":"query { viewer { login } }"}),
        )?;
        let scopes = response
            .headers()
            .get("x-oauth-scopes")
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        let body = response.json().map_err(|_| HttpError::default())?;
        Ok(GithubValidationResponse { scopes, body })
    }

    fn graphql(
        &self,
        credential: &str,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, HttpError> {
        let response = github_request(credential, body)?;
        let status = response.status().as_u16();
        response.json().map_err(|_| HttpError {
            status: Some(status),
            ..HttpError::default()
        })
    }
}

fn github_request(
    credential: &str,
    body: &serde_json::Value,
) -> Result<reqwest::blocking::Response, HttpError> {
    let response = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|_| HttpError::default())?
        .post("https://api.github.com/graphql")
        .header("User-Agent", "kvojps-monitor")
        .bearer_auth(credential)
        .json(body)
        .send()
        .map_err(|_| HttpError::default())?;
    let status = response.status().as_u16();
    if !response.status().is_success() {
        return Err(HttpError::response(
            status,
            response
                .headers()
                .get("retry-after")
                .and_then(|value| value.to_str().ok()),
        ));
    }
    Ok(response)
}

/// Secrets are only read into memory. No filesystem fallback is permitted.
pub trait Keyring: Send {
    fn read(&self, account: &str) -> Result<Option<String>, ()>;
    fn save(&self, account: &str, secret: &str) -> Result<(), ()>;
    /// Removing a missing entry succeeds.
    fn delete(&self, account: &str) -> Result<(), ()>;
}
pub struct UnavailableKeyring;
impl Keyring for UnavailableKeyring {
    fn read(&self, _: &str) -> Result<Option<String>, ()> {
        Err(())
    }
    fn save(&self, _: &str, _: &str) -> Result<(), ()> {
        Err(())
    }
    fn delete(&self, _: &str) -> Result<(), ()> {
        Err(())
    }
}
pub struct SecretService;
impl Keyring for SecretService {
    fn read(&self, account: &str) -> Result<Option<String>, ()> {
        let entry = keyring::Entry::new("kvojps-monitor", account).map_err(|_| ())?;
        match entry.get_password() {
            Ok(secret) => Ok(Some(secret)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(_) => Err(()),
        }
    }
    fn save(&self, account: &str, secret: &str) -> Result<(), ()> {
        keyring::Entry::new("kvojps-monitor", account)
            .map_err(|_| ())?
            .set_password(secret)
            .map_err(|_| ())
    }
    fn delete(&self, account: &str) -> Result<(), ()> {
        let entry = keyring::Entry::new("kvojps-monitor", account).map_err(|_| ())?;
        match entry.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(_) => Err(()),
        }
    }
}

/// Programs reachable through PATH.
pub trait Programs: Send {
    fn installed(&self, name: &str) -> bool;
}
pub struct SystemPrograms;
impl Programs for SystemPrograms {
    fn installed(&self, name: &str) -> bool {
        use std::os::unix::fs::PermissionsExt;
        std::env::var_os("PATH").is_some_and(|path| {
            std::env::split_paths(&path).any(|dir| {
                dir.join(name)
                    .metadata()
                    .is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
            })
        })
    }
}

/// Changes to one file, without blocking.
pub trait Watch: Send {
    /// True when the file changed since the last call.
    fn changed(&mut self) -> bool;
}
/// inotify on the file's directory, since writers replace the file by renaming.
/// Without inotify every call reports a change, so the file is simply reread.
pub struct FileWatch {
    inotify: Option<rustix::fd::OwnedFd>,
    name: std::ffi::OsString,
}
impl FileWatch {
    pub fn new(path: &std::path::Path) -> Self {
        use rustix::fs::inotify;
        let dir = path.parent().unwrap_or(std::path::Path::new("/"));
        let _ = std::fs::create_dir_all(dir);
        let inotify = inotify::init(inotify::CreateFlags::CLOEXEC | inotify::CreateFlags::NONBLOCK)
            .ok()
            .filter(|fd| {
                inotify::add_watch(
                    fd,
                    dir,
                    inotify::WatchFlags::CLOSE_WRITE | inotify::WatchFlags::MOVED_TO,
                )
                .is_ok()
            });
        FileWatch {
            inotify,
            name: path.file_name().unwrap_or_default().to_owned(),
        }
    }
}
impl Watch for FileWatch {
    fn changed(&mut self) -> bool {
        use std::os::unix::ffi::OsStrExt;
        let Some(fd) = &self.inotify else {
            return true;
        };
        let mut buffer = [std::mem::MaybeUninit::uninit(); 4096];
        let mut events = rustix::fs::inotify::Reader::new(fd, &mut buffer);
        let mut changed = false;
        while let Ok(event) = events.next() {
            changed |= event
                .file_name()
                .is_some_and(|name| name.to_bytes() == self.name.as_bytes());
        }
        changed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_watch_sees_a_replaced_file_once_and_ignores_its_neighbours() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("cache/claude-statusline.json");
        let mut watch = FileWatch::new(&file);
        assert!(!watch.changed());
        std::fs::write(dir.path().join("cache/other.json"), "{}").unwrap();
        assert!(!watch.changed());
        let temporary = dir.path().join("cache/claude-statusline.json.tmp");
        std::fs::write(&temporary, "{}").unwrap();
        std::fs::rename(&temporary, &file).unwrap();
        assert!(watch.changed());
        assert!(!watch.changed());
    }
}

/// Nonblocking, best-effort metadata channel between hook subprocesses and core.
pub trait HookSocket: Send {
    fn send(&mut self, bytes: &[u8]) -> io::Result<()>;
    fn receive(&mut self) -> Option<Vec<u8>>;
}
pub struct UnixHookSocket(std::os::unix::net::UnixDatagram);
impl UnixHookSocket {
    pub fn sender(path: &std::path::Path) -> io::Result<Self> {
        let socket = std::os::unix::net::UnixDatagram::unbound()?;
        socket.set_nonblocking(true)?;
        socket.connect(path)?;
        Ok(Self(socket))
    }
    pub fn bind(path: &std::path::Path) -> io::Result<Self> {
        use std::os::unix::fs::PermissionsExt;
        if path.exists() {
            std::fs::remove_file(path)?;
        }
        let socket = std::os::unix::net::UnixDatagram::bind(path)?;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
        socket.set_nonblocking(true)?;
        Ok(Self(socket))
    }
}
impl HookSocket for UnixHookSocket {
    fn send(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.0.send(bytes).map(|_| ())
    }
    fn receive(&mut self) -> Option<Vec<u8>> {
        let mut bytes = vec![0; 65536];
        let length = self.0.recv(&mut bytes).ok()?;
        bytes.truncate(length);
        Some(bytes)
    }
}
