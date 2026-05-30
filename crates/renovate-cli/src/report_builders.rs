//! Report-building helpers that convert raw extracted deps + update-map
//! results into `output::DepReport` lists.
//!
//! Each `build_dep_reports_*` function is a pure data-mapping step: it merges
//! the full dependency list (including skipped entries) with the lookup results
//! produced by the corresponding datasource.

use std::collections::HashMap;

use crate::output;
use renovate_core::branch;
use renovate_core::repo_config::RepoConfig;
use renovate_core::versioning::semver_generic;

/// Compute branch name, PR title, and update type for an `UpdateAvailable` dep.
fn dep_meta(
    dep_name: &str,
    current_constraint: &str,
    latest: &str,
    repo_cfg: &RepoConfig,
) -> (Option<String>, Option<String>, Option<String>) {
    let update_type = semver_generic::classify_semver_update(current_constraint, latest);
    let (is_major, is_minor, is_patch) = match update_type {
        Some(semver_generic::UpdateType::Major) => (true, false, false),
        Some(semver_generic::UpdateType::Minor) => (false, true, false),
        Some(semver_generic::UpdateType::Patch) => (false, false, true),
        _ => (false, false, false),
    };
    let update_type_str = match update_type {
        Some(semver_generic::UpdateType::Major) => Some("major".to_owned()),
        Some(semver_generic::UpdateType::Minor) => Some("minor".to_owned()),
        Some(semver_generic::UpdateType::Patch) => Some("patch".to_owned()),
        _ => None,
    };

    let new_major = semver_generic::parse_padded(latest).map(|v| v.major).unwrap_or(0);
    let new_minor = semver_generic::parse_padded(latest).map(|v| v.minor).unwrap_or(0);

    let topic = branch::branch_topic(
        dep_name,
        new_major,
        new_minor,
        is_patch,
        is_minor,
        repo_cfg.separate_minor_patch,
        repo_cfg.separate_multiple_minor,
    );

    let branch_name = if let Some(limit) = repo_cfg.hashed_branch_length {
        branch::hashed_branch_name(
            &repo_cfg.branch_prefix,
            &repo_cfg.additional_branch_prefix,
            &topic,
            limit,
        )
    } else {
        branch::branch_name_with_strict(
            &repo_cfg.branch_prefix,
            &repo_cfg.additional_branch_prefix,
            &topic,
            repo_cfg.branch_name_strict,
        )
    };

    let pr_title = branch::pr_title(
        dep_name,
        latest,
        is_major,
        &branch::PrTitleConfig {
            semantic_commits: repo_cfg.semantic_commits.as_deref(),
            action: None,
            custom_prefix: None,
            commit_message_topic: None,
            semantic_commit_type: &repo_cfg.semantic_commit_type,
            semantic_commit_scope: &repo_cfg.semantic_commit_scope,
            commit_message_extra: None,
            commit_message_suffix: None,
            current_version: Some(current_constraint),
            new_value: None,
        },
    );

    (Some(branch_name), Some(pr_title), update_type_str)
}

pub(crate) fn build_dep_reports_cargo(
    all_deps: &[renovate_core::extractors::cargo::ExtractedDep],
    actionable: &[&renovate_core::extractors::cargo::ExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::versioning::cargo::UpdateSummary,
            renovate_core::datasources::crates_io::CratesIoError,
        >,
    >,
    timestamps: &HashMap<String, renovate_core::datasources::crates_io::CrateTimestamps>,    repo_cfg: &RepoConfig,

) -> Vec<output::DepReport> {
    let mut reports = Vec::new();
    for dep in all_deps.iter().filter(|d| d.skip_reason.is_some()) {
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
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.dep_name.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let summary = update_map.get(&dep.dep_name).and_then(|r| r.as_ref().ok());
        let new_value = match update_map.get(&dep.dep_name) {
            Some(Ok(s)) if s.update_available => {
                let new_ver = s.latest.as_deref().unwrap_or_default();
                let current_ver = s.current_constraint.trim().trim_start_matches('=').trim();
                // Use Bump for ranges (shows what would change in Cargo.toml),
                // Replace for exact pins.
                let stripped = s.current_constraint.trim();
                let is_range = stripped.contains(['^', '~', '>', '<', ',', '*'])
                    || (!stripped.starts_with('=')
                        && stripped
                            .split_once('.')
                            .and_then(|(_, rest)| rest.split_once('.'))
                            .is_none());
                let strategy = if is_range {
                    renovate_core::versioning::cargo::RangeStrategy::Bump
                } else {
                    renovate_core::versioning::cargo::RangeStrategy::Replace
                };
                renovate_core::versioning::cargo::get_new_value(
                    &s.current_constraint,
                    strategy,
                    current_ver,
                    new_ver,
                )
            }
            _ => None,
        };
        let status = match update_map.get(&dep.dep_name) {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_constraint.clone(),
                latest: s.latest.clone().unwrap_or_default(),
            },
            Some(Ok(s)) => output::DepStatus::UpToDate {
                latest: s.latest.clone(),
            },
            Some(Err(e)) => output::DepStatus::LookupError {
                message: e.to_string(),
            },
            None => output::DepStatus::UpToDate { latest: None },
        };
        let (branch_name, pr_title, update_type) = match &status {
            output::DepStatus::UpdateAvailable { current, latest } => {
                dep_meta(&dep.dep_name, current, latest, repo_cfg)
            }
            _ => (None, None, None),
        };
        // Release timestamp for the absolute latest version.
        let release_timestamp = summary
            .and_then(|s| s.latest.as_deref())
            .and_then(|latest| timestamps.get(&dep.package_name)?.get(latest).cloned());
        // Current-version timestamp for exact pins (`= 1.2.3` syntax).
        // Cargo ranges (^, ~, bare version without =) don't reveal the
        // installed version, so we can't look up a timestamp.
        let current_version_timestamp = {
            let v = dep.current_value.trim();
            if let Some(pinned) = v.strip_prefix('=') {
                let pinned = pinned.trim();
                timestamps
                    .get(&dep.package_name)
                    .and_then(|ts| ts.get(pinned))
                    .cloned()
            } else {
                None
            }
        };
        reports.push(output::DepReport {
            branch_name,
            group_name: None,
            automerge: None,
            labels: Vec::new(),
            assignees: Vec::new(),
            reviewers: Vec::new(),
            update_type,
            pr_priority: None,
            pr_title,
            release_timestamp,
            current_version_timestamp,
            dep_type: Some(dep.dep_type.as_renovate_str().to_owned()),
            package_name: (dep.dep_name != dep.package_name).then(|| dep.package_name.clone()),
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value,
            name: dep.dep_name.clone(),
            status,
        });
    }
    reports
}

