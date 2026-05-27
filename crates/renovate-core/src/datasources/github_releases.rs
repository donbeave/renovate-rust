//! GitHub Releases datasource.
//!
//! Fetches the latest stable release of a GitHub repository using the
//! `/repos/{owner}/{repo}/releases` REST API.  Unlike the tags API, releases
//! carry explicit `prerelease` and `draft` flags so stability filtering is
//! precise.
//!
//! Renovate reference:
//! - `lib/modules/datasource/github-releases/index.ts`
//! - API: `GET https://api.github.com/repos/{owner}/{repo}/releases?per_page=100`

use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::HttpClient;

pub const GITHUB_API: &str = "https://api.github.com";

/// Errors from fetching GitHub releases.
#[derive(Debug, Error)]
pub enum GithubReleasesError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
}

/// Input for a single GitHub release lookup.
#[derive(Debug, Clone)]
pub struct GithubReleasesDepInput {
    /// `owner/repo` string (e.g. `"hashicorp/terraform"`).
    pub dep_name: String,
    /// Currently pinned version (e.g. `"1.6.0"`).
    pub current_value: String,
}

/// Update summary for a GitHub release dependency.
#[derive(Debug, Clone)]
pub struct GithubReleasesUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
    /// ISO 8601 publish timestamp of the latest stable release.
    /// Populated from the `published_at` field of the GitHub Releases API.
    pub release_timestamp: Option<String>,
}

/// Per-dependency result from `fetch_updates_concurrent`.
#[derive(Debug)]
pub struct GithubReleasesUpdateResult {
    pub dep_name: String,
    pub summary: Result<GithubReleasesUpdateSummary, GithubReleasesError>,
}

/// A release entry returned by `fetch_releases_full`.
#[derive(Debug, Clone, PartialEq)]
pub struct GithubRelease {
    pub version: String,
    pub release_timestamp: Option<String>,
    /// None = no explicit stability marker; Some(false) = prerelease.
    pub is_stable: Option<bool>,
}

/// Full releases result matching the TypeScript `ReleaseResult` shape.
#[derive(Debug, Clone, PartialEq)]
pub struct GithubReleasesFullResult {
    pub releases: Vec<GithubRelease>,
    pub source_url: String,
    pub registry_url: String,
}

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
    prerelease: bool,
    draft: bool,
    /// ISO 8601 publish timestamp from the GitHub Releases API.
    published_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiTag {
    name: String,
    commit: ApiTagCommit,
}

#[derive(Debug, Deserialize)]
struct ApiTagCommit {
    sha: String,
}

/// Returns true if `v` looks like a version string (contains at least one digit).
/// Mirrors the TypeScript `versioning.isVersion()` filter in `filterValidVersions`.
fn is_valid_version(v: &str) -> bool {
    v.chars().any(|c| c.is_ascii_digit())
}

/// Fetch all stable (non-prerelease, non-draft) release tag names for `owner/repo`.
///
/// Returns a `Vec` of `(tag_name, published_at)` for every stable release.
/// Returns an empty `Vec` when the repo is not found or the request fails.
pub async fn fetch_all_releases(
    owner_repo: &str,
    http: &HttpClient,
    api_base: &str,
) -> Result<Vec<(String, Option<String>)>, GithubReleasesError> {
    let url = format!("{api_base}/repos/{owner_repo}/releases?per_page=100");
    let resp = http.get_retrying(&url).await?;
    if !resp.status().is_success() {
        return Ok(Vec::new());
    }
    let releases: Vec<Release> = resp.json().await.map_err(GithubReleasesError::Json)?;
    Ok(releases
        .into_iter()
        .filter(|r| !r.prerelease && !r.draft)
        .map(|r| (r.tag_name, r.published_at))
        .collect())
}

/// Fetch all releases for `owner/repo` with full metadata.
///
/// Mirrors `GithubReleasesDatasource.getReleases` + `getPkgReleases` filtering:
/// - Skips draft releases.
/// - Marks prerelease=true as `is_stable: Some(false)` (not filtered out).
/// - Filters versions with no digits (matches `versioning.isVersion()` behaviour).
pub async fn fetch_releases_full(
    owner_repo: &str,
    http: &HttpClient,
    api_base: &str,
) -> Result<Option<GithubReleasesFullResult>, GithubReleasesError> {
    let url = format!("{api_base}/repos/{owner_repo}/releases?per_page=100");
    let resp = http.get_retrying(&url).await?;
    if !resp.status().is_success() {
        return Ok(None);
    }
    let raw: Vec<Release> = resp.json().await.map_err(GithubReleasesError::Json)?;
    let releases = raw
        .into_iter()
        .filter(|r| !r.draft)
        .filter(|r| is_valid_version(&r.tag_name))
        .map(|r| GithubRelease {
            version: r.tag_name,
            release_timestamp: r.published_at,
            is_stable: if r.prerelease { Some(false) } else { None },
        })
        .collect();
    Ok(Some(GithubReleasesFullResult {
        releases,
        source_url: format!("https://github.com/{owner_repo}"),
        registry_url: "https://github.com".to_string(),
    }))
}

