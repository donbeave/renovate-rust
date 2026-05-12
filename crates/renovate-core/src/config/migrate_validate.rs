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
    validate_custom_datasources(map, &mut errors);
    validate_status_check_names(map, &mut errors);
    validate_base_branch_patterns(map, &mut errors);
    validate_enabled_managers(map, &mut errors);
    validate_manager_file_patterns(map, &mut errors);
    validate_custom_managers(map, &mut errors);
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

        if rule_map
            .get("matchManagers")
            .is_some_and(|value| !value.is_array())
        {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": "Invalid `packageRules.matchManagers` configuration: is not an array"
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

fn validate_enabled_managers(map: &Map<String, Value>, errors: &mut Vec<Value>) {
    let Some(Value::Array(enabled_managers)) = map.get("enabledManagers") else {
        return;
    };

    let unsupported: Vec<_> = enabled_managers
        .iter()
        .filter_map(Value::as_str)
        .filter(|manager| !is_supported_manager(manager))
        .collect();

    if !unsupported.is_empty() {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": format!("Unsupported enabledManagers: {}", unsupported.join(", "))
        }));
    }
}

fn validate_custom_datasources(map: &Map<String, Value>, errors: &mut Vec<Value>) {
    let Some(Value::Object(custom_datasources)) = map.get("customDatasources") else {
        return;
    };

    for (name, datasource) in custom_datasources {
        let Some(datasource) = datasource.as_object() else {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": format!("Invalid `customDatasources.{name}` configuration: customDatasource is not an object")
            }));
            continue;
        };

        if datasource
            .get("defaultRegistryUrlTemplate")
            .is_some_and(|value| !value.is_string())
        {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": "Invalid `customDatasources.defaultRegistryUrlTemplate` configuration: is a string"
            }));
        }

        if datasource
            .get("description")
            .is_some_and(|value| !value.is_string())
        {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": "Invalid `customDatasources.description` configuration: is not an array of strings"
            }));
        }

        for key in datasource.keys() {
            if !matches!(
                key.as_str(),
                "defaultRegistryUrlTemplate" | "description" | "transformTemplates"
            ) {
                errors.push(json!({
                    "topic": "Configuration Error",
                    "message": format!("Invalid `customDatasources.{key}` configuration: key is not allowed")
                }));
            }
        }

        if datasource.get("transformTemplates").is_some_and(|value| {
            !value
                .as_array()
                .is_some_and(|templates| templates.iter().all(Value::is_string))
        }) {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": "Invalid `customDatasources.transformTemplates` configuration: is not an array of string"
            }));
        }
    }
}

fn is_supported_manager(manager: &str) -> bool {
    matches!(
        manager,
        "cargo" | "custom.regex" | "gradle" | "maven" | "npm" | "pip-compile"
    )
}

fn validate_status_check_names(map: &Map<String, Value>, errors: &mut Vec<Value>) {
    let Some(Value::Object(status_check_names)) = map.get("statusCheckNames") else {
        return;
    };

    for (key, value) in status_check_names {
        if !matches!(
            key.as_str(),
            "artifactError" | "configValidation" | "mergeConfidence"
        ) {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": format!("Invalid `statusCheckNames.statusCheckNames.{key}` configuration: key is not allowed.")
            }));
            continue;
        }

        if !value.is_string() && !value.is_null() {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": format!("Invalid `statusCheckNames.{key}` configuration: status check is not a string.")
            }));
        }
    }
}

fn validate_base_branch_patterns(map: &Map<String, Value>, errors: &mut Vec<Value>) {
    let Some(Value::Array(patterns)) = map.get("baseBranchPatterns") else {
        return;
    };

    for pattern in patterns.iter().filter_map(Value::as_str) {
        if validate_renovate_regex_literal(pattern).is_err() {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": format!("Invalid regExp for baseBranchPatterns: `{pattern}`")
            }));
        }
    }
}

