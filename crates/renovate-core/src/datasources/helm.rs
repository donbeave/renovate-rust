//! Helm repository datasource.
//!
//! Fetches chart versions from a Helm repository's `index.yaml` manifest.
//!
//! Renovate reference:
//! - `lib/modules/datasource/helm/index.ts` — `HelmDatasource`
//! - Helm repo index spec: https://helm.sh/docs/helm/helm_repo_index/
//!
//! ## Repository Index Format
//!
//! Each Helm repository serves an `index.yaml` at its root URL:
//!
//! ```yaml
//! apiVersion: v1
//! entries:
//!   redis:
//!     - name: redis
//!       version: 17.0.0
//!       created: "2024-01-01T00:00:00Z"
//!       digest: abc123
//!       home: https://redis.io
//!       sources:
//!         - https://github.com/redis/redis
//! ```

use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::HttpClient;

/// Errors from fetching Helm repository metadata.
#[derive(Debug, Error)]
pub enum HelmError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("failed to fetch index.yaml from {0}")]
    IndexFetch(String),
    #[error("external host error")]
    ExternalHost,
}

/// Input for a single Helm chart lookup.
#[derive(Debug, Clone)]
pub struct HelmDepInput {
    pub name: String,
    pub current_value: String,
    pub repository_url: String,
}

/// Update summary for a Helm chart dependency.
#[derive(Debug, Clone)]
pub struct HelmUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
    /// ISO 8601 timestamp from the `created` field of the chart entry in `index.yaml`.
    pub release_timestamp: Option<String>,
}

/// Per-dependency result from `fetch_updates_concurrent`.
#[derive(Debug)]
pub struct HelmUpdateResult {
    pub name: String,
    pub summary: Result<HelmUpdateSummary, HelmError>,
}

/// A single chart release entry from `fetch_releases`.
#[derive(Debug, Clone, PartialEq)]
pub struct HelmRelease {
    pub version: String,
    pub release_timestamp: Option<String>,
    pub new_digest: Option<String>,
}

/// Full result from `fetch_releases`.
#[derive(Debug, Clone, PartialEq)]
pub struct HelmReleasesResult {
    pub releases: Vec<HelmRelease>,
    pub homepage: Option<String>,
    pub source_url: Option<String>,
    pub registry_url: String,
}

// ── YAML schema types ──────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct IndexYaml {
    entries: std::collections::HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Deserialize)]
struct RawChartEntry {
    version: Option<String>,
    created: Option<String>,
    digest: Option<String>,
    home: Option<String>,
    #[serde(default)]
    sources: Vec<String>,
    #[serde(default)]
    urls: Vec<String>,
}

// ── Source URL extraction (mirrors schema.ts getSourceUrl) ───────────────────

fn is_possible_chart_repo(url: &str) -> bool {
    let url_lower = url.to_lowercase();
    // Must be a known git-hosting platform (mirrors TypeScript detectPlatform).
    let is_git_host = url_lower.contains("github")
        || url_lower.contains("gitlab")
        || url_lower.contains("bitbucket");
    if !is_git_host {
        return false;
    }
    // Extract the repository name, mirroring TypeScript `parseGitUrl(url).name`.
    //
    // For GitHub-style URLs: `/{owner}/{repo}/...` → repo is 2nd path segment.
    // For GitLab-style URLs: `/{group[/subgroup]}/{repo}/-/...` → repo is the
    //   segment immediately before `/-/`.
    let path_after_host = url_lower
        .find("://")
        .and_then(|i| {
            url_lower[i + 3..]
                .find('/')
                .map(|j| &url_lower[i + 3 + j + 1..])
        })
        .unwrap_or("");

    let repo_name = if url_lower.contains("/-/") {
        // GitLab: last segment before `/-/`
        url_lower
            .split("/-/")
            .next()
            .and_then(|s| s.rsplit('/').find(|seg| !seg.is_empty()))
            .unwrap_or("")
    } else {
        // GitHub / Bitbucket: 2nd path segment (after owner) = repo name
        let after_owner = path_after_host
            .find('/')
            .map(|i| &path_after_host[i + 1..])
            .unwrap_or("");
        let repo = after_owner
            .find('/')
            .map(|i| &after_owner[..i])
            .unwrap_or(after_owner);
        repo.strip_suffix(".git").unwrap_or(repo)
    };
    // Match: chart/charts/helm/helm-charts — mirrors /charts?|helm|helm-charts/i
    repo_name.contains("chart") || repo_name.contains("helm")
}

