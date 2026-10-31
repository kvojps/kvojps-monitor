use crate::module::Module;
use crate::ports::{Clock, Http, Processes};
use crate::snapshot::Snapshot;
use chrono::{DateTime, Datelike, Utc};
use serde_json::{json, Value};

pub struct Github<P, H, R> {
    processes: P,
    http: H,
    clock: R,
    calendar_at: Option<u64>,
    repo_at: Option<u64>,
    manual_at: Option<u64>,
    attempt_at: Option<u64>,
    data: Value,
}
impl<P: Processes, H: Http, R: Clock> Github<P, H, R> {
    pub fn new(processes: P, http: H, clock: R) -> Self {
        Self {
            processes,
            http,
            clock,
            calendar_at: None,
            repo_at: None,
            manual_at: None,
            attempt_at: None,
            data: json!({"state":"missingGh", "calendar":null, "featured":null, "refreshing":false}),
        }
    }
    pub fn snapshot(&self, refreshing: bool) -> Snapshot {
        let mut data = self.data.clone();
        data["refreshing"] = json!(refreshing);
        Snapshot {
            module: Module::Github,
            updated_at: self.repo_at,
            data,
        }
    }
    pub fn due(&self, visible: bool) -> bool {
        let now = self.clock.now();
        self.calendar_at
            .is_none_or(|t| now.saturating_sub(t) >= 3_600_000 || year(t) != year(now))
            || self
                .repo_at
                .is_none_or(|t| now.saturating_sub(t) >= if visible { 300_000 } else { 900_000 })
    }
    pub fn manual(&mut self) -> bool {
        let now = self.clock.now();
        if self
            .manual_at
            .is_some_and(|t| now.saturating_sub(t) < 10_000)
        {
            return false;
        }
        self.manual_at = Some(now);
        true
    }
    pub fn should_collect(&self, visible: bool, manual: bool) -> bool {
        let now = self.clock.now();
        let manual_allowed = manual
            && self
                .manual_at
                .is_none_or(|t| now.saturating_sub(t) >= 10_000);
        manual_allowed
            || (self.due(visible)
                && (self.data["state"] == "ready"
                    || self
                        .attempt_at
                        .is_none_or(|t| now.saturating_sub(t) >= 30_000)))
    }
    /// Use the same clock for collection decisions, windows and calendar updates.
    pub fn step(&mut self, visible: bool, manual: bool) -> Option<Snapshot> {
        if !self.should_collect(visible, manual) {
            return None;
        }
        let force = manual && self.manual();
        self.refresh(force)
    }
    pub fn refresh(&mut self, force: bool) -> Option<Snapshot> {
        let now = self.clock.now();
        self.attempt_at = Some(now);
        let calendar = force
            || self
                .calendar_at
                .is_none_or(|t| now.saturating_sub(t) >= 3_600_000 || year(t) != year(now));
        // Never store the token in the struct or snapshot.
        let Ok(token) = self.processes.execute("gh", &["auth", "token"]) else {
            self.data["state"] = json!("missingGh");
            return Some(self.snapshot(false));
        };
        let result = self.query(token.trim(), now, calendar);
        match result {
            Ok(()) => {
                self.repo_at = Some(now);
                self.data["state"] = json!("ready");
            }
            Err(()) => self.data["state"] = json!("error"),
        }
        Some(self.snapshot(false))
    }
    fn query(&mut self, token: &str, now: u64, calendar: bool) -> Result<(), ()> {
        let year = year(now);
        let field = if calendar {
            "contributionsCollection(from:$from,to:$to) { restrictedContributionsCount contributionCalendar { totalContributions weeks { contributionDays { date contributionCount color } } } }"
        } else {
            ""
        };
        let declaration = if calendar {
            "($from:DateTime!,$to:DateTime!)"
        } else {
            ""
        };
        let query = format!("query{declaration} {{ viewer {{ id login {field} repositories(first:20, affiliations:[OWNER,COLLABORATOR,ORGANIZATION_MEMBER], orderBy:{{field:PUSHED_AT,direction:DESC}}) {{ nodes {{ nameWithOwner }} }} }} }}");
        let response = self.http.graphql(token, &json!({"query":query,"variables":{"from":format!("{year}-01-01T00:00:00Z"),"to":format!("{year}-12-31T23:59:59Z")}}))?;
        if response.get("errors").is_some() {
            return Err(());
        }
        let viewer = &response["data"]["viewer"];
        let id = viewer["id"].as_str().ok_or(())?;
        let mut featured: Option<Value> = None;
        let since = timestamp(now.saturating_sub(7 * 24 * 3_600_000));
        let until = timestamp(now);
        for repo in viewer["repositories"]["nodes"].as_array().ok_or(())? {
            let full_name = repo["nameWithOwner"].as_str().ok_or(())?;
            let (owner, name) = full_name.split_once('/').ok_or(())?;
            let response = self.http.graphql(token, &json!({"query": "query($owner:String!,$name:String!,$id:ID!,$since:GitTimestamp!,$until:GitTimestamp!) { repository(owner:$owner,name:$name) { defaultBranchRef { target { ... on Commit { window:history(first:1,author:{id:$id},since:$since,until:$until) { totalCount nodes { committedDate messageHeadline } } total:history(author:{id:$id}) { totalCount } } } } } }", "variables":{"owner":owner,"name":name,"id":id,"since":since,"until":until}}))?;
            if response.get("errors").is_some() {
                return Err(());
            }
            let commit = &response["data"]["repository"]["defaultBranchRef"]["target"];
            if commit.is_null() {
                continue;
            }
            let n = commit["window"]["totalCount"].as_u64().ok_or(())?;
            if n == 0 {
                continue;
            }
            let last = &commit["window"]["nodes"][0];
            let data = last["committedDate"].as_str().ok_or(())?;
            let ms = DateTime::parse_from_rfc3339(data)
                .map_err(|_| ())?
                .timestamp_millis();
            let candidate = json!({"name":full_name,"commits":n,"total":commit["total"]["totalCount"],"title":last["messageHeadline"],"when":ms});
            if featured.as_ref().is_none_or(|d| {
                (n, ms)
                    > (
                        d["commits"].as_u64().unwrap_or(0),
                        d["when"].as_i64().unwrap_or(0),
                    )
            }) {
                featured = Some(candidate);
            }
        }
        if calendar {
            let collection = &viewer["contributionsCollection"];
            if !collection["contributionCalendar"].is_object() {
                return Err(());
            }
            // totalContributions includes private contributions; restricted is the anonymous subset, so do not add it again.
            self.data["calendar"] = collection.clone();
            self.calendar_at = Some(now);
        }
        self.data["login"] = viewer["login"].clone();
        self.data["featured"] = json!(featured);
        Ok(())
    }
}
fn year(ms: u64) -> i32 {
    DateTime::<Utc>::from_timestamp_millis(ms as i64)
        .unwrap()
        .year()
}
fn timestamp(ms: u64) -> String {
    DateTime::<Utc>::from_timestamp_millis(ms as i64)
        .unwrap()
        .to_rfc3339()
}