fn validate_manager_file_patterns(map: &Map<String, Value>, errors: &mut Vec<Value>) {
    for (manager, config) in map {
        let Some(config) = config.as_object() else {
            continue;
        };
        let Some(Value::Array(patterns)) = config.get("managerFilePatterns") else {
            continue;
        };
        for pattern in patterns.iter().filter_map(Value::as_str) {
            if validate_renovate_regex_literal(pattern).is_err() {
                errors.push(json!({
                    "topic": "Configuration Error",
                    "message": format!("Invalid regExp for {manager}.managerFilePatterns: `{pattern}`")
                }));
            }
        }
    }
}

fn validate_custom_managers(map: &Map<String, Value>, errors: &mut Vec<Value>) {
    let Some(Value::Array(custom_managers)) = map.get("customManagers") else {
        return;
    };

    for manager in custom_managers {
        let Some(manager) = manager.as_object() else {
            continue;
        };

        let custom_type = manager.get("customType").and_then(Value::as_str);
        match custom_type {
            Some("regex" | "jsonata") => {}
            Some(value) => {
                errors.push(json!({
                    "topic": "Configuration Error",
                    "message": format!("Invalid customType: {value}. Key is not a custom manager")
                }));
                continue;
            }
            None => {
                errors.push(json!({
                    "topic": "Configuration Error",
                    "message": "Each Custom Manager must contain a non-empty customType string"
                }));
                continue;
            }
        }

        let invalid_manager_file_patterns = manager
            .get("managerFilePatterns")
            .and_then(Value::as_array)
            .is_none_or(|patterns| patterns.is_empty());

        if invalid_manager_file_patterns {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": "Each Custom Manager must contain a non-empty managerFilePatterns array"
            }));
        }

        if let Some(Value::Array(patterns)) = manager.get("managerFilePatterns") {
            for pattern in patterns.iter().filter_map(Value::as_str) {
                if validate_renovate_regex_literal(pattern).is_err() {
                    errors.push(json!({
                        "topic": "Configuration Error",
                        "message": format!("Invalid regExp for customManagers.managerFilePatterns: `{pattern}`")
                    }));
                }
            }
        }

        if invalid_manager_file_patterns {
            continue;
        }

        match custom_type {
            Some("regex") => validate_regex_custom_manager(manager, errors),
            Some("jsonata") => validate_jsonata_custom_manager(manager, errors),
            _ => {}
        }
    }
}

fn validate_regex_custom_manager(manager: &Map<String, Value>, errors: &mut Vec<Value>) {
    let Some(match_strings) = manager.get("matchStrings").and_then(Value::as_array) else {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "Each Custom Manager must contain a non-empty matchStrings array"
        }));
        return;
    };

    if match_strings.is_empty() {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "Each Custom Manager `matchStrings` array must have at least one item."
        }));
        return;
    }

    for pattern in match_strings.iter().filter_map(Value::as_str) {
        if let Err(message) = validate_regex_pattern(pattern) {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": format!("Invalid regExp for customManagers.matchStrings: {message}")
            }));
        }
    }

    if !has_template_or_capture(manager, "depNameTemplate", "depName") {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "Each Custom Manager must contain a depName capture group or depNameTemplate"
        }));
    }
    if !has_template_or_capture(manager, "datasourceTemplate", "datasource") {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "Each Custom Manager must contain a datasource capture group or datasourceTemplate"
        }));
    }
    if !has_template_or_capture(manager, "currentValueTemplate", "currentValue") {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "Each Custom Manager must contain a currentValue capture group or currentValueTemplate"
        }));
    }
}

