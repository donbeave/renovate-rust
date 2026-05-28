//! Gitea releases datasource.
//!
//! Fetches releases from the Gitea Releases API for a given repository.
//!
//! Renovate reference: `lib/modules/datasource/gitea-releases/index.ts`
//! API: `GET {host}/api/v1/repos/{owner}/{repo}/releases?draft=false`

use chrono::{DateTime, Utc};
use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY_URL: &str = "https://gitea.com";
pub const DATASOURCE_ID: &str = "gitea-releases";

/// Errors from the Gitea releases datasource.
#[derive(Debug, Error)]
pub enum GiteaReleasesError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

#[derive(Debug, Deserialize)]
struct ApiRelease {
    tag_name: String,
    published_at: Option<String>,
    prerelease: bool,
}

#[derive(Debug, Deserialize)]
struct CommitRef {
    sha: String,
}

#[derive(Debug, Deserialize)]
struct ApiCommitEntry {
    sha: String,
}

#[derive(Debug, Deserialize)]
struct ApiTagEntry {
    commit: CommitRef,
}

/// One release entry returned by `fetch_releases`.
#[derive(Debug, Clone)]
pub struct GiteaRelease {
    pub version: String,
    pub git_ref: String,
    pub release_timestamp: Option<String>,
    pub is_stable: bool,
}

/// Result of a `fetch_releases` call.
#[derive(Debug, Clone)]
pub struct GiteaReleasesResult {
    pub registry_url: String,
    pub source_url: String,
    pub releases: Vec<GiteaRelease>,
}

/// Strip /api/v1 suffix and append /api/v1/ to build the API base URL.
pub(crate) fn get_api_url(registry_url: &str) -> String {
    let base = registry_url.trim_end_matches("/api/v1");
    let base = base.trim_end_matches('/');
    format!("{}/api/v1/", base)
}

/// Convert an RFC 3339 timestamp to UTC ISO 8601 with milliseconds.
fn rfc3339_to_utc_iso(s: &str) -> Option<String> {
    let dt = DateTime::parse_from_rfc3339(s).ok()?;
    let utc: DateTime<Utc> = dt.with_timezone(&Utc);
    let ms = utc.timestamp_subsec_millis();
    Some(format!("{}.{:03}Z", utc.format("%Y-%m-%dT%H:%M:%S"), ms))
}

/// Fetch Gitea releases for a repository.
///
/// Returns `None` for 4xx client errors. Propagates 5xx errors.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<GiteaReleasesResult>, GiteaReleasesError> {
    let api_url = get_api_url(registry_url);
    let url = format!("{}repos/{}/releases?draft=false", api_url, package_name);

    let entries: Vec<ApiRelease> = match http.get_json(&url).await {
        Ok(v) => v,
        Err(crate::http::HttpError::Status { status, .. }) if status.is_client_error() => {
            return Ok(None);
        }
        Err(crate::http::HttpError::Request(_)) => return Ok(None),
        Err(e) => return Err(GiteaReleasesError::Http(e)),
    };

    let releases = entries
        .into_iter()
        .map(|r| GiteaRelease {
            git_ref: r.tag_name.clone(),
            version: r.tag_name,
            release_timestamp: r.published_at.as_deref().and_then(rfc3339_to_utc_iso),
            is_stable: !r.prerelease,
        })
        .collect();

    let base = registry_url.trim_end_matches('/');
    Ok(Some(GiteaReleasesResult {
        registry_url: base.to_owned(),
        source_url: format!("{}/{}", base, package_name),
        releases,
    }))
}

