//! Clap CLI struct and associated argument types.
//!
//! Each flag and env-var pair mirrors the Renovate option surface from
//! `lib/config/options/index.ts`. Only a first cut of the most important
//! global flags are wired here; the full surface will grow slice-by-slice.
//!
//! ## Legacy coercions
//!
//! Several enum variants exist purely to accept deprecated string values that
//! `migrateArgs` produces or that old Renovate callers may supply:
//! - `DryRunArg::LegacyTrue` accepts `--dry-run=true` → semantic "full"
//! - `DryRunArg::LegacyFalse` / `LegacyNull` accept the "disable" forms
//! - `RequireConfigArg::LegacyTrue` / `LegacyFalse` for `--require-config=true`
//!
//! Callers convert these to canonical values via [`DryRunArg::canonical`] /
//! [`RequireConfigArg::canonical`] before passing to the config layer.

use clap::{ArgAction, Parser, ValueEnum};

/// Platform type — the `--platform` flag.
///
/// Source: `PLATFORM_HOST_TYPES` in `lib/constants/platforms.ts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub(crate) enum Platform {
    Azure,
    Bitbucket,
    BitbucketServer,
    Codecommit,
    Forgejo,
    Gerrit,
    Gitea,
    Github,
    Gitlab,
    Local,
    ScmManager,
}

/// `--dry-run` values, including the legacy boolean strings that
/// `migrateArgs` and old CLI callers may produce.
///
/// Renovate source: `dryRun` option in `lib/config/options/index.ts`
/// and coercions in `lib/workers/global/config/parse/cli.ts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub(crate) enum DryRunArg {
    Extract,
    Lookup,
    Full,
    /// `--dry-run=true` produced by `migrateArgs` rewriting bare `--dry-run`.
    /// Semantically equivalent to `full`. Carries a deprecation warning in
    /// Renovate; we preserve the variant so callers can emit the same warning.
    #[clap(name = "true")]
    LegacyTrue,
    /// `--dry-run=false` / `--dry-run=null` — disables dry-run.
    #[clap(name = "false")]
    LegacyFalse,
    #[clap(name = "null")]
    LegacyNull,
}

/// `--require-config` values, including the legacy boolean strings.
///
/// Renovate source: `requireConfig` option and cli.ts coercions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub(crate) enum RequireConfigArg {
    Required,
    Optional,
    Ignored,
    /// `--require-config=true` → "required" (legacy). Carries a deprecation
    /// warning in Renovate.
    #[clap(name = "true")]
    LegacyTrue,
    /// `--require-config=false` → "optional" (legacy).
    #[clap(name = "false")]
    LegacyFalse,
}

/// `--fork-processing` values.
///
/// Renovate source: `forkProcessing` option.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub(crate) enum ForkProcessing {
    Auto,
    Enabled,
    Disabled,
}

/// `--recreate-when` values.
///
/// Renovate source: `recreateWhen` option.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub(crate) enum RecreateWhen {
    Auto,
    Always,
    Never,
}

/// Renovate-compatible CLI.
///
/// This struct holds only the flag definitions and their clap metadata. All
/// business logic lives in the core crates or dedicated modules. The flag set
/// grows slice-by-slice toward full Renovate parity.
#[derive(Debug, Parser)]
#[command(
    name = "renovate",
    bin_name = "renovate",
    about = "Automated dependency updates. Flexible so you don't need to be.",
    long_about = None,
    // Disable clap's built-in -V/--version so we can use Renovate's lowercase
    // -v/--version with bare-version output.
    disable_version_flag = true,
)]
pub(crate) struct Cli {
    // ── Global meta ──────────────────────────────────────────────────────────
    /// Print the version and exit.
    #[arg(short = 'v', long = "version", action = ArgAction::SetTrue, global = true)]
    pub(crate) version: bool,

    // ── Platform / auth ──────────────────────────────────────────────────────
    /// Platform type of repository. Env: RENOVATE_PLATFORM.
    #[arg(long, value_enum, env = "RENOVATE_PLATFORM")]
    pub(crate) platform: Option<Platform>,

    /// Repository auth token. Env: RENOVATE_TOKEN.
    #[arg(long, env = "RENOVATE_TOKEN")]
    pub(crate) token: Option<String>,

    /// Custom API endpoint. Env: RENOVATE_ENDPOINT.
    #[arg(long, env = "RENOVATE_ENDPOINT")]
    pub(crate) endpoint: Option<String>,

    // ── Run behavior ─────────────────────────────────────────────────────────
    /// Perform a dry run. Values: extract, lookup, full.
    /// Bare --dry-run is rewritten to --dry-run=true by migrateArgs, which
    /// maps to "full". Env: RENOVATE_DRY_RUN.
    #[arg(long, value_enum, env = "RENOVATE_DRY_RUN")]
    pub(crate) dry_run: Option<DryRunArg>,

    /// Control behavior for missing repository config files.
    /// Env: RENOVATE_REQUIRE_CONFIG.
    #[arg(long, value_enum, env = "RENOVATE_REQUIRE_CONFIG")]
    pub(crate) require_config: Option<RequireConfigArg>,

    /// Whether to process forked repositories.
    /// Env: RENOVATE_FORK_PROCESSING.
    #[arg(long, value_enum, env = "RENOVATE_FORK_PROCESSING")]
    pub(crate) fork_processing: Option<ForkProcessing>,

    // ── PR behavior ──────────────────────────────────────────────────────────
    /// Controls if platform-native auto-merge is used.
    /// Env: RENOVATE_PLATFORM_AUTOMERGE.
    #[arg(long, env = "RENOVATE_PLATFORM_AUTOMERGE")]
    pub(crate) platform_automerge: Option<bool>,

    /// When to recreate closed PRs.
    /// Env: RENOVATE_RECREATE_WHEN.
    #[arg(long, value_enum, env = "RENOVATE_RECREATE_WHEN")]
    pub(crate) recreate_when: Option<RecreateWhen>,

    // ── Post-upgrade task security ────────────────────────────────────────────
    /// Allowed post-upgrade commands (comma-separated or JSON array).
    /// Env: RENOVATE_ALLOWED_COMMANDS.
    #[arg(long, env = "RENOVATE_ALLOWED_COMMANDS")]
    pub(crate) allowed_commands: Option<String>,

    /// Allow command templating in post-upgrade tasks.
    /// Env: RENOVATE_ALLOW_COMMAND_TEMPLATING.
    #[arg(long, env = "RENOVATE_ALLOW_COMMAND_TEMPLATING")]
    pub(crate) allow_command_templating: Option<bool>,

    // ── Registry / host rules ─────────────────────────────────────────────────
    // Accepted as raw strings; full JSON5 parsing is a separate slice.
    /// Host rules (JSON array or object). Env: RENOVATE_HOST_RULES.
    #[arg(long, env = "RENOVATE_HOST_RULES")]
    pub(crate) host_rules: Option<String>,

    /// Registry aliases (JSON object). Env: RENOVATE_REGISTRY_ALIASES.
    #[arg(long, env = "RENOVATE_REGISTRY_ALIASES")]
    pub(crate) registry_aliases: Option<String>,

    // ── Repositories ─────────────────────────────────────────────────────────
    /// Repositories to process (positional). Later slices dispatch these
    /// into the worker pipeline.
    #[arg(value_name = "repositories")]
    pub(crate) repositories: Vec<String>,
}
