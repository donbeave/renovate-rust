//! Create onboarding branch.
//!
//! Mirrors `lib/workers/repository/onboarding/branch/create.ts`.

use serde::{Deserialize, Serialize};

use crate::config::GlobalConfig;
use crate::workers::types::RenovateConfig;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OnboardingBranchCreateResult {
    pub commit: Option<String>,
    pub branch_name: Option<String>,
    pub config_file: Option<String>,
    pub dry_run: bool,
}

pub fn create_onboarding_branch(
    _config: &RenovateConfig,
    global_config: &GlobalConfig,
    config_file_name: &str,
    _config_contents: &str,
) -> OnboardingBranchCreateResult {
    let branch_name = global_config
        .onboarding_branch
        .as_deref()
        .unwrap_or("renovate/configure")
        .to_owned();

    if global_config.dry_run.is_some() {
        return OnboardingBranchCreateResult {
            commit: None,
            branch_name: Some(branch_name),
            config_file: Some(config_file_name.to_owned()),
            dry_run: true,
        };
    }

    let commit_message = global_config
        .onboarding_commit_message
        .as_deref()
        .unwrap_or_else(|| "Add Renovate configuration")
        .to_owned();

    OnboardingBranchCreateResult {
        commit: Some(commit_message),
        branch_name: Some(branch_name),
        config_file: Some(config_file_name.to_owned()),
        dry_run: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_onboarding_branch_dry_run() {
        let config = RenovateConfig::default();
        let global = GlobalConfig {
            dry_run: Some(crate::config::DryRun::Full),
            ..Default::default()
        };
        let result = create_onboarding_branch(&config, &global, "renovate.json", "{}");
        assert!(result.dry_run);
        assert!(result.commit.is_none());
        assert_eq!(result.config_file, Some("renovate.json".to_owned()));
    }

    #[test]
    fn create_onboarding_branch_normal() {
        let config = RenovateConfig::default();
        let global = GlobalConfig::default();
        let result = create_onboarding_branch(
            &config,
            &global,
            "renovate.json",
            "{\"$schema\":...}",
        );
        assert!(!result.dry_run);
        assert!(result.commit.is_some());
        assert_eq!(result.branch_name, Some("renovate/configure".to_owned()));
    }

    #[test]
    fn create_onboarding_branch_custom_message() {
        let config = RenovateConfig::default();
        let global = GlobalConfig {
            onboarding_commit_message: Some("chore: add config".to_owned()),
            ..Default::default()
        };
        let result = create_onboarding_branch(&config, &global, "renovate.json", "{}");
        assert_eq!(result.commit, Some("chore: add config".to_owned()));
    }

    #[test]
    fn create_result_default() {
        let r = OnboardingBranchCreateResult::default();
        assert!(r.commit.is_none());
        assert!(r.branch_name.is_none());
        assert!(!r.dry_run);
    }

    #[test]
    fn create_result_serialization_roundtrip() {
        let r = OnboardingBranchCreateResult {
            commit: Some("abc".into()),
            branch_name: Some("renovate/configure".into()),
            config_file: Some("renovate.json".into()),
            dry_run: false,
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: OnboardingBranchCreateResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.commit, r.commit);
    }
}