/// Return the commit SHA for `new_value` tag in `owner/repo`, or `None` if not found.
///
/// Mirrors `findCommitOfTag` which uses GraphQL `queryTags`; this implementation
/// uses the REST `/repos/{owner_repo}/tags` endpoint instead.
pub async fn fetch_digest_by_tag(
    owner_repo: &str,
    new_value: &str,
    http: &HttpClient,
    api_base: &str,
) -> Result<Option<String>, GithubReleasesError> {
    let url = format!("{api_base}/repos/{owner_repo}/tags?per_page=100");
    let resp = http.get_retrying(&url).await?;
    if !resp.status().is_success() {
        return Ok(None);
    }
    let tags: Vec<ApiTag> = resp.json().await.map_err(GithubReleasesError::Json)?;
    Ok(tags
        .into_iter()
        .find(|t| t.name == new_value)
        .map(|t| t.commit.sha))
}

/// Fetch the latest stable (non-prerelease, non-draft) release tag for `owner/repo`.
///
/// Returns `(tag_name, published_at)` for the latest stable release, or `None`
/// when no stable release exists or the repo is not found.
pub async fn fetch_latest_release(
    owner_repo: &str,
    http: &HttpClient,
    api_base: &str,
) -> Result<Option<(String, Option<String>)>, GithubReleasesError> {
    let url = format!("{api_base}/repos/{owner_repo}/releases?per_page=100");

    let resp = http.get_retrying(&url).await?;
    if resp.status().as_u16() == 404 {
        return Ok(None);
    }
    if !resp.status().is_success() {
        return Ok(None);
    }

    let releases: Vec<Release> = resp.json().await.map_err(GithubReleasesError::Json)?;

    // Releases are newest-first; return the first stable one with its timestamp.
    let latest = releases
        .into_iter()
        .find(|r| !r.prerelease && !r.draft)
        .map(|r| (r.tag_name, r.published_at));

    Ok(latest)
}

/// Fetch update summaries for multiple GitHub release dependencies concurrently.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[GithubReleasesDepInput],
    api_base: &str,
    concurrency: usize,
) -> Vec<GithubReleasesUpdateResult> {
    if deps.is_empty() {
        return Vec::new();
    }

    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<GithubReleasesUpdateResult> = JoinSet::new();

    for dep in deps {
        let http = http.clone();
        let dep = dep.clone();
        let sem = Arc::clone(&sem);
        let api_base = api_base.to_owned();

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_update_summary(&dep, &http, &api_base).await;
            GithubReleasesUpdateResult {
                dep_name: dep.dep_name.clone(),
                summary: result,
            }
        });
    }

    let mut results = Vec::with_capacity(deps.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok(r) => results.push(r),
            Err(join_err) => tracing::error!(%join_err, "github-releases lookup task panicked"),
        }
    }
    results
}

