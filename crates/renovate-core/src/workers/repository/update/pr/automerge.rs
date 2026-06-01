//! PR auto-merge logic.
//!
//! Mirrors `lib/workers/repository/update/pr/automerge.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrAutomergeResult {
    Automerged,
    BranchModified,
    BranchNotGreen,
    Conflicted,
    DryRun,
    PlatformNotReady,
    PlatformRejection,
    OffSchedule,
}

#[derive(Debug, Clone, Default)]
pub struct PrAutomergeConfig {
    pub automerge: bool,
    pub automerge_type: Option<String>,
    pub branch_status: Option<String>,
    pub is_conflicted: bool,
    pub is_modified: bool,
    pub is_scheduled_now: bool,
    pub dry_run: bool,
    pub ignore_tests: bool,
    pub cannot_merge: bool,
    pub branch_name: Option<String>,
    pub pr_number: Option<u64>,
}

pub fn automerge_pr(config: &PrAutomergeConfig) -> PrAutomergeResult {
    if !config.automerge {
        return PrAutomergeResult::BranchNotGreen;
    }

    if !config.is_scheduled_now {
        return PrAutomergeResult::OffSchedule;
    }

    if config.is_conflicted {
        return PrAutomergeResult::Conflicted;
    }

    if config.cannot_merge && !config.ignore_tests {
        return PrAutomergeResult::PlatformNotReady;
    }

    if config.branch_status.as_deref() != Some("green") {
        return PrAutomergeResult::BranchNotGreen;
    }

    if config.is_modified {
        return PrAutomergeResult::BranchModified;
    }

    if config.dry_run {
        return PrAutomergeResult::DryRun;
    }

    PrAutomergeResult::Automerged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pr_automerge_result_variants() {
        assert_ne!(PrAutomergeResult::Automerged, PrAutomergeResult::DryRun);
        assert_ne!(
            PrAutomergeResult::Conflicted,
            PrAutomergeResult::OffSchedule
        );
    }

    #[test]
    fn pr_automerge_config_default() {
        let c = PrAutomergeConfig::default();
        assert!(!c.automerge);
        assert!(!c.is_conflicted);
        assert!(!c.is_modified);
        assert!(!c.dry_run);
    }

    #[test]
    fn automerge_pr_not_enabled() {
        let config = PrAutomergeConfig {
            automerge: false,
            ..Default::default()
        };
        assert_eq!(automerge_pr(&config), PrAutomergeResult::BranchNotGreen);
    }

    #[test]
    fn automerge_pr_off_schedule() {
        let config = PrAutomergeConfig {
            automerge: true,
            is_scheduled_now: false,
            ..Default::default()
        };
        assert_eq!(automerge_pr(&config), PrAutomergeResult::OffSchedule);
    }

    #[test]
    fn automerge_pr_conflicted() {
        let config = PrAutomergeConfig {
            automerge: true,
            is_scheduled_now: true,
            is_conflicted: true,
            ..Default::default()
        };
        assert_eq!(automerge_pr(&config), PrAutomergeResult::Conflicted);
    }

    #[test]
    fn automerge_pr_cannot_merge() {
        let config = PrAutomergeConfig {
            automerge: true,
            is_scheduled_now: true,
            cannot_merge: true,
            ..Default::default()
        };
        assert_eq!(automerge_pr(&config), PrAutomergeResult::PlatformNotReady);
    }

    #[test]
    fn automerge_pr_cannot_merge_ignore_tests() {
        let config = PrAutomergeConfig {
            automerge: true,
            is_scheduled_now: true,
            cannot_merge: true,
            ignore_tests: true,
            branch_status: Some("green".into()),
            ..Default::default()
        };
        assert_eq!(automerge_pr(&config), PrAutomergeResult::Automerged);
    }

    #[test]
    fn automerge_pr_not_green() {
        let config = PrAutomergeConfig {
            automerge: true,
            is_scheduled_now: true,
            branch_status: Some("yellow".into()),
            ..Default::default()
        };
        assert_eq!(automerge_pr(&config), PrAutomergeResult::BranchNotGreen);
    }

    #[test]
    fn automerge_pr_modified() {
        let config = PrAutomergeConfig {
            automerge: true,
            is_scheduled_now: true,
            branch_status: Some("green".into()),
            is_modified: true,
            ..Default::default()
        };
        assert_eq!(automerge_pr(&config), PrAutomergeResult::BranchModified);
    }

    #[test]
    fn automerge_pr_dry_run() {
        let config = PrAutomergeConfig {
            automerge: true,
            is_scheduled_now: true,
            branch_status: Some("green".into()),
            dry_run: true,
            ..Default::default()
        };
        assert_eq!(automerge_pr(&config), PrAutomergeResult::DryRun);
    }

    #[test]
    fn automerge_pr_success() {
        let config = PrAutomergeConfig {
            automerge: true,
            is_scheduled_now: true,
            branch_status: Some("green".into()),
            ..Default::default()
        };
        assert_eq!(automerge_pr(&config), PrAutomergeResult::Automerged);
    }
}
