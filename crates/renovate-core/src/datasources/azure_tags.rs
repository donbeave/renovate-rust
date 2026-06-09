//! Azure Tags datasource.
//!
//! Fetches versions from Azure DevOps Git tags API.
//!
//! Renovate reference: `lib/modules/datasource/azure-tags/index.ts`
//! API: `GET https://dev.azure.com/{org}/{project}/_apis/git/repositories/{repo}/refs?filter=heads/tags`

use thiserror::Error;

use crate::datasources::{Release, ReleaseResult};
use crate::http::HttpClient;

pub const DATASOURCE_ID: &str = "azure-tags";

#[derive(Debug, Error)]
pub enum AzureTagsError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
    #[error("no tags found for '{repo}'")]
    NotFound { repo: String },
}

#[derive(Debug, Clone)]
pub struct AzureTagsConfig {
    pub registry_url: String,
    pub repo: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct AzureRef {
    name: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct AzureRefsResponse {
    #[serde(default)]
    value: Vec<AzureRef>,
}

pub fn get_source_url(registry_url: &str, package_name: &str) -> String {
    let base = registry_url.trim_end_matches('/');
    format!("{}/_git/{}", base, package_name)
}

pub fn get_cache_key(registry_url: &str, repo: &str, tag_type: &str) -> String {
    format!("{}:{}:{}", registry_url, repo, tag_type)
}

pub async fn fetch_versions(
    http: &HttpClient,
    config: &AzureTagsConfig,
) -> Result<ReleaseResult, AzureTagsError> {
    let base = config.registry_url.trim_end_matches('/');
    let url = format!(
        "{}/_apis/git/repositories/{}/refs?filter=heads/tags",
        base, config.repo
    );

    let resp = http.get_retrying(&url).await?;

    if resp.status().as_u16() == 404 {
        return Err(AzureTagsError::NotFound {
            repo: config.repo.clone(),
        });
    }
    if !resp.status().is_success() {
        return Err(AzureTagsError::Http(crate::http::HttpError::Status {
            status: resp.status(),
            url,
        }));
    }

    let result: AzureRefsResponse = resp.json().await?;

    let releases: Vec<Release> = result
        .value
        .into_iter()
        .filter_map(|r| {
            let name = r.name?;
            let tag_name = name.strip_prefix("refs/heads/tags/").unwrap_or(&name);
            Some(Release {
                version: tag_name.to_owned(),
                git_ref: Some(tag_name.to_owned()),
                ..Default::default()
            })
        })
        .collect();

    Ok(ReleaseResult {
        source_url: Some(get_source_url(&config.registry_url, &config.repo)),
        releases,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn datasource_id_is_correct() {
        assert_eq!(DATASOURCE_ID, "azure-tags");
    }

    #[test]
    fn get_source_url_builds_correctly() {
        let url = get_source_url("https://dev.azure.com/org/project", "my-repo");
        assert_eq!(url, "https://dev.azure.com/org/project/_git/my-repo");
    }

    #[test]
    fn get_source_url_trims_trailing_slash() {
        let url = get_source_url("https://dev.azure.com/org/project/", "my-repo");
        assert_eq!(url, "https://dev.azure.com/org/project/_git/my-repo");
    }

    #[test]
    fn get_cache_key_combines_parts() {
        let key = get_cache_key("https://dev.azure.com/org/project", "repo", "tags");
        assert_eq!(key, "https://dev.azure.com/org/project:repo:tags");
    }

    // Ported: "returns tags from azure devops" — lib/modules/datasource/azure-tags/index.spec.ts line 20
    #[tokio::test]
    async fn returns_tags_from_azure_devops() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/_apis/git/repositories/repo/refs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "value": [
                    { "name": "refs/heads/tags/tag1" },
                    { "name": "refs/heads/tags/tag2" }
                ]
            })))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let config = AzureTagsConfig {
            registry_url: format!("{}/", server.uri()),
            repo: "repo".to_string(),
        };
        let result = fetch_versions(&http, &config).await.unwrap();
        let expected_source = format!("{}/_git/repo", server.uri().trim_end_matches('/'));
        assert_eq!(result.source_url.as_deref(), Some(expected_source.as_str()));
        assert_eq!(result.releases.len(), 2);
        assert_eq!(result.releases[0].version, "tag1");
        assert_eq!(result.releases[0].git_ref.as_deref(), Some("tag1"));
        assert_eq!(result.releases[1].version, "tag2");
        assert_eq!(result.releases[1].git_ref.as_deref(), Some("tag2"));
    }

    // Ported: "filters out undefined names" — lib/modules/datasource/azure-tags/index.spec.ts line 47
    #[tokio::test]
    async fn filters_out_undefined_names() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/_apis/git/repositories/repo/refs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "value": [
                    { "name": "refs/heads/tags/tag1" },
                    { "name": null },
                    { "name": "refs/heads/tags/tag2" },
                    {}
                ]
            })))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let config = AzureTagsConfig {
            registry_url: format!("{}/", server.uri()),
            repo: "repo".to_string(),
        };
        let result = fetch_versions(&http, &config).await.unwrap();
        assert_eq!(result.releases.len(), 2);
        assert_eq!(result.releases[0].version, "tag1");
        assert_eq!(result.releases[1].version, "tag2");
    }

    // Ported: "handles api errors" — lib/modules/datasource/azure-tags/index.spec.ts line 70
    #[tokio::test]
    async fn handles_api_errors() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/_apis/git/repositories/repo/refs"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let config = AzureTagsConfig {
            registry_url: format!("{}/", server.uri()),
            repo: "repo".to_string(),
        };
        let result = fetch_versions(&http, &config).await;
        assert!(result.is_err());
    }
}
