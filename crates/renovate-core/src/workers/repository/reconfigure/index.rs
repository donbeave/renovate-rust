//! Repository reconfiguration.
//!
//! Mirrors `lib/workers/repository/reconfigure/index.ts`.

use serde::{Deserialize, Serialize};

use crate::workers::types::RenovateConfig;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReconfigureResult {
    pub reconfigure_branch: Option<String>,
    pub config: Option<serde_json::Value>,
    pub processed: bool,
    pub errors: Vec<String>,
}

pub fn reconfigure_repository(
    _config: &RenovateConfig,
    branch_prefix: &str,
) -> ReconfigureResult {
    let reconfigure_branch = format!("{branch_prefix}reconfigure");

    ReconfigureResult {
        reconfigure_branch: Some(reconfigure_branch),
        config: None,
        processed: false,
        errors: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reconfigure_result_default() {
        let r = ReconfigureResult::default();
        assert!(r.reconfigure_branch.is_none());
        assert!(r.config.is_none());
        assert!(!r.processed);
        assert!(r.errors.is_empty());
    }

    #[test]
    fn reconfigure_repository_returns_result() {
        let config = RenovateConfig::default();
        let result = reconfigure_repository(&config, "renovate/");
        assert!(result.reconfigure_branch.is_some());
        assert_eq!(
            result.reconfigure_branch,
            Some("renovate/reconfigure".to_owned())
        );
    }

    #[test]
    fn reconfigure_repository_custom_prefix() {
        let config = RenovateConfig::default();
        let result = reconfigure_repository(&config, "custom/");
        assert_eq!(
            result.reconfigure_branch,
            Some("custom/reconfigure".to_owned())
        );
    }

    #[test]
    fn reconfigure_result_serialization_roundtrip() {
        let r = ReconfigureResult {
            reconfigure_branch: Some("renovate/reconfigure".into()),
            config: Some(serde_json::json!({"enabled": true})),
            processed: true,
            errors: vec![],
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: ReconfigureResult = serde_json::from_str(&json).unwrap();
        assert!(back.processed);
        assert!(back.config.is_some());
    }
}
