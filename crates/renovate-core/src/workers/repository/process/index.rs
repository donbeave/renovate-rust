//! Process orchestrator.
//!
//! Mirrors `lib/workers/repository/process/index.ts`.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::workers::repository::common::PackageFile;
use crate::workers::repository::update::branch::types::BranchConfig;
use crate::workers::types::RenovateConfig;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcessResult {
    pub branches: Vec<BranchConfig>,
    pub branch_list: Vec<String>,
    pub package_files: HashMap<String, Vec<PackageFile>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatsResult {
    pub file_count: usize,
    pub dep_count: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Stats {
    pub managers: HashMap<String, StatsResult>,
    pub total: StatsResult,
}

pub fn extract_stats(package_files: &HashMap<String, Vec<PackageFile>>) -> Stats {
    let mut stats = Stats::default();
    for (manager, files) in package_files {
        let file_count = files.len();
        let dep_count: usize = files.iter().map(|f| f.deps.len()).sum();
        stats.managers.insert(
            manager.clone(),
            StatsResult {
                file_count,
                dep_count,
            },
        );
        stats.total.file_count += file_count;
        stats.total.dep_count += dep_count;
    }
    stats
}

pub fn is_multi_base_branch(_config: &RenovateConfig, base_branch_patterns: &[String]) -> bool {
    if base_branch_patterns.is_empty() {
        return false;
    }
    base_branch_patterns.len() > 1 || base_branch_patterns[0].starts_with('/')
}

pub fn process_repository(_config: &RenovateConfig) -> ProcessResult {
    ProcessResult {
        branches: Vec::new(),
        branch_list: Vec::new(),
        package_files: HashMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workers::types::Upgrade;

    #[test]
    fn process_result_default() {
        let r = ProcessResult::default();
        assert!(r.branches.is_empty());
        assert!(r.branch_list.is_empty());
        assert!(r.package_files.is_empty());
    }

    #[test]
    fn process_result_construct() {
        let r = ProcessResult {
            branches: vec![BranchConfig::default()],
            branch_list: vec!["renovate/lodash-4.x".into()],
            package_files: HashMap::new(),
        };
        assert_eq!(r.branches.len(), 1);
        assert_eq!(r.branch_list.len(), 1);
    }

    #[test]
    fn extract_stats_empty() {
        let stats = extract_stats(&HashMap::new());
        assert!(stats.managers.is_empty());
        assert_eq!(stats.total.file_count, 0);
        assert_eq!(stats.total.dep_count, 0);
    }

    #[test]
    fn extract_stats_with_files() {
        let mut pf = HashMap::new();
        pf.insert(
            "npm".into(),
            vec![PackageFile {
                package_file: "package.json".into(),
                deps: vec![
                    Upgrade {
                        dep_name: Some("lodash".into()),
                        ..Default::default()
                    },
                    Upgrade {
                        dep_name: Some("express".into()),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
        );
        pf.insert(
            "cargo".into(),
            vec![PackageFile {
                package_file: "Cargo.toml".into(),
                deps: vec![Upgrade {
                    dep_name: Some("serde".into()),
                    ..Default::default()
                }],
                ..Default::default()
            }],
        );
        let stats = extract_stats(&pf);
        assert_eq!(stats.managers.len(), 2);
        assert_eq!(stats.total.file_count, 2);
        assert_eq!(stats.total.dep_count, 3);
        assert_eq!(stats.managers["npm"].file_count, 1);
        assert_eq!(stats.managers["npm"].dep_count, 2);
        assert_eq!(stats.managers["cargo"].file_count, 1);
        assert_eq!(stats.managers["cargo"].dep_count, 1);
    }

    #[test]
    fn is_multi_base_branch_empty() {
        let config = RenovateConfig::default();
        assert!(!is_multi_base_branch(&config, &[]));
    }

    #[test]
    fn is_multi_base_branch_single() {
        let config = RenovateConfig::default();
        assert!(!is_multi_base_branch(&config, &["main".into()]));
    }

    #[test]
    fn is_multi_base_branch_multiple() {
        let config = RenovateConfig::default();
        assert!(is_multi_base_branch(&config, &["main".into(), "develop".into()]));
    }

    #[test]
    fn is_multi_base_branch_regex() {
        let config = RenovateConfig::default();
        assert!(is_multi_base_branch(&config, &["/^release/.*/".into()]));
    }

    #[test]
    fn process_repository_returns_empty() {
        let config = RenovateConfig::default();
        let result = process_repository(&config);
        assert!(result.branches.is_empty());
        assert!(result.branch_list.is_empty());
    }

    #[test]
    fn stats_result_default() {
        let s = StatsResult::default();
        assert_eq!(s.file_count, 0);
        assert_eq!(s.dep_count, 0);
    }

    #[test]
    fn stats_default() {
        let s = Stats::default();
        assert!(s.managers.is_empty());
        assert_eq!(s.total.file_count, 0);
    }

    #[test]
    fn process_result_serialization_roundtrip() {
        let r = ProcessResult {
            branches: vec![],
            branch_list: vec!["renovate-main".to_owned()],
            package_files: HashMap::new(),
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: ProcessResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.branch_list, vec!["renovate-main".to_owned()]);
    }
}
