//! GitHub platform client.
//!
//! Implements [`PlatformClient`] against the GitHub REST API v3.
//!
//! Renovate reference: `lib/modules/platform/github/index.ts`.

use base64::Engine as _;
use serde::Deserialize;

use crate::http::{HttpClient, HttpError};
use crate::platform::{CurrentUser, PlatformClient, PlatformError, RawFile};

/// Default GitHub API base URL.
pub const GITHUB_API_BASE: &str = "https://api.github.com";

/// GitHub platform client. Authenticated with a personal access token or
/// GitHub App installation token.
#[derive(Debug, Clone)]
pub struct GithubClient {
    http: HttpClient,
    api_base: String,
}

impl GithubClient {
    /// Create a new GitHub client with the given token and the default API
    /// base URL (`https://api.github.com`).
    pub fn new(token: impl Into<String>) -> Result<Self, HttpError> {
        Self::with_endpoint(token, GITHUB_API_BASE)
    }

    /// Create a new GitHub client with a custom API endpoint (GitHub
    /// Enterprise Server).
    pub fn with_endpoint(
        token: impl Into<String>,
        api_base: impl Into<String>,
    ) -> Result<Self, HttpError> {
        Ok(Self {
            http: HttpClient::with_token(token)?,
            api_base: api_base.into().trim_end_matches('/').to_owned(),
        })
    }
}

/// Minimal GitHub user response.
#[derive(Debug, Deserialize)]
struct GithubUser {
    login: String,
}

/// GitHub Contents API response for a file.
#[derive(Debug, Deserialize)]
struct GithubContent {
    content: Option<String>,
    encoding: Option<String>,
}

/// GitHub Git Trees API response.
#[derive(Debug, Deserialize)]
struct GithubTree {
    tree: Vec<GithubTreeEntry>,
    truncated: bool,
}

#[derive(Debug, Deserialize)]
struct GithubTreeEntry {
    path: Option<String>,
    #[serde(rename = "type")]
    entry_type: Option<String>,
}

