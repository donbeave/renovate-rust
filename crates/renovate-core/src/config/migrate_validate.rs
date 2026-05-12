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
    validate_manager_file_pattern_parents(map, &mut warnings);
    validate_manager_object_nesting(map, &mut errors);
    validate_registry_aliases(map, &mut errors);
    validate_host_type_parent(map, &mut warnings);
    validate_extends(source, map, &mut errors);
    validate_top_level_registry_urls(map, &mut warnings);
    validate_schedule(map, &mut errors);
    validate_host_rules(map, &mut errors);
    validate_env(map, &mut errors);
    validate_positive_integers(map, &mut errors);
    validate_bump_version(map, &mut errors);
    validate_custom_managers(map, &mut errors);
    validate_constraints(map, &mut errors, &mut warnings);
    validate_constraints_versioning(map, &mut errors);
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
    matches!(
        key,
        "binarySource" | "customEnvVariables" | "ignorePrAuthor" | "username"
    )
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

        if is_selector_only_package_rule(rule_map) {
            warnings.push(json!({
                "topic": "Configuration Warning",
                "message": "packageRules entries should include at least one action in addition to match selectors"
            }));
        }

        if rule_map.get("matchUpdateTypes").is_some() && rule_map.get("registryUrls").is_some() {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": "packageRules cannot combine matchUpdateTypes and registryUrls"
            }));
        }

        if let Some(Value::Array(patterns)) = rule_map.get("matchRepositories")
            && contains_match_all_with_other_patterns(patterns)
        {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": "packageRules matchRepositories contains * or ** along with other patterns"
            }));
        }

        if rule_map
            .get("extends")
            .and_then(Value::as_array)
            .is_some_and(|presets| {
                presets
                    .iter()
                    .filter_map(Value::as_str)
                    .any(|preset| preset.starts_with("group:"))
            })
        {
            warnings.push(json!({
                "topic": "Configuration Warning",
                "message": "Nested group presets inside packageRules can be hard to reason about"
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

fn is_selector_only_package_rule(rule: &Map<String, Value>) -> bool {
    !rule.is_empty()
        && rule
            .keys()
            .all(|key| key.starts_with("match") || key == "excludePackageNames")
}

fn contains_match_all_with_other_patterns(patterns: &[Value]) -> bool {
    patterns.len() > 1
        && patterns
            .iter()
            .filter_map(Value::as_str)
            .any(|pattern| matches!(pattern, "*" | "**"))
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
        "cargo"
            | "custom.regex"
            | "dockerfile"
            | "gradle"
            | "maven"
            | "npm"
            | "pip-compile"
            | "pyenv"
    )
}

fn is_supported_constraint(name: &str) -> bool {
    matches!(name, "golang" | "gomodMod" | "node")
}

fn is_additional_constraint(name: &str) -> bool {
    matches!(name, "gomodMod")
}

fn is_valid_versioning_scheme(versioning: &str) -> bool {
    versioning == "semver" || versioning.starts_with("regex:")
}

fn is_simple_version_constraint(version: &str) -> bool {
    version == "latest"
        || version
            .chars()
            .all(|ch| ch.is_ascii_digit() || matches!(ch, '.' | '<' | '>' | '=' | '^' | '~' | ' '))
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

fn validate_manager_file_pattern_parents(map: &Map<String, Value>, warnings: &mut Vec<Value>) {
    if map.get("managerFilePatterns").is_some() {
        warnings.push(json!({
            "topic": "managerFilePatterns",
            "message": "\"managerFilePatterns\" can't be used in \".\". Allowed objects: manager config and customManagers"
        }));
    }

    for (manager, config) in map {
        let Some(config) = config.as_object() else {
            continue;
        };
        for (child, child_config) in config {
            if child_config
                .as_object()
                .is_some_and(|child| child.get("managerFilePatterns").is_some())
            {
                warnings.push(json!({
                    "topic": format!("{manager}.{child}.managerFilePatterns"),
                    "message": format!("\"managerFilePatterns\" can't be used in \"{child}\". Allowed objects: manager config and customManagers")
                }));
            }
        }
    }
}

fn validate_manager_object_nesting(map: &Map<String, Value>, errors: &mut Vec<Value>) {
    for (manager, config) in map {
        if !is_supported_manager(manager) {
            continue;
        }
        let Some(config) = config.as_object() else {
            continue;
        };
        for child in config.keys() {
            if is_supported_manager(child) {
                errors.push(json!({
                    "topic": "Configuration Error",
                    "message": format!("Manager `{child}` cannot be nested inside manager `{manager}`")
                }));
            }
        }
    }
}

fn validate_registry_aliases(map: &Map<String, Value>, errors: &mut Vec<Value>) {
    let Some(Value::Object(registry_aliases)) = map.get("registryAliases") else {
        return;
    };

    for (alias, value) in registry_aliases {
        if !value.is_string() {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": format!("Invalid `registryAliases.registryAliases.{alias}` configuration: value is not a string")
            }));
        }
    }
}

fn validate_host_type_parent(map: &Map<String, Value>, warnings: &mut Vec<Value>) {
    if map.get("hostType").is_some() {
        warnings.push(json!({
            "topic": "hostType",
            "message": "\"hostType\" should be configured inside hostRules"
        }));
    }
}

fn validate_extends(source: &str, map: &Map<String, Value>, errors: &mut Vec<Value>) {
    let Some(Value::Array(extends)) = map.get("extends") else {
        return;
    };

    if extends.iter().any(|value| !value.is_string()) {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "Invalid `extends` configuration: presets must be strings"
        }));
        return;
    }

    if extends.iter().filter_map(Value::as_str).any(|preset| {
        preset.starts_with("github>") && (preset.contains("//") && preset.contains(['@', '#']))
    }) {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "Invalid preset syntax"
        }));
    }

    if source != "global"
        && extends
            .iter()
            .filter_map(Value::as_str)
            .any(|preset| preset.starts_with("global:"))
    {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "`global:` presets can only be used in global configuration"
        }));
    }
}

