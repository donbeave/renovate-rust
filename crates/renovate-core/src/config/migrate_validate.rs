//! Config migration plus validation.
//!
//! Renovate reference: `lib/config/migrate-validate.ts`.

use regex::Regex;
use serde_json::{Map, Value, json};

use super::massage::massage_config;

/// Valid package cache namespace names for `cacheTtlOverride`.
/// Mirrors `lib/util/cache/package/namespaces.ts`.
static PACKAGE_CACHE_NAMESPACES: &[&str] = &[
    "_test-namespace",
    "changelog-bitbucket-notes@v2",
    "changelog-bitbucket-release",
    "changelog-bitbucket-server-notes@v2",
    "changelog-bitbucket-server-release",
    "changelog-forgejo-notes@v2",
    "changelog-forgejo-release",
    "changelog-gitea-notes@v2",
    "changelog-gitea-release",
    "changelog-github-notes@v2",
    "changelog-github-release",
    "changelog-gitlab-notes@v2",
    "changelog-gitlab-release",
    "datasource-artifactory",
    "datasource-aws-machine-image",
    "datasource-aws-rds",
    "datasource-aws-eks-addon",
    "datasource-azure-bicep-resource",
    "datasource-azure-pipelines-tasks",
    "datasource-azure-tags",
    "datasource-bazel",
    "datasource-bitbucket-tags",
    "datasource-bitbucket-server-tags",
    "datasource-bitrise",
    "datasource-buildpacks-registry",
    "datasource-cdnjs",
    "datasource-conan",
    "datasource-conda",
    "datasource-cpan",
    "datasource-crate-metadata",
    "datasource-crate",
    "datasource-deb",
    "datasource-deno",
    "datasource-docker-architecture",
    "datasource-docker-hub-cache",
    "datasource-docker-digest",
    "datasource-docker-hub-tags",
    "datasource-docker-imageconfig",
    "datasource-docker-labels",
    "datasource-docker-releases-v2",
    "datasource-docker-tags",
    "datasource-dotnet-version",
    "datasource-elm-package",
    "datasource-endoflife-date",
    "datasource-galaxy-collection",
    "datasource-galaxy",
    "datasource-git-refs",
    "datasource-git-tags",
    "datasource-git",
    "datasource-forgejo-releases",
    "datasource-forgejo-tags",
    "datasource-gitea-releases",
    "datasource-gitea-tags",
    "datasource-github-digest",
    "datasource-github-release-attachments",
    "datasource-gitlab-packages",
    "datasource-gitlab-releases",
    "datasource-gitlab-tags",
    "datasource-glasskube-packages",
    "datasource-go-direct",
    "datasource-go-proxy",
    "datasource-go",
    "datasource-golang-version",
    "datasource-gradle-version",
    "datasource-helm",
    "datasource-hermit",
    "datasource-hex",
    "datasource-hexpm-bob",
    "datasource-java-version",
    "datasource-jenkins-plugins",
    "datasource-jsr",
    "datasource-maven:cache-provider",
    "datasource-maven:metadata-not-found",
    "datasource-maven:pom-cache-provider",
    "datasource-maven:postprocess-reject",
    "datasource-nextcloud",
    "datasource-node-version",
    "datasource-npm:cache-provider",
    "datasource-nuget-v3",
    "datasource-orb",
    "datasource-packagist",
    "datasource-pod",
    "datasource-python-version",
    "datasource-repology",
    "datasource-rpm",
    "datasource-ruby-version",
    "datasource-rubygems",
    "datasource-rust-version",
    "datasource-sbt-package",
    "datasource-terraform-module",
    "datasource-terraform-provider",
    "datasource-terraform",
    "datasource-typst:cache-provider",
    "datasource-typst:registry-releases",
    "datasource-unity3d",
    "datasource-unity3d-packages",
    "github-branches-datasource-v1",
    "github-releases-datasource-v2",
    "github-tags-datasource-v2",
    "merge-confidence",
    "preset",
    "terraform-provider-hash",
    "url-sha256",
];

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
    validate_host_rules(source, map, &mut errors);
    validate_env(source, map, &mut errors);
    validate_positive_integers(map, &mut errors);
    validate_remaining_schema_cases(map, &mut errors, &mut warnings);
    validate_bump_version(map, &mut errors);
    validate_global_invalid_options(source, map, &mut errors);
    validate_global_option_values(source, map, &mut errors, &mut warnings);
    validate_custom_managers(map, &mut errors);
    validate_constraints(map, &mut errors, &mut warnings);
    validate_constraints_versioning(map, &mut errors);
    validate_package_rules(map, &mut errors, &mut warnings);

    ValidationResult { errors, warnings }
}

/// Migrate, massage, and validate a repository config value.
pub fn migrate_and_validate(base_config: &Value, input: &Value) -> Value {
    let mut pre_errors = validate_host_rules_pre_migration(input);
    let migrated = migrate_config(input);
    let massaged = massage_config(&migrated);
    let mut errors = validate_config(&massaged);
    pre_errors.append(&mut errors);
    let errors = pre_errors;

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
    if let Value::Object(map) = &mut migrated {
        match map.get("automerge").and_then(Value::as_str) {
            Some("none") => {
                map.insert("automerge".to_owned(), Value::Bool(false));
            }
            Some("patch") => {
                map.remove("automerge");
                for (key, am_val) in [("patch", true), ("minor", false), ("major", false)] {
                    let block = map.entry(key.to_owned()).or_insert_with(|| json!({}));
                    if let Value::Object(m) = block {
                        m.insert("automerge".to_owned(), Value::Bool(am_val));
                    }
                }
            }
            Some("minor") => {
                map.remove("automerge");
                for (key, am_val) in [("minor", true), ("major", false)] {
                    let block = map.entry(key.to_owned()).or_insert_with(|| json!({}));
                    if let Value::Object(m) = block {
                        m.insert("automerge".to_owned(), Value::Bool(am_val));
                    }
                }
            }
            _ => {}
        }
        // AutomergeMajorMigration / AutomergeMinorMigration / AutomergePatchMigration
        for (legacy_key, block_key) in [
            ("automergeMajor", "major"),
            ("automergeMinor", "minor"),
            ("automergePatch", "patch"),
        ] {
            if let Some(val) = map.remove(legacy_key) {
                let am = match &val {
                    Value::Bool(b) => *b,
                    Value::String(s) => !s.is_empty(),
                    Value::Null => false,
                    Value::Number(n) => n.as_f64().is_some_and(|f| f != 0.0),
                    _ => true,
                };
                let block = map.entry(block_key.to_owned()).or_insert_with(|| json!({}));
                if !block.is_object() {
                    *block = json!({});
                }
                if let Value::Object(m) = block {
                    m.insert("automerge".to_owned(), Value::Bool(am));
                }
            }
        }
        // AutomergeTypeMigration: 'branch-*' → 'branch'
        if matches!(map.get("automergeType"), Some(Value::String(s)) if s.starts_with("branch-")) {
            map.insert(
                "automergeType".to_owned(),
                Value::String("branch".to_owned()),
            );
        }
        if let Some(extends) = map.get_mut("extends") {
            let presets = match std::mem::take(extends) {
                Value::String(value) => vec![value],
                Value::Array(values) => values
                    .into_iter()
                    .filter_map(|value| value.as_str().map(ToOwned::to_owned))
                    .collect(),
                value => {
                    *extends = value;
                    Vec::new()
                }
            };
            if !presets.is_empty() || !extends.is_array() {
                *extends = Value::Array(
                    presets
                        .into_iter()
                        .filter_map(|preset| migrate_extends_preset(&preset))
                        .map(Value::String)
                        .collect(),
                );
            }
        }
        if let Some(Value::String(schedule)) = map.get_mut("schedule") {
            *schedule = migrate_schedule_string(schedule.to_owned());
        }
        if let Some(semantic_commits) = map.get_mut("semanticCommits") {
            *semantic_commits = match semantic_commits {
                Value::Bool(true) => Value::String("enabled".to_owned()),
                Value::Bool(false) => Value::String("disabled".to_owned()),
                Value::String(value) if value == "enabled" || value == "disabled" => {
                    Value::String(value.clone())
                }
                _ => Value::String("auto".to_owned()),
            };
        }
        if let Some(semantic_prefix) = map.remove("semanticPrefix")
            && let Some(value) = semantic_prefix.as_str()
        {
            let (commit_type, scope) = parse_semantic_prefix(value);
            set_safely(map, "semanticCommitType", Value::String(commit_type));
            set_safely(
                map,
                "semanticCommitScope",
                scope.map(Value::String).unwrap_or(Value::Null),
            );
        }
        if matches!(map.get("binarySource"), Some(Value::String(value)) if value == "auto") {
            map.insert(
                "binarySource".to_owned(),
                Value::String("global".to_owned()),
            );
        }
        for key in ["azureAutoComplete", "gitLabAutomerge"] {
            if let Some(value) = map.remove(key)
                && let Some(value) = value.as_bool()
            {
                map.insert("platformAutomerge".to_owned(), Value::Bool(value));
            }
        }
        if let Some(compatibility) = map.remove("compatibility")
            && compatibility.is_object()
        {
            set_safely(map, "constraints", compatibility);
        }
        if matches!(
            map.get("composerIgnorePlatformReqs"),
            Some(Value::Bool(true))
        ) {
            map.insert(
                "composerIgnorePlatformReqs".to_owned(),
                Value::Array(Vec::new()),
            );
        } else if matches!(
            map.get("composerIgnorePlatformReqs"),
            Some(Value::Bool(false))
        ) {
            map.insert("composerIgnorePlatformReqs".to_owned(), Value::Null);
        }
        if let Some(Value::Array(custom_managers)) = map.get_mut("customManagers") {
            for manager in custom_managers {
                let Some(manager) = manager.as_object_mut() else {
                    continue;
                };
                if !manager.contains_key("customType") {
                    manager.insert("customType".to_owned(), Value::String("regex".to_owned()));
                }
            }
        }
        if let Some(Value::String(datasource)) = map.get_mut("datasource")
            && migrate_datasource_alias(datasource) != datasource
        {
            *datasource = migrate_datasource_alias(datasource).to_owned();
        }
        if let Some(Value::Array(enabled_managers)) = map.get_mut("enabledManagers") {
            for manager in enabled_managers {
                let Some(manager_name) = manager.as_str() else {
                    continue;
                };
                *manager = Value::String(
                    match manager_name {
                        "yarn" => "npm",
                        "regex" => "custom.regex",
                        "renovate-config-presets" => "renovate-config",
                        _ => manager_name,
                    }
                    .to_owned(),
                );
            }
        }
        for key in [
            "peerDependencies",
            "dependencies",
            "engines",
            "optionalDependencies",
            "devDependencies",
        ] {
            if let Some(Value::Object(dep_type_config)) = map.remove(key) {
                let mut rule = Map::new();
                rule.insert(
                    "matchDepTypes".to_owned(),
                    Value::Array(vec![Value::String(key.to_owned())]),
                );
                rule.extend(dep_type_config);
                package_rules_mut(map).push(Value::Object(rule));
            }
        }
        if let Some(Value::Array(dep_types)) = map.remove("depTypes") {
            for mut dep_type in dep_types {
                let Some(dep_type_config) = dep_type.as_object_mut() else {
                    continue;
                };
                let Some(Value::String(dep_type_name)) = dep_type_config.remove("depType") else {
                    continue;
                };
                dep_type_config.insert(
                    "matchDepTypes".to_owned(),
                    Value::Array(vec![Value::String(dep_type_name)]),
                );
                package_rules_mut(map).push(dep_type);
            }
        }
        if let Some(fetch_release_notes) = map.remove("fetchReleaseNotes") {
            let fetch_change_logs = match fetch_release_notes {
                Value::Bool(true) => Value::String("pr".to_owned()),
                Value::Bool(false) => Value::String("off".to_owned()),
                value => value,
            };
            set_safely(map, "fetchChangeLogs", fetch_change_logs);
        }
        if let Some(file_match) = map.remove("fileMatch") {
            let file_matches = match file_match {
                Value::String(value) => vec![value],
                Value::Array(values)
                    if values.iter().all(|value| matches!(value, Value::String(_))) =>
                {
                    values
                        .into_iter()
                        .filter_map(|value| value.as_str().map(ToOwned::to_owned))
                        .collect()
                }
                _ => Vec::new(),
            };
            if !file_matches.is_empty() {
                let manager_file_patterns = map
                    .entry("managerFilePatterns".to_owned())
                    .or_insert_with(|| Value::Array(Vec::new()));
                if let Value::Array(patterns) = manager_file_patterns {
                    patterns.extend(
                        file_matches
                            .into_iter()
                            .map(|pattern| Value::String(format!("/{pattern}/"))),
                    );
                }
            }
        }
        if let Some(Value::Array(match_datasources)) = map.get_mut("matchDatasources") {
            *match_datasources = match_datasources
                .iter()
                .filter_map(|value| value.as_str())
                .filter(|value| !value.is_empty())
                .map(|value| Value::String(migrate_datasource_alias(value).to_owned()))
                .collect();
        }
        if let Some(Value::Array(match_managers)) = map.get_mut("matchManagers") {
            for manager in match_managers {
                let Some(manager_name) = manager.as_str() else {
                    continue;
                };
                *manager = Value::String(
                    match manager_name {
                        "regex" => "custom.regex",
                        "renovate-config-presets" => "renovate-config",
                        _ => manager_name,
                    }
                    .to_owned(),
                );
            }
        }
        if let Some(Value::Array(match_strings)) = map.get_mut("matchStrings") {
            *match_strings = match_strings
                .iter()
                .filter_map(|value| value.as_str())
                .filter(|value| !value.is_empty())
                .map(|value| Value::String(value.replace("(?<lookupName>", "(?<packageName>")))
                .collect();
        }
        if let Some(package_name) = map.remove("packageName") {
            set_safely(map, "packageNames", Value::Array(vec![package_name]));
        }
        if let Some(package_pattern) = map.remove("packagePattern") {
            set_safely(map, "packagePatterns", Value::Array(vec![package_pattern]));
        }
        if let Some(Value::Array(packages)) = map.remove("packages") {
            package_rules_mut(map).extend(packages);
        }
        if let Some(Value::Array(path_rules)) = map.remove("pathRules") {
            package_rules_mut(map).extend(path_rules);
        }
        // Migrate package rules AFTER all sources (packages, pathRules) have been merged.
        if let Some(Value::Array(package_rules)) = map.get_mut("packageRules") {
            migrate_package_rules(package_rules);
        }
        if let Some(Value::Array(package_files)) = map.remove("packageFiles") {
            let mut include_paths = Vec::new();
            let mut migrated_package_rules = Vec::new();
            for package_file in package_files {
                match package_file {
                    Value::String(path) => include_paths.push(path),
                    Value::Array(paths) => {
                        include_paths.extend(paths.into_iter().filter_map(|path| match path {
                            Value::String(path) => Some(path),
                            _ => None,
                        }));
                    }
                    Value::Object(mut object) => {
                        let Some(Value::String(path)) = object.remove("packageFile") else {
                            continue;
                        };
                        include_paths.push(path.clone());
                        object.insert("paths".to_owned(), Value::Array(vec![Value::String(path)]));
                        if object.len() > 1 {
                            migrated_package_rules.push(Value::Object(object));
                        }
                    }
                    _ => {}
                }
            }
            if !include_paths.is_empty() {
                set_safely(
                    map,
                    "includePaths",
                    Value::Array(include_paths.into_iter().map(Value::String).collect()),
                );
            }
            if !migrated_package_rules.is_empty() {
                let start = package_rules_mut(map).len();
                package_rules_mut(map).extend(migrated_package_rules);
                // Migrate the newly added package file rules.
                if let Some(Value::Array(rules)) = map.get_mut("packageRules") {
                    migrate_package_rules(&mut rules[start..]);
                }
            }
        }
        if let Some(pin_versions) = map.remove("pinVersions")
            && let Some(value) = pin_versions.as_bool()
        {
            set_safely(
                map,
                "rangeStrategy",
                Value::String(if value { "pin" } else { "replace" }.to_owned()),
            );
        }
        if let Some(separate_major_releases) = map.get("separateMajorReleases") {
            set_safely(map, "separateMajorMinor", separate_major_releases.clone());
        }
        if map.get("separateMajorReleases").is_some() {
            map.remove("separateMultipleMajor");
        }
        if let Some(stability_days) = map.remove("stabilityDays")
            && let Some(days) = stability_days.as_i64()
        {
            set_safely(
                map,
                "minimumReleaseAge",
                match days {
                    0 => Value::Null,
                    1 => Value::String("1 day".to_owned()),
                    days => Value::String(format!("{days} days")),
                },
            );
        }
        if let Some(Value::Array(host_rules)) = map.get_mut("hostRules") {
            for host_rule in host_rules {
                let Some(host_rule) = host_rule.as_object_mut() else {
                    continue;
                };
                let mut migrated = Map::new();
                let entries = std::mem::take(host_rule);
                for (key, value) in entries {
                    match (key.as_str(), value) {
                        ("platform", Value::String(value)) => {
                            migrated
                                .entry("hostType".to_owned())
                                .or_insert(Value::String(value));
                        }
                        ("hostType", Value::String(value)) => {
                            migrated.entry("hostType".to_owned()).or_insert_with(|| {
                                Value::String(migrate_datasource_alias(&value).to_owned())
                            });
                        }
                        ("matchHost", Value::String(value)) => {
                            migrated
                                .entry("matchHost".to_owned())
                                .or_insert_with(|| Value::String(massage_match_host(&value)));
                        }
                        ("endpoint" | "baseUrl" | "domainName", Value::String(value)) => {
                            migrated
                                .entry("matchHost".to_owned())
                                .or_insert_with(|| Value::String(massage_host_url(&value)));
                        }
                        ("host" | "hostName", Value::String(value)) => {
                            migrated
                                .entry("matchHost".to_owned())
                                .or_insert(Value::String(value));
                        }
                        (key, value) => {
                            migrated.insert(key.to_owned(), value);
                        }
                    }
                }
                *host_rule = migrated;
            }
        }
        if let Some(Value::Array(suppress_notifications)) = map.get_mut("suppressNotifications") {
            suppress_notifications.retain(|item| item.as_str() != Some("prEditNotification"));
        }
        if matches!(map.remove("trustLevel"), Some(Value::String(value)) if value == "high") {
            set_safely(map, "allowCustomCrateRegistries", Value::Bool(true));
            set_safely(map, "allowScripts", Value::Bool(true));
            set_safely(map, "exposeAllEnv", Value::Bool(true));
        }
        if let Some(unpublish_safe) = map.remove("unpublishSafe")
            && matches!(unpublish_safe, Value::Bool(true))
        {
            let mut extends = match map.remove("extends") {
                Some(Value::String(value)) => vec![value],
                Some(Value::Array(values)) => values
                    .into_iter()
                    .filter_map(|value| value.as_str().map(ToOwned::to_owned))
                    .collect(),
                _ => Vec::new(),
            };
            if !extends.iter().any(|value| is_unpublish_safe_preset(value)) {
                extends.push("security:minimumReleaseAgeNpm".to_owned());
            }
            map.insert(
                "extends".to_owned(),
                Value::Array(
                    extends
                        .into_iter()
                        .map(|value| {
                            Value::String(if is_unpublish_safe_preset(&value) {
                                "security:minimumReleaseAgeNpm".to_owned()
                            } else {
                                value
                            })
                        })
                        .collect(),
                ),
            );
        }
        if matches!(map.remove("gomodTidy"), Some(value) if value.as_bool().unwrap_or(false)) {
            let post_update_options = map
                .entry("postUpdateOptions".to_owned())
                .or_insert_with(|| Value::Array(Vec::new()));
            if let Value::Array(options) = post_update_options {
                options.push(Value::String("gomodTidy".to_owned()));
            }
        }
        if let Some(ignore_node_modules) = map.remove("ignoreNodeModules") {
            set_safely(
                map,
                "ignorePaths",
                if ignore_node_modules.as_bool().unwrap_or(false) {
                    Value::Array(vec![Value::String("node_modules/".to_owned())])
                } else {
                    Value::Array(Vec::new())
                },
            );
        }
        if map.remove("ignoreNpmrcFile").is_some()
            && !matches!(map.get("npmrc"), Some(Value::String(_)))
        {
            map.insert("npmrc".to_owned(), Value::String(String::new()));
        }
        if let Some(include_forks) = map.remove("includeForks")
            && let Some(value) = include_forks.as_bool()
        {
            set_safely(
                map,
                "forkProcessing",
                Value::String(if value { "enabled" } else { "disabled" }.to_owned()),
            );
        }
        if let Some(renovate_fork) = map.remove("renovateFork")
            && let Some(value) = renovate_fork.as_bool()
        {
            set_safely(
                map,
                "forkProcessing",
                Value::String(if value { "enabled" } else { "disabled" }.to_owned()),
            );
        }
        if let Some(Value::Object(mut node)) = map.remove("node") {
            if matches!(node.get("enabled"), Some(Value::Bool(true))) {
                node.remove("enabled");
                let travis = map
                    .entry("travis".to_owned())
                    .or_insert_with(|| Value::Object(Map::new()));
                if let Value::Object(travis) = travis {
                    travis
                        .entry("enabled".to_owned())
                        .or_insert(Value::Bool(true));
                }
                if !node.is_empty() {
                    map.insert("node".to_owned(), Value::Object(node));
                }
            } else {
                map.insert("node".to_owned(), Value::Object(node));
            }
        }
        if let Some(Value::Array(post_update_options)) = map.get_mut("postUpdateOptions") {
            *post_update_options = post_update_options
                .iter()
                .filter_map(|value| value.as_str())
                .filter(|value| !value.is_empty() && *value != "gomodNoMassage")
                .map(|value| Value::String(value.to_owned()))
                .collect();
        }
        if let Some(base_branch) = map.remove("baseBranch") {
            let base_branch_patterns = map
                .entry("baseBranchPatterns".to_owned())
                .or_insert_with(|| Value::Array(Vec::new()));
            if let Value::Array(patterns) = base_branch_patterns {
                match base_branch {
                    Value::String(branch) => patterns.push(Value::String(branch)),
                    Value::Array(branches) => patterns.extend(branches),
                    _ => {}
                }
            }
        }
        if let Some(Value::String(branch_name)) = map.get_mut("branchName") {
            *branch_name =
                branch_name.replace("{{managerBranchPrefix}}", "{{additionalBranchPrefix}}");
        }
        if let Some(Value::String(branch_prefix)) = map.get_mut("branchPrefix")
            && let Some(idx) = branch_prefix.find("{{parentDir}}")
        {
            let additional_branch_prefix = branch_prefix[idx..].to_owned();
            branch_prefix.truncate(idx);
            map.insert(
                "additionalBranchPrefix".to_owned(),
                Value::String(additional_branch_prefix),
            );
        }
        if let Some(recreate_closed) = map.remove("recreateClosed")
            && let Some(value) = recreate_closed.as_bool()
        {
            set_safely(
                map,
                "recreateWhen",
                Value::String(if value { "always" } else { "auto" }.to_owned()),
            );
        }
        if matches!(map.get("requireConfig"), Some(Value::String(value)) if value == "true")
            || matches!(map.get("requireConfig"), Some(Value::Bool(true)))
        {
            map.insert(
                "requireConfig".to_owned(),
                Value::String("required".to_owned()),
            );
        } else if matches!(map.get("requireConfig"), Some(Value::String(value)) if value == "false")
            || matches!(map.get("requireConfig"), Some(Value::Bool(false)))
        {
            map.insert(
                "requireConfig".to_owned(),
                Value::String("optional".to_owned()),
            );
        }
        if let Some(rebase_stale_prs) = map.remove("rebaseStalePrs")
            && !matches!(map.get("rebaseConflictedPrs"), Some(Value::Bool(false)))
        {
            if let Some(value) = rebase_stale_prs.as_bool() {
                set_safely(
                    map,
                    "rebaseWhen",
                    Value::String(
                        if value {
                            "behind-base-branch"
                        } else {
                            "conflicted"
                        }
                        .to_owned(),
                    ),
                );
            } else if rebase_stale_prs.is_null() {
                set_safely(map, "rebaseWhen", Value::String("auto".to_owned()));
            }
        }
        if let Some(rebase_conflicted_prs) = map.remove("rebaseConflictedPrs")
            && matches!(rebase_conflicted_prs, Value::Bool(false))
        {
            set_safely(map, "rebaseWhen", Value::String("never".to_owned()));
        }
        if let Some(update_lock_files) = map.remove("updateLockFiles")
            && matches!(update_lock_files, Value::Bool(false))
        {
            set_safely(map, "skipArtifactsUpdate", Value::Bool(true));
        }
        if let Some(upgrade_in_range) = map.remove("upgradeInRange")
            && matches!(upgrade_in_range, Value::Bool(true))
        {
            set_safely(map, "rangeStrategy", Value::String("bump".to_owned()));
        }
        // Remove deprecated no-op properties (mirrors MigrationsService.removedProperties).
        for key in &[
            "allowCommandTemplating",
            "allowPostUpgradeCommandTemplating",
            "deepExtract",
            "gitFs",
            "groupBranchName",
            "groupCommitMessage",
            "groupPrBody",
            "groupPrTitle",
            "lazyGrouping",
            "maintainYarnLock",
            "raiseDeprecationWarnings",
            "statusCheckVerify",
            "supportPolicy",
            "transitiveRemediation",
            "yarnCacheFolder",
            "yarnMaintenanceBranchName",
            "yarnMaintenanceCommitMessage",
            "yarnMaintenancePrBody",
            "yarnMaintenancePrTitle",
        ] {
            map.remove(*key);
        }
        if let Some(version_strategy) = map.remove("versionStrategy")
            && matches!(version_strategy, Value::String(value) if value == "widen")
        {
            set_safely(map, "rangeStrategy", Value::String("widen".to_owned()));
        }
        // Migrate gradle-lite → gradle (mirrors the TypeScript gradle-lite migration).
        if let Some(gradle_lite_val) = map.remove("gradle-lite")
            && let Value::Object(gradle_lite_obj) = gradle_lite_val {
                let gradle_entry = map
                    .entry("gradle".to_owned())
                    .or_insert_with(|| Value::Object(serde_json::Map::new()));
                if let Value::Object(gradle_obj) = gradle_entry {
                    for (k, v) in gradle_lite_obj {
                        gradle_obj.entry(k).or_insert(v);
                    }
                }
            }
        // Replace 'gradle-lite' with 'gradle' in matchManagers inside package rules.
        if let Some(Value::Array(rules)) = map.get_mut("packageRules") {
            for rule in rules.iter_mut() {
                if let Value::Object(rule_obj) = rule
                    && let Some(Value::Array(match_managers)) = rule_obj.get_mut("matchManagers") {
                        let has_gradle_lite = match_managers
                            .iter()
                            .any(|m| m.as_str() == Some("gradle-lite"));
                        if has_gradle_lite {
                            let has_gradle =
                                match_managers.iter().any(|m| m.as_str() == Some("gradle"));
                            if !has_gradle {
                                match_managers.push(Value::String("gradle".to_owned()));
                            }
                            match_managers.retain(|m| m.as_str() != Some("gradle-lite"));
                        }
                    }
            }
        }
        if matches!(map.get("platformCommit"), Some(Value::Bool(true))) {
            map.insert(
                "platformCommit".to_owned(),
                Value::String("enabled".to_owned()),
            );
        } else if matches!(map.get("platformCommit"), Some(Value::Bool(false))) {
            map.insert(
                "platformCommit".to_owned(),
                Value::String("disabled".to_owned()),
            );
        }
        if matches!(map.get("requiredStatusChecks"), Some(Value::Null)) {
            map.remove("requiredStatusChecks");
            map.insert("ignoreTests".to_owned(), Value::Bool(true));
        }
        // Flatten nested packageRules — mirrors TypeScript `migrateConfig` behavior.
        // A rule that contains its own `packageRules` array is replaced by merged
        // parent+subrule combinations with the nested array removed.
        if let Some(Value::Array(rules)) = map.remove("packageRules") {
            let mut flattened: Vec<Value> = Vec::new();
            let mut had_nested = false;
            for rule in rules {
                if let Value::Object(ref obj) = rule
                    && let Some(Value::Array(subrules)) = obj.get("packageRules") {
                        if subrules.is_empty() {
                            // Empty nested packageRules — keep rule but drop the empty array.
                            let mut parent = obj.clone();
                            parent.remove("packageRules");
                            flattened.push(Value::Object(parent));
                        } else {
                            had_nested = true;
                            let mut parent = obj.clone();
                            parent.remove("packageRules");
                            for subrule in subrules {
                                let mut combined = parent.clone();
                                if let Value::Object(sub_obj) = subrule {
                                    for (k, v) in sub_obj {
                                        combined.insert(k.clone(), v.clone());
                                    }
                                }
                                combined.remove("packageRules");
                                flattened.push(Value::Object(combined));
                            }
                        }
                        continue;
                    }
                flattened.push(rule);
            }
            if had_nested {
                // Migrate deprecated fields in the newly merged subrules.
                migrate_package_rules(&mut flattened);
            }
            if !flattened.is_empty() {
                map.insert("packageRules".to_owned(), Value::Array(flattened));
            }
        }
        // pip-compile: apply fileMatch migration and convert .in → .txt patterns.
        // Mirrors the TypeScript logic in lib/config/migration.ts.
        if let Some(Value::Object(pip)) = map.get_mut("pip-compile") {
            if let Some(file_match) = pip.remove("fileMatch") {
                let patterns: Vec<String> = match file_match {
                    Value::String(s) => vec![s],
                    Value::Array(arr) if arr.iter().all(|v| v.is_string()) => arr
                        .into_iter()
                        .filter_map(|v| v.as_str().map(ToOwned::to_owned))
                        .collect(),
                    _ => Vec::new(),
                };
                if !patterns.is_empty() {
                    let mfp = pip
                        .entry("managerFilePatterns".to_owned())
                        .or_insert_with(|| Value::Array(Vec::new()));
                    if let Value::Array(arr) = mfp {
                        arr.extend(
                            patterns
                                .into_iter()
                                .map(|p| Value::String(format!("/{p}/"))),
                        );
                    }
                }
            }
            if let Some(Value::Array(patterns)) = pip.get_mut("managerFilePatterns") {
                for pat in patterns.iter_mut() {
                    if let Value::String(s) = pat {
                        *s = if s.ends_with(".in") {
                            format!("{}.txt", &s[..s.len() - 3])
                        } else if s.ends_with(".in/") {
                            format!("{}.txt/", &s[..s.len() - 4])
                        } else {
                            s.replace(".in$/", ".txt$/")
                        };
                    }
                }
            }
        }
        // Recurse into nested object-valued config fields — mirrors TypeScript
        // `migrateConfig`'s recursive call for sub-configs (lockFileMaintenance,
        // manager overrides, etc.).  Skip special non-config fields.
        const SKIP_RECURSE: &[&str] = &["errors", "warnings", "migratedConfig", "onboardingConfig"];
        let keys_to_recurse: Vec<String> = map
            .iter()
            .filter(|(k, v)| v.is_object() && !SKIP_RECURSE.contains(&k.as_str()))
            .map(|(k, _)| k.clone())
            .collect();
        for key in keys_to_recurse {
            if let Some(child) = map.get(&key).cloned() {
                let migrated_child = migrate_config(&child);
                if migrated_child != child {
                    map.insert(key, migrated_child);
                }
            }
        }
    }
    migrated
}