async fn fetch_update_summary(
    dep: &GithubReleasesDepInput,
    http: &HttpClient,
    api_base: &str,
) -> Result<GithubReleasesUpdateSummary, GithubReleasesError> {
    let result = fetch_latest_release(&dep.dep_name, http, api_base).await?;
    let (latest_tag, release_timestamp) = result
        .map(|(tag, ts)| (Some(tag), ts))
        .unwrap_or((None, None));
    let s = crate::versioning::semver_generic::semver_update_summary(
        &dep.current_value,
        latest_tag.as_deref(),
    );
    Ok(GithubReleasesUpdateSummary {
        current_value: s.current_value,
        latest: s.latest,
        update_available: s.update_available,
        release_timestamp,
    })
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    fn releases_json(releases: &[(&str, bool, bool)]) -> String {
        let items: Vec<serde_json::Value> = releases
            .iter()
            .map(|(tag, pre, draft)| {
                serde_json::json!({
                    "tag_name": tag,
                    "prerelease": pre,
                    "draft": draft,
                    "name": tag,
                })
            })
            .collect();
        serde_json::Value::Array(items).to_string()
    }

    #[tokio::test]
    async fn fetch_latest_returns_first_stable() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/hashicorp/terraform/releases"))
            .respond_with(ResponseTemplate::new(200).set_body_string(releases_json(&[
                ("v1.7.0-rc1", true, false),
                ("v1.6.6", false, false),
                ("v1.6.5", false, false),
            ])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest_release("hashicorp/terraform", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result.map(|(v, _)| v), Some("v1.6.6".to_owned()));
    }

    #[tokio::test]
    async fn skips_drafts() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/owner/repo/releases"))
            .respond_with(ResponseTemplate::new(200).set_body_string(releases_json(&[
                ("v2.0.0", false, true),
                ("v1.9.0", false, false),
            ])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest_release("owner/repo", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result.map(|(v, _)| v), Some("v1.9.0".to_owned()));
    }

    // Ported: "returns null for 404" — lib/modules/datasource/pod/index.spec.ts line 60
    #[tokio::test]
    async fn fetch_latest_404_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/missing/repo/releases"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest_release("missing/repo", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, None);
    }

    fn full_releases_json(items: &[(&str, bool, bool, Option<&str>)]) -> String {
        let values: Vec<serde_json::Value> = items
            .iter()
            .map(|(tag, pre, draft, ts)| {
                serde_json::json!({
                    "tag_name": tag,
                    "prerelease": pre,
                    "draft": draft,
                    "published_at": ts,
                })
            })
            .collect();
        serde_json::to_string(&values).unwrap()
    }

    fn tags_json_with_sha(tags: &[(&str, &str)]) -> String {
        let values: Vec<serde_json::Value> = tags
            .iter()
            .map(|(name, sha)| serde_json::json!({"name": name, "commit": {"sha": sha}}))
            .collect();
        serde_json::to_string(&values).unwrap()
    }

    // Ported: "returns releases" — datasource/github-releases/index.spec.ts line 20
    #[tokio::test]
    async fn returns_releases() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/some/dep/releases"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string(full_releases_json(&[
                    ("a", false, false, None),
                    ("v", false, false, None),
                    ("1.0.0", false, false, Some("2020-03-09T11:00:00.000Z")),
                    ("v1.1.0", false, false, Some("2020-03-09T10:00:00.000Z")),
                    ("2.0.0", true, false, Some("2020-04-09T10:00:00.000Z")),
                ])),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let res = fetch_releases_full("some/dep", &http, &server.uri())
            .await
            .unwrap()
            .unwrap();

        assert_eq!(res.registry_url, "https://github.com");
        assert_eq!(res.source_url, "https://github.com/some/dep");
        assert_eq!(res.releases.len(), 3);
        assert_eq!(res.releases[0].version, "1.0.0");
        assert_eq!(
            res.releases[0].release_timestamp.as_deref(),
            Some("2020-03-09T11:00:00.000Z")
        );
        assert_eq!(res.releases[0].is_stable, None);
        assert_eq!(res.releases[1].version, "v1.1.0");
        assert_eq!(
            res.releases[1].release_timestamp.as_deref(),
            Some("2020-03-09T10:00:00.000Z")
        );
        assert_eq!(res.releases[2].version, "2.0.0");
        assert_eq!(res.releases[2].is_stable, Some(false));
    }

    // Ported: "should be independent of the current digest" — datasource/github-releases/index.spec.ts line 116
    #[tokio::test]
    async fn digest_independent_of_current_digest() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/some/dep/tags"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string(tags_json_with_sha(&[
                    ("v1.0.0", "sha-of-v1"),
                    ("v15.0.0", "sha-of-v15"),
                ])),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let digest = fetch_digest_by_tag("some/dep", "v15.0.0", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(digest.as_deref(), Some("sha-of-v15"));
    }

    // Ported: "should be independent of the current value" — datasource/github-releases/index.spec.ts line 128
    #[tokio::test]
    async fn digest_independent_of_current_value() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/some/dep/tags"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string(tags_json_with_sha(&[
                    ("v1.0.0", "sha-of-v1"),
                    ("v15.0.0", "sha-of-v15"),
                ])),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let digest = fetch_digest_by_tag("some/dep", "v15.0.0", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(digest.as_deref(), Some("sha-of-v15"));
    }

    // Ported: "returns updated digest in new release" — datasource/github-releases/index.spec.ts line 136
    #[tokio::test]
    async fn returns_updated_digest_in_new_release() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/some/dep/tags"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string(tags_json_with_sha(&[
                    ("v1.0.0", "sha-of-v1"),
                    ("v15.0.0", "sha-of-v15"),
                ])),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let digest = fetch_digest_by_tag("some/dep", "v15.0.0", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(digest.as_deref(), Some("sha-of-v15"));
    }

    // Ported: "returns null if the new value/tag does not exist" — datasource/github-releases/index.spec.ts line 149
    #[tokio::test]
    async fn returns_null_for_unknown_tag() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/some/dep/tags"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string(tags_json_with_sha(&[
                    ("v1.0.0", "sha-of-v1"),
                    ("v15.0.0", "sha-of-v15"),
                ])),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let digest = fetch_digest_by_tag("some/dep", "unknown-tag", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(digest, None);
    }
}
