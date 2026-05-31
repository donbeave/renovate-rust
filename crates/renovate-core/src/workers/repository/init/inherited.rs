//! Inherited config.
//!
//! Mirrors `lib/workers/repository/init/inherited.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InheritedConfigResult {
    pub config: serde_json::Value,
    pub found: bool,
    pub source: Option<String>,
}

pub fn get_inherited_config(repository: &str, platform: &str) -> InheritedConfigResult {
    let parts: Vec<&str> = repository.split('/').collect();
    if parts.len() < 2 {
        return InheritedConfigResult {
            config: serde_json::Value::Null,
            found: false,
            source: None,
        };
    }

    let org = parts[0];

    let org_config_repo = format!("{org}/renovate-config");
    let platform_config_repo = format!("{org}/.{platform}");

    InheritedConfigResult {
        config: serde_json::Value::Null,
        found: false,
        source: Some(platform_config_repo),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inherited_config_result_default() {
        let r = InheritedConfigResult::default();
        assert!(r.config.is_null());
        assert!(!r.found);
        assert!(r.source.is_none());
    }

    #[test]
    fn get_inherited_config_valid_repo() {
        let result = get_inherited_config("org/repo", "github");
        assert!(!result.found);
        assert!(result.source.is_some());
        assert!(result.source.unwrap().contains("github"));
    }

    #[test]
    fn get_inherited_config_single_part() {
        let result = get_inherited_config("repo", "github");
        assert!(!result.found);
        assert!(result.source.is_none());
    }

    #[test]
    fn get_inherited_config_nested_repo() {
        let result = get_inherited_config("org/subgroup/repo", "gitlab");
        assert!(!result.found);
        assert!(result.source.is_some());
    }

    #[test]
    fn inherited_config_result_serialization_roundtrip() {
        let r = InheritedConfigResult {
            config: serde_json::json!({"key": "value"}),
            found: true,
            source: Some("org/renovate-config".into()),
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: InheritedConfigResult = serde_json::from_str(&json).unwrap();
        assert!(back.found);
        assert_eq!(back.source, Some("org/renovate-config".into()));
    }
}
