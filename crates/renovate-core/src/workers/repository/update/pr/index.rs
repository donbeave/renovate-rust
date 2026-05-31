//! PR creation orchestrator.
//!
//! Mirrors `lib/workers/repository/update/pr/index.ts`.

use serde::{Deserialize, Serialize};

use crate::workers::types::PrBlockedBy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrResult {
    Created,
    Updated,
    Skipped,
    Blocked,
}

#[derive(Debug, Clone, Default)]
pub struct EnsurePrConfig {
    pub branch_name: String,
    pub base_branch: String,
    pub pr_title: Option<String>,
    pub pr_body: Option<String>,
    pub existing_pr_number: Option<u64>,
    pub automerge: bool,
    pub automerge_type: Option<String>,
    pub force_pr: bool,
    pub pr_creation: Option<String>,
    pub branch_status: Option<String>,
    pub stability_status: Option<String>,
    pub pr_not_pending_hours: Option<u64>,
    pub is_vulnerability_alert: bool,
    pub dependency_dashboard_check: Option<String>,
    pub dry_run: bool,
}

#[derive(Debug, Clone)]
pub struct EnsurePrResult {
    pub pr_result: PrResult,
    pub pr_no: Option<u64>,
    pub pr_blocked_by: Option<PrBlockedBy>,
}

pub fn ensure_pr(config: &EnsurePrConfig) -> EnsurePrResult {
    if config.dry_run {
        return EnsurePrResult {
            pr_result: PrResult::Skipped,
            pr_no: None,
            pr_blocked_by: None,
        };
    }

    if config.automerge {
        let automerge_type = config.automerge_type.as_deref().unwrap_or("pr");
        if automerge_type.starts_with("branch") && !config.force_pr {
            return EnsurePrResult {
                pr_result: PrResult::Blocked,
                pr_no: None,
                pr_blocked_by: Some(PrBlockedBy::BranchAutomerge),
            };
        }
    }

    let pr_creation = config.pr_creation.as_deref().unwrap_or("immediate");

    if pr_creation == "status-success"
        && config.branch_status.as_deref() != Some("green")
    {
        return EnsurePrResult {
            pr_result: PrResult::Blocked,
            pr_no: None,
            pr_blocked_by: Some(PrBlockedBy::AwaitingTests),
        };
    }

    if pr_creation == "approval"
        && config.dependency_dashboard_check.as_deref() != Some("approvePr")
    {
        return EnsurePrResult {
            pr_result: PrResult::Blocked,
            pr_no: None,
            pr_blocked_by: Some(PrBlockedBy::NeedsApproval),
        };
    }

    if config.existing_pr_number.is_some() {
        return EnsurePrResult {
            pr_result: PrResult::Updated,
            pr_no: config.existing_pr_number,
            pr_blocked_by: None,
        };
    }

    EnsurePrResult {
        pr_result: PrResult::Created,
        pr_no: None,
        pr_blocked_by: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pr_result_variants() {
        assert_ne!(PrResult::Created, PrResult::Updated);
        assert_ne!(PrResult::Skipped, PrResult::Blocked);
    }

    #[test]
    fn ensure_pr_config_default() {
        let c = EnsurePrConfig::default();
        assert!(c.branch_name.is_empty());
        assert!(c.base_branch.is_empty());
        assert!(c.pr_title.is_none());
        assert!(!c.automerge);
        assert!(!c.force_pr);
        assert!(!c.dry_run);
    }

    #[test]
    fn ensure_pr_dry_run() {
        let config = EnsurePrConfig {
            dry_run: true,
            ..Default::default()
        };
        let result = ensure_pr(&config);
        assert_eq!(result.pr_result, PrResult::Skipped);
    }

    #[test]
    fn ensure_pr_branch_automerge() {
        let config = EnsurePrConfig {
            automerge: true,
            automerge_type: Some("branch".into()),
            ..Default::default()
        };
        let result = ensure_pr(&config);
        assert_eq!(result.pr_result, PrResult::Blocked);
        assert_eq!(result.pr_blocked_by, Some(PrBlockedBy::BranchAutomerge));
    }

    #[test]
    fn ensure_pr_branch_automerge_forced() {
        let config = EnsurePrConfig {
            automerge: true,
            automerge_type: Some("branch".into()),
            force_pr: true,
            ..Default::default()
        };
        let result = ensure_pr(&config);
        assert_eq!(result.pr_result, PrResult::Created);
    }

    #[test]
    fn ensure_pr_status_success_not_green() {
        let config = EnsurePrConfig {
            pr_creation: Some("status-success".into()),
            branch_status: Some("yellow".into()),
            ..Default::default()
        };
        let result = ensure_pr(&config);
        assert_eq!(result.pr_result, PrResult::Blocked);
        assert_eq!(result.pr_blocked_by, Some(PrBlockedBy::AwaitingTests));
    }

    #[test]
    fn ensure_pr_status_success_green() {
        let config = EnsurePrConfig {
            pr_creation: Some("status-success".into()),
            branch_status: Some("green".into()),
            ..Default::default()
        };
        let result = ensure_pr(&config);
        assert_eq!(result.pr_result, PrResult::Created);
    }

    #[test]
    fn ensure_pr_approval() {
        let config = EnsurePrConfig {
            pr_creation: Some("approval".into()),
            ..Default::default()
        };
        let result = ensure_pr(&config);
        assert_eq!(result.pr_result, PrResult::Blocked);
        assert_eq!(result.pr_blocked_by, Some(PrBlockedBy::NeedsApproval));
    }

    #[test]
    fn ensure_pr_approval_with_dashboard() {
        let config = EnsurePrConfig {
            pr_creation: Some("approval".into()),
            dependency_dashboard_check: Some("approvePr".into()),
            ..Default::default()
        };
        let result = ensure_pr(&config);
        assert_eq!(result.pr_result, PrResult::Created);
    }

    #[test]
    fn ensure_pr_existing_pr() {
        let config = EnsurePrConfig {
            existing_pr_number: Some(42),
            ..Default::default()
        };
        let result = ensure_pr(&config);
        assert_eq!(result.pr_result, PrResult::Updated);
        assert_eq!(result.pr_no, Some(42));
    }

    #[test]
    fn ensure_pr_new_pr() {
        let config = EnsurePrConfig {
            branch_name: "renovate/lodash-4.x".into(),
            base_branch: "main".into(),
            ..Default::default()
        };
        let result = ensure_pr(&config);
        assert_eq!(result.pr_result, PrResult::Created);
    }
}
