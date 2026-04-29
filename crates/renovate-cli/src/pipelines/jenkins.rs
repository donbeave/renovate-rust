//! Jenkins and OCB (OpenTelemetry Collector Builder) managers.

use super::*;

pub(crate) async fn process(ctx: &mut RepoPipelineCtx<'_>) {
    let client = ctx.client;
    let http = ctx.http;
    let _ = http;
    let config = ctx.config;
    let _ = config;
    let owner = ctx.owner;
    let repo = ctx.repo;
    let repo_slug = ctx.repo_slug;
    let repo_cfg = ctx.repo_cfg;
    let _ = repo_cfg;
    let detected = ctx.detected;

    // ── Jenkins plugins (plugins.txt / plugins.yml) ───────────────────────────
    for jenkins_path in manager_files(detected, "jenkins") {
        match client.get_raw_file(owner, repo, &jenkins_path).await {
            Ok(Some(raw)) => {
                let deps = if jenkins_path.ends_with(".txt") {
                    renovate_core::extractors::jenkins::extract_txt(&raw.content)
                } else {
                    renovate_core::extractors::jenkins::extract_yml(&raw.content)
                };
                tracing::debug!(
                    repo = %repo_slug, file = %jenkins_path,
                    total = deps.len(),
                    "extracted jenkins plugin deps"
                );
                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    let status = if let Some(reason) = &dep.skip_reason {
                        output::DepStatus::Skipped {
                            reason: format!("{reason:?}").to_lowercase(),
                        }
                    } else if let Some(ver) = &dep.version {
                        match renovate_core::datasources::jenkins_plugins::fetch_latest(
                            http,
                            &dep.artifact_id,
                            ver,
                        )
                        .await
                        {
                            Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                                current: ver.clone(),
                                latest: s.latest.unwrap_or_default(),
                            },
                            Ok(s) => output::DepStatus::UpToDate { latest: s.latest },
                            Err(e) => output::DepStatus::LookupError {
                                message: e.to_string(),
                            },
                        }
                    } else {
                        output::DepStatus::Skipped {
                            reason: "unspecified-version".into(),
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
                        versioning: None,
                        dependency_dashboard_approval: None,
                        replacement_name: None,
                        replacement_version: None,
                        name: dep.artifact_id.clone(),
                        status,
                    });
                }
                ctx.report.files.push(output::FileReport {
                    path: jenkins_path.clone(),
                    manager: "jenkins".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%jenkins_path, "jenkins plugins file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%jenkins_path, %err, "failed to fetch jenkins plugins file");
                ctx.had_error = true;
            }
        }
    }

    // ── OCB (OpenTelemetry Collector Builder) ─────────────────────────────────
    for ocb_path in manager_files(detected, "ocb") {
        match client.get_raw_file(owner, repo, &ocb_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::ocb::extract(&raw.content);
                let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
                tracing::debug!(
                    repo = %repo_slug, file = %ocb_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted OCB go module deps"
                );
                let dep_inputs: Vec<gomod_datasource::GoModDepInput> = actionable
                    .iter()
                    .map(|d| {
                        // The collector otelcol_version is stored without a `v` prefix;
                        // the Go proxy always returns versions with `v`. Normalise here.
                        let ver = if d.dep_type == "collector" && !d.current_value.starts_with('v')
                        {
                            format!("v{}", d.current_value)
                        } else {
                            d.current_value.clone()
                        };
                        gomod_datasource::GoModDepInput {
                            module_path: d.dep_name.clone(),
                            current_value: ver,
                        }
                    })
                    .collect();
                let updates = gomod_datasource::fetch_updates_concurrent(
                    http,
                    &dep_inputs,
                    gomod_datasource::GO_PROXY_BASE,
                    8,
                )
                .await;
                let update_map: HashMap<_, _> = updates
                    .into_iter()
                    .map(|r| (r.module_path, r.summary))
                    .collect();
                let dep_reports: Vec<output::DepReport> = deps
                    .iter()
                    .map(|d| {
                        if let Some(ref reason) = d.skip_reason {
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

                                dep_type: None,
                                package_name: None,
                                range_strategy: None,
                                follow_tag: None,
                                pin_digests: None,
                                versioning: None,
                                dependency_dashboard_approval: None,
                                replacement_name: None,
                                replacement_version: None,
                                name: d.dep_name.clone(),
                                status: output::DepStatus::Skipped {
                                    reason: format!("{reason:?}").to_lowercase(),
                                },
                            };
                        }
                        let status = match update_map.get(&d.dep_name) {
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

                            dep_type: None,
                            package_name: None,
                            range_strategy: None,
                            follow_tag: None,
                            pin_digests: None,
                            versioning: None,
                            dependency_dashboard_approval: None,
                            replacement_name: None,
                            replacement_version: None,
                            name: d.dep_name.clone(),
                            status,
                        }
                    })
                    .collect();
                ctx.report.files.push(output::FileReport {
                    path: ocb_path.clone(),
                    manager: "ocb".into(),
                    deps: dep_reports,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%ocb_path, "OCB config not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%ocb_path, %err, "failed to fetch OCB config");
                ctx.had_error = true;
            }
        }
    }

    // ── Homebrew formula (Formula/*.rb) ────────────────────────────────────────
}
