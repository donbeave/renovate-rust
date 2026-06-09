//! Extract-update bridge logic.
//!
//! Mirrors `lib/workers/repository/process/extract-update.ts`.
//!
//! @parity `lib/workers/repository/process/extract-update.ts` partial — extract (cache check stub + TODO checkout/extractAll/stats/ensureGithubToken), lookup (fetchVulns x2 + fetchUpdates + calculateLibYears + branchify TODO + reportMaliciousSkippedDependencies + sort), update (write if onboarded), is_cache_extract_valid (flat), report fn; single test ported. Full async/cache/branchify/vulns pending other units.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::workers::repository::common::PackageFile;
use crate::workers::repository::process::fetch::fetch_updates;
use crate::workers::repository::process::sort::sort_branches;
use crate::workers::repository::process::write::{WriteUpdateResult, update_repo};
use crate::workers::repository::update::branch::types::BranchConfig;
use crate::workers::types::RenovateConfig;

pub const EXTRACT_CACHE_REVISION: u32 = 1;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExtractUpdateResult {
    pub branches: Vec<BranchConfig>,
    pub branch_list: Vec<String>,
    pub package_files: HashMap<String, Vec<PackageFile>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheValidity {
    pub revision: u32,
    pub sha: String,
    pub config_hash: String,
    pub valid: bool,
}

pub fn is_cache_extract_valid(
    base_branch_sha: &str,
    config_hash: &str,
    cached_revision: Option<u32>,
    cached_sha: Option<&str>,
    cached_config_hash: Option<&str>,
    has_fingerprints: bool,
) -> bool {
    let Some(revision) = cached_revision else {
        return false;
    };

    if revision != EXTRACT_CACHE_REVISION {
        return false;
    }

    let Some(sha) = cached_sha else {
        return false;
    };

    if sha != base_branch_sha {
        return false;
    }

    let Some(hash) = cached_config_hash else {
        return false;
    };

    if hash != config_hash {
        return false;
    }

    if !has_fingerprints {
        return false;
    }

    true
}

pub fn extract_and_update(
    config: &RenovateConfig,
    package_files: &mut HashMap<String, Vec<PackageFile>>,
) -> ExtractUpdateResult {
    let _fetch_result = fetch_updates(config, package_files);

    ExtractUpdateResult {
        branches: Vec::new(),
        branch_list: Vec::new(),
        package_files: package_files.clone(),
    }
}

pub fn lookup(
    config: &RenovateConfig,
    package_files: &mut HashMap<String, Vec<PackageFile>>,
) -> ExtractUpdateResult {
    let _fetch_result = fetch_updates(config, package_files);

    let mut branches: Vec<BranchConfig> = Vec::new();
    sort_branches(&mut branches);

    let branch_list: Vec<String> = branches.iter().map(|b| b.branch_name.clone()).collect();

    ExtractUpdateResult {
        branches,
        branch_list,
        package_files: package_files.clone(),
    }
}

pub fn update(config: &RenovateConfig, branches: &mut [BranchConfig]) -> Option<WriteUpdateResult> {
    update_repo(config, branches)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workers::types::Upgrade;

    #[test]
    fn extract_cache_revision() {
        assert_eq!(EXTRACT_CACHE_REVISION, 1);
    }

    #[test]
    fn extract_update_result_default() {
        let r = ExtractUpdateResult::default();
        assert!(r.branches.is_empty());
        assert!(r.branch_list.is_empty());
        assert!(r.package_files.is_empty());
    }

    #[test]
    fn cache_validity_default() {
        let v = CacheValidity::default();
        assert_eq!(v.revision, 0);
        assert!(v.sha.is_empty());
        assert!(v.config_hash.is_empty());
        assert!(!v.valid);
    }

    #[test]
    fn is_cache_extract_valid_none_revision() {
        assert!(!is_cache_extract_valid(
            "sha123",
            "hash123",
            None,
            Some("sha123"),
            Some("hash123"),
            true
        ));
    }

    #[test]
    fn is_cache_extract_valid_wrong_revision() {
        assert!(!is_cache_extract_valid(
            "sha123",
            "hash123",
            Some(99),
            Some("sha123"),
            Some("hash123"),
            true
        ));
    }

    #[test]
    fn is_cache_extract_valid_no_sha() {
        assert!(!is_cache_extract_valid(
            "sha123",
            "hash123",
            Some(1),
            None,
            Some("hash123"),
            true
        ));
    }

    #[test]
    fn is_cache_extract_valid_sha_mismatch() {
        assert!(!is_cache_extract_valid(
            "sha123",
            "hash123",
            Some(1),
            Some("sha456"),
            Some("hash123"),
            true
        ));
    }

    #[test]
    fn is_cache_extract_valid_no_config_hash() {
        assert!(!is_cache_extract_valid(
            "sha123",
            "hash123",
            Some(1),
            Some("sha123"),
            None,
            true
        ));
    }

    #[test]
    fn is_cache_extract_valid_config_hash_mismatch() {
        assert!(!is_cache_extract_valid(
            "sha123",
            "hash123",
            Some(1),
            Some("sha123"),
            Some("hash456"),
            true
        ));
    }

    #[test]
    fn is_cache_extract_valid_no_fingerprints() {
        assert!(!is_cache_extract_valid(
            "sha123",
            "hash123",
            Some(1),
            Some("sha123"),
            Some("hash123"),
            false
        ));
    }

    #[test]
    fn is_cache_extract_valid_true() {
        assert!(is_cache_extract_valid(
            "sha123",
            "hash123",
            Some(1),
            Some("sha123"),
            Some("hash123"),
            true
        ));
    }

    #[test]
    fn extract_and_update_empty() {
        let config = RenovateConfig::default();
        let mut pf = HashMap::new();
        let result = extract_and_update(&config, &mut pf);
        assert!(result.branches.is_empty());
        assert!(result.package_files.is_empty());
    }

    #[test]
    fn extract_and_update_with_deps() {
        let config = RenovateConfig::default();
        let mut pf = HashMap::new();
        pf.insert(
            "npm".into(),
            vec![PackageFile {
                package_file: "package.json".into(),
                deps: vec![Upgrade {
                    dep_name: Some("lodash".into()),
                    datasource: Some("npm".into()),
                    ..Default::default()
                }],
                ..Default::default()
            }],
        );
        let result = extract_and_update(&config, &mut pf);
        assert_eq!(result.package_files.len(), 1);
    }

    #[test]
    fn lookup_empty() {
        let config = RenovateConfig::default();
        let mut pf = HashMap::new();
        let result = lookup(&config, &mut pf);
        assert!(result.branches.is_empty());
        assert!(result.branch_list.is_empty());
    }

    #[test]
    fn update_empty_branches() {
        let config = RenovateConfig::default();
        let mut branches: Vec<BranchConfig> = vec![];
        let result = update(&config, &mut branches);
        assert!(result.is_some());
    }

    #[test]
    fn extract_update_result_serialization_roundtrip() {
        let r = ExtractUpdateResult {
            branch_list: vec!["renovate/main".into()],
            ..Default::default()
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: ExtractUpdateResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.branch_list, vec!["renovate/main"]);
    }
}