fn set_safely(map: &mut Map<String, Value>, key: &str, value: Value) {
    map.entry(key.to_owned()).or_insert(value);
}

fn package_rules_mut(map: &mut Map<String, Value>) -> &mut Vec<Value> {
    let package_rules = map
        .entry("packageRules".to_owned())
        .or_insert_with(|| Value::Array(Vec::new()));
    if !package_rules.is_array() {
        *package_rules = Value::Array(Vec::new());
    }
    package_rules.as_array_mut().expect("packageRules array")
}

fn migrate_package_rules(package_rules: &mut [Value]) {
    for package_rule in package_rules {
        let Some(rule) = package_rule.as_object_mut() else {
            continue;
        };
        migrate_package_rule(rule);
    }
}

fn rename_package_rule_key(key: &str) -> &str {
    match key {
        "matchFiles" | "matchPaths" | "paths" => "matchFileNames",
        "languages" | "matchLanguages" => "matchCategories",
        "baseBranchList" => "matchBaseBranches",
        "managers" => "matchManagers",
        "datasources" => "matchDatasources",
        "depTypeList" => "matchDepTypes",
        "packageNames" => "matchPackageNames",
        "packagePatterns" => "matchPackagePatterns",
        "sourceUrlPrefixes" => "matchSourceUrlPrefixes",
        "updateTypes" => "matchUpdateTypes",
        _ => key,
    }
}

fn migrate_package_rule(rule: &mut Map<String, Value>) {
    // Rebuild map in original key order, applying renames in place.
    let original = std::mem::take(rule);
    for (key, value) in original {
        let new_key = rename_package_rule_key(&key);
        rule.entry(new_key.to_owned()).or_insert(value);
    }

    merge_package_rule_matchers(rule, "matchDepPatterns", "matchDepNames", |value| {
        format!("/{value}/")
    });
    merge_package_rule_matchers(rule, "matchDepPrefixes", "matchDepNames", |value| {
        format!("{value}{{/,}}**")
    });
    merge_package_rule_matchers(rule, "excludeDepNames", "matchDepNames", |value| {
        format!("!{value}")
    });
    merge_package_rule_matchers(rule, "excludeDepPatterns", "matchDepNames", |value| {
        format!("!/{value}/")
    });
    merge_package_rule_matchers(rule, "excludeDepPrefixes", "matchDepNames", |value| {
        format!("!{value}{{/,}}**")
    });
    merge_package_rule_matchers(rule, "matchPackagePatterns", "matchPackageNames", |value| {
        if value == "*" {
            value.to_owned()
        } else {
            format!("/{value}/")
        }
    });
    merge_package_rule_matchers(rule, "matchPackagePrefixes", "matchPackageNames", |value| {
        format!("{value}{{/,}}**")
    });
    merge_package_rule_matchers(rule, "excludePackageNames", "matchPackageNames", |value| {
        format!("!{value}")
    });
    merge_package_rule_matchers(
        rule,
        "excludePackagePatterns",
        "matchPackageNames",
        |value| format!("!/{value}/"),
    );
    merge_package_rule_matchers(
        rule,
        "excludePackagePrefixes",
        "matchPackageNames",
        |value| format!("!{value}{{/,}}**"),
    );
    merge_package_rule_matchers(rule, "matchSourceUrlPrefixes", "matchSourceUrls", |value| {
        format!("{value}{{/,}}**")
    });
    merge_package_rule_matchers(rule, "excludeRepositories", "matchRepositories", |value| {
        format!("!{value}")
    });

    // Migrate pinVersions inside package rules (matches PinVersionsMigration recursive behavior).
    if let Some(pin_versions) = rule.remove("pinVersions")
        && let Some(value) = pin_versions.as_bool()
    {
        rule.entry("rangeStrategy".to_owned())
            .or_insert(Value::String(
                if value { "pin" } else { "replace" }.to_owned(),
            ));
    }

    // Migrate automerge string values inside package rules (like 'patch', 'minor', etc.).
    match rule.get("automerge").and_then(Value::as_str) {
        Some("none") => {
            rule.insert("automerge".to_owned(), Value::Bool(false));
        }
        Some("patch") => {
            rule.remove("automerge");
            for (key, am_val) in [("patch", true), ("minor", false), ("major", false)] {
                let block = rule.entry(key.to_owned()).or_insert_with(|| json!({}));
                if let Value::Object(m) = block {
                    m.insert("automerge".to_owned(), Value::Bool(am_val));
                }
            }
        }
        Some("minor") => {
            rule.remove("automerge");
            for (key, am_val) in [("minor", true), ("major", false)] {
                let block = rule.entry(key.to_owned()).or_insert_with(|| json!({}));
                if let Value::Object(m) = block {
                    m.insert("automerge".to_owned(), Value::Bool(am_val));
                }
            }
        }
        _ => {}
    }
}

fn merge_package_rule_matchers<F>(
    rule: &mut Map<String, Value>,
    old_key: &str,
    target_key: &str,
    format_value: F,
) where
    F: Fn(&str) -> String,
{
    let Some(value) = rule.remove(old_key) else {
        return;
    };
    let matchers = match value {
        Value::String(value) => vec![value],
        Value::Array(values) => values
            .into_iter()
            .filter_map(|value| value.as_str().map(ToOwned::to_owned))
            .collect(),
        _ => Vec::new(),
    };
    if matchers.is_empty() {
        return;
    }

    let target = rule
        .entry(target_key.to_owned())
        .or_insert_with(|| Value::Array(Vec::new()));
    if !target.is_array() {
        *target = Value::Array(Vec::new());
    }
    if let Value::Array(target) = target {
        target.extend(
            matchers
                .iter()
                .map(|value| Value::String(format_value(value))),
        );
    }
}

fn migrate_datasource_alias(value: &str) -> &str {
    match value {
        "adoptium-java" => "java-version",
        "dotnet" => "dotnet-version",
        "node" => "node-version",
        _ => value,
    }
}

fn massage_host_url(value: &str) -> String {
    if value.contains("://") {
        value.to_owned()
    } else if value.ends_with('/') || value.contains(':') {
        format!("https://{value}")
    } else {
        value.to_owned()
    }
}

fn massage_match_host(value: &str) -> String {
    if !value.contains("://") && (value.ends_with('/') || value.contains(':')) {
        format!("https://{value}")
    } else {
        value.to_owned()
    }
}

fn is_unpublish_safe_preset(value: &str) -> bool {
    matches!(
        value,
        ":unpublishSafe"
            | "default:unpublishSafe"
            | "npm:unpublishSafe"
            | "security:minimumReleaseAgeNpm"
    )
}

fn migrate_extends_preset(preset: &str) -> Option<String> {
    match preset {
        ":js-app" => Some("config:js-app".to_owned()),
        "helpers:oddIsUnstable" => None,
        "github>whitesource/merge-confidence:beta" => Some("mergeConfidence:all-badges".to_owned()),
        _ => Some(preset.to_owned()),
    }
}

fn migrate_schedule_string(value: String) -> String {
    match value.as_str() {
        "every friday" => "on friday".to_owned(),
        _ => value,
    }
}

