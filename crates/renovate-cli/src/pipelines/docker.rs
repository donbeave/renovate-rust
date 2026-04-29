//! Docker ecosystem managers: Dockerfile, Docker Compose, Dev Container, Quadlet.

use super::*;

pub(crate) async fn process(ctx: &mut RepoPipelineCtx<'_>) {
    let client = ctx.client;
    let http = ctx.http;
    let config = ctx.config;
    let owner = ctx.owner;
    let repo = ctx.repo;
    let repo_slug = ctx.repo_slug;
    let repo_cfg = ctx.repo_cfg;
    let _ = repo_cfg;
    let detected = ctx.detected;
    let gh_api_base = github_tags_datasource::api_base_from_endpoint(config.endpoint.as_deref());
    let gh_http = if let Some(ref token) = config.token {
        HttpClient::with_token(token).unwrap_or_else(|_| http.clone())
    } else {
        http.clone()
    };

    // ── Dockerfile ────────────────────────────────────────────────────────────
    for df_file_path in manager_files(detected, "dockerfile") {
        match client.get_raw_file(owner, repo, &df_file_path).await {
            Ok(Some(raw)) => match renovate_core::extractors::dockerfile::extract(&raw.content) {
                Ok(deps) => {
                    tracing::debug!(repo = %repo_slug, file = %df_file_path, total = deps.len(), "extracted dockerfile images");
                    ctx.report.files.push(output::FileReport {
                        path: df_file_path.clone(),
                        manager: "dockerfile".into(),
                        deps: docker_hub_reports(http, &deps).await,
                    });
                }
                Err(err) => {
                    tracing::warn!(repo=%repo_slug, file=%df_file_path, %err, "failed to parse Dockerfile")
                }
            },
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%df_file_path, "Dockerfile not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%df_file_path, %err, "failed to fetch Dockerfile");
                ctx.had_error = true;
            }
        }
    }

    // ── docker-compose ────────────────────────────────────────────────────────
    for compose_file_path in manager_files(detected, "docker-compose") {
        match client.get_raw_file(owner, repo, &compose_file_path).await {
            Ok(Some(raw)) => {
                match renovate_core::extractors::docker_compose::extract(&raw.content) {
                    Ok(deps) => {
                        let actionable: Vec<_> =
                            deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                        tracing::debug!(
                            repo = %repo_slug, file = %compose_file_path,
                            total = deps.len(), actionable = actionable.len(),
                            "extracted docker-compose images"
                        );

                        let dep_inputs: Vec<docker_datasource::DockerDepInput> = actionable
                            .iter()
                            .filter_map(|d| {
                                let tag = d.tag.as_deref()?;
                                Some(docker_datasource::DockerDepInput {
                                    dep_name: format!("{}:{tag}", d.image),
                                    image: d.image.clone(),
                                    tag: tag.to_owned(),
                                })
                            })
                            .collect();

                        let updates = docker_datasource::fetch_updates_concurrent(
                            http,
                            &dep_inputs,
                            docker_datasource::DOCKER_HUB_API,
                            10,
                        )
                        .await;
                        let update_map: HashMap<_, _> = updates
                            .into_iter()
                            .map(|r| (r.dep_name, r.summary))
                            .collect();

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
                                package_name: None,
                                range_strategy: None,
                                follow_tag: None,
                                pin_digests: None,
                                versioning: None,
                                dependency_dashboard_approval: None,
                                replacement_name: None,
                                replacement_version: None,
                                name: dep.image.clone(),
                                status: output::DepStatus::Skipped {
                                    reason: format!("{:?}", dep.skip_reason.as_ref().unwrap())
                                        .to_lowercase(),
                                },
                            });
                        }
                        for dep in &actionable {
                            let dep_name = match &dep.tag {
                                Some(t) => format!("{}:{t}", dep.image),
                                None => dep.image.clone(),
                            };
                            let status = match update_map.get(&dep_name) {
                                Some(Ok(s)) if s.update_available => {
                                    output::DepStatus::UpdateAvailable {
                                        current: s.current_tag.clone(),
                                        latest: s.latest.clone().unwrap_or_default(),
                                    }
                                }
                                Some(Ok(s)) => output::DepStatus::UpToDate {
                                    latest: s.latest.clone(),
                                },
                                Some(Err(docker_datasource::DockerHubError::NonDockerHub(_))) => {
                                    output::DepStatus::Skipped {
                                        reason: "non-docker-hub registry".into(),
                                    }
                                }
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
                                package_name: None,
                                range_strategy: None,
                                follow_tag: None,
                                pin_digests: None,
                                versioning: None,
                                dependency_dashboard_approval: None,
                                replacement_name: None,
                                replacement_version: None,
                                name: dep_name,
                                status,
                            });
                        }
                        ctx.report.files.push(output::FileReport {
                            path: compose_file_path.clone(),
                            manager: "docker-compose".into(),
                            deps: file_deps,
                        });
                    }
                    Err(err) => tracing::warn!(repo=%repo_slug, file=%compose_file_path, %err,
                        "failed to parse docker-compose file"),
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%compose_file_path, "compose file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%compose_file_path, %err,
                    "failed to fetch docker-compose file");
                ctx.had_error = true;
            }
        }
    }

    // ── Dev Container (devcontainer.json) ────────────────────────────────────
    for dc_path in manager_files(detected, "devcontainer") {
        match client.get_raw_file(owner, repo, &dc_path).await {
            Ok(Some(raw)) => {
                let extracted = renovate_core::extractors::devcontainer::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %dc_path,
                    docker = extracted.docker_deps.len(),
                    version = extracted.version_deps.len(),
                    "extracted devcontainer deps"
                );
                let mut dep_reports = docker_hub_reports(http, &extracted.docker_deps).await;

                for vdep in &extracted.version_deps {
                    use renovate_core::extractors::asdf::AsdfDatasource;
                    let lookup_key = match &vdep.datasource {
                        AsdfDatasource::GithubTags { repo, tag_strip } => {
                            format!("{}|{}", repo, tag_strip)
                        }
                        AsdfDatasource::GithubReleases { repo, tag_strip } => {
                            format!("{}|{}", repo, tag_strip)
                        }
                    };
                    let (ds_repo, tag_strip) =
                        lookup_key.split_once('|').unwrap_or((&lookup_key, ""));
                    let tag_result = match &vdep.datasource {
                        AsdfDatasource::GithubTags { .. } => {
                            renovate_core::datasources::github_tags::fetch_latest_tag(
                                ds_repo,
                                &gh_http,
                                gh_api_base,
                            )
                            .await
                            .map_err(|e| e.to_string())
                        }
                        AsdfDatasource::GithubReleases { .. } => {
                            github_releases_datasource::fetch_latest_release(
                                ds_repo,
                                &gh_http,
                                gh_api_base,
                            )
                            .await
                            .map(|r| r.map(|(tag, _)| tag))
                            .map_err(|e| e.to_string())
                        }
                    };
                    let status = match tag_result {
                        Ok(Some(tag)) => {
                            let stripped = tag.trim_start_matches(tag_strip);
                            let latest_ver = if vdep.tool == "ruby" {
                                stripped.replace('_', ".")
                            } else {
                                stripped.to_owned()
                            };
                            let s =
                                renovate_core::versioning::semver_generic::semver_update_summary(
                                    &vdep.current_value,
                                    Some(latest_ver.as_str()),
                                );
                            if s.update_available {
                                output::DepStatus::UpdateAvailable {
                                    current: vdep.current_value.clone(),
                                    latest: latest_ver,
                                }
                            } else {
                                output::DepStatus::UpToDate {
                                    latest: Some(latest_ver),
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

                        dep_type: None,
                        package_name: None,
                        range_strategy: None,
                        follow_tag: None,
                        pin_digests: None,
                        versioning: None,
                        dependency_dashboard_approval: None,
                        replacement_name: None,
                        replacement_version: None,
                        name: vdep.tool.to_owned(),
                        status,
                    });
                }

                ctx.report.files.push(output::FileReport {
                    path: dc_path.clone(),
                    manager: "devcontainer".into(),
                    deps: dep_reports,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%dc_path, "devcontainer.json not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%dc_path, %err, "failed to fetch devcontainer.json");
                ctx.had_error = true;
            }
        }
    }

    // ── Quadlet (.container / .image / .volume) ───────────────────────────────
    for qlet_path in manager_files(detected, "quadlet") {
        match client.get_raw_file(owner, repo, &qlet_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::quadlet::extract(&raw.content);
                tracing::debug!(repo = %repo_slug, file = %qlet_path, total = deps.len(), "extracted quadlet images");
                ctx.report.files.push(output::FileReport {
                    path: qlet_path.clone(),
                    manager: "quadlet".into(),
                    deps: docker_hub_reports(http, &deps).await,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%qlet_path, "quadlet file not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%qlet_path, %err, "failed to fetch quadlet file");
                ctx.had_error = true;
            }
        }
    }
}
