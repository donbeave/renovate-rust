//! Python version datasource.
//!
//! Fetches available Python releases from python.org, filters by prebuilt
//! availability (containerbase/python-prebuild on GitHub Releases), and
//! annotates each release with EOL data from endoflife.date.
//!
//! Renovate reference: `lib/modules/datasource/python-version/index.ts`
//!
//! ## API
//!
//! `GET https://www.python.org/api/v2/downloads/release`
//! `GET https://api.github.com/repos/containerbase/python-prebuild/releases?per_page=100`
//! `GET https://endoflife.date/api/python.json`

use std::collections::{HashMap, HashSet};

use serde::Deserialize;
use thiserror::Error;

use crate::datasources::{endoflife, github_releases};
use crate::http::HttpClient;

pub const DEFAULT_REGISTRY_URL: &str = "https://www.python.org/api/v2/downloads/release";
pub const GITHUB_BASE_URL: &str = "https://api.github.com";
pub const EOL_REGISTRY: &str = "https://endoflife.date/api";
const PREBUILD_REPO: &str = "containerbase/python-prebuild";

#[derive(Debug, Error)]
pub enum PythonVersionError {
    #[error("external host error")]
    ExternalHost,
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("EOL fetch error: {0}")]
    Eol(#[from] endoflife::EolError),
    #[error("GitHub releases error: {0}")]
    Github(#[from] github_releases::GithubReleasesError),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PythonRelease {
    pub version: String,
    pub release_timestamp: Option<String>,
    pub is_stable: bool,
    pub is_deprecated: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct PythonReleasesResult {
    pub releases: Vec<PythonRelease>,
}

#[derive(Debug, Deserialize)]
struct RawPythonRelease {
    name: String,
    release_date: Option<String>,
    pre_release: bool,
}

fn parse_raw(raw: RawPythonRelease) -> PythonRelease {
    let version = raw
        .name
        .strip_prefix("Python ")
        .unwrap_or(&raw.name)
        .to_owned();
    PythonRelease {
        version,
        release_timestamp: raw.release_date,
        is_stable: !raw.pre_release,
        is_deprecated: None,
    }
}

fn build_eol_map(eol_result: Option<endoflife::EndoflifeResult>) -> HashMap<String, bool> {
    match eol_result {
        None => HashMap::new(),
        Some(result) => result
            .releases
            .into_iter()
            .map(|r| {
                let minor = r
                    .version
                    .splitn(3, '.')
                    .take(2)
                    .collect::<Vec<_>>()
                    .join(".");
                (minor, r.is_deprecated)
            })
            .collect(),
    }
}

fn annotate_with_eol(releases: &mut [PythonRelease], eol_map: &HashMap<String, bool>) {
    for release in releases.iter_mut() {
        let minor = release
            .version
            .splitn(3, '.')
            .take(2)
            .collect::<Vec<_>>()
            .join(".");
        release.is_deprecated = eol_map.get(&minor).copied();
    }
}

fn sort_releases(releases: &mut [PythonRelease]) {
    releases.sort_by(|a, b| {
        let parse = |v: &str| -> Vec<u32> { v.split('.').filter_map(|p| p.parse().ok()).collect() };
        parse(&a.version).cmp(&parse(&b.version))
    });
}

/// Fetch available Python releases.
///
/// - 429 from python.org → fall back to prebuild releases if any, else `Ok(None)`
/// - 5xx from python.org → `Err(PythonVersionError::ExternalHost)`
/// - network error from python.org → `Ok(None)` (fallback: true semantics)
/// - empty result after filtering → `Ok(None)`
pub async fn fetch_releases(
    registry_url: &str,
    github_api_base: &str,
    eol_registry: &str,
    http: &HttpClient,
) -> Result<Option<PythonReleasesResult>, PythonVersionError> {
    let prebuild = github_releases::fetch_all_releases(PREBUILD_REPO, http, github_api_base)
        .await
        .map_err(PythonVersionError::Github)?;
    let prebuild_set: HashSet<String> = prebuild.iter().map(|(tag, _)| tag.clone()).collect();

    let eol_result = endoflife::fetch_releases(eol_registry, "python", http)
        .await
        .ok()
        .flatten();
    let eol_map = build_eol_map(eol_result);

    // Use a non-retrying request so we can detect 429 immediately.
    let Ok(resp) = http.get(registry_url).send().await else { return Ok(None) };

    let status = resp.status();

    if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
        if prebuild.is_empty() {
            return Ok(None);
        }
        let mut releases: Vec<PythonRelease> = prebuild
            .into_iter()
            .map(|(tag, ts)| PythonRelease {
                version: tag,
                release_timestamp: ts,
                is_stable: true,
                is_deprecated: None,
            })
            .collect();
        annotate_with_eol(&mut releases, &eol_map);
        sort_releases(&mut releases);
        return Ok(Some(PythonReleasesResult { releases }));
    }

    if status.is_server_error() {
        return Err(PythonVersionError::ExternalHost);
    }

    if !status.is_success() {
        return Ok(None);
    }

    let Ok(body) = resp.text().await else { return Ok(None) };

    let raw_releases: Vec<RawPythonRelease> =
        serde_json::from_str(&body).map_err(PythonVersionError::Json)?;

    if raw_releases.is_empty() {
        return Ok(None);
    }

    let mut releases: Vec<PythonRelease> = raw_releases
        .into_iter()
        .map(parse_raw)
        .filter(|r| r.is_stable)
        .filter(|r| prebuild_set.contains(&r.version))
        .collect();

    if releases.is_empty() {
        return Ok(None);
    }

    annotate_with_eol(&mut releases, &eol_map);
    sort_releases(&mut releases);

    Ok(Some(PythonReleasesResult { releases }))
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    const EOL_JSON: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/python-version/__fixtures__/eol.json"
    );
    const RELEASE_JSON: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/python-version/__fixtures__/release.json"
    );

    const GITHUB_PATH: &str = "/repos/containerbase/python-prebuild/releases";
    const PYTHON_PATH: &str = "/api/v2/downloads/release";
    const EOL_PATH: &str = "/python.json";

    const PREBUILD_JSON: &str = r#"[
        {"tag_name":"3.12.1","prerelease":false,"draft":false,"name":"3.12.1"},
        {"tag_name":"3.12.0","prerelease":false,"draft":false,"name":"3.12.0"},
        {"tag_name":"3.7.8","prerelease":false,"draft":false,"name":"3.7.8"}
    ]"#;

    async fn mount_github(server: &MockServer) {
        Mock::given(method("GET"))
            .and(path(GITHUB_PATH))
            .respond_with(ResponseTemplate::new(200).set_body_string(PREBUILD_JSON))
            .mount(server)
            .await;
    }

    async fn mount_eol(server: &MockServer) {
        Mock::given(method("GET"))
            .and(path(EOL_PATH))
            .respond_with(ResponseTemplate::new(200).set_body_string(EOL_JSON))
            .mount(server)
            .await;
    }

    fn registry_url(server: &MockServer) -> String {
        format!("{}{}", server.uri(), PYTHON_PATH)
    }

    // Ported: "returns Python EOL data" — datasource/python-version/index.spec.ts line 14
    #[tokio::test]
    async fn returns_python_eol_data() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(EOL_PATH))
            .respond_with(ResponseTemplate::new(200).set_body_string(EOL_JSON))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = endoflife::fetch_releases(&server.uri(), "python", &http)
            .await
            .unwrap()
            .unwrap();
        let release = result
            .releases
            .iter()
            .find(|r| r.version == "3.7.17")
            .unwrap();
        assert!(release.is_deprecated);
    }

    // Ported: "throws for 500" — datasource/python-version/index.spec.ts line 63
    #[tokio::test]
    async fn throws_for_500() {
        let server = MockServer::start().await;
        mount_github(&server).await;
        mount_eol(&server).await;
        Mock::given(method("GET"))
            .and(path(PYTHON_PATH))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result =
            fetch_releases(&registry_url(&server), &server.uri(), &server.uri(), &http).await;
        assert!(result.is_err(), "5xx should propagate as Err");
    }

    // Ported: "returns null for error" — datasource/python-version/index.spec.ts line 73
    #[tokio::test]
    async fn returns_null_for_error() {
        let server = MockServer::start().await;
        mount_github(&server).await;
        mount_eol(&server).await;
        // Use an unreachable address for python.org to simulate a network/connection error.
        let bad_url = "http://127.0.0.1:1/api/v2/downloads/release";

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(bad_url, &server.uri(), &server.uri(), &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "falls back to prebuild releases on 429" — datasource/python-version/index.spec.ts line 83
    #[tokio::test]
    async fn falls_back_to_prebuild_releases_on_429() {
        let server = MockServer::start().await;
        mount_github(&server).await;
        mount_eol(&server).await;
        Mock::given(method("GET"))
            .and(path(PYTHON_PATH))
            .respond_with(ResponseTemplate::new(429))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&registry_url(&server), &server.uri(), &server.uri(), &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 3);
        let versions: Vec<&str> = result.releases.iter().map(|r| r.version.as_str()).collect();
        assert!(versions.contains(&"3.12.1"));
        assert!(versions.contains(&"3.12.0"));
        assert!(versions.contains(&"3.7.8"));

        let r78 = result
            .releases
            .iter()
            .find(|r| r.version == "3.7.8")
            .unwrap();
        assert_eq!(r78.is_deprecated, Some(true));

        let r121 = result
            .releases
            .iter()
            .find(|r| r.version == "3.12.1")
            .unwrap();
        assert_eq!(r121.is_deprecated, Some(false));
    }

    // Ported: "returns null on 429 when prebuild releases are unavailable" — datasource/python-version/index.spec.ts line 102
    #[tokio::test]
    async fn returns_null_on_429_when_prebuild_unavailable() {
        let server = MockServer::start().await;
        // No GitHub mock → wiremock returns 404 → fetch_all_releases returns empty Vec
        mount_eol(&server).await;
        Mock::given(method("GET"))
            .and(path(PYTHON_PATH))
            .respond_with(ResponseTemplate::new(429))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&registry_url(&server), &server.uri(), &server.uri(), &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null for empty 200 OK" — datasource/python-version/index.spec.ts line 116
    #[tokio::test]
    async fn returns_null_for_empty_200() {
        let server = MockServer::start().await;
        mount_github(&server).await;
        mount_eol(&server).await;
        Mock::given(method("GET"))
            .and(path(PYTHON_PATH))
            .respond_with(ResponseTemplate::new(200).set_body_string("[]"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&registry_url(&server), &server.uri(), &server.uri(), &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns the correct data" — datasource/python-version/index.spec.ts line 134
    #[tokio::test]
    async fn returns_the_correct_data() {
        let server = MockServer::start().await;
        mount_github(&server).await;
        mount_eol(&server).await;
        Mock::given(method("GET"))
            .and(path(PYTHON_PATH))
            .respond_with(ResponseTemplate::new(200).set_body_string(RELEASE_JSON))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&registry_url(&server), &server.uri(), &server.uri(), &http)
            .await
            .unwrap()
            .unwrap();

        // Sorted ascending: [3.7.8, 3.12.0]
        assert_eq!(result.releases[0].version, "3.7.8");
        assert_eq!(result.releases[0].is_deprecated, Some(true));
        assert!(result.releases[0].is_stable);
        assert_eq!(
            result.releases[0].release_timestamp.as_deref(),
            Some("2020-06-27T12:55:01Z")
        );
    }

    // Ported: "only returns stable versions" — datasource/python-version/index.spec.ts line 147
    #[tokio::test]
    async fn only_returns_stable_versions() {
        let server = MockServer::start().await;
        mount_github(&server).await;
        mount_eol(&server).await;
        Mock::given(method("GET"))
            .and(path(PYTHON_PATH))
            .respond_with(ResponseTemplate::new(200).set_body_string(RELEASE_JSON))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&registry_url(&server), &server.uri(), &server.uri(), &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 2);
        for release in &result.releases {
            assert!(release.is_stable);
        }
    }

    // Ported: "only returns versions that are prebuilt" — datasource/python-version/index.spec.ts line 158
    #[tokio::test]
    async fn only_returns_versions_that_are_prebuilt() {
        let server = MockServer::start().await;
        mount_github(&server).await;
        mount_eol(&server).await;
        Mock::given(method("GET"))
            .and(path(PYTHON_PATH))
            .respond_with(ResponseTemplate::new(200).set_body_string(RELEASE_JSON))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&registry_url(&server), &server.uri(), &server.uri(), &http)
            .await
            .unwrap()
            .unwrap();

        // Prebuilt: 3.12.1, 3.12.0, 3.7.8. Stable+prebuilt: 3.12.0 and 3.7.8.
        // Neither 3.12.2 nor 3.7.9 should appear.
        let versions: Vec<&str> = result.releases.iter().map(|r| r.version.as_str()).collect();
        assert!(!versions.contains(&"3.12.2"));
        assert!(!versions.contains(&"3.7.9"));
        assert!(!versions.contains(&"3.12.0a1"));
    }

    // Ported: "returns isDeprecated status for Python 3 minor releases" — datasource/python-version/index.spec.ts line 170
    #[tokio::test]
    async fn returns_is_deprecated_status() {
        let server = MockServer::start().await;
        mount_github(&server).await;
        mount_eol(&server).await;
        Mock::given(method("GET"))
            .and(path(PYTHON_PATH))
            .respond_with(ResponseTemplate::new(200).set_body_string(RELEASE_JSON))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&registry_url(&server), &server.uri(), &server.uri(), &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 2);
        for release in &result.releases {
            assert!(
                release.is_deprecated.is_some(),
                "release {} should have is_deprecated set",
                release.version
            );
        }
    }
}
