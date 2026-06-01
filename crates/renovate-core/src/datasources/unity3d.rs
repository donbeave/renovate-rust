//! Unity3D Editor version datasource.
//!
//! Fetches available Unity Editor versions from the Unity Releases API.
//!
//! Renovate reference: `lib/modules/datasource/unity3d/index.ts`
//!
//! ## API
//!
//! `GET https://services.api.unity.com/unity/editor/release/v1/releases?stream=LTS&limit=25&offset=0`
//!
//! Response: `{ "total": N, "results": [{ "version": "...", "releaseDate": "...", "shortRevision": "...", "releaseNotes": {"url": "..."} }] }`

use serde::Deserialize;

use crate::http::HttpClient;

pub const BASE_URL: &str = "https://services.api.unity.com/unity/editor/release/v1/releases";
pub const STREAM_LTS: &str =
    "https://services.api.unity.com/unity/editor/release/v1/releases?stream=LTS";
pub const STREAM_TECH: &str =
    "https://services.api.unity.com/unity/editor/release/v1/releases?stream=TECH";
pub const STREAM_ALPHA: &str =
    "https://services.api.unity.com/unity/editor/release/v1/releases?stream=ALPHA";
pub const STREAM_BETA: &str =
    "https://services.api.unity.com/unity/editor/release/v1/releases?stream=BETA";

pub const LEGACY_LTS: &str = "https://unity.com/releases/editor/lts-releases.xml";
pub const LEGACY_STABLE: &str = "https://unity.com/releases/editor/releases.xml";
pub const LEGACY_BETA: &str = "https://unity.com/releases/editor/beta/latest.xml";

const HOMEPAGE: &str = "https://unity.com/";
const PAGE_LIMIT: u64 = 25;

/// A single release returned by `fetch_releases`.
#[derive(Debug, Clone, PartialEq)]
pub struct UnityRelease {
    pub version: String,
    pub release_timestamp: Option<String>,
    pub changelog_url: Option<String>,
    pub is_stable: bool,
}

/// Full result from `fetch_releases`.
#[derive(Debug, Clone, PartialEq)]
pub struct UnityReleasesResult {
    pub releases: Vec<UnityRelease>,
    pub homepage: String,
    pub registry_url: String,
}

#[derive(Debug, Deserialize)]
struct ApiRelease {
    version: String,
    #[serde(rename = "releaseDate")]
    release_date: Option<String>,
    #[serde(rename = "releaseNotes")]
    release_notes: ApiReleaseNotes,
    #[serde(rename = "shortRevision")]
    short_revision: String,
}

#[derive(Debug, Deserialize)]
struct ApiReleaseNotes {
    url: String,
}

#[derive(Debug, Deserialize)]
struct ApiPage {
    total: u64,
    results: Vec<ApiRelease>,
}

/// Update summary for the legacy `fetch_latest_lts` API.
#[derive(Debug)]
pub struct Unity3dUpdateSummary {
    pub latest: Option<String>,
    pub latest_with_revision: Option<String>,
    pub update_available: bool,
}

/// Error from the Unity3D datasource.
#[derive(Debug, thiserror::Error)]
pub enum Unity3dError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(reqwest::Error),
}

/// Map a legacy stream URL to the current API stream URL.
///
/// Mirrors `translateStream` from `unity3d/index.ts`.
pub fn translate_stream(registry_url: &str) -> &str {
    match registry_url {
        LEGACY_LTS | LEGACY_STABLE => STREAM_LTS,
        LEGACY_BETA => STREAM_BETA,
        other => other,
    }
}

/// Returns true if `url` targets the LTS stream (contains `stream=LTS`).
fn is_lts_stream(url: &str) -> bool {
    url.to_ascii_uppercase().contains("STREAM=LTS")
}

/// Fetch all releases for `registry_url`.
///
/// - `with_hash`: include `shortRevision` in the version string (for `m_EditorVersionWithRevision`)
/// - Paginates through all result pages.
/// - `is_stable` = true for LTS stream only.
pub async fn fetch_releases(
    registry_url: &str,
    with_hash: bool,
    http: &HttpClient,
) -> Result<UnityReleasesResult, Unity3dError> {
    let stream_url = translate_stream(registry_url);
    let is_stable = is_lts_stream(stream_url);

    let mut releases = Vec::new();
    let mut offset: u64 = 0;
    let mut total: Option<u64> = None;

    loop {
        let url = format!("{stream_url}&limit={PAGE_LIMIT}&offset={offset}");
        let resp = http.get_retrying(&url).await?;
        let page: ApiPage = resp.json().await.map_err(Unity3dError::Json)?;

        total.get_or_insert(page.total);

        for r in page.results {
            releases.push(UnityRelease {
                version: if with_hash {
                    format!("{} ({})", r.version, r.short_revision)
                } else {
                    r.version
                },
                release_timestamp: r.release_date,
                changelog_url: Some(r.release_notes.url),
                is_stable,
            });
        }

        offset += PAGE_LIMIT;
        if offset >= total.unwrap_or(0) {
            break;
        }
    }

    Ok(UnityReleasesResult {
        releases,
        homepage: HOMEPAGE.to_owned(),
        registry_url: stream_url.to_owned(),
    })
}

