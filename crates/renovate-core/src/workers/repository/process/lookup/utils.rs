//! Replacement utils for lookup (addReplacementUpdateIfValid, isReplacementRulesConfigured,
//! determineNewReplacementName, determineNewReplacementValue, getNewVersion).
//!
//! @parity `lib/workers/repository/process/lookup/utils.ts` partial — addReplacementUpdateIfValid + isReplacementRulesConfigured + determineNewReplacementName/Value + getNewVersion (template/compile stubs, getNewValue with isReplacement); single test ported (covering "handles replacements - name only without pinDigests enabled" from index.spec). Full callers (in lookup/index) and template/getRangeStrategy wiring pending other units.
//!
//! Mirrors `lib/workers/repository/process/lookup/utils.ts`.

use crate::workers::repository::process::lookup::types::{LookupUpdate, LookupUpdateConfig};

/// Port of addReplacementUpdateIfValid.
pub fn add_replacement_update_if_valid(
    updates: &mut Vec<LookupUpdate>,
    config: &LookupUpdateConfig,
) {
    let replacement_new_name = determine_new_replacement_name(config);
    let replacement_new_value = determine_new_replacement_value(config);

    if config.rollback.package_name.as_deref() != Some(replacement_new_name.as_str())
        || config.rollback.current_value.as_deref() != replacement_new_value.as_deref()
    {
        updates.push(LookupUpdate {
            update_type: Some(
                crate::workers::repository::process::lookup::types::UpdateType::Replacement,
            ),
            new_name: Some(replacement_new_name),
            new_value: replacement_new_value,
            ..Default::default()
        });
    }
}

/// Port of isReplacementRulesConfigured.
pub fn is_replacement_rules_configured(config: &LookupUpdateConfig) -> bool {
    is_non_empty_string(config.replacement_name.as_deref())
        || is_non_empty_string(config.replacement_name_template.as_deref())
        || is_non_empty_string(config.replacement_version.as_deref())
        || is_non_empty_string(config.replacement_version_template.as_deref())
}

fn is_non_empty_string(s: Option<&str>) -> bool {
    s.map_or(false, |s| !s.is_empty())
}

/// Port of determineNewReplacementName.
pub fn determine_new_replacement_name(config: &LookupUpdateConfig) -> String {
    if let Some(name) = &config.replacement_name {
        return name.clone();
    }
    if let Some(tpl) = &config.replacement_name_template {
        // template.compile(tpl, config, true) — simplified for unit (real uses util/template)
        return tpl.clone(); // stub; in full would compile with context
    }
    config.rollback.package_name.clone().unwrap_or_default()
}

/// Port of determineNewReplacementValue.
pub fn determine_new_replacement_value(config: &LookupUpdateConfig) -> Option<String> {
    let new_version = get_new_version(config);
    if new_version.is_none() {
        return config.rollback.current_value.clone();
    }
    // versioningApi.getNewValue({ currentValue, newVersion, rangeStrategy, isReplacement: true })
    // stub for unit; in full would use actual
    new_version
}

fn get_new_version(config: &LookupUpdateConfig) -> Option<String> {
    if let Some(v) = &config.replacement_version {
        return Some(v.clone());
    }
    if let Some(tpl) = &config.replacement_version_template {
        // template.compile(tpl, config, true)
        return Some(tpl.clone()); // stub
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_replacements_name_only_without_pin_digests_enabled() {
        // Ported: "handles replacements - name only without pinDigests enabled" — lib/workers/repository/process/lookup/index.spec.ts line 4847
        // (exercises addReplacementUpdateIfValid + isReplacementRulesConfigured + determine* for replacementName case)
        let mut config = LookupUpdateConfig {
            rollback: crate::workers::repository::process::lookup::types::RollbackConfig {
                package_name: Some("openjdk".to_string()),
                current_value: Some("17.0.0".to_string()),
                datasource: Some("docker".to_string()),
                ..Default::default()
            },
            replacement_name: Some("eclipse-temurin".into()),
            ..Default::default()
        };

        assert!(is_replacement_rules_configured(&config));

        let mut updates = vec![];
        add_replacement_update_if_valid(&mut updates, &config);

        assert_eq!(updates.len(), 1);
        let u = &updates[0];
        assert_eq!(
            u.update_type,
            Some(crate::workers::repository::process::lookup::types::UpdateType::Replacement)
        );
        assert_eq!(u.new_name.as_deref(), Some("eclipse-temurin"));
        // newValue falls back to current in this stub (real getNewValue would compute)
        assert_eq!(u.new_value.as_deref(), Some("17.0.0"));
    }
}
