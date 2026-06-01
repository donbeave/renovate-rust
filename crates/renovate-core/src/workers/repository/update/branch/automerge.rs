//! Branch auto-merge logic.
//!
//! Mirrors `lib/workers/repository/update/branch/automerge.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutomergeResult {
    Automerged,
    AutomergeAbortedPrExists,
    BranchStatusError,
    Failed,
    NoAutomerge,
    Stale,
    OffSchedule,
    NotReady,
}

impl std::fmt::Display for AutomergeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Automerged => write!(f, "automerged"),
            Self::AutomergeAbortedPrExists => write!(f, "automerge aborted - PR exists"),
            Self::BranchStatusError => write!(f, "branch status error"),
            Self::Failed => write!(f, "failed"),
            Self::NoAutomerge => write!(f, "no automerge"),
            Self::Stale => write!(f, "stale"),
            Self::OffSchedule => write!(f, "off schedule"),
            Self::NotReady => write!(f, "not ready"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AutomergeConfig {
    pub automerge: bool,
    pub automerge_type: Option<String>,
    pub branch_name: Option<String>,
    pub base_branch: Option<String>,
    pub is_scheduled_now: bool,
    pub branch_status: BranchStatus,
    pub ignore_tests: bool,
    pub dry_run: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BranchStatus {
    #[default]
    Yellow,
    Green,
    Red,
}

pub fn try_automerge(config: &AutomergeConfig) -> AutomergeResult {
    if !config.automerge {
        return AutomergeResult::NoAutomerge;
    }

    let automerge_type = config.automerge_type.as_deref().unwrap_or("pr");

    if automerge_type != "branch" {
        return AutomergeResult::NoAutomerge;
    }

    if !config.is_scheduled_now {
        return AutomergeResult::OffSchedule;
    }

    match config.branch_status {
        BranchStatus::Green => {
            if config.dry_run {
                AutomergeResult::NoAutomerge
            } else {
                AutomergeResult::Automerged
            }
        }
        BranchStatus::Red => AutomergeResult::BranchStatusError,
        BranchStatus::Yellow => AutomergeResult::NotReady,
    }
}

pub fn check_automerge_status(
    automerge: bool,
    automerge_type: &str,
    branch_status: BranchStatus,
    is_scheduled: bool,
) -> AutomergeResult {
    if !automerge {
        return AutomergeResult::NoAutomerge;
    }

    if automerge_type != "branch" {
        return AutomergeResult::NoAutomerge;
    }

    if !is_scheduled {
        return AutomergeResult::OffSchedule;
    }

    match branch_status {
        BranchStatus::Green => AutomergeResult::Automerged,
        BranchStatus::Red => AutomergeResult::BranchStatusError,
        BranchStatus::Yellow => AutomergeResult::NotReady,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn automerge_result_variants() {
        assert_ne!(AutomergeResult::Automerged, AutomergeResult::Failed);
        assert_ne!(AutomergeResult::NoAutomerge, AutomergeResult::Stale);
    }

    #[test]
    fn automerge_result_display() {
        assert_eq!(AutomergeResult::Automerged.to_string(), "automerged");
        assert_eq!(AutomergeResult::Failed.to_string(), "failed");
        assert_eq!(AutomergeResult::OffSchedule.to_string(), "off schedule");
        assert_eq!(
            AutomergeResult::BranchStatusError.to_string(),
            "branch status error"
        );
        assert_eq!(
            AutomergeResult::AutomergeAbortedPrExists.to_string(),
            "automerge aborted - PR exists"
        );
        assert_eq!(AutomergeResult::NotReady.to_string(), "not ready");
        assert_eq!(AutomergeResult::NoAutomerge.to_string(), "no automerge");
        assert_eq!(AutomergeResult::Stale.to_string(), "stale");
    }

    #[test]
    fn try_automerge_disabled() {
        let config = AutomergeConfig {
            automerge: false,
            ..Default::default()
        };
        assert_eq!(try_automerge(&config), AutomergeResult::NoAutomerge);
    }

    #[test]
    fn try_automerge_not_branch_type() {
        let config = AutomergeConfig {
            automerge: true,
            automerge_type: Some("pr".into()),
            ..Default::default()
        };
        assert_eq!(try_automerge(&config), AutomergeResult::NoAutomerge);
    }

    #[test]
    fn try_automerge_off_schedule() {
        let config = AutomergeConfig {
            automerge: true,
            automerge_type: Some("branch".into()),
            is_scheduled_now: false,
            ..Default::default()
        };
        assert_eq!(try_automerge(&config), AutomergeResult::OffSchedule);
    }

    #[test]
    fn try_automerge_green() {
        let config = AutomergeConfig {
            automerge: true,
            automerge_type: Some("branch".into()),
            is_scheduled_now: true,
            branch_status: BranchStatus::Green,
            ..Default::default()
        };
        assert_eq!(try_automerge(&config), AutomergeResult::Automerged);
    }

    #[test]
    fn try_automerge_red() {
        let config = AutomergeConfig {
            automerge: true,
            automerge_type: Some("branch".into()),
            is_scheduled_now: true,
            branch_status: BranchStatus::Red,
            ..Default::default()
        };
        assert_eq!(try_automerge(&config), AutomergeResult::BranchStatusError);
    }

    #[test]
    fn try_automerge_yellow() {
        let config = AutomergeConfig {
            automerge: true,
            automerge_type: Some("branch".into()),
            is_scheduled_now: true,
            branch_status: BranchStatus::Yellow,
            ..Default::default()
        };
        assert_eq!(try_automerge(&config), AutomergeResult::NotReady);
    }

    #[test]
    fn try_automerge_dry_run() {
        let config = AutomergeConfig {
            automerge: true,
            automerge_type: Some("branch".into()),
            is_scheduled_now: true,
            branch_status: BranchStatus::Green,
            dry_run: true,
            ..Default::default()
        };
        assert_eq!(try_automerge(&config), AutomergeResult::NoAutomerge);
    }

    #[test]
    fn check_automerge_status_green() {
        assert_eq!(
            check_automerge_status(true, "branch", BranchStatus::Green, true),
            AutomergeResult::Automerged
        );
    }

    #[test]
    fn check_automerge_status_not_branch() {
        assert_eq!(
            check_automerge_status(true, "pr", BranchStatus::Green, true),
            AutomergeResult::NoAutomerge
        );
    }

    #[test]
    fn check_automerge_status_disabled() {
        assert_eq!(
            check_automerge_status(false, "branch", BranchStatus::Green, true),
            AutomergeResult::NoAutomerge
        );
    }

    #[test]
    fn automerge_config_default() {
        let c = AutomergeConfig::default();
        assert!(!c.automerge);
        assert!(c.automerge_type.is_none());
        assert!(c.branch_name.is_none());
        assert!(!c.dry_run);
        assert!(!c.ignore_tests);
    }

    #[test]
    fn branch_status_default() {
        assert_eq!(BranchStatus::default(), BranchStatus::Yellow);
    }
}
