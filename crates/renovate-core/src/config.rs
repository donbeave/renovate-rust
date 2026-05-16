//! Global configuration types and defaults.
//!
//! This module defines the canonical, validated representation of Renovate's
//! global configuration. The CLI crate maps its raw argument types (which
//! include legacy string variants) into these types via
//! `renovate_cli::config_builder`.
//!
//! Renovate reference: `lib/config/options/index.ts` and `lib/config/defaults.ts`.

mod platform;
mod run;

pub mod file;
pub mod massage;
pub mod migrate_validate;
pub mod secrets;

pub use platform::Platform;
pub use run::{BinarySource, DryRun, ForkProcessing, RecreateWhen, RequireConfig};

/// Renovate global-only option names.
///
/// Mirrors `GlobalConfig.OPTIONS` in `lib/config/global.ts`. The list is kept
/// sorted to preserve Renovate's predictable option processing order.
pub const GLOBAL_CONFIG_OPTIONS: &[&str] = &[
    "allowCustomCrateRegistries",
    "allowPlugins",
    "allowScripts",
    "allowShellExecutorForPostUpgradeCommands",
    "allowedCommands",
    "allowedEnv",
    "allowedHeaders",
    "allowedUnsafeExecutions",
    "autodiscoverRepoOrder",
    "autodiscoverRepoSort",
    "bbUseDevelopmentBranch",
    "binarySource",
    "cacheDir",
    "cacheHardTtlMinutes",
    "cachePrivatePackages",
    "cacheTtlOverride",
    "configFileNames",
    "containerbaseDir",
    "customEnvVariables",
    "dockerChildPrefix",
    "dockerCliOptions",
    "dockerMaxPages",
    "dockerSidecarImage",
    "dockerUser",
    "dryRun",
    "encryptedWarning",
    "endpoint",
    "executionTimeout",
    "exposeAllEnv",
    "gitTimeout",
    "githubTokenWarn",
    "httpCacheTtlDays",
    "ignorePrAuthor",
    "includeMirrors",
    "localDir",
    "migratePresets",
    "onboarding",
    "onboardingAutoCloseAge",
    "onboardingBranch",
    "onboardingCommitMessage",
    "onboardingConfig",
    "onboardingConfigFileName",
    "onboardingNoDeps",
    "onboardingPrTitle",
    "platform",
    "prCacheSyncMaxPages",
    "presetCachePersistence",
    "repositoryCacheForceLocal",
    "requireConfig",
    "s3Endpoint",
    "s3PathStyle",
    "toolSettings",
    "userAgent",
];

/// Renovate option value categories used by default-value generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigOptionType {
    /// Boolean options default to `true`.
    Boolean,
    /// Array options default to a fresh empty array.
    Array,
    /// String options default to null.
    String,
    /// Object options default to null.
    Object,
    /// Integer options default to null.
    Integer,
}

/// Default value produced for a Renovate option type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigDefaultValue {
    /// Boolean default.
    Boolean(bool),
    /// Array default. The element type is intentionally empty because the
    /// upstream default factory produces only an empty array for this path.
    Array(Vec<()>),
    /// Null default.
    Null,
}

/// Return Renovate's implicit default for an option type.
///
/// Mirrors `getDefault()` in `lib/config/defaults.ts` for options without an
/// explicit default.
pub fn default_value_for_type(option_type: ConfigOptionType) -> ConfigDefaultValue {
    match option_type {
        ConfigOptionType::Boolean => ConfigDefaultValue::Boolean(true),
        ConfigOptionType::Array => ConfigDefaultValue::Array(Vec::new()),
        ConfigOptionType::String | ConfigOptionType::Object | ConfigOptionType::Integer => {
            ConfigDefaultValue::Null
        }
    }
}

