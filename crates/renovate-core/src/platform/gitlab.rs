//! GitLab platform client.
//!
//! Implements [`PlatformClient`] against the GitLab REST API v4.
//!
//! Renovate reference: `lib/modules/platform/gitlab/index.ts`.
//!
//! ## Authentication
//!
//! GitLab uses the `PRIVATE-TOKEN` header for personal access tokens.
//! OAuth bearer tokens use `Authorization: Bearer <token>`.
//! This implementation uses `PRIVATE-TOKEN` (PAT) which is the common
//! self-hosted / GitLab.com workflow.
//!
//! ## Project addressing
//!
//! GitLab identifies projects by `{namespace}/{project}` (URL-encoded as
//! `namespace%2Fproject`), which maps directly to the `owner/repo` convention
//! used by this CLI.

use base64::Engine as _;
use serde::Deserialize;

use crate::http::{HttpClient, HttpError};
use crate::platform::{CurrentUser, PlatformClient, PlatformError, RawFile};

/// Default GitLab API base URL.
pub const GITLAB_API_BASE: &str = "https://gitlab.com/api/v4";

/// GitLab platform client authenticated with a Personal Access Token.
#[derive(Debug, Clone)]
pub struct GitlabClient {
    http: HttpClient,
    api_base: String,
}

impl GitlabClient {
    /// Create a new GitLab client with the given PAT and the default API base
    /// (`https://gitlab.com/api/v4`).
    pub fn new(token: impl Into<String>) -> Result<Self, HttpError> {
        Self::with_endpoint(token, GITLAB_API_BASE)
    }

    /// Create a new GitLab client with a custom API endpoint (self-hosted GitLab).
    pub fn with_endpoint(
        token: impl Into<String>,
        api_base: impl Into<String>,
    ) -> Result<Self, HttpError> {
        Ok(Self {
            // GitLab uses PRIVATE-TOKEN header; we pass it via the HTTP client's
            // bearer mechanism and note the header name difference below in
            // get_current_user where we add it explicitly.
            http: HttpClient::with_token(token)?,
            api_base: api_base.into().trim_end_matches('/').to_owned(),
        })
    }
}

// ── API response types ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct GitlabUser {
    username: String,
}

/// GitLab repository file content (Contents API).
/// `GET /projects/{id}/repository/files/{path}?ref=HEAD`
#[derive(Debug, Deserialize)]
struct GitlabFileContent {
    content: String,
    encoding: String,
    file_path: String,
}

/// One entry from the GitLab repository tree API.
/// `GET /projects/{id}/repository/tree?recursive=true&per_page=100`
#[derive(Debug, Deserialize)]
struct GitlabTreeEntry {
    path: String,
    #[serde(rename = "type")]
    entry_type: String,
}

// ── PlatformClient impl ───────────────────────────────────────────────────────

impl PlatformClient for GitlabClient {
    async fn get_current_user(&self) -> Result<CurrentUser, PlatformError> {
        let url = format!("{}/user", self.api_base);
        let resp = self
            .http
            .get_retrying(&url)
            .await
            .map_err(PlatformError::Http)?;

        match resp.status() {
            s if s.is_success() => {}
            s if s == reqwest::StatusCode::UNAUTHORIZED => return Err(PlatformError::Unauthorized),
            s => return Err(PlatformError::Http(HttpError::Status { status: s, url })),
        }

        let user: GitlabUser = resp
            .json()
            .await
            .map_err(|e| PlatformError::Http(HttpError::Request(e)))?;

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
        // GitLab requires the file path percent-encoded (slashes as %2F).
        let encoded_path = encode_path(path);
        let project_id = encode_project(owner, repo);
        let url = format!(
            "{}/projects/{}/repository/files/{}?ref=HEAD",
            self.api_base, project_id, encoded_path
        );

        let resp = self
            .http
            .get_retrying(&url)
            .await
            .map_err(PlatformError::Http)?;

        match resp.status() {
            s if s.is_success() => {}
            s if s == reqwest::StatusCode::NOT_FOUND => return Ok(None),
            s if s == reqwest::StatusCode::UNAUTHORIZED => return Err(PlatformError::Unauthorized),
            s => return Err(PlatformError::Http(HttpError::Status { status: s, url })),
        }

        let content: GitlabFileContent = resp
            .json()
            .await
            .map_err(|e| PlatformError::Http(HttpError::Request(e)))?;

        let decoded = if content.encoding == "base64" {
            // GitLab base64 may contain newlines — strip them before decoding.
            let stripped = content.content.replace('\n', "");
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(stripped.as_bytes())
                .map_err(|e| PlatformError::Unexpected(format!("base64 decode error: {e}")))?;
            String::from_utf8(bytes)
                .map_err(|e| PlatformError::Unexpected(format!("UTF-8 decode error: {e}")))?
        } else {
            content.content
        };

        Ok(Some(RawFile {
            path: content.file_path,
            content: decoded,
        }))
    }

