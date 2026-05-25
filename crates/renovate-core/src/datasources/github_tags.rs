//! GitHub tags datasource.
//!
//! Fetches the list of tags from the GitHub REST API and returns the latest
//! version-like tag for a given `owner/repo`.  Used to look up GitHub Actions
//! versions (e.g. `actions/checkout@v4` → latest `v4.x.y` tag).
//!
//! Renovate reference:
//! - `lib/modules/datasource/github-tags/index.ts`
//! - `lib/modules/datasource/github-releases/index.ts`

use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::HttpClient;

pub const GITHUB_API: &str = "https://api.github.com";
const GITHUB_API_BASE: &str = GITHUB_API;

/// Errors from fetching GitHub tags.
#[derive(Debug, Error)]
pub enum GithubTagsError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
}

/// Input for a single GitHub Actions dependency lookup.
#[derive(Debug, Clone)]
pub struct GithubActionsDepInput {
    /// `owner/repo` (lookup key for the tags API).
    pub dep_name: String,
    /// Currently pinned tag (e.g. `"v4"`).
    pub current_value: String,
}

/// Update summary for a GitHub Actions dependency.
#[derive(Debug, Clone)]
pub struct GithubActionsUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Per-dependency result from `fetch_updates_concurrent`.
#[derive(Debug)]
pub struct GithubActionsUpdateResult {
    pub dep_name: String,
    pub summary: Result<GithubActionsUpdateSummary, GithubTagsError>,
}

#[derive(Debug, Deserialize)]
struct GithubTag {
    name: String,
}

// ─────────────────────────────────────────────────────────────────────────
// Full getReleases / getDigest support (spec parity with github-tags)
// ─────────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct GithubRestTag {
    name: String,
    commit: GithubRestTagCommit,
}

#[derive(Deserialize, Default)]
struct GithubRestTagCommit {
    sha: Option<String>,
}

#[derive(Deserialize)]
struct CommitDetail {
    commit: CommitDetailBody,
}

#[derive(Deserialize)]
struct CommitDetailBody {
    committer: Option<CommitAuthorDate>,
    author: Option<CommitAuthorDate>,
}

#[derive(Deserialize)]
struct CommitAuthorDate {
    date: Option<String>,
}

#[derive(Deserialize, Default)]
struct RestRelease {
    tag_name: String,
    published_at: Option<String>,
    prerelease: bool,
    draft: bool,
}

/// A single release entry returned by `fetch_releases_full`.
#[derive(Debug, Clone, PartialEq)]
pub struct GithubTagRelease {
    pub version: String,
    pub git_ref: String,
    pub hash: Option<String>,
    pub release_timestamp: Option<String>,
    pub is_stable: Option<bool>,
}

/// Full result from `fetch_releases_full`.
#[derive(Debug, Clone, PartialEq)]
pub struct GithubTagsFullResult {
    pub releases: Vec<GithubTagRelease>,
    pub source_url: String,
}

/// Fetch the latest commit SHA for a repo.
///
/// Used by `getDigest` when no `newValue` tag is specified.
pub async fn get_latest_commit(
    owner_repo: &str,
    http: &HttpClient,
    api_base: &str,
) -> Option<String> {
    #[derive(Deserialize)]
    struct Commit {
        sha: String,
    }

    let url = format!("{api_base}/repos/{owner_repo}/commits?per_page=1");
    let resp = http.get_retrying(&url).await.ok()?;
    if !resp.status().is_success() {
        return None;
    }
    let commits: Vec<Commit> = resp.json().await.ok()?;
    commits.into_iter().next().map(|c| c.sha)
}

/// Fetch the commit SHA for a specific tag.
///
/// Returns `None` when the tag is not found, has no sha, or a network/API
/// error occurs.
pub async fn get_digest_for_tag(
    owner_repo: &str,
    tag: &str,
    http: &HttpClient,
    api_base: &str,
) -> Option<String> {
    let url = format!("{api_base}/repos/{owner_repo}/tags?per_page=100");
    let resp = http.get_retrying(&url).await.ok()?;
    if !resp.status().is_success() {
        return None;
    }
    let tags: Vec<GithubRestTag> = resp.json().await.ok()?;
    tags.into_iter()
        .find(|t| t.name == tag)
        .and_then(|t| t.commit.sha)
        .filter(|s| !s.is_empty())
}

