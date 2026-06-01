//! Gerrit platform client.
//!
//! Implements [`PlatformClient`] against the Gerrit REST API.
//!
//! Renovate reference: `lib/modules/platform/gerrit/index.ts`.
//!
//! ## Authentication
//!
//! Gerrit uses HTTP Basic Auth with `username:http_password`.
//!
//! ## Notes
//!
//! Gerrit uses "changes" instead of "pull requests". The `PlatformClient`
//! trait methods map to Gerrit change operations where applicable.
//! The standalone functions provide Gerrit-specific operations.

use base64::Engine as _;
use serde::{Deserialize, Serialize};

use crate::http::{HttpClient, HttpError};
use crate::platform::{CombinedBranchStatus, CurrentUser, PlatformClient, PlatformError, RawFile, RepoInitResult};

#[derive(Debug, Clone)]
pub struct GerritClient {
    http: HttpClient,
    api_base: String,
    username: String,
    http_password: String,
}

impl GerritClient {
    pub fn new(
        server_url: impl Into<String>,
        username: impl Into<String>,
        http_password: impl Into<String>,
    ) -> Result<Self, HttpError> {
        Self::with_endpoint(server_url, username, http_password)
    }

    pub fn with_endpoint(
        server_url: impl Into<String>,
        username: impl Into<String>,
        http_password: impl Into<String>,
    ) -> Result<Self, HttpError> {
        let base = server_url.into().trim_end_matches('/').to_owned();
        let api_base = format!("{base}/a");
        Ok(Self {
            http: HttpClient::new()?,
            api_base,
            username: username.into(),
            http_password: http_password.into(),
        })
    }

