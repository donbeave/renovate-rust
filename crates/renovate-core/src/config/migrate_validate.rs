//! Config migration plus validation.
//!
//! Renovate reference: `lib/config/migrate-validate.ts`.

use regex::Regex;
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

    validate_template_options(map, &mut errors);
    validate_package_rules(map, &mut errors, &mut warnings);

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

fn validate_template_options(map: &Map<String, Value>, errors: &mut Vec<Value>) {
    for key in ["commitMessage"] {
        if let Some(template) = map.get(key).and_then(Value::as_str)
            && has_invalid_template(template)
        {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": format!("Invalid template for {key}")
            }));
        }
    }
}

fn validate_package_rules(
    map: &Map<String, Value>,
    errors: &mut Vec<Value>,
    warnings: &mut Vec<Value>,
) {
    let Some(Value::Array(package_rules)) = map.get("packageRules") else {
        return;
    };

    let has_base_branch_patterns = map
        .get("baseBranchPatterns")
        .and_then(Value::as_array)
        .is_some_and(|patterns| !patterns.is_empty());

    for rule in package_rules {
        let Some(rule_map) = rule.as_object() else {
            continue;
        };

        if !has_base_branch_patterns
            && rule_map
                .get("matchBaseBranches")
                .and_then(Value::as_array)
                .is_some_and(|branches| !branches.is_empty())
        {
            warnings.push(json!({
                "topic": "Configuration Warning",
                "message": "packageRules.matchBaseBranches should only be used when baseBranchPatterns is configured."
            }));
        }

        for key in [
            "allowedVersions",
            "matchCurrentValue",
            "matchCurrentVersion",
            "matchNewValue",
        ] {
            if let Some(pattern) = rule_map.get(key).and_then(Value::as_str)
                && let Err(message) = validate_renovate_regex_literal(pattern)
            {
                errors.push(json!({
                    "topic": "Configuration Error",
                    "message": format!("Invalid regex for {key}: {message}")
                }));
            }
        }

        if let Some(Value::Array(expressions)) = rule_map.get("matchJsonata") {
            for expression in expressions.iter().filter_map(Value::as_str) {
                if has_invalid_jsonata_expression(expression) {
                    errors.push(json!({
                        "topic": "Configuration Error",
                        "message": "Invalid JSONata expression"
                    }));
                }
            }
        }
    }
}

fn has_invalid_template(template: &str) -> bool {
    template.contains("{{{") && !template.contains("}}}")
}

fn has_invalid_jsonata_expression(expression: &str) -> bool {
    expression.contains("{{{") || braces_are_unbalanced(expression)
}

fn braces_are_unbalanced(expression: &str) -> bool {
    let mut depth = 0usize;
    for ch in expression.chars() {
        match ch {
            '{' => depth += 1,
            '}' => {
                let Some(next_depth) = depth.checked_sub(1) else {
                    return true;
                };
                depth = next_depth;
            }
            _ => {}
        }
    }
    depth != 0
}

