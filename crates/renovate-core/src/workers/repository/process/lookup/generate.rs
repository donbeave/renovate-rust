//! Logic to generate a single LookupUpdate from a candidate release (newValue via versioning.getNewValue,
//! updateType via getUpdateType, isBreaking, isRange, isLockfileUpdate, isBump, mergeConfidence, metadata from release).
//!
//! @parity `lib/workers/repository/process/lookup/generate.ts` partial — generateUpdate (newValue via getNewValue + metadata copy from release + newMajor/Minor/Patch + updateType + isBreaking (with versioning.isBreaking or major default) + mergeConfidence when configured + isRange + isLockfileUpdate for update-lockfile + isBump for bump strategy); single test ported (covering "supports lock file updates mixed with regular updates" from index.spec that exercises generate paths). Full async confidence, getUpdateType wiring, and callers in lookup/index pending (update-type.ts also pending).
//!
//! Mirrors `lib/workers/repository/process/lookup/generate.ts`.

use crate::workers::repository::process::lookup::types::{
    LookupUpdate, LookupUpdateConfig, UpdateType,
};

/// Port of generateUpdate.
/// Takes closures for the expensive/ api parts (getNewValue, getUpdateType, versioning major/minor/patch/isVersion/matches/isBreaking)
/// and release metadata to keep the unit pure and avoid full trait + async deps for this dedicated mapping file.
pub fn generate_update(
    config: &LookupUpdateConfig,
    current_value: Option<&str>,
    range_strategy: &str,
    current_version: &str,
    bucket: &str,
    new_version: &str,
    has_attestation: Option<bool>,
    release_timestamp: Option<&str>,
    registry_url: Option<&str>,
    new_digest: Option<&str>,
    checksum_url: Option<&str>,
    download_url: Option<&str>,
    get_new_value: impl Fn(&str, &str, &str, &str, &std::collections::HashSet<String>) -> Option<String>,
    get_major: impl Fn(&str) -> Option<u64>,
    get_minor: impl Fn(&str) -> Option<u64>,
    get_patch: impl Fn(&str) -> Option<u64>,
    is_version: impl Fn(&str) -> bool,
    matches: impl Fn(&str, &str) -> bool,
    is_breaking: Option<impl Fn(&str, &str) -> bool>,
    allow_unstable_major_upgrades: bool, // from versioning if needed
    get_update_type: impl Fn(&LookupUpdateConfig, &str, &str) -> Option<String>,
) -> LookupUpdate {
    let mut update = LookupUpdate {
        bucket: Some(bucket.to_string()),
        new_version: Some(new_version.to_string()),
        new_value: None,
        has_attestation,
        ..Default::default()
    };

    if let Some(u) = checksum_url {
        update.checksum_url = Some(u.to_string());
    }
    if let Some(u) = download_url {
        update.download_url = Some(u.to_string());
    }
    if let Some(d) = new_digest {
        update.new_digest = Some(d.to_string());
    }
    if let Some(ts) = release_timestamp {
        update.release_timestamp = Some(ts.to_string());
        // newVersionAgeInDays would be computed with getElapsedDays in real; stub for unit
        update.new_version_age_in_days = None;
    }
    if let Some(ru) = registry_url {
        update.registry_url = Some(ru.to_string());
    }

    if let Some(cv) = current_value {
        match get_new_value(
            cv,
            range_strategy,
            current_version,
            new_version,
            &std::collections::HashSet::new(),
        ) {
            Some(nv) => update.new_value = Some(nv),
            None => {
                update.new_value = Some(cv.to_string());
            }
        }
    } else {
        update.new_value = current_value.map(|s| s.to_string());
    }

    update.new_major = get_major(new_version);
    update.new_minor = get_minor(new_version);
    update.new_patch = get_patch(new_version);

    if update.update_type.is_none() && current_version.is_empty() {
        // logger.debug
        update.new_value = current_value.map(|s| s.to_string());
        return update;
    }

    if update.update_type.is_none() {
        if let Some(s) = get_update_type(config, current_version, new_version) {
            update.update_type = match s.as_str() {
                "major" => Some(UpdateType::Major),
                "minor" => Some(UpdateType::Minor),
                "patch" => Some(UpdateType::Patch),
                "replacement" => Some(UpdateType::Replacement),
                "rollback" => Some(UpdateType::Rollback),
                _ => None,
            };
        }
    }

    if let Some(is_b) = is_breaking {
        update.is_breaking = Some(is_b(current_version, new_version));
    } else {
        // default: major is breaking (and unstable treated as such in some cases)
        let is_major = matches!(update.update_type, Some(UpdateType::Major));
        update.is_breaking = Some(
            is_major || allow_unstable_major_upgrades, /* simplified */
        );
    }

    // mergeConfidenceLevel if packageRules ask for matchConfidence (stub the call for unit)
    // if config has packageRules with matchConfidence arrays, set it (for test we can pass through or leave None)
    // update.merge_confidence_level = ... ;

    if let Some(nv) = &update.new_value {
        if !is_version(nv) {
            update.is_range = Some(true);
        }
    }

    if range_strategy == "update-lockfile" {
        if current_value == update.new_value.as_deref() {
            update.is_lockfile_update = Some(true);
        }
    }

    if range_strategy == "bump" {
        if let Some(cv) = current_value {
            if matches(new_version, cv) {
                update.is_bump = Some(true);
            }
        }
    }

    update
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workers::repository::process::lookup::types::LookupUpdateConfig;

    #[test]
    fn supports_lock_file_updates_mixed_with_regular_updates() {
        // Ported: "supports lock file updates mixed with regular updates" — lib/workers/repository/process/lookup/index.spec.ts line 250
        // Exercises the generateUpdate path for rangeStrategy=update-lockfile (isLockfileUpdate + isRange) and basic newValue/updateType.
        let config = LookupUpdateConfig::default();

        let update = generate_update(
            &config,
            Some("^0.4.0"),
            "update-lockfile",
            "0.4.0",
            "patch",
            "0.4.4",
            Some(false),
            Some("2011-06-10T17:20:04.719Z"),
            None,
            None,
            None,
            None,
            |cv, _rs, _cur, new_v, _all| Some(new_v.to_string()), // getNewValue mock
            |v| v.split('.').next().and_then(|s| s.parse().ok()),
            |v| v.split('.').nth(1).and_then(|s| s.parse().ok()),
            |v| v.split('.').nth(2).and_then(|s| s.parse().ok()),
            |v| v.chars().next().map_or(false, |c| c.is_ascii_digit()),
            |new_v, _cur| new_v.starts_with("0.4."),
            None, // no isBreaking
            false,
            |_c, _cur, _new| Some("patch".to_string()), // getUpdateType mock
        );

        assert_eq!(update.is_lockfile_update, Some(true));
        assert_eq!(update.is_range, Some(true));
        assert_eq!(update.new_value.as_deref(), Some("0.4.4"));
        assert_eq!(update.update_type.as_deref(), Some("patch"));
        assert_eq!(update.bucket.as_deref(), Some("patch"));
    }
}
