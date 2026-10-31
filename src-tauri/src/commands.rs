//! UI commands sent to the core.

use std::sync::Mutex;

use serde::Serialize;
use tauri::{AppHandle, State};

use crate::collection;
use crate::module::Module;
use crate::panel;
use crate::snapshot::{Snapshot, Snapshots};
use crate::state::AppState;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitialState {
    last_module: Module,
    snapshots: Vec<Snapshot>,
}

#[tauri::command]
pub fn initial_state(state: State<Mutex<AppState>>, snapshots: State<Snapshots>) -> InitialState {
    InitialState {
        last_module: state.lock().unwrap().last_module(),
        snapshots: snapshots.all(),
    }
}

/// None when the UI shows Settings.
#[tauri::command]
pub fn report_module_visible(
    module: Option<Module>,
    state: State<Mutex<AppState>>,
    app: AppHandle,
) {
    state.lock().unwrap().show(module);
    collection::visibility_changed(&app);
}

#[tauri::command]
pub fn close_panel(app: AppHandle) {
    panel::close(&app);
}

#[tauri::command]
pub fn refresh_github(collector: State<crate::github::Collector>) {
    let _ = collector.0.send(crate::github::Command::Refresh);
}

#[tauri::command]
pub fn run_speed_test(collector: State<crate::network::Collector>) {
    let _ = collector.0.send(crate::network::Command::FullSpeed);
}
#[tauri::command]
pub fn set_automatic_speed(enabled: bool, collector: State<crate::network::Collector>) {
    let _ = collector
        .0
        .send(crate::network::Command::AutomaticSpeed(enabled));
}

#[tauri::command]
pub async fn save_github_token(
    token: String,
    collector: State<'_, crate::github::Collector>,
) -> Result<String, crate::github::CredentialError> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    collector
        .0
        .send(crate::github::Command::SaveToken(token, tx))
        .map_err(|_| crate::github::CredentialError::KeyringUnavailable)?;
    rx.await
        .map_err(|_| crate::github::CredentialError::KeyringUnavailable)?
}
