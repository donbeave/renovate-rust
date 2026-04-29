//! Custom manager (`customManagers`) pipeline.
//!
//! For each custom manager entry in `repo_cfg.custom_managers`:
//! 1. Find files in `filtered_files` that match the manager's `file_patterns`.
//! 2. Fetch each matching file's content.
//! 3. Apply `extract_deps()` to extract dep+datasource+version tuples.
//! 4. Look up the latest version using the dep's declared datasource.
//! 5. Build `DepReport` entries and attach them to the report.
//!
//! Only `customType: "regex"` with the `"any"` strategy is implemented.
//! JSONata and other strategies are silently skipped.
//!
//! Renovate reference:
//! `lib/modules/manager/custom/regex/` — extraction logic

use super::*;
use renovate_core::repo_config::CustomExtractedDep;

pub(crate) async fn process(ctx: &mut RepoPipelineCtx<'_>) {
    let repo_cfg = ctx.repo_cfg;
    if repo_cfg.custom_managers.is_empty() {
        return;
    }
    let client = ctx.client;
    let http = ctx.http;
    let owner = ctx.owner;
    let repo = ctx.repo;
    let repo_slug = ctx.repo_slug;
    let filtered_files = ctx.filtered_files;

    for cm in &repo_cfg.custom_managers {
        if cm.custom_type != "regex" {
            tracing::debug!(
                repo = %repo_slug,
                custom_type = %cm.custom_type,
                "customManagers: customType not supported; skipping"
            );
            continue;
        }
        if cm.match_strings.is_empty() {
            continue;
        }

        // Find files matching this custom manager's file patterns.
        let matching_files: Vec<&str> = filtered_files
            .iter()
            .filter(|f| cm.matches_file(f))
            .map(String::as_str)
            .collect();

        if matching_files.is_empty() {
            continue;
        }

        for file_path in matching_files {
            let content = match client.get_raw_file(owner, repo, file_path).await {
                Ok(Some(raw)) => raw.content,
                Ok(None) => {
                    tracing::debug!(repo = %repo_slug, file = %file_path, "customManagers: file not found");
                    continue;
                }
                Err(err) => {
                    tracing::warn!(repo = %repo_slug, file = %file_path, %err,
                        "customManagers: failed to fetch file");
                    continue;
                }
            };

            let extracted = cm.extract_deps(&content);
            if extracted.is_empty() {
                continue;
            }

            let deps = lookup_custom_deps(http, repo_slug, &extracted).await;
            if !deps.is_empty() {
                ctx.report.files.push(output::FileReport {
                    path: file_path.to_owned(),
                    manager: "custom-regex".to_owned(),
                    deps,
                });
            }
        }
    }
}

