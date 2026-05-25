//! Gitea tags datasource.
//!
//! Fetches tags from the Gitea Tags API for a given repository.
//!
//! Renovate reference: `lib/modules/datasource/gitea-tags/index.ts`
//! API: `GET {host}/api/v1/repos/{owner}/{repo}/tags`

use chrono::{DateTime, Utc};
use serde::Deserialize;
use thiserror::Error;

use crate::datasources::gitea_releases::get_api_url;
use crate::http::HttpClient;

pub const DEFAULT_REGISTRY_URL: &str = "https://gitea.com";
pub const DATASOURCE_ID: &str = "gitea-tags";

/// Errors from the Gitea tags datasource.
#[derive(Debug, Error)]
pub enum GiteaTagsError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

#[derive(Debug, Deserialize)]
struct TagCommit {
    sha: String,
    created: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiTag {
    name: String,
    commit: TagCommit,
}

#[derive(Debug, Deserialize)]
struct ApiTagEntry {
    commit: TagCommit,
}

#[derive(Debug, Deserialize)]
struct ApiCommitEntry {
    sha: String,
}

/// One tag entry returned by `fetch_releases`.
#[derive(Debug, Clone)]
pub struct GiteaTag {
    pub version: String,
    pub git_ref: String,
    pub new_digest: String,
    pub release_timestamp: Option<String>,
}

/// Result of a `fetch_releases` call.
#[derive(Debug, Clone)]
pub struct GiteaTagsResult {
    pub registry_url: String,
    pub source_url: String,
    pub releases: Vec<GiteaTag>,
}

/// Convert an RFC 3339 timestamp to UTC ISO 8601 with milliseconds.
fn rfc3339_to_utc_iso(s: &str) -> Option<String> {
    let dt = DateTime::parse_from_rfc3339(s).ok()?;
    let utc: DateTime<Utc> = dt.with_timezone(&Utc);
    let ms = utc.timestamp_subsec_millis();
    Some(format!("{}.{:03}Z", utc.format("%Y-%m-%dT%H:%M:%S"), ms))
}

/// Fetch Gitea tags for a repository.
///
/// Returns `None` for 4xx client errors. Propagates 5xx errors.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<GiteaTagsResult>, GiteaTagsError> {
    let api_url = get_api_url(registry_url);
    let url = format!("{}repos/{}/tags", api_url, package_name);

    let entries: Vec<ApiTag> = match http.get_json(&url).await {
        Ok(v) => v,
        Err(crate::http::HttpError::Status { status, .. }) if status.is_client_error() => {
            return Ok(None)
        }
        Err(crate::http::HttpError::Request(_)) => return Ok(None),
        Err(e) => return Err(GiteaTagsError::Http(e)),
    };

    let releases = entries
        .into_iter()
        .map(|t| GiteaTag {
            git_ref: t.name.clone(),
            version: t.name,
            new_digest: t.commit.sha,
            release_timestamp: t.commit.created.as_deref().and_then(rfc3339_to_utc_iso),
        })
        .collect();

