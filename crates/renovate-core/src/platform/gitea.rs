//! Gitea platform client.
//!
//! Implements [`PlatformClient`] against the Gitea REST API v1.
//!
//! Renovate reference: `lib/modules/platform/gitea/index.ts`.
//!
//! ## Authentication
//!
//! Gitea uses Bearer token authentication (via `token` header or `Authorization: token <value>`).

use base64::Engine as _;
use serde::{Deserialize, Serialize};

use crate::http::{HttpClient, HttpError};
use crate::platform::{CombinedBranchStatus, CurrentUser, PlatformClient, PlatformError, RawFile};
use crate::platform::gitea_forgejo_utils::{
    ContentsListResponse, ContentsResponse, ContentsType, get_merge_method,
};

pub const GITEA_API_VERSION: &str = "api/v1";

#[derive(Debug, Clone)]
pub struct GiteaClient {
    http: HttpClient,
    api_base: String,
    #[allow(dead_code)]
    server_url: String,
}

impl GiteaClient {
    pub fn new(
        server_url: impl Into<String>,
        token: impl Into<String>,
    ) -> Result<Self, HttpError> {
        Self::with_endpoint(server_url, token)
    }

    pub fn with_endpoint(
        server_url: impl Into<String>,
        token: impl Into<String>,
    ) -> Result<Self, HttpError> {
        let server = server_url.into().trim_end_matches('/').to_owned();
        let api_base = format!("{server}/{GITEA_API_VERSION}");
        Ok(Self {
            http: HttpClient::with_token(token)?,
            api_base,
            server_url: server,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GiteaUser {
    pub login: String,
    pub id: Option<i64>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GiteaRepo {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub full_name: Option<String>,
    pub clone_url: Option<String>,
    pub ssh_url: Option<String>,
    pub permissions: Option<GiteaPermissions>,
    pub mirror: Option<bool>,
    pub has_pull_requests: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GiteaPermissions {
    pub admin: Option<bool>,
    pub push: Option<bool>,
    pub pull: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GiteaPr {
    pub number: i64,
    pub title: String,
    pub body: Option<String>,
    pub state: Option<String>,
    pub head: GiteaPrBranch,
    pub base: GiteaPrBranch,
    pub mergeable: Option<bool>,
    pub merged: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub user: Option<GiteaUser>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GiteaPrBranch {
    #[serde(rename = "ref")]
    pub ref_name: String,
    pub label: Option<String>,
    pub repo: Option<GiteaRepo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GiteaComment {
    pub id: Option<i64>,
    pub body: Option<String>,
    pub created_at: Option<String>,
    pub user: Option<GiteaUser>,
}

#[derive(Debug, Serialize)]
struct CreatePrRequest {
    title: String,
    body: String,
    head: String,
    base: String,
}

#[derive(Debug, Serialize)]
struct CommentRequest {
    body: String,
}

pub async fn create_pr(
    client: &GiteaClient,
    owner: &str,
    repo: &str,
    source_branch: &str,
    target_branch: &str,
    title: &str,
    body: &str,
) -> Result<Option<i64>, PlatformError> {
    let url = format!("{}/repos/{owner}/{repo}/pulls", client.api_base);
    let request = CreatePrRequest {
        title: title.to_owned(),
        body: body.to_owned(),
        head: source_branch.to_owned(),
        base: target_branch.to_owned(),
    };
    let request_json = serde_json::to_string(&request)
        .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;

    match client.http.post_json::<GiteaPr>(&url, &request_json).await {
        Ok(pr) => Ok(Some(pr.number)),
        Err(HttpError::Status { status, .. })
            if status == reqwest::StatusCode::CONFLICT
                || status == reqwest::StatusCode::UNPROCESSABLE_ENTITY =>
        {
            Ok(None)
        }
        Err(e) => Err(PlatformError::Http(e)),
    }
}

pub async fn list_prs(
    client: &GiteaClient,
    owner: &str,
    repo: &str,
    state: Option<&str>,
) -> Result<Vec<GiteaPr>, PlatformError> {
    let state_param = state.unwrap_or("open");
    let url = format!(
        "{}/repos/{owner}/{repo}/pulls?state={state_param}&limit=50",
        client.api_base
    );
    client
        .http
        .get_json(&url)
        .await
        .map_err(PlatformError::Http)
}

pub async fn merge_pr(
    client: &GiteaClient,
    owner: &str,
    repo: &str,
    pr_id: i64,
    strategy: Option<&str>,
) -> Result<(), PlatformError> {
    let method = get_merge_method(strategy).unwrap_or("merge");
    let url = format!(
        "{}/repos/{owner}/{repo}/pulls/{pr_id}/merge",
        client.api_base
    );
    #[derive(Serialize)]
    struct MergeRequest {
        r#do: String,
    }
    let request = MergeRequest {
        r#do: method.to_owned(),
    };
    let request_json = serde_json::to_string(&request)
        .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;

    let rb = client
        .http
        .inner
        .post(&url)
        .header("Content-Type", "application/json")
        .body(request_json);
    let rb = match &client.http.token {
        Some(t) => rb.bearer_auth(t),
        None => rb,
    };
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
    client: &GiteaClient,
    owner: &str,
    repo: &str,
    pr_id: i64,
    text: &str,
) -> Result<GiteaComment, PlatformError> {
    let url = format!(
        "{}/repos/{owner}/{repo}/issues/{pr_id}/comments",
        client.api_base
    );
    let request = CommentRequest {
        body: text.to_owned(),
    };
    let request_json = serde_json::to_string(&request)
        .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;

    client
        .http
        .post_json::<GiteaComment>(&url, &request_json)
        .await
        .map_err(PlatformError::Http)
}

pub async fn get_pr(
    client: &GiteaClient,
    owner: &str,
    repo: &str,
    pr_id: i64,
) -> Result<GiteaPr, PlatformError> {
    let url = format!(
        "{}/repos/{owner}/{repo}/pulls/{pr_id}",
        client.api_base
    );
    client
        .http
        .get_json(&url)
        .await
        .map_err(|e| match e {
            HttpError::Status { status, .. } if status == reqwest::StatusCode::NOT_FOUND => {
                PlatformError::Unexpected("PR not found".to_owned())
            }
            other => PlatformError::Http(other),
        })
}

impl PlatformClient for GiteaClient {
    async fn get_current_user(&self) -> Result<CurrentUser, PlatformError> {
        let url = format!("{}/user", self.api_base);
        let user: GiteaUser = self
            .http
            .get_json(&url)
            .await
            .map_err(|e| match e {
                HttpError::Status { status, .. }
                    if status == reqwest::StatusCode::UNAUTHORIZED =>
                {
                    PlatformError::Unauthorized
                }
                other => PlatformError::Http(other),
            })?;
        Ok(CurrentUser { login: user.login })
    }

    async fn get_raw_file(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
    ) -> Result<Option<RawFile>, PlatformError> {
        let url = format!(
            "{}/repos/{owner}/{repo}/contents/{path}?ref=HEAD",
            self.api_base
        );
        let resp = self
            .http
            .get_retrying(&url)
            .await
            .map_err(PlatformError::Http)?;
        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        if !resp.status().is_success() {
            return Err(PlatformError::Http(HttpError::Status {
                status: resp.status(),
                url,
            }));
        }
        let contents: ContentsResponse = resp
            .json()
            .await
            .map_err(HttpError::Request)
            .map_err(PlatformError::Http)?;
        let decoded = contents
            .content
            .map(|c| {
                let stripped: String = c.chars().filter(|ch| !ch.is_whitespace()).collect();
                base64::engine::general_purpose::STANDARD
                    .decode(stripped)
                    .map_err(|e| PlatformError::Unexpected(format!("base64 decode: {e}")))
                    .and_then(|bytes| {
                        String::from_utf8(bytes)
                            .map_err(|e| PlatformError::Unexpected(format!("utf8: {e}")))
                    })
            })
            .transpose()?;
        Ok(Some(RawFile {
            path: path.to_owned(),
            content: decoded.unwrap_or_default(),
        }))
    }

    async fn get_file_list(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<String>, PlatformError> {
        let url = format!(
            "{}/repos/{owner}/{repo}/contents?ref=HEAD",
            self.api_base
        );
        let contents: ContentsListResponse = self
            .http
            .get_json(&url)
            .await
            .map_err(PlatformError::Http)?;
        Ok(contents
            .into_iter()
            .filter_map(|c| {
                if matches!(c.content_type, ContentsType::File) {
                    Some(c.path)
                } else {
                    None
                }
            })
            .collect())
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
            "{}/repos/{owner}/{repo}/pulls/{pr_number}",
            self.api_base
        );
        if title.is_none() && body.is_none() && state.is_none() {
            return Ok(());
        }
        #[derive(Serialize)]
        struct UpdatePrRequest {
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            body: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            state: Option<String>,
        }
        let request = UpdatePrRequest {
            title: title.map(|s| s.to_owned()),
            body: body.map(|s| s.to_owned()),
            state: state.map(|s| s.to_owned()),
        };
        let request_json = serde_json::to_string(&request)
            .map_err(|e| PlatformError::Unexpected(format!("JSON serialize: {e}")))?;
        let resp = self.http.patch_json(&url, &request_json).await?;
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
            "Gitea branch status not yet implemented".to_owned(),
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
            "Gitea write_file not yet implemented".to_owned(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;
    use crate::platform::gitea_forgejo_utils::trim_trailing_api_path;

    fn make_client(server_uri: &str) -> GiteaClient {
        GiteaClient::new(server_uri, "test-token").unwrap()
    }

    // Rust-specific: gitea behavior test
    #[test]
    fn trim_trailing_api_path_strips_api_v1() {
        assert_eq!(
            trim_trailing_api_path("https://gitea.example.com/api/v1"),
            "https://gitea.example.com/"
        );
    }

    #[tokio::test]
    async fn get_current_user_returns_login() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user"))
            .and(header("authorization", "Bearer test-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "login": "gitea-user",
                "id": 1
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let user = client.get_current_user().await.unwrap();
        assert_eq!(user.login, "gitea-user");
    }

    #[tokio::test]
    async fn get_current_user_unauthorized() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let err = client.get_current_user().await.unwrap_err();
        assert!(matches!(err, PlatformError::Unauthorized));
    }

    #[tokio::test]
    async fn create_pr_returns_number() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "number": 7,
                "title": "Test PR",
                "head": {"ref": "feature"},
                "base": {"ref": "main"}
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let pr_id = create_pr(&client, "owner", "repo", "feature", "main", "Test PR", "body")
            .await
            .unwrap();
        assert_eq!(pr_id, Some(7));
    }

    #[tokio::test]
    async fn create_pr_conflict_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(409))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let pr_id = create_pr(&client, "owner", "repo", "feature", "main", "Test PR", "body")
            .await
            .unwrap();
        assert_eq!(pr_id, None);
    }

    #[tokio::test]
    async fn list_prs_returns_values() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/pulls"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"number": 1, "title": "PR 1", "head": {"ref": "f1"}, "base": {"ref": "main"}},
                {"number": 2, "title": "PR 2", "head": {"ref": "f2"}, "base": {"ref": "main"}}
            ])))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let prs = list_prs(&client, "owner", "repo", None).await.unwrap();
        assert_eq!(prs.len(), 2);
        assert_eq!(prs[0].number, 1);
    }

    #[tokio::test]
    async fn merge_pr_succeeds() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/pulls/3/merge"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        merge_pr(&client, "owner", "repo", 3, None).await.unwrap();
    }

    #[tokio::test]
    async fn add_comment_returns_comment() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/issues/1/comments"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "id": 42,
                "body": "hello",
                "user": {"login": "testuser"}
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let comment = add_comment(&client, "owner", "repo", 1, "hello")
            .await
            .unwrap();
        assert_eq!(comment.id, Some(42));
    }

    #[tokio::test]
    async fn get_pr_returns_pr() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/pulls/5"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "number": 5,
                "title": "My PR",
                "state": "open",
                "head": {"ref": "feat"},
                "base": {"ref": "main"}
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let pr = get_pr(&client, "owner", "repo", 5).await.unwrap();
        assert_eq!(pr.number, 5);
        assert_eq!(pr.title, "My PR");
    }

