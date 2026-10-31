//! General settings actions: the logs folder and the release page, opened through
//! the process port.
use std::path::Path;

use crate::ports::Processes;
use crate::releases::Release;

/// Open the folder of the metadata-only logs, creating it when nothing was logged yet.
pub fn open_logs(processes: &impl Processes, dir: &Path) -> bool {
    if std::fs::create_dir_all(dir).is_err() {
        return false;
    }
    processes
        .execute("xdg-open", &[&dir.to_string_lossy()])
        .is_ok()
}

/// Open the page of a release found by the daily check, never any other URL.
pub fn open_release(processes: &impl Processes, release: Option<&Release>) -> bool {
    release
        .filter(|r| r.url.starts_with(crate::releases::RELEASES))
        .is_some_and(|r| processes.execute("xdg-open", &[&r.url]).is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[derive(Clone, Default)]
    struct Launcher(Arc<Mutex<Vec<Vec<String>>>>);
    impl Processes for Launcher {
        fn execute(&self, program: &str, args: &[&str]) -> Result<String, ()> {
            let mut call = vec![program.to_string()];
            call.extend(args.iter().map(|a| a.to_string()));
            self.0.lock().unwrap().push(call);
            Ok(String::new())
        }
    }

    #[test]
    fn opening_logs_creates_the_folder_and_opens_it() {
        let d = tempfile::tempdir().unwrap();
        let logs = d.path().join("kvojps-monitor");
        let launcher = Launcher::default();

        assert!(open_logs(&launcher, &logs));

        assert!(logs.is_dir());
        assert_eq!(
            *launcher.0.lock().unwrap(),
            [vec![
                "xdg-open".to_string(),
                logs.to_string_lossy().into_owned()
            ]]
        );
    }

    #[test]
    fn only_a_known_release_page_is_opened() {
        let launcher = Launcher::default();
        assert!(!open_release(&launcher, None));
        assert!(!open_release(
            &launcher,
            Some(&Release {
                version: "9.9.9".into(),
                url: "https://example.com/releases/".into()
            })
        ));
        assert!(launcher.0.lock().unwrap().is_empty());

        let url = "https://github.com/Kvojps/kvojps-monitor/releases/tag/v0.2.0";
        assert!(open_release(
            &launcher,
            Some(&Release {
                version: "0.2.0".into(),
                url: url.into()
            })
        ));
        assert_eq!(
            *launcher.0.lock().unwrap(),
            [vec!["xdg-open".to_string(), url.to_string()]]
        );
    }
}
