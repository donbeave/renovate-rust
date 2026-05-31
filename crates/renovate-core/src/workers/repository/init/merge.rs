//! Config merge.
//!
//! Mirrors `lib/workers/repository/init/merge.ts`.

use serde::{Deserialize, Serialize};

use crate::workers::types::RenovateConfig;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MergeResult {
    pub config: RenovateConfig,
    pub merged: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

pub fn merge_configs(
    base_config: &RenovateConfig,
    override_config: Option<&RenovateConfig>,
) -> MergeResult {
    let Some(override_config) = override_config else {
        return MergeResult {
            config: base_config.clone(),
            merged: false,
            errors: Vec::new(),
            warnings: Vec::new(),
        };
    };

    let mut merged = base_config.clone();

    if let Some(v) = &override_config.branch_prefix {
        merged.branch_prefix = Some(v.clone());
    }
    if let Some(v) = &override_config.additional_branch_prefix {
        merged.additional_branch_prefix = Some(v.clone());
    }
    if let Some(v) = &override_config.branch_name {
        merged.branch_name = Some(v.clone());
    }
    if let Some(v) = &override_config.commit_message {
        merged.commit_message = Some(v.clone());
    }
    if let Some(v) = &override_config.enabled {
        merged.enabled = Some(*v);
    }
    if let Some(v) = &override_config.managers {
        merged.managers = Some(v.clone());
    }
    if let Some(v) = &override_config.datasources {
        merged.datasources = Some(v.clone());
    }
    if let Some(v) = &override_config.labels {
        merged.labels = Some(v.clone());
    }
    if let Some(v) = &override_config.pr_hourly_limit {
        merged.pr_hourly_limit = Some(*v);
    }

    MergeResult {
        config: merged,
        merged: true,
        errors: Vec::new(),
        warnings: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_result_default() {
        let r = MergeResult::default();
        assert!(!r.merged);
        assert!(r.errors.is_empty());
        assert!(r.warnings.is_empty());
    }

    #[test]
    fn merge_configs_no_override() {
        let base = RenovateConfig::default();
        let result = merge_configs(&base, None);
        assert!(!result.merged);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn merge_configs_with_override() {
        let base = RenovateConfig::default();
        let override_config = RenovateConfig {
            branch_prefix: Some("custom/".to_owned()),
            enabled: Some(false),
            ..Default::default()
        };
        let result = merge_configs(&base, Some(&override_config));
        assert!(result.merged);
        assert_eq!(result.config.branch_prefix, Some("custom/".to_owned()));
        assert_eq!(result.config.enabled, Some(false));
    }

    #[test]
    fn merge_configs_preserves_base() {
        let base = RenovateConfig {
            branch_prefix: Some("renovate/".to_owned()),
            ..Default::default()
        };
        let override_config = RenovateConfig {
            enabled: Some(true),
            ..Default::default()
        };
        let result = merge_configs(&base, Some(&override_config));
        assert_eq!(result.config.branch_prefix, Some("renovate/".to_owned()));
        assert_eq!(result.config.enabled, Some(true));
    }

    #[test]
    fn merge_result_serialization_roundtrip() {
        let r = MergeResult {
            config: RenovateConfig {
                enabled: Some(true),
                ..Default::default()
            },
            merged: true,
            errors: vec!["err".into()],
            warnings: vec![],
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: MergeResult = serde_json::from_str(&json).unwrap();
        assert!(back.merged);
        assert_eq!(back.errors.len(), 1);
    }
}
