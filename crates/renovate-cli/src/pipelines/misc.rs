//! Miscellaneous managers: Renovate config extends presets, Hermit.
//! Unity3D, Cloud Native Buildpacks, Heroku/Render, Renovate config presets, Hermit.
//! Vendir, Copier, Batect, Jenkins, OCB, Homebrew, Unity3D, CNB, Heroku, Renovate presets, Hermit.
//! Vendir, Copier, Batect, Git submodules, Puppet, Jenkins, OCB, Homebrew,
//! Unity3D, Cloud Native Buildpacks, Heroku/Render, Renovate config presets, Hermit.

use super::*;

pub(crate) async fn process(ctx: &mut RepoPipelineCtx<'_>) {
    let client = ctx.client;
    let http = ctx.http;
    let config = ctx.config;
    let owner = ctx.owner;
    let repo = ctx.repo;
    let repo_slug = ctx.repo_slug;
    let repo_cfg = ctx.repo_cfg;
    let detected = ctx.detected;
    let gh_api_base = github_tags_datasource::api_base_from_endpoint(config.endpoint.as_deref());
    let gh_http = if let Some(ref token) = config.token {
        HttpClient::with_token(token).unwrap_or_else(|_| http.clone())
    } else {
        http.clone()
    };
    let filtered_files = ctx.filtered_files;

    // ── Renovate config extends presets ───────────────────────────────────────
    for rc_path in manager_files(detected, "renovate-config-presets") {
        match client.get_raw_file(owner, repo, &rc_path).await {
            Ok(Some(raw)) => {
                use renovate_core::extractors::renovate_config_presets::{
                    PresetSkipReason, PresetSource,
                };
                let deps =
                    renovate_core::extractors::renovate_config_presets::extract(&raw.content);
                let actionable_count = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg
                                .is_dep_ignored_for_manager(&d.repo, "renovate-config-presets")
                    })
                    .count();
                tracing::debug!(
                    repo = %repo_slug, file = %rc_path,
                    total = deps.len(), actionable = actionable_count,
                    "extracted renovate config preset deps"
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
                            dependency_dashboard_approval: None,
                            replacement_name: None,
                            replacement_version: None,
                            name: dep.repo.clone(),
                            status: output::DepStatus::Skipped {
                                reason: match reason {
                                    PresetSkipReason::UnspecifiedVersion => {
                                        "unspecified-version".to_owned()
                                    }
                                    PresetSkipReason::UnsupportedDatasource => {
                                        "unsupported-datasource".to_owned()
                                    }
                                },
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored_for_manager(&dep.repo, "renovate-config-presets") {
                        continue;
                    }
                    let status = match &dep.source {
                        PresetSource::GitHub => {
                            match github_tags_datasource::fetch_latest_tag(
                                &dep.repo,
                                &gh_http,
                                gh_api_base,
                            )
                            .await
                            {
                                Ok(Some(tag)) => {
                                    let stripped = tag.trim_start_matches('v');
                                    let s = renovate_core::versioning::semver_generic::semver_update_summary(
                                        dep.current_value.trim_start_matches('v'),
                                        Some(stripped),
                                    );
                                    if s.update_available {
                                        output::DepStatus::UpdateAvailable {
                                            current: dep.current_value.clone(),
                                            latest: tag,
                                        }
                                    } else {
                                        output::DepStatus::UpToDate { latest: Some(tag) }
                                    }
                                }
                                Ok(None) => output::DepStatus::UpToDate { latest: None },
                                Err(e) => output::DepStatus::LookupError {
                                    message: e.to_string(),
                                },
                            }
                        }
                        PresetSource::GitLab => {
                            match renovate_core::datasources::gitlab_tags::fetch_latest_tag(
                                &dep.repo,
                                http,
                                renovate_core::datasources::gitlab_tags::GITLAB_API,
                            )
                            .await
                            {
                                Ok(Some(tag)) => {
                                    let stripped = tag.trim_start_matches('v');
                                    let s = renovate_core::versioning::semver_generic::semver_update_summary(
                                        dep.current_value.trim_start_matches('v'),
                                        Some(stripped),
                                    );
                                    if s.update_available {
                                        output::DepStatus::UpdateAvailable {
                                            current: dep.current_value.clone(),
                                            latest: tag,
                                        }
                                    } else {
                                        output::DepStatus::UpToDate { latest: Some(tag) }
                                    }
                                }
                                Ok(None) => output::DepStatus::UpToDate { latest: None },
                                Err(e) => output::DepStatus::LookupError {
                                    message: e.to_string(),
                                },
                            }
                        }
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
                        name: dep.repo.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: rc_path.clone(),
                        manager: "renovate-config-presets".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::debug!(repo=%repo_slug, file=%rc_path, "renovate config not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%rc_path, %err, "failed to fetch renovate config");
                ctx.had_error = true;
            }
        }
    }

    // ── Hermit (bin/.*.pkg filenames) ────────────────────────────────────────
    // Hermit encodes package name+version in hidden `.*.pkg` filenames inside
    // `bin/`.  We skip fetching file content and parse the path list directly.
    if !manager_files(detected, "hermit").is_empty() {
        let deps = renovate_core::extractors::hermit::extract_from_file_list(filtered_files);
        let actionable_count = deps.iter().filter(|d| d.skip_reason.is_none()).count();
        tracing::debug!(
            repo = %repo_slug,
            total = deps.len(), actionable = actionable_count,
            "extracted hermit package deps"
        );
        let mut dep_reports: Vec<output::DepReport> = Vec::new();
        for dep in &deps {
            if let Some(ref reason) = dep.skip_reason {
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
                    name: dep.name.clone(),
                    status: output::DepStatus::Skipped {
                        reason: format!("{reason:?}").to_lowercase(),
                    },
                });
                continue;
            }
            if repo_cfg.is_dep_ignored_for_manager(&dep.name, "hermit") {
                continue;
            }
            let status = match renovate_core::datasources::hermit::fetch_latest(
                &dep.name,
                &dep.current_value,
                renovate_core::datasources::hermit::DEFAULT_REGISTRY,
                http,
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
                dependency_dashboard_approval: None,
                replacement_name: None,
                replacement_version: None,
                name: dep.name.clone(),
                status,
            });
        }
        if !dep_reports.is_empty() {
            ctx.report.files.push(output::FileReport {
                path: "bin/".to_owned(),
                manager: "hermit".into(),
                deps: dep_reports,
            });
        }
    }
}
