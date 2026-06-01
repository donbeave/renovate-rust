//! Bitbucket Cloud platform client.
//!
//! Implements [`PlatformClient`] against the Bitbucket Cloud REST API v2.
//!
//! Renovate reference: `lib/modules/platform/bitbucket/index.ts`.
//!
//! ## Authentication
//!
//! Bitbucket Cloud uses HTTP Basic Auth with `username:app_password`.
//! The `HttpClient` bearer-token mechanism is used with a base64-encoded
//! `username:app_password` pair passed as the token for compatibility;
//! however, this client constructs basic-auth headers directly where needed.

use serde::{Deserialize, Serialize};

use crate::http::{HttpClient, HttpError};
use crate::platform::{CombinedBranchStatus, CurrentUser, PlatformClient, PlatformError, RawFile, RepoInitResult};

pub const BITBUCKET_API_BASE: &str = "https://api.bitbucket.org/2.0";

#[derive(Debug, Clone)]
pub struct BitbucketClient {
    http: HttpClient,
    api_base: String,
    username: String,
    app_password: String,
}

impl BitbucketClient {
    pub fn new(
        username: impl Into<String>,
        app_password: impl Into<String>,
    ) -> Result<Self, HttpError> {
        Self::with_endpoint(username, app_password, BITBUCKET_API_BASE)
    }

    pub fn with_endpoint(
        username: impl Into<String>,
        app_password: impl Into<String>,
        api_base: impl Into<String>,
    ) -> Result<Self, HttpError> {
        Ok(Self {
            http: HttpClient::new()?,
            api_base: api_base.into().trim_end_matches('/').to_owned(),
            username: username.into(),
            app_password: app_password.into(),
        })
    }

    fn auth_header(&self) -> String {
        use base64::Engine as _;
        let creds = format!("{}:{}", self.username, self.app_password);
        format!(
            "Basic {}",
            base64::engine::general_purpose::STANDARD.encode(creds)
        )
    }

