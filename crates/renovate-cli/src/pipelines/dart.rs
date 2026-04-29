//! Dart/Flutter (`pubspec.yaml`) and FVM version manager pipelines.

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

    // ── Dart/Flutter pub (pubspec.yaml) ───────────────────────────────────────
    for pub_file_path in manager_files(detected, "pub") {
        match client.get_raw_file(owner, repo, &pub_file_path).await {
            Ok(Some(raw)) => {
                let deps = pubspec_extractor::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.name, "pub")
                    })
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %pub_file_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted pub dependencies"
                );
                let dep_inputs: Vec<pub_datasource::PubDepInput> = actionable
                    .iter()
                    .map(|d| pub_datasource::PubDepInput {
                        name: d.name.clone(),
                        current_value: d.current_value.clone(),
                    })
                    .collect();
                let updates = pub_datasource::fetch_updates_concurrent(
                    http,
                    &dep_inputs,
                    pub_datasource::PUB_DEV_API,
                    10,
                )
                .await;
                let update_map: HashMap<_, _> =
                    updates.into_iter().map(|r| (r.name, r.summary)).collect();
                ctx.report.files.push(output::FileReport {
                    path: pub_file_path.clone(),
                    manager: "pub".into(),
                    deps: build_dep_reports_pub(&deps, &actionable, &update_map),
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%pub_file_path, "pubspec.yaml not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%pub_file_path, %err,
                    "failed to fetch pubspec.yaml");
                ctx.had_error = true;
            }
        }
    }

    // ── FVM Flutter Version Manager (.fvmrc / .fvm/fvm_config.json) ────────
    for fvm_path in manager_files(detected, "fvm") {
        match client.get_raw_file(owner, repo, &fvm_path).await {
            Ok(Some(raw)) => {
                if let Some(dep) = renovate_core::extractors::fvm::extract(&raw.content) {
                    tracing::debug!(
                        repo = %repo_slug, file = %fvm_path,
                        version = %dep.version, "extracted fvm flutter version"
                    );
                    let status = match github_tags_datasource::fetch_latest_tag(
                        "flutter/flutter",
                        &gh_http,
                        gh_api_base,
                    )
                    .await
                    {
                        Ok(Some(latest)) if latest != dep.version => {
                            output::DepStatus::UpdateAvailable {
                                current: dep.version.clone(),
                                latest,
                            }
                        }
                        Ok(Some(latest)) => output::DepStatus::UpToDate {
                            latest: Some(latest),
                        },
                        Ok(None) => output::DepStatus::UpToDate { latest: None },
                        Err(e) => output::DepStatus::LookupError {
                            message: e.to_string(),
                        },
                    };
                    ctx.report.files.push(output::FileReport {
                        path: fvm_path.clone(),
                        manager: "fvm".into(),
                        deps: vec![output::DepReport {
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
                            name: "flutter".to_owned(),
                            status,
                        }],
                    });
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%fvm_path, "fvm config not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%fvm_path, %err, "failed to fetch fvm config");
                ctx.had_error = true;
            }
        }
    }
}
