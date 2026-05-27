//! Forgejo releases datasource.
//!
//! Forgejo is a soft-fork of Gitea with a compatible API.  This module
//! delegates to the shared Gitea implementation, exposing Forgejo-specific
//! constants.
//!
//! Renovate reference: `lib/modules/datasource/forgejo-releases/index.ts`
//! API: `GET {host}/api/v1/repos/{owner}/{repo}/releases?draft=false`

use crate::datasources::gitea_releases::{GiteaRelease, GiteaReleasesError, GiteaReleasesResult};
use crate::http::HttpClient;

pub const DEFAULT_REGISTRY_URL: &str = "https://code.forgejo.org";
pub const DATASOURCE_ID: &str = "forgejo-releases";

pub type ForgejoRelease = GiteaRelease;
pub type ForgejoReleasesResult = GiteaReleasesResult;
pub type ForgejoReleasesError = GiteaReleasesError;

/// Fetch Forgejo releases for a repository.
///
/// See [`crate::datasources::gitea_releases::fetch_releases`] for details.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<ForgejoReleasesResult>, ForgejoReleasesError> {
    crate::datasources::gitea_releases::fetch_releases(registry_url, package_name, http).await
}

/// Fetch the latest commit SHA or the SHA for a specific tag.
///
/// See [`crate::datasources::gitea_releases::get_digest`] for details.
pub async fn get_digest(
    registry_url: &str,
    package_name: &str,
    new_value: Option<&str>,
    http: &HttpClient,
) -> Result<Option<String>, ForgejoReleasesError> {
    crate::datasources::gitea_releases::get_digest(registry_url, package_name, new_value, http)
        .await
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    // Ported: "returns tags from forgejo.com" — forgejo-releases/index.spec.ts line 9
    #[tokio::test]
    async fn returns_tags_from_forgejo_com() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/forgejo/helm-chart/releases"))
            .and(query_param("draft", "false"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "id": 266974,
                    "tag_name": "v9.2.1",
                    "prerelease": false,
                    "published_at": "2023-08-27T12:56:50Z",
                }
            ])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "forgejo/helm-chart", &http)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(result.releases.len(), 1);
        assert_eq!(result.releases[0].version, "v9.2.1");
        assert_eq!(
            result.releases[0].release_timestamp.as_deref(),
            Some("2023-08-27T12:56:50.000Z")
        );
        assert!(result.releases[0].is_stable);
        assert!(result.source_url.ends_with("/forgejo/helm-chart"));
    }

    // Ported: "returns tags from codeberg.org" — forgejo-releases/index.spec.ts line 106
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
                    "published_at": "2023-07-31T07:04:49Z",
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
        assert!(result.source_url.ends_with("/forgejo-contrib/forgejo-helm"));
    }

    // Ported: "returns commits from codeberg.org" — forgejo-releases/index.spec.ts line 236
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

    // Ported: "returns commits from forgejo.com" — forgejo-releases/index.spec.ts line 283
    #[tokio::test]
    async fn returns_commits_from_forgejo_com_empty() {
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

    // Ported: "returns tags commit hash from forgejo.com" — forgejo-releases/index.spec.ts line 299
    #[tokio::test]
    async fn returns_tags_commit_hash_from_forgejo_com() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/forgejo/helm-chart/tags/v9.0.1"))
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
        let result = get_digest(&server.uri(), "forgejo/helm-chart", Some("v9.0.1"), &http)
            .await
            .unwrap();
        assert_eq!(
            result.as_deref(),
            Some("29c9bbb4bfec04ab22761cc2d999eb0fcb8acbed")
        );
    }
}
