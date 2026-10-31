//! Daily check for a newer release on GitHub. No notification and no auto-update:
//! Settings shows a discreet line linking to the release page.
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ports::{Clock, HttpError};
use crate::state::AppState;

const DAY_MS: u64 = 24 * 60 * 60 * 1000;
const RETRY_MS: u64 = 60 * 60 * 1000;
pub const RELEASES: &str = "https://github.com/Kvojps/kvojps-monitor/releases/";

/// A newer published release; public metadata only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Release {
    pub version: String,
    pub url: String,
}

/// `GET /repos/Kvojps/kvojps-monitor/releases/latest`, without credentials.
pub trait ReleaseFeed: Send {
    fn latest(&self) -> Result<Value, HttpError>;
}

pub struct GithubReleases;
impl ReleaseFeed for GithubReleases {
    fn latest(&self) -> Result<Value, HttpError> {
        let response = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|_| HttpError::default())?
            .get("https://api.github.com/repos/Kvojps/kvojps-monitor/releases/latest")
            .header("User-Agent", "kvojps-monitor")
            .header("Accept", "application/vnd.github+json")
            .send()
            .map_err(|_| HttpError::default())?;
        let status = response.status().as_u16();
        if !response.status().is_success() {
            return Err(HttpError::response(status, None));
        }
        response.json().map_err(|_| HttpError {
            status: Some(status),
            ..HttpError::default()
        })
    }
}

/// Numeric `major.minor.patch`, with an optional `v`; pre-releases are ignored.
fn version(text: &str) -> Option<(u64, u64, u64)> {
    let mut parts = text.trim().trim_start_matches('v').split('.');
    let mut next = || parts.next()?.parse::<u64>().ok();
    let parsed = (next()?, next()?, next()?);
    parts.next().is_none().then_some(parsed)
}

/// Whether `candidate` is newer than `current`.
pub fn is_newer(candidate: &str, current: &str) -> bool {
    matches!((version(candidate), version(current)), (Some(a), Some(b)) if a > b)
}

/// A release of this app newer than `current`, linking only to its release page.
fn newer(current: &str, body: &Value) -> Option<Release> {
    let tag = body["tag_name"].as_str()?;
    let url = body["html_url"].as_str()?;
    if body["draft"].as_bool() == Some(true) || body["prerelease"].as_bool() == Some(true) {
        return None;
    }
    if !url.starts_with(RELEASES) || !is_newer(tag, current) {
        return None;
    }
    Some(Release {
        version: tag.trim_start_matches('v').to_string(),
        url: url.to_string(),
    })
}

pub struct Checker<F, C> {
    feed: F,
    clock: C,
    current: String,
    retry_at: u64,
}

impl<F: ReleaseFeed, C: Clock> Checker<F, C> {
    pub fn new(feed: F, clock: C, current: &str) -> Self {
        Self {
            feed,
            clock,
            current: current.to_string(),
            retry_at: 0,
        }
    }

    /// Check once a day while enabled and online; a failure retries an hour later.
    pub fn step(&mut self, state: &Mutex<AppState>, online: bool) {
        let now = self.clock.now();
        {
            let mut state = state.lock().unwrap();
            state.keep_release_if(|release| is_newer(&release.version, &self.current));
            if !online || !state.version_check() || now < self.retry_at {
                return;
            }
            if state
                .version_checked_at()
                .is_some_and(|at| now < at.saturating_add(DAY_MS))
            {
                return;
            }
        }
        // The request runs without holding the state lock.
        let found = match self.feed.latest() {
            Ok(body) => newer(&self.current, &body),
            // No published release yet.
            Err(HttpError {
                status: Some(404), ..
            }) => None,
            Err(_) => {
                self.retry_at = now.saturating_add(RETRY_MS);
                return;
            }
        };
        state.lock().unwrap().record_version_check(now, found);
    }
}

