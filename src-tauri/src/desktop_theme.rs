//! Desktop color scheme from the XDG Settings portal, so the "system" theme follows
//! later changes in GNOME even when the webview misses them.
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SystemTheme {
    Dark,
    Light,
}

const NAMESPACE: &str = "org.freedesktop.appearance";
const KEY: &str = "color-scheme";

/// Portal values: 1 prefers dark; 0 (GNOME's default style) and 2 render light.
fn scheme(value: u32) -> SystemTheme {
    if value == 1 {
        SystemTheme::Dark
    } else {
        SystemTheme::Light
    }
}

/// Body `(ssv)` of `SettingChanged`; None for any other setting.
fn setting_changed(message: &zbus::Message) -> Option<SystemTheme> {
    let (namespace, key, value): (String, String, zbus::zvariant::OwnedValue) =
        message.body().deserialize().ok()?;
    if namespace != NAMESPACE || key != KEY {
        return None;
    }
    u32::try_from(value).ok().map(scheme)
}

/// Latest desktop theme; None until the portal answers.
#[derive(Default, Clone)]
pub struct Current(std::sync::Arc<std::sync::Mutex<Option<SystemTheme>>>);
impl Current {
    pub fn get(&self) -> Option<SystemTheme> {
        *self.0.lock().unwrap()
    }
}

/// Read the portal and follow its changes, emitting `system-theme` to the panel.
pub fn start(app: &tauri::AppHandle) {
    use tauri::{Emitter, Manager};
    let current = Current::default();
    app.manage(current.clone());
    let app = app.clone();
    std::thread::spawn(move || loop {
        let _ = (|| -> zbus::Result<()> {
            let bus = zbus::blocking::Connection::session()?;
            let proxy = zbus::blocking::Proxy::new(
                &bus,
                "org.freedesktop.portal.Desktop",
                "/org/freedesktop/portal/desktop",
                "org.freedesktop.portal.Settings",
            )?;
            let signals = proxy.receive_signal("SettingChanged")?;
            let initial: zbus::zvariant::OwnedValue = proxy.call("ReadOne", &(NAMESPACE, KEY))?;
            let publish = |theme: SystemTheme| {
                *current.0.lock().unwrap() = Some(theme);
                let _ = app.emit("system-theme", theme);
            };
            if let Ok(value) = u32::try_from(initial) {
                publish(scheme(value));
            }
            for message in signals {
                if let Some(theme) = setting_changed(&message) {
                    publish(theme);
                }
            }
            Ok(())
        })();
        // Without the portal the panel falls back to the webview media query.
        std::thread::sleep(std::time::Duration::from_secs(30));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn changed(namespace: &str, key: &str, value: u32) -> zbus::Message {
        zbus::Message::signal(
            "/org/freedesktop/portal/desktop",
            "org.freedesktop.portal.Settings",
            "SettingChanged",
        )
        .unwrap()
        .build(&(namespace, key, zbus::zvariant::Value::U32(value)))
        .unwrap()
    }

    #[test]
    fn portal_color_scheme_maps_to_a_theme_and_follows_changes() {
        let fixture: serde_json::Value =
            serde_json::from_str(include_str!("../fixtures/appearance/portal.json")).unwrap();
        assert_eq!(
            scheme(fixture["colorScheme"].as_u64().unwrap() as u32),
            SystemTheme::Dark
        );

        let changes: Vec<_> = fixture["changedSequence"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| {
                setting_changed(&changed(
                    "org.freedesktop.appearance",
                    "color-scheme",
                    v.as_u64().unwrap() as u32,
                ))
            })
            .collect();
        // GNOME's default style reports "no preference" and renders light.
        assert_eq!(
            changes,
            [
                Some(SystemTheme::Light),
                Some(SystemTheme::Light),
                Some(SystemTheme::Dark)
            ]
        );
    }

    #[test]
    fn unrelated_settings_are_ignored() {
        assert_eq!(
            setting_changed(&changed("org.freedesktop.appearance", "accent-color", 1)),
            None
        );
        assert_eq!(
            setting_changed(&changed("org.gnome.desktop.interface", "color-scheme", 1)),
            None
        );
    }
}
