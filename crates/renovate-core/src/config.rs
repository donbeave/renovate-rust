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
        }
    }
}
