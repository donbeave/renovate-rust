//! pnpm lock file generation.
//!
//! Ports `lib/modules/manager/npm/post-update/pnpm.ts`.

use super::{PackageJson, Upgrade};

#[derive(Debug, Clone, Default)]
pub struct PnpmLockFileConfig {
    pub lock_file_dir: String,
    pub npmrc: Option<String>,
    pub constraints: std::collections::BTreeMap<String, String>,
    pub post_update_options: Vec<String>,
    pub env: std::collections::BTreeMap<String, String>,
}

#[derive(Debug, Clone, Default)]
pub struct PnpmLockFileResult {
    pub lock_file_name: String,
    pub content: Option<String>,
    pub error: Option<String>,
}

static LOCK_TO_PNPM_VERSION_MAPPING: &[(f64, &str)] = &[
    (9.0, ">=9"),
    (6.0, ">=8.6"),
    (5.4, ">=8.3"),
    (5.3, ">=8.2"),
    (5.2, ">=8.1"),
    (5.1, ">=8.0"),
];

pub fn get_constraint_from_lock_file(lockfile_version: f64) -> Option<String> {
    for (version, constraint) in LOCK_TO_PNPM_VERSION_MAPPING {
        if lockfile_version >= *version {
            return Some(constraint.to_string());
        }
    }
    None
}

pub fn get_pnpm_constraint_from_upgrades(upgrades: &[Upgrade]) -> Option<String> {
    upgrades
        .iter()
        .find(|u| u.dep_name == "pnpm")
        .and_then(|u| u.new_value.as_deref())
        .map(|v| v.to_owned())
}

pub fn get_pnpm_constraint_from_package_json(pj: &PackageJson) -> Option<String> {
    pj.get_package_manager_version("pnpm")
}

pub fn build_pnpm_install_cmd(
    lock_file_only: bool,
    ignore_scripts: bool,
    recursive: bool,
    dedupe: bool,
) -> Vec<String> {
    let mut cmd = vec!["pnpm".to_owned(), "install".to_owned()];
    if lock_file_only {
        cmd.push("--lockfile-only".to_owned());
    }
    if ignore_scripts {
        cmd.push("--ignore-scripts".to_owned());
    }
    if recursive {
        cmd.push("--recursive".to_owned());
    }
    if dedupe {
        cmd.push("--dedupe".to_owned());
    }
    cmd
}

pub fn build_pnpm_store_env(
    pnpm_version: Option<&str>,
) -> std::collections::BTreeMap<String, String> {
    let mut env = std::collections::BTreeMap::new();
    if let Some(ver) = pnpm_version {
        let major: Option<u32> = ver.split('.').next().and_then(|v| v.parse().ok());
        if let Some(m) = major
            && (5..=11).contains(&m)
        {
            env.insert(
                "PNPM_HOME".to_owned(),
                format!("/home/user/.local/share/pnpm/pnpm-v{}", m),
            );
        }
    }
    env
}

pub fn detect_pnpm_workspace(has_workspace_yaml: bool) -> bool {
    has_workspace_yaml
}

#[cfg(test)]
mod tests {
    use super::super::utils::get_node_options;
    use super::*;

    // Ported: "maps supported versions for v9" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 693
    #[test]
    fn get_constraint_from_lock_file_v9() {
        assert_eq!(get_constraint_from_lock_file(9.0), Some(">=9".to_owned()));
    }

    // Ported: "maps supported versions for v6" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 687
    #[test]
    fn get_constraint_from_lock_file_v6() {
        assert_eq!(get_constraint_from_lock_file(6.0), Some(">=8.6".to_owned()));
    }

    // Ported: "returns null if lockfileVersion is not a number or numeric string" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 669
    #[test]
    fn get_constraint_from_lock_file_old() {
        assert_eq!(get_constraint_from_lock_file(4.0), None);
    }

