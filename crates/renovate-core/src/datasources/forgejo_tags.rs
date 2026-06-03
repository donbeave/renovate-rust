//! Forgejo tags datasource.
//!
//! Forgejo is a soft-fork of Gitea with a compatible API.  This module
//! delegates to the shared Gitea tags implementation, exposing
//! Forgejo-specific constants.
//!
//! Renovate reference: `lib/modules/datasource/forgejo-tags/index.ts`
//! API: `GET {host}/api/v1/repos/{owner}/{repo}/tags`

use crate::datasources::gitea_tags::{GiteaTag, GiteaTagsError, GiteaTagsResult};
use crate::http::HttpClient;

pub const DEFAULT_REGISTRY_URL: &str = "https://code.forgejo.org";
pub const DATASOURCE_ID: &str = "forgejo-tags";

pub type ForgejoTag = GiteaTag;
pub type ForgejoTagsResult = GiteaTagsResult;
pub type ForgejoTagsError = GiteaTagsError;

/// Fetch Forgejo tags for a repository.
///
/// See [`crate::datasources::gitea_tags::fetch_releases`] for details.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<ForgejoTagsResult>, ForgejoTagsError> {
    crate::datasources::gitea_tags::fetch_releases(registry_url, package_name, http).await
}

/// Fetch the latest commit SHA or the SHA for a specific tag.
///
/// See [`crate::datasources::gitea_tags::get_digest`] for details.
pub async fn get_digest(
    registry_url: &str,
    package_name: &str,
    new_value: Option<&str>,
    http: &HttpClient,
) -> Result<Option<String>, ForgejoTagsError> {
    crate::datasources::gitea_tags::get_digest(registry_url, package_name, new_value, http).await
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    // Ported: "returns tags from code.forgejo.org" — lib/modules/datasource/forgejo-tags/index.spec.ts line 9
    #[tokio::test]
    async fn returns_tags_from_forgejo_org() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/forgejo/helm-chart/tags"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {
                    "name": "v9.2.0",
                    "commit": { "sha": "35fcb41ce2d03b44186cc82d4ea619dc2fcb6f7d", "created": "2023-08-21T16:27:29Z" },
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
        assert_eq!(result.releases[0].version, "v9.2.0");
        assert_eq!(
            result.releases[0].new_digest,
            "35fcb41ce2d03b44186cc82d4ea619dc2fcb6f7d"
        );
        assert!(result.source_url.ends_with("/forgejo/helm-chart"));
    }

    // Ported: "returns tags from codeberg.org" — lib/modules/datasource/forgejo-tags/index.spec.ts line 129
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
            ])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "forgejo-contrib/forgejo-helm", &http)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(result.releases.len(), 2);
        assert_eq!(result.releases[0].version, "0.11.0");
        assert_eq!(result.releases[1].version, "v0.10.1");
        assert!(result.source_url.ends_with("/forgejo-contrib/forgejo-helm"));
    }

    // Ported: "returns commits from codeberg.org" — lib/modules/datasource/forgejo-tags/index.spec.ts line 214
    #[tokio::test]
    async fn returns_commits_from_codeberg_org() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/forgejo-contrib/forgejo-helm/commits"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                { "sha": "7eb7edcfd00ffab9dddbae9e9b2deace305c9a84" }
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

    // Ported: "returns null from code.forgejo.org when no commits found" — lib/modules/datasource/forgejo-tags/index.spec.ts line 261
    #[tokio::test]
    async fn returns_null_when_no_commits() {
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

    // Ported: "returns tags commit hash from code.forgejo.org" — lib/modules/datasource/forgejo-tags/index.spec.ts line 277
    #[tokio::test]
    async fn returns_tags_commit_hash() {
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
