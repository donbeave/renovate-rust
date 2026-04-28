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
}

/// Per-dependency result from `fetch_updates_concurrent`.
#[derive(Debug)]
pub struct HelmUpdateResult {
    pub name: String,
    pub summary: Result<HelmUpdateSummary, HelmError>,
}

// ── Fetch functions ───────────────────────────────────────────────────────────

/// Fetch the latest version of a Helm chart from a repository's `index.yaml`.
pub async fn fetch_latest(
    chart_name: &str,
    repository_url: &str,
    http: &HttpClient,
) -> Result<Option<String>, HelmError> {
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
    let latest = fetch_latest(&dep.name, &dep.repository_url, http).await?;
    let current = strip_constraint_operators(&dep.current_value);
    let update_available = latest
        .as_deref()
        .is_some_and(|l| !current.is_empty() && l != current);
    Ok(HelmUpdateSummary {
        current_value: dep.current_value.clone(),
        latest,
        update_available,
    })
}

// ── index.yaml line-scanner ───────────────────────────────────────────────────

/// Parse a Helm `index.yaml` and return the latest version of `chart_name`.
///
/// The scanner uses three states:
/// 1. Searching for `entries:` at indent 0.
/// 2. Searching for `  {chart_name}:` at indent 2.
/// 3. Searching for the first `    version:` value (entries are newest-first).
pub fn parse_latest_version(index_yaml: &str, chart_name: &str) -> Option<String> {
    #[derive(PartialEq)]
    enum State {
        Entries,
        Chart,
        Version,
    }

    let chart_key = format!("{chart_name}:");
    let mut state = State::Entries;

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
                // The chart key appeared; now find the first `version:` field.
                // Version fields are indented at 4+ spaces.
                if indent >= 4 && trimmed.starts_with("version:") {
                    let val = trimmed["version:".len()..].trim().trim_matches('"');
                    if !val.is_empty() {
                        return Some(val.to_owned());
                    }
                }
                // If we hit another indent-2 key, we've left this chart's block.
                if indent == 2 && trimmed != "-" && !trimmed.starts_with('-') {
                    break;
                }
            }
        }
    }

    None
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
            parse_latest_version(&yaml, "redis"),
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
            parse_latest_version(&yaml, "postgresql"),
            Some("12.0.0".to_owned())
        );
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
        assert_eq!(result, Some("17.5.0".to_owned()));
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