/// Check in the background, after the login sequence has started the remote sources.
pub fn start(app: &tauri::AppHandle) {
    use tauri::Manager;
    let app = app.clone();
    std::thread::spawn(move || {
        let mut checker = Checker::new(
            GithubReleases,
            crate::ports::SystemClock,
            env!("CARGO_PKG_VERSION"),
        );
        loop {
            let online = app
                .try_state::<crate::lifecycle::Shared>()
                .map(|s| *s.lock().unwrap())
                .is_some_and(|status| {
                    status.online && crate::ports::SystemClock.now() >= status.ready_for(6)
                });
            checker.step(&app.state::<Mutex<AppState>>(), online);
            std::thread::sleep(std::time::Duration::from_secs(60));
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::fakes::FakeClock;
    use crate::ports::HttpError;
    use crate::state::AppState;
    use serde_json::Value;
    use std::sync::{Arc, Mutex};

    const DAY: u64 = 24 * 60 * 60 * 1000;
    const HOUR: u64 = 60 * 60 * 1000;

    fn fixture(name: &str) -> Value {
        let path = crate::ports::fakes::fixtures().join(format!("releases/{name}.json"));
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }

    /// Answers with a recorded response and counts requests.
    #[derive(Clone)]
    struct Feed {
        answer: Arc<Mutex<Result<Value, HttpError>>>,
        calls: Arc<Mutex<u32>>,
    }
    impl Feed {
        fn new(answer: Result<Value, HttpError>) -> Self {
            Self {
                answer: Arc::new(Mutex::new(answer)),
                calls: Arc::default(),
            }
        }
        fn answer(&self, answer: Result<Value, HttpError>) {
            *self.answer.lock().unwrap() = answer;
        }
        fn calls(&self) -> u32 {
            *self.calls.lock().unwrap()
        }
    }
    impl ReleaseFeed for Feed {
        fn latest(&self) -> Result<Value, HttpError> {
            *self.calls.lock().unwrap() += 1;
            self.answer.lock().unwrap().clone()
        }
    }

    fn not_found() -> Result<Value, HttpError> {
        Err(HttpError::response(404, None))
    }

    fn setup(
        answer: Result<Value, HttpError>,
    ) -> (tempfile::TempDir, Mutex<AppState>, Feed, FakeClock) {
        let d = tempfile::tempdir().unwrap();
        let state = Mutex::new(AppState::load(d.path().into()));
        (d, state, Feed::new(answer), FakeClock::at(1_000 * DAY))
    }

    #[test]
    fn a_newer_release_is_found_and_checked_again_only_after_a_day() {
        let (_d, state, feed, clock) = setup(Ok(fixture("newer")));
        let mut checker = Checker::new(feed.clone(), clock.clone(), "0.1.0");

        checker.step(&state, true);
        assert_eq!(
            state.lock().unwrap().available_release(),
            Some(Release {
                version: "0.2.0".into(),
                url: "https://github.com/Kvojps/kvojps-monitor/releases/tag/v0.2.0".into()
            })
        );

        clock.advance(23 * HOUR);
        checker.step(&state, true);
        assert_eq!(feed.calls(), 1);

        clock.advance(HOUR);
        checker.step(&state, true);
        assert_eq!(feed.calls(), 2);
    }

    #[test]
    fn a_repository_without_releases_has_no_update() {
        let (_d, state, feed, clock) = setup(not_found());
        let mut checker = Checker::new(feed.clone(), clock.clone(), "0.1.0");

        checker.step(&state, true);
        clock.advance(HOUR);
        checker.step(&state, true);

        assert_eq!(state.lock().unwrap().available_release(), None);
        // 404 is an answer, not a failure: the next check is a day later.
        assert_eq!(feed.calls(), 1);
    }

    #[test]
    fn the_current_version_or_a_foreign_link_is_not_an_update() {
        let (_d, state, feed, clock) = setup(Ok(fixture("newer")));
        Checker::new(feed.clone(), clock.clone(), "0.2.0").step(&state, true);
        assert_eq!(state.lock().unwrap().available_release(), None);

        let (_d, state, feed, clock) = setup(Ok(fixture("recorded")));
        Checker::new(feed, clock, "0.1.0").step(&state, true);
        assert_eq!(state.lock().unwrap().available_release(), None);
    }

    #[test]
    fn disabled_or_offline_never_asks_github() {
        let (_d, state, feed, clock) = setup(Ok(fixture("newer")));
        let mut checker = Checker::new(feed.clone(), clock.clone(), "0.1.0");

        checker.step(&state, false);
        state.lock().unwrap().set_version_check(false);
        checker.step(&state, true);

        assert_eq!(feed.calls(), 0);
        assert_eq!(state.lock().unwrap().available_release(), None);
    }

    #[test]
    fn disabling_hides_a_found_update() {
        let (_d, state, feed, clock) = setup(Ok(fixture("newer")));
        Checker::new(feed, clock, "0.1.0").step(&state, true);

        state.lock().unwrap().set_version_check(false);

        assert_eq!(state.lock().unwrap().available_release(), None);
    }

    #[test]
    fn a_failure_keeps_the_known_update_and_retries_after_an_hour() {
        let (_d, state, feed, clock) = setup(Ok(fixture("newer")));
        let mut checker = Checker::new(feed.clone(), clock.clone(), "0.1.0");
        checker.step(&state, true);
        clock.advance(DAY);
        feed.answer(Err(HttpError::response(503, None)));

        checker.step(&state, true);
        clock.advance(HOUR - 1);
        checker.step(&state, true);
        assert_eq!(feed.calls(), 2);
        assert!(state.lock().unwrap().available_release().is_some());

        clock.advance(1);
        checker.step(&state, true);
        assert_eq!(feed.calls(), 3);
    }

    #[test]
    fn a_restart_within_the_day_does_not_check_again_and_keeps_the_update() {
        let (d, state, feed, clock) = setup(Ok(fixture("newer")));
        Checker::new(feed.clone(), clock.clone(), "0.1.0").step(&state, true);

        let restarted = Mutex::new(AppState::load(d.path().into()));
        clock.advance(HOUR);
        Checker::new(feed.clone(), clock, "0.1.0").step(&restarted, true);

        assert_eq!(feed.calls(), 1);
        assert!(restarted.lock().unwrap().available_release().is_some());
    }

    #[test]
    fn installing_the_announced_version_hides_the_line() {
        let (d, state, feed, clock) = setup(Ok(fixture("newer")));
        Checker::new(feed.clone(), clock.clone(), "0.1.0").step(&state, true);

        let upgraded = Mutex::new(AppState::load(d.path().into()));
        Checker::new(feed, clock, "0.2.0").step(&upgraded, true);

        assert_eq!(upgraded.lock().unwrap().available_release(), None);
    }
}
