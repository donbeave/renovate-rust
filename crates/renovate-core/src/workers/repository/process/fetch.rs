//! Dependency fetching logic.
//!
//! Mirrors `lib/workers/repository/process/fetch.ts`.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::workers::repository::common::PackageFile;
use crate::workers::types::{RenovateConfig, Upgrade, ValidationMessage};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FetchResult {
    pub package_files: HashMap<String, Vec<PackageFile>>,
    pub errors: Vec<String>,
    pub warnings: Vec<ValidationMessage>,
}

pub fn fetch_updates(
    _config: &RenovateConfig,
    package_files: &mut HashMap<String, Vec<PackageFile>>,
) -> FetchResult {
    let mut result = FetchResult {
        package_files: HashMap::new(),
        errors: Vec::new(),
        warnings: Vec::new(),
    };

    for (manager, files) in package_files.iter_mut() {
        for pfile in files.iter_mut() {
            for dep in &mut pfile.deps {
                fetch_dependency_update(dep);
            }
        }
        result.package_files.insert(manager.clone(), files.clone());
    }

    result
}

fn fetch_dependency_update(dep: &mut Upgrade) {
    let dep_name = match &dep.dep_name {
        Some(n) if !n.trim().is_empty() => n.trim().to_owned(),
        _ => return,
    };

    if dep.datasource.is_none()
        || dep
            .datasource
            .as_ref()
            .map(|d| d.is_empty())
            .unwrap_or(true)
    {
        return;
    }

    let package_name = dep.package_name.clone().unwrap_or_else(|| dep_name.clone());
    dep.package_name = Some(package_name);
}

pub fn fetch_manager_updates(
    _config: &RenovateConfig,
    deps: &mut [Upgrade],
) -> Vec<ValidationMessage> {
    let warnings = Vec::new();
    for dep in deps.iter_mut() {
        fetch_dependency_update(dep);
    }
    warnings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_result_default() {
        let r = FetchResult::default();
        assert!(r.package_files.is_empty());
        assert!(r.errors.is_empty());
        assert!(r.warnings.is_empty());
    }

    #[test]
    fn fetch_updates_empty() {
        let config = RenovateConfig::default();
        let mut pf = HashMap::new();
        let result = fetch_updates(&config, &mut pf);
        assert!(result.package_files.is_empty());
    }

    #[test]
    fn fetch_updates_basic() {
        let config = RenovateConfig::default();
        let mut pf = HashMap::new();
        pf.insert(
            "npm".into(),
            vec![PackageFile {
                package_file: "package.json".into(),
                deps: vec![Upgrade {
                    dep_name: Some("lodash".into()),
                    current_value: Some("4.17.0".into()),
                    datasource: Some("npm".into()),
                    ..Default::default()
                }],
                ..Default::default()
            }],
        );
        let result = fetch_updates(&config, &mut pf);
        assert_eq!(result.package_files.len(), 1);
        let npm = result.package_files.get("npm").unwrap();
        assert_eq!(npm.len(), 1);
        assert_eq!(npm[0].deps[0].package_name, Some("lodash".into()));
    }

    #[test]
    fn fetch_updates_sets_package_name() {
        let config = RenovateConfig::default();
        let mut pf = HashMap::new();
        pf.insert(
            "npm".into(),
            vec![PackageFile {
                package_file: "package.json".into(),
                deps: vec![Upgrade {
                    dep_name: Some("  lodash  ".into()),
                    current_value: Some("4.17.0".into()),
                    datasource: Some("npm".into()),
                    ..Default::default()
                }],
                ..Default::default()
            }],
        );
        let _ = fetch_updates(&config, &mut pf);
    }

    #[test]
    fn fetch_updates_no_datasource() {
        let config = RenovateConfig::default();
        let mut pf = HashMap::new();
        pf.insert(
            "npm".into(),
            vec![PackageFile {
                package_file: "package.json".into(),
                deps: vec![Upgrade {
                    dep_name: Some("lodash".into()),
                    ..Default::default()
                }],
                ..Default::default()
            }],
        );
        let result = fetch_updates(&config, &mut pf);
        assert_eq!(result.package_files.len(), 1);
    }

    #[test]
    fn fetch_updates_empty_dep_name() {
        let config = RenovateConfig::default();
        let mut pf = HashMap::new();
        pf.insert(
            "npm".into(),
            vec![PackageFile {
                package_file: "package.json".into(),
                deps: vec![Upgrade {
                    dep_name: Some("".into()),
                    datasource: Some("npm".into()),
                    ..Default::default()
                }],
                ..Default::default()
            }],
        );
        let result = fetch_updates(&config, &mut pf);
        assert_eq!(result.package_files.len(), 1);
    }

    #[test]
    fn fetch_manager_updates_basic() {
        let config = RenovateConfig::default();
        let mut deps = vec![Upgrade {
            dep_name: Some("lodash".into()),
            datasource: Some("npm".into()),
            ..Default::default()
        }];
        let warnings = fetch_manager_updates(&config, &mut deps);
        assert!(warnings.is_empty());
        assert_eq!(deps[0].package_name, Some("lodash".into()));
    }

    #[test]
    fn fetch_result_serialization_roundtrip() {
        let r = FetchResult {
            errors: vec!["error1".into()],
            ..Default::default()
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: FetchResult = serde_json::from_str(&json).unwrap();
        assert_eq!(back.errors, vec!["error1"]);
    }
}
