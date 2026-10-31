use super::*;
use crate::ports::{fakes::FakeClock, Keyring};
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
struct Vault(Arc<Mutex<Option<String>>>);
impl Keyring for Vault {
    fn read(&self, _: &str) -> Result<Option<String>, ()> {
        Ok(self.0.lock().unwrap().clone())
    }
    fn save(&self, _: &str, secret: &str) -> Result<(), ()> {
        *self.0.lock().unwrap() = Some(secret.into());
        Ok(())
    }
}
struct Validation;
impl Http for Validation {
    fn graphql(&self, _: &str, _: &Value) -> Result<Value, HttpError> {
        unreachable!()
    }
    fn github_validation(
        &self,
        _: &str,
    ) -> Result<crate::ports::GithubValidationResponse, HttpError> {
        Ok(serde_json::from_str(include_str!("../../fixtures/github/validation.json")).unwrap())
    }
}
#[test]
fn validated_token_is_saved_only_in_keyring_and_publishes_identity() {
    let vault = Vault::default();
    let mut github = Github::new(super::tests::Gh, Validation, FakeClock::at(1793448000000))
        .with_keyring(vault.clone());
    assert_eq!(github.save_token("private-token"), Ok("exemplo".into()));
    assert_eq!(
        vault.read("github").unwrap().as_deref(),
        Some("private-token")
    );
    let data = github.snapshot(false).data;
    assert_eq!(
        data["credential"],
        json!({"source":"token", "login":"exemplo"})
    );
    assert!(!data.to_string().contains("private-token"));
}

#[test]
fn unavailable_keyring_refuses_saving_and_keeps_setup_state() {
    let mut github = Github::new(super::tests::Gh, Validation, FakeClock::at(1793448000000));
    assert_eq!(
        github.save_token("private-token"),
        Err(CredentialError::KeyringUnavailable)
    );
    assert_eq!(github.snapshot(false).data["state"], "missingGh");
    assert!(github.snapshot(false).data["credential"].is_null());
}

struct InvalidValidation(Option<u16>, bool);
impl Http for InvalidValidation {
    fn graphql(&self, _: &str, _: &Value) -> Result<Value, HttpError> {
        unreachable!()
    }
    fn github_validation(
        &self,
        _: &str,
    ) -> Result<crate::ports::GithubValidationResponse, HttpError> {
        if self.1 {
            let mut response = Validation.github_validation("")?;
            response.scopes = None;
            Ok(response)
        } else {
            Err(HttpError {
                status: self.0,
                ..HttpError::default()
            })
        }
    }
}
#[test]
fn validation_failures_preserve_existing_secret() {
    for (status, missing_scope, expected) in [
        (Some(401), false, CredentialError::InvalidToken),
        (Some(403), false, CredentialError::InvalidToken),
        (None, false, CredentialError::Offline),
        (Some(503), false, CredentialError::Offline),
        (None, true, CredentialError::MissingRepo),
    ] {
        let vault = Vault::default();
        vault.save("github", "previous-secret").unwrap();
        let mut github = Github::new(
            super::tests::Gh,
            InvalidValidation(status, missing_scope),
            FakeClock::at(1793448000000),
        )
        .with_keyring(vault.clone());
        assert_eq!(github.save_token("invalid-secret"), Err(expected));
        assert_eq!(
            vault.read("github").unwrap().as_deref(),
            Some("previous-secret")
        );
        assert!(github.snapshot(false).data["credential"].is_null());
    }
}

struct RecoveringApi(Arc<Mutex<Option<u16>>>);
impl Http for RecoveringApi {
    fn graphql(&self, token: &str, body: &Value) -> Result<Value, HttpError> {
        if let Some(status) = *self.0.lock().unwrap() {
            return Err(HttpError {
                status: Some(status),
                ..HttpError::default()
            });
        }
        super::scenarios::Api { empty: false }.graphql(token, body)
    }
    fn github_validation(
        &self,
        token: &str,
    ) -> Result<crate::ports::GithubValidationResponse, HttpError> {
        Validation.github_validation(token)
    }
}
#[test]
fn saved_token_resumes_collection_after_rejection_and_survives_restart() {
    let vault = Vault::default();
    vault.save("github", "old-secret").unwrap();
    let failure = Arc::new(Mutex::new(Some(401)));
    let clock = FakeClock::at(1793448000000);
    let mut github = Github::new(
        super::tests::Gh,
        RecoveringApi(failure.clone()),
        clock.clone(),
    )
    .with_keyring(vault.clone());
    assert_eq!(
        github.step(true, false).unwrap().data["state"],
        "invalidCredential"
    );
    assert!(github.step(true, true).is_none());
    github.save_token("new-secret").unwrap();
    *failure.lock().unwrap() = None;
    let snapshot = github.step(true, false).unwrap();
    assert_eq!(snapshot.data["state"], "ready");
    assert_eq!(
        snapshot.data["credential"],
        json!({"source":"token", "login":"exemplo"})
    );
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../src/lib/github/fixtures/own-token.json");
    if std::env::var_os("UPDATE_FIXTURES").is_some() {
        std::fs::write(
            &path,
            serde_json::to_string_pretty(&snapshot.data).unwrap() + "\n",
        )
        .unwrap();
    }
    let fixture: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    assert_eq!(snapshot.data, fixture);
    let mut restarted =
        Github::new(super::tests::Gh, RecoveringApi(failure), clock).with_keyring(vault);
    assert_eq!(
        restarted.step(true, false).unwrap().data["credential"]["source"],
        "token"
    );
}

