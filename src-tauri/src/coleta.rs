//! Coletores dos Módulos. Os que dependem do Módulo visível são avisados quando ele muda.

use std::sync::Mutex;

use tauri::{AppHandle, Manager};

use crate::estado::Estado;
use crate::sistema;

pub fn iniciar(app: &AppHandle) {
    app.manage(sistema::Coletor::iniciar(app.clone()));
}

/// Chamada depois de toda mudança no Painel ou na vista.
pub fn visibilidade_mudou(app: &AppHandle) {
    let visivel = app
        .state::<Mutex<Estado>>()
        .lock()
        .unwrap()
        .modulo_visivel();
    app.state::<sistema::Coletor>().informar(visivel);
}
