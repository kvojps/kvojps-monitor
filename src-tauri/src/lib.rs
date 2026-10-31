mod autostart;
mod coleta;
mod comandos;
mod estado;
mod github;
mod icone;
mod indicador;
mod modulo;
mod painel;
mod portas;
mod rede;
mod sistema;
mod snapshot;
mod strings;

use std::sync::Mutex;

use ksni::TrayMethods;
use tauri::{Manager, RunEvent, WindowEvent};

use autostart::Execucao;
use estado::Estado;
use indicador::Indicador;
use snapshot::Snapshots;

pub fn run() {
    tauri::Builder::default()
        // Registrado primeiro: um segundo lançamento só abre o Painel da instância viva e sai.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            painel::abrir(app)
        }))
        .manage(Snapshots::default())
        .setup(|app| {
            let dir_config = dirs::config_dir().ok_or("sem diretório de configuração")?;
            let mut estado = Estado::carregar(dir_config.join("kvojps-monitor"));

            let execucao = Execucao {
                executavel: std::env::current_exe()?,
                appimage: std::env::var_os("APPIMAGE").map(Into::into),
            };
            let primeira = estado.primeira_execucao();
            if let Err(e) =
                autostart::sincronizar(&dir_config.join("autostart"), &execucao, primeira)
            {
                eprintln!("autostart: falha ao gravar a entrada: {e}");
            } else if primeira {
                estado.concluir_primeira_execucao();
            }
            app.manage(Mutex::new(estado));
            coleta::iniciar(app.handle());

            let indicador = Indicador {
                app: app.handle().clone(),
                estado: Default::default(),
            };
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                match indicador.spawn().await {
                    Ok(handle) => {
                        app_handle.manage(handle);
                    }
                    Err(e) => eprintln!("indicador: falha ao registrar o Indicador: {e}"),
                }
            });
            Ok(())
        })
        .on_window_event(|janela, evento| {
            if janela.label() != painel::ROTULO {
                return;
            }
            match evento {
                // Perder o foco fecha o Painel, como num menu da barra superior.
                WindowEvent::Focused(false) => painel::fechar(janela.app_handle()),
                WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    painel::fechar(janela.app_handle());
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            comandos::estado_inicial,
            comandos::informar_modulo_visivel,
            comandos::fechar_painel,
            comandos::atualizar_github,
        ])
        .build(tauri::generate_context!())
        .expect("falha ao iniciar o kvojps-monitor")
        .run(|_, evento| {
            // Fechar o Painel destrói a única janela; o app segue vivo no Indicador.
            if let RunEvent::ExitRequested {
                code: None, api, ..
            } = evento
            {
                api.prevent_exit();
            }
        });
}
