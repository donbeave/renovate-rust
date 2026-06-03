//! GitLab releases datasource.
//!
//! Fetches releases from the GitLab Releases API for a given project.
//!
//! Renovate reference: `lib/modules/datasource/gitlab-releases/index.ts`
//! API: `GET {host}/api/v4/projects/{url_encoded_name}/releases`

use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY_URL: &str = "https://gitlab.com";
pub const DATASOURCE_ID: &str = "gitlab-releases";

/// Errors from the GitLab releases datasource.
#[derive(Debug, Error)]
pub enum GitlabReleasesError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

#[derive(Debug, Deserialize)]
struct ApiRelease {
    tag_name: String,
    released_at: Option<String>,
}

/// One release entry.
#[derive(Debug, Clone)]
pub struct GitlabRelease {
    pub version: String,
    pub git_ref: String,
    pub release_timestamp: Option<String>,
}

/// Result of a `fetch_releases` call.
#[derive(Debug, Clone)]
pub struct GitlabReleasesResult {
    pub releases: Vec<GitlabRelease>,
    pub source_url: String,
}

fn percent_encode_path(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' => c.to_string(),
            _ => format!("%{:02X}", c as u32),
        })
        .collect()
}

fn rfc3339_to_utc_iso(s: &str) -> Option<String> {
    let dt = DateTime::parse_from_rfc3339(s).ok()?;
    let utc: DateTime<Utc> = dt.with_timezone(&Utc);
    let ms = utc.timestamp_subsec_millis();
    Some(format!("{}.{:03}Z", utc.format("%Y-%m-%dT%H:%M:%S"), ms))
}

fn is_fatal_status(status: StatusCode) -> bool {
    status.is_server_error() || status == StatusCode::TOO_MANY_REQUESTS
}

/// Fetch GitLab releases for a project.
///
/// - `registry_url`: GitLab host (e.g. `"https://gitlab.com"`)
/// - `package_name`: project path like `"some/dep2"`
///
/// Returns `None` for 404 and other client errors. Propagates 5xx errors.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<GitlabReleasesResult>, GitlabReleasesError> {
    if registry_url.is_empty() {
        return Ok(None);
    }

    let encoded = percent_encode_path(package_name);
    let url = format!(
        "{}/api/v4/projects/{}/releases",
        registry_url.trim_end_matches('/'),
        encoded
    );

    let api_releases: Vec<ApiRelease> = match http.get_json(&url).await {
        Ok(v) => v,
        Err(crate::http::HttpError::Status { status, .. }) if is_fatal_status(status) => {
            return Err(GitlabReleasesError::Http(crate::http::HttpError::Status {
                status,
                url: url.clone(),
            }));
        }
        Err(crate::http::HttpError::Status { .. }) => return Ok(None),
        Err(crate::http::HttpError::Request(_)) => return Ok(None),
        Err(crate::http::HttpError::Parse(_)) => return Ok(None),
    };

    let releases: Vec<GitlabRelease> = api_releases
        .into_iter()
        .map(|r| GitlabRelease {
            git_ref: r.tag_name.clone(),
            version: r.tag_name,
            release_timestamp: r.released_at.as_deref().and_then(rfc3339_to_utc_iso),
        })
        .collect();

    let registry_url_trimmed = registry_url.trim_end_matches('/');
    Ok(Some(GitlabReleasesResult {
        source_url: format!("{registry_url_trimmed}/{package_name}"),
        releases,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "returns releases from custom registry" — lib/modules/datasource/gitlab-releases/index.spec.ts line 18
    #[test]
    fn parse_releases_from_api_response() {
        let json = r#"[{"tag_name":"v1.0.0","released_at":"2021-01-01T00:00:00.000Z"},{"tag_name":"v1.1.0","released_at":"2021-03-01T00:00:00.000Z"}]"#;
        let api_releases: Vec<ApiRelease> = serde_json::from_str(json).unwrap();
        assert_eq!(api_releases.len(), 2);
        assert_eq!(api_releases[0].tag_name, "v1.0.0");
        assert_eq!(
            api_releases[0].released_at.as_deref(),
            Some("2021-01-01T00:00:00.000Z")
        );
    }

    // Ported: "return null if not found" — lib/modules/datasource/gitlab-releases/index.spec.ts line 45
    #[test]
    fn client_errors_return_none() {
        // 4xx client errors are handled by is_fatal_status → false → Ok(None)
        assert!(!is_fatal_status(StatusCode::NOT_FOUND));
    }

    // Ported: "returns releases from default registry" — lib/modules/datasource/gitlab-releases/index.spec.ts line 32
    #[tokio::test]
    async fn returns_releases_from_default_registry() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v4/projects/some%2Fdep2/releases"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"tag_name": "v1.0.0", "released_at": "2021-01-01T00:00:00.000Z"},
                {"tag_name": "v1.1.0", "released_at": "2021-03-01T00:00:00.000Z"}
            ])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "some/dep2", &http)
            .await
            .unwrap()
            .expect("should return releases");
        assert_eq!(result.releases.len(), 2);
        assert_eq!(result.releases[0].version, "v1.0.0");
        assert_eq!(result.releases[1].version, "v1.1.0");
    }

    // Rust-specific: gitlab_releases behavior test
    #[test]
    fn rfc3339_utc_conversion() {
        assert_eq!(
            rfc3339_to_utc_iso("2021-01-01T00:00:00.000Z").as_deref(),
            Some("2021-01-01T00:00:00.000Z")
        );
        // Offset conversion: -06:00 → UTC +6h
        assert_eq!(
            rfc3339_to_utc_iso("2020-03-04T12:01:37.000-06:00").as_deref(),
            Some("2020-03-04T18:01:37.000Z")
        );
    }
}
