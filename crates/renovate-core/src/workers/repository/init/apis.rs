//! API initialization.
//!
//! Mirrors `lib/workers/repository/init/apis.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiConfig {
    pub platform: Option<String>,
    pub endpoint: Option<String>,
    pub token: Option<String>,
    pub initialized: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiInitResult {
    pub config: ApiConfig,
    pub errors: Vec<String>,
}

pub fn init_apis(platform: &str, endpoint: Option<&str>) -> ApiInitResult {
    ApiInitResult {
        config: ApiConfig {
            platform: Some(platform.to_owned()),
            endpoint: endpoint.map(|s| s.to_owned()),
            token: None,
            initialized: true,
        },
        errors: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_config_default() {
        let c = ApiConfig::default();
        assert!(c.platform.is_none());
        assert!(!c.initialized);
    }

    #[test]
    fn api_init_result_default() {
        let r = ApiInitResult::default();
        assert!(r.errors.is_empty());
    }

    #[test]
    fn init_apis_github() {
        let result = init_apis("github", None);
        assert!(result.config.initialized);
        assert_eq!(result.config.platform, Some("github".to_owned()));
        assert!(result.config.endpoint.is_none());
        assert!(result.errors.is_empty());
    }

    #[test]
    fn init_apis_gitlab_with_endpoint() {
        let result = init_apis("gitlab", Some("https://gitlab.example.com/api/v4"));
        assert_eq!(result.config.endpoint, Some("https://gitlab.example.com/api/v4".to_owned()));
    }

    #[test]
    fn api_config_serialization_roundtrip() {
        let c = ApiConfig {
            platform: Some("github".into()),
            endpoint: Some("https://api.github.com".into()),
            token: None,
            initialized: true,
        };
        let json = serde_json::to_string(&c).unwrap();
        let back: ApiConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(back.platform, Some("github".into()));
    }
}
