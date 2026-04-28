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
) -> Result<(Vec<String>, String), PypiError> {
    let normalized = normalize_name(package_name);
    let url = format!("{}/{}/json", api_base.trim_end_matches('/'), normalized);

    let resp = http.get(&url).send().await.map_err(HttpError::Request)?;
    if !resp.status().is_success() {
        return Err(PypiError::Http(HttpError::Status {
            status: resp.status(),
            url,
        }));
    }

    let body = resp.text().await.map_err(HttpError::Request)?;
    let data: PypiResponse =
        serde_json::from_str(&body).map_err(|e| PypiError::Parse(e.to_string()))?;

    let latest = data.info.version;

    // Collect non-yanked versions (a version is active if at least one of its
    // files is not yanked, or if the files list is empty/no yanked field).
    let mut versions: Vec<String> = data
        .releases
        .into_iter()
        .filter(|(_, files)| {
            // Keep the version if it has no files (pre-release/metadata-only)
            // or if at least one file is not yanked.
            files.is_empty() || files.iter().any(|f| !f.yanked)
        })
        .map(|(v, _)| v)
        .collect();

    // Sort by version string (best-effort semver; PEP 440 ordering is a later
    // slice — for most packages semver ordering is correct).
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

    Ok((versions, latest))
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
            let summary =
                result.map(|(_versions, latest)| pep440_update_summary(&specifier, Some(&latest)));
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
        let (versions, latest) = fetch_versions(&http, "django", &server.uri())
            .await
            .unwrap();
        assert_eq!(latest, "4.2.7");
        assert_eq!(versions, vec!["4.0.0", "4.2.5", "4.2.7"]);
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
        let (_, latest) = fetch_versions(&http, "my_package", &server.uri())
            .await
            .unwrap();
        assert_eq!(latest, "1.0.0");
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
