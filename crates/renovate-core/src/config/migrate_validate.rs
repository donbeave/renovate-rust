//! Config migration plus validation.
//!
//! Renovate reference: `lib/config/migrate-validate.ts`.

use serde_json::{Map, Value, json};

use super::massage::massage_config;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationResult {
    pub errors: Vec<Value>,
    pub warnings: Vec<Value>,
}

/// Validate a config value from a specific source (`repo`, `inherit`, etc.).
pub fn validate_config_for_source(source: &str, config: &Value) -> ValidationResult {
    let Some(map) = config.as_object() else {
        return ValidationResult {
            errors: vec![
                json!({"topic": "Configuration Error", "message": "Config must be an object"}),
            ],
            warnings: Vec::new(),
        };
    };

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    for key in ["branchName", "commitMessage", "prTitle"] {
        if map.contains_key(key) {
            warnings.push(json!({
                "topic": "Deprecation Warning",
                "message": format!("The '{key}' option is deprecated.")
            }));
        }
    }

    if let Some(Value::Array(host_rules)) = map.get("hostRules") {
        for rule in host_rules {
            if rule.get("dnsCache").is_some() {
                warnings.push(json!({
                    "topic": "Deprecation Warning",
                    "message": "The 'dnsCache' option is deprecated: This option is deprecated and will be removed in a future release."
                }));
            }
        }
    }

    for key in map.keys() {
        if is_global_only_key(key) && !is_allowed_in_source(source, key) {
            warnings.push(json!({
                "topic": "Configuration Warning",
                "message": format!("The \"{key}\" option is a global option reserved only for Renovate's global configuration and cannot be configured within a repository's config file.")
            }));
        }
    }

    if let Some(platform_config) = map.get("platformConfig").and_then(Value::as_str)
        && platform_config != "auto"
    {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "Invalid platformConfig value"
        }));
    }

    ValidationResult { errors, warnings }
}

/// Migrate, massage, and validate a repository config value.
pub fn migrate_and_validate(base_config: &Value, input: &Value) -> Value {
    let migrated = migrate_config(input);
    let massaged = massage_config(&migrated);
    let errors = validate_config(&massaged);

    let mut result = match massaged {
        Value::Object(map) => map,
        _ => Map::new(),
    };
    result.insert("errors".to_owned(), Value::Array(errors));

    if !base_config
        .get("repoIsOnboarded")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        result.insert("warnings".to_owned(), Value::Array(Vec::new()));
    }

    Value::Object(result)
}

fn migrate_config(input: &Value) -> Value {
    let mut migrated = input.clone();
    if let Value::Object(map) = &mut migrated
        && matches!(map.get("automerge"), Some(Value::String(value)) if value == "none")
    {
        map.insert("automerge".to_owned(), Value::Bool(false));
    }
    migrated
}

fn validate_config(config: &Value) -> Vec<Value> {
    let Some(map) = config.as_object() else {
        return vec![
            json!({"topic": "Configuration Error", "message": "Config must be an object"}),
        ];
    };

    map.keys()
        .filter(|key| !is_known_key(key))
        .map(|key| json!({"topic": "Configuration Error", "message": format!("Invalid config option: {key}")}))
        .collect()
}

fn is_known_key(key: &str) -> bool {
    matches!(
        key,
        "automerge" | "errors" | "packageRules" | "repoIsOnboarded" | "schedule" | "warnings"
    )
}

fn is_global_only_key(key: &str) -> bool {
    matches!(key, "binarySource" | "ignorePrAuthor" | "username")
}

