use super::super::{combine, SharedCache};
use super::*;
use crate::persistence::DiskStorage;
use crate::ports::fakes::{fixtures, FakeClock};
use crate::snapshot::Snapshot;
use serde_json::json;
use std::sync::{Arc, Mutex};

/// Shortly after the documented status line payload: both windows still valid.
const NOW: u64 = 1738420000000;
const FIVE_HOUR_RESET: u64 = 1738425600000;

#[derive(Clone)]
struct Installed(Arc<Mutex<bool>>);
impl Programs for Installed {
    fn installed(&self, name: &str) -> bool {
        name == "claude" && *self.0.lock().unwrap()
    }
}

#[derive(Clone, Default)]
struct Changes(Arc<Mutex<bool>>);
impl Changes {
    fn touch(&self) {
        *self.0.lock().unwrap() = true;
    }
}
impl Watch for Changes {
    fn changed(&mut self) -> bool {
        std::mem::take(&mut *self.0.lock().unwrap())
    }
}

/// A home with Claude Code installed, logged in, and another app's status line.
struct Home {
    dir: tempfile::TempDir,
    installed: Installed,
    changes: Changes,
    clock: FakeClock,
}
impl Home {
    fn new() -> Self {
        let home = Home {
            dir: tempfile::tempdir().unwrap(),
            installed: Installed(Arc::new(Mutex::new(true))),
            changes: Changes::default(),
            clock: FakeClock::at(NOW),
        };
        home.copy("claude/settings.json", ".claude/settings.json");
        home.copy("claude/credentials.json", ".claude/.credentials.json");
        home
    }
    fn copy(&self, fixture: &str, path: &str) {
        self.write(
            path,
            &std::fs::read_to_string(fixtures().join(fixture)).unwrap(),
        );
    }
    fn path(&self, path: &str) -> std::path::PathBuf {
        self.dir.path().join(path)
    }
    fn write(&self, path: &str, content: &str) {
        std::fs::create_dir_all(self.path(path).parent().unwrap()).unwrap();
        std::fs::write(self.path(path), content).unwrap();
    }
    fn read(&self, path: &str) -> String {
        std::fs::read_to_string(self.path(path)).unwrap()
    }
    fn settings(&self) -> Value {
        serde_json::from_str(&self.read(".claude/settings.json")).unwrap()
    }
    fn locations(&self) -> Locations {
        Locations {
            settings: ".claude/settings.json".into(),
            credentials: ".claude/.credentials.json".into(),
            config: ".config/kvojps-monitor/claude-statusline.json".into(),
            wrapper: ".local/share/kvojps-monitor/claude-statusline".into(),
            cache: ".cache/kvojps-monitor/claude-statusline.json".into(),
            executable: "/usr/bin/kvojps-monitor".into(),
        }
    }
    fn claude(&self) -> Claude<DiskStorage, Installed, Changes, FakeClock> {
        Claude::new(
            DiskStorage(self.dir.path().into()),
            self.installed.clone(),
            self.changes.clone(),
            self.clock.clone(),
            self.locations(),
        )
    }
    /// What the wrapper's copy step writes after Claude Code runs it.
    fn status_line_runs(&self, payload: &str) {
        let cache = extract(payload, self.clock.now()).unwrap();
        self.write(".cache/kvojps-monitor/claude-statusline.json", &cache);
        self.changes.touch();
    }
}

fn payload() -> String {
    std::fs::read_to_string(fixtures().join("claude/statusline-payload.json")).unwrap()
}

const WRAPPER_COMMAND: &str = "sh '.local/share/kvojps-monitor/claude-statusline'";

#[test]
fn settings_never_change_without_the_click() {
    let home = Home::new();
    let before = home.read(".claude/settings.json");
    let mut claude = home.claude();
    claude.step(false);
    home.clock.advance(600_000);
    claude.step(true);
    assert_eq!(home.read(".claude/settings.json"), before);
    assert_eq!(claude.data()["statusLine"], "off");
}

