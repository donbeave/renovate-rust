//! Copier template manager (.copier-answers.yml).

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

    // ── Copier (.copier-answers.yml) ─────────────────────────────────────────
    for copier_path in manager_files(detected, "copier") {
        match client.get_raw_file(owner, repo, &copier_path).await {
            Ok(Some(raw)) => {
                if let Some(dep) = renovate_core::extractors::copier::extract(&raw.content) {
                    tracing::debug!(
                        repo = %repo_slug, file = %copier_path,
                        src = %dep.src_path, version = %dep.current_value,
                        "extracted copier template dep"
                    );
                    let status = if !dep.github_repo.is_empty() {
                        match github_tags_datasource::fetch_latest_tag(
                            &dep.github_repo,
                            &gh_http,
                            gh_api_base,
                        )
                        .await
                        {
                            Ok(Some(latest)) if latest != dep.current_value => {
                                output::DepStatus::UpdateAvailable {
                                    current: dep.current_value.clone(),
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
                        }
                    } else {
                        output::DepStatus::Skipped {
                            reason: "non-github template source".into(),
                        }
                    };
                    ctx.report.files.push(output::FileReport {
                        path: copier_path.clone(),
                        manager: "copier".into(),
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
                            name: dep.src_path.clone(),
                            status,
                        }],
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%copier_path, "copier answers file not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%copier_path, %err, "failed to fetch copier answers file");
                ctx.had_error = true;
            }
        }
    }

}
