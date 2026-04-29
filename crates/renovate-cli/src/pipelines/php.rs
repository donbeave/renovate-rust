//! PHP Composer (`composer.json`) package manager pipeline.

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

    // ── Composer (composer.json) ──────────────────────────────────────────────
    for composer_file_path in manager_files(detected, "composer") {
        match client.get_raw_file(owner, repo, &composer_file_path).await {
            Ok(Some(raw)) => match composer_extractor::extract(&raw.content) {
                Ok(deps) => {
                    let actionable: Vec<_> = deps
                        .iter()
                        .filter(|d| {
                            d.skip_reason.is_none()
                                && !repo_cfg.is_dep_ignored_for_manager(&d.name, "composer")
                        })
                        .collect();
                    tracing::debug!(
                        repo = %repo_slug, file = %composer_file_path,
                        total = deps.len(), actionable = actionable.len(),
                        "extracted composer dependencies"
                    );
                    let dep_inputs: Vec<packagist_datasource::PackagistDepInput> = actionable
                        .iter()
                        .map(|d| packagist_datasource::PackagistDepInput {
                            package_name: d.name.clone(),
                            current_value: d.current_value.clone(),
                        })
                        .collect();
                    let updates = packagist_datasource::fetch_updates_concurrent(
                        http,
                        &dep_inputs,
                        packagist_datasource::PACKAGIST_API,
                        10,
                    )
                    .await;
                    let update_map: HashMap<_, _> = updates
                        .into_iter()
                        .map(|r| (r.package_name, r.summary))
                        .collect();
                    ctx.report.files.push(output::FileReport {
                        path: composer_file_path.clone(),
                        manager: "composer".into(),
                        deps: build_dep_reports_composer(&deps, &actionable, &update_map),
                    });
                }
                Err(err) => {
                    tracing::warn!(repo=%repo_slug, file=%composer_file_path, %err,
                        "failed to parse composer.json")
                }
            },
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%composer_file_path, "composer.json not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%composer_file_path, %err,
                    "failed to fetch composer.json");
                ctx.had_error = true;
            }
        }
    }
}
