//! Typst package registry datasource.
//!
//! Renovate reference: `lib/modules/datasource/typst/index.ts`
//! API: `GET https://packages.typst.org/preview/index.json`

use chrono::TimeZone;
use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY: &str = "https://packages.typst.org/preview/index.json";
pub const DATASOURCE_ID: &str = "typst";

#[derive(Debug, Error)]
pub enum TypstError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

#[derive(Debug, Clone)]
pub struct TypstRelease {
    pub version: String,
    pub release_timestamp: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TypstResult {
    pub releases: Vec<TypstRelease>,
    pub source_url: Option<String>,
}

// ── API types ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct TypstEntry {
    name: String,
    version: String,
    repository: Option<String>,
    #[serde(rename = "updatedAt")]
    updated_at: Option<i64>,
}

fn unix_to_timestamp(secs: i64) -> Option<String> {
    chrono::Utc
        .timestamp_opt(secs, 0)
        .single()
        .map(|dt| dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string())
}

/// Fetch Typst package releases.
///
/// Namespace other than "preview" → `Ok(None)`.
/// Package not found → `Ok(None)`. Any HTTP error → `Ok(None)`.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<TypstResult>, TypstError> {
    let (namespace, pkg) = match package_name.split_once('/') {
        Some(parts) => parts,
        None => return Ok(None),
    };

    if namespace != "preview" {
        return Ok(None);
    }

    let text = match http
        .get_raw_with_accept(registry_url, "application/json")
        .await
    {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    let entries: Vec<TypstEntry> = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    if entries.is_empty() {
        return Ok(None);
    }

    let mut source_url: Option<String> = None;
    let releases: Vec<TypstRelease> = entries
        .into_iter()
        .filter(|e| e.name == pkg)
        .map(|e| {
            source_url = e.repository.clone();
            TypstRelease {
                version: e.version,
                release_timestamp: e.updated_at.and_then(unix_to_timestamp),
            }
        })
        .collect();

    if releases.is_empty() {
        return Ok(None);
    }

    Ok(Some(TypstResult {
        releases,
        source_url,
    }))
}

/// Update summary used by pipeline.
#[derive(Debug)]
pub struct TypstUpdateSummary {
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Fetch latest version (pipeline helper).
pub async fn fetch_latest(
    http: &HttpClient,
    package_name: &str,
    current_value: &str,
) -> Result<TypstUpdateSummary, TypstError> {
    let result = fetch_releases(DEFAULT_REGISTRY, package_name, http).await?;
    let latest = result.and_then(|r| r.releases.into_iter().last().map(|rel| rel.version));
    let update_available = latest
        .as_deref()
        .map(|l| l != current_value)
        .unwrap_or(false);
    Ok(TypstUpdateSummary {
        latest,
        update_available,
    })
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    const EXAMPLE_REGISTRY: &str = r#"[
        {
            "name": "example-package",
            "version": "0.1.0",
            "entrypoint": "src/lib.typ",
            "authors": ["Author One <author@example.com>"],
            "license": "MIT",
            "description": "An example package",
            "repository": "https://github.com/example/repo",
            "keywords": ["example"],
            "updatedAt": 1704708827
        },
        {
            "name": "example-package",
            "version": "0.2.0",
            "entrypoint": "src/lib.typ",
            "authors": ["Author One <author@example.com>"],
            "license": "MIT",
            "description": "An example package",
            "repository": "https://github.com/example/repo",
            "keywords": ["example"],
            "updatedAt": 1704808827
        },
        {
            "name": "example-package",
            "version": "1.0.0",
            "entrypoint": "src/lib.typ",
            "authors": ["Author One <author@example.com>"],
            "license": "MIT",
            "description": "An example package",
            "repository": "https://github.com/example/repo",
            "keywords": ["example"],
            "updatedAt": 1704908827
        }
    ]"#;

    // Ported: "processes real data" — datasource/typst/index.spec.ts line 7
    #[tokio::test]
    async fn processes_real_data() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/preview/index.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(EXAMPLE_REGISTRY))
            .mount(&server)
            .await;

        let registry_url = format!("{}/preview/index.json", server.uri());
        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&registry_url, "preview/example-package", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            result.source_url.as_deref(),
            Some("https://github.com/example/repo")
        );
        assert_eq!(result.releases.len(), 3);
        assert_eq!(result.releases[0].version, "0.1.0");
        assert_eq!(
            result.releases[0].release_timestamp.as_deref(),
            Some("2024-01-08T10:13:47.000Z")
        );
        assert_eq!(result.releases[1].version, "0.2.0");
        assert_eq!(
            result.releases[1].release_timestamp.as_deref(),
            Some("2024-01-09T14:00:27.000Z")
        );
        assert_eq!(result.releases[2].version, "1.0.0");
        assert_eq!(
            result.releases[2].release_timestamp.as_deref(),
            Some("2024-01-10T17:47:07.000Z")
        );
    }

    // Ported: "returns null for unsupported namespace" — datasource/typst/index.spec.ts line 74
    #[tokio::test]
    async fn returns_null_for_unsupported_namespace() {
        let http = HttpClient::new().unwrap();
        let result = fetch_releases(DEFAULT_REGISTRY, "unsupported/example-package", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null when package not found in registry" — datasource/typst/index.spec.ts line 83
    #[tokio::test]
    async fn returns_null_when_package_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/preview/index.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(
                r#"[{"name":"other-package","version":"1.0.0","repository":"https://github.com/example/other","updatedAt":1704708827}]"#,
            ))
            .mount(&server)
            .await;

        let registry_url = format!("{}/preview/index.json", server.uri());
        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&registry_url, "preview/nonexistent-package", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "handles multiple versions of the same package" — datasource/typst/index.spec.ts line 111
    #[tokio::test]
    async fn handles_multiple_versions() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/preview/index.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(r#"[
                {"name":"multi-version","version":"0.1.0","repository":"https://github.com/example/multi","updatedAt":1704708827},
                {"name":"multi-version","version":"0.2.0","repository":"https://github.com/example/multi","updatedAt":1704808827}
            ]"#))
            .mount(&server)
            .await;

        let registry_url = format!("{}/preview/index.json", server.uri());
        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&registry_url, "preview/multi-version", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 2);
        assert_eq!(result.releases[0].version, "0.1.0");
        assert_eq!(result.releases[1].version, "0.2.0");
        assert_eq!(
            result.source_url.as_deref(),
            Some("https://github.com/example/multi")
        );
    }

    // Ported: "handles registry fetch errors" — datasource/typst/index.spec.ts line 163
    #[tokio::test]
    async fn handles_registry_fetch_errors() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/preview/index.json"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let registry_url = format!("{}/preview/index.json", server.uri());
        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&registry_url, "preview/error-package", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "handles empty registry response" — datasource/typst/index.spec.ts line 179
    #[tokio::test]
    async fn handles_empty_registry_response() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/preview/index.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string("[]"))
            .mount(&server)
            .await;

        let registry_url = format!("{}/preview/index.json", server.uri());
        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&registry_url, "preview/empty-package", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }
}
