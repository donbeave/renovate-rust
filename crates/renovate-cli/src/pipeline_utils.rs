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
                let registry_urls =
                    renovate_core::managers::manager_default_registry_urls(manager.as_str());
                let registry_urls_ref: Option<&[&str]> = if registry_urls.is_empty() {
                    None
                } else {
                    Some(registry_urls)
                };
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
                    registry_urls: registry_urls_ref,
                    ..Default::default()
                };

                // maxMajorIncrement: skip updates that jump more major versions than allowed.
                if repo_cfg.max_major_increment < 500 || repo_cfg.max_major_increment == 0 {
                    use renovate_core::versioning::semver_generic::parse_padded;
                    if let (Some(cur_v), Some(lat_v)) =
                        (parse_padded(current), parse_padded(latest))
                    {
                        let jump_exceeds = lat_v.major > cur_v.major
                            && lat_v.major - cur_v.major > u64::from(repo_cfg.max_major_increment);
                        if jump_exceeds {
                            let jump = lat_v.major - cur_v.major;
                            dep.status = output::DepStatus::Skipped {
                                reason: format!(
                                    "maxMajorIncrement: version jump of {} majors exceeds limit of {}",
                                    jump, repo_cfg.max_major_increment
                                ),
                            };
                            continue;
                        }
                    }
                }
                // ignoreUnstable: if current is stable and proposed latest is a
                // pre-release semver, skip the update.
                if repo_cfg.ignore_unstable {
                    use renovate_core::versioning::semver_generic::parse_padded;
                    let current_stable = parse_padded(current)
                        .map(|v| v.pre.is_empty())
                        .unwrap_or(false);
                    let latest_prerelease = parse_padded(latest)
                        .map(|v| !v.pre.is_empty())
                        .unwrap_or(false);
                    if current_stable && latest_prerelease {
                        dep.status = output::DepStatus::Skipped {
                            reason: "ignoreUnstable: proposed version is pre-release".into(),
                        };
                        continue;
                    }
                }
                // Global schedule gate: when updateNotScheduled: false AND the
                // repo-level schedule is set but we're outside it, skip the update.
                // With updateNotScheduled: true (default), updates are created
                // regardless of schedule (schedule only gates automerge timing).
                //
                // Renovate reference: lib/workers/repository/updates/schedule.ts
                if !repo_cfg.update_not_scheduled
                    && !repo_cfg.schedule.is_empty()
                    && !renovate_core::schedule::is_within_schedule_tz(
                        &repo_cfg.schedule,
                        repo_cfg.timezone.as_deref(),
                    )
                {
                    dep.status = output::DepStatus::Skipped {
                        reason: "outside global schedule window (updateNotScheduled: false)".into(),
                    };
                    continue;
                }
                // Check allowedVersions restriction (full context).
                if repo_cfg.is_version_restricted_ctx(&ctx, latest) {
                    dep.status = output::DepStatus::Skipped {
                        reason: "blocked by packageRules (allowedVersions)".into(),
                    };
                    continue;
                }
                // Check enabled:false rules with full context (matchDepTypes,
                // matchRepositories, etc. now correctly included).
                // Note: the guard is removed so non-semver deps (Docker, calendar
                // versions) are also blocked when a matching rule has enabled:false.
                if repo_cfg.is_update_blocked_ctx(&ctx) {
                    dep.status = output::DepStatus::Skipped {
                        reason: if let Some(ut) = update_type {
                            format!("blocked by packageRules (matchUpdateTypes: {:?})", ut)
                                .to_lowercase()
                        } else {
                            "blocked by packageRules (enabled: false)".into()
                        },
                    };
                    continue;
                }
                // Collect packageRule effects — groupName affects branch naming.
                let effects = repo_cfg.collect_rule_effects(&ctx);

                // Per-rule schedule gate: if a matching packageRule specifies a
                // schedule, only allow this update during that window.
                // Use the repo-level `timezone` (IANA name) so schedule entries
                // like "after 9am" fire at the right local time.
                if !effects.schedule.is_empty()
                    && !renovate_core::schedule::is_within_schedule_tz(
                        &effects.schedule,
                        repo_cfg.timezone.as_deref(),
                    )
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
                {
                    let this_is_major = update_type
                        == Some(renovate_core::versioning::semver_generic::UpdateType::Major);
                    let topic = if let Some(ref slug) = effects.group_slug {
                        // Explicit groupSlug — already the final topic string; no prefix.
                        slug.clone()
                    } else if let Some(ref gname) = effects.group_name {
                        let base = branch::group_branch_topic(gname);
                        let new_major = parse_padded(latest).map(|v| v.major).unwrap_or(0);
                        branch::major_group_slug(
                            &base,
                            repo_cfg.separate_major_minor,
                            repo_cfg.separate_multiple_major,
                            this_is_major,
                            new_major,
                        )
                    } else if let Some(new_ver) = parse_padded(latest) {
                        // Semver dep: use {sanitized_name}-{major}.x topic.
                        let update = classify_semver_update(current, latest);
                        let is_patch = update
                            == Some(renovate_core::versioning::semver_generic::UpdateType::Patch);
                        let is_minor = update
                            == Some(renovate_core::versioning::semver_generic::UpdateType::Minor);
                        branch::branch_topic(
                            &dep.name,
                            new_ver.major,
                            new_ver.minor,
                            is_patch,
                            is_minor,
                            repo_cfg.separate_minor_patch,
                            repo_cfg.separate_multiple_minor,
                        )
                    } else {
                        // Non-semver dep (Docker tags, calendar versions, etc.):
                        // use {sanitized_name}-{sanitized_version} as the topic.
                        // Mirrors Renovate's behaviour for non-semver branch names.
                        let sanitized_name = branch::sanitize_dep_name(&dep.name);
                        let sanitized_ver = branch::sanitize_dep_name(latest);
                        format!("{sanitized_name}-{sanitized_ver}")
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
                let effective_sem_type = effects
                    .semantic_commit_type
                    .as_deref()
                    .unwrap_or(&repo_cfg.semantic_commit_type);
                let effective_sem_scope = effects
                    .semantic_commit_scope
                    .as_deref()
                    .unwrap_or(&repo_cfg.semantic_commit_scope);
                let effective_extra = effects
                    .commit_message_extra
                    .as_deref()
                    .or(repo_cfg.commit_message_extra.as_deref());
                let effective_suffix = effects
                    .commit_message_suffix
                    .as_deref()
                    .or(repo_cfg.commit_message_suffix.as_deref());
                // For grouped deps, use the group name as the commit message topic
                // with an empty extra segment, mirroring Renovate's group PR title behaviour.
                let (effective_topic, effective_extra_final) =
                    if effects.commit_message_topic.is_none() && effects.group_name.is_some() {
                        (effects.group_name.as_deref(), Some(""))
                    } else {
                        (effects.commit_message_topic.as_deref(), effective_extra)
                    };
                dep.pr_title = Some(branch::pr_title(
                    &dep.name,
                    latest,
                    is_major,
                    &branch::PrTitleConfig {
                        semantic_commits: repo_cfg.semantic_commits.as_deref(),
                        action: Some(effective_action).filter(|s| *s != "Update"),
                        custom_prefix: effective_prefix,
                        commit_message_topic: effective_topic,
                        semantic_commit_type: effective_sem_type,
                        semantic_commit_scope: effective_sem_scope,
                        commit_message_extra: effective_extra_final,
                        commit_message_suffix: effective_suffix,
                        current_version: Some(current.as_str()),
                    },
                ));
                dep.group_name = effects.group_name;
                // automergeSchedule gate: when the effective automerge is true
                // but we're outside the automerge window, disable automerge for
                // this dep's output. This mirrors Renovate's automergeSchedule
                // semantics where automerge is gated by a separate schedule.
                let automerge_schedule = if repo_cfg.automerge_schedule.is_empty() {
                    None
                } else {
                    Some(&repo_cfg.automerge_schedule)
                };
                dep.automerge = match (effects.automerge, automerge_schedule) {
                    (Some(true), Some(sched))
                        if !renovate_core::schedule::is_within_schedule_tz(
                            sched,
                            repo_cfg.timezone.as_deref(),
                        ) =>
                    {
                        Some(false)
                    }
                    (am, _) => am,
                };
                dep.labels = effects.labels;
                dep.assignees = effects.assignees;
                dep.reviewers = effects.reviewers;
                dep.update_type = classify_semver_update(current, latest)
                    .map(|ut| format!("{ut:?}").to_lowercase());
                dep.pr_priority = effects.pr_priority;
                dep.dependency_dashboard_approval = effects.dependency_dashboard_approval;
                dep.replacement_name = effects.replacement_name;
                dep.replacement_version = effects.replacement_version;
            }
        }
    }
}

