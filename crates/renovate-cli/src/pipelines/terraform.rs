//! Infrastructure-as-Code managers: Terraform, Terragrunt, TFLint, Azure Bicep.

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

    // ── Terraform (.tf / .tofu) ───────────────────────────────────────────────
    for tf_file_path in manager_files(detected, "terraform") {
        match client.get_raw_file(owner, repo, &tf_file_path).await {
            Ok(Some(raw)) => {
                let deps = terraform_extractor::extract(&raw.content);
                let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                tracing::debug!(
                    repo = %repo_slug, file = %tf_file_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted terraform deps"
                );
                let dep_inputs: Vec<terraform_datasource::TerraformDepInput> = actionable
                    .iter()
                    .map(|d| terraform_datasource::TerraformDepInput {
                        name: d.name.clone(),
                        current_value: d.current_value.clone(),
                        kind: match d.dep_type {
                            terraform_extractor::TerraformDepType::Provider => {
                                terraform_datasource::TerraformLookupKind::Provider
                            }
                            terraform_extractor::TerraformDepType::Module => {
                                terraform_datasource::TerraformLookupKind::Module
                            }
                        },
                    })
                    .collect();
                let updates = terraform_datasource::fetch_updates_concurrent(
                    http,
                    &dep_inputs,
                    terraform_datasource::TERRAFORM_REGISTRY,
                    8,
                )
                .await;
                let update_map: HashMap<_, _> =
                    updates.into_iter().map(|r| (r.name, r.summary)).collect();
                ctx.report.files.push(output::FileReport {
                    path: tf_file_path.clone(),
                    manager: "terraform".into(),
                    deps: build_dep_reports_terraform(&deps, &actionable, &update_map),
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%tf_file_path, "Terraform file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%tf_file_path, %err, "failed to fetch Terraform file");
                ctx.had_error = true;
            }
        }
    }

    // ── Terragrunt (terragrunt.hcl) ───────────────────────────────────────────
    for tg_path in manager_files(detected, "terragrunt") {
        match client.get_raw_file(owner, repo, &tg_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::terragrunt::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %tg_path,
                    total = deps.len(),
                    "extracted terragrunt module deps"
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
                    if repo_cfg.is_dep_ignored_for_manager(&dep.dep_name, "terragrunt") {
                        continue;
                    }

                    use renovate_core::extractors::terragrunt::TerragruntSource;
                    let status = match &dep.source {
                        Some(TerragruntSource::GitHub(gh_repo)) => {
                            let tag_result =
                                renovate_core::datasources::github_tags::fetch_latest_tag(
                                    gh_repo,
                                    &gh_http,
                                    gh_api_base,
                                )
                                .await
                                .map_err(|e| e.to_string());
                            match tag_result {
                                Ok(Some(tag)) => {
                                    let stripped = tag.trim_start_matches('v');
                                    let clean = dep.current_value.trim_start_matches('v');
                                    let s = renovate_core::versioning::semver_generic::semver_update_summary(clean, Some(stripped));
                                    if s.update_available {
                                        output::DepStatus::UpdateAvailable {
                                            current: dep.current_value.clone(),
                                            latest: tag.clone(),
                                        }
                                    } else {
                                        output::DepStatus::UpToDate { latest: Some(tag) }
                                    }
                                }
                                Ok(None) => output::DepStatus::UpToDate { latest: None },
                                Err(e) => output::DepStatus::LookupError { message: e },
                            }
                        }
                        Some(TerragruntSource::TerraformRegistry { hostname, module }) => {
                            let registry_base = format!("https://{hostname}/api/v1/modules");
                            let inputs = vec![terraform_datasource::TerraformDepInput {
                                name: module.clone(),
                                current_value: dep.current_value.clone(),
                                kind: terraform_datasource::TerraformLookupKind::Module,
                            }];
                            let updates = terraform_datasource::fetch_updates_concurrent(
                                http,
                                &inputs,
                                &registry_base,
                                8,
                            )
                            .await;
                            let update_map: HashMap<_, _> =
                                updates.into_iter().map(|r| (r.name, r.summary)).collect();
                            match update_map.get(module) {
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
                        }
                        _ => output::DepStatus::Skipped {
                            reason: "unsupported-source".into(),
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
                        path: tg_path.clone(),
                        manager: "terragrunt".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%tg_path, "terragrunt.hcl not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%tg_path, %err, "failed to fetch terragrunt.hcl");
                ctx.had_error = true;
            }
        }
    }

    // ── TFLint plugin (.tflint.hcl) ──────────────────────────────────────────
    for tflint_path in manager_files(detected, "tflint-plugin") {
        match client.get_raw_file(owner, repo, &tflint_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::tflint_plugin::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %tflint_path,
                    total = deps.len(),
                    "extracted tflint plugin deps"
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
                    if repo_cfg.is_dep_ignored_for_manager(&dep.dep_name, "tflint-plugin") {
                        continue;
                    }
                    let status = match github_releases_datasource::fetch_latest_release(
                        &dep.dep_name,
                        &gh_http,
                        gh_api_base,
                    )
                    .await
                    {
                        Ok(Some(tag)) => {
                            let stripped = tag.trim_start_matches('v');
                            let s =
                                renovate_core::versioning::semver_generic::semver_update_summary(
                                    &dep.current_value,
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
                        path: tflint_path.clone(),
                        manager: "tflint-plugin".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%tflint_path, ".tflint.hcl not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%tflint_path, %err, "failed to fetch .tflint.hcl");
                ctx.had_error = true;
            }
        }
    }

    // ── Azure Bicep (*.bicep) ─────────────────────────────────────────────────
    for bicep_path in manager_files(detected, "bicep") {
        match client.get_raw_file(owner, repo, &bicep_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::bicep::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %bicep_path,
                    total = deps.len(),
                    "extracted bicep resource type deps"
                );
                let mut dep_reports = Vec::new();
                for dep in &deps {
                    if repo_cfg.is_dep_ignored_for_manager(&dep.dep_name, "bicep") {
                        continue;
                    }
                    let status = match renovate_core::datasources::azure_bicep::fetch_latest(
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
                        path: bicep_path.clone(),
                        manager: "bicep".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%bicep_path, "bicep file not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%bicep_path, %err, "failed to fetch bicep file");
                ctx.had_error = true;
            }
        }
    }
}
