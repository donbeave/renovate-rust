//! Check onboarding status.
//!
//! Mirrors `lib/workers/repository/onboarding/branch/check.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RebaseNeeded {
    Needed,
    NotNeeded,
    Conflicted,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OnboardingStatus {
    pub is_onboarded: bool,
    pub has_config_file: bool,
    pub has_package_json_config: bool,
    pub has_closed_pr: bool,
    pub rebase_needed: Option<RebaseNeeded>,
}

pub fn check_onboarding_status(
    has_config_file: bool,
    has_package_json_config: bool,
    has_closed_pr: bool,
) -> OnboardingStatus {
    let is_onboarded = has_config_file || has_package_json_config;

    let rebase_needed = if has_closed_pr && !is_onboarded {
        Some(RebaseNeeded::Conflicted)
    } else {
        None
    };

    OnboardingStatus {
        is_onboarded,
        has_config_file,
        has_package_json_config,
        has_closed_pr,
        rebase_needed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rebase_needed_variants() {
        assert_ne!(RebaseNeeded::Needed, RebaseNeeded::NotNeeded);
        assert_ne!(RebaseNeeded::Conflicted, RebaseNeeded::Needed);
    }

    #[test]
    fn onboarding_status_default() {
        let s = OnboardingStatus::default();
        assert!(!s.is_onboarded);
        assert!(!s.has_config_file);
        assert!(!s.has_package_json_config);
        assert!(!s.has_closed_pr);
        assert!(s.rebase_needed.is_none());
    }

    #[test]
    fn check_onboarding_status_onboarded_via_config() {
        let status = check_onboarding_status(true, false, false);
        assert!(status.is_onboarded);
        assert!(status.has_config_file);
    }

    #[test]
    fn check_onboarding_status_onboarded_via_package_json() {
        let status = check_onboarding_status(false, true, false);
        assert!(status.is_onboarded);
        assert!(status.has_package_json_config);
    }

    #[test]
    fn check_onboarding_status_not_onboarded() {
        let status = check_onboarding_status(false, false, false);
        assert!(!status.is_onboarded);
    }

    #[test]
    fn check_onboarding_status_closed_pr_conflicted() {
        let status = check_onboarding_status(false, false, true);
        assert!(!status.is_onboarded);
        assert_eq!(status.rebase_needed, Some(RebaseNeeded::Conflicted));
    }

    #[test]
    fn onboarding_status_serialization_roundtrip() {
        let s = OnboardingStatus {
            is_onboarded: true,
            has_config_file: true,
            has_package_json_config: false,
            has_closed_pr: false,
            rebase_needed: Some(RebaseNeeded::Needed),
        };
        let json = serde_json::to_string(&s).unwrap();
        let back: OnboardingStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(back.is_onboarded, s.is_onboarded);
        assert_eq!(back.rebase_needed, s.rebase_needed);
    }
}