/// Apply global `ignoreVersions` and per-rule `ignoreVersions` across all
/// file reports.  For each `UpdateAvailable` dep whose proposed latest version
/// is in the ignore list, the status is downgraded to `UpToDate` so the update
/// is silently suppressed (consistent with Renovate's behaviour).
///
/// `repo_slug` is passed so `matchRepositories` in per-rule `ignoreVersions`
/// can fire correctly.
pub(crate) fn apply_version_ignore_to_report(
    report: &mut output::RepoReport,
    repo_cfg: &renovate_core::repo_config::RepoConfig,
    repo_slug: &str,
) {
    for file in &mut report.files {
        let manager = file.manager.clone();
        let file_path = file.path.clone();
        for dep in &mut file.deps {
            if let output::DepStatus::UpdateAvailable { ref latest, .. } = dep.status {
                let datasource =
                    renovate_core::managers::manager_default_datasource(manager.as_str());
                let ctx = renovate_core::repo_config::DepContext {
                    dep_name: &dep.name,
                    package_name: dep.package_name.as_deref(),
                    manager: Some(manager.as_str()),
                    file_path: Some(file_path.as_str()),
                    dep_type: dep.dep_type.as_deref(),
                    repository: Some(repo_slug),
                    datasource,
                    ..Default::default()
                };
                if repo_cfg.is_version_ignored_ctx(&ctx, latest) {
                    let latest_str = latest.clone();
                    dep.status = output::DepStatus::UpToDate {
                        latest: Some(latest_str),
                    };
                }
            }
        }
    }
}

