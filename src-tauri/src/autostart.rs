use std::io;
use std::path::{Path, PathBuf};

/// How this process was launched.
pub struct Execution {
    /// Running binary (/usr/bin/kvojps-monitor in the .deb).
    pub executable: PathBuf,
    /// AppImage file path from APPIMAGE when running as an AppImage.
    pub appimage: Option<PathBuf>,
}

const FILE: &str = "kvojps-monitor.desktop";

/// Enable autostart on first run, then keep an existing entry pointing to the current
/// executable, since an AppImage may move. If the user removes the entry,
/// do not recreate it.
pub fn synchronize(
    dir_autostart: &Path,
    execution: &Execution,
    first_execution: bool,
) -> io::Result<()> {
    let file = dir_autostart.join(FILE);
    if !first_execution && !file.exists() {
        return Ok(());
    }
    let target = execution.appimage.as_ref().unwrap_or(&execution.executable);
    std::fs::create_dir_all(dir_autostart)?;
    std::fs::write(file, entry(target))
}

/// Whether the app starts with the login.
pub fn enabled(dir_autostart: &Path) -> bool {
    dir_autostart.join(FILE).exists()
}

/// The user's switch in Settings: write the entry for this execution or remove it.
pub fn set(dir_autostart: &Path, execution: &Execution, on: bool) -> io::Result<()> {
    if on {
        return synchronize(dir_autostart, execution, true);
    }
    match std::fs::remove_file(dir_autostart.join(FILE)) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        result => result,
    }
}

fn entry(target: &Path) -> String {
    format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Name=kvojps-monitor\n\
         Exec={}\n\
         Icon=kvojps-monitor\n\
         Terminal=false\n\
         X-GNOME-Autostart-enabled=true\n",
        exec_argument(target)
    )
}

/// Desktop Entry Exec quoting: first quote the argument, then escape the string value,
/// doubling its backslashes.
fn exec_argument(path: &Path) -> String {
    let mut quoted = String::from("\"");
    for c in path.to_string_lossy().chars() {
        match c {
            '"' | '`' | '$' | '\\' => {
                quoted.push('\\');
                quoted.push(c);
            }
            '%' => quoted.push_str("%%"),
            _ => quoted.push(c),
        }
    }
    quoted.push('"');
    quoted.replace('\\', "\\\\")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn deb() -> Execution {
        Execution {
            executable: "/usr/bin/kvojps-monitor".into(),
            appimage: None,
        }
    }

    fn appimage(path: &str) -> Execution {
        Execution {
            executable: "/tmp/.mount_kvojpsXYZ/usr/bin/kvojps-monitor".into(),
            appimage: Some(path.into()),
        }
    }

    fn line_exec(dir: &Path) -> String {
        let content = fs::read_to_string(dir.join("kvojps-monitor.desktop")).unwrap();
        content
            .lines()
            .find(|l| l.starts_with("Exec="))
            .unwrap()
            .to_string()
    }

    #[test]
    fn first_run_enables_autostart() {
        let d = tempfile::tempdir().unwrap();
        let dir = d.path().join("autostart");

        synchronize(&dir, &deb(), true).unwrap();

        let content = fs::read_to_string(dir.join("kvojps-monitor.desktop")).unwrap();
        assert!(content.starts_with("[Desktop Entry]\n"));
        assert!(content.contains("\nType=Application\n"));
        assert_eq!(line_exec(&dir), "Exec=\"/usr/bin/kvojps-monitor\"");
    }

    #[test]
    fn appimage_entry_targets_the_image_instead_of_its_mount() {
        let d = tempfile::tempdir().unwrap();

        synchronize(
            d.path(),
            &appimage("/home/example/Apps/kvojps-monitor.AppImage"),
            true,
        )
        .unwrap();

        assert_eq!(
            line_exec(d.path()),
            "Exec=\"/home/example/Apps/kvojps-monitor.AppImage\""
        );
    }

    #[test]
    fn moving_appimage_updates_the_existing_entry() {
        let d = tempfile::tempdir().unwrap();
        synchronize(
            d.path(),
            &appimage("/home/example/Downloads/kvojps-monitor.AppImage"),
            true,
        )
        .unwrap();

        synchronize(
            d.path(),
            &appimage("/home/example/Apps/kvojps-monitor.AppImage"),
            false,
        )
        .unwrap();

        assert_eq!(
            line_exec(d.path()),
            "Exec=\"/home/example/Apps/kvojps-monitor.AppImage\""
        );
    }

    #[test]
    fn removed_autostart_entry_stays_removed_after_first_run() {
        let d = tempfile::tempdir().unwrap();
        synchronize(d.path(), &appimage("/home/example/a.AppImage"), true).unwrap();
        fs::remove_file(d.path().join("kvojps-monitor.desktop")).unwrap();

        synchronize(d.path(), &appimage("/home/example/a.AppImage"), false).unwrap();
        synchronize(d.path(), &deb(), false).unwrap();

        assert!(!d.path().join("kvojps-monitor.desktop").exists());
    }

    #[test]
    fn exec_path_quotes_spaces_and_reserved_characters() {
        let d = tempfile::tempdir().unwrap();

        synchronize(
            d.path(),
            &appimage("/home/example/Meus Apps/$x 100%.AppImage"),
            true,
        )
        .unwrap();

        assert_eq!(
            line_exec(d.path()),
            "Exec=\"/home/example/Meus Apps/\\\\$x 100%%.AppImage\""
        );
    }

    #[test]
    fn turning_autostart_off_removes_the_entry_and_it_stays_off() {
        let d = tempfile::tempdir().unwrap();
        synchronize(d.path(), &deb(), true).unwrap();
        assert!(enabled(d.path()));

        set(d.path(), &deb(), false).unwrap();
        synchronize(d.path(), &deb(), false).unwrap();

        assert!(!enabled(d.path()));
    }

    #[test]
    fn turning_autostart_on_writes_the_current_appimage_path() {
        let d = tempfile::tempdir().unwrap();
        set(d.path(), &appimage("/home/example/Apps/k.AppImage"), true).unwrap();
        assert!(enabled(d.path()));
        assert_eq!(line_exec(d.path()), "Exec=\"/home/example/Apps/k.AppImage\"");

        // Turning it off twice is not an error.
        set(d.path(), &deb(), false).unwrap();
        set(d.path(), &deb(), false).unwrap();
        assert!(!enabled(d.path()));
    }

    #[test]
    fn turning_autostart_off_keeps_other_entries() {
        let d = tempfile::tempdir().unwrap();
        fs::write(d.path().join("other.desktop"), "x").unwrap();
        set(d.path(), &deb(), true).unwrap();

        set(d.path(), &deb(), false).unwrap();

        assert!(d.path().join("other.desktop").exists());
    }
}