fn validate_top_level_registry_urls(map: &Map<String, Value>, warnings: &mut Vec<Value>) {
    for key in ["registryUrls", "defaultRegistryUrls"] {
        if map.get(key).is_some() {
            warnings.push(json!({
                "topic": "Configuration Warning",
                "message": format!("Setting `{key}` at the top level of your config will apply it to all managers")
            }));
        }
    }
}

fn validate_schedule(map: &Map<String, Value>, errors: &mut Vec<Value>) {
    let Some(Value::Array(schedules)) = map.get("schedule") else {
        return;
    };

    for schedule in schedules.iter().filter_map(Value::as_str) {
        let fields: Vec<_> = schedule.split_whitespace().collect();
        if fields.len() == 5 && fields.first().is_some_and(|minute| *minute != "*") {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": format!("Invalid schedule: `Invalid schedule: \"{schedule}\" has cron syntax, but doesn't have * as minutes`")
            }));
        }
    }
}

fn validate_host_rules(map: &Map<String, Value>, errors: &mut Vec<Value>) {
    let Some(Value::Array(host_rules)) = map.get("hostRules") else {
        return;
    };

    for (idx, rule) in host_rules.iter().enumerate() {
        let Some(rule) = rule.as_object() else {
            continue;
        };

        if let Some(match_host) = rule.get("matchHost") {
            match match_host.as_str() {
                None => errors.push(json!({
                    "topic": "Configuration Error",
                    "message": format!("Configuration option `hostRules[{idx}].matchHost` should be a string")
                })),
                Some("") => errors.push(json!({
                    "topic": "Configuration Error",
                    "message": "Invalid value for hostRules matchHost. It cannot be an empty string."
                })),
                Some(value) if value.contains("://") && !value.starts_with("http") => {
                    errors.push(json!({
                        "topic": "Configuration Error",
                        "message": format!("hostRules matchHost `{value}` is not a valid URL.")
                    }));
                }
                _ => {}
            }
        }

        if let Some(Value::Object(headers)) = rule.get("headers") {
            for (header, value) in headers {
                if !value.is_string() {
                    errors.push(json!({
                        "topic": "Configuration Error",
                        "message": "Invalid hostRules headers value configuration: header must be a string."
                    }));
                } else if !header.starts_with("X-") {
                    errors.push(json!({
                        "topic": "Configuration Error",
                        "message": format!("hostRules header `{header}` is not allowed by this bot's `allowedHeaders`.")
                    }));
                }
            }
        }
    }
}

fn validate_env(map: &Map<String, Value>, errors: &mut Vec<Value>) {
    if let Some(Value::Object(env)) = map.get("env") {
        for (name, value) in env {
            if !name.starts_with("SOME") {
                errors.push(json!({
                    "topic": "Configuration Error",
                    "message": format!("Env variable name `{name}` is not allowed by this bot's `allowedEnv`.")
                }));
            }
            if !value.is_string() {
                errors.push(json!({
                    "topic": "Configuration Error",
                    "message": format!("Invalid env variable value: `env.{name}` must be a string.")
                }));
            }
        }
    }

    for (key, value) in map {
        if key == "env" {
            continue;
        }
        if value
            .as_object()
            .is_some_and(|object| object.get("env").is_some())
        {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": format!("The \"env\" object can only be configured at the top level of a config but was found inside \"{key}\"")
            }));
        }
    }

    if let Some(Value::Array(package_rules)) = map.get("packageRules") {
        for (idx, rule) in package_rules.iter().enumerate() {
            if rule
                .as_object()
                .is_some_and(|object| object.get("env").is_some())
            {
                errors.push(json!({
                    "topic": "Configuration Error",
                    "message": format!("The \"env\" object can only be configured at the top level of a config but was found inside \"packageRules[{idx}]\"")
                }));
            }
        }
    }
}

