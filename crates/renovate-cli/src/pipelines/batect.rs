//! Batect build tool manager (batect.yml and batect wrapper script).

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

    // ── Batect (batect.yml / batect-bundle.yml) ───────────────────────────────
    for batect_path in manager_files(detected, "batect") {
        match client.get_raw_file(owner, repo, &batect_path).await {
            Ok(Some(raw)) => {
                let deps = renovate_core::extractors::batect::extract(&raw.content);
                tracing::debug!(
                    repo = %repo_slug, file = %batect_path,
                    total = deps.len(), "extracted batect images"
                );
                ctx.report.files.push(output::FileReport {
                    path: batect_path.clone(),
                    manager: "batect".into(),
                    deps: docker_hub_reports(http, &deps).await,
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%batect_path, "batect config not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%batect_path, %err, "failed to fetch batect config");
                ctx.had_error = true;
            }
        }
    }

    // ── Batect wrapper script (`batect`) ─────────────────────────────────────
    for bw_path in manager_files(detected, "batect-wrapper") {
        match client.get_raw_file(owner, repo, &bw_path).await {
            Ok(Some(raw)) => {
                if let Some(dep) = renovate_core::extractors::batect_wrapper::extract(&raw.content)
                {
                    tracing::debug!(
                        repo = %repo_slug, file = %bw_path,
                        version = %dep.version, "extracted batect wrapper version"
                    );
                    let release_result = github_releases_datasource::fetch_latest_release(
                        renovate_core::extractors::batect_wrapper::BATECT_REPO,
                        &gh_http,
                        gh_api_base,
                    )
                    .await;
                    let release_timestamp = release_result
                        .as_ref()
                        .ok()
                        .and_then(|r| r.as_ref())
                        .and_then(|(_, ts)| ts.clone());
                    let status = match release_result {
                        Ok(Some((latest, _))) if latest != dep.version => {
                            output::DepStatus::UpdateAvailable {
                                current: dep.version.clone(),
                                latest,
                            }
                        }
                        Ok(Some((latest, _))) => output::DepStatus::UpToDate {
                            latest: Some(latest),
                        },
                        Ok(None) => output::DepStatus::UpToDate { latest: None },
                        Err(e) => output::DepStatus::LookupError {
                            message: e.to_string(),
                        },
                    };
                    ctx.report.files.push(output::FileReport {
                        path: bw_path.clone(),
                        manager: "batect-wrapper".into(),
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
                            release_timestamp,
                            current_version_timestamp: None,

                            dep_type: None,
                            package_name: None,
                            dependency_dashboard_approval: None,
                            replacement_name: None,
                            replacement_version: None,
                            name: renovate_core::extractors::batect_wrapper::BATECT_REPO.to_owned(),
                            status,
                        }],
                    });
                }
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%bw_path, "batect wrapper script not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%bw_path, %err, "failed to fetch batect wrapper");
                ctx.had_error = true;
            }
        }
    }

    for unity_path in manager_files(detected, "unity3d") {
        match client.get_raw_file(owner, repo, &unity_path).await {
            Ok(Some(raw)) => {
                use renovate_core::extractors::unity3d::Unity3dVersionKind;
                let Some(dep) = renovate_core::extractors::unity3d::extract(&raw.content) else {
                    continue;
                };
                let with_revision = dep.kind == Unity3dVersionKind::WithRevision;
                let status = match renovate_core::datasources::unity3d::fetch_latest_lts(
                    http,
                    with_revision,
                )
                .await
                {
                    Ok(s) => {
                        let latest_str = if with_revision {
                            s.latest_with_revision.clone()
                        } else {
                            s.latest.clone()
                        };
                        match latest_str {
                            Some(latest) if latest != dep.current_value => {
                                output::DepStatus::UpdateAvailable {
                                    current: dep.current_value.clone(),
                                    latest,
                                }
                            }
                            Some(latest) => output::DepStatus::UpToDate {
                                latest: Some(latest),
                            },
                            None => output::DepStatus::UpToDate { latest: None },
                        }
                    }
                    Err(e) => output::DepStatus::LookupError {
                        message: e.to_string(),
                    },
                };
                ctx.report.files.push(output::FileReport {
                    path: unity_path.clone(),
                    manager: "unity3d".into(),
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
                        dependency_dashboard_approval: None,
                        replacement_name: None,
                        replacement_version: None,
                        name: "Unity Editor".to_owned(),
                        status,
                    }],
                });
            }
            Ok(None) => {
                tracing::warn!(repo=%repo_slug, file=%unity_path, "ProjectVersion.txt not found")
            }
            Err(err) => {
                tracing::error!(repo=%repo_slug, file=%unity_path, %err, "failed to fetch ProjectVersion.txt");
                ctx.had_error = true;
            }
        }
    }
}
