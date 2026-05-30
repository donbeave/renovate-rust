//! Extract result types.
//!
//! Mirrors `lib/workers/repository/extract/types.ts` (`ExtractResults`)
//! and `lib/workers/types.ts` (`ExtractResult`).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::workers::repository::common::PackageFile;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExtractResult {
    pub extraction_fingerprints: HashMap<String, Option<String>>,
    pub package_files: HashMap<String, Vec<PackageFile>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExtractResults {
    pub manager: String,
    pub package_files: Option<Vec<PackageFile>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ManagerFile {
    pub manager: String,
    pub file_list: Vec<String>,
    pub enabled: bool,
    pub manager_file_patterns: Option<Vec<String>>,
    pub include_paths: Option<Vec<String>>,
    pub ignore_paths: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workers::types::Upgrade;

    #[test]
    fn extract_result_default() {
        let r = ExtractResult::default();
        assert!(r.extraction_fingerprints.is_empty());
        assert!(r.package_files.is_empty());
    }

    #[test]
    fn extract_result_with_package_files() {
        let pf = PackageFile {
            package_file: "package.json".into(),
            deps: vec![Upgrade {
                dep_name: Some("express".into()),
                ..Default::default()
            }],
            ..Default::default()
        };
        let mut r = ExtractResult::default();
        r.package_files.insert("npm".into(), vec![pf]);
        r.extraction_fingerprints
            .insert("npm".into(), Some("fp1".into()));

        assert_eq!(r.package_files.len(), 1);
        assert_eq!(r.extraction_fingerprints.len(), 1);
        let npm_files = r.package_files.get("npm").unwrap();
        assert_eq!(npm_files.len(), 1);
        assert_eq!(npm_files[0].package_file, "package.json");
    }

    #[test]
    fn extract_results_construct() {
        let er = ExtractResults {
            manager: "cargo".into(),
            package_files: Some(vec![]),
        };
        assert_eq!(er.manager, "cargo");
        assert!(er.package_files.is_some());
    }

    #[test]
    fn extract_results_null_package_files() {
        let er = ExtractResults {
            manager: "npm".into(),
            package_files: None,
        };
        assert!(er.package_files.is_none());
    }

    #[test]
    fn manager_file_construct() {
        let mf = ManagerFile {
            manager: "npm".into(),
            file_list: vec!["package.json".into()],
            enabled: true,
            ..Default::default()
        };
        assert_eq!(mf.manager, "npm");
        assert_eq!(mf.file_list.len(), 1);
        assert!(mf.enabled);
    }

    #[test]
    fn extract_result_serialization_roundtrip() {
        let mut r = ExtractResult::default();
        r.package_files.insert("npm".into(), vec![]);
        r.extraction_fingerprints
            .insert("npm".into(), Some("fp".into()));
        let json = serde_json::to_string(&r).unwrap();
        let back: ExtractResult = serde_json::from_str(&json).unwrap();
        assert!(back.package_files.contains_key("npm"));
        assert_eq!(
            back.extraction_fingerprints.get("npm"),
            Some(&Some("fp".into()))
        );
    }
}
