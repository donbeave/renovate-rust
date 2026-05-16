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
//! Callers convert these to canonical values in `config_builder` before
//! passing them to the core config layer.

use clap::{ArgAction, Parser, ValueEnum};

/// Report output format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub(crate) enum OutputFormat {
    /// Colored, human-readable text (default).
    Human,
    /// Machine-readable JSON array of repository update reports.
    Json,
}

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

/// `--binary-source` values.
///
/// Renovate source: `binarySource` option.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub(crate) enum BinarySource {
    Global,
    Docker,
    Install,
    Hermit,
    /// Deprecated value migrated by Renovate config migration to `global`.
    Auto,
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

/// `--platform-commit` values.
///
/// Renovate source: `platformCommit` option.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub(crate) enum PlatformCommit {
    Auto,
    Disabled,
    Enabled,
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

    /// Username for platform authentication. Env: RENOVATE_USERNAME.
    #[arg(long, env = "RENOVATE_USERNAME")]
    pub(crate) username: Option<String>,

    /// Password for platform authentication. Env: RENOVATE_PASSWORD.
    #[arg(long, env = "RENOVATE_PASSWORD")]
    pub(crate) password: Option<String>,

    /// HTTP user-agent override.
    /// Env: RENOVATE_USER_AGENT.
    #[arg(long, env = "RENOVATE_USER_AGENT")]
    pub(crate) user_agent: Option<String>,

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

    /// Controls how third-party tools are invoked.
    /// Env: RENOVATE_BINARY_SOURCE.
    #[arg(long, value_enum, env = "RENOVATE_BINARY_SOURCE")]
    pub(crate) binary_source: Option<BinarySource>,

    /// Enable config migration.
    /// Env: RENOVATE_CONFIG_MIGRATION.
    #[arg(
        long,
        env = "RENOVATE_CONFIG_MIGRATION",
        num_args = 0..=1,
        default_missing_value = "true"
    )]
    pub(crate) config_migration: Option<bool>,

    // ── PR behavior ──────────────────────────────────────────────────────────
    /// Enable or disable Renovate processing.
    /// Env: RENOVATE_ENABLED.
    #[arg(
        long,
        env = "RENOVATE_ENABLED",
        num_args = 0..=1,
        default_missing_value = "true"
    )]
    pub(crate) enabled: Option<bool>,

    /// Whether to automerge branches or PRs automatically.
    /// Env: RENOVATE_AUTOMERGE.
    #[arg(
        long,
        env = "RENOVATE_AUTOMERGE",
        num_args = 0..=1,
        default_missing_value = "true"
    )]
    pub(crate) automerge: Option<bool>,

    /// Controls if platform-native auto-merge is used.
    /// Env: RENOVATE_PLATFORM_AUTOMERGE.
    #[arg(
        long,
        env = "RENOVATE_PLATFORM_AUTOMERGE",
        num_args = 0..=1,
        default_missing_value = "true"
    )]
    pub(crate) platform_automerge: Option<bool>,

    /// Use platform API to perform commits instead of Git directly.
    /// Env: RENOVATE_PLATFORM_COMMIT.
    #[arg(long, value_enum, env = "RENOVATE_PLATFORM_COMMIT")]
    pub(crate) platform_commit: Option<PlatformCommit>,

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

    /// Allowed host rule header patterns (comma-separated or JSON array).
    /// Env: RENOVATE_ALLOWED_HEADERS.
    #[arg(long, env = "RENOVATE_ALLOWED_HEADERS")]
    pub(crate) allowed_headers: Option<String>,

    /// Allowed post-upgrade environment variable patterns.
    /// Env: RENOVATE_ALLOWED_ENV.
    #[arg(long, env = "RENOVATE_ALLOWED_ENV")]
    pub(crate) allowed_env: Option<String>,

    /// Detect global manager config from the filesystem.
    /// Env: RENOVATE_DETECT_GLOBAL_MANAGER_CONFIG.
    #[arg(
        long,
        env = "RENOVATE_DETECT_GLOBAL_MANAGER_CONFIG",
        num_args = 0..=1,
        default_missing_value = "true"
    )]
    pub(crate) detect_global_manager_config: Option<bool>,

    /// Detect host rules from environment variables.
    /// Env: RENOVATE_DETECT_HOST_RULES_FROM_ENV.
    #[arg(
        long,
        env = "RENOVATE_DETECT_HOST_RULES_FROM_ENV",
        num_args = 0..=1,
        default_missing_value = "true"
    )]
    pub(crate) detect_host_rules_from_env: Option<bool>,

    // ── Self-hosted global options ───────────────────────────────────────────
    /// Merge Confidence API endpoint.
    /// Env: RENOVATE_MERGE_CONFIDENCE_ENDPOINT.
    #[arg(long, env = "RENOVATE_MERGE_CONFIDENCE_ENDPOINT")]
    pub(crate) merge_confidence_endpoint: Option<String>,

    /// Merge Confidence datasources (comma-separated or JSON array).
    /// Env: RENOVATE_MERGE_CONFIDENCE_DATASOURCES.
    #[arg(long, env = "RENOVATE_MERGE_CONFIDENCE_DATASOURCES")]
    pub(crate) merge_confidence_datasources: Option<String>,

    /// Autodiscover repository sort key.
    /// Env: RENOVATE_AUTODISCOVER_REPO_SORT.
    #[arg(long, env = "RENOVATE_AUTODISCOVER_REPO_SORT")]
    pub(crate) autodiscover_repo_sort: Option<String>,

    /// Autodiscover repository order.
    /// Env: RENOVATE_AUTODISCOVER_REPO_ORDER.
    #[arg(long, env = "RENOVATE_AUTODISCOVER_REPO_ORDER")]
    pub(crate) autodiscover_repo_order: Option<String>,

    /// Maximum number of Docker tag pages to fetch.
    /// Env: RENOVATE_DOCKER_MAX_PAGES.
    #[arg(long, env = "RENOVATE_DOCKER_MAX_PAGES")]
    pub(crate) docker_max_pages: Option<u32>,

    /// Delete the self-hosted config file after reading it.
    /// Env: RENOVATE_DELETE_CONFIG_FILE.
    #[arg(
        long,
        env = "RENOVATE_DELETE_CONFIG_FILE",
        num_args = 0..=1,
        default_missing_value = "true"
    )]
    pub(crate) delete_config_file: Option<bool>,

    /// S3 endpoint for cache/storage.
    /// Env: RENOVATE_S3_ENDPOINT.
    #[arg(long, env = "RENOVATE_S3_ENDPOINT")]
    pub(crate) s3_endpoint: Option<String>,

    /// Use S3 path-style access.
    /// Env: RENOVATE_S3_PATH_STYLE.
    #[arg(
        long,
        env = "RENOVATE_S3_PATH_STYLE",
        num_args = 0..=1,
        default_missing_value = "true"
    )]
    pub(crate) s3_path_style: Option<bool>,

    /// Force local repository cache behavior.
    /// Env: RENOVATE_REPOSITORY_CACHE_FORCE_LOCAL.
    #[arg(
        long,
        env = "RENOVATE_REPOSITORY_CACHE_FORCE_LOCAL",
        num_args = 0..=1,
        default_missing_value = "true"
    )]
    pub(crate) repository_cache_force_local: Option<bool>,

    /// Repository cache mode.
    /// Env: RENOVATE_REPOSITORY_CACHE.
    #[arg(long, env = "RENOVATE_REPOSITORY_CACHE")]
    pub(crate) repository_cache: Option<String>,

    /// Repository cache storage type.
    /// Env: RENOVATE_REPOSITORY_CACHE_TYPE.
    #[arg(long, env = "RENOVATE_REPOSITORY_CACHE_TYPE")]
    pub(crate) repository_cache_type: Option<String>,

    /// Base directory for local Renovate files.
    /// Env: RENOVATE_BASE_DIR.
    #[arg(long, env = "RENOVATE_BASE_DIR")]
    pub(crate) base_dir: Option<String>,

    /// Cache directory.
    /// Env: RENOVATE_CACHE_DIR.
    #[arg(long, env = "RENOVATE_CACHE_DIR")]
    pub(crate) cache_dir: Option<String>,

    /// Containerbase cache directory.
    /// Env: RENOVATE_CONTAINERBASE_DIR.
    #[arg(long, env = "RENOVATE_CONTAINERBASE_DIR")]
    pub(crate) containerbase_dir: Option<String>,

    /// Default child process execution timeout in minutes.
    /// Env: RENOVATE_EXECUTION_TIMEOUT.
    #[arg(long, env = "RENOVATE_EXECUTION_TIMEOUT")]
    pub(crate) execution_timeout: Option<u32>,

    /// Git task timeout in milliseconds.
    /// Env: RENOVATE_GIT_TIMEOUT.
    #[arg(long, env = "RENOVATE_GIT_TIMEOUT")]
    pub(crate) git_timeout: Option<u32>,

    /// HTTP cache TTL in days.
    /// Env: RENOVATE_HTTP_CACHE_TTL_DAYS.
    #[arg(long, env = "RENOVATE_HTTP_CACHE_TTL_DAYS")]
    pub(crate) http_cache_ttl_days: Option<u32>,

    /// Report output type.
    /// Env: RENOVATE_REPORT_TYPE.
    #[arg(long, env = "RENOVATE_REPORT_TYPE")]
    pub(crate) report_type: Option<String>,

    /// Report output path.
    /// Env: RENOVATE_REPORT_PATH.
    #[arg(long, env = "RENOVATE_REPORT_PATH")]
    pub(crate) report_path: Option<String>,

    /// Labels to apply to created PRs.
    /// Env: RENOVATE_LABELS.
    #[arg(long, env = "RENOVATE_LABELS", value_delimiter = ',')]
    pub(crate) labels: Vec<String>,

    // ── Registry / host rules ─────────────────────────────────────────────────
    /// Host rules (JSON array or object). Env: RENOVATE_HOST_RULES.
    #[arg(long, env = "RENOVATE_HOST_RULES")]
    pub(crate) host_rules: Option<String>,

    /// Registry aliases (JSON object). Env: RENOVATE_REGISTRY_ALIASES.
    #[arg(long, env = "RENOVATE_REGISTRY_ALIASES")]
    pub(crate) registry_aliases: Option<String>,

    /// Onboarding config (JSON object). Env: RENOVATE_ONBOARDING_CONFIG.
    #[arg(long, env = "RENOVATE_ONBOARDING_CONFIG")]
    pub(crate) onboarding_config: Option<String>,

    // ── Output control ───────────────────────────────────────────────────────
    /// Suppress per-dependency listing; show only per-file and run summaries.
    /// Env: RENOVATE_QUIET.
    #[arg(long, short = 'q', env = "RENOVATE_QUIET", default_value_t = false)]
    pub(crate) quiet: bool,

    /// Output format for the update report.
    /// `human` (default) prints colored human-readable output.
    /// `json` emits machine-readable JSON to stdout.
    /// Env: RENOVATE_OUTPUT_FORMAT.
    #[arg(long, env = "RENOVATE_OUTPUT_FORMAT", default_value = "human")]
    pub(crate) output_format: OutputFormat,

    // ── Repositories ─────────────────────────────────────────────────────────
    /// Repositories to process (positional). Later slices dispatch these
    /// into the worker pipeline.
    #[arg(value_name = "repositories")]
    pub(crate) repositories: Vec<String>,
}
