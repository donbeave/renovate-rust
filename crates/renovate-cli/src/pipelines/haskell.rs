//! Haskell Cabal manager (*.cabal files).

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

    // ── Haskell Cabal (*.cabal) ───────────────────────────────────────────────
    for cabal_path in manager_files(detected, "haskell-cabal") {
        match client.get_raw_file(owner, repo, &cabal_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::cabal::extract(&raw.content);
                let actionable_count = deps
                    .iter()
                    .filter(|d| {
                        !repo_cfg.is_dep_ignored_for_manager(&d.package_name, "haskell-cabal")
                    })
                    .count();
                tracing::debug!(
                    repo = %repo_slug, file = %cabal_path,
                    total = deps.len(), actionable = actionable_count,
                    "extracted cabal deps"
                );
                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    if repo_cfg.is_dep_ignored_for_manager(&dep.package_name, "haskell-cabal") {
                        continue;
                    }
                    let status = match renovate_core::datasources::hackage::fetch_latest(
                        http,
                        &dep.package_name,
                    )
                    .await
                    {
                        Ok(s) => {
                            if let Some(ref l) = s.latest {
                                // Compare latest against the constraint if it's a plain version.
                                let current_ver =
                                    dep.current_value.trim_start_matches("==").trim().to_owned();
                                if !current_ver.is_empty()
                                    && !current_ver.contains(['<', '>', '&'])
                                    && l != &current_ver
                                {
                                    output::DepStatus::UpdateAvailable {
                                        current: current_ver,
                                        latest: l.clone(),
                                    }
                                } else {
                                    output::DepStatus::UpToDate {
                                        latest: s.latest.clone(),
                                    }
                                }
                            } else {
                                output::DepStatus::UpToDate { latest: None }
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
                        pr_priority: None,
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,

                        dep_type: None,
                        package_name: None,
                        dependency_dashboard_approval: None,
                        replacement_name: None,
                        replacement_version: None,
                        name: dep.package_name.clone(),
                        status,
                    });
                }
                ctx.report.files.push(output::FileReport {
                    path: cabal_path.clone(),
                    manager: "haskell-cabal".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%cabal_path, "cabal file not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%cabal_path, %err, "failed to fetch cabal file");
                ctx.had_error = true;
            }
        }
    }

    // ── Jsonnet Bundler (jsonnetfile.json) ───────────────────────────────────
}