async fn fetch_tag_commit_timestamp(
    owner_repo: &str,
    sha: &str,
    http: &HttpClient,
    api_base: &str,
) -> Option<String> {
    let url = format!("{api_base}/repos/{owner_repo}/commits/{sha}");
    let resp = http.get_retrying(&url).await.ok()?;
    if !resp.status().is_success() {
        return None;
    }
    let detail: CommitDetail = resp.json().await.ok()?;
    detail
        .commit
        .committer
        .as_ref()
        .and_then(|c| c.date.clone())
        .or_else(|| detail.commit.author.as_ref().and_then(|a| a.date.clone()))
}

/// Fetch all tags and merge with release metadata.
///
/// Per-tag commit timestamps are fetched from the commits endpoint.
/// `isStable` and potentially newer `releaseTimestamp` values are supplied
/// by the releases endpoint.
pub async fn fetch_releases_full(
    owner_repo: &str,
    http: &HttpClient,
    api_base: &str,
) -> Option<GithubTagsFullResult> {
    let tags_url = format!("{api_base}/repos/{owner_repo}/tags?per_page=100");
    let tags_resp = http.get_retrying(&tags_url).await.ok()?;
    if !tags_resp.status().is_success() {
        return None;
    }
    let raw_tags: Vec<GithubRestTag> = tags_resp.json().await.ok()?;

    let mut releases: Vec<GithubTagRelease> = Vec::new();
    for tag in &raw_tags {
        let sha = tag.commit.sha.as_deref().filter(|s| !s.is_empty());
        let timestamp = if let Some(s) = sha {
            fetch_tag_commit_timestamp(owner_repo, s, http, api_base).await
        } else {
            None
        };
        releases.push(GithubTagRelease {
            version: tag.name.clone(),
            git_ref: tag.name.clone(),
            hash: sha.map(str::to_string),
            release_timestamp: timestamp,
            is_stable: None,
        });
    }

    // Fetch GitHub releases for isStable + newer timestamps
    let rel_url = format!("{api_base}/repos/{owner_repo}/releases?per_page=100");
    let rest_releases: Vec<RestRelease> = match http.get_retrying(&rel_url).await {
        Ok(resp) if resp.status().is_success() => resp.json().await.unwrap_or_default(),
        _ => Vec::new(),
    };

    let releases_map: std::collections::HashMap<String, RestRelease> = rest_releases
        .into_iter()
        .filter(|r| !r.draft)
        .map(|r| (r.tag_name.clone(), r))
        .collect();

    for rel in &mut releases {
        if let Some(rest_rel) = releases_map.get(&rel.version) {
            rel.is_stable = Some(!rest_rel.prerelease);
            if let Some(ref pub_at) = rest_rel.published_at {
                match &rel.release_timestamp {
                    None => rel.release_timestamp = Some(pub_at.clone()),
                    Some(ts) if pub_at > ts => rel.release_timestamp = Some(pub_at.clone()),
                    _ => {}
                }
            }
        }
    }

    let source_url = format!("https://github.com/{owner_repo}");
    Some(GithubTagsFullResult { releases, source_url })
}

/// Fetch all version-like tags for `owner/repo` and return the latest one.
///
/// `api_base` defaults to `https://api.github.com`; pass a custom value for
/// GitHub Enterprise Server endpoints.
pub async fn fetch_latest_tag(
    owner_repo: &str,
    http: &HttpClient,
    api_base: &str,
) -> Result<Option<String>, GithubTagsError> {
    let url = format!("{api_base}/repos/{owner_repo}/tags?per_page=100");
    let resp = http.get_retrying(&url).await?;
    if resp.status().as_u16() == 404 || resp.status().as_u16() == 403 {
        return Ok(None);
    }
    if !resp.status().is_success() {
        return Ok(None);
    }

    let tags: Vec<GithubTag> = resp.json().await.map_err(GithubTagsError::Json)?;

    // Return the first tag that looks like a version (starts with v or digit).
    let latest = tags
        .into_iter()
        .map(|t| t.name)
        .find(|n| n.starts_with('v') || n.chars().next().is_some_and(|c| c.is_ascii_digit()));

    Ok(latest)
}