fn validate_positive_integers(map: &Map<String, Value>, errors: &mut Vec<Value>) {
    if map
        .get("azureWorkItemId")
        .and_then(Value::as_i64)
        .is_some_and(|value| value < 0)
    {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "Configuration option `azureWorkItemId` should be a positive integer. Found negative value instead."
        }));
    }
}

fn validate_bump_version(map: &Map<String, Value>, errors: &mut Vec<Value>) {
    if let Some(Value::Object(bump_version)) = map.get("bumpVersion") {
        validate_bump_version_object("bumpVersion", bump_version, errors);
    }

    if let Some(Value::Array(package_rules)) = map.get("packageRules") {
        for (idx, rule) in package_rules.iter().enumerate() {
            if let Some(bump_version) = rule.get("bumpVersion").and_then(Value::as_object) {
                validate_bump_version_object(
                    &format!("packageRules[{idx}].bumpVersion"),
                    bump_version,
                    errors,
                );
            }
        }
    }
}

fn validate_bump_version_object(
    path: &str,
    bump_version: &Map<String, Value>,
    errors: &mut Vec<Value>,
) {
    if bump_version
        .get("filePatterns")
        .and_then(Value::as_array)
        .is_none_or(|patterns| patterns.is_empty())
    {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": format!("{path} must contain a non-empty filePatterns array")
        }));
    } else if bump_version
        .get("matchStrings")
        .and_then(Value::as_array)
        .is_none_or(|patterns| patterns.is_empty())
    {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": format!("{path} must contain a non-empty matchStrings array")
        }));
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

        validate_custom_manager_keys(manager, errors);

        match custom_type {
            Some("regex") => validate_regex_custom_manager(manager, errors),
            Some("jsonata") => validate_jsonata_custom_manager(manager, errors),
            _ => {}
        }
    }
}

fn validate_custom_manager_keys(manager: &Map<String, Value>, errors: &mut Vec<Value>) {
    for key in manager.keys() {
        if !matches!(
            key.as_str(),
            "customType"
                | "currentValueTemplate"
                | "datasourceTemplate"
                | "depNameTemplate"
                | "depTypeTemplate"
                | "extractVersionTemplate"
                | "fileFormat"
                | "managerFilePatterns"
                | "matchStrings"
                | "packageNameTemplate"
                | "registryUrlTemplate"
                | "versioningTemplate"
        ) {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": format!("Invalid customManager field: {key}")
            }));
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

    let mut has_invalid_query = false;
    for query in match_strings.iter().filter_map(Value::as_str) {
        if has_invalid_jsonata_expression(query) {
            has_invalid_query = true;
            errors.push(json!({
                "topic": "Configuration Error",
                "message": format!("Invalid JSONata query for customManagers: `{query}`")
            }));
        }
    }
    if has_invalid_query {
        return;
    }

    if !has_template_or_query_field(manager, "currentValueTemplate", "currentValue") {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "JSONata Managers must contain currentValueTemplate configuration or currentValue in the query "
        }));
    }
    if !has_template_or_query_field(manager, "datasourceTemplate", "datasource") {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "JSONata Managers must contain datasourceTemplate configuration or datasource in the query "
        }));
    }
    if !has_template_or_query_field(manager, "depNameTemplate", "depName")
        && !has_template_or_query_field(manager, "packageNameTemplate", "packageName")
    {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "JSONata Managers must contain depName or packageName in the query or their templates"
        }));
    }
}

fn validate_constraints(
    map: &Map<String, Value>,
    errors: &mut Vec<Value>,
    warnings: &mut Vec<Value>,
) {
    let Some(constraints) = map.get("constraints") else {
        return;
    };

    let Some(constraints) = constraints.as_object() else {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "Configuration option `constraints` should be a json object"
        }));
        return;
    };

    for (name, value) in constraints {
        let Some(version) = value.as_str() else {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": format!("Configuration option `constraints.{name}` should be an object of key-value pairs of constraints and their value")
            }));
            continue;
        };

        if !is_supported_constraint(name) {
            warnings.push(json!({
                "topic": "Configuration Error",
                "message": format!("Configuration option `constraints.{name}`: `{name}` is not a supported constraint name")
            }));
        } else if name == "node" && !is_simple_version_constraint(version) {
            warnings.push(json!({
                "topic": "Configuration Error",
                "message": format!("Configuration option `constraints.node={version}` is not a valid tool version constraint, according to `node` versioning")
            }));
        }
    }
}