pub(crate) fn build_dep_reports_npm(
    all_deps: &[renovate_core::extractors::npm::NpmExtractedDep],
    actionable: &[&renovate_core::extractors::npm::NpmExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::versioning::npm::NpmUpdateSummary,
            renovate_core::datasources::npm::NpmError,
        >,
    >,
    // Per-package release timestamps keyed by exact version string.
    // Used to populate `current_version_timestamp` for `matchCurrentAge` rules.
    version_timestamps: &HashMap<String, HashMap<String, String>>,    repo_cfg: &RepoConfig,

) -> Vec<output::DepReport> {
    let mut reports = Vec::new();
    for dep in all_deps.iter().filter(|d| d.skip_reason.is_some()) {
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
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.name.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let summary = update_map.get(&dep.name);
        let lookup_name = dep.package_name.as_ref().unwrap_or(&dep.name);
        let release_timestamp = summary
            .and_then(|r| r.as_ref().ok())
            .and_then(|s| s.latest_timestamp.clone());
        // Resolve current_version_timestamp for exact pins (e.g. "4.17.21").
        // For ranges we don't know the installed version, so leave as None.
        let current_version_timestamp = {
            let stripped = dep.current_value.trim().trim_start_matches('=').trim();
            // Heuristic: no range operators + starts with digit → exact pin.
            let is_exact = stripped.starts_with(|c: char| c.is_ascii_digit())
                && !stripped.contains(['^', '~', '>', '<', '*', ' ', ',']);
            if is_exact {
                version_timestamps
                    .get(lookup_name)
                    .and_then(|ts| ts.get(stripped))
                    .cloned()
            } else {
                None
            }
        };
        let new_value = match summary {
            Some(Ok(s)) if s.update_available => {
                let new_ver = s.latest.as_deref().unwrap_or_default();
                let current_ver = s
                    .current_constraint
                    .trim()
                    .trim_start_matches('=')
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .trim_start_matches(['^', '~', '>', '<']);
                renovate_core::versioning::npm::get_new_value(
                    &s.current_constraint,
                    "replace",
                    current_ver,
                    new_ver,
                )
            }
            _ => None,
        };
        let status = match summary {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_constraint.clone(),
                latest: s.latest.clone().unwrap_or_default(),
            },
            Some(Ok(s)) => output::DepStatus::UpToDate {
                latest: s.latest.clone(),
            },
            Some(Err(e)) => output::DepStatus::LookupError {
                message: e.to_string(),
            },
            None => output::DepStatus::UpToDate { latest: None },
        };
        let (branch_name, pr_title, update_type) = match &status {
            output::DepStatus::UpdateAvailable { current, latest } => {
                dep_meta(&dep.name, current, latest, repo_cfg)
            }
            _ => (None, None, None),
        };
        reports.push(output::DepReport {
            branch_name,
            group_name: None,
            automerge: None,
            labels: Vec::new(),
            assignees: Vec::new(),
            reviewers: Vec::new(),
            update_type,
            pr_priority: None,
            pr_title,
            release_timestamp,
            current_version_timestamp,
            dep_type: Some(dep.dep_type.as_renovate_str().to_owned()),
            package_name: dep.package_name.clone(),
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value,
            name: dep.name.clone(),
            status,
        });
    }
    reports
}

pub(crate) fn build_dep_reports_github_actions(
    all_deps: &[renovate_core::extractors::github_actions::GithubActionsExtractedDep],
    actionable: &[&renovate_core::extractors::github_actions::GithubActionsExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::datasources::github_tags::GithubActionsUpdateSummary,
            renovate_core::datasources::github_tags::GithubTagsError,
        >,
    >,
    repo_cfg: &RepoConfig,
) -> Vec<output::DepReport> {
    let mut reports = Vec::new();
    for dep in all_deps.iter().filter(|d| d.skip_reason.is_some()) {
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
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.action.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let status = match update_map.get(&dep.action) {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_value.clone(),
                latest: s.latest.clone().unwrap_or_default(),
            },
            Some(Ok(s)) => output::DepStatus::UpToDate {
                latest: s.latest.clone(),
            },
            Some(Err(e)) => output::DepStatus::LookupError {
                message: e.to_string(),
            },
            None => output::DepStatus::UpToDate { latest: None },
        };
        let (branch_name, pr_title, update_type) = match &status {
            output::DepStatus::UpdateAvailable { current, latest } => {
                dep_meta(&dep.action, current, latest, repo_cfg)
            }
            _ => (None, None, None),
        };
        reports.push(output::DepReport {
            branch_name,
            group_name: None,
            automerge: None,
            labels: Vec::new(),
            assignees: Vec::new(),
            reviewers: Vec::new(),
            update_type,
            pr_priority: None,
            pr_title,
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
            new_value: None,
            name: dep.action.clone(),
            status,
        });
    }
    reports
}

