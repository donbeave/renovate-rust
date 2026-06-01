//! Status checks for branches.
//!
//! Mirrors `lib/workers/repository/update/branch/status-checks.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum StatusCheckResult {
    #[default]
    Pending,
    Passing,
    Failing,
}

#[derive(Debug, Clone, Default)]
pub struct StatusCheckConfig {
    pub branch_name: String,
    pub ignore_tests: bool,
    pub internal_checks_as_success: bool,
}

pub fn check_status(branch_status: Option<&str>, ignore_tests: bool) -> StatusCheckResult {
    if ignore_tests {
        return StatusCheckResult::Passing;
    }

    match branch_status {
        Some("green") => StatusCheckResult::Passing,
        Some("red") => StatusCheckResult::Failing,
        _ => StatusCheckResult::Pending,
    }
}

pub fn wait_for_status_checks(
    commit_sha: Option<&str>,
    branch_exists: bool,
    pr_creation: Option<&str>,
    user_rebase_requested: bool,
    has_artifact_errors: bool,
) -> bool {
    if has_artifact_errors {
        return false;
    }

    if !branch_exists || user_rebase_requested {
        return false;
    }

    let Some(sha) = commit_sha else {
        return false;
    };

    if sha.is_empty() {
        return false;
    }

    matches!(pr_creation, Some("immediate")) || !branch_exists
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_check_result_default() {
        assert_eq!(StatusCheckResult::default(), StatusCheckResult::Pending);
    }

    #[test]
    fn status_check_result_variants() {
        assert_ne!(StatusCheckResult::Passing, StatusCheckResult::Failing);
        assert_ne!(StatusCheckResult::Pending, StatusCheckResult::Passing);
    }

    #[test]
    fn check_status_green() {
        assert_eq!(
            check_status(Some("green"), false),
            StatusCheckResult::Passing
        );
    }

    #[test]
    fn check_status_red() {
        assert_eq!(check_status(Some("red"), false), StatusCheckResult::Failing);
    }

    #[test]
    fn check_status_yellow() {
        assert_eq!(
            check_status(Some("yellow"), false),
            StatusCheckResult::Pending
        );
    }

    #[test]
    fn check_status_none() {
        assert_eq!(check_status(None, false), StatusCheckResult::Pending);
    }

    #[test]
    fn check_status_ignore_tests() {
        assert_eq!(check_status(Some("red"), true), StatusCheckResult::Passing);
    }

    #[test]
    fn status_check_config_default() {
        let c = StatusCheckConfig::default();
        assert!(c.branch_name.is_empty());
        assert!(!c.ignore_tests);
        assert!(!c.internal_checks_as_success);
    }

    #[test]
    fn wait_for_status_checks_with_artifact_errors() {
        assert!(!wait_for_status_checks(
            Some("abc123"),
            true,
            Some("immediate"),
            false,
            true
        ));
    }

    #[test]
    fn wait_for_status_checks_no_commit() {
        assert!(!wait_for_status_checks(
            None,
            true,
            Some("immediate"),
            false,
            false
        ));
    }

    #[test]
    fn wait_for_status_checks_empty_sha() {
        assert!(!wait_for_status_checks(
            Some(""),
            true,
            Some("immediate"),
            false,
            false
        ));
    }
}
