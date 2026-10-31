//! Open and close the panel. Keeping a hidden webview exceeded 150 MB PSS (ticket 01),
//! so create the window on open and destroy it on close.
//!
//! The panel fills 95% of the monitor work area. Mutter places the window
//! (ADR 0001): the app only chooses its size.

use std::sync::Mutex;

use tauri::{AppHandle, Manager, Monitor, PhysicalSize, WebviewWindow, WebviewWindowBuilder};

use crate::collection;
use crate::state::AppState;

pub const LABEL: &str = "panel";

pub fn open(app: &AppHandle) {
    app.state::<Mutex<AppState>>().lock().unwrap().open_panel();
    collection::visibility_changed(app);
    if let Some(window) = app.get_webview_window(LABEL) {
        fit(&window);
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
    let mut builder = match WebviewWindowBuilder::from_config(app, config) {
        Ok(builder) => builder,
        Err(e) => return eprintln!("panel: failed to create window: {e}"),
    };
    // Start with the primary monitor size; fit() corrects it once the compositor
    // has chosen the monitor.
    if let Some(monitor) = app.primary_monitor().ok().flatten() {
        let size = work_area_size(&monitor).to_logical::<f64>(monitor.scale_factor());
        builder = builder.inner_size(size.width, size.height);
    }
    match builder.build() {
        Ok(window) => {
            fit(&window);
            let _ = window.set_focus();
        }
        Err(e) => eprintln!("panel: failed to create window: {e}"),
    }
}

/// Resize to the work area of the monitor the window is on.
fn fit(window: &WebviewWindow) {
    let monitor = window
        .current_monitor()
        .ok()
        .flatten()
        .or_else(|| window.primary_monitor().ok().flatten());
    if let Some(monitor) = monitor {
        let _ = window.set_size(work_area_size(&monitor));
    }
}

fn work_area_size(monitor: &Monitor) -> PhysicalSize<u32> {
    let area = monitor.work_area().size;
    let (width, height) = panel_size(area.width, area.height);
    PhysicalSize::new(width, height)
}

/// 95% of the work area, in physical pixels so scale is already applied.
fn panel_size(width: u32, height: u32) -> (u32, u32) {
    (width * 95 / 100, height * 95 / 100)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_fills_ninety_five_percent_of_the_work_area() {
        // 1920x1080 at 100% with the 32 px GNOME top bar; never rounds above 95%.
        assert_eq!(panel_size(1920, 1048), (1824, 995));
        // 2880x1800 at 200%: the work area already comes in physical pixels.
        assert_eq!(panel_size(2880, 1736), (2736, 1649));
    }
}
