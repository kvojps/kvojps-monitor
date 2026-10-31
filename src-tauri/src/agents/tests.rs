use super::*;
use crate::ports::fakes::FakeClock;
use std::sync::{Arc, Mutex};
#[derive(Clone)]
struct Process(Arc<Mutex<Result<String, ()>>>);
impl Processes for Process {
    fn execute(&self, program: &str, args: &[&str]) -> Result<String, ()> {
        assert_eq!(program, "claude");
        assert_eq!(args, ["agents", "--json", "--all"]);
        self.0.lock().unwrap().clone()
    }
}
#[test]
fn recorded_blocked_session_sets_accent_and_polling_runs_every_two_seconds() {
    let process = Process(Arc::new(Mutex::new(Ok(include_str!(
        "../../fixtures/agents/recorded.json"
    )
    .into()))));
    let clock = FakeClock::at(1800000000000);
    let mut monitor = Monitor::new(process, clock.clone());
    let data = monitor.step().unwrap();
    assert!(data.accent_dot);
    assert_eq!(data.sessions[0].project, "example-project");
    assert_eq!(data.sessions[0].state, SessionState::Waiting);
    assert!(monitor.step().is_none());
    clock.advance(2000);
    assert!(monitor.step().is_some());
}

#[test]
fn only_observed_live_sessions_are_retained_and_expire_after_two_hours() {
    let process = Process(Arc::new(Mutex::new(Ok(include_str!(
        "../../fixtures/agents/recorded.json"
    )
    .into()))));
    let clock = FakeClock::at(1800000000000);
    let mut monitor = Monitor::new(process.clone(), clock.clone());
    assert!(monitor.step().unwrap().closed.is_empty());
    *process.0.lock().unwrap() = Err(());
    clock.advance(2000);
    assert_eq!(monitor.step().unwrap().sessions.len(), 1);
    *process.0.lock().unwrap() = Ok("[]".into());
    clock.advance(2000);
    let data = monitor.step().unwrap();
    assert!(data.sessions.is_empty());
    assert!(!data.accent_dot);
    assert_eq!(data.closed.len(), 1);
    clock.advance(7_200_000);
    assert!(monitor.step().unwrap().closed.is_empty());
}

