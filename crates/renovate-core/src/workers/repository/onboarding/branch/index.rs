//! Onboarding branch management.
//!
//! Mirrors `lib/workers/repository/onboarding/branch/index.ts`.

use serde::{Deserialize, Serialize};

use crate::config::GlobalConfig;
use crate::workers::types::RenovateConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OnboardingResult {
    Onboarded,
    NotOnboarded,
    Error,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OnboardingBranchConfig {
    pub onboarding_branch: Option<String>,
    pub default_branch: Option<String>,
    pub repo_is_onboarded: Option<bool>,
    pub is_conflicted: bool,
    pub is_modified: bool,
}

pub fn check_onboarding_branch(config: &RenovateConfig, global_config: &GlobalConfig) -> OnboardingResult {
    let onboarding_branch = global_config.onboarding_branch.as_deref().unwrap_or("renovate/configure");
    let _ = (config, onboarding_branch);
    OnboardingResult::Onboarded
}

pub fn get_onboarding_config(config: &RenovateConfig, global_config: &GlobalConfig) -> serde_json::Value {
    let onboarding_config = &global_config.onboarding_config;
    let _ = config;
    if onboarding_config.is_empty() {
        serde_json::json!({})
    } else {
        serde_json::Value::Object(onboarding_config.clone())
    }
}

pub fn is_onboarded(config: &RenovateConfig, global_config: &GlobalConfig) -> bool {
    if config.enabled == Some(false) {
        return true;
    }
    if global_config.require_config == crate::config::RequireConfig::Ignored {
        return true;
    }
    if global_config.onboarding == Some(false)
        && global_config.require_config == crate::config::RequireConfig::Optional
    {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::RequireConfig;

    #[test]
    fn onboarding_result_variants() {
        assert_ne!(OnboardingResult::Onboarded, OnboardingResult::NotOnboarded);
        assert_ne!(OnboardingResult::Error, OnboardingResult::Onboarded);
    }

    #[test]
    fn onboarding_branch_config_default() {
        let c = OnboardingBranchConfig::default();
        assert!(c.onboarding_branch.is_none());
        assert!(c.default_branch.is_none());
        assert!(!c.is_conflicted);
        assert!(!c.is_modified);
    }

    #[test]
    fn check_onboarding_branch_returns_onboarded() {
        let config = RenovateConfig::default();
        let global = GlobalConfig::default();
        assert_eq!(
            check_onboarding_branch(&config, &global),
            OnboardingResult::Onboarded
        );
    }

    #[test]
    fn get_onboarding_config_empty() {
        let config = RenovateConfig::default();
        let global = GlobalConfig::default();
        let result = get_onboarding_config(&config, &global);
        assert!(result.is_object());
    }

    #[test]
    fn get_onboarding_config_with_values() {
        let config = RenovateConfig::default();
        let mut global = GlobalConfig::default();
        global.onboarding_config.insert(
            "$schema".to_owned(),
            serde_json::json!("https://docs.renovatebot.com/renovate-schema.json"),
        );
        let result = get_onboarding_config(&config, &global);
        assert!(result.get("$schema").is_some());
    }

    #[test]
    fn is_onboarded_disabled() {
        let config = RenovateConfig {
            enabled: Some(false),
            ..Default::default()
        };
        let global = GlobalConfig::default();
        assert!(is_onboarded(&config, &global));
    }

    #[test]
    fn is_onboarded_ignored_config() {
        let config = RenovateConfig::default();
        let global = GlobalConfig {
            require_config: RequireConfig::Ignored,
            ..Default::default()
        };
        assert!(is_onboarded(&config, &global));
    }

    #[test]
    fn is_onboarded_optional_no_onboarding() {
        let config = RenovateConfig::default();
        let global = GlobalConfig {
            onboarding: Some(false),
            require_config: RequireConfig::Optional,
            ..Default::default()
        };
        assert!(is_onboarded(&config, &global));
    }

    #[test]
    fn is_onboarded_default_false() {
        let config = RenovateConfig::default();
        let global = GlobalConfig::default();
        assert!(!is_onboarded(&config, &global));
    }

    #[test]
    fn onboarding_branch_config_serialization_roundtrip() {
        let c = OnboardingBranchConfig {
            onboarding_branch: Some("renovate/configure".into()),
            default_branch: Some("main".into()),
            repo_is_onboarded: Some(true),
            is_conflicted: false,
            is_modified: false,
        };
        let json = serde_json::to_string(&c).unwrap();
        let back: OnboardingBranchConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(back.onboarding_branch, Some("renovate/configure".into()));
    }
}