fn validate_jsonata_custom_manager(manager: &Map<String, Value>, errors: &mut Vec<Value>) {
    if manager.get("fileFormat").and_then(Value::as_str).is_none() {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "Each JSONata manager must contain a fileFormat field."
        }));
    }

    let Some(match_strings) = manager.get("matchStrings").and_then(Value::as_array) else {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "Each Custom Manager must contain a non-empty matchStrings array"
        }));
        return;
    };

    if match_strings.is_empty() {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "Each Custom Manager must contain a non-empty matchStrings array"
        }));
        return;
    }

    for query in match_strings.iter().filter_map(Value::as_str) {
        if has_invalid_jsonata_expression(query) {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": format!("Invalid JSONata query for customManagers: `{query}`")
            }));
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

    if regex_body.contains("?+") {
        return Err("unsupported possessive quantifier".to_owned());
    }

    Regex::new(&regex_body)
        .map(|_| ())
        .map_err(|err| err.to_string())
}

fn validate_regex_pattern(pattern: &str) -> Result<(), String> {
    let normalized = normalize_named_capture_syntax(pattern);
    Regex::new(&normalized)
        .map(|_| ())
        .map_err(|err| err.to_string())
}

fn normalize_named_capture_syntax(pattern: &str) -> String {
    Regex::new(r"\(\?<([A-Za-z_][A-Za-z0-9_]*)>")
        .expect("valid named capture regex")
        .replace_all(pattern, "(?P<$1>")
        .into_owned()
}

