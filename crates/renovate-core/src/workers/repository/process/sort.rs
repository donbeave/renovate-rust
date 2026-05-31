//! Dependency sorting logic.
//!
//! Mirrors `lib/workers/repository/process/sort.ts`.

use crate::workers::repository::update::branch::types::BranchConfig;

const SORT_ORDER: [&str; 6] = [
    "pin",
    "digest",
    "patch",
    "minor",
    "major",
    "lockFileMaintenance",
];

fn sort_order_index(update_type: &str) -> usize {
    SORT_ORDER
        .iter()
        .position(|&s| s == update_type)
        .unwrap_or(SORT_ORDER.len())
}

fn get_pr_priority(branch: &BranchConfig) -> i32 {
    branch.config.pr_priority.unwrap_or(0)
}

fn is_vulnerability_alert(branch: &BranchConfig) -> bool {
    branch.is_vulnerability_alert.unwrap_or(false)
}

pub fn sort_branches(branches: &mut [BranchConfig]) {
    branches.sort_by(|a, b| {
        let a_vuln = is_vulnerability_alert(a);
        let b_vuln = is_vulnerability_alert(b);
        if a_vuln && !b_vuln {
            return std::cmp::Ordering::Less;
        }
        if !a_vuln && b_vuln {
            return std::cmp::Ordering::Greater;
        }

        let pr_priority_diff = get_pr_priority(b) - get_pr_priority(a);
        if pr_priority_diff != 0 {
            return pr_priority_diff.cmp(&0);
        }

        let a_update_type = a
            .upgrades
            .first()
            .and_then(|u| u.upgrade.update_type.as_deref())
            .unwrap_or("");
        let b_update_type = b
            .upgrades
            .first()
            .and_then(|u| u.upgrade.update_type.as_deref())
            .unwrap_or("");

        let a_sort = sort_order_index(a_update_type);
        let b_sort = sort_order_index(b_update_type);
        if a_sort != b_sort {
            return a_sort.cmp(&b_sort);
        }

        let a_title = a
            .upgrades
            .first()
            .and_then(|u| u.config.commit_message_topic.clone())
            .unwrap_or_default();
        let b_title = b
            .upgrades
            .first()
            .and_then(|u| u.config.commit_message_topic.clone())
            .unwrap_or_default();
        a_title.cmp(&b_title)
    });
}

pub fn sort_upgrades(branches: &mut [BranchConfig]) {
    sort_branches(branches);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workers::types::{BranchUpgrade, RenovateConfig, Upgrade};

    fn make_branch(
        branch_name: &str,
        update_type: &str,
        pr_priority: Option<i32>,
    ) -> BranchConfig {
        BranchConfig {
            branch_name: branch_name.into(),
            base_branch: "main".into(),
            upgrades: vec![BranchUpgrade {
                upgrade: Upgrade {
                    update_type: Some(update_type.into()),
                    ..Default::default()
                },
                ..Default::default()
            }],
            config: RenovateConfig {
                pr_priority,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn make_vuln_branch(branch_name: &str) -> BranchConfig {
        BranchConfig {
            branch_name: branch_name.into(),
            base_branch: "main".into(),
            is_vulnerability_alert: Some(true),
            upgrades: vec![BranchUpgrade {
                upgrade: Upgrade {
                    update_type: Some("major".into()),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        }
    }

    #[test]
    fn sort_branches_empty() {
        let mut branches: Vec<BranchConfig> = vec![];
        sort_branches(&mut branches);
        assert!(branches.is_empty());
    }

    #[test]
    fn sort_branches_by_update_type() {
        let mut branches = vec![
            make_branch("minor", "minor", None),
            make_branch("major", "major", None),
            make_branch("patch", "patch", None),
            make_branch("pin", "pin", None),
            make_branch("digest", "digest", None),
        ];
        sort_branches(&mut branches);
        assert_eq!(branches[0].branch_name, "pin");
        assert_eq!(branches[1].branch_name, "digest");
        assert_eq!(branches[2].branch_name, "patch");
        assert_eq!(branches[3].branch_name, "minor");
        assert_eq!(branches[4].branch_name, "major");
    }

    #[test]
    fn sort_branches_vulnerability_first() {
        let mut branches = vec![
            make_branch("normal", "minor", None),
            make_vuln_branch("vuln"),
            make_branch("other", "patch", None),
        ];
        sort_branches(&mut branches);
        assert_eq!(branches[0].branch_name, "vuln");
    }

    #[test]
    fn sort_branches_by_pr_priority() {
        let mut branches = vec![
            make_branch("low", "minor", Some(1)),
            make_branch("high", "minor", Some(10)),
            make_branch("medium", "minor", Some(5)),
        ];
        sort_branches(&mut branches);
        assert_eq!(branches[0].branch_name, "high");
        assert_eq!(branches[1].branch_name, "medium");
        assert_eq!(branches[2].branch_name, "low");
    }

    #[test]
    fn sort_branches_priority_over_update_type() {
        let mut branches = vec![
            make_branch("low-major", "major", Some(1)),
            make_branch("high-patch", "patch", Some(10)),
        ];
        sort_branches(&mut branches);
        assert_eq!(branches[0].branch_name, "high-patch");
    }

    #[test]
    fn sort_order_index_known() {
        assert_eq!(sort_order_index("pin"), 0);
        assert_eq!(sort_order_index("digest"), 1);
        assert_eq!(sort_order_index("patch"), 2);
        assert_eq!(sort_order_index("minor"), 3);
        assert_eq!(sort_order_index("major"), 4);
        assert_eq!(sort_order_index("lockFileMaintenance"), 5);
    }

    #[test]
    fn sort_order_index_unknown() {
        assert_eq!(sort_order_index("unknown"), SORT_ORDER.len());
    }

    #[test]
    fn get_pr_priority_default() {
        let branch = BranchConfig::default();
        assert_eq!(get_pr_priority(&branch), 0);
    }

    #[test]
    fn get_pr_priority_custom() {
        let mut branch = BranchConfig::default();
        branch.config.pr_priority = Some(42);
        assert_eq!(get_pr_priority(&branch), 42);
    }

    #[test]
    fn sort_upgrades_alias() {
        let mut branches = vec![
            make_branch("minor", "minor", None),
            make_branch("major", "major", None),
        ];
        sort_upgrades(&mut branches);
        assert_eq!(branches[0].branch_name, "minor");
        assert_eq!(branches[1].branch_name, "major");
    }

    #[test]
    fn sort_branches_single() {
        let mut branches = vec![make_branch("only", "minor", None)];
        sort_branches(&mut branches);
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0].branch_name, "only");
    }
}