/// Fetch the latest commit SHA, or the SHA for a specific tag.
///
/// - `new_value = None`: returns SHA of the most recent commit on the default branch.
/// - `new_value = Some(tag)`: returns SHA of the commit that tag points to.
///
/// Returns `None` when there are no commits or on 4xx.
pub async fn get_digest(
    registry_url: &str,
    package_name: &str,
    new_value: Option<&str>,
    http: &HttpClient,
) -> Result<Option<String>, GiteaReleasesError> {
    let api_url = get_api_url(registry_url);

    if let Some(tag) = new_value {
        let url = format!("{}repos/{}/tags/{}", api_url, package_name, tag);
        let entry: ApiTagEntry = match http.get_json(&url).await {
            Ok(v) => v,
            Err(crate::http::HttpError::Status { status, .. }) if status.is_client_error() => {
                return Ok(None);
            }
            Err(crate::http::HttpError::Request(_)) => return Ok(None),
            Err(e) => return Err(GiteaReleasesError::Http(e)),
        };
        return Ok(Some(entry.commit.sha));
    }

    let url = format!(
        "{}repos/{}/commits?stat=false&verification=false&files=false&page=1&limit=1",
        api_url, package_name
    );
    let commits: Vec<ApiCommitEntry> = match http.get_json(&url).await {
        Ok(v) => v,
        Err(crate::http::HttpError::Status { status, .. }) if status.is_client_error() => {
            return Ok(None);
        }
        Err(crate::http::HttpError::Request(_)) => return Ok(None),
        Err(e) => return Err(GiteaReleasesError::Http(e)),
    };

    Ok(commits.into_iter().next().map(|c| c.sha))
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    // Ported: "returns tags from gitea.com" — gitea-releases/index.spec.ts line 9
    #[tokio::test]
    async fn returns_tags_from_gitea_com() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/gitea/helm-chart/releases"))
            .and(query_param("draft", "false"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": 266974,
                    "tag_name": "v9.2.1",
                    "name": "v9.2.1",
                    "body": "Fix a helm template check",
                    "prerelease": false,
                    "published_at": "2023-08-27T12:56:50Z",
                }
            ])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "gitea/helm-chart", &http)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(result.releases.len(), 1);
        assert_eq!(result.releases[0].version, "v9.2.1");
        assert_eq!(result.releases[0].git_ref, "v9.2.1");
        assert_eq!(
            result.releases[0].release_timestamp.as_deref(),
            Some("2023-08-27T12:56:50.000Z")
        );
        assert!(result.releases[0].is_stable);
        assert!(result.source_url.ends_with("/gitea/helm-chart"));
    }

    // Ported: "returns tags from codeberg.org" — gitea-releases/index.spec.ts line 100
    #[tokio::test]
    async fn returns_tags_from_codeberg_org() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/forgejo-contrib/forgejo-helm/releases"))
            .and(query_param("draft", "false"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": 1585652,
                    "tag_name": "0.11.0",
                    "prerelease": false,
                    "published_at": "2023-08-25T08:27:19Z",
                },
                {
                    "id": 1549992,
                    "tag_name": "v0.10.1",
                    "prerelease": false,
                    "published_at": "2023-08-02T20:31:19Z",
                },
                {
                    "id": 1533110,
                    "tag_name": "v0.10.0",
                    "prerelease": false,
                    "published_at": "2023-07-27T06:19:02Z",
                }
            ])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "forgejo-contrib/forgejo-helm", &http)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(result.releases.len(), 3);
        assert_eq!(result.releases[0].version, "0.11.0");
        assert_eq!(result.releases[1].version, "v0.10.1");
        assert_eq!(result.releases[2].version, "v0.10.0");
        assert_eq!(
            result.releases[2].release_timestamp.as_deref(),
            Some("2023-07-27T06:19:02.000Z")
        );
        assert!(result.source_url.ends_with("/forgejo-contrib/forgejo-helm"));
    }

    // Ported: "returns commits from codeberg.org" — gitea-releases/index.spec.ts line 230
    #[tokio::test]
    async fn returns_commits_from_codeberg_org() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/forgejo-contrib/forgejo-helm/commits"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "sha": "7eb7edcfd00ffab9dddbae9e9b2deace305c9a84",
                }
            ])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = get_digest(&server.uri(), "forgejo-contrib/forgejo-helm", None, &http)
            .await
            .unwrap();
        assert_eq!(
            result.as_deref(),
            Some("7eb7edcfd00ffab9dddbae9e9b2deace305c9a84")
        );
    }

    // Ported: "returns commits from gitea.com" — gitea-releases/index.spec.ts line 277
    #[tokio::test]
    async fn returns_commits_from_gitea_com_empty() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/some/dep2/commits"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = get_digest(&server.uri(), "some/dep2", None, &http)
            .await
            .unwrap();
        assert_eq!(result, None);
    }

    // Ported: "returns tags commit hash from gitea.com" — gitea-releases/index.spec.ts line 293
    #[tokio::test]
    async fn returns_tags_commit_hash_from_gitea_com() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/gitea/helm-chart/tags/v9.0.1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "name": "v9.0.1",
                "commit": {
                    "sha": "29c9bbb4bfec04ab22761cc2d999eb0fcb8acbed",
                    "created": "2023-07-19T08:42:55+02:00",
                }
            })))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = get_digest(&server.uri(), "gitea/helm-chart", Some("v9.0.1"), &http)
            .await
            .unwrap();
        assert_eq!(
            result.as_deref(),
            Some("29c9bbb4bfec04ab22761cc2d999eb0fcb8acbed")
        );
    }

    #[test]
    fn get_api_url_strips_api_v1_suffix() {
        assert_eq!(
            get_api_url("https://gitea.com/api/v1"),
            "https://gitea.com/api/v1/"
        );
        assert_eq!(
            get_api_url("https://gitea.com"),
            "https://gitea.com/api/v1/"
        );
        assert_eq!(
            get_api_url("https://codeberg.org"),
            "https://codeberg.org/api/v1/"
        );
    }
}