#[test]
fn documented_states_and_terminal_history_are_reconciled_without_content_leaks() {
    let process = Process(Arc::new(Mutex::new(Ok("[]".into()))));
    let clock = FakeClock::at(1800000000000);
    let mut monitor = Monitor::new(process.clone(), clock.clone());
    for (field, value, expected) in [
        ("status", "busy", SessionState::Working),
        ("status", "waiting", SessionState::Waiting),
        ("status", "idle", SessionState::Done),
        ("state", "working", SessionState::Working),
        ("state", "blocked", SessionState::Waiting),
        ("state", "done", SessionState::Done),
        ("state", "failed", SessionState::Failed),
        ("state", "stopped", SessionState::Done),
    ] {
        // Variants of the recorded schema for states not currently running locally.
        let mut row = serde_json::json!({"pid":123,"sessionId":"test","cwd":"/tmp/project","startedAt":1000,
            "prompt":"PRIVATE_PROMPT", "response":"PRIVATE_RESPONSE", "token":"PRIVATE_TOKEN"});
        row[field] = value.into();
        *process.0.lock().unwrap() = Ok(serde_json::json!([row]).to_string());
        clock.advance(2000);
        let data = monitor.step().unwrap();
        assert_eq!(data.sessions[0].state, expected);
        assert!(!serde_json::to_string(&data).unwrap().contains("PRIVATE_"));
    }
    *process.0.lock().unwrap() =
        Ok(r#"[{"id":"old","cwd":"/tmp/old","startedAt":1,"state":"done"}]"#.into());
    clock.advance(2000);
    let data = monitor.step().unwrap();
    assert!(data.sessions.is_empty());
    assert_eq!(data.closed.len(), 1);
    assert_eq!(data.closed[0].id, "test");
}

#[test]
fn closed_history_is_bounded_and_terminal_jobs_seen_at_startup_are_ignored() {
    let process = Process(Arc::new(Mutex::new(Ok(
        r#"[{"id":"old","cwd":"/tmp","startedAt":1,"state":"failed"}]"#.into(),
    ))));
    let clock = FakeClock::at(1800000000000);
    let mut monitor = Monitor::new(process.clone(), clock.clone());
    let data = monitor.step().unwrap();
    assert!(data.sessions.is_empty());
    assert!(data.closed.is_empty());
    let rows: Vec<_> = (0..7).map(|id| serde_json::json!({"id":id.to_string(),"cwd":"/tmp","startedAt":1,"status":"busy","pid":id+100})).collect();
    *process.0.lock().unwrap() = Ok(serde_json::to_string(&rows).unwrap());
    clock.advance(2000);
    assert_eq!(monitor.step().unwrap().sessions.len(), 7);
    *process.0.lock().unwrap() = Ok("not json".into());
    clock.advance(2000);
    assert_eq!(monitor.step().unwrap().sessions.len(), 7);
    *process.0.lock().unwrap() = Ok("[]".into());
    clock.advance(2000);
    assert_eq!(monitor.step().unwrap().closed.len(), 5);
    let mut restarted = Monitor::new(process, clock);
    assert!(restarted.step().unwrap().closed.is_empty());
}

#[test]
fn background_failure_keeps_its_outcome_in_closed_history() {
    let process = Process(Arc::new(Mutex::new(Ok(include_str!(
        "../../fixtures/agents/recorded.json"
    )
    .into()))));
    let clock = FakeClock::at(1800000000000);
    let mut monitor = Monitor::new(process.clone(), clock.clone());
    monitor.step().unwrap();
    let mut records: serde_json::Value =
        serde_json::from_str(include_str!("../../fixtures/agents/recorded.json")).unwrap();
    records[0]["state"] = "failed".into();
    *process.0.lock().unwrap() = Ok(records.to_string());
    clock.advance(2000);
    let data = monitor.step().unwrap();
    assert!(data.sessions.is_empty());
    assert_eq!(data.closed[0].state, SessionState::Closed);
    assert_eq!(data.closed[0].last_state, Some(SessionState::Failed));
}

#[test]
fn codex_sessions_share_history_and_keep_provider_identity() {
    let process = Process(Arc::new(Mutex::new(Ok("[]".into()))));
    let clock = FakeClock::at(1800000000000);
    let source = codex::tests::source();
    let mut monitor = Monitor::new(process, clock.clone()).with_codex(source);
    let data = monitor.step().unwrap();
    assert_eq!(data.sessions.len(), 1);
    let json = serde_json::to_value(&data).unwrap();
    assert_eq!(json["sessions"][0]["provider"], "codex");
    assert_eq!(json["sessions"][0]["state"], "working");
    assert_eq!(json["codexWaitingUnavailable"], true);
}

#[test]
fn codex_rollout_outcomes_process_exit_and_read_failures_are_reconciled() {
    let source = codex::tests::source();
    let root = source.test_root();
    let process = Process(Arc::new(Mutex::new(Ok(include_str!(
        "../../fixtures/agents/recorded.json"
    )
    .into()))));
    let clock = FakeClock::at(1800000000000);
    let mut monitor = Monitor::new(process.clone(), clock.clone()).with_codex(source);
    assert_eq!(monitor.step().unwrap().sessions.len(), 2);
    for (event, expected) in [
        ("task_complete", SessionState::Done),
        ("task_started", SessionState::Working),
        ("error", SessionState::Failed),
        ("turn_aborted", SessionState::Done),
    ] {
        // Recorded successful/interrupted turns; error is an explicit schema variant.
        let recorded = match event {
            "task_complete" => include_str!("../../fixtures/agents/codex/task_complete.jsonl"),
            "turn_aborted" => include_str!("../../fixtures/agents/codex/turn_aborted.jsonl"),
            "task_started" => include_str!("../../fixtures/agents/codex/rollout.jsonl"),
            _ => {
                r#"{"type":"event_msg","payload":{"type":"error","message":"PRIVATE_RESPONSE","token":"PRIVATE_TOKEN"}}"#
            }
        };
        std::fs::write(root.join("rollout.jsonl"), recorded).unwrap();
        clock.advance(2000);
        let data = monitor.step().unwrap();
        let session = data
            .sessions
            .iter()
            .find(|s| s.provider == Provider::Codex)
            .unwrap();
        assert_eq!(session.state, expected);
        assert!(data.accent_dot);
        assert!(!serde_json::to_string(&data).unwrap().contains("PRIVATE_"));
    }
    std::fs::write(root.join("state_5.sqlite"), "invalid sqlite PRIVATE_TOKEN").unwrap();
    clock.advance(2000);
    let data = monitor.step().unwrap();
    assert!(data.failed);
    assert_eq!(data.sessions.len(), 2);
}

#[test]
fn codex_closed_history_only_contains_previously_live_sessions() {
    let source = codex::tests::source();
    let root = source.test_root();
    let process = Process(Arc::new(Mutex::new(Ok("[]".into()))));
    let clock = FakeClock::at(1800000000000);
    let mut monitor = Monitor::new(process.clone(), clock.clone()).with_codex(source);
    monitor.step().unwrap();
    std::fs::remove_file(root.join("proc/123/fd/4")).unwrap();
    clock.advance(2000);
    let data = monitor.step().unwrap();
    assert!(data.sessions.is_empty());
    assert_eq!(data.closed.len(), 1);
    assert_eq!(data.closed[0].last_state, Some(SessionState::Working));
    clock.advance(7_200_000);
    assert!(monitor.step().unwrap().closed.is_empty());
    let mut restarted = Monitor::new(process, clock).with_codex(codex::Adapter::new(
        root,
        crate::ports::RootFilesystem::at("/nonexistent"),
    ));
    assert!(restarted.step().unwrap().closed.is_empty());
}

#[test]
fn unavailable_codex_rollout_retains_last_reading_and_database_is_read_only() {
    let source = codex::tests::source();
    let root = source.test_root();
    let before = std::fs::read(root.join("state_5.sqlite")).unwrap();
    let process = Process(Arc::new(Mutex::new(Err(()))));
    let clock = FakeClock::at(1800000000000);
    let mut monitor = Monitor::new(process, clock.clone()).with_codex(source);
    let data = monitor.step().unwrap();
    assert!(data.failed); // Claude failure must not hide Codex.
    assert_eq!(data.sessions[0].state, SessionState::Working);
    assert_eq!(std::fs::read(root.join("state_5.sqlite")).unwrap(), before);
    assert!(!root.join("state_5.sqlite-journal").exists());
    std::fs::remove_file(root.join("rollout.jsonl")).unwrap();
    clock.advance(2000);
    let data = monitor.step().unwrap();
    assert!(data.failed);
    assert_eq!(data.sessions[0].state, SessionState::Working);
}

#[test]
fn injected_sqlite_and_proc_failures_never_close_a_live_codex_session() {
    use crate::ports::{Files, Space};
    use std::{io, path::PathBuf};
    struct UnavailableFiles {
        inner: crate::ports::fakes::FakeRoot,
        unavailable: Arc<Mutex<Option<&'static str>>>,
    }
    impl Files for UnavailableFiles {
        fn read(&self, path: &str) -> io::Result<String> {
            self.inner.read(path)
        }
        fn list(&self, path: &str) -> io::Result<Vec<String>> {
            if *self.unavailable.lock().unwrap() == Some(path) {
                return Err(io::ErrorKind::PermissionDenied.into());
            }
            self.inner.list(path)
        }
        fn space(&self, path: &str) -> io::Result<Space> {
            self.inner.space(path)
        }
        fn read_link(&self, path: &str) -> io::Result<PathBuf> {
            if *self.unavailable.lock().unwrap() == Some(path) {
                return Err(io::ErrorKind::PermissionDenied.into());
            }
            self.inner.read_link(path)
        }
        fn read_sqlite(&self, path: &str, query: &str) -> io::Result<Vec<serde_json::Value>> {
            if *self.unavailable.lock().unwrap() == Some(path) {
                return Err(io::ErrorKind::PermissionDenied.into());
            }
            self.inner.read_sqlite(path, query)
        }
    }
    let source = codex::tests::source();
    let files = source.test_files();
    files.write("/proc/456/status", "Uid:\t1000\t1000\t1000\t1000\n");
    files.write("/proc/self/status", "Uid:\t1000\t1000\t1000\t1000\n");
    files.write("/proc/456/comm", "systemd\n");
    let unavailable = Arc::new(Mutex::new(None));
    let source = codex::Adapter::new(
        "/".into(),
        UnavailableFiles {
            inner: files,
            unavailable: unavailable.clone(),
        },
    );
    let clock = FakeClock::at(1800000000000);
    let process = Process(Arc::new(Mutex::new(Ok("[]".into()))));
    let mut monitor = Monitor::new(process, clock.clone()).with_codex(source);
    assert_eq!(
        monitor.step().unwrap().sessions[0].state,
        SessionState::Working
    );
    *unavailable.lock().unwrap() = Some("/proc/456/exe");
    clock.advance(2000);
    let data = monitor.step().unwrap();
    assert!(
        !data.failed,
        "An unrelated inaccessible process must not hide Codex sessions"
    );
    assert_eq!(data.sessions[0].state, SessionState::Working);
    for path in ["/state_5.sqlite", "/proc/123/fd", "/proc/123/exe"] {
        *unavailable.lock().unwrap() = Some(path);
        clock.advance(2000);
        let data = monitor.step().unwrap();
        assert!(data.failed);
        assert_eq!(data.sessions[0].state, SessionState::Working);
        assert!(data.closed.is_empty());
    }
}
