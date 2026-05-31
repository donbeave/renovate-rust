//! Dependency dashboard formatting.
//!
//! Mirrors `lib/workers/repository/dependency-dashboard.ts`.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::workers::repository::common::PackageFile;
use crate::workers::repository::update::branch::types::BranchConfig;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DashboardEntry {
    pub branch_name: String,
    pub pr_title: Option<String>,
    pub state: Option<String>,
    pub result: Option<String>,
    pub is_modified: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DependencyDashboard {
    pub title: String,
    pub body: String,
    pub entries: Vec<DashboardEntry>,
    pub package_files: HashMap<String, Vec<PackageFile>>,
}

impl DependencyDashboard {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_owned(),
            body: String::new(),
            entries: Vec::new(),
            package_files: HashMap::new(),
        }
    }
}

pub fn format_dashboard(
    branches: &[BranchConfig],
    package_files: &HashMap<String, Vec<PackageFile>>,
    errors: &[String],
    warnings: &[String],
) -> DependencyDashboard {
    let mut dashboard = DependencyDashboard::new("Dependency Dashboard");

    let mut body = String::new();

    if !branches.is_empty() {
        body.push_str("### Pending Approval\n\n");
        body.push_str("These PRs will be created by Renovate only when approved.\n\n");
        for branch in branches {
            let entry = DashboardEntry {
                branch_name: branch.branch_name.clone(),
                pr_title: Some(branch.branch_name.clone()),
                state: None,
                result: None,
                is_modified: None,
            };
            dashboard.entries.push(entry);
        }
    }

    if !package_files.is_empty() {
        body.push_str("### Detected Dependencies\n\n");
        for (manager, files) in package_files {
            for file in files {
                body.push_str(&format!("- `{}` ({})\n", file.package_file, manager));
            }
        }
        body.push('\n');
    }

    if !errors.is_empty() {
        body.push_str(&format!("### Errors ({})\n\n", errors.len()));
        for e in errors {
            body.push_str(&format!("- {}\n", e));
        }
        body.push('\n');
    }

    if !warnings.is_empty() {
        body.push_str(&format!("### Warnings ({})\n\n", warnings.len()));
        for w in warnings {
            body.push_str(&format!("- {}\n", w));
        }
        body.push('\n');
    }

    dashboard.body = body;
    dashboard
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workers::types::Upgrade;

    #[test]
    fn dashboard_entry_default() {
        let e = DashboardEntry::default();
        assert!(e.branch_name.is_empty());
        assert!(e.pr_title.is_none());
    }

    #[test]
    fn dependency_dashboard_new() {
        let d = DependencyDashboard::new("My Dashboard");
        assert_eq!(d.title, "My Dashboard");
        assert!(d.body.is_empty());
        assert!(d.entries.is_empty());
    }

    #[test]
    fn format_dashboard_empty() {
        let d = format_dashboard(&[], &HashMap::new(), &[], &[]);
        assert!(d.body.is_empty());
        assert!(d.entries.is_empty());
    }

    #[test]
    fn format_dashboard_with_branches() {
        let branches = vec![BranchConfig {
            branch_name: "renovate/lodash-4.x".into(),
            ..Default::default()
        }];
        let d = format_dashboard(&branches, &HashMap::new(), &[], &[]);
        assert_eq!(d.entries.len(), 1);
        assert!(d.body.contains("Pending Approval"));
    }

    #[test]
    fn format_dashboard_with_package_files() {
        let mut pf = HashMap::new();
        pf.insert(
            "npm".to_owned(),
            vec![PackageFile {
                package_file: "package.json".to_owned(),
                deps: vec![],
                ..Default::default()
            }],
        );
        let d = format_dashboard(&[], &pf, &[], &[]);
        assert!(d.body.contains("Detected Dependencies"));
        assert!(d.body.contains("package.json"));
    }

    #[test]
    fn format_dashboard_with_errors_and_warnings() {
        let d = format_dashboard(
            &[],
            &HashMap::new(),
            &["error1".to_owned()],
            &["warning1".to_owned()],
        );
        assert!(d.body.contains("### Errors"));
        assert!(d.body.contains("### Warnings"));
    }

    #[test]
    fn dashboard_entry_serialization_roundtrip() {
        let e = DashboardEntry {
            branch_name: "renovate/pkg".into(),
            pr_title: Some("Update pkg".into()),
            state: Some("open".into()),
            result: Some("pr-created".into()),
            is_modified: Some(false),
        };
        let json = serde_json::to_string(&e).unwrap();
        let back: DashboardEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(back.branch_name, "renovate/pkg");
    }
}
