//! Helm ecosystem managers: Helm, Helm Values, Helmfile, Helmsman, Fleet, helm-requirements.

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

    // ── Helm (Chart.yaml / requirements.yaml) ────────────────────────────────
    for helm_file_path in manager_files(detected, "helmv3") {
        match client.get_raw_file(owner, repo, &helm_file_path).await {
            Ok(Some(raw)) => {
                let deps = helm_extractor::extract(&raw.content);
                let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                tracing::debug!(
                    repo = %repo_slug, file = %helm_file_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted helm chart deps"
                );
                let dep_inputs: Vec<helm_datasource::HelmDepInput> = actionable
                    .iter()
                    .map(|d| helm_datasource::HelmDepInput {
                        name: d.name.clone(),
                        current_value: d.current_value.clone(),
                        repository_url: d.repository.clone(),
                    })
                    .collect();
                let updates = helm_datasource::fetch_updates_concurrent(http, &dep_inputs, 8).await;
                let update_map: HashMap<_, _> =
                    updates.into_iter().map(|r| (r.name, r.summary)).collect();
                ctx.report.files.push(output::FileReport {
                    path: helm_file_path.clone(),
                    manager: "helmv3".into(),
                    deps: build_dep_reports_helm(&deps, &actionable, &update_map),
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%helm_file_path, "Chart.yaml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%helm_file_path, %err, "failed to fetch Chart.yaml");
                ctx.had_error = true;
            }
        }
    }

    // ── Helm Values (values.yaml) ─────────────────────────────────────────────
    for hv_path in manager_files(detected, "helm-values") {
        match client.get_raw_file(owner, repo, &hv_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::helm_values::extract(&raw.content);
                tracing::debug!(repo = %repo_slug, file = %hv_path, total = deps.len(), "extracted helm values images");
                ctx.report.files.push(output::FileReport {
                    path: hv_path.clone(),
                    manager: "helm-values".into(),
                    deps: docker_hub_reports(http, &deps).await,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%hv_path, "values.yaml not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%hv_path, %err, "failed to fetch values.yaml");
                ctx.had_error = true;
            }
        }
    }

    // ── Helmfile (helmfile.yaml / helmfile.d/*.yaml) ──────────────────────────
    for hf_path in manager_files(detected, "helmfile") {
        match client.get_raw_file(owner, repo, &hf_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::helmfile::extract(&raw.content);
                let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                tracing::debug!(
                    repo = %repo_slug, file = %hf_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted helmfile chart deps"
                );
                let dep_inputs: Vec<helm_datasource::HelmDepInput> = actionable
                    .iter()
                    .map(|d| helm_datasource::HelmDepInput {
                        name: d.name.clone(),
                        current_value: d.current_value.clone(),
                        repository_url: d.repository.clone(),
                    })
                    .collect();
                let updates = helm_datasource::fetch_updates_concurrent(http, &dep_inputs, 8).await;
                let update_map: HashMap<_, _> =
                    updates.into_iter().map(|r| (r.name, r.summary)).collect();
                ctx.report.files.push(output::FileReport {
                    path: hf_path.clone(),
                    manager: "helmfile".into(),
                    deps: build_dep_reports_helm(&deps, &actionable, &update_map),
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%hf_path, "helmfile not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%hf_path, %err, "failed to fetch helmfile");
                ctx.had_error = true;
            }
        }
    }

    // ── Fleet (fleet.yaml / GitRepo CRDs) ────────────────────────────────────
    for fleet_path in manager_files(detected, "fleet") {
        match client.get_raw_file(owner, repo, &fleet_path).await {
            Ok(Some(raw)) => {
                let is_fleet_yaml =
                    renovate_core::extractors::fleet::is_fleet_yaml_path(&fleet_path);
                let extracted =
                    renovate_core::extractors::fleet::extract(&raw.content, is_fleet_yaml);
                tracing::debug!(
                    repo = %repo_slug, file = %fleet_path,
                    helm = extracted.helm_deps.len(),
                    git = extracted.git_deps.len(),
                    "extracted fleet deps"
                );

                let mut dep_reports: Vec<output::DepReport> = Vec::new();

                // Helm deps
                let helm_actionable: Vec<_> = extracted
                    .helm_deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.chart, "fleet")
                            && !d.current_value.is_empty()
                    })
                    .collect();
                if !helm_actionable.is_empty() {
                    let dep_inputs: Vec<helm_datasource::HelmDepInput> = helm_actionable
                        .iter()
                        .map(|d| helm_datasource::HelmDepInput {
                            name: d.chart.clone(),
                            current_value: d.current_value.clone(),
                            repository_url: d.registry_url.clone(),
                        })
                        .collect();
                    let updates =
                        helm_datasource::fetch_updates_concurrent(http, &dep_inputs, 8).await;
                    let update_map: HashMap<_, _> =
                        updates.into_iter().map(|r| (r.name, r.summary)).collect();
                    for dep in &extracted.helm_deps {
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
                                name: dep.chart.clone(),
                                status: output::DepStatus::Skipped {
                                    reason: format!("{reason:?}").to_lowercase(),
                                },
                            });
                            continue;
                        }
                        let status = match update_map.get(&dep.chart) {
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
                            name: dep.chart.clone(),
                            status,
                        });
                    }
                }

                // Git repo deps
                for git_dep in &extracted.git_deps {
                    if let Some(ref reason) = git_dep.skip_reason {
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
                            name: git_dep.repo_url.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                        continue;
                    }
                    let repo_name = git_dep
                        .repo_url
                        .trim_end_matches('/')
                        .trim_end_matches(".git")
                        .trim_start_matches("https://github.com/")
                        .trim_start_matches("http://github.com/");
                    let tag_result = renovate_core::datasources::github_tags::fetch_latest_tag(
                        repo_name,
                        &gh_http,
                        gh_api_base,
                    )
                    .await
                    .map_err(|e| e.to_string());
                    let status = match tag_result {
                        Ok(Some(tag)) => {
                            let s =
                                renovate_core::versioning::semver_generic::semver_update_summary(
                                    &git_dep.current_value,
                                    Some(&tag),
                                );
                            if s.update_available {
                                output::DepStatus::UpdateAvailable {
                                    current: git_dep.current_value.clone(),
                                    latest: tag,
                                }
                            } else {
                                output::DepStatus::UpToDate { latest: Some(tag) }
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
                        name: git_dep.repo_url.clone(),
                        status,
                    });
                }

                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: fleet_path.clone(),
                        manager: "fleet".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%fleet_path, "fleet file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%fleet_path, %err, "failed to fetch fleet file");
                ctx.had_error = true;
            }
        }
    }

    // ── Helmsman DSF (helmsman.yml / helmsman.d/*.yml) ────────────────────────
    for hsm_path in manager_files(detected, "helmsman") {
        match client.get_raw_file(owner, repo, &hsm_path).await {
            Ok(Some(raw)) => {
                use renovate_core::extractors::helmsman::HelmsmanSkipReason;
                let deps = renovate_core::extractors::helmsman::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.dep_name, "helmsman")
                    })
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %hsm_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted helmsman deps"
                );
                let helm_inputs: Vec<helm_datasource::HelmDepInput> = actionable
                    .iter()
                    .map(|d| helm_datasource::HelmDepInput {
                        name: d.chart_name.clone(),
                        current_value: d.current_value.clone(),
                        repository_url: d.registry_url.clone(),
                    })
                    .collect();
                let updates =
                    helm_datasource::fetch_updates_concurrent(http, &helm_inputs, 8).await;
                let update_map: HashMap<_, _> =
                    updates.into_iter().map(|r| (r.name, r.summary)).collect();
                let dep_reports: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        if let Some(reason) = &dep.skip_reason {
                            return output::DepReport {
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
                                name: dep.dep_name.clone(),
                                status: output::DepStatus::Skipped {
                                    reason: match reason {
                                        HelmsmanSkipReason::UnspecifiedVersion => {
                                            "unspecified-version".to_owned()
                                        }
                                        HelmsmanSkipReason::InvalidChart => {
                                            "invalid-name".to_owned()
                                        }
                                        HelmsmanSkipReason::NoRepository => {
                                            "no-repository".to_owned()
                                        }
                                    },
                                },
                            };
                        }
                        let status = match update_map.get(&dep.chart_name) {
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
                            name: dep.dep_name.clone(),
                            status,
                        }
                    })
                    .collect();
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: hsm_path.clone(),
                        manager: "helmsman".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%hsm_path, "helmsman file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%hsm_path, %err, "failed to fetch helmsman file");
                ctx.had_error = true;
            }
        }
    }

    // ── helm-requirements (Helm v2 requirements.yaml) ─────────────────────────
    // Already handled by the helmv3 pipeline; register the manager name alias
    // so detection works, but skip files already captured by helmv3.
    // (No separate processing needed — helmv3 covers the same pattern.)
}