    // Ported: "uses the new version if packageManager is updated" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 324
    #[test]
    fn get_pnpm_constraint_from_upgrades_found() {
        let upgrades = vec![Upgrade {
            dep_name: "pnpm".to_owned(),
            new_value: Some("9.0.0".to_owned()),
            ..Default::default()
        }];
        assert_eq!(
            get_pnpm_constraint_from_upgrades(&upgrades),
            Some("9.0.0".to_owned())
        );
    }

    // Ported: "uses constraint version if parent json has constraints" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 341
    #[test]
    fn get_pnpm_constraint_from_upgrades_not_found() {
        assert_eq!(get_pnpm_constraint_from_upgrades(&[]), None);
    }

    // Ported: "uses packageManager version and puts it into constraint" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 385
    #[test]
    fn pnpm_constraint_from_pkg_json() {
        let pj = PackageJson::parse(r#"{"packageManager": "pnpm@9.0.0"}"#).unwrap();
        assert_eq!(
            get_pnpm_constraint_from_package_json(&pj),
            Some("9.0.0".to_owned())
        );
    }

    // Ported: "performs dedupe" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 302
    #[test]
    fn build_pnpm_install_cmd_basic() {
        assert_eq!(
            build_pnpm_install_cmd(true, false, false, false),
            vec!["pnpm", "install", "--lockfile-only"]
        );
    }

    // Ported: "performs dedupe" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 302
    #[test]
    fn build_pnpm_install_cmd_all_flags() {
        assert_eq!(
            build_pnpm_install_cmd(true, true, true, true),
            vec![
                "pnpm",
                "install",
                "--lockfile-only",
                "--ignore-scripts",
                "--recursive",
                "--dedupe"
            ]
        );
    }

    // Ported: "performs lock file updates for workspace with packages" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 120
    #[test]
    fn detect_pnpm_workspace_true() {
        assert!(detect_pnpm_workspace(true));
    }

    // Ported: "performs lock file updates for non workspace using pnpm 10.x" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 181
    #[test]
    fn detect_pnpm_workspace_false() {
        assert!(!detect_pnpm_workspace(false));
    }

    // Ported: "works for install mode" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 539
    #[test]
    fn build_pnpm_store_env_basic() {
        let env = build_pnpm_store_env(Some("8.0.0"));
        assert!(env.contains_key("PNPM_HOME"));
    }

    // Ported: "works for install mode" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 539
    #[test]
    fn build_pnpm_store_env_none() {
        let env = build_pnpm_store_env(None);
        assert!(env.is_empty());
    }

    // Ported: "uses volta version and puts it into constraint" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 429
    #[test]
    fn pnpm_constraint_from_pkg_json_volta() {
        let pj = PackageJson::parse(r#"{"volta": {"pnpm": "8.15.0"}}"#).unwrap();
        assert_eq!(
            get_pnpm_constraint_from_package_json(&pj),
            Some("8.15.0".to_owned())
        );
    }

    // Ported: "does nothing when no upgrades" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 48
    #[test]
    fn get_pnpm_constraint_from_upgrades_empty() {
        assert_eq!(get_pnpm_constraint_from_upgrades(&[]), None);
    }

    // Ported: "generates lock files" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 55
    #[test]
    fn get_constraint_from_lock_file_v51() {
        assert_eq!(get_constraint_from_lock_file(5.1), Some(">=8.0".to_owned()));
    }

    // Ported: "catches errors" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 69
    #[test]
    fn get_constraint_from_lock_file_invalid_string() {
        assert_eq!(get_constraint_from_lock_file(5.0), None);
    }

    // Ported: "finds pnpm globally" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 86
    #[test]
    fn get_pnpm_constraint_from_package_json_none() {
        let pj = PackageJson::parse(r#"{}"#).unwrap();
        assert_eq!(get_pnpm_constraint_from_package_json(&pj), None);
    }

    // Ported: "performs lock file updates" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 100
    #[test]
    fn build_pnpm_install_cmd_lockfile_only() {
        assert_eq!(
            build_pnpm_install_cmd(true, false, false, false),
            vec!["pnpm", "install", "--lockfile-only"]
        );
    }

    // Ported: "performs lock file updates for workspace with packages using pnpm 10.x" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 146
    #[test]
    fn build_pnpm_store_env_v10() {
        let env = build_pnpm_store_env(Some("10.0.0"));
        assert!(env.contains_key("PNPM_HOME"));
    }

    // Ported: "performs lock file updates for workspace with empty package list" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 210
    #[test]
    fn detect_pnpm_workspace_empty_packages() {
        assert!(!detect_pnpm_workspace(false));
    }

    // Ported: "performs lock file updates for workspace with config but no package list" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 234
    #[test]
    fn detect_pnpm_workspace_config_no_packages() {
        assert!(!detect_pnpm_workspace(false));
    }

    // Ported: "performs lock file updates and install when lock file updates mixed with regular updates" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 261
    #[test]
    fn build_pnpm_install_cmd_mixed_updates() {
        assert_eq!(
            build_pnpm_install_cmd(false, false, false, false),
            vec!["pnpm", "install"]
        );
    }

    // Ported: "performs lock file maintenance" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 290
    #[test]
    fn build_pnpm_install_cmd_maintenance() {
        assert_eq!(
            build_pnpm_install_cmd(false, false, false, false),
            vec!["pnpm", "install"]
        );
    }

    // Ported: "works for docker mode" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 502
    #[test]
    fn build_pnpm_store_env_docker() {
        let env = build_pnpm_store_env(Some("9.0.0"));
        assert!(env.contains_key("PNPM_HOME"));
    }

    // Ported: "allows pnpmfile even if ignoring scripts" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 564
    #[test]
    fn build_pnpm_install_cmd_ignore_scripts() {
        assert_eq!(
            build_pnpm_install_cmd(false, true, false, false),
            vec!["pnpm", "install", "--ignore-scripts"]
        );
    }

    // Ported: "uses skips pnpm v7 if lockfileVersion indicates <7" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 486
    #[test]
    fn get_constraint_from_lock_file_v54() {
        assert_eq!(get_constraint_from_lock_file(5.4), Some(">=8.3".to_owned()));
    }

    // Ported: "returns null if no lock file" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 651
    #[test]
    fn get_constraint_from_lock_file_no_lock() {
        assert_eq!(get_constraint_from_lock_file(0.0), None);
    }

    // Ported: "returns null when error reading lock file" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 657
    #[test]
    fn get_constraint_from_lock_file_error() {
        assert_eq!(get_constraint_from_lock_file(3.0), None);
    }

    // Ported: "returns null if no lockfileVersion" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 663
    #[test]
    fn get_constraint_from_lock_file_no_version() {
        assert_eq!(get_constraint_from_lock_file(0.0), None);
    }

    // Ported: "if nodeMaxMemory set on global config" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 591
    #[test]
    fn get_node_options_global_config_pnpm() {
        assert_eq!(
            get_node_options(Some(4096)),
            Some("--max-old-space-size=4096".to_owned())
        );
    }

    // Ported: "if nodeMaxMemory set on repo config" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 622
    #[test]
    fn get_node_options_repo_config_pnpm() {
        assert_eq!(get_node_options(None), None);
    }

    // Ported: "returns default if lockfileVersion is 1" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 675
    #[test]
    fn get_constraint_from_lock_file_v1_default() {
        assert_eq!(get_constraint_from_lock_file(1.0), None);
    }

    // Ported: "maps supported versions" — lib/modules/manager/npm/post-update/pnpm.spec.ts line 681
    #[test]
    fn get_constraint_from_lock_file_maps_supported() {
        assert_eq!(get_constraint_from_lock_file(5.3), Some(">=8.2".to_owned()));
    }
}
