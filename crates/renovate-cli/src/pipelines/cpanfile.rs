//! cpanfile Perl package manager.

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

    // ── cpanfile (Perl) ──────────────────────────────────────────────────────
    for cpan_path in manager_files(detected, "cpanfile") {
        match client.get_raw_file(owner, repo, &cpan_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::cpanfile::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %cpan_path,
                    total = deps.len(),
                    "extracted cpanfile perl deps"
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
                            name: dep.dep_name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored_for_manager(&dep.dep_name, "cpanfile") {
                        continue;
                    }
                    let status = match renovate_core::datasources::cpan::fetch_latest(
                        http,
                        &dep.dep_name,
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
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: cpan_path.clone(),
                        manager: "cpanfile".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%cpan_path, "cpanfile not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%cpan_path, %err, "failed to fetch cpanfile");
                ctx.had_error = true;
            }
        }
    }


    for jb_path in manager_files(detected, "jsonnet-bundler") {
        match client.get_raw_file(owner, repo, &jb_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::jsonnet_bundler::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        !d.github_repo.is_empty()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.remote, "jsonnet-bundler")
                    })
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %jb_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted jsonnet-bundler deps"
                );
                let gh_inputs: Vec<github_tags_datasource::GithubActionsDepInput> = actionable
                    .iter()
                    .map(|d| github_tags_datasource::GithubActionsDepInput {
                        dep_name: d.github_repo.clone(),
                        current_value: d.version.clone(),
                    })
                    .collect();
                let gh_updates = github_tags_datasource::fetch_updates_concurrent(
                    &gh_http,
                    &gh_inputs,
                    gh_api_base,
                    8,
                )
                .await;
                let update_map: HashMap<String, (bool, Option<String>, Option<String>)> = {
                    let mut m = HashMap::new();
                    for r in gh_updates {
                        match r.summary {
                            Ok(s) => {
                                m.insert(r.dep_name, (s.update_available, s.latest, None));
                            }
                            Err(e) => {
                                m.insert(r.dep_name, (false, None, Some(e.to_string())));
                            }
                        }
                    }
                    m
                };
                let file_deps: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        let status = if dep.github_repo.is_empty() {
                            output::DepStatus::Skipped {
                                reason: "non-github remote".into(),
                            }
                        } else {
                            match update_map.get(&dep.github_repo) {
                                Some((true, Some(latest), _)) => {
                                    output::DepStatus::UpdateAvailable {
                                        current: dep.version.clone(),
                                        latest: latest.clone(),
                                    }
                                }
                                Some((_, latest, None)) => output::DepStatus::UpToDate {
                                    latest: latest.clone(),
                                },
                                Some((_, _, Some(err))) => output::DepStatus::LookupError {
                                    message: err.clone(),
                                },
                                None => output::DepStatus::UpToDate { latest: None },
                            }
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

                            dep_type: None,
                             package_name: None,
                            name: dep.remote.clone(),
                            status,
                        }
                    })
                    .collect();
                ctx.report.files.push(output::FileReport {
                    path: jb_path.clone(),
                    manager: "jsonnet-bundler".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%jb_path, "jsonnetfile.json not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%jb_path, %err, "failed to fetch jsonnetfile.json");
                ctx.had_error = true;
            }
        }
    }

}