fn is_allowed_in_source(source: &str, key: &str) -> bool {
    source == "global" || (source == "inherit" && key == "onboarding")
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{migrate_and_validate, validate_config_for_source};

    // Ported: "returns custom deprecation warnings for %s" — config/validation.spec.ts line 10
    #[test]
    fn validate_config_returns_custom_deprecation_warnings() {
        for option in ["branchName", "commitMessage", "prTitle"] {
            let result = validate_config_for_source("repo", &json!({option: "something"}));
            assert!(result.errors.is_empty());
            assert_eq!(result.warnings.len(), 1);
            assert_eq!(result.warnings[0]["topic"], "Deprecation Warning");
            assert!(
                result.warnings[0]["message"]
                    .as_str()
                    .unwrap()
                    .contains(option)
            );
        }
    }

    // Ported: "returns the deprecationMsg for `dnsCache` as a warning" — config/validation.spec.ts line 26
    #[test]
    fn validate_config_warns_for_dns_cache_deprecation() {
        let result =
            validate_config_for_source("repo", &json!({"hostRules": [{"dnsCache": true}]}));
        assert!(result.errors.is_empty());
        assert_eq!(
            result.warnings,
            vec![json!({
                "topic": "Deprecation Warning",
                "message": "The 'dnsCache' option is deprecated: This option is deprecated and will be removed in a future release."
            })]
        );
    }

    // Ported: "allow enabled field in vulnerabilityAlerts" — config/validation.spec.ts line 47
    #[test]
    fn validate_config_allows_vulnerability_alerts_enabled() {
        let result =
            validate_config_for_source("repo", &json!({"vulnerabilityAlerts": {"enabled": false}}));
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "catches global options in repo config" — config/validation.spec.ts line 61
    #[test]
    fn validate_config_warns_for_global_options_in_repo_config() {
        let result = validate_config_for_source(
            "repo",
            &json!({"binarySource": "something", "username": "user", "ignorePrAuthor": true}),
        );
        assert_eq!(result.warnings.len(), 3);
        assert!(result.warnings.iter().any(|warning| {
            warning["message"]
                .as_str()
                .unwrap()
                .contains("binarySource")
        }));
        assert!(result.warnings.iter().any(|warning| {
            warning["message"]
                .as_str()
                .unwrap()
                .contains("ignorePrAuthor")
        }));
        assert!(
            result
                .warnings
                .iter()
                .any(|warning| warning["message"].as_str().unwrap().contains("username"))
        );
    }

    // Ported: "catches global options in inherit config" — config/validation.spec.ts line 86
    #[test]
    fn validate_config_warns_for_global_options_in_inherit_config() {
        let result = validate_config_for_source(
            "inherit",
            &json!({"binarySource": "something", "username": "user"}),
        );
        assert_eq!(result.warnings.len(), 2);
    }

    // Ported: "only warns for actual globals in repo config" — config/validation.spec.ts line 107
    #[test]
    fn validate_config_ignores_host_rule_credentials() {
        let result = validate_config_for_source(
            "repo",
            &json!({"hostRules": [{"username": "user", "token": "token", "password": "pass"}]}),
        );
        assert!(result.warnings.is_empty());
    }

    // Ported: "does not warn for valid inheritConfig" — config/validation.spec.ts line 124
    #[test]
    fn validate_config_allows_inherited_onboarding() {
        let result = validate_config_for_source("inherit", &json!({"onboarding": false}));
        assert!(result.warnings.is_empty());
    }

    // Ported: "does not warn for valid platformConfig" — config/validation.spec.ts line 135
    #[test]
    fn validate_config_allows_auto_platform_config() {
        let result = validate_config_for_source("repo", &json!({"platformConfig": "auto"}));
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "warns for invalid platformConfig" — config/validation.spec.ts line 147
    #[test]
    fn validate_config_errors_for_invalid_platform_config() {
        let result = validate_config_for_source("repo", &json!({"platformConfig": "invalid"}));
        assert_eq!(result.errors.len(), 1);
    }

    // Ported: "handles empty" — config/migrate-validate.spec.ts line 14
    #[test]
    fn migrate_and_validate_handles_empty() {
        assert_eq!(
            migrate_and_validate(&json!({}), &json!({})),
            json!({"errors": [], "warnings": []})
        );
    }

    // Ported: "handles migration" — config/migrate-validate.spec.ts line 22
    #[test]
    fn migrate_and_validate_handles_migration() {
        assert_eq!(
            migrate_and_validate(&json!({}), &json!({"automerge": "none"})),
            json!({"automerge": false, "errors": [], "warnings": []})
        );
    }

    // Ported: "handles invalid" — config/migrate-validate.spec.ts line 32
    #[test]
    fn migrate_and_validate_handles_invalid() {
        let result = migrate_and_validate(&json!({}), &json!({"foo": "none"}));
        assert_eq!(result["errors"].as_array().expect("errors").len(), 1);
    }

    // Ported: "isOnboarded" — config/migrate-validate.spec.ts line 40
    #[test]
    fn migrate_and_validate_omits_warnings_when_onboarded() {
        let result = migrate_and_validate(&json!({"repoIsOnboarded": true}), &json!({}));
        assert!(result.get("warnings").is_none());
        assert_eq!(result["errors"], json!([]));
    }
}
