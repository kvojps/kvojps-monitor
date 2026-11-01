// Fixture mode bypasses every collector; the default build still lints them.
#![cfg_attr(feature = "fixtures", allow(dead_code))]

mod agents;
mod ai;
mod autostart;
#[cfg(feature = "fixtures")]
mod capture;
mod collection;
mod commands;
mod desktop_theme;
#[cfg(feature = "fixtures")]
mod fixtures;
mod general;
mod github;
mod icon;
mod indicator;
mod lifecycle;
mod module;
mod network;
mod panel;
mod persistence;
mod ports;
mod releases;
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

/// Status line mode: never starts the app.
pub fn copy_status_line(cache: &str) {
    ai::copy_from_stdin(std::path::Path::new(cache));
}

pub fn run() {
    tauri::Builder::default()
        // Register first: a second launch opens the live instance panel and exits.
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            second_launch(app, &args)
        }))
        .manage(Snapshots::default())
        .setup(|app| {
            let dir_config = config_dir().ok_or("configuration directory unavailable")?;
            let mut state = AppState::load(dir_config.join("kvojps-monitor"));
            #[cfg(feature = "fixtures")]
            state.complete_welcome();

            let execution = Execution {
                executable: std::env::current_exe()?,
                appimage: std::env::var_os("APPIMAGE").map(Into::into),
            };
            let first = state.first_execution();
            let dir_autostart = dir_config.join("autostart");
            if let Err(e) = autostart::synchronize(
                &crate::persistence::DiskStorage(dir_autostart.clone()),
                &execution,
                first,
            ) {
                eprintln!("autostart: failed to write entry: {e}");
            } else if first {
                state.complete_first_execution();
            }
            app.manage(Mutex::new(state));
            app.manage(commands::Startup {
                dir: dir_autostart,
                execution,
            });
            desktop_theme::start(app.handle());
            start_sources(app.handle());

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
            commands::set_agent_notifications,
            commands::set_agent_hooks,
            commands::report_visible_modules,
            commands::choose_anchor,
            commands::close_panel,
            commands::refresh_ai,
            commands::connect_claude,
            commands::disconnect_claude,
            commands::set_claude_usage,
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
            commands::general_settings,
            commands::set_autostart,
            commands::set_version_check,
            commands::open_release,
            commands::open_logs,
            commands::set_appearance,
            commands::complete_welcome,
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

#[cfg(not(feature = "fixtures"))]
fn second_launch(app: &tauri::AppHandle, _args: &[String]) {
    panel::open(app)
}

/// In fixture mode, `--capture` asks the running instance for a panel capture.
#[cfg(feature = "fixtures")]
fn second_launch(app: &tauri::AppHandle, args: &[String]) {
    match capture::request(args) {
        None => panel::open(app),
        Some(Ok(request)) => capture::start(app, request),
        Some(Err(e)) => eprintln!("capture: {e}"),
    }
}

#[cfg(not(feature = "fixtures"))]
fn config_dir() -> Option<std::path::PathBuf> {
    dirs::config_dir()
}

#[cfg(not(feature = "fixtures"))]
fn start_sources(app: &tauri::AppHandle) {
    collection::start(app);
    releases::start(app);
}

/// Fixture mode keeps preferences and autostart out of the user's configuration.
#[cfg(feature = "fixtures")]
fn config_dir() -> Option<std::path::PathBuf> {
    Some(fixtures::config_dir())
}

#[cfg(feature = "fixtures")]
fn start_sources(app: &tauri::AppHandle) {
    fixtures::start(app);
}

/// Best-effort hook subprocess entry point, before GUI/single-instance setup.
pub fn run_hook(provider: &str, event: &str) {
    agents::hooks::run(provider, event);
}

#[cfg(test)]
mod build_tests {
    /// Production builds never contain fixture mode: the feature is off by default,
    /// no build command enables it, and fixtures.rs refuses release builds.
    #[test]
    fn production_build_excludes_fixture_mode() {
        let manifest = include_str!("../Cargo.toml");
        let features = manifest
            .split("[features]")
            .nth(1)
            .expect("Cargo.toml declares features");
        let features = features.split("\n[").next().unwrap();
        assert!(features.contains("default = []"));
        let fixtures = features
            .lines()
            .find(|l| l.starts_with("fixtures = "))
            .expect("fixtures feature declared");
        // The feature only enables optional crates; nothing else depends on them.
        assert_eq!(
            fixtures,
            r#"fixtures = ["dep:webkit2gtk", "dep:javascriptcore-rs", "dep:cairo-rs"]"#
        );
        for dependency in ["webkit2gtk", "javascriptcore-rs", "cairo-rs"] {
            let line = manifest
                .lines()
                .find(|l| l.starts_with(&format!("{dependency} = ")))
                .expect("optional dependency declared");
            assert!(line.contains("optional = true"), "{line}");
        }
        let config = include_str!("../tauri.conf.json");
        assert!(!config.contains("fixtures"));
        assert!(
            include_str!("fixtures.rs").contains("#[cfg(not(debug_assertions))]\ncompile_error!")
        );
        let scripts = include_str!("../../package.json");
        assert!(scripts.contains("\"dev:fixtures\": \"tauri dev --features fixtures\""));
        assert!(scripts.contains("\"build\": \"vite build\""));
    }
}
