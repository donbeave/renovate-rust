//! PyPI datasource.
//!
//! Fetches available releases for a Python package from the PyPI JSON API
//! (`https://pypi.org/pypi/{name}/json`).
//!
//! Renovate reference:
//! - `lib/modules/datasource/pypi/index.ts` — `PypiDatasource`
//! - `lib/modules/datasource/pypi/types.ts`  — `PypiJSON`
//!
//! ## Protocol
//!
//! `GET https://pypi.org/pypi/{name}/json`
//!
//! The response includes:
//! - `info.version` — the latest stable version (as determined by PyPI)
//! - `releases`     — a map of `version → [{yanked, …}, …]`
//!
//! A version is yanked when every file in its release array has `"yanked": true`.
//! We use `info.version` as the authoritative "latest stable" pointer.
//!
//! Package names are normalized per PEP 503 before the lookup:
//! lowercase, runs of `[-_.]` → `-`.

use std::collections::HashMap;
use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::{HttpClient, HttpError};
use crate::versioning::pep440::{Pep440UpdateSummary, pep440_update_summary};

/// Default PyPI JSON API base URL.
pub const PYPI_API: &str = "https://pypi.org/pypi";

/// Errors from PyPI lookups.
#[derive(Debug, Error)]
pub enum PypiError {
    #[error("HTTP error: {0}")]
    Http(#[from] HttpError),
    #[error("failed to parse PyPI response: {0}")]
    Parse(String),
    #[error("package '{0}' not found in versions cache")]
    NotFound(String),
}

/// Minimal PyPI JSON API response structure.
#[derive(Debug, Deserialize)]
struct PypiResponse {
    info: PypiInfo,
    #[serde(default)]
    releases: HashMap<String, Vec<PypiRelease>>,
}

#[derive(Debug, Deserialize)]
struct PypiInfo {
    version: String,
}

#[derive(Debug, Deserialize)]
struct PypiRelease {
    #[serde(default)]
    yanked: bool,
    /// ISO 8601 publish timestamp (without timezone suffix), e.g. `"2023-01-15T10:30:00"`.
    #[serde(default)]
    upload_time: Option<String>,
}

/// Normalize a Python package name for use in the PyPI URL.
///
/// Per PEP 503: lowercase, runs of `[-_.]` replaced by `-`.
fn normalize_name(name: &str) -> String {
    let lower = name.to_lowercase();
    let mut result = String::with_capacity(lower.len());
    let mut prev_sep = false;
    for ch in lower.chars() {
        if ch == '-' || ch == '_' || ch == '.' {
            if !prev_sep {
                result.push('-');
            }
            prev_sep = true;
        } else {
            result.push(ch);
            prev_sep = false;
        }
    }
    result
}

/// Fetch the latest stable version and all non-yanked version strings for a
/// Python package from the PyPI JSON API.
///
/// Returns `(versions_sorted_oldest_first, latest_stable_version)`.
pub async fn fetch_versions(
    http: &HttpClient,
    package_name: &str,
    api_base: &str,
) -> Result<PypiVersionsEntry, PypiError> {
    let normalized = normalize_name(package_name);
    let url = format!("{}/{}/json", api_base.trim_end_matches('/'), normalized);

    let resp = http.get_retrying(&url).await?;
    if !resp.status().is_success() {
        return Err(PypiError::Http(HttpError::Status {
            status: resp.status(),
            url,
        }));
    }

    let body = resp.text().await.map_err(HttpError::Request)?;
    let data: PypiResponse =
        serde_json::from_str(&body).map_err(|e| PypiError::Parse(e.to_string()))?;

    let latest = data.info.version.clone();

    // Collect non-yanked versions and extract the latest timestamp.
    let mut latest_timestamp: Option<String> = None;
    let mut versions: Vec<String> = data
        .releases
        .iter()
        .filter(|(_, files)| files.is_empty() || files.iter().any(|f| !f.yanked))
        .map(|(v, files)| {
            if v == &latest {
                // Pick the earliest (by upload_time) non-yanked file's timestamp.
                latest_timestamp = files
                    .iter()
                    .filter(|f| !f.yanked)
                    .filter_map(|f| f.upload_time.as_deref())
                    .min()
                    .map(|t| {
                        // PyPI uses naive ISO datetime; append Z to make it UTC-parseable.
                        if t.ends_with('Z') || t.contains('+') {
                            t.to_owned()
                        } else {
                            format!("{t}Z")
                        }
                    });
            }
            v.clone()
        })
        .collect();

    versions.sort_by(|a, b| {
        let av = semver::Version::parse(a);
        let bv = semver::Version::parse(b);
        match (av, bv) {
            (Ok(a), Ok(b)) => a.cmp(&b),
            (Ok(_), Err(_)) => std::cmp::Ordering::Less,
            (Err(_), Ok(_)) => std::cmp::Ordering::Greater,
            (Err(_), Err(_)) => a.cmp(b),
        }
    });

    Ok(PypiVersionsEntry {
        versions,
        latest,
        latest_timestamp,
    })
}

/// Input descriptor for a single pip dependency in a batch fetch.
#[derive(Debug, Clone)]
pub struct PypiDepInput {
    /// Normalized package name used for the lookup.
    pub dep_name: String,
    /// The version specifier string from requirements.txt (e.g. `"==1.2.3"`).
    pub specifier: String,
}

/// Result for a single pip dependency after a batch fetch.
#[derive(Debug)]
pub struct PypiDepUpdate {
    pub dep_name: String,
    pub summary: Result<Pep440UpdateSummary, PypiError>,
}

/// Fetch version info for a batch of pip dependencies concurrently.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[PypiDepInput],
    api_base: &str,
    concurrency: usize,
) -> Vec<PypiDepUpdate> {
    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<PypiDepUpdate> = JoinSet::new();

    for dep in deps {
        let http = http.clone();
        let sem = Arc::clone(&sem);
        let dep_name = dep.dep_name.clone();
        let specifier = dep.specifier.clone();
        let api_base = api_base.to_owned();

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_versions(&http, &dep_name, &api_base).await;
            let summary = result.map(|entry| {
                let mut s = pep440_update_summary(&specifier, Some(entry.latest.as_str()));
                s.latest_timestamp = entry.latest_timestamp;
                s
            });
            PypiDepUpdate { dep_name, summary }
        });
    }

    let mut results = Vec::with_capacity(deps.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok(update) => results.push(update),
            Err(join_err) => tracing::error!(%join_err, "pypi datasource task panicked"),
        }
    }
    results
}

