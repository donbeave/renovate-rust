//! Check for existing PRs.
//!
//! Mirrors `lib/workers/repository/update/branch/check-existing.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExistingPrResult {
    Exists,
    NotExists,
    Modified,
}

#[derive(Debug, Clone, Default)]
pub struct ExistingPrInfo {
    pub number: Option<u64>,
    pub state: Option<String>,
    pub branch_name: Option<String>,
    pub title: Option<String>,
    pub target_branch: Option<String>,
}

pub fn check_existing_pr(
    recreate_closed: bool,
    existing_pr: Option<&ExistingPrInfo>,
    branch_prefix: &str,
    branch_prefix_old: &str,
    branch_name: &str,
) -> ExistingPrResult {
    if recreate_closed {
        return ExistingPrResult::NotExists;
    }

    let Some(pr) = existing_pr else {
        return ExistingPrResult::NotExists;
    };

    if pr.state.as_deref() == Some("open") {
        return ExistingPrResult::Modified;
    }

    if pr.state.as_deref() == Some("merged") {
        return ExistingPrResult::Exists;
    }

    if branch_prefix != branch_prefix_old {
        let old_branch_name = branch_name.replace(branch_prefix, branch_prefix_old);
        if pr.branch_name.as_deref() == Some(old_branch_name.as_str()) {
            return ExistingPrResult::Exists;
        }
    }

    if pr.branch_name.as_deref() == Some(branch_name) {
        return ExistingPrResult::Exists;
    }

    ExistingPrResult::NotExists
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn existing_pr_result_variants() {
        assert_ne!(ExistingPrResult::Exists, ExistingPrResult::NotExists);
        assert_ne!(ExistingPrResult::NotExists, ExistingPrResult::Modified);
    }

    #[test]
    fn existing_pr_info_default() {
        let info = ExistingPrInfo::default();
        assert!(info.number.is_none());
        assert!(info.state.is_none());
        assert!(info.branch_name.is_none());
    }

    #[test]
    fn check_existing_pr_recreate_closed() {
        let result = check_existing_pr(true, None, "renovate/", "renovate/", "renovate/lodash-4.x");
        assert_eq!(result, ExistingPrResult::NotExists);
    }

    #[test]
    fn check_existing_pr_no_existing() {
        let result = check_existing_pr(false, None, "renovate/", "renovate/", "renovate/lodash-4.x");
        assert_eq!(result, ExistingPrResult::NotExists);
    }

    #[test]
    fn check_existing_pr_closed_found() {
        let pr = ExistingPrInfo {
            number: Some(42),
            state: Some("closed".into()),
            branch_name: Some("renovate/lodash-4.x".into()),
            ..Default::default()
        };
        let result = check_existing_pr(false, Some(&pr), "renovate/", "renovate/", "renovate/lodash-4.x");
        assert_eq!(result, ExistingPrResult::Exists);
    }

    #[test]
    fn check_existing_pr_merged() {
        let pr = ExistingPrInfo {
            number: Some(42),
            state: Some("merged".into()),
            branch_name: Some("renovate/lodash-4.x".into()),
            ..Default::default()
        };
        let result = check_existing_pr(false, Some(&pr), "renovate/", "renovate/", "renovate/lodash-4.x");
        assert_eq!(result, ExistingPrResult::Exists);
    }

    #[test]
    fn check_existing_pr_open_is_modified() {
        let pr = ExistingPrInfo {
            number: Some(42),
            state: Some("open".into()),
            branch_name: Some("renovate/lodash-4.x".into()),
            ..Default::default()
        };
        let result = check_existing_pr(false, Some(&pr), "renovate/", "renovate/", "renovate/lodash-4.x");
        assert_eq!(result, ExistingPrResult::Modified);
    }

    #[test]
    fn check_existing_pr_old_prefix() {
        let pr = ExistingPrInfo {
            number: Some(42),
            state: Some("closed".into()),
            branch_name: Some("renovate-old/lodash-4.x".into()),
            ..Default::default()
        };
        let result = check_existing_pr(false, Some(&pr), "renovate/", "renovate-old/", "renovate/lodash-4.x");
        assert_eq!(result, ExistingPrResult::Exists);
    }
}