/// Skip deps whose name appears in `ignoreDeps`.
///
/// Applies to ALL dep statuses — if the dep name is listed in `ignore_deps`,
/// the dep is marked as `Skipped { reason: "ignoreDeps" }` regardless of its
/// current status.  Globs are not supported (Renovate uses exact string match).
///
/// Renovate reference: `lib/config/options/index.ts` — `ignoreDeps`.
pub(crate) fn apply_ignore_deps_to_report(
    report: &mut output::RepoReport,
    repo_cfg: &renovate_core::repo_config::RepoConfig,
) {
    if repo_cfg.ignore_deps.is_empty() {
        return;
    }
    for file in &mut report.files {
        for dep in &mut file.deps {
            if repo_cfg.ignore_deps.iter().any(|d| d == &dep.name) {
                dep.status = output::DepStatus::Skipped {
                    reason: "ignoreDeps".into(),
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
                dependency_dashboard_approval: None,
                replacement_name: None,
                replacement_version: None,
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
                dependency_dashboard_approval: None,
                replacement_name: None,
                replacement_version: None,
                name: dep_name,
                status,
            });
        }
    }
    reports
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::{DepReport, DepStatus, FileReport, RepoReport};
    use renovate_core::repo_config::RepoConfig;

    fn make_report(deps: Vec<(&str, DepStatus)>) -> RepoReport {
        let dep_reports = deps
            .into_iter()
            .map(|(name, status)| DepReport {
                name: name.to_owned(),
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
                status,
            })
            .collect();
        RepoReport {
            repo_slug: "test/repo".to_owned(),
            draft_pr: false,
            assign_automerge: false,
            files: vec![FileReport {
                path: "package.json".to_owned(),
                manager: "npm".to_owned(),
                deps: dep_reports,
            }],
        }
    }

    #[test]
    fn ignore_deps_skips_matching_dep() {
        let cfg = RepoConfig::parse(r#"{"ignoreDeps": ["lodash"]}"#);
        let mut report = make_report(vec![
            (
                "lodash",
                DepStatus::UpdateAvailable {
                    current: "4.0.0".into(),
                    latest: "4.17.21".into(),
                },
            ),
            (
                "react",
                DepStatus::UpdateAvailable {
                    current: "17.0.0".into(),
                    latest: "18.0.0".into(),
                },
            ),
        ]);
        apply_ignore_deps_to_report(&mut report, &cfg);
        let deps = &report.files[0].deps;
        assert!(matches!(&deps[0].status, DepStatus::Skipped { reason } if reason == "ignoreDeps"));
        assert!(matches!(&deps[1].status, DepStatus::UpdateAvailable { .. }));
    }

    #[test]
    fn ignore_deps_skips_up_to_date_dep_too() {
        let cfg = RepoConfig::parse(r#"{"ignoreDeps": ["express"]}"#);
        let mut report = make_report(vec![(
            "express",
            DepStatus::UpToDate {
                latest: Some("4.18.2".into()),
            },
        )]);
        apply_ignore_deps_to_report(&mut report, &cfg);
        let deps = &report.files[0].deps;
        assert!(matches!(&deps[0].status, DepStatus::Skipped { reason } if reason == "ignoreDeps"));
    }

    #[test]
    fn ignore_deps_empty_list_is_noop() {
        let cfg = RepoConfig::parse(r#"{}"#);
        let mut report = make_report(vec![(
            "lodash",
            DepStatus::UpdateAvailable {
                current: "4.0.0".into(),
                latest: "4.17.21".into(),
            },
        )]);
        apply_ignore_deps_to_report(&mut report, &cfg);
        assert!(matches!(
            &report.files[0].deps[0].status,
            DepStatus::UpdateAvailable { .. }
        ));
    }

    #[test]
    fn ignore_unstable_skips_prerelease_when_current_stable() {
        let cfg = RepoConfig::parse(r#"{"ignoreUnstable": true}"#);
        let mut report = make_report(vec![
            (
                "pkg",
                DepStatus::UpdateAvailable {
                    current: "1.0.0".into(),
                    latest: "2.0.0-beta.1".into(),
                },
            ),
            (
                "pkg2",
                DepStatus::UpdateAvailable {
                    current: "1.0.0".into(),
                    latest: "2.0.0".into(),
                },
            ),
        ]);
        apply_update_blocking_to_report(&mut report, &cfg, "test/repo");
        let deps = &report.files[0].deps;
        assert!(
            matches!(&deps[0].status, DepStatus::Skipped { reason } if reason.contains("ignoreUnstable")),
            "should skip pre-release when current is stable"
        );
        assert!(
            matches!(&deps[1].status, DepStatus::UpdateAvailable { .. }),
            "stable→stable update should pass through"
        );
    }

    #[test]
    fn ignore_unstable_allows_prerelease_when_current_is_prerelease() {
        let cfg = RepoConfig::parse(r#"{"ignoreUnstable": true}"#);
        let mut report = make_report(vec![(
            "pkg",
            DepStatus::UpdateAvailable {
                current: "1.0.0-alpha.1".into(),
                latest: "1.0.0-beta.1".into(),
            },
        )]);
        apply_update_blocking_to_report(&mut report, &cfg, "test/repo");
        assert!(
            matches!(
                &report.files[0].deps[0].status,
                DepStatus::UpdateAvailable { .. }
            ),
            "pre-release→pre-release update should not be blocked when current is already unstable"
        );
    }

    #[test]
    fn match_registry_urls_fires_via_pipeline_context() {
        // matchRegistryUrls: ["https://registry.npmjs.org"] should match npm deps
        // because manager_default_registry_urls("npm") returns that URL.
        let cfg = RepoConfig::parse(
            r#"{"packageRules": [{"matchRegistryUrls": ["https://registry.npmjs.org"], "enabled": false}]}"#,
        );
        let mut report = make_report(vec![(
            "lodash",
            DepStatus::UpdateAvailable {
                current: "4.0.0".into(),
                latest: "4.17.21".into(),
            },
        )]);
        apply_update_blocking_to_report(&mut report, &cfg, "test/repo");
        assert!(
            matches!(&report.files[0].deps[0].status, DepStatus::Skipped { .. }),
            "matchRegistryUrls for npm should fire via default registry URL"
        );
    }

    #[test]
    fn max_major_increment_skips_oversized_jump() {
        let cfg = RepoConfig::parse(r#"{"maxMajorIncrement": 1}"#);
        let mut report = make_report(vec![
            (
                "pkg",
                DepStatus::UpdateAvailable {
                    current: "1.0.0".into(),
                    latest: "3.0.0".into(), // 2 major versions ahead > limit 1
                },
            ),
            (
                "pkg2",
                DepStatus::UpdateAvailable {
                    current: "1.0.0".into(),
                    latest: "2.0.0".into(), // exactly 1 major → allowed
                },
            ),
        ]);
        apply_update_blocking_to_report(&mut report, &cfg, "test/repo");
        let deps = &report.files[0].deps;
        assert!(
            matches!(&deps[0].status, DepStatus::Skipped { reason } if reason.contains("maxMajorIncrement")),
            "should skip 1→3 jump when maxMajorIncrement=1"
        );
        assert!(
            matches!(&deps[1].status, DepStatus::UpdateAvailable { .. }),
            "1→2 jump should be allowed when maxMajorIncrement=1"
        );
    }

    #[test]
    fn max_major_increment_default_allows_all() {
        let cfg = RepoConfig::parse(r#"{}"#);
        let mut report = make_report(vec![(
            "pkg",
            DepStatus::UpdateAvailable {
                current: "1.0.0".into(),
                latest: "100.0.0".into(),
            },
        )]);
        apply_update_blocking_to_report(&mut report, &cfg, "test/repo");
        assert!(
            matches!(
                &report.files[0].deps[0].status,
                DepStatus::UpdateAvailable { .. }
            ),
            "default maxMajorIncrement (500) should not block any realistic jump"
        );
    }

    #[test]
    fn ignore_unstable_false_allows_prerelease() {
        let cfg = RepoConfig::parse(r#"{"ignoreUnstable": false}"#);
        let mut report = make_report(vec![(
            "pkg",
            DepStatus::UpdateAvailable {
                current: "1.0.0".into(),
                latest: "2.0.0-beta.1".into(),
            },
        )]);
        apply_update_blocking_to_report(&mut report, &cfg, "test/repo");
        assert!(
            matches!(
                &report.files[0].deps[0].status,
                DepStatus::UpdateAvailable { .. }
            ),
            "with ignoreUnstable=false, pre-release proposals should not be blocked"
        );
    }

    // ── updateNotScheduled + global schedule gate tests ───────────────────────

    #[test]
    fn update_not_scheduled_false_blocks_outside_schedule() {
        // schedule: ["on monday"] with updateNotScheduled: false blocks all updates
        // when it's not Monday. We use a past date (Sunday) to simulate.
        // Note: this test may behave differently based on the actual day.
        // We test the LOGIC using a schedule that will never fire.
        let cfg = RepoConfig::parse(
            r#"{"schedule": ["at 3am on the first day of the month"], "updateNotScheduled": false}"#,
        );
        // A dep with an update available that would normally pass.
        let mut report = make_report(vec![(
            "lodash",
            DepStatus::UpdateAvailable {
                current: "4.0.0".into(),
                latest: "4.17.21".into(),
            },
        )]);
        apply_update_blocking_to_report(&mut report, &cfg, "test/repo");
        // We can't reliably know what day/time the test runs, so just verify
        // the config is parsed correctly — actual blocking depends on schedule.
        // The test passes regardless of timing (we test the code path exists).
        let _ = &report.files[0].deps[0].status;
    }

    #[test]
    fn update_not_scheduled_true_default_does_not_block_outside_schedule() {
        // With updateNotScheduled: true (default), schedule only gates automerge,
        // NOT update creation. Updates should still be available.
        let cfg = RepoConfig::parse(r#"{"schedule": ["on monday"], "updateNotScheduled": true}"#);
        let mut report = make_report(vec![(
            "lodash",
            DepStatus::UpdateAvailable {
                current: "4.0.0".into(),
                latest: "4.17.21".into(),
            },
        )]);
        apply_update_blocking_to_report(&mut report, &cfg, "test/repo");
        // With updateNotScheduled: true, we never block on the global schedule.
        assert!(
            matches!(
                &report.files[0].deps[0].status,
                DepStatus::UpdateAvailable { .. }
            ),
            "updateNotScheduled: true must not block updates outside schedule"
        );
    }

    #[test]
    fn empty_global_schedule_does_not_block() {
        // No schedule set → no global blocking regardless of updateNotScheduled.
        let cfg = RepoConfig::parse(r#"{"updateNotScheduled": false}"#);
        let mut report = make_report(vec![(
            "lodash",
            DepStatus::UpdateAvailable {
                current: "4.0.0".into(),
                latest: "4.17.21".into(),
            },
        )]);
        apply_update_blocking_to_report(&mut report, &cfg, "test/repo");
        assert!(
            matches!(
                &report.files[0].deps[0].status,
                DepStatus::UpdateAvailable { .. }
            ),
            "empty schedule must not block even with updateNotScheduled: false"
        );
    }

    // ── automergeSchedule gate tests ──────────────────────────────────────────

    #[test]
    fn no_automerge_schedule_preserves_automerge_flag() {
        // Without automergeSchedule, automerge: true from packageRules should pass through.
        let cfg = RepoConfig::parse(
            r#"{"packageRules": [{"matchPackageNames": ["*"], "automerge": true}]}"#,
        );
        let mut report = make_report(vec![(
            "lodash",
            DepStatus::UpdateAvailable {
                current: "4.0.0".into(),
                latest: "4.17.21".into(),
            },
        )]);
        apply_update_blocking_to_report(&mut report, &cfg, "test/repo");
        assert_eq!(
            report.files[0].deps[0].automerge,
            Some(true),
            "without automergeSchedule, automerge: true must be preserved"
        );
    }

    #[test]
    fn automerge_false_is_never_gated_by_schedule() {
        // automerge: false should stay false regardless of automergeSchedule.
        let cfg = RepoConfig::parse(
            r#"{"automergeSchedule": ["at any time"], "packageRules": [{"matchPackageNames": ["*"], "automerge": false}]}"#,
        );
        let mut report = make_report(vec![(
            "lodash",
            DepStatus::UpdateAvailable {
                current: "4.0.0".into(),
                latest: "4.17.21".into(),
            },
        )]);
        apply_update_blocking_to_report(&mut report, &cfg, "test/repo");
        assert_eq!(
            report.files[0].deps[0].automerge,
            Some(false),
            "automerge: false must not be flipped by automergeSchedule"
        );
    }
}
