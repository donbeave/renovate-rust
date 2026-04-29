//! Puppet manager (Puppetfile).

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

    // ── Puppet (Puppetfile) ───────────────────────────────────────────────────
    for pf_path in manager_files(detected, "puppet") {
        match client.get_raw_file(owner, repo, &pf_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::puppet::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %pf_path,
                    total = deps.len(),
                    "extracted puppet deps"
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
                            name: dep.name.clone(),
                            status: output::DepStatus::Skipped {
                                reason: format!("{reason:?}").to_lowercase(),
                            },
                        });
                        continue;
                    }
                    if repo_cfg.is_dep_ignored_for_manager(&dep.name, "puppet") {
                        continue;
                    }

                    use renovate_core::extractors::puppet::PuppetSource;
                    let status = match &dep.source {
                        PuppetSource::PuppetForge { forge_url } => {
                            let registry = forge_url.as_deref().unwrap_or("");
                            match renovate_core::datasources::puppet_forge::fetch_latest(
                                http,
                                &dep.name,
                                &dep.current_value,
                                registry,
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
                            }
                        }
                        PuppetSource::GitHub(gh_repo) => {
                            let tag_result =
                                renovate_core::datasources::github_tags::fetch_latest_tag(
                                    gh_repo,
                                    &gh_http,
                                    gh_api_base,
                                )
                                .await
                                .map_err(|e| e.to_string());
                            match tag_result {
                                Ok(Some(tag)) => {
                                    let stripped = tag.trim_start_matches('v');
                                    let clean = dep.current_value.trim_start_matches('v');
                                    let s = renovate_core::versioning::semver_generic::semver_update_summary(clean, Some(stripped));
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
                        PuppetSource::Git(_) => output::DepStatus::Skipped {
                            reason: "non-github-git".into(),
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
                        name: dep.name.clone(),
                        status,
                    });
                }
                if !dep_reports.is_empty() {
                    ctx.report.files.push(output::FileReport {
                        path: pf_path.clone(),
                        manager: "puppet".into(),
                        deps: dep_reports,
                    });
                }
            }
            Ok(None) => tracing::warn!(repo=%repo_slug, file=%pf_path, "Puppetfile not found"),
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%pf_path, %err, "failed to fetch Puppetfile");
                ctx.had_error = true;
            }
        }
    }

}
