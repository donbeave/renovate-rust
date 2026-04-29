//! Report-building helpers that convert raw extracted deps + update-map
//! results into `output::DepReport` lists.
//!
//! Each `build_dep_reports_*` function is a pure data-mapping step: it merges
//! the full dependency list (including skipped entries) with the lookup results
//! produced by the corresponding datasource.

use std::collections::HashMap;

use crate::output;

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
    timestamps: &HashMap<String, renovate_core::datasources::crates_io::CrateTimestamps>,
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
            name: dep.dep_name.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let summary = update_map.get(&dep.dep_name).and_then(|r| r.as_ref().ok());
        let status = match update_map.get(&dep.dep_name) {
            Some(Ok(s)) if s.update_available => output::DepStatus::UpdateAvailable {
                current: s.current_constraint.clone(),
                latest: s.latest_compatible.clone().unwrap_or_default(),
            },
            Some(Ok(s)) => output::DepStatus::UpToDate {
                latest: s.latest_compatible.clone(),
            },
            Some(Err(e)) => output::DepStatus::LookupError {
                message: e.to_string(),
            },
            None => output::DepStatus::UpToDate { latest: None },
        };
        // Release timestamp for the latest compatible version.
        let release_timestamp = summary
            .and_then(|s| s.latest_compatible.as_deref())
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
            current_version_timestamp,
            dep_type: Some(dep.dep_type.as_renovate_str().to_owned()),
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
    version_timestamps: &HashMap<String, HashMap<String, String>>,
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
        // Resolve current_version_timestamp for exact pins (e.g. "4.17.21").
        // For ranges we don't know the installed version, so leave as None.
        let current_version_timestamp = {
            let stripped = dep.current_value.trim().trim_start_matches('=').trim();
            // Heuristic: no range operators + starts with digit → exact pin.
            let is_exact = stripped.starts_with(|c: char| c.is_ascii_digit())
                && !stripped.contains(['^', '~', '>', '<', '*', ' ', ',']);
            if is_exact {
                version_timestamps
                    .get(&dep.name)
                    .and_then(|ts| ts.get(stripped))
                    .cloned()
            } else {
                None
            }
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
            release_timestamp,
            current_version_timestamp,
            dep_type: Some(dep.dep_type.as_renovate_str().to_owned()),
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
            dep_type: Some(dep.dep_type.as_renovate_str().to_owned()),
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
            dep_type: Some(dep.dep_type.as_renovate_str().to_owned()),
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
            name: dep.package_id.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let status = match update_map.get(&dep.package_id) {
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

            dep_type: Some(dep.dep_type.as_renovate_str().to_owned()),
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

            dep_type: Some(dep.dep_type.as_renovate_str().to_owned()),
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
            name: dep.module_path.clone(),
            status: output::DepStatus::Skipped {
                reason: format!("{:?}", dep.skip_reason.as_ref().unwrap()).to_lowercase(),
            },
        });
    }
    for dep in actionable {
        let status = match update_map.get(&dep.module_path) {
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
            release_timestamp,
            current_version_timestamp: None,

            dep_type: Some(dep.dep_type.as_renovate_str().to_owned()),
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
            release_timestamp,
            current_version_timestamp,
            dep_type: None,
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

            dep_type: Some(dep.dep_type.as_renovate_str().to_owned()),
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

            dep_type: Some(dep.dep_type.as_renovate_str().to_owned()),
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
            release_timestamp,
            current_version_timestamp: None,

            dep_type: None,
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
            release_timestamp,
            current_version_timestamp: None,

            dep_type: None,
            name: dep.name.clone(),
            status,
        });
    }
    reports
}
