//! Git submodules manager (.gitmodules).

use super::*;

pub(crate) async fn process(ctx: &mut RepoPipelineCtx<'_>) {
    let client = ctx.client;
    let _ = ctx.http;
    let config = ctx.config;
    let _ = config;
    let owner = ctx.owner;
    let repo = ctx.repo;
    let repo_slug = ctx.repo_slug;
    let repo_cfg = ctx.repo_cfg;
    let detected = ctx.detected;

    // ── Git submodules (.gitmodules) ─────────────────────────────────────────
    // Note: git-submodules is disabled by default in Renovate (defaultConfig.enabled = false).
    // We still process it here; users enable it via enabledManagers or packageRules.
    // currentDigest (submodule commit SHA) is not extracted — requires the Git trees API.
    for gm_path in manager_files(detected, "git-submodules") {
        match client.get_raw_file(owner, repo, &gm_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::git_submodules::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %gm_path,
                    total = deps.len(),
                    "extracted git submodule deps"
                );
                let mut dep_reports = Vec::new();
                for dep in &deps {
                    if repo_cfg.is_dep_ignored_for_manager(&dep.name, "git-submodules") {
                        dep_reports.push(output::DepReport {
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            update_type: None,
                            pr_priority: None,
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,

                            dep_type: None,
                            package_name: None,
                            range_strategy: None,
                            follow_tag: None,
                            pin_digests: None,
                            dependency_dashboard_approval: None,
                            replacement_name: None,
                            replacement_version: None,
                            name: dep.name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: "ignored".into(),
                            },
                        });
                        continue;
                    }
                    // Without currentDigest we cannot determine if an update is available.
                    // Report the dep with its branch as context; future slice adds digest lookup.
                    let status = if let Some(branch) = &dep.branch {
                        output::DepStatus::UpToDate {
                            latest: Some(branch.clone()),
                        }
                    } else {
                        output::DepStatus::UpToDate { latest: None }
                    };
                    dep_reports.push(output::DepReport {
                        branch_name: None,
                        group_name: None,
                        automerge: None,
                        labels: Vec::new(),
                        assignees: Vec::new(),
                        reviewers: Vec::new(),
                        update_type: None,
                        pr_priority: None,
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,

                        dep_type: None,
                        package_name: None,
                        range_strategy: None,
                        follow_tag: None,
                        pin_digests: None,
                        dependency_dashboard_approval: None,
                        replacement_name: None,
                        replacement_version: None,
                        name: dep.path.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: gm_path.clone(),
                        manager: "git-submodules".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%gm_path, ".gitmodules not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%gm_path, %err, "failed to fetch .gitmodules");
                ctx.had_error = true;
            }
        }
    }
}
