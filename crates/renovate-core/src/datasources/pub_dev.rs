//! Dart pub.dev datasource.
//!
//! Renovate reference: `lib/modules/datasource/dart/index.ts`
//! API: `GET {registry}/api/packages/{name}`

use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY: &str = "https://pub.dartlang.org";
pub const DATASOURCE_ID: &str = "dart";

#[derive(Debug, Error)]
pub enum PubError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

#[derive(Debug, Clone)]
pub struct DartRelease {
    pub version: String,
    pub release_timestamp: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DartResult {
    pub releases: Vec<DartRelease>,
    pub homepage: Option<String>,
    pub source_url: Option<String>,
}

// ── API types ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct DartApiResponse {
    versions: Option<Vec<DartVersionEntry>>,
    latest: Option<DartLatest>,
}

#[derive(Debug, Deserialize)]
struct DartVersionEntry {
    version: String,
    published: Option<String>,
    #[serde(default)]
    retracted: bool,
}

#[derive(Debug, Deserialize)]
struct DartLatest {
    pubspec: Option<DartPubspec>,
}

#[derive(Debug, Deserialize)]
struct DartPubspec {
    homepage: Option<String>,
    repository: Option<String>,
}

/// Normalize any ISO 8601 timestamp to millisecond precision.
/// "2017-05-09T18:25:24.268386Z" → "2017-05-09T18:25:24.268Z"
fn normalize_timestamp(ts: &str) -> Option<String> {
    ts.parse::<chrono::DateTime<chrono::Utc>>()
        .ok()
        .map(|dt| dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string())
}

/// Fetch Dart package releases.
///
/// 4xx / parse error / network error → `Ok(None)`. 5xx → `Err`.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<DartResult>, PubError> {
    let base = registry_url.trim_end_matches('/');
    let url = format!("{base}/api/packages/{package_name}");

    let text = match http.get_raw_with_accept(&url, "application/json").await {
        Ok(v) => v,
        Err(e) => {
            let is_server_err = matches!(
                &e,
                crate::http::HttpError::Status { status, .. } if status.is_server_error()
            );
            if is_server_err {
                return Err(PubError::Http(e));
            }
            return Ok(None);
        }
    };

    let api: DartApiResponse = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    let versions = match api.versions {
        Some(v) if !v.is_empty() => v,
        _ => return Ok(None),
    };
    if api.latest.is_none() {
        return Ok(None);
    }

    let releases: Vec<DartRelease> = versions
        .into_iter()
        .filter(|v| !v.retracted)
        .map(|v| DartRelease {
            version: v.version,
            release_timestamp: v.published.as_deref().and_then(normalize_timestamp),
        })
        .collect();

    let pubspec = api.latest.as_ref().and_then(|l| l.pubspec.as_ref());
    let homepage = pubspec.and_then(|p| p.homepage.clone());
    let source_url = pubspec.and_then(|p| p.repository.clone());

    Ok(Some(DartResult {
        releases,
        homepage,
        source_url,
    }))
}

// ── Pipeline helpers ───────────────────────────────────────────────────────

/// Input for a single pub.dev package lookup.
#[derive(Debug, Clone)]
pub struct PubDepInput {
    pub name: String,
    pub current_value: String,
}

/// Update summary for a pub dependency.
#[derive(Debug, Clone)]
pub struct PubUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
    pub release_timestamp: Option<String>,
}

/// Per-dependency result from `fetch_updates_concurrent`.
#[derive(Debug)]
pub struct PubUpdateResult {
    pub name: String,
    pub summary: Result<PubUpdateSummary, PubError>,
}

/// Fetch update summaries for multiple pub packages concurrently.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[PubDepInput],
    registry_url: &str,
    concurrency: usize,
) -> Vec<PubUpdateResult> {
    if deps.is_empty() {
        return Vec::new();
    }

    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<PubUpdateResult> = JoinSet::new();

    for dep in deps {
        let http = http.clone();
        let dep = dep.clone();
        let sem = Arc::clone(&sem);
        let registry_url = registry_url.to_owned();

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_dep_summary(&dep, &http, &registry_url).await;
            PubUpdateResult {
                name: dep.name.clone(),
                summary: result,
            }
        });
    }

    let mut results = Vec::with_capacity(deps.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok(r) => results.push(r),
            Err(join_err) => tracing::error!(%join_err, "pub.dev lookup task panicked"),
        }
    }
    results
}

