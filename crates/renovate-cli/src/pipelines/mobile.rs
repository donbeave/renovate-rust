//! Mobile/cross-platform managers: Swift Package Manager, XcodeGen, Mint, Mix (Elixir), Gleam.

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

    // ── Swift Package Manager (Package.swift) ─────────────────────────────────
    for spm_file_path in manager_files(detected, "swift") {
        match client.get_raw_file(owner, repo, &spm_file_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::spm::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| d.skip_reason.is_none() && !d.owner_repo.is_empty())
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %spm_file_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted swift package deps"
                );
                // GitHub packages → github_tags datasource.
                let gh_inputs: Vec<github_tags_datasource::GithubActionsDepInput> = actionable
                    .iter()
                    .filter(|d| {
                        matches!(
                            d.git_host,
                            Some(renovate_core::extractors::spm::GitHost::GitHub)
                        )
                    })
                    .map(|d| github_tags_datasource::GithubActionsDepInput {
                        dep_name: d.owner_repo.clone(),
                        current_value: d.current_value.clone(),
                    })
                    .collect();
                let gh_updates = github_tags_datasource::fetch_updates_concurrent(
                    http,
                    &gh_inputs,
                    github_tags_datasource::GITHUB_API,
                    8,
                )
                .await;
                // GitLab packages → gitlab_tags datasource.
                let gl_inputs: Vec<renovate_core::datasources::gitlab_tags::GitlabTagsDepInput> =
                    actionable
                        .iter()
                        .filter(|d| {
                            matches!(
                                d.git_host,
                                Some(renovate_core::extractors::spm::GitHost::GitLab)
                            )
                        })
                        .map(
                            |d| renovate_core::datasources::gitlab_tags::GitlabTagsDepInput {
                                dep_name: d.owner_repo.clone(),
                                current_value: d.current_value.clone(),
                            },
                        )
                        .collect();
                let gl_updates = renovate_core::datasources::gitlab_tags::fetch_updates_concurrent(
                    http,
                    &gl_inputs,
                    renovate_core::datasources::gitlab_tags::GITLAB_API,
                    8,
                )
                .await;
                // Unified map: dep_name → (update_available, latest, error_msg).
                let mut spm_map: HashMap<String, (bool, Option<String>, Option<String>)> =
                    HashMap::new();
                for r in gh_updates {
                    match r.summary {
                        Ok(s) => {
                            spm_map.insert(r.dep_name, (s.update_available, s.latest, None));
                        }
                        Err(e) => {
                            spm_map.insert(r.dep_name, (false, None, Some(e.to_string())));
                        }
                    }
                }
                for r in gl_updates {
                    match r.summary {
                        Ok(s) => {
                            spm_map.insert(r.dep_name, (s.update_available, s.latest, None));
                        }
                        Err(e) => {
                            spm_map.insert(r.dep_name, (false, None, Some(e.to_string())));
                        }
                    }
                }
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
                        name: dep.owner_repo.clone(),
                        status: output::DepStatus::Skipped {
                            reason: format!("{:?}", dep.skip_reason.as_ref().unwrap())
                                .to_lowercase(),
                        },
                    });
                }
                for dep in &actionable {
                    let status = match spm_map.get(&dep.owner_repo) {
                        Some((true, Some(latest), _)) => output::DepStatus::UpdateAvailable {
                            current: dep.current_value.clone(),
                            latest: latest.clone(),
                        },
                        Some((false, latest, None)) => output::DepStatus::UpToDate {
                            latest: latest.clone(),
                        },
                        Some((_, _, Some(err_msg))) => output::DepStatus::LookupError {
                            message: err_msg.clone(),
                        },
                        _ => output::DepStatus::Skipped {
                            reason: "lookup-pending".into(),
                        },
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
                        name: dep.owner_repo.clone(),
                        status,
                    });
                }
                ctx.report.files.push(output::FileReport {
                    path: spm_file_path.clone(),
                    manager: "swift".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%spm_file_path, "Package.swift not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%spm_file_path, %err, "failed to fetch Package.swift");
                ctx.had_error = true;
            }
        }
    }

    // ── Mint (Mintfile) ───────────────────────────────────────────────────────
    for mint_path in manager_files(detected, "mint") {
        match client.get_raw_file(owner, repo, &mint_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::mint::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %mint_path,
                    total = deps.len(), "extracted mint deps"
                );
                let gh_inputs: Vec<github_tags_datasource::GithubActionsDepInput> = deps
                    .iter()
                    .map(|d| github_tags_datasource::GithubActionsDepInput {
                        dep_name: d.repo.clone(),
                        current_value: d.version.clone(),
                    })
                    .collect();
                let updates = github_tags_datasource::fetch_updates_concurrent(
                    &gh_http,
                    &gh_inputs,
                    gh_api_base,
                    8,
                )
                .await;
                let update_map: HashMap<_, _> = updates
                    .into_iter()
                    .map(|r| (r.dep_name, r.summary))
                    .collect();
                let file_deps: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        let status = match update_map.get(&dep.repo) {
                            Some(Ok(s)) if s.update_available => {
                                output::DepStatus::UpdateAvailable {
                                    current: dep.version.clone(),
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
                        };
                        output::DepReport {
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
                            name: dep.repo.clone(),
                            status,
                        }
                    })
                    .collect();
                ctx.report.files.push(output::FileReport {
                    path: mint_path.clone(),
                    manager: "mint".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%mint_path, "Mintfile not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%mint_path, %err, "failed to fetch Mintfile");
                ctx.had_error = true;
            }
        }
    }

    // ── XcodeGen (project.yml) ────────────────────────────────────────────────
    for xg_path in manager_files(detected, "xcodegen") {
        match client.get_raw_file(owner, repo, &xg_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::xcodegen::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %xg_path,
                    total = deps.len(),
                    "extracted xcodegen swift package deps"
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
                            name: dep.name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored_for_manager(&dep.name, "xcodegen") {
                        continue;
                    }
                    let gh_repo = match &dep.source {
                        Some(renovate_core::extractors::xcodegen::XcodeGenSource::GitHub(r)) => {
                            r.as_str()
                        }
                        _ => {
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
                                name: dep.name.clone(),
                                status: output::DepStatus::Skipped {
                                    reason: "non-github-source".into(),
                                },
                            });
                            continue;
                        }
                    };
                    let tag_result = renovate_core::datasources::github_tags::fetch_latest_tag(
                        gh_repo,
                        &gh_http,
                        gh_api_base,
                    )
                    .await
                    .map_err(|e| e.to_string());
                    let status = match tag_result {
                        Ok(Some(tag)) => {
                            let stripped = tag.trim_start_matches('v');
                            let clean_current = dep.current_value.trim_start_matches('v');
                            let s =
                                renovate_core::versioning::semver_generic::semver_update_summary(
                                    clean_current,
                                    Some(stripped),
                                );
                            if s.update_available {
                                output::DepStatus::UpdateAvailable {
                                    current: dep.current_value.clone(),
                                    latest: stripped.to_owned(),
                                }
                            } else {
                                output::DepStatus::UpToDate {
                                    latest: Some(stripped.to_owned()),
                                }
                            }
                        }
                        Ok(None) => output::DepStatus::UpToDate { latest: None },
                        Err(e) => output::DepStatus::LookupError { message: e },
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
                        name: dep.name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: xg_path.clone(),
                        manager: "xcodegen".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%xg_path, "project.yml not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%xg_path, %err, "failed to fetch project.yml");
                ctx.had_error = true;
            }
        }
    }

    // ── Mix (mix.exs) ─────────────────────────────────────────────────────────
    for mix_file_path in manager_files(detected, "mix") {
        match client.get_raw_file(owner, repo, &mix_file_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::mix::extract(&raw.content);
                let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                tracing::debug!(
                    repo = %repo_slug, file = %mix_file_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted mix deps"
                );
                let dep_inputs: Vec<renovate_core::datasources::hex::HexDepInput> = actionable
                    .iter()
                    .map(|d| renovate_core::datasources::hex::HexDepInput {
                        name: d.name.clone(),
                        current_value: d.current_value.clone(),
                    })
                    .collect();
                let updates = renovate_core::datasources::hex::fetch_updates_concurrent(
                    http,
                    &dep_inputs,
                    renovate_core::datasources::hex::HEX_API,
                    10,
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
                        name: dep.name.clone(),
                        status,
                    });
                }
                ctx.report.files.push(output::FileReport {
                    path: mix_file_path.clone(),
                    manager: "mix".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%mix_file_path, "mix.exs not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%mix_file_path, %err, "failed to fetch mix.exs");
                ctx.had_error = true;
            }
        }
    }

    // ── Gleam (gleam.toml) ────────────────────────────────────────────────────
    for gleam_path in manager_files(detected, "gleam") {
        match client.get_raw_file(owner, repo, &gleam_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::gleam::extract(&raw.content);
                let actionable: Vec<_> = deps.iter().collect();
                tracing::debug!(
                    repo = %repo_slug, file = %gleam_path,
                    total = deps.len(), "extracted gleam deps"
                );
                let dep_inputs: Vec<renovate_core::datasources::hex::HexDepInput> = actionable
                    .iter()
                    .map(|d| renovate_core::datasources::hex::HexDepInput {
                        name: d.name.clone(),
                        current_value: d.version.clone(),
                    })
                    .collect();
                let updates = renovate_core::datasources::hex::fetch_updates_concurrent(
                    http,
                    &dep_inputs,
                    renovate_core::datasources::hex::HEX_API,
                    8,
                )
                .await;
                let update_map: HashMap<_, _> =
                    updates.into_iter().map(|r| (r.name, r.summary)).collect();
                let file_deps: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        let status = match update_map.get(&dep.name) {
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
                        };
                        output::DepReport {
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
                            name: dep.name.clone(),
                            status,
                        }
                    })
                    .collect();
                ctx.report.files.push(output::FileReport {
                    path: gleam_path.clone(),
                    manager: "gleam".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%gleam_path, "gleam.toml not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%gleam_path, %err, "failed to fetch gleam.toml");
                ctx.had_error = true;
            }
        }
    }
}