pub(crate) fn build_dep_reports_maven(
    all_deps: &[renovate_core::extractors::maven::MavenExtractedDep],
    actionable: &[&renovate_core::extractors::maven::MavenExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::datasources::maven::MavenUpdateSummary,
            renovate_core::datasources::maven::MavenError,
        >,
    >,
    repo_cfg: &RepoConfig,
) -> Vec<output::DepReport> {
    let mut reports = Vec::new();
    for dep in all_deps.iter().filter(|d| d.skip_reason.is_some()) {
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
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.dep_name.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let summary = update_map.get(&dep.dep_name);
        let release_timestamp = summary
            .and_then(|r| r.as_ref().ok())
            .and_then(|s| s.release_timestamp.clone());
        let status = match summary {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_version.clone(),
                latest: s.latest.clone().unwrap_or_default(),
            },
            Some(Ok(s)) => output::DepStatus::UpToDate {
                latest: s.latest.clone(),
            },
            Some(Err(e)) => output::DepStatus::LookupError {
                message: e.to_string(),
            },
            None => output::DepStatus::UpToDate { latest: None },
        };
        let (branch_name, pr_title, update_type) = match &status {
            output::DepStatus::UpdateAvailable { current, latest } => {
                dep_meta(&dep.dep_name, current, latest, repo_cfg)
            }
            _ => (None, None, None),
        };
        reports.push(output::DepReport {
            branch_name,
            group_name: None,
            automerge: None,
            labels: Vec::new(),
            assignees: Vec::new(),
            reviewers: Vec::new(),
            update_type,
            pr_priority: None,
            pr_title,
            release_timestamp,
            current_version_timestamp: None,
            dep_type: Some(dep.renovate_dep_type().to_owned()),
            package_name: None,
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.dep_name.clone(),
            status,
        });
    }
    reports
}

pub(crate) fn build_dep_reports_pub(
    all_deps: &[renovate_core::extractors::pubspec::PubspecExtractedDep],
    actionable: &[&renovate_core::extractors::pubspec::PubspecExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::datasources::pub_dev::PubUpdateSummary,
            renovate_core::datasources::pub_dev::PubError,
        >,
    >,
    repo_cfg: &RepoConfig,
) -> Vec<output::DepReport> {
    let mut reports = Vec::new();
    for dep in all_deps.iter().filter(|d| d.skip_reason.is_some()) {
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
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.name.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let summary = update_map.get(&dep.name);
        let release_timestamp = summary
            .and_then(|r| r.as_ref().ok())
            .and_then(|s| s.release_timestamp.clone());
        let status = match summary {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_value.clone(),
                latest: s.latest.clone().unwrap_or_default(),
            },
            Some(Ok(s)) => output::DepStatus::UpToDate {
                latest: s.latest.clone(),
            },
            Some(Err(e)) => output::DepStatus::LookupError {
                message: e.to_string(),
            },
            None => output::DepStatus::UpToDate { latest: None },
        };
        let (branch_name, pr_title, update_type) = match &status {
            output::DepStatus::UpdateAvailable { current, latest } => {
                dep_meta(&dep.name, current, latest, repo_cfg)
            }
            _ => (None, None, None),
        };
        reports.push(output::DepReport {
            branch_name,
            group_name: None,
            automerge: None,
            labels: Vec::new(),
            assignees: Vec::new(),
            reviewers: Vec::new(),
            update_type,
            pr_priority: None,
            pr_title,
            release_timestamp,
            current_version_timestamp: None,
            dep_type: Some(dep.dep_type.as_renovate_str().to_owned()),
            package_name: None,
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.name.clone(),
            status,
        });
    }
    reports
}

pub(crate) fn build_dep_reports_nuget(
    all_deps: &[renovate_core::extractors::nuget::NuGetExtractedDep],
    actionable: &[&renovate_core::extractors::nuget::NuGetExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::datasources::nuget::NuGetUpdateSummary,
            renovate_core::datasources::nuget::NuGetError,
        >,
    >,
    repo_cfg: &RepoConfig,
) -> Vec<output::DepReport> {
    let mut reports = Vec::new();
    for dep in all_deps.iter().filter(|d| d.skip_reason.is_some()) {
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
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.package_id.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let summary = update_map.get(&dep.package_id);
        let release_timestamp = summary
            .and_then(|r| r.as_ref().ok())
            .and_then(|s| s.release_timestamp.clone());
        let status = match summary {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_value.clone(),
                latest: s.latest.clone().unwrap_or_default(),
            },
            Some(Ok(s)) => output::DepStatus::UpToDate {
                latest: s.latest.clone(),
            },
            Some(Err(e)) => output::DepStatus::LookupError {
                message: e.to_string(),
            },
            None => output::DepStatus::UpToDate { latest: None },
        };
        let (branch_name, pr_title, update_type) = match &status {
            output::DepStatus::UpdateAvailable { current, latest } => {
                dep_meta(&dep.package_id, current, latest, repo_cfg)
            }
            _ => (None, None, None),
        };
        reports.push(output::DepReport {
            branch_name,
            group_name: None,
            automerge: None,
            labels: Vec::new(),
            assignees: Vec::new(),
            reviewers: Vec::new(),
            update_type,
            pr_priority: None,
            pr_title,
            release_timestamp,
            current_version_timestamp: None,

            dep_type: Some(dep.dep_type.as_renovate_str().to_owned()),
            package_name: None,
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.package_id.clone(),
            status,
        });
    }
    reports
}

