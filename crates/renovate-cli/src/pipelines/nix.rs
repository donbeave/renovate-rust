//! Nix flakes manager (flake.nix / flake.lock).

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

    // ── Nix flakes (flake.nix / flake.lock) ──────────────────────────────────
    for flake_path in manager_files(detected, "nix") {
        // flake.nix is the trigger file; actual data lives in sibling flake.lock.
        let lock_path = {
            let p = std::path::Path::new(&flake_path);
            let dir = p.parent().map(|d| d.to_str().unwrap_or("")).unwrap_or("");
            if dir.is_empty() {
                "flake.lock".to_owned()
            } else {
                format!("{dir}/flake.lock")
            }
        };
        match client.get_raw_file(owner, repo, &lock_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::nix::extract(&raw.content);
                let actionable: Vec<_> = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.input_name, "nix")
                    })
                    .collect();
                tracing::debug!(
                    repo = %repo_slug, file = %flake_path,
                    total = deps.len(), actionable = actionable.len(),
                    "extracted nix flake inputs"
                );
                // Build GitHub Tags lookups for github-type inputs.
                let gh_inputs: Vec<github_tags_datasource::GithubActionsDepInput> = actionable
                    .iter()
                    .filter(|d| {
                        matches!(
                            d.input_type,
                            renovate_core::extractors::nix::FlakeInputType::Github
                        )
                    })
                    .filter_map(|d| {
                        // package_name is "https://github.com/owner/repo"
                        let pkg = d.package_name.as_deref()?;
                        let repo_path = pkg
                            .strip_prefix("https://github.com/")
                            .unwrap_or(pkg)
                            .to_owned();
                        Some(github_tags_datasource::GithubActionsDepInput {
                            dep_name: repo_path,
                            current_value: d.current_ref.clone().unwrap_or_default(),
                        })
                    })
                    .collect();
                let gh_updates = github_tags_datasource::fetch_updates_concurrent(
                    &gh_http,
                    &gh_inputs,
                    gh_api_base,
                    8,
                )
                .await;
                let gh_map: HashMap<String, (bool, Option<String>, Option<String>)> = {
                    let mut m = HashMap::new();
                    for r in gh_updates {
                        match r.summary {
                            Ok(s) => {
                                m.insert(r.dep_name, (s.update_available, s.latest, None));
                            }
                            Err(e) => {
                                m.insert(r.dep_name, (false, None, Some(e.to_string())));
                            }
                        }
                    }
                    m
                };
                let file_deps: Vec<output::DepReport> = deps
                    .iter()
                    .map(|dep| {
                        let status = if let Some(reason) = &dep.skip_reason {
                            output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            }
                        } else if matches!(
                            dep.input_type,
                            renovate_core::extractors::nix::FlakeInputType::Github
                        ) {
                            let pkg = dep
                                .package_name
                                .as_deref()
                                .unwrap_or("")
                                .strip_prefix("https://github.com/")
                                .unwrap_or("")
                                .to_owned();
                            match gh_map.get(&pkg) {
                                Some((true, Some(latest), _)) => {
                                    output::DepStatus::UpdateAvailable {
                                        current: dep.locked_rev.clone(),
                                        latest: latest.clone(),
                                    }
                                }
                                Some((_, latest, None)) => output::DepStatus::UpToDate {
                                    latest: latest.clone(),
                                },
                                Some((_, _, Some(err))) => output::DepStatus::LookupError {
                                    message: err.clone(),
                                },
                                None => output::DepStatus::UpToDate { latest: None },
                            }
                        } else {
                            output::DepStatus::Skipped {
                                reason: "non-github flake input (datasource pending)".into(),
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
                            name: dep.input_name.clone(),
                            status,
                        }
                    })
                    .collect();
                ctx.report.files.push(output::FileReport {
                    path: flake_path.clone(),
                    manager: "nix".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => {
                tracing::debug!(repo=%repo_slug, file=%flake_path, "flake.lock not found (no lock file)")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%flake_path, %err, "failed to fetch flake.lock");
                ctx.had_error = true;
            }
        }
    }
}