fn get_source_url(entry: &RawChartEntry) -> Option<String> {
    // Check if the first URL is a GitHub release URL
    if let Some(first_url) = entry.urls.first()
        && let Some(captures) = {
            // match https://github.com/{owner}/{repo}/releases/
            let prefix = "https://github.com/";
            if let Some(rest) = first_url.strip_prefix(prefix) {
                let parts: Vec<&str> = rest.splitn(4, '/').collect();
                if parts.len() >= 3 && parts[2] == "releases" {
                    Some(format!("{prefix}{}/{}", parts[0], parts[1]))
                } else {
                    None
                }
            } else {
                None
            }
        }
    {
        return Some(captures);
    }

    // Check home URL
    if let Some(home) = &entry.home
        && is_possible_chart_repo(home)
    {
        return Some(home.clone());
    }

    // Check sources
    for source in &entry.sources {
        if is_possible_chart_repo(source) {
            return Some(source.clone());
        }
    }

    // Fallback: first source
    entry.sources.first().cloned()
}

// ── Parse functions ───────────────────────────────────────────────────────────

/// Parse all versions for `chart_name` from index.yaml text.
///
/// Returns `None` if the chart is not found or has no valid versions.
pub fn parse_all_versions(index_yaml: &str, chart_name: &str) -> Option<HelmReleasesResult> {
    parse_all_versions_with_registry(index_yaml, chart_name, "")
}

pub fn parse_all_versions_with_registry(
    index_yaml: &str,
    chart_name: &str,
    registry_url: &str,
) -> Option<HelmReleasesResult> {
    if index_yaml.trim().is_empty() {
        return None;
    }

    let parsed: IndexYaml = serde_yaml::from_str(index_yaml).ok()?;
    let chart_entries = parsed.entries.get(chart_name)?;

    // Deserialize the chart entry list
    let raw_entries: Vec<RawChartEntry> = serde_yaml::from_value(chart_entries.clone()).ok()?;

    // Filter to entries with a valid version
    let valid: Vec<&RawChartEntry> = raw_entries.iter().filter(|e| e.version.is_some()).collect();
    if valid.is_empty() {
        return None;
    }

    let first = valid[0];
    let homepage = first.home.clone();
    let source_url = get_source_url(first);

    let releases = valid
        .iter()
        .map(|e| HelmRelease {
            version: e.version.clone().unwrap(),
            release_timestamp: e.created.clone().and_then(|s| normalize_timestamp(&s)),
            new_digest: e.digest.clone().filter(|d| !d.is_empty()),
        })
        .collect();

    Some(HelmReleasesResult {
        releases,
        homepage,
        source_url,
        registry_url: registry_url.to_owned(),
    })
}

/// Normalize a Helm `created` timestamp to ISO 8601.
///
/// Helm uses `2019-06-02T08:56:36.116031089Z` (with nanoseconds) which
/// we trim to millisecond precision.
fn normalize_timestamp(s: &str) -> Option<String> {
    if s.is_empty() {
        return None;
    }
    // Already ISO 8601 - just trim nano/pico-second precision beyond 3 digits
    // e.g. "2019-06-02T08:56:36.116031089Z" → "2019-06-02T08:56:36.116Z"
    if let Some(dot_pos) = s.find('.') {
        let (before_dot, after_dot) = s.split_at(dot_pos);
        // after_dot starts with '.'
        let rest = &after_dot[1..]; // digits + 'Z' or timezone
        let (digits, suffix) = rest
            .find(|c: char| !c.is_ascii_digit())
            .map(|i| rest.split_at(i))
            .unwrap_or((rest, ""));
        let millis = &digits[..digits.len().min(3)];
        let padded = format!("{millis:0<3}");
        return Some(format!("{before_dot}.{padded}{suffix}"));
    }
    Some(s.to_owned())
}