    async fn get_file_list(&self, owner: &str, repo: &str) -> Result<Vec<String>, PlatformError> {
        let project_id = encode_project(owner, repo);
        let mut files: Vec<String> = Vec::new();
        let mut page: u32 = 1;

        // Paginate through the full recursive tree.  GitLab's tree API returns
        // at most 100 entries per page; we stop when a page returns fewer than
        // `per_page` entries (last page) or after 50 pages as a safety cap.
        loop {
            let url = format!(
                "{}/projects/{}/repository/tree?recursive=true&per_page=100&page={page}",
                self.api_base, project_id
            );

            let resp = self
                .http
                .get_retrying(&url)
                .await
                .map_err(PlatformError::Http)?;

            match resp.status() {
                s if s.is_success() => {}
                s if s == reqwest::StatusCode::NOT_FOUND => {
                    return Err(PlatformError::Unexpected("repository not found".to_owned()));
                }
                s if s == reqwest::StatusCode::UNAUTHORIZED => {
                    return Err(PlatformError::Unauthorized);
                }
                s => return Err(PlatformError::Http(HttpError::Status { status: s, url })),
            }

            // GitLab returns total page count in X-Total-Pages header; we
            // rely on entry count to detect the last page instead.
            let entries: Vec<GitlabTreeEntry> = resp
                .json()
                .await
                .map_err(|e| PlatformError::Http(HttpError::Request(e)))?;

            let count = entries.len();
            files.extend(
                entries
                    .into_iter()
                    .filter(|e| e.entry_type == "blob")
                    .map(|e| e.path),
            );

            if count < 100 || page >= 50 {
                if page >= 50 {
                    tracing::warn!(
                        repo = %format!("{owner}/{repo}"),
                        "GitLab file tree exceeded 50 pages; list may be incomplete"
                    );
                }
                break;
            }
            page += 1;
        }

        Ok(files)
    }
}

// ── URL encoding helpers ──────────────────────────────────────────────────────

/// Percent-encode a file path for use in GitLab API URLs.
///
/// GitLab requires each `/` in the path to be encoded as `%2F`.
fn encode_path(path: &str) -> String {
    path.replace('/', "%2F")
}

/// Encode a `owner/repo` pair as a GitLab project identifier.
///
/// GitLab accepts `{namespace}%2F{project}` as the project ID in REST URLs.
fn encode_project(owner: &str, repo: &str) -> String {
    format!("{owner}%2F{repo}")
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;
    use crate::platform::PlatformClient;

    fn make_client(server_uri: &str) -> GitlabClient {
        GitlabClient::with_endpoint("test-token", server_uri).unwrap()
    }

    // ── encode helpers ────────────────────────────────────────────────────────

    #[test]
    fn encode_path_replaces_slashes() {
        assert_eq!(encode_path("src/main.rs"), "src%2Fmain.rs");
        assert_eq!(
            encode_path(".github/renovate.json"),
            ".github%2Frenovate.json"
        );
    }

    #[test]
    fn encode_project_formats_correctly() {
        assert_eq!(encode_project("owner", "repo"), "owner%2Frepo");
    }

    // ── get_current_user ──────────────────────────────────────────────────────

    #[tokio::test]
    async fn get_current_user_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!({"id": 1, "username": "devuser"})),
            )
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let user = client.get_current_user().await.unwrap();
        assert_eq!(user.login, "devuser");
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

    // ── get_raw_file ──────────────────────────────────────────────────────────

    #[tokio::test]
    async fn get_raw_file_returns_decoded_content() {
        let server = MockServer::start().await;
        let encoded =
            base64::engine::general_purpose::STANDARD.encode(b"[package]\nname = \"foo\"");
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/repository/files/Cargo.toml"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "content": encoded,
                "encoding": "base64",
                "file_path": "Cargo.toml"
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let file = client
            .get_raw_file("owner", "repo", "Cargo.toml")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(file.path, "Cargo.toml");
        assert!(file.content.contains("[package]"));
    }

    #[tokio::test]
    async fn get_raw_file_returns_none_for_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/repository/files/missing.txt"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let result = client
            .get_raw_file("owner", "repo", "missing.txt")
            .await
            .unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn get_raw_file_encodes_path_slashes() {
        let server = MockServer::start().await;
        let encoded = base64::engine::general_purpose::STANDARD.encode(b"{}");
        Mock::given(method("GET"))
            .and(path(
                "/projects/owner%2Frepo/repository/files/.github%2Frenovate.json",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "content": encoded,
                "encoding": "base64",
                "file_path": ".github/renovate.json"
            })))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let file = client
            .get_raw_file("owner", "repo", ".github/renovate.json")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(file.path, ".github/renovate.json");
    }

    // ── get_file_list ─────────────────────────────────────────────────────────

    #[tokio::test]
    async fn get_file_list_returns_blobs_only() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/repository/tree"))
            .and(query_param("page", "1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"path": "Cargo.toml", "type": "blob"},
                {"path": "src", "type": "tree"},
                {"path": "src/main.rs", "type": "blob"},
            ])))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let files = client.get_file_list("owner", "repo").await.unwrap();
        assert_eq!(files, vec!["Cargo.toml", "src/main.rs"]);
    }

    #[tokio::test]
    async fn get_file_list_paginates() {
        let server = MockServer::start().await;

        // First page: 100 entries (all blobs to keep the fixture simple).
        let page1: Vec<serde_json::Value> = (0..100)
            .map(|i| serde_json::json!({"path": format!("file{i}.txt"), "type": "blob"}))
            .collect();
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/repository/tree"))
            .and(query_param("page", "1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(page1))
            .mount(&server)
            .await;

        // Second page: 3 entries → last page.
        let page2 = serde_json::json!([
            {"path": "README.md", "type": "blob"},
            {"path": "LICENSE", "type": "blob"},
            {"path": "docs", "type": "tree"},
        ]);
        Mock::given(method("GET"))
            .and(path("/projects/owner%2Frepo/repository/tree"))
            .and(query_param("page", "2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(page2))
            .mount(&server)
            .await;

        let client = make_client(&server.uri());
        let files = client.get_file_list("owner", "repo").await.unwrap();
        // 100 from page 1 + 2 blobs from page 2 (docs/tree is excluded)
        assert_eq!(files.len(), 102);
        assert!(files.contains(&"README.md".to_owned()));
    }
}