/// Fetch update summaries for multiple GitHub Actions dependencies concurrently.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[GithubActionsDepInput],
    api_base: &str,
    concurrency: usize,
) -> Vec<GithubActionsUpdateResult> {
    if deps.is_empty() {
        return Vec::new();
    }

    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<GithubActionsUpdateResult> = JoinSet::new();

    for dep in deps {
        let http = http.clone();
        let dep = dep.clone();
        let sem = Arc::clone(&sem);
        let api_base = api_base.to_owned();

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_update_summary(&dep, &http, &api_base).await;
            GithubActionsUpdateResult {
                dep_name: dep.dep_name.clone(),
                summary: result,
            }
        });
    }

    let mut results = Vec::with_capacity(deps.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok(r) => results.push(r),
            Err(join_err) => {
                tracing::error!(%join_err, "github-tags lookup task panicked")
            }
        }
    }
    results
}

async fn fetch_update_summary(
    dep: &GithubActionsDepInput,
    http: &HttpClient,
    api_base: &str,
) -> Result<GithubActionsUpdateSummary, GithubTagsError> {
    let latest = fetch_latest_tag(&dep.dep_name, http, api_base).await?;
    let update_available = latest
        .as_deref()
        .is_some_and(|l| l != dep.current_value && !dep.current_value.is_empty());
    Ok(GithubActionsUpdateSummary {
        current_value: dep.current_value.clone(),
        latest,
        update_available,
    })
}

