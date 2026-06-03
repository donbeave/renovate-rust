//! CircleCI Orb datasource.
//!
//! Renovate reference: `lib/modules/datasource/orb/index.ts`
//! API: `POST {registry}/graphql-unstable`

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY: &str = "https://circleci.com";
pub const DATASOURCE_ID: &str = "orb";
const MAX_VERSIONS: u32 = 100;
const ORB_QUERY: &str = "query($packageName: String!, $maxVersions: Int!) { \
  orb(name: $packageName) { name, homeUrl, isPrivate, \
  versions(count: $maxVersions) { version, createdAt } } }";

#[derive(Debug, Error)]
pub enum OrbError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

#[derive(Debug, Clone)]
pub struct OrbRelease {
    pub version: String,
    pub release_timestamp: Option<String>,
}

#[derive(Debug, Clone)]
pub struct OrbResult {
    pub releases: Vec<OrbRelease>,
    pub homepage: Option<String>,
    pub is_private: bool,
}

// ── API types ──────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
struct GqlRequest<'a> {
    query: &'a str,
    variables: GqlVariables<'a>,
}

#[derive(Debug, Serialize)]
struct GqlVariables<'a> {
    #[serde(rename = "packageName")]
    package_name: &'a str,
    #[serde(rename = "maxVersions")]
    max_versions: u32,
}

#[derive(Debug, Deserialize)]
struct GqlResponse {
    data: Option<GqlData>,
}

#[derive(Debug, Deserialize)]
struct GqlData {
    orb: Option<OrbApiData>,
}

#[derive(Debug, Deserialize)]
struct OrbApiData {
    #[serde(rename = "homeUrl")]
    home_url: Option<String>,
    #[serde(default, rename = "isPrivate")]
    is_private: bool,
    #[serde(default)]
    versions: Vec<OrbVersion>,
}

#[derive(Debug, Deserialize)]
struct OrbVersion {
    version: String,
    #[serde(rename = "createdAt")]
    created_at: Option<String>,
}

fn normalize_timestamp(ts: &str) -> Option<String> {
    ts.parse::<chrono::DateTime<chrono::Utc>>()
        .ok()
        .map(|dt| dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string())
}

/// Fetch CircleCI Orb releases.
///
/// Missing orb / empty response → `Ok(None)`. All HTTP errors → `Ok(None)`.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<OrbResult>, OrbError> {
    let base = registry_url.trim_end_matches('/');
    let url = format!("{base}/graphql-unstable");

    let body = GqlRequest {
        query: ORB_QUERY,
        variables: GqlVariables {
            package_name,
            max_versions: MAX_VERSIONS,
        },
    };
    let body_str = serde_json::to_string(&body).unwrap_or_default();

    let resp: GqlResponse = match http.post_json(&url, &body_str).await {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    let Some(orb) = resp.data.and_then(|d| d.orb) else {
        return Ok(None);
    };

    let homepage_str = orb.home_url.as_deref().unwrap_or("").trim().to_owned();
    let homepage = if homepage_str.is_empty() {
        format!("https://circleci.com/developer/orbs/orb/{package_name}")
    } else {
        homepage_str
    };

    // GraphQL returns versions newest-first; reverse to get oldest-first
    // (matches Renovate's sortAndRemoveDuplicates which sorts by semver ascending).
    let mut releases: Vec<OrbRelease> = orb
        .versions
        .into_iter()
        .rev()
        .map(|v| OrbRelease {
            version: v.version,
            release_timestamp: v.created_at.as_deref().and_then(normalize_timestamp),
        })
        .collect();

    // Sort by semver ascending (stable sort preserves relative order for equal versions).
    releases.sort_by(|a, b| {
        let av = semver::Version::parse(&a.version).ok();
        let bv = semver::Version::parse(&b.version).ok();
        match (av, bv) {
            (Some(av), Some(bv)) => av.cmp(&bv),
            (Some(_), None) => std::cmp::Ordering::Greater,
            (None, Some(_)) => std::cmp::Ordering::Less,
            (None, None) => a.version.cmp(&b.version),
        }
    });

    Ok(Some(OrbResult {
        releases,
        homepage: Some(homepage),
        is_private: orb.is_private,
    }))
}

// ── Pipeline helpers ───────────────────────────────────────────────────────

/// Input for a single orb lookup.
#[derive(Debug, Clone)]
pub struct OrbDepInput {
    pub package_name: String,
    pub current_value: String,
}

/// Update summary for an orb dependency.
#[derive(Debug, Clone)]
pub struct OrbUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Per-dependency result from `fetch_updates_concurrent`.
#[derive(Debug)]
pub struct OrbUpdateResult {
    pub package_name: String,
    pub summary: Result<OrbUpdateSummary, OrbError>,
}