struct AuthenticatedGh;
impl Processes for AuthenticatedGh {
    fn execute(&self, _: &str, _: &[&str]) -> Result<String, ()> {
        Ok("gh-secret".into())
    }
}
struct PreferredGh;
impl Http for PreferredGh {
    fn graphql(&self, token: &str, body: &Value) -> Result<Value, HttpError> {
        assert_eq!(token, "gh-secret");
        super::scenarios::Api { empty: false }.graphql(token, body)
    }
}
#[test]
fn authenticated_gh_takes_precedence_without_copying_its_token() {
    let vault = Vault::default();
    vault.save("github", "own-secret").unwrap();
    let mut github = Github::new(AuthenticatedGh, PreferredGh, FakeClock::at(1793448000000))
        .with_keyring(vault.clone());
    assert_eq!(
        github.step(true, false).unwrap().data["credential"],
        json!({"source":"gh", "login":"exemplo"})
    );
    assert_eq!(vault.read("github").unwrap().as_deref(), Some("own-secret"));
}

#[test]
fn own_token_and_commit_content_never_reach_cache_or_logs() {
    use crate::persistence::{Cache, DiskStorage, LocalPersistence, Logs};
    let cache = tempfile::tempdir().unwrap();
    let logs = tempfile::tempdir().unwrap();
    let vault = Vault::default();
    let mut github = Github::with_persistence(
        super::tests::Gh,
        RecoveringApi(Default::default()),
        FakeClock::at(1793448000000),
        LocalPersistence {
            cache: Cache(DiskStorage(cache.path().into())),
            logs: Logs(DiskStorage(logs.path().into())),
        },
    )
    .with_keyring(vault);
    github.save_token("own-private-secret").unwrap();
    github.step(true, false).unwrap();
    for text in [
        std::fs::read_to_string(cache.path().join("github.json")).unwrap(),
        std::fs::read_to_string(logs.path().join("info.log")).unwrap(),
    ] {
        assert!(!text.contains("own-private-secret"));
        assert!(!text.contains("Atualiza documentação"));
    }
}

struct CapturedValidation {
    scopes: Option<&'static str>,
    body: Option<Value>,
}
impl Http for CapturedValidation {
    fn graphql(&self, _: &str, _: &Value) -> Result<Value, HttpError> {
        unreachable!()
    }
    fn github_validation(
        &self,
        token: &str,
    ) -> Result<crate::ports::GithubValidationResponse, HttpError> {
        let mut response = Validation.github_validation(token)?;
        response.scopes = self.scopes.map(str::to_owned);
        if let Some(body) = &self.body {
            response.body = body.clone();
        }
        Ok(response)
    }
}
#[test]
fn only_exact_repo_scope_and_valid_viewer_can_be_saved() {
    for (scopes, body, expected) in [
        (
            Some("repo:status, public_repo"),
            None,
            CredentialError::MissingRepo,
        ),
        (
            Some("read:org, repo:invite"),
            None,
            CredentialError::MissingRepo,
        ),
        (Some(""), None, CredentialError::MissingRepo),
        (None, None, CredentialError::MissingRepo),
        (
            Some("repo"),
            Some(json!({"data":{"viewer":null}})),
            CredentialError::InvalidToken,
        ),
        (
            Some("repo"),
            Some(json!({"errors":[{"message":"not authorized"}]})),
            CredentialError::InvalidToken,
        ),
    ] {
        let vault = Vault::default();
        let mut github = Github::new(
            super::tests::Gh,
            CapturedValidation { scopes, body },
            FakeClock::at(1793448000000),
        )
        .with_keyring(vault.clone());
        assert_eq!(github.save_token("private-token"), Err(expected));
        assert_eq!(vault.read("github").unwrap(), None);
    }
}

struct RotatingGh(Arc<Mutex<String>>);
impl Processes for RotatingGh {
    fn execute(&self, _: &str, _: &[&str]) -> Result<String, ()> {
        Ok(self.0.lock().unwrap().clone())
    }
}
struct AccountApi;
impl Http for AccountApi {
    fn graphql(&self, token: &str, body: &Value) -> Result<Value, HttpError> {
        let mut response = super::scenarios::Api { empty: false }.graphql(token, body)?;
        if token == "account-b" && response["data"]["viewer"].is_object() {
            response["data"]["viewer"]["login"] = json!("other-account");
            response["data"]["viewer"]["contributionsCollection"]["contributionCalendar"]
                ["totalContributions"] = json!(999);
            assert!(
                body["query"]
                    .as_str()
                    .unwrap()
                    .contains("contributionsCollection"),
                "account change must request its calendar"
            );
        }
        Ok(response)
    }
}
#[test]
fn automatic_account_change_refreshes_the_calendar_with_the_identity() {
    let token = Arc::new(Mutex::new("account-a".into()));
    let clock = FakeClock::at(1793448000000);
    let mut github = Github::new(RotatingGh(token.clone()), AccountApi, clock.clone());
    github.step(true, false).unwrap();
    *token.lock().unwrap() = "account-b".into();
    clock.advance(300_000);
    let data = github.step(true, false).unwrap().data;
    assert_eq!(data["credential"]["login"], "other-account");
    assert_eq!(
        data["calendar"]["contributionCalendar"]["totalContributions"],
        999
    );
}
