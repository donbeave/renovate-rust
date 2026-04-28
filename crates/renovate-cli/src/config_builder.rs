//! Converts parsed CLI arguments into a [`GlobalConfig`].
//!
//! This module is the bridge between the CLI-facing types (which include
//! Renovate's legacy string variants) and the canonical core types. It also
//! emits the same deprecation warnings that Renovate's `getConfig` emits when
//! callers pass legacy boolean values for `--dry-run` and `--require-config`.
//!
//! Renovate reference: `lib/workers/global/config/parse/cli.ts` `getConfig`.

use renovate_core::config::{
    DryRun, ForkProcessing, GlobalConfig, Platform, RecreateWhen, RequireConfig,
};

use crate::cli::{
    Cli, DryRunArg, ForkProcessing as CliForkProcessing, Platform as CliPlatform,
    RecreateWhen as CliRecreateWhen, RequireConfigArg,
};

/// Apply CLI arguments on top of a `base` [`GlobalConfig`].
///
/// Only fields that were explicitly supplied on the command line (i.e. `Some`
/// in the `Cli` struct) override the base. This allows the caller to merge
/// `defaults → file config → CLI config` by calling this function last.
///
/// Applies Renovate-compatible coercions and emits `tracing::warn` for
/// deprecated value forms (e.g. `--dry-run=true` → `full`).
pub(crate) fn build(cli: &Cli, base: GlobalConfig) -> GlobalConfig {
    let mut config = base;

    if let Some(p) = cli.platform {
        config.platform = map_platform(p);
    }
    if let Some(ref t) = cli.token {
        config.token = Some(t.clone());
    }
    if let Some(ref e) = cli.endpoint {
        config.endpoint = Some(e.clone());
    }

    if let Some(dry) = map_dry_run(cli.dry_run) {
        config.dry_run = Some(dry);
    } else if cli.dry_run.is_some() {
        // --dry-run=false / --dry-run=null explicitly disables dry-run.
        config.dry_run = None;
    }

    if let Some(rc) = map_require_config_explicit(cli.require_config) {
        config.require_config = rc;
    }

    if let Some(fp) = cli.fork_processing {
        config.fork_processing = map_fork_processing(fp);
    }
    if let Some(pa) = cli.platform_automerge {
        config.platform_automerge = pa;
    }
    if let Some(rw) = cli.recreate_when {
        config.recreate_when = map_recreate_when(rw);
    }

    // --allowed-commands: Renovate accepts comma-separated or JSON array.
    // For now pass the raw string through; full JSON5 parsing is a later slice.
    if let Some(ref raw) = cli.allowed_commands {
        config.allowed_commands = raw.split(',').map(|s| s.trim().to_owned()).collect();
    }
    if let Some(act) = cli.allow_command_templating {
        config.allow_command_templating = act;
    }

    config
}

fn map_platform(p: CliPlatform) -> Platform {
    match p {
        CliPlatform::Azure => Platform::Azure,
        CliPlatform::Bitbucket => Platform::Bitbucket,
        CliPlatform::BitbucketServer => Platform::BitbucketServer,
        CliPlatform::Codecommit => Platform::Codecommit,
        CliPlatform::Forgejo => Platform::Forgejo,
        CliPlatform::Gerrit => Platform::Gerrit,
        CliPlatform::Gitea => Platform::Gitea,
        CliPlatform::Github => Platform::Github,
        CliPlatform::Gitlab => Platform::Gitlab,
        CliPlatform::Local => Platform::Local,
        CliPlatform::ScmManager => Platform::ScmManager,
    }
}

fn map_dry_run(arg: Option<DryRunArg>) -> Option<DryRun> {
    match arg? {
        DryRunArg::Extract => Some(DryRun::Extract),
        DryRunArg::Lookup => Some(DryRun::Lookup),
        DryRunArg::Full => Some(DryRun::Full),
        DryRunArg::LegacyTrue => {
            tracing::warn!("cli config dryRun property has been changed to full");
            Some(DryRun::Full)
        }
        DryRunArg::LegacyFalse | DryRunArg::LegacyNull => None,
    }
}

/// Returns `Some(value)` only when the arg was explicitly provided.
fn map_require_config_explicit(arg: Option<RequireConfigArg>) -> Option<RequireConfig> {
    match arg? {
        RequireConfigArg::Required => Some(RequireConfig::Required),
        RequireConfigArg::Optional => Some(RequireConfig::Optional),
        RequireConfigArg::Ignored => Some(RequireConfig::Ignored),
        RequireConfigArg::LegacyTrue => {
            tracing::warn!("cli config requireConfig property has been changed to required");
            Some(RequireConfig::Required)
        }
        RequireConfigArg::LegacyFalse => {
            tracing::warn!("cli config requireConfig property has been changed to optional");
            Some(RequireConfig::Optional)
        }
    }
}

