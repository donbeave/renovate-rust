//! Heroku/Render runtime.txt Python version manager.

use super::*;

pub(crate) async fn process(ctx: &mut RepoPipelineCtx<'_>) {
    let client = ctx.client;
    let http = ctx.http;
    let config = ctx.config;
    let owner = ctx.owner;
    let repo = ctx.repo;
    let repo_slug = ctx.repo_slug;
    let repo_cfg = ctx.repo_cfg;
    let _ = repo_cfg;
    let detected = ctx.detected;
    let gh_api_base = github_tags_datasource::api_base_from_endpoint(config.endpoint.as_deref());
    let gh_http = if let Some(ref token) = config.token {
        HttpClient::with_token(token).unwrap_or_else(|_| http.clone())
    } else {
        http.clone()
    };

    // ── Heroku/Render runtime.txt (Python version) ────────────────────────────
    for rt_path in manager_files(detected, "runtime-version") {
        match client.get_raw_file(owner, repo, &rt_path).await {
            Ok(Some(raw)) => {
                let Some(dep) = renovate_core::extractors::runtime_version::extract(&raw.content)
                else {
                    continue;
                };
                // Python CPython: GitHub Releases on python/cpython, tags like v3.11.0
                let status = match github_releases_datasource::fetch_latest_release(
                    "python/cpython",
                    &gh_http,
                    gh_api_base,
                )
                .await
                {
                    Ok(Some((tag, _))) => {
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
                    Err(e) => output::DepStatus::LookupError {
                        message: e.to_string(),
                    },
                };
                ctx.report.files.push(output::FileReport {
                    path: rt_path.clone(),
                    manager: "runtime-version".into(),
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
                        name: dep.dep_name.clone(),
                        status,
                    }],
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%rt_path, "runtime.txt not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%rt_path, %err, "failed to fetch runtime.txt");
                ctx.had_error = true;
            }
        }
    }
}
