//! Typst manager (*.typ files).

use super::*;

pub(crate) async fn process(ctx: &mut RepoPipelineCtx<'_>) {
    let client = ctx.client;
    let http = ctx.http;
    let _ = http;
    let config = ctx.config;
    let _ = config;
    let owner = ctx.owner;
    let repo = ctx.repo;
    let repo_slug = ctx.repo_slug;
    let repo_cfg = ctx.repo_cfg;
    let detected = ctx.detected;

    // ── Typst (*.typ) ────────────────────────────────────────────────────────
    for typ_path in manager_files(detected, "typst") {
        match client.get_raw_file(owner, repo, &typ_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::typst::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %typ_path,
                    total = deps.len(),
                    "extracted typst package deps"
                );
                let mut dep_reports = Vec::new();
                for dep in &deps {
                    if let Some(reason) = &dep.skip_reason {
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
                            dependency_dashboard_approval: None,
                            replacement_name: None,
                            replacement_version: None,
                            name: dep.package_name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored_for_manager(&dep.package_name, "typst") {
                        continue;
                    }
                    let status = match renovate_core::datasources::typst::fetch_latest(
                        http,
                        &dep.package_name,
                        &dep.current_value,
                    )
                    .await
                    {
                        Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                            current: s.current_value,
                            latest: s.latest.unwrap_or_default(),
                        },
                        Ok(s) => output::DepStatus::UpToDate { latest: s.latest },
                        Err(e) => output::DepStatus::LookupError {
                            message: e.to_string(),
                        },
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
                        dependency_dashboard_approval: None,
                        replacement_name: None,
                        replacement_version: None,
                        name: dep.package_name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: typ_path.clone(),
                        manager: "typst".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%typ_path, "typst file not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%typ_path, %err, "failed to fetch typst file");
                ctx.had_error = true;
            }
        }
    }
}
