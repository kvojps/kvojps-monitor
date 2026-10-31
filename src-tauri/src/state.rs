use std::fs;
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
    /// First run has already enabled autostart.
    #[serde(alias = "inicializado")]
    initialized: bool,
}

/// Core panel state: current visibility and preferences persisted between launches.
pub struct AppState {
    dir_config: PathBuf,
    preferences: Preferences,
    panel_open: bool,
    /// Current UI view; None means Settings.
    view: Option<Module>,
}

impl AppState {
    /// Load preferences; missing or unreadable files count as first run.
    pub fn load(dir_config: PathBuf) -> Self {
        let preferences: Preferences = fs::read(dir_config.join(PREFERENCES_FILE))
            .ok()
            .and_then(|bytes| serde_json::from_slice(&bytes).ok())
            .unwrap_or_default();
        let view = Some(preferences.last_module);
        AppState {
            dir_config,
            preferences,
            panel_open: false,
            view,
        }
    }

    pub fn last_module(&self) -> Module {
        self.preferences.last_module
    }

    /// The UI now shows module; None means Settings.
    pub fn show(&mut self, module: Option<Module>) {
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
        self.view = Some(self.preferences.last_module);
    }

    pub fn close_panel(&mut self) {
        self.panel_open = false;
    }

    /// Currently visible module; some collectors depend on this.
    pub fn module_visible(&self) -> Option<Module> {
        self.view.filter(|_| self.panel_open)
    }

    fn save(&self) {
        // A write failure only loses the remembered module; the app keeps running.
        let _ = fs::create_dir_all(&self.dir_config).and_then(|_| {
            let json =
                serde_json::to_vec_pretty(&self.preferences).map_err(std::io::Error::other)?;
            fs::write(self.dir_config.join(PREFERENCES_FILE), json)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            serde_json::json!({"lastModule":"system","initialized":true})
        );
        let reloaded = AppState::load(d.path().into());
        assert_eq!(reloaded.last_module(), Module::System);
        assert!(!reloaded.first_execution());
    }
}
