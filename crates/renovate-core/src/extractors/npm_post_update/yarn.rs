//! Yarn lock file generation.
//!
//! Ports `lib/modules/manager/npm/post-update/yarn.ts`.

use super::{PackageJson, Upgrade};

#[derive(Debug, Clone, Default)]
pub struct YarnLockFileConfig {
    pub lock_file_dir: String,
    pub npmrc: Option<String>,
    pub constraints: std::collections::BTreeMap<String, String>,
    pub post_update_options: Vec<String>,
    pub env: std::collections::BTreeMap<String, String>,
}

#[derive(Debug, Clone, Default)]
pub struct YarnLockFileResult {
    pub lock_file_name: String,
    pub content: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YarnMajorVersion {
    V1,
    V2Plus,
}

pub fn detect_yarn_version(yarn_constraint: Option<&str>) -> YarnMajorVersion {
    if let Some(constraint) = yarn_constraint {
        if let Some(major_str) = constraint.split('.').next()
            && let Ok(major) = major_str.parse::<u32>()
                && major >= 2 {
                    return YarnMajorVersion::V2Plus;
                }
        if constraint.starts_with(">=2")
            || constraint.starts_with("^2")
            || constraint.starts_with("^3")
            || constraint.starts_with("^4")
        {
            return YarnMajorVersion::V2Plus;
        }
    }
    YarnMajorVersion::V1
}

pub fn check_yarnrc(yarnrc_content: &str) -> YarnRcInfo {
    let mut info = YarnRcInfo::default();
    for line in yarnrc_content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = trimmed.split_once(' ') {
            match key {
                "--install.offline-mirror" | "install.offline-mirror" => {
                    info.offline_mirror = Some(value.trim().to_owned());
                }
                "--yarn-path" | "yarn-path" => {
                    info.yarn_path = Some(value.trim().to_owned());
                }
                _ => {}
            }
        }
    }
    info
}

#[derive(Debug, Clone, Default)]
pub struct YarnRcInfo {
    pub offline_mirror: Option<String>,
    pub yarn_path: Option<String>,
}

pub fn is_yarn_update(upgrade: &Upgrade) -> bool {
    upgrade.dep_name == "yarn"
}

pub fn get_yarn_constraint_from_package_json(pj: &PackageJson) -> Option<String> {
    pj.get_package_manager_version("yarn")
}

pub fn get_yarn_constraint_from_upgrades(upgrades: &[Upgrade]) -> Option<String> {
    upgrades
        .iter()
        .find(|u| u.dep_name == "yarn")
        .and_then(|u| u.new_value.as_deref())
        .map(|v| v.to_owned())
}

pub fn build_yarn_install_cmd(
    version: YarnMajorVersion,
    lock_file_only: bool,
    ignore_scripts: bool,
    mode_flag: bool,
) -> Vec<String> {
    let mut cmd = vec!["yarn".to_owned()];
    match version {
        YarnMajorVersion::V1 => {
            cmd.push("install".to_owned());
            if lock_file_only {
                cmd.push("--frozen-lockfile".to_owned());
            }
            if ignore_scripts {
                cmd.push("--ignore-scripts".to_owned());
            }
        }
        YarnMajorVersion::V2Plus => {
            cmd.push("install".to_owned());
            if mode_flag && lock_file_only {
                cmd.push("--mode".to_owned());
                cmd.push("update-lockfile".to_owned());
            }
            if ignore_scripts {
                cmd.push("--immutable".to_owned());
            }
        }
    }
    cmd
}

pub fn build_yarn_upgrade_cmd(version: YarnMajorVersion, dep_name: &str) -> Vec<String> {
    match version {
        YarnMajorVersion::V1 => {
            vec!["yarn".to_owned(), "upgrade".to_owned(), dep_name.to_owned()]
        }
        YarnMajorVersion::V2Plus => {
            vec![
                "yarn".to_owned(),
                "up".to_owned(),
                "-R".to_owned(),
                dep_name.to_owned(),
            ]
        }
    }
}

pub fn fuzzy_match_additional_yarnrc_yml(additional: &str, existing: &str) -> String {
    let add_registries: std::collections::BTreeMap<String, serde_json::Value> =
        serde_yaml::from_str(additional).unwrap_or_default();

    let existing_registries: std::collections::BTreeMap<String, serde_json::Value> =
        serde_yaml::from_str(existing).unwrap_or_default();

    let keys_to_replace: Vec<(String, String)> = add_registries
        .keys()
        .filter_map(|key| {
            let normalized = key.trim_end_matches('/');
            for existing_key in existing_registries.keys() {
                let existing_normalized = existing_key.trim_end_matches('/');
                if normalized == existing_normalized
                    || normalize_registry_url(key) == normalize_registry_url(existing_key)
                {
                    return Some((key.clone(), existing_key.clone()));
                }
            }
            None
        })
        .collect();

    let mut result = additional.to_owned();
    for (old_key, new_key) in keys_to_replace {
        result = result.replace(&old_key, &new_key);
    }
    result
}

fn normalize_registry_url(url: &str) -> String {
    let normalized = url.trim_end_matches('/').to_owned();
    if !normalized.starts_with("http://") && !normalized.starts_with("https://")
        && let Some(rest) = normalized.split_once("://").map(|(_, r)| r) {
            return format!("https://{}", rest);
        }
    normalized
}

