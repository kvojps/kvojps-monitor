//! Núcleo → UI: um snapshot por Módulo, empurrado no evento `snapshot:<módulo>` quando muda.

use std::collections::HashMap;
use std::sync::Mutex;

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};

use crate::modulo::Modulo;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub modulo: Modulo,
    /// Epoch em ms da última atualização; None enquanto não há dado.
    pub atualizado_em: Option<u64>,
    pub dados: serde_json::Value,
}

/// Último snapshot de cada Módulo, entregue à UI quando ela carrega.
#[derive(Default)]
pub struct Snapshots(Mutex<HashMap<Modulo, Snapshot>>);

impl Snapshots {
    fn guardar(&self, snapshot: Snapshot) {
        self.0.lock().unwrap().insert(snapshot.modulo, snapshot);
    }

    pub fn todos(&self) -> Vec<Snapshot> {
        self.0.lock().unwrap().values().cloned().collect()
    }
}

/// Guarda o snapshot e o empurra para a UI. Os coletores dos Módulos chegam nos próximos tickets.
#[allow(dead_code)]
pub fn publicar(app: &AppHandle, snapshot: Snapshot) {
    let evento = format!("snapshot:{}", snapshot.modulo.id());
    app.state::<Snapshots>().guardar(snapshot.clone());
    let _ = app.emit(&evento, snapshot);
}
