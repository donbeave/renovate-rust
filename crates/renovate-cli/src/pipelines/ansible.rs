//! Ansible managers: ansible-galaxy and ansible task files.

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

    // ── Ansible Galaxy (requirements.yml) ────────────────────────────────────
    for ag_path in manager_files(detected, "ansible-galaxy") {
        match client.get_raw_file(owner, repo, &ag_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::ansible_galaxy::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.dep_name, "ansible-galaxy")
                    })
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %ag_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted ansible-galaxy role deps"
                );
                // Only GitHub-URL-sourced roles are actionable right now.
                let gh_inputs: Vec<github_tags_datasource::GithubActionsDepInput> = actionable
                    .iter()
                    .filter_map(|d| {
                        if let renovate_core::extractors::ansible_galaxy::AnsibleGalaxySource::GitHub { owner_repo } = &d.source {
                            Some(github_tags_datasource::GithubActionsDepInput {
                                dep_name: owner_repo.clone(),
                                current_value: d.current_value.clone(),
                            })
                        } else {
                            None
                        }
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
                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    let status = if let Some(reason) = &dep.skip_reason {
                        output::DepStatus::Skipped {
                            reason: format!("{reason:?}").to_lowercase(),
                        }
                    } else if repo_cfg.is_dep_ignored_for_manager(&dep.dep_name, "ansible-galaxy") {
                        output::DepStatus::Skipped {
                            reason: "ignored".to_owned(),
                        }
                    } else {
                        let gh_key = if let renovate_core::extractors::ansible_galaxy::AnsibleGalaxySource::GitHub { owner_repo } = &dep.source {
                            Some(owner_repo.as_str())
                        } else { None };
                        match gh_key.and_then(|k| update_map.get(k)) {
                            Some((true, Some(latest), _)) => output::DepStatus::UpdateAvailable {
                                current: dep.current_value.clone(),
                                latest: latest.clone(),
                            },
                            Some((false, latest, None)) => output::DepStatus::UpToDate {
                                latest: latest.clone(),
                            },
                            Some((_, _, Some(e))) => {
                                output::DepStatus::LookupError { message: e.clone() }
                            }
                            _ => output::DepStatus::UpToDate { latest: None },
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
                        package_name: None,
                        range_strategy: None,
                        follow_tag: None,
                        pin_digests: None,
                        dependency_dashboard_approval: None,
                        replacement_name: None,
                        replacement_version: None,
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                ctx.report.files.push(output::FileReport {
                    path: ag_path.clone(),
                    manager: "ansible-galaxy".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%ag_path, "requirements.yml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%ag_path, %err, "failed to fetch requirements.yml");
                ctx.had_error = true;
            }
        }
    }

    // ── Ansible task files (tasks/*.yml) ─────────────────────────────────────
    for ansible_path in manager_files(detected, "ansible") {
        match client.get_raw_file(owner, repo, &ansible_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::ansible::extract(&raw.content);
                tracing::debug!(repo = %repo_slug, file = %ansible_path, total = deps.len(), "extracted ansible images");
                ctx.report.files.push(output::FileReport {
                    path: ansible_path.clone(),
                    manager: "ansible".into(),
                    deps: docker_hub_reports(http, &deps).await,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%ansible_path, "ansible task file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%ansible_path, %err, "failed to fetch ansible task file");
                ctx.had_error = true;
            }
        }
    }
}
