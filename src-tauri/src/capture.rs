//! Development-only panel capture for agents (fixture mode). GNOME refuses
//! `org.gnome.Shell.Screenshot` to ordinary clients, so the running instance renders
//! its own webview with WebKitGTK's snapshot and measures the panel's scroll.
//!
//! A second launch carries the request through the single instance:
//!
//! ```sh
//! src-tauri/target/debug/kvojps-monitor --capture /tmp/out/b-1100 --size 1100x700 --theme dark
//! ```
//!
//! The running instance reopens the panel at that logical size and theme, waits for
//! the layout, then writes `<prefix>.png` and, last, `<prefix>.json` (metrics or error).

use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use serde_json::{json, Value};
use tauri::{AppHandle, LogicalSize, Manager};
use webkit2gtk::{SnapshotOptions, SnapshotRegion, WebViewExt};

use crate::panel;
use crate::state::{AppState, Appearance, Theme};

#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    /// Output path without extension.
    pub prefix: PathBuf,
    pub width: u32,
    pub height: u32,
    /// `None` keeps the saved appearance.
    pub theme: Option<Theme>,
}

/// `None` when the launch is not a capture request.
pub fn request(args: &[String]) -> Option<Result<Request, String>> {
    let position = args.iter().position(|a| a == "--capture")?;
    Some(parse(&args[position..]))
}

fn parse(args: &[String]) -> Result<Request, String> {
    let mut request = Request {
        prefix: PathBuf::new(),
        width: 1100,
        height: 700,
        theme: None,
    };
    let mut rest = args.iter();
    while let Some(flag) = rest.next() {
        let value = rest
            .next()
            .ok_or_else(|| format!("{flag} expects a value"))?;
        match flag.as_str() {
            "--capture" => request.prefix = PathBuf::from(value),
            "--size" => {
                let (w, h) = value
                    .split_once('x')
                    .ok_or_else(|| format!("expected WIDTHxHEIGHT, got {value:?}"))?;
                let dimension = |d: &str| {
                    d.parse::<u32>()
                        .ok()
                        .filter(|n| (200..=4000).contains(n))
                        .ok_or_else(|| format!("invalid size {value:?}"))
                };
                request.width = dimension(w)?;
                request.height = dimension(h)?;
            }
            "--theme" => {
                request.theme = Some(match value.as_str() {
                    "dark" => Theme::Dark,
                    "light" => Theme::Light,
                    "system" => Theme::System,
                    _ => return Err(format!("invalid theme {value:?}")),
                })
            }
            _ => return Err(format!("unknown option {flag:?}")),
        }
    }
    if !request.prefix.is_absolute() {
        return Err("--capture expects an absolute path prefix".into());
    }
    Ok(request)
}

/// Layout, theme and scroll of the panel. `overflowing` lists every scroll container
/// whose content exceeds its box; empty means no scroll. `styles` lists the
/// stylesheets the Vite dev server injected: a component missing there rendered
/// without its CSS (seen on a cold Vite start), so the capture is not valid.
const METRICS_SCRIPT: &str = r#"(() => {
  const box = (e) => ({ scrollHeight: e.scrollHeight, clientHeight: e.clientHeight, scrollWidth: e.scrollWidth, clientWidth: e.clientWidth });
  const name = (e) => e.tagName.toLowerCase() + (typeof e.className === "string" && e.className.trim() ? "." + e.className.trim().split(/\s+/).join(".") : "");
  const overflowing = [...document.querySelectorAll("*")].filter((e) => {
    const style = getComputedStyle(e);
    const scrolls = (o) => o === "auto" || o === "scroll";
    return (scrolls(style.overflowY) && e.scrollHeight > e.clientHeight + 1) || (scrolls(style.overflowX) && e.scrollWidth > e.clientWidth + 1);
  }).map((e) => ({ element: name(e), ...box(e) }));
  const scroller = document.querySelector(".panel .scroller");
  return JSON.stringify({
    ready: !!document.querySelector(".panel .composition") && document.fonts.status === "loaded",
    width: innerWidth,
    height: innerHeight,
    devicePixelRatio,
    theme: document.documentElement.dataset.theme ?? null,
    scroller: scroller ? box(scroller) : null,
    document: box(document.documentElement),
    overflowing,
    styles: [...document.querySelectorAll("style[data-vite-dev-id]")].map((e) => e.dataset.viteDevId.split("/src/").pop().split("?")[0]).sort(),
  });
})()"#;

/// Time for entry transitions after the layout reaches the requested size.
const SETTLE: Duration = Duration::from_millis(1500);
const TIMEOUT: Duration = Duration::from_secs(15);

