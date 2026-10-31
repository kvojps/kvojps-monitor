//! Module collectors. Notify collectors that depend on visibility when it changes.

use std::sync::Mutex;

use tauri::{AppHandle, Manager};

use crate::state::AppState;
use crate::system;

pub fn start(app: &AppHandle) {
    app.manage(system::Collector::start(app.clone()));
    crate::network::start(app.clone());
    crate::lifecycle::start(app);
    crate::github::start(app.clone());
}

/// Call after every panel or view change.
pub fn visibility_changed(app: &AppHandle) {
    let visible = app
        .state::<Mutex<AppState>>()
        .lock()
        .unwrap()
        .module_visible();
    app.state::<system::Collector>().report(visible);
    if let Some(collector) = app.try_state::<crate::network::Collector>() {
        let _ = collector.0.send(crate::network::Command::Visible(
            visible == Some(crate::module::Module::Network),
        ));
    }
}
