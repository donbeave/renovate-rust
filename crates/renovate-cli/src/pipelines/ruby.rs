//! Ruby ecosystem managers: Bundler (Gemfile), gemspec, CocoaPods.

use super::*;

pub(crate) async fn process(ctx: &mut RepoPipelineCtx<'_>) {
    let client = ctx.client;
    let http = ctx.http;
    let config = ctx.config;
    let _ = config;
    let owner = ctx.owner;
    let repo = ctx.repo;
    let repo_slug = ctx.repo_slug;
    let repo_cfg = ctx.repo_cfg;
    let detected = ctx.detected;

    // ── Bundler (Gemfile) ─────────────────────────────────────────────────────
    for gemfile_path in manager_files(detected, "bundler") {
        match client.get_raw_file(owner, repo, &gemfile_path).await {
            Ok(Some(raw)) => {
                let deps = bundler_extractor::extract(&raw.content);
                let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                tracing::debug!(
                    repo = %repo_slug, file = %gemfile_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted bundler gems"
                );
                let dep_inputs: Vec<rubygems_datasource::GemDepInput> = actionable
                    .iter()
                    .map(|d| rubygems_datasource::GemDepInput {
                        name: d.name.clone(),
                        current_value: d.current_value.clone(),
                    })
                    .collect();
                let updates = rubygems_datasource::fetch_updates_concurrent(
                    http,
                    &dep_inputs,
                    rubygems_datasource::RUBYGEMS_API,
                    10,
                )
                .await;
                let update_map: HashMap<_, _> =
                    updates.into_iter().map(|r| (r.name, r.summary)).collect();
                ctx.report.files.push(output::FileReport {
                    path: gemfile_path.clone(),
                    manager: "bundler".into(),
                    deps: build_dep_reports_bundler(&deps, &actionable, &update_map),
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%gemfile_path, "Gemfile not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%gemfile_path, %err, "failed to fetch Gemfile");
                ctx.had_error = true;
            }
        }
    }

    // ── gemspec (.gemspec) ────────────────────────────────────────────────────
    for gemspec_path in manager_files(detected, "gemspec") {
        match client.get_raw_file(owner, repo, &gemspec_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::gemspec::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !d.current_value.is_empty()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.name, "gemspec")
                    })
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %gemspec_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted gemspec deps"
                );
                let dep_inputs: Vec<rubygems_datasource::GemDepInput> = actionable
                    .iter()
                    .map(|d| rubygems_datasource::GemDepInput {
                        name: d.name.clone(),
                        current_value: d.current_value.clone(),
                    })
                    .collect();
                let updates = rubygems_datasource::fetch_updates_concurrent(
                    http,
                    &dep_inputs,
                    rubygems_datasource::RUBYGEMS_API,
                    10,
                )
                .await;
                let update_map: HashMap<_, _> =
                    updates.into_iter().map(|r| (r.name, r.summary)).collect();
                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    let status = if let Some(reason) = &dep.skip_reason {
                        output::DepStatus::Skipped {
                            reason: format!("{reason:?}").to_lowercase(),
                        }
                    } else if dep.current_value.is_empty() {
                        output::DepStatus::Skipped {
                            reason: "no-version".to_owned(),
                        }
                    } else if repo_cfg.is_dep_ignored_for_manager(&dep.name, "gemspec") {
                        output::DepStatus::Skipped {
                            reason: "ignored".to_owned(),
                        }
                    } else {
                        match update_map.get(&dep.name) {
                            Some(Ok(s)) if s.update_available => {
                                output::DepStatus::UpdateAvailable {
                                    current: s.current_value.clone(),
                                    latest: s.latest.clone().unwrap_or_default(),
                                }
                            }
                            Some(Ok(s)) => output::DepStatus::UpToDate {
                                latest: s.latest.clone(),
                            },
                            Some(Err(e)) => output::DepStatus::LookupError {
                                message: e.to_string(),
                            },
                            None => output::DepStatus::UpToDate { latest: None },
                        }
                    };
                    file_deps.push(output::DepReport {
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
                        name: dep.name.clone(),
                        status,
                    });
                }
                ctx.report.files.push(output::FileReport {
                    path: gemspec_path.clone(),
                    manager: "gemspec".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%gemspec_path, ".gemspec not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%gemspec_path, %err, "failed to fetch .gemspec");
                ctx.had_error = true;
            }
        }
    }

    // ── CocoaPods (Podfile) ───────────────────────────────────────────────────
    for podfile_path in manager_files(detected, "cocoapods") {
        match client.get_raw_file(owner, repo, &podfile_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::cocoapods::extract(&raw.content);
                let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                tracing::debug!(
                    repo = %repo_slug, file = %podfile_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted cocoapods deps"
                );
                let dep_inputs: Vec<renovate_core::datasources::cocoapods::PodDepInput> =
                    actionable
                        .iter()
                        .map(|d| renovate_core::datasources::cocoapods::PodDepInput {
                            name: d.name.clone(),
                            current_value: d.current_value.clone(),
                        })
                        .collect();
                let updates = renovate_core::datasources::cocoapods::fetch_updates_concurrent(
                    http,
                    &dep_inputs,
                    renovate_core::datasources::cocoapods::TRUNK_API,
                    8,
                )
                .await;
                let update_map: HashMap<_, _> =
                    updates.into_iter().map(|r| (r.name, r.summary)).collect();
                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in deps.iter().filter(|d| d.skip_reason.is_some()) {
                    file_deps.push(output::DepReport {
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
                        name: dep.name.clone(),
                        status: output::DepStatus::Skipped {
                            reason: format!("{:?}", dep.skip_reason.as_ref().unwrap())
                                .to_lowercase(),
                        },
                    });
                }
                for dep in &actionable {
                    let status = match update_map.get(&dep.name) {
                        Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                            current: s.current_value.clone(),
                            latest: s.latest.clone().unwrap_or_default(),
                        },
                        Some(Ok(s)) => output::DepStatus::UpToDate {
                            latest: s.latest.clone(),
                        },
                        Some(Err(e)) => output::DepStatus::LookupError {
                            message: e.to_string(),
                        },
                        None => output::DepStatus::UpToDate { latest: None },
                    };
                    file_deps.push(output::DepReport {
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
                        name: dep.name.clone(),
                        status,
                    });
                }
                ctx.report.files.push(output::FileReport {
                    path: podfile_path.clone(),
                    manager: "cocoapods".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%podfile_path, "Podfile not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%podfile_path, %err, "failed to fetch Podfile");
                ctx.had_error = true;
            }
        }
    }
}