    #[tokio::test]
    async fn get_pr_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/pulls/999"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let err = get_pr(&client, "owner", "repo", 999).await.unwrap_err();
        assert!(matches!(err, PlatformError::Unexpected(_)));
    }

    #[tokio::test]
    async fn get_raw_file_returns_decoded_content() {
        let server = MockServer::start().await;
        let b64 = base64::engine::general_purpose::STANDARD.encode(b"hello gitea");
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/contents/test.txt"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "name": "test.txt",
                "path": "test.txt",
                "type": "file",
                "content": b64
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let file = client
            .get_raw_file("owner", "repo", "test.txt")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(file.content, "hello gitea");
    }

    #[tokio::test]
    async fn get_file_list_returns_files_only() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/contents"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"name": "README.md", "path": "README.md", "type": "file"},
                {"name": "src", "path": "src", "type": "dir"},
                {"name": "main.rs", "path": "main.rs", "type": "file"}
            ])))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let files = client.get_file_list("owner", "repo").await.unwrap();
        assert_eq!(files, vec!["README.md", "main.rs"]);
    }

    #[tokio::test]
    async fn get_raw_file_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/contents/missing.txt"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let file = client.get_raw_file("owner", "repo", "missing.txt").await.unwrap();
        assert!(file.is_none());
    }

    #[tokio::test]
    async fn get_file_list_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/contents"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let result = client.get_file_list("owner", "repo").await;
        assert!(result.is_err());
    }
}
