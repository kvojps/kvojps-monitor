use crate::persistence::{DiskStorage, Storage};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::module::Module;

// Keep the existing filename so upgrades retain user preferences.
const PREFERENCES_FILE: &str = "preferencias.json";

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Preferences {
    #[serde(alias = "ultimoModulo")]
    last_module: Module,
    agent_notifications: crate::agents::notifications::Preferences,
    /// First run has already enabled autostart.
    #[serde(alias = "inicializado")]
    initialized: bool,
}

/// Core panel state: current visibility and preferences persisted between launches.
pub struct AppState {
    storage: Box<dyn Storage>,
    preferences: Preferences,
    panel_open: bool,
    agent_target: Option<String>,
    /// Current UI view; None means Settings.
    view: Option<Module>,
}

impl AppState {
    /// Load preferences; missing or unreadable files count as first run.
    pub fn load(dir_config: PathBuf) -> Self {
        Self::with_storage(DiskStorage(dir_config))
    }

    pub fn with_storage(storage: impl Storage + 'static) -> Self {
        let preferences: Preferences = storage
            .read(PREFERENCES_FILE)
            .ok()
            .and_then(|json| serde_json::from_str(&json).ok())
            .unwrap_or_default();
        let view = Some(preferences.last_module);
        AppState {
            storage: Box::new(storage),
            preferences,
            panel_open: false,
            agent_target: None,
            view,
        }
    }

    pub fn last_module(&self) -> Module {
        self.preferences.last_module
    }

    /// The UI now shows module; None means Settings.
    pub fn show(&mut self, module: Option<Module>) {
        self.agent_target = None;
        self.view = module;
        if let Some(m) = module {
            self.preferences.last_module = m;
            self.save();
        }
    }

    pub fn first_execution(&self) -> bool {
        !self.preferences.initialized
    }

    pub fn complete_first_execution(&mut self) {
        self.preferences.initialized = true;
        self.save();
    }

    /// Recreate the panel at the last visited module on each opening.
    pub fn open_panel(&mut self) {
        self.panel_open = true;
        self.view = Some(self.opening_module());
    }

    pub fn close_panel(&mut self) {
        self.panel_open = false;
        self.agent_target = None;
    }

    /// Currently visible module; some collectors depend on this.
    pub fn module_visible(&self) -> Option<Module> {
        self.view.filter(|_| self.panel_open)
    }

