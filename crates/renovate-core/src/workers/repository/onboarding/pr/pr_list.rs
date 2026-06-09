//! PR list description for onboarding PR body.
//!
//! @parity `lib/workers/repository/onboarding/pr/pr-list.ts` partial — getExpectedPrList / get_pr_list_description (empty case, multiple, lockFileMaintenance special text, baseBranch, schedule TODO, prTitle sanitize TODO, upgrades list with dedup, commitHourlyLimit + prHourlyLimit with emojify + links); single test ported. Full sanitizing, schedule, prTitle for summary, some upgrade fields pending types or callers.
//!
//! Mirrors `lib/workers/repository/onboarding/pr/pr-list.ts`.

use crate::util::emoji::emojify;
use crate::workers::repository::update::branch::types::BranchConfig;
use crate::workers::types::RenovateConfig;

pub fn get_pr_list_description(config: &RenovateConfig, branches: &[BranchConfig]) -> String {
    let mut desc = String::from("\n### What to Expect\n\n");

    if branches.is_empty() {
        desc.push_str(
            "It looks like your repository dependencies are already up-to-date and no Pull Requests will be necessary right away.\n",
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
        // summary uses branch_name (prTitle sanitizing in full TS for @org/repo)
        desc.push_str(&format!(
            "<details>\n<summary>{}</summary>\n\n",
            branch.branch_name
        ));
        // schedule omitted (not on this BranchConfig variant); baseBranch if present
        if !branch.base_branch.is_empty() {
            desc.push_str(&format!("  - Merge into: `{}`\n", branch.base_branch));
        }
        desc.push_str(&format!("  - Branch name: `{}`\n", branch.branch_name));

        // upgrades with special lockFileMaintenance + generic + dedup (ported logic)
        let mut seen: Vec<String> = vec![];
        for upg in &branch.upgrades {
            let u = &upg.upgrade;
            let text = if u.update_type.as_deref() == Some("lockFileMaintenance") {
                "  - Regenerate lock files to use latest dependency versions\n".to_string()
            } else {
                let d = u.dep_name.as_deref().unwrap_or("dep");
                let v = u
                    .new_version
                    .as_deref()
                    .or(u.new_value.as_deref())
                    .unwrap_or("val");
                format!("  - Upgrade {} to `{}`\n", d, v)
            };
            if !seen.contains(&text) {
                desc.push_str(&text);
                seen.push(text);
            }
        }

        desc.push_str("\n\n</details>\n\n");
    }

    // both limits with emojify, matching TS text (commit preferred over pr)
    let commit_limit = config.commit_hourly_limit.unwrap_or(0);
    let pr_limit = config.pr_hourly_limit.unwrap_or(0);
    if commit_limit > 0 && commit_limit < 5 && (commit_limit as usize) < branches.len() {
        desc.push_str(&emojify(&format!(
            "\n\n:children_crossing: Branch creation and rebasing will be limited to maximum {} per hour, so it doesn't swamp any CI resources or overwhelm the project. See docs for `commitHourlyLimit` for details.\n\n",
            commit_limit
        )));
    } else if pr_limit > 0 && pr_limit < 5 && (pr_limit as usize) < branches.len() {
        desc.push_str(&emojify(&format!(
            "\n\n:children_crossing: PR creation will be limited to maximum {} per hour, so it doesn't swamp any CI resources or overwhelm the project. See [docs for `prHourlyLimit`](https://docs.renovatebot.com/configuration-options/#prhourlylimit) for details.\n\n",
            pr_limit
        )));
    }

    desc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_pr_list_description_empty() {
        // Ported: "handles empty" — lib/workers/repository/onboarding/pr/pr-list.spec.ts line 16
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