fn has_template_or_capture(
    manager: &Map<String, Value>,
    template_key: &str,
    capture: &str,
) -> bool {
    manager.get(template_key).and_then(Value::as_str).is_some()
        || manager
            .get("matchStrings")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .any(|pattern| pattern.contains(&format!("?<{capture}>")))
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

    // Ported: "catches invalid customDatasources content" — config/validation.spec.ts line 347
    #[test]
    fn validate_config_catches_invalid_custom_datasources_content() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "customDatasources": {
                    "foo": {
                        "description": 3,
                        "randomKey": "",
                        "defaultRegistryUrlTemplate": [],
                        "transformTemplates": [{}]
                    },
                    "bar": {
                        "description": "foo",
                        "defaultRegistryUrlTemplate": "bar",
                        "transformTemplates": ["foo = \"bar\"", "bar[0]"]
                    }
                }
            }),
        );
        let messages = validation_error_messages(&result);
        assert_eq!(messages.len(), 4);
        assert!(messages.contains(
            &"Invalid `customDatasources.defaultRegistryUrlTemplate` configuration: is a string"
        ));
        assert!(messages.contains(
            &"Invalid `customDatasources.description` configuration: is not an array of strings"
        ));
        assert!(
            messages.contains(
                &"Invalid `customDatasources.randomKey` configuration: key is not allowed"
            )
        );
        assert!(messages.contains(
            &"Invalid `customDatasources.transformTemplates` configuration: is not an array of string"
        ));
    }

    // Ported: "validates invalid statusCheckNames" — config/validation.spec.ts line 384
    #[test]
    fn validate_config_validates_invalid_status_check_names() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "statusCheckNames": {
                    "randomKey": "",
                    "mergeConfidence": 10,
                    "configValidation": "",
                    "artifactError": null
                }
            }),
        );
        let messages = validation_error_messages(&result);
        assert_eq!(messages.len(), 2);
        assert!(messages.contains(&"Invalid `statusCheckNames.mergeConfidence` configuration: status check is not a string."));
        assert!(messages.contains(&"Invalid `statusCheckNames.statusCheckNames.randomKey` configuration: key is not allowed."));
    }

    // Ported: "catches invalid customDatasources record type" — config/validation.spec.ts line 408
    #[test]
    fn validate_config_catches_invalid_custom_datasources_record_type() {
        let result =
            validate_config_for_source("repo", &json!({"customDatasources": {"randomKey": ""}}));
        assert_eq!(
            validation_error_messages(&result),
            vec![
                "Invalid `customDatasources.randomKey` configuration: customDatasource is not an object"
            ]
        );
    }

    // Ported: "catches invalid baseBranchPatterns regex" — config/validation.spec.ts line 423
    #[test]
    fn validate_config_catches_invalid_base_branch_patterns_regex() {
        let result = validate_config_for_source(
            "repo",
            &json!({"baseBranchPatterns": ["/***$}{]][/", "/branch/i"]}),
        );
        assert_eq!(
            result.errors,
            vec![json!({
                "topic": "Configuration Error",
                "message": "Invalid regExp for baseBranchPatterns: `/***$}{]][/`"
            })]
        );
    }

    // Ported: "included managers of the wrong type" — config/validation.spec.ts line 466
    #[test]
    fn validate_config_errors_for_match_managers_wrong_type() {
        let result = validate_config_for_source(
            "repo",
            &json!({"packageRules": [{"matchManagers": "string not an array", "enabled": true}]}),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(
            validation_error_messages(&result),
            vec!["Invalid `packageRules.matchManagers` configuration: is not an array"]
        );
    }

    // Ported: "empty configuration" — config/validation.spec.ts line 484
    #[test]
    fn validate_config_allows_empty_configuration() {
        let result = validate_config_for_source("repo", &json!({}));
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "single not supported manager" — config/validation.spec.ts line 503
    #[test]
    fn validate_config_errors_for_unsupported_enabled_managers() {
        for config in [
            json!({"enabledManagers": ["foo"]}),
            json!({"enabledManagers": ["foo", "bar"]}),
            json!({"enabledManagers": ["foo", "npm", "gradle", "maven"]}),
        ] {
            let result = validate_config_for_source("repo", &config);
            assert!(result.warnings.is_empty());
            assert_eq!(result.errors.len(), 1);
        }
    }

    // Ported: "errors for unsafe managerFilePatterns" — config/validation.spec.ts line 608
    #[test]
    fn validate_config_errors_for_unsafe_manager_file_patterns() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "npm": {"managerFilePatterns": ["/abc ([a-z]+) ([a-z]+))/"]},
                "dockerfile": {"managerFilePatterns": ["/x?+/"]}
            }),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(result.errors.len(), 2);
    }

    // Ported: "validates regEx for each managerFilePatterns of format regex" — config/validation.spec.ts line 627
    #[test]
    fn validate_config_validates_custom_manager_file_pattern_regex() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "customManagers": [{
                    "customType": "regex",
                    "managerFilePatterns": ["/js/", "/***$}{]][/"],
                    "matchStrings": ["^(?<depName>foo)(?<currentValue>bar)$"],
                    "datasourceTemplate": "maven",
                    "versioningTemplate": "gradle"
                }]
            }),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(result.errors.len(), 1);
    }

    // Ported: "errors if customManager has empty managerFilePatterns" — config/validation.spec.ts line 649
    #[test]
    fn validate_config_errors_for_empty_custom_manager_file_patterns() {
        let result = validate_config_for_source(
            "repo",
            &json!({"customManagers": [{"customType": "regex", "managerFilePatterns": []}]}),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(
            validation_error_messages(&result),
            vec!["Each Custom Manager must contain a non-empty managerFilePatterns array"]
        );
    }

    // Ported: "errors if no customManager customType" — config/validation.spec.ts line 675
    #[test]
    fn validate_config_errors_for_missing_custom_manager_type() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "customManagers": [{
                    "managerFilePatterns": ["some-file"],
                    "matchStrings": ["^(?<depName>foo)(?<currentValue>bar)$"],
                    "datasourceTemplate": "maven",
                    "versioningTemplate": "gradle"
                }]
            }),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(
            validation_error_messages(&result),
            vec!["Each Custom Manager must contain a non-empty customType string"]
        );
    }

    // Ported: "errors if invalid customManager customType" — config/validation.spec.ts line 703
    #[test]
    fn validate_config_errors_for_invalid_custom_manager_type() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "customManagers": [{
                    "customType": "unknown",
                    "managerFilePatterns": ["some-file"],
                    "matchStrings": ["^(?<depName>foo)(?<currentValue>bar)$"],
                    "datasourceTemplate": "maven",
                    "versioningTemplate": "gradle"
                }]
            }),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(
            validation_error_messages(&result),
            vec!["Invalid customType: unknown. Key is not a custom manager"]
        );
    }

    // Ported: "errors if empty customManager matchStrings" — config/validation.spec.ts line 732
    #[test]
    fn validate_config_errors_for_empty_custom_manager_match_strings() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "customManagers": [
                    {
                        "customType": "regex",
                        "managerFilePatterns": ["foo"],
                        "matchStrings": [],
                        "depNameTemplate": "foo",
                        "datasourceTemplate": "bar",
                        "currentValueTemplate": "baz"
                    },
                    {
                        "customType": "jsonata",
                        "fileFormat": "json",
                        "managerFilePatterns": ["foo"],
                        "depNameTemplate": "foo",
                        "datasourceTemplate": "bar",
                        "currentValueTemplate": "baz"
                    }
                ]
            }),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(result.errors.len(), 2);
    }

    // Ported: "validates regEx for each matchStrings" — config/validation.spec.ts line 793
    #[test]
    fn validate_config_validates_custom_manager_match_string_regex() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "customManagers": [{
                    "customType": "regex",
                    "managerFilePatterns": ["Dockerfile"],
                    "matchStrings": ["***$}{]]["],
                    "depNameTemplate": "foo",
                    "datasourceTemplate": "bar",
                    "currentValueTemplate": "baz"
                }]
            }),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(result.errors.len(), 1);
    }

    // Ported: "error if no fileFormat in custom JSONata manager" — config/validation.spec.ts line 815
    #[test]
    fn validate_config_errors_for_jsonata_manager_missing_file_format() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "customManagers": [{
                    "customType": "jsonata",
                    "managerFilePatterns": ["package.json"],
                    "matchStrings": ["packages.{\"depName\": name, \"currentValue\": version, \"datasource\": \"npm\"}"]
                }]
            }),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(
            validation_error_messages(&result),
            vec!["Each JSONata manager must contain a fileFormat field."]
        );
    }

    // Ported: "validates JSONata query for each matchStrings" — config/validation.spec.ts line 841
    #[test]
    fn validate_config_validates_jsonata_manager_queries() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "customManagers": [{
                    "customType": "jsonata",
                    "fileFormat": "json",
                    "managerFilePatterns": ["package.json"],
                    "matchStrings": ["packages.{"],
                    "depNameTemplate": "foo",
                    "datasourceTemplate": "bar",
                    "currentValueTemplate": "baz"
                }]
            }),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(
            validation_error_messages(&result),
            vec!["Invalid JSONata query for customManagers: `packages.{`"]
        );
    }

    // Ported: "validates all possible regex manager options" — config/validation.spec.ts line 871
    #[test]
    fn validate_config_validates_all_regex_custom_manager_options() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "customManagers": [{
                    "customType": "regex",
                    "managerFilePatterns": ["Dockerfile"],
                    "matchStrings": ["***$}{]]["]
                }]
            }),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(result.errors.len(), 4);
    }

    // Ported: "passes if customManager fields are present" — config/validation.spec.ts line 890
    #[test]
    fn validate_config_allows_valid_custom_managers() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "customManagers": [
                    {
                        "customType": "regex",
                        "managerFilePatterns": ["Dockerfile"],
                        "matchStrings": ["ENV (?<currentValue>.*?)\\s"],
                        "depNameTemplate": "foo",
                        "datasourceTemplate": "bar",
                        "registryUrlTemplate": "foobar",
                        "extractVersionTemplate": "^(?<version>v\\d+\\.\\d+)",
                        "depTypeTemplate": "apple"
                    },
                    {
                        "customType": "jsonata",
                        "fileFormat": "json",
                        "managerFilePatterns": ["package.json"],
                        "matchStrings": ["packages.{\"depName\": depName, \"currentValue\": version, \"datasource\": \"npm\"}"]
                    }
                ]
            }),
        );
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
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

    fn validation_error_messages(result: &super::ValidationResult) -> Vec<&str> {
        result
            .errors
            .iter()
            .map(|error| error["message"].as_str().unwrap())
            .collect()
    }
}
