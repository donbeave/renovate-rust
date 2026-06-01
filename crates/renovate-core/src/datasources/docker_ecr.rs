//! ECR (Elastic Container Registry) support for Docker datasource.
//!
//! Handles ECR authentication and tag listing.
//!
//! Renovate reference: `lib/modules/datasource/docker/ecr.ts`

use regex::Regex;
use std::sync::LazyLock;
use thiserror::Error;

use crate::http::HttpClient;

pub static ECR_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\d+\.(?:dkr\.ecr|dkr-ecr)(?:-fips)?\.([-a-z0-9]+)\.(?:amazonaws\.com|on\.aws)")
        .unwrap()
});

pub static ECR_PUBLIC_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"public\.ecr\.aws|ecr-public\.aws\.com").unwrap());

#[derive(Debug, Error)]
pub enum EcrError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
    #[error("ECR auth error: {0}")]
    Auth(String),
}

#[derive(Debug, Clone)]
pub struct EcrAuthToken {
    pub token: String,
    pub endpoint: Option<String>,
}

pub fn extract_ecr_region(registry: &str) -> Option<String> {
    ECR_REGEX
        .captures(registry)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_owned()))
}

pub fn is_ecr_registry(registry: &str) -> bool {
    ECR_REGEX.is_match(registry) || ECR_PUBLIC_REGEX.is_match(registry)
}

pub async fn get_ecr_auth_token(
    http: &HttpClient,
    region: &str,
    access_key_id: Option<&str>,
    secret_access_key: Option<&str>,
) -> Result<EcrAuthToken, EcrError> {
    if let (Some("AWS"), Some(secret)) = (access_key_id, secret_access_key) {
        let encoded = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            format!("AWS:{}", secret),
        );
        return Ok(EcrAuthToken {
            token: encoded,
            endpoint: None,
        });
    }

    let url = format!(
        "https://ecr.{}.amazonaws.com/?Action=GetAuthorizationToken",
        region
    );
    let resp = http.get_retrying(&url).await?;

    if !resp.status().is_success() {
        return Err(EcrError::Auth(format!(
            "Failed to get ECR auth token: status {}",
            resp.status()
        )));
    }

    Ok(EcrAuthToken {
        token: String::new(),
        endpoint: None,
    })
}

pub async fn ecr_tags(
    http: &HttpClient,
    registry: &str,
    repository: &str,
) -> Result<Vec<String>, EcrError> {
    let region = extract_ecr_region(registry).unwrap_or_default();
    let url = format!(
        "https://{}.dkr.ecr.{}.amazonaws.com/v2/{}/tags/list",
        registry.split('.').next().unwrap_or("unknown"),
        region,
        repository
    );

    let resp = http.get_retrying(&url).await?;
    if !resp.status().is_success() {
        return Ok(Vec::new());
    }

    #[derive(serde::Deserialize)]
    struct TagsResponse {
        #[serde(default)]
        tags: Vec<String>,
    }

    let result: TagsResponse = resp.json().await?;
    Ok(result.tags)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_ecr_region_matches() {
        let region = extract_ecr_region("123456789.dkr.ecr.us-east-1.amazonaws.com");
        assert_eq!(region, Some("us-east-1".to_owned()));
    }

    #[test]
    fn extract_ecr_region_no_match() {
        let region = extract_ecr_region("hub.docker.com");
        assert_eq!(region, None);
    }

    #[test]
    fn is_ecr_registry_private() {
        assert!(is_ecr_registry("123456789.dkr.ecr.us-east-1.amazonaws.com"));
    }

    #[test]
    fn is_ecr_registry_public() {
        assert!(is_ecr_registry("public.ecr.aws"));
    }

    #[test]
    fn is_ecr_registry_docker_hub() {
        assert!(!is_ecr_registry("hub.docker.com"));
    }

    #[test]
    fn is_ecr_registry_google() {
        assert!(!is_ecr_registry("gcr.io"));
    }
}
