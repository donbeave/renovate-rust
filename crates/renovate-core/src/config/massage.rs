//! Config normalization before validation.
//!
//! Renovate reference: `lib/config/massage.ts`.

use serde_json::{Map, Value};

const UPDATE_TYPES: &[&str] = &["major", "minor", "patch", "pin", "digest", "rollback"];

/// Config fields that accept a bare string as a shorthand for a single-element array.
///
/// Mirrors the `allowString: true` option entries in `lib/config/options/index.ts`.
const ALLOW_STRING_FIELDS: &[&str] = &[
    "description",
    "schedule",
    "automergeSchedule",
    "autodiscoverFilter",
    "matchCategories",
    "matchRepositories",
    "matchBaseBranches",
    "matchManagers",
    "matchDatasources",
    "matchDepTypes",
    "matchPackageNames",
    "matchDepNames",
    "matchSourceUrls",
    "matchRegistryUrls",
    "managerFilePatterns",
    "gitNoVerify",
];

/// Return a massaged Renovate config value.
pub fn massage_config(config: &Value) -> Value {
    match config {
        Value::Object(map) => {
            let mut massaged = massage_object(map);
            massage_package_rules(&mut massaged);
            Value::Object(massaged)
        }
        other => other.clone(),
    }
}

fn massage_object(map: &Map<String, Value>) -> Map<String, Value> {
    let mut massaged = Map::new();
    for (key, value) in map {
        let value = if key == "minimumReleaseAge" && is_zero_duration(value) {
            Value::Null
        } else if ALLOW_STRING_FIELDS.contains(&key.as_str()) && value.is_string() {
            Value::Array(vec![value.clone()])
        } else if let Value::Array(values) = value {
            Value::Array(
                values
                    .iter()
                    .map(|value| {
                        if value.is_object() {
                            massage_config(value)
                        } else {
                            value.clone()
                        }
                    })
                    .collect(),
            )
        } else if value.is_object() && key != "encrypted" {
            massage_config(value)
        } else {
            value.clone()
        };
        massaged.insert(key.clone(), value);
    }
    massaged
}

fn massage_package_rules(config: &mut Map<String, Value>) {
    let Some(Value::Array(package_rules)) = config.get("packageRules") else {
        return;
    };
    if package_rules.is_empty() {
        return;
    }

    let mut new_rules = Vec::new();
    for rule in package_rules {
        let Value::Object(rule_map) = rule else {
            new_rules.push(rule.clone());
            continue;
        };

        new_rules.push(rule.clone());
        for update_type in UPDATE_TYPES {
            let Some(Value::Object(update_config)) = rule_map.get(*update_type) else {
                continue;
            };

            let mut new_rule = Map::new();
            for (key, value) in rule_map {
                if key.starts_with("match") || key.starts_with("exclude") {
                    new_rule.insert(key.clone(), value.clone());
                }
            }
            let mut match_update_types = rule_map
                .get("matchUpdateTypes")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();
            match_update_types.push(Value::String((*update_type).to_owned()));
            new_rule.insert(
                "matchUpdateTypes".to_owned(),
                Value::Array(match_update_types),
            );
            for (key, value) in update_config {
                new_rule.insert(key.clone(), value.clone());
            }
            new_rules.push(Value::Object(new_rule));
        }
    }

    for rule in &mut new_rules {
        if let Value::Object(rule_map) = rule {
            for update_type in UPDATE_TYPES {
                rule_map.remove(*update_type);
            }
        }
    }

    new_rules.retain(|rule| {
        let Value::Object(rule_map) = rule else {
            return true;
        };
        !rule_map
            .keys()
            .all(|key| key.starts_with("match") || key.starts_with("exclude"))
    });
    config.insert("packageRules".to_owned(), Value::Array(new_rules));
}