    fn auth_header(&self) -> String {
        let creds = format!("{}:{}", self.username, self.http_password);
        format!(
            "Basic {}",
            base64::engine::general_purpose::STANDARD.encode(creds)
        )
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GerritAccountInfo {
    #[serde(rename = "_account_id")]
    pub account_id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GerritChangeInfo {
    pub id: Option<String>,
    pub number: Option<String>,
    pub change_id: Option<String>,
    pub subject: Option<String>,
    pub status: Option<String>,
    pub project: Option<String>,
    pub branch: Option<String>,
    pub owner: Option<GerritAccountInfo>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub submittable: Option<bool>,
    pub mergeable: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GerritChange {
    pub id: Option<String>,
    pub change_id: Option<String>,
    pub subject: Option<String>,
    pub status: Option<String>,
    pub project: Option<String>,
    pub branch: Option<String>,
    pub owner: Option<GerritAccountInfo>,
    pub labels: Option<std::collections::HashMap<String, GerritLabelInfo>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GerritLabelInfo {
    pub all: Option<Vec<GerritApprovalInfo>>,
    pub recommended: Option<GerritAccountInfo>,
    pub disliked: Option<GerritAccountInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GerritApprovalInfo {
    pub value: Option<i64>,
    #[serde(rename = "_account_id")]
    pub account_id: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GerritSubmitInput {
    pub on_behalf_of: Option<String>,
}

#[derive(Debug, Serialize)]
struct SubmitInput {
    on_behalf_of: Option<String>,
}

#[derive(Debug, Serialize)]
struct AddReviewerInput {
    reviewer: String,
    #[serde(rename = "confirmed")]
    confirmed: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GerritReviewerInfo {
    pub confirmed: Option<bool>,
    pub reviewers: Option<Vec<GerritAccountInfo>>,
}

pub async fn submit_change(
    client: &GerritClient,
    change_id: &str,
) -> Result<GerritChangeInfo, PlatformError> {
    let url = format!("{}/changes/{change_id}/submit", client.api_base);
    let request = SubmitInput { on_behalf_of: None };
    let request_json = serde_json::to_string(&request)
        .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;

    let rb = client
        .http
        .inner
        .post(&url)
        .header("Authorization", client.auth_header())
        .header("Content-Type", "application/json")
        .body(request_json);
    let resp = rb.send().await.map_err(HttpError::Request)?;
    if resp.status() == reqwest::StatusCode::CONFLICT {
        return Err(PlatformError::Unexpected(
            "Change cannot be submitted".to_owned(),
        ));
    }
    if !resp.status().is_success() {
        return Err(PlatformError::Http(HttpError::Status {
            status: resp.status(),
            url,
        }));
    }
    let body = resp.text().await.map_err(HttpError::Request)?;
    let info: GerritChangeInfo =
        strip_gerrit_json_prefix(&body).map_err(PlatformError::Unexpected)?;
    Ok(info)
}

pub async fn list_changes(
    client: &GerritClient,
    project: &str,
    status: Option<&str>,
) -> Result<Vec<GerritChangeInfo>, PlatformError> {
    let status_param = status.unwrap_or("open");
    let url = format!(
        "{}/changes/?q=project:{}+status:{}&n=25",
        client.api_base, project, status_param
    );
    let rb = client
        .http
        .inner
        .get(&url)
        .header("Authorization", client.auth_header());
    let resp = rb.send().await.map_err(HttpError::Request)?;
    if !resp.status().is_success() {
        return Err(PlatformError::Http(HttpError::Status {
            status: resp.status(),
            url,
        }));
    }
    let body = resp.text().await.map_err(HttpError::Request)?;
    let changes: Vec<GerritChangeInfo> =
        strip_gerrit_json_prefix_list(&body).map_err(PlatformError::Unexpected)?;
    Ok(changes)
}

pub async fn add_reviewer(
    client: &GerritClient,
    change_id: &str,
    reviewer: &str,
) -> Result<GerritReviewerInfo, PlatformError> {
    let url = format!("{}/changes/{change_id}/reviewers", client.api_base);
    let request = AddReviewerInput {
        reviewer: reviewer.to_owned(),
        confirmed: None,
    };
    let request_json = serde_json::to_string(&request)
        .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;

    let rb = client
        .http
        .inner
        .post(&url)
        .header("Authorization", client.auth_header())
        .header("Content-Type", "application/json")
        .body(request_json);
    let resp = rb.send().await.map_err(HttpError::Request)?;
    if !resp.status().is_success() {
        return Err(PlatformError::Http(HttpError::Status {
            status: resp.status(),
            url,
        }));
    }
    let body = resp.text().await.map_err(HttpError::Request)?;
    let info: GerritReviewerInfo =
        strip_gerrit_json_prefix_value(&body).map_err(PlatformError::Unexpected)?;
    Ok(info)
}

pub async fn get_change(
    client: &GerritClient,
    change_id: &str,
) -> Result<GerritChangeInfo, PlatformError> {
    let url = format!("{}/changes/{change_id}/detail", client.api_base);
    let rb = client
        .http
        .inner
        .get(&url)
        .header("Authorization", client.auth_header());
    let resp = rb.send().await.map_err(HttpError::Request)?;
    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(PlatformError::Unexpected("Change not found".to_owned()));
    }
    if !resp.status().is_success() {
        return Err(PlatformError::Http(HttpError::Status {
            status: resp.status(),
            url,
        }));
    }
    let body = resp.text().await.map_err(HttpError::Request)?;
    let info: GerritChangeInfo =
        strip_gerrit_json_prefix(&body).map_err(PlatformError::Unexpected)?;
    Ok(info)
}

fn strip_gerrit_json_prefix(body: &str) -> Result<GerritChangeInfo, String> {
    let json_str = body.strip_prefix(")]}'\n").unwrap_or(body);
    serde_json::from_str(json_str).map_err(|e| format!("JSON parse: {e}"))
}

fn strip_gerrit_json_prefix_list(body: &str) -> Result<Vec<GerritChangeInfo>, String> {
    let json_str = body.strip_prefix(")]}'\n").unwrap_or(body);
    serde_json::from_str(json_str).map_err(|e| format!("JSON parse: {e}"))
}

fn strip_gerrit_json_prefix_value<T: serde::de::DeserializeOwned>(body: &str) -> Result<T, String> {
    let json_str = body.strip_prefix(")]}'\n").unwrap_or(body);
    serde_json::from_str(json_str).map_err(|e| format!("JSON parse: {e}"))
}

impl PlatformClient for GerritClient {
    async fn init_repo(
        &self,
        _owner: &str,
        _repo: &str,
    ) -> Result<RepoInitResult, PlatformError> {
        Ok(RepoInitResult {
            default_branch: "main".to_owned(),
            is_fork: false,
            repo_fingerprint: String::new(),
            merge_method: None,
            auto_merge_allowed: false,
            has_issues_enabled: true,
            has_vulnerability_alerts_enabled: false,
        })
    }
    async fn get_current_user(&self) -> Result<CurrentUser, PlatformError> {
        let url = format!("{}/accounts/self", self.api_base);
        let rb = self
            .http
            .inner
            .get(&url)
            .header("Authorization", self.auth_header());
        let resp = rb.send().await.map_err(HttpError::Request)?;
        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(PlatformError::Unauthorized);
        }
        if !resp.status().is_success() {
            return Err(PlatformError::Http(HttpError::Status {
                status: resp.status(),
                url,
            }));
        }
        let body = resp.text().await.map_err(HttpError::Request)?;
        let account: GerritAccountInfo =
            strip_gerrit_json_prefix_value(&body).map_err(PlatformError::Unexpected)?;
        Ok(CurrentUser {
            login: account
                .username
                .or(account.name)
                .unwrap_or_else(|| "gerrit-user".to_owned()),
        })
    }

    async fn get_raw_file(
        &self,
        _owner: &str,
        _repo: &str,
        _path: &str,
    ) -> Result<Option<RawFile>, PlatformError> {
        Err(PlatformError::NotSupported(
            "Gerrit get_raw_file not yet implemented".to_owned(),
        ))
    }

    async fn get_file_list(&self, _owner: &str, _repo: &str) -> Result<Vec<String>, PlatformError> {
        Err(PlatformError::NotSupported(
            "Gerrit get_file_list not yet implemented".to_owned(),
        ))
    }

    async fn create_pr(
        &self,
        _owner: &str,
        _repo: &str,
        _source_branch: &str,
        _target_branch: &str,
        _title: &str,
        _body: &str,
    ) -> Result<Option<i64>, PlatformError> {
        Err(PlatformError::NotSupported(
            "Gerrit uses changes, not pull requests".to_owned(),
        ))
    }

    async fn update_pr(
        &self,
        _owner: &str,
        _repo: &str,
        _pr_number: i64,
        _title: Option<&str>,
        _body: Option<&str>,
        _state: Option<&str>,
    ) -> Result<(), PlatformError> {
        Err(PlatformError::NotSupported(
            "Gerrit uses changes, not pull requests".to_owned(),
        ))
    }

    async fn get_branch_status(
        &self,
        _owner: &str,
        _repo: &str,
        _branch: &str,
    ) -> Result<CombinedBranchStatus, PlatformError> {
        Err(PlatformError::NotSupported(
            "Gerrit branch status not yet implemented".to_owned(),
        ))
    }

    async fn write_file(
        &self,
        _owner: &str,
        _repo: &str,
        _path: &str,
        _content: &str,
    ) -> Result<(), PlatformError> {
        Err(PlatformError::NotSupported(
            "Gerrit write_file not yet implemented".to_owned(),
        ))
    }

    async fn get_pr_list(
        &self,
        _owner: &str,
        _repo: &str,
        _state: Option<&str>,
    ) -> Result<Vec<crate::platform::GhPr>, PlatformError> {
        Err(PlatformError::NotSupported("Gerrit PR list".to_owned()))
    }

    async fn get_pr(
        &self,
        _owner: &str,
        _repo: &str,
        _pr_number: i64,
    ) -> Result<Option<crate::platform::GhPr>, PlatformError> {
        Err(PlatformError::NotSupported("Gerrit get PR".to_owned()))
    }

    async fn get_branch_pr(
        &self,
        _owner: &str,
        _repo: &str,
        _branch: &str,
    ) -> Result<Option<crate::platform::GhPr>, PlatformError> {
        Err(PlatformError::NotSupported("Gerrit get branch PR".to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    fn make_client(server_uri: &str) -> GerritClient {
        GerritClient::new(server_uri, "testuser", "testpass").unwrap()
    }

    // Rust-specific: gerrit behavior test
    #[test]
    fn auth_header_uses_basic_encoding() {
        let client = GerritClient::new("http://gerrit.example.com", "user", "pass").unwrap();
        let auth = client.auth_header();
        assert!(auth.starts_with("Basic "));
        let encoded = &auth["Basic ".len()..];
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .unwrap();
        assert_eq!(String::from_utf8(decoded).unwrap(), "user:pass");
    }

    // Rust-specific: gerrit behavior test
    #[test]
    fn strip_gerrit_json_prefix_removes_magic() {
        let body = ")]}'\n{\"id\": \"test\"}";
        let result: GerritChangeInfo = strip_gerrit_json_prefix_value(body).unwrap();
        assert_eq!(result.id, Some("test".to_owned()));
    }

    // Rust-specific: gerrit behavior test
    #[test]
    fn strip_gerrit_json_prefix_handles_no_prefix() {
        let body = "{\"id\": \"test\"}";
        let result: GerritChangeInfo = strip_gerrit_json_prefix_value(body).unwrap();
        assert_eq!(result.id, Some("test".to_owned()));
    }

    #[tokio::test]
    async fn get_current_user_returns_login() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/a/accounts/self"))
            .and(header("Authorization", make_client("").auth_header()))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                ")]}'\n{\"_account_id\": 1, \"name\": \"Test\", \"username\": \"testuser\"}",
            ))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let user = client.get_current_user().await.unwrap();
        assert_eq!(user.login, "testuser");
    }

    #[tokio::test]
    async fn get_current_user_unauthorized() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/a/accounts/self"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let err = client.get_current_user().await.unwrap_err();
        assert!(matches!(err, PlatformError::Unauthorized));
    }

    #[tokio::test]
    async fn submit_change_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/a/changes/123/submit"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(")]}'\n{\"id\": \"123\", \"status\": \"MERGED\"}"),
            )
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let result = submit_change(&client, "123").await.unwrap();
        assert_eq!(result.status, Some("MERGED".to_owned()));
    }

