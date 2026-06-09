//! Get updated package files (getUpdatedPackageFiles: collect contents via auto-replace or manager updateDependency/updateLockedDependency, update artifacts, check pending versions from minReleaseAge, return PackageFilesResult with updatedPackageFiles + artifacts + errors + notices).
//!
//! Mirrors `lib/workers/repository/update/branch/get-updated.ts`.

#![allow(
    unused,
    unused_mut,
    unused_variables,
    unused_assignments,
    dead_code,
    reason = "Port debt in this unit; strict unused lints from workspace deny; will clean as port completes."
)]

use std::collections::{HashMap, HashSet};

/// Local stubs/types for parity with TS BranchConfig / results (used only within this unit for now).
#[derive(Debug, Clone, Default)]
pub struct BranchConfig {
    pub base_branch: Option<String>,
    pub manager: Option<String>,
    pub branch_name: Option<String>,
    pub upgrades: Vec<BranchUpgradeConfig>,
    pub reuse_existing_branch: Option<bool>,
    pub package_files: Option<HashMap<String, Vec<PackageFileEntry>>>,
    pub minimum_release_age_behaviour: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct BranchUpgradeConfig {
    pub manager: Option<String>,
    pub package_file: Option<String>,
    pub dep_name: Option<String>,
    pub new_version: Option<String>,
    pub current_version: Option<String>,
    pub lock_file: Option<String>,
    pub lock_files: Option<Vec<String>>,
    pub update_type: Option<String>,
    pub is_remediation: Option<bool>,
    pub is_lockfile_update: Option<bool>,
    pub bump_version: Option<bool>,
    pub package_file_version: Option<String>,
    pub pending_versions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default)]
pub struct PackageFileEntry {
    pub package_file: String,
}

#[derive(Debug, Clone, Default)]
pub struct PackageFilesResult {
    pub artifact_errors: Vec<ArtifactError>,
    pub reuse_existing_branch: Option<bool>,
    pub updated_package_files: Vec<FileAddition>,
    pub updated_artifacts: Vec<FileChange>,
    pub artifact_notices: Vec<ArtifactNotice>,
}

#[derive(Debug, Clone, Default)]
pub struct FileAddition {
    pub r#type: String, // "addition"
    pub path: String,
    pub contents: String,
}

