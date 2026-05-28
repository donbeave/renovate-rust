//! Hermit package manager datasource.
//!
//! Fetches available versions for a named package from the Hermit package
//! index hosted at `cashapp/hermit-packages` on GitHub.
//!
//! Renovate reference: `lib/modules/datasource/hermit/index.ts`
//!
//! ## Protocol
//!
//! 1. `GET {api_base}/repos/{owner}/{repo}/releases/tags/index`
//!    → `{ assets: [{ name, url }] }`
//! 2. Find `index.json` asset, fetch its `url`.
//! 3. Parse the resulting `[{ Name, Versions, Channels, Repository }]` array.
//! 4. Find the entry matching `packageName`, return all versions + channels.

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

/// Default Hermit package registry.
pub const DEFAULT_REGISTRY: &str = "https://github.com/cashapp/hermit-packages";

/// GitHub API base used by the production implementation.
const GH_API_BASE: &str = "https://api.github.com";

/// Errors from Hermit datasource lookups.
#[derive(Debug, Error)]
pub enum HermitError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(reqwest::Error),
}

/// A single release entry returned by `fetch_releases`.
#[derive(Debug, Clone, PartialEq)]
pub struct HermitRelease {
    pub version: String,
    pub source_url: Option<String>,
}

/// Full result from `fetch_releases`.
#[derive(Debug, Clone, PartialEq)]
pub struct HermitReleasesResult {
    pub releases: Vec<HermitRelease>,
    pub source_url: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct HermitSearchResult {
    name: String,
    #[serde(default)]
    versions: Vec<String>,
    #[serde(default)]
    channels: Vec<String>,
    repository: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize)]
struct GithubAsset {
    name: String,
    url: String,
}

/// Parse `(owner, repo)` from a GitHub URL with exactly two path segments.
///
/// Returns `None` for non-GitHub URLs, missing segments, or extra path segments.
fn parse_github_repo(url: &str) -> Option<(String, String)> {
    let path = url.strip_prefix("https://github.com/")?;
    let trimmed = path.trim_end_matches('/');
    if trimmed.is_empty() {
        return None;
    }
    let (owner, rest) = trimmed.split_once('/')?;
    // `rest` must not contain a '/' (no extra path segments allowed)
    if rest.contains('/') || rest.is_empty() {
        return None;
    }
    Some((owner.to_owned(), rest.to_owned()))
}

/// Fetch all releases for `package_name` from the Hermit index at `registry_url`.
///
/// `api_base` is the GitHub API base URL (`https://api.github.com` in production).
pub async fn fetch_releases(
    package_name: &str,
    registry_url: Option<&str>,
    http: &HttpClient,
    api_base: &str,
) -> Result<Option<HermitReleasesResult>, HermitError> {
    let Some(registry) = registry_url else {
        return Ok(None);
    };

    let Some((owner, repo)) = parse_github_repo(registry) else {
        return Ok(None);
    };

    let release_url = format!("{api_base}/repos/{owner}/{repo}/releases/tags/index");
    let resp = http.get_retrying(&release_url).await?;
    if !resp.status().is_success() {
        return Ok(None);
    }
    let release: GithubRelease = match resp.json().await {
        Ok(r) => r,
        Err(_) => return Ok(None),
    };

    // Find the index.json asset
    let Some(asset) = release.assets.iter().find(|a| a.name == "index.json") else {
        return Ok(None);
    };

    let index_resp = http.get_retrying(&asset.url).await?;
    if !index_resp.status().is_success() {
        return Err(HermitError::Http(crate::http::HttpError::Status {
            status: index_resp.status(),
            url: asset.url.clone(),
        }));
    }

    // Try to parse JSON; return null for invalid content
    let entries: Vec<HermitSearchResult> = match index_resp.json().await {
        Ok(e) => e,
        Err(_) => return Ok(None),
    };

    // Find the entry for this package
    let Some(entry) = entries
        .iter()
        .find(|e| e.name.eq_ignore_ascii_case(package_name))
    else {
        return Ok(None);
    };

    let source_url = entry.repository.clone();
    let releases: Vec<HermitRelease> = entry
        .versions
        .iter()
        .chain(entry.channels.iter())
        .map(|v| HermitRelease {
            version: v.clone(),
            source_url: source_url.clone(),
        })
        .collect();

    Ok(Some(HermitReleasesResult {
        releases,
        source_url,
    }))
}

