//! Abrir e fechar o Painel. Em repouso, com a webview só ocultada, o app passava de 150 MB (PSS
//! medido no ticket 01); por isso a janela é criada ao abrir e destruída ao fechar.

use std::sync::Mutex;

use tauri::{AppHandle, Manager, WebviewWindowBuilder};

use crate::coleta;
use crate::estado::Estado;

pub const ROTULO: &str = "painel";

pub fn abrir(app: &AppHandle) {
    app.state::<Mutex<Estado>>().lock().unwrap().abrir_painel();
    coleta::visibilidade_mudou(app);
    if let Some(janela) = app.get_webview_window(ROTULO) {
        let _ = janela.show();
        let _ = janela.set_focus();
        return;
    }
    let config = app
        .config()
        .app
        .windows
        .iter()
        .find(|j| j.label == ROTULO)
        .expect("janela do Painel ausente em tauri.conf.json");
    match WebviewWindowBuilder::from_config(app, config).and_then(|b| b.build()) {
        Ok(janela) => {
            let _ = janela.set_focus();
        }
        Err(e) => eprintln!("painel: falha ao criar a janela: {e}"),
    }
}

pub fn fechar(app: &AppHandle) {
    app.state::<Mutex<Estado>>().lock().unwrap().fechar_painel();
    coleta::visibilidade_mudou(app);
    if let Some(janela) = app.get_webview_window(ROTULO) {
        let _ = janela.destroy();
    }
}
