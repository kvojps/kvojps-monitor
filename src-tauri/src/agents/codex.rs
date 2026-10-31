//! Unstable Codex SQLite/rollout formats are confined to this adapter.
use super::{Provider, Session, SessionState};
use crate::ports::Files;
use rusqlite::{Connection, OpenFlags};
use std::{collections::HashSet, path::PathBuf};

/// Read-only local session source; no content is persisted or logged.
pub(super) trait Source: Send {
    fn sessions(&self, tracked: &[String]) -> Result<Vec<(Session, bool)>, ()>;
}
pub(super) struct Adapter<F> {
    root: PathBuf,
    files: F,
}
impl<F: Files> Adapter<F> {
    pub fn new(root: PathBuf, files: F) -> Self {
        Self { root, files }
    }

    #[cfg(test)]
    pub(super) fn test_root(&self) -> PathBuf {
        self.root.clone()
    }

    fn live_rollouts(&self) -> Result<HashSet<PathBuf>, ()> {
        let mut paths = HashSet::new();
        for pid in self.files.list("/proc").map_err(|_| ())? {
            if !pid.bytes().all(|b| b.is_ascii_digit()) {
                continue;
            }
            let Ok(exe) = self.files.read_link(&format!("/proc/{pid}/exe")) else {
                continue;
            };
            if exe.file_name().is_none_or(|name| name != "codex") {
                continue;
            }
            let fd_dir = format!("/proc/{pid}/fd");
            for fd in self.files.list(&fd_dir).unwrap_or_default() {
                if let Ok(path) = self.files.read_link(&format!("{fd_dir}/{fd}")) {
                    paths.insert(path);
                }
            }
        }
        Ok(paths)
    }
}
impl<F: Files> Source for Adapter<F> {
    fn sessions(&self, tracked: &[String]) -> Result<Vec<(Session, bool)>, ()> {
        let path = self.root.join("state_5.sqlite");
        // A missing CLI/database is an empty source, never create a database.
        if !path.try_exists().map_err(|_| ())? {
            return Ok(vec![]);
        }
        let connection = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .map_err(|_| ())?;
        let mut query = connection.prepare("SELECT id, rollout_path, cwd, title, created_at, updated_at, git_branch FROM threads").map_err(|_| ())?;
        let rows = query
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, i64>(4)?,
                    row.get::<_, i64>(5)?,
                    row.get::<_, Option<String>>(6)?,
                ))
            })
            .map_err(|_| ())?;
        let live = self.live_rollouts()?;
        let mut sessions = Vec::new();
        for row in rows {
            let (id, rollout, cwd, title, created, updated, branch) = row.map_err(|_| ())?;
            let running = live.contains(&PathBuf::from(&rollout));
            if !running && !tracked.iter().any(|known| known == &format!("codex:{id}")) {
                continue;
            }
            let mut state = SessionState::Done;
            {
                let content = self.files.read(&rollout).map_err(|_| ())?;
                for line in content.lines() {
                    let Ok(event) = serde_json::from_str::<serde_json::Value>(line) else {
                        continue;
                    };
                    if event["type"] != "event_msg" {
                        continue;
                    }
                    match event["payload"]["type"].as_str() {
                        Some("task_started") => state = SessionState::Working,
                        Some("task_complete" | "turn_aborted") => state = SessionState::Done,
                        Some("error") => state = SessionState::Failed,
                        _ => {}
                    }
                }
            }
            sessions.push((
                Session {
                    id: format!("codex:{id}"),
                    provider: Provider::Codex,
                    project: std::path::Path::new(&cwd)
                        .file_name()
                        .map(|p| p.to_string_lossy().into_owned())
                        .unwrap_or(cwd),
                    name: Some(title),
                    branch,
                    updated_at: Some(u64::try_from(updated).map_err(|_| ())?.saturating_mul(1000)),
                    state,
                    waiting_for: None,
                    started_at: u64::try_from(created).map_err(|_| ())?.saturating_mul(1000),
                    closed_at: None,
                    last_state: None,
                },
                running,
            ));
        }
        Ok(sessions)
    }
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;
    use crate::ports::fakes::FakeRoot;
    pub fn source() -> Adapter<FakeRoot> {
        let files = FakeRoot::from("agents/codex");
        let root = files.dir.path().to_path_buf();
        std::fs::create_dir_all(root.join("proc/123/fd")).unwrap();
        std::os::unix::fs::symlink("/usr/bin/codex", root.join("proc/123/exe")).unwrap();
        std::os::unix::fs::symlink("/rollout.jsonl", root.join("proc/123/fd/4")).unwrap();
        Adapter::new(root, files)
    }
}