fn map_fork_processing(fp: CliForkProcessing) -> ForkProcessing {
    match fp {
        CliForkProcessing::Auto => ForkProcessing::Auto,
        CliForkProcessing::Enabled => ForkProcessing::Enabled,
        CliForkProcessing::Disabled => ForkProcessing::Disabled,
    }
}

fn map_recreate_when(rw: CliRecreateWhen) -> RecreateWhen {
    match rw {
        CliRecreateWhen::Auto => RecreateWhen::Auto,
        CliRecreateWhen::Always => RecreateWhen::Always,
        CliRecreateWhen::Never => RecreateWhen::Never,
    }
}

#[cfg(test)]
mod tests {
    use renovate_core::config::{DryRun, GlobalConfig, Platform, RequireConfig};

    use super::build;
    use crate::cli::{Cli, DryRunArg, RequireConfigArg};

    fn cli_with(mutate: impl FnOnce(&mut Cli)) -> Cli {
        let mut cli = Cli {
            version: false,
            platform: None,
            token: None,
            endpoint: None,
            dry_run: None,
            require_config: None,
            fork_processing: None,
            platform_automerge: None,
            recreate_when: None,
            allowed_commands: None,
            allow_command_templating: None,
            host_rules: None,
            registry_aliases: None,
            repositories: Vec::new(),
        };
        mutate(&mut cli);
        cli
    }

    #[test]
    fn default_cli_produces_default_config() {
        let cli = cli_with(|_| {});
        let config = build(&cli, GlobalConfig::default());
        assert_eq!(config, GlobalConfig::default());
    }

    #[test]
    fn platform_github_is_mapped() {
        use crate::cli::Platform as CliPlatform;
        let cli = cli_with(|c| c.platform = Some(CliPlatform::Github));
        assert_eq!(
            build(&cli, GlobalConfig::default()).platform,
            Platform::Github
        );
    }

    #[test]
    fn platform_gitlab_is_mapped() {
        use crate::cli::Platform as CliPlatform;
        let cli = cli_with(|c| c.platform = Some(CliPlatform::Gitlab));
        assert_eq!(
            build(&cli, GlobalConfig::default()).platform,
            Platform::Gitlab
        );
    }

    #[test]
    fn token_is_set() {
        let cli = cli_with(|c| c.token = Some("mytoken".to_owned()));
        assert_eq!(
            build(&cli, GlobalConfig::default()).token.as_deref(),
            Some("mytoken")
        );
    }

    #[test]
    fn dry_run_full_is_mapped() {
        let cli = cli_with(|c| c.dry_run = Some(DryRunArg::Full));
        assert_eq!(
            build(&cli, GlobalConfig::default()).dry_run,
            Some(DryRun::Full)
        );
    }

    #[test]
    fn dry_run_legacy_true_maps_to_full() {
        // --dry-run (bare) → --dry-run=true via migrateArgs → Full.
        let cli = cli_with(|c| c.dry_run = Some(DryRunArg::LegacyTrue));
        assert_eq!(
            build(&cli, GlobalConfig::default()).dry_run,
            Some(DryRun::Full)
        );
    }

    #[test]
    fn dry_run_legacy_false_disables_dry_run() {
        let cli = cli_with(|c| c.dry_run = Some(DryRunArg::LegacyFalse));
        assert_eq!(build(&cli, GlobalConfig::default()).dry_run, None);
    }

    #[test]
    fn require_config_legacy_true_maps_to_required() {
        let cli = cli_with(|c| c.require_config = Some(RequireConfigArg::LegacyTrue));
        assert_eq!(
            build(&cli, GlobalConfig::default()).require_config,
            RequireConfig::Required
        );
    }

    #[test]
    fn require_config_legacy_false_maps_to_optional() {
        let cli = cli_with(|c| c.require_config = Some(RequireConfigArg::LegacyFalse));
        assert_eq!(
            build(&cli, GlobalConfig::default()).require_config,
            RequireConfig::Optional
        );
    }

    #[test]
    fn allowed_commands_comma_split() {
        let cli = cli_with(|c| c.allowed_commands = Some("foo,bar, baz".to_owned()));
        let config = build(&cli, GlobalConfig::default());
        assert_eq!(
            config.allowed_commands,
            vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()],
        );
    }
}