impl PlatformClient for GithubClient {
    async fn get_current_user(&self) -> Result<CurrentUser, PlatformError> {
        let url = format!("{}/user", self.api_base);
        let user: GithubUser = self.http.get_json(&url).await.map_err(|e| match e {
            HttpError::Status { status, .. } if status == reqwest::StatusCode::UNAUTHORIZED => {
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
            "{}/repos/{}/{}/contents/{}",
            self.api_base, owner, repo, path
        );
        let result: Result<GithubContent, _> = self.http.get_json(&url).await;
        match result {
            Ok(content) => {
                let raw = decode_github_content(content)?;
                Ok(Some(RawFile {
                    path: path.to_owned(),
                    content: raw,
                }))
            }
            Err(HttpError::Status { status, .. }) if status == reqwest::StatusCode::NOT_FOUND => {
                Ok(None)
            }
            Err(e) => Err(PlatformError::Http(e)),
        }
    }

    async fn get_file_list(&self, owner: &str, repo: &str) -> Result<Vec<String>, PlatformError> {
        // Use HEAD as the tree reference; GitHub resolves it to the default branch.
        let url = format!(
            "{}/repos/{}/{}/git/trees/HEAD?recursive=1",
            self.api_base, owner, repo
        );
        let tree: GithubTree = self
            .http
            .get_json(&url)
            .await
            .map_err(PlatformError::Http)?;
        if tree.truncated {
            tracing::warn!(
                repo = %format!("{owner}/{repo}"),
                "file tree truncated — some managers may not be detected"
            );
        }
        let files = tree
            .tree
            .into_iter()
            .filter(|e| e.entry_type.as_deref() == Some("blob"))
            .filter_map(|e| e.path)
            .collect();
        Ok(files)
    }
}

fn decode_github_content(c: GithubContent) -> Result<String, PlatformError> {
    let raw_content = c.content.unwrap_or_default();
    match c.encoding.as_deref() {
        Some("base64") | None => {
            // GitHub wraps lines at 60 chars; strip whitespace before decoding.
            let stripped: String = raw_content
                .chars()
                .filter(|ch| !ch.is_whitespace())
                .collect();
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(stripped)
                .map_err(|e| PlatformError::Unexpected(format!("base64 decode: {e}")))?;
            String::from_utf8(bytes)
                .map_err(|e| PlatformError::Unexpected(format!("utf8 decode: {e}")))
        }
        Some(enc) => Err(PlatformError::Unexpected(format!(
            "unsupported encoding: {enc}"
        ))),
    }
}

// ── GitHub URL utilities ──────────────────────────────────────────────────────

const DEFAULT_SOURCE_URL_BASE: &str = "https://github.com/";
const DEFAULT_API_BASE_URL: &str = "https://api.github.com/";

/// Return the source URL base for the given registry URL, ensuring a trailing slash.
///
/// Mirrors `lib/util/github/url.ts` `getSourceUrlBase()`.
pub fn get_source_url_base(registry_url: Option<&str>) -> String {
    let base = registry_url.unwrap_or(DEFAULT_SOURCE_URL_BASE);
    if base.ends_with('/') {
        base.to_owned()
    } else {
        format!("{base}/")
    }
}

/// Return the GitHub REST API v3 base URL for the given registry URL.
///
/// Mirrors `lib/util/github/url.ts` `getApiBaseUrl()`.
pub fn get_api_base_url(registry_url: Option<&str>) -> String {
    let source_base = get_source_url_base(registry_url);
    if source_base == DEFAULT_SOURCE_URL_BASE || source_base == DEFAULT_API_BASE_URL {
        return DEFAULT_API_BASE_URL.to_owned();
    }
    if source_base.ends_with("/api/v3/") {
        return source_base;
    }
    format!("{source_base}api/v3/")
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{header, header_exists, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    #[tokio::test]
    async fn get_current_user_returns_login() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user"))
            .and(header_exists("authorization"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!({"login": "renovate-bot", "id": 1})),
            )
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("test-token", server.uri()).unwrap();
        let user = client.get_current_user().await.unwrap();
        assert_eq!(user.login, "renovate-bot");
    }

    #[tokio::test]
    async fn get_current_user_returns_unauthorized_on_401() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("bad-token", server.uri()).unwrap();
        let err = client.get_current_user().await.unwrap_err();
        assert!(matches!(err, PlatformError::Unauthorized));
    }

    #[tokio::test]
    async fn get_current_user_sends_bearer_auth_header() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user"))
            .and(header("authorization", "Bearer my-secret-token"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"login": "user"})),
            )
            .expect(1)
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("my-secret-token", server.uri()).unwrap();
        client.get_current_user().await.unwrap();
    }

    #[tokio::test]
    async fn github_enterprise_custom_endpoint() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/user"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"login": "ghe-user"})),
            )
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let user = client.get_current_user().await.unwrap();
        assert_eq!(user.login, "ghe-user");
    }

    #[tokio::test]
    async fn get_raw_file_returns_decoded_content() {
        let server = MockServer::start().await;
        // Base64 of '{"extends":["config:recommended"]}'
        let b64 = base64::engine::general_purpose::STANDARD
            .encode(r#"{"extends":["config:recommended"]}"#);
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/contents/renovate.json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "content": b64,
                "encoding": "base64"
            })))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let file = client
            .get_raw_file("owner", "repo", "renovate.json")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(file.path, "renovate.json");
        assert!(file.content.contains("config:recommended"));
    }

    // Ported: "ensures trailing slash" — util/github/url.spec.ts line 6
    #[test]
    fn github_get_source_url_base_trailing_slash() {
        assert_eq!(
            get_source_url_base(Some("https://gh.my-company.com")),
            "https://gh.my-company.com/"
        );
    }

    // Ported: "defaults to github.com" — util/github/url.spec.ts line 11
    #[test]
    fn github_get_source_url_base_default() {
        assert_eq!(get_source_url_base(None), "https://github.com/");
    }

    // Ported: "maps to api.github.com" — util/github/url.spec.ts line 17
    #[test]
    fn github_get_api_base_url_maps_to_api() {
        assert_eq!(
            get_api_base_url(Some("https://github.com/")),
            "https://api.github.com/"
        );
    }

    // Ported: "supports local github installations" — util/github/url.spec.ts line 21
    #[test]
    fn github_get_api_base_url_local_install() {
        assert_eq!(
            get_api_base_url(Some("https://gh.my-company.com/")),
            "https://gh.my-company.com/api/v3/"
        );
        assert_eq!(
            get_api_base_url(Some("https://gh.my-company.com/api/v3/")),
            "https://gh.my-company.com/api/v3/"
        );
    }

    #[tokio::test]
    async fn get_raw_file_returns_none_on_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/contents/renovate.json"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = GithubClient::with_endpoint("token", server.uri()).unwrap();
        let result = client
            .get_raw_file("owner", "repo", "renovate.json")
            .await
            .unwrap();
        assert!(result.is_none());
    }
}