/// Fetch update summaries for multiple orb deps concurrently.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[OrbDepInput],
    concurrency: usize,
) -> Vec<OrbUpdateResult> {
    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<OrbUpdateResult> = JoinSet::new();

    for dep in deps {
        let http = http.clone();
        let dep = dep.clone();
        let sem = Arc::clone(&sem);

        set.spawn(async move {
            let _permit = sem.acquire().await.expect("semaphore closed");
            let result = fetch_releases(DEFAULT_REGISTRY, &dep.package_name, &http).await;
            let summary = match result {
                Err(e) => Err(e),
                Ok(None) => Ok(OrbUpdateSummary {
                    current_value: dep.current_value.clone(),
                    latest: None,
                    update_available: false,
                }),
                Ok(Some(r)) => {
                    let latest = r.releases.last().map(|rel| rel.version.clone());
                    let update_available = latest
                        .as_deref()
                        .map(|l| l != dep.current_value)
                        .unwrap_or(false);
                    Ok(OrbUpdateSummary {
                        current_value: dep.current_value.clone(),
                        latest,
                        update_available,
                    })
                }
            };
            OrbUpdateResult {
                package_name: dep.package_name.clone(),
                summary,
            }
        });
    }

    let mut results = Vec::with_capacity(deps.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok(r) => results.push(r),
            Err(join_err) => tracing::error!(%join_err, "orb lookup task panicked"),
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    const ORB_DATA: &str = r#"{
        "data": {
            "orb": {
                "name": "hutson/library-release-workflows",
                "homeUrl": "",
                "versions": [
                    {"version": "4.2.0", "createdAt": "2018-12-13T23:19:09.356Z"},
                    {"version": "4.1.6", "createdAt": "2018-12-12T18:56:42.563Z"},
                    {"version": "4.1.5", "createdAt": "2018-12-12T17:13:31.542Z"},
                    {"version": "4.1.4", "createdAt": "2018-12-11T22:13:29.297Z"},
                    {"version": "4.1.3", "createdAt": "2018-12-11T21:40:44.870Z"},
                    {"version": "4.1.2", "createdAt": "2018-12-11T21:28:37.846Z"},
                    {"version": "4.1.1"},
                    {"version": "4.1.0", "createdAt": "2018-12-11T18:14:41.116Z"},
                    {"version": "4.0.0", "createdAt": "2018-12-11T17:41:26.595Z"},
                    {"version": "3.0.0", "createdAt": "2018-12-11T05:28:14.080Z"}
                ]
            }
        }
    }"#;

    // Ported: "returns null for empty result" — lib/modules/datasource/orb/index.spec.ts line 32
    #[tokio::test]
    async fn returns_null_for_empty_result() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/graphql-unstable"))
            .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(
            &server.uri(),
            "hyper-expanse/library-release-workflows",
            &http,
        )
        .await
        .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null for missing orb" — lib/modules/datasource/orb/index.spec.ts line 42
    #[tokio::test]
    async fn returns_null_for_missing_orb() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/graphql-unstable"))
            .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"data":{}}"#))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(
            &server.uri(),
            "hyper-expanse/library-release-wonkflows",
            &http,
        )
        .await
        .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null for 404" — lib/modules/datasource/orb/index.spec.ts line 55
    #[tokio::test]
    async fn returns_null_for_404() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/graphql-unstable"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(
            &server.uri(),
            "hyper-expanse/library-release-workflows",
            &http,
        )
        .await
        .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null for unknown error" — lib/modules/datasource/orb/index.spec.ts line 65
    #[tokio::test]
    async fn returns_null_for_unknown_error() {
        let http = HttpClient::new().unwrap();
        let result = fetch_releases(
            "http://127.0.0.1:1",
            "hyper-expanse/library-release-workflows",
            &http,
        )
        .await
        .unwrap();
        assert!(result.is_none());
    }

    // Ported: "processes real data" — lib/modules/datasource/orb/index.spec.ts line 75
    #[tokio::test]
    async fn processes_real_data() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/graphql-unstable"))
            .respond_with(ResponseTemplate::new(200).set_body_string(ORB_DATA))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let package_name = "hyper-expanse/library-release-workflows";
        let result = fetch_releases(&server.uri(), package_name, &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 10);
        // Sorted by semver ascending
        assert_eq!(result.releases[0].version, "3.0.0");
        assert_eq!(
            result.releases[0].release_timestamp.as_deref(),
            Some("2018-12-11T05:28:14.080Z")
        );
        // Version without createdAt has no timestamp
        let r_411 = result
            .releases
            .iter()
            .find(|r| r.version == "4.1.1")
            .unwrap();
        assert!(r_411.release_timestamp.is_none());
        assert_eq!(result.releases[9].version, "4.2.0");
        assert_eq!(
            result.releases[9].release_timestamp.as_deref(),
            Some("2018-12-13T23:19:09.356Z")
        );
        // homeUrl empty → default URL
        assert_eq!(
            result.homepage.as_deref(),
            Some("https://circleci.com/developer/orbs/orb/hyper-expanse/library-release-workflows")
        );
        assert!(!result.is_private);
    }

    // Ported: "processes homeUrl" — lib/modules/datasource/orb/index.spec.ts line 85
    #[tokio::test]
    async fn processes_home_url() {
        let server = MockServer::start().await;
        let body = ORB_DATA.replace(r#""homeUrl": """#, r#""homeUrl": "https://google.com""#);
        Mock::given(method("POST"))
            .and(path("/graphql-unstable"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(
            &server.uri(),
            "hyper-expanse/library-release-workflows",
            &http,
        )
        .await
        .unwrap()
        .unwrap();

        assert_eq!(result.homepage.as_deref(), Some("https://google.com"));
    }

    // Ported: "supports other registries" — lib/modules/datasource/orb/index.spec.ts line 96
    #[tokio::test]
    async fn supports_other_registries() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/graphql-unstable"))
            .respond_with(ResponseTemplate::new(200).set_body_string(ORB_DATA))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        // Use the mock server as a custom registry
        let result = fetch_releases(
            &server.uri(),
            "hyper-expanse/library-release-workflows",
            &http,
        )
        .await
        .unwrap();
        assert!(result.is_some());
    }
}
