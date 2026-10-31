mod ai;
mod autostart;
mod collection;
mod commands;
mod github;
mod icon;
mod indicator;
mod lifecycle;
mod module;
mod network;
mod panel;
mod persistence;
mod ports;
mod snapshot;
mod state;
mod strings;
mod system;
mod trello;

use std::sync::Mutex;

use ksni::TrayMethods;
use tauri::{Manager, RunEvent, WindowEvent};

use autostart::Execution;
use indicator::Indicator;
use snapshot::Snapshots;
use state::AppState;

pub fn run() {
    tauri::Builder::default()
        // Register first: a second launch opens the live instance panel and exits.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            panel::open(app)
        }))
        .manage(Snapshots::default())
        .setup(|app| {
            let dir_config = dirs::config_dir().ok_or("configuration directory unavailable")?;
            let mut state = AppState::load(dir_config.join("kvojps-monitor"));

            let execution = Execution {
                executable: std::env::current_exe()?,
                appimage: std::env::var_os("APPIMAGE").map(Into::into),
            };
            let first = state.first_execution();
            if let Err(e) = autostart::synchronize(&dir_config.join("autostart"), &execution, first)
            {
                eprintln!("autostart: failed to write entry: {e}");
            } else if first {
                state.complete_first_execution();
            }
            app.manage(Mutex::new(state));
            collection::start(app.handle());

            let indicator = Indicator {
                app: app.handle().clone(),
                state: Default::default(),
            };
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                match indicator.spawn().await {
                    Ok(handle) => {
                        app_handle.manage(handle);
                    }
                    Err(e) => eprintln!("indicator: failed to register tray: {e}"),
                }
            });
            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() != panel::LABEL {
                return;
            }
            match event {
                // Close the panel on focus loss, like a top bars menu.
                WindowEvent::Focused(false) => panel::close(window.app_handle()),
                WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    panel::close(window.app_handle());
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::initial_state,
            commands::report_module_visible,
            commands::close_panel,
            commands::refresh_ai,
            commands::refresh_github,
            commands::save_github_token,
            commands::refresh_trello,
            commands::connect_trello,
            commands::select_trello_board,
            commands::disconnect_trello,
            commands::open_trello_authorization,
            commands::move_trello_card,
            commands::open_trello_card,
            commands::open_trello_admin,
            commands::run_speed_test,
            commands::set_automatic_speed,
        ])
        .build(tauri::generate_context!())
        .expect("failed to start kvojps-monitor")
        .run(|_, event| {
            // Closing destroys the only window; the app stays alive in the indicator.
            if let RunEvent::ExitRequested {
                code: None, api, ..
            } = event
            {
                api.prevent_exit();
            }
        });
}
