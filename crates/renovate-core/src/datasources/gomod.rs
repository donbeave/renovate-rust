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
    /// ISO 8601 publication timestamp from the Go module proxy `@latest` response.
    pub release_timestamp: Option<String>,
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
    /// RFC 3339 publication timestamp, e.g. `"2024-01-15T10:30:00Z"`.
    #[serde(rename = "Time")]
    time: Option<String>,
}

/// Fetch the latest version of a Go module from the proxy.
///
/// Returns `(version, published_at)` where `published_at` is an ISO 8601
/// timestamp from the `Time` field of the Go module proxy `@latest` response.
pub async fn fetch_latest(
    module_path: &str,
    http: &HttpClient,
    proxy_base: &str,
) -> Result<Option<(String, Option<String>)>, GoModError> {
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
    Ok(Some((info.version, info.time)))
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
            // Strip timestamp — batch cache only needs the version string.
            let latest = fetch_latest(&path, &http, &proxy_base)
                .await
                .ok()
                .flatten()
                .map(|(v, _)| v);
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
        release_timestamp: None, // not available from the batch cache
    }
}

async fn fetch_update_summary(
    dep: &GoModDepInput,
    http: &HttpClient,
    proxy_base: &str,
) -> Result<GoModUpdateSummary, GoModError> {
    let result = fetch_latest(&dep.module_path, http, proxy_base).await?;
    let (latest, release_timestamp) = result.map(|(v, ts)| (Some(v), ts)).unwrap_or((None, None));
    let update_available = latest
        .as_deref()
        .is_some_and(|l| l != dep.current_value && !dep.current_value.is_empty());
    Ok(GoModUpdateSummary {
        current_value: dep.current_value.clone(),
        latest,
        update_available,
        release_timestamp,
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

// ── get_source_url ────────────────────────────────────────────────────────────

/// Map a Go datasource + package name to a source URL.
///
/// Mirrors `getSourceUrl` from `lib/modules/datasource/go/common.ts`.
///
/// `registry_url` is optional; each datasource falls back to its default host
/// when not provided.
pub fn get_source_url(
    datasource: &str,
    package_name: &str,
    registry_url: Option<&str>,
) -> Option<String> {
    let url = match datasource {
        "github-tags" => {
            let base = registry_url.unwrap_or("https://github.com");
            format!("{}/{package_name}", base.trim_end_matches('/'))
        }
        "gitlab-tags" => {
            let base = registry_url.unwrap_or("https://gitlab.com");
            format!("{}/{package_name}", base.trim_end_matches('/'))
        }
        "bitbucket-tags" => {
            let base = registry_url.unwrap_or("https://bitbucket.org");
            format!("{}/{package_name}", base.trim_end_matches('/'))
        }
        "gitea-tags" => {
            let base = registry_url.unwrap_or("https://gitea.com");
            format!("{}/{package_name}", base.trim_end_matches('/'))
        }
        "forgejo-tags" => {
            let base = registry_url.unwrap_or("https://code.forgejo.org");
            format!("{}/{package_name}", base.trim_end_matches('/'))
        }
        _ => return None,
    };
    Some(url)
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    // Ported: "encodeCase" — modules/datasource/go/releases-goproxy.spec.ts line 27
    #[test]
    fn encode_module_path_all_lowercase() {
        assert_eq!(
            encode_module_path("github.com/gorilla/mux"),
            "github.com/gorilla/mux"
        );
    }

    // Ported: "encodeCase" — modules/datasource/go/releases-goproxy.spec.ts line 27
    #[test]
    fn encode_module_path_capital_letters() {
        assert_eq!(
            encode_module_path("github.com/Azure/azure-sdk-for-go"),
            "github.com/!azure/azure-sdk-for-go"
        );
        assert_eq!(
            encode_module_path("github.com/FOO/bar"),
            "github.com/!f!o!o/bar"
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
        let (version, timestamp) = result.unwrap();
        assert_eq!(version, "v1.8.1");
        assert_eq!(timestamp.as_deref(), Some("2023-09-01T00:00:00Z"));
    }

    // Ported: "returns null for 404" — lib/modules/datasource/pod/index.spec.ts line 60
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

    // ── get_source_url ────────────────────────────────────────────────────────

    // Ported: "($datasource, $packageName) => $expected" — datasource/go/common.spec.ts line 5
    #[test]
    fn get_source_url_maps_datasource_to_url() {
        assert_eq!(
            get_source_url("bitbucket-tags", "foo/bar", None),
            Some("https://bitbucket.org/foo/bar".to_owned())
        );
        assert_eq!(
            get_source_url("forgejo-tags", "go-chi/cache", None),
            Some("https://code.forgejo.org/go-chi/cache".to_owned())
        );
        assert_eq!(
            get_source_url("gitea-tags", "go-chi/cache", None),
            Some("https://gitea.com/go-chi/cache".to_owned())
        );
        assert_eq!(
            get_source_url("github-tags", "go-foo/foo", None),
            Some("https://github.com/go-foo/foo".to_owned())
        );
        assert_eq!(
            get_source_url("gitlab-tags", "foo/bar", None),
            Some("https://gitlab.com/foo/bar".to_owned())
        );
        // git-tags is not handled → None
        assert_eq!(
            get_source_url("git-tags", "https://dev.azure.com/foo/bar/_git/baz", None),
            None
        );
    }
}