    #[tokio::test]
    async fn submit_change_conflict() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/a/changes/123/submit"))
            .respond_with(ResponseTemplate::new(409))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let err = submit_change(&client, "123").await.unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(_)));
    }

    #[tokio::test]
    async fn list_changes_returns_values() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/a/changes/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                ")]}'\n[{\"id\": \"1\", \"subject\": \"Change 1\"}, {\"id\": \"2\", \"subject\": \"Change 2\"}]",
            ))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let changes = list_changes(&client, "myproject", None).await.unwrap();
        assert_eq!(changes.len(), 2);
        assert_eq!(changes[0].id, Some("1".to_owned()));
    }

    #[tokio::test]
    async fn add_reviewer_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/a/changes/123/reviewers"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                ")]}'\n{\"confirmed\": true, \"reviewers\": [{\"_account_id\": 5}]}",
            ))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let result = add_reviewer(&client, "123", "reviewer@example.com")
            .await
            .unwrap();
        assert_eq!(result.confirmed, Some(true));
    }

    #[tokio::test]
    async fn get_change_returns_detail() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/a/changes/123/detail"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                ")]}'\n{\"id\": \"123\", \"subject\": \"My change\", \"status\": \"NEW\"}",
            ))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let change = get_change(&client, "123").await.unwrap();
        assert_eq!(change.id, Some("123".to_owned()));
        assert_eq!(change.subject, Some("My change".to_owned()));
    }

    #[tokio::test]
    async fn get_change_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/a/changes/999/detail"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let err = get_change(&client, "999").await.unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(_)));
    }

    #[tokio::test]
    async fn submit_change_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/a/changes/123/submit"))
            .respond_with(ResponseTemplate::new(409))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let result = submit_change(&client, "123").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn list_changes_empty() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/a/changes/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(")]}'\n[]"))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let changes = list_changes(&client, "myproject", Some("status:open"))
            .await
            .unwrap();
        assert!(changes.is_empty());
    }

    #[tokio::test]
    async fn add_reviewer_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/a/changes/123/reviewers"))
            .respond_with(ResponseTemplate::new(422))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let result = add_reviewer(&client, "123", "user@example.com").await;
        assert!(result.is_err());
    }
}