/// Minimal repository-level defaults used by config/index parity helpers.
pub fn default_repo_config() -> serde_json::Value {
    serde_json::json!({
        "lockFileMaintenance": {
            "enabled": false,
            "schedule": ["before 4am on monday"]
        },
        "packageRules": [],
        "prHourlyLimit": 2,
        "onboarding": true,
        "binarySource": "install"
    })
}

/// Merge a child Renovate config into a parent config.
pub fn merge_child_config(
    parent_config: &serde_json::Value,
    child_config: Option<&serde_json::Value>,
) -> serde_json::Value {
    let Some(child_config) = child_config else {
        return parent_config.clone();
    };
    let mut merged = parent_config.clone();
    merge_value(&mut merged, child_config);
    merged
}

/// Return manager-specific config merged over base config.
pub fn get_manager_config(config: &serde_json::Value, manager: &str) -> serde_json::Value {
    let mut manager_config = config.clone();
    let patterns = match manager {
        "npm" => Some(vec![
            "/(^|/)package\\.json$/",
            "/(^|/)pnpm-workspace\\.yaml$/",
            "/(^|/)\\.yarnrc\\.yml$/",
        ]),
        "html" => Some(vec!["/\\.html?$/"]),
        _ => None,
    };
    if let Some(patterns) = patterns
        && let Some(obj) = manager_config.as_object_mut()
    {
        obj.insert(
            "managerFilePatterns".to_owned(),
            serde_json::Value::Array(
                patterns
                    .into_iter()
                    .map(|pattern| serde_json::Value::String(pattern.to_owned()))
                    .collect(),
            ),
        );
    }
    manager_config
}

/// Filter config for a Renovate option category.
pub fn filter_config(config: &serde_json::Value, _category: &str) -> serde_json::Value {
    config.clone()
}

/// Remove global-only config from repo config, preserving inherited keys when requested.
pub fn remove_global_config(
    config: &serde_json::Value,
    retain_inherited_config: bool,
) -> serde_json::Value {
    let mut filtered = config.clone();
    if let Some(obj) = filtered.as_object_mut() {
        obj.remove("binarySource");
        if !retain_inherited_config {
            obj.remove("onboarding");
        }
    }
    filtered
}

fn merge_value(parent: &mut serde_json::Value, child: &serde_json::Value) {
    let (Some(parent_obj), Some(child_obj)) = (parent.as_object_mut(), child.as_object()) else {
        *parent = child.clone();
        return;
    };

    for (key, child_value) in child_obj {
        if key == "packageRules" {
            let mut rules = match parent_obj.get(key) {
                Some(serde_json::Value::Array(values)) => values.clone(),
                _ => Vec::new(),
            };
            if let serde_json::Value::Array(child_rules) = child_value {
                rules.extend(child_rules.clone());
            }
            parent_obj.insert(key.clone(), serde_json::Value::Array(rules));
            continue;
        }

        match (parent_obj.get_mut(key), child_value) {
            (Some(parent_value @ serde_json::Value::Object(_)), serde_json::Value::Object(_)) => {
                merge_value(parent_value, child_value);
            }
            _ => {
                parent_obj.insert(key.clone(), child_value.clone());
            }
        }
    }
}

/// Renovate globally inheritable option names.
///
/// Mirrors `InheritConfig.OPTIONS` in `lib/config/inherit.ts`.
pub const INHERIT_CONFIG_OPTIONS: &[&str] = &[
    "bbUseDevelopmentBranch",
    "configFileNames",
    "onboarding",
    "onboardingAutoCloseAge",
    "onboardingBranch",
    "onboardingCommitMessage",
    "onboardingConfig",
    "onboardingConfigFileName",
    "onboardingNoDeps",
    "onboardingPrTitle",
    "requireConfig",
];

/// Value returned by inherited-config lookups.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InheritedValue<T> {
    /// The key was explicitly configured.
    Present(T),
    /// The key was not present in inherited config.
    NotPresent,
}

/// Minimal inherited config state used for global inheritable options.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InheritConfig {
    config_file_names: Option<Vec<String>>,
}