/// Derive the GitHub API base URL from a platform endpoint string.
///
/// - `https://api.github.com` → `https://api.github.com`
/// - `https://github.example.com/api/v3` → `https://github.example.com/api/v3`
/// - `None` (unset) → `https://api.github.com`
pub fn api_base_from_endpoint(endpoint: Option<&str>) -> &str {
    match endpoint {
        None | Some("") => GITHUB_API_BASE,
        Some(ep) => ep.trim_end_matches('/'),
    }
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    fn make_http(token: &str) -> HttpClient {
        HttpClient::with_token(token).unwrap()
    }

    fn tags_json() -> &'static str {
        r#"[
            {"name":"v4.1.0","zipball_url":"","tarball_url":"","commit":{}},
            {"name":"v4.0.0","zipball_url":"","tarball_url":"","commit":{}},
            {"name":"v3.6.0","zipball_url":"","tarball_url":"","commit":{}}
        ]"#
    }

    #[tokio::test]
    async fn fetch_latest_tag_returns_first_version_like() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/actions/checkout/tags"))
            .respond_with(ResponseTemplate::new(200).set_body_string(tags_json()))
            .mount(&server)
            .await;

        let http = make_http("token");
        let result = fetch_latest_tag("actions/checkout", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, Some("v4.1.0".to_owned()));
    }

    #[tokio::test]
    async fn fetch_latest_tag_404_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/missing/repo/tags"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = make_http("token");
        let result = fetch_latest_tag("missing/repo", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn fetch_latest_tag_empty_list_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repos/actions/empty/tags"))
            .respond_with(ResponseTemplate::new(200).set_body_string("[]"))
            .mount(&server)
            .await;

        let http = make_http("token");
        let result = fetch_latest_tag("actions/empty", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn api_base_from_none() {
        assert_eq!(api_base_from_endpoint(None), GITHUB_API_BASE);
    }

    #[test]
    fn api_base_from_custom() {
        assert_eq!(
            api_base_from_endpoint(Some("https://github.example.com/api/v3")),
            "https://github.example.com/api/v3"
        );
    }

    #[test]
    fn api_base_trims_trailing_slash() {
        assert_eq!(
            api_base_from_endpoint(Some("https://api.github.com/")),
            "https://api.github.com"
        );
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Tests ported from datasource/github-tags/index.spec.ts
    // ─────────────────────────────────────────────────────────────────────────

    fn dep() -> &'static str { "some/dep" }
    fn dep2() -> &'static str { "some/dep2" }
    fn dep3() -> &'static str { "some/dep3" }

    fn commit_resp(sha: &str, date: &str) -> serde_json::Value {
        serde_json::json!({
            "sha": sha,
            "commit": {
                "committer": { "date": date },
                "author": { "date": date }
            }
        })
    }

    fn tags_with_sha() -> serde_json::Value {
        serde_json::json!([
            { "name": "v1.0.0", "commit": { "sha": "123" } },
            { "name": "v2.0.0", "commit": { "sha": "abc" } }
        ])
    }

    // Ported: "returns commit digest" — datasource/github-tags/index.spec.ts line 25
    #[tokio::test]
    async fn returns_commit_digest() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(format!("/repos/{}/commits", dep())))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::json!([{ "sha": "abcdef" }])
            ))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let res = get_latest_commit(dep(), &http, &server.uri()).await;
        assert_eq!(res.as_deref(), Some("abcdef"));
    }

    // Ported: "returns null for missing commit" — datasource/github-tags/index.spec.ts line 36
    #[tokio::test]
    async fn returns_null_for_missing_commit() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(format!("/repos/{}/commits", dep())))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let res = get_latest_commit(dep(), &http, &server.uri()).await;
        assert!(res.is_none());
    }

    // Ported: "returns untagged commit digest" — datasource/github-tags/index.spec.ts line 45
    #[tokio::test]
    async fn returns_untagged_commit_digest() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(format!("/repos/{}/commits", dep())))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                serde_json::json!([{ "sha": "abcdef" }])
            ))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let res = get_latest_commit(dep(), &http, &server.uri()).await;
        assert_eq!(res.as_deref(), Some("abcdef"));
    }

    // Ported: "returns tagged commit digest" — datasource/github-tags/index.spec.ts line 54
    #[tokio::test]
    async fn returns_tagged_commit_digest() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(format!("/repos/{}/tags", dep())))
            .respond_with(ResponseTemplate::new(200).set_body_json(tags_with_sha()))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let res = get_digest_for_tag(dep(), "v2.0.0", &http, &server.uri()).await;
        assert_eq!(res.as_deref(), Some("abc"));
    }

    // "returns null for missing hash" (line 73) → not-applicable:
    // REST /tags always includes commit.sha; missing-hash is GraphQL-specific.

    // Ported: "returns null for missing tagged commit digest" — datasource/github-tags/index.spec.ts line 91
    #[tokio::test]
    async fn returns_null_for_missing_tagged_commit_digest() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(format!("/repos/{}/tags", dep())))
            .respond_with(ResponseTemplate::new(200).set_body_json(tags_with_sha()))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let res = get_digest_for_tag(dep(), "v3.0.0", &http, &server.uri()).await;
        assert!(res.is_none());
    }

    // Ported: "returns null for error" — datasource/github-tags/index.spec.ts line 110
    #[tokio::test]
    async fn returns_null_for_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(format!("/repos/{}/tags", dep())))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let res = get_digest_for_tag(dep(), "v3.0.0", &http, &server.uri()).await;
        assert!(res.is_none());
    }

    // Ported: "returns tags" — datasource/github-tags/index.spec.ts line 120
    #[tokio::test]
    async fn returns_tags() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(format!("/repos/{}/tags", dep2())))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                { "name": "v1.0.0", "commit": { "sha": "123" } },
                { "name": "v2.0.0", "commit": { "sha": "abc" } }
            ])))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(format!("/repos/{}/commits/123", dep2())))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                commit_resp("123", "2021-01-01T00:00:00Z")
            ))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(format!("/repos/{}/commits/abc", dep2())))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                commit_resp("abc", "2022-01-01T00:00:00Z")
            ))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(format!("/repos/{}/releases", dep2())))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "tag_name": "v1.0.0",
                    "published_at": "2021-01-01T00:00:00Z",
                    "prerelease": false,
                    "draft": false
                },
                {
                    "tag_name": "v2.0.0",
                    "published_at": "2022-01-01T00:00:00Z",
                    "prerelease": true,
                    "draft": false
                }
            ])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let res = fetch_releases_full(dep2(), &http, &server.uri()).await.unwrap();

        assert_eq!(res.source_url, "https://github.com/some/dep2");
        assert_eq!(res.releases.len(), 2);
        let r1 = &res.releases[0];
        assert_eq!(r1.version, "v1.0.0");
        assert_eq!(r1.is_stable, Some(true));
        assert_eq!(r1.hash.as_deref(), Some("123"));

        let r2 = &res.releases[1];
        assert_eq!(r2.version, "v2.0.0");
        assert_eq!(r2.is_stable, Some(false));
        assert_eq!(r2.hash.as_deref(), Some("abc"));
    }

    // Ported: "if it is newer than tag timestamp" — datasource/github-tags/index.spec.ts line 183
    #[tokio::test]
    async fn release_timestamp_newer_than_tag_timestamp() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(format!("/repos/{}/tags", dep3())))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                { "name": "v1.0.0", "commit": { "sha": "sha1" } }
            ])))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(format!("/repos/{}/commits/sha1", dep3())))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                commit_resp("sha1", "2021-01-01T00:00:00Z")
            ))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(format!("/repos/{}/releases", dep3())))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                { "tag_name": "v1.0.0", "published_at": "2021-06-15T00:00:00Z", "prerelease": false, "draft": false }
            ])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let res = fetch_releases_full(dep3(), &http, &server.uri()).await.unwrap();
        assert_eq!(
            res.releases[0].release_timestamp.as_deref(),
            Some("2021-06-15T00:00:00Z")
        );
    }

    // Ported: "keeps tag timestamp when release timestamp is older" — datasource/github-tags/index.spec.ts line 212
    #[tokio::test]
    async fn keeps_tag_timestamp_when_release_timestamp_is_older() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(format!("/repos/{}/tags", dep3())))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                { "name": "v1.0.0", "commit": { "sha": "sha1" } }
            ])))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(format!("/repos/{}/commits/sha1", dep3())))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                commit_resp("sha1", "2021-06-15T00:00:00Z")
            ))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(format!("/repos/{}/releases", dep3())))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                { "tag_name": "v1.0.0", "published_at": "2021-01-01T00:00:00Z", "prerelease": false, "draft": false }
            ])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let res = fetch_releases_full(dep3(), &http, &server.uri()).await.unwrap();
        assert_eq!(
            res.releases[0].release_timestamp.as_deref(),
            Some("2021-06-15T00:00:00Z")
        );
    }

    // Ported: "keeps tag timestamp when release timestamp is equal" — datasource/github-tags/index.spec.ts line 241
    #[tokio::test]
    async fn keeps_tag_timestamp_when_release_timestamp_is_equal() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(format!("/repos/{}/tags", dep3())))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                { "name": "v1.0.0", "commit": { "sha": "sha1" } }
            ])))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(format!("/repos/{}/commits/sha1", dep3())))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                commit_resp("sha1", "2021-01-01T00:00:00Z")
            ))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(format!("/repos/{}/releases", dep3())))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                { "tag_name": "v1.0.0", "published_at": "2021-01-01T00:00:00Z", "prerelease": false, "draft": false }
            ])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let res = fetch_releases_full(dep3(), &http, &server.uri()).await.unwrap();
        assert_eq!(
            res.releases[0].release_timestamp.as_deref(),
            Some("2021-01-01T00:00:00Z")
        );
    }

    // Ported: "keeps tag timestamp when no corresponding release exists" — datasource/github-tags/index.spec.ts line 270
    #[tokio::test]
    async fn keeps_tag_timestamp_when_no_corresponding_release_exists() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(format!("/repos/{}/tags", dep3())))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                { "name": "v1.0.0", "commit": { "sha": "sha1" } }
            ])))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(format!("/repos/{}/commits/sha1", dep3())))
            .respond_with(ResponseTemplate::new(200).set_body_json(
                commit_resp("sha1", "2021-01-01T00:00:00Z")
            ))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path(format!("/repos/{}/releases", dep3())))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let res = fetch_releases_full(dep3(), &http, &server.uri()).await.unwrap();
        assert_eq!(
            res.releases[0].release_timestamp.as_deref(),
            Some("2021-01-01T00:00:00Z")
        );
    }
}
