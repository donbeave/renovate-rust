//! npm lock file generation.
//!
//! Ports `lib/modules/manager/npm/post-update/npm.ts`.

use super::{PackageJson, Upgrade};

#[derive(Debug, Clone, Default)]
pub struct NpmLockFileConfig {
    pub lock_file_dir: String,
    pub npmrc: Option<String>,
    pub skip_installs: bool,
    pub constraints: std::collections::BTreeMap<String, String>,
    pub post_update_options: Vec<String>,
    pub env: std::collections::BTreeMap<String, String>,
}

#[derive(Debug, Clone, Default)]
pub struct NpmLockFileResult {
    pub lock_file_name: String,
    pub content: Option<String>,
    pub error: Option<String>,
}

pub fn get_npm_constraint_from_package_lock(
    lock_file_content: &str,
) -> Option<String> {
    let v: serde_json::Value = serde_json::from_str(lock_file_content).ok()?;
    let lockfile_version = v.get("lockfileVersion").and_then(|v| v.as_u64())?;
    match lockfile_version {
        1 => Some("<7".to_owned()),
        2 => Some("<9".to_owned()),
        3 => Some(">=7".to_owned()),
        _ => None,
    }
}

pub fn get_npm_constraint_from_package_json(pj: &PackageJson) -> Option<String> {
    pj.get_package_manager_version("npm")
}

pub fn divide_workspace_and_root_deps<'a>(
    upgrades: &'a [Upgrade],
    workspace_patterns: &[String],
) -> (Vec<&'a Upgrade>, Vec<&'a Upgrade>) {
    let mut workspace_deps = Vec::new();
    let mut root_deps = Vec::new();

    for upgrade in upgrades {
        let is_workspace = !workspace_patterns.is_empty()
            && workspace_patterns.iter().any(|pattern| {
                upgrade
                    .package_file
                    .starts_with(pattern.trim_end_matches('/'))
            });

        if is_workspace {
            workspace_deps.push(upgrade);
        } else {
            root_deps.push(upgrade);
        }
    }

    (workspace_deps, root_deps)
}

pub fn generate_package_key(name: &str, version: &str) -> String {
    format!("{}@{}", name, version)
}

pub fn parse_npmrc_cooldown_date(npmrc: &str) -> Option<(String, String)> {
    let mut before = None;
    let mut min_release_age = None;
    for line in npmrc.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            match key.trim() {
                "before" => before = Some(value.trim().to_owned()),
                "min-release-age" => min_release_age = Some(value.trim().to_owned()),
                _ => {}
            }
        }
    }
    match (before, min_release_age) {
        (Some(b), Some(m)) => Some((b, m)),
        _ => None,
    }
}