/// Cached versions entry: `(sorted versions oldest-first, latest_stable)`.
/// Cached PyPI versions entry for a single package.
#[derive(Debug, Clone)]
pub struct PypiVersionsEntry {
    /// Versions sorted oldest-first by PEP 440 semantics.
    pub versions: Vec<String>,
    /// Latest stable version reported by `info.version`.
    pub latest: String,
    /// ISO 8601 publish timestamp for the latest version, if available.
    pub latest_timestamp: Option<String>,
}

/// Fetch versions for a batch of unique package names concurrently.
///
/// Returns a `HashMap` from normalised package name to `(versions, latest)`.
/// Packages that fail to fetch are omitted.  Use for cross-file deduplication
/// when the same package may appear in multiple requirements files.
pub async fn fetch_versions_batch(
    http: &HttpClient,
    package_names: &[String],
    api_base: &str,
    concurrency: usize,
) -> HashMap<String, PypiVersionsEntry> {
    if package_names.is_empty() {
        return HashMap::new();
    }

    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<(String, Option<PypiVersionsEntry>)> = JoinSet::new();

    for name in package_names {
        let http = http.clone();
        let sem = Arc::clone(&sem);
        let name = name.clone();
        let api_base = api_base.to_owned();

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_versions(&http, &name, &api_base).await;
            (name, result.ok())
        });
    }

    let mut cache = HashMap::with_capacity(package_names.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok((name, Some(entry))) => {
                cache.insert(name, entry);
            }
            Ok((name, None)) => {
                tracing::debug!(package = %name, "pypi fetch failed (package skipped)")
            }
            Err(join_err) => tracing::error!(%join_err, "pypi batch fetch task panicked"),
        }
    }
    cache
}

/// Compute a `Pep440UpdateSummary` from a pre-fetched versions cache entry.
pub fn summary_from_cache(specifier: &str, entry: &PypiVersionsEntry) -> Pep440UpdateSummary {
    let mut s = pep440_update_summary(specifier, Some(entry.latest.as_str()));
    s.latest_timestamp = entry.latest_timestamp.clone();
    s
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;
    use crate::http::HttpClient;

    // ── normalize_name ────────────────────────────────────────────────────────

    #[test]
    fn normalize_underscores() {
        assert_eq!(normalize_name("my_package"), "my-package");
        assert_eq!(normalize_name("Django"), "django");
        assert_eq!(normalize_name("Pillow"), "pillow");
    }

    #[test]
    fn normalize_consecutive_separators() {
        assert_eq!(normalize_name("some--double"), "some-double");
    }

    // ── fetch_versions (wiremock) ─────────────────────────────────────────────

    fn pypi_response(name: &str, latest: &str, releases: &[&str]) -> String {
        let releases_obj: String = releases
            .iter()
            .map(|v| format!(r#""{v}":[{{"yanked":false}}]"#))
            .collect::<Vec<_>>()
            .join(",");
        format!(
            r#"{{"info":{{"name":"{name}","version":"{latest}"}},"releases":{{{releases_obj}}}}}"#
        )
    }

    #[tokio::test]
    async fn fetch_versions_returns_sorted() {
        let server = MockServer::start().await;
        let body = pypi_response("django", "4.2.7", &["4.0.0", "4.2.7", "4.2.5"]);
        Mock::given(method("GET"))
            .and(path("/django/json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let entry = fetch_versions(&http, "django", &server.uri())
            .await
            .unwrap();
        assert_eq!(entry.latest, "4.2.7");
        assert_eq!(entry.versions, vec!["4.0.0", "4.2.5", "4.2.7"]);
    }

    #[tokio::test]
    async fn fetch_versions_normalizes_name() {
        let server = MockServer::start().await;
        let body = pypi_response("my-package", "1.0.0", &["1.0.0"]);
        Mock::given(method("GET"))
            .and(path("/my-package/json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let entry = fetch_versions(&http, "my_package", &server.uri())
            .await
            .unwrap();
        assert_eq!(entry.latest, "1.0.0");
    }

    #[tokio::test]
    async fn fetch_versions_404_returns_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nonexistent/json"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_versions(&http, "nonexistent", &server.uri()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn fetch_updates_concurrent_reports_update() {
        let server = MockServer::start().await;
        let body = pypi_response("requests", "2.31.0", &["2.28.0", "2.29.0", "2.31.0"]);
        Mock::given(method("GET"))
            .and(path("/requests/json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let deps = vec![PypiDepInput {
            dep_name: "requests".into(),
            specifier: "==2.28.0".into(),
        }];
        let results = fetch_updates_concurrent(&http, &deps, &server.uri(), 10).await;
        assert_eq!(results.len(), 1);
        let s = results[0].summary.as_ref().unwrap();
        assert!(s.update_available);
        assert_eq!(s.latest.as_deref(), Some("2.31.0"));
    }
}
