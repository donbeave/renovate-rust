//! Branch update orchestrator.
//!
//! Mirrors `lib/workers/repository/update/branch/index.ts`.

use serde::{Deserialize, Serialize};

use crate::workers::types::{BranchResult, PrBlockedBy};

pub use super::types::BranchConfig;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcessBranchResult {
    pub branch_exists: bool,
    pub updates_verified: Option<bool>,
    pub pr_blocked_by: Option<PrBlockedBy>,
    pub pr_no: Option<u64>,
    pub result: BranchResult,
    pub commit_sha: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ProcessBranchConfig {
    pub branch_name: String,
    pub base_branch: String,
    pub branch_exists: bool,
    pub is_scheduled_now: bool,
    pub automerge: bool,
    pub automerge_type: Option<String>,
    pub force_pr: bool,
    pub is_vulnerability_alert: bool,
    pub dependency_dashboard_check: Option<String>,
    pub mode: Option<String>,
    pub dependency_dashboard_approval: Option<bool>,
    pub minimum_group_size: Option<usize>,
    pub upgrade_count: usize,
}

pub fn should_process_branch(config: &ProcessBranchConfig) -> BranchResult {
    if !config.branch_exists {
        if let Some(ref mode) = config.mode
            && mode == "silent" && config.dependency_dashboard_check.is_none()
        {
            return BranchResult::NeedsApproval;
        }

        if config.dependency_dashboard_approval.unwrap_or(false)
            && config.dependency_dashboard_check.is_none()
        {
            return BranchResult::NeedsApproval;
        }

        if let Some(min) = config.minimum_group_size
            && min > config.upgrade_count && config.dependency_dashboard_check.is_none()
        {
            return BranchResult::MinimumGroupSizeNotMet;
        }
    }

    if !config.is_scheduled_now && config.dependency_dashboard_check.is_none()
        && !config.branch_exists
    {
        return BranchResult::NotScheduled;
    }

    BranchResult::Done
}

pub fn process_branch(config: &ProcessBranchConfig) -> ProcessBranchResult {
    let result = should_process_branch(config);

    if result != BranchResult::Done {
        return ProcessBranchResult {
            branch_exists: config.branch_exists,
            result,
            ..Default::default()
        };
    }

    ProcessBranchResult {
        branch_exists: config.branch_exists,
        result: BranchResult::Done,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_branch_result_default() {
        let r = ProcessBranchResult::default();
        assert!(!r.branch_exists);
        assert!(r.updates_verified.is_none());
        assert!(r.pr_blocked_by.is_none());
        assert!(r.pr_no.is_none());
        assert_eq!(r.result, BranchResult::Done);
        assert!(r.commit_sha.is_none());
    }

    #[test]
    fn process_branch_config_default() {
        let c = ProcessBranchConfig::default();
        assert!(c.branch_name.is_empty());
        assert!(c.base_branch.is_empty());
        assert!(!c.branch_exists);
        assert!(!c.automerge);
    }

    #[test]
    fn should_process_branch_done() {
        let config = ProcessBranchConfig {
            branch_exists: true,
            is_scheduled_now: true,
            ..Default::default()
        };
        assert_eq!(should_process_branch(&config), BranchResult::Done);
    }

    #[test]
    fn should_process_branch_needs_approval_silent() {
        let config = ProcessBranchConfig {
            branch_exists: false,
            mode: Some("silent".into()),
            ..Default::default()
        };
        assert_eq!(should_process_branch(&config), BranchResult::NeedsApproval);
    }

    #[test]
    fn should_process_branch_needs_approval_dashboard() {
        let config = ProcessBranchConfig {
            branch_exists: false,
            dependency_dashboard_approval: Some(true),
            ..Default::default()
        };
        assert_eq!(should_process_branch(&config), BranchResult::NeedsApproval);
    }

    #[test]
    fn should_process_branch_not_scheduled_new_branch() {
        let config = ProcessBranchConfig {
            branch_exists: false,
            is_scheduled_now: false,
            ..Default::default()
        };
        assert_eq!(should_process_branch(&config), BranchResult::NotScheduled);
    }

    #[test]
    fn should_process_branch_minimum_group_size() {
        let config = ProcessBranchConfig {
            branch_exists: false,
            minimum_group_size: Some(3),
            upgrade_count: 1,
            is_scheduled_now: true,
            ..Default::default()
        };
        assert_eq!(
            should_process_branch(&config),
            BranchResult::MinimumGroupSizeNotMet
        );
    }

    #[test]
    fn should_process_branch_minimum_group_size_met() {
        let config = ProcessBranchConfig {
            branch_exists: false,
            minimum_group_size: Some(3),
            upgrade_count: 5,
            is_scheduled_now: true,
            ..Default::default()
        };
        assert_eq!(should_process_branch(&config), BranchResult::Done);
    }

    #[test]
    fn should_process_branch_silent_with_dashboard_check() {
        let config = ProcessBranchConfig {
            branch_exists: false,
            mode: Some("silent".into()),
            dependency_dashboard_check: Some("rebase".into()),
            is_scheduled_now: true,
            ..Default::default()
        };
        assert_eq!(should_process_branch(&config), BranchResult::Done);
    }

    #[test]
    fn process_branch_returns_done() {
        let config = ProcessBranchConfig {
            branch_name: "renovate/lodash-4.x".into(),
            base_branch: "main".into(),
            branch_exists: true,
            is_scheduled_now: true,
            ..Default::default()
        };
        let result = process_branch(&config);
        assert!(result.branch_exists);
        assert_eq!(result.result, BranchResult::Done);
    }

    #[test]
    fn process_branch_returns_not_scheduled() {
        let config = ProcessBranchConfig {
            branch_name: "renovate/lodash-4.x".into(),
            base_branch: "main".into(),
            branch_exists: false,
            is_scheduled_now: false,
            ..Default::default()
        };
        let result = process_branch(&config);
        assert!(!result.branch_exists);
        assert_eq!(result.result, BranchResult::NotScheduled);
    }
}
