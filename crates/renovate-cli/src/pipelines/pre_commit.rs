//! pre-commit manager (.pre-commit-config.yaml).

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

    // ── pre-commit (.pre-commit-config.yaml) ──────────────────────────────────
    for pc_path in manager_files(detected, "pre-commit") {
        match client.get_raw_file(owner, repo, &pc_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::pre_commit::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.dep_name, "pre-commit")
                    })
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %pc_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted pre-commit hook deps"
                );

                // Partition actionable by host.
                let gh_actionable: Vec<_> = actionable
                    .iter()
                    .filter(|d| {
                        d.git_host == Some(renovate_core::extractors::pre_commit::GitHost::GitHub)
                    })
                    .collect();
                let gl_actionable: Vec<_> = actionable
                    .iter()
                    .filter(|d| {
                        d.git_host == Some(renovate_core::extractors::pre_commit::GitHost::GitLab)
                    })
                    .collect();

                let gh_inputs: Vec<github_tags_datasource::GithubActionsDepInput> = gh_actionable
                    .iter()
                    .map(|d| github_tags_datasource::GithubActionsDepInput {
                        dep_name: d.dep_name.clone(),
                        current_value: d.current_value.trim_matches('\'').to_owned(),
                    })
                    .collect();
                let gh_updates = github_tags_datasource::fetch_updates_concurrent(
                    &gh_http,
                    &gh_inputs,
                    gh_api_base,
                    8,
                )
                .await;
                let mut update_map: HashMap<String, (bool, Option<String>, Option<String>)> =
                    HashMap::new();
                for r in gh_updates {
                    match r.summary {
                        Ok(s) => {
                            update_map.insert(r.dep_name, (s.update_available, s.latest, None));
                        }
                        Err(e) => {
                            update_map.insert(r.dep_name, (false, None, Some(e.to_string())));
                        }
                    }
                }

                let gl_inputs: Vec<renovate_core::datasources::gitlab_tags::GitlabTagsDepInput> =
                    gl_actionable
                        .iter()
                        .map(
                            |d| renovate_core::datasources::gitlab_tags::GitlabTagsDepInput {
                                dep_name: d.dep_name.clone(),
                                current_value: d.current_value.trim_matches('\'').to_owned(),
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
                for r in gl_updates {
                    match r.summary {
                        Ok(s) => {
                            update_map.insert(r.dep_name, (s.update_available, s.latest, None));
                        }
                        Err(e) => {
                            update_map.insert(r.dep_name, (false, None, Some(e.to_string())));
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

                        dep_type: None,
                        name: dep.dep_name.clone(),
                        status: output::DepStatus::Skipped {
                            reason: format!("{:?}", dep.skip_reason.as_ref().unwrap())
                                .to_lowercase(),
                        },
                    });
                }
                for dep in &actionable {
                    let status = match update_map.get(&dep.dep_name) {
                        Some((true, Some(latest), _)) => output::DepStatus::UpdateAvailable {
                            current: dep.current_value.trim_matches('\'').to_owned(),
                            latest: latest.clone(),
                        },
                        Some((false, latest, None)) => output::DepStatus::UpToDate {
                            latest: latest.clone(),
                        },
                        Some((_, _, Some(err_msg))) => output::DepStatus::LookupError {
                            message: err_msg.clone(),
                        },
                        _ => output::DepStatus::UpToDate { latest: None },
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
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                ctx.report.files.push(output::FileReport {
                    path: pc_path.clone(),
                    manager: "pre-commit".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%pc_path, ".pre-commit-config.yaml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%pc_path, %err, "failed to fetch .pre-commit-config.yaml");
                ctx.had_error = true;
            }
        }
    }



}
