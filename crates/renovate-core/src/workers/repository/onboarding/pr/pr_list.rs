//! PR list description for onboarding PR body.
//!
//! Mirrors `lib/workers/repository/onboarding/pr/pr-list.ts`.

use crate::workers::repository::update::branch::types::BranchConfig;
use crate::workers::types::RenovateConfig;

pub fn get_pr_list_description(config: &RenovateConfig, branches: &[BranchConfig]) -> String {
    let mut desc = String::from("\n### What to Expect\n\n");

    if branches.is_empty() {
        desc.push_str(
            "It looks like your repository dependencies are already up-to-date \
             and no Pull Requests will be necessary right away.\n",
        );
        return desc;
    }

    desc.push_str(&format!(
        "With your current configuration, Renovate will create {} Pull Request",
        branches.len()
    ));
    desc.push_str(if branches.len() > 1 {
        "s:\n\n"
    } else {
        ":\n\n"
    });

    for branch in branches {
        desc.push_str(&format!(
            "<details>\n<summary>{}</summary>\n\n",
            branch.branch_name
        ));
        desc.push_str(&format!("  - Branch name: `{}`\n", branch.branch_name));
        desc.push_str("\n\n</details>\n\n");
    }

    let hourly_limit = config.pr_hourly_limit.unwrap_or(0);
    if hourly_limit > 0 && hourly_limit < 5 && hourly_limit < branches.len() as i64 {
        desc.push_str(&format!(
            "\nPR creation will be limited to maximum {} per hour, \
             so it doesn't swamp any CI resources or overwhelm the project.\n\n",
            hourly_limit
        ));
    }

    desc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_pr_list_description_empty() {
        let config = RenovateConfig::default();
        let desc = get_pr_list_description(&config, &[]);
        assert!(desc.contains("already up-to-date"));
    }

    #[test]
    fn get_pr_list_description_single() {
        let config = RenovateConfig::default();
        let branches = vec![BranchConfig {
            branch_name: "renovate/lodash-4.x".into(),
            ..Default::default()
        }];
        let desc = get_pr_list_description(&config, &branches);
        assert!(desc.contains("1 Pull Request:"));
        assert!(desc.contains("renovate/lodash-4.x"));
    }

    #[test]
    fn get_pr_list_description_multiple() {
        let config = RenovateConfig::default();
        let branches = vec![
            BranchConfig {
                branch_name: "renovate/lodash-4.x".into(),
                ..Default::default()
            },
            BranchConfig {
                branch_name: "renovate/express-5.x".into(),
                ..Default::default()
            },
        ];
        let desc = get_pr_list_description(&config, &branches);
        assert!(desc.contains("2 Pull Requests:"));
    }

    #[test]
    fn get_pr_list_description_hourly_limit() {
        let config = RenovateConfig {
            pr_hourly_limit: Some(2),
            ..Default::default()
        };
        let branches: Vec<BranchConfig> = (0..5)
            .map(|i| BranchConfig {
                branch_name: format!("renovate/pkg-{i}"),
                ..Default::default()
            })
            .collect();
        let desc = get_pr_list_description(&config, &branches);
        assert!(desc.contains("maximum 2 per hour"));
    }

    #[test]
    fn get_pr_list_description_no_hourly_limit() {
        let config = RenovateConfig {
            pr_hourly_limit: Some(0),
            ..Default::default()
        };
        let branches = vec![BranchConfig {
            branch_name: "renovate/pkg".into(),
            ..Default::default()
        }];
        let desc = get_pr_list_description(&config, &branches);
        assert!(!desc.contains("maximum"));
    }
}