/// Parse the latest version and its `created` timestamp from a Helm
/// `index.yaml` for the given `chart_name`.
pub fn parse_latest_version(
    index_yaml: &str,
    chart_name: &str,
) -> Option<(String, Option<String>)> {
    parse_all_versions(index_yaml, chart_name)
        .and_then(|r| r.releases.into_iter().next())
        .map(|rel| (rel.version, rel.release_timestamp))
}

/// Fetch all releases of a Helm chart from a repository's `index.yaml`.
///
/// - 4xx → `Ok(None)`
/// - 5xx → `Err(HelmError::ExternalHost)`
/// - network error → `Ok(None)` (fallback: true semantics)
/// - chart not found → `Ok(None)`
pub async fn fetch_releases(
    chart_name: &str,
    repository_url: &str,
    http: &HttpClient,
) -> Result<Option<HelmReleasesResult>, HelmError> {
    let base = repository_url.trim_end_matches('/');
    let url = format!("{base}/index.yaml");

    let Ok(resp) = http.get(&url).send().await else {
        return Ok(None);
    };

    let status = resp.status();
    if status.is_server_error() {
        return Err(HelmError::ExternalHost);
    }
    if !status.is_success() {
        return Ok(None);
    }

    let Ok(text) = resp.text().await else {
        return Ok(None);
    };

    Ok(parse_all_versions_with_registry(
        &text,
        chart_name,
        repository_url,
    ))
}

/// Fetch the latest version of a Helm chart from a repository's `index.yaml`.
///
/// Returns `(version, created_at)` or `Err` if the fetch/parse fails.
pub async fn fetch_latest(
    chart_name: &str,
    repository_url: &str,
    http: &HttpClient,
) -> Result<Option<(String, Option<String>)>, HelmError> {
    let base = repository_url.trim_end_matches('/');
    let url = format!("{base}/index.yaml");

    let resp = http.get_retrying(&url).await?;
    if !resp.status().is_success() {
        return Err(HelmError::IndexFetch(url));
    }

    let text = resp
        .text()
        .await
        .map_err(|_| HelmError::IndexFetch(repository_url.to_owned()))?;

    Ok(parse_latest_version(&text, chart_name))
}

/// Fetch update summaries for multiple Helm chart deps concurrently.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[HelmDepInput],
    concurrency: usize,
) -> Vec<HelmUpdateResult> {
    if deps.is_empty() {
        return Vec::new();
    }

    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<HelmUpdateResult> = JoinSet::new();

    for dep in deps {
        let http = http.clone();
        let dep = dep.clone();
        let sem = Arc::clone(&sem);

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_update_summary(&dep, &http).await;
            HelmUpdateResult {
                name: dep.name.clone(),
                summary: result,
            }
        });
    }

    let mut results = Vec::with_capacity(deps.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok(r) => results.push(r),
            Err(join_err) => tracing::error!(%join_err, "helm lookup task panicked"),
        }
    }
    results
}

async fn fetch_update_summary(
    dep: &HelmDepInput,
    http: &HttpClient,
) -> Result<HelmUpdateSummary, HelmError> {
    let result = fetch_latest(&dep.name, &dep.repository_url, http).await?;
    let (latest, release_timestamp) = result.map(|(v, ts)| (Some(v), ts)).unwrap_or((None, None));
    let current = strip_constraint_operators(&dep.current_value);
    let update_available = latest
        .as_deref()
        .is_some_and(|l| !current.is_empty() && l != current);
    Ok(HelmUpdateSummary {
        current_value: dep.current_value.clone(),
        latest,
        update_available,
        release_timestamp,
    })
}