/// Compatibility wrapper — fetch latest LTS version.
pub async fn fetch_latest_lts(
    http: &HttpClient,
    with_revision: bool,
) -> Result<Unity3dUpdateSummary, Unity3dError> {
    let result = fetch_releases(STREAM_LTS, with_revision, http).await?;
    let latest = result.releases.first().map(|r| r.version.clone());
    let latest_plain = result.releases.first().map(|r| {
        r.version
            .split_once(' ')
            .map(|(v, _)| v.to_owned())
            .unwrap_or_else(|| r.version.clone())
    });
    Ok(Unity3dUpdateSummary {
        latest: latest.clone(),
        latest_with_revision: if with_revision { latest } else { latest_plain },
        update_available: !result.releases.is_empty(),
    })
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    fn lts_fixture() -> serde_json::Value {
        serde_json::json!({
            "offset": 0, "limit": 25, "total": 2,
            "results": [
                {
                    "version": "6000.0.32f1",
                    "releaseDate": "2024-12-19T15:34:00.072Z",
                    "releaseNotes": {"url": "https://storage.googleapis.com/live-platform-resources-prd/templates/assets/6000_0_32f1_a564232097/6000_0_32f1_a564232097.md"},
                    "stream": "LTS",
                    "shortRevision": "b2e806cf271c",
                },
                {
                    "version": "2022.3.55f1",
                    "releaseDate": "2024-12-17T16:21:09.410Z",
                    "releaseNotes": {"url": "https://storage.googleapis.com/live-platform-resources-prd/templates/assets/2022_3_55f1_e905b1c414/2022_3_55f1_e905b1c414.md"},
                    "stream": "LTS",
                    "shortRevision": "9f374180d209",
                }
            ]
        })
    }

    fn tech_fixture() -> serde_json::Value {
        serde_json::json!({
            "offset": 0, "limit": 25, "total": 2,
            "results": [
                {
                    "version": "6000.0.22f1",
                    "releaseDate": "2024-10-02T19:04:27.205Z",
                    "releaseNotes": {"url": "https://storage.googleapis.com/live-platform-resources-prd/templates/assets/6000_0_22f1_bde815b68f/6000_0_22f1_bde815b68f.md"},
                    "stream": "TECH",
                    "shortRevision": "001fa5a8e29a",
                },
                {
                    "version": "6000.0.21f1",
                    "releaseDate": "2024-09-24T16:11:20.586Z",
                    "releaseNotes": {"url": "https://storage.googleapis.com/live-platform-resources-prd/templates/assets/6000_0_21f1_2b136c8c81/6000_0_21f1_2b136c8c81.md"},
                    "stream": "TECH",
                    "shortRevision": "e2bacb8dee3a",
                }
            ]
        })
    }

    fn alpha_fixture() -> serde_json::Value {
        serde_json::json!({
            "offset": 0, "limit": 25, "total": 2,
            "results": [
                {
                    "version": "6000.1.0a9",
                    "releaseDate": "2024-12-18T08:40:10.134Z",
                    "releaseNotes": {"url": "https://storage.googleapis.com/live-platform-resources-prd/templates/assets/6000_1_0a9_a19280e20b/6000_1_0a9_a19280e20b.md"},
                    "stream": "ALPHA",
                    "shortRevision": "a19280e20b",
                },
                {
                    "version": "6000.1.0a8",
                    "releaseDate": "2024-12-10T20:17:32.592Z",
                    "releaseNotes": {"url": "https://storage.googleapis.com/live-platform-resources-prd/templates/assets/6000_1_0a8_2d1304db16/6000_1_0a8_2d1304db16.md"},
                    "stream": "ALPHA",
                    "shortRevision": "2d1304db16",
                }
            ]
        })
    }

    fn beta_fixture() -> serde_json::Value {
        serde_json::json!({
            "offset": 0, "limit": 25, "total": 2,
            "results": [
                {
                    "version": "6000.0.0b16",
                    "releaseDate": "2024-04-19T15:47:47.012Z",
                    "releaseNotes": {"url": "https://storage.googleapis.com/live-platform-resources-prd/templates/assets/6000_0_0b16_c8ac27cff6/6000_0_0b16_c8ac27cff6.md"},
                    "stream": "BETA",
                    "shortRevision": "c8ac27cff6",
                },
                {
                    "version": "6000.0.0b15",
                    "releaseDate": "2024-04-13T00:46:31.309Z",
                    "releaseNotes": {"url": "https://storage.googleapis.com/live-platform-resources-prd/templates/assets/6000_0_0b15_d7e1e209b0/6000_0_0b15_d7e1e209b0.md"},
                    "stream": "BETA",
                    "shortRevision": "d7e1e209b0",
                }
            ]
        })
    }

    const API_PATH: &str = "/unity/editor/release/v1/releases";

    fn stream_param(stream: &str) -> &'static str {
        if stream.contains("LTS") {
            "LTS"
        } else if stream.contains("TECH") {
            "TECH"
        } else if stream.contains("ALPHA") {
            "ALPHA"
        } else {
            "BETA"
        }
    }

    async fn mount_stream(server: &MockServer, stream: &str, fixture: serde_json::Value) {
        Mock::given(method("GET"))
            .and(path(API_PATH))
            .and(query_param("stream", stream_param(stream)))
            .and(query_param("limit", "25"))
            .and(query_param("offset", "0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(fixture))
            .mount(server)
            .await;
    }

    fn make_stream_url(server: &MockServer, stream: &str) -> String {
        format!(
            "{}{}?stream={}",
            server.uri(),
            API_PATH,
            stream_param(stream)
        )
    }

    // Ported: "returns lts if requested %s" — datasource/unity3d/index.spec.ts line 52
    #[tokio::test]
    async fn returns_lts_if_requested() {
        let server = MockServer::start().await;
        mount_stream(&server, STREAM_LTS, lts_fixture()).await;

        let http = HttpClient::new().unwrap();
        let stream_url = make_stream_url(&server, STREAM_LTS);
        let result = fetch_releases(&stream_url, false, &http).await.unwrap();

        assert_eq!(result.homepage, "https://unity.com/");
        assert_eq!(result.releases.len(), 2);
        let versions: Vec<&str> = result.releases.iter().map(|r| r.version.as_str()).collect();
        assert!(versions.contains(&"6000.0.32f1"));
        assert!(versions.contains(&"2022.3.55f1"));
        assert!(result.releases.iter().all(|r| r.is_stable));
    }

    // Ported: "returns tech if requested" — datasource/unity3d/index.spec.ts line 88
    #[tokio::test]
    async fn returns_tech_if_requested() {
        let server = MockServer::start().await;
        mount_stream(&server, STREAM_TECH, tech_fixture()).await;

        let http = HttpClient::new().unwrap();
        let stream_url = make_stream_url(&server, STREAM_TECH);
        let result = fetch_releases(&stream_url, false, &http).await.unwrap();

        assert_eq!(result.releases.len(), 2);
        let versions: Vec<&str> = result.releases.iter().map(|r| r.version.as_str()).collect();
        assert!(versions.contains(&"6000.0.21f1"));
        assert!(versions.contains(&"6000.0.22f1"));
        assert!(result.releases.iter().all(|r| !r.is_stable));
    }

    // Ported: "returns alpha if requested" — datasource/unity3d/index.spec.ts line 120
    #[tokio::test]
    async fn returns_alpha_if_requested() {
        let server = MockServer::start().await;
        mount_stream(&server, STREAM_ALPHA, alpha_fixture()).await;

        let http = HttpClient::new().unwrap();
        let stream_url = make_stream_url(&server, STREAM_ALPHA);
        let result = fetch_releases(&stream_url, false, &http).await.unwrap();

        assert_eq!(result.releases.len(), 2);
        let versions: Vec<&str> = result.releases.iter().map(|r| r.version.as_str()).collect();
        assert!(versions.contains(&"6000.1.0a8"));
        assert!(versions.contains(&"6000.1.0a9"));
        assert!(result.releases.iter().all(|r| !r.is_stable));
    }

    // Ported: "returns beta if requested %s" — datasource/unity3d/index.spec.ts line 152
    #[tokio::test]
    async fn returns_beta_if_requested() {
        let server = MockServer::start().await;
        mount_stream(&server, STREAM_BETA, beta_fixture()).await;

        let http = HttpClient::new().unwrap();
        let stream_url = make_stream_url(&server, STREAM_BETA);
        let result = fetch_releases(&stream_url, false, &http).await.unwrap();

        assert_eq!(result.releases.len(), 2);
        let versions: Vec<&str> = result.releases.iter().map(|r| r.version.as_str()).collect();
        assert!(versions.contains(&"6000.0.0b15"));
        assert!(versions.contains(&"6000.0.0b16"));
        assert!(result.releases.iter().all(|r| !r.is_stable));
    }

    // Ported: "returns lts releases by default" — datasource/unity3d/index.spec.ts line 187
    #[tokio::test]
    async fn returns_lts_releases_by_default() {
        let server = MockServer::start().await;
        mount_stream(&server, STREAM_LTS, lts_fixture()).await;

        let http = HttpClient::new().unwrap();
        let stream_url = make_stream_url(&server, STREAM_LTS);
        let result = fetch_releases(&stream_url, false, &http).await.unwrap();

        assert!(result.releases.iter().all(|r| r.is_stable));
        assert!(!result.releases.iter().any(|r| r.version.contains('b')));
    }

    // Ported: "returns hash if requested" — datasource/unity3d/index.spec.ts line 235
    #[tokio::test]
    async fn returns_hash_if_requested() {
        let server = MockServer::start().await;
        mount_stream(&server, STREAM_LTS, lts_fixture()).await;

        let http = HttpClient::new().unwrap();
        let stream_url = make_stream_url(&server, STREAM_LTS);
        let result = fetch_releases(&stream_url, true, &http).await.unwrap();

        assert!(result.releases.iter().all(|r| r.version.contains('(')));
    }

    // Ported: "returns no hash if not requested" — datasource/unity3d/index.spec.ts line 258
    #[tokio::test]
    async fn returns_no_hash_if_not_requested() {
        let server = MockServer::start().await;
        mount_stream(&server, STREAM_LTS, lts_fixture()).await;

        let http = HttpClient::new().unwrap();
        let stream_url = make_stream_url(&server, STREAM_LTS);
        let result = fetch_releases(&stream_url, false, &http).await.unwrap();

        assert!(result.releases.iter().all(|r| !r.version.contains('(')));
    }

    // Ported: "returns only lts by default" — datasource/unity3d/index.spec.ts line 281
    #[tokio::test]
    async fn returns_only_lts_by_default() {
        let server = MockServer::start().await;
        mount_stream(&server, STREAM_LTS, lts_fixture()).await;

        let http = HttpClient::new().unwrap();
        let stream_url = make_stream_url(&server, STREAM_LTS);
        let result = fetch_releases(&stream_url, true, &http).await.unwrap();

        // All versions should have 'f' or 'p' (LTS), not 'b' or 'a' (prerelease)
        assert!(result.releases.iter().all(|r| {
            let v = r
                .version
                .split_once(' ')
                .map(|(v, _)| v)
                .unwrap_or(&r.version);
            v.contains('f') || v.contains('p')
        }));
    }

    // Ported: "uses pagination" — datasource/unity3d/index.spec.ts line 306
    #[tokio::test]
    async fn uses_pagination() {
        let server = MockServer::start().await;
        let path_suffix = "/unity/editor/release/v1/releases";
        let total = 30u64;

        // Page 1: 25 releases
        let mut page1_results = Vec::new();
        for i in 1..=25u32 {
            page1_results.push(serde_json::json!({
                "version": format!("6000.0.{i}f1"),
                "releaseDate": "2024-12-18T08:40:10.134Z",
                "releaseNotes": {"url": "testUrl"},
                "shortRevision": format!("{i:012x}"),
            }));
        }

        Mock::given(method("GET"))
            .and(path(path_suffix))
            .and(query_param("stream", "LTS"))
            .and(query_param("limit", "25"))
            .and(query_param("offset", "0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "total": total, "results": page1_results
            })))
            .mount(&server)
            .await;

        // Page 2: 5 releases
        let mut page2_results = Vec::new();
        for i in 26..=30u32 {
            page2_results.push(serde_json::json!({
                "version": format!("6000.0.{i}f1"),
                "releaseDate": "2024-12-18T08:40:10.134Z",
                "releaseNotes": {"url": "testUrl"},
                "shortRevision": format!("{i:012x}"),
            }));
        }

        Mock::given(method("GET"))
            .and(path(path_suffix))
            .and(query_param("stream", "LTS"))
            .and(query_param("limit", "25"))
            .and(query_param("offset", "25"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "total": total, "results": page2_results
            })))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let stream_url = format!(
            "{}/unity/editor/release/v1/releases?stream=LTS",
            server.uri()
        );
        let result = fetch_releases(&stream_url, false, &http).await.unwrap();

        assert_eq!(result.releases.len(), 30);
    }

    #[test]
    fn translate_stream_maps_legacy() {
        assert_eq!(
            translate_stream("https://unity.com/releases/editor/lts-releases.xml"),
            STREAM_LTS
        );
        assert_eq!(
            translate_stream("https://unity.com/releases/editor/beta/latest.xml"),
            STREAM_BETA
        );
        assert_eq!(
            translate_stream("https://example.com/custom"),
            "https://example.com/custom"
        );
    }
}