fn is_zero_duration(value: &Value) -> bool {
    matches!(
        value.as_str(),
        Some("0 days" | "0 day" | "0 hours" | "0 minutes" | "0 seconds")
    )
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::massage_config;

    // Ported: "returns empty" — lib/config/massage.spec.ts line 6
    #[test]
    fn massage_config_returns_empty() {
        assert_eq!(massage_config(&json!({})), json!({}));
    }

    // Ported: "massages strings to array" — lib/config/massage.spec.ts line 12
    #[test]
    fn massage_config_converts_allowed_string_to_array() {
        assert_eq!(
            massage_config(&json!({"schedule": "before 5am"})),
            json!({"schedule": ["before 5am"]})
        );
        // description also has allowString: true
        assert_eq!(
            massage_config(&json!({"description": "some description"})),
            json!({"description": ["some description"]})
        );
    }

    // Ported: "normalizes zero minimumReleaseAge to null" — lib/config/massage.spec.ts line 20
    #[test]
    fn massage_config_normalizes_zero_minimum_release_age() {
        assert_eq!(
            massage_config(&json!({"minimumReleaseAge": "0 days"})),
            json!({"minimumReleaseAge": null})
        );
    }

    // Ported: "normalizes zero minimumReleaseAge in packageRules" — lib/config/massage.spec.ts line 30
    #[test]
    fn massage_config_normalizes_zero_minimum_release_age_in_package_rules() {
        let result = massage_config(&json!({
            "packageRules": [{
                "matchPackageNames": ["foo"],
                "minimumReleaseAge": "0 days",
                "patch": {"minimumReleaseAge": "0 days"}
            }]
        }));

        assert_eq!(
            result,
            json!({"packageRules": [
                {"matchPackageNames": ["foo"], "minimumReleaseAge": null},
                {"matchPackageNames": ["foo"], "matchUpdateTypes": ["patch"], "minimumReleaseAge": null}
            ]})
        );
    }

    // Ported: "massages packageRules matchUpdateTypes" — lib/config/massage.spec.ts line 58
    #[test]
    fn massage_config_expands_package_rule_update_types() {
        let result = massage_config(&json!({
            "packageRules": [{
                "matchPackageNames": ["foo"],
                "separateMajorMinor": false,
                "minor": {"semanticCommitType": "feat"},
                "patch": {"semanticCommitType": "fix"}
            }]
        }));

        assert_eq!(
            result,
            json!({"packageRules": [
                {"matchPackageNames": ["foo"], "separateMajorMinor": false},
                {"matchPackageNames": ["foo"], "matchUpdateTypes": ["minor"], "semanticCommitType": "feat"},
                {"matchPackageNames": ["foo"], "matchUpdateTypes": ["patch"], "semanticCommitType": "fix"}
            ]})
        );
    }

    // Ported: "filters packageRules with only match/exclude" — lib/config/massage.spec.ts line 95
    #[test]
    fn massage_config_filters_package_rules_with_only_match_or_exclude() {
        let result = massage_config(&json!({
            "packageRules": [{
                "matchBaseBranches": ["main"],
                "major": {"enabled": true}
            }]
        }));

        assert_eq!(
            result,
            json!({"packageRules": [
                {"matchBaseBranches": ["main"], "matchUpdateTypes": ["major"], "enabled": true}
            ]})
        );
    }

    // Ported: "does not massage lockFileMaintenance" — lib/config/massage.spec.ts line 110
    #[test]
    fn massage_config_does_not_expand_lock_file_maintenance() {
        let result = massage_config(&json!({
            "packageRules": [{
                "matchManagers": ["helmv3"],
                "matchBaseBranches": ["release/ft10/1.9.x"],
                "lockFileMaintenance": {"enabled": true},
                "schedule": ["at any time"]
            }]
        }));

        assert_eq!(
            result,
            json!({"packageRules": [{
                "lockFileMaintenance": {"enabled": true},
                "matchBaseBranches": ["release/ft10/1.9.x"],
                "matchManagers": ["helmv3"],
                "schedule": ["at any time"]
            }]})
        );
    }
}
