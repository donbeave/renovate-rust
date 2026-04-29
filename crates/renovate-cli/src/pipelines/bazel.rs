//! Bazel managers: Bazel Module (MODULE.bazel) and Bazel WORKSPACE.

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

    // ── Bazel Module (MODULE.bazel) ───────────────────────────────────────────
    for bm_path in manager_files(detected, "bazel-module") {
        match client.get_raw_file(owner, repo, &bm_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::bazel_module::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %bm_path,
                    total = deps.len(),
                    "extracted bazel module deps"
                );
                let mut dep_reports = Vec::new();
                for dep in &deps {
                    if let Some(reason) = &dep.skip_reason {
                        dep_reports.push(output::DepReport {
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
                            name: dep.name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored_for_manager(&dep.name, "bazel-module") {
                        continue;
                    }
                    let status = match renovate_core::datasources::bazel::fetch_latest(
                        http,
                        &dep.name,
                        &dep.current_value,
                    )
                    .await
                    {
                        Ok(s) if s.update_available => output::DepStatus::UpdateAvailable {
                            current: s.current_value,
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
                        pr_priority: None,
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,

                        dep_type: None,
                        package_name: None,
                        dependency_dashboard_approval: None,
                        replacement_name: None,
                        replacement_version: None,
                        name: dep.name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: bm_path.clone(),
                        manager: "bazel-module".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%bm_path, "MODULE.bazel not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%bm_path, %err, "failed to fetch MODULE.bazel");
                ctx.had_error = true;
            }
        }
    }

    // ── Bazel WORKSPACE / .bzl http_archive() ────────────────────────────────
    for bazel_path in manager_files(detected, "bazel") {
        match client.get_raw_file(owner, repo, &bazel_path).await {
            Ok(Some(raw)) => {
                use renovate_core::extractors::bazel::{BazelSkipReason, BazelSource};
                let deps = renovate_core::extractors::bazel::extract(&raw.content);
                let actionable_count = deps
                    .iter()
                    .filter(|d| {
                        d.skip_reason.is_none()
                            && !repo_cfg.is_dep_ignored_for_manager(&d.dep_name, "bazel")
                    })
                    .count();
                tracing::debug!(
                    repo = %repo_slug, file = %bazel_path,
                    total = deps.len(), actionable = actionable_count,
                    "extracted bazel http_archive deps"
                );
                let mut dep_reports: Vec<output::DepReport> = Vec::new();
                for dep in &deps {
                    if let Some(reason) = &dep.skip_reason {
                        dep_reports.push(output::DepReport {
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
                            name: dep.dep_name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: match reason {
                                    BazelSkipReason::NoGithubUrl => "no-github-url".to_owned(),
                                    BazelSkipReason::MissingSha256 => "missing-sha256".to_owned(),
                                },
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored_for_manager(&dep.dep_name, "bazel") {
                        continue;
                    }
                    let status = match &dep.source {
                        BazelSource::GithubTags { repo: gh_repo } => {
                            match renovate_core::datasources::github_tags::fetch_latest_tag(
                                gh_repo,
                                &gh_http,
                                gh_api_base,
                            )
                            .await
                            {
                                Ok(Some(tag)) => {
                                    let stripped = tag.trim_start_matches('v');
                                    let s = renovate_core::versioning::semver_generic::semver_update_summary(
                                        &dep.current_value, Some(stripped),
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
                                Err(e) => output::DepStatus::LookupError {
                                    message: e.to_string(),
                                },
                            }
                        }
                        BazelSource::GithubReleases { repo: gh_repo } => {
                            match github_releases_datasource::fetch_latest_release(
                                gh_repo,
                                &gh_http,
                                gh_api_base,
                            )
                            .await
                            {
                                Ok(Some((tag, _))) => {
                                    let stripped = tag.trim_start_matches('v');
                                    let s = renovate_core::versioning::semver_generic::semver_update_summary(
                                        &dep.current_value, Some(stripped),
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
                                Err(e) => output::DepStatus::LookupError {
                                    message: e.to_string(),
                                },
                            }
                        }
                        BazelSource::Unsupported => output::DepStatus::Skipped {
                            reason: "no-github-url".to_owned(),
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
                        pr_priority: None,
                        pr_title: None,
                        release_timestamp: None,
                        current_version_timestamp: None,

                        dep_type: None,
                        package_name: None,
                        dependency_dashboard_approval: None,
                        replacement_name: None,
                        replacement_version: None,
                        name: dep.dep_name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: bazel_path.clone(),
                        manager: "bazel".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%bazel_path, "bazel file not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%bazel_path, %err, "failed to fetch bazel file");
                ctx.had_error = true;
            }
        }
    }
}