/// Look up the latest versions for a list of custom-extracted deps.
///
/// Dispatches to the appropriate datasource based on `dep.datasource`.
/// Unknown datasources are silently skipped with a debug log.
async fn lookup_custom_deps(
    http: &renovate_core::http::HttpClient,
    repo_slug: &str,
    extracted: &[CustomExtractedDep],
) -> Vec<output::DepReport> {
    let mut deps = Vec::new();
    for dep in extracted {
        let pkg = dep.package_name.as_deref().unwrap_or(dep.dep_name.as_str());
        let status = match dep.datasource.as_str() {
            "npm" => {
                match npm_datasource::fetch_versions(http, pkg, npm_datasource::NPM_REGISTRY).await
                {
                    Ok(entry) => {
                        if let Some(latest) = entry.latest_tag.as_deref() {
                            version_status(&dep.current_value, latest)
                        } else {
                            output::DepStatus::LookupError {
                                message: format!("npm: no 'latest' tag for '{pkg}'"),
                            }
                        }
                    }
                    Err(e) => output::DepStatus::LookupError {
                        message: format!("npm lookup failed: {e}"),
                    },
                }
            }
            "crate" => {
                use renovate_core::datasources::crates_io as crates_datasource;
                match crates_datasource::fetch_versions(http, pkg, crates_datasource::CRATES_IO_API)
                    .await
                {
                    Ok(records) => {
                        if let Some(latest) = records.iter().find(|r| !r.yanked) {
                            version_status(&dep.current_value, &latest.vers)
                        } else {
                            output::DepStatus::LookupError {
                                message: format!("crates.io: no non-yanked versions for '{pkg}'"),
                            }
                        }
                    }
                    Err(e) => output::DepStatus::LookupError {
                        message: format!("crates.io lookup failed: {e}"),
                    },
                }
            }
            "github-releases" => {
                match github_releases_datasource::fetch_latest_release(
                    pkg,
                    http,
                    github_releases_datasource::GITHUB_API,
                )
                .await
                {
                    Ok(Some((tag_name, _ts))) => version_status(&dep.current_value, &tag_name),
                    Ok(None) => output::DepStatus::LookupError {
                        message: format!("GitHub releases: no releases for '{pkg}'"),
                    },
                    Err(e) => output::DepStatus::LookupError {
                        message: format!("GitHub releases lookup failed: {e}"),
                    },
                }
            }
            "github-tags" => {
                match github_tags_datasource::fetch_latest_tag(
                    pkg,
                    http,
                    github_tags_datasource::GITHUB_API,
                )
                .await
                {
                    Ok(Some(tag)) => version_status(&dep.current_value, &tag),
                    Ok(None) => output::DepStatus::LookupError {
                        message: format!("GitHub tags: no tags for '{pkg}'"),
                    },
                    Err(e) => output::DepStatus::LookupError {
                        message: format!("GitHub tags lookup failed: {e}"),
                    },
                }
            }
            "pypi" => {
                use renovate_core::datasources::pypi as pypi_datasource;
                match pypi_datasource::fetch_versions(http, pkg, pypi_datasource::PYPI_API).await {
                    Ok(entry) => version_status(&dep.current_value, &entry.latest),
                    Err(e) => output::DepStatus::LookupError {
                        message: format!("PyPI lookup failed: {e}"),
                    },
                }
            }
            "maven" => {
                use renovate_core::datasources::maven as maven_datasource;
                match maven_datasource::fetch_latest(pkg, http).await {
                    Ok(Some(ver)) => version_status(&dep.current_value, &ver),
                    Ok(None) => output::DepStatus::LookupError {
                        message: format!("Maven Central: artifact '{pkg}' not found"),
                    },
                    Err(e) => output::DepStatus::LookupError {
                        message: format!("Maven lookup failed: {e}"),
                    },
                }
            }
            "nuget" => {
                use renovate_core::datasources::nuget as nuget_datasource;
                match nuget_datasource::fetch_latest(pkg, http, nuget_datasource::NUGET_API).await {
                    Ok(Some(ver)) => version_status(&dep.current_value, &ver),
                    Ok(None) => output::DepStatus::LookupError {
                        message: format!("NuGet: package '{pkg}' not found"),
                    },
                    Err(e) => output::DepStatus::LookupError {
                        message: format!("NuGet lookup failed: {e}"),
                    },
                }
            }
            "gitlab-tags" => {
                use renovate_core::datasources::gitlab_tags as gitlab_tags_datasource;
                match gitlab_tags_datasource::fetch_latest_tag(
                    pkg,
                    http,
                    gitlab_tags_datasource::GITLAB_API,
                )
                .await
                {
                    Ok(Some(tag)) => version_status(&dep.current_value, &tag),
                    Ok(None) => output::DepStatus::LookupError {
                        message: format!("GitLab tags: no tags for '{pkg}'"),
                    },
                    Err(e) => output::DepStatus::LookupError {
                        message: format!("GitLab tags lookup failed: {e}"),
                    },
                }
            }
            // docker/docker-hub — use the dep's registryUrl or the docker hub API.
            // customManagers provides image name as depName and tag as currentValue.
            "docker" => {
                let image = pkg;
                let tag = dep.current_value.trim_start_matches('v');
                use renovate_core::datasources::docker_hub;
                let dep_input = docker_hub::DockerDepInput {
                    dep_name: format!("{image}:{tag}"),
                    image: image.to_owned(),
                    tag: tag.to_owned(),
                };
                match docker_hub::fetch_updates_concurrent(
                    http,
                    &[dep_input],
                    docker_hub::DOCKER_HUB_API,
                    1,
                )
                .await
                .into_iter()
                .next()
                {
                    Some(update) => match update.summary {
                        Ok(summary) => {
                            if let Some(latest) = summary.latest {
                                version_status(&dep.current_value, &latest)
                            } else {
                                output::DepStatus::UpToDate { latest: None }
                            }
                        }
                        Err(e) => output::DepStatus::LookupError {
                            message: format!("Docker Hub lookup failed: {e}"),
                        },
                    },
                    None => output::DepStatus::LookupError {
                        message: format!("Docker Hub: image '{image}' not found or no tags"),
                    },
                }
            }
            other => {
                tracing::debug!(
                    repo = %repo_slug,
                    datasource = %other,
                    dep = %dep.dep_name,
                    "customManagers: unsupported datasource; skipping"
                );
                continue;
            }
        };

        deps.push(output::DepReport {
            name: dep.dep_name.clone(),
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
            package_name: dep.package_name.clone(),
            status,
        });
    }
    deps
}

/// Convert current/latest version strings into a `DepStatus`.
fn version_status(current: &str, latest: &str) -> output::DepStatus {
    let cur_clean = current
        .trim_start_matches('v')
        .trim_start_matches('^')
        .trim_start_matches('~');
    let lat_clean = latest.trim_start_matches('v');
    if cur_clean == lat_clean {
        output::DepStatus::UpToDate {
            latest: Some(latest.to_owned()),
        }
    } else {
        output::DepStatus::UpdateAvailable {
            current: current.to_owned(),
            latest: latest.to_owned(),
        }
    }
}
