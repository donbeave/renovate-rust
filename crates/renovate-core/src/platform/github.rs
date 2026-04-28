//! GitHub platform client.
//!
//! Implements [`PlatformClient`] against the GitHub REST API v3.
//!
//! Renovate reference: `lib/modules/platform/github/index.ts`.

use serde::Deserialize;

use crate::http::{HttpClient, HttpError};
use crate::platform::{CurrentUser, PlatformClient, PlatformError};

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
        // wiremock asserts the expected call count on Drop
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
}