fn validate_renovate_regex_literal(pattern: &str) -> Result<(), String> {
    let pattern = pattern.strip_prefix('!').unwrap_or(pattern);
    let Some(rest) = pattern.strip_prefix('/') else {
        return Ok(());
    };
    let Some(close) = rest.rfind('/') else {
        return Ok(());
    };
    let body = &rest[..close];
    let flags = &rest[close + 1..];

    let regex_body = if flags == "i" {
        format!("(?i){body}")
    } else {
        body.to_owned()
    };

    Regex::new(&regex_body)
        .map(|_| ())
        .map_err(|err| err.to_string())
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

    // Ported: "catches invalid templates" — config/validation.spec.ts line 156
    #[test]
    fn validate_config_catches_invalid_templates() {
        let result =
            validate_config_for_source("repo", &json!({"commitMessage": "{{{something}}"}));
        assert_eq!(result.errors.len(), 1);
    }

    // Ported: "catches invalid jsonata expressions" — config/validation.spec.ts line 165
    #[test]
    fn validate_config_catches_invalid_jsonata_expressions() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "packageRules": [{
                    "matchJsonata": ["packageName = \"foo\"", "{{{something wrong}"],
                    "enabled": true
                }]
            }),
        );
        assert_eq!(result.errors.len(), 1);
        assert!(
            result.errors[0]["message"]
                .as_str()
                .unwrap()
                .contains("Invalid JSONata expression")
        );
    }

    // Ported: "catches invalid allowedVersions regex" — config/validation.spec.ts line 179
    #[test]
    fn validate_config_catches_invalid_allowed_versions_regex() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "packageRules": [
                    {"matchPackageNames": ["foo"], "allowedVersions": "/^2/"},
                    {"matchPackageNames": ["bar"], "allowedVersions": "/***$}{]][/"},
                    {"matchPackageNames": ["baz"], "allowedVersions": "!/^2/"},
                    {"matchPackageNames": ["quack"], "allowedVersions": "!/***$}{]][/"},
                    {"matchPackageNames": ["quack"], "allowedVersions": "/quaCk/i"}
                ]
            }),
        );
        assert_eq!(result.errors.len(), 2);
    }

    // Ported: "catches invalid matchCurrentValue" — config/validation.spec.ts line 209
    #[test]
    fn validate_config_catches_invalid_match_current_value_regex() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "packageRules": [
                    {"matchPackageNames": ["foo"], "matchCurrentValue": "/^2/", "enabled": true},
                    {"matchPackageNames": ["bar"], "matchCurrentValue": "^1", "enabled": true},
                    {"matchPackageNames": ["quack"], "matchCurrentValue": "<1.0.0", "enabled": true},
                    {"matchPackageNames": ["foo"], "matchCurrentValue": "/^2/i", "enabled": true},
                    {"matchPackageNames": ["bad"], "matchNewValue": "/^2(/", "enabled": true}
                ]
            }),
        );
        assert_eq!(result.errors.len(), 1);
    }

    // Ported: "catches invalid matchNewValue" — config/validation.spec.ts line 243
    #[test]
    fn validate_config_catches_invalid_match_new_value_regex() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "packageRules": [
                    {"matchPackageNames": ["foo"], "matchNewValue": "/^2/", "enabled": true},
                    {"matchPackageNames": ["bar"], "matchNewValue": "^1", "enabled": true},
                    {"matchPackageNames": ["quack"], "matchNewValue": "<1.0.0", "enabled": true},
                    {"matchPackageNames": ["foo"], "matchNewValue": "/^2/i", "enabled": true},
                    {"matchPackageNames": ["bad"], "matchNewValue": "/^2(/", "enabled": true}
                ]
            }),
        );
        assert_eq!(result.errors.len(), 1);
    }

    // Ported: "validates matchBaseBranches" — config/validation.spec.ts line 277
    #[test]
    fn validate_config_validates_match_base_branches() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "baseBranchPatterns": ["foo"],
                "packageRules": [{"matchBaseBranches": ["foo"], "addLabels": ["foo"]}]
            }),
        );
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "catches invalid matchBaseBranches when baseBranchPatterns is not defined" — config/validation.spec.ts line 295
    #[test]
    fn validate_config_warns_for_match_base_branches_without_base_branch_patterns() {
        let result = validate_config_for_source(
            "repo",
            &json!({"packageRules": [{"matchBaseBranches": ["foo"], "addLabels": ["foo"]}]}),
        );
        assert!(result.errors.is_empty());
        assert_eq!(result.warnings.len(), 1);
    }

    // Ported: "catches invalid matchCurrentVersion regex" — config/validation.spec.ts line 312
    #[test]
    fn validate_config_catches_invalid_match_current_version_regex() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "packageRules": [
                    {"matchPackageNames": ["foo"], "matchCurrentVersion": "/^2/", "enabled": true},
                    {"matchPackageNames": ["bar"], "matchCurrentVersion": "/***$}{]][/", "enabled": true},
                    {"matchPackageNames": ["baz"], "matchCurrentVersion": "!/^2/", "enabled": true},
                    {"matchPackageNames": ["quack"], "matchCurrentVersion": "!/***$}{]][/", "enabled": true},
                    {"matchPackageNames": ["foo"], "matchCurrentVersion": "/^2/i", "enabled": true}
                ]
            }),
        );
        assert_eq!(result.errors.len(), 2);
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
