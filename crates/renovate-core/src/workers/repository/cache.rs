//! Repository cache management.
//!
//! Mirrors `lib/workers/repository/cache.ts`.

use serde::{Deserialize, Serialize};

use crate::workers::repository::update::branch::types::BranchConfig;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RepositoryCache {
    pub branches: Vec<BranchCacheEntry>,
    pub modified: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BranchCacheEntry {
    pub branch_name: String,
    pub sha: Option<String>,
    pub base_branch: Option<String>,
    pub base_branch_sha: Option<String>,
    pub automerge: bool,
    pub is_modified: Option<bool>,
    pub is_pristine: Option<bool>,
    pub pr_no: Option<u64>,
    pub pr_title: Option<String>,
    pub result: Option<String>,
}

pub fn get_cache() -> RepositoryCache {
    RepositoryCache::default()
}

pub fn set_cache(branches: &[BranchConfig]) -> RepositoryCache {
    let mut cache = RepositoryCache::default();
    for branch in branches {
        let entry = BranchCacheEntry {
            branch_name: branch.branch_name.clone(),
            sha: None,
            base_branch: Some(branch.base_branch.clone()),
            base_branch_sha: None,
            automerge: branch.automerge.unwrap_or(false),
            is_modified: None,
            is_pristine: None,
            pr_no: branch.pr_no,
            pr_title: None,
            result: None,
        };
        cache.branches.push(entry);
    }
    cache.modified = true;
    cache
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repository_cache_default() {
        let c = RepositoryCache::default();
        assert!(c.branches.is_empty());
        assert!(!c.modified);
    }

    #[test]
    fn branch_cache_entry_default() {
        let e = BranchCacheEntry::default();
        assert!(e.branch_name.is_empty());
        assert!(e.sha.is_none());
        assert!(!e.automerge);
    }

    #[test]
    fn get_cache_returns_default() {
        let cache = get_cache();
        assert!(cache.branches.is_empty());
    }

    #[test]
    fn set_cache_from_branches() {
        let branches = vec![BranchConfig {
            branch_name: "renovate/pkg-1".into(),
            base_branch: "main".into(),
            automerge: Some(true),
            pr_no: Some(42),
            ..Default::default()
        }];
        let cache = set_cache(&branches);
        assert_eq!(cache.branches.len(), 1);
        assert!(cache.modified);
        assert_eq!(cache.branches[0].branch_name, "renovate/pkg-1");
        assert_eq!(cache.branches[0].base_branch, Some("main".into()));
        assert!(cache.branches[0].automerge);
        assert_eq!(cache.branches[0].pr_no, Some(42));
    }

    #[test]
    fn set_cache_empty() {
        let cache = set_cache(&[]);
        assert!(cache.branches.is_empty());
        assert!(cache.modified);
    }

    #[test]
    fn repository_cache_serialization_roundtrip() {
        let c = RepositoryCache {
            branches: vec![BranchCacheEntry {
                branch_name: "test".into(),
                sha: Some("abc".into()),
                automerge: true,
                ..Default::default()
            }],
            modified: true,
        };
        let json = serde_json::to_string(&c).unwrap();
        let back: RepositoryCache = serde_json::from_str(&json).unwrap();
        assert_eq!(back.branches.len(), 1);
        assert!(back.modified);
    }
}
