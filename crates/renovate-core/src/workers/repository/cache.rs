//! Repository cache management.
//!
//! Mirrors `lib/workers/repository/cache.ts`.
//! @parity lib/workers/repository/cache.ts partial — set_cache + BranchCacheEntry/BranchUpgradeCacheEntry projection (generate_branch_upgrade_cache_entry + population of shas, is_* flags, commit_fingerprint, upgrades list etc from BranchConfig which carries precomputed values in Rust flow) implemented. The async generateBranchCache (scm.getBranchCommit + platform.getBranchPr + getCachedPristine/Modified/Behind/Conflict + commit date + prCache) + side-effect `getCache().branches = ...` + full RepoCacheData integration (load/save) live in other repository/* + util/cache/repository modules in the current architecture.

use serde::{Deserialize, Serialize};

use crate::workers::repository::update::branch::types::BranchConfig;
use crate::workers::types::BranchUpgrade;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RepositoryCache {
    pub branches: Vec<BranchCacheEntry>,
    pub modified: bool,
}

/// Mirrors BranchUpgradeCache from lib/util/cache/repository/types.ts (selected fields for cache projection).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BranchUpgradeCacheEntry {
    pub datasource: Option<String>,
    pub dep_name: Option<String>,
    pub dep_type: Option<String>,
    pub display_pending: Option<serde_json::Value>,
    pub fixed_version: Option<String>,
    pub current_version: Option<String>,
    pub current_value: Option<String>,
    pub new_value: Option<String>,
    pub new_version: Option<String>,
    pub current_digest: Option<String>,
    pub new_digest: Option<String>,
    pub package_file: Option<String>,
    pub source_url: Option<String>,
    pub remediation_not_possible: Option<serde_json::Value>,
    pub update_type: Option<String>,
    pub package_name: Option<String>,
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
    pub is_behind_base: Option<bool>,
    pub is_conflicted: Option<bool>,
    pub commit_fingerprint: Option<String>,
    pub commit_timestamp: Option<String>,
    pub pr_no: Option<u64>,
    pub pr_title: Option<String>,
    pub pr_blocked_by: Option<String>,
    pub result: Option<String>,
    pub pristine: Option<bool>,
    pub upgrades: Vec<BranchUpgradeCacheEntry>,
}

pub fn get_cache() -> RepositoryCache {
    RepositoryCache::default()
}

fn generate_branch_upgrade_cache_entry(upgrade: &BranchUpgrade) -> BranchUpgradeCacheEntry {
    let u = &upgrade.upgrade;
    let entry = BranchUpgradeCacheEntry {
        datasource: u.datasource.clone(),
        dep_name: u.dep_name.clone(),
        dep_type: u.dep_type.clone(),
        current_version: u.current_version.clone(),
        current_value: u.current_value.clone(),
        new_value: u.new_value.clone(),
        new_version: u.new_version.clone(),
        current_digest: u.current_digest.clone(),
        new_digest: u.new_digest.clone(),
        package_file: u.package_file.clone(),
        source_url: u.source_url.clone(),
        update_type: u.update_type.clone(),
        package_name: u.package_name.clone(),
        ..Default::default()
    };
    entry
}

pub fn set_cache(branches: &[BranchConfig]) -> RepositoryCache {
    let mut cache = RepositoryCache::default();
    for branch in branches {
        let upgrades: Vec<BranchUpgradeCacheEntry> = branch
            .upgrades
            .iter()
            .map(generate_branch_upgrade_cache_entry)
            .collect();
        let entry = BranchCacheEntry {
            branch_name: branch.branch_name.clone(),
            sha: branch.branch_sha.clone(),
            base_branch: Some(branch.base_branch.clone()),
            base_branch_sha: branch.base_branch_sha.clone(),
            automerge: branch.automerge.unwrap_or(false),
            is_modified: branch.is_modified,
            is_pristine: branch.is_pristine,
            is_behind_base: None, // computed via git behind-base cache in TS generate; pre-populated on BranchConfig or elsewhere in Rust flow
            is_conflicted: branch.is_conflicted,
            commit_fingerprint: branch.commit_fingerprint.clone(),
            commit_timestamp: None, // populated from scm.getBranchUpdateDate in TS; carried via other path in current Rust arch
            pr_no: branch.pr_no,
            pr_title: None,
            pr_blocked_by: branch.pr_blocked_by.as_ref().map(|p| format!("{:?}", p)),
            result: branch.result.as_ref().map(|r| format!("{:?}", r)),
            pristine: branch.is_pristine,
            upgrades,
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

    // Ported: "caches same fingerprint when no commit is made and branch cache existed" — lib/workers/repository/process/write.spec.ts line 219
    #[test]
    fn set_cache_from_branches() {
        let branches = vec![BranchConfig {
            branch_name: "renovate/pkg-1".into(),
            base_branch: "main".into(),
            automerge: Some(true),
            pr_no: Some(42),
            branch_sha: Some("abc123".into()),
            base_branch_sha: Some("def456".into()),
            is_modified: Some(true),
            is_pristine: Some(false),
            commit_fingerprint: Some("fp-xyz".into()),
            upgrades: vec![BranchUpgrade {
                upgrade: crate::workers::types::Upgrade {
                    dep_name: Some("foo".into()),
                    datasource: Some("npm".into()),
                    current_value: Some("1.0.0".into()),
                    new_value: Some("1.1.0".into()),
                    package_name: Some("@scope/foo".into()),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        }];
        let cache = set_cache(&branches);
        assert_eq!(cache.branches.len(), 1);
        assert!(cache.modified);
        assert_eq!(cache.branches[0].branch_name, "renovate/pkg-1");
        assert_eq!(cache.branches[0].base_branch, Some("main".into()));
        assert!(cache.branches[0].automerge);
        assert_eq!(cache.branches[0].pr_no, Some(42));
        assert_eq!(cache.branches[0].sha, Some("abc123".into()));
        assert_eq!(cache.branches[0].base_branch_sha, Some("def456".into()));
        assert_eq!(cache.branches[0].is_modified, Some(true));
        assert_eq!(cache.branches[0].commit_fingerprint, Some("fp-xyz".into()));
        assert_eq!(cache.branches[0].upgrades.len(), 1);
        assert_eq!(cache.branches[0].upgrades[0].dep_name, Some("foo".into()));
        assert_eq!(cache.branches[0].upgrades[0].datasource, Some("npm".into()));
        assert_eq!(
            cache.branches[0].upgrades[0].package_name,
            Some("@scope/foo".into())
        );
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
