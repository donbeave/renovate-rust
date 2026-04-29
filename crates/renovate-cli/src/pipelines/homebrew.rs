//! Homebrew formula manager (Formula/*.rb).

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

    // ── Homebrew formula (Formula/*.rb) ────────────────────────────────────────
    for hb_path in manager_files(detected, "homebrew") {
        match client.get_raw_file(owner, repo, &hb_path).await {
            Ok(Some(raw)) => {
                use homebrew_extractor::{GitHubUrlType, HomebrewSkipReason, HomebrewSource};
                let Some(dep) = homebrew_extractor::extract(&raw.content) else {
                    continue;
                };
                if repo_cfg.is_dep_ignored_for_manager(&dep.formula_name, "homebrew") {
                    continue;
                }
                let status = if let Some(reason) = &dep.skip_reason {
                    output::DepStatus::Skipped {
                        reason: match reason {
                            HomebrewSkipReason::InvalidSha256 => "invalid-sha256".to_owned(),
                            HomebrewSkipReason::UnsupportedUrl => "unsupported-url".to_owned(),
                            HomebrewSkipReason::MissingUrl => "missing-url".to_owned(),
                        },
                    }
                } else {
                    match &dep.source {
                        HomebrewSource::GitHub {
                            repo: gh_repo,
                            url_type,
                        } => {
                            let result = match url_type {
                                GitHubUrlType::Archive => {
                                    renovate_core::datasources::github_tags::fetch_latest_tag(
                                        gh_repo,
                                        &gh_http,
                                        gh_api_base,
                                    )
                                    .await
                                    .map_err(|e| e.to_string())
                                }
                                GitHubUrlType::Release => {
                                    github_releases_datasource::fetch_latest_release(
                                        gh_repo,
                                        &gh_http,
                                        gh_api_base,
                                    )
                                    .await
                                    .map(|r| r.map(|(tag, _)| tag))
                                    .map_err(|e| e.to_string())
                                }
                            };
                            match result {
                                Ok(Some(tag)) => {
                                    let stripped = tag.trim_start_matches('v');
                                    let s = renovate_core::versioning::semver_generic::semver_update_summary(
                                        &dep.current_value,
                                        Some(stripped),
                                    );
                                    if s.update_available {
                                        output::DepStatus::UpdateAvailable {
                                            current: dep.current_value.clone(),
                                            latest: stripped.to_owned(),
                                        }
                                    } else {
                                        output::DepStatus::UpToDate {
                                            latest: Some(stripped.to_owned()),
                                        }
                                    }
                                }
                                Ok(None) => output::DepStatus::UpToDate { latest: None },
                                Err(e) => output::DepStatus::LookupError { message: e },
                            }
                        }
                        HomebrewSource::Npm { package } => {
                            let npm_input = vec![npm_datasource::NpmDepInput {
                                dep_name: package.clone(),
                                constraint: dep.current_value.clone(),
                            }];
                            let mut updates = npm_datasource::fetch_updates_concurrent(
                                http,
                                &npm_input,
                                npm_datasource::NPM_REGISTRY,
                                1,
                            )
                            .await;
                            match updates.pop().map(|r| r.summary) {
                                Some(Ok(s)) if s.update_available => {
                                    output::DepStatus::UpdateAvailable {
                                        current: dep.current_value.clone(),
                                        latest: s.latest.unwrap_or_default(),
                                    }
                                }
                                Some(Ok(s)) => output::DepStatus::UpToDate { latest: s.latest },
                                Some(Err(e)) => output::DepStatus::LookupError {
                                    message: e.to_string(),
                                },
                                None => output::DepStatus::UpToDate { latest: None },
                            }
                        }
                        HomebrewSource::Unsupported(_) => output::DepStatus::Skipped {
                            reason: "unsupported-url".to_owned(),
                        },
                    }
                };
                ctx.report.files.push(output::FileReport {
                    path: hb_path.clone(),
                    manager: "homebrew".into(),
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
                        range_strategy: None,
                        follow_tag: None,
                        pin_digests: None,
                        versioning: None,
                        dependency_dashboard_approval: None,
                        replacement_name: None,
                        replacement_version: None,
                        name: dep.formula_name.clone(),
                        status,
                    }],
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%hb_path, "homebrew formula not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%hb_path, %err, "failed to fetch homebrew formula");
                ctx.had_error = true;
            }
        }
    }

    // ── Unity3D ProjectVersion.txt ─────────────────────────────────────────────
}
