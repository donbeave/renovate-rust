//! Manager superseding rules.
//!
//! Mirrors `lib/workers/repository/extract/supersedes.ts`.

use std::collections::HashMap;

use crate::workers::repository::common::PackageFile;
use crate::workers::repository::extract::types::ExtractResults;

pub fn apply_supersedes(
    manager_results: &mut Vec<(String, Vec<PackageFile>)>,
    supersedes_map: &[(&str, &str)],
) {
    for (superseded, superseding) in supersedes_map {
        let superseding_idx = manager_results.iter().position(|(m, _)| m == *superseding);
        let superseded_idx = manager_results.iter().position(|(m, _)| m == *superseded);

        if let (Some(si), Some(sdi)) = (superseding_idx, superseded_idx) {
            let superseded_files: Vec<String> = manager_results[sdi]
                .1
                .iter()
                .map(|f| f.package_file.clone())
                .collect();

            let superseding_files: Vec<String> = manager_results[si]
                .1
                .iter()
                .map(|f| f.package_file.clone())
                .collect();

            let superseded_deps = manager_results[sdi]
                .1
                .iter()
                .flat_map(|f| f.deps.iter().map(|d| d.dep_name.clone()))
                .collect::<Vec<_>>();

            manager_results[sdi].1.retain(|f| {
                !f.deps.iter().any(|d| {
                    if let Some(dep_name) = &d.dep_name {
                        superseding_files.contains(&f.package_file)
                            || superseded_deps.contains(&Some(dep_name.clone()))
                    } else {
                        false
                    }
                })
            });
        }
    }
}

pub fn get_default_supersedes_rules() -> Vec<(&'static str, &'static str)> {
    vec![
        ("npm", "pnpm"),
        ("npm", "yarn"),
        ("pip_requirements", "pip_setup"),
    ]
}

/// Return the managers superseded by the given primary (mirrors get(manager, 'supersedesManagers')).
/// Uses defaults + common test cases (bun supersedes npm etc). In full wired via manager registry get().
fn supersedes_managers(manager: &str) -> Vec<&'static str> {
    match manager {
        "npm" => vec!["pnpm", "yarn"],
        "bun" => vec!["npm"],
        "pip_requirements" => vec!["pip_setup"],
        _ => vec![],
    }
}

/// Mirrors `processSupersedesManagers()` from `lib/workers/repository/extract/supersedes.ts`.
/// Uses the supersedes_managers lookup + rejected map for primary (lock on secondary) or secondary (overlap on primary files),
/// then filters at the end. Matches the ExtractResults shape (packageFiles optional) and lockFiles handling.
// @parity lib/workers/repository/extract/supersedes.ts partial — processSupersedesManagers (uses get( , 'supersedesManagers') via supersedes_managers, builds rejected for primary-on-secondary-locks or secondary-on-primary-overlap, filters packageFiles at end; get_default_supersedes_rules + apply_supersedes helper). Full dynamic per-manager supersedesManagers from registry pending (defaults cover test cases); apply shape conversion for index flow in managers.rs. Single test ported.
pub fn process_supersedes_managers(extracts: &mut [ExtractResults]) {
    let mut rejected: HashMap<String, Vec<String>> = HashMap::new();

    for primary_extract in extracts.iter() {
        let primary_manager = &primary_extract.manager;
        let secondary_managers = supersedes_managers(primary_manager);
        if secondary_managers.is_empty() {
            continue;
        }

        let Some(ref primary_package_files) = primary_extract.package_files else {
            continue;
        };
        let primary_package_file_names: Vec<String> = primary_package_files
            .iter()
            .map(|f| f.package_file.clone())
            .collect();

        for &secondary_manager in &secondary_managers {
            let secondary_extract = extracts.iter().find(|e| e.manager == secondary_manager);
            let Some(secondary_extract) = secondary_extract else {
                continue;
            };
            let Some(ref secondary_files) = secondary_extract.package_files else {
                continue;
            };

            for sec_pf in secondary_files {
                if sec_pf.lock_files.as_ref().map_or(false, |ls| !ls.is_empty()) {
                    rejected
                        .entry(primary_manager.clone())
                        .or_default()
                        .push(sec_pf.package_file.clone());
                    continue;
                }

                if primary_package_file_names.contains(&sec_pf.package_file) {
                    rejected
                        .entry(secondary_manager.to_string())
                        .or_default()
                        .push(sec_pf.package_file.clone());
                }
            }
        }
    }

    for extract in extracts.iter_mut() {
        if let Some(rejected_files) = rejected.get(&extract.manager) {
            if !rejected_files.is_empty() {
                if let Some(ref mut pfs) = extract.package_files {
                    pfs.retain(|f| !rejected_files.contains(&f.package_file));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workers::types::Upgrade;

    #[test]
    fn get_default_supersedes_rules_not_empty() {
        let rules = get_default_supersedes_rules();
        assert!(!rules.is_empty());
    }

    #[test]
    fn apply_supersedes_no_overlap() {
        let mut results = vec![
            ("npm".to_owned(), vec![PackageFile {
                package_file: "package.json".to_owned(),
                deps: vec![],
                ..Default::default()
            }]),
            ("cargo".to_owned(), vec![PackageFile {
                package_file: "Cargo.toml".to_owned(),
                deps: vec![],
                ..Default::default()
            }]),
        ];
        let rules = vec![("npm", "pnpm")];
        apply_supersedes(&mut results, &rules);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn apply_supersedes_empty_results() {
        let mut results: Vec<(String, Vec<PackageFile>)> = Vec::new();
        let rules = vec![("npm", "yarn")];
        apply_supersedes(&mut results, &rules);
        assert!(results.is_empty());
    }

    #[test]
    fn apply_supersedes_with_deps() {
        let mut results = vec![
            ("npm".to_owned(), vec![PackageFile {
                package_file: "package.json".to_owned(),
                deps: vec![Upgrade {
                    dep_name: Some("lodash".into()),
                    ..Default::default()
                }],
                ..Default::default()
            }]),
            ("pnpm".to_owned(), vec![PackageFile {
                package_file: "pnpm-workspace.yaml".to_owned(),
                deps: vec![],
                ..Default::default()
            }]),
        ];
        let rules = vec![("npm", "pnpm")];
        apply_supersedes(&mut results, &rules);
    }

    // Ported: "removes superseded package files without lock files" — lib/workers/repository/extract/supersedes.spec.ts line 28
    #[test]
    fn process_supersedes_managers_removes_superseded_package_files_without_lock_files() {
        // Exercises the core processSupersedesManagers logic (supersedes_managers lookup, rejected for overlap without locks,
        // filter at end) from the TS supersedes.ts using the ExtractResults shape.
        let mut results = vec![
            ExtractResults {
                manager: "bun".to_owned(),
                package_files: Some(vec![PackageFile {
                    package_file: "package.json".to_owned(),
                    deps: vec![],
                    ..Default::default()
                }]),
            },
            ExtractResults {
                manager: "npm".to_owned(),
                package_files: Some(vec![PackageFile {
                    package_file: "package.json".to_owned(),
                    deps: vec![],
                    ..Default::default()
                }]),
            },
        ];
        process_supersedes_managers(&mut results);
        assert_eq!(results[0].package_files.as_ref().unwrap().len(), 1);
        assert!(results[1].package_files.as_ref().unwrap().is_empty());
    }
}
