//! Package file collection.
//!
//! Mirrors `lib/workers/repository/package-files.ts`.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::workers::repository::common::PackageFile;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PackageFiles {
    pub data: HashMap<String, HashMap<String, Vec<PackageFile>>>,
}

impl PackageFiles {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, base_branch: &str, package_files: HashMap<String, Vec<PackageFile>>) {
        self.data.insert(base_branch.to_owned(), package_files);
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn total_file_count(&self) -> usize {
        self.data
            .values()
            .flat_map(|m| m.values())
            .map(|v| v.len())
            .sum()
    }

    pub fn total_dep_count(&self) -> usize {
        self.data
            .values()
            .flat_map(|m| m.values())
            .flat_map(|v| v.iter())
            .map(|f| f.deps.len())
            .sum()
    }

    pub fn get_dashboard_markdown(&self) -> String {
        if self.data.is_empty() {
            return "None detected\n\n".to_owned();
        }

        let mut md = String::from("## Detected Dependencies\n\n");

        let mut branches: Vec<_> = self.data.keys().collect();
        branches.sort();

        for branch in branches {
            if let Some(managers) = self.data.get(branch) {
                let mut manager_names: Vec<_> = managers.keys().collect();
                manager_names.sort();

                for manager in manager_names {
                    if let Some(files) = managers.get(manager) {
                        md.push_str(&format!(
                            "<details><summary>{} ({})</summary>\n<blockquote>\n\n",
                            manager,
                            files.len()
                        ));
                        for file in files {
                            md.push_str(&format!(
                                "<details><summary>{} ({})</summary>\n\n",
                                file.package_file,
                                file.deps.len()
                            ));
                            for dep in &file.deps {
                                let version = dep
                                    .current_value
                                    .as_deref()
                                    .unwrap_or("unknown version");
                                let name = dep.dep_name.as_deref().unwrap_or("unknown");
                                md.push_str(&format!(" - `{name} {version}`\n"));
                            }
                            md.push_str("\n</details>\n\n");
                        }
                        md.push_str("</blockquote>\n</details>\n\n");
                    }
                }
            }
        }

        md
    }
}

pub fn collect_package_files(
    extractions: &HashMap<String, Vec<PackageFile>>,
) -> PackageFiles {
    let mut pf = PackageFiles::new();
    pf.add("main", extractions.clone());
    pf
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workers::types::Upgrade;

    #[test]
    fn package_files_new() {
        let pf = PackageFiles::new();
        assert!(pf.is_empty());
        assert!(pf.data.is_empty());
    }

    #[test]
    fn package_files_add() {
        let mut pf = PackageFiles::new();
        let mut files = HashMap::new();
        files.insert(
            "npm".to_owned(),
            vec![PackageFile {
                package_file: "package.json".to_owned(),
                deps: vec![],
                ..Default::default()
            }],
        );
        pf.add("main", files);
        assert!(!pf.is_empty());
        assert_eq!(pf.total_file_count(), 1);
    }

    #[test]
    fn package_files_clear() {
        let mut pf = PackageFiles::new();
        pf.add("main", HashMap::new());
        pf.clear();
        assert!(pf.is_empty());
    }

    #[test]
    fn package_files_total_counts() {
        let mut pf = PackageFiles::new();
        let mut files = HashMap::new();
        files.insert(
            "npm".to_owned(),
            vec![PackageFile {
                package_file: "package.json".to_owned(),
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
        pf.add("main", files);
        assert_eq!(pf.total_file_count(), 1);
        assert_eq!(pf.total_dep_count(), 2);
    }

    #[test]
    fn package_files_get_dashboard_markdown_empty() {
        let pf = PackageFiles::new();
        let md = pf.get_dashboard_markdown();
        assert!(md.contains("None detected"));
    }

    #[test]
    fn package_files_get_dashboard_markdown_with_files() {
        let mut pf = PackageFiles::new();
        let mut files = HashMap::new();
        files.insert(
            "npm".to_owned(),
            vec![PackageFile {
                package_file: "package.json".to_owned(),
                deps: vec![],
                ..Default::default()
            }],
        );
        pf.add("main", files);
        let md = pf.get_dashboard_markdown();
        assert!(md.contains("Detected Dependencies"));
        assert!(md.contains("npm"));
        assert!(md.contains("package.json"));
    }

    #[test]
    fn collect_package_files_creates_instance() {
        let mut extractions = HashMap::new();
        extractions.insert(
            "cargo".to_owned(),
            vec![PackageFile {
                package_file: "Cargo.toml".to_owned(),
                deps: vec![],
                ..Default::default()
            }],
        );
        let pf = collect_package_files(&extractions);
        assert!(!pf.is_empty());
        assert_eq!(pf.total_file_count(), 1);
    }

    #[test]
    fn package_files_serialization_roundtrip() {
        let mut pf = PackageFiles::new();
        pf.add("main", HashMap::new());
        let json = serde_json::to_string(&pf).unwrap();
        let back: PackageFiles = serde_json::from_str(&json).unwrap();
        assert!(back.data.contains_key("main"));
    }
}