pub(crate) fn build_dep_reports_composer(
    all_deps: &[renovate_core::extractors::composer::ComposerExtractedDep],
    actionable: &[&renovate_core::extractors::composer::ComposerExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::datasources::packagist::PackagistUpdateSummary,
            renovate_core::datasources::packagist::PackagistError,
        >,
    >,
    repo_cfg: &RepoConfig,
) -> Vec<output::DepReport> {
    let mut reports = Vec::new();
    for dep in all_deps.iter().filter(|d| d.skip_reason.is_some()) {
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
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.name.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let summary = update_map.get(&dep.name);
        let release_timestamp = summary
            .and_then(|r| r.as_ref().ok())
            .and_then(|s| s.release_timestamp.clone());
        let status = match summary {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_value.clone(),
                latest: s.latest.clone().unwrap_or_default(),
            },
            Some(Ok(s)) => output::DepStatus::UpToDate {
                latest: s.latest.clone(),
            },
            Some(Err(e)) => output::DepStatus::LookupError {
                message: e.to_string(),
            },
            None => output::DepStatus::UpToDate { latest: None },
        };
        let (branch_name, pr_title, update_type) = match &status {
            output::DepStatus::UpdateAvailable { current, latest } => {
                dep_meta(&dep.name, current, latest, repo_cfg)
            }
            _ => (None, None, None),
        };
        reports.push(output::DepReport {
            branch_name,
            group_name: None,
            automerge: None,
            labels: Vec::new(),
            assignees: Vec::new(),
            reviewers: Vec::new(),
            update_type,
            pr_priority: None,
            pr_title,
            release_timestamp,
            current_version_timestamp: None,

            dep_type: Some(dep.dep_type.as_renovate_str().to_owned()),
            package_name: None,
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.name.clone(),
            status,
        });
    }
    reports
}

pub(crate) fn build_dep_reports_gomod(
    all_deps: &[renovate_core::extractors::gomod::GoModExtractedDep],
    actionable: &[&renovate_core::extractors::gomod::GoModExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::datasources::gomod::GoModUpdateSummary,
            renovate_core::datasources::gomod::GoModError,
        >,
    >,
    repo_cfg: &RepoConfig,
) -> Vec<output::DepReport> {
    let mut reports = Vec::new();
    for dep in all_deps.iter().filter(|d| d.skip_reason.is_some()) {
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
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.module_path.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let summary = update_map.get(&dep.module_path);
        let release_timestamp = summary
            .and_then(|r| r.as_ref().ok())
            .and_then(|s| s.release_timestamp.clone());
        let status = match summary {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_value.clone(),
                latest: s.latest.clone().unwrap_or_default(),
            },
            Some(Ok(s)) => output::DepStatus::UpToDate {
                latest: s.latest.clone(),
            },
            Some(Err(e)) => output::DepStatus::LookupError {
                message: e.to_string(),
            },
            None => output::DepStatus::UpToDate { latest: None },
        };
        let (branch_name, pr_title, update_type) = match &status {
            output::DepStatus::UpdateAvailable { current, latest } => {
                dep_meta(&dep.module_path, current, latest, repo_cfg)
            }
            _ => (None, None, None),
        };
        reports.push(output::DepReport {
            branch_name,
            group_name: None,
            automerge: None,
            labels: Vec::new(),
            assignees: Vec::new(),
            reviewers: Vec::new(),
            update_type,
            pr_priority: None,
            pr_title,
            release_timestamp,
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
            new_value: None,
            name: dep.module_path.clone(),
            status,
        });
    }
    reports
}

pub(crate) fn build_dep_reports_poetry(
    all_deps: &[renovate_core::extractors::poetry::PoetryExtractedDep],
    actionable: &[&renovate_core::extractors::poetry::PoetryExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::versioning::pep440::Pep440UpdateSummary,
            renovate_core::datasources::pypi::PypiError,
        >,
    >,
    repo_cfg: &RepoConfig,
) -> Vec<output::DepReport> {
    let mut reports = Vec::new();
    for dep in all_deps.iter().filter(|d| d.skip_reason.is_some()) {
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
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.name.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let summary = update_map.get(&dep.name);
        let release_timestamp = summary
            .and_then(|r| r.as_ref().ok())
            .and_then(|s| s.latest_timestamp.clone());
        let current_version_timestamp = summary
            .and_then(|r| r.as_ref().ok())
            .and_then(|s| s.current_version_timestamp.clone());
        let status = match summary {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_specifier.clone(),
                latest: s.latest.clone().unwrap_or_default(),
            },
            Some(Ok(s)) => output::DepStatus::UpToDate {
                latest: s.latest.clone(),
            },
            Some(Err(e)) => output::DepStatus::LookupError {
                message: e.to_string(),
            },
            None => output::DepStatus::UpToDate { latest: None },
        };
                let (branch_name, pr_title, update_type) = match &status {
            output::DepStatus::UpdateAvailable { current, latest } => {
                dep_meta(&dep.name, current, latest, repo_cfg)
            }
            _ => (None, None, None),
        };
        reports.push(output::DepReport {
            branch_name,
            group_name: None,
            automerge: None,
            labels: Vec::new(),
            assignees: Vec::new(),
            reviewers: Vec::new(),
            update_type,
            pr_priority: None,
            pr_title,
            release_timestamp,
            current_version_timestamp,
            dep_type: Some(dep.dep_type.as_renovate_str().to_owned()),
            package_name: dep.package_name.clone(),
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.name.clone(),
            status,
        });
    }
    reports
}

