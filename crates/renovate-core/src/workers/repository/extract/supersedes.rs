//! Manager superseding rules.
//!
//! Mirrors `lib/workers/repository/extract/supersedes.ts`.

use crate::workers::repository::common::PackageFile;

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
}
