//! Lookup worker types.
//!
//! Mirrors `lib/workers/repository/process/lookup/types.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UpdateType {
    Pin,
    Replace,
    Bump,
    Major,
    Minor,
    Patch,
    Digest,
    PinDigest,
    LockFileMaintenance,
    LockfileUpdate,
    Rollback,
    Replacement,
}

impl UpdateType {
    pub fn as_str(&self) -> &'static str {
        match self {
            UpdateType::Pin => "pin",
            UpdateType::Replace => "replace",
            UpdateType::Bump => "bump",
            UpdateType::Major => "major",
            UpdateType::Minor => "minor",
            UpdateType::Patch => "patch",
            UpdateType::Digest => "digest",
            UpdateType::PinDigest => "pinDigest",
            UpdateType::LockFileMaintenance => "lockFileMaintenance",
            UpdateType::LockfileUpdate => "lockfileUpdate",
            UpdateType::Rollback => "rollback",
            UpdateType::Replacement => "replacement",
        }
    }
}

impl std::fmt::Display for UpdateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for UpdateType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pin" => Ok(UpdateType::Pin),
            "replace" => Ok(UpdateType::Replace),
            "bump" => Ok(UpdateType::Bump),
            "major" => Ok(UpdateType::Major),
            "minor" => Ok(UpdateType::Minor),
            "patch" => Ok(UpdateType::Patch),
            "digest" => Ok(UpdateType::Digest),
            "pinDigest" => Ok(UpdateType::PinDigest),
            "lockFileMaintenance" => Ok(UpdateType::LockFileMaintenance),
            "lockfileUpdate" => Ok(UpdateType::LockfileUpdate),
            "rollback" => Ok(UpdateType::Rollback),
            "replacement" => Ok(UpdateType::Replacement),
            _ => Err(format!("unknown update type: {s}")),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LookupUpdate {
    pub bucket: Option<String>,
    pub new_value: Option<String>,
    pub new_version: Option<String>,
    pub new_digest: Option<String>,
    pub new_major: Option<u64>,
    pub new_minor: Option<u64>,
    pub new_patch: Option<u64>,
    pub new_name: Option<String>,
    pub new_name_sanitized: Option<String>,
    pub update_type: Option<UpdateType>,
    pub branch_name: Option<String>,
    pub commit_message_action: Option<String>,
    pub is_bump: Option<bool>,
    pub is_lockfile_update: Option<bool>,
    pub is_pin: Option<bool>,
    pub is_pin_digest: Option<bool>,
    pub is_range: Option<bool>,
    pub is_rollback: Option<bool>,
    pub is_replacement: Option<bool>,
    pub is_single_version: Option<bool>,
    pub is_breaking: Option<bool>,
    pub pending_checks: Option<bool>,
    pub pending_versions: Option<Vec<String>>,
    pub release_timestamp: Option<String>,
    pub registry_url: Option<String>,
    pub semantic_commit_type: Option<String>,
    pub user_strings: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RollbackConfig {
    pub current_value: Option<String>,
    pub package_name: String,
    pub dep_name: Option<String>,
    pub package_file: Option<String>,
    pub versioning: Option<String>,
    pub datasource: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FilterConfig {
    pub allowed_versions: Option<String>,
    pub dep_name: Option<String>,
    pub follow_tag: Option<String>,
    pub ignore_deprecated: Option<bool>,
    pub ignore_unstable: Option<bool>,
    pub max_major_increment: Option<u64>,
    pub respect_latest: Option<bool>,
    pub update_pinned_dependencies: Option<bool>,
    pub versioning: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LookupUpdateConfig {
    #[serde(flatten)]
    pub rollback: RollbackConfig,
    #[serde(flatten)]
    pub filter: FilterConfig,
    pub current_version: Option<String>,
    pub current_digest: Option<String>,
    pub locked_version: Option<String>,
    pub digest_one_and_only: Option<bool>,
    pub rollback_prs: Option<bool>,
    pub is_vulnerability_alert: Option<bool>,
    pub minimum_confidence: Option<String>,
    pub replacement_name: Option<String>,
    pub replacement_name_template: Option<String>,
    pub replacement_version: Option<String>,
    pub replacement_version_template: Option<String>,
    pub extract_version: Option<String>,
    pub vulnerability_fix_version: Option<String>,
    pub vulnerability_fix_strategy: Option<String>,
    pub abandonment_threshold: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateResult {
    pub source_directory: Option<String>,
    pub changelog_content: Option<String>,
    pub changelog_url: Option<String>,
    pub dependency_url: Option<String>,
    pub homepage: Option<String>,
    pub deprecation_message: Option<String>,
    pub source_url: Option<String>,
    pub current_version: Option<String>,
    pub is_single_version: Option<bool>,
    pub lookup_name: Option<String>,
    pub skip_reason: Option<String>,
    pub registry_url: Option<String>,
    pub fixed_version: Option<String>,
    pub updates: Vec<LookupUpdate>,
    pub warnings: Vec<crate::workers::types::ValidationMessage>,
    pub versioning: Option<String>,
    pub current_version_age_in_days: Option<u64>,
    pub current_version_timestamp: Option<String>,
    pub vulnerability_fix_version: Option<String>,
    pub vulnerability_fix_strategy: Option<String>,
    pub most_recent_timestamp: Option<String>,
    pub is_abandoned: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_type_as_str() {
        assert_eq!(UpdateType::Pin.as_str(), "pin");
        assert_eq!(UpdateType::Major.as_str(), "major");
        assert_eq!(UpdateType::Minor.as_str(), "minor");
        assert_eq!(UpdateType::Patch.as_str(), "patch");
        assert_eq!(UpdateType::Digest.as_str(), "digest");
        assert_eq!(UpdateType::Bump.as_str(), "bump");
        assert_eq!(UpdateType::Replace.as_str(), "replace");
        assert_eq!(UpdateType::PinDigest.as_str(), "pinDigest");
        assert_eq!(UpdateType::LockFileMaintenance.as_str(), "lockFileMaintenance");
        assert_eq!(UpdateType::LockfileUpdate.as_str(), "lockfileUpdate");
        assert_eq!(UpdateType::Rollback.as_str(), "rollback");
        assert_eq!(UpdateType::Replacement.as_str(), "replacement");
    }

    #[test]
    fn update_type_display() {
        assert_eq!(format!("{}", UpdateType::Major), "major");
        assert_eq!(format!("{}", UpdateType::Pin), "pin");
    }

    #[test]
    fn update_type_from_str() {
        assert_eq!("major".parse(), Ok(UpdateType::Major));
        assert_eq!("minor".parse(), Ok(UpdateType::Minor));
        assert_eq!("patch".parse(), Ok(UpdateType::Patch));
        assert_eq!("pin".parse(), Ok(UpdateType::Pin));
        assert_eq!("digest".parse(), Ok(UpdateType::Digest));
        assert_eq!("bump".parse(), Ok(UpdateType::Bump));
        assert_eq!("replacement".parse(), Ok(UpdateType::Replacement));
        assert_eq!("rollback".parse(), Ok(UpdateType::Rollback));
        assert_eq!("pinDigest".parse(), Ok(UpdateType::PinDigest));
        assert_eq!(
            "lockFileMaintenance".parse(),
            Ok(UpdateType::LockFileMaintenance)
        );
        assert_eq!("lockfileUpdate".parse(), Ok(UpdateType::LockfileUpdate));
    }

    #[test]
    fn update_type_from_str_unknown() {
        let result: Result<UpdateType, String> = "unknown".parse();
        assert!(result.is_err());
    }

    #[test]
    fn update_type_serialization_roundtrip() {
        let variants = [
            UpdateType::Pin,
            UpdateType::Major,
            UpdateType::Minor,
            UpdateType::Patch,
            UpdateType::Digest,
            UpdateType::Replacement,
        ];
        for v in &variants {
            let json = serde_json::to_string(v).unwrap();
            let back: UpdateType = serde_json::from_str(&json).unwrap();
            assert_eq!(*v, back);
        }
    }

    #[test]
    fn lookup_update_default() {
        let u = LookupUpdate::default();
        assert!(u.new_value.is_none());
        assert!(u.update_type.is_none());
        assert!(u.is_pin.is_none());
    }

    #[test]
    fn lookup_update_construct() {
        let u = LookupUpdate {
            new_value: Some("^4.18.0".into()),
            new_major: Some(4),
            update_type: Some(UpdateType::Bump),
            is_bump: Some(true),
            ..Default::default()
        };
        assert_eq!(u.new_value, Some("^4.18.0".into()));
        assert_eq!(u.new_major, Some(4));
        assert_eq!(u.update_type, Some(UpdateType::Bump));
    }

    #[test]
    fn rollback_config_construct() {
        let c = RollbackConfig {
            package_name: "lodash".into(),
            dep_name: Some("lodash".into()),
            datasource: "npm".into(),
            current_value: Some("^4.0.0".into()),
            ..Default::default()
        };
        assert_eq!(c.package_name, "lodash");
        assert_eq!(c.datasource, "npm");
    }

    #[test]
    fn filter_config_default() {
        let c = FilterConfig::default();
        assert!(c.allowed_versions.is_none());
        assert!(c.ignore_deprecated.is_none());
    }

    #[test]
    fn update_result_default() {
        let r = UpdateResult::default();
        assert!(r.updates.is_empty());
        assert!(r.warnings.is_empty());
        assert!(r.skip_reason.is_none());
    }

    #[test]
    fn update_result_with_updates() {
        let r = UpdateResult {
            current_version: Some("1.0.0".into()),
            updates: vec![
                LookupUpdate {
                    new_value: Some("2.0.0".into()),
                    update_type: Some(UpdateType::Major),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        assert_eq!(r.updates.len(), 1);
        assert_eq!(r.updates[0].update_type, Some(UpdateType::Major));
    }

    #[test]
    fn lookup_update_serialization_roundtrip() {
        let u = LookupUpdate {
            new_value: Some("1.2.3".into()),
            update_type: Some(UpdateType::Minor),
            is_pin: Some(false),
            pending_versions: Some(vec!["1.2.4".into()]),
            ..Default::default()
        };
        let json = serde_json::to_string(&u).unwrap();
        let back: LookupUpdate = serde_json::from_str(&json).unwrap();
        assert_eq!(back.new_value, Some("1.2.3".into()));
        assert_eq!(back.update_type, Some(UpdateType::Minor));
    }
}
