//! Write updates logic.
//!
//! Mirrors `lib/workers/repository/process/write.ts`.

use serde::{Deserialize, Serialize};

use crate::workers::repository::update::branch::types::{
    BranchConfig, CacheFingerprintMatchResult,
};
use crate::workers::types::{RenovateConfig, UpgradeFingerprintConfig};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum WriteUpdateResult {
    #[default]
    Done,
    Automerged,
}

pub fn generate_commit_fingerprint_config(branch: &BranchConfig) -> Vec<UpgradeFingerprintConfig> {
    branch
        .upgrades
        .iter()
        .map(|upgrade| UpgradeFingerprintConfig {
            auto_replace_string_template: upgrade.auto_replace_string_template.clone(),
            current_digest: upgrade.upgrade.current_digest.clone(),
            current_value: upgrade.upgrade.current_value.clone(),
            current_version: upgrade.upgrade.current_version.clone(),
            datasource: upgrade.upgrade.datasource.clone(),
            dep_name: upgrade.upgrade.dep_name.clone(),
            lock_file: upgrade.upgrade.lock_file.clone(),
            locked_version: None,
            manager: upgrade.upgrade.manager.clone(),
            new_name: upgrade.upgrade.new_name.clone(),
            new_digest: upgrade.upgrade.new_digest.clone(),
            new_value: upgrade.upgrade.new_value.clone(),
            new_version: upgrade.upgrade.new_version.clone(),
            package_file: upgrade.upgrade.package_file.clone(),
            replace_string: None,
        })
        .collect()
}

pub fn compare_cache_fingerprint(
    cached_fingerprint: Option<&str>,
    commit_fingerprint: &str,
) -> CacheFingerprintMatchResult {
    match cached_fingerprint {
        None => CacheFingerprintMatchResult::NoFingerprint,
        Some(fp) if fp != commit_fingerprint => CacheFingerprintMatchResult::NoMatch,
        Some(_) => CacheFingerprintMatchResult::Matched,
    }
}

pub fn write_updates(_config: &RenovateConfig, branches: &mut [BranchConfig]) -> WriteUpdateResult {
    for branch in branches.iter_mut() {
        let result = process_branch(branch);
        branch.result = Some(result);
        if result == crate::workers::types::BranchResult::Automerged {
            return WriteUpdateResult::Automerged;
        }
    }
    WriteUpdateResult::Done
}

fn process_branch(branch: &mut BranchConfig) -> crate::workers::types::BranchResult {
    if branch.upgrades.is_empty() {
        return crate::workers::types::BranchResult::NoWork;
    }
    crate::workers::types::BranchResult::Done
}

pub fn update_repo(
    config: &RenovateConfig,
    branches: &mut [BranchConfig],
) -> Option<WriteUpdateResult> {
    Some(write_updates(config, branches))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workers::types::{BranchUpgrade, Upgrade};

    fn make_branch(name: &str) -> BranchConfig {
        BranchConfig {
            branch_name: name.into(),
            base_branch: "main".into(),
            ..Default::default()
        }
    }

    fn make_branch_with_upgrade(name: &str) -> BranchConfig {
        BranchConfig {
            branch_name: name.into(),
            base_branch: "main".into(),
            upgrades: vec![BranchUpgrade {
                upgrade: Upgrade {
                    dep_name: Some("lodash".into()),
                    current_value: Some("4.17.0".into()),
                    new_value: Some("4.18.2".into()),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        }
    }

    #[test]
    fn write_update_result_default() {
        assert_eq!(WriteUpdateResult::default(), WriteUpdateResult::Done);
    }

    #[test]
    fn write_update_result_variants() {
        assert_ne!(WriteUpdateResult::Done, WriteUpdateResult::Automerged);
    }

    #[test]
    fn write_update_result_serialization() {
        let json = serde_json::to_string(&WriteUpdateResult::Done).unwrap();
        let back: WriteUpdateResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back, WriteUpdateResult::Done);
    }

    #[test]
    fn compare_cache_fingerprint_none() {
        let result = compare_cache_fingerprint(None, "abc");
        assert_eq!(result, CacheFingerprintMatchResult::NoFingerprint);
    }

    #[test]
    fn compare_cache_fingerprint_match() {
        let result = compare_cache_fingerprint(Some("abc"), "abc");
        assert_eq!(result, CacheFingerprintMatchResult::Matched);
    }

    #[test]
    fn compare_cache_fingerprint_no_match() {
        let result = compare_cache_fingerprint(Some("abc"), "def");
        assert_eq!(result, CacheFingerprintMatchResult::NoMatch);
    }

    #[test]
    fn generate_commit_fingerprint_config_empty() {
        let branch = make_branch("test");
        let configs = generate_commit_fingerprint_config(&branch);
        assert!(configs.is_empty());
    }

    #[test]
    fn generate_commit_fingerprint_config_with_upgrade() {
        let branch = make_branch_with_upgrade("test");
        let configs = generate_commit_fingerprint_config(&branch);
        assert_eq!(configs.len(), 1);
        assert_eq!(configs[0].dep_name, Some("lodash".into()));
        assert_eq!(configs[0].current_value, Some("4.17.0".into()));
        assert_eq!(configs[0].new_value, Some("4.18.2".into()));
    }

    #[test]
    fn write_updates_empty() {
        let config = RenovateConfig::default();
        let mut branches: Vec<BranchConfig> = vec![];
        let result = write_updates(&config, &mut branches);
        assert_eq!(result, WriteUpdateResult::Done);
    }

    #[test]
    fn write_updates_no_work() {
        let config = RenovateConfig::default();
        let mut branches = vec![make_branch("test")];
        let result = write_updates(&config, &mut branches);
        assert_eq!(result, WriteUpdateResult::Done);
        assert_eq!(
            branches[0].result,
            Some(crate::workers::types::BranchResult::NoWork)
        );
    }

    #[test]
    fn write_updates_with_upgrade() {
        let config = RenovateConfig::default();
        let mut branches = vec![make_branch_with_upgrade("test")];
        let result = write_updates(&config, &mut branches);
        assert_eq!(result, WriteUpdateResult::Done);
        assert_eq!(
            branches[0].result,
            Some(crate::workers::types::BranchResult::Done)
        );
    }

    #[test]
    fn update_repo_basic() {
        let config = RenovateConfig::default();
        let mut branches = vec![make_branch_with_upgrade("test")];
        let result = update_repo(&config, &mut branches);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), WriteUpdateResult::Done);
    }

    #[test]
    fn update_repo_empty() {
        let config = RenovateConfig::default();
        let mut branches: Vec<BranchConfig> = vec![];
        let result = update_repo(&config, &mut branches);
        assert!(result.is_some());
    }
}
