//! Comandos pontuais da UI para o núcleo.

use std::sync::Mutex;

use serde::Serialize;
use tauri::{AppHandle, State};

use crate::estado::Estado;
use crate::modulo::Modulo;
use crate::painel;
use crate::snapshot::{Snapshot, Snapshots};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EstadoInicial {
    ultimo_modulo: Modulo,
    snapshots: Vec<Snapshot>,
}

#[tauri::command]
pub fn estado_inicial(estado: State<Mutex<Estado>>, snapshots: State<Snapshots>) -> EstadoInicial {
    EstadoInicial {
        ultimo_modulo: estado.lock().unwrap().ultimo_modulo(),
        snapshots: snapshots.todos(),
    }
}

/// `None` quando a UI mostra as Configurações.
#[tauri::command]
pub fn informar_modulo_visivel(modulo: Option<Modulo>, estado: State<Mutex<Estado>>) {
    estado.lock().unwrap().mostrar(modulo);
}

#[tauri::command]
pub fn fechar_painel(app: AppHandle) {
    painel::fechar(&app);
}