async fn fetch_dep_summary(
    dep: &PubDepInput,
    http: &HttpClient,
    registry_url: &str,
) -> Result<PubUpdateSummary, PubError> {
    let result = fetch_releases(registry_url, &dep.name, http).await?;
    let (latest, release_timestamp) = match result {
        None => (None, None),
        Some(r) => {
            let latest = r.releases.last().map(|rel| rel.version.clone());
            let ts = r
                .releases
                .last()
                .and_then(|rel| rel.release_timestamp.clone());
            (latest, ts)
        }
    };
    let update_available = latest
        .as_deref()
        .map(|l| l != dep.current_value)
        .unwrap_or(false);
    Ok(PubUpdateSummary {
        current_value: dep.current_value.clone(),
        latest,
        update_available,
        release_timestamp,
    })
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    const SHARED_PREFS_JSON: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/dart/__fixtures__/shared_preferences.json"
    );

    // Ported: "returns null for empty result" — lib/modules/datasource/dart/index.spec.ts line 13
    #[tokio::test]
    async fn returns_null_for_empty_result() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/packages/non_sense"))
            .respond_with(ResponseTemplate::new(200).set_body_string("}"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "non_sense", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null for empty fields" — lib/modules/datasource/dart/index.spec.ts line 23
    #[tokio::test]
    async fn returns_null_for_empty_fields() {
        // Missing versions field
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/packages/shared_preferences"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string(r#"{"latest":{"pubspec":{}}}"#),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "shared_preferences", &http)
            .await
            .unwrap();
        assert!(result.is_none());

        // Missing latest field
        let server2 = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/packages/shared_preferences"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"{"versions":[{"version":"0.5.8","published":"2020-07-08T04:36:42.000Z"}]}"#,
            ))
            .mount(&server2)
            .await;

        let result2 = fetch_releases(&server2.uri(), "shared_preferences", &http)
            .await
            .unwrap();
        assert!(result2.is_none());
    }

    // Ported: "returns null for 404" — lib/modules/datasource/dart/index.spec.ts line 55
    #[tokio::test]
    async fn returns_null_for_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/packages/shared_preferences"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "shared_preferences", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "throws for 5xx" — lib/modules/datasource/dart/index.spec.ts line 65
    #[tokio::test]
    async fn throws_for_5xx() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/packages/shared_preferences"))
            .respond_with(ResponseTemplate::new(502))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "shared_preferences", &http).await;
        assert!(result.is_err());
    }

    // Ported: "returns null for unknown error" — lib/modules/datasource/dart/index.spec.ts line 75
    // Network errors (connection refused) return None rather than Err.
    #[tokio::test]
    async fn returns_null_for_unknown_error() {
        let http = HttpClient::new().unwrap();
        // Use an invalid URL to trigger a network error.
        let result = fetch_releases("http://127.0.0.1:1", "pkg", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "processes real data" — lib/modules/datasource/dart/index.spec.ts line 85
    #[tokio::test]
    async fn processes_real_data() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/packages/shared_preferences"))
            .respond_with(ResponseTemplate::new(200).set_body_string(SHARED_PREFS_JSON))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "shared_preferences", &http)
            .await
            .unwrap()
            .unwrap();

        // 45 versions in fixture, 1 retracted → 44 non-retracted
        assert_eq!(result.releases.len(), 44);
        assert_eq!(result.releases[0].version, "0.1.1");
        assert_eq!(
            result.releases[0].release_timestamp.as_deref(),
            Some("2017-05-09T18:25:24.268Z")
        );
        assert_eq!(
            result.releases.last().map(|r| r.version.as_str()),
            Some("0.5.8")
        );
        assert_eq!(
            result.homepage.as_deref(),
            Some(
                "https://github.com/flutter/plugins/tree/master/packages/shared_preferences/shared_preferences"
            )
        );
        assert_eq!(
            result.source_url.as_deref(),
            Some("https://github.com/flutter/plugins")
        );
    }
}
