//! Version bumping after dependency update.
//!
//! Mirrors `lib/workers/repository/update/branch/bump-versions.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BumpType {
    Major,
    Minor,
    Patch,
}

#[derive(Debug, Clone, Default)]
pub struct BumpVersionConfig {
    pub bump_type: Option<BumpType>,
    pub current_version: Option<String>,
    pub package_file: Option<String>,
}

pub fn determine_bump_type(
    current_major: u64,
    new_major: u64,
    current_minor: u64,
    new_minor: u64,
    is_pin: bool,
) -> BumpType {
    if is_pin {
        return BumpType::Patch;
    }
    if new_major > current_major {
        return BumpType::Major;
    }
    if new_minor > current_minor {
        return BumpType::Minor;
    }
    BumpType::Patch
}

pub fn bump_version(version: &str, bump_type: BumpType) -> Option<String> {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.is_empty() {
        return None;
    }

    let major: u64 = parts.first()?.parse().ok()?;
    let minor: u64 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch: u64 = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);

    let result = match bump_type {
        BumpType::Major => format!("{}.0.0", major + 1),
        BumpType::Minor => format!("{major}.{}.0", minor + 1),
        BumpType::Patch => format!("{major}.{minor}.{}", patch + 1),
    };

    Some(result)
}

pub fn bump_versions(
    updated_package_files: &[crate::workers::types::FileChange],
    updated_artifacts: &[crate::workers::types::FileChange],
    config: &BumpVersionConfig,
) -> Vec<crate::workers::types::FileChange> {
    if updated_package_files.is_empty() && updated_artifacts.is_empty() {
        return vec![];
    }

    let Some(bump_type) = config.bump_type else {
        return vec![];
    };

    let mut bumped = vec![];
    for file in updated_package_files {
        let Some(ref contents) = file.contents else {
            continue;
        };
        if let Some(ref current) = config.current_version
            && contents.contains(current)
            && let Some(new_version) = bump_version(current, bump_type)
        {
            bumped.push(crate::workers::types::FileChange {
                path: file.path.clone(),
                contents: Some(contents.replace(current, &new_version)),
            });
        }
    }
    bumped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bump_type_variants() {
        assert_ne!(BumpType::Major, BumpType::Minor);
        assert_ne!(BumpType::Minor, BumpType::Patch);
    }

    #[test]
    fn bump_version_config_default() {
        let c = BumpVersionConfig::default();
        assert!(c.bump_type.is_none());
        assert!(c.current_version.is_none());
    }

    #[test]
    fn determine_bump_type_major() {
        assert_eq!(determine_bump_type(1, 2, 0, 0, false), BumpType::Major);
    }

    #[test]
    fn determine_bump_type_minor() {
        assert_eq!(determine_bump_type(1, 1, 0, 1, false), BumpType::Minor);
    }

    #[test]
    fn determine_bump_type_patch() {
        assert_eq!(determine_bump_type(1, 1, 0, 0, false), BumpType::Patch);
    }

    #[test]
    fn determine_bump_type_pin() {
        assert_eq!(determine_bump_type(0, 0, 0, 0, true), BumpType::Patch);
    }

    #[test]
    fn bump_version_major() {
        assert_eq!(bump_version("1.2.3", BumpType::Major), Some("2.0.0".to_owned()));
    }

    #[test]
    fn bump_version_minor() {
        assert_eq!(bump_version("1.2.3", BumpType::Minor), Some("1.3.0".to_owned()));
    }

    #[test]
    fn bump_version_patch() {
        assert_eq!(bump_version("1.2.3", BumpType::Patch), Some("1.2.4".to_owned()));
    }

    #[test]
    fn bump_version_two_parts() {
        assert_eq!(bump_version("1.2", BumpType::Patch), Some("1.2.1".to_owned()));
    }

    #[test]
    fn bump_version_single_part() {
        assert_eq!(bump_version("5", BumpType::Major), Some("6.0.0".to_owned()));
    }

    #[test]
    fn bump_version_empty_returns_none() {
        assert_eq!(bump_version("", BumpType::Patch), None);
    }

    #[test]
    fn bump_versions_empty_files() {
        let config = BumpVersionConfig {
            bump_type: Some(BumpType::Patch),
            current_version: Some("1.0.0".into()),
            ..Default::default()
        };
        let result = bump_versions(&[], &[], &config);
        assert!(result.is_empty());
    }

    #[test]
    fn bump_versions_no_bump_type() {
        let files = vec![crate::workers::types::FileChange {
            path: "package.json".into(),
            contents: Some("1.0.0".into()),
        }];
        let config = BumpVersionConfig::default();
        let result = bump_versions(&files, &[], &config);
        assert!(result.is_empty());
    }
}