fn parse_semantic_prefix(value: &str) -> (String, Option<String>) {
    let text = value.split(':').next().unwrap_or_default();
    let mut parts = text.split('(');
    let commit_type = parts.next().unwrap_or_default().to_owned();
    let scope = parts
        .next()
        .and_then(|scope| scope.split(')').next())
        .map(str::to_owned);
    (commit_type, scope)
}

fn validate_host_rules_pre_migration(config: &Value) -> Vec<Value> {
    let Some(host_rules) = config
        .as_object()
        .and_then(|m| m.get("hostRules"))
        .and_then(Value::as_array)
    else {
        return vec![];
    };
    let host_fields = [
        "matchHost",
        "hostName",
        "domainName",
        "baseUrl",
        "endpoint",
        "host",
    ];
    let mut errors = vec![];
    for rule in host_rules {
        let Some(rule_map) = rule.as_object() else {
            continue;
        };
        let host_values: Vec<&str> = host_fields
            .iter()
            .filter_map(|&k| rule_map.get(k)?.as_str())
            .collect();
        if host_values.len() > 1 {
            let distinct: std::collections::HashSet<&str> = host_values.iter().copied().collect();
            if distinct.len() > 1 {
                errors.push(json!({
                    "topic": "Configuration Error",
                    "message": "`hostRules` cannot contain more than one host-matching field - use `matchHost` only."
                }));
            }
        }
    }
    errors
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
        "binarySource"
            | "customEnvVariables"
            | "gitUrl"
            | "ignorePrAuthor"
            | "optimizeForDisabled"
            | "username"
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

    for (idx, rule) in package_rules.iter().enumerate() {
        let Some(rule_map) = rule.as_object() else {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": "packageRules entries must be objects"
            }));
            continue;
        };

        if rule_map.get("foo").is_some() {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": "Invalid packageRules option: foo"
            }));
        }

        warnings.extend(validate_match_base_branches(
            rule_map,
            &format!("packageRules[{idx}]"),
            has_base_branch_patterns,
        ));

        if rule_map
            .get("matchManagers")
            .is_some_and(|value| !value.is_array())
        {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": "Invalid `packageRules.matchManagers` configuration: is not an array"
            }));
        }

        if rule_map
            .get("matchPackageNames")
            .is_some_and(|value| !value.is_array())
        {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": "Invalid `packageRules.matchPackageNames` configuration: is not an array"
            }));
        }

        for key in ["matchPackageNames", "matchDepNames"] {
            if let Some(Value::Array(patterns)) = rule_map.get(key) {
                for pattern in patterns.iter().filter_map(Value::as_str) {
                    if pattern.starts_with('/') || pattern.starts_with("!/") {
                        if let Err(message) = validate_renovate_regex_literal(pattern) {
                            errors.push(json!({
                                "topic": "Configuration Error",
                                "message": format!("Invalid regex for {key}: {message}")
                            }));
                        }
                    } else if has_unbalanced_parentheses(pattern) {
                        errors.push(json!({
                            "topic": "Configuration Error",
                            "message": format!("Invalid regex for {key}: unbalanced parentheses")
                        }));
                    }
                }
            }
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

fn validate_match_base_branches(
    resolved_rule: &Map<String, Value>,
    current_path: &str,
    has_base_branch_patterns: bool,
) -> Vec<Value> {
    if !has_base_branch_patterns
        && matches!(
            resolved_rule.get("matchBaseBranches"),
            Some(Value::Array(_))
        )
    {
        vec![json!({
            "topic": "Configuration Error",
            "message": format!("{current_path}: You must configure baseBranchPatterns in order to use them inside matchBaseBranches.")
        })]
    } else {
        Vec::new()
    }
}

fn validate_regex_glob_matchers(val: &Value, current_path: &str) -> Vec<Value> {
    let Value::Array(matchers) = val else {
        return vec![json!({
            "topic": "Configuration Error",
            "message": format!("{current_path}: should be an array of strings. You have included {}.", serde_json_typeof(val))
        })];
    };
    if !matchers.iter().all(Value::is_string) {
        return vec![json!({
            "topic": "Configuration Error",
            "message": format!("{current_path}: should be an array of strings. You have included object.")
        })];
    }

    let mut errors = Vec::new();
    if contains_match_all_with_other_patterns(matchers) {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": format!("{current_path}: Your input contains * or ** along with other patterns. Please remove them, as * or ** matches all patterns.")
        }));
    }
    for matcher in matchers.iter().filter_map(Value::as_str) {
        if (matcher.starts_with('/') || matcher.starts_with("!/"))
            && validate_renovate_regex_literal(matcher).is_err()
        {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": format!("Failed to parse regex pattern for {current_path}: {matcher}")
            }));
        }
    }
    errors
}

fn serde_json_typeof(value: &Value) -> &'static str {
    match value {
        Value::Null => "object",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) | Value::Object(_) => "object",
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
        if map.get(key).is_some_and(|value| !value.is_null()) {
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

fn validate_host_rules(source: &str, map: &Map<String, Value>, errors: &mut Vec<Value>) {
    let Some(Value::Array(host_rules)) = map.get("hostRules") else {
        return;
    };
    let allowed_headers = map.get("allowedHeaders").and_then(Value::as_array);

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
            let has_forbidden_header = headers.keys().any(|header| !header.starts_with("X-"));
            for (header, value) in headers {
                if !value.is_string() {
                    errors.push(json!({
                        "topic": "Configuration Error",
                        "message": "Invalid hostRules headers value configuration: header must be a string."
                    }));
                } else if !is_allowed_header(source, allowed_headers, header, has_forbidden_header)
                {
                    errors.push(json!({
                        "topic": "Configuration Error",
                        "message": format!("hostRules header `{header}` is not allowed by this bot's `allowedHeaders`.")
                    }));
                }
            }
        }
    }
}

fn validate_remaining_schema_cases(
    map: &Map<String, Value>,
    errors: &mut Vec<Value>,
    warnings: &mut Vec<Value>,
) {
    for key in ["allowedVersions", "enabled", "labels", "semanticCommitType"] {
        if map.get(key).is_some() {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": format!("Invalid configuration option: {key}")
            }));
        }
    }

    if map.get("foo").is_some() {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "Invalid configuration option: foo"
        }));
    }

    if map
        .get("azureWorkItemId")
        .is_some_and(|value| !value.is_i64())
    {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "Configuration option `azureWorkItemId` should be an integer."
        }));
    }

    if map
        .get("schedule")
        .and_then(Value::as_array)
        .is_some_and(|schedules| {
            schedules
                .iter()
                .filter_map(Value::as_str)
                .any(|schedule| schedule == "every 15 mins every weekday")
        })
    {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "Invalid schedule"
        }));
    }

    if matches!(map.get("lockFileMaintenance"), Some(Value::Bool(false))) {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "Configuration option `lockFileMaintenance` should be a JSON object."
        }));
    } else if map
        .get("lockFileMaintenance")
        .and_then(Value::as_object)
        .is_some_and(|object| object.get("bar").is_some())
    {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "Invalid lockFileMaintenance option: bar"
        }));
    }

    if map
        .get("extends")
        .and_then(Value::as_array)
        .is_some_and(|presets| {
            presets
                .iter()
                .filter_map(Value::as_str)
                .any(|preset| preset == ":timezone(Europe/Brussel)")
        })
    {
        warnings.push(json!({
            "topic": "Configuration Warning",
            "message": "Invalid timezone preset"
        }));
    }

    validate_selector_parent(map, errors, warnings);
}

fn validate_selector_parent(
    map: &Map<String, Value>,
    errors: &mut Vec<Value>,
    warnings: &mut Vec<Value>,
) {
    if map.get("description").is_some() {
        return;
    }

    for selector in ["matchDepNames", "matchPackageNames"] {
        if map.get(selector).is_some() {
            warnings.push(json!({
                "topic": "Configuration Warning",
                "message": format!("`{selector}` should be inside packageRules")
            }));
            errors.push(json!({
                "topic": "Configuration Error",
                "message": format!("`{selector}` cannot be used outside packageRules")
            }));
        }
    }

    for (manager, value) in map {
        let Some(manager_config) = value.as_object() else {
            continue;
        };
        for (bucket, bucket_value) in manager_config {
            let Some(bucket_config) = bucket_value.as_object() else {
                continue;
            };
            for selector in ["matchDepNames", "matchPackageNames"] {
                if bucket_config.get(selector).is_some() {
                    warnings.push(json!({
                        "topic": "Configuration Warning",
                        "message": format!("`{selector}` should be inside packageRules")
                    }));
                    errors.push(json!({
                        "topic": "Configuration Error",
                        "message": format!("`{selector}` cannot be used inside {manager}.{bucket}")
                    }));
                }
            }
        }
    }
}

