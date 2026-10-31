//! Core to UI: publish one snapshot per module on snapshot:<module> whenever it changes.

use std::collections::HashMap;
use std::sync::Mutex;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};

use crate::module::Module;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub module: Module,
    /// Last update in epoch milliseconds; None until data exists.
    pub updated_at: Option<u64>,
    pub data: serde_json::Value,
}

/// Latest snapshot of each module, delivered when the UI loads.
#[derive(Default)]
pub struct Snapshots(Mutex<HashMap<Module, Snapshot>>);

impl Snapshots {
    fn store(&self, snapshot: Snapshot) {
        self.0.lock().unwrap().insert(snapshot.module, snapshot);
    }

    pub fn all(&self) -> Vec<Snapshot> {
        self.0.lock().unwrap().values().cloned().collect()
    }
}

/// Store the snapshot and push it to the UI.
pub fn publish(app: &AppHandle, snapshot: Snapshot) {
    let event = format!("snapshot:{}", snapshot.module.id());
    app.state::<Snapshots>().store(snapshot.clone());
    let _ = app.emit(&event, snapshot);
}