pub fn start(app: &AppHandle, request: Request) {
    if let Some(theme) = request.theme {
        let state = app.state::<Mutex<AppState>>();
        let mut state = state.lock().unwrap();
        let appearance = Appearance {
            theme,
            ..state.appearance()
        };
        if let Err(e) = state.set_appearance(appearance) {
            eprintln!("capture: failed to save appearance: {e}");
        }
    }
    // Reopen so the webview starts from the saved appearance and a fresh layout.
    panel::close(app);
    let app = app.clone();
    std::thread::spawn(move || {
        let outcome = reopen(&app).and_then(|()| capture(&app, &request));
        let report = match outcome {
            Ok(metrics) => json!({ "png": png_path(&request), "metrics": metrics }),
            Err(e) => json!({ "error": e }),
        };
        let path = request.prefix.with_extension("json");
        let text = serde_json::to_string_pretty(&report).unwrap();
        match std::fs::write(&path, text) {
            Ok(()) => eprintln!("capture: wrote {}", path.display()),
            Err(e) => eprintln!("capture: failed to write {}: {e}", path.display()),
        }
    });
}

/// Destroying a window is asynchronous: opening before it is gone would only show
/// the dying window.
fn reopen(app: &AppHandle) -> Result<(), String> {
    let gone = |app: &AppHandle| app.get_webview_window(panel::LABEL).is_none();
    poll(|| gone(app)).map_err(|()| "previous panel window was not destroyed".to_owned())?;
    let handle = app.clone();
    app.run_on_main_thread(move || panel::open(&handle))
        .map_err(|e| format!("open: {e}"))?;
    poll(|| !gone(app)).map_err(|()| "panel window did not open".to_owned())
}

fn poll(done: impl Fn() -> bool) -> Result<(), ()> {
    let deadline = Instant::now() + TIMEOUT;
    while !done() {
        if Instant::now() > deadline {
            return Err(());
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    Ok(())
}

fn png_path(request: &Request) -> PathBuf {
    request.prefix.with_extension("png")
}

fn capture(app: &AppHandle, request: &Request) -> Result<Value, String> {
    if let Some(dir) = request.prefix.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("output directory: {e}"))?;
    }
    let window = app
        .get_webview_window(panel::LABEL)
        .ok_or("panel window unavailable")?;
    let opened = wait(&window, |m| m["ready"] == true)?;

    // Some compositors give a smaller viewport than the size asked for (tao reserves
    // window margins on Wayland), so ask again with the measured difference.
    let mut asked = (f64::from(request.width), f64::from(request.height));
    let mut metrics = opened.clone();
    for _ in 0..3 {
        window
            .set_size(LogicalSize::new(asked.0, asked.1))
            .map_err(|e| format!("resize: {e}"))?;
        std::thread::sleep(Duration::from_millis(300));
        metrics = wait(&window, |m| m["ready"] == true)?;
        let (width, height) = viewport(&metrics);
        let missing = (
            f64::from(request.width) - width,
            f64::from(request.height) - height,
        );
        if missing.0.abs() <= 1.0 && missing.1.abs() <= 1.0 {
            break;
        }
        asked = (asked.0 + missing.0, asked.1 + missing.1);
    }
    let (width, height) = viewport(&metrics);
    if (width - f64::from(request.width)).abs() > 1.0
        || (height - f64::from(request.height)).abs() > 1.0
    {
        return Err(format!(
            "viewport stayed at {width} × {height}; opened at {}",
            opened
        ));
    }
    std::thread::sleep(SETTLE);
    let mut metrics = measure(&window)?;
    let missing = missing_styles(&metrics);
    if !missing.is_empty() {
        return Err(format!(
            "rendered without the CSS of {missing:?}; restart `npm run dev:fixtures`"
        ));
    }
    snapshot(&window, png_path(request))?;
    // The size the panel takes on its own, before the requested resize.
    metrics["opened"] = json!({ "width": opened["width"], "height": opened["height"] });
    Ok(metrics)
}

/// Stylesheets every panel capture needs, as listed in `styles`.
const REQUIRED_STYLES: [&str; 8] = [
    "tokens.css",
    "lib/Panel.svelte",
    "lib/ai/AiModule.svelte",
    "lib/agents/AgentsModule.svelte",
    "lib/github/GithubModule.svelte",
    "lib/trello/TrelloModule.svelte",
    "lib/network/NetworkModule.svelte",
    "lib/system/SystemModule.svelte",
];

fn missing_styles(metrics: &Value) -> Vec<&'static str> {
    let loaded: Vec<&str> = metrics["styles"]
        .as_array()
        .map(|styles| styles.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    REQUIRED_STYLES
        .into_iter()
        .filter(|style| !loaded.contains(style))
        .collect()
}