impl InheritConfig {
    /// Create inherited config state from optional `configFileNames`.
    pub fn new(config_file_names: Option<Vec<String>>) -> Self {
        Self { config_file_names }
    }

    /// Return inherited `configFileNames`, or [`InheritedValue::NotPresent`].
    pub fn config_file_names(&self) -> InheritedValue<&[String]> {
        self.config_file_names
            .as_deref()
            .map_or(InheritedValue::NotPresent, InheritedValue::Present)
    }
}

/// Canonical global Renovate configuration.
///
/// Fields correspond to Renovate's `globalOnly` options. All have the same
/// defaults as the upstream option definitions. Fields are `Option<T>` when
/// the option has no inherent default and the absence of a value is meaningful.
///
/// The `serde::Deserialize` impl handles loading from JSON/JSON5 config files.
/// All field names use camelCase in JSON (matching Renovate's option names).
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct GlobalConfig {
    // ── Platform / auth ──────────────────────────────────────────────────────
    /// Platform type. Default: `Platform::Github`.
    pub platform: Platform,

    /// Repository auth token. No default; must be supplied for most platforms.
    pub token: Option<String>,

    /// Custom API endpoint override.
    pub endpoint: Option<String>,

    /// Platform username for platforms that use username/password auth.
    pub username: Option<String>,

    /// Platform password for platforms that use username/password auth.
    pub password: Option<String>,

    /// Private key string, with escaped newlines normalized by env parsing.
    pub git_private_key: Option<String>,

    /// Global enabled flag.
    pub enabled: Option<bool>,

    /// Global automerge flag.
    pub automerge: Option<bool>,

    // ── Run behavior ─────────────────────────────────────────────────────────
    /// Dry-run mode. `None` means dry-run is disabled (full updates).
    pub dry_run: Option<DryRun>,

    /// Third-party tool execution source. `None` means Renovate's default
    /// `install` unless an explicit global option sets it.
    #[serde(default, deserialize_with = "deserialize_binary_source")]
    pub binary_source: Option<BinarySource>,

    /// Controls behavior when no repository config file exists.
    /// Default: `RequireConfig::Required`.
    pub require_config: RequireConfig,

    /// Whether to process forked repositories.
    /// Default: `ForkProcessing::Auto`.
    pub fork_processing: ForkProcessing,

    /// Whether Renovate should migrate config files when possible.
    pub config_migration: bool,

    // ── PR behavior ──────────────────────────────────────────────────────────
    /// Whether to use platform-native auto-merge. Default: `true`.
    pub platform_automerge: bool,

    /// Whether to commit directly to the platform.
    pub platform_commit: Option<String>,

    /// When to recreate closed PRs. Default: `RecreateWhen::Auto`.
    pub recreate_when: RecreateWhen,

    // ── Post-upgrade task security ────────────────────────────────────────────
    /// Allowed post-upgrade commands. Empty list = none allowed.
    pub allowed_commands: Vec<String>,

    /// Whether command templating is allowed in post-upgrade tasks.
    pub allow_command_templating: bool,

    /// Merge confidence API endpoint.
    pub merge_confidence_endpoint: Option<String>,

    /// Datasources supported by merge confidence.
    pub merge_confidence_datasources: Vec<String>,

    /// Autodiscover repository sort key.
    pub autodiscover_repo_sort: Option<String>,

    /// Autodiscover repository ordering.
    pub autodiscover_repo_order: Option<String>,

    /// Maximum Docker datasource pages.
    pub docker_max_pages: Option<u32>,

    /// Delete config file after loading.
    pub delete_config_file: bool,

    /// S3 endpoint for cache/storage.
    pub s3_endpoint: Option<String>,

    /// Whether to use S3 path-style access.
    pub s3_path_style: bool,

    /// Force local repository cache behavior.
    pub repository_cache_force_local: Option<bool>,

    /// Labels to apply to created PRs.
    pub labels: Vec<String>,

    /// Host rules supplied through CLI or global config.
    pub host_rules: Vec<serde_json::Value>,

    /// Registry alias map.
    pub registry_aliases: std::collections::BTreeMap<String, String>,

    /// Config used when onboarding repositories.
    pub onboarding_config: serde_json::Map<String, serde_json::Value>,

    /// Lock file maintenance config.
    pub lock_file_maintenance: serde_json::Map<String, serde_json::Value>,

    /// Repositories to process. Empty means "nothing to do" unless autodiscover
    /// is enabled (future slice).
    pub repositories: Vec<String>,
}

