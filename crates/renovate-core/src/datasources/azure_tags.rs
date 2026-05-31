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
}
