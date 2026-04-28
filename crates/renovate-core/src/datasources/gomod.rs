//! Go module proxy datasource.
//!
//! Fetches the latest version of a Go module from the Go module proxy.
//!
//! Renovate reference:
//! - `lib/modules/datasource/go/index.ts`
//! - API: `https://proxy.golang.org/{module}/@latest`
//!
//! ## URL encoding
//!
//! The Go proxy spec requires capital letters in module paths to be encoded as
//! `!` + lowercase (e.g. `github.com/Azure/sdk` → `github.com/!azure/sdk`).
//! This encoding is applied before making requests.

use std::collections::HashMap;
use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::HttpClient;

pub const GO_PROXY_BASE: &str = "https://proxy.golang.org";

/// Errors from fetching Go proxy metadata.
#[derive(Debug, Error)]
pub enum GoModError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
}

/// Input for a single Go module dependency lookup.
#[derive(Debug, Clone)]
pub struct GoModDepInput {
    /// Go module path (e.g. `github.com/gorilla/mux`).
    pub module_path: String,
    /// Currently declared version.
    pub current_value: String,
}

/// Update summary for a Go module dependency.
#[derive(Debug, Clone)]
pub struct GoModUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Per-dependency result from `fetch_updates_concurrent`.
#[derive(Debug)]
pub struct GoModUpdateResult {
    pub module_path: String,
    pub summary: Result<GoModUpdateSummary, GoModError>,
}

#[derive(Debug, Deserialize)]
struct ProxyLatest {
    #[serde(rename = "Version")]
    version: String,
}

/// Fetch the latest version of a Go module from the proxy.
pub async fn fetch_latest(
    module_path: &str,
    http: &HttpClient,
    proxy_base: &str,
) -> Result<Option<String>, GoModError> {
    let encoded = encode_module_path(module_path);
    let url = format!("{proxy_base}/{encoded}/@latest");

    let resp = http.get_retrying(&url).await?;
    if resp.status().as_u16() == 404 || resp.status().as_u16() == 410 {
        return Ok(None);
    }
    if !resp.status().is_success() {
        return Ok(None);
    }

    let info: ProxyLatest = resp.json().await.map_err(GoModError::Json)?;
    Ok(Some(info.version))
}

/// Fetch update summaries for multiple Go module dependencies concurrently.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[GoModDepInput],
    proxy_base: &str,
    concurrency: usize,
) -> Vec<GoModUpdateResult> {
    if deps.is_empty() {
        return Vec::new();
    }

    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<GoModUpdateResult> = JoinSet::new();

    for dep in deps {
        let http = http.clone();
        let dep = dep.clone();
        let sem = Arc::clone(&sem);
        let proxy_base = proxy_base.to_owned();

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_update_summary(&dep, &http, &proxy_base).await;
            GoModUpdateResult {
                module_path: dep.module_path.clone(),
                summary: result,
            }
        });
    }

    let mut results = Vec::with_capacity(deps.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok(r) => results.push(r),
            Err(join_err) => tracing::error!(%join_err, "go proxy lookup task panicked"),
        }
    }
    results
}

/// Cached Go module latest version entry.
pub type GoModLatestEntry = Option<String>;

/// Fetch the latest version for a batch of unique Go module paths concurrently.
///
/// Returns a `HashMap` from module path to latest version string (None if not found).
pub async fn fetch_latest_batch(
    http: &HttpClient,
    module_paths: &[String],
    proxy_base: &str,
    concurrency: usize,
) -> HashMap<String, GoModLatestEntry> {
    if module_paths.is_empty() {
        return HashMap::new();
    }

    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<(String, GoModLatestEntry)> = JoinSet::new();

    for path in module_paths {
        let http = http.clone();
        let path = path.clone();
        let proxy_base = proxy_base.to_owned();
        let sem = Arc::clone(&sem);

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let latest = fetch_latest(&path, &http, &proxy_base).await.ok().flatten();
            (path, latest)
        });
    }

    let mut cache = HashMap::with_capacity(module_paths.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok((path, latest)) => {
                cache.insert(path, latest);
            }
            Err(join_err) => tracing::error!(%join_err, "go proxy batch fetch task panicked"),
        }
    }
    cache
}

/// Compute a `GoModUpdateSummary` from a pre-fetched latest version entry.
pub fn summary_from_cache(current_value: &str, latest: GoModLatestEntry) -> GoModUpdateSummary {
    let update_available = latest
        .as_deref()
        .is_some_and(|l| l != current_value && !current_value.is_empty());
    GoModUpdateSummary {
        current_value: current_value.to_owned(),
        latest,
        update_available,
    }
}

async fn fetch_update_summary(
    dep: &GoModDepInput,
    http: &HttpClient,
    proxy_base: &str,
) -> Result<GoModUpdateSummary, GoModError> {
    let latest = fetch_latest(&dep.module_path, http, proxy_base).await?;
    let update_available = latest
        .as_deref()
        .is_some_and(|l| l != dep.current_value && !dep.current_value.is_empty());
    Ok(GoModUpdateSummary {
        current_value: dep.current_value.clone(),
        latest,
        update_available,
    })
}

/// Encode a Go module path per the Go module proxy protocol.
///
/// Capital letters are replaced by `!` + their lowercase equivalent.
/// Example: `github.com/Azure/sdk` → `github.com/!azure/sdk`.
pub fn encode_module_path(path: &str) -> String {
    let mut out = String::with_capacity(path.len() + 4);
    for ch in path.chars() {
        if ch.is_ascii_uppercase() {
            out.push('!');
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    #[test]
    fn encode_module_path_all_lowercase() {
        assert_eq!(
            encode_module_path("github.com/gorilla/mux"),
            "github.com/gorilla/mux"
        );
    }

    #[test]
    fn encode_module_path_capital_letters() {
        assert_eq!(
            encode_module_path("github.com/Azure/azure-sdk-for-go"),
            "github.com/!azure/azure-sdk-for-go"
        );
    }

    #[tokio::test]
    async fn fetch_latest_returns_version() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/github.com/gorilla/mux/@latest"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(r#"{"Version":"v1.8.1","Time":"2023-09-01T00:00:00Z"}"#),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("github.com/gorilla/mux", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, Some("v1.8.1".to_owned()));
    }

    #[tokio::test]
    async fn fetch_latest_404_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/github.com/missing/pkg/@latest"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("github.com/missing/pkg", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, None);
    }
}
