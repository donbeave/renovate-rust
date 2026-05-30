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

pub fn build_pnpm_store_env(pnpm_version: Option<&str>) -> std::collections::BTreeMap<String, String> {
    let mut env = std::collections::BTreeMap::new();
    if let Some(ver) = pnpm_version {
        let major: Option<u32> = ver.split('.').next().and_then(|v| v.parse().ok());
        if let Some(m) = major
            && (5..=11).contains(&m) {
                env.insert("PNPM_HOME".to_owned(), format!("/home/user/.local/share/pnpm/pnpm-v{}", m));
            }
    }
    env
}

pub fn detect_pnpm_workspace(has_workspace_yaml: bool) -> bool {
    has_workspace_yaml
}

#[cfg(test)]
mod tests {
    use super::*;

    // Rust-specific: pnpm behavior test
    #[test]
    fn get_constraint_from_lock_file_v9() {
        assert_eq!(
            get_constraint_from_lock_file(9.0),
            Some(">=9".to_owned())
        );
    }

    // Rust-specific: pnpm behavior test
    #[test]
    fn get_constraint_from_lock_file_v6() {
        assert_eq!(
            get_constraint_from_lock_file(6.0),
            Some(">=8.6".to_owned())
        );
    }

    // Rust-specific: pnpm behavior test
    #[test]
    fn get_constraint_from_lock_file_old() {
        assert_eq!(get_constraint_from_lock_file(4.0), None);
    }

    // Rust-specific: pnpm behavior test
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

    // Rust-specific: pnpm behavior test
    #[test]
    fn get_pnpm_constraint_from_upgrades_not_found() {
        assert_eq!(
            get_pnpm_constraint_from_upgrades(&[]),
            None
        );
    }

    // Rust-specific: pnpm behavior test
    #[test]
    fn pnpm_constraint_from_pkg_json() {
        let pj = PackageJson::parse(
            r#"{"packageManager": "pnpm@9.0.0"}"#,
        )
        .unwrap();
        assert_eq!(
            get_pnpm_constraint_from_package_json(&pj),
            Some("9.0.0".to_owned())
        );
    }

    // Rust-specific: pnpm behavior test
    #[test]
    fn build_pnpm_install_cmd_basic() {
        assert_eq!(
            build_pnpm_install_cmd(true, false, false, false),
            vec!["pnpm", "install", "--lockfile-only"]
        );
    }

    // Rust-specific: pnpm behavior test
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

    // Rust-specific: pnpm behavior test
    #[test]
    fn detect_pnpm_workspace_true() {
        assert!(detect_pnpm_workspace(true));
    }

    // Rust-specific: pnpm behavior test
    #[test]
    fn detect_pnpm_workspace_false() {
        assert!(!detect_pnpm_workspace(false));
    }
}
