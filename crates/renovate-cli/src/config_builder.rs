//! Converts parsed CLI arguments into a [`GlobalConfig`].
//!
//! This module is the bridge between the CLI-facing types (which include
//! Renovate's legacy string variants) and the canonical core types. It also
//! emits the same deprecation warnings that Renovate's `getConfig` emits when
//! callers pass legacy boolean values for `--dry-run` and `--require-config`.
//!
//! Renovate reference: `lib/workers/global/config/parse/cli.ts` `getConfig`.

use renovate_core::config::{
    BinarySource, DryRun, ForkProcessing, GlobalConfig, Platform, RecreateWhen, RequireConfig,
};
use serde_json::{Map, Value};

use crate::cli::{
    BinarySource as CliBinarySource, Cli, DryRunArg, ForkProcessing as CliForkProcessing,
    Platform as CliPlatform, PlatformCommit as CliPlatformCommit,
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
    if let Some(ref user_agent) = cli.user_agent {
        config.user_agent = Some(user_agent.clone());
    }

    if let Some(dry) = map_dry_run(cli.dry_run) {
        config.dry_run = Some(dry);
    } else if cli.dry_run.is_some() {
        // --dry-run=false / --dry-run=null explicitly disables dry-run.
        config.dry_run = None;
    }

    if let Some(ref mode) = cli.mode {
        config.mode = Some(mode.clone());
    }

    if let Some(rc) = map_require_config_explicit(cli.require_config) {
        config.require_config = rc;
    }

    if let Some(fp) = cli.fork_processing {
        config.fork_processing = map_fork_processing(fp);
    }
    if let Some(binary_source) = cli.binary_source {
        config.binary_source = Some(map_binary_source(binary_source));
    }
    if let Some(config_migration) = cli.config_migration {
        config.config_migration = config_migration;
    }
    if let Some(print_config) = cli.print_config {
        config.print_config = Some(print_config);
    }
    if let Some(onboarding) = cli.onboarding {
        config.onboarding = Some(onboarding);
    }
    if let Some(ref names) = cli.config_file_names {
        config.config_file_names = Some(parse_string_list(names)?);
    }
    if let Some(ref raw) = cli.migrate_presets {
        config.migrate_presets = parse_string_map(raw)?;
    }
    if let Some(ref raw) = cli.custom_env_variables {
        config.custom_env_variables = parse_string_map(raw)?;
    }
    if let Some(ref raw) = cli.cache_ttl_override {
        config.cache_ttl_override = parse_json_object(raw)?;
    }
    if let Some(ref no_deps) = cli.onboarding_no_deps {
        config.onboarding_no_deps = Some(no_deps.clone());
    }
    if let Some(rebase) = cli.onboarding_rebase_checkbox {
        config.onboarding_rebase_checkbox = Some(rebase);
    }
    if let Some(limit) = cli.pr_commits_per_run_limit {
        config.pr_commits_per_run_limit = Some(limit);
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
    if let Some(platform_commit) = cli.platform_commit {
        config.platform_commit = Some(map_platform_commit(platform_commit).to_owned());
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
    if let Some(allow) = cli.allow_plugins {
        config.allow_plugins = Some(allow);
    }
    if let Some(allow) = cli.allow_scripts {
        config.allow_scripts = Some(allow);
    }
    if let Some(allow) = cli.allow_shell_executor_for_post_upgrade_commands {
        config.allow_shell_executor_for_post_upgrade_commands = Some(allow);
    }
    if let Some(allow) = cli.allow_custom_crate_registries {
        config.allow_custom_crate_registries = Some(allow);
    }
    if let Some(ref allowed_headers) = cli.allowed_headers {
        config.allowed_headers = Some(parse_string_list(allowed_headers)?);
    }
    if let Some(ref allowed_env) = cli.allowed_env {
        config.allowed_env = Some(parse_string_list(allowed_env)?);
    }
    if let Some(ref allowed) = cli.allowed_unsafe_executions {
        config.allowed_unsafe_executions = Some(parse_string_list(allowed)?);
    }
    if let Some(expose) = cli.expose_all_env {
        config.expose_all_env = Some(expose);
    }
    if let Some(detect) = cli.detect_global_manager_config {
        config.detect_global_manager_config = Some(detect);
    }
    if let Some(detect) = cli.detect_host_rules_from_env {
        config.detect_host_rules_from_env = Some(detect);
    }
    if let Some(ref endpoint) = cli.merge_confidence_endpoint {
        config.merge_confidence_endpoint = Some(endpoint.clone());
    }
    if let Some(ref datasources) = cli.merge_confidence_datasources {
        config.merge_confidence_datasources = parse_string_list(datasources)?;
    }
    if let Some(ref sort) = cli.autodiscover_repo_sort {
        config.autodiscover_repo_sort = Some(sort.clone());
    }
    if let Some(ref order) = cli.autodiscover_repo_order {
        config.autodiscover_repo_order = Some(order.clone());
    }
    if let Some(autodiscover) = cli.autodiscover {
        config.autodiscover = Some(autodiscover);
    }
    if let Some(ref filters) = cli.autodiscover_filter {
        config.autodiscover_filter = Some(parse_string_list(filters)?);
    }
    if let Some(ref namespaces) = cli.autodiscover_namespaces {
        config.autodiscover_namespaces = Some(parse_string_list(namespaces)?);
    }
    if let Some(ref projects) = cli.autodiscover_projects {
        config.autodiscover_projects = Some(parse_string_list(projects)?);
    }
    if let Some(ref topics) = cli.autodiscover_topics {
        config.autodiscover_topics = Some(parse_string_list(topics)?);
    }
    if let Some(max_pages) = cli.docker_max_pages {
        config.docker_max_pages = Some(max_pages);
    }
    if let Some(delete_config_file) = cli.delete_config_file {
        config.delete_config_file = delete_config_file;
    }
    if let Some(ref endpoint) = cli.s3_endpoint {
        config.s3_endpoint = Some(endpoint.clone());
    }
    if let Some(path_style) = cli.s3_path_style {
        config.s3_path_style = path_style;
    }
    if let Some(force_local) = cli.repository_cache_force_local {
        config.repository_cache_force_local = Some(force_local);
    }
    if let Some(ref cache) = cli.repository_cache {
        config.repository_cache = Some(cache.clone());
    }
    if let Some(ref cache_type) = cli.repository_cache_type {
        config.repository_cache_type = Some(cache_type.clone());
    }
    if let Some(ref base_dir) = cli.base_dir {
        config.base_dir = Some(base_dir.clone());
    }
    if let Some(ref local_dir) = cli.local_dir {
        config.local_dir = Some(local_dir.clone());
    }
    if let Some(ref cache_dir) = cli.cache_dir {
        config.cache_dir = Some(cache_dir.clone());
    }
    if let Some(ref containerbase_dir) = cli.containerbase_dir {
        config.containerbase_dir = Some(containerbase_dir.clone());
    }
    if let Some(ref prefix) = cli.docker_child_prefix {
        config.docker_child_prefix = Some(prefix.clone());
    }
    if let Some(ref options) = cli.docker_cli_options {
        config.docker_cli_options = Some(options.clone());
    }
    if let Some(ref image) = cli.docker_sidecar_image {
        config.docker_sidecar_image = Some(image.clone());
    }
    if let Some(ref user) = cli.docker_user {
        config.docker_user = Some(user.clone());
    }
    if let Some(timeout) = cli.execution_timeout {
        config.execution_timeout = Some(timeout);
    }
    if let Some(timeout) = cli.git_timeout {
        config.git_timeout = Some(timeout);
    }
    if let Some(days) = cli.http_cache_ttl_days {
        config.http_cache_ttl_days = Some(days);
    }
    if let Some(minutes) = cli.cache_hard_ttl_minutes {
        config.cache_hard_ttl_minutes = Some(minutes);
    }
    if let Some(cache) = cli.cache_private_packages {
        config.cache_private_packages = Some(cache);
    }
    if let Some(persist) = cli.preset_cache_persistence {
        config.preset_cache_persistence = Some(persist);
    }
    if let Some(include) = cli.include_mirrors {
        config.include_mirrors = Some(include);
    }
    if let Some(warn) = cli.github_token_warn {
        config.github_token_warn = Some(warn);
    }
    if let Some(ignore) = cli.ignore_pr_author {
        config.ignore_pr_author = Some(ignore);
    }
    if let Some(ref warning) = cli.encrypted_warning {
        config.encrypted_warning = Some(warning.clone());
    }
    if let Some(use_development_branch) = cli.bb_use_development_branch {
        config.bb_use_development_branch = Some(use_development_branch);
    }
    if let Some(max_pages) = cli.pr_cache_sync_max_pages {
        config.pr_cache_sync_max_pages = Some(max_pages);
    }
    if let Some(ref report_type) = cli.report_type {
        config.report_type = Some(report_type.clone());
    }
    if let Some(ref report_path) = cli.report_path {
        config.report_path = Some(report_path.clone());
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

fn map_binary_source(binary_source: CliBinarySource) -> BinarySource {
    match binary_source {
        CliBinarySource::Global | CliBinarySource::Auto => BinarySource::Global,
        CliBinarySource::Docker => BinarySource::Docker,
        CliBinarySource::Install => BinarySource::Install,
        CliBinarySource::Hermit => BinarySource::Hermit,
    }
}

fn map_platform_commit(platform_commit: CliPlatformCommit) -> &'static str {
    match platform_commit {
        CliPlatformCommit::Auto => "auto",
        CliPlatformCommit::Disabled => "disabled",
        CliPlatformCommit::Enabled => "enabled",
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
    use renovate_core::config::{
        BinarySource, DryRun, GlobalConfig, Platform, RecreateWhen, RequireConfig,
    };

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
            user_agent: None,
            dry_run: None,
            mode: None,
            require_config: None,
            fork_processing: None,
            binary_source: None,
            config_migration: None,
            print_config: None,
            onboarding: None,
            config_file_names: None,
            migrate_presets: None,
            custom_env_variables: None,
            cache_ttl_override: None,
            onboarding_no_deps: None,
            onboarding_rebase_checkbox: None,
            pr_commits_per_run_limit: None,
            enabled: None,
            automerge: None,
            platform_automerge: None,
            platform_commit: None,
            recreate_when: None,
            allowed_commands: None,
            allow_command_templating: None,
            allow_plugins: None,
            allow_scripts: None,
            allow_shell_executor_for_post_upgrade_commands: None,
            allow_custom_crate_registries: None,
            allowed_headers: None,
            allowed_env: None,
            allowed_unsafe_executions: None,
            expose_all_env: None,
            detect_global_manager_config: None,
            detect_host_rules_from_env: None,
            merge_confidence_endpoint: None,
            merge_confidence_datasources: None,
            autodiscover_repo_sort: None,
            autodiscover_repo_order: None,
            autodiscover: None,
            autodiscover_filter: None,
            autodiscover_namespaces: None,
            autodiscover_projects: None,
            autodiscover_topics: None,
            docker_max_pages: None,
            delete_config_file: None,
            s3_endpoint: None,
            s3_path_style: None,
            repository_cache_force_local: None,
            repository_cache: None,
            repository_cache_type: None,
            base_dir: None,
            local_dir: None,
            cache_dir: None,
            containerbase_dir: None,
            docker_child_prefix: None,
            docker_cli_options: None,
            docker_sidecar_image: None,
            docker_user: None,
            execution_timeout: None,
            git_timeout: None,
            http_cache_ttl_days: None,
            cache_hard_ttl_minutes: None,
            cache_private_packages: None,
            preset_cache_persistence: None,
            include_mirrors: None,
            github_token_warn: None,
            ignore_pr_author: None,
            encrypted_warning: None,
            bb_use_development_branch: None,
            pr_cache_sync_max_pages: None,
            report_type: None,
            report_path: None,
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

    #[test]
    fn runtime_global_flags_are_parsed() {
        let config = parse_and_build(&[
            "--user-agent=renovate-rust-test",
            "--mode=silent",
            "--base-dir=/tmp/renovate",
            "--cache-dir=/tmp/renovate/cache",
            "--containerbase-dir=/tmp/renovate/containerbase",
            "--docker-child-prefix=rr_",
            "--docker-cli-options=--network=host",
            "--docker-sidecar-image=example/sidecar:1",
            "--docker-user=1000:1000",
            "--execution-timeout=20",
            "--git-timeout=10000",
            "--http-cache-ttl-days=45",
        ]);

        assert_eq!(config.user_agent.as_deref(), Some("renovate-rust-test"));
        assert_eq!(config.mode.as_deref(), Some("silent"));
        assert_eq!(config.base_dir.as_deref(), Some("/tmp/renovate"));
        assert_eq!(config.cache_dir.as_deref(), Some("/tmp/renovate/cache"));
        assert_eq!(
            config.containerbase_dir.as_deref(),
            Some("/tmp/renovate/containerbase")
        );
        assert_eq!(config.docker_child_prefix.as_deref(), Some("rr_"));
        assert_eq!(
            config.docker_cli_options.as_deref(),
            Some("--network=host")
        );
        assert_eq!(
            config.docker_sidecar_image.as_deref(),
            Some("example/sidecar:1")
        );
        assert_eq!(config.docker_user.as_deref(), Some("1000:1000"));
        assert_eq!(config.execution_timeout, Some(20));
        assert_eq!(config.git_timeout, Some(10000));
        assert_eq!(config.http_cache_ttl_days, Some(45));
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

    #[test]
    fn binary_source_flag_is_parsed() {
        assert_eq!(
            parse_and_build(&["--binary-source=hermit"]).binary_source,
            Some(BinarySource::Hermit)
        );
    }

    #[test]
    fn binary_source_auto_maps_to_global() {
        assert_eq!(
            parse_and_build(&["--binary-source=auto"]).binary_source,
            Some(BinarySource::Global)
        );
    }

    #[test]
    fn platform_commit_flag_is_parsed() {
        assert_eq!(
            parse_and_build(&["--platform-commit=enabled"])
                .platform_commit
                .as_deref(),
            Some("enabled")
        );
    }

    #[test]
    fn self_hosted_global_flags_are_parsed() {
        let config = parse_and_build(&[
            "--allow-plugins",
            "--allow-scripts=false",
            "--allow-shell-executor-for-post-upgrade-commands",
            "--allow-custom-crate-registries=true",
            "--allowed-headers=X-*,Authorization",
            "--allowed-env=['SOME_*','OTHER_*']",
            "--allowed-unsafe-executions=bazelModDeps,goGenerate",
            "--expose-all-env",
            "--detect-global-manager-config",
            "--detect-host-rules-from-env=false",
            "--merge-confidence-endpoint=https://mc.example",
            "--merge-confidence-datasources=docker,npm",
            "--autodiscover-repo-sort=updated",
            "--autodiscover-repo-order=desc",
            "--autodiscover",
            "--autodiscover-filter=org/*,!org/archived",
            "--autodiscover-namespaces=backend,frontend",
            r#"--autodiscover-projects=["api","web"]"#,
            "--autodiscover-topics=renovate,dependencies",
            "--docker-max-pages=7",
            "--delete-config-file",
            "--s3-endpoint=https://s3.example",
            "--s3-path-style=true",
            "--repository-cache-force-local=false",
            "--repository-cache=enabled",
            "--repository-cache-type=s3",
            "--cache-hard-ttl-minutes=1440",
            "--cache-private-packages=true",
            "--preset-cache-persistence",
            "--include-mirrors",
            "--github-token-warn=false",
            "--ignore-pr-author",
            "--report-type=file",
            "--report-path=./report.json",
            "--local-dir=/tmp/renovate/repo",
            "--encrypted-warning=encrypted config ignored",
            "--bb-use-development-branch",
            "--pr-cache-sync-max-pages=5",
        ]);

        assert_eq!(config.allow_plugins, Some(true));
        assert_eq!(config.allow_scripts, Some(false));
        assert_eq!(
            config.allow_shell_executor_for_post_upgrade_commands,
            Some(true)
        );
        assert_eq!(config.allow_custom_crate_registries, Some(true));
        assert_eq!(
            config.allowed_headers,
            Some(vec!["X-*".to_owned(), "Authorization".to_owned()])
        );
        assert_eq!(
            config.allowed_env,
            Some(vec!["SOME_*".to_owned(), "OTHER_*".to_owned()])
        );
        assert_eq!(
            config.allowed_unsafe_executions,
            Some(vec!["bazelModDeps".to_owned(), "goGenerate".to_owned()])
        );
        assert_eq!(config.expose_all_env, Some(true));
        assert_eq!(config.detect_global_manager_config, Some(true));
        assert_eq!(config.detect_host_rules_from_env, Some(false));
        assert_eq!(
            config.merge_confidence_endpoint.as_deref(),
            Some("https://mc.example")
        );
        assert_eq!(config.merge_confidence_datasources, vec!["docker", "npm"]);
        assert_eq!(config.autodiscover_repo_sort.as_deref(), Some("updated"));
        assert_eq!(config.autodiscover_repo_order.as_deref(), Some("desc"));
        assert_eq!(config.autodiscover, Some(true));
        assert_eq!(
            config.autodiscover_filter,
            Some(vec!["org/*".to_owned(), "!org/archived".to_owned()])
        );
        assert_eq!(
            config.autodiscover_namespaces,
            Some(vec!["backend".to_owned(), "frontend".to_owned()])
        );
        assert_eq!(
            config.autodiscover_projects,
            Some(vec!["api".to_owned(), "web".to_owned()])
        );
        assert_eq!(
            config.autodiscover_topics,
            Some(vec!["renovate".to_owned(), "dependencies".to_owned()])
        );
        assert_eq!(config.docker_max_pages, Some(7));
        assert!(config.delete_config_file);
        assert_eq!(config.s3_endpoint.as_deref(), Some("https://s3.example"));
        assert!(config.s3_path_style);
        assert_eq!(config.repository_cache_force_local, Some(false));
        assert_eq!(config.repository_cache.as_deref(), Some("enabled"));
        assert_eq!(config.repository_cache_type.as_deref(), Some("s3"));
        assert_eq!(config.cache_hard_ttl_minutes, Some(1440));
        assert_eq!(config.cache_private_packages, Some(true));
        assert_eq!(config.preset_cache_persistence, Some(true));
        assert_eq!(config.include_mirrors, Some(true));
        assert_eq!(config.github_token_warn, Some(false));
        assert_eq!(config.ignore_pr_author, Some(true));
        assert_eq!(config.local_dir.as_deref(), Some("/tmp/renovate/repo"));
        assert_eq!(
            config.encrypted_warning.as_deref(),
            Some("encrypted config ignored")
        );
        assert_eq!(config.bb_use_development_branch, Some(true));
        assert_eq!(config.pr_cache_sync_max_pages, Some(5));
        assert_eq!(config.report_type.as_deref(), Some("file"));
        assert_eq!(config.report_path.as_deref(), Some("./report.json"));
    }

    // Ported: "supports boolean no value" — workers/global/config/parse/cli.spec.ts line 36
    #[test]
    fn config_migration_bare_sets_true() {
        assert!(parse_and_build(&["--config-migration"]).config_migration);
    }

    #[test]
    fn print_config_flag_is_parsed() {
        assert_eq!(parse_and_build(&["--print-config"]).print_config, Some(true));
        assert_eq!(
            parse_and_build(&["--print-config=false"]).print_config,
            Some(false)
        );
    }

    #[test]
    fn onboarding_flags_are_parsed() {
        let config = parse_and_build(&[
            "--onboarding=false",
            "--config-file-names=renovate.json,.github/renovate.json5",
            r#"--migrate-presets={"config:old":"config:new","config:removed":""}"#,
            r#"--custom-env-variables={"EXAMPLE":"value"}"#,
            r#"--cache-ttl-override={"datasource-npm":30}"#,
            "--onboarding-no-deps=enabled",
            "--onboarding-rebase-checkbox",
            "--pr-commits-per-run-limit=4",
        ]);

        assert_eq!(config.onboarding, Some(false));
        assert_eq!(
            config.config_file_names,
            Some(vec![
                "renovate.json".to_owned(),
                ".github/renovate.json5".to_owned()
            ])
        );
        assert_eq!(
            config.migrate_presets.get("config:old").map(String::as_str),
            Some("config:new")
        );
        assert_eq!(
            config.migrate_presets.get("config:removed").map(String::as_str),
            Some("")
        );
        assert_eq!(
            config.custom_env_variables.get("EXAMPLE").map(String::as_str),
            Some("value")
        );
        assert_eq!(
            config
                .cache_ttl_override
                .get("datasource-npm")
                .and_then(serde_json::Value::as_u64),
            Some(30)
        );
        assert_eq!(config.onboarding_no_deps.as_deref(), Some("enabled"));
        assert_eq!(config.onboarding_rebase_checkbox, Some(true));
        assert_eq!(config.pr_commits_per_run_limit, Some(4));
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
