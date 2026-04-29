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
//!       ...
//!     - name: redis
//!       version: 16.0.0
//! ```
//!
//! Entries under each chart name are sorted newest-first; we take the first
//! `version:` value we encounter under the target chart name.

use std::sync::Arc;

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

// ── Fetch functions ───────────────────────────────────────────────────────────

/// Fetch the latest version of a Helm chart from a repository's `index.yaml`.
///
/// Returns `(version, created_at)` where `created_at` is the chart entry's
/// `created` timestamp from `index.yaml`.
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

// ── index.yaml line-scanner ───────────────────────────────────────────────────

/// Parse the latest version and its `created` timestamp from a Helm
/// `index.yaml` for the given `chart_name`.
///
/// The scanner uses three states:
/// 1. Searching for `entries:` at indent 0.
/// 2. Searching for `  {chart_name}:` at indent 2.
/// 3. Collecting `version:` and `created:` from the first entry (newest).
///
/// Returns `Some((version, created_at))` or `None` if the chart is not found.
pub fn parse_latest_version(
    index_yaml: &str,
    chart_name: &str,
) -> Option<(String, Option<String>)> {
    #[derive(PartialEq)]
    enum State {
        Entries,
        Chart,
        Version,
    }

    let chart_key = format!("{chart_name}:");
    let mut state = State::Entries;
    let mut found_version: Option<String> = None;
    let mut found_created: Option<String> = None;

    for line in index_yaml.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let indent = leading_spaces(line);

        match state {
            State::Entries => {
                if indent == 0 && trimmed == "entries:" {
                    state = State::Chart;
                }
            }
            State::Chart => {
                // The chart key is at indent 2 inside `entries`.
                if indent == 2 && trimmed == chart_key {
                    state = State::Version;
                }
                // Back to top-level key → we've passed entries.
                if indent == 0 && trimmed != "entries:" {
                    break;
                }
            }
            State::Version => {
                // Collect version and created fields from the first chart entry.
                if indent >= 4 {
                    if let Some(rest) = trimmed.strip_prefix("version:") {
                        let val = rest.trim().trim_matches('"');
                        if !val.is_empty() && found_version.is_none() {
                            found_version = Some(val.to_owned());
                        }
                    } else if let Some(rest) = trimmed.strip_prefix("created:") {
                        let val = rest.trim().trim_matches('"');
                        if !val.is_empty() && found_created.is_none() {
                            found_created = Some(val.to_owned());
                        }
                    }
                }
                // Stop collecting when we reach the second entry (indent 2, starts with `-`).
                if indent == 2
                    && (trimmed == "-" || trimmed.starts_with('-'))
                    && found_version.is_some()
                {
                    break; // first entry complete
                }
                // If we hit another indent-2 non-list key, we've left this chart's block.
                if indent == 2 && trimmed != "-" && !trimmed.starts_with('-') {
                    break;
                }
            }
        }
    }

    found_version.map(|v| (v, found_created))
}

fn leading_spaces(line: &str) -> usize {
    line.len() - line.trim_start().len()
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

    fn index_yaml(charts: &[(&str, &[&str])]) -> String {
        let mut out = "apiVersion: v1\nentries:\n".to_owned();
        for (name, versions) in charts {
            out.push_str(&format!("  {name}:\n"));
            for v in *versions {
                out.push_str(&format!("    - name: {name}\n      version: {v}\n"));
            }
        }
        out
    }

    #[test]
    fn parse_finds_first_version() {
        let yaml = index_yaml(&[("redis", &["17.0.0", "16.0.0", "15.0.0"])]);
        assert_eq!(
            parse_latest_version(&yaml, "redis").map(|(v, _)| v),
            Some("17.0.0".to_owned())
        );
    }

    #[test]
    fn parse_returns_none_for_unknown_chart() {
        let yaml = index_yaml(&[("redis", &["17.0.0"])]);
        assert_eq!(parse_latest_version(&yaml, "postgresql"), None);
    }

    #[test]
    fn parse_handles_multiple_charts() {
        let yaml = index_yaml(&[
            ("redis", &["17.0.0"]),
            ("postgresql", &["12.0.0", "11.0.0"]),
        ]);
        assert_eq!(
            parse_latest_version(&yaml, "postgresql").map(|(v, _)| v),
            Some("12.0.0".to_owned())
        );
    }

    #[test]
    fn parse_extracts_created_timestamp() {
        let yaml = "apiVersion: v1\nentries:\n  redis:\n    - name: redis\n      version: 17.0.0\n      created: \"2024-01-15T10:30:00.000Z\"\n";
        let result = parse_latest_version(yaml, "redis");
        assert!(result.is_some());
        let (version, created) = result.unwrap();
        assert_eq!(version, "17.0.0");
        assert_eq!(created.as_deref(), Some("2024-01-15T10:30:00.000Z"));
    }

    #[test]
    fn strip_tilde_gt() {
        assert_eq!(strip_constraint_operators("~> 17.0.0"), "17.0.0");
        assert_eq!(strip_constraint_operators(">= 17.0.0"), "17.0.0");
        assert_eq!(strip_constraint_operators("17.0.0"), "17.0.0");
    }

    #[tokio::test]
    async fn fetch_latest_from_mock_server() {
        let server = MockServer::start().await;
        let yaml = index_yaml(&[("redis", &["17.5.0", "17.0.0"])]);

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
        let yaml = index_yaml(&[("redis", &["17.5.0"]), ("nginx", &["1.2.0"])]);

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
}