#[test]
fn connecting_chains_the_existing_status_line_and_disconnecting_restores_it_exactly() {
    let home = Home::new();
    let before = home.read(".claude/settings.json");
    let original = home.settings();
    let mut claude = home.claude();
    claude.connect().unwrap();

    let settings = home.settings();
    assert_eq!(
        settings["statusLine"],
        json!({"type": "command", "command": WRAPPER_COMMAND, "padding": 1})
    );
    // Everything else, including another app's hooks, stays as it was.
    for key in ["hooks", "enabledPlugins", "theme", "autoMode"] {
        assert_eq!(settings[key], original[key], "{key}");
    }
    let wrapper = home.read(".local/share/kvojps-monitor/claude-statusline");
    assert!(
        wrapper.contains(r#"sh -c '/bin/sh "${HOME-}/.orca/agent-hooks/claude-statusline.sh"'"#)
    );
    claude.step(false);
    assert_eq!(claude.data()["statusLine"], "connected");

    claude.disconnect().unwrap();
    assert_eq!(home.read(".claude/settings.json"), before);
    assert!(!home
        .path(".local/share/kvojps-monitor/claude-statusline")
        .exists());
    assert!(!home
        .path(".config/kvojps-monitor/claude-statusline.json")
        .exists());
    claude.step(false);
    assert_eq!(claude.data()["statusLine"], "off");
}

#[test]
fn without_a_previous_status_line_disconnecting_removes_the_key() {
    let home = Home::new();
    let mut settings = home.settings();
    settings.as_object_mut().unwrap().remove("statusLine");
    home.write(
        ".claude/settings.json",
        &(serde_json::to_string_pretty(&settings).unwrap() + "\n"),
    );
    let before = home.read(".claude/settings.json");
    let mut claude = home.claude();
    claude.connect().unwrap();
    assert_eq!(
        home.settings()["statusLine"],
        json!({"type": "command", "command": WRAPPER_COMMAND})
    );
    assert!(home
        .read(".local/share/kvojps-monitor/claude-statusline")
        .ends_with("exit 0\n"));
    claude.disconnect().unwrap();
    assert_eq!(home.read(".claude/settings.json"), before);
}

#[test]
fn connecting_twice_keeps_the_original_status_line() {
    let home = Home::new();
    let before = home.read(".claude/settings.json");
    let mut claude = home.claude();
    claude.connect().unwrap();
    claude.connect().unwrap();
    claude.disconnect().unwrap();
    assert_eq!(home.read(".claude/settings.json"), before);
}

#[test]
fn unreadable_settings_are_never_overwritten() {
    let home = Home::new();
    home.write(".claude/settings.json", "{ invalid");
    let mut claude = home.claude();
    assert_eq!(claude.connect(), Err(ClaudeError::InvalidSettings));
    assert_eq!(home.read(".claude/settings.json"), "{ invalid");
    assert!(!home
        .path(".config/kvojps-monitor/claude-statusline.json")
        .exists());
}

#[test]
fn a_missing_settings_file_gets_only_the_status_line() {
    let home = Home::new();
    std::fs::remove_file(home.path(".claude/settings.json")).unwrap();
    let mut claude = home.claude();
    claude.connect().unwrap();
    assert_eq!(
        home.settings(),
        json!({"statusLine": {"type": "command", "command": WRAPPER_COMMAND}})
    );
}

#[test]
fn a_status_line_replaced_by_another_tool_warns_and_is_never_rewritten() {
    let home = Home::new();
    let mut claude = home.claude();
    claude.connect().unwrap();
    let mut settings = home.settings();
    settings["statusLine"] = json!({"type": "command", "command": "other-tool statusline"});
    let replaced = serde_json::to_string_pretty(&settings).unwrap() + "\n";
    home.write(".claude/settings.json", &replaced);

    home.clock.advance(CHECK_INTERVAL);
    assert!(claude.step(false));
    assert_eq!(claude.data()["statusLine"], "replaced");
    home.clock.advance(600_000);
    claude.step(true);
    assert_eq!(home.read(".claude/settings.json"), replaced);

    // Reconnecting wraps the new status line; disconnecting restores it.
    claude.connect().unwrap();
    assert!(home
        .read(".local/share/kvojps-monitor/claude-statusline")
        .contains("sh -c 'other-tool statusline'"));
    claude.disconnect().unwrap();
    assert_eq!(home.read(".claude/settings.json"), replaced);
}

#[test]
fn disconnecting_after_a_replacement_leaves_the_other_tool_in_place() {
    let home = Home::new();
    let mut claude = home.claude();
    claude.connect().unwrap();
    let mut settings = home.settings();
    settings["statusLine"]["command"] = json!("other-tool statusline");
    let replaced = serde_json::to_string_pretty(&settings).unwrap() + "\n";
    home.write(".claude/settings.json", &replaced);
    claude.disconnect().unwrap();
    assert_eq!(home.read(".claude/settings.json"), replaced);
    claude.step(true);
    assert_eq!(claude.data()["statusLine"], "off");
}

#[test]
fn the_copy_keeps_only_the_five_hour_and_weekly_windows() {
    let cache: Value = serde_json::from_str(&extract(&payload(), NOW).unwrap()).unwrap();
    assert_eq!(
        cache,
        json!({
            "read_at": NOW,
            "rate_limits": {
                "five_hour": {"used_percentage": 23.5, "resets_at": 1738425600},
                "seven_day": {"used_percentage": 41.2, "resets_at": 1738857600},
            },
        })
    );
    // Before the first answer of a session, or without Pro/Max, nothing is copied.
    let mut without: Value = serde_json::from_str(&payload()).unwrap();
    without.as_object_mut().unwrap().remove("rate_limits");
    assert_eq!(extract(&without.to_string(), NOW), None);
    assert_eq!(extract("not json", NOW), None);
}

/// The generated wrapper, run by a real shell as Claude Code would.
#[test]
fn the_wrapper_passes_stdin_to_the_original_command_and_returns_its_output() {
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;
    let home = Home::new();
    // Stands in for the app binary: records its arguments and stdin.
    let copier = home.path("copier");
    std::fs::write(
        &copier,
        "#!/bin/sh\nprintf '%s' \"$1 $2\" > \"$0.args\"\ncat > \"$0.stdin\"\n",
    )
    .unwrap();
    std::fs::set_permissions(&copier, std::fs::Permissions::from_mode(0o755)).unwrap();
    let mut settings = home.settings();
    settings["statusLine"]["command"] = json!("read line; printf 'original: %s' \"$line\"; exit 3");
    home.write(
        ".claude/settings.json",
        &serde_json::to_string_pretty(&settings).unwrap(),
    );
    let mut locations = home.locations();
    locations.executable = copier.to_string_lossy().into_owned();
    let mut claude = Claude::new(
        DiskStorage(home.dir.path().into()),
        home.installed.clone(),
        home.changes.clone(),
        home.clock.clone(),
        locations,
    );
    claude.connect().unwrap();

    let mut child = std::process::Command::new("sh")
        .arg(home.path(".local/share/kvojps-monitor/claude-statusline"))
        .current_dir(home.dir.path())
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"{\"rate_limits\": {}}\n\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "original: {\"rate_limits\": {}}"
    );
    assert_eq!(output.status.code(), Some(3));
    // The copy runs in the background so the status line never waits for it.
    let stdin = home.path("copier.stdin");
    for _ in 0..100 {
        if std::fs::read_to_string(&stdin).is_ok_and(|s| !s.is_empty()) {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    assert_eq!(
        std::fs::read_to_string(stdin).unwrap(),
        "{\"rate_limits\": {}}\n\n"
    );
    assert_eq!(
        home.read("copier.args"),
        "statusline .cache/kvojps-monitor/claude-statusline.json"
    );
}

#[test]
fn a_status_line_update_publishes_both_windows_with_the_plan() {
    let home = Home::new();
    let mut claude = home.claude();
    claude.connect().unwrap();
    claude.step(false);
    assert!(!claude.step(false));
    home.clock.advance(5_000);
    home.status_line_runs(&payload());
    assert!(claude.step(false));
    assert_eq!(
        claude.data(),
        json!({
            "state": "ready",
            "plan": "pro",
            "statusLine": "connected",
            "reading": {
                "source": "statusLine",
                "readAt": NOW + 5_000,
                "buckets": [{
                    "id": "claude",
                    "name": null,
                    "primary": {"usedPercent": 23.5, "durationMins": 300, "resetsAt": FIVE_HOUR_RESET, "reset": false},
                    "secondary": {"usedPercent": 41.2, "durationMins": 10080, "resetsAt": 1738857600000u64, "reset": false},
                }],
            },
        })
    );
    assert_eq!(claude.updated_at(), Some(NOW + 5_000));
}

#[test]
fn without_a_reading_the_module_asks_for_a_claude_code_session() {
    let home = Home::new();
    let mut claude = home.claude();
    claude.connect().unwrap();
    claude.step(false);
    assert_eq!(
        claude.data(),
        json!({"state": "empty", "plan": "pro", "statusLine": "connected", "reading": null})
    );
}

#[test]
fn a_restarted_window_never_assumes_zero_percent() {
    let home = Home::new();
    let mut claude = home.claude();
    home.status_line_runs(&payload());
    claude.step(false);
    home.clock.advance(FIVE_HOUR_RESET - NOW + 60_000);
    let primary = &claude.data()["reading"]["buckets"][0]["primary"];
    assert_eq!(primary["reset"], true);
    assert_eq!(primary["usedPercent"], Value::Null);
    assert_eq!(primary["resetsAt"], FIVE_HOUR_RESET);
}

#[test]
fn a_restart_is_published_without_a_new_reading() {
    let home = Home::new();
    let mut claude = home.claude();
    home.status_line_runs(&payload());
    claude.step(false);
    home.clock.advance(FIVE_HOUR_RESET - NOW - 1_000);
    assert!(!claude.step(false));
    home.clock.advance(2_000);
    assert!(claude.step(false));
    assert!(!claude.step(false));
}

#[test]
fn a_window_dropped_after_its_reset_stays_shown_as_restarted() {
    let home = Home::new();
    let mut claude = home.claude();
    home.status_line_runs(&payload());
    claude.step(false);
    // Claude Code drops a window once its reset passes.
    home.clock.advance(FIVE_HOUR_RESET - NOW + 60_000);
    let mut later: Value = serde_json::from_str(&payload()).unwrap();
    later["rate_limits"]
        .as_object_mut()
        .unwrap()
        .remove("five_hour");
    later["rate_limits"]["seven_day"]["used_percentage"] = json!(45.0);
    home.status_line_runs(&later.to_string());
    claude.step(false);
    let bucket = &claude.data()["reading"]["buckets"][0];
    assert_eq!(bucket["primary"]["reset"], true);
    assert_eq!(bucket["primary"]["usedPercent"], Value::Null);
    assert_eq!(bucket["secondary"]["usedPercent"], 45.0);
}

#[test]
fn an_older_copy_never_replaces_a_newer_reading() {
    let home = Home::new();
    let mut claude = home.claude();
    home.status_line_runs(&payload());
    claude.step(false);
    let older = extract(&payload().replace("23.5", "10.0"), NOW - 60_000).unwrap();
    home.write(".cache/kvojps-monitor/claude-statusline.json", &older);
    home.changes.touch();
    assert!(!claude.step(false));
    assert_eq!(
        claude.data()["reading"]["buckets"][0]["primary"]["usedPercent"],
        23.5
    );
}

#[test]
fn without_the_cli_or_its_credentials_the_module_explains_it_is_missing() {
    let home = Home::new();
    *home.installed.0.lock().unwrap() = false;
    let mut claude = home.claude();
    claude.step(false);
    assert_eq!(claude.data()["state"], "missing");

    let home = Home::new();
    std::fs::remove_file(home.path(".claude/.credentials.json")).unwrap();
    let mut claude = home.claude();
    claude.step(false);
    assert_eq!(claude.data()["state"], "missing");
    assert_eq!(claude.data()["plan"], Value::Null);
}

#[test]
fn the_plan_comes_from_the_subscription_type() {
    let home = Home::new();
    let credentials = home
        .read(".claude/.credentials.json")
        .replace("\"pro\"", "\"max\"");
    home.write(".claude/.credentials.json", &credentials);
    let mut claude = home.claude();
    claude.step(false);
    assert_eq!(claude.data()["plan"], "max");
}

fn disk(dir: &std::path::Path) -> crate::persistence::LocalPersistence<DiskStorage, DiskStorage> {
    use crate::persistence::{Cache, LocalPersistence, Logs};
    LocalPersistence {
        cache: Cache(DiskStorage(dir.join("cache"))),
        logs: Logs(DiskStorage(dir.join("state"))),
    }
}

#[test]
fn restart_shows_the_cached_reading_and_never_the_token() {
    let home = Home::new();
    let state = tempfile::tempdir().unwrap();
    let mut claude = home.claude().with_persistence(disk(state.path()));
    home.status_line_runs(&payload());
    claude.step(false);
    let cached = std::fs::read_to_string(state.path().join("cache/ai.json")).unwrap();
    for forbidden in ["sk-ant", "Token", "token", "/current/working", "my-session"] {
        assert!(!cached.contains(forbidden), "{forbidden}");
    }
    let data = claude.data();
    for forbidden in ["sk-ant", "Token"] {
        assert!(!data.to_string().contains(forbidden), "{forbidden}");
    }
    let log = std::fs::read_to_string(state.path().join("state/info.log")).unwrap();
    assert_eq!(
        serde_json::from_str::<Value>(log.trim()).unwrap(),
        json!({"source": "ai", "event": "updated", "httpStatus": null, "durationMs": 0})
    );

    home.clock.advance(3_600_000);
    let restored = home.claude().with_persistence(disk(state.path()));
    assert_eq!(restored.data()["reading"], data["reading"]);
    assert_eq!(restored.data()["plan"], "pro");
}

#[test]
fn codex_and_claude_share_one_cache_entry() {
    let state = tempfile::tempdir().unwrap();
    let shared = SharedCache::new(disk(state.path()));
    let home = Home::new();
    let mut claude = home.claude().with_persistence(shared.clone());
    home.status_line_runs(&payload());
    claude.step(false);
    shared.save(&Snapshot {
        module: Module::Ai,
        updated_at: Some(NOW - 1),
        data: json!({"refreshing": false, "codex": {"plan": "plus", "reading": null}}),
    });
    let cached = crate::persistence::Persistence::load(&disk(state.path()), Module::Ai).unwrap();
    assert_eq!(cached.data["codex"]["plan"], "plus");
    assert_eq!(cached.data["claude"]["plan"], "pro");
    assert_eq!(cached.updated_at, Some(NOW));
}

/// UI fixtures in src/lib/ai/fixtures/ are snapshots published by the core.
#[test]
fn ui_fixtures_match_published_snapshots() {
    let codex = || {
        let full: Value =
            serde_json::from_str(include_str!("../../../src/lib/ai/fixtures/full.json")).unwrap();
        Snapshot {
            module: Module::Ai,
            updated_at: Some(1793460871000),
            data: full,
        }
    };
    let connected = {
        let home = Home::new();
        let mut claude = home.claude();
        claude.connect().unwrap();
        home.status_line_runs(&payload());
        claude.step(false);
        combine(codex(), &claude)
    };
    let replaced = {
        let home = Home::new();
        let mut claude = home.claude();
        claude.connect().unwrap();
        home.status_line_runs(&payload());
        let mut settings = home.settings();
        settings["statusLine"]["command"] = json!("other-tool statusline");
        home.write(".claude/settings.json", &settings.to_string());
        claude.step(false);
        combine(codex(), &claude)
    };
    let empty = {
        let home = Home::new();
        let mut claude = home.claude();
        claude.connect().unwrap();
        claude.step(false);
        combine(codex(), &claude)
    };
    let off = {
        let home = Home::new();
        let mut claude = home.claude();
        claude.step(false);
        combine(codex(), &claude)
    };
    let missing = {
        let home = Home::new();
        *home.installed.0.lock().unwrap() = false;
        let mut claude = home.claude();
        claude.step(false);
        combine(codex(), &claude)
    };
    for (name, snapshot) in [
        ("claude-missing", missing),
        ("claude-connected", connected),
        ("claude-replaced", replaced),
        ("claude-empty", empty),
        ("claude-off", off),
    ] {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../src/lib/ai/fixtures/{name}.json"));
        if std::env::var_os("UPDATE_FIXTURES").is_some() {
            std::fs::write(
                &path,
                serde_json::to_string_pretty(&snapshot.data).unwrap() + "\n",
            )
            .unwrap();
        }
        let fixture: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        assert_eq!(snapshot.data, fixture, "{name}");
    }
}
