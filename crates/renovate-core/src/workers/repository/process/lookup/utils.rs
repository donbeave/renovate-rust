//! Lookup utility functions.
//!
//! Mirrors `lib/workers/repository/process/lookup/utils.ts`.

use super::types::{LookupUpdate, LookupUpdateConfig, UpdateType};

pub fn get_from_config<'a>(config: &'a LookupUpdateConfig, key: &str) -> Option<&'a str> {
    match key {
        "replacementName" => config.replacement_name.as_deref(),
        "replacementVersion" => config.replacement_version.as_deref(),
        "datasource" => Some(&config.rollback.datasource),
        "packageName" => Some(&config.rollback.package_name),
        "currentValue" => config.rollback.current_value.as_deref(),
        "currentDigest" => config.current_digest.as_deref(),
        "currentVersion" => config.current_version.as_deref(),
        "lockedVersion" => config.locked_version.as_deref(),
        _ => None,
    }
}

pub fn add_candidate(
    updates: &mut Vec<LookupUpdate>,
    new_name: Option<String>,
    new_value: Option<String>,
    update_type: UpdateType,
) {
    updates.push(LookupUpdate {
        update_type: Some(update_type),
        new_name,
        new_value,
        ..Default::default()
    });
}

pub fn add_replacement_update_if_valid(
    updates: &mut Vec<LookupUpdate>,
    config: &LookupUpdateConfig,
) {
    let new_name = determine_new_replacement_name(config);
    let new_value = determine_new_replacement_value(config);

    if new_name != config.rollback.package_name
        || new_value.as_deref() != config.rollback.current_value.as_deref()
    {
        updates.push(LookupUpdate {
            update_type: Some(UpdateType::Replacement),
            new_name: Some(new_name),
            new_value,
            ..Default::default()
        });
    }
}

pub fn is_replacement_rules_configured(config: &LookupUpdateConfig) -> bool {
    config.replacement_name.is_some()
        || config.replacement_name_template.is_some()
        || config.replacement_version.is_some()
        || config.replacement_version_template.is_some()
}

pub fn determine_new_replacement_name(config: &LookupUpdateConfig) -> String {
    if let Some(ref name) = config.replacement_name {
        return name.clone();
    }
    if let Some(ref template) = config.replacement_name_template {
        return apply_template(template, config);
    }
    config.rollback.package_name.clone()
}

pub fn determine_new_replacement_value(config: &LookupUpdateConfig) -> Option<String> {
    let new_version = get_new_version(config);
    if new_version.is_none() {
        return config.rollback.current_value.clone();
    }
    new_version
}

fn get_new_version(config: &LookupUpdateConfig) -> Option<String> {
    if config.replacement_version.is_some() {
        return config.replacement_version.clone();
    }
    if let Some(ref template) = config.replacement_version_template {
        return Some(apply_template(template, config));
    }
    None
}

fn apply_template(template: &str, _config: &LookupUpdateConfig) -> String {
    let mut result = template.to_owned();
    result = result.replace("{{packageName}}", "TODO");
    result = result.replace("{{depName}}", "TODO");
    result = result.replace("{{currentValue}}", "TODO");
    result = result.replace("{{currentVersion}}", "TODO");
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workers::repository::process::lookup::types::RollbackConfig;

    fn make_config() -> LookupUpdateConfig {
        LookupUpdateConfig {
            rollback: RollbackConfig {
                package_name: "lodash".into(),
                current_value: Some("4.17.0".into()),
                dep_name: Some("lodash".into()),
                datasource: "npm".into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn get_from_config_datasource() {
        let config = make_config();
        assert_eq!(get_from_config(&config, "datasource"), Some("npm"));
    }

    #[test]
    fn get_from_config_package_name() {
        let config = make_config();
        assert_eq!(get_from_config(&config, "packageName"), Some("lodash"));
    }

    #[test]
    fn get_from_config_current_value() {
        let config = make_config();
        assert_eq!(get_from_config(&config, "currentValue"), Some("4.17.0"));
    }

    #[test]
    fn get_from_config_unknown_key() {
        let config = make_config();
        assert_eq!(get_from_config(&config, "unknown"), None);
    }

    #[test]
    fn add_candidate_basic() {
        let mut updates = Vec::new();
        add_candidate(
            &mut updates,
            Some("new-pkg".into()),
            Some("2.0.0".into()),
            UpdateType::Replacement,
        );
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0].update_type, Some(UpdateType::Replacement));
        assert_eq!(updates[0].new_name, Some("new-pkg".into()));
    }

    #[test]
    fn add_candidate_multiple() {
        let mut updates = Vec::new();
        add_candidate(&mut updates, None, Some("2.0.0".into()), UpdateType::Major);
        add_candidate(&mut updates, None, Some("1.0.1".into()), UpdateType::Patch);
        assert_eq!(updates.len(), 2);
    }

    #[test]
    fn add_replacement_update_if_valid_new_name() {
        let mut config = make_config();
        config.replacement_name = Some("new-lodash".into());
        let mut updates = Vec::new();
        add_replacement_update_if_valid(&mut updates, &config);
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0].new_name, Some("new-lodash".into()));
    }

    #[test]
    fn add_replacement_update_if_valid_same_name_same_value() {
        let mut config = make_config();
        config.replacement_name = Some("lodash".into());
        config.replacement_version = Some("4.17.0".into());
        let mut updates = Vec::new();
        add_replacement_update_if_valid(&mut updates, &config);
        assert!(updates.is_empty());
    }

    #[test]
    fn is_replacement_rules_configured_false() {
        let config = make_config();
        assert!(!is_replacement_rules_configured(&config));
    }

    #[test]
    fn is_replacement_rules_configured_true() {
        let mut config = make_config();
        config.replacement_name = Some("new-pkg".into());
        assert!(is_replacement_rules_configured(&config));
    }

    #[test]
    fn is_replacement_rules_configured_template() {
        let mut config = make_config();
        config.replacement_name_template = Some("{{packageName}}-v2".into());
        assert!(is_replacement_rules_configured(&config));
    }

    #[test]
    fn determine_new_replacement_name_explicit() {
        let mut config = make_config();
        config.replacement_name = Some("new-lodash".into());
        assert_eq!(determine_new_replacement_name(&config), "new-lodash");
    }

    #[test]
    fn determine_new_replacement_name_default() {
        let config = make_config();
        assert_eq!(determine_new_replacement_name(&config), "lodash");
    }

    #[test]
    fn determine_new_replacement_value_with_version() {
        let mut config = make_config();
        config.replacement_version = Some("5.0.0".into());
        assert_eq!(determine_new_replacement_value(&config), Some("5.0.0".into()));
    }

    #[test]
    fn determine_new_replacement_value_fallback() {
        let config = make_config();
        assert_eq!(determine_new_replacement_value(&config), Some("4.17.0".into()));
    }
}