pub(crate) fn build_dep_reports_pip(
    all_deps: &[renovate_core::extractors::pip::PipExtractedDep],
    actionable: &[&renovate_core::extractors::pip::PipExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::versioning::pep440::Pep440UpdateSummary,
            renovate_core::datasources::pypi::PypiError,
        >,
    >,
    repo_cfg: &RepoConfig,
) -> Vec<output::DepReport> {
    let mut reports = Vec::new();
    for dep in all_deps.iter().filter(|d| d.skip_reason.is_some()) {
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
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.name.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let summary = update_map.get(&dep.name);
        let release_timestamp = summary
            .and_then(|r| r.as_ref().ok())
            .and_then(|s| s.latest_timestamp.clone());
        let current_version_timestamp = summary
            .and_then(|r| r.as_ref().ok())
            .and_then(|s| s.current_version_timestamp.clone());
        let status = match summary {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_specifier.clone(),
                latest: s.latest.clone().unwrap_or_default(),
            },
            Some(Ok(s)) => output::DepStatus::UpToDate {
                latest: s.latest.clone(),
            },
            Some(Err(e)) => output::DepStatus::LookupError {
                message: e.to_string(),
            },
            None => output::DepStatus::UpToDate { latest: None },
        };
                let (branch_name, pr_title, update_type) = match &status {
            output::DepStatus::UpdateAvailable { current, latest } => {
                dep_meta(&dep.name, current, latest, repo_cfg)
            }
            _ => (None, None, None),
        };
        reports.push(output::DepReport {
            branch_name,
            group_name: None,
            automerge: None,
            labels: Vec::new(),
            assignees: Vec::new(),
            reviewers: Vec::new(),
            update_type,
            pr_priority: None,
            pr_title,
            release_timestamp,
            current_version_timestamp,
            dep_type: None,
            package_name: None,
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.name.clone(),
            status,
        });
    }
    reports
}

pub(crate) fn build_dep_reports_bundler(
    all_deps: &[renovate_core::extractors::bundler::BundlerExtractedDep],
    actionable: &[&renovate_core::extractors::bundler::BundlerExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::datasources::rubygems::GemUpdateSummary,
            renovate_core::datasources::rubygems::RubyGemsError,
        >,
    >,
    repo_cfg: &RepoConfig,
) -> Vec<output::DepReport> {
    let mut reports = Vec::new();
    for dep in all_deps.iter().filter(|d| d.skip_reason.is_some()) {
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
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.name.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let summary = update_map.get(&dep.name);
        let release_timestamp = summary
            .and_then(|r| r.as_ref().ok())
            .and_then(|s| s.release_timestamp.clone());
        let status = match summary {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_value.clone(),
                latest: s.latest.clone().unwrap_or_default(),
            },
            Some(Ok(s)) => output::DepStatus::UpToDate {
                latest: s.latest.clone(),
            },
            Some(Err(e)) => output::DepStatus::LookupError {
                message: e.to_string(),
            },
            None => output::DepStatus::UpToDate { latest: None },
        };
                let (branch_name, pr_title, update_type) = match &status {
            output::DepStatus::UpdateAvailable { current, latest } => {
                dep_meta(&dep.name, current, latest, repo_cfg)
            }
            _ => (None, None, None),
        };
        reports.push(output::DepReport {
            branch_name,
            group_name: None,
            automerge: None,
            labels: Vec::new(),
            assignees: Vec::new(),
            reviewers: Vec::new(),
            update_type,
            pr_priority: None,
            pr_title,
            release_timestamp,
            current_version_timestamp: None,

            dep_type: Some(dep.dep_type.as_renovate_str().to_owned()),
            package_name: None,
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.name.clone(),
            status,
        });
    }
    reports
}

pub(crate) fn build_dep_reports_terraform(
    all_deps: &[renovate_core::extractors::terraform::TerraformExtractedDep],
    actionable: &[&renovate_core::extractors::terraform::TerraformExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::datasources::terraform::TerraformUpdateSummary,
            renovate_core::datasources::terraform::TerraformError,
        >,
    >,
    repo_cfg: &RepoConfig,
) -> Vec<output::DepReport> {
    let mut reports = Vec::new();
    for dep in all_deps.iter().filter(|d| d.skip_reason.is_some()) {
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
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.name.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let status = match update_map.get(&dep.name) {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_value.clone(),
                latest: s.latest.clone().unwrap_or_default(),
            },
            Some(Ok(s)) => output::DepStatus::UpToDate {
                latest: s.latest.clone(),
            },
            Some(Err(e)) => output::DepStatus::LookupError {
                message: e.to_string(),
            },
            None => output::DepStatus::UpToDate { latest: None },
        };
                let (branch_name, pr_title, update_type) = match &status {
            output::DepStatus::UpdateAvailable { current, latest } => {
                dep_meta(&dep.name, current, latest, repo_cfg)
            }
            _ => (None, None, None),
        };
        reports.push(output::DepReport {
            branch_name,
            group_name: None,
            automerge: None,
            labels: Vec::new(),
            assignees: Vec::new(),
            reviewers: Vec::new(),
            update_type,
            pr_priority: None,
            pr_title,
            release_timestamp: None,
            current_version_timestamp: None,

            dep_type: Some(dep.dep_type.as_renovate_str().to_owned()),
            package_name: dep.package_name.clone(),
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.name.clone(),
            status,
        });
    }
    reports
}