pub fn get_optimize_command() -> Vec<String> {
    vec![
        "sed".to_owned(),
        "-i".to_owned(),
        "s/stepTimer(&timer)/stepTimer(&timer); if (timer.skip === true) return/g".to_owned(),
        "yarn.js".to_owned(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    // Rust-specific: yarn behavior test
    #[test]
    fn detect_yarn_v1() {
        assert_eq!(
            detect_yarn_version(Some("1.22.19")),
            YarnMajorVersion::V1
        );
    }

    // Rust-specific: yarn behavior test
    #[test]
    fn detect_yarn_v2() {
        assert_eq!(
            detect_yarn_version(Some("2.4.3")),
            YarnMajorVersion::V2Plus
        );
    }

    // Rust-specific: yarn behavior test
    #[test]
    fn detect_yarn_v4() {
        assert_eq!(
            detect_yarn_version(Some("4.1.0")),
            YarnMajorVersion::V2Plus
        );
    }

    // Rust-specific: yarn behavior test
    #[test]
    fn detect_yarn_none() {
        assert_eq!(detect_yarn_version(None), YarnMajorVersion::V1);
    }

    // Rust-specific: yarn behavior test
    #[test]
    fn detect_yarn_caret4() {
        assert_eq!(
            detect_yarn_version(Some("^4.0.0")),
            YarnMajorVersion::V2Plus
        );
    }

    // Rust-specific: yarn behavior test
    #[test]
    fn check_yarnrc_offline_mirror() {
        let info = check_yarnrc("--install.offline-mirror true\n");
        assert_eq!(info.offline_mirror.as_deref(), Some("true"));
    }

    // Rust-specific: yarn behavior test
    #[test]
    fn check_yarnrc_yarn_path() {
        let info = check_yarnrc("--yarn-path .yarn/releases/yarn-4.1.0.cjs\n");
        assert_eq!(
            info.yarn_path.as_deref(),
            Some(".yarn/releases/yarn-4.1.0.cjs")
        );
    }

    // Rust-specific: yarn behavior test
    #[test]
    fn check_yarnrc_empty() {
        let info = check_yarnrc("");
        assert!(info.offline_mirror.is_none());
        assert!(info.yarn_path.is_none());
    }

    // Rust-specific: yarn behavior test
    #[test]
    fn yarn_update_true() {
        let u = Upgrade {
            dep_name: "yarn".to_owned(),
            ..Default::default()
        };
        assert!(is_yarn_update(&u));
    }

    // Rust-specific: yarn behavior test
    #[test]
    fn yarn_update_false() {
        let u = Upgrade {
            dep_name: "npm".to_owned(),
            ..Default::default()
        };
        assert!(!is_yarn_update(&u));
    }

    // Rust-specific: yarn behavior test
    #[test]
    fn yarn_constraint_from_upgrades_found() {
        let upgrades = vec![Upgrade {
            dep_name: "yarn".to_owned(),
            new_value: Some("4.1.0".to_owned()),
            ..Default::default()
        }];
        assert_eq!(
            get_yarn_constraint_from_upgrades(&upgrades),
            Some("4.1.0".to_owned())
        );
    }

    // Rust-specific: yarn behavior test
    #[test]
    fn yarn_constraint_from_upgrades_not_found() {
        assert_eq!(get_yarn_constraint_from_upgrades(&[]), None);
    }

    // Rust-specific: yarn behavior test
    #[test]
    fn yarn_constraint_from_pkg_json() {
        let pj = PackageJson::parse(r#"{"packageManager": "yarn@4.1.0"}"#).unwrap();
        assert_eq!(
            get_yarn_constraint_from_package_json(&pj),
            Some("4.1.0".to_owned())
        );
    }

    // Rust-specific: yarn behavior test
    #[test]
    fn yarn_install_v1_frozen() {
        assert_eq!(
            build_yarn_install_cmd(YarnMajorVersion::V1, true, false, false),
            vec!["yarn", "install", "--frozen-lockfile"]
        );
    }

    // Rust-specific: yarn behavior test
    #[test]
    fn yarn_install_v2_mode() {
        assert_eq!(
            build_yarn_install_cmd(YarnMajorVersion::V2Plus, true, false, true),
            vec!["yarn", "install", "--mode", "update-lockfile"]
        );
    }

    // Rust-specific: yarn behavior test
    #[test]
    fn yarn_upgrade_v1_cmd() {
        assert_eq!(
            build_yarn_upgrade_cmd(YarnMajorVersion::V1, "lodash"),
            vec!["yarn", "upgrade", "lodash"]
        );
    }

    // Rust-specific: yarn behavior test
    #[test]
    fn yarn_upgrade_v2_cmd() {
        assert_eq!(
            build_yarn_upgrade_cmd(YarnMajorVersion::V2Plus, "lodash"),
            vec!["yarn", "up", "-R", "lodash"]
        );
    }

    // Rust-specific: yarn behavior test
    #[test]
    fn optimize_command() {
        let cmd = get_optimize_command();
        assert_eq!(cmd[0], "sed");
    }
}
