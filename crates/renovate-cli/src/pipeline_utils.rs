//! Pipeline utility helpers shared across multiple manager processing blocks.

use std::collections::HashMap;

use renovate_core::http::HttpClient;

use crate::output;

/// Apply `packageRules` `matchUpdateTypes`+`enabled:false` blocking across all
/// file reports.  For each `UpdateAvailable` dep, classifies the semver bump
/// type and converts to `Skipped` when a matching rule blocks it.
///
/// `repo_slug` is passed as `"owner/repo"` so `matchRepositories` rules can fire.
pub(crate) fn apply_update_blocking_to_report(
    report: &mut output::RepoReport,
    repo_cfg: &renovate_core::repo_config::RepoConfig,
    repo_slug: &str,
) {
    use renovate_core::branch;
    use renovate_core::versioning::semver_generic::{classify_semver_update, parse_padded};
    for file in &mut report.files {
        let manager = file.manager.clone();
        let file_path = file.path.clone();
        for dep in &mut file.deps {
            if let output::DepStatus::UpdateAvailable {
                ref current,
                ref latest,
            } = dep.status
            {
                // Build the full context once — reused for all blocking checks AND
                // effect collection so matchers like matchDepTypes / matchRepositories
                // / matchDatasources fire consistently across every check.
                let update_type = classify_semver_update(current, latest);
                let datasource =
                    renovate_core::managers::manager_default_datasource(manager.as_str());
                let ctx = renovate_core::repo_config::DepContext {
                    dep_name: &dep.name,
                    package_name: dep.package_name.as_deref(),
                    manager: Some(manager.as_str()),
                    file_path: Some(file_path.as_str()),
                    current_value: Some(current.as_str()),
                    new_value: Some(latest.as_str()),
                    update_type,
                    current_version_timestamp: dep.current_version_timestamp.as_deref(),
                    dep_type: dep.dep_type.as_deref(),
                    repository: Some(repo_slug),
                    datasource,
                    ..Default::default()
                };

                // Check allowedVersions restriction (full context).
                if repo_cfg.is_version_restricted_ctx(&ctx, latest) {
                    dep.status = output::DepStatus::Skipped {
                        reason: "blocked by packageRules (allowedVersions)".into(),
                    };
                    continue;
                }
                // Check enabled:false rules with full context (matchDepTypes,
                // matchRepositories, etc. now correctly included).
                if update_type.is_some() && repo_cfg.is_update_blocked_ctx(&ctx) {
                    dep.status = output::DepStatus::Skipped {
                        reason: format!(
                            "blocked by packageRules (matchUpdateTypes: {:?})",
                            update_type.unwrap()
                        )
                        .to_lowercase(),
                    };
                    continue;
                }
                // Collect packageRule effects — groupName affects branch naming.
                let effects = repo_cfg.collect_rule_effects(&ctx);

                // Per-rule schedule gate: if a matching packageRule specifies a
                // schedule, only allow this update during that window.
                if !effects.schedule.is_empty()
                    && !renovate_core::schedule::is_within_schedule(&effects.schedule)
                {
                    dep.status = output::DepStatus::Skipped {
                        reason: "outside schedule window (packageRule)".into(),
                    };
                    continue;
                }

                // Per-rule minimumReleaseAge: last matching rule with minimumReleaseAge
                // wins; overrides the global setting for this dep.
                let effective_min_age = effects
                    .minimum_release_age
                    .as_deref()
                    .or(repo_cfg.minimum_release_age.as_deref());
                if effective_min_age.is_some()
                    && !renovate_core::schedule::is_within_release_age(
                        dep.release_timestamp.as_deref(),
                        effective_min_age,
                    )
                {
                    dep.status = output::DepStatus::Skipped {
                        reason: "newer than minimumReleaseAge".into(),
                    };
                    continue;
                }

                // Compute the proposed branch name.
                // When groupName is set, use the group slug as the topic so all
                // grouped deps share one branch (matching Renovate's behaviour).
                // Explicit groupSlug overrides the auto-derived slug from groupName.
                if let Some(new_ver) = parse_padded(latest) {
                    let topic = if let Some(ref slug) = effects.group_slug {
                        // Explicit groupSlug — already the final topic string.
                        slug.clone()
                    } else if let Some(ref gname) = effects.group_name {
                        branch::group_branch_topic(gname)
                    } else {
                        let is_patch = classify_semver_update(current, latest)
                            == Some(renovate_core::versioning::semver_generic::UpdateType::Patch);
                        branch::branch_topic(
                            &dep.name,
                            new_ver.major,
                            new_ver.minor,
                            is_patch,
                            repo_cfg.separate_minor_patch,
                        )
                    };
                    dep.branch_name = Some(if let Some(len) = repo_cfg.hashed_branch_length {
                        branch::hashed_branch_name(
                            &repo_cfg.branch_prefix,
                            &repo_cfg.additional_branch_prefix,
                            &topic,
                            len,
                        )
                    } else {
                        branch::branch_name(
                            &repo_cfg.branch_prefix,
                            &repo_cfg.additional_branch_prefix,
                            &topic,
                        )
                    });
                }
                // Generate PR title.
                let is_major = classify_semver_update(current, latest)
                    == Some(renovate_core::versioning::semver_generic::UpdateType::Major);
                // Per-rule overrides win over repo-level settings.
                let effective_action = effects
                    .commit_message_action
                    .as_deref()
                    .unwrap_or(repo_cfg.commit_message_action.as_str());
                let effective_prefix = effects
                    .commit_message_prefix
                    .as_deref()
                    .or(repo_cfg.commit_message_prefix.as_deref());
                dep.pr_title = Some(branch::pr_title(
                    &dep.name,
                    latest,
                    is_major,
                    repo_cfg.semantic_commits.as_deref(),
                    Some(effective_action).filter(|s| *s != "Update"),
                    effective_prefix,
                    effects.commit_message_topic.as_deref(),
                ));
                dep.group_name = effects.group_name;
                dep.automerge = effects.automerge;
                dep.labels = effects.labels;
                dep.assignees = effects.assignees;
                dep.reviewers = effects.reviewers;
                dep.update_type = classify_semver_update(current, latest)
                    .map(|ut| format!("{ut:?}").to_lowercase());
                dep.pr_priority = effects.pr_priority;
            }
        }
    }
}

