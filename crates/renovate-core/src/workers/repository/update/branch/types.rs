//! Branch update types.
//!
//! Mirrors `lib/workers/types.ts` (`BranchConfig`, `BranchResult`,
//! `PrBlockedBy`, `CacheFingerprintMatchResult`).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::workers::repository::common::PackageFile;
use crate::workers::types::{
    BranchResult, BranchUpgrade, PrBlockedBy, RenovateConfig,
    ValidationMessage,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum CacheFingerprintMatchResult {
    Matched,
    NoMatch,
    #[default]
    NoFingerprint,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BranchConfig {
    pub branch_name: String,
    pub base_branch: String,
    pub upgrades: Vec<BranchUpgrade>,
    pub result: Option<BranchResult>,
    pub pr_blocked_by: Option<PrBlockedBy>,
    pub pr_no: Option<u64>,
    pub automerge_comment: Option<String>,
    pub automerged_previously: Option<bool>,
    pub branch_automerge_failure_message: Option<String>,
    pub confidence_status: Option<String>,
    pub dependency_dashboard_rebase_all_open: Option<bool>,
    pub dependency_dashboard_all_pending: Option<bool>,
    pub dependency_dashboard_all_rate_limited: Option<bool>,
    pub dependency_dashboard_all_awaiting_schedule: Option<bool>,
    pub errors: Option<Vec<ValidationMessage>>,
    pub force_pr: Option<bool>,
    pub has_types: Option<bool>,
    pub is_modified: Option<bool>,
    pub is_scheduled_now: Option<bool>,
    pub dependency_dashboard_checks: Option<HashMap<String, String>>,
    pub dependency_dashboard_pr_approval: Option<bool>,
    pub release_timestamp: Option<String>,
    pub force_commit: Option<bool>,
    pub rebase_requested: Option<bool>,
    pub package_files: Option<HashMap<String, Vec<PackageFile>>>,
    pub stability_status: Option<String>,
    pub stop_updating: Option<bool>,
    pub is_conflicted: Option<bool>,
    pub commit_fingerprint: Option<String>,
    pub cache_fingerprint_match: Option<CacheFingerprintMatchResult>,
    pub pr_not_pending_hours: Option<u64>,
    pub is_pristine: Option<bool>,
    pub branch_sha: Option<String>,
    pub base_branch_sha: Option<String>,
    pub automerge: Option<bool>,
    pub is_vulnerability_alert: Option<bool>,
    pub config: RenovateConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_fingerprint_default_is_no_fingerprint() {
        assert_eq!(
            CacheFingerprintMatchResult::default(),
            CacheFingerprintMatchResult::NoFingerprint
        );
    }

    #[test]
    fn cache_fingerprint_variants() {
        assert_ne!(
            CacheFingerprintMatchResult::Matched,
            CacheFingerprintMatchResult::NoMatch
        );
        assert_ne!(
            CacheFingerprintMatchResult::NoMatch,
            CacheFingerprintMatchResult::NoFingerprint
        );
    }

    #[test]
    fn branch_config_default() {
        let c = BranchConfig::default();
        assert!(c.branch_name.is_empty());
        assert!(c.base_branch.is_empty());
        assert!(c.upgrades.is_empty());
        assert!(c.result.is_none());
        assert!(c.pr_blocked_by.is_none());
    }

    #[test]
    fn branch_config_construct() {
        let c = BranchConfig {
            branch_name: "renovate/lodash-4.x".into(),
            base_branch: "main".into(),
            upgrades: vec![BranchUpgrade {
                upgrade: crate::workers::types::Upgrade {
                    dep_name: Some("lodash".into()),
                    current_value: Some("4.17.0".into()),
                    new_value: Some("4.18.2".into()),
                    ..Default::default()
                },
                branch_name: Some("renovate/lodash-4.x".into()),
                ..Default::default()
            }],
            result: Some(BranchResult::PrCreated),
            ..Default::default()
        };
        assert_eq!(c.branch_name, "renovate/lodash-4.x");
        assert_eq!(c.base_branch, "main");
        assert_eq!(c.upgrades.len(), 1);
        assert_eq!(c.result, Some(BranchResult::PrCreated));
    }

    #[test]
    fn branch_config_serialization_roundtrip() {
        let c = BranchConfig {
            branch_name: "renovate/react-18.x".into(),
            base_branch: "develop".into(),
            is_modified: Some(true),
            automerge: Some(false),
            cache_fingerprint_match: Some(CacheFingerprintMatchResult::Matched),
            ..Default::default()
        };
        let json = serde_json::to_string(&c).unwrap();
        let back: BranchConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(back.branch_name, "renovate/react-18.x");
        assert_eq!(back.base_branch, "develop");
        assert_eq!(back.is_modified, Some(true));
        assert_eq!(
            back.cache_fingerprint_match,
            Some(CacheFingerprintMatchResult::Matched)
        );
    }

    #[test]
    fn branch_result_serialization_roundtrip() {
        let results = [
            BranchResult::Done,
            BranchResult::PrCreated,
            BranchResult::Automerged,
            BranchResult::Error,
            BranchResult::Pending,
        ];
        for r in &results {
            let json = serde_json::to_string(r).unwrap();
            let back: BranchResult = serde_json::from_str(&json).unwrap();
            assert_eq!(*r, back);
        }
    }

    #[test]
    fn pr_blocked_by_variants_distinct() {
        let v1 = PrBlockedBy::BranchAutomerge;
        let v2 = PrBlockedBy::NeedsApproval;
        let v3 = PrBlockedBy::AwaitingTests;
        let v4 = PrBlockedBy::RateLimited;
        let v5 = PrBlockedBy::Error;
        let all = [v1, v2, v3, v4, v5];
        for (i, a) in all.iter().enumerate() {
            for (j, b) in all.iter().enumerate() {
                if i != j {
                    assert_ne!(a, b);
                }
            }
        }
    }

    #[test]
    fn cache_fingerprint_serialization() {
        let variants = [
            CacheFingerprintMatchResult::Matched,
            CacheFingerprintMatchResult::NoMatch,
            CacheFingerprintMatchResult::NoFingerprint,
        ];
        for v in &variants {
            let json = serde_json::to_string(v).unwrap();
            let back: CacheFingerprintMatchResult = serde_json::from_str(&json).unwrap();
            assert_eq!(*v, back);
        }
    }

    #[test]
    fn branch_config_with_package_files() {
        let mut c = BranchConfig {
            branch_name: "renovate/main".into(),
            base_branch: "main".into(),
            ..Default::default()
        };
        let mut pf_map = HashMap::new();
        pf_map.insert(
            "npm".into(),
            vec![crate::workers::repository::common::PackageFile {
                package_file: "package.json".into(),
                ..Default::default()
            }],
        );
        c.package_files = Some(pf_map);
        assert!(c.package_files.is_some());
        assert_eq!(c.package_files.as_ref().unwrap().len(), 1);
    }
}