pub(crate) fn build_dep_reports_helm(
    all_deps: &[renovate_core::extractors::helm::HelmExtractedDep],
    actionable: &[&renovate_core::extractors::helm::HelmExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::datasources::helm::HelmUpdateSummary,
            renovate_core::datasources::helm::HelmError,
        >,
    >,
    repo_cfg: &RepoConfig,
) -> Vec<output::DepReport> {
    let mut reports = Vec::new();
    for dep in all_deps.iter().filter(|d| d.skip_reason.is_some()) {
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
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.name.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let summary = update_map.get(&dep.name);
        let release_timestamp = summary
            .and_then(|r| r.as_ref().ok())
            .and_then(|s| s.release_timestamp.clone());
        let status = match summary {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_value.clone(),
                latest: s.latest.clone().unwrap_or_default(),
            },
            Some(Ok(s)) => output::DepStatus::UpToDate {
                latest: s.latest.clone(),
            },
            Some(Err(e)) => output::DepStatus::LookupError {
                message: e.to_string(),
            },
            None => output::DepStatus::UpToDate { latest: None },
        };
                let (branch_name, pr_title, update_type) = match &status {
            output::DepStatus::UpdateAvailable { current, latest } => {
                dep_meta(&dep.name, current, latest, repo_cfg)
            }
            _ => (None, None, None),
        };
        reports.push(output::DepReport {
            branch_name,
            group_name: None,
            automerge: None,
            labels: Vec::new(),
            assignees: Vec::new(),
            reviewers: Vec::new(),
            update_type,
            pr_priority: None,
            pr_title,
            release_timestamp,
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
            new_value: None,
            name: dep.name.clone(),
            status,
        });
    }
    reports
}

pub(crate) fn build_dep_reports_gradle(
    all_deps: &[renovate_core::extractors::gradle::GradleExtractedDep],
    actionable: &[&renovate_core::extractors::gradle::GradleExtractedDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::datasources::maven::MavenUpdateSummary,
            renovate_core::datasources::maven::MavenError,
        >,
    >,
    repo_cfg: &RepoConfig,
) -> Vec<output::DepReport> {
    let mut reports = Vec::new();
    for dep in all_deps.iter().filter(|d| d.skip_reason.is_some()) {
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
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.dep_name.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let status = match update_map.get(&dep.dep_name) {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_version.clone(),
                latest: s.latest.clone().unwrap_or_default(),
            },
            Some(Ok(s)) => output::DepStatus::UpToDate {
                latest: s.latest.clone(),
            },
            Some(Err(e)) => output::DepStatus::LookupError {
                message: e.to_string(),
            },
            None => output::DepStatus::UpToDate { latest: None },
        };
                let (branch_name, pr_title, update_type) = match &status {
            output::DepStatus::UpdateAvailable { current, latest } => {
                dep_meta(&dep.dep_name, current, latest, repo_cfg)
            }
            _ => (None, None, None),
        };
        reports.push(output::DepReport {
            branch_name,
            group_name: None,
            automerge: None,
            labels: Vec::new(),
            assignees: Vec::new(),
            reviewers: Vec::new(),
            update_type,
            pr_priority: None,
            pr_title,
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
            new_value: None,
            name: dep.dep_name.clone(),
            status,
        });
    }
    reports
}

pub(crate) fn build_dep_reports_setup_cfg(
    all_deps: &[renovate_core::extractors::setup_cfg::SetupCfgDep],
    actionable: &[&renovate_core::extractors::setup_cfg::SetupCfgDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::versioning::pep440::Pep440UpdateSummary,
            renovate_core::datasources::pypi::PypiError,
        >,
    >,
    repo_cfg: &RepoConfig,
) -> Vec<output::DepReport> {
    let mut reports = Vec::new();
    for dep in all_deps.iter().filter(|d| d.skip_reason.is_some()) {
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
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.name.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let summary = update_map.get(&dep.name);
        let release_timestamp = summary
            .and_then(|r| r.as_ref().ok())
            .and_then(|s| s.latest_timestamp.clone());
        let current_version_timestamp = summary
            .and_then(|r| r.as_ref().ok())
            .and_then(|s| s.current_version_timestamp.clone());
        let status = match summary {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_specifier.clone(),
                latest: s.latest.clone().unwrap_or_default(),
            },
            Some(Ok(s)) => output::DepStatus::UpToDate {
                latest: s.latest.clone(),
            },
            Some(Err(e)) => output::DepStatus::LookupError {
                message: e.to_string(),
            },
            None => output::DepStatus::UpToDate { latest: None },
        };
                let (branch_name, pr_title, update_type) = match &status {
            output::DepStatus::UpdateAvailable { current, latest } => {
                dep_meta(&dep.name, current, latest, repo_cfg)
            }
            _ => (None, None, None),
        };
        reports.push(output::DepReport {
            branch_name,
            group_name: None,
            automerge: None,
            labels: Vec::new(),
            assignees: Vec::new(),
            reviewers: Vec::new(),
            update_type,
            pr_priority: None,
            pr_title,
            release_timestamp,
            current_version_timestamp,
            dep_type: None,
            package_name: None,
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.name.clone(),
            status,
        });
    }
    reports
}

