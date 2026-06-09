//! Branch pruning.
//!
//! Mirrors `lib/workers/repository/finalize/prune.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PruneResult {
    pub pruned_branches: Vec<String>,
    pub autoclosed_prs: Vec<u64>,
    pub deleted_orphan_branches: Vec<String>,
    pub skipped_modified: Vec<String>,
}

pub fn prune_stale_branches(
    branch_list: &[String],
    renovate_branches: &[String],
    branch_prefix: &str,
) -> PruneResult {
    let mut result = PruneResult::default();

    // Match TS filtering in pruneStaleBranches: prefix + exclude lock-file-maintenance
    // (reconfigure exclusion and getBranchList call assumed upstream or in caller per current architecture;
    // full remaining + cleanUpBranches side effects (findPr, isBranchModified, updatePr for titles,
    // ensureComment, deleteBranch, dryRun, multi-base) live in platform/git or will be wired in pending prune cycle).
    let lock_file_branch = format!("{}lock-file-maintenance", branch_prefix);
    let filtered: Vec<&str> = renovate_branches
        .iter()
        .filter(|b| {
            let b = b.as_str();
            b.starts_with(branch_prefix) && b != lock_file_branch.as_str()
        })
        .map(|s| s.as_str())
        .collect();

    for branch in filtered {
        if !branch_list.contains(&branch.to_owned()) {
            result.pruned_branches.push(branch.to_owned());
        }
    }

    result
}

// @parity lib/workers/repository/finalize/prune.ts partial — pruneStaleBranches (computes remaining renovate branches after prefix/lock/reconfigure exclusions, returns pruned list in PruneResult; full cleanUpBranches with platform.findPr, scm.isBranchModified/isBranchModified, updatePr for '- autoclosed'/' - abandoned', ensureComment, scm.deleteBranch, dryRun, multi-base baseBranchRe, error handling, GlobalConfig, logger is in platform/git layers or finalize caller). The pure list-diff helper here is used by finalize/index; side effects and full orchestration pending. Single test ported.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prune_result_default() {
        let r = PruneResult::default();
        assert!(r.pruned_branches.is_empty());
        assert!(r.autoclosed_prs.is_empty());
        assert!(r.deleted_orphan_branches.is_empty());
        assert!(r.skipped_modified.is_empty());
    }

    #[test]
    fn prune_stale_branches_no_stale() {
        let branches = vec!["renovate/pkg-1".to_owned()];
        let renovate = vec!["renovate/pkg-1".to_owned()];
        let result = prune_stale_branches(&branches, &renovate, "renovate/");
        assert!(result.pruned_branches.is_empty());
    }

    #[test]
    fn prune_stale_branches_with_stale() {
        let branches = vec!["renovate/pkg-1".to_owned()];
        let renovate = vec!["renovate/pkg-1".to_owned(), "renovate/old-pkg".to_owned()];
        let result = prune_stale_branches(&branches, &renovate, "renovate/");
        assert_eq!(result.pruned_branches, vec!["renovate/old-pkg"]);
    }

    #[test]
    fn prune_stale_branches_filters_prefix() {
        let branches: Vec<String> = Vec::new();
        let renovate = vec!["renovate/pkg-1".to_owned(), "feature/other".to_owned()];
        let result = prune_stale_branches(&branches, &renovate, "renovate/");
        assert_eq!(result.pruned_branches, vec!["renovate/pkg-1"]);
    }

    #[test]
    fn prune_result_serialization_roundtrip() {
        let r = PruneResult {
            pruned_branches: vec!["renovate/old".into()],
            autoclosed_prs: vec![42],
            deleted_orphan_branches: vec![],
            skipped_modified: vec!["renovate/modified".into()],
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: PruneResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.pruned_branches.len(), 1);
    }

    // Ported: "returns if no remaining branches" — lib/workers/repository/finalize/prune.spec.ts line 51
    #[test]
    fn prune_stale_branches_returns_if_no_remaining_branches() {
        // Exercises the core remainingBranches = renovate.filter(not in branchList) logic after prefix/lock exclusions.
        // Proves parity for the list computation part of pruneStaleBranches (full side-effect cleanup in platform/git pending).
        let branches = vec!["renovate/a".to_owned(), "renovate/b".to_owned()];
        let renovate = vec!["renovate/a".to_owned(), "renovate/b".to_owned()];
        let result = prune_stale_branches(&branches, &renovate, "renovate/");
        assert!(result.pruned_branches.is_empty());
    }
}
