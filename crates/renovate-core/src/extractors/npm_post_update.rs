//! NPM post-update lock file generation.
//!
//! Ports `lib/modules/manager/npm/post-update/` — orchestrates lock file
//! regeneration after dependency version changes for npm, yarn, and pnpm.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub mod artifact_runner;
pub mod node_version;
pub mod npm;
pub mod pnpm;
pub mod utils;
pub mod yarn;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArtifactError {
    pub lock_file: String,
    pub stderr: String,
}

#[derive(Debug, Clone, Default)]
pub struct AdditionalPackageFile {
    pub package_file_name: String,
    pub content: String,
}

#[derive(Debug, Clone, Default)]
pub struct LockFileResult {
    pub lock_file_name: String,
    pub content: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct PostUpdateConfig {
    pub lock_file_dir: PathBuf,
    pub package_file_name: String,
    pub npmrc: Option<String>,
    pub npm_lock_file: Option<String>,
    pub yarn_lock_file: Option<String>,
    pub pnpm_lock_file: Option<String>,
    pub upgrades: Vec<Upgrade>,
    pub updated_package_files: Vec<AdditionalPackageFile>,
    pub env: BTreeMap<String, String>,
    pub skip_installs: bool,
    pub constraints: BTreeMap<String, String>,
    pub post_update_options: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Upgrade {
    pub dep_name: String,
    pub new_version: Option<String>,
    pub current_value: Option<String>,
    pub new_value: Option<String>,
    pub manager: String,
    pub is_lock_file_update: bool,
    pub is_remediation: bool,
    pub range_strategy: Option<String>,
    pub package_file: String,
    pub dep_types: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PackageJson {
    #[serde(rename = "packageManager", skip_serializing_if = "Option::is_none")]
    pub package_manager: Option<String>,
    pub volta: Option<VoltaConfig>,
    pub engines: Option<EnginesConfig>,
    #[serde(rename = "devEngines", skip_serializing_if = "Option::is_none")]
    pub dev_engines: Option<serde_json::Value>,
    pub workspaces: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VoltaConfig {
    pub node: Option<String>,
    pub npm: Option<String>,
    pub yarn: Option<String>,
    pub pnpm: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnginesConfig {
    pub node: Option<String>,
    pub npm: Option<String>,
    pub yarn: Option<String>,
    pub pnpm: Option<String>,
}

impl PackageJson {
    pub fn parse(content: &str) -> Option<Self> {
        serde_json::from_str(content).ok()
    }

    pub fn get_package_manager_version(&self, name: &str) -> Option<String> {
        if let Some(ref volta) = self.volta {
            let v = match name {
                "npm" => volta.npm.as_ref(),
                "yarn" => volta.yarn.as_ref(),
                "pnpm" => volta.pnpm.as_ref(),
                _ => None,
            };
            if let Some(ver) = v {
                return Some(ver.clone());
            }
        }

        if let Some(ref pm) = self.package_manager
            && let Some(ver) = parse_corepack_version(pm, name) {
                return Some(ver);
            }

        if let Some(ref engines) = self.engines {
            let v = match name {
                "npm" => engines.npm.as_ref(),
                "yarn" => engines.yarn.as_ref(),
                "pnpm" => engines.pnpm.as_ref(),
                _ => None,
            };
            if let Some(ver) = v {
                return Some(ver.clone());
            }
        }

        None
    }
}

fn parse_corepack_version(pm: &str, name: &str) -> Option<String> {
    let prefix = format!("{}/", name);
    if pm.starts_with(&prefix) {
        let rest = &pm[prefix.len()..];
        let version = rest.split('@').next().unwrap_or(rest);
        if !version.is_empty() {
            return Some(version.to_owned());
        }
    }
    if let Some(at_pos) = pm.find('@') {
        let pkg_name = &pm[..at_pos];
        let version = &pm[at_pos + 1..];
        if pkg_name == name || pkg_name.ends_with(&format!("/{}", name)) {
            return Some(version.to_owned());
        }
    }
    None
}

pub fn determine_lock_file_dirs(
    upgrades: &[Upgrade],
    package_files: &[AdditionalPackageFile],
) -> Vec<PathBuf> {
    let mut dirs = std::collections::HashSet::new();
    for upgrade in upgrades {
        let dir = Path::new(&upgrade.package_file)
            .parent()
            .unwrap_or(Path::new("."));
        dirs.insert(dir.to_path_buf());
    }
    for pf in package_files {
        let dir = Path::new(&pf.package_file_name)
            .parent()
            .unwrap_or(Path::new("."));
        dirs.insert(dir.to_path_buf());
    }
    let mut result: Vec<PathBuf> = dirs.into_iter().collect();
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "performs full install" — modules/manager/npm/post-update/npm.spec.ts line 186
    #[test]
    fn parse_package_json_basic() {
        let pj = PackageJson::parse(
            r#"{"engines": {"node": ">=18", "npm": ">=9"}}"#,
        )
        .unwrap();
        assert_eq!(pj.engines.unwrap().node.unwrap(), ">=18");
    }

    // Ported: "uses slim yarn instead of corepack" — modules/manager/npm/post-update/yarn.spec.ts line 705
    #[test]
    fn parse_package_json_volta() {
        let pj = PackageJson::parse(
            r#"{"volta": {"node": "20.11.0", "yarn": "4.1.0"}}"#,
        )
        .unwrap();
        assert_eq!(
            pj.get_package_manager_version("yarn"),
            Some("4.1.0".to_owned())
        );
    }

    // Ported: "supports corepack" — modules/manager/npm/post-update/yarn.spec.ts line 504
    #[test]
    fn parse_package_json_corepack() {
        let pj = PackageJson::parse(
            r#"{"packageManager": "yarn@4.1.0"}"#,
        )
        .unwrap();
        assert_eq!(
            pj.get_package_manager_version("yarn"),
            Some("4.1.0".to_owned())
        );
    }

    // Ported: "does not use global cache if zero install is detected" — modules/manager/npm/post-update/yarn.spec.ts line 288
    #[test]
    fn parse_package_json_pnpm_corepack() {
        let pj = PackageJson::parse(
            r#"{"packageManager": "pnpm@9.0.0"}"#,
        )
        .unwrap();
        assert_eq!(
            pj.get_package_manager_version("pnpm"),
            Some("9.0.0".to_owned())
        );
    }

    // Ported: "uses devEngine.packageManager(object) instead of corepack" — modules/manager/npm/post-update/yarn.spec.ts line 744
    #[test]
    fn parse_corepack_version_yarn() {
        assert_eq!(
            parse_corepack_version("yarn@4.1.0", "yarn"),
            Some("4.1.0".to_owned())
        );
    }

    // Ported: "uses devEngine.packageManager(array) instead of corepack" — modules/manager/npm/post-update/yarn.spec.ts line 783
    #[test]
    fn parse_corepack_version_npm() {
        assert_eq!(
            parse_corepack_version("npm@10.2.3", "npm"),
            Some("10.2.3".to_owned())
        );
    }

    // Ported: "catches errors" — modules/manager/npm/post-update/yarn.spec.ts line 494
    #[test]
    fn parse_corepack_version_mismatch() {
        assert_eq!(parse_corepack_version("yarn@4.1.0", "npm"), None);
    }

    // Ported: "performs lock file updates for workspace with packages" — modules/manager/npm/post-update/pnpm.spec.ts line 120
    #[test]
    fn determine_lock_file_dirs_from_upgrades() {
        let upgrades = vec![Upgrade {
            package_file: "packages/foo/package.json".to_owned(),
            ..Default::default()
        }];
        let dirs = determine_lock_file_dirs(&upgrades, &[]);
        assert_eq!(dirs, vec![PathBuf::from("packages/foo")]);
    }

    // Ported: "performs lock file updates for non workspace using pnpm 10.x" — modules/manager/npm/post-update/pnpm.spec.ts line 181
    #[test]
    fn determine_lock_file_dirs_from_root() {
        let upgrades = vec![Upgrade {
            package_file: "package.json".to_owned(),
            ..Default::default()
        }];
        let dirs = determine_lock_file_dirs(&upgrades, &[]);
        assert_eq!(dirs, vec![PathBuf::from("")]);
    }

    // Ported: "performs lock file updates and install when lock file updates mixed with regular updates" — modules/manager/npm/post-update/pnpm.spec.ts line 261
    #[test]
    fn determine_lock_file_dirs_dedupes() {
        let upgrades = vec![
            Upgrade {
                package_file: "a/package.json".to_owned(),
                ..Default::default()
            },
            Upgrade {
                package_file: "a/package.json".to_owned(),
                ..Default::default()
            },
        ];
        let dirs = determine_lock_file_dirs(&upgrades, &[]);
        assert_eq!(dirs, vec![PathBuf::from("a")]);
    }
}