pub(crate) fn build_dep_reports_pipfile(
    all_deps: &[renovate_core::extractors::pipfile::PipfileDep],
    actionable: &[&renovate_core::extractors::pipfile::PipfileDep],
    update_map: &HashMap<
        String,
        Result<
            renovate_core::versioning::pep440::Pep440UpdateSummary,
            renovate_core::datasources::pypi::PypiError,
        >,
    >,
    repo_cfg: &RepoConfig,
) -> Vec<output::DepReport> {
    let mut reports = Vec::new();
    for dep in all_deps.iter().filter(|d| d.skip_reason.is_some()) {
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
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.name.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let summary = update_map.get(&dep.name);
        let release_timestamp = summary
            .and_then(|r| r.as_ref().ok())
            .and_then(|s| s.latest_timestamp.clone());
        let current_version_timestamp = summary
            .and_then(|r| r.as_ref().ok())
            .and_then(|s| s.current_version_timestamp.clone());
        let status = match summary {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_specifier.clone(),
                latest: s.latest.clone().unwrap_or_default(),
            },
            Some(Ok(s)) => output::DepStatus::UpToDate {
                latest: s.latest.clone(),
            },
            Some(Err(e)) => output::DepStatus::LookupError {
                message: e.to_string(),
            },
            None => output::DepStatus::UpToDate { latest: None },
        };
        let (branch_name, pr_title, update_type) = match &status {
            output::DepStatus::UpdateAvailable { current, latest } => {
                dep_meta(&dep.name, current, latest, repo_cfg)
            }
            _ => (None, None, None),
        };
        reports.push(output::DepReport {
            branch_name,
            group_name: None,
            automerge: None,
            labels: Vec::new(),
            assignees: Vec::new(),
            reviewers: Vec::new(),
            update_type,
            pr_priority: None,
            pr_title,
            release_timestamp,
            current_version_timestamp,
            dep_type: None,
            package_name: None,
            range_strategy: None,
            follow_tag: None,
            pin_digests: None,
            versioning: None,
            dependency_dashboard_approval: None,
            replacement_name: None,
            replacement_version: None,
            new_value: None,
            name: dep.name.clone(),
            status,
        });
    }
    reports
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_repo_cfg() -> RepoConfig {
        RepoConfig {
            enabled: true,
            ignore_deps: Vec::new(),
            update_internal_deps: false,
            ignore_paths: Vec::new(),
            include_paths: Vec::new(),
            exclude_commit_paths: Vec::new(),
            registry_aliases: std::collections::BTreeMap::new(),
            package_rules: Vec::new(),
            enabled_managers: Vec::new(),
            disabled_managers: Vec::new(),
            git_ignored_authors: Vec::new(),
            ignore_versions: Vec::new(),
            schedule: Vec::new(),
            automerge_schedule: Vec::new(),
            timezone: None,
            automerge: false,
            automerge_type: None,
            automerge_strategy: String::new(),
            automerge_comment: String::new(),
            ignore_tests: false,
            prune_branch_after_automerge: false,
            labels: Vec::new(),
            add_labels: Vec::new(),
            assignees: Vec::new(),
            assignees_from_code_owners: false,
            expand_code_owners_groups: false,
            assignees_sample_size: None,
            reviewers: Vec::new(),
            ignore_reviewers: Vec::new(),
            reviewers_from_code_owners: false,
            filter_unavailable_users: false,
            fork_mode_disallow_maintainer_edits: false,
            confidential: false,
            reviewers_sample_size: None,
            additional_reviewers: Vec::new(),
            bb_auto_resolve_pr_tasks: false,
            bb_use_default_reviewers: false,
            draft_pr: false,
            azure_work_item_id: 0,
            auto_approve: false,
            assign_automerge: false,
            platform_automerge: true,
            keep_updated_label: None,
            rebase_label: String::new(),
            stop_updating_label: String::new(),
            branch_prefix: "renovate/".to_owned(),
            branch_prefix_old: String::new(),
            branch_name: String::new(),
            additional_branch_prefix: String::new(),
            branch_name_strict: false,
            separate_minor_patch: false,
            separate_multiple_minor: false,
            hashed_branch_length: None,
            semantic_commits: None,
            semantic_commit_type: "chore".to_owned(),
            semantic_commit_scope: "deps".to_owned(),
            max_major_increment: 500,
            ignore_unstable: true,
            update_not_scheduled: true,
            dependency_dashboard: false,
            dependency_dashboard_title: String::new(),
            dependency_dashboard_header: String::new(),
            dependency_dashboard_footer: String::new(),
            dependency_dashboard_labels: Vec::new(),
            recreate_when: String::new(),
            rebase_when: String::new(),
            recreate_closed: false,
            delete_branch_after_merge: false,
            delete_branch_on_close: false,
            branch_concurrent_limit: 0,
            pr_concurrent_limit: 0,
            pr_hourly_limit: 0,
            commit_message: String::new(),
            commit_message_prefix: String::new(),
            commit_message_action: String::new(),
            commit_message_topic: String::new(),
            commit_message_extra: String::new(),
            commit_message_suffix: String::new(),
            pr_body: String::new(),
            pr_body_columns: Vec::new(),
            pr_body_definitions: std::collections::HashMap::new(),
            pr_body_notes: Vec::new(),
            pr_creation: String::new(),
            pr_priority: 0,
            pr_not_pending_hours: 0,
            minimum_release_age: None,
            minimum_release_age_behaviour: String::new(),
            internal_checks_filter: String::new(),
            internal_checks_merge_confidence: Vec::new(),
            stop_updating_message: String::new(),
            stop_updating_title: String::new(),
            vulnerability_fix_strategy: String::new(),
            vulnerability_range_strategy: String::new(),
            vulnerability_commit_message_suffix: String::new(),
            vulnerability_branch_topic: String::new(),
            vulnerability_pr_creation: String::new(),
            lock_file_recreate_when: String::new(),
            lock_file_rebase_stale_prs: false,
            lock_file_branch_topic: String::new(),
            lock_file_commit_message_action: String::new(),
            lock_file_schedule: Vec::new(),
            lock_file_pr_body_definitions: std::collections::HashMap::new(),
            group_branch_topic: String::new(),
            group_commit_message_topic: String::new(),
            group_name: None,
            group_slug: None,
            match_update_types: Vec::new(),
            match_datasources: Vec::new(),
            match_categories: Vec::new(),
            match_managers: Vec::new(),
            match_dep_types: Vec::new(),
            match_package_names: Vec::new(),
            match_dep_names: Vec::new(),
            match_current_value: Vec::new(),
            match_current_version: Vec::new(),
            match_new_value: Vec::new(),
            match_file_names: Vec::new(),
            match_paths: Vec::new(),
            match_repositories: Vec::new(),
            match_base_branches: Vec::new(),
            match_labels: Vec::new(),
            ignore_unstable_datasources: Vec::new(),
            pin_digests: false,
            pin_dependencies: false,
            range_strategy: None,
            rollback_pr: false,
            use_base_branch_config: String::new(),
            git_no_verify: Vec::new(),
            platform_commit: false,
            git_url: None,
            optimize_for_disabled: false,
            unicode_emoji: false,
            require_config: None,
            onboarding: true,
            onboarding_branch: String::new(),
            onboarding_pr_title: String::new(),
            onboarding_commit_message: String::new(),
            config_migration: false,
            config_migration_branch: String::new(),
            config_migration_commit_message: String::new(),
            config_migration_pr_title: String::new(),
            cache_ttls: std::collections::HashMap::new(),
            host_rules: Vec::new(),
            custom_env_vars: std::collections::HashMap::new(),
            post_update_options: Vec::new(),
            allow_custom_crate_registries: false,
            allow_scripts: false,
            allowed_post_upgrade_commands: Vec::new(),
            binary_source: String::new(),
            docker_user: None,
            docker_password: None,
            merge_confidence_endpoint: None,
            osvvulnerability: false,
            osv_no_single_match: false,
            clone_submodules: false,
            clone_short: false,
            clone_depth: None,
            persist_repo_data: false,
            write_discovered_repos: false,
            checked_branches: Vec::new(),
            pr_footer: String::new(),
            pr_header: String::new(),
            include_release_notes: true,
            fetch_release_notes: true,
            fetch_change_log: true,
            fetch_commits: true,
            include_changelogs: true,
            suppress_notifications: Vec::new(),
            autodiscover: false,
            autodiscover_filter: Vec::new(),
            autodiscover_topics: Vec::new(),
            autodiscover_sort: String::new(),
            autodiscover_namespace: None,
            base_branches: Vec::new(),
            default_branch: String::new(),
        }
    }

    #[test]
    fn dep_meta_major_update() {
        let cfg = test_repo_cfg();
        let (branch, title, utype) = dep_meta("lodash", "^4.0.0", "5.1.2", &cfg);
        assert_eq!(branch, Some("renovate/lodash-5.x".to_owned()));
        assert_eq!(title, Some("Update dependency lodash to v5".to_owned()));
        assert_eq!(utype, Some("major".to_owned()));
    }

    #[test]
    fn dep_meta_minor_update() {
        let cfg = test_repo_cfg();
        let (branch, title, utype) = dep_meta("lodash", "^4.0.0", "4.17.21", &cfg);
        assert_eq!(branch, Some("renovate/lodash-4.x".to_owned()));
        assert_eq!(title, Some("Update dependency lodash to v4.17.21".to_owned()));
        assert_eq!(utype, Some("minor".to_owned()));
    }

    #[test]
    fn dep_meta_patch_update() {
        let mut cfg = test_repo_cfg();
        cfg.separate_minor_patch = true;
        let (branch, title, utype) = dep_meta("lodash", "^4.17.0", "4.17.21", &cfg);
        assert_eq!(branch, Some("renovate/lodash-4.17.x".to_owned()));
        assert_eq!(title, Some("Update dependency lodash to v4.17.21".to_owned()));
        assert_eq!(utype, Some("patch".to_owned()));
    }

    #[test]
    fn dep_meta_scoped_package() {
        let cfg = test_repo_cfg();
        let (branch, title, utype) = dep_meta("@angular/core", "^17.0.0", "17.1.0", &cfg);
        assert_eq!(branch, Some("renovate/angular-core-17.x".to_owned()));
        assert_eq!(title, Some("Update dependency angular-core to v17.1.0".to_owned()));
        assert_eq!(utype, Some("minor".to_owned()));
    }

    #[test]
    fn dep_meta_semantic_commits() {
        let mut cfg = test_repo_cfg();
        cfg.semantic_commits = Some("enabled");
        let (_branch, title, _utype) = dep_meta("express", "4.18.2", "4.19.0", &cfg);
        assert_eq!(title, Some("chore(deps): Update dependency express to v4.19.0".to_owned()));
    }

    #[test]
    fn dep_meta_hashed_branch_length() {
        let mut cfg = test_repo_cfg();
        cfg.hashed_branch_length = Some(20);
        let (branch, _title, _utype) = dep_meta("lodash", "^4.0.0", "5.1.2", &cfg);
        assert!(branch.as_ref().unwrap().starts_with("renovate/"));
        assert_eq!(branch.unwrap().len(), 20);
    }
}