fn strip_constraint_operators(constraint: &str) -> &str {
    constraint
        .trim()
        .trim_start_matches(['~', '>', '<', '=', '!', ' ', '^'])
        .split(',')
        .next()
        .unwrap_or("")
        .trim()
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    const INDEX_YAML: &str =
        include_str!("../../../../../renovate/lib/modules/datasource/helm/__fixtures__/index.yaml");
    const INDEX_EMPTY_PACKAGE_YAML: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/helm/__fixtures__/index_emptypackage.yaml"
    );
    const INDEX_BLANK_DIGEST_YAML: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/helm/__fixtures__/index_blank-digest.yaml"
    );

    fn make_index_yaml(charts: &[(&str, &[&str])]) -> String {
        let mut out = "apiVersion: v1\nentries:\n".to_owned();
        for (name, versions) in charts {
            out.push_str(&format!("  {name}:\n"));
            for v in *versions {
                out.push_str(&format!("  - name: {name}\n    version: {v}\n"));
            }
        }
        out
    }

    const SAMPLE_YAML: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/helm/__fixtures__/sample.yaml"
    );

    // Ported: "works" — lib/modules/datasource/helm/schema.spec.ts line 7
    #[test]
    fn schema_source_url_extraction() {
        let cases: &[(&str, Option<&str>, Option<&str>)] = &[
            (
                "airflow",
                Some("https://github.com/bitnami/charts/tree/master/bitnami/airflow"),
                Some("https://github.com/bitnami/charts/tree/master/bitnami/airflow"),
            ),
            (
                "coredns",
                Some("https://coredns.io"),
                Some("https://github.com/coredns/helm"),
            ),
            (
                "pgadmin4",
                Some("https://www.pgadmin.org/"),
                Some("https://github.com/rowanruseler/helm-charts"),
            ),
            (
                "private-chart-github",
                Some("https://github.example.com/some-org/charts/tree/master/private-chart"),
                Some("https://github.example.com/some-org/charts/tree/master/private-chart"),
            ),
            (
                "private-chart-gitlab",
                Some("https://gitlab.example.com/some/group/charts/-/tree/master/private-chart"),
                Some("https://gitlab.example.com/some/group/charts/-/tree/master/private-chart"),
            ),
        ];
        for (chart_name, expected_homepage, expected_source_url) in cases {
            let result = parse_all_versions(SAMPLE_YAML, chart_name)
                .unwrap_or_else(|| panic!("chart '{chart_name}' not found in sample.yaml"));
            assert_eq!(
                result.homepage.as_deref(),
                *expected_homepage,
                "homepage mismatch for {chart_name}"
            );
            assert_eq!(
                result.source_url.as_deref(),
                *expected_source_url,
                "source_url mismatch for {chart_name}"
            );
        }
    }

    // Ported: "returns null if packageName was not provided" — lib/modules/datasource/helm/index.spec.ts line 12
    #[test]
    fn returns_null_if_package_name_not_provided() {
        let result = parse_all_versions(INDEX_YAML, "");
        assert!(result.is_none());
    }

    // Ported: "returns null if repository was not provided" — lib/modules/datasource/helm/index.spec.ts line 22
    #[tokio::test]
    async fn returns_null_if_repository_not_provided() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/index.yaml"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases("some_chart", &server.uri(), &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null for empty response" — lib/modules/datasource/helm/index.spec.ts line 37
    // Ported: "returns null for missing response body" — lib/modules/datasource/helm/index.spec.ts line 51
    #[tokio::test]
    async fn fetch_releases_empty_body_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/index.yaml"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases("non_existent_chart", &server.uri(), &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null for 404" — lib/modules/datasource/helm/index.spec.ts line 65
    #[tokio::test]
    async fn fetch_releases_404_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/index.yaml"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases("some_chart", &server.uri(), &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "throws for 5xx" — lib/modules/datasource/helm/index.spec.ts line 79
    #[tokio::test]
    async fn fetch_releases_5xx_returns_err() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/index.yaml"))
            .respond_with(ResponseTemplate::new(502))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases("some_chart", &server.uri(), &http).await;
        assert!(result.is_err());
    }

    // Ported: "returns null for unknown error" — lib/modules/datasource/helm/index.spec.ts line 93
    #[tokio::test]
    async fn fetch_releases_network_error_returns_none() {
        let http = HttpClient::new().unwrap();
        let result = fetch_releases("some_chart", "http://127.0.0.1:1", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null if index.yaml in response is empty" — lib/modules/datasource/helm/index.spec.ts line 107
    #[test]
    fn parse_comment_only_index_returns_none() {
        assert!(parse_all_versions("# A comment", "redis").is_none());
    }

    // Ported: "returns null if index.yaml in response is invalid" — lib/modules/datasource/helm/index.spec.ts line 120
    #[test]
    fn parse_invalid_yaml_returns_none() {
        let invalid = "some\n  invalid:\n  [\n  yaml";
        assert!(parse_all_versions(invalid, "non_existent_chart").is_none());
    }

    // Ported: "returns null if packageName is not in index.yaml" — lib/modules/datasource/helm/index.spec.ts line 139
    #[test]
    fn parse_returns_none_for_unknown_chart() {
        assert!(parse_all_versions(INDEX_YAML, "non_existent_chart").is_none());
    }

    // Ported: "returns list of versions for normal response" — lib/modules/datasource/helm/index.spec.ts line 152
    #[tokio::test]
    async fn fetch_releases_returns_versions() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/index.yaml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(INDEX_YAML))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases("ambassador", &server.uri(), &http)
            .await
            .unwrap()
            .unwrap();

        assert!(!result.releases.is_empty());
        assert_eq!(result.releases.len(), 27);
    }

    // Ported: "returns list of versions for other packages if one packages has no versions" — lib/modules/datasource/helm/index.spec.ts line 166
    #[tokio::test]
    async fn fetch_releases_skips_empty_package() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/index.yaml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(INDEX_EMPTY_PACKAGE_YAML))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases("ambassador", &server.uri(), &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 1);
        assert_eq!(
            result.homepage.as_deref(),
            Some("https://www.getambassador.io/")
        );
        assert_eq!(
            result.source_url.as_deref(),
            Some("https://github.com/datawire/ambassador")
        );
    }

    // Ported: "adds trailing slash to subdirectories" — lib/modules/datasource/helm/index.spec.ts line 184
    #[tokio::test]
    async fn fetch_releases_from_subdirectory() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/subdir/index.yaml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(INDEX_YAML))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let repository_url = format!("{}/subdir", server.uri());
        let result = fetch_releases("ambassador", &repository_url, &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 27);
        assert_eq!(
            result.homepage.as_deref(),
            Some("https://www.getambassador.io/")
        );
        assert_eq!(
            result.source_url.as_deref(),
            Some("https://github.com/datawire/ambassador")
        );
    }

    // Ported: "uses undefined as the newDigest when no digest is provided" — lib/modules/datasource/helm/index.spec.ts line 203
    #[tokio::test]
    async fn fetch_releases_blank_digest_is_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/index.yaml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(INDEX_BLANK_DIGEST_YAML))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases("blank-digest", &server.uri(), &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 1);
        assert_eq!(result.releases[0].version, "3.2.1");
        assert_eq!(
            result.releases[0].release_timestamp.as_deref(),
            Some("2023-09-05T13:24:19.046Z")
        );
        assert!(result.releases[0].new_digest.is_none());
    }

    // Rust-specific: helm behavior test
    #[test]
    fn parse_finds_first_version() {
        let yaml = make_index_yaml(&[("redis", &["17.0.0", "16.0.0", "15.0.0"])]);
        assert_eq!(
            parse_latest_version(&yaml, "redis").map(|(v, _)| v),
            Some("17.0.0".to_owned())
        );
    }

    // Rust-specific: helm behavior test
    #[test]
    fn parse_handles_multiple_charts() {
        let yaml = make_index_yaml(&[
            ("redis", &["17.0.0"]),
            ("postgresql", &["12.0.0", "11.0.0"]),
        ]);
        let result = parse_all_versions(&yaml, "postgresql").unwrap();
        assert_eq!(result.releases.len(), 2);
        assert_eq!(result.releases[0].version, "12.0.0");
    }

    // Rust-specific: helm behavior test
    #[test]
    fn parse_extracts_created_timestamp() {
        let yaml = "apiVersion: v1\nentries:\n  redis:\n  - name: redis\n    version: 17.0.0\n    created: \"2024-01-15T10:30:00.000Z\"\n";
        let result = parse_all_versions(yaml, "redis").unwrap();
        assert_eq!(result.releases[0].version, "17.0.0");
        assert_eq!(
            result.releases[0].release_timestamp.as_deref(),
            Some("2024-01-15T10:30:00.000Z")
        );
    }

    // Rust-specific: helm behavior test
    #[test]
    fn strip_tilde_gt() {
        assert_eq!(strip_constraint_operators("~> 17.0.0"), "17.0.0");
        assert_eq!(strip_constraint_operators(">= 17.0.0"), "17.0.0");
        assert_eq!(strip_constraint_operators("17.0.0"), "17.0.0");
    }

    #[tokio::test]
    async fn fetch_latest_empty_body_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/index.yaml"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("redis", &server.uri(), &http).await.unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn fetch_latest_from_mock_server() {
        let server = MockServer::start().await;
        let yaml = make_index_yaml(&[("redis", &["17.5.0", "17.0.0"])]);

        Mock::given(method("GET"))
            .and(path("/index.yaml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(yaml))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("redis", &server.uri(), &http).await.unwrap();
        assert_eq!(result.map(|(v, _)| v), Some("17.5.0".to_owned()));
    }

    #[tokio::test]
    async fn fetch_latest_from_subdirectory_repository() {
        let server = MockServer::start().await;
        let yaml = make_index_yaml(&[("redis", &["17.5.0"])]);

        Mock::given(method("GET"))
            .and(path("/subdir/index.yaml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(yaml))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let repository_url = format!("{}/subdir", server.uri());
        let result = fetch_latest("redis", &repository_url, &http).await.unwrap();
        assert_eq!(result.map(|(v, _)| v), Some("17.5.0".to_owned()));
    }

    #[tokio::test]
    async fn fetch_latest_404_returns_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/index.yaml"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("redis", &server.uri(), &http).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn concurrent_fetch_returns_all() {
        let server = MockServer::start().await;
        let yaml = make_index_yaml(&[("redis", &["17.5.0"]), ("nginx", &["1.2.0"])]);

        Mock::given(method("GET"))
            .and(path("/index.yaml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(yaml))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let deps = vec![
            HelmDepInput {
                name: "redis".to_owned(),
                current_value: "17.0.0".to_owned(),
                repository_url: server.uri(),
            },
            HelmDepInput {
                name: "nginx".to_owned(),
                current_value: "1.0.0".to_owned(),
                repository_url: server.uri(),
            },
        ];

        let results = fetch_updates_concurrent(&http, &deps, 4).await;
        assert_eq!(results.len(), 2);

        let redis = results.iter().find(|r| r.name == "redis").unwrap();
        assert!(redis.summary.as_ref().unwrap().update_available);

        let nginx = results.iter().find(|r| r.name == "nginx").unwrap();
        assert!(nginx.summary.as_ref().unwrap().update_available);
    }

    // Rust-specific: helm behavior test
    #[test]
    fn normalize_timestamp_trims_nanoseconds() {
        assert_eq!(
            normalize_timestamp("2019-06-02T08:56:36.116031089Z").as_deref(),
            Some("2019-06-02T08:56:36.116Z")
        );
        assert_eq!(
            normalize_timestamp("2024-01-15T10:30:00.000Z").as_deref(),
            Some("2024-01-15T10:30:00.000Z")
        );
        assert_eq!(
            normalize_timestamp("2023-09-05T13:24:19.046604000Z").as_deref(),
            Some("2023-09-05T13:24:19.046Z")
        );
    }

    #[test]
    fn parse_all_versions_with_registry_basic() {
        let index_yaml = r#"
entries:
  nginx:
    - version: 1.0.0
      created: "2024-01-01T00:00:00Z"
    - version: 1.1.0
      created: "2024-02-01T00:00:00Z"
"#;
        let result =
            parse_all_versions_with_registry(index_yaml, "nginx", "https://charts.example.com")
                .unwrap();
        assert_eq!(result.releases.len(), 2);
        assert_eq!(result.releases[0].version, "1.0.0");
        assert_eq!(result.releases[1].version, "1.1.0");
    }

    #[test]
    fn parse_all_versions_with_registry_missing_chart() {
        let index_yaml = r#"
entries:
  nginx:
    - version: 1.0.0
"#;
        assert!(
            parse_all_versions_with_registry(index_yaml, "missing", "https://charts.example.com")
                .is_none()
        );
    }
}