fn validate_constraints_versioning(map: &Map<String, Value>, errors: &mut Vec<Value>) {
    let Some(constraints_versioning) = map.get("constraintsVersioning") else {
        return;
    };

    let Some(constraints_versioning) = constraints_versioning.as_object() else {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "Configuration option `constraintsVersioning` should be a json object"
        }));
        return;
    };

    for (name, value) in constraints_versioning {
        let Some(versioning) = value.as_str() else {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": format!("Configuration option `constraintsVersioning.{name}` should be an object of key-value pairs of additional constraint names and their versioning")
            }));
            continue;
        };

        if name == "golang" {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": "Configuration option `constraintsVersioning.golang` is not a valid additional constraint name, as `golang` is a tool name, and `constraintsVersioning` can only override the versioning for a non-tool constraint"
            }));
        } else if !is_additional_constraint(name) {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": format!("Configuration option `constraintsVersioning.{name}`: `{name}` is not a known additional constraint name")
            }));
        } else if !is_valid_versioning_scheme(versioning) {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": format!("Configuration option `constraintsVersioning.{name}={versioning}`: `{versioning}` is not a valid versioning scheme")
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

fn has_template_or_query_field(
    manager: &Map<String, Value>,
    template_key: &str,
    field: &str,
) -> bool {
    manager.get(template_key).and_then(Value::as_str).is_some()
        || manager
            .get("matchStrings")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .any(|query| query.contains(field))
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

    // Ported: "errors if extra customManager fields are present" — config/validation.spec.ts line 922
    #[test]
    fn validate_config_errors_for_extra_custom_manager_fields() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "customManagers": [{
                    "customType": "regex",
                    "managerFilePatterns": ["Dockerfile"],
                    "matchStrings": ["ENV (?<currentValue>.*?)\\s"],
                    "depNameTemplate": "foo",
                    "datasourceTemplate": "bar",
                    "depTypeTemplate": "apple",
                    "automerge": true
                }]
            }),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(result.errors.len(), 1);
    }

    // Ported: "errors if customManager fields are missing" — config/validation.spec.ts line 945
    #[test]
    fn validate_config_errors_for_missing_regex_custom_manager_fields() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "customManagers": [{
                    "customType": "regex",
                    "managerFilePatterns": ["Dockerfile"],
                    "matchStrings": ["ENV (.*?)\\s"],
                    "depNameTemplate": "foo",
                    "datasourceTemplate": "bar"
                }]
            }),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(result.errors.len(), 1);
    }

    // Ported: "errors if customManager fields are missing: JSONataManager" — config/validation.spec.ts line 967
    #[test]
    fn validate_config_errors_for_missing_jsonata_custom_manager_fields() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "customManagers": [{
                    "customType": "jsonata",
                    "fileFormat": "json",
                    "managerFilePatterns": ["package.json"],
                    "matchStrings": ["packages"]
                }]
            }),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(
            validation_error_messages(&result),
            vec![
                "JSONata Managers must contain currentValueTemplate configuration or currentValue in the query ",
                "JSONata Managers must contain datasourceTemplate configuration or datasource in the query ",
                "JSONata Managers must contain depName or packageName in the query or their templates",
            ]
        );
    }

    // Ported: "ignore keys" — config/validation.spec.ts line 1000
    #[test]
    fn validate_config_ignores_schema_key() {
        let result = validate_config_for_source("repo", &json!({"$schema": "renovate.json"}));
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "validates timezone preset" — config/validation.spec.ts line 1013
    #[test]
    fn validate_config_allows_timezone_presets() {
        let result = validate_config_for_source(
            "repo",
            &json!({"extends": [":timezone", ":timezone(Europe/Berlin)"]}),
        );
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "can contain a valid tool name for Containerbase" — config/validation.spec.ts line 1027
    #[test]
    fn validate_config_allows_containerbase_constraint_tool() {
        let result =
            validate_config_for_source("repo", &json!({"constraints": {"golang": "1.26.0"}}));
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "can contain a constraint for a non-Containerbase tool" — config/validation.spec.ts line 1042
    #[test]
    fn validate_config_allows_non_containerbase_constraint_tool() {
        let result =
            validate_config_for_source("repo", &json!({"constraints": {"gomodMod": "latest"}}));
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "warns if an unsupported constraint is specified" — config/validation.spec.ts line 1057
    #[test]
    fn validate_config_warns_for_unsupported_constraint() {
        let result =
            validate_config_for_source("repo", &json!({"constraints": {"not-supported": "4.5.6"}}));
        assert!(result.errors.is_empty());
        assert_eq!(
            result.warnings,
            vec![json!({
                "topic": "Configuration Error",
                "message": "Configuration option `constraints.not-supported`: `not-supported` is not a supported constraint name"
            })]
        );
    }

    // Ported: "warns if a constraint is not valid" — config/validation.spec.ts line 1079
    #[test]
    fn validate_config_warns_for_invalid_constraint_value() {
        let result =
            validate_config_for_source("repo", &json!({"constraints": {"node": "1.2.3foo"}}));
        assert!(result.errors.is_empty());
        assert_eq!(
            result.warnings,
            vec![json!({
                "topic": "Configuration Error",
                "message": "Configuration option `constraints.node=1.2.3foo` is not a valid tool version constraint, according to `node` versioning"
            })]
        );
    }

    // Ported: "errors if constraints is a malformed object" — config/validation.spec.ts line 1100
    #[test]
    fn validate_config_errors_for_malformed_constraints_object() {
        let result =
            validate_config_for_source("repo", &json!({"constraints": {"packageRules": [{}]}}));
        assert!(result.warnings.is_empty());
        assert_eq!(
            result.errors,
            vec![json!({
                "topic": "Configuration Error",
                "message": "Configuration option `constraints.packageRules` should be an object of key-value pairs of constraints and their value"
            })]
        );
    }

    // Ported: "errors if constraints is a malformed array" — config/validation.spec.ts line 1120
    #[test]
    fn validate_config_errors_for_malformed_constraints_array() {
        let result = validate_config_for_source("repo", &json!({"constraints": [1, 2, 3]}));
        assert!(result.warnings.is_empty());
        assert_eq!(
            result.errors,
            vec![json!({
                "topic": "Configuration Error",
                "message": "Configuration option `constraints` should be a json object"
            })]
        );
    }

    // Ported: "cannot contain a valid tool name for Containerbase" — config/validation.spec.ts line 1142
    #[test]
    fn validate_config_errors_for_containerbase_tool_constraints_versioning() {
        let result = validate_config_for_source(
            "repo",
            &json!({"constraintsVersioning": {"golang": "semver"}}),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(
            result.errors,
            vec![json!({
                "topic": "Configuration Error",
                "message": "Configuration option `constraintsVersioning.golang` is not a valid additional constraint name, as `golang` is a tool name, and `constraintsVersioning` can only override the versioning for a non-tool constraint"
            })]
        );
    }

    // Ported: "can contain a constraint for a non-Containerbase tool" — config/validation.spec.ts line 1164
    #[test]
    fn validate_config_allows_non_containerbase_constraints_versioning() {
        let result = validate_config_for_source(
            "repo",
            &json!({"constraintsVersioning": {"gomodMod": "semver"}}),
        );
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "cannot contain an additional constraint name with an invalid versioning scheme" — config/validation.spec.ts line 1179
    #[test]
    fn validate_config_errors_for_invalid_constraints_versioning_scheme() {
        let result = validate_config_for_source(
            "repo",
            &json!({"constraintsVersioning": {"gomodMod": "not-supported-versioning"}}),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(
            result.errors,
            vec![json!({
                "topic": "Configuration Error",
                "message": "Configuration option `constraintsVersioning.gomodMod=not-supported-versioning`: `not-supported-versioning` is not a valid versioning scheme"
            })]
        );
    }

    // Ported: "can contain an additional constraint name with a regex versioning scheme" — config/validation.spec.ts line 1200
    #[test]
    fn validate_config_allows_regex_constraints_versioning_scheme() {
        let result = validate_config_for_source(
            "repo",
            &json!({"constraintsVersioning": {"gomodMod": "regex:^(?<major>\\d+?)\\.(?<minor>\\d+?)(\\.(?<patch>\\d+))?$"}}),
        );
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "cannot contain an unsupported constraint" — config/validation.spec.ts line 1216
    #[test]
    fn validate_config_errors_for_unknown_constraints_versioning_name() {
        let result = validate_config_for_source(
            "repo",
            &json!({"constraintsVersioning": {"not-supported": "4.5.6"}}),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(
            result.errors,
            vec![json!({
                "topic": "Configuration Error",
                "message": "Configuration option `constraintsVersioning.not-supported`: `not-supported` is not a known additional constraint name"
            })]
        );
    }

    // Ported: "errors if constraintsVersioning is a malformed object" — config/validation.spec.ts line 1238
    #[test]
    fn validate_config_errors_for_malformed_constraints_versioning_object() {
        let result = validate_config_for_source(
            "repo",
            &json!({"constraintsVersioning": {"packageRules": [{}]}}),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(
            result.errors,
            vec![json!({
                "topic": "Configuration Error",
                "message": "Configuration option `constraintsVersioning.packageRules` should be an object of key-value pairs of additional constraint names and their versioning"
            })]
        );
    }

    // Ported: "errors if constraintsVersioning is a malformed array" — config/validation.spec.ts line 1260
    #[test]
    fn validate_config_errors_for_malformed_constraints_versioning_array() {
        let result =
            validate_config_for_source("repo", &json!({"constraintsVersioning": [1, 2, 3]}));
        assert!(result.warnings.is_empty());
        assert_eq!(
            result.errors,
            vec![json!({
                "topic": "Configuration Error",
                "message": "Configuration option `constraintsVersioning` should be a json object"
            })]
        );
    }

    // Ported: "validates object with ignored children" — config/validation.spec.ts line 1281
    #[test]
    fn validate_config_allows_object_with_ignored_children() {
        let result = validate_config_for_source("repo", &json!({"prBodyDefinitions": {}}));
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "validates valid registryAlias objects" — config/validation.spec.ts line 1294
    #[test]
    fn validate_config_allows_valid_registry_aliases() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "registryAliases": {
                    "example1": "http://www.example.com",
                    "example2": "https://www.example2.com/example"
                }
            }),
        );
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "errors if registryAliases depth is more than 1" — config/validation.spec.ts line 1309
    #[test]
    fn validate_config_errors_for_nested_registry_aliases() {
        let result = validate_config_for_source(
            "repo",
            &json!({"registryAliases": {"sample": {"example1": "http://www.example.com"}}}),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(
            validation_error_messages(&result),
            vec![
                "Invalid `registryAliases.registryAliases.sample` configuration: value is not a string"
            ]
        );
    }

    // Ported: "errors if registryAliases have invalid value" — config/validation.spec.ts line 1331
    #[test]
    fn validate_config_errors_for_invalid_registry_alias_value() {
        let result = validate_config_for_source(
            "repo",
            &json!({"registryAliases": {"example1": 123, "example2": "http://www.example.com"}}),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(
            validation_error_messages(&result),
            vec![
                "Invalid `registryAliases.registryAliases.example1` configuration: value is not a string"
            ]
        );
    }

    // Ported: "errors if managerFilePatterns has wrong parent" — config/validation.spec.ts line 1352
    #[test]
    fn validate_config_warns_for_wrong_manager_file_patterns_parent() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "managerFilePatterns": ["foo"],
                "npm": {
                    "managerFilePatterns": ["package\\.json"],
                    "minor": {"managerFilePatterns": ["bar"]}
                },
                "customManagers": [{
                    "customType": "regex",
                    "managerFilePatterns": ["build.gradle"],
                    "matchStrings": ["^(?<depName>foo)(?<currentValue>bar)$"],
                    "datasourceTemplate": "maven",
                    "versioningTemplate": "gradle"
                }]
            }),
        );
        assert!(result.errors.is_empty());
        assert_eq!(result.warnings.len(), 2);
    }

    // Ported: "errors if manager objects are nested" — config/validation.spec.ts line 1395
    #[test]
    fn validate_config_errors_for_nested_manager_objects() {
        let result = validate_config_for_source(
            "repo",
            &json!({"pyenv": {"enabled": false}, "maven": {"gradle": {"enabled": false}}}),
        );
        assert_eq!(result.errors.len(), 1);
        assert!(result.warnings.is_empty());
    }

    // Ported: "warns if hostType has the wrong parent" — config/validation.spec.ts line 1415
    #[test]
    fn validate_config_warns_for_host_type_wrong_parent() {
        let result = validate_config_for_source("repo", &json!({"hostType": "npm"}));
        assert!(result.errors.is_empty());
        assert_eq!(result.warnings.len(), 1);
    }

    // Ported: "validates preset values" — config/validation.spec.ts line 1429
    #[test]
    fn validate_config_errors_for_non_string_preset_values() {
        let result = validate_config_for_source("repo", &json!({"extends": ["foo", "bar", 42]}));
        assert!(result.warnings.is_empty());
        assert_eq!(result.errors.len(), 1);
    }

    // Ported: "errors on invalid preset syntax" — config/validation.spec.ts line 1442
    #[test]
    fn validate_config_errors_for_invalid_preset_syntax() {
        let result = validate_config_for_source(
            "repo",
            &json!({"extends": ["github>owner/repo//path@commitHash", "github>owner/repo//path#commitHash"]}),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(result.errors.len(), 1);
    }

    // Ported: "warns if only selectors in packageRules" — config/validation.spec.ts line 1459
    #[test]
    fn validate_config_warns_for_selector_only_package_rules() {
        let result = validate_config_for_source(
            "repo",
            &json!({"packageRules": [{"matchDepTypes": ["foo"], "matchPackageNames": ["bar"]}]}),
        );
        assert!(result.errors.is_empty());
        assert_eq!(result.warnings.len(), 1);
    }

    // Ported: "errors if invalid combinations in packageRules" — config/validation.spec.ts line 1473
    #[test]
    fn validate_config_errors_for_invalid_package_rule_combinations() {
        let result = validate_config_for_source(
            "repo",
            &json!({"packageRules": [{"matchUpdateTypes": ["major"], "registryUrls": ["https://registry.npmjs.org"]}]}),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(result.errors.len(), 1);
    }

    // Ported: "warns when registryUrls is set at the top level of repo config" — config/validation.spec.ts line 1492
    #[test]
    fn validate_config_warns_for_top_level_registry_urls() {
        let result = validate_config_for_source(
            "repo",
            &json!({"registryUrls": ["https://registry.npmjs.org"]}),
        );
        assert!(result.errors.is_empty());
        assert_eq!(result.warnings.len(), 1);
        assert!(
            result.warnings[0]["message"]
                .as_str()
                .unwrap()
                .contains("Setting `registryUrls` at the top level")
        );
    }

    // Ported: "warns when defaultRegistryUrls is set at the top level of repo config" — config/validation.spec.ts line 1507
    #[test]
    fn validate_config_warns_for_top_level_default_registry_urls() {
        let result = validate_config_for_source(
            "repo",
            &json!({"defaultRegistryUrls": ["https://registry.npmjs.org"]}),
        );
        assert!(result.errors.is_empty());
        assert_eq!(result.warnings.len(), 1);
        assert!(
            result.warnings[0]["message"]
                .as_str()
                .unwrap()
                .contains("Setting `defaultRegistryUrls` at the top level")
        );
    }

    // Ported: "warns on nested group packageRules" — config/validation.spec.ts line 1522
    #[test]
    fn validate_config_warns_on_nested_group_package_rules() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "extends": ["group:fortawesome"],
                "packageRules": [{"automerge": true, "extends": ["group:fortawesome"]}]
            }),
        );
        assert!(result.errors.is_empty());
        assert_eq!(result.warnings.len(), 1);
    }

    // Ported: "does not error on use of `global:` presets in `globalExtends`" — config/validation.spec.ts line 1541
    #[test]
    fn validate_config_allows_global_presets_in_global_extends() {
        let result =
            validate_config_for_source("global", &json!({"globalExtends": ["global:safeEnv"]}));
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "does not error on use of `global:` presets in global `extends`" — config/validation.spec.ts line 1554
    #[test]
    fn validate_config_allows_global_presets_in_global_extends_field() {
        let result = validate_config_for_source("global", &json!({"extends": ["global:safeEnv"]}));
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "errors on use of `global:` presets in inherit `extends`" — config/validation.spec.ts line 1567
    #[test]
    fn validate_config_errors_for_global_presets_in_inherit_extends() {
        let result = validate_config_for_source("inherit", &json!({"extends": ["global:safeEnv"]}));
        assert_eq!(result.errors.len(), 1);
        assert!(result.warnings.is_empty());
    }

    // Ported: "errors on use of `global:` presets in repo `extends`" — config/validation.spec.ts line 1580
    #[test]
    fn validate_config_errors_for_global_presets_in_repo_extends() {
        let result = validate_config_for_source("repo", &json!({"extends": ["global:safeEnv"]}));
        assert_eq!(result.errors.len(), 1);
        assert!(result.warnings.is_empty());
    }

    // Ported: "warns if customEnvVariables are found in repo config" — config/validation.spec.ts line 1594
    #[test]
    fn validate_config_warns_for_custom_env_variables_in_repo_config() {
        let result = validate_config_for_source(
            "repo",
            &json!({"customEnvVariables": {"example1": "abc", "example2": "123"}}),
        );
        assert_eq!(result.warnings.len(), 1);
        assert!(
            result.warnings[0]["message"]
                .as_str()
                .unwrap()
                .contains("customEnvVariables")
        );
    }

    // Ported: "errors if schedule is cron and has no * minutes" — config/validation.spec.ts line 1613
    #[test]
    fn validate_config_errors_for_cron_schedule_without_wildcard_minutes() {
        let result = validate_config_for_source("repo", &json!({"schedule": ["30 5 * * *"]}));
        assert!(result.warnings.is_empty());
        assert_eq!(
            validation_error_messages(&result),
            vec![
                "Invalid schedule: `Invalid schedule: \"30 5 * * *\" has cron syntax, but doesn't have * as minutes`"
            ]
        );
    }

    // Ported: "errors if invalid matchHost values in hostRules" — config/validation.spec.ts line 1631
    #[test]
    fn validate_config_errors_for_invalid_host_rule_match_host_values() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "hostRules": [
                    {"matchHost": "://", "token": "token"},
                    {"matchHost": "", "token": "token"},
                    {"matchHost": null, "token": "token"},
                    {"hostType": "github", "token": "token"}
                ]
            }),
        );
        let messages = validation_error_messages(&result);
        assert_eq!(messages.len(), 3);
        assert!(
            messages.contains(&"Configuration option `hostRules[2].matchHost` should be a string")
        );
        assert!(
            messages
                .contains(&"Invalid value for hostRules matchHost. It cannot be an empty string.")
        );
        assert!(messages.contains(&"hostRules matchHost `://` is not a valid URL."));
    }

    // Ported: "errors if forbidden header in hostRules" — config/validation.spec.ts line 1673
    #[test]
    fn validate_config_errors_for_forbidden_host_rule_header() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "hostRules": [{
                    "matchHost": "https://domain.com/all-versions",
                    "headers": {"X-Auth-Token": "token", "unallowedHeader": "token"}
                }]
            }),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(
            validation_error_messages(&result),
            vec![
                "hostRules header `unallowedHeader` is not allowed by this bot's `allowedHeaders`."
            ]
        );
    }

    // Ported: "errors if headers values are not string" — config/validation.spec.ts line 1701
    #[test]
    fn validate_config_errors_for_non_string_host_rule_header_values() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "hostRules": [{
                    "matchHost": "https://domain.com/all-versions",
                    "headers": {"X-Auth-Token": 10}
                }]
            }),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(
            validation_error_messages(&result),
            vec!["Invalid hostRules headers value configuration: header must be a string."]
        );
    }

    // Ported: "catches invalid variable name in env config option" — config/validation.spec.ts line 1755
    #[test]
    fn validate_config_catches_invalid_env_variable_name_and_value() {
        let result = validate_config_for_source(
            "repo",
            &json!({"env": {"randomKey": "", "SOME_VAR": "some_value", "SOME_OTHER_VAR": 10}}),
        );
        let messages = validation_error_messages(&result);
        assert_eq!(messages.len(), 2);
        assert!(
            messages.contains(
                &"Env variable name `randomKey` is not allowed by this bot's `allowedEnv`."
            )
        );
        assert!(
            messages
                .contains(&"Invalid env variable value: `env.SOME_OTHER_VAR` must be a string.")
        );
        assert!(result.warnings.is_empty());
    }

    // Ported: "catches env config option if configured inside a parent" — config/validation.spec.ts line 1783
    #[test]
    fn validate_config_catches_nested_env_config() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "npm": {"env": {"SOME_VAR": "some_value"}},
                "packageRules": [{"matchManagers": ["regex"], "env": {"SOME_VAR": "some_value"}}]
            }),
        );
        assert_eq!(
            validation_error_messages(&result),
            vec![
                "The \"env\" object can only be configured at the top level of a config but was found inside \"npm\"",
                "The \"env\" object can only be configured at the top level of a config but was found inside \"packageRules[0]\"",
            ]
        );
        assert!(result.warnings.is_empty());
    }

    // Ported: "catches when * or ** is combined with others patterns in a regexOrGlob option" — config/validation.spec.ts line 1820
    #[test]
    fn validate_config_catches_match_all_combined_with_other_patterns() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "packageRules": [
                    {"matchRepositories": ["groupA/**", "groupB/**"], "enabled": false},
                    {"matchRepositories": ["*", "repo"], "enabled": true}
                ]
            }),
        );
        assert_eq!(result.errors.len(), 1);
        assert!(result.warnings.is_empty());
    }

    // Ported: "catches when negative number is used for integer type" — config/validation.spec.ts line 1848
    #[test]
    fn validate_config_catches_negative_integer_options() {
        let result = validate_config_for_source("repo", &json!({"azureWorkItemId": -2}));
        assert_eq!(
            validation_error_messages(&result),
            vec![
                "Configuration option `azureWorkItemId` should be a positive integer. Found negative value instead."
            ]
        );
    }

    // Ported: "validates prPriority" — config/validation.spec.ts line 1862
    #[test]
    fn validate_config_allows_negative_pr_priority() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "packageRules": [
                    {"matchDepNames": ["somedep"], "prPriority": -2},
                    {"matchDepNames": ["some-other-dep"], "prPriority": 2}
                ]
            }),
        );
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "errors if no bumpVersion filePattern is provided" — config/validation.spec.ts line 1883
    #[test]
    fn validate_config_errors_for_bump_version_without_file_patterns() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "bumpVersion": {
                    "matchStrings": ["^(?<depName>foo)(?<currentValue>bar)$"],
                    "bumpType": "patch"
                },
                "packageRules": [{
                    "matchPackageNames": ["foo"],
                    "bumpVersion": {
                        "matchStrings": ["^(?<depName>foo)(?<currentValue>bar)$"],
                        "bumpType": "patch"
                    }
                }]
            }),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(result.errors.len(), 2);
    }

    // Ported: "errors if no matchStrings are provided for bumpVersion" — config/validation.spec.ts line 1909
    #[test]
    fn validate_config_errors_for_bump_version_without_match_strings() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "bumpVersion": {"filePatterns": ["foo"]},
                "packageRules": [{
                    "matchPackageNames": ["foo"],
                    "bumpVersion": {"filePatterns": ["bar"]}
                }]
            }),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(result.errors.len(), 2);
    }

    // Ported: "allow bumpVersion" — config/validation.spec.ts line 1933
    #[test]
    fn validate_config_matches_upstream_bump_version_allow_case() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "bumpVersion": {"filePatterns": ["foo"]},
                "packageRules": [{
                    "matchPackageNames": ["foo"],
                    "bumpVersion": {"filePatterns": ["bar"]}
                }]
            }),
        );
        assert!(result.warnings.is_empty());
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

    fn validation_error_messages(result: &super::ValidationResult) -> Vec<&str> {
        result
            .errors
            .iter()
            .map(|error| error["message"].as_str().unwrap())
            .collect()
    }
}