fn viewport(metrics: &Value) -> (f64, f64) {
    let size = |key: &str| metrics[key].as_f64().unwrap_or(0.0);
    (size("width"), size("height"))
}

/// Measure until `done` holds, up to `TIMEOUT`.
fn wait(window: &tauri::WebviewWindow, done: impl Fn(&Value) -> bool) -> Result<Value, String> {
    let deadline = Instant::now() + TIMEOUT;
    loop {
        let metrics = measure(window)?;
        if done(&metrics) {
            return Ok(metrics);
        }
        if Instant::now() > deadline {
            return Err(format!("layout not ready: {metrics}"));
        }
        std::thread::sleep(Duration::from_millis(200));
    }
}

/// Run `METRICS_SCRIPT` in the panel webview on the main thread.
fn measure(window: &tauri::WebviewWindow) -> Result<Value, String> {
    let (send, receive) = mpsc::channel();
    window
        .with_webview(move |webview| {
            webview.inner().evaluate_javascript(
                METRICS_SCRIPT,
                None,
                None,
                None::<&webkit2gtk::gio::Cancellable>,
                move |result| {
                    use javascriptcore::ValueExt;
                    let _ = send.send(
                        result
                            .map(|value| value.to_str().to_string())
                            .map_err(|e| e.to_string()),
                    );
                },
            );
        })
        .map_err(|e| format!("webview: {e}"))?;
    let text = receive
        .recv_timeout(TIMEOUT)
        .map_err(|_| "metrics script did not answer".to_owned())??;
    serde_json::from_str(&text).map_err(|e| format!("metrics: {e}"))
}

/// Render the visible webview area and save it as PNG.
fn snapshot(window: &tauri::WebviewWindow, path: PathBuf) -> Result<(), String> {
    let (send, receive) = mpsc::channel();
    window
        .with_webview(move |webview| {
            webview.inner().snapshot(
                SnapshotRegion::Visible,
                SnapshotOptions::NONE,
                None::<&webkit2gtk::gio::Cancellable>,
                move |result| {
                    let saved = result.map_err(|e| e.to_string()).and_then(|surface| {
                        let image = cairo::ImageSurface::try_from(surface)
                            .map_err(|_| "snapshot is not an image surface".to_owned())?;
                        let mut file = std::fs::File::create(&path).map_err(|e| e.to_string())?;
                        image.write_to_png(&mut file).map_err(|e| e.to_string())
                    });
                    let _ = send.send(saved);
                },
            );
        })
        .map_err(|e| format!("webview: {e}"))?;
    receive
        .recv_timeout(TIMEOUT)
        .map_err(|_| "snapshot did not answer".to_owned())?
        .map_err(|e| format!("snapshot: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(text: &str) -> Vec<String> {
        text.split_whitespace().map(str::to_owned).collect()
    }

    #[test]
    fn ordinary_second_launch_is_not_a_capture() {
        assert_eq!(request(&args("kvojps-monitor")), None);
    }

    #[test]
    fn parses_prefix_size_and_theme() {
        let parsed = request(&args(
            "kvojps-monitor --capture /tmp/b-1000 --size 1000x700 --theme light",
        ));
        assert_eq!(
            parsed,
            Some(Ok(Request {
                prefix: "/tmp/b-1000".into(),
                width: 1000,
                height: 700,
                theme: Some(Theme::Light),
            }))
        );
        let default = request(&args("kvojps-monitor --capture /tmp/b"))
            .unwrap()
            .unwrap();
        assert_eq!(
            (default.width, default.height, default.theme),
            (1100, 700, None)
        );
    }

    #[test]
    fn a_panel_without_a_module_stylesheet_is_not_captured() {
        let all = json!({ "styles": REQUIRED_STYLES });
        assert!(missing_styles(&all).is_empty());
        let cold = json!({ "styles": ["tokens.css", "lib/Panel.svelte", "lib/trello/TrelloModule.svelte",
            "lib/network/NetworkModule.svelte", "lib/system/SystemModule.svelte", "lib/agents/AgentsModule.svelte"] });
        assert_eq!(
            missing_styles(&cold),
            ["lib/ai/AiModule.svelte", "lib/github/GithubModule.svelte"]
        );
    }

    #[test]
    fn rejects_invalid_requests() {
        for invalid in [
            "--capture",
            "--capture relative/b",
            "--capture /tmp/b --size 1100",
            "--capture /tmp/b --size 10x700",
            "--capture /tmp/b --theme blue",
            "--capture /tmp/b --zoom 2",
        ] {
            assert!(matches!(request(&args(invalid)), Some(Err(_))), "{invalid}");
        }
    }
}