/// Compatibility wrapper for ci.rs — fetch latest version via Hermit index.
#[derive(Debug)]
pub struct HermitUpdateSummary {
    pub versions: Vec<String>,
    pub channels: Vec<String>,
    pub latest: Option<String>,
    pub source_url: Option<String>,
    pub update_available: bool,
}

pub async fn fetch_latest(
    package_name: &str,
    current_value: &str,
    registry_url: &str,
    http: &HttpClient,
) -> Result<HermitUpdateSummary, HermitError> {
    let result = fetch_releases(package_name, Some(registry_url), http, GH_API_BASE).await?;
    match result {
        None => Ok(HermitUpdateSummary {
            versions: vec![],
            channels: vec![],
            latest: None,
            source_url: None,
            update_available: false,
        }),
        Some(r) => {
            let latest = r.releases.last().map(|rel| rel.version.clone());
            let update_available = latest.as_deref().is_some_and(|l| l != current_value);
            let (versions, channels): (Vec<_>, Vec<_>) = r
                .releases
                .iter()
                .partition(|rel| !rel.version.starts_with('@'));
            Ok(HermitUpdateSummary {
                versions: versions.into_iter().map(|r| r.version.clone()).collect(),
                channels: channels.into_iter().map(|r| r.version.clone()).collect(),
                latest,
                source_url: r.source_url,
                update_available,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    fn release_with_index(server: &MockServer) -> serde_json::Value {
        let asset_url = format!(
            "{}/repos/cashapp/hermit-packages/releases/assets/38492",
            server.uri()
        );
        serde_json::json!({
            "assets": [
                { "name": "source.tar.gz", "url": format!("{}/repos/cashapp/hermit-packages/releases/assets/99999", server.uri()) },
                { "name": "index.json", "url": asset_url },
            ]
        })
    }

    fn go_entry() -> serde_json::Value {
        serde_json::json!([{
            "Name": "go",
            "Versions": ["1.17.9", "1.17.10", "1.18", "1.18.1"],
            "Channels": ["@1.17", "@1.18"],
            "CurrentVersion": "1.17.9",
            "Repository": "https://github.com/golang/golang",
            "Description": "golang",
        }])
    }

    // Ported: "should return result from hermit list" — datasource/hermit/index.spec.ts line 14
    #[tokio::test]
    async fn should_return_result_from_hermit_list() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/repos/cashapp/hermit-packages/releases/tags/index"))
            .respond_with(ResponseTemplate::new(200).set_body_json(release_with_index(&server)))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/repos/cashapp/hermit-packages/releases/assets/38492"))
            .respond_with(ResponseTemplate::new(200).set_body_json(go_entry()))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(
            "go",
            Some("https://github.com/cashapp/hermit-packages"),
            &http,
            &server.uri(),
        )
        .await
        .unwrap()
        .unwrap();

        assert_eq!(
            result.source_url.as_deref(),
            Some("https://github.com/golang/golang")
        );
        assert_eq!(result.releases.len(), 6);
        let versions: Vec<&str> = result.releases.iter().map(|r| r.version.as_str()).collect();
        assert_eq!(
            versions,
            vec!["1.17.9", "1.17.10", "1.18", "1.18.1", "@1.17", "@1.18"]
        );
        assert!(
            result
                .releases
                .iter()
                .all(|r| r.source_url.as_deref() == Some("https://github.com/golang/golang"))
        );
    }

    // Ported: "should fail on no result found" — datasource/hermit/index.spec.ts line 82
    #[tokio::test]
    async fn should_fail_on_no_result_found() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/repos/cashapp/hermit-packages/releases/tags/index"))
            .respond_with(ResponseTemplate::new(200).set_body_json(release_with_index(&server)))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/repos/cashapp/hermit-packages/releases/assets/38492"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(
            "go",
            Some("https://github.com/cashapp/hermit-packages"),
            &http,
            &server.uri(),
        )
        .await
        .unwrap();
        assert!(result.is_none());
    }

    // Ported: "should fail on network error" — datasource/hermit/index.spec.ts line 109
    #[tokio::test]
    async fn should_fail_on_network_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/repos/cashapp/hermit-packages/releases/tags/index"))
            .respond_with(ResponseTemplate::new(200).set_body_json(release_with_index(&server)))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/repos/cashapp/hermit-packages/releases/assets/38492"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(
            "go",
            Some("https://github.com/cashapp/hermit-packages"),
            &http,
            &server.uri(),
        )
        .await;
        assert!(result.is_err());
    }

    // Ported: "should get null result on non github url given" — datasource/hermit/index.spec.ts line 134
    #[tokio::test]
    async fn should_get_null_on_non_github_url() {
        let http = HttpClient::new().unwrap();
        let result = fetch_releases(
            "go",
            Some("https://gitlab.com/owner/project"),
            &http,
            "https://api.github.com",
        )
        .await
        .unwrap();
        assert!(result.is_none());
    }

    // Ported: "should get null result on missing repo or owner" — datasource/hermit/index.spec.ts line 141
    #[tokio::test]
    async fn should_get_null_on_missing_repo_or_owner() {
        let http = HttpClient::new().unwrap();
        let r1 = fetch_releases("go", Some("https://github.com/test"), &http, "")
            .await
            .unwrap();
        assert!(r1.is_none());
        let r2 = fetch_releases("go", Some("https://github.com/"), &http, "")
            .await
            .unwrap();
        assert!(r2.is_none());
    }

    // Ported: "should get null for extra path provided in registry url" — datasource/hermit/index.spec.ts line 155
    #[tokio::test]
    async fn should_get_null_for_extra_path() {
        let http = HttpClient::new().unwrap();
        let result = fetch_releases(
            "go",
            Some("https://github.com/test/repo/extra-path"),
            &http,
            "",
        )
        .await
        .unwrap();
        assert!(result.is_none());
    }

    // Ported: "should get null result on empty registryUrl" — datasource/hermit/index.spec.ts line 166
    #[tokio::test]
    async fn should_get_null_on_empty_registry_url() {
        let http = HttpClient::new().unwrap();
        let result = fetch_releases("go", None, &http, "").await.unwrap();
        assert!(result.is_none());
    }

    // Ported: "should fail on missing index.json asset" — datasource/hermit/index.spec.ts line 174
    #[tokio::test]
    async fn should_fail_on_missing_index_json() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/repos/cashapp/hermit-packages/releases/tags/index"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "assets": [{ "name": "source.tar.gz", "url": format!("{}/source", server.uri()) }]
            })))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(
            "go",
            Some("https://github.com/cashapp/hermit-packages"),
            &http,
            &server.uri(),
        )
        .await
        .unwrap();
        assert!(result.is_none());
    }

    // Ported: "should get null on invalid index.json asset" — datasource/hermit/index.spec.ts line 196
    #[tokio::test]
    async fn should_get_null_on_invalid_index_json() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/repos/cashapp/hermit-packages/releases/tags/index"))
            .respond_with(ResponseTemplate::new(200).set_body_json(release_with_index(&server)))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/repos/cashapp/hermit-packages/releases/assets/38492"))
            .respond_with(ResponseTemplate::new(200).set_body_string("invalid content"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(
            "go",
            Some("https://github.com/cashapp/hermit-packages"),
            &http,
            &server.uri(),
        )
        .await
        .unwrap();
        assert!(result.is_none());
    }

    // Ported: "should get null on invalid registry url" — datasource/hermit/index.spec.ts line 224
    #[tokio::test]
    async fn should_get_null_on_invalid_registry_url() {
        let http = HttpClient::new().unwrap();
        let result = fetch_releases("go", Some("invalid url"), &http, "")
            .await
            .unwrap();
        assert!(result.is_none());
    }
}