pub struct Collector(pub std::sync::mpsc::Sender<bool>);
pub fn start(app: tauri::AppHandle) {
    use tauri::Manager;
    let (tx, rx) = std::sync::mpsc::channel();
    app.manage(Collector(tx));
    std::thread::spawn(move || {
        let mut github = Github::new(
            crate::ports::SystemProcesses,
            crate::ports::GithubHttp,
            crate::ports::SystemClock,
        );
        loop {
            let manual = rx
                .recv_timeout(std::time::Duration::from_secs(1))
                .unwrap_or(false);
            let visible = app
                .state::<std::sync::Mutex<crate::state::AppState>>()
                .lock()
                .unwrap()
                .module_visible()
                == Some(Module::Github);
            // Publish previous data before blocking on HTTP.
            if github.should_collect(visible, manual) {
                crate::snapshot::publish(&app, github.snapshot(true));
            }
            if let Some(snapshot) = github.step(visible, manual) {
                crate::snapshot::publish(&app, snapshot);
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::fakes::FakeClock;
    pub(super) struct Gh;
    impl crate::ports::Processes for Gh {
        fn execute(&self, _: &str, _: &[&str]) -> Result<String, ()> {
            Err(())
        }
    }
    pub(super) struct Http;
    impl crate::ports::Http for Http {
        fn graphql(&self, _: &str, _: &serde_json::Value) -> Result<serde_json::Value, ()> {
            panic!("HTTP must not be called without credentials")
        }
    }
    #[test]
    fn missing_gh_publishes_setup_without_credentials() {
        let mut github = Github::new(Gh, Http, FakeClock::at(1793448000000));
        let s = github.refresh(true).unwrap();
        assert_eq!(s.data["state"], "missingGh");
        assert!(!serde_json::to_string(&s).unwrap().contains("token"));
    }
}

#[cfg(test)]
mod scenarios {
    use super::*;
    use crate::ports::fakes::FakeClock;
    struct Gh;
    impl Processes for Gh {
        fn execute(&self, _: &str, _: &[&str]) -> Result<String, ()> {
            Ok("test-secret".into())
        }
    }
    struct Api {
        empty: bool,
    }
    impl Http for Api {
        fn graphql(&self, _: &str, body: &Value) -> Result<Value, ()> {
            if body["query"].as_str().unwrap().contains("viewer {") {
                Ok(
                    serde_json::from_str(include_str!("../../fixtures/github/viewer.json"))
                        .unwrap(),
                )
            } else {
                let mut v: Value =
                    serde_json::from_str(include_str!("../../fixtures/github/history.json"))
                        .unwrap();
                let c = &mut v["data"]["repository"]["defaultBranchRef"]["target"];
                if self.empty {
                    c["window"]["totalCount"] = json!(0);
                }
                // Controlled capture variant: another repository ties but has a more recent commit.
                if body["variables"]["name"] == "repo-1" {
                    c["window"]["nodes"][0]["committedDate"] = json!("2026-10-31T12:00:00Z");
                }
                Ok(v)
            }
        }
    }
    #[test]
    fn ties_choose_latest_commit_without_exposing_token() {
        let mut g = Github::new(Gh, Api { empty: false }, FakeClock::at(1793448000000));
        let s = g.refresh(true).unwrap();
        assert_eq!(s.data["featured"]["name"], "exemplo/repo-1");
        assert_eq!(s.data["featured"]["commits"], 27);
        assert!(!serde_json::to_string(&s).unwrap().contains("test-secret"));
    }
    #[test]
    fn empty_window_does_not_replace_featured_repository() {
        let mut g = Github::new(Gh, Api { empty: true }, FakeClock::at(1793448000000));
        assert!(g.refresh(true).unwrap().data["featured"].is_null());
    }
    #[test]
    fn visible_and_manual_cadence_use_injected_clock() {
        let r = FakeClock::at(1793448000000);
        let mut g = Github::new(Gh, Api { empty: false }, r.clone());
        g.refresh(true);
        assert!(!g.due(true));
        r.advance(300_000);
        assert!(g.due(true));
        assert!(!g.due(false));
        assert!(g.manual());
        assert!(!g.manual());
        r.advance(10_000);
        assert!(g.manual());
    }
    struct CalendarYear(FakeClock);
    impl Http for CalendarYear {
        fn graphql(&self, token: &str, body: &Value) -> Result<Value, ()> {
            if body["query"]
                .as_str()
                .unwrap()
                .contains("contributionsCollection")
            {
                assert_eq!(
                    body["variables"]["from"],
                    format!("{}-01-01T00:00:00Z", year(self.0.now()))
                );
                assert_eq!(
                    body["variables"]["to"],
                    format!("{}-12-31T23:59:59Z", year(self.0.now()))
                );
            }
            Api { empty: false }.graphql(token, body)
        }
    }
    #[test]
    fn year_rollover_refreshes_calendar_before_one_hour() {
        let r = FakeClock::at(1798761599000); // 2026-12-31 23:59:59 UTC
        let mut g = Github::new(Gh, CalendarYear(r.clone()), r.clone());
        g.refresh(true);
        r.advance(2000);
        assert!(g.due(false));
        assert_eq!(g.step(false, false).unwrap().data["state"], "ready");
    }
    struct ApiError;
    impl Http for ApiError {
        fn graphql(&self, _: &str, _: &Value) -> Result<Value, ()> {
            Err(())
        }
    }
    #[test]
    fn ui_snapshots_match_core_output() {
        let clock = FakeClock::at(1793448000000);
        let mut full = Github::new(Gh, Api { empty: false }, clock.clone());
        let mut empty = Github::new(Gh, Api { empty: true }, clock.clone());
        let mut error = Github::new(Gh, ApiError, clock.clone());
        let mut without = Github::new(super::tests::Gh, super::tests::Http, clock);
        for (name, snapshot) in [
            ("full", full.refresh(true).unwrap()),
            ("empty", empty.refresh(true).unwrap()),
            ("missing-gh", without.refresh(true).unwrap()),
            ("error", error.refresh(true).unwrap()),
        ] {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(format!("../src/lib/github/fixtures/{name}.json"));
            if std::env::var_os("UPDATE_FIXTURES").is_some() {
                std::fs::write(
                    &path,
                    serde_json::to_string_pretty(&snapshot.data).unwrap() + "\n",
                )
                .unwrap();
            }
            let fixture: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
            assert_eq!(snapshot.data, fixture);
        }
    }

    #[test]
    fn missing_gh_backs_off_and_manual_refresh_waits_ten_seconds() {
        let r = FakeClock::at(1793448000000);
        let mut g = Github::new(super::tests::Gh, super::tests::Http, r.clone());
        assert!(g.step(false, false).is_some());
        assert!(g.step(false, false).is_none());
        assert!(g.step(false, true).is_some());
        r.advance(9999);
        assert!(g.step(false, true).is_none());
        r.advance(1);
        assert!(g.step(false, true).is_some());
        r.advance(30000);
        assert!(g.step(false, false).is_some());
    }
    struct CadenciaCalendar {
        clock: FakeClock,
    }
    impl Http for CadenciaCalendar {
        fn graphql(&self, token: &str, body: &Value) -> Result<Value, ()> {
            if body["query"].as_str().unwrap().contains("viewer {") {
                let mut response = Api { empty: false }.graphql(token, body)?;
                let calendar = body["query"]
                    .as_str()
                    .unwrap()
                    .contains("contributionsCollection");
                let mut total = 1;
                if self.clock.now() >= 1793451600000 {
                    total = 99;
                }
                if calendar {
                    response["data"]["viewer"]["contributionsCollection"]["contributionCalendar"]
                        ["totalContributions"] = json!(total);
                } else {
                    response["data"]["viewer"]
                        .as_object_mut()
                        .unwrap()
                        .remove("contributionsCollection");
                }
                Ok(response)
            } else {
                Api { empty: false }.graphql(token, body)
            }
        }
    }
    #[test]
    fn calendar_waits_one_hour_while_featured_refreshes_every_five_minutes() {
        let r = FakeClock::at(1793448000000);
        let mut g = Github::new(Gh, CadenciaCalendar { clock: r.clone() }, r.clone());
        let initial = g.step(true, false).unwrap();
        assert_eq!(
            initial.data["calendar"]["contributionCalendar"]["totalContributions"],
            1
        );
        r.advance(300000);
        let seguinte = g.step(true, false).unwrap();
        assert_eq!(seguinte.updated_at, Some(1793448300000));
        assert_eq!(seguinte.data["calendar"], initial.data["calendar"]);
        r.advance(3300000);
        let time = g.step(false, false).unwrap();
        assert_eq!(
            time.data["calendar"]["contributionCalendar"]["totalContributions"],
            99
        );
    }
}