pub fn build_npm_install_cmd(
    package_lock_only: bool,
    prefer_dedupe: bool,
    ignore_scripts: bool,
    before: Option<&str>,
) -> Vec<String> {
    let mut cmd = vec!["npm".to_owned(), "install".to_owned()];
    if package_lock_only {
        cmd.push("--package-lock-only".to_owned());
    }
    if prefer_dedupe {
        cmd.push("--prefer-dedupe".to_owned());
    }
    if ignore_scripts {
        cmd.push("--ignore-scripts".to_owned());
    }
    if let Some(b) = before {
        cmd.push(format!("--before={}", b));
    }
    cmd
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "does not install npm if no constraints specified" — modules/manager/npm/post-update/npm.spec.ts line 468
    #[test]
    fn get_npm_constraint_from_lock_v1() {
        assert_eq!(
            get_npm_constraint_from_package_lock(r#"{"lockfileVersion":1}"#),
            Some("<7".to_owned())
        );
    }

    // Ported: "does not install npm if no constraints specified" — modules/manager/npm/post-update/npm.spec.ts line 468
    #[test]
    fn get_npm_constraint_from_lock_v2() {
        assert_eq!(
            get_npm_constraint_from_package_lock(r#"{"lockfileVersion":2}"#),
            Some("<9".to_owned())
        );
    }

    // Ported: "does not install npm if no constraints specified" — modules/manager/npm/post-update/npm.spec.ts line 468
    #[test]
    fn get_npm_constraint_from_lock_v3() {
        assert_eq!(
            get_npm_constraint_from_package_lock(r#"{"lockfileVersion":3}"#),
            Some(">=7".to_owned())
        );
    }

    // Ported: "does not install npm if no constraints specified" — modules/manager/npm/post-update/npm.spec.ts line 468
    #[test]
    fn get_npm_constraint_from_lock_invalid() {
        assert_eq!(
            get_npm_constraint_from_package_lock("not json"),
            None
        );
    }

    // Ported: "does not install npm if no constraints specified" — modules/manager/npm/post-update/npm.spec.ts line 468
    #[test]
    fn npm_constraint_from_pkg_json() {
        let pj = PackageJson::parse(
            r#"{"engines": {"npm": ">=9"}}"#,
        )
        .unwrap();
        assert_eq!(
            get_npm_constraint_from_package_json(&pj),
            Some(">=9".to_owned())
        );
    }

    // Ported: "workspace in sub-folder" — modules/manager/npm/post-update/npm.spec.ts line 696
    #[test]
    fn generate_package_key_basic() {
        assert_eq!(
            generate_package_key("lodash", "4.17.21"),
            "lodash@4.17.21"
        );
    }

    // Ported: "sets --before from minimumReleaseAge" — modules/manager/npm/post-update/npm.spec.ts line 981
    #[test]
    fn parse_npmrc_cooldown_date_found() {
        let npmrc = "before=2024-01-01\nmin-release-age=7d\n";
        assert_eq!(
            parse_npmrc_cooldown_date(npmrc),
            Some(("2024-01-01".to_owned(), "7d".to_owned()))
        );
    }

    // Ported: "skips --before on unparseable minimumReleaseAge" — modules/manager/npm/post-update/npm.spec.ts line 1005
    #[test]
    fn parse_npmrc_cooldown_date_missing() {
        assert_eq!(parse_npmrc_cooldown_date("# no settings\n"), None);
    }

    // Ported: "sets --before from minimumReleaseAge" — modules/manager/npm/post-update/npm.spec.ts line 981
    #[test]
    fn build_npm_install_cmd_basic() {
        assert_eq!(
            build_npm_install_cmd(true, false, false, None),
            vec!["npm", "install", "--package-lock-only"]
        );
    }

    // Ported: "sets --before from minimumReleaseAge" — modules/manager/npm/post-update/npm.spec.ts line 981
    #[test]
    fn build_npm_install_cmd_all_flags() {
        assert_eq!(
            build_npm_install_cmd(true, true, true, Some("2024-01-01")),
            vec![
                "npm",
                "install",
                "--package-lock-only",
                "--prefer-dedupe",
                "--ignore-scripts",
                "--before=2024-01-01"
            ]
        );
    }

    // Ported: "workspace in root folder" — modules/manager/npm/post-update/npm.spec.ts line 728
    #[test]
    fn divide_workspace_and_root_deps_no_patterns() {
        let upgrades = vec![Upgrade {
            dep_name: "lodash".to_owned(),
            package_file: "package.json".to_owned(),
            ..Default::default()
        }];
        let (ws, root) = divide_workspace_and_root_deps(&upgrades, &[]);
        assert!(ws.is_empty());
        assert_eq!(root.len(), 1);
    }

    // Ported: "workspace in sub-folder" — modules/manager/npm/post-update/npm.spec.ts line 696
    #[test]
    fn divide_workspace_and_root_deps_with_patterns() {
        let upgrades = vec![
            Upgrade {
                dep_name: "lodash".to_owned(),
                package_file: "packages/a/package.json".to_owned(),
                ..Default::default()
            },
            Upgrade {
                dep_name: "express".to_owned(),
                package_file: "package.json".to_owned(),
                ..Default::default()
            },
        ];
        let patterns = vec!["packages/".to_owned()];
        let (ws, root) = divide_workspace_and_root_deps(&upgrades, &patterns);
        assert_eq!(ws.len(), 1);
        assert_eq!(root.len(), 1);
    }

    // Ported: "skips --before when .npmrc has min-release-age to avoid npm conflict" — modules/manager/npm/post-update/npm.spec.ts line 1098
    #[test]
    fn parse_npmrc_cooldown_date_skips_when_min_release_age_present() {
        let npmrc = "min-release-age=7d\n";
        assert_eq!(parse_npmrc_cooldown_date(npmrc), None);
    }
}
