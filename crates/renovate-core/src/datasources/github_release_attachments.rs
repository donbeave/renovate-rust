//! GitHub Release Attachments datasource.
//!
//! Fetches release assets from the GitHub Releases API for a specific tag.
//!
//! Renovate reference: `lib/modules/datasource/github-release-attachments/index.ts`
//! API: `GET https://api.github.com/repos/{owner}/{repo}/releases/tags/{tag}`

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const GITHUB_API: &str = "https://api.github.com";
pub const DATASOURCE_ID: &str = "github-release-attachments";

#[derive(Debug, Error)]
pub enum GithubReleaseAttachmentsError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
}

/// A single release asset from the GitHub Releases API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseAsset {
    pub name: String,
    pub url: String,
    pub size: u64,
    pub content_type: Option<String>,
    pub download_count: u64,
}

#[derive(Debug, Deserialize)]
struct ApiAsset {
    name: String,
    url: String,
    size: u64,
    content_type: Option<String>,
    download_count: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct ApiRelease {
    tag_name: String,
    name: Option<String>,
    prerelease: bool,
    #[allow(dead_code)]
    draft: bool,
    published_at: Option<String>,
    assets: Vec<ApiAsset>,
}

/// Full release info including assets and metadata.
#[derive(Debug, Clone)]
pub struct GithubReleaseWithAssets {
    pub tag_name: String,
    pub name: Option<String>,
    pub prerelease: bool,
    pub published_at: Option<String>,
    pub assets: Vec<ReleaseAsset>,
}

/// Fetch release assets for a specific tag from the GitHub Releases API.
///
/// Returns `Ok(None)` for 404 or other non-success responses.
/// Returns `Err` for server errors (5xx).
pub async fn fetch_release_assets(
    http: &HttpClient,
    repo: &str,
    tag: &str,
    api_base: &str,
) -> Result<Option<GithubReleaseWithAssets>, GithubReleaseAttachmentsError> {
    let url = format!("{api_base}/repos/{repo}/releases/tags/{tag}");

    let resp = http.get_retrying(&url).await?;
    let status = resp.status();

    if status.as_u16() == 404 {
        return Ok(None);
    }
    if status.is_server_error() {
        return Err(GithubReleaseAttachmentsError::Http(
            crate::http::HttpError::Status { status, url },
        ));
    }
    if !status.is_success() {
        return Ok(None);
    }

    let release: ApiRelease = resp.json().await?;

    let assets = release
        .assets
        .into_iter()
        .map(|a| ReleaseAsset {
            name: a.name,
            url: a.url,
            size: a.size,
            content_type: a.content_type,
            download_count: a.download_count.unwrap_or(0),
        })
        .collect();

    Ok(Some(GithubReleaseWithAssets {
        tag_name: release.tag_name,
        name: release.name,
        prerelease: release.prerelease,
        published_at: release.published_at,
        assets,
    }))
}

/// Fetch assets for all releases of a repository.
///
/// Returns releases ordered newest-first, with their attached assets.
pub async fn fetch_all_release_assets(
    http: &HttpClient,
    repo: &str,
    api_base: &str,
) -> Result<Vec<GithubReleaseWithAssets>, GithubReleaseAttachmentsError> {
    let url = format!("{api_base}/repos/{repo}/releases?per_page=100");

    let resp = http.get_retrying(&url).await?;
    let status = resp.status();

    if status.as_u16() == 404 {
        return Ok(Vec::new());
    }
    if !status.is_success() {
        return Ok(Vec::new());
    }

    let releases: Vec<ApiRelease> = resp.json().await?;

    let result = releases
        .into_iter()
        .map(|release| {
            let assets = release
                .assets
                .into_iter()
                .map(|a| ReleaseAsset {
                    name: a.name,
                    url: a.url,
                    size: a.size,
                    content_type: a.content_type,
                    download_count: a.download_count.unwrap_or(0),
                })
                .collect();
            GithubReleaseWithAssets {
                tag_name: release.tag_name,
                name: release.name,
                prerelease: release.prerelease,
                published_at: release.published_at,
                assets,
            }
        })
        .collect();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    fn release_json(tag: &str, assets: &[(&str, &str, u64)]) -> String {
        let asset_items: Vec<serde_json::Value> = assets
            .iter()
            .map(|(name, url, size)| {
                serde_json::json!({
                    "name": name,
                    "url": url,
                    "size": size,
                    "content_type": "application/octet-stream",
                    "download_count": 42
                })
            })
            .collect();
        serde_json::json!({
            "tag_name": tag,
            "name": tag,
            "prerelease": false,
            "draft": false,
            "published_at": "2024-01-15T10:00:00Z",
            "assets": asset_items
        })
        .to_string()
    }

    #[tokio::test]
    async fn fetch_release_assets_returns_assets() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/releases/tags/v1.0.0"))
            .respond_with(ResponseTemplate::new(200).set_body_string(release_json(
                "v1.0.0",
                &[
                    ("binary.tar.gz", "https://api.github.com/assets/1", 1024),
                    ("checksums.txt", "https://api.github.com/assets/2", 256),
                ],
            )))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_release_assets(&http, "owner/repo", "v1.0.0", &server.uri())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.tag_name, "v1.0.0");
        assert_eq!(result.assets.len(), 2);
        assert_eq!(result.assets[0].name, "binary.tar.gz");
        assert_eq!(result.assets[0].size, 1024);
        assert_eq!(result.assets[1].name, "checksums.txt");
        assert_eq!(result.assets[1].download_count, 42);
        assert_eq!(
            result.published_at.as_deref(),
            Some("2024-01-15T10:00:00Z")
        );
    }

    #[tokio::test]
    async fn fetch_release_assets_404_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/releases/tags/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_release_assets(&http, "owner/repo", "nonexistent", &server.uri())
            .await
            .unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn fetch_release_assets_5xx_returns_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/releases/tags/v1.0.0"))
            .respond_with(ResponseTemplate::new(502))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_release_assets(&http, "owner/repo", "v1.0.0", &server.uri()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn fetch_release_assets_empty_assets_list() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/releases/tags/v2.0.0"))
            .respond_with(ResponseTemplate::new(200).set_body_string(release_json(
                "v2.0.0",
                &[],
            )))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_release_assets(&http, "owner/repo", "v2.0.0", &server.uri())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(result.assets.len(), 0);
    }

    #[tokio::test]
    async fn fetch_all_release_assets_returns_all() {
        let server = MockServer::start().await;
        let releases = serde_json::json!([
            {
                "tag_name": "v2.0.0",
                "name": "v2.0.0",
                "prerelease": false,
                "draft": false,
                "published_at": "2024-02-01T00:00:00Z",
                "assets": [{"name": "bin.tar.gz", "url": "https://example.com/1", "size": 100, "download_count": 10}]
            },
            {
                "tag_name": "v1.0.0",
                "name": "v1.0.0",
                "prerelease": false,
                "draft": false,
                "published_at": "2024-01-01T00:00:00Z",
                "assets": [{"name": "bin.tar.gz", "url": "https://example.com/2", "size": 50, "download_count": 5}]
            }
        ]);
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/releases"))
            .respond_with(ResponseTemplate::new(200).set_body_json(releases))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let results = fetch_all_release_assets(&http, "owner/repo", &server.uri())
            .await
            .unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].tag_name, "v2.0.0");
        assert_eq!(results[1].tag_name, "v1.0.0");
    }

    #[tokio::test]
    async fn fetch_all_release_assets_404_returns_empty() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/missing/repo/releases"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let results = fetch_all_release_assets(&http, "missing/repo", &server.uri())
            .await
            .unwrap();
        assert!(results.is_empty());
    }

    // Rust-specific: github_release_attachments behavior test
    #[test]
    fn datasource_id_is_correct() {
        assert_eq!(DATASOURCE_ID, "github-release-attachments");
    }
}