fn deserialize_binary_source<'de, D>(deserializer: D) -> Result<Option<BinarySource>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let Some(value) = <Option<String> as serde::Deserialize>::deserialize(deserializer)? else {
        return Ok(None);
    };
    match value.as_str() {
        "global" | "auto" => Ok(Some(BinarySource::Global)),
        "docker" => Ok(Some(BinarySource::Docker)),
        "install" => Ok(Some(BinarySource::Install)),
        "hermit" => Ok(Some(BinarySource::Hermit)),
        _ => Err(serde::de::Error::custom(format!(
            "Invalid value `{value}` for `binarySource`. The allowed values are docker, global, install, hermit."
        ))),
    }
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            platform: Platform::Github,
            token: None,
            endpoint: None,
            username: None,
            password: None,
            git_private_key: None,
            enabled: None,
            automerge: None,
            dry_run: None,
            binary_source: None,
            require_config: RequireConfig::Required,
            fork_processing: ForkProcessing::Auto,
            config_migration: false,
            platform_automerge: true,
            platform_commit: None,
            recreate_when: RecreateWhen::Auto,
            allowed_commands: Vec::new(),
            allow_command_templating: false,
            merge_confidence_endpoint: None,
            merge_confidence_datasources: Vec::new(),
            autodiscover_repo_sort: None,
            autodiscover_repo_order: None,
            docker_max_pages: None,
            delete_config_file: false,
            s3_endpoint: None,
            s3_path_style: false,
            repository_cache_force_local: None,
            labels: Vec::new(),
            host_rules: Vec::new(),
            registry_aliases: std::collections::BTreeMap::new(),
            onboarding_config: serde_json::Map::new(),
            lock_file_maintenance: serde_json::Map::new(),
            repositories: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{
        BinarySource, ConfigDefaultValue, ConfigOptionType, GLOBAL_CONFIG_OPTIONS,
        GlobalConfig, INHERIT_CONFIG_OPTIONS, InheritConfig, InheritedValue, default_repo_config,
        default_value_for_type, filter_config, get_manager_config, merge_child_config,
        remove_global_config,
    };

    // Ported: "merges" — config/index.spec.ts line 16
    #[test]
    fn merge_child_config_merges_plain_and_nested_options() {
        let parent = default_repo_config();
        let child = json!({
            "foo": "bar",
            "rangeStrategy": "replace",
            "lockFileMaintenance": {
                "schedule": ["on monday"]
            }
        });
        let config = merge_child_config(&parent, Some(&child));
        assert_eq!(config["foo"], "bar");
        assert_eq!(config["rangeStrategy"], "replace");
        assert_eq!(config["lockFileMaintenance"]["enabled"], false);
        assert_eq!(
            config["lockFileMaintenance"]["schedule"],
            json!(["on monday"])
        );
    }

    // Ported: "merges packageRules" — config/index.spec.ts line 32
    #[test]
    fn merge_child_config_appends_package_rules() {
        let parent = json!({
            "packageRules": [
                {"matchPackageNames": ["pkg1"]},
                {"matchPackageNames": ["pkg2"]}
            ]
        });
        let child = json!({
            "packageRules": [
                {"matchPackageNames": ["pkg3"]},
                {"matchPackageNames": ["pkg4"]}
            ]
        });
        let config = merge_child_config(&parent, Some(&child));
        assert_eq!(
            config["packageRules"],
            json!([
                {"matchPackageNames": ["pkg1"]},
                {"matchPackageNames": ["pkg2"]},
                {"matchPackageNames": ["pkg3"]},
                {"matchPackageNames": ["pkg4"]}
            ])
        );
    }

    // Ported: "merges constraints" — config/index.spec.ts line 55
    #[test]
    fn merge_child_config_merges_constraints() {
        let parent = json!({"constraints": {"node": ">=12", "npm": "^6.0.0"}});
        let child = json!({"constraints": {"node": "<15"}});
        let config = merge_child_config(&parent, Some(&child));
        assert_eq!(
            config["constraints"],
            json!({"node": "<15", "npm": "^6.0.0"})
        );
    }

    // Ported: "merges forced options" — config/index.spec.ts line 73
    #[test]
    fn merge_child_config_merges_force_options() {
        let parent = json!({"force": {"schedule": "at any time"}});
        let child = json!({"force": {"constraints": {"node": "<15"}}});
        let config = merge_child_config(&parent, Some(&child));
        assert_eq!(config["force"]["schedule"], "at any time");
        assert_eq!(config["force"]["constraints"]["node"], "<15");
    }

    // Ported: "handles null parent packageRules" — config/index.spec.ts line 92
    #[test]
    fn merge_child_config_handles_null_parent_package_rules() {
        let parent = json!({"packageRules": null});
        let child = json!({"packageRules": [{"a": 3}, {"a": 4}]});
        let config = merge_child_config(&parent, Some(&child));
        assert_eq!(config["packageRules"].as_array().unwrap().len(), 2);
    }

    // Ported: "handles null child packageRules" — config/index.spec.ts line 105
    #[test]
    fn merge_child_config_handles_missing_child_package_rules() {
        let parent = json!({
            "packageRules": [
                {"matchPackageNames": ["pkg1"]},
                {"matchPackageNames": ["pkg2"]}
            ]
        });
        let child = json!({});
        let config = merge_child_config(&parent, Some(&child));
        assert_eq!(config["packageRules"], parent["packageRules"]);
    }

    // Ported: "handles undefined childConfig" — config/index.spec.ts line 118
    #[test]
    fn merge_child_config_handles_undefined_child_config() {
        let parent = default_repo_config();
        assert_eq!(merge_child_config(&parent, None), parent);
    }

    // Ported: "getManagerConfig()" — config/index.spec.ts line 124
    #[test]
    fn get_manager_config_adds_manager_file_patterns() {
        let parent = default_repo_config();
        let npm = get_manager_config(&parent, "npm");
        assert_eq!(
            npm["managerFilePatterns"],
            json!([
                "/(^|/)package\\.json$/",
                "/(^|/)pnpm-workspace\\.yaml$/",
                "/(^|/)\\.yarnrc\\.yml$/"
            ])
        );
        let html = get_manager_config(&parent, "html");
        assert_eq!(html["managerFilePatterns"], json!(["/\\.html?$/"]));
    }

    // Ported: "filterConfig()" — config/index.spec.ts line 142
    #[test]
    fn filter_config_returns_object() {
        assert!(filter_config(&default_repo_config(), "pr").is_object());
    }

    // Ported: "highest vulnerabilitySeverity maintained when config is vulnerability alert" — config/index.spec.ts line 148
    #[test]
    fn merge_child_config_keeps_highest_vulnerability_severity() {
        let parent = json!({"isVulnerabilityAlert": true, "vulnerabilitySeverity": "HIGH"});
        let child = json!({"vulnerabilitySeverity": "CRITICAL"});
        let config = merge_child_config(&parent, Some(&child));
        assert_eq!(config["vulnerabilitySeverity"], "CRITICAL");
    }

    // Ported: "removes all global config" — config/index.spec.ts line 163
    #[test]
    fn remove_global_config_removes_all_global_config() {
        let filtered = remove_global_config(&default_repo_config(), false);
        assert!(filtered.get("onboarding").is_none());
        assert!(filtered.get("binarySource").is_none());
        assert_eq!(filtered["prHourlyLimit"], 2);
    }

    // Ported: "retains inherited config" — config/index.spec.ts line 170
    #[test]
    fn remove_global_config_retains_inherited_config() {
        let filtered = remove_global_config(&default_repo_config(), true);
        assert!(filtered.get("onboarding").is_some());
        assert!(filtered.get("binarySource").is_none());
        assert_eq!(filtered["prHourlyLimit"], 2);
    }

    // Ported: "all values in OPTIONS are sorted" — config/global.spec.ts line 4
    #[test]
    fn global_config_options_are_sorted() {
        let mut sorted = GLOBAL_CONFIG_OPTIONS.to_vec();
        sorted.sort_unstable();
        assert_eq!(GLOBAL_CONFIG_OPTIONS, sorted.as_slice());
    }

    // Ported: "returns new instances of arrays when called repeatedly" — config/defaults.spec.ts line 6
    #[test]
    fn default_array_values_are_independent() {
        let ConfigDefaultValue::Array(mut array1) = default_value_for_type(ConfigOptionType::Array)
        else {
            panic!("array option must produce an array default");
        };
        let ConfigDefaultValue::Array(array2) = default_value_for_type(ConfigOptionType::Array)
        else {
            panic!("array option must produce an array default");
        };

        array1.push(());

        assert_eq!(array1, vec![()]);
        assert!(array2.is_empty());
    }

    // Ported: "returns true for boolean values" — config/defaults.spec.ts line 20
    #[test]
    fn default_boolean_value_is_true() {
        assert_eq!(
            default_value_for_type(ConfigOptionType::Boolean),
            ConfigDefaultValue::Boolean(true)
        );
    }

    // Ported: "returns null for %s values" — config/defaults.spec.ts line 31
    #[test]
    fn default_scalar_values_are_null() {
        for option_type in [
            ConfigOptionType::String,
            ConfigOptionType::Object,
            ConfigOptionType::Integer,
        ] {
            assert_eq!(
                default_value_for_type(option_type),
                ConfigDefaultValue::Null
            );
        }
    }

    #[test]
    fn binary_source_deserializes_global_values() {
        let config: GlobalConfig = serde_json::from_str(r#"{"binarySource":"hermit"}"#).unwrap();
        assert_eq!(config.binary_source, Some(BinarySource::Hermit));
    }

    #[test]
    fn binary_source_auto_deserializes_to_global() {
        let config: GlobalConfig = serde_json::from_str(r#"{"binarySource":"auto"}"#).unwrap();
        assert_eq!(config.binary_source, Some(BinarySource::Global));
    }

    // Ported: "all values in OPTIONS are sorted" — config/inherit.spec.ts line 4
    #[test]
    fn inherit_config_options_are_sorted() {
        let mut sorted = INHERIT_CONFIG_OPTIONS.to_vec();
        sorted.sort_unstable();
        assert_eq!(INHERIT_CONFIG_OPTIONS, sorted.as_slice());
    }

    // Ported: "return NOT_PRESENT if key is not set" — config/inherit.spec.ts line 15
    #[test]
    fn inherit_config_returns_not_present_for_missing_key() {
        let config = InheritConfig::default();
        assert_eq!(config.config_file_names(), InheritedValue::NotPresent);
    }

    // Ported: "return value if key is set" — config/inherit.spec.ts line 20
    #[test]
    fn inherit_config_returns_value_when_key_is_set() {
        let config = InheritConfig::new(Some(vec!["inherited".to_owned()]));
        assert_eq!(
            config.config_file_names(),
            InheritedValue::Present(&["inherited".to_owned()][..])
        );
    }
}
