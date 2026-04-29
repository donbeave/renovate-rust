//! Cargo (`Cargo.toml`) package manager pipeline.

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

    // ── Cargo ─────────────────────────────────────────────────────────────────
    // Two-pass: deduplicate crate lookups across Cargo workspace members.
    {
        let cargo_files = manager_files(detected, "cargo");
        let mut cargo_file_deps: Vec<(
            String,
            Vec<renovate_core::extractors::cargo::ExtractedDep>,
        )> = Vec::new();
        for cargo_file_path in &cargo_files {
            match client.get_raw_file(owner, repo, cargo_file_path).await {
                Ok(Some(raw)) => match cargo_extractor::extract(&raw.content) {
                    Ok(deps) => cargo_file_deps.push((cargo_file_path.clone(), deps)),
                    Err(err) => tracing::warn!(repo=%repo_slug, file=%cargo_file_path, %err,
                        "failed to parse Cargo.toml"),
                },
                Ok(None) => {
                    tracing::warn!(repo=%repo_slug, file=%cargo_file_path, "Cargo.toml not found")
                }
                Err(err) => {
                    tracing::error!(repo=%repo_slug, file=%cargo_file_path, %err,
                        "failed to fetch Cargo.toml");
                    ctx.had_error = true;
                }
            }
        }
        let unique_crate_names: Vec<String> = {
            let mut seen = std::collections::HashSet::new();
            cargo_file_deps
                .iter()
                .flat_map(|(_, deps)| deps.iter())
                .filter(|d| {
                    d.skip_reason.is_none()
                        && !repo_cfg.is_dep_ignored_for_manager(&d.dep_name, "cargo")
                })
                .filter(|d| seen.insert(d.package_name.clone()))
                .map(|d| d.package_name.clone())
                .collect()
        };
        tracing::debug!(
            repo = %repo_slug,
            files = cargo_file_deps.len(),
            unique_crates = unique_crate_names.len(),
            "fetching crate versions (deduplicated)"
        );
        let (versions_cache, timestamps_cache) = tokio::join!(
            crates_io::fetch_versions_batch(
                http,
                &unique_crate_names,
                crates_io::CRATES_IO_SPARSE_INDEX,
                10,
            ),
            crates_io::fetch_timestamps_batch(
                http,
                &unique_crate_names,
                crates_io::CRATES_IO_API,
                10,
            ),
        );
        for (cargo_file_path, deps) in cargo_file_deps {
            let actionable: Vec<_> = deps
                .iter()
                .filter(|d| {
                    d.skip_reason.is_none()
                        && !repo_cfg.is_dep_ignored_for_manager(&d.dep_name, "cargo")
                })
                .collect();
            let update_map: HashMap<_, Result<renovate_core::versioning::cargo::UpdateSummary, _>> =
                actionable
                    .iter()
                    .map(|d| {
                        let summary = versions_cache
                            .get(&d.package_name)
                            .map(|entry| crates_io::summary_from_cache(&d.current_value, entry))
                            .ok_or_else(|| {
                                crates_io::CratesIoError::NotFound(d.package_name.clone())
                            });
                        (d.dep_name.clone(), summary)
                    })
                    .collect();
            ctx.report.files.push(output::FileReport {
                path: cargo_file_path.clone(),
                manager: "cargo".into(),
                deps: build_dep_reports_cargo(&deps, &actionable, &update_map, &timestamps_cache),
            });
        }
    }
}
