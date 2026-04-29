//! JavaScript/Node managers: npm, Bun, Meteor, HTML CDN, and CDN URLs.

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

    // ── npm ───────────────────────────────────────────────────────────────────
    // Two-pass: collect unique packages across all package.json files, fetch
    // versions once per unique name, then build per-file reports.  This avoids
    // redundant registry calls in monorepos where packages appear in multiple
    // workspaces.
    {
        let npm_files = manager_files(detected, "npm");
        // Pass 1: fetch files and extract deps.
        let mut npm_file_deps: Vec<(String, Vec<renovate_core::extractors::npm::NpmExtractedDep>)> =
            Vec::new();
        for npm_file_path in &npm_files {
            match client.get_raw_file(owner, repo, npm_file_path).await {
                Ok(Some(raw)) => match npm_extractor::extract(&raw.content) {
                    Ok(deps) => npm_file_deps.push((npm_file_path.clone(), deps)),
                    Err(err) => {
                        tracing::warn!(repo=%repo_slug, file=%npm_file_path, %err,
                            "failed to parse package.json")
                    }
                },
                Ok(None) => {
                    tracing::warn!(repo=%repo_slug, file=%npm_file_path, "package.json not found")
                }
                Err(err) => {
                    tracing::error!(repo=%repo_slug, file=%npm_file_path, %err,
                        "failed to fetch package.json");
                    ctx.had_error = true;
                }
            }
        }
        // Collect unique actionable package names.
        let unique_names: Vec<String> = {
            let mut seen = std::collections::HashSet::new();
            npm_file_deps
                .iter()
                .flat_map(|(_, deps)| deps.iter())
                .filter(|d| {
                    d.skip_reason.is_none()
                        && !repo_cfg.is_dep_ignored_with_dep_type(
                            &d.name,
                            "npm",
                            d.dep_type.as_renovate_str(),
                        )
                })
                .filter(|d| seen.insert(d.name.clone()))
                .map(|d| d.name.clone())
                .collect()
        };
        tracing::debug!(
            repo = %repo_slug,
            files = npm_file_deps.len(),
            unique_packages = unique_names.len(),
            "fetching npm versions (deduplicated)"
        );
        // Pass 2: fetch versions for all unique packages at once.
        let versions_cache = npm_datasource::fetch_versions_batch(
            http,
            &unique_names,
            npm_datasource::NPM_REGISTRY,
            10,
        )
        .await;
        // Build a per-package version-timestamp map from the cache for matchCurrentAge.
        let npm_version_ts: HashMap<String, HashMap<String, String>> = versions_cache
            .iter()
            .map(|(name, entry)| (name.clone(), entry.version_timestamps.clone()))
            .collect();
        // Pass 3: build per-file reports.
        for (npm_file_path, deps) in npm_file_deps {
            let actionable: Vec<_> = deps
                .iter()
                .filter(|d| {
                    d.skip_reason.is_none()
                        && !repo_cfg.is_dep_ignored_with_dep_type(
                            &d.name,
                            "npm",
                            d.dep_type.as_renovate_str(),
                        )
                })
                .collect();
            let update_map: HashMap<
                _,
                Result<renovate_core::versioning::npm::NpmUpdateSummary, _>,
            > = actionable
                .iter()
                .map(|d| {
                    let summary = versions_cache
                        .get(&d.name)
                        .map(|entry| npm_datasource::summary_from_cache(&d.current_value, entry))
                        .ok_or(npm_datasource::NpmError::NotFound(d.name.clone()));
                    (d.name.clone(), summary)
                })
                .collect();
            ctx.report.files.push(output::FileReport {
                path: npm_file_path.clone(),
                manager: "npm".into(),
                deps: build_dep_reports_npm(&deps, &actionable, &update_map, &npm_version_ts),
            });
        }
    }

    // ── Bun (bun.lockb / bun.lock) → package.json ────────────────────────────
    for bun_lockfile_path in manager_files(detected, "bun") {
        // Derive sibling package.json path from the lockfile path.
        let package_json_path = if let Some(dir) = bun_lockfile_path.rfind('/') {
            format!("{}/package.json", &bun_lockfile_path[..dir])
        } else {
            "package.json".to_owned()
        };
        match client.get_raw_file(owner, repo, &package_json_path).await {
            Ok(Some(raw)) => match npm_extractor::extract(&raw.content) {
                Ok(deps) => {
                    let actionable: Vec<_> = deps
                        .iter()
                        .filter(|d| {
                            d.skip_reason.is_none()
                                && !repo_cfg.is_dep_ignored_for_manager(&d.name, "bun")
                        })
                        .collect();
                    tracing::debug!(
                        repo = %repo_slug, file = %bun_lockfile_path,
                        package_json = %package_json_path,
                        total = deps.len(), actionable = actionable.len(),
                        "extracted bun dependencies from package.json"
                    );
                    let dep_inputs: Vec<npm_datasource::NpmDepInput> = actionable
                        .iter()
                        .map(|d| npm_datasource::NpmDepInput {
                            dep_name: d.name.clone(),
                            constraint: d.current_value.clone(),
                        })
                        .collect();
                    let updates = npm_datasource::fetch_updates_concurrent(
                        http,
                        &dep_inputs,
                        npm_datasource::NPM_REGISTRY,
                        10,
                    )
                    .await;
                    let update_map: HashMap<_, _> = updates
                        .into_iter()
                        .map(|r| (r.dep_name, r.summary))
                        .collect();
                    ctx.report.files.push(output::FileReport {
                        path: package_json_path.clone(),
                        manager: "bun".into(),
                        // Bun uses concurrent fetch (no versions_cache here) so
                        // current_version_timestamp is not available in this path.
                        deps: build_dep_reports_npm(
                            &deps,
                            &actionable,
                            &update_map,
                            &HashMap::new(),
                        ),
                    });
                }
                Err(err) => {
                    tracing::warn!(repo=%repo_slug, file=%package_json_path, %err, "failed to parse package.json for bun")
                }
            },
            Ok(None) => {
                tracing::debug!(repo=%repo_slug, file=%package_json_path, "no package.json sibling for bun lockfile")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%bun_lockfile_path, %err, "failed to fetch bun package.json");
                ctx.had_error = true;
            }
        }
    }

    // ── Meteor (package.js Npm.depends) ──────────────────────────────────────
    for meteor_path in manager_files(detected, "meteor") {
        match client.get_raw_file(owner, repo, &meteor_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::meteor::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| !repo_cfg.is_dep_ignored_for_manager(&d.name, "meteor"))
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %meteor_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted meteor npm deps"
                );
                let npm_inputs: Vec<npm_datasource::NpmDepInput> = actionable
                    .iter()
                    .map(|d| npm_datasource::NpmDepInput {
                        dep_name: d.name.clone(),
                        constraint: d.current_value.clone(),
                    })
                    .collect();
                let npm_updates = npm_datasource::fetch_updates_concurrent(
                    http,
                    &npm_inputs,
                    npm_datasource::NPM_REGISTRY,
                    8,
                )
                .await;
                let update_map: HashMap<_, _> = npm_updates
                    .into_iter()
                    .map(|r| (r.dep_name, r.summary))
                    .collect();
                let file_deps: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        let status = match update_map.get(&dep.name) {
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
                            name: dep.name.clone(),
                            status,
                        }
                    })
                    .collect();
                ctx.report.files.push(output::FileReport {
                    path: meteor_path.clone(),
                    manager: "meteor".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%meteor_path, "meteor package.js not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%meteor_path, %err, "failed to fetch meteor package.js");
                ctx.had_error = true;
            }
        }
    }

    // ── HTML (cdnjs) ─────────────────────────────────────────────────────────
    for html_path in manager_files(detected, "html") {
        match client.get_raw_file(owner, repo, &html_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::html::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %html_path,
                    total = deps.len(),
                    "extracted HTML cdnjs dependencies"
                );
                let mut dep_reports = Vec::new();
                for dep in &deps {
                    if repo_cfg.is_dep_ignored_for_manager(&dep.dep_name, "html") {
                        continue;
                    }
                    let library = &dep.dep_name;
                    let status = match renovate_core::datasources::cdnjs::fetch_latest(
                        http,
                        library,
                        &dep.current_value,
                    )
                    .await
                    {
                        Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                            current: s.current_value.clone(),
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
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: html_path.clone(),
                        manager: "html".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%html_path, "HTML file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%html_path, %err, "failed to fetch HTML file");
                ctx.had_error = true;
            }
        }
    }

    // ── cdnurl (user-configured Cloudflare CDN URLs) ──────────────────────────
    // Reuses the html extractor; upstream differs only in that it skips SRI hash
    // updates (which we don't do anyway) and has empty default patterns.
    for cdnurl_path in manager_files(detected, "cdnurl") {
        match client.get_raw_file(owner, repo, &cdnurl_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::html::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %cdnurl_path,
                    total = deps.len(),
                    "extracted cdnurl cdnjs dependencies"
                );
                let mut dep_reports = Vec::new();
                for dep in &deps {
                    if repo_cfg.is_dep_ignored_for_manager(&dep.dep_name, "cdnurl") {
                        continue;
                    }
                    let status = match renovate_core::datasources::cdnjs::fetch_latest(
                        http,
                        &dep.dep_name,
                        &dep.current_value,
                    )
                    .await
                    {
                        Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                            current: s.current_value.clone(),
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
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: cdnurl_path.clone(),
                        manager: "cdnurl".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%cdnurl_path, "cdnurl file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%cdnurl_path, %err, "failed to fetch cdnurl file");
                ctx.had_error = true;
            }
        }
    }
}