/// Apply global `ignoreVersions` and per-rule `ignoreVersions` across all
/// file reports.  For each `UpdateAvailable` dep whose proposed latest version
/// is in the ignore list, the status is downgraded to `UpToDate` so the update
/// is silently suppressed (consistent with Renovate's behaviour).
pub(crate) fn apply_version_ignore_to_report(
    report: &mut output::RepoReport,
    repo_cfg: &renovate_core::repo_config::RepoConfig,
) {
    for file in &mut report.files {
        let manager = file.manager.clone();
        for dep in &mut file.deps {
            if let output::DepStatus::UpdateAvailable { ref latest, .. } = dep.status
                && repo_cfg.is_version_ignored(&dep.name, &manager, latest)
            {
                let latest_str = latest.clone();
                dep.status = output::DepStatus::UpToDate {
                    latest: Some(latest_str),
                };
            }
        }
    }
}

/// Return the matched files for a given manager name (empty slice if not
/// detected).
pub(crate) fn manager_files(
    detected: &[renovate_core::managers::DetectedManager],
    name: &str,
) -> Vec<String> {
    detected
        .iter()
        .find(|m| m.name == name)
        .map(|m| m.matched_files.clone())
        .unwrap_or_default()
}

/// Fetch Docker Hub updates for `deps` and build a `DepReport` list.
///
/// Identical logic is shared across Cloud Build, Drone CI, Bitbucket Pipelines,
/// GitLab CI, CircleCI, Dockerfile, Docker Compose, and similar managers.
pub(crate) async fn docker_hub_reports(
    http: &HttpClient,
    deps: &[renovate_core::extractors::dockerfile::DockerfileExtractedDep],
) -> Vec<output::DepReport> {
    use renovate_core::datasources::docker_hub as docker_datasource;

    let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
    let dep_inputs: Vec<docker_datasource::DockerDepInput> = actionable
        .iter()
        .filter_map(|d| {
            let tag = d.tag.as_deref()?;
            Some(docker_datasource::DockerDepInput {
                dep_name: format!("{}:{tag}", d.image),
                image: d.image.clone(),
                tag: tag.to_owned(),
            })
        })
        .collect();
    let updates = docker_datasource::fetch_updates_concurrent(
        http,
        &dep_inputs,
        docker_datasource::DOCKER_HUB_API,
        10,
    )
    .await;
    let update_map: HashMap<String, _> = updates
        .into_iter()
        .map(|r| (r.dep_name, r.summary))
        .collect();

    let mut reports = Vec::new();
    for dep in deps {
        if let Some(reason) = &dep.skip_reason {
            reports.push(output::DepReport {
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
                name: dep.image.clone(),
                status: output::DepStatus::Skipped {
                    reason: format!("{reason:?}").to_lowercase(),
                },
            });
        } else {
            let dep_name = match &dep.tag {
                Some(t) => format!("{}:{t}", dep.image),
                None => dep.image.clone(),
            };
            let status = match update_map.get(&dep_name) {
                Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                    current: s.current_tag.clone(),
                    latest: s.latest.clone().unwrap_or_default(),
                },
                Some(Ok(s)) => output::DepStatus::UpToDate {
                    latest: s.latest.clone(),
                },
                Some(Err(docker_datasource::DockerHubError::NonDockerHub(_))) => {
                    output::DepStatus::Skipped {
                        reason: "non-docker-hub registry".into(),
                    }
                }
                Some(Err(e)) => output::DepStatus::LookupError {
                    message: e.to_string(),
                },
                None => output::DepStatus::UpToDate { latest: None },
            };
            reports.push(output::DepReport {
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
                name: dep_name,
                status,
            });
        }
    }
    reports
}
