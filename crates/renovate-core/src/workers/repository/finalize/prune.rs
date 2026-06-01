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

    let filtered: Vec<&str> = renovate_branches
        .iter()
        .filter(|b| b.starts_with(branch_prefix))
        .map(|s| s.as_str())
        .collect();

    for branch in filtered {
        if !branch_list.contains(&branch.to_owned()) {
            result.pruned_branches.push(branch.to_owned());
        }
    }

    result
}

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
}
