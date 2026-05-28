//! Hackage datasource for Haskell packages.
//!
//! Fetches package versions from the Hackage registry.
//!
//! Renovate reference:
//! - `lib/modules/datasource/hackage/index.ts`
//! - API: `GET https://hackage.haskell.org/package/{name}.json`
//!
//! The response is a JSON object: `{"1.0.0": "normal", "0.9.0": "deprecated"}`.

use std::collections::HashMap;

use thiserror::Error;

use crate::http::HttpClient;

const HACKAGE_BASE: &str = "https://hackage.haskell.org";

pub const DEFAULT_REGISTRY_URL: &str = "https://hackage.haskell.org/";
pub const DATASOURCE_ID: &str = "hackage";

#[derive(Debug, Clone)]
pub struct HackageRelease {
    pub version: String,
    pub changelog_url: String,
    pub is_deprecated: bool,
}

#[derive(Debug, Clone)]
pub struct HackageResult {
    pub releases: Vec<HackageRelease>,
}

/// Build a single release entry (mirrors TypeScript versionToRelease).
pub fn version_to_release(
    version: &str,
    package_name: &str,
    registry_url: &str,
    deprecated: bool,
) -> HackageRelease {
    let base = registry_url.trim_end_matches('/');
    HackageRelease {
        version: version.to_owned(),
        changelog_url: format!("{}/package/{}-{}/changelog", base, package_name, version),
        is_deprecated: deprecated,
    }
}

/// Fetch all releases for a Hackage package.
/// Returns `None` for 404 or missing registryUrl.
/// Returns `Err` for 5xx.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<HackageResult>, HackageError> {
    let base = registry_url.trim_end_matches('/');
    let url = format!("{}/package/{}.json", base, urlencoding(package_name));

    let Ok(resp) = http.get_retrying(&url).await else { return Ok(None) };

    let status = resp.status();
    if status.is_client_error() {
        return Ok(None);
    }
    if status.is_server_error() {
        return Err(HackageError::Http(crate::http::HttpError::Status {
            status,
            url,
        }));
    }

    let versions: HashMap<String, String> = match resp.json().await {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    if versions.is_empty() {
        return Ok(None);
    }

    let mut releases: Vec<HackageRelease> = versions
        .iter()
        .map(|(v, status)| {
            version_to_release(v, package_name, registry_url, status == "deprecated")
        })
        .collect();

    releases.sort_by(|a, b| cmp_pvp(&a.version, &b.version));

    Ok(Some(HackageResult { releases }))
}

/// Errors from fetching Hackage package metadata.
#[derive(Debug, Error)]
pub enum HackageError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("package not found: {0}")]
    NotFound(String),
}

/// Update summary for a Hackage dep.
#[derive(Debug, Clone)]
pub struct HackageUpdateSummary {
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Fetch the latest non-deprecated version of a Hackage package.
pub async fn fetch_latest(
    http: &HttpClient,
    package_name: &str,
) -> Result<HackageUpdateSummary, HackageError> {
    let encoded = urlencoding(package_name);
    let url = format!("{HACKAGE_BASE}/package/{encoded}.json");
    let versions: HashMap<String, String> = http.get_json(&url).await?;

    // Filter out deprecated versions; find latest by version ordering.
    let mut valid: Vec<String> = versions
        .into_iter()
        .filter(|(_, status)| status != "deprecated")
        .map(|(v, _)| v)
        .collect();

    valid.sort_by(|a, b| cmp_pvp(a, b));
    let latest = valid.pop();

    Ok(HackageUpdateSummary {
        update_available: false, // caller compares
        latest,
    })
}

/// Simple URL encoding for package names (handles `+` in names).
fn urlencoding(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' => c.to_string(),
            _ => format!("%{:02X}", c as u8),
        })
        .collect()
}

/// Compare two PVP version strings numerically.
fn cmp_pvp(a: &str, b: &str) -> std::cmp::Ordering {
    let parts = |s: &str| -> Vec<u64> {
        s.split('.')
            .map(|p| p.parse::<u64>().unwrap_or(0))
            .collect()
    };
    let av = parts(a);
    let bv = parts(b);
    for i in 0..av.len().max(bv.len()) {
        let ai = av.get(i).copied().unwrap_or(0);
        let bi = bv.get(i).copied().unwrap_or(0);
        match ai.cmp(&bi) {
            std::cmp::Ordering::Equal => continue,
            other => return other,
        }
    }
    std::cmp::Ordering::Equal
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    #[test]
    fn cmp_pvp_ordering() {
        use std::cmp::Ordering::*;
        assert_eq!(cmp_pvp("2.0.0", "1.9.9"), Greater);
        assert_eq!(cmp_pvp("1.0.0", "1.0.0"), Equal);
        assert_eq!(cmp_pvp("4.7.0.0", "4.7.0.1"), Less);
    }

    // Ported: "should make release with given version" — datasource/hackage/index.spec.ts line 10
    #[test]
    fn version_to_release_sets_version() {
        let release = version_to_release("3.1.0", "base", "http://localhost/", false);
        assert_eq!(release.version, "3.1.0");
        assert_eq!(
            release.changelog_url,
            "http://localhost/package/base-3.1.0/changelog"
        );
        assert!(!release.is_deprecated);
    }

    // Ported: "returns null for 404" — datasource/hackage/index.spec.ts line 27
    #[tokio::test]
    async fn returns_null_for_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/package/base.json"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let registry_url = format!("{}/", server.uri());
        let result = fetch_releases(&registry_url, "base", &http).await.unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns releases for 200" — datasource/hackage/index.spec.ts line 33
    #[tokio::test]
    async fn returns_releases_for_200() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/package/base.json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "4.19.0.1": "deprecated",
                "4.20.0.1": "normal"
            })))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let registry_url = format!("{}/", server.uri());
        let result = fetch_releases(&registry_url, "base", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 2);
        assert_eq!(result.releases[0].version, "4.19.0.1");
        assert!(result.releases[0].is_deprecated);
        assert!(
            result.releases[0]
                .changelog_url
                .contains("base-4.19.0.1/changelog")
        );
        assert_eq!(result.releases[1].version, "4.20.0.1");
        assert!(!result.releases[1].is_deprecated);
    }
}