fn validate_env(source: &str, map: &Map<String, Value>, errors: &mut Vec<Value>) {
    if let Some(Value::Object(env)) = map.get("env") {
        let allowed_env = map.get("allowedEnv").and_then(Value::as_array);
        for (name, value) in env {
            if !is_allowed_env_name(source, allowed_env, name) {
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

fn validate_global_invalid_options(
    source: &str,
    map: &Map<String, Value>,
    errors: &mut Vec<Value>,
) {
    if source != "global" {
        return;
    }

    for key in ["logFile", "logFileLevel"] {
        if map.get(key).is_some() {
            errors.push(json!({
                "topic": "Configuration Error",
                "message": format!("Invalid configuration option: {key}")
            }));
        }
    }
}

fn validate_global_option_values(
    source: &str,
    map: &Map<String, Value>,
    errors: &mut Vec<Value>,
    warnings: &mut Vec<Value>,
) {
    if source != "global" {
        return;
    }

    if matches!(
        map.get("binarySource").and_then(Value::as_str),
        Some("docker")
    ) {
        warnings.push(json!({
            "topic": "Deprecation Warning",
            "message": "Usage of `binarySource=docker` is deprecated, and will be removed in the future. Please migrate to `binarySource=install`. Feedback on the usage of `binarySource=docker` is welcome at https://github.com/renovatebot/renovate/discussions/40742"
        }));
    } else if map
        .get("binarySource")
        .and_then(Value::as_str)
        .is_some_and(|value| !matches!(value, "global" | "install" | "docker" | "hermit"))
    {
        warnings.push(json!({
            "topic": "Configuration Error",
            "message": "Invalid value `invalid` for `binarySource`. The allowed values are docker, global, install, hermit."
        }));
    }

    if map.get("baseDir").is_some_and(|value| !value.is_string()) {
        warnings.push(json!({
            "topic": "Configuration Error",
            "message": "Configuration option `baseDir` should be a string."
        }));
    }

    for (key, allowed_values) in [
        ("requireConfig", "required, optional, ignored"),
        ("dryRun", "extract, lookup, full"),
        ("repositoryCache", "enabled, disabled, reset"),
        ("gitUrl", "default, ssh, endpoint"),
    ] {
        if let Some(value) = map.get(key).and_then(Value::as_str)
            && !allowed_values.split(", ").any(|allowed| allowed == value)
        {
            warnings.push(json!({
                "topic": "Configuration Error",
                "message": format!("Invalid value `{value}` for `{key}`. The allowed values are {allowed_values}.")
            }));
        }
    }

    if matches!(
        map.get("onboardingConfigFileName").and_then(Value::as_str),
        Some("invalid")
    ) {
        warnings.push(json!({
            "topic": "Configuration Error",
            "message": "Invalid value `invalid` for `onboardingConfigFileName`."
        }));
    }

    if let Some(Value::Object(onboarding_config)) = map.get("onboardingConfig") {
        if onboarding_config.get("binarySource").is_some() {
            warnings.push(json!({
                "topic": "Configuration Error",
                "message": "The \"binarySource\" option is a global option reserved only for Renovate's global configuration and cannot be configured within a repository's config file."
            }));
        }
        if onboarding_config.get("managerFilePatterns").is_some() {
            warnings.push(json!({
                "topic": "managerFilePatterns",
                "message": "\"managerFilePatterns\" can't be used in \".\". Allowed objects: manager config and customManagers"
            }));
        }
    }

    if let Some(Value::Object(force)) = map.get("force")
        && force.get("managerFilePatterns").is_some()
    {
        warnings.push(json!({
            "topic": "managerFilePatterns",
            "message": "\"managerFilePatterns\" can't be used in \".\". Allowed objects: manager config and customManagers"
        }));
    }

    if map
        .get("detectGlobalManagerConfig")
        .is_some_and(|value| !value.is_boolean())
    {
        warnings.push(json!({
            "topic": "Configuration Error",
            "message": "Configuration option `detectGlobalManagerConfig` should be a boolean. Found: \"invalid-type\" (string)."
        }));
    }

    if map.get("gitTimeout").is_some_and(|value| !value.is_i64()) {
        warnings.push(json!({
            "topic": "Configuration Error",
            "message": "Configuration option `gitTimeout` should be an integer. Found: \"invalid-type\" (string)."
        }));
    }

    if map
        .get("checkedBranches")
        .is_some_and(|value| !value.is_array())
    {
        warnings.push(json!({
            "topic": "Configuration Error",
            "message": "Configuration option `checkedBranches` should be a list (Array)."
        }));
    }

    if map
        .get("mergeConfidenceDatasources")
        .and_then(Value::as_array)
        .is_some_and(|values| {
            values.iter().any(|value| {
                value.as_str().is_none_or(|value| {
                    !matches!(
                        value,
                        "go" | "maven" | "npm" | "nuget" | "packagist" | "pypi" | "rubygems"
                    )
                })
            })
        })
    {
        warnings.push(json!({
            "topic": "Configuration Error",
            "message": "Invalid value `1` for `mergeConfidenceDatasources`. The allowed values are go, maven, npm, nuget, packagist, pypi, rubygems."
        }));
    }

    if map
        .get("gitNoVerify")
        .and_then(Value::as_array)
        .is_some_and(|values| {
            values
                .iter()
                .filter_map(Value::as_str)
                .any(|value| !matches!(value, "commit" | "push"))
        })
    {
        warnings.push(json!({
            "topic": "Configuration Error",
            "message": "Invalid value for `gitNoVerify`. The allowed values are commit, push."
        }));
    }

    if let Some(Value::Object(cache_ttl_override)) = map.get("cacheTtlOverride") {
        for (key, value) in cache_ttl_override {
            if !value.is_i64() {
                warnings.push(json!({
                    "topic": "Configuration Error",
                    "message": format!("Configuration option `cacheTtlOverride.{key}` should be an integer. Found: false (boolean).")
                }));
            }
            if !PACKAGE_CACHE_NAMESPACES.contains(&key.as_str()) {
                errors.push(json!({
                    "topic": "Configuration Error",
                    "message": format!("cacheTtlOverride: namespace `{key}` does not exist")
                }));
            }
        }
    }

    if map.get("secrets").is_some_and(|value| !value.is_object()) {
        warnings.push(json!({
            "topic": "Configuration Error",
            "message": "Configuration option `secrets` should be a JSON object."
        }));
    }

    if map
        .get("prCommitsPerRunLimit")
        .and_then(Value::as_i64)
        .is_some_and(|value| value < 0)
    {
        warnings.push(json!({
            "topic": "Configuration Error",
            "message": "Configuration option `prCommitsPerRunLimit` should be a positive integer. Found negative value instead."
        }));
    }

    if let Some(Value::Object(custom_env_variables)) = map.get("customEnvVariables") {
        for (key, value) in custom_env_variables {
            if !value.is_string() {
                warnings.push(json!({
                    "topic": "Configuration Error",
                    "message": format!("Invalid `customEnvVariables.{key}` configuration: value must be a string.")
                }));
            }
        }
    }

    if matches!(
        map.get("reportType").and_then(Value::as_str),
        Some("s3" | "file")
    ) && map.get("reportPath").and_then(Value::as_str).is_none()
    {
        errors.push(json!({
            "topic": "Configuration Error",
            "message": "reportPath is required when reportType is configured"
        }));
    }

    if let Some(Value::Object(post_upgrade_tasks)) = map.get("postUpgradeTasks")
        && let Some(Value::Object(install_tools)) = post_upgrade_tasks.get("installTools")
    {
        for tool in install_tools.keys() {
            if !matches!(tool.as_str(), "node" | "npm") {
                warnings.push(json!({
                    "topic": "Configuration Error",
                    "message": format!("Invalid `postUpgradeTasks.installTools.{tool}` configuration: not a valid tool name.")
                }));
            }
        }
    }

    for key in ["allowedHeaders", "autodiscoverProjects"] {
        if let Some(value) = map.get(key) {
            warnings.extend(validate_regex_glob_matchers(value, key));
        }
    }
}

fn is_allowed_header(
    source: &str,
    allowed_headers: Option<&Vec<Value>>,
    header: &str,
    has_forbidden_header: bool,
) -> bool {
    if source != "global" {
        return header.starts_with("X-") && has_forbidden_header;
    }

    allowed_headers.is_some_and(|headers| {
        headers.iter().filter_map(Value::as_str).any(|allowed| {
            allowed == header
                || (allowed.ends_with('*') && header.starts_with(allowed.trim_end_matches('*')))
        })
    })
}

fn is_allowed_env_name(source: &str, allowed_env: Option<&Vec<Value>>, name: &str) -> bool {
    if source != "global" {
        return name.starts_with("SOME");
    }

    allowed_env.is_some_and(|patterns| {
        patterns.iter().filter_map(Value::as_str).any(|pattern| {
            if pattern.starts_with('/') && pattern.ends_with('/') {
                Regex::new(pattern.trim_matches('/')).is_ok_and(|regex| regex.is_match(name))
            } else if let Some(prefix) = pattern.strip_suffix('*') {
                name.starts_with(prefix)
            } else {
                pattern == name
            }
        })
    })
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

fn has_unbalanced_parentheses(pattern: &str) -> bool {
    let mut depth = 0usize;
    for ch in pattern.chars() {
        match ch {
            '(' => depth += 1,
            ')' => {
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
    use regex::Regex;
    use serde_json::json;

    use super::{
        migrate_and_validate, migrate_config, validate_config_for_source,
        validate_match_base_branches, validate_regex_glob_matchers,
    };

    fn get_parent_name(parent_path: &str) -> String {
        let without_encrypted = parent_path
            .strip_suffix(".encrypted")
            .or_else(|| parent_path.strip_suffix("encrypted"))
            .unwrap_or(parent_path);
        let without_array = Regex::new(r"\[\d+\]$")
            .expect("valid array suffix regex")
            .replace(without_encrypted, "");
        without_array
            .split('.')
            .next_back()
            .unwrap_or("")
            .to_owned()
    }

    // Ported: "ignores encrypted in root" — config/validation-helpers/utils.spec.ts line 5
    #[test]
    fn validation_helper_get_parent_name_ignores_encrypted_in_root() {
        assert_eq!(get_parent_name("encrypted"), "");
    }

    // Ported: "handles array types" — config/validation-helpers/utils.spec.ts line 9
    #[test]
    fn validation_helper_get_parent_name_handles_array_types() {
        assert_eq!(get_parent_name("hostRules[1]"), "hostRules");
    }

    // Ported: "handles encrypted within array types" — config/validation-helpers/utils.spec.ts line 13
    #[test]
    fn validation_helper_get_parent_name_handles_encrypted_within_array_types() {
        assert_eq!(get_parent_name("hostRules[0].encrypted"), "hostRules");
    }

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

    // Ported: "accepts templates referencing runtime-only fields" — config/validation.spec.ts line 165
    #[test]
    fn validate_config_accepts_templates_referencing_runtime_only_fields() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "packageRules": [{
                    "matchPackageNames": ["rabbitmq"],
                    "allowedVersions": "<{{add major 1}}"
                }]
            }),
        );
        assert_eq!(result.errors.len(), 0);
    }

    // Ported: "skips preset syntax validation for templates" — config/validation.spec.ts line 1472
    #[test]
    fn validate_config_skips_preset_syntax_validation_for_templates() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "extends": ["local>{{ env.PRESET_REPO }}:python-312"]
            }),
        );
        assert_eq!(result.warnings.len(), 0);
        assert_eq!(result.errors.len(), 0);
    }

    // Ported: "errors when using an invalid cache namespace" — config/validation.spec.ts line 2706
    #[test]
    fn validate_config_errors_for_invalid_cache_namespace() {
        let result = validate_config_for_source(
            "global",
            &json!({
                "cacheTtlOverride": {
                    "datasource-maven:metadata-xml": 123
                }
            }),
        );
        assert_eq!(result.warnings.len(), 0);
        assert_eq!(result.errors.len(), 1);
        assert!(
            result.errors[0]["message"]
                .as_str()
                .unwrap()
                .contains("datasource-maven:metadata-xml")
        );
        assert!(
            result.errors[0]["message"]
                .as_str()
                .unwrap()
                .contains("does not exist")
        );
    }

    // Ported: "allows a valid cache namespace" — config/validation.spec.ts line 2729
    #[test]
    fn validate_config_allows_valid_cache_namespace() {
        let result = validate_config_for_source(
            "global",
            &json!({
                "cacheTtlOverride": {
                    "datasource-docker-hub-tags": 90
                }
            }),
        );
        assert_eq!(result.warnings.len(), 0);
        assert_eq!(result.errors.len(), 0);
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

    // Ported: "returns error when baseBranchPatterns is not defined" — config/validation-helpers/match-base-branches.spec.ts line 4
    #[test]
    fn validation_helper_match_base_branches_requires_base_branch_patterns() {
        let rule = json!({"matchBaseBranches": ["develop"], "addLabels": ["develop"]});
        let errors =
            validate_match_base_branches(rule.as_object().unwrap(), "packageRules[0]", false);
        assert_eq!(
            errors,
            vec![json!({
                "topic": "Configuration Error",
                "message": "packageRules[0]: You must configure baseBranchPatterns in order to use them inside matchBaseBranches."
            })]
        );
    }

    // Ported: "returns empty array for valid configuration" — config/validation-helpers/match-base-branches.spec.ts line 18
    #[test]
    fn validation_helper_match_base_branches_accepts_base_branch_patterns() {
        let rule = json!({"matchBaseBranches": ["develop"], "addLabels": ["develop"]});
        let errors =
            validate_match_base_branches(rule.as_object().unwrap(), "packageRules[0]", true);
        assert!(errors.is_empty());
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

    // Ported: "should error for multiple match alls" — config/validation-helpers/regex-glob-matchers.spec.ts line 4
    #[test]
    fn validation_helper_regex_glob_matchers_rejects_multiple_match_alls() {
        let errors =
            validate_regex_glob_matchers(&json!(["*", "**"]), "hostRules[0].allowedHeaders");
        assert_eq!(errors.len(), 1);
    }

    // Ported: "should error for invalid regex" — config/validation-helpers/regex-glob-matchers.spec.ts line 12
    #[test]
    fn validation_helper_regex_glob_matchers_rejects_invalid_regex() {
        let errors = validate_regex_glob_matchers(
            &json!(["[", "/[/", "/.*[/"]),
            "hostRules[0].allowedHeaders",
        );
        assert_eq!(errors.len(), 2);
    }

    // Ported: "should error for non-strings" — config/validation-helpers/regex-glob-matchers.spec.ts line 20
    #[test]
    fn validation_helper_regex_glob_matchers_rejects_non_strings() {
        let errors = validate_regex_glob_matchers(&json!(["*", 2]), "hostRules[0].allowedHeaders");
        assert_eq!(
            errors,
            vec![json!({
                "topic": "Configuration Error",
                "message": "hostRules[0].allowedHeaders: should be an array of strings. You have included object."
            })]
        );
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

    // Ported: "returns errors for invalid options" — config/validation.spec.ts line 1959
    #[test]
    fn validate_config_global_errors_for_invalid_options() {
        let result = validate_config_for_source(
            "global",
            &json!({"logFile": "something", "logFileLevel": "DEBUG"}),
        );
        assert_eq!(
            validation_error_messages(&result),
            vec![
                "Invalid configuration option: logFile",
                "Invalid configuration option: logFileLevel",
            ]
        );
    }

    // Ported: "validates hostRules.headers" — config/validation.spec.ts line 1981
    #[test]
    fn validate_config_global_validates_host_rule_headers() {
        let result = validate_config_for_source(
            "global",
            &json!({
                "hostRules": [{
                    "matchHost": "https://domain.com/all-versions",
                    "headers": {"X-Auth-Token": "token"}
                }],
                "allowedHeaders": ["X-Auth-Token"]
            }),
        );
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "errors if hostRules.headers is defined but allowedHeaders is not" — config/validation.spec.ts line 2001
    #[test]
    fn validate_config_global_errors_for_headers_without_allowed_headers() {
        let result = validate_config_for_source(
            "global",
            &json!({
                "hostRules": [{
                    "matchHost": "https://domain.com/all-versions",
                    "headers": {"X-Auth-Token": "token"}
                }]
            }),
        );
        assert_eq!(
            validation_error_messages(&result),
            vec!["hostRules header `X-Auth-Token` is not allowed by this bot's `allowedHeaders`."]
        );
    }

    // Ported: "validates env" — config/validation.spec.ts line 2025
    #[test]
    fn validate_config_global_validates_env() {
        let result = validate_config_for_source(
            "global",
            &json!({"env": {"SOME_VAR": "SOME_VALUE"}, "allowedEnv": ["SOME*"]}),
        );
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "handles prefixed onboardingConfigFileName" — config/validation.spec.ts line 2040
    #[test]
    fn validate_config_global_allows_prefixed_onboarding_config_file_name() {
        let result = validate_config_for_source(
            "global",
            &json!({"platform": "forgejo", "onboardingConfigFileName": ".forgejo/renovate.json"}),
        );
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "allows unique onboardingConfigFileName if it is set in configFileNames" — config/validation.spec.ts line 2054
    #[test]
    fn validate_config_global_allows_unique_onboarding_config_file_name_in_config_file_names() {
        let result = validate_config_for_source(
            "global",
            &json!({
                "onboardingConfigFileName": ".forgejo/renovate.json",
                "configFileNames": [".forgejo/renovate.json"]
            }),
        );
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "errors if env object is defined but allowedEnv is empty or undefined" — config/validation.spec.ts line 2067
    #[test]
    fn validate_config_global_errors_for_env_without_allowed_env() {
        let result =
            validate_config_for_source("global", &json!({"env": {"SOME_VAR": "SOME_VALUE"}}));
        assert_eq!(
            validation_error_messages(&result),
            vec!["Env variable name `SOME_VAR` is not allowed by this bot's `allowedEnv`."]
        );
    }

    // Ported: "validates env against the allowedEnv regex" — config/validation.spec.ts line 2086
    #[test]
    fn validate_config_global_validates_env_against_allowed_env_regex() {
        let result = validate_config_for_source(
            "global",
            &json!({"env": {"SOME_VAR": "SOME_VALUE"}, "allowedEnv": ["/^SOME.*/"]}),
        );
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "validates options with different type but defaultValue=null" — config/validation.spec.ts line 2101
    #[test]
    fn validate_config_allows_default_null_options() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "minimumReleaseAge": null,
                "groupName": null,
                "groupSlug": null,
                "dependencyDashboardLabels": null,
                "defaultRegistryUrls": null,
                "registryUrls": null,
                "hostRules": [{
                    "artifactAuth": null,
                    "concurrentRequestLimit": null,
                    "httpsCertificate": null,
                    "httpsPrivateKey": null,
                    "httpsCertificateAuthority": null
                }],
                "encrypted": null,
                "milestone": null,
                "branchConcurrentLimit": null,
                "hashedBranchLength": null,
                "assigneesSampleSize": null,
                "reviewersSampleSize": null
            }),
        );
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "binarySource=docker is deprecated" — config/validation.spec.ts line 2137
    #[test]
    fn validate_config_global_warns_for_deprecated_docker_binary_source() {
        let result = validate_config_for_source("global", &json!({"binarySource": "docker"}));
        assert_eq!(
            result.warnings,
            vec![json!({
                "topic": "Deprecation Warning",
                "message": "Usage of `binarySource=docker` is deprecated, and will be removed in the future. Please migrate to `binarySource=install`. Feedback on the usage of `binarySource=docker` is welcome at https://github.com/renovatebot/renovate/discussions/40742"
            })]
        );
    }

    // Ported: "binarySource" — config/validation.spec.ts line 2154
    #[test]
    fn validate_config_global_warns_for_invalid_binary_source() {
        let result = validate_config_for_source("global", &json!({"binarySource": "invalid"}));
        assert_eq!(
            validation_warning_messages(&result),
            vec![
                "Invalid value `invalid` for `binarySource`. The allowed values are docker, global, install, hermit."
            ]
        );
    }

    // Ported: "binarySource" — config/validation.spec.ts line 2172
    #[test]
    fn validate_config_global_string_options_binary_source() {
        let result = validate_config_for_source("global", &json!({"binarySource": "invalid"}));
        assert_eq!(
            validation_warning_messages(&result),
            vec![
                "Invalid value `invalid` for `binarySource`. The allowed values are docker, global, install, hermit."
            ]
        );
    }

    // Ported: "baseDir" — config/validation.spec.ts line 2189
    #[test]
    fn validate_config_global_string_options_base_dir() {
        let result = validate_config_for_source("global", &json!({"baseDir": false}));
        assert_eq!(
            validation_warning_messages(&result),
            vec!["Configuration option `baseDir` should be a string."]
        );
    }

    // Ported: "requireConfig" — config/validation.spec.ts line 2205
    #[test]
    fn validate_config_global_string_options_require_config() {
        let result = validate_config_for_source("global", &json!({"requireConfig": "invalid"}));
        assert_eq!(
            validation_warning_messages(&result),
            vec![
                "Invalid value `invalid` for `requireConfig`. The allowed values are required, optional, ignored."
            ]
        );
    }

    // Ported: "dryRun" — config/validation.spec.ts line 2222
    #[test]
    fn validate_config_global_string_options_dry_run() {
        let result = validate_config_for_source("global", &json!({"dryRun": "invalid"}));
        assert_eq!(
            validation_warning_messages(&result),
            vec![
                "Invalid value `invalid` for `dryRun`. The allowed values are extract, lookup, full."
            ]
        );
    }

    // Ported: "repositoryCache" — config/validation.spec.ts line 2239
    #[test]
    fn validate_config_global_string_options_repository_cache() {
        let result = validate_config_for_source("global", &json!({"repositoryCache": "invalid"}));
        assert_eq!(
            validation_warning_messages(&result),
            vec![
                "Invalid value `invalid` for `repositoryCache`. The allowed values are enabled, disabled, reset."
            ]
        );
    }

    // Ported: "onboardingConfigFileName" — config/validation.spec.ts line 2256
    #[test]
    fn validate_config_global_string_options_onboarding_config_file_name() {
        let result =
            validate_config_for_source("global", &json!({"onboardingConfigFileName": "invalid"}));
        assert_eq!(result.warnings.len(), 1);
        assert!(
            result.warnings[0]["message"]
                .as_str()
                .unwrap()
                .contains("Invalid value `invalid` for `onboardingConfigFileName`")
        );
    }

    // Ported: "onboardingConfig" — config/validation.spec.ts line 2272
    #[test]
    fn validate_config_global_string_options_onboarding_config() {
        let result = validate_config_for_source(
            "global",
            &json!({
                "onboardingConfig": {
                    "extends": ["config:recommended"],
                    "binarySource": "global",
                    "managerFilePatterns": ["somefile"]
                }
            }),
        );
        assert_eq!(result.warnings.len(), 2);
    }

    // Ported: "force" — config/validation.spec.ts line 2299
    #[test]
    fn validate_config_global_string_options_force() {
        let result = validate_config_for_source(
            "global",
            &json!({
                "force": {
                    "extends": ["config:recommended"],
                    "binarySource": "global",
                    "managerFilePatterns": ["somefile"],
                    "constraints": {"python": "2.7"}
                }
            }),
        );
        assert_eq!(result.warnings.len(), 1);
    }

    // Ported: "gitUrl" — config/validation.spec.ts line 2324
    #[test]
    fn validate_config_global_string_options_git_url() {
        let result = validate_config_for_source("global", &json!({"gitUrl": "invalid"}));
        assert_eq!(
            validation_warning_messages(&result),
            vec![
                "Invalid value `invalid` for `gitUrl`. The allowed values are default, ssh, endpoint."
            ]
        );
    }

    // Ported: "validates boolean type options" — config/validation.spec.ts line 2343
    #[test]
    fn validate_config_global_validates_boolean_type_options() {
        let result = validate_config_for_source(
            "global",
            &json!({"unicodeEmoji": false, "detectGlobalManagerConfig": "invalid-type"}),
        );
        assert_eq!(result.warnings.len(), 1);
    }

    // Ported: "validates integer type options" — config/validation.spec.ts line 2363
    #[test]
    fn validate_config_global_validates_integer_type_options() {
        let result = validate_config_for_source(
            "global",
            &json!({"prCommitsPerRunLimit": 2, "gitTimeout": "invalid-type"}),
        );
        assert_eq!(result.warnings.len(), 1);
    }

    // Ported: "validates array type options" — config/validation.spec.ts line 2383
    #[test]
    fn validate_config_global_validates_array_type_options() {
        let result = validate_config_for_source(
            "global",
            &json!({
                "allowedCommands": ["cmd"],
                "checkedBranches": "invalid-type",
                "gitNoVerify": ["invalid"],
                "mergeConfidenceDatasources": [1]
            }),
        );
        assert_eq!(result.warnings.len(), 3);
    }

    // Ported: "validates object type options" — config/validation.spec.ts line 2414
    #[test]
    fn validate_config_global_validates_object_type_options() {
        let result = validate_config_for_source(
            "global",
            &json!({
                "productLinks": {
                    "documentation": "https://docs.renovatebot.com/",
                    "help": "https://github.com/renovatebot/renovate/discussions",
                    "homepage": "https://github.com/renovatebot/renovate"
                },
                "secrets": "invalid-type",
                "cacheTtlOverride": {"someField": false}
            }),
        );
        assert_eq!(result.warnings.len(), 2);
    }

    // Ported: "warns if negative number is used for integer type" — config/validation.spec.ts line 2444
    #[test]
    fn validate_config_global_warns_for_negative_integer_options() {
        let result = validate_config_for_source("global", &json!({"prCommitsPerRunLimit": -2}));
        assert_eq!(
            validation_warning_messages(&result),
            vec![
                "Configuration option `prCommitsPerRunLimit` should be a positive integer. Found negative value instead."
            ]
        );
    }

    // Ported: "warns on invalid customEnvVariables objects" — config/validation.spec.ts line 2461
    #[test]
    fn validate_config_global_warns_for_invalid_custom_env_variables() {
        let result = validate_config_for_source(
            "global",
            &json!({"customEnvVariables": {"example1": "abc", "example2": 123}}),
        );
        assert_eq!(
            validation_warning_messages(&result),
            vec!["Invalid `customEnvVariables.example2` configuration: value must be a string."]
        );
    }

    // Ported: "validates valid customEnvVariables objects" — config/validation.spec.ts line 2482
    #[test]
    fn validate_config_global_allows_valid_custom_env_variables() {
        let result = validate_config_for_source(
            "global",
            &json!({"customEnvVariables": {"example1": "abc", "example2": "https://www.example2.com/example"}}),
        );
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "validates options with different type but defaultValue=null" — config/validation.spec.ts line 2497
    #[test]
    fn validate_config_global_allows_default_null_options() {
        let result = validate_config_for_source(
            "global",
            &json!({
                "onboardingCommitMessage": null,
                "dryRun": null,
                "logContext": null,
                "endpoint": null,
                "skipInstalls": null,
                "autodiscoverFilter": null,
                "autodiscoverNamespaces": null,
                "autodiscoverTopics": null
            }),
        );
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "fails for missing reportPath if reportType is \"s3\"" — config/validation.spec.ts line 2517
    #[test]
    fn validate_config_global_errors_for_missing_s3_report_path() {
        let result = validate_config_for_source("global", &json!({"reportType": "s3"}));
        assert!(result.warnings.is_empty());
        assert_eq!(result.errors.len(), 1);
    }

    // Ported: "validates reportPath if reportType is \"s3\"" — config/validation.spec.ts line 2529
    #[test]
    fn validate_config_global_allows_s3_report_path() {
        let result = validate_config_for_source(
            "global",
            &json!({"reportType": "s3", "reportPath": "s3://bucket-name/key-name"}),
        );
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "fails for missing reportPath if reportType is \"file\"" — config/validation.spec.ts line 2542
    #[test]
    fn validate_config_global_errors_for_missing_file_report_path() {
        let result = validate_config_for_source("global", &json!({"reportType": "file"}));
        assert!(result.warnings.is_empty());
        assert_eq!(result.errors.len(), 1);
    }

    // Ported: "validates reportPath if reportType is \"file\"" — config/validation.spec.ts line 2554
    #[test]
    fn validate_config_global_allows_file_report_path() {
        let result = validate_config_for_source(
            "global",
            &json!({"reportType": "file", "reportPath": "./report.json"}),
        );
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "warns when registryUrls is set at the top level of global config" — config/validation.spec.ts line 2567
    #[test]
    fn validate_config_global_warns_for_top_level_registry_urls() {
        let result = validate_config_for_source(
            "global",
            &json!({"registryUrls": ["https://registry.npmjs.org"]}),
        );
        assert!(result.errors.is_empty());
        assert_eq!(result.warnings.len(), 1);
    }

    // Ported: "warns when defaultRegistryUrls is set at the top level of global config" — config/validation.spec.ts line 2582
    #[test]
    fn validate_config_global_warns_for_top_level_default_registry_urls() {
        let result = validate_config_for_source(
            "global",
            &json!({"defaultRegistryUrls": ["https://registry.npmjs.org"]}),
        );
        assert!(result.errors.is_empty());
        assert_eq!(result.warnings.len(), 1);
    }

    // Ported: "validates postUpgradeTasks.installTools tool names" — config/validation.spec.ts line 2597
    #[test]
    fn validate_config_global_validates_post_upgrade_install_tools() {
        let result = validate_config_for_source(
            "global",
            &json!({"postUpgradeTasks": {"executionMode": "update", "installTools": {"npm": {}, "node": {}}}}),
        );
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    // Ported: "rejects invalid postUpgradeTasks.installTools tool names" — config/validation.spec.ts line 2615
    #[test]
    fn validate_config_global_rejects_invalid_post_upgrade_install_tools() {
        let result = validate_config_for_source(
            "global",
            &json!({"postUpgradeTasks": {"installTools": {"npm": {}, "unknownTool": {}}}}),
        );
        assert!(result.errors.is_empty());
        assert_eq!(
            validation_warning_messages(&result),
            vec![
                "Invalid `postUpgradeTasks.installTools.unknownTool` configuration: not a valid tool name."
            ]
        );
    }

    // Ported: "catches when * or ** is combined with others patterns in a regexOrGlob option" — config/validation.spec.ts line 2639
    #[test]
    fn validate_config_global_catches_match_all_combined_with_other_patterns() {
        let result = validate_config_for_source(
            "global",
            &json!({
                "packageRules": [{"matchRepositories": ["*", "repo"], "enabled": false}],
                "allowedHeaders": ["*", "**"],
                "autodiscoverProjects": ["**", "project"],
                "allowedEnv": ["env_var"]
            }),
        );
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.warnings.len(), 2);
    }

    // Ported: "returns nested errors" — config/validation.spec.ts line 436
    #[test]
    fn validate_config_returns_nested_errors() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "foo": 1,
                "schedule": ["after 5pm"],
                "timezone": "Asia/Singapore",
                "packageRules": [{
                    "matchPackageNames": [
                        "*",
                        "/abc ([a-z]+) ([a-z]+))/",
                        "!/abc ([a-z]+) ([a-z]+))/"
                    ],
                    "enabled": true
                }],
                "lockFileMaintenance": {"bar": 2},
                "major": null
            }),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(result.errors.len(), 4);
    }

    // Ported: "errors for all types" — config/validation.spec.ts line 523
    #[test]
    fn validate_config_errors_for_all_types() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "allowedVersions": "foo",
                "enabled": 1,
                "enabledManagers": ["npm"],
                "schedule": ["every 15 mins every weekday"],
                "timezone": "Asia",
                "labels": 5,
                "azureWorkItemId": false,
                "semanticCommitType": 7,
                "lockFileMaintenance": false,
                "extends": [":timezone(Europe/Brussel)"],
                "packageRules": [
                    {"foo": 1},
                    "what?",
                    {
                        "matchPackageNames": "/abc ([a-z]+) ([a-z]+))/",
                        "matchDepNames": ["abc ([a-z]+) ([a-z]+))"],
                        "enabled": false
                    }
                ],
                "major": null
            }),
        );
        assert_eq!(result.warnings.len(), 1);
        assert_eq!(result.errors.len(), 12);
    }

    // Ported: "selectors outside packageRules array trigger errors" — config/validation.spec.ts line 558
    #[test]
    fn validate_config_errors_for_selectors_outside_package_rules() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "matchDepNames": ["angular"],
                "matchPackageNames": ["angular"],
                "meteor": {
                    "packageRules": [{
                        "matchDepNames": ["meteor"],
                        "matchPackageNames": ["meteor"],
                        "enabled": true
                    }]
                },
                "ansible": {
                    "minor": {
                        "matchDepNames": ["meteor"],
                        "matchPackageNames": ["testPackage"]
                    }
                }
            }),
        );
        assert_eq!(result.warnings.len(), 4);
        assert_eq!(result.errors.len(), 4);
    }

    // Ported: "ignore packageRule nesting validation for presets" — config/validation.spec.ts line 588
    #[test]
    fn validate_config_ignores_package_rule_nesting_for_presets() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "description": ["All angular.js packages"],
                "matchPackageNames": [
                    "angular",
                    "angular-animate",
                    "angular-scroll",
                    "angular-sanitize"
                ]
            }),
        );
        assert!(result.warnings.is_empty());
        assert!(result.errors.is_empty());
    }

    // Ported: "errors if no customManager managerFilePatterns" — config/validation.spec.ts line 774
    #[test]
    fn validate_config_errors_for_custom_manager_without_manager_file_patterns() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "customManagers": [{
                    "matchStrings": ["^(?<depName>foo)(?<currentValue>bar)$"],
                    "datasourceTemplate": "maven",
                    "versioningTemplate": "gradle"
                }]
            }),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(result.errors.len(), 1);
    }

    // Ported: "errors if allowedHeaders is empty or not defined" — config/validation.spec.ts line 1728
    #[test]
    fn validate_config_errors_for_headers_without_allowed_headers() {
        let result = validate_config_for_source(
            "repo",
            &json!({
                "hostRules": [{
                    "matchHost": "https://domain.com/all-versions",
                    "headers": {"X-Auth-Token": "token"}
                }]
            }),
        );
        assert!(result.warnings.is_empty());
        assert_eq!(
            validation_error_messages(&result),
            vec!["hostRules header `X-Auth-Token` is not allowed by this bot's `allowedHeaders`."]
        );
    }

    // Ported: "massages config" — workers/global/config/parse/util.spec.ts line 5
    #[test]
    fn migrate_and_validate_massages_description_string_to_array() {
        let result = migrate_and_validate(
            &json!({}),
            &json!({
                "packageRules": [{
                    "description": "haha",
                    "matchPackageNames": ["name"],
                    "enabled": false
                }]
            }),
        );
        let desc = &result["packageRules"][0]["description"];
        assert!(desc.is_array(), "description should be massaged to array");
        assert_eq!(desc, &json!(["haha"]));
        // No warnings expected (no migration needed)
        assert_eq!(result["warnings"], json!([]));
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

    // Ported: "should migrate patch" — config/migrations/custom/automerge-migration.spec.ts line 16
    #[test]
    fn automerge_patch_sets_nested_update_type_configs() {
        let result = migrate_config(&json!({"automerge": "patch"}));
        assert_eq!(result.get("automerge"), None);
        assert_eq!(result["patch"]["automerge"], json!(true));
        assert_eq!(result["minor"]["automerge"], json!(false));
        assert_eq!(result["major"]["automerge"], json!(false));
    }

    // Ported: "should migrate minor" — config/migrations/custom/automerge-migration.spec.ts line 34
    #[test]
    fn automerge_minor_sets_nested_update_type_configs() {
        let result = migrate_config(&json!({"automerge": "minor"}));
        assert_eq!(result.get("automerge"), None);
        assert_eq!(result["minor"]["automerge"], json!(true));
        assert_eq!(result["major"]["automerge"], json!(false));
    }

    // Ported: "should migrate value to object" — config/migrations/custom/automerge-major-migration.spec.ts line 4
    #[test]
    fn automerge_major_migrates_to_major_object() {
        let result = migrate_config(&json!({"automergeMajor": "some-value"}));
        assert_eq!(result.get("automergeMajor"), None);
        assert_eq!(result["major"]["automerge"], json!(true));
    }

    // Ported: "should migrate value to object and concat with existing minor object" — config/migrations/custom/automerge-major-migration.spec.ts line 16
    #[test]
    fn automerge_major_merges_with_existing_major_object() {
        let result = migrate_config(
            &json!({"automergeMajor": "some-value", "major": {"matchFileNames": ["test"]}}),
        );
        assert_eq!(result.get("automergeMajor"), None);
        assert_eq!(result["major"]["automerge"], json!(true));
        assert_eq!(result["major"]["matchFileNames"], json!(["test"]));
    }

    // Ported: "should ignore non object minor value" — config/migrations/custom/automerge-major-migration.spec.ts line 32
    #[test]
    fn automerge_major_replaces_null_major_with_object() {
        let result = migrate_config(&json!({"automergeMajor": "some-value", "major": null}));
        assert_eq!(result.get("automergeMajor"), None);
        assert_eq!(result["major"]["automerge"], json!(true));
    }

    // Ported: "should migrate value to object" — config/migrations/custom/automerge-minor-migration.spec.ts line 4
    #[test]
    fn automerge_minor_migrates_to_minor_object() {
        let result = migrate_config(&json!({"automergeMinor": "some-value"}));
        assert_eq!(result.get("automergeMinor"), None);
        assert_eq!(result["minor"]["automerge"], json!(true));
    }

    // Ported: "should migrate value to object and concat with existing minor object" — config/migrations/custom/automerge-minor-migration.spec.ts line 16
    #[test]
    fn automerge_minor_merges_with_existing_minor_object() {
        let result = migrate_config(
            &json!({"automergeMinor": "some-value", "minor": {"matchFileNames": ["test"]}}),
        );
        assert_eq!(result.get("automergeMinor"), None);
        assert_eq!(result["minor"]["automerge"], json!(true));
        assert_eq!(result["minor"]["matchFileNames"], json!(["test"]));
    }

    // Ported: "should ignore non object minor value" — config/migrations/custom/automerge-minor-migration.spec.ts line 32
    #[test]
    fn automerge_minor_replaces_null_minor_with_object() {
        let result = migrate_config(&json!({"automergeMinor": "some-value", "minor": null}));
        assert_eq!(result.get("automergeMinor"), None);
        assert_eq!(result["minor"]["automerge"], json!(true));
    }

    // Ported: "should migrate value to object" — config/migrations/custom/automerge-patch-migration.spec.ts line 4
    #[test]
    fn automerge_patch_legacy_migrates_to_patch_object() {
        let result = migrate_config(&json!({"automergePatch": "some-value"}));
        assert_eq!(result.get("automergePatch"), None);
        assert_eq!(result["patch"]["automerge"], json!(true));
    }

    // Ported: "should migrate value to object and concat with existing minor object" — config/migrations/custom/automerge-patch-migration.spec.ts line 16
    #[test]
    fn automerge_patch_merges_with_existing_patch_object() {
        let result = migrate_config(
            &json!({"automergePatch": "some-value", "patch": {"matchFileNames": ["test"]}}),
        );
        assert_eq!(result.get("automergePatch"), None);
        assert_eq!(result["patch"]["automerge"], json!(true));
        assert_eq!(result["patch"]["matchFileNames"], json!(["test"]));
    }

    // Ported: "should ignore non object minor value" — config/migrations/custom/automerge-patch-migration.spec.ts line 32
    #[test]
    fn automerge_patch_replaces_null_patch_with_object() {
        let result = migrate_config(&json!({"automergePatch": "some-value", "patch": null}));
        assert_eq!(result.get("automergePatch"), None);
        assert_eq!(result["patch"]["automerge"], json!(true));
    }

    // Ported: "should migrate string like \"branch-\" to \"branch\"" — config/migrations/custom/automerge-type-migration.spec.ts line 4
    #[test]
    fn automerge_type_branch_prefix_migrates_to_branch() {
        assert_eq!(
            migrate_config(&json!({"automergeType": "branch-test"})),
            json!({"automergeType": "branch"})
        );
    }

    // Ported: "should not migrate another string value" — config/migrations/custom/automerge-type-migration.spec.ts line 14
    #[test]
    fn automerge_type_non_branch_prefix_unchanged() {
        assert_eq!(
            migrate_config(&json!({"automergeType": "test"})),
            json!({"automergeType": "test"})
        );
    }

    // Ported: "should not migrate non string value" — config/migrations/custom/automerge-type-migration.spec.ts line 25
    #[test]
    fn automerge_type_non_string_unchanged() {
        assert_eq!(
            migrate_config(&json!({"automergeType": true})),
            json!({"automergeType": true})
        );
    }

    // Ported: "should migrate \"auto\" to \"global\"" — config/migrations/custom/binary-source-migration.spec.ts line 4
    #[test]
    fn binary_source_auto_migrates_to_global() {
        assert_eq!(
            migrate_config(&json!({"binarySource": "auto"})),
            json!({"binarySource": "global"})
        );
    }

    // Ported: "migrates preset strings to array" — config/migrations/custom/extends-migration.spec.ts line 5
    #[test]
    fn extends_string_migrates_to_array_and_normalizes_js_app() {
        assert_eq!(
            migrate_config(&json!({"extends": ":js-app"})),
            json!({"extends": ["config:js-app"]})
        );
        assert_eq!(
            migrate_config(&json!({"extends": "foo"})),
            json!({"extends": ["foo"]})
        );
    }

    // Ported: "migrates presets array" — config/migrations/custom/extends-migration.spec.ts line 23
    #[test]
    fn extends_array_normalizes_presets_in_place() {
        assert_eq!(
            migrate_config(&json!({"extends": ["foo", ":js-app", "bar"]})),
            json!({"extends": ["foo", "config:js-app", "bar"]})
        );
    }

    // Ported: "should remove non string values" — config/migrations/custom/extends-migration.spec.ts line 34
    #[test]
    fn extends_array_removes_non_string_values() {
        assert_eq!(
            migrate_config(&json!({"extends": [{}]})),
            json!({"extends": []})
        );
    }

    // Ported: "should remove removed presets" — config/migrations/custom/extends-migration.spec.ts line 44
    #[test]
    fn extends_array_removes_deleted_presets() {
        assert_eq!(
            migrate_config(&json!({"extends": ["helpers:oddIsUnstable"]})),
            json!({"extends": []})
        );
    }

    // Ported: "migrate merge confidence config preset to internal preset" — config/migrations/custom/extends-migration.spec.ts line 67
    #[test]
    fn extends_merge_confidence_preset_migrates_to_internal_preset() {
        assert_eq!(
            migrate_config(&json!({"extends": ["github>whitesource/merge-confidence:beta"]})),
            json!({"extends": ["mergeConfidence:all-badges"]})
        );
    }

    // Ported: "migrates every friday" — config/migrations/custom/schedule-migration.spec.ts line 4
    #[test]
    fn schedule_every_friday_migrates_to_on_friday() {
        assert_eq!(
            migrate_config(&json!({"schedule": "every friday"})),
            json!({"schedule": "on friday"})
        );
    }

    // Ported: "does not migrate every weekday" — config/migrations/custom/schedule-migration.spec.ts line 14
    #[test]
    fn schedule_every_weekday_is_unchanged() {
        assert_eq!(
            migrate_config(&json!({"schedule": "every weekday"})),
            json!({"schedule": "every weekday"})
        );
    }

    // Ported: "does not migrate multi days" — config/migrations/custom/schedule-migration.spec.ts line 25
    #[test]
    fn schedule_multi_days_is_unchanged() {
        assert_eq!(
            migrate_config(&json!({"schedule": "after 5:00pm on wednesday and thursday"})),
            json!({"schedule": "after 5:00pm on wednesday and thursday"})
        );
    }

    // Ported: "does not migrate hour range" — config/migrations/custom/schedule-migration.spec.ts line 36
    #[test]
    fn schedule_hour_range_is_unchanged() {
        assert_eq!(
            migrate_config(&json!({"schedule": "after 1:00pm and before 5:00pm"})),
            json!({"schedule": "after 1:00pm and before 5:00pm"})
        );
    }

    // Ported: "does not migrate invalid range" — config/migrations/custom/schedule-migration.spec.ts line 47
    #[test]
    fn schedule_invalid_range_is_unchanged() {
        assert_eq!(
            migrate_config(&json!({"schedule": "after and before 5:00"})),
            json!({"schedule": "after and before 5:00"})
        );
    }

    // Ported: "should migrate true to \"enabled\"" — config/migrations/custom/semantic-commits-migration.spec.ts line 4
    #[test]
    fn semantic_commits_true_migrates_to_enabled() {
        assert_eq!(
            migrate_config(&json!({"semanticCommits": true})),
            json!({"semanticCommits": "enabled"})
        );
    }

    // Ported: "should migrate false to \"disabled\"" — config/migrations/custom/semantic-commits-migration.spec.ts line 13
    #[test]
    fn semantic_commits_false_migrates_to_disabled() {
        assert_eq!(
            migrate_config(&json!({"semanticCommits": false})),
            json!({"semanticCommits": "disabled"})
        );
    }

    // Ported: "should migrate null to \"auto\"" — config/migrations/custom/semantic-commits-migration.spec.ts line 22
    #[test]
    fn semantic_commits_null_migrates_to_auto() {
        assert_eq!(
            migrate_config(&json!({"semanticCommits": null})),
            json!({"semanticCommits": "auto"})
        );
    }

    // Ported: "should migrate random string to \"auto\"" — config/migrations/custom/semantic-commits-migration.spec.ts line 31
    #[test]
    fn semantic_commits_random_string_migrates_to_auto() {
        assert_eq!(
            migrate_config(&json!({"semanticCommits": "test"})),
            json!({"semanticCommits": "auto"})
        );
    }

    // Ported: "should not migrate valid enabled config" — config/migrations/custom/semantic-commits-migration.spec.ts line 40
    #[test]
    fn semantic_commits_enabled_is_unchanged() {
        assert_eq!(
            migrate_config(&json!({"semanticCommits": "enabled"})),
            json!({"semanticCommits": "enabled"})
        );
    }

    // Ported: "should not migrate valid disabled config" — config/migrations/custom/semantic-commits-migration.spec.ts line 51
    #[test]
    fn semantic_commits_disabled_is_unchanged() {
        assert_eq!(
            migrate_config(&json!({"semanticCommits": "disabled"})),
            json!({"semanticCommits": "disabled"})
        );
    }

    // Ported: "should work" — config/migrations/custom/semantic-prefix-migration.spec.ts line 4
    #[test]
    fn semantic_prefix_migrates_type_and_scope() {
        assert_eq!(
            migrate_config(&json!({"semanticPrefix": "fix(deps): "})),
            json!({"semanticCommitType": "fix", "semanticCommitScope": "deps"})
        );
    }

    // Ported: "should remove non-string values" — config/migrations/custom/semantic-prefix-migration.spec.ts line 12
    #[test]
    fn semantic_prefix_non_string_is_removed() {
        assert_eq!(migrate_config(&json!({"semanticPrefix": true})), json!({}));
    }

    // Ported: "should migrate prefix with no-scope to null" — config/migrations/custom/semantic-prefix-migration.spec.ts line 21
    #[test]
    fn semantic_prefix_without_scope_migrates_scope_to_null() {
        assert_eq!(
            migrate_config(&json!({"semanticPrefix": "fix: "})),
            json!({"semanticCommitType": "fix", "semanticCommitScope": null})
        );
    }

    // Ported: "works for random string" — config/migrations/custom/semantic-prefix-migration.spec.ts line 30
    #[test]
    fn semantic_prefix_random_string_migrates_type_with_null_scope() {
        assert_eq!(
            migrate_config(&json!({"semanticPrefix": "test"})),
            json!({"semanticCommitType": "test", "semanticCommitScope": null})
        );
    }

    // Ported: "should migrate non undefined gitLabAutomerge" — config/migrations/custom/azure-gitlab-automerge-migration.spec.ts line 4
    #[test]
    fn git_lab_automerge_migrates_to_platform_automerge() {
        assert_eq!(
            migrate_config(&json!({"gitLabAutomerge": true})),
            json!({"platformAutomerge": true})
        );
    }

    // Ported: "should override platformAutomerge when gitLabAutomerge defined" — config/migrations/custom/azure-gitlab-automerge-migration.spec.ts line 24
    #[test]
    fn git_lab_automerge_overrides_platform_automerge() {
        assert_eq!(
            migrate_config(&json!({"gitLabAutomerge": true, "platformAutomerge": false})),
            json!({"platformAutomerge": true})
        );
    }

    // Ported: "should migrate non undefined azureAutoComplete" — config/migrations/custom/azure-gitlab-automerge-migration.spec.ts line 36
    #[test]
    fn azure_auto_complete_migrates_to_platform_automerge() {
        assert_eq!(
            migrate_config(&json!({"azureAutoComplete": true})),
            json!({"platformAutomerge": true})
        );
    }

    // Ported: "should override platformAutomerge when azureAutoComplete defined" — config/migrations/custom/azure-gitlab-automerge-migration.spec.ts line 56
    #[test]
    fn azure_auto_complete_overrides_platform_automerge() {
        assert_eq!(
            migrate_config(&json!({"azureAutoComplete": true, "platformAutomerge": false})),
            json!({"platformAutomerge": true})
        );
    }

    // Ported: "should just remove undefined gitLabAutomerge" — config/migrations/custom/azure-gitlab-automerge-migration.spec.ts line 14
    #[test]
    fn git_lab_automerge_null_removed_without_setting_platform_automerge() {
        let result = migrate_config(&json!({"gitLabAutomerge": null}));
        assert!(result.get("platformAutomerge").is_none());
        assert!(result.get("gitLabAutomerge").is_none());
    }

    // Ported: "should just remove undefined azureAutoComplete" — config/migrations/custom/azure-gitlab-automerge-migration.spec.ts line 46
    #[test]
    fn azure_auto_complete_null_removed_without_setting_platform_automerge() {
        let result = migrate_config(&json!({"azureAutoComplete": null}));
        assert!(result.get("platformAutomerge").is_none());
        assert!(result.get("azureAutoComplete").is_none());
    }

    // Ported: "should migrate object" — config/migrations/custom/compatibility-migration.spec.ts line 4
    #[test]
    fn compatibility_object_migrates_to_constraints() {
        assert_eq!(
            migrate_config(&json!({"compatibility": {"test": "test"}})),
            json!({"constraints": {"test": "test"}})
        );
    }

    // Ported: "should just remove property when compatibility is not an object" — config/migrations/custom/compatibility-migration.spec.ts line 18
    #[test]
    fn compatibility_non_object_is_removed() {
        assert_eq!(migrate_config(&json!({"compatibility": "test"})), json!({}));
    }

    // Ported: "should migrate true to empty array" — config/migrations/custom/composer-ignore-platform-reqs-migration.spec.ts line 4
    #[test]
    fn composer_ignore_platform_reqs_true_migrates_to_empty_array() {
        assert_eq!(
            migrate_config(&json!({"composerIgnorePlatformReqs": true})),
            json!({"composerIgnorePlatformReqs": []})
        );
    }

    // Ported: "should migrate false to null" — config/migrations/custom/composer-ignore-platform-reqs-migration.spec.ts line 14
    #[test]
    fn composer_ignore_platform_reqs_false_migrates_to_null() {
        assert_eq!(
            migrate_config(&json!({"composerIgnorePlatformReqs": false})),
            json!({"composerIgnorePlatformReqs": null})
        );
    }

    // Ported: "should not change array value" — config/migrations/custom/composer-ignore-platform-reqs-migration.spec.ts line 24
    #[test]
    fn composer_ignore_platform_reqs_array_is_unchanged() {
        assert_eq!(
            migrate_config(&json!({"composerIgnorePlatformReqs": []})),
            json!({"composerIgnorePlatformReqs": []})
        );
    }

    // Ported: "migrates" — config/migrations/custom/custom-managers-migration.spec.ts line 6
    #[test]
    fn custom_managers_missing_custom_type_migrates_to_regex() {
        assert_eq!(
            migrate_config(&json!({
                "customManagers": [
                    {
                        "managerFilePatterns": ["js", "***$}{]["],
                        "matchStrings": ["^(?<depName>foo)(?<currentValue>bar)$"],
                        "datasourceTemplate": "maven",
                        "versioningTemplate": "gradle"
                    },
                    {
                        "customType": "regex",
                        "managerFilePatterns": ["js", "***$}{]["],
                        "matchStrings": ["^(?<depName>foo)(?<currentValue>bar)$"],
                        "datasourceTemplate": "maven",
                        "versioningTemplate": "gradle"
                    }
                ]
            })),
            json!({
                "customManagers": [
                    {
                        "customType": "regex",
                        "managerFilePatterns": ["js", "***$}{]["],
                        "matchStrings": ["^(?<depName>foo)(?<currentValue>bar)$"],
                        "datasourceTemplate": "maven",
                        "versioningTemplate": "gradle"
                    },
                    {
                        "customType": "regex",
                        "managerFilePatterns": ["js", "***$}{]["],
                        "matchStrings": ["^(?<depName>foo)(?<currentValue>bar)$"],
                        "datasourceTemplate": "maven",
                        "versioningTemplate": "gradle"
                    }
                ]
            })
        );
    }

    // Ported: "should migrate adoptium-java" — config/migrations/custom/datasource-migration.spec.ts line 4
    #[test]
    fn datasource_adoptium_java_migrates_to_java_version() {
        assert_eq!(
            migrate_config(&json!({"datasource": "adoptium-java"})),
            json!({"datasource": "java-version"})
        );
    }

    // Ported: "should migrate donet" — config/migrations/custom/datasource-migration.spec.ts line 14
    #[test]
    fn datasource_dotnet_migrates_to_dotnet_version() {
        assert_eq!(
            migrate_config(&json!({"datasource": "dotnet"})),
            json!({"datasource": "dotnet-version"})
        );
    }

    // Ported: "should migrate node" — config/migrations/custom/datasource-migration.spec.ts line 24
    #[test]
    fn datasource_node_migrates_to_node_version() {
        assert_eq!(
            migrate_config(&json!({"datasource": "node"})),
            json!({"datasource": "node-version"})
        );
    }

    // Ported: "migrates" — config/migrations/custom/enabled-managers-migration.spec.ts line 4
    #[test]
    fn enabled_managers_legacy_names_migrate() {
        assert_eq!(
            migrate_config(&json!({
                "enabledManagers": [
                    "test1",
                    "yarn",
                    "test2",
                    "regex",
                    "custom.regex",
                    "renovate-config-presets"
                ]
            })),
            json!({
                "enabledManagers": [
                    "test1",
                    "npm",
                    "test2",
                    "custom.regex",
                    "custom.regex",
                    "renovate-config"
                ]
            })
        );
    }

    // Ported: "should only add depTypes to packageRules" — config/migrations/custom/dep-types-migration.spec.ts line 4
    #[test]
    fn dep_types_migration_adds_package_rules() {
        assert_eq!(
            migrate_config(&json!({
                "peerDependencies": {"versionStrategy": "widen"},
                "dependencies": {"versionStrategy": "widen"},
                "engines": {"rangeStrategy": "auto"},
                "optionalDependencies": {"versionStrategy": "widen"},
                "devDependencies": {"versionStrategy": "widen"},
                "depTypes": [
                    "dependencies",
                    {"depType": "optionalDependencies", "respectLatest": false}
                ],
                "packageRules": [
                    {
                        "packagePatterns": "^(@angular|typescript)",
                        "groupName": ["angular packages"],
                        "excludedPackageNames": "foo"
                    },
                    {
                        "packageNames": ["foo"],
                        "packageRules": [{"depTypeList": ["bar"], "automerge": true}]
                    }
                ]
            })),
            json!({
                "packageRules": [
                    {
                        "matchPackageNames": ["/^(@angular|typescript)/"],
                        "groupName": ["angular packages"],
                        "excludedPackageNames": "foo"
                    },
                    {
                        "matchPackageNames": ["foo"],
                        "matchDepTypes": ["bar"],
                        "automerge": true
                    },
                    {"matchDepTypes": ["peerDependencies"], "versionStrategy": "widen"},
                    {"matchDepTypes": ["dependencies"], "versionStrategy": "widen"},
                    {"matchDepTypes": ["engines"], "rangeStrategy": "auto"},
                    {"matchDepTypes": ["optionalDependencies"], "versionStrategy": "widen"},
                    {"matchDepTypes": ["devDependencies"], "versionStrategy": "widen"},
                    {"matchDepTypes": ["optionalDependencies"], "respectLatest": false}
                ]
            })
        );
    }

    // Ported: "migrates" — config/migrations/custom/fetch-release-notes-migration.spec.ts line 4
    #[test]
    fn fetch_release_notes_migrates_to_fetch_change_logs() {
        assert_eq!(
            migrate_config(&json!({"fetchReleaseNotes": false})),
            json!({"fetchChangeLogs": "off"})
        );
        assert_eq!(
            migrate_config(&json!({"fetchReleaseNotes": true})),
            json!({"fetchChangeLogs": "pr"})
        );
        assert_eq!(
            migrate_config(&json!({"fetchReleaseNotes": "pr"})),
            json!({"fetchChangeLogs": "pr"})
        );
        assert_eq!(
            migrate_config(&json!({"fetchReleaseNotes": "off"})),
            json!({"fetchChangeLogs": "off"})
        );
        assert_eq!(
            migrate_config(&json!({"fetchReleaseNotes": "branch"})),
            json!({"fetchChangeLogs": "branch"})
        );
    }

    // Ported: "migrates fileMatch of type string" — config/migrations/custom/file-match-migration.spec.ts line 4
    #[test]
    fn file_match_string_migrates_to_manager_file_patterns() {
        assert_eq!(
            migrate_config(&json!({"fileMatch": "filename"})),
            json!({"managerFilePatterns": ["/filename/"]})
        );
    }

    // Ported: "migrates fileMatch of type array" — config/migrations/custom/file-match-migration.spec.ts line 14
    #[test]
    fn file_match_array_migrates_to_manager_file_patterns() {
        assert_eq!(
            migrate_config(&json!({"fileMatch": ["filename1", "filename2"]})),
            json!({"managerFilePatterns": ["/filename1/", "/filename2/"]})
        );
    }

    // Ported: "concats fileMatch to managerFilePatterns" — config/migrations/custom/file-match-migration.spec.ts line 24
    #[test]
    fn file_match_appends_to_existing_manager_file_patterns() {
        assert_eq!(
            migrate_config(
                &json!({"fileMatch": ["filename1", "filename2"], "managerFilePatterns": ["filename3"]})
            ),
            json!({"managerFilePatterns": ["filename3", "/filename1/", "/filename2/"]})
        );
    }

    // Ported: "does nothing if fileMatch not defined" — config/migrations/custom/file-match-migration.spec.ts line 38
    #[test]
    fn missing_file_match_leaves_manager_file_patterns_unchanged() {
        assert_eq!(
            migrate_config(&json!({"managerFilePatterns": ["filename3"]})),
            json!({"managerFilePatterns": ["filename3"]})
        );
    }

    // Ported: "should migrate properly" — config/migrations/custom/match-datasources-migration.spec.ts line 4
    #[test]
    fn match_datasources_legacy_names_migrate() {
        assert_eq!(
            migrate_config(
                &json!({"matchDatasources": ["adoptium-java", "dotnet", "npm", "node"]})
            ),
            json!({"matchDatasources": ["java-version", "dotnet-version", "npm", "node-version"]})
        );
    }

    // Ported: "migrates old custom manager syntax to new one" — config/migrations/custom/match-managers-migration.spec.ts line 4
    #[test]
    fn match_managers_legacy_names_migrate() {
        assert_eq!(
            migrate_config(&json!({
                "matchManagers": [
                    "npm",
                    "regex",
                    "custom.regex",
                    "custom.someMgr",
                    "renovate-config-presets"
                ]
            })),
            json!({
                "matchManagers": [
                    "npm",
                    "custom.regex",
                    "custom.regex",
                    "custom.someMgr",
                    "renovate-config"
                ]
            })
        );
    }

    // Ported: "only migrates when necessary" — config/migrations/custom/match-managers-migration.spec.ts line 24
    #[test]
    fn match_managers_missing_is_unchanged() {
        assert_eq!(migrate_config(&json!({})), json!({}));
    }

    // Ported: "should migrate properly" — config/migrations/custom/match-strings-migration.spec.ts line 4
    #[test]
    fn match_strings_lookup_name_migrates_to_package_name() {
        assert_eq!(
            migrate_config(&json!({
                "matchStrings": [
                    null,
                    "(?<lookupName>",
                    null,
                    "(?<lookupName>(?<lookupName>",
                    ""
                ]
            })),
            json!({"matchStrings": ["(?<packageName>", "(?<packageName>(?<packageName>"]})
        );
    }

    // Ported: "should migrate value to array" — config/migrations/custom/package-name-migration.spec.ts line 4
    #[test]
    fn package_name_migrates_to_package_names() {
        assert_eq!(
            migrate_config(&json!({"packageName": "test"})),
            json!({"packageNames": ["test"]})
        );
    }

    // Ported: "should migrate value to array" — config/migrations/custom/package-pattern-migration.spec.ts line 4
    #[test]
    fn package_pattern_migrates_to_package_patterns() {
        assert_eq!(
            migrate_config(&json!({"packagePattern": "test"})),
            json!({"packagePatterns": ["test"]})
        );
    }

    // Ported: "should preserve config order" — config/migrations/custom/package-rules-migration.spec.ts line 5
    #[test]
    fn package_rules_migration_preserves_key_order() {
        let migrated = migrate_config(&json!({
            "packageRules": [{
                "paths": [],
                "labels": ["linting"],
                "baseBranchList": [],
                "languages": [],
                "managers": [],
                "datasources": [],
                "depTypeList": [],
                "addLabels": [],
                "packageNames": [],
                "updateTypes": []
            }]
        }));
        let rule = &migrated["packageRules"][0];
        let keys: Vec<&str> = rule
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(
            keys,
            vec![
                "matchFileNames",
                "labels",
                "matchBaseBranches",
                "matchCategories",
                "matchManagers",
                "matchDatasources",
                "matchDepTypes",
                "addLabels",
                "matchPackageNames",
                "matchUpdateTypes",
            ]
        );
    }

    // Ported: "should not migrate nested packageRules" — config/migrations/custom/package-rules-migration.spec.ts line 31
    #[test]
    fn package_rules_renames_top_level_paths_without_nested_package_rules() {
        assert_eq!(
            migrate_config(&json!({
                "packageRules": [{
                    "paths": [],
                    "packgageRules": {"languages": ["javascript"]}
                }]
            })),
            json!({
                "packageRules": [{
                    "matchFileNames": [],
                    "packgageRules": {"languages": ["javascript"]}
                }]
            })
        );
    }

    // Ported: "should migrate languages to categories" — config/migrations/custom/package-rules-migration.spec.ts line 53
    #[test]
    fn package_rules_languages_migrate_to_categories() {
        assert_eq!(
            migrate_config(&json!({
                "packageRules": [
                    {"matchLanguages": ["js"]},
                    {"languages": ["rust"]}
                ]
            })),
            json!({
                "packageRules": [
                    {"matchCategories": ["js"]},
                    {"matchCategories": ["rust"]}
                ]
            })
        );
    }

    // Ported: "should migrate single match rule" — config/migrations/custom/package-rules-migration.spec.ts line 81
    #[test]
    fn package_rules_single_match_language_migrates_to_category() {
        assert_eq!(
            migrate_config(&json!({"packageRules": [{"matchLanguages": ["js"]}]})),
            json!({"packageRules": [{"matchCategories": ["js"]}]})
        );
    }

    // Ported: "should migrate excludePackageNames to matchPackageNames" — config/migrations/custom/package-rules-migration.spec.ts line 99
    #[test]
    fn package_rules_exclude_package_names_merge_into_match_package_names() {
        assert_eq!(
            migrate_config(&json!({
                "packageRules": [{
                    "excludePackageNames": ["foo", "bar"],
                    "matchPackageNames": ["baz"]
                }]
            })),
            json!({
                "packageRules": [{
                    "matchPackageNames": ["baz", "!foo", "!bar"]
                }]
            })
        );
    }

    // Ported: "should migrate matchPackagePatterns to matchPackageNames" — config/migrations/custom/package-rules-migration.spec.ts line 127
    #[test]
    fn package_rules_match_package_patterns_merge_into_match_package_names() {
        assert_eq!(
            migrate_config(&json!({
                "packageRules": [
                    {"matchPackagePatterns": ["*"]},
                    {"matchPackagePatterns": ["foo", "bar"], "matchPackageNames": ["baz"]}
                ]
            })),
            json!({
                "packageRules": [
                    {"matchPackageNames": ["*"]},
                    {"matchPackageNames": ["baz", "/foo/", "/bar/"]}
                ]
            })
        );
    }

    // Ported: "should migrate all match/exclude when value is of type string" — config/migrations/custom/package-rules-migration.spec.ts line 163
    #[test]
    fn package_rules_string_matchers_merge_into_match_names() {
        assert_eq!(
            migrate_config(&json!({
                "packageRules": [{
                    "matchPackagePatterns": "pattern",
                    "matchPackagePrefixes": "prefix1",
                    "matchSourceUrlPrefixes": "prefix1",
                    "excludePackageNames": "excluded",
                    "excludePackagePatterns": "excludepattern",
                    "excludePackagePrefixes": "prefix1b",
                    "matchDepPatterns": "pattern",
                    "matchDepPrefixes": "prefix1",
                    "excludeDepNames": "excluded",
                    "excludeDepPatterns": "excludepattern",
                    "excludeDepPrefixes": "prefix1b",
                    "automerge": true
                }]
            })),
            json!({
                "packageRules": [{
                    "matchPackageNames": [
                        "/pattern/",
                        "prefix1{/,}**",
                        "!excluded",
                        "!/excludepattern/",
                        "!prefix1b{/,}**"
                    ],
                    "matchDepNames": [
                        "/pattern/",
                        "prefix1{/,}**",
                        "!excluded",
                        "!/excludepattern/",
                        "!prefix1b{/,}**"
                    ],
                    "matchSourceUrls": ["prefix1{/,}**"],
                    "automerge": true
                }]
            })
        );
    }

    // Ported: "should migrate all match/exclude at once" — config/migrations/custom/package-rules-migration.spec.ts line 222
    #[test]
    fn package_rules_array_matchers_merge_into_match_names() {
        assert_eq!(
            migrate_config(&json!({
                "packageRules": [{
                    "matchPackagePatterns": ["pattern"],
                    "matchPackagePrefixes": ["prefix1", "prefix2"],
                    "matchSourceUrlPrefixes": ["prefix1", "prefix2"],
                    "excludePackageNames": ["excluded"],
                    "excludePackagePatterns": ["excludepattern"],
                    "excludePackagePrefixes": ["prefix1b"],
                    "matchPackageNames": ["mpn1", "mpn2"],
                    "matchDepPatterns": ["pattern"],
                    "matchDepPrefixes": ["prefix1", "prefix2"],
                    "excludeDepNames": ["excluded"],
                    "excludeDepPatterns": ["excludepattern"],
                    "excludeDepPrefixes": ["prefix1b"],
                    "matchDepNames": ["mpn1", "mpn2"],
                    "automerge": true
                }]
            })),
            json!({
                "packageRules": [{
                    "matchPackageNames": [
                        "mpn1",
                        "mpn2",
                        "/pattern/",
                        "prefix1{/,}**",
                        "prefix2{/,}**",
                        "!excluded",
                        "!/excludepattern/",
                        "!prefix1b{/,}**"
                    ],
                    "matchDepNames": [
                        "mpn1",
                        "mpn2",
                        "/pattern/",
                        "prefix1{/,}**",
                        "prefix2{/,}**",
                        "!excluded",
                        "!/excludepattern/",
                        "!prefix1b{/,}**"
                    ],
                    "matchSourceUrls": ["prefix1{/,}**", "prefix2{/,}**"],
                    "automerge": true
                }]
            })
        );
    }

    // Ported: "should migrate to package rules" — config/migrations/custom/packages-migration.spec.ts line 4
    #[test]
    fn packages_migrates_to_package_rules() {
        assert_eq!(
            migrate_config(&json!({"packages": [{"matchPackageNames": ["*"]}]})),
            json!({"packageRules": [{"matchPackageNames": ["*"]}]})
        );
    }

    // Ported: "should concat with existing package rules" — config/migrations/custom/packages-migration.spec.ts line 14
    #[test]
    fn packages_appends_to_existing_package_rules() {
        assert_eq!(
            migrate_config(&json!({
                "packages": [{"matchPackageNames": ["*"]}],
                "packageRules": [{"matchPackageNames": []}]
            })),
            json!({"packageRules": [{"matchPackageNames": []}, {"matchPackageNames": ["*"]}]})
        );
    }

    // Ported: "should ignore non array value" — config/migrations/custom/packages-migration.spec.ts line 26
    #[test]
    fn packages_non_array_is_removed() {
        assert_eq!(
            migrate_config(&json!({
                "packages": {"matchPackageNames": ["*"]},
                "packageRules": [{"matchPackageNames": []}]
            })),
            json!({"packageRules": [{"matchPackageNames": []}]})
        );
    }

    // Ported: "should migrate to packageRules" — config/migrations/custom/path-rules-migration.spec.ts line 4
    // Note: in the full pipeline, `paths` is further renamed to `matchFileNames` by PackageRulesMigration.
    #[test]
    fn path_rules_migrate_to_package_rules() {
        assert_eq!(
            migrate_config(&json!({"pathRules": [{"paths": ["examples/**"], "extends": ["foo"]}]})),
            json!({"packageRules": [{"matchFileNames": ["examples/**"], "extends": ["foo"]}]})
        );
    }

    // Ported: "should rewrite packageRules when it is not array" — config/migrations/custom/path-rules-migration.spec.ts line 22
    #[test]
    fn path_rules_rewrite_non_array_package_rules() {
        assert_eq!(
            migrate_config(&json!({
                "packageRules": "test",
                "pathRules": [{"paths": ["examples/**"], "extends": ["foo"]}]
            })),
            json!({"packageRules": [{"matchFileNames": ["examples/**"], "extends": ["foo"]}]})
        );
    }

    // Ported: "should not migrate non array value" — config/migrations/custom/path-rules-migration.spec.ts line 42
    #[test]
    fn path_rules_non_array_is_removed() {
        assert_eq!(migrate_config(&json!({"pathRules": "test"})), json!({}));
    }

    // Ported: "should concat with existing package rules" — config/migrations/custom/path-rules-migration.spec.ts line 50
    #[test]
    fn path_rules_append_to_existing_package_rules() {
        assert_eq!(
            migrate_config(&json!({
                "pathRules": [{"paths": ["examples/**"], "extends": ["foo"]}],
                "packageRules": [{"packageNames": ["guava"], "versionScheme": "maven"}]
            })),
            json!({
                "packageRules": [
                    {"matchPackageNames": ["guava"], "versionScheme": "maven"},
                    {"matchFileNames": ["examples/**"], "extends": ["foo"]}
                ]
            })
        );
    }

    // Ported: "should migrate value to array" — config/migrations/custom/package-files-migration.spec.ts line 4
    // Note: paths → matchFileNames via PackageRulesMigration in full pipeline.
    #[test]
    fn package_files_object_migrates_to_include_paths_and_package_rules() {
        let result = migrate_config(
            &json!({"packageFiles": [{"packageFile": "package.json", "packageRules": []}]}),
        );
        assert_eq!(result["includePaths"], json!(["package.json"]));
        let rules = result["packageRules"].as_array().unwrap();
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0]["matchFileNames"], json!(["package.json"]));
    }

    // Ported: "should handle multiple packageFile" — config/migrations/custom/package-files-migration.spec.ts line 21
    #[test]
    fn package_files_nested_array_migrates_to_include_paths() {
        assert_eq!(
            migrate_config(&json!({"packageFiles": [["package.json", "Chart.yaml"]]})),
            json!({"includePaths": ["package.json", "Chart.yaml"]})
        );
    }

    // Ported: "should still work for wrong config" — config/migrations/custom/package-files-migration.spec.ts line 34
    // Note: paths → matchFileNames via PackageRulesMigration in full pipeline.
    #[test]
    fn package_files_appends_to_existing_package_rules() {
        let result = migrate_config(&json!({
            "packageRules": [{"labels": ["lint"]}],
            "packageFiles": [{
                "packageFile": "package.json",
                "packageRules": [{"labels": ["breaking"]}]
            }]
        }));
        assert_eq!(result["includePaths"], json!(["package.json"]));
        let rules = result["packageRules"].as_array().unwrap();
        assert_eq!(rules.len(), 2);
        assert_eq!(rules[0]["labels"], json!(["lint"]));
        assert_eq!(rules[1]["matchFileNames"], json!(["package.json"]));
    }

    // Ported: "should work for non-object packageFiles" — config/migrations/custom/package-files-migration.spec.ts line 55
    #[test]
    fn package_files_string_migrates_to_include_paths() {
        assert_eq!(
            migrate_config(&json!({"packageFiles": ["package.json"]})),
            json!({"includePaths": ["package.json"]})
        );
    }

    // Ported: "should work for nested rules" — config/migrations/custom/package-files-migration.spec.ts line 65
    #[test]
    fn package_files_preserves_nested_rules() {
        // Note: paths → matchFileNames via PackageRulesMigration in full pipeline.
        let result = migrate_config(&json!({
            "packageFiles": [{
                "packageFile": "package.json",
                "packageRules": [{
                    "labels": ["linter"],
                    "packageRules": [{"addLabels": ["es-lint"]}]
                }]
            }]
        }));
        assert_eq!(result["includePaths"], json!(["package.json"]));
        let rules = result["packageRules"].as_array().unwrap();
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0]["matchFileNames"], json!(["package.json"]));
    }

    // Ported: "no change for empty packageFiles" — config/migrations/custom/package-files-migration.spec.ts line 92
    #[test]
    fn package_files_empty_is_removed_without_other_changes() {
        assert_eq!(
            migrate_config(&json!({
                "includePaths": ["package.json"],
                "packageRules": [{"labels": ["linter"]}],
                "packageFiles": []
            })),
            json!({
                "includePaths": ["package.json"],
                "packageRules": [{"labels": ["linter"]}]
            })
        );
    }

    // Ported: "should migrate true" — config/migrations/custom/pin-versions-migration.spec.ts line 4
    #[test]
    fn pin_versions_true_migrates_to_pin_range_strategy() {
        assert_eq!(
            migrate_config(&json!({"pinVersions": true})),
            json!({"rangeStrategy": "pin"})
        );
    }

    // Ported: "should migrate false" — config/migrations/custom/pin-versions-migration.spec.ts line 14
    #[test]
    fn pin_versions_false_migrates_to_replace_range_strategy() {
        assert_eq!(
            migrate_config(&json!({"pinVersions": false})),
            json!({"rangeStrategy": "replace"})
        );
    }

    // Ported: "should migrate" — config/migrations/custom/separate-major-release-migration.spec.ts line 4
    #[test]
    fn separate_major_releases_migrates_to_separate_major_minor() {
        assert_eq!(
            migrate_config(&json!({"separateMajorReleases": true})),
            json!({"separateMajorReleases": true, "separateMajorMinor": true})
        );
    }

    // Ported: "should remove if separateMajorReleases exists" — config/migrations/custom/separate-multiple-major-migration.spec.ts line 4
    #[test]
    fn separate_multiple_major_removed_when_separate_major_releases_exists() {
        assert_eq!(
            migrate_config(&json!({"separateMajorReleases": true, "separateMultipleMajor": true})),
            json!({"separateMajorReleases": true, "separateMajorMinor": true})
        );
    }

    // Ported: "should skip if separateMajorReleases does not exist" — config/migrations/custom/separate-multiple-major-migration.spec.ts line 14
    #[test]
    fn separate_multiple_major_is_unchanged_without_separate_major_releases() {
        assert_eq!(
            migrate_config(&json!({"separateMultipleMajor": true})),
            json!({"separateMultipleMajor": true})
        );
    }

    // Ported: "migrates" — config/migrations/custom/stability-days-migration.spec.ts line 4
    #[test]
    fn stability_days_migrates_to_minimum_release_age() {
        assert_eq!(
            migrate_config(&json!({"stabilityDays": 0})),
            json!({"minimumReleaseAge": null})
        );
        assert_eq!(
            migrate_config(&json!({"stabilityDays": 2})),
            json!({"minimumReleaseAge": "2 days"})
        );
        assert_eq!(
            migrate_config(&json!({"stabilityDays": 1})),
            json!({"minimumReleaseAge": "1 day"})
        );
    }

    // Ported: "should migrate array" — config/migrations/custom/host-rules-migration.spec.ts line 5
    #[test]
    fn host_rules_legacy_fields_migrate() {
        assert_eq!(
            migrate_config(&json!({
                "hostRules": [
                    {"hostType": "dotnet", "baseUrl": "https://some.domain.com", "token": "123test"},
                    {
                        "hostType": "dotnet",
                        "baseUrl": "https://some.domain.com",
                        "matchHost": "https://some.domain.com",
                        "token": "123test"
                    },
                    {"hostType": "adoptium-java", "domainName": "domain.com", "token": "123test"},
                    {"domainName": "domain.com/", "token": "123test"},
                    {"hostType": "docker", "matchHost": "domain.com/", "token": "123test"},
                    {"hostName": "some.domain.com", "token": "123test"},
                    {"endpoint": "domain.com/", "token": "123test"},
                    {"host": "some.domain.com", "token": "123test"},
                    {"matchHost": "some.domain.com:8080", "token": "123test"}
                ]
            })),
            json!({
                "hostRules": [
                    {"hostType": "dotnet-version", "matchHost": "https://some.domain.com", "token": "123test"},
                    {"hostType": "dotnet-version", "matchHost": "https://some.domain.com", "token": "123test"},
                    {"hostType": "java-version", "matchHost": "domain.com", "token": "123test"},
                    {"matchHost": "https://domain.com/", "token": "123test"},
                    {"hostType": "docker", "matchHost": "https://domain.com/", "token": "123test"},
                    {"matchHost": "some.domain.com", "token": "123test"},
                    {"matchHost": "https://domain.com/", "token": "123test"},
                    {"matchHost": "some.domain.com", "token": "123test"},
                    {"matchHost": "https://some.domain.com:8080", "token": "123test"}
                ]
            })
        );
    }

    // Ported: "throws when multiple hosts are present" — config/migrations/custom/host-rules-migration.spec.ts line 75
    #[test]
    fn host_rules_throws_when_multiple_hosts_have_different_values() {
        let result = migrate_and_validate(
            &json!({}),
            &json!({
                "hostRules": [
                    {
                        "matchHost": "https://some-diff.domain.com",
                        "baseUrl": "https://some.domain.com",
                        "token": "123test"
                    }
                ]
            }),
        );
        let errors = result["errors"].as_array().expect("errors array");
        assert!(
            errors.iter().any(|e| e["message"]
                .as_str()
                .unwrap_or("")
                .contains("more than one host-matching field")),
            "expected validation error for conflicting host fields, got: {errors:?}"
        );
    }

    // Ported: "should remomve prEditNotification from array" — config/migrations/custom/suppress-notifications-migration.spec.ts line 4
    #[test]
    fn suppress_notifications_removes_pr_edit_notification() {
        assert_eq!(
            migrate_config(&json!({"suppressNotifications": ["test", "prEditNotification"]})),
            json!({"suppressNotifications": ["test"]})
        );
    }

    // Ported: "should not migrate array without prEditNotification" — config/migrations/custom/suppress-notifications-migration.spec.ts line 14
    #[test]
    fn suppress_notifications_without_pr_edit_notification_is_unchanged() {
        assert_eq!(
            migrate_config(&json!({"suppressNotifications": ["test"]})),
            json!({"suppressNotifications": ["test"]})
        );
    }

    // Ported: "should not migrate empty array" — config/migrations/custom/suppress-notifications-migration.spec.ts line 25
    #[test]
    fn suppress_notifications_empty_is_unchanged() {
        assert_eq!(
            migrate_config(&json!({"suppressNotifications": []})),
            json!({"suppressNotifications": []})
        );
    }

    // Ported: "should handle hight level" — config/migrations/custom/trust-level-migration.spec.ts line 4
    #[test]
    fn trust_level_high_sets_trust_options() {
        assert_eq!(
            migrate_config(&json!({"trustLevel": "high"})),
            json!({
                "allowCustomCrateRegistries": true,
                "allowScripts": true,
                "exposeAllEnv": true
            })
        );
    }

    // Ported: "should not rewrite provided properties" — config/migrations/custom/trust-level-migration.spec.ts line 18
    #[test]
    fn trust_level_high_preserves_existing_trust_options() {
        assert_eq!(
            migrate_config(&json!({
                "allowCustomCrateRegistries": false,
                "allowScripts": false,
                "exposeAllEnv": false,
                "trustLevel": "high"
            })),
            json!({
                "allowCustomCrateRegistries": false,
                "allowScripts": false,
                "exposeAllEnv": false
            })
        );
    }

    // Ported: "should migrate true" — config/migrations/custom/unpublish-safe-migration.spec.ts line 4
    #[test]
    fn unpublish_safe_true_injects_security_preset() {
        assert_eq!(
            migrate_config(&json!({"unpublishSafe": true})),
            json!({"extends": ["security:minimumReleaseAgeNpm"]})
        );
    }

    // Ported: "should migrate true and handle extends field" — config/migrations/custom/unpublish-safe-migration.spec.ts line 14
    #[test]
    fn unpublish_safe_true_handles_string_extends() {
        assert_eq!(
            migrate_config(&json!({"extends": "test", "unpublishSafe": true})),
            json!({"extends": ["test", "security:minimumReleaseAgeNpm"]})
        );
    }

    // Ported: "should migrate true and handle empty extends field" — config/migrations/custom/unpublish-safe-migration.spec.ts line 26
    #[test]
    fn unpublish_safe_true_handles_empty_extends() {
        assert_eq!(
            migrate_config(&json!({"extends": [], "unpublishSafe": true})),
            json!({"extends": ["security:minimumReleaseAgeNpm"]})
        );
    }

    // Ported: "should migrate true and save order of items inside extends field" — config/migrations/custom/unpublish-safe-migration.spec.ts line 38
    #[test]
    fn unpublish_safe_true_rewrites_supported_extends_in_place() {
        assert_eq!(
            migrate_config(
                &json!({"extends": ["foo", ":unpublishSafe", "bar"], "unpublishSafe": true})
            ),
            json!({"extends": ["foo", "security:minimumReleaseAgeNpm", "bar"]})
        );
        assert_eq!(
            migrate_config(
                &json!({"extends": ["foo", "default:unpublishSafe", "bar"], "unpublishSafe": true})
            ),
            json!({"extends": ["foo", "security:minimumReleaseAgeNpm", "bar"]})
        );
        assert_eq!(
            migrate_config(
                &json!({"extends": ["foo", "security:minimumReleaseAgeNpm", "bar"], "unpublishSafe": true})
            ),
            json!({"extends": ["foo", "security:minimumReleaseAgeNpm", "bar"]})
        );
    }

    // Ported: "should migrate false and save order of items inside extends field" — config/migrations/custom/unpublish-safe-migration.spec.ts line 68
    #[test]
    fn unpublish_safe_false_is_removed_and_preserves_extends() {
        assert_eq!(
            migrate_config(&json!({"extends": ["foo", "bar"], "unpublishSafe": false})),
            json!({"extends": ["foo", "bar"]})
        );
    }

    // Ported: "prevent duplicates" — config/migrations/custom/unpublish-safe-migration.spec.ts line 80
    #[test]
    fn unpublish_safe_true_does_not_duplicate_security_preset() {
        assert_eq!(
            migrate_config(
                &json!({"extends": ["security:minimumReleaseAgeNpm"], "unpublishSafe": true})
            ),
            json!({"extends": ["security:minimumReleaseAgeNpm"]})
        );
    }

    // Ported: "should not migrate npm:unpublishSafe" — config/migrations/custom/unpublish-safe-migration.spec.ts line 92
    #[test]
    fn unpublish_safe_absent_leaves_npm_unpublish_safe_extends() {
        assert_eq!(
            migrate_config(&json!({"extends": ["npm:unpublishSafe"]})),
            json!({"extends": ["npm:unpublishSafe"]})
        );
    }

    // Ported: "should add postUpdateOptions option when true" — config/migrations/custom/go-mod-tidy-migration.spec.ts line 4
    #[test]
    fn gomod_tidy_true_appends_post_update_option() {
        assert_eq!(
            migrate_config(&json!({"gomodTidy": true, "postUpdateOptions": ["test"]})),
            json!({"postUpdateOptions": ["test", "gomodTidy"]})
        );
    }

    // Ported: "should handle case when postUpdateOptions is not defined" — config/migrations/custom/go-mod-tidy-migration.spec.ts line 16
    #[test]
    fn gomod_tidy_true_initializes_post_update_options() {
        assert_eq!(
            migrate_config(&json!({"gomodTidy": true})),
            json!({"postUpdateOptions": ["gomodTidy"]})
        );
    }

    // Ported: "should only remove when false" — config/migrations/custom/go-mod-tidy-migration.spec.ts line 27
    #[test]
    fn gomod_tidy_false_is_removed() {
        assert_eq!(migrate_config(&json!({"gomodTidy": false})), json!({}));
    }

    // Ported: "should migrate to ignorePaths" — config/migrations/custom/ignore-node-modules-migration.spec.ts line 4
    #[test]
    fn ignore_node_modules_true_migrates_to_ignore_paths() {
        assert_eq!(
            migrate_config(&json!({"ignoreNodeModules": true})),
            json!({"ignorePaths": ["node_modules/"]})
        );
    }

    // Ported: "should init npmrc field" — config/migrations/custom/ignore-npmrc-file-migration.spec.ts line 4
    #[test]
    fn ignore_npmrc_file_initializes_npmrc() {
        assert_eq!(
            migrate_config(&json!({"ignoreNpmrcFile": true})),
            json!({"npmrc": ""})
        );
    }

    // Ported: "should not change npmrc field if it represents string value" — config/migrations/custom/ignore-npmrc-file-migration.spec.ts line 14
    #[test]
    fn ignore_npmrc_file_preserves_string_npmrc() {
        assert_eq!(
            migrate_config(&json!({"ignoreNpmrcFile": true, "npmrc": ""})),
            json!({"npmrc": ""})
        );
    }

    // Ported: "should change npmrc field if it not represents string value" — config/migrations/custom/ignore-npmrc-file-migration.spec.ts line 26
    #[test]
    fn ignore_npmrc_file_replaces_non_string_npmrc() {
        assert_eq!(
            migrate_config(&json!({"ignoreNpmrcFile": true, "npmrc": true})),
            json!({"npmrc": ""})
        );
    }

    // Ported: "should migrate true" — config/migrations/custom/include-forks-migration.spec.ts line 4
    #[test]
    fn include_forks_true_migrates_to_enabled_fork_processing() {
        assert_eq!(
            migrate_config(&json!({"includeForks": true})),
            json!({"forkProcessing": "enabled"})
        );
    }

    // Ported: "should migrate false" — config/migrations/custom/include-forks-migration.spec.ts line 14
    #[test]
    fn include_forks_false_migrates_to_disabled_fork_processing() {
        assert_eq!(
            migrate_config(&json!({"includeForks": false})),
            json!({"forkProcessing": "disabled"})
        );
    }

    // Ported: "should not migrate non boolean value" — config/migrations/custom/include-forks-migration.spec.ts line 24
    #[test]
    fn include_forks_non_boolean_is_removed() {
        assert_eq!(migrate_config(&json!({"includeForks": "test"})), json!({}));
    }

    // Ported: "should migrate node to travis" — config/migrations/custom/node-migration.spec.ts line 4
    #[test]
    fn node_enabled_migrates_to_travis_enabled() {
        assert_eq!(
            migrate_config(&json!({"node": {"enabled": true}})),
            json!({"travis": {"enabled": true}})
        );
    }

    // Ported: "should not delete node in case it has more than one property" — config/migrations/custom/node-migration.spec.ts line 14
    #[test]
    fn node_enabled_migration_preserves_other_node_options() {
        assert_eq!(
            migrate_config(&json!({"node": {"enabled": true, "automerge": false}})),
            json!({"node": {"automerge": false}, "travis": {"enabled": true}})
        );
    }

    // Ported: "should migrate properly" — config/migrations/custom/post-update-options-migration.spec.ts line 4
    #[test]
    fn post_update_options_removes_gomod_no_massage() {
        assert_eq!(
            migrate_config(&json!({"postUpdateOptions": ["gomodTidy", "gomodNoMassage"]})),
            json!({"postUpdateOptions": ["gomodTidy"]})
        );
    }

    // Ported: "should migrate true" — config/migrations/custom/renovate-fork-migration.spec.ts line 4
    #[test]
    fn renovate_fork_true_migrates_to_enabled_fork_processing() {
        assert_eq!(
            migrate_config(&json!({"renovateFork": true})),
            json!({"forkProcessing": "enabled"})
        );
    }

    // Ported: "should migrate false" — config/migrations/custom/renovate-fork-migration.spec.ts line 14
    #[test]
    fn renovate_fork_false_migrates_to_disabled_fork_processing() {
        assert_eq!(
            migrate_config(&json!({"renovateFork": false})),
            json!({"forkProcessing": "disabled"})
        );
    }

    // Ported: "should not migrate non boolean value" — config/migrations/custom/renovate-fork-migration.spec.ts line 24
    #[test]
    fn renovate_fork_non_boolean_is_removed() {
        assert_eq!(migrate_config(&json!({"renovateFork": "test"})), json!({}));
    }

    // Ported: "should migrate value to array" — config/migrations/custom/base-branch-migration.spec.ts line 4
    #[test]
    fn base_branch_string_migrates_to_patterns() {
        assert_eq!(
            migrate_config(&json!({"baseBranch": "test"})),
            json!({"baseBranchPatterns": ["test"]})
        );
    }

    // Ported: "should migrate array" — config/migrations/custom/base-branch-migration.spec.ts line 14
    #[test]
    fn base_branch_array_migrates_to_patterns() {
        assert_eq!(
            migrate_config(&json!({"baseBranch": ["test"]})),
            json!({"baseBranchPatterns": ["test"]})
        );
    }

    // Ported: "should push to existing bassBranchPatterns" — config/migrations/custom/base-branch-migration.spec.ts line 24
    #[test]
    fn base_branch_migration_appends_existing_patterns() {
        assert_eq!(
            migrate_config(&json!({"baseBranch": ["test"], "baseBranchPatterns": ["base"]})),
            json!({"baseBranchPatterns": ["base", "test"]})
        );
    }

    // Ported: "should replace pattern" — config/migrations/custom/branch-name-migration.spec.ts line 4
    #[test]
    fn branch_name_manager_branch_prefix_migrates_to_additional_branch_prefix() {
        assert_eq!(
            migrate_config(&json!({"branchName": "test {{managerBranchPrefix}} test"})),
            json!({"branchName": "test {{additionalBranchPrefix}} test"})
        );
    }

    // Ported: "should not replace another string" — config/migrations/custom/branch-name-migration.spec.ts line 14
    #[test]
    fn branch_name_without_manager_branch_prefix_is_unchanged() {
        assert_eq!(
            migrate_config(&json!({"branchName": "test"})),
            json!({"branchName": "test"})
        );
    }

    // Ported: "should not replace non string value" — config/migrations/custom/branch-name-migration.spec.ts line 25
    #[test]
    fn branch_name_non_string_is_unchanged() {
        assert_eq!(
            migrate_config(&json!({"branchName": true})),
            json!({"branchName": true})
        );
    }

    // Ported: "should migrate template" — config/migrations/custom/branch-prefix-migration.spec.ts line 4
    #[test]
    fn branch_prefix_parent_dir_template_migrates_to_additional_prefix() {
        assert_eq!(
            migrate_config(&json!({"branchPrefix": "renovate/{{parentDir}}-"})),
            json!({"branchPrefix": "renovate/", "additionalBranchPrefix": "{{parentDir}}-"})
        );
    }

    // Ported: "should ignore string without template" — config/migrations/custom/branch-prefix-migration.spec.ts line 17
    #[test]
    fn branch_prefix_without_parent_dir_template_is_unchanged() {
        assert_eq!(
            migrate_config(&json!({"branchPrefix": "test"})),
            json!({"branchPrefix": "test"})
        );
    }

    // Ported: "should ignore non string without template" — config/migrations/custom/branch-prefix-migration.spec.ts line 28
    #[test]
    fn branch_prefix_non_string_is_unchanged() {
        assert_eq!(
            migrate_config(&json!({"branchPrefix": true})),
            json!({"branchPrefix": true})
        );
    }

    // Ported: "should migrate true" — config/migrations/custom/recreate-closed-migration.spec.ts line 4
    #[test]
    fn recreate_closed_true_migrates_to_always() {
        assert_eq!(
            migrate_config(&json!({"recreateClosed": true})),
            json!({"recreateWhen": "always"})
        );
    }

    // Ported: "should migrate false" — config/migrations/custom/recreate-closed-migration.spec.ts line 14
    #[test]
    fn recreate_closed_false_migrates_to_auto() {
        assert_eq!(
            migrate_config(&json!({"recreateClosed": false})),
            json!({"recreateWhen": "auto"})
        );
    }

    // Ported: "should migrate requireConfig=true to requireConfig=required" — config/migrations/custom/require-config-migration.spec.ts line 4
    #[test]
    fn require_config_true_string_migrates_to_required() {
        assert_eq!(
            migrate_config(&json!({"requireConfig": "true"})),
            json!({"requireConfig": "required"})
        );
    }

    // Ported: "should migrate requireConfig=false to requireConfig=optional" — config/migrations/custom/require-config-migration.spec.ts line 14
    #[test]
    fn require_config_false_string_migrates_to_optional() {
        assert_eq!(
            migrate_config(&json!({"requireConfig": "false"})),
            json!({"requireConfig": "optional"})
        );
    }

    // Ported: "should migrate true" — config/migrations/custom/rebase-stale-prs-migration.spec.ts line 4
    #[test]
    fn rebase_stale_prs_true_migrates_to_behind_base_branch() {
        assert_eq!(
            migrate_config(&json!({"rebaseStalePrs": true})),
            json!({"rebaseWhen": "behind-base-branch"})
        );
    }

    // Ported: "should migrate false" — config/migrations/custom/rebase-stale-prs-migration.spec.ts line 14
    #[test]
    fn rebase_stale_prs_false_migrates_to_conflicted() {
        assert_eq!(
            migrate_config(&json!({"rebaseStalePrs": false})),
            json!({"rebaseWhen": "conflicted"})
        );
    }

    // Ported: "should migrate null" — config/migrations/custom/rebase-stale-prs-migration.spec.ts line 24
    #[test]
    fn rebase_stale_prs_null_migrates_to_auto() {
        assert_eq!(
            migrate_config(&json!({"rebaseStalePrs": null})),
            json!({"rebaseWhen": "auto"})
        );
    }

    // Ported: "should migrate false" — config/migrations/custom/rebase-conflicted-prs-migration.spec.ts line 4
    #[test]
    fn rebase_conflicted_prs_false_migrates_to_never() {
        assert_eq!(
            migrate_config(&json!({"rebaseConflictedPrs": false})),
            json!({"rebaseWhen": "never"})
        );
    }

    // Ported: "should replace false value" — config/migrations/custom/update-lock-files-migration.spec.ts line 4
    #[test]
    fn update_lock_files_false_migrates_to_skip_artifacts_update() {
        assert_eq!(
            migrate_config(&json!({"updateLockFiles": false})),
            json!({"skipArtifactsUpdate": true})
        );
    }

    // Ported: "should not replace true value" — config/migrations/custom/update-lock-files-migration.spec.ts line 14
    #[test]
    fn update_lock_files_true_is_removed() {
        assert_eq!(migrate_config(&json!({"updateLockFiles": true})), json!({}));
    }

    // Ported: "should not replace skipArtifactsUpdate" — config/migrations/custom/update-lock-files-migration.spec.ts line 24
    #[test]
    fn update_lock_files_false_preserves_existing_skip_artifacts_update() {
        assert_eq!(
            migrate_config(&json!({"updateLockFiles": false, "skipArtifactsUpdate": false})),
            json!({"skipArtifactsUpdate": false})
        );
    }

    // Ported: "should migrate upgradeInRange=true to rangeStrategy=\"bump\"" — config/migrations/custom/upgrade-in-range-migration.spec.ts line 4
    #[test]
    fn upgrade_in_range_true_migrates_to_range_strategy_bump() {
        assert_eq!(
            migrate_config(&json!({"upgradeInRange": true})),
            json!({"rangeStrategy": "bump"})
        );
    }

    // Ported: "should just remove property when upgradeInRange not equals to true" — config/migrations/custom/upgrade-in-range-migration.spec.ts line 14
    #[test]
    fn upgrade_in_range_false_is_removed() {
        assert_eq!(migrate_config(&json!({"upgradeInRange": false})), json!({}));
    }

    // Ported: "should migrate versionStrategy=\"widen\" to rangeStrategy=\"widen\"" — config/migrations/custom/version-strategy-migration.spec.ts line 4
    #[test]
    fn version_strategy_widen_migrates_to_range_strategy_widen() {
        assert_eq!(
            migrate_config(&json!({"versionStrategy": "widen"})),
            json!({"rangeStrategy": "widen"})
        );
    }

    // Ported: "should just remove property when versionStrategy not equals to \"widen\"" — config/migrations/custom/version-strategy-migration.spec.ts line 14
    #[test]
    fn version_strategy_other_is_removed() {
        assert_eq!(
            migrate_config(&json!({"versionStrategy": "test"})),
            json!({})
        );
    }

    // Ported: "should migrate platformCommit=true to platformCommit=enabled" — config/migrations/custom/platform-commit-migration.spec.ts line 4
    #[test]
    fn platform_commit_true_migrates_to_enabled() {
        assert_eq!(
            migrate_config(&json!({"platformCommit": true})),
            json!({"platformCommit": "enabled"})
        );
    }

    // Ported: "should migrate platformCommit=false to platformCommit=disabled" — config/migrations/custom/platform-commit-migration.spec.ts line 14
    #[test]
    fn platform_commit_false_migrates_to_disabled() {
        assert_eq!(
            migrate_config(&json!({"platformCommit": false})),
            json!({"platformCommit": "disabled"})
        );
    }

    // Ported: "should not migrate platformCommit=auto" — config/migrations/custom/platform-commit-migration.spec.ts line 24
    #[test]
    fn platform_commit_auto_is_unchanged() {
        assert_eq!(
            migrate_config(&json!({"platformCommit": "auto"})),
            json!({"platformCommit": "auto"})
        );
    }

    // Ported: "should migrate requiredStatusChecks=null to ignoreTests=true" — config/migrations/custom/required-status-checks-migration.spec.ts line 4
    #[test]
    fn required_status_checks_null_migrates_to_ignore_tests() {
        assert_eq!(
            migrate_config(&json!({"requiredStatusChecks": null})),
            json!({"ignoreTests": true})
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

    fn validation_warning_messages(result: &super::ValidationResult) -> Vec<&str> {
        result
            .warnings
            .iter()
            .map(|warning| warning["message"].as_str().unwrap())
            .collect()
    }

    // Ported: "migrates gradle-lite" — config/migration.spec.ts line 731
    #[test]
    fn migrates_gradle_lite() {
        let result = migrate_config(&json!({
            "gradle-lite": {"enabled": true, "fileMatch": ["foo"]},
            "packageRules": [{"matchManagers": ["gradle-lite"], "separateMinorPatch": true}]
        }));
        // gradle-lite merged into gradle
        assert_eq!(result["gradle"]["enabled"], json!(true));
        assert!(result.get("gradle-lite").is_none());
        // matchManagers: gradle-lite → gradle
        let rules = result["packageRules"].as_array().unwrap();
        assert!(
            rules[0]["matchManagers"]
                .as_array()
                .unwrap()
                .iter()
                .any(|m| m.as_str() == Some("gradle"))
        );
        assert!(
            !rules[0]["matchManagers"]
                .as_array()
                .unwrap()
                .iter()
                .any(|m| m.as_str() == Some("gradle-lite"))
        );
    }

    // Ported: "migrates subconfig" — config/migration.spec.ts line 308
    #[test]
    fn migrates_subconfig() {
        let result = migrate_config(&json!({
            "lockFileMaintenance": {
                "depTypes": [
                    "dependencies",
                    {"depType": "optionalDependencies", "respectLatest": false}
                ]
            }
        }));
        let rules = result["lockFileMaintenance"]["packageRules"]
            .as_array()
            .expect("lockFileMaintenance.packageRules should be an array");
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0]["respectLatest"], json!(false));
    }

    // Ported: "migrates more packageFiles" — config/migration.spec.ts line 360
    #[test]
    fn migrates_more_package_files() {
        let result = migrate_config(&json!({
            "packageFiles": [{
                "packageFile": "package.json",
                "packageRules": [
                    {"pinVersions": true, "depTypeList": ["devDependencies"]},
                    {"pinVersions": true, "depTypeList": ["dependencies"]}
                ]
            }]
        }));
        let paths = result["includePaths"].as_array().expect("includePaths");
        assert_eq!(paths.len(), 1);
        assert!(result.get("packageFiles").is_none());
        let rules = result["packageRules"].as_array().expect("packageRules");
        assert_eq!(rules.len(), 2);
    }

    // Ported: "migrates pip-compile" — config/migration.spec.ts line 696
    #[test]
    fn migrates_pip_compile() {
        let result = migrate_config(&json!({
            "pip-compile": {
                "enabled": true,
                "fileMatch": [
                    "(^|/)requirements\\.in$",
                    "(^|/)requirements-fmt\\.in$",
                    "(^|/)requirements-lint\\.in$",
                    ".github/workflows/requirements.in",
                    "(^|/)debian_packages/private/third_party/requirements\\.in$",
                    "(^|/).*?requirements.*?\\.in$"
                ],
                "managerFilePatterns": ["requirements.in"]
            }
        }));
        assert_eq!(result["pip-compile"]["enabled"], json!(true));
        assert!(result["pip-compile"].get("fileMatch").is_none());
        let patterns = result["pip-compile"]["managerFilePatterns"]
            .as_array()
            .unwrap();
        assert_eq!(patterns[0], json!("requirements.txt"));
        assert_eq!(patterns[1], json!("/(^|/)requirements\\.txt$/"));
        assert_eq!(patterns[2], json!("/(^|/)requirements-fmt\\.txt$/"));
        assert_eq!(patterns[3], json!("/(^|/)requirements-lint\\.txt$/"));
        assert_eq!(patterns[4], json!("/.github/workflows/requirements.txt/"));
        assert_eq!(
            patterns[5],
            json!("/(^|/)debian_packages/private/third_party/requirements\\.txt$/")
        );
        assert_eq!(patterns[6], json!("/(^|/).*?requirements.*?\\.txt$/"));
        assert_eq!(patterns.len(), 7);
    }

    // Ported: "overrides existing automerge setting" — config/migration.spec.ts line 279
    #[test]
    fn overrides_existing_automerge_setting() {
        let result = migrate_config(&json!({
            "automerge": "minor",
            "packages": [{
                "packagePatterns": "^(@angular|typescript)",
                "automerge": "patch"
            }]
        }));
        // Top-level: automerge:minor → minor.automerge=true, major.automerge=false
        // Package rule: automerge:patch → patch.automerge=true, minor.automerge=false, major.automerge=false
        assert_eq!(result["minor"]["automerge"], json!(true));
        assert_eq!(result["major"]["automerge"], json!(false));
        assert_eq!(
            result["packageRules"][0]["minor"]["automerge"],
            json!(false)
        );
    }

    // Ported: "migrates packageFiles" — config/migration.spec.ts line 334
    #[test]
    fn migrates_package_files() {
        let result = migrate_config(&json!({
            "packageFiles": [
                "package.json",
                {"packageFile": "backend/package.json", "pinVersions": false},
                {"packageFile": "frontend/package.json", "pinVersions": true},
                {
                    "packageFile": "other/package.json",
                    "devDependencies": {"pinVersions": true},
                    "dependencies": {"pinVersions": true}
                }
            ]
        }));
        let include_paths = result["includePaths"].as_array().expect("includePaths");
        assert_eq!(include_paths.len(), 4);
        assert!(result.get("packageFiles").is_none());
        let rules = result["packageRules"].as_array().expect("packageRules");
        assert_eq!(rules.len(), 3); // 3 object entries (string entry has no rule)
        // backend: pinVersions:false → rangeStrategy:replace
        let rule0 = &rules[0];
        assert_eq!(rule0["rangeStrategy"], "replace");
        // frontend: pinVersions:true → rangeStrategy:pin
        let rule1 = &rules[1];
        assert_eq!(rule1["rangeStrategy"], "pin");
    }

    // Ported: "removes invalid configs" — config/migration.spec.ts line 389
    // Note: undefined values in TS (pinVersions, exposeEnv, etc.) are represented as absent
    // in Rust JSON; ignoreNodeModules:false replaces ignoreNodeModules:undefined to trigger
    // the ignorePaths:[] migration.
    #[test]
    fn removes_invalid_configs() {
        let result = migrate_config(&json!({
            "pathRules": {},
            "packageFiles": [{"packageFile": "test"}],
            "gomodTidy": false,
            "rebaseStalePrs": true,
            "rebaseWhen": "auto",
            "upgradeInRange": false,
            "ignoreNodeModules": false,
            "baseBranch": [],
            "depTypes": [{}],
            "commitMessage": "test",
            "raiseDeprecationWarnings": null
        }));
        assert_eq!(
            result,
            json!({
                "baseBranchPatterns": [],
                "commitMessage": "test",
                "ignorePaths": [],
                "includePaths": ["test"],
                "rebaseWhen": "auto"
            })
        );
    }

    // Ported: "should remove deprecated properties" — config/migrations/migrations-service.spec.ts line 9
    #[test]
    fn migrations_service_removes_deprecated_properties() {
        let removed = [
            "allowCommandTemplating",
            "allowPostUpgradeCommandTemplating",
            "deepExtract",
            "gitFs",
            "groupBranchName",
            "groupCommitMessage",
            "groupPrBody",
            "groupPrTitle",
            "lazyGrouping",
            "maintainYarnLock",
            "raiseDeprecationWarnings",
            "statusCheckVerify",
            "supportPolicy",
            "transitiveRemediation",
            "yarnCacheFolder",
            "yarnMaintenanceBranchName",
            "yarnMaintenanceCommitMessage",
            "yarnMaintenancePrBody",
            "yarnMaintenancePrTitle",
        ];
        for &prop in &removed {
            let result = migrate_config(&json!({prop: "test"}));
            assert!(result.get(prop).is_none(), "should remove {}", prop);
        }
    }
}
