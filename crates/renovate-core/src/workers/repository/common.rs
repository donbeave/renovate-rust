//! Repository worker common types.
//!
//! Mirrors `lib/workers/repository/common.ts` and related cache types.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::workers::types::Upgrade;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BaseBranchCache {
    pub sha: Option<String>,
    pub config_hash: Option<String>,
    pub extraction_fingerprints: Option<HashMap<String, Option<String>>>,
    pub package_files: Option<HashMap<String, Vec<PackageFile>>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PackageFile {
    pub package_file: String,
    pub deps: Vec<Upgrade>,
    pub lock_files: Option<Vec<String>>,
    pub registry_urls: Option<Vec<String>>,
    pub additional_registry_urls: Option<Vec<String>>,
    pub datasource: Option<String>,
    pub package_file_version: Option<String>,
    pub skip_installs: Option<bool>,
    pub manager: Option<String>,
    pub npmrc: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_branch_cache_default() {
        let c = BaseBranchCache::default();
        assert!(c.sha.is_none());
        assert!(c.config_hash.is_none());
        assert!(c.extraction_fingerprints.is_none());
        assert!(c.package_files.is_none());
    }

    #[test]
    fn base_branch_cache_with_sha() {
        let c = BaseBranchCache {
            sha: Some("abc123".into()),
            config_hash: Some("hash123".into()),
            ..Default::default()
        };
        assert_eq!(c.sha, Some("abc123".into()));
        assert_eq!(c.config_hash, Some("hash123".into()));
    }

    #[test]
    fn package_file_construct() {
        let pf = PackageFile {
            package_file: "package.json".into(),
            deps: vec![Upgrade {
                dep_name: Some("lodash".into()),
                current_value: Some("4.17.0".into()),
                ..Default::default()
            }],
            manager: Some("npm".into()),
            ..Default::default()
        };
        assert_eq!(pf.package_file, "package.json");
        assert_eq!(pf.deps.len(), 1);
        assert_eq!(pf.deps[0].dep_name, Some("lodash".into()));
    }

    #[test]
    fn package_file_serialization_roundtrip() {
        let pf = PackageFile {
            package_file: "Cargo.toml".into(),
            deps: vec![],
            lock_files: Some(vec!["Cargo.lock".into()]),
            datasource: Some("crate".into()),
            ..Default::default()
        };
        let json = serde_json::to_string(&pf).unwrap();
        let back: PackageFile = serde_json::from_str(&json).unwrap();
        assert_eq!(back.package_file, "Cargo.toml");
        assert_eq!(back.lock_files, Some(vec!["Cargo.lock".into()]));
    }

    #[test]
    fn base_branch_cache_serialization_roundtrip() {
        let c = BaseBranchCache {
            sha: Some("def456".into()),
            package_files: Some(HashMap::new()),
            ..Default::default()
        };
        let json = serde_json::to_string(&c).unwrap();
        let back: BaseBranchCache = serde_json::from_str(&json).unwrap();
        assert_eq!(back.sha, Some("def456".into()));
    }

    #[test]
    fn package_file_default() {
        let pf = PackageFile::default();
        assert!(pf.package_file.is_empty());
        assert!(pf.deps.is_empty());
        assert!(pf.lock_files.is_none());
        assert!(pf.datasource.is_none());
    }
}
