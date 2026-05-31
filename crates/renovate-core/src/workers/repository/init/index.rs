//! Repository initialization.
//!
//! Mirrors `lib/workers/repository/init/index.ts`.

use serde::{Deserialize, Serialize};

use crate::workers::types::RenovateConfig;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InitResult {
    pub config: RenovateConfig,
    pub default_branch: Option<String>,
    pub base_branches: Vec<String>,
    pub initialized: bool,
}

pub fn init_repository(config: &RenovateConfig) -> InitResult {
    InitResult {
        config: config.clone(),
        default_branch: None,
        base_branches: Vec::new(),
        initialized: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_result_default() {
        let r = InitResult::default();
        assert!(r.default_branch.is_none());
        assert!(r.base_branches.is_empty());
        assert!(!r.initialized);
    }

    #[test]
    fn init_repository_returns_result() {
        let config = RenovateConfig::default();
        let result = init_repository(&config);
        assert!(result.initialized);
        assert!(result.base_branches.is_empty());
    }

    #[test]
    fn init_result_serialization_roundtrip() {
        let r = InitResult {
            config: RenovateConfig::default(),
            default_branch: Some("main".into()),
            base_branches: vec!["main".into()],
            initialized: true,
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: InitResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.default_branch, Some("main".into()));
        assert!(back.initialized);
    }
}
