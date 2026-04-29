//! Vendir manager (vendir.yml).

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
    let detected = ctx.detected;

    // ── Vendir (vendir.yml) ───────────────────────────────────────────────────
    for vendir_path in manager_files(detected, "vendir") {
        match client.get_raw_file(owner, repo, &vendir_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::vendir::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| !repo_cfg.is_dep_ignored_for_manager(&d.chart_name, "vendir"))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %vendir_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted vendir helm charts"
                );
                let helm_inputs: Vec<helm_datasource::HelmDepInput> = actionable
                    .iter()
                    .map(|d| helm_datasource::HelmDepInput {
                        name: d.chart_name.clone(),
                        current_value: d.version.clone(),
                        repository_url: d.repo_url.clone(),
                    })
                    .collect();
                let updates =
                    helm_datasource::fetch_updates_concurrent(http, &helm_inputs, 8).await;
                let update_map: HashMap<_, _> =
                    updates.into_iter().map(|r| (r.name, r.summary)).collect();
                let file_deps: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        let status = match update_map.get(&dep.chart_name) {
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

                            dep_type: None,
                           package_name: None,
                            name: dep.chart_name.clone(),
                            status,
                        }
                    })
                    .collect();
                ctx.report.files.push(output::FileReport {
                    path: vendir_path.clone(),
                    manager: "vendir".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%vendir_path, "vendir.yml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%vendir_path, %err, "failed to fetch vendir.yml");
                ctx.had_error = true;
            }
        }
    }

}