    pub fn agent_notifications(&self) -> crate::agents::notifications::Preferences {
        self.preferences.agent_notifications
    }
    pub fn set_agent_notifications(
        &mut self,
        preferences: crate::agents::notifications::Preferences,
    ) {
        self.preferences.agent_notifications = preferences;
        self.save();
    }
    pub fn target_agent(&mut self, session: String) {
        self.agent_target = Some(session);
    }
    pub fn agent_target(&self) -> Option<String> {
        self.agent_target.clone()
    }
    pub fn opening_module(&self) -> Module {
        if self.agent_target.is_some() {
            Module::Agents
        } else {
            self.last_module()
        }
    }
    fn save(&self) {
        // A write failure loses preferences; the app keeps running.
        if let Ok(json) = serde_json::to_string_pretty(&self.preferences) {
            let _ = self.storage.write(PREFERENCES_FILE, &json);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn dir() -> tempfile::TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn first_run_opens_ai_module() {
        let d = dir();
        assert_eq!(AppState::load(d.path().into()).last_module(), Module::Ai);
    }

    #[test]
    fn last_visited_module_persists_between_launches() {
        let d = dir();
        let mut state = AppState::load(d.path().into());
        state.show(Some(Module::Network));

        assert_eq!(
            AppState::load(d.path().into()).last_module(),
            Module::Network
        );
    }

    #[test]
    fn settings_preserves_last_module() {
        let d = dir();
        let mut state = AppState::load(d.path().into());
        state.show(Some(Module::Github));
        state.show(None);

        assert_eq!(
            AppState::load(d.path().into()).last_module(),
            Module::Github
        );
    }

    #[test]
    fn unreadable_preferences_count_as_first_run() {
        let d = dir();
        fs::write(d.path().join(PREFERENCES_FILE), "{ invalid json").unwrap();

        assert_eq!(AppState::load(d.path().into()).last_module(), Module::Ai);
    }

    #[test]
    fn closed_panel_has_no_visible_module() {
        let d = dir();
        let mut state = AppState::load(d.path().into());
        state.show(Some(Module::System));

        assert_eq!(state.module_visible(), None);
    }

    #[test]
    fn opening_panel_restores_last_visible_module() {
        let d = dir();
        let mut state = AppState::load(d.path().into());
        state.show(Some(Module::System));
        state.open_panel();
        assert_eq!(state.module_visible(), Some(Module::System));

        state.show(Some(Module::Trello));
        assert_eq!(state.module_visible(), Some(Module::Trello));

        state.close_panel();
        assert_eq!(state.module_visible(), None);
    }

    #[test]
    fn settings_has_no_visible_module() {
        let d = dir();
        let mut state = AppState::load(d.path().into());
        state.open_panel();
        state.show(None);
        assert_eq!(state.module_visible(), None);

        // Closing destroys the webview; reopening reloads the last module.
        state.close_panel();
        state.open_panel();
        assert_eq!(state.module_visible(), Some(Module::Ai));
    }

    #[test]
    fn first_run_ends_when_completed() {
        let d = dir();
        let mut state = AppState::load(d.path().into());
        assert!(state.first_execution());

        state.complete_first_execution();

        assert!(!AppState::load(d.path().into()).first_execution());
    }

    #[test]
    fn completing_first_run_preserves_last_module() {
        let d = dir();
        let mut state = AppState::load(d.path().into());
        state.show(Some(Module::Agents));
        state.complete_first_execution();

        assert_eq!(
            AppState::load(d.path().into()).last_module(),
            Module::Agents
        );
    }
    #[test]
    fn legacy_preferences_preserve_last_module_and_first_run_status() {
        let d = dir();
        fs::write(
            d.path().join(PREFERENCES_FILE),
            r#"{"ultimoModulo":"rede","inicializado":true}"#,
        )
        .unwrap();
        let mut state = AppState::load(d.path().into());
        assert_eq!(state.last_module(), Module::Network);
        assert!(!state.first_execution());
        state.show(Some(Module::System));
        let saved: serde_json::Value =
            serde_json::from_slice(&fs::read(d.path().join(PREFERENCES_FILE)).unwrap()).unwrap();
        assert_eq!(
            saved,
            serde_json::json!({"lastModule":"system","initialized":true,"agentNotifications":{"waiting":true,"done":true,"failed":true}})
        );
        let reloaded = AppState::load(d.path().into());
        assert_eq!(reloaded.last_module(), Module::System);
        assert!(!reloaded.first_execution());
    }
}

#[cfg(test)]
mod agent_notification_tests {
    use super::*;
    use std::{
        io,
        sync::{Arc, Mutex},
    };
    #[derive(Clone, Default)]
    struct MemoryStorage(Arc<Mutex<Option<String>>>);
    impl Storage for MemoryStorage {
        fn read(&self, _: &str) -> io::Result<String> {
            self.0
                .lock()
                .unwrap()
                .clone()
                .ok_or(io::ErrorKind::NotFound.into())
        }
        fn write(&self, _: &str, data: &str) -> io::Result<()> {
            *self.0.lock().unwrap() = Some(data.into());
            Ok(())
        }
        fn rename(&self, _: &str, _: &str) -> io::Result<()> {
            Err(io::ErrorKind::Unsupported.into())
        }
        fn remove(&self, _: &str) -> io::Result<()> {
            Err(io::ErrorKind::Unsupported.into())
        }
    }
    #[test]
    fn notification_target_overrides_opening_module_until_panel_closes() {
        let storage = MemoryStorage::default();
        let mut state = AppState::with_storage(storage.clone());
        state.show(Some(Module::Network));
        state.target_agent("session-1".into());
        state.open_panel();
        assert_eq!(state.module_visible(), Some(Module::Agents));
        assert_eq!(state.agent_target().as_deref(), Some("session-1"));
        assert_eq!(state.last_module(), Module::Network);
        state.close_panel();
        state.open_panel();
        assert_eq!(state.module_visible(), Some(Module::Network));
        assert!(state.agent_target().is_none());
    }
    #[test]
    fn agent_switches_default_to_enabled_and_persist_without_session_content() {
        let storage = MemoryStorage::default();
        let mut state = AppState::with_storage(storage.clone());
        let defaults = state.agent_notifications();
        assert!(defaults.waiting && defaults.done && defaults.failed);
        state.target_agent("PRIVATE_SESSION".into());
        state.set_agent_notifications(crate::agents::notifications::Preferences {
            waiting: false,
            done: true,
            failed: false,
        });
        let reloaded = AppState::with_storage(storage.clone());
        let saved = reloaded.agent_notifications();
        assert!(!saved.waiting && saved.done && !saved.failed);
        assert!(!storage.read(PREFERENCES_FILE).unwrap().contains("PRIVATE_"));
    }
}
