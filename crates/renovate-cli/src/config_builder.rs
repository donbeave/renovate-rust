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
use serde_json::{Map, Value};

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
#[cfg(test)]
pub(crate) fn build(cli: &Cli, base: GlobalConfig) -> GlobalConfig {
    try_build(cli, base).expect("CLI config should be valid")
}

/// Apply CLI arguments on top of a base config, returning an error for invalid
/// structured JSON flag values.
pub(crate) fn try_build(cli: &Cli, base: GlobalConfig) -> Result<GlobalConfig, String> {
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
    if let Some(ref username) = cli.username {
        config.username = Some(username.clone());
    }
    if let Some(ref password) = cli.password {
        config.password = Some(password.clone());
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
    if let Some(config_migration) = cli.config_migration {
        config.config_migration = config_migration;
    }
    if let Some(enabled) = cli.enabled {
        config.enabled = Some(enabled);
    }
    if let Some(automerge) = cli.automerge {
        config.automerge = Some(automerge);
    }
    if let Some(pa) = cli.platform_automerge {
        config.platform_automerge = pa;
    }
    if let Some(rw) = cli.recreate_when {
        config.recreate_when = map_recreate_when(rw);
    }

    if let Some(ref raw) = cli.allowed_commands {
        config.allowed_commands = parse_string_list(raw)?;
    }
    if let Some(act) = cli.allow_command_templating {
        config.allow_command_templating = act;
    }
    if !cli.labels.is_empty() {
        config.labels = trim_list(&cli.labels);
    }
    if let Some(ref raw) = cli.host_rules {
        config.host_rules = parse_json_array(raw)?;
    }
    if let Some(ref raw) = cli.registry_aliases {
        config.registry_aliases = parse_string_map(raw)?;
    }
    if let Some(ref raw) = cli.onboarding_config {
        config.onboarding_config = parse_json_object(raw)?;
    }

    if !cli.repositories.is_empty() {
        config.repositories = cli.repositories.clone();
    }

    Ok(config)
}

pub(crate) fn parse_json_array(raw: &str) -> Result<Vec<Value>, String> {
    if raw.is_empty() {
        return Ok(Vec::new());
    }
    match json5::from_str::<Value>(raw) {
        Ok(Value::Array(values)) => Ok(values),
        Ok(Value::Object(object)) => Ok(vec![Value::Object(object)]),
        _ => Err(format!("Invalid JSON value: '{raw}'")),
    }
}

pub(crate) fn parse_json_object(raw: &str) -> Result<Map<String, Value>, String> {
    if raw.is_empty() {
        return Ok(Map::new());
    }
    match json5::from_str::<Value>(raw) {
        Ok(Value::Object(object)) => Ok(object),
        _ => Err(format!("Invalid JSON value: '{raw}'")),
    }
}

fn parse_string_map(raw: &str) -> Result<std::collections::BTreeMap<String, String>, String> {
    let object = parse_json_object(raw)?;
    object
        .into_iter()
        .map(|(key, value)| match value {
            Value::String(value) => Ok((key, value)),
            _ => Err(format!("Invalid JSON value: '{raw}'")),
        })
        .collect()
}

fn parse_string_list(raw: &str) -> Result<Vec<String>, String> {
    if raw.is_empty() {
        return Ok(Vec::new());
    }
    match json5::from_str::<Value>(raw) {
        Ok(Value::Array(values)) => values
            .into_iter()
            .map(|value| match value {
                Value::String(value) => Ok(value),
                _ => Err(format!("Invalid JSON value: '{raw}'")),
            })
            .collect(),
        Ok(_) => Err(format!("Invalid JSON value: '{raw}'")),
        Err(_) => Ok(raw
            .split(',')
            .map(str::trim)
            .filter(|item| !item.is_empty())
            .map(str::to_owned)
            .collect()),
    }
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

fn trim_list(values: &[String]) -> Vec<String> {
    values
        .iter()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use clap::Parser as _;
    use renovate_core::config::{DryRun, GlobalConfig, Platform, RecreateWhen, RequireConfig};

    use super::{build, try_build};
    use crate::cli::{Cli, DryRunArg, RequireConfigArg};
    use crate::migrate::migrate_args;

    fn cli_with(mutate: impl FnOnce(&mut Cli)) -> Cli {
        let mut cli = Cli {
            version: false,
            platform: None,
            token: None,
            endpoint: None,
            username: None,
            password: None,
            dry_run: None,
            require_config: None,
            fork_processing: None,
            config_migration: None,
            enabled: None,
            automerge: None,
            platform_automerge: None,
            recreate_when: None,
            allowed_commands: None,
            allow_command_templating: None,
            labels: Vec::new(),
            host_rules: None,
            registry_aliases: None,
            onboarding_config: None,
            quiet: false,
            output_format: crate::cli::OutputFormat::Human,
            repositories: Vec::new(),
        };
        mutate(&mut cli);
        cli
    }

    fn parse_and_build(args: &[&str]) -> GlobalConfig {
        let argv = std::iter::once("renovate").chain(args.iter().copied());
        let cli = Cli::try_parse_from(argv).expect("CLI args should parse");
        build(&cli, GlobalConfig::default())
    }

    fn migrate_parse_and_build(args: &[&str]) -> GlobalConfig {
        let raw: Vec<String> = std::iter::once("renovate")
            .chain(args.iter().copied())
            .map(str::to_owned)
            .collect();
        let migrated = migrate_args(&raw);
        let cli = Cli::try_parse_from(migrated).expect("CLI args should parse after migration");
        build(&cli, GlobalConfig::default())
    }

    // Ported: "returns empty argv" — workers/global/config/parse/cli.spec.ts line 32
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

    // Ported: "supports string" — workers/global/config/parse/cli.spec.ts line 84
    #[test]
    fn token_is_set() {
        let cli = cli_with(|c| c.token = Some("mytoken".to_owned()));
        assert_eq!(
            build(&cli, GlobalConfig::default()).token.as_deref(),
            Some("mytoken")
        );
    }

    #[test]
    fn username_and_password_are_set() {
        let cli = cli_with(|c| {
            c.username = Some("some-user".to_owned());
            c.password = Some("app-password".to_owned());
        });
        let config = build(&cli, GlobalConfig::default());
        assert_eq!(config.username.as_deref(), Some("some-user"));
        assert_eq!(config.password.as_deref(), Some("app-password"));
    }

    #[test]
    fn username_and_password_flags_are_parsed() {
        let config = parse_and_build(&["--username", "some-user", "--password=app-password"]);
        assert_eq!(config.username.as_deref(), Some("some-user"));
        assert_eq!(config.password.as_deref(), Some("app-password"));
    }

    // Ported: "supports repositories" — workers/global/config/parse/cli.spec.ts line 89
    #[test]
    fn repositories_are_set() {
        let cli = cli_with(|c| {
            c.repositories = vec!["foo".to_owned(), "bar".to_owned()];
        });
        assert_eq!(
            build(&cli, GlobalConfig::default()).repositories,
            vec!["foo".to_owned(), "bar".to_owned()]
        );
    }

    // Ported: "supports boolean no value" — workers/global/config/parse/cli.spec.ts line 36
    #[test]
    fn config_migration_bare_sets_true() {
        assert!(parse_and_build(&["--config-migration"]).config_migration);
    }

    // Ported: "supports boolean space true" — workers/global/config/parse/cli.spec.ts line 42
    #[test]
    fn config_migration_space_true_sets_true() {
        assert!(parse_and_build(&["--config-migration", "true"]).config_migration);
    }

    // Ported: "throws exception for invalid boolean value" — workers/global/config/parse/cli.spec.ts line 48
    #[test]
    fn config_migration_invalid_boolean_is_rejected() {
        let err = Cli::try_parse_from(["renovate", "--config-migration", "badvalue"])
            .expect_err("bad boolean should be rejected");
        assert!(err.to_string().contains("badvalue"));
    }

    // Ported: "supports boolean space false" — workers/global/config/parse/cli.spec.ts line 58
    #[test]
    fn config_migration_space_false_sets_false() {
        assert!(!parse_and_build(&["--config-migration", "false"]).config_migration);
    }

    // Ported: "supports boolean equals true" — workers/global/config/parse/cli.spec.ts line 64
    #[test]
    fn config_migration_equals_true_sets_true() {
        assert!(parse_and_build(&["--config-migration=true"]).config_migration);
    }

    // Ported: "supports boolean equals false" — workers/global/config/parse/cli.spec.ts line 69
    #[test]
    fn config_migration_equals_false_sets_false() {
        assert!(!parse_and_build(&["--config-migration=false"]).config_migration);
    }

    #[test]
    fn enabled_flag_sets_enabled_config() {
        assert_eq!(parse_and_build(&["--enabled=false"]).enabled, Some(false));
    }

    #[test]
    fn automerge_flag_sets_automerge_config() {
        assert_eq!(parse_and_build(&["--automerge"]).automerge, Some(true));
    }

    // Ported: "supports list single" — workers/global/config/parse/cli.spec.ts line 74
    #[test]
    fn labels_single_value_is_set() {
        assert_eq!(parse_and_build(&["--labels=a"]).labels, vec!["a"]);
    }

    // Ported: "supports list multiple" — workers/global/config/parse/cli.spec.ts line 79
    #[test]
    fn labels_comma_separated_values_are_set() {
        assert_eq!(
            parse_and_build(&["--labels=a, b,c,"]).labels,
            vec!["a", "b", "c"]
        );
    }

    // Ported: "parses json lists correctly" — workers/global/config/parse/cli.spec.ts line 95
    #[test]
    fn host_rules_json_list_is_parsed() {
        let config = parse_and_build(&[
            r#"--host-rules=[{"matchHost":"docker.io","hostType":"docker","username":"user","password":"password"}]"#,
        ]);
        assert_eq!(config.host_rules.len(), 1);
        assert_eq!(config.host_rules[0]["matchHost"], "docker.io");
        assert_eq!(config.host_rules[0]["hostType"], "docker");
        assert_eq!(config.host_rules[0]["username"], "user");
        assert_eq!(config.host_rules[0]["password"], "password");
    }

    // Ported: "parses [] correctly as empty list of hostRules" — workers/global/config/parse/cli.spec.ts line 111
    #[test]
    fn host_rules_empty_array_is_parsed() {
        assert!(parse_and_build(&["--host-rules=[]"]).host_rules.is_empty());
    }

    // Ported: "parses an empty string correctly as empty list of hostRules" — workers/global/config/parse/cli.spec.ts line 118
    #[test]
    fn host_rules_empty_string_is_parsed() {
        assert!(parse_and_build(&["--host-rules="]).host_rules.is_empty());
    }

    // Ported: "\"$arg\" -> $config" — workers/global/config/parse/cli.spec.ts line 125
    #[test]
    fn migrated_cli_aliases_produce_expected_config() {
        let cases = [
            ("--endpoints=", true, RecreateWhen::Auto),
            ("--azure-auto-complete=false", false, RecreateWhen::Auto),
            ("--azure-auto-complete=true", true, RecreateWhen::Auto),
            ("--azure-auto-complete", true, RecreateWhen::Auto),
            ("--git-lab-automerge=false", false, RecreateWhen::Auto),
            ("--git-lab-automerge=true", true, RecreateWhen::Auto),
            ("--git-lab-automerge", true, RecreateWhen::Auto),
            ("--recreate-closed=false", true, RecreateWhen::Auto),
            ("--recreate-closed=true", true, RecreateWhen::Always),
            ("--recreate-closed", true, RecreateWhen::Always),
            ("--recreate-when=auto", true, RecreateWhen::Auto),
            ("--recreate-when=always", true, RecreateWhen::Always),
            ("--recreate-when=never", true, RecreateWhen::Never),
        ];

        for (arg, expected_platform_automerge, expected_recreate) in cases {
            let config = migrate_parse_and_build(&[arg]);
            assert_eq!(
                config.platform_automerge, expected_platform_automerge,
                "{arg}"
            );
            if arg == "--endpoints=" {
                assert!(config.host_rules.is_empty(), "{arg}");
            }
            assert_eq!(config.recreate_when, expected_recreate, "{arg}");
        }
    }

    // Ported: "parses json object correctly when empty" — workers/global/config/parse/cli.spec.ts line 145
    #[test]
    fn onboarding_config_empty_string_is_parsed() {
        assert!(
            parse_and_build(&["--onboarding-config="])
                .onboarding_config
                .is_empty()
        );
    }

    // Ported: "parses json {} object correctly" — workers/global/config/parse/cli.spec.ts line 152
    #[test]
    fn onboarding_config_empty_object_is_parsed() {
        assert!(
            parse_and_build(&["--onboarding-config={}"])
                .onboarding_config
                .is_empty()
        );
    }

    // Ported: "parses json object correctly" — workers/global/config/parse/cli.spec.ts line 159
    #[test]
    fn onboarding_config_object_is_parsed() {
        let config =
            parse_and_build(&[r#"--onboarding-config={"extends": ["config:recommended"]}"#]);
        assert_eq!(config.onboarding_config["extends"][0], "config:recommended");
    }

    #[test]
    fn onboarding_config_json5_object_is_parsed() {
        let config = parse_and_build(&[
            r#"--onboarding-config={extends:['config:recommended'],}"#,
        ]);
        assert_eq!(config.onboarding_config["extends"][0], "config:recommended");
    }

    // Ported: "throws exception for invalid json object" — workers/global/config/parse/cli.spec.ts line 168
    #[test]
    fn onboarding_config_invalid_json_is_rejected() {
        let cli = Cli::try_parse_from(["renovate", "--onboarding-config=Hello_World"])
            .expect("raw CLI string should parse");
        let err = try_build(&cli, GlobalConfig::default()).expect_err("invalid JSON should fail");
        assert_eq!(err, "Invalid JSON value: 'Hello_World'");
    }

    #[test]
    fn dry_run_full_is_mapped() {
        let cli = cli_with(|c| c.dry_run = Some(DryRunArg::Full));
        assert_eq!(
            build(&cli, GlobalConfig::default()).dry_run,
            Some(DryRun::Full)
        );
    }

    // Ported: "dryRun boolean true" — workers/global/config/parse/cli.spec.ts line 175
    #[test]
    fn dry_run_legacy_true_maps_to_full() {
        let cli = cli_with(|c| c.dry_run = Some(DryRunArg::LegacyTrue));
        assert_eq!(
            build(&cli, GlobalConfig::default()).dry_run,
            Some(DryRun::Full)
        );
    }

    // Ported: "dryRun boolean false" — workers/global/config/parse/cli.spec.ts line 185
    #[test]
    fn dry_run_legacy_false_disables_dry_run() {
        let cli = cli_with(|c| c.dry_run = Some(DryRunArg::LegacyFalse));
        assert_eq!(build(&cli, GlobalConfig::default()).dry_run, None);
    }

    // Ported: "dryRun  null" — workers/global/config/parse/cli.spec.ts line 190
    #[test]
    fn dry_run_legacy_null_disables_dry_run() {
        let cli = cli_with(|c| c.dry_run = Some(DryRunArg::LegacyNull));
        assert_eq!(build(&cli, GlobalConfig::default()).dry_run, None);
    }

    // Ported: "requireConfig boolean true" — workers/global/config/parse/cli.spec.ts line 195
    #[test]
    fn require_config_legacy_true_maps_to_required() {
        let cli = cli_with(|c| c.require_config = Some(RequireConfigArg::LegacyTrue));
        assert_eq!(
            build(&cli, GlobalConfig::default()).require_config,
            RequireConfig::Required
        );
    }

    // Ported: "requireConfig boolean false" — workers/global/config/parse/cli.spec.ts line 205
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
        let cli = cli_with(|c| c.allowed_commands = Some("foo,bar, baz,".to_owned()));
        let config = build(&cli, GlobalConfig::default());
        assert_eq!(
            config.allowed_commands,
            vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()],
        );
    }

    #[test]
    fn allowed_commands_json_array_parsed() {
        let cli = cli_with(|c| c.allowed_commands = Some(r#"["foo","bar baz"]"#.to_owned()));
        let config = build(&cli, GlobalConfig::default());
        assert_eq!(
            config.allowed_commands,
            vec!["foo".to_owned(), "bar baz".to_owned()],
        );
    }

    #[test]
    fn allowed_commands_json5_array_parsed() {
        let cli = cli_with(|c| c.allowed_commands = Some(r#"['foo','bar baz',]"#.to_owned()));
        let config = build(&cli, GlobalConfig::default());
        assert_eq!(
            config.allowed_commands,
            vec!["foo".to_owned(), "bar baz".to_owned()],
        );
    }
}
