//! GitHub Digest datasource.
//!
//! Fetches commit digests (SHA hashes) for GitHub repositories at specific refs.
//!
//! Renovate reference: `lib/modules/datasource/github-digest/index.ts`
//! API: `GET https://api.github.com/repos/{owner}/{repo}/commits/{ref}`

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const GITHUB_API: &str = "https://api.github.com";
pub const DATASOURCE_ID: &str = "github-digest";

#[derive(Debug, Error)]
pub enum GithubDigestError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
}

#[derive(Debug, Deserialize)]
struct CommitObject {
    sha: String,
}

#[derive(Debug, Deserialize)]
struct CommitResponse {
    sha: String,
    #[serde(rename = "commit")]
    _commit: serde_json::Value,
}

/// Fetch the commit digest (SHA) for a specific ref or tag in a GitHub repo.
///
/// Returns `Ok(None)` for 404 responses or when the ref is not found.
/// Returns `Err` for server errors (5xx).
pub async fn fetch_digest(
    http: &HttpClient,
    repo: &str,
    ref_or_tag: &str,
    api_base: &str,
) -> Result<Option<String>, GithubDigestError> {
    let url = format!("{api_base}/repos/{repo}/commits/{ref_or_tag}");

    let resp = http.get_retrying(&url).await?;
    let status = resp.status();

    if status.as_u16() == 404 {
        return Ok(None);
    }
    if status.is_client_error() {
        return Ok(None);
    }
    if status.is_server_error() {
        return Err(GithubDigestError::Http(crate::http::HttpError::Status {
            status,
            url,
        }));
    }

    let body: CommitResponse = resp.json().await?;
    Ok(Some(body.sha))
}

/// Fetch the commit digest for a branch HEAD using the branches API.
///
/// Useful when the commits API returns a 404 for branch names.
pub async fn fetch_digest_from_branch(
    http: &HttpClient,
    repo: &str,
    branch: &str,
    api_base: &str,
) -> Result<Option<String>, GithubDigestError> {
    let url = format!("{api_base}/repos/{repo}/branches/{branch}");

    let resp = http.get_retrying(&url).await?;
    let status = resp.status();

    if status.as_u16() == 404 {
        return Ok(None);
    }
    if !status.is_success() {
        return Ok(None);
    }

    #[derive(Deserialize)]
    struct BranchResponse {
        commit: CommitObject,
    }

    let body: BranchResponse = resp.json().await?;
    Ok(Some(body.commit.sha))
}

/// Fetch the commit digest for a tag using the tags API.
///
/// Returns the SHA of the tag object (not the underlying commit).
pub async fn fetch_digest_from_tag(
    http: &HttpClient,
    repo: &str,
    tag: &str,
    api_base: &str,
) -> Result<Option<String>, GithubDigestError> {
    let url = format!("{api_base}/repos/{repo}/git/refs/tags/{tag}");

    let resp = http.get_retrying(&url).await?;
    let status = resp.status();

    if status.as_u16() == 404 {
        return Ok(None);
    }
    if !status.is_success() {
        return Ok(None);
    }

    #[derive(Deserialize)]
    struct RefObject {
        sha: String,
    }
    #[derive(Deserialize)]
    struct RefResponse {
        object: RefObject,
    }

    let body: RefResponse = resp.json().await?;
    Ok(Some(body.object.sha))
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    #[tokio::test]
    async fn fetch_digest_returns_sha() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/commits/main"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "sha": "abc123def456",
                "commit": {"message": "test"}
            })))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let digest = fetch_digest(&http, "owner/repo", "main", &server.uri())
            .await
            .unwrap();
        assert_eq!(digest.as_deref(), Some("abc123def456"));
    }

    #[tokio::test]
    async fn fetch_digest_404_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/commits/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let digest = fetch_digest(&http, "owner/repo", "nonexistent", &server.uri())
            .await
            .unwrap();
        assert!(digest.is_none());
    }

    #[tokio::test]
    async fn fetch_digest_5xx_returns_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/commits/main"))
            .respond_with(ResponseTemplate::new(502))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_digest(&http, "owner/repo", "main", &server.uri()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn fetch_digest_from_branch_returns_sha() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/branches/develop"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "commit": {"sha": "branch-sha-123"}
            })))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let digest = fetch_digest_from_branch(&http, "owner/repo", "develop", &server.uri())
            .await
            .unwrap();
        assert_eq!(digest.as_deref(), Some("branch-sha-123"));
    }

    #[tokio::test]
    async fn fetch_digest_from_branch_404_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/branches/missing"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let digest = fetch_digest_from_branch(&http, "owner/repo", "missing", &server.uri())
            .await
            .unwrap();
        assert!(digest.is_none());
    }

    #[tokio::test]
    async fn fetch_digest_from_tag_returns_sha() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/git/refs/tags/v1.0.0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "object": {"sha": "tag-sha-456"}
            })))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let digest = fetch_digest_from_tag(&http, "owner/repo", "v1.0.0", &server.uri())
            .await
            .unwrap();
        assert_eq!(digest.as_deref(), Some("tag-sha-456"));
    }

    #[tokio::test]
    async fn fetch_digest_from_tag_404_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/git/refs/tags/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let digest = fetch_digest_from_tag(&http, "owner/repo", "nonexistent", &server.uri())
            .await
            .unwrap();
        assert!(digest.is_none());
    }

    // Rust-specific: github_digest behavior test
    #[test]
    fn datasource_id_is_correct() {
        assert_eq!(DATASOURCE_ID, "github-digest");
    }
}
