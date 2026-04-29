//! Cloud Native Buildpacks manager (project.toml).

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

    // ── Cloud Native Buildpacks (project.toml) ────────────────────────────────
    for bp_path in manager_files(detected, "buildpacks") {
        match client.get_raw_file(owner, repo, &bp_path).await {
            Ok(Some(raw)) => {
                use renovate_core::extractors::buildpacks::{
                    BuildpacksSkipReason, BuildpacksSource,
                };
                let deps = renovate_core::extractors::buildpacks::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %bp_path,
                    total = deps.len(),
                    "extracted buildpacks deps"
                );
                let mut dep_reports: Vec<output::DepReport> = Vec::new();
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
                            name: dep.dep_name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: match reason {
                                    BuildpacksSkipReason::DockerImage => "docker-image".to_owned(),
                                    BuildpacksSkipReason::NoVersion => "no-version".to_owned(),
                                    BuildpacksSkipReason::UnsupportedUri => {
                                        "unsupported-url".to_owned()
                                    }
                                },
                            },
                        });
                        continue;
                    }
                    if dep.source != BuildpacksSource::Registry {
                        continue;
                    }
                    if repo_cfg.is_dep_ignored_for_manager(&dep.dep_name, "buildpacks") {
                        continue;
                    }
                    let status =
                        match renovate_core::datasources::buildpacks_registry::fetch_latest(
                            http,
                            &dep.dep_name,
                            &dep.current_value,
                        )
                        .await
                        {
                            Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                                current: dep.current_value.clone(),
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
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: bp_path.clone(),
                        manager: "buildpacks".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%bp_path, "project.toml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%bp_path, %err, "failed to fetch project.toml");
                ctx.had_error = true;
            }
        }
    }

}