    fn repo_slug(owner: &str, repo: &str) -> String {
        format!("{owner}/{repo}")
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BitbucketUser {
    pub username: String,
    pub display_name: Option<String>,
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BitbucketRepo {
    pub uuid: Option<String>,
    pub full_name: Option<String>,
    pub name: Option<String>,
    pub is_private: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BitbucketPr {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "source")]
    pub source: BitbucketPrBranch,
    #[serde(rename = "destination")]
    pub destination: BitbucketPrBranch,
    pub created_on: Option<String>,
    pub updated_on: Option<String>,
    pub author: Option<BitbucketPrAuthor>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BitbucketPrBranch {
    pub branch: BitbucketBranchRef,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BitbucketBranchRef {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BitbucketPrAuthor {
    pub display_name: Option<String>,
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BitbucketComment {
    pub id: Option<i64>,
    pub content: Option<BitbucketCommentContent>,
    pub created_on: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BitbucketCommentContent {
    pub raw: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BitbucketPrList {
    pub values: Vec<BitbucketPr>,
    pub next: Option<String>,
}

#[derive(Debug, Serialize)]
struct CreatePrRequest {
    title: String,
    description: String,
    source: CreatePrBranchRef,
    destination: CreatePrBranchRef,
}

#[derive(Debug, Serialize)]
struct CreatePrBranchRef {
    branch: BranchName,
}

#[derive(Debug, Serialize)]
struct BranchName {
    name: String,
}

#[derive(Debug, Serialize)]
struct MergePrRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    close_source_branch: Option<bool>,
}

#[derive(Debug, Serialize)]
struct CommentRequest {
    content: CommentContent,
}

#[derive(Debug, Serialize)]
struct CommentContent {
    raw: String,
}

pub async fn create_pr(
    client: &BitbucketClient,
    owner: &str,
    repo: &str,
    source_branch: &str,
    target_branch: &str,
    title: &str,
    body: &str,
) -> Result<Option<i64>, PlatformError> {
    let url = format!(
        "{}/repositories/{}/pullrequests",
        client.api_base,
        BitbucketClient::repo_slug(owner, repo)
    );
    let request = CreatePrRequest {
        title: title.to_owned(),
        description: body.to_owned(),
        source: CreatePrBranchRef {
            branch: BranchName {
                name: source_branch.to_owned(),
            },
        },
        destination: CreatePrBranchRef {
            branch: BranchName {
                name: target_branch.to_owned(),
            },
        },
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
        if resp.status() == reqwest::StatusCode::CONFLICT {
            return Ok(None);
        }
        return Err(PlatformError::Http(HttpError::Status {
            status: resp.status(),
            url,
        }));
    }
    let pr: BitbucketPr = resp.json().await.map_err(HttpError::Request)?;
    Ok(Some(pr.id))
}

pub async fn list_prs(
    client: &BitbucketClient,
    owner: &str,
    repo: &str,
    state: Option<&str>,
) -> Result<Vec<BitbucketPr>, PlatformError> {
    let state_param = state.unwrap_or("OPEN");
    let url = format!(
        "{}/repositories/{}/pullrequests?state={}",
        client.api_base,
        BitbucketClient::repo_slug(owner, repo),
        state_param
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
    let list: BitbucketPrList = resp.json().await.map_err(HttpError::Request)?;
    Ok(list.values)
}

pub async fn merge_pr(
    client: &BitbucketClient,
    owner: &str,
    repo: &str,
    pr_id: i64,
) -> Result<(), PlatformError> {
    let url = format!(
        "{}/repositories/{}/pullrequests/{}/merge",
        client.api_base,
        BitbucketClient::repo_slug(owner, repo),
        pr_id
    );
    let request = MergePrRequest {
        message: None,
        close_source_branch: Some(true),
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
    Ok(())
}

pub async fn add_comment(
    client: &BitbucketClient,
    owner: &str,
    repo: &str,
    pr_id: i64,
    text: &str,
) -> Result<BitbucketComment, PlatformError> {
    let url = format!(
        "{}/repositories/{}/pullrequests/{}/comments",
        client.api_base,
        BitbucketClient::repo_slug(owner, repo),
        pr_id
    );
    let request = CommentRequest {
        content: CommentContent {
            raw: text.to_owned(),
        },
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
    let comment: BitbucketComment = resp.json().await.map_err(HttpError::Request)?;
    Ok(comment)
}

pub async fn get_pr(
    client: &BitbucketClient,
    owner: &str,
    repo: &str,
    pr_id: i64,
) -> Result<BitbucketPr, PlatformError> {
    let url = format!(
        "{}/repositories/{}/pullrequests/{}",
        client.api_base,
        BitbucketClient::repo_slug(owner, repo),
        pr_id
    );
    let rb = client
        .http
        .inner
        .get(&url)
        .header("Authorization", client.auth_header());
    let resp = rb.send().await.map_err(HttpError::Request)?;
    if !resp.status().is_success() {
        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(PlatformError::Unexpected("PR not found".to_owned()));
        }
        return Err(PlatformError::Http(HttpError::Status {
            status: resp.status(),
            url,
        }));
    }
    let pr: BitbucketPr = resp.json().await.map_err(HttpError::Request)?;
    Ok(pr)
}

impl PlatformClient for BitbucketClient {
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
        let url = format!("{}/user", self.api_base);
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
        let user: BitbucketUser = resp.json().await.map_err(HttpError::Request)?;
        Ok(CurrentUser {
            login: user.username,
        })
    }

    async fn get_raw_file(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
    ) -> Result<Option<RawFile>, PlatformError> {
        let url = format!(
            "{}/repositories/{}/{}/src/HEAD/{}",
            self.api_base, owner, repo, path
        );
        let rb = self
            .http
            .inner
            .get(&url)
            .header("Authorization", self.auth_header());
        let resp = rb.send().await.map_err(HttpError::Request)?;
        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(PlatformError::Unauthorized);
        }
        if !resp.status().is_success() {
            return Err(PlatformError::Http(HttpError::Status {
                status: resp.status(),
                url,
            }));
        }
        let content = resp.text().await.map_err(HttpError::Request)?;
        Ok(Some(RawFile {
            path: path.to_owned(),
            content,
        }))
    }

    async fn get_file_list(&self, owner: &str, repo: &str) -> Result<Vec<String>, PlatformError> {
        let url = format!(
            "{}/repositories/{}/{}/src?format=meta",
            self.api_base, owner, repo
        );
        let rb = self
            .http
            .inner
            .get(&url)
            .header("Authorization", self.auth_header());
        let resp = rb.send().await.map_err(HttpError::Request)?;
        if !resp.status().is_success() {
            return Err(PlatformError::Http(HttpError::Status {
                status: resp.status(),
                url,
            }));
        }
        #[derive(Deserialize)]
        struct FileListResponse {
            values: Vec<FileEntry>,
        }
        #[derive(Deserialize)]
        struct FileEntry {
            path: Option<String>,
            #[serde(rename = "type")]
            entry_type: Option<String>,
        }
        let list: FileListResponse = resp.json().await.map_err(HttpError::Request)?;
        let files = list
            .values
            .into_iter()
            .filter(|e| e.entry_type.as_deref() == Some("commit_file"))
            .filter_map(|e| e.path)
            .collect();
        Ok(files)
    }

    async fn create_pr(
        &self,
        owner: &str,
        repo: &str,
        source_branch: &str,
        target_branch: &str,
        title: &str,
        body: &str,
    ) -> Result<Option<i64>, PlatformError> {
        create_pr(self, owner, repo, source_branch, target_branch, title, body).await
    }

    async fn update_pr(
        &self,
        owner: &str,
        repo: &str,
        pr_number: i64,
        title: Option<&str>,
        body: Option<&str>,
        state: Option<&str>,
    ) -> Result<(), PlatformError> {
        let url = format!(
            "{}/repositories/{}/pullrequests/{}",
            self.api_base,
            BitbucketClient::repo_slug(owner, repo),
            pr_number
        );
        if title.is_none() && body.is_none() && state.is_none() {
            return Ok(());
        }
        #[derive(Serialize)]
        struct UpdatePrRequest {
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            state: Option<String>,
        }
        let request = UpdatePrRequest {
            title: title.map(|s| s.to_owned()),
            description: body.map(|s| s.to_owned()),
            state: state.map(|s| s.to_uppercase()),
        };
        let request_json = serde_json::to_string(&request)
            .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;
        let rb = self
            .http
            .inner
            .put(&url)
            .header("Authorization", self.auth_header())
            .header("Content-Type", "application/json")
            .body(request_json);
        let resp = rb.send().await.map_err(HttpError::Request)?;
        if !resp.status().is_success() {
            return Err(PlatformError::Http(HttpError::Status {
                status: resp.status(),
                url,
            }));
        }
        Ok(())
    }

    async fn get_branch_status(
        &self,
        _owner: &str,
        _repo: &str,
        _branch: &str,
    ) -> Result<CombinedBranchStatus, PlatformError> {
        Err(PlatformError::NotSupported(
            "Bitbucket branch status not yet implemented".to_owned(),
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
            "Bitbucket write_file not yet implemented".to_owned(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use base64::Engine as _;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    fn make_client(server_uri: &str) -> BitbucketClient {
        BitbucketClient::with_endpoint("testuser", "testpass", server_uri).unwrap()
    }

    // Rust-specific: bitbucket behavior test
    #[test]
    fn auth_header_uses_basic_encoding() {
        let client = BitbucketClient::new("user", "pass").unwrap();
        let auth = client.auth_header();
        assert!(auth.starts_with("Basic "));
        let encoded = &auth["Basic ".len()..];
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .unwrap();
        assert_eq!(String::from_utf8(decoded).unwrap(), "user:pass");
    }

    // Rust-specific: bitbucket behavior test
    #[test]
    fn repo_slug_formats_owner_and_repo() {
        assert_eq!(BitbucketClient::repo_slug("owner", "repo"), "owner/repo");
    }

    #[tokio::test]
    async fn get_current_user_returns_login() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user"))
            .and(header("Authorization", make_client("").auth_header()))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "username": "testuser",
                "display_name": "Test User",
                "uuid": "{abc123}"
            })))
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
            .and(path("/user"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let err = client.get_current_user().await.unwrap_err();
        assert!(matches!(err, PlatformError::Unauthorized));
    }

    #[tokio::test]
    async fn create_pr_returns_pr_id() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repositories/owner/repo/pullrequests"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "id": 42,
                "title": "Test PR",
                "source": {"branch": {"name": "feature"}},
                "destination": {"branch": {"name": "main"}}
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let pr_id = create_pr(
            &client, "owner", "repo", "feature", "main", "Test PR", "body",
        )
        .await
        .unwrap();
        assert_eq!(pr_id, Some(42));
    }

    #[tokio::test]
    async fn create_pr_conflict_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repositories/owner/repo/pullrequests"))
            .respond_with(ResponseTemplate::new(409))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let pr_id = create_pr(
            &client, "owner", "repo", "feature", "main", "Test PR", "body",
        )
        .await
        .unwrap();
        assert_eq!(pr_id, None);
    }

    #[tokio::test]
    async fn list_prs_returns_values() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repositories/owner/repo/pullrequests"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "values": [
                    {"id": 1, "title": "PR 1", "source": {"branch": {"name": "f1"}}, "destination": {"branch": {"name": "main"}}},
                    {"id": 2, "title": "PR 2", "source": {"branch": {"name": "f2"}}, "destination": {"branch": {"name": "main"}}}
                ]
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let prs = list_prs(&client, "owner", "repo", None).await.unwrap();
        assert_eq!(prs.len(), 2);
        assert_eq!(prs[0].id, 1);
        assert_eq!(prs[1].id, 2);
    }

    #[tokio::test]
    async fn merge_pr_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repositories/owner/repo/pullrequests/7/merge"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        merge_pr(&client, "owner", "repo", 7).await.unwrap();
    }

    #[tokio::test]
    async fn add_comment_returns_comment() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repositories/owner/repo/pullrequests/3/comments"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "id": 99,
                "content": {"raw": "hello"},
                "created_on": "2024-01-01T00:00:00Z"
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let comment = add_comment(&client, "owner", "repo", 3, "hello")
            .await
            .unwrap();
        assert_eq!(comment.id, Some(99));
    }

    #[tokio::test]
    async fn get_pr_returns_pr() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repositories/owner/repo/pullrequests/5"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 5,
                "title": "My PR",
                "description": "desc",
                "state": "OPEN",
                "source": {"branch": {"name": "feat"}},
                "destination": {"branch": {"name": "main"}}
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let pr = get_pr(&client, "owner", "repo", 5).await.unwrap();
        assert_eq!(pr.id, 5);
        assert_eq!(pr.title, "My PR");
    }

    #[tokio::test]
    async fn get_pr_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repositories/owner/repo/pullrequests/999"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let err = get_pr(&client, "owner", "repo", 999).await.unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(_)));
    }

    #[tokio::test]
    async fn update_pr_sends_put() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/repositories/owner/repo/pullrequests/10"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        client
            .update_pr("owner", "repo", 10, Some("new title"), None, None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn create_pr_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repositories/owner/repo/pullrequests"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let result = create_pr(&client, "owner", "repo", "title", "body", "src", "dst").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn list_prs_empty() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repositories/owner/repo/pullrequests"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"values": []})),
            )
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let prs = list_prs(&client, "owner", "repo", None).await.unwrap();
        assert!(prs.is_empty());
    }

    #[tokio::test]
    async fn merge_pr_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/repositories/owner/repo/pullrequests/5/merge"))
            .respond_with(ResponseTemplate::new(409))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let result = merge_pr(&client, "owner", "repo", 5).await;
        assert!(result.is_err());
    }
}
