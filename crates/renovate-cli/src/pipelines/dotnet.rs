//! .NET managers: NuGet (`.csproj`/`.props`) and Cake build scripts.

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

    // ── NuGet (.csproj / .props / .targets) ──────────────────────────────────
    // Two-pass NuGet dedup: .NET solutions share many NuGet packages across projects.
    {
        let nuget_files = manager_files(detected, "nuget");
        let mut nuget_file_deps: Vec<(
            String,
            Vec<renovate_core::extractors::nuget::NuGetExtractedDep>,
        )> = Vec::new();
        for nuget_file_path in &nuget_files {
            match client.get_raw_file(owner, repo, nuget_file_path).await {
                Ok(Some(raw)) => match nuget_extractor::extract(&raw.content) {
                    Ok(deps) => nuget_file_deps.push((nuget_file_path.clone(), deps)),
                    Err(err) => tracing::warn!(repo=%repo_slug, file=%nuget_file_path, %err,
                        "failed to parse nuget project file"),
                },
                Ok(None) => tracing::warn!(repo=%repo_slug, file=%nuget_file_path,
                    "nuget file not found"),
                Err(err) => {
                    tracing::error!(repo=%repo_slug, file=%nuget_file_path, %err,
                        "failed to fetch nuget file");
                    ctx.had_error = true;
                }
            }
        }
        let unique_ids: Vec<String> = {
            let mut seen = std::collections::HashSet::new();
            nuget_file_deps
                .iter()
                .flat_map(|(_, deps)| deps.iter())
                .filter(|d| {
                    d.skip_reason.is_none()
                        && !repo_cfg.is_dep_ignored_for_manager(&d.package_id, "nuget")
                })
                .filter(|d| seen.insert(d.package_id.clone()))
                .map(|d| d.package_id.clone())
                .collect()
        };
        tracing::debug!(
            repo = %repo_slug,
            files = nuget_file_deps.len(),
            unique_packages = unique_ids.len(),
            "fetching nuget versions (deduplicated)"
        );
        let latest_cache = nuget_datasource::fetch_latest_batch(
            http,
            &unique_ids,
            nuget_datasource::NUGET_API,
            10,
        )
        .await;
        for (nuget_file_path, deps) in nuget_file_deps {
            let actionable: Vec<_> = deps
                .iter()
                .filter(|d| {
                    d.skip_reason.is_none()
                        && !repo_cfg.is_dep_ignored_for_manager(&d.package_id, "nuget")
                })
                .collect();
            let update_map: HashMap<_, Result<nuget_datasource::NuGetUpdateSummary, _>> =
                actionable
                    .iter()
                    .map(|d| {
                        let latest = latest_cache.get(&d.package_id).cloned().unwrap_or(None);
                        let summary = Ok::<_, nuget_datasource::NuGetError>(
                            nuget_datasource::summary_from_cache(&d.current_value, &latest),
                        );
                        (d.package_id.clone(), summary)
                    })
                    .collect();
            ctx.report.files.push(output::FileReport {
                path: nuget_file_path.clone(),
                manager: "nuget".into(),
                deps: build_dep_reports_nuget(&deps, &actionable, &update_map),
            });
        }
    }

    // ── Cake build scripts (.cake) ────────────────────────────────────────────
    for cake_path in manager_files(detected, "cake") {
        match client.get_raw_file(owner, repo, &cake_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::cake::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        !d.current_value.is_empty()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.package_name, "cake")
                    })
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %cake_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted cake deps"
                );
                let nuget_inputs: Vec<nuget_datasource::NuGetDepInput> = actionable
                    .iter()
                    .map(|d| nuget_datasource::NuGetDepInput {
                        package_id: d.package_name.clone(),
                        current_value: d.current_value.clone(),
                    })
                    .collect();
                let nuget_updates = nuget_datasource::fetch_updates_concurrent(
                    http,
                    &nuget_inputs,
                    nuget_datasource::NUGET_API,
                    8,
                )
                .await;
                let update_map: HashMap<_, _> = nuget_updates
                    .into_iter()
                    .map(|r| (r.package_id, r.summary))
                    .collect();
                let file_deps: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        let status = if dep.current_value.is_empty() {
                            output::DepStatus::Skipped {
                                reason: "no-version".into(),
                            }
                        } else {
                            match update_map.get(&dep.package_name) {
                                Some(Ok(s)) if s.update_available => {
                                    output::DepStatus::UpdateAvailable {
                                        current: dep.current_value.clone(),
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
                            dependency_dashboard_approval: None,
                            replacement_name: None,
                            replacement_version: None,
                            name: dep.package_name.clone(),
                            status,
                        }
                    })
                    .collect();
                ctx.report.files.push(output::FileReport {
                    path: cake_path.clone(),
                    manager: "cake".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%cake_path, "cake script not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%cake_path, %err, "failed to fetch cake script");
                ctx.had_error = true;
            }
        }
    }
}
