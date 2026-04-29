//! Conan C++ package manager (conanfile.txt / conanfile.py).

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
    let _gh_api_base = github_tags_datasource::api_base_from_endpoint(config.endpoint.as_deref());
    let gh_http = if let Some(ref token) = config.token {
        HttpClient::with_token(token).unwrap_or_else(|_| http.clone())
    } else {
        http.clone()
    };

    // ── Conan (conanfile.txt / conanfile.py) ─────────────────────────────────
    for conan_path in manager_files(detected, "conan") {
        match client.get_raw_file(owner, repo, &conan_path).await {
            Ok(Some(raw)) => {
                let deps = if conan_path.ends_with(".py") {
                    renovate_core::extractors::conan::extract_py(&raw.content)
                } else {
                    renovate_core::extractors::conan::extract_txt(&raw.content)
                };
                let actionable_count = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.name, "conan")
                    })
                    .count();
                tracing::debug!(
                    repo = %repo_slug, file = %conan_path,
                    total = deps.len(), actionable = actionable_count,
                    "extracted conan deps"
                );
                let mut file_deps: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    if let Some(reason) = &dep.skip_reason {
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
                            name: dep.name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored_for_manager(&dep.name, "conan") {
                        continue;
                    }
                    let status = match renovate_core::datasources::conan::fetch_latest(
                        &gh_http,
                        &dep.name,
                        &dep.current_value,
                    )
                    .await
                    {
                        Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                            current: dep.current_value.clone(),
                            latest: s.latest.unwrap_or_default(),
                        },
                        Ok(s) => output::DepStatus::UpToDate { latest: s.latest },
                        Err(renovate_core::datasources::conan::ConanError::NotFound(_)) => {
                            output::DepStatus::Skipped {
                                reason: "package not found in conan-center-index".into(),
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
                        name: dep.name.clone(),
                        status,
                    });
                }
                ctx.report.files.push(output::FileReport {
                    path: conan_path.clone(),
                    manager: "conan".into(),
                    deps: file_deps,
                });
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%conan_path, "conanfile not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%conan_path, %err, "failed to fetch conanfile");
                ctx.had_error = true;
            }
        }
    }

    // ── Haskell Cabal (*.cabal) ───────────────────────────────────────────────
}
