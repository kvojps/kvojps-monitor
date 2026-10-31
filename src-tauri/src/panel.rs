//! Open and close the panel. Keeping a hidden webview exceeded 150 MB PSS (ticket 01),
//! so create the window on open and destroy it on close.

use std::sync::Mutex;

use tauri::{AppHandle, Manager, WebviewWindowBuilder};

use crate::collection;
use crate::state::AppState;

pub const LABEL: &str = "panel";

pub fn open(app: &AppHandle) {
    app.state::<Mutex<AppState>>().lock().unwrap().open_panel();
    collection::visibility_changed(app);
    if let Some(window) = app.get_webview_window(LABEL) {
        let _ = window.show();
        let _ = window.set_focus();
        return;
    }
    let config = app
        .config()
        .app
        .windows
        .iter()
        .find(|j| j.label == LABEL)
        .expect("panel window missing from tauri.conf.json");
    match WebviewWindowBuilder::from_config(app, config).and_then(|b| b.build()) {
        Ok(window) => {
            let _ = window.set_focus();
        }
        Err(e) => eprintln!("panel: failed to create window: {e}"),
    }
}

pub fn close(app: &AppHandle) {
    app.state::<Mutex<AppState>>().lock().unwrap().close_panel();
    collection::visibility_changed(app);
    if let Some(window) = app.get_webview_window(LABEL) {
        let _ = window.destroy();
    }
}

/// Notification clicks override the remembered module, also in an already open panel.
pub fn open_agent(app: &AppHandle, session: String) {
    use tauri::Emitter;
    app.state::<Mutex<AppState>>()
        .lock()
        .unwrap()
        .target_agent(session.clone());
    open(app);
    let _ = app.emit_to(LABEL, "agent-target", session);
}
