//! Ansible Galaxy roles datasource (v1 API).
//!
//! Fetches role releases from the Ansible Galaxy v1 roles API.
//!
//! Renovate reference: `lib/modules/datasource/galaxy/index.ts`
//! API: `GET <registryUrl>api/v1/roles/?owner__username=<user>&name=<role>`

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY_URL: &str = "https://galaxy.ansible.com/";
pub const DATASOURCE_ID: &str = "galaxy";

/// Errors from the galaxy datasource.
#[derive(Debug, Error)]
pub enum GalaxyError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

#[derive(Debug, Deserialize)]
struct VersionEntry {
    name: String,
    created: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SummaryFields {
    versions: Vec<VersionEntry>,
}

#[derive(Debug, Deserialize)]
struct RoleResult {
    summary_fields: SummaryFields,
    github_user: Option<String>,
    github_repo: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    results: Vec<RoleResult>,
}

/// One Galaxy role release.
#[derive(Debug, Clone)]
pub struct GalaxyRelease {
    pub version: String,
    pub release_timestamp: Option<String>,
}

/// Result of a `fetch_releases` call.
#[derive(Debug, Clone)]
pub struct GalaxyResult {
    pub source_url: Option<String>,
    pub dependency_url: String,
    pub releases: Vec<GalaxyRelease>,
}

/// Fetch Ansible Galaxy role releases.
///
/// Returns `None` for 4xx, network errors, or empty/unmatched results.
/// Returns `Err` for 5xx server errors.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<GalaxyResult>, GalaxyError> {
    let parts: Vec<&str> = package_name.splitn(2, '.').collect();
    let user_name = parts[0];
    let project_name = parts.get(1).copied().unwrap_or("undefined");

    let base = registry_url.trim_end_matches('/');
    let api_url = format!(
        "{}/api/v1/roles/?owner__username={}&name={}",
        base, user_name, project_name
    );
    let project_url = format!("{}/{}/{}", base, user_name, project_name);

    let mut body: ApiResponse = match http.get_json(&api_url).await {
        Ok(v) => v,
        Err(crate::http::HttpError::Status { status, .. }) if status.is_client_error() => {
            return Ok(None);
        }
        Err(crate::http::HttpError::Request(_)) => return Ok(None),
        Err(e) => return Err(GalaxyError::Http(e)),
    };

    if body.results.len() > 1 {
        body.results
            .retain(|r| r.github_user.as_deref() == Some(user_name));
        if body.results.is_empty() {
            return Ok(None);
        }
    }

    if body.results.is_empty() {
        return Ok(None);
    }

    let result = &body.results[0];
    let source_url = match (&result.github_user, &result.github_repo) {
        (Some(user), Some(repo)) if !user.is_empty() && !repo.is_empty() => {
            Some(format!("https://github.com/{}/{}", user, repo))
        }
        _ => None,
    };

    let releases = result
        .summary_fields
        .versions
        .iter()
        .map(|v| GalaxyRelease {
            version: v.name.clone(),
            release_timestamp: v.created.clone(),
        })
        .collect();

    Ok(Some(GalaxyResult {
        source_url,
        dependency_url: project_url,
        releases,
    }))
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    // Ported: "returns null for empty result" — lib/modules/datasource/galaxy/index.spec.ts line 11
    #[tokio::test]
    async fn returns_null_for_empty_result() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/roles/"))
            .and(query_param("owner__username", "non_existent_crate"))
            .and(query_param("name", "undefined"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "non_existent_crate", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null for missing fields" — lib/modules/datasource/galaxy/index.spec.ts line 24
    #[tokio::test]
    async fn returns_null_for_missing_fields() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/roles/"))
            .and(query_param("owner__username", "non_existent_crate"))
            .and(query_param("name", "undefined"))
            .respond_with(ResponseTemplate::new(200).set_body_string("undefined"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "non_existent_crate", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null for empty list" — lib/modules/datasource/galaxy/index.spec.ts line 37
    #[tokio::test]
    async fn returns_null_for_empty_list() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/roles/"))
            .and(query_param("owner__username", "non_existent_crate"))
            .and(query_param("name", "undefined"))
            .respond_with(ResponseTemplate::new(200).set_body_string("\n"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "non_existent_crate", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null for 404" (username only) — lib/modules/datasource/galaxy/index.spec.ts line 50
    #[tokio::test]
    async fn returns_null_for_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "some_crate", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null for unknown error" — lib/modules/datasource/galaxy/index.spec.ts line 63
    #[tokio::test]
    async fn returns_null_for_request_error() {
        // Simulate a network/request error by parsing invalid JSON (leads to parse error)
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_string("invalid json blob"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "some_crate", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "processes real data" — lib/modules/datasource/galaxy/index.spec.ts line 76
    #[tokio::test]
    async fn processes_real_data() {
        let fixture = include_str!(
            "../../../../../renovate/lib/modules/datasource/galaxy/__fixtures__/timezone.json"
        );
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/roles/"))
            .and(query_param("owner__username", "yatesr"))
            .and(query_param("name", "timezone"))
            .respond_with(ResponseTemplate::new(200).set_body_string(fixture))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "yatesr.timezone", &http)
            .await
            .unwrap();
        assert!(result.is_some());
    }

    // Ported: "handles multiple results when one user matches exactly" — lib/modules/datasource/galaxy/index.spec.ts line 90
    #[tokio::test]
    async fn handles_multiple_results_matching_user() {
        let fixture = include_str!(
            "../../../../../renovate/lib/modules/datasource/galaxy/__fixtures__/datadog.json"
        );
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/roles/"))
            .and(query_param("owner__username", "datadog"))
            .and(query_param("name", "datadog"))
            .respond_with(ResponseTemplate::new(200).set_body_string(fixture))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "datadog.datadog", &http)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(result.releases.len(), 11);
    }

    // Ported: "rejects multiple results when no user matches exactly" — lib/modules/datasource/galaxy/index.spec.ts line 103
    #[tokio::test]
    async fn rejects_multiple_results_no_user_match() {
        let fixture = include_str!(
            "../../../../../renovate/lib/modules/datasource/galaxy/__fixtures__/datadog.json"
        );
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/roles/"))
            .and(query_param("owner__username", "nope"))
            .and(query_param("name", "nope"))
            .respond_with(ResponseTemplate::new(200).set_body_string(fixture))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "nope.nope", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "return null if searching random username and project name" — lib/modules/datasource/galaxy/index.spec.ts line 115
    #[tokio::test]
    async fn returns_null_for_empty_results() {
        let fixture = include_str!(
            "../../../../../renovate/lib/modules/datasource/galaxy/__fixtures__/empty"
        );
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/roles/"))
            .and(query_param("owner__username", "foo"))
            .and(query_param("name", "bar"))
            .respond_with(ResponseTemplate::new(200).set_body_string(fixture))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "foo.bar", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "throws for 5xx" — lib/modules/datasource/galaxy/index.spec.ts line 127
    #[tokio::test]
    async fn throws_for_5xx() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(502))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "some_crate", &http).await;
        assert!(result.is_err());
    }

    // Ported: "throws for 404" (foo.bar) — lib/modules/datasource/galaxy/index.spec.ts line 140
    #[tokio::test]
    async fn returns_null_for_404_dotted() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/roles/"))
            .and(query_param("owner__username", "foo"))
            .and(query_param("name", "bar"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&format!("{}/", server.uri()), "foo.bar", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }
}
