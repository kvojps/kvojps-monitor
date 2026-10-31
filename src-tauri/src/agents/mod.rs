//! Local Claude and Codex session polling. Session metadata stays in memory only.
mod codex;
use crate::ports::{Clock, Processes};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SessionState {
    Working,
    Waiting,
    Done,
    Failed,
    Closed,
}
#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
enum Provider {
    Claude,
    Codex,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    id: String,
    provider: Provider,
    branch: Option<String>,
    updated_at: Option<u64>,
    project: String,
    name: Option<String>,
    state: SessionState,
    waiting_for: Option<String>,
    started_at: u64,
    closed_at: Option<u64>,
    last_state: Option<SessionState>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    sessions: Vec<Session>,
    closed: Vec<Session>,
    accent_dot: bool,
    failed: bool,
    observed_at: u64,
    codex_waiting_unavailable: bool,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Record {
    id: Option<String>,
    session_id: Option<String>,
    pid: Option<u64>,
    cwd: String,
    name: Option<String>,
    started_at: u64,
    status: Option<String>,
    state: Option<String>,
    waiting_for: Option<String>,
}
impl Record {
    fn session(self) -> Result<(Session, bool), ()> {
        let state = match self.status.as_deref().or(self.state.as_deref()) {
            Some("busy" | "working") => SessionState::Working,
            Some("waiting" | "blocked") => SessionState::Waiting,
            Some("idle" | "done" | "stopped") => SessionState::Done,
            Some("failed") => SessionState::Failed,
            _ => return Err(()),
        };
        let terminal = self.pid.is_none()
            && matches!(self.state.as_deref(), Some("done" | "failed" | "stopped"));
        let id = self
            .session_id
            .or(self.id)
            .or_else(|| self.pid.map(|pid| format!("{pid}:{}", self.started_at)))
            .ok_or(())?;
        Ok((
            Session {
                id,
                provider: Provider::Claude,
                branch: None,
                updated_at: None,
                project: std::path::Path::new(&self.cwd)
                    .file_name()
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or(self.cwd),
                name: self.name,
                state,
                waiting_for: self.waiting_for,
                started_at: self.started_at,
                closed_at: None,
                last_state: None,
            },
            !terminal,
        ))
    }
}
pub struct Monitor<P, C> {
    codex: Option<Box<dyn codex::Source>>,
    process: P,
    clock: C,
    next: Option<u64>,
    sessions: Vec<Session>,
    closed: Vec<Session>,
}
impl<P: Processes, C: Clock> Monitor<P, C> {
    pub fn new(process: P, clock: C) -> Self {
        Self {
            codex: None,
            process,
            clock,
            next: None,
            sessions: vec![],
            closed: vec![],
        }
    }
    fn with_codex(mut self, source: impl codex::Source + 'static) -> Self {
        self.codex = Some(Box::new(source));
        self
    }
    pub fn step(&mut self) -> Option<Data> {
        let now = self.clock.now();
        if self.next.is_some_and(|next| now < next) {
            return None;
        }
        self.next = Some(now.saturating_add(2000));
        let result = self
            .process
            .execute("claude", &["agents", "--json", "--all"])
            .and_then(|json| serde_json::from_str::<Vec<Record>>(&json).map_err(|_| ()))
            .and_then(|records| {
                records
                    .into_iter()
                    .map(Record::session)
                    .collect::<Result<Vec<_>, _>>()
            });
        let mut failed = result.is_err();
        let mut sessions = result.unwrap_or_else(|_| {
            self.sessions
                .iter()
                .filter(|s| s.provider == Provider::Claude)
                .cloned()
                .map(|s| (s, true))
                .collect()
        });
        if let Some(source) = &self.codex {
            match source.sessions(
                &self
                    .sessions
                    .iter()
                    .filter(|s| s.provider == Provider::Codex)
                    .map(|s| s.id.clone())
                    .collect::<Vec<_>>(),
            ) {
                Ok(records) => sessions.extend(records),
                Err(()) => {
                    failed = true;
                    sessions.extend(
                        self.sessions
                            .iter()
                            .filter(|s| s.provider == Provider::Codex)
                            .cloned()
                            .map(|s| (s, true)),
                    );
                }
            }
        }
        {
            let outcomes: std::collections::HashMap<_, _> = sessions
                .iter()
                .map(|(s, _)| (s.id.clone(), s.state))
                .collect();
            // Terminal background jobs without a live process are historical.
            let mut live = vec![];
            for (session, live_process) in sessions.drain(..) {
                if live_process {
                    live.push(session);
                }
            }
            for previous in &self.sessions {
                if !live.iter().any(|s| s.id == previous.id) {
                    let mut closed = previous.clone();
                    closed.last_state = Some(
                        outcomes
                            .get(&previous.id)
                            .copied()
                            .unwrap_or(previous.state),
                    );
                    closed.state = SessionState::Closed;
                    closed.closed_at = Some(now);
                    closed.waiting_for = None;
                    self.closed.push(closed);
                }
            }
            self.closed
                .retain(|s| !live.iter().any(|active| active.id == s.id));
            self.sessions = live;
        }
        self.closed
            .retain(|s| now.saturating_sub(s.closed_at.unwrap()) < 7_200_000);
        self.closed.sort_by_key(|s| std::cmp::Reverse(s.closed_at));
        self.closed.truncate(5);
        self.sessions
            .sort_by_key(|s| (s.state != SessionState::Waiting, s.started_at, s.id.clone()));
        Some(Data {
            accent_dot: self
                .sessions
                .iter()
                .any(|s| s.state == SessionState::Waiting),
            sessions: self.sessions.clone(),
            closed: self.closed.clone(),
            failed,
            observed_at: now,
            codex_waiting_unavailable: true,
        })
    }
}
pub fn start(app: tauri::AppHandle) {
    std::thread::spawn(move || {
        let mut monitor = Monitor::new(crate::ports::SystemProcesses, crate::ports::SystemClock)
            .with_codex(codex::Adapter::new(
                dirs::home_dir().unwrap_or_default().join(".codex"),
                crate::ports::RootFilesystem::system(),
            ));
        loop {
            if let Some(data) = monitor.step() {
                if let Some(handle) =
                    tauri::Manager::try_state::<ksni::Handle<crate::indicator::Indicator>>(&app)
                {
                    let dot = data.accent_dot;
                    tauri::async_runtime::block_on(
                        handle.update(move |i| i.state.accent_dot = dot),
                    );
                }
                crate::snapshot::publish(
                    &app,
                    crate::snapshot::Snapshot {
                        module: crate::module::Module::Agents,
                        updated_at: Some(data.observed_at),
                        data: serde_json::to_value(data).unwrap(),
                    },
                );
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}
#[cfg(test)]
mod tests;
