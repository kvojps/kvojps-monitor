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