    let base = registry_url.trim_end_matches('/');
    Ok(Some(GiteaTagsResult {
        registry_url: base.to_string(),
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
) -> Result<Option<String>, GiteaTagsError> {
    let api_url = get_api_url(registry_url);

    if let Some(tag) = new_value {
        let url = format!("{}repos/{}/tags/{}", api_url, package_name, tag);
        let entry: ApiTagEntry = match http.get_json(&url).await {
            Ok(v) => v,
            Err(crate::http::HttpError::Status { status, .. }) if status.is_client_error() => {
                return Ok(None)
            }
            Err(crate::http::HttpError::Request(_)) => return Ok(None),
            Err(e) => return Err(GiteaTagsError::Http(e)),
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
            return Ok(None)
        }
        Err(crate::http::HttpError::Request(_)) => return Ok(None),
        Err(e) => return Err(GiteaTagsError::Http(e)),
    };

    Ok(commits.into_iter().next().map(|c| c.sha))
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    // Ported: "returns tags from gitea.com" — gitea-tags/index.spec.ts line 9
    #[tokio::test]
    async fn returns_tags_from_gitea_com() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/gitea/helm-chart/tags"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "name": "v9.2.0",
                    "commit": { "sha": "35fcb41ce2d03b44186cc82d4ea619dc2fcb6f7d", "created": "2023-08-21T16:27:29Z" },
                },
                {
                    "name": "v9.1.0",
                    "commit": { "sha": "1ea6cb4633c2e01d02dc910bcb67d7710842abc7", "created": "2023-07-31T09:04:49+02:00" },
                },
                {
                    "name": "v9.0.4",
                    "commit": { "sha": "478fd6044e971d3c991e34fa449201397c2f5ea8", "created": "2023-07-22T14:06:30+02:00" },
                },
            ])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "gitea/helm-chart", &http)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(result.releases.len(), 3);
        assert_eq!(result.releases[0].version, "v9.2.0");
        assert_eq!(
            result.releases[0].new_digest,
            "35fcb41ce2d03b44186cc82d4ea619dc2fcb6f7d"
        );
        assert_eq!(
            result.releases[0].release_timestamp.as_deref(),
            Some("2023-08-21T16:27:29.000Z")
        );
        // Offset timezone converted to UTC
        assert_eq!(
            result.releases[1].release_timestamp.as_deref(),
            Some("2023-07-31T07:04:49.000Z")
        );
        assert!(result.source_url.ends_with("/gitea/helm-chart"));
    }

    // Ported: "returns tags from codeberg.org" — gitea-tags/index.spec.ts line 124
    #[tokio::test]
    async fn returns_tags_from_codeberg_org() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/forgejo-contrib/forgejo-helm/tags"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "name": "0.11.0",
                    "commit": { "sha": "7eb7edcfd00ffab9dddbae9e9b2deace305c9a84", "created": "2023-08-25T08:26:28Z" },
                },
                {
                    "name": "v0.10.1",
                    "commit": { "sha": "ecd9b535a128d4e6d643fde37bb05cd751b8d9f4", "created": "2023-08-02T20:30:48Z" },
                },
                {
                    "name": "v0.10.0",
                    "commit": { "sha": "5677cf39d4818da26931783a3438a660ae5f427d", "created": "2023-07-27T06:18:23Z" },
                },
            ])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result =
            fetch_releases(&server.uri(), "forgejo-contrib/forgejo-helm", &http)
                .await
                .unwrap()
                .unwrap();
        assert_eq!(result.releases.len(), 3);
        assert_eq!(result.releases[0].version, "0.11.0");
        assert_eq!(result.releases[1].version, "v0.10.1");
        assert_eq!(result.releases[2].version, "v0.10.0");
        assert!(result.source_url.ends_with("/forgejo-contrib/forgejo-helm"));
    }

    // Ported: "returns commits from codeberg.org" — gitea-tags/index.spec.ts line 209
    #[tokio::test]
    async fn returns_commits_from_codeberg_org() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/api/v1/repos/forgejo-contrib/forgejo-helm/commits",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                { "sha": "7eb7edcfd00ffab9dddbae9e9b2deace305c9a84" }
            ])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = get_digest(
            &server.uri(),
            "forgejo-contrib/forgejo-helm",
            None,
            &http,
        )
        .await
        .unwrap();
        assert_eq!(
            result.as_deref(),
            Some("7eb7edcfd00ffab9dddbae9e9b2deace305c9a84")
        );
    }

    // Ported: "returns commits from gitea.com" — gitea-tags/index.spec.ts line 256
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

    // Ported: "returns tags commit hash from gitea.com" — gitea-tags/index.spec.ts line 272
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
        let result = get_digest(
            &server.uri(),
            "gitea/helm-chart",
            Some("v9.0.1"),
            &http,
        )
        .await
        .unwrap();
        assert_eq!(
            result.as_deref(),
            Some("29c9bbb4bfec04ab22761cc2d999eb0fcb8acbed")
        );
    }
}
