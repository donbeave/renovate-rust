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

pub use platform::Platform;
pub use run::{DryRun, ForkProcessing, RecreateWhen, RequireConfig};

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

    // ── Run behavior ─────────────────────────────────────────────────────────
    /// Dry-run mode. `None` means dry-run is disabled (full updates).
    pub dry_run: Option<DryRun>,

    /// Controls behavior when no repository config file exists.
    /// Default: `RequireConfig::Required`.
    pub require_config: RequireConfig,

    /// Whether to process forked repositories.
    /// Default: `ForkProcessing::Auto`.
    pub fork_processing: ForkProcessing,

    // ── PR behavior ──────────────────────────────────────────────────────────
    /// Whether to use platform-native auto-merge. Default: `true`.
    pub platform_automerge: bool,

    /// When to recreate closed PRs. Default: `RecreateWhen::Auto`.
    pub recreate_when: RecreateWhen,

    // ── Post-upgrade task security ────────────────────────────────────────────
    /// Allowed post-upgrade commands. Empty list = none allowed.
    pub allowed_commands: Vec<String>,

    /// Whether command templating is allowed in post-upgrade tasks.
    pub allow_command_templating: bool,

    /// Repositories to process. Empty means "nothing to do" unless autodiscover
    /// is enabled (future slice).
    pub repositories: Vec<String>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            platform: Platform::Github,
            token: None,
            endpoint: None,
            dry_run: None,
            require_config: RequireConfig::Required,
            fork_processing: ForkProcessing::Auto,
            platform_automerge: true,
            recreate_when: RecreateWhen::Auto,
            allowed_commands: Vec::new(),
            allow_command_templating: false,
            repositories: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ConfigDefaultValue, ConfigOptionType, GLOBAL_CONFIG_OPTIONS, default_value_for_type,
    };

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
}
