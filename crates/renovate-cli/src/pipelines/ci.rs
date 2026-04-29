//! CI/CD platform managers: GitHub Actions, GitLab CI, CircleCI, Buildkite, Travis, Azure Pipelines, Bitrise, and more.

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

    // ── GitHub Actions ────────────────────────────────────────────────────────
    for gha_file_path in manager_files(detected, "github-actions") {
        match client.get_raw_file(owner, repo, &gha_file_path).await {
            Ok(Some(raw)) => {
                let deps = github_actions_extractor::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !d.current_value.is_empty()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.action, "github-actions")
                    })
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %gha_file_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted github-actions dependencies"
                );
                let dep_inputs: Vec<github_tags_datasource::GithubActionsDepInput> = actionable
                    .iter()
                    .map(|d| github_tags_datasource::GithubActionsDepInput {
                        dep_name: d.action.clone(),
                        current_value: d.current_value.clone(),
                    })
                    .collect();
                let updates = github_tags_datasource::fetch_updates_concurrent(
                    &gh_http,
                    &dep_inputs,
                    gh_api_base,
                    10,
                )
                .await;
                let update_map: HashMap<_, _> = updates
                    .into_iter()
                    .map(|r| (r.dep_name, r.summary))
                    .collect();

                // Also extract container/services Docker images from this workflow file.
                let docker_deps = github_actions_extractor::extract_docker_images(&raw.content);
                let docker_actionable: Vec<_> = docker_deps
                    .iter()
                    .filter(|d| d.skip_reason.is_none())
                    .collect();
                let docker_inputs: Vec<docker_datasource::DockerDepInput> = docker_actionable
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
                let docker_updates = docker_datasource::fetch_updates_concurrent(
                    http,
                    &docker_inputs,
                    docker_datasource::DOCKER_HUB_API,
                    10,
                )
                .await;
                let docker_update_map: HashMap<_, _> = docker_updates
                    .into_iter()
                    .map(|r| (r.dep_name, r.summary))
                    .collect();

                let mut all_deps =
                    build_dep_reports_github_actions(&deps, &actionable, &update_map);
                for dep in &docker_deps {
                    if let Some(reason) = &dep.skip_reason {
                        all_deps.push(output::DepReport {
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            update_type: None,
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: dep.image.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                    } else {
                        let dep_name = match &dep.tag {
                            Some(t) => format!("{}:{t}", dep.image),
                            None => dep.image.clone(),
                        };
                        let status = match docker_update_map.get(&dep_name) {
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
                        all_deps.push(output::DepReport {
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            update_type: None,
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: dep_name,
                            status,
                        });
                    }
                }

                // Extract and report `runs-on:` runner label versions.
                let runner_deps = github_actions_extractor::extract_runner_labels(&raw.content);
                for rdep in &runner_deps {
                    let s = renovate_core::datasources::github_runners::update_summary(
                        &rdep.runner_name,
                        &rdep.current_value,
                    );
                    let dep_name = format!("{}-{}", rdep.runner_name, rdep.current_value);
                    let status = if s.update_available {
                        output::DepStatus::UpdateAvailable {
                            current: s.current.clone(),
                            latest: s.latest.unwrap_or_default(),
                        }
                    } else if s.deprecated {
                        output::DepStatus::Skipped {
                            reason: "deprecated runner".into(),
                        }
                    } else {
                        output::DepStatus::UpToDate { latest: s.latest }
                    };
                    all_deps.push(output::DepReport {
                        branch_name: None,
                        group_name: None,
                        automerge: None,
                        labels: Vec::new(),
                        assignees: Vec::new(),
                        reviewers: Vec::new(),
                        update_type: None,
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
                        name: dep_name,
                        status,
                    });
                }
                ctx.report.files.push(output::FileReport {
                    path: gha_file_path.clone(),
                    manager: "github-actions".into(),
                    deps: all_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%gha_file_path, "workflow file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%gha_file_path, %err, "failed to fetch workflow file");
                ctx.had_error = true;
            }
        }
    }

    // ── Travis CI (.travis.yml) ───────────────────────────────────────────────
    for travis_path in manager_files(detected, "travis") {
        match client.get_raw_file(owner, repo, &travis_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::travis::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %travis_path,
                    total = deps.len(), "extracted travis node_js versions"
                );
                // Reuse the Node.js GitHub Releases lookup (same as nvmrc/node-version).
                let gh_inputs: Vec<github_tags_datasource::GithubActionsDepInput> = deps
                    .iter()
                    .map(|d| github_tags_datasource::GithubActionsDepInput {
                        dep_name: "nodejs/node".to_owned(),
                        current_value: d.version.clone(),
                    })
                    .collect();
                let updates = github_tags_datasource::fetch_updates_concurrent(
                    &gh_http,
                    &gh_inputs,
                    gh_api_base,
                    4,
                )
                .await;
                let file_deps: Vec<output::DepReport> = deps
                    .iter()
                    .zip(updates.iter())
                    .map(|(dep, result)| {
                        let status = match &result.summary {
                            Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                                current: dep.version.clone(),
                                latest: s
                                    .latest
                                    .as_deref()
                                    .map(|l| l.strip_prefix('v').unwrap_or(l).to_owned())
                                    .unwrap_or_default(),
                            },
                            Ok(s) => output::DepStatus::UpToDate {
                                latest: s
                                    .latest
                                    .as_deref()
                                    .map(|l| l.strip_prefix('v').unwrap_or(l).to_owned()),
                            },
                            Err(e) => output::DepStatus::LookupError {
                                message: e.to_string(),
                            },
                        };
                        output::DepReport {
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            update_type: None,
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: format!("node@{}", dep.version),
                            status,
                        }
                    })
                    .collect();
                ctx.report.files.push(output::FileReport {
                    path: travis_path.clone(),
                    manager: "travis".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%travis_path, ".travis.yml not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%travis_path, %err, "failed to fetch .travis.yml");
                ctx.had_error = true;
            }
        }
    }

    // ── GitLab CI (.gitlab-ci.yml) ────────────────────────────────────────────
    for glci_path in manager_files(detected, "gitlabci") {
        match client.get_raw_file(owner, repo, &glci_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::gitlabci::extract(&raw.content);
                let docker_deps: Vec<_> = deps.iter().map(|d| d.dep.clone()).collect();
                tracing::debug!(repo = %repo_slug, file = %glci_path, total = deps.len(), "extracted gitlab-ci images");
                ctx.report.files.push(output::FileReport {
                    path: glci_path.clone(),
                    manager: "gitlabci".into(),
                    deps: docker_hub_reports(http, &docker_deps).await,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%glci_path, ".gitlab-ci.yml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%glci_path, %err, "failed to fetch .gitlab-ci.yml");
                ctx.had_error = true;
            }
        }
    }

    // ── GitLab CI includes (.gitlab-ci.yml include: project refs) ────────────
    for glci_inc_path in manager_files(detected, "gitlabci-include") {
        match client.get_raw_file(owner, repo, &glci_inc_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::gitlabci_include::extract(&raw.content);
                if !deps.is_empty() {
                    tracing::debug!(
                        repo = %repo_slug, file = %glci_inc_path,
                        total = deps.len(), "extracted gitlab-ci include refs"
                    );
                    let gl_inputs: Vec<
                        renovate_core::datasources::gitlab_tags::GitlabTagsDepInput,
                    > = deps
                        .iter()
                        .map(
                            |d| renovate_core::datasources::gitlab_tags::GitlabTagsDepInput {
                                dep_name: d.project.clone(),
                                current_value: d.ref_value.clone(),
                            },
                        )
                        .collect();
                    let gl_updates =
                        renovate_core::datasources::gitlab_tags::fetch_updates_concurrent(
                            http,
                            &gl_inputs,
                            renovate_core::datasources::gitlab_tags::GITLAB_API,
                            8,
                        )
                        .await;
                    let update_map: HashMap<_, _> = gl_updates
                        .into_iter()
                        .map(|r| (r.dep_name, r.summary))
                        .collect();
                    let file_deps: Vec<output::DepReport> = deps
                        .iter()
                        .map(|dep| {
                            let status = match update_map.get(&dep.project) {
                                Some(Ok(s)) if s.update_available => {
                                    output::DepStatus::UpdateAvailable {
                                        current: dep.ref_value.clone(),
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
                                pr_title: None,
                                release_timestamp: None,
                                current_version_timestamp: None,
                                name: dep.project.clone(),
                                status,
                            }
                        })
                        .collect();
                    ctx.report.files.push(output::FileReport {
                        path: glci_inc_path.clone(),
                        manager: "gitlabci-include".into(),
                        deps: file_deps,
                    });
                }
            }
            Ok(None) => {}
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%glci_inc_path, %err, "failed to fetch .gitlab-ci.yml (includes)");
                ctx.had_error = true;
            }
        }
    }

    // ── CircleCI (.circleci/config.yml) ──────────────────────────────────────
    for cci_path in manager_files(detected, "circleci") {
        match client.get_raw_file(owner, repo, &cci_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::circleci::extract(&raw.content);
                let docker_deps: Vec<_> = deps.iter().map(|d| d.dep.clone()).collect();
                let orb_deps = renovate_core::extractors::circleci::extract_orbs(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %cci_path,
                    docker = docker_deps.len(), orbs = orb_deps.len(),
                    "extracted circleci deps"
                );
                let mut all_deps = docker_hub_reports(http, &docker_deps).await;
                if !orb_deps.is_empty() {
                    let orb_inputs: Vec<renovate_core::datasources::orb::OrbDepInput> = orb_deps
                        .iter()
                        .map(|o| renovate_core::datasources::orb::OrbDepInput {
                            package_name: o.package_name.clone(),
                            current_value: o.version.clone(),
                        })
                        .collect();
                    let orb_updates = renovate_core::datasources::orb::fetch_updates_concurrent(
                        http,
                        &orb_inputs,
                        6,
                    )
                    .await;
                    let orb_map: HashMap<_, _> = orb_updates
                        .into_iter()
                        .map(|r| (r.package_name, r.summary))
                        .collect();
                    for orb in &orb_deps {
                        let status = match orb_map.get(&orb.package_name) {
                            Some(Ok(s)) if s.update_available => {
                                output::DepStatus::UpdateAvailable {
                                    current: orb.version.clone(),
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
                        all_deps.push(output::DepReport {
                            branch_name: None,
                            group_name: None,
                            automerge: None,
                            labels: Vec::new(),
                            assignees: Vec::new(),
                            reviewers: Vec::new(),
                            update_type: None,
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: orb.package_name.clone(),
                            status,
                        });
                    }
                }
                ctx.report.files.push(output::FileReport {
                    path: cci_path.clone(),
                    manager: "circleci".into(),
                    deps: all_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%cci_path, "circleci config not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%cci_path, %err, "failed to fetch circleci config");
                ctx.had_error = true;
            }
        }
    }

    // ── Buildkite pipeline YAML ───────────────────────────────────────────────
    for bk_path in manager_files(detected, "buildkite") {
        match client.get_raw_file(owner, repo, &bk_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::buildkite::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.dep_name, "buildkite")
                    })
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %bk_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted buildkite plugins"
                );

                // Group by unique GitHub repo for batched lookups.
                let gh_inputs: Vec<github_tags_datasource::GithubActionsDepInput> = actionable
                    .iter()
                    .filter_map(|d| {
                        if let Some(
                            renovate_core::extractors::buildkite::BuildkiteDatasource::GithubTags {
                                repo: gr,
                            },
                        ) = &d.datasource
                        {
                            Some(github_tags_datasource::GithubActionsDepInput {
                                dep_name: gr.clone(),
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
                    } else if !repo_cfg.is_dep_ignored_for_manager(&dep.dep_name, "buildkite") {
                        let gh_repo = dep.datasource.as_ref().map(
                            |renovate_core::extractors::buildkite::BuildkiteDatasource::GithubTags { repo: gr }| {
                                gr.as_str()
                            },
                        );
                        match gh_repo.and_then(|r| update_map.get(r)) {
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
                            _ => output::DepStatus::UpToDate { latest: None },
                        }
                    } else {
                        output::DepStatus::Skipped {
                            reason: "ignored".to_owned(),
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
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                ctx.report.files.push(output::FileReport {
                    path: bk_path.clone(),
                    manager: "buildkite".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%bk_path, "buildkite pipeline not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%bk_path, %err, "failed to fetch buildkite pipeline");
                ctx.had_error = true;
            }
        }
    }

    // ── Cloud Build (cloudbuild.yaml) ─────────────────────────────────────────
    for cb_path in manager_files(detected, "cloudbuild") {
        match client.get_raw_file(owner, repo, &cb_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::cloudbuild::extract(&raw.content);
                tracing::debug!(repo = %repo_slug, file = %cb_path, total = deps.len(), "extracted cloudbuild images");
                ctx.report.files.push(output::FileReport {
                    path: cb_path.clone(),
                    manager: "cloudbuild".into(),
                    deps: docker_hub_reports(http, &deps).await,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%cb_path, "cloudbuild.yaml not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%cb_path, %err, "failed to fetch cloudbuild.yaml");
                ctx.had_error = true;
            }
        }
    }

    // ── Azure Pipelines ───────────────────────────────────────────────────────
    for az_path in manager_files(detected, "azure-pipelines") {
        match client.get_raw_file(owner, repo, &az_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::azure_pipelines::extract(&raw.content);
                tracing::debug!(repo = %repo_slug, file = %az_path, total = deps.len(), "extracted azure-pipelines deps");
                // Separate containers (docker lookup) from tasks (pending datasource).
                let container_images: Vec<_> = deps
                    .iter()
                    .filter_map(|d| match d {
                        renovate_core::extractors::azure_pipelines::AzPipelinesDep::Container(
                            c,
                        ) => Some(c.clone()),
                        renovate_core::extractors::azure_pipelines::AzPipelinesDep::Task(_) => None,
                    })
                    .collect();
                let mut file_deps = docker_hub_reports(http, &container_images).await;
                for dep in &deps {
                    if let renovate_core::extractors::azure_pipelines::AzPipelinesDep::Task(t) = dep
                    {
                        let status = match renovate_core::datasources::azure_pipelines_tasks::fetch_latest(
                            http,
                            &t.name,
                            &t.version,
                        )
                        .await
                        {
                            Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                                current: t.version.clone(),
                                latest: s.latest.unwrap_or_default(),
                            },
                            Ok(s) => output::DepStatus::UpToDate { latest: s.latest },
                            Err(renovate_core::datasources::azure_pipelines_tasks::AzureTasksError::NotFound(_)) => {
                                output::DepStatus::Skipped {
                                    reason: "task not found in registry".into(),
                                }
                            }
                            Err(e) => output::DepStatus::LookupError {
                                message: e.to_string(),
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
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: format!("{}@{}", t.name, t.version),
                            status,
                        });
                    }
                }
                ctx.report.files.push(output::FileReport {
                    path: az_path.clone(),
                    manager: "azure-pipelines".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%az_path, "azure-pipelines file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%az_path, %err, "failed to fetch azure-pipelines file");
                ctx.had_error = true;
            }
        }
    }

    // ── Bitbucket Pipelines (*-pipelines.yml) ────────────────────────────────
    for bb_path in manager_files(detected, "bitbucket-pipelines") {
        match client.get_raw_file(owner, repo, &bb_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::bitbucket_pipelines::extract(&raw.content);
                tracing::debug!(repo = %repo_slug, file = %bb_path, total = deps.len(), "extracted bitbucket-pipelines images");
                ctx.report.files.push(output::FileReport {
                    path: bb_path.clone(),
                    manager: "bitbucket-pipelines".into(),
                    deps: docker_hub_reports(http, &deps).await,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%bb_path, "bitbucket-pipelines file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%bb_path, %err, "failed to fetch bitbucket-pipelines file");
                ctx.had_error = true;
            }
        }
    }

    // ── Drone CI (.drone.yml) ─────────────────────────────────────────────────
    for drone_path in manager_files(detected, "droneci") {
        match client.get_raw_file(owner, repo, &drone_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::droneci::extract(&raw.content);
                tracing::debug!(repo = %repo_slug, file = %drone_path, total = deps.len(), "extracted droneci images");
                ctx.report.files.push(output::FileReport {
                    path: drone_path.clone(),
                    manager: "droneci".into(),
                    deps: docker_hub_reports(http, &deps).await,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%drone_path, ".drone.yml not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%drone_path, %err, "failed to fetch .drone.yml");
                ctx.had_error = true;
            }
        }
    }

    // ── Crow CI (.crow/*.yml) ─────────────────────────────────────────────────
    for crow_path in manager_files(detected, "crow") {
        match client.get_raw_file(owner, repo, &crow_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::crow::extract(&raw.content);
                tracing::debug!(repo = %repo_slug, file = %crow_path, total = deps.len(), "extracted crow-ci images");
                ctx.report.files.push(output::FileReport {
                    path: crow_path.clone(),
                    manager: "crow".into(),
                    deps: docker_hub_reports(http, &deps).await,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%crow_path, "crow CI file not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%crow_path, %err, "failed to fetch crow CI file");
                ctx.had_error = true;
            }
        }
    }

    // ── Vela CI (.vela.yml) ───────────────────────────────────────────────────
    for vela_path in manager_files(detected, "velaci") {
        match client.get_raw_file(owner, repo, &vela_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::velaci::extract(&raw.content);
                tracing::debug!(repo = %repo_slug, file = %vela_path, total = deps.len(), "extracted vela-ci images");
                ctx.report.files.push(output::FileReport {
                    path: vela_path.clone(),
                    manager: "velaci".into(),
                    deps: docker_hub_reports(http, &deps).await,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%vela_path, ".vela.yml not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%vela_path, %err, "failed to fetch .vela.yml");
                ctx.had_error = true;
            }
        }
    }

    // ── Woodpecker CI (.woodpecker.yml / .woodpecker/*.yml) ──────────────────
    for wp_path in manager_files(detected, "woodpecker") {
        match client.get_raw_file(owner, repo, &wp_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::woodpecker::extract(&raw.content);
                tracing::debug!(repo = %repo_slug, file = %wp_path, total = deps.len(), "extracted woodpecker images");
                ctx.report.files.push(output::FileReport {
                    path: wp_path.clone(),
                    manager: "woodpecker".into(),
                    deps: docker_hub_reports(http, &deps).await,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%wp_path, "woodpecker config not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%wp_path, %err, "failed to fetch woodpecker config");
                ctx.had_error = true;
            }
        }
    }

    // ── Bitrise CI (bitrise.yml / bitrise.yaml) ────────────────────────────────
    for br_path in manager_files(detected, "bitrise") {
        match client.get_raw_file(owner, repo, &br_path).await {
            Ok(Some(raw)) => {
                use renovate_core::extractors::bitrise::{BitriseSkipReason, BitriseSource};
                let deps = renovate_core::extractors::bitrise::extract(&raw.content);
                let actionable_count = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.dep_name, "bitrise")
                    })
                    .count();
                tracing::debug!(
                    repo = %repo_slug, file = %br_path,
                    total = deps.len(), actionable = actionable_count,
                    "extracted bitrise steps"
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
                            pr_title: None,
                            release_timestamp: None,
                            current_version_timestamp: None,
                            name: dep.dep_name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: match reason {
                                    BitriseSkipReason::UnspecifiedVersion => {
                                        "unspecified-version".to_owned()
                                    }
                                },
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored_for_manager(&dep.dep_name, "bitrise") {
                        continue;
                    }
                    let current = dep.current_value.as_deref().unwrap_or("");
                    let status = match &dep.source {
                        BitriseSource::Git { repo_url } => {
                            match github_tags_datasource::fetch_latest_tag(
                                repo_url,
                                &gh_http,
                                gh_api_base,
                            )
                            .await
                            {
                                Ok(Some(tag)) => {
                                    let stripped = tag.trim_start_matches('v');
                                    let s = renovate_core::versioning::semver_generic::semver_update_summary(
                                        current,
                                        Some(stripped),
                                    );
                                    if s.update_available {
                                        output::DepStatus::UpdateAvailable {
                                            current: current.to_owned(),
                                            latest: stripped.to_owned(),
                                        }
                                    } else {
                                        output::DepStatus::UpToDate {
                                            latest: Some(stripped.to_owned()),
                                        }
                                    }
                                }
                                Ok(None) => output::DepStatus::UpToDate { latest: None },
                                Err(e) => output::DepStatus::LookupError {
                                    message: e.to_string(),
                                },
                            }
                        }
                        BitriseSource::Steplib { registry_url } => {
                            let registry = registry_url
                                .as_deref()
                                .unwrap_or(bitrise_datasource::DEFAULT_STEPLIB_URL);
                            match bitrise_datasource::fetch_latest(
                                http,
                                &dep.dep_name,
                                current,
                                registry,
                            )
                            .await
                            {
                                Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                                    current: current.to_owned(),
                                    latest: s.latest.unwrap_or_default(),
                                },
                                Ok(s) => output::DepStatus::UpToDate { latest: s.latest },
                                Err(e) => output::DepStatus::LookupError {
                                    message: e.to_string(),
                                },
                            }
                        }
                        BitriseSource::Local => output::DepStatus::Skipped {
                            reason: "local-dependency".to_owned(),
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
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: br_path.clone(),
                        manager: "bitrise".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%br_path, "bitrise.yml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%br_path, %err, "failed to fetch bitrise.yml");
                ctx.had_error = true;
            }
        }
    }
}