#[derive(Debug, Clone, Default)]
pub struct FileChange {
    pub r#type: String,
    pub path: String,
    pub contents: Option<String>,
    pub is_symlink: Option<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct ArtifactError {
    pub file_name: Option<String>,
    pub stderr: String,
}

#[derive(Debug, Clone, Default)]
pub struct ArtifactNotice {
    pub file: Option<String>,
    pub notice: Option<String>,
}

/// Stubs for cross-module (full impls in pending sibling units or core util/*).
mod git {
    pub async fn get_file(_path: &str, _branch: Option<&str>) -> Option<String> {
        None
    }
}

mod logger {
    pub fn trace<T: std::fmt::Debug>(_o: &T, _m: Option<&str>) {}
    pub fn debug(_m: &str) {}
    pub fn error<T: std::fmt::Debug>(_o: &T, _m: &str) {}
    pub fn warn<T: std::fmt::Debug>(_o: &T, _m: &str) {}
}

const WORKER_FILE_UPDATE_FAILED: &str = "file-update-failed";

mod manager {
    use super::BranchConfig;
    use super::BranchUpgradeConfig;
    pub fn get(_manager: &str, _key: &str) -> Option<fn()> {
        None
    }
    pub async fn extract_package_file(
        _manager: &str,
        _content: &str,
        _file: &str,
        _config: &BranchConfig,
    ) -> Option<Extracted> {
        None
    }
    #[derive(Default)]
    pub struct Extracted {
        pub deps: Vec<ExtractedDep>,
    }
    #[derive(Default)]
    pub struct ExtractedDep {
        pub dep_name: Option<String>,
        pub package_name: Option<String>,
        pub locked_version: Option<String>,
        pub new_version: Option<String>,
        pub current_version: Option<String>,
        pub current_value: Option<String>,
    }
}

mod auto_replace {
    use super::BranchUpgradeConfig;
    pub fn do_auto_replace(
        _upgrade: &BranchUpgradeConfig,
        _content: &str,
        _reuse: bool,
        _first: bool,
    ) -> Option<String> {
        None
    }
}

mod util {
    pub fn is_non_empty_array<T>(v: &Option<Vec<T>>) -> bool {
        v.as_ref().map_or(false, |x| !x.is_empty())
    }
    pub fn coerce_string(s: String, _fallback: String) -> String {
        s
    }
}

/// Port helpers (getFileContent, sort, hasAny, getManagersFor... etc) — simplified for parity + stubs.
fn get_file_content(
    updated: &HashMap<String, String>,
    path: &str,
    config: &BranchConfig,
) -> Option<String> {
    if let Some(c) = updated.get(path) {
        return Some(c.clone());
    }
    let branch = if config.reuse_existing_branch.unwrap_or(false) {
        config.branch_name.clone()
    } else {
        config.base_branch.clone()
    };
    // sync stub (real git get is async in TS; not exercised by 'handles empty')
    None
}

fn has_any(set: &HashSet<String>, targets: &HashSet<String>) -> bool {
    for t in targets {
        if set.contains(t) {
            return true;
        }
    }
    false
}

fn get_managers_for_package_files<T: AsRef<str>>(
    files: &[T],
    manager_files: &HashMap<String, HashSet<String>>,
) -> HashSet<String> {
    let names: HashSet<_> = files.iter().map(|f| f.as_ref().to_string()).collect();
    manager_files
        .keys()
        .filter(|m| has_any(&manager_files[*m], &names))
        .cloned()
        .collect()
}

fn sort_package_files(_config: &BranchConfig, _manager: &str, _files: &mut Vec<impl AsRef<str>>) {
    // stub (order not critical for empty path)
}

fn update_artifacts(_cfg: UpdateArtifactsConfig) -> Option<Vec<UpdateArtifactsResult>> {
    None
}

#[derive(Default)]
pub struct UpdateArtifactsConfig {
    pub package_file: String,
    pub updated_package_files: Vec<FileAddition>,
    pub config: BranchConfig,
    pub artifact_errors: Vec<ArtifactError>,
    pub artifact_notices: Vec<ArtifactNotice>,
}

#[derive(Default)]
pub struct UpdateArtifactsResult {
    pub file: Option<FileChange>,
    pub notice: Option<ArtifactNotice>,
    pub artifact_error: Option<ArtifactError>,
}

fn update_locked_dependency(
    _args: &BranchUpgradeConfig,
) -> (String, Option<HashMap<String, String>>) {
    ("unsupported".to_string(), None)
}

fn check_for_pending_versions(
    _manager: &str,
    _pkg: &str,
    _content: &str,
    _deps: &[BranchUpgradeConfig],
    _errors: &mut Vec<ArtifactError>,
    _config: &BranchConfig,
) {
    // stub (hit only on non-empty + min age cases)
}

fn apply_manager_bump_package_version(
    content: Option<String>,
    _upgrade: &BranchUpgradeConfig,
) -> Option<String> {
    content
}

/// Main port of getUpdatedPackageFiles (structure mirrors TS; many manager/datasource paths stubbed since full wiring pending other units).
pub fn get_updated_package_files(config: BranchConfig) -> PackageFilesResult {
    logger::trace(&config, None);
    let reuse_existing_branch = config.reuse_existing_branch.unwrap_or(false);
    logger::debug(&format!(
        "manager.getUpdatedPackageFiles() reuseExistingBranch={}",
        reuse_existing_branch
    ));

    let mut updated_file_contents: HashMap<String, String> = HashMap::new();
    let mut non_updated_file_contents: HashMap<String, String> = HashMap::new();
    let mut manager_package_files: HashMap<String, HashSet<String>> = HashMap::new();
    let mut package_file_updated_deps: HashMap<String, Vec<BranchUpgradeConfig>> = HashMap::new();
    let mut lock_file_maintenance_files: Vec<String> = vec![];
    let mut first_update = true;

    for upgrade in &config.upgrades {
        let manager = upgrade.manager.clone().unwrap_or_default();
        let package_file = upgrade.package_file.clone().unwrap_or_default();
        let dep_name = upgrade.dep_name.clone().unwrap_or_default();
        let new_version = upgrade.new_version.clone().unwrap_or_default();
        let current_version = upgrade.current_version.clone().unwrap_or_default();
        // (dynamic manager get for updateLockedDependency stubbed out to avoid shadow of local fn + Option call; direct fn used below)

        manager_package_files
            .entry(manager.clone())
            .or_default()
            .insert(package_file.clone());
        package_file_updated_deps
            .entry(package_file.clone())
            .or_default()
            .push(upgrade.clone());

        let package_file_content = get_file_content(&updated_file_contents, &package_file, &config);
        let mut lock_file_content: Option<String> = None;
        let lock_file = upgrade
            .lock_file
            .clone()
            .or_else(|| upgrade.lock_files.as_ref().and_then(|l| l.first().cloned()))
            .unwrap_or_default();
        if !lock_file.is_empty() {
            lock_file_content = get_file_content(&updated_file_contents, &lock_file, &config);
        }

        if reuse_existing_branch
            && (package_file_content.is_none()
                || (!lock_file.is_empty() && lock_file_content.is_none()))
        {
            logger::debug(&format!("Rebasing branch after file not found"));
            // recursive re-call path (simplified; would pass updated config)
            return get_updated_package_files(BranchConfig {
                reuse_existing_branch: Some(false),
                ..config.clone()
            });
        }

        if upgrade.update_type.as_deref() == Some("lockFileMaintenance") {
            lock_file_maintenance_files.push(package_file.clone());
        } else if upgrade.is_remediation.unwrap_or(false) {
            let (status, files) = update_locked_dependency(upgrade);
            if reuse_existing_branch && status != "already-updated" {
                return get_updated_package_files(BranchConfig {
                    reuse_existing_branch: Some(false),
                    ..config.clone()
                });
            }
            if let Some(f) = files {
                for (k, v) in f {
                    updated_file_contents.insert(k.clone(), v);
                    non_updated_file_contents.remove(&k);
                }
            }
            if status == "update-failed" || status == "unsupported" {
                // upgrade.remediationNotPossible = true; (stub, mut not needed for empty)
            }
        } else if upgrade.is_lockfile_update.unwrap_or(false) {
            // stub (not exercised by unit tests for this cycle)
            logger::debug(&format!("isLockFileUpdate (stub)"));
        } else {
            // autoReplace / updateDependency path (stubbed)
            logger::debug(&format!("autoReplace/updateDep path (stub)"));
        }
    }

    let updated_package_files: Vec<FileAddition> = updated_file_contents
        .iter()
        .map(|(name, contents)| FileAddition {
            r#type: "addition".to_string(),
            path: name.clone(),
            contents: contents.clone(),
        })
        .collect();

    let mut updated_artifacts: Vec<FileChange> = vec![];
    let mut artifact_errors: Vec<ArtifactError> = vec![];
    let mut artifact_notices: Vec<ArtifactNotice> = vec![];

    if !updated_package_files.is_empty() {
        logger::debug("updateArtifacts for updatedPackageFiles");
        let updated_package_file_managers = get_managers_for_package_files(
            &updated_package_files
                .iter()
                .map(|f| f.path.as_str())
                .collect::<Vec<_>>(),
            &manager_package_files,
        );
        for manager in &updated_package_file_managers {
            // getPackageFilesForManager + updateArtifact etc (stubbed; not hit by empty test)
            let _ = update_artifacts(UpdateArtifactsConfig {
                package_file: manager.clone(),
                updated_package_files: updated_package_files.clone(),
                config: config.clone(),
                artifact_errors: artifact_errors.clone(),
                artifact_notices: artifact_notices.clone(),
            });
        }
        // would call checkForPendingVersions per manager etc (stubbed)
    }

    PackageFilesResult {
        artifact_errors,
        reuse_existing_branch: None,
        updated_package_files,
        updated_artifacts,
        artifact_notices,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_empty() {
        // Ported: "handles empty" — lib/workers/repository/update/branch/get-updated.spec.ts line 119
        let config = BranchConfig {
            base_branch: Some("base-branch".to_string()),
            manager: Some("some-manager".to_string()),
            branch_name: Some("renovate/pin".to_string()),
            upgrades: vec![],
            ..Default::default()
        };
        let res = get_updated_package_files(config);
        assert_eq!(res.artifact_errors.len(), 0);
        assert_eq!(res.reuse_existing_branch, None);
        assert_eq!(res.updated_artifacts.len(), 0);
        assert_eq!(res.updated_package_files.len(), 0);
        assert_eq!(res.artifact_notices.len(), 0);
    }
}

// @parity `lib/workers/repository/update/branch/get-updated.ts` partial — getUpdatedPackageFiles (loop upgrades for content via autoReplace/updateDependency/updateLocked, assemble FileAdditions, updateArtifacts + check pending versions for minReleaseAge, return PackageFilesResult); single test ported (covering "handles empty" — lib/workers/repository/update/branch/get-updated.spec.ts line 119). Full manager extract/updateLocked, artifact update, rebase recursion, lockFileMaintenance, remediation, git-submodules special, bumpVersion, pending version artifactErrors, and cross wiring pending other units.
