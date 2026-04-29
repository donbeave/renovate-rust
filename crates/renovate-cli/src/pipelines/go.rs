//! Go modules (`go.mod`) package manager pipeline.

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

    // ── Go modules (go.mod) ──────────────────────────────────────────────────
    // Two-pass Go module dedup: Go workspaces may have multiple go.mod files.
    {
        let gomod_files = manager_files(detected, "gomod");
        let mut gomod_file_deps: Vec<(
            String,
            Vec<renovate_core::extractors::gomod::GoModExtractedDep>,
        )> = Vec::new();
        for gomod_file_path in &gomod_files {
            match client.get_raw_file(owner, repo, gomod_file_path).await {
                Ok(Some(raw)) => {
                    let deps = gomod_extractor::extract(&raw.content);
                    gomod_file_deps.push((gomod_file_path.clone(), deps));
                }
                Ok(None) => {
                    tracing::warn!(repo=%repo_slug, file=%gomod_file_path, "go.mod not found")
                }
                Err(err) => {
                    tracing::error!(repo=%repo_slug, file=%gomod_file_path, %err,
                        "failed to fetch go.mod");
                    ctx.had_error = true;
                }
            }
        }
        let unique_modules: Vec<String> = {
            let mut seen = std::collections::HashSet::new();
            gomod_file_deps
                .iter()
                .flat_map(|(_, deps)| deps.iter())
                .filter(|d| {
                    d.skip_reason.is_none()
                        && !repo_cfg.is_dep_ignored_for_manager(&d.module_path, "gomod")
                        && !d.current_value.is_empty()
                })
                .filter(|d| seen.insert(d.module_path.clone()))
                .map(|d| d.module_path.clone())
                .collect()
        };
        tracing::debug!(
            repo = %repo_slug,
            files = gomod_file_deps.len(),
            unique_modules = unique_modules.len(),
            "fetching go module versions (deduplicated)"
        );
        let latest_cache = gomod_datasource::fetch_latest_batch(
            http,
            &unique_modules,
            gomod_datasource::GO_PROXY_BASE,
            10,
        )
        .await;
        for (gomod_file_path, deps) in gomod_file_deps {
            let actionable: Vec<_> = deps
                .iter()
                .filter(|d| {
                    d.skip_reason.is_none()
                        && !repo_cfg.is_dep_ignored_for_manager(&d.module_path, "gomod")
                        && !d.current_value.is_empty()
                })
                .collect();
            let update_map: HashMap<_, Result<gomod_datasource::GoModUpdateSummary, _>> =
                actionable
                    .iter()
                    .map(|d| {
                        let latest = latest_cache.get(&d.module_path).cloned().unwrap_or(None);
                        let summary = Ok::<_, gomod_datasource::GoModError>(
                            gomod_datasource::summary_from_cache(&d.current_value, latest),
                        );
                        (d.module_path.clone(), summary)
                    })
                    .collect();
            ctx.report.files.push(output::FileReport {
                path: gomod_file_path.clone(),
                manager: "gomod".into(),
                deps: build_dep_reports_gomod(&deps, &actionable, &update_map),
            });
        }
    }
}
