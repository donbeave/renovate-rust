//! Packagist datasource for PHP Composer packages.
//!
//! Fetches available versions from the Packagist metadata API v2.
//!
//! Renovate reference:
//! - `lib/modules/datasource/packagist/index.ts`
//! - API: `https://repo.packagist.org/p2/{vendor}/{package}.json`

use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::HttpClient;

pub const PACKAGIST_API: &str = "https://repo.packagist.org";

/// Errors from fetching Packagist metadata.
#[derive(Debug, Error)]
pub enum PackagistError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
}

/// Input for a single Packagist package lookup.
#[derive(Debug, Clone)]
pub struct PackagistDepInput {
    pub package_name: String,
    pub current_value: String,
}

/// Update summary for a Composer dependency.
#[derive(Debug, Clone)]
pub struct PackagistUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
    /// ISO 8601 release timestamp from the Packagist `time` field of the latest stable version.
    pub release_timestamp: Option<String>,
}

/// Per-dependency result from `fetch_updates_concurrent`.
#[derive(Debug)]
pub struct PackagistUpdateResult {
    pub package_name: String,
    pub summary: Result<PackagistUpdateSummary, PackagistError>,
}

// The p2 endpoint returns versions newest-first for each package.
#[derive(Debug, Deserialize)]
struct P2Response {
    packages: std::collections::HashMap<String, Vec<P2Version>>,
}

#[derive(Debug, Deserialize)]
struct P2Version {
    version: String,
    /// ISO 8601 release timestamp for this version.
    time: Option<String>,
}

/// Fetch the latest stable version of a Packagist package.
///
/// `package_name` must be `vendor/package` (e.g. `symfony/framework-bundle`).
pub async fn fetch_latest(
    package_name: &str,
    http: &HttpClient,
    api_base: &str,
) -> Result<Option<(String, Option<String>)>, PackagistError> {
    let url = format!("{api_base}/p2/{package_name}.json");

    let resp = http.get_retrying(&url).await?;
    if resp.status().as_u16() == 404 {
        return Ok(None);
    }
    if !resp.status().is_success() {
        return Ok(None);
    }

    let p2: P2Response = resp.json().await.map_err(PackagistError::Json)?;

    let Some(versions) = p2.packages.get(package_name) else {
        return Ok(None);
    };

    // Versions are newest-first; return the first stable release with its timestamp.
    let latest = versions
        .iter()
        .find(|v| is_stable(&v.version))
        .map(|v| (v.version.clone(), v.time.clone()));

    Ok(latest)
}

/// Fetch update summaries for multiple Composer packages concurrently.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[PackagistDepInput],
    api_base: &str,
    concurrency: usize,
) -> Vec<PackagistUpdateResult> {
    if deps.is_empty() {
        return Vec::new();
    }

    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<PackagistUpdateResult> = JoinSet::new();

    for dep in deps {
        let http = http.clone();
        let dep = dep.clone();
        let sem = Arc::clone(&sem);
        let api_base = api_base.to_owned();

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_update_summary(&dep, &http, &api_base).await;
            PackagistUpdateResult {
                package_name: dep.package_name.clone(),
                summary: result,
            }
        });
    }

    let mut results = Vec::with_capacity(deps.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok(r) => results.push(r),
            Err(join_err) => tracing::error!(%join_err, "packagist lookup task panicked"),
        }
    }
    results
}

async fn fetch_update_summary(
    dep: &PackagistDepInput,
    http: &HttpClient,
    api_base: &str,
) -> Result<PackagistUpdateSummary, PackagistError> {
    let result = fetch_latest(&dep.package_name, http, api_base).await?;
    let (latest_version, release_timestamp) =
        result.map(|(v, ts)| (Some(v), ts)).unwrap_or((None, None));
    let s = crate::versioning::semver_generic::semver_update_summary(
        &dep.current_value,
        latest_version.as_deref(),
    );
    Ok(PackagistUpdateSummary {
        current_value: s.current_value,
        latest: s.latest,
        update_available: s.update_available,
        release_timestamp,
    })
}

/// Returns `true` if a version string looks like a stable release.
///
/// Excludes versions containing `-dev`, `-alpha`, `-beta`, `-RC`, and
/// versions that look like branch aliases (`dev-*`).
fn is_stable(version: &str) -> bool {
    if version.starts_with("dev-") || version.ends_with("-dev") {
        return false;
    }
    let lower = version.to_ascii_lowercase();
    !lower.contains("-alpha")
        && !lower.contains("-beta")
        && !lower.contains("-rc")
        && !lower.contains("-patch")
        && !lower.contains("dev")
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    fn symfony_p2_json(package: &str, version: &str) -> String {
        serde_json::json!({
            "packages": {
                package: [
                    {"version": version, "version_normalized": "7.1.0.0"},
                    {"version": "v7.0.0", "version_normalized": "7.0.0.0"}
                ]
            }
        })
        .to_string()
    }

    #[test]
    fn is_stable_accepts_semver() {
        assert!(is_stable("v7.1.0"));
        assert!(is_stable("2.15.0"));
        assert!(is_stable("1.0.0"));
    }

    #[test]
    fn is_stable_rejects_prerelease() {
        assert!(!is_stable("v7.1.0-beta1"));
        assert!(!is_stable("v7.1.0-RC1"));
        assert!(!is_stable("dev-master"));
        assert!(!is_stable("1.x-dev"));
    }

    #[tokio::test]
    async fn fetch_latest_returns_first_stable() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/p2/symfony/framework-bundle.json"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(symfony_p2_json("symfony/framework-bundle", "v7.1.0")),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("symfony/framework-bundle", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result.map(|(v, _)| v), Some("v7.1.0".to_owned()));
    }

    #[tokio::test]
    async fn fetch_latest_404_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/p2/missing/package.json"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("missing/package", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn fetch_latest_skips_prerelease_picks_stable() {
        let server = MockServer::start().await;
        let body = serde_json::json!({
            "packages": {
                "vendor/pkg": [
                    {"version": "v8.0.0-RC1"},
                    {"version": "v7.1.0"},
                    {"version": "v7.0.0"}
                ]
            }
        })
        .to_string();
        Mock::given(method("GET"))
            .and(path("/p2/vendor/pkg.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("vendor/pkg", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result.map(|(v, _)| v), Some("v7.1.0".to_owned()));
    }
}
