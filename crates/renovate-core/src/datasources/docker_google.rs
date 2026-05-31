//! Google Container Registry support for Docker datasource.
//!
//! Handles GCR and Artifact Registry tag listing.
//!
//! Renovate reference: `lib/modules/datasource/docker/google.ts`

use regex::Regex;
use std::sync::LazyLock;
use thiserror::Error;

use crate::http::HttpClient;

pub static GOOGLE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(((eu|us|asia)\.)?gcr\.io|[a-z0-9-]+-docker\.pkg\.dev)").unwrap()
});

#[derive(Debug, Error)]
pub enum GcrError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
}

pub fn is_google_registry(registry: &str) -> bool {
    GOOGLE_REGEX.is_match(registry)
}

pub async fn get_gcr_tags(
    http: &HttpClient,
    registry: &str,
    image: &str,
) -> Result<Vec<String>, GcrError> {
    let url = format!("https://{}/v2/{}/tags/list", registry, image);
    let resp = http.get_retrying(&url).await?;

    if !resp.status().is_success() {
        return Ok(Vec::new());
    }

    #[derive(serde::Deserialize)]
    struct TagsResponse {
        #[serde(default)]
        tags: Option<Vec<String>>,
    }

    let result: TagsResponse = resp.json().await?;
    Ok(result.tags.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_google_registry_gcr() {
        assert!(is_google_registry("gcr.io"));
    }

    #[test]
    fn is_google_registry_eu_gcr() {
        assert!(is_google_registry("eu.gcr.io"));
    }

    #[test]
    fn is_google_registry_artifact_registry() {
        assert!(is_google_registry("us-central1-docker.pkg.dev"));
    }

    #[test]
    fn is_google_registry_docker_hub() {
        assert!(!is_google_registry("hub.docker.com"));
    }

    #[test]
    fn is_google_registry_ghcr() {
        assert!(!is_google_registry("ghcr.io"));
    }
}
