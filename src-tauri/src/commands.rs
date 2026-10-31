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
    agent_target: Option<String>,
    snapshots: Vec<Snapshot>,
}

#[tauri::command]
pub fn initial_state(state: State<Mutex<AppState>>, snapshots: State<Snapshots>) -> InitialState {
    let state = state.lock().unwrap();
    InitialState {
        agent_target: state.agent_target(),
        last_module: state.opening_module(),
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
pub fn refresh_ai(collector: State<crate::ai::Collector>) {
    let _ = collector.0.send(crate::ai::Command::Refresh);
}

/// Only on the user's click: changes `~/.claude/settings.json`.
#[tauri::command]
pub async fn connect_claude(
    collector: State<'_, crate::ai::Collector>,
) -> Result<(), crate::ai::ClaudeError> {
    ask_ai(collector, crate::ai::Command::ConnectClaude).await
}

#[tauri::command]
pub async fn disconnect_claude(
    collector: State<'_, crate::ai::Collector>,
) -> Result<(), crate::ai::ClaudeError> {
    ask_ai(collector, crate::ai::Command::DisconnectClaude).await
}

/// Only on the user's click, after the warning about the undocumented endpoint.
#[tauri::command]
pub async fn set_claude_usage(
    enabled: bool,
    collector: State<'_, crate::ai::Collector>,
) -> Result<(), crate::ai::ClaudeError> {
    ask_ai(collector, |reply| {
        crate::ai::Command::SetClaudeUsage(enabled, reply)
    })
    .await
}

async fn ask_ai(
    collector: State<'_, crate::ai::Collector>,
    command: impl FnOnce(
        tokio::sync::oneshot::Sender<Result<(), crate::ai::ClaudeError>>,
    ) -> crate::ai::Command,
) -> Result<(), crate::ai::ClaudeError> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    collector
        .0
        .send(command(tx))
        .map_err(|_| crate::ai::ClaudeError::Unwritable)?;
    rx.await.map_err(|_| crate::ai::ClaudeError::Unwritable)?
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

#[tauri::command]
pub fn refresh_trello(collector: State<crate::trello::Collector>) {
    let _ = collector.0.send(crate::trello::Command::Refresh);
}

/// Send a command with a reply channel to the Trello collector thread.
async fn ask_trello<T>(
    collector: State<'_, crate::trello::Collector>,
    command: impl FnOnce(
        tokio::sync::oneshot::Sender<Result<T, crate::trello::CredentialError>>,
    ) -> crate::trello::Command,
) -> Result<T, crate::trello::CredentialError> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    collector
        .0
        .send(command(tx))
        .map_err(|_| crate::trello::CredentialError::KeyringUnavailable)?;
    rx.await
        .map_err(|_| crate::trello::CredentialError::KeyringUnavailable)?
}

#[tauri::command]
pub async fn connect_trello(
    key: String,
    token: String,
    collector: State<'_, crate::trello::Collector>,
) -> Result<String, crate::trello::CredentialError> {
    ask_trello(collector, |reply| {
        crate::trello::Command::Connect(key, token, reply)
    })
    .await
}

#[tauri::command]
pub async fn select_trello_board(
    board: String,
    collector: State<'_, crate::trello::Collector>,
) -> Result<(), crate::trello::CredentialError> {
    ask_trello(collector, |reply| {
        crate::trello::Command::SelectBoard(board, reply)
    })
    .await
}

#[tauri::command]
pub async fn disconnect_trello(
    collector: State<'_, crate::trello::Collector>,
) -> Result<(), crate::trello::CredentialError> {
    ask_trello(collector, crate::trello::Command::Disconnect).await
}

/// The collector remembers the key, since the browser closes the panel.
#[tauri::command]
pub async fn open_trello_authorization(
    key: String,
    collector: State<'_, crate::trello::Collector>,
) -> Result<(), crate::trello::CredentialError> {
    ask_trello(collector, |reply| {
        crate::trello::Command::Authorize(key, reply)
    })
    .await
}

/// Rejects when Trello refuses or cannot be reached; the panel then undoes its
/// optimistic move.
#[tauri::command]
pub async fn move_trello_card(
    card: String,
    list: String,
    collector: State<'_, crate::trello::Collector>,
) -> Result<(), crate::trello::MoveError> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    collector
        .0
        .send(crate::trello::Command::Move(card, list, tx))
        .map_err(|_| crate::trello::MoveError)?;
    rx.await.map_err(|_| crate::trello::MoveError)?
}

// Async commands run off the main thread while xdg-open hands the URL over.
#[tauri::command]
pub async fn open_trello_card(url: String) {
    crate::trello::open_card(&crate::ports::SystemProcesses, &url);
}

#[tauri::command]
pub async fn open_trello_admin() {
    crate::trello::open_admin(&crate::ports::SystemProcesses);
}

#[tauri::command]
pub fn set_agent_notifications(
    preferences: crate::agents::notifications::Preferences,
    state: State<Mutex<AppState>>,
) {
    state.lock().unwrap().set_agent_notifications(preferences);
}
