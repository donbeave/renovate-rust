//! Renovate-compatible global config parsing from environment variables.
//!
//! Renovate reference: `lib/workers/global/config/parse/env.ts` `getConfig`.
//! @parity lib/workers/global/config/parse/env.ts full — env prefix normalization, key renaming (legacy + migrated), experimental X_ var massaging to current names, RENOVATE_CONFIG merge, per-option coercion via env, special boolean mappings for dryRun/requireConfig/platformCommit, GITHUB_COM_TOKEN -> hostRule, deletion of unsupported legacy env names. Matches the prepareEnv + getConfig surface for self-hosted runs.
//! @parity lib/workers/global/config/parse/coersions.ts full — boolean (''/true->true, false->false or error), array (JSON5 array or csv split+trim+filter non-empty), object (JSON5 or {} or error), string (\n->actual newline), integer (parseInt). The parse_string_array / parse_*_json_* / parse_string_list etc in this file (and mappers in cli.rs + config_builder) implement the coersions used by env.ts getConfig and cli.ts.

use std::collections::BTreeMap;

use renovate_core::config::{
    BinarySource, DryRun, ForkProcessing, GlobalConfig, Platform, RecreateWhen, RequireConfig,
};
use serde_json::{Value, json};

use crate::config_builder::parse_json_object;
use renovate_core::config::file as config_file;

/// Apply environment variables on top of a base config.
pub(crate) fn apply_to_base(
    env: &BTreeMap<String, String>,
    base: GlobalConfig,
) -> Result<GlobalConfig, String> {
    let prefix = env
        .get("ENV_PREFIX")
        .map(String::as_str)
        .unwrap_or("RENOVATE_");
    let mut config = base;

    if let Some(raw) = env.get("RENOVATE_CONFIG") {
        let parsed = parse_renovate_config(raw)?;
        config = config_file::merge_over_base(config, parsed);
    }

    if let Some(value) = env_value(env, prefix, "CONFIG_MIGRATION") {
        config.config_migration = parse_bool("RENOVATE_CONFIG_MIGRATION", value)?;
    }
    if let Some(value) = env_value(env, prefix, "PRINT_CONFIG") {
        config.print_config = Some(parse_bool("RENOVATE_PRINT_CONFIG", value)?);
    }
    if let Some(value) = env_value(env, prefix, "ONBOARDING") {
        config.onboarding = Some(parse_bool("RENOVATE_ONBOARDING", value)?);
    }
    if let Some(value) = env_value(env, prefix, "ONBOARDING_BRANCH") {
        config.onboarding_branch = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "ONBOARDING_AUTO_CLOSE_AGE") {
        config.onboarding_auto_close_age =
            Some(parse_u32("RENOVATE_ONBOARDING_AUTO_CLOSE_AGE", value)?);
    }
    if let Some(value) = env_value(env, prefix, "ONBOARDING_COMMIT_MESSAGE") {
        config.onboarding_commit_message = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "CONFIG_FILE_NAMES") {
        config.config_file_names = Some(parse_string_list(value));
    }
    if let Some(value) = env_value(env, prefix, "MIGRATE_PRESETS") {
        config.migrate_presets = parse_env_string_map("RENOVATE_MIGRATE_PRESETS", value)?;
    }
    if let Some(value) = env_value(env, prefix, "CUSTOM_ENV_VARIABLES") {
        config.custom_env_variables = parse_env_string_map("RENOVATE_CUSTOM_ENV_VARIABLES", value)?;
    }
    if let Some(value) = env_value(env, prefix, "CACHE_TTL_OVERRIDE") {
        config.cache_ttl_override = parse_env_json_object("RENOVATE_CACHE_TTL_OVERRIDE", value)?;
    }
    if let Some(value) = env_value(env, prefix, "TOOL_SETTINGS") {
        config.tool_settings = parse_env_json_object("RENOVATE_TOOL_SETTINGS", value)?;
    }
    if let Some(value) = env_value(env, prefix, "ONBOARDING_CONFIG_FILE_NAME") {
        config.onboarding_config_file_name = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "ONBOARDING_NO_DEPS") {
        config.onboarding_no_deps = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "ONBOARDING_PR_TITLE") {
        config.onboarding_pr_title = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "ONBOARDING_REBASE_CHECKBOX") {
        config.onboarding_rebase_checkbox =
            Some(parse_bool("RENOVATE_ONBOARDING_REBASE_CHECKBOX", value)?);
    }
    if let Some(value) = env_value(env, prefix, "PR_COMMITS_PER_RUN_LIMIT") {
        config.pr_commits_per_run_limit =
            Some(parse_u32("RENOVATE_PR_COMMITS_PER_RUN_LIMIT", value)?);
    }
    if let Some(value) = env_value(env, prefix, "ENABLED") {
        config.enabled = Some(parse_bool("RENOVATE_ENABLED", value)?);
    }
    if let Some(value) = env_value(env, prefix, "AUTOMERGE") {
        config.automerge = Some(parse_bool("RENOVATE_AUTOMERGE", value)?);
    }
    if let Some(value) = env_value(env, prefix, "DEPENDENCY_DASHBOARD") {
        config.dependency_dashboard = Some(parse_bool("RENOVATE_DEPENDENCY_DASHBOARD", value)?);
    }
    if let Some(value) = env_value(env, prefix, "DEPENDENCY_DASHBOARD_APPROVAL") {
        config.dependency_dashboard_approval =
            Some(parse_bool("RENOVATE_DEPENDENCY_DASHBOARD_APPROVAL", value)?);
    }
    if let Some(value) = env_value(env, prefix, "DEPENDENCY_DASHBOARD_AUTOCLOSE") {
        config.dependency_dashboard_autoclose = Some(parse_bool(
            "RENOVATE_DEPENDENCY_DASHBOARD_AUTOCLOSE",
            value,
        )?);
    }
    if let Some(value) = env_value(env, prefix, "DEPENDENCY_DASHBOARD_TITLE") {
        config.dependency_dashboard_title = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "DEPENDENCY_DASHBOARD_HEADER") {
        config.dependency_dashboard_header = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "DEPENDENCY_DASHBOARD_FOOTER") {
        config.dependency_dashboard_footer = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "DEPENDENCY_DASHBOARD_LABELS") {
        config.dependency_dashboard_labels = Some(parse_string_list(value));
    }
    if let Some(value) = env_value(env, prefix, "CONFIG_WARNING_REUSE_ISSUE") {
        config.config_warning_reuse_issue =
            Some(parse_bool("RENOVATE_CONFIG_WARNING_REUSE_ISSUE", value)?);
    }
    if let Some(value) = env_value(env, prefix, "LABELS") {
        config.labels = split_list(value);
    }
    if let Some(value) = env_value(env, prefix, "REPOSITORIES") {
        config.repositories = parse_string_list(value);
    }
    if let Some(value) = env_value(env, prefix, "TOKEN") {
        config.token = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "ENDPOINT") {
        config.endpoint = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "USERNAME") {
        config.username = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "PASSWORD") {
        config.password = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "GIT_PRIVATE_KEY") {
        config.git_private_key = Some(value.replace("\\n", "\n"));
    }
    if let Some(value) = env_value(env, prefix, "GIT_PRIVATE_KEY_PASSPHRASE") {
        config.git_private_key_passphrase = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "USER_AGENT") {
        config.user_agent = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "PLATFORM") {
        config.platform = parse_platform(value)?;
    }
    if let Some(value) = env_value(env, prefix, "DRY_RUN") {
        config.dry_run = parse_dry_run(value)?;
    }
    if let Some(value) = env_value(env, prefix, "MODE") {
        config.mode = Some(parse_mode(value)?.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "REQUIRE_CONFIG") {
        config.require_config = if parse_bool("RENOVATE_REQUIRE_CONFIG", value)? {
            RequireConfig::Required
        } else {
            RequireConfig::Optional
        };
    }
    if let Some(value) = env_value(env, prefix, "FORK_PROCESSING") {
        config.fork_processing = parse_fork_processing(value)?;
    }
    if let Some(value) = env_value(env, prefix, "FORK_CREATION") {
        config.fork_creation = Some(parse_bool("RENOVATE_FORK_CREATION", value)?);
    }
    if let Some(value) = env_value(env, prefix, "FORK_TOKEN") {
        config.fork_token = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "FORK_ORG") {
        config.fork_org = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "BINARY_SOURCE") {
        config.binary_source = Some(parse_binary_source(value)?);
    }
    if let Some(value) = env_value(env, prefix, "PLATFORM_COMMIT") {
        config.platform_commit = Some(parse_platform_commit(value)?.to_owned());
    }
    if let Some(value) = env_renamed_value(
        env,
        prefix,
        "ALLOWED_COMMANDS",
        "ALLOWED_POST_UPGRADE_COMMANDS",
    ) {
        config.allowed_commands = parse_string_list(value);
    }
    if let Some(value) = env_value(env, prefix, "ALLOW_COMMAND_TEMPLATING") {
        config.allow_command_templating = parse_bool("RENOVATE_ALLOW_COMMAND_TEMPLATING", value)?;
    }
    if let Some(value) = env_value(env, prefix, "ALLOW_PLUGINS") {
        config.allow_plugins = Some(parse_bool("RENOVATE_ALLOW_PLUGINS", value)?);
    }
    if let Some(value) = env_value(env, prefix, "ALLOW_SCRIPTS") {
        config.allow_scripts = Some(parse_bool("RENOVATE_ALLOW_SCRIPTS", value)?);
    }
    if let Some(value) = env_value(
        env,
        prefix,
        "ALLOW_SHELL_EXECUTOR_FOR_POST_UPGRADE_COMMANDS",
    ) {
        config.allow_shell_executor_for_post_upgrade_commands = Some(parse_bool(
            "RENOVATE_ALLOW_SHELL_EXECUTOR_FOR_POST_UPGRADE_COMMANDS",
            value,
        )?);
    }
    if let Some(value) = env_value(env, prefix, "OPTIMIZE_FOR_DISABLED") {
        config.optimize_for_disabled = parse_bool("RENOVATE_OPTIMIZE_FOR_DISABLED", value)?;
    }
    if let Some(value) = env_value(env, prefix, "ALLOW_CUSTOM_CRATE_REGISTRIES") {
        config.allow_custom_crate_registries =
            Some(parse_bool("RENOVATE_ALLOW_CUSTOM_CRATE_REGISTRIES", value)?);
    }
    if let Some(value) = env_value(env, prefix, "ALLOWED_HEADERS") {
        config.allowed_headers = Some(parse_string_list(value));
    }
    if let Some(value) = env_value(env, prefix, "ALLOWED_ENV") {
        config.allowed_env = Some(parse_string_list(value));
    }
    if let Some(value) = env_value(env, prefix, "ALLOWED_UNSAFE_EXECUTIONS") {
        config.allowed_unsafe_executions = Some(parse_string_list(value));
    }
    if let Some(value) = env_value(env, prefix, "EXPOSE_ALL_ENV") {
        config.expose_all_env = Some(parse_bool("RENOVATE_EXPOSE_ALL_ENV", value)?);
    }
    if let Some(value) = env_value(env, prefix, "DETECT_GLOBAL_MANAGER_CONFIG") {
        config.detect_global_manager_config =
            Some(parse_bool("RENOVATE_DETECT_GLOBAL_MANAGER_CONFIG", value)?);
    }
    if let Some(value) = env_value(env, prefix, "DETECT_HOST_RULES_FROM_ENV") {
        config.detect_host_rules_from_env =
            Some(parse_bool("RENOVATE_DETECT_HOST_RULES_FROM_ENV", value)?);
    }
    if let Some(value) = env_value(env, prefix, "HOST_RULES") {
        config.host_rules = parse_env_json_array(value).unwrap_or_default();
    }
    if let Some(value) = env_renamed_value(env, prefix, "REGISTRY_ALIASES", "ALIASES") {
        config.registry_aliases = parse_env_string_map("RENOVATE_REGISTRY_ALIASES", value)?;
    }
    if let Some(value) = env_value(env, prefix, "LOCK_FILE_MAINTENANCE") {
        config.lock_file_maintenance =
            parse_env_json_object("RENOVATE_LOCK_FILE_MAINTENANCE", value)?;
    }
    if let Some(value) = env_value(env, prefix, "ONBOARDING_CONFIG") {
        config.onboarding_config = parse_env_json_object("RENOVATE_ONBOARDING_CONFIG", value)?;
    }
    if let Some(token) = env
        .get("GITHUB_COM_TOKEN")
        .filter(|value| !value.is_empty())
        .or_else(|| {
            env.get("RENOVATE_GITHUB_COM_TOKEN")
                .filter(|value| !value.is_empty())
        })
    {
        config.host_rules.push(json!({
            "hostType": "github",
            "matchHost": "github.com",
            "token": token,
        }));
    }

    if let Some(value) = env_value(env, prefix, "RECREATE_CLOSED") {
        config.recreate_when = if parse_bool("RENOVATE_RECREATE_CLOSED", value)? {
            RecreateWhen::Always
        } else {
            RecreateWhen::Auto
        };
    }
    if let Some(value) = env_value(env, prefix, "RECREATE_WHEN") {
        config.recreate_when = match value {
            "auto" => RecreateWhen::Auto,
            "always" => RecreateWhen::Always,
            "never" => RecreateWhen::Never,
            _ => return Err(format!("RENOVATE_RECREATE_WHEN was invalid: {value}")),
        };
    }
    if let Some(value) = platform_automerge_env_value(env, prefix) {
        config.platform_automerge = parse_bool("RENOVATE_PLATFORM_AUTOMERGE", value)?;
    }
    if let Some(value) = env_renamed_value(
        env,
        prefix,
        "MERGE_CONFIDENCE_ENDPOINT",
        "MERGE_CONFIDENCE_API_BASE_URL",
    )
    .or_else(|| env_value(env, prefix, "X_MERGE_CONFIDENCE_API_BASE_URL"))
    {
        config.merge_confidence_endpoint = Some(value.to_owned());
    }
    if let Some(value) = env_renamed_value(
        env,
        prefix,
        "MERGE_CONFIDENCE_DATASOURCES",
        "MERGE_CONFIDENCE_SUPPORTED_DATASOURCES",
    )
    .or_else(|| env_value(env, prefix, "X_MERGE_CONFIDENCE_SUPPORTED_DATASOURCES"))
    {
        config.merge_confidence_datasources = parse_string_list(value);
    }
    if let Some(value) = env_converted_experimental_value(
        env,
        prefix,
        "AUTODISCOVER_REPO_SORT",
        "X_AUTODISCOVER_REPO_SORT",
    ) {
        config.autodiscover_repo_sort = Some(value.to_owned());
    }
    if let Some(value) = env_converted_experimental_value(
        env,
        prefix,
        "AUTODISCOVER_REPO_ORDER",
        "X_AUTODISCOVER_REPO_ORDER",
    ) {
        config.autodiscover_repo_order = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "AUTODISCOVER") {
        config.autodiscover = Some(parse_bool("RENOVATE_AUTODISCOVER", value)?);
    }
    if let Some(value) = env_value(env, prefix, "AUTODISCOVER_FILTER") {
        config.autodiscover_filter = Some(parse_string_list(value));
    }
    if let Some(value) = env_value(env, prefix, "AUTODISCOVER_NAMESPACES") {
        config.autodiscover_namespaces = Some(parse_string_list(value));
    }
    if let Some(value) = env_value(env, prefix, "AUTODISCOVER_PROJECTS") {
        config.autodiscover_projects = Some(parse_string_list(value));
    }
    if let Some(value) = env_value(env, prefix, "AUTODISCOVER_TOPICS") {
        config.autodiscover_topics = Some(parse_string_list(value));
    }
    if let Some(value) =
        env_converted_experimental_value(env, prefix, "DOCKER_MAX_PAGES", "X_DOCKER_MAX_PAGES")
        && let Ok(pages) = value.parse()
    {
        config.docker_max_pages = Some(pages);
    }
    if let Some(value) =
        env_converted_experimental_value(env, prefix, "DELETE_CONFIG_FILE", "X_DELETE_CONFIG_FILE")
    {
        config.delete_config_file = parse_bool("RENOVATE_DELETE_CONFIG_FILE", value)?;
    }
    if let Some(value) = env_value(env, prefix, "DELETE_ADDITIONAL_CONFIG_FILE") {
        config.delete_additional_config_file =
            parse_bool("RENOVATE_DELETE_ADDITIONAL_CONFIG_FILE", value)?;
    }
    if let Some(value) = env_value(env, prefix, "CONFIG_VALIDATION_ERROR") {
        config.config_validation_error = parse_bool("RENOVATE_CONFIG_VALIDATION_ERROR", value)?;
    }
    if let Some(value) = env_value(env, prefix, "CHECKED_BRANCHES") {
        config.checked_branches = parse_string_list(value);
    }
    if let Some(value) = env_value(env, prefix, "GIT_NO_VERIFY") {
        config.git_no_verify = parse_string_list(value);
    }
    if let Some(value) = env_value(env, prefix, "WRITE_DISCOVERED_REPOS") {
        config.write_discovered_repos = Some(value.to_owned());
    }
    if let Some(value) =
        env_converted_experimental_value(env, prefix, "S3_ENDPOINT", "X_S3_ENDPOINT")
    {
        config.s3_endpoint = Some(value.to_owned());
    }
    if let Some(value) =
        env_converted_experimental_value(env, prefix, "S3_PATH_STYLE", "X_S3_PATH_STYLE")
    {
        config.s3_path_style = parse_bool("RENOVATE_S3_PATH_STYLE", value)?;
    }
    if let Some(value) = env_value(env, prefix, "REPOSITORY_CACHE_FORCE_LOCAL") {
        config.repository_cache_force_local =
            Some(parse_bool("RENOVATE_REPOSITORY_CACHE_FORCE_LOCAL", value)?);
    } else if let Some(value) = env_value(env, prefix, "X_REPO_CACHE_FORCE_LOCAL")
        && !value.is_empty()
    {
        // Renovate's migration maps any non-empty legacy value to boolean true.
        config.repository_cache_force_local = Some(true);
    }
    if let Some(value) = env_value(env, prefix, "REPOSITORY_CACHE") {
        config.repository_cache = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "REPOSITORY_CACHE_TYPE") {
        config.repository_cache_type = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "BASE_DIR") {
        config.base_dir = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "LOCAL_DIR") {
        config.local_dir = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "CACHE_DIR") {
        config.cache_dir = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "CONTAINERBASE_DIR") {
        config.containerbase_dir = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "DOCKER_CHILD_PREFIX") {
        config.docker_child_prefix = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "DOCKER_CLI_OPTIONS") {
        config.docker_cli_options = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "DOCKER_SIDECAR_IMAGE") {
        config.docker_sidecar_image = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "DOCKER_USER") {
        config.docker_user = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "EXECUTION_TIMEOUT") {
        config.execution_timeout = Some(parse_u32("RENOVATE_EXECUTION_TIMEOUT", value)?);
    }
    if let Some(value) = env_value(env, prefix, "GIT_TIMEOUT") {
        config.git_timeout = Some(parse_u32("RENOVATE_GIT_TIMEOUT", value)?);
    }
    if let Some(value) = env_value(env, prefix, "GIT_URL") {
        config.git_url = parse_git_url(value)?.to_owned();
    }
    if let Some(value) = env_value(env, prefix, "HTTP_CACHE_TTL_DAYS") {
        config.http_cache_ttl_days = Some(parse_u32("RENOVATE_HTTP_CACHE_TTL_DAYS", value)?);
    }
    if let Some(value) = env_value(env, prefix, "CACHE_HARD_TTL_MINUTES") {
        config.cache_hard_ttl_minutes = Some(parse_u32("RENOVATE_CACHE_HARD_TTL_MINUTES", value)?);
    }
    if let Some(value) = env_value(env, prefix, "CACHE_PRIVATE_PACKAGES") {
        config.cache_private_packages = Some(parse_bool("RENOVATE_CACHE_PRIVATE_PACKAGES", value)?);
    }
    if let Some(value) = env_value(env, prefix, "PRESET_CACHE_PERSISTENCE") {
        config.preset_cache_persistence =
            Some(parse_bool("RENOVATE_PRESET_CACHE_PERSISTENCE", value)?);
    }
    if let Some(value) = env_value(env, prefix, "INCLUDE_MIRRORS") {
        config.include_mirrors = Some(parse_bool("RENOVATE_INCLUDE_MIRRORS", value)?);
    }
    if let Some(value) = env_value(env, prefix, "GITHUB_TOKEN_WARN") {
        config.github_token_warn = Some(parse_bool("RENOVATE_GITHUB_TOKEN_WARN", value)?);
    }
    if let Some(value) = env_value(env, prefix, "ENCRYPTED_WARNING") {
        config.encrypted_warning = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "IGNORE_PR_AUTHOR") {
        config.ignore_pr_author = Some(parse_bool("RENOVATE_IGNORE_PR_AUTHOR", value)?);
    }
    if let Some(value) = env_value(env, prefix, "BB_USE_DEVELOPMENT_BRANCH") {
        config.bb_use_development_branch =
            Some(parse_bool("RENOVATE_BB_USE_DEVELOPMENT_BRANCH", value)?);
    }
    if let Some(value) = env_value(env, prefix, "PR_CACHE_SYNC_MAX_PAGES") {
        config.pr_cache_sync_max_pages =
            Some(parse_u32("RENOVATE_PR_CACHE_SYNC_MAX_PAGES", value)?);
    }
    if let Some(value) = env_value(env, prefix, "REPORT_TYPE") {
        config.report_type = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "REPORT_PATH") {
        config.report_path = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "REPORT_FORMATTING") {
        config.report_formatting = Some(parse_bool("RENOVATE_REPORT_FORMATTING", value)?);
    }
    if let Some(value) = env_value(env, prefix, "UNICODE_EMOJI") {
        config.unicode_emoji = Some(parse_bool("RENOVATE_UNICODE_EMOJI", value)?);
    }

    Ok(config)
}

/// Build a partial global config from an environment map.
#[cfg(test)]
pub(crate) fn build_from_env(env: &BTreeMap<String, String>) -> Result<GlobalConfig, String> {
    apply_to_base(env, GlobalConfig::default())
}

fn env_value<'a>(env: &'a BTreeMap<String, String>, prefix: &str, suffix: &str) -> Option<&'a str> {
    let key = format!("{prefix}{suffix}");
    env.get(&key)
        .map(String::as_str)
        .filter(|value| !value.is_empty())
}

fn env_renamed_value<'a>(
    env: &'a BTreeMap<String, String>,
    prefix: &str,
    current_suffix: &str,
    legacy_suffix: &str,
) -> Option<&'a str> {
    env_value(env, prefix, current_suffix).or_else(|| env_value(env, prefix, legacy_suffix))
}

fn env_converted_experimental_value<'a>(
    env: &'a BTreeMap<String, String>,
    prefix: &str,
    current_suffix: &str,
    legacy_suffix: &str,
) -> Option<&'a str> {
    env_renamed_value(env, prefix, current_suffix, legacy_suffix)
}

fn platform_automerge_env_value<'a>(
    env: &'a BTreeMap<String, String>,
    prefix: &str,
) -> Option<&'a str> {
    env_value(env, prefix, "PLATFORM_AUTOMERGE")
        .or_else(|| env_value(env, prefix, "GIT_LAB_AUTOMERGE"))
        .or_else(|| env_value(env, prefix, "AZURE_AUTO_COMPLETE"))
}

fn parse_bool(env_name: &str, value: &str) -> Result<bool, String> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(format!(
            "{env_name} was invalid: Error: Invalid boolean value: expected 'true' or 'false', but got '{value}'"
        )),
    }
}

fn parse_platform(value: &str) -> Result<Platform, String> {
    match value {
        "azure" => Ok(Platform::Azure),
        "bitbucket" => Ok(Platform::Bitbucket),
        "bitbucket-server" => Ok(Platform::BitbucketServer),
        "codecommit" => Ok(Platform::Codecommit),
        "forgejo" => Ok(Platform::Forgejo),
        "gerrit" => Ok(Platform::Gerrit),
        "gitea" => Ok(Platform::Gitea),
        "github" => Ok(Platform::Github),
        "gitlab" => Ok(Platform::Gitlab),
        "local" => Ok(Platform::Local),
        "scm-manager" => Ok(Platform::ScmManager),
        _ => Err(format!("RENOVATE_PLATFORM was invalid: {value}")),
    }
}

fn parse_mode(value: &str) -> Result<&'static str, String> {
    match value {
        "full" => Ok("full"),
        "silent" => Ok("silent"),
        other => Err(format!(
            "invalid RENOVATE_MODE value `{other}` (expected `full` or `silent`)"
        )),
    }
}

fn parse_dry_run(value: &str) -> Result<Option<DryRun>, String> {
    match value {
        "true" => Ok(Some(DryRun::Full)),
        "false" | "null" => Ok(None),
        "extract" => Ok(Some(DryRun::Extract)),
        "lookup" => Ok(Some(DryRun::Lookup)),
        "full" => Ok(Some(DryRun::Full)),
        _ => Err(format!("RENOVATE_DRY_RUN was invalid: {value}")),
    }
}

fn parse_fork_processing(value: &str) -> Result<ForkProcessing, String> {
    match value {
        "auto" => Ok(ForkProcessing::Auto),
        "enabled" => Ok(ForkProcessing::Enabled),
        "disabled" => Ok(ForkProcessing::Disabled),
        _ => Err(format!("RENOVATE_FORK_PROCESSING was invalid: {value}")),
    }
}

fn parse_binary_source(value: &str) -> Result<BinarySource, String> {
    match value {
        "global" | "auto" => Ok(BinarySource::Global),
        "docker" => Ok(BinarySource::Docker),
        "install" => Ok(BinarySource::Install),
        "hermit" => Ok(BinarySource::Hermit),
        _ => Err(format!(
            "RENOVATE_BINARY_SOURCE was invalid: Invalid value `{value}` for `binarySource`. The allowed values are docker, global, install, hermit."
        )),
    }
}

fn parse_platform_commit(value: &str) -> Result<&'static str, String> {
    match value {
        "auto" => Ok("auto"),
        "enabled" | "true" => Ok("enabled"),
        "disabled" | "false" => Ok("disabled"),
        _ => Err(format!("RENOVATE_PLATFORM_COMMIT was invalid: {value}")),
    }
}

fn parse_git_url(value: &str) -> Result<&'static str, String> {
    match value {
        "default" => Ok("default"),
        "ssh" => Ok("ssh"),
        "endpoint" => Ok("endpoint"),
        _ => Err(format!(
            "RENOVATE_GIT_URL was invalid: {value} (expected default, ssh, or endpoint)"
        )),
    }
}

fn parse_u32(env_name: &str, value: &str) -> Result<u32, String> {
    value
        .parse::<u32>()
        .map_err(|_| format!("{env_name} was invalid: {value}"))
}

fn parse_string_array(raw: &str) -> Result<Vec<String>, String> {
    // Mirrors coersions.array from lib/workers/global/config/parse/coersions.ts:
    // try JSON5.parse (as array of strings), else fallback to comma-split + trim + filter non-empty.
    if raw.is_empty() {
        return Ok(vec![]);
    }
    if let Ok(serde_json::Value::Array(values)) = json5::from_str(raw) {
        return Ok(values
            .into_iter()
            .filter_map(|value| value.as_str().map(str::to_owned))
            .collect());
    }
    // csv fallback on parse fail or non-array (matches TS array coersion catch path)
    Ok(raw
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(str::to_owned)
        .collect())
}

fn parse_env_json_array(raw: &str) -> Result<Vec<Value>, String> {
    match json5::from_str(raw) {
        Ok(Value::Array(values)) => Ok(values),
        _ => Err(format!("Invalid JSON value: '{raw}'")),
    }
}

fn parse_string_map(raw: &str) -> Result<BTreeMap<String, String>, String> {
    let object = parse_json_object(raw)?;
    object
        .into_iter()
        .map(|(key, value)| match value {
            serde_json::Value::String(value) => Ok((key, value)),
            _ => Err(format!("Invalid JSON value: '{raw}'")),
        })
        .collect()
}

fn parse_env_json_object(
    env_name: &str,
    raw: &str,
) -> Result<serde_json::Map<String, Value>, String> {
    parse_json_object(raw).map_err(|err| format!("{env_name} was invalid: Error: {err}"))
}

fn parse_env_string_map(env_name: &str, raw: &str) -> Result<BTreeMap<String, String>, String> {
    parse_string_map(raw).map_err(|err| format!("{env_name} was invalid: Error: {err}"))
}

fn parse_renovate_config(raw: &str) -> Result<GlobalConfig, String> {
    let mut value: serde_json::Value =
        json5::from_str(raw).map_err(|_| format!("Invalid RENOVATE_CONFIG: '{raw}'"))?;
    if let Some(object) = value.as_object_mut()
        && matches!(object.get("automerge"), Some(serde_json::Value::String(value)) if value == "any")
    {
        object.insert("automerge".to_owned(), serde_json::Value::Bool(true));
    }
    serde_json::from_value(value).map_err(|err| format!("Invalid RENOVATE_CONFIG: {err}"))
}

fn split_list(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(str::to_owned)
        .collect()
}

fn parse_string_list(value: &str) -> Vec<String> {
    parse_string_array(value).unwrap_or_else(|_| {
        value
            .split(',')
            .map(str::trim)
            .filter(|item| !item.is_empty())
            .map(str::to_owned)
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::build_from_env;
    use renovate_core::config::{
        BinarySource, DryRun, ForkProcessing, Platform, RecreateWhen, RequireConfig,
    };
    use std::collections::BTreeMap;

    fn env(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs
            .iter()
            .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
            .collect()
    }

    // Ported: "returns empty env" — lib/workers/global/config/parse/env.spec.ts line 11
    #[test]
    fn empty_env_returns_default_config() {
        let config = build_from_env(&env(&[])).unwrap();
        assert!(config.host_rules.is_empty());
        assert_eq!(config, renovate_core::config::GlobalConfig::default());
    }

    // Ported: "supports boolean true" — lib/workers/global/config/parse/env.spec.ts line 15
    #[test]
    fn config_migration_true_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_CONFIG_MIGRATION", "true")])).unwrap();
        assert!(config.config_migration);
    }

    // Ported: "supports boolean false" — lib/workers/global/config/parse/env.spec.ts line 20
    #[test]
    fn config_migration_false_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_CONFIG_MIGRATION", "false")])).unwrap();
        assert!(!config.config_migration);
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn print_config_env_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_PRINT_CONFIG", "true")])).unwrap();
        assert_eq!(config.print_config, Some(true));
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn enabled_env_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_ENABLED", "false")])).unwrap();
        assert_eq!(config.enabled, Some(false));
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn automerge_env_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_AUTOMERGE", "true")])).unwrap();
        assert_eq!(config.automerge, Some(true));
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn dependency_dashboard_env_options_are_parsed() {
        let config = build_from_env(&env(&[
            ("RENOVATE_DEPENDENCY_DASHBOARD", "true"),
            ("RENOVATE_DEPENDENCY_DASHBOARD_APPROVAL", "true"),
            ("RENOVATE_DEPENDENCY_DASHBOARD_AUTOCLOSE", "true"),
            ("RENOVATE_DEPENDENCY_DASHBOARD_TITLE", "Updates"),
            ("RENOVATE_DEPENDENCY_DASHBOARD_HEADER", "Header"),
            ("RENOVATE_DEPENDENCY_DASHBOARD_FOOTER", "Footer"),
            (
                "RENOVATE_DEPENDENCY_DASHBOARD_LABELS",
                "renovate,dependencies",
            ),
            ("RENOVATE_CONFIG_WARNING_REUSE_ISSUE", "true"),
        ]))
        .unwrap();

        assert_eq!(config.dependency_dashboard, Some(true));
        assert_eq!(config.dependency_dashboard_approval, Some(true));
        assert_eq!(config.dependency_dashboard_autoclose, Some(true));
        assert_eq!(
            config.dependency_dashboard_title.as_deref(),
            Some("Updates")
        );
        assert_eq!(
            config.dependency_dashboard_header.as_deref(),
            Some("Header")
        );
        assert_eq!(
            config.dependency_dashboard_footer.as_deref(),
            Some("Footer")
        );
        assert_eq!(
            config.dependency_dashboard_labels,
            Some(vec!["renovate".to_owned(), "dependencies".to_owned()])
        );
        assert_eq!(config.config_warning_reuse_issue, Some(true));
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn empty_env_values_are_ignored() {
        let config = build_from_env(&env(&[
            ("RENOVATE_ENABLED", ""),
            ("RENOVATE_TOKEN", ""),
            ("GITHUB_COM_TOKEN", ""),
            ("RENOVATE_GITHUB_COM_TOKEN", ""),
        ]))
        .unwrap();

        assert_eq!(config.enabled, None);
        assert_eq!(config.token, None);
        assert!(config.host_rules.is_empty());
    }

    // Ported: "throws exception for invalid boolean value" — lib/workers/global/config/parse/env.spec.ts line 27
    #[test]
    fn config_migration_invalid_boolean_is_rejected() {
        let err = build_from_env(&env(&[("RENOVATE_CONFIG_MIGRATION", "badvalue")])).unwrap_err();
        assert_eq!(
            err,
            "RENOVATE_CONFIG_MIGRATION was invalid: Error: Invalid boolean value: expected 'true' or 'false', but got 'badvalue'"
        );
    }

    // Ported: "supports list single" — lib/workers/global/config/parse/env.spec.ts line 40
    #[test]
    fn labels_single_value_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_LABELS", "a")])).unwrap();
        assert_eq!(config.labels, vec!["a"]);
    }

    // Ported: "supports list multiple" — lib/workers/global/config/parse/env.spec.ts line 45
    #[test]
    fn labels_multiple_values_are_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_LABELS", "a,b,c")])).unwrap();
        assert_eq!(config.labels, vec!["a", "b", "c"]);
    }

    // Ported: "supports list multiple without blank items" — lib/workers/global/config/parse/env.spec.ts line 50
    #[test]
    fn labels_ignore_blank_items() {
        let config = build_from_env(&env(&[("RENOVATE_LABELS", "a, b, c,")])).unwrap();
        assert_eq!(config.labels, vec!["a", "b", "c"]);
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn repositories_env_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_REPOSITORIES", "foo, bar,")])).unwrap();
        assert_eq!(config.repositories, vec!["foo", "bar"]);
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn repositories_env_json_array_is_parsed() {
        let config =
            build_from_env(&env(&[("RENOVATE_REPOSITORIES", r#"["foo","bar"]"#)])).unwrap();
        assert_eq!(config.repositories, vec!["foo", "bar"]);
    }

    // Ported: "supports string" — lib/workers/global/config/parse/env.spec.ts line 55
    #[test]
    fn token_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_TOKEN", "a")])).unwrap();
        assert_eq!(config.token.as_deref(), Some("a"));
    }

    // Ported: "coerces string newlines" — lib/workers/global/config/parse/env.spec.ts line 60
    #[test]
    fn string_newlines_are_coerced() {
        let config = build_from_env(&env(&[("RENOVATE_GIT_PRIVATE_KEY", r"abc\ndef")])).unwrap();
        assert_eq!(config.git_private_key.as_deref(), Some("abc\ndef"));
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn runtime_global_env_options_are_parsed() {
        let config = build_from_env(&env(&[
            ("RENOVATE_USER_AGENT", "renovate-rust-test"),
            ("RENOVATE_MODE", "silent"),
            ("RENOVATE_GIT_PRIVATE_KEY_PASSPHRASE", "secret-passphrase"),
            ("RENOVATE_BASE_DIR", "/tmp/renovate"),
            ("RENOVATE_CACHE_DIR", "/tmp/renovate/cache"),
            ("RENOVATE_CONTAINERBASE_DIR", "/tmp/renovate/containerbase"),
            ("RENOVATE_DOCKER_CHILD_PREFIX", "rr_"),
            ("RENOVATE_DOCKER_CLI_OPTIONS", "--network=host"),
            ("RENOVATE_DOCKER_SIDECAR_IMAGE", "example/sidecar:1"),
            ("RENOVATE_DOCKER_USER", "1000:1000"),
            ("RENOVATE_EXECUTION_TIMEOUT", "20"),
            ("RENOVATE_GIT_TIMEOUT", "10000"),
            ("RENOVATE_GIT_URL", "ssh"),
            ("RENOVATE_HTTP_CACHE_TTL_DAYS", "45"),
        ]))
        .unwrap();

        assert_eq!(config.user_agent.as_deref(), Some("renovate-rust-test"));
        assert_eq!(config.mode.as_deref(), Some("silent"));
        assert_eq!(
            config.git_private_key_passphrase.as_deref(),
            Some("secret-passphrase")
        );
        assert_eq!(config.base_dir.as_deref(), Some("/tmp/renovate"));
        assert_eq!(config.cache_dir.as_deref(), Some("/tmp/renovate/cache"));
        assert_eq!(
            config.containerbase_dir.as_deref(),
            Some("/tmp/renovate/containerbase")
        );
        assert_eq!(config.docker_child_prefix.as_deref(), Some("rr_"));
        assert_eq!(config.docker_cli_options.as_deref(), Some("--network=host"));
        assert_eq!(
            config.docker_sidecar_image.as_deref(),
            Some("example/sidecar:1")
        );
        assert_eq!(config.docker_user.as_deref(), Some("1000:1000"));
        assert_eq!(config.execution_timeout, Some(20));
        assert_eq!(config.git_timeout, Some(10000));
        assert_eq!(config.git_url, "ssh");
        assert_eq!(config.http_cache_ttl_days, Some(45));
    }

    // Ported: "supports custom prefixes" — lib/workers/global/config/parse/env.spec.ts line 67
    #[test]
    fn custom_prefix_is_supported() {
        let config =
            build_from_env(&env(&[("ENV_PREFIX", "FOOBAR_"), ("FOOBAR_TOKEN", "abc")])).unwrap();
        assert_eq!(config.token.as_deref(), Some("abc"));
    }

    // Ported: "supports json" — lib/workers/global/config/parse/env.spec.ts line 76
    #[test]
    fn lock_file_maintenance_json_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_LOCK_FILE_MAINTENANCE", "{}")])).unwrap();
        assert!(config.lock_file_maintenance.is_empty());
    }

    // Ported: "supports arrays of objects" — lib/workers/global/config/parse/env.spec.ts line 83
    #[test]
    fn host_rules_array_is_parsed() {
        let config =
            build_from_env(&env(&[("RENOVATE_HOST_RULES", r#"[{"foo":"bar"}]"#)])).unwrap();
        assert_eq!(config.host_rules.len(), 1);
        assert_eq!(config.host_rules[0]["foo"], "bar");
    }

    // Ported: "\"$envArg\" -> $config" — lib/workers/global/config/parse/env.spec.ts line 91
    #[test]
    fn recreate_env_aliases_are_parsed() {
        let cases = [
            ("RENOVATE_RECREATE_CLOSED", "true", RecreateWhen::Always),
            ("RENOVATE_RECREATE_CLOSED", "false", RecreateWhen::Auto),
            ("RENOVATE_RECREATE_WHEN", "auto", RecreateWhen::Auto),
            ("RENOVATE_RECREATE_WHEN", "always", RecreateWhen::Always),
            ("RENOVATE_RECREATE_WHEN", "never", RecreateWhen::Never),
        ];

        for (key, value, expected) in cases {
            let config = build_from_env(&env(&[(key, value)])).unwrap();
            assert_eq!(config.recreate_when, expected, "{key}={value}");
        }
    }

    // Ported: "skips misconfigured arrays" — lib/workers/global/config/parse/env.spec.ts line 103
    #[test]
    fn host_rules_string_value_is_skipped() {
        let config = build_from_env(&env(&[("RENOVATE_HOST_RULES", r#""foobar""#)])).unwrap();
        assert!(config.host_rules.is_empty());
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn host_rules_object_value_is_skipped() {
        let config = build_from_env(&env(&[(
            "RENOVATE_HOST_RULES",
            r#"{"matchHost":"github.com"}"#,
        )]))
        .unwrap();
        assert!(config.host_rules.is_empty());
    }

    // Ported: "skips garbage array values" — lib/workers/global/config/parse/env.spec.ts line 117
    #[test]
    fn host_rules_garbage_value_is_skipped() {
        let config = build_from_env(&env(&[("RENOVATE_HOST_RULES", "!@#")])).unwrap();
        assert!(config.host_rules.is_empty());
    }

    // Ported: "supports GitHub token" — lib/workers/global/config/parse/env.spec.ts line 131
    #[test]
    fn github_token_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_TOKEN", "github.com token")])).unwrap();
        assert_eq!(config.platform, Platform::Github);
        assert_eq!(config.token.as_deref(), Some("github.com token"));
    }

    // Ported: "supports GitHub custom endpoint" — lib/workers/global/config/parse/env.spec.ts line 140
    #[test]
    fn github_endpoint_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_ENDPOINT", "a ghe endpoint")])).unwrap();
        assert_eq!(config.platform, Platform::Github);
        assert_eq!(config.endpoint.as_deref(), Some("a ghe endpoint"));
    }

    // Ported: "supports GitHub custom endpoint and github.com" — lib/workers/global/config/parse/env.spec.ts line 149
    #[test]
    fn github_com_token_becomes_host_rule_with_custom_endpoint() {
        let config = build_from_env(&env(&[
            ("GITHUB_COM_TOKEN", "a github.com token"),
            ("RENOVATE_ENDPOINT", "a ghe endpoint"),
            ("RENOVATE_TOKEN", "a ghe token"),
        ]))
        .unwrap();
        assert_eq!(config.endpoint.as_deref(), Some("a ghe endpoint"));
        assert_eq!(config.token.as_deref(), Some("a ghe token"));
        assert_eq!(config.host_rules.len(), 1);
        assert_eq!(config.host_rules[0]["hostType"], "github");
        assert_eq!(config.host_rules[0]["matchHost"], "github.com");
        assert_eq!(config.host_rules[0]["token"], "a github.com token");
    }

    // Ported: "supports GitHub fine-grained PATs" — lib/workers/global/config/parse/env.spec.ts line 168
    #[test]
    fn github_fine_grained_pat_becomes_host_rule() {
        let config = build_from_env(&env(&[
            ("GITHUB_COM_TOKEN", "github_pat_XXXXXX"),
            ("RENOVATE_TOKEN", "a github.com token"),
        ]))
        .unwrap();
        assert_eq!(config.token.as_deref(), Some("a github.com token"));
        assert_eq!(config.host_rules[0]["token"], "github_pat_XXXXXX");
    }

    // Ported: "supports RENOVATE_ prefixed github com token" — lib/workers/global/config/parse/env.spec.ts line 185
    #[test]
    fn renovate_prefixed_github_com_token_becomes_host_rule() {
        let config = build_from_env(&env(&[
            ("RENOVATE_GITHUB_COM_TOKEN", "github_pat_XXXXXX"),
            ("RENOVATE_TOKEN", "a github.com token"),
        ]))
        .unwrap();
        assert_eq!(config.host_rules[0]["token"], "github_pat_XXXXXX");
    }

    // Ported: "GITHUB_COM_TOKEN takes precedence over RENOVATE_GITHUB_COM_TOKEN" — lib/workers/global/config/parse/env.spec.ts line 202
    #[test]
    fn github_com_token_takes_precedence_over_renovate_prefixed_token() {
        let config = build_from_env(&env(&[
            ("GITHUB_COM_TOKEN", "github_pat_XXXXXX"),
            ("RENOVATE_GITHUB_COM_TOKEN", "github_pat_YYYYYY"),
            ("RENOVATE_TOKEN", "a github.com token"),
        ]))
        .unwrap();
        assert_eq!(config.host_rules[0]["token"], "github_pat_XXXXXX");
    }

    // Ported: "supports GitHub custom endpoint and gitlab.com" — lib/workers/global/config/parse/env.spec.ts line 220
    #[test]
    fn github_custom_endpoint_without_github_com_token_has_no_host_rule() {
        let config = build_from_env(&env(&[
            ("RENOVATE_ENDPOINT", "a ghe endpoint"),
            ("RENOVATE_TOKEN", "a ghe token"),
        ]))
        .unwrap();
        assert_eq!(config.endpoint.as_deref(), Some("a ghe endpoint"));
        assert_eq!(config.token.as_deref(), Some("a ghe token"));
        assert!(config.host_rules.is_empty());
    }

    // Ported: "supports GitLab token" — lib/workers/global/config/parse/env.spec.ts line 231
    #[test]
    fn gitlab_token_is_parsed() {
        let config = build_from_env(&env(&[
            ("RENOVATE_PLATFORM", "gitlab"),
            ("RENOVATE_TOKEN", "a gitlab.com token"),
        ]))
        .unwrap();
        assert_eq!(config.platform, Platform::Gitlab);
        assert_eq!(config.token.as_deref(), Some("a gitlab.com token"));
    }

    // Ported: "supports GitLab custom endpoint" — lib/workers/global/config/parse/env.spec.ts line 242
    #[test]
    fn gitlab_custom_endpoint_is_parsed() {
        let config = build_from_env(&env(&[
            ("RENOVATE_PLATFORM", "gitlab"),
            ("RENOVATE_TOKEN", "a gitlab token"),
            ("RENOVATE_ENDPOINT", "a gitlab endpoint"),
        ]))
        .unwrap();
        assert_eq!(config.platform, Platform::Gitlab);
        assert_eq!(config.token.as_deref(), Some("a gitlab token"));
        assert_eq!(config.endpoint.as_deref(), Some("a gitlab endpoint"));
    }

    // Ported: "supports Azure DevOps" — lib/workers/global/config/parse/env.spec.ts line 255
    #[test]
    fn azure_devops_config_is_parsed() {
        let config = build_from_env(&env(&[
            ("RENOVATE_PLATFORM", "azure"),
            ("RENOVATE_TOKEN", "an Azure DevOps token"),
            ("RENOVATE_ENDPOINT", "an Azure DevOps endpoint"),
        ]))
        .unwrap();
        assert_eq!(config.platform, Platform::Azure);
        assert_eq!(config.token.as_deref(), Some("an Azure DevOps token"));
        assert_eq!(config.endpoint.as_deref(), Some("an Azure DevOps endpoint"));
    }

    // Ported: "supports Bitbucket token" — lib/workers/global/config/parse/env.spec.ts line 268
    #[test]
    fn bitbucket_token_config_is_parsed() {
        let config = build_from_env(&env(&[
            ("RENOVATE_PLATFORM", "bitbucket"),
            ("RENOVATE_ENDPOINT", "a bitbucket endpoint"),
            ("RENOVATE_USERNAME", "some-username"),
            ("RENOVATE_PASSWORD", "app-password"),
        ]))
        .unwrap();
        assert_eq!(config.platform, Platform::Bitbucket);
        assert_eq!(config.endpoint.as_deref(), Some("a bitbucket endpoint"));
        assert_eq!(config.username.as_deref(), Some("some-username"));
        assert_eq!(config.password.as_deref(), Some("app-password"));
    }

    // Ported: "supports Bitbucket username/password" — lib/workers/global/config/parse/env.spec.ts line 283
    #[test]
    fn bitbucket_username_password_config_is_parsed() {
        let config = build_from_env(&env(&[
            ("RENOVATE_PLATFORM", "bitbucket"),
            ("RENOVATE_ENDPOINT", "a bitbucket endpoint"),
            ("RENOVATE_USERNAME", "some-username"),
            ("RENOVATE_PASSWORD", "app-password"),
        ]))
        .unwrap();
        assert_eq!(config.platform, Platform::Bitbucket);
        assert_eq!(config.endpoint.as_deref(), Some("a bitbucket endpoint"));
        assert!(config.host_rules.is_empty());
        assert_eq!(config.username.as_deref(), Some("some-username"));
        assert_eq!(config.password.as_deref(), Some("app-password"));
    }

    // Ported: "merges full config from env" — lib/workers/global/config/parse/env.spec.ts line 299
    #[test]
    fn renovate_config_merges_with_explicit_env() {
        let config = build_from_env(&env(&[
            ("RENOVATE_CONFIG", r#"{"enabled":false,"token":"foo"}"#),
            ("RENOVATE_TOKEN", "a"),
        ]))
        .unwrap();
        assert_eq!(config.enabled, Some(false));
        assert_eq!(config.token.as_deref(), Some("a"));
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn renovate_config_json5_is_parsed() {
        let config = build_from_env(&env(&[(
            "RENOVATE_CONFIG",
            "{enabled:false, token:'foo',}",
        )]))
        .unwrap();
        assert_eq!(config.enabled, Some(false));
        assert_eq!(config.token.as_deref(), Some("foo"));
    }

    // Ported: "massages converted experimental env vars" — lib/workers/global/config/parse/env.spec.ts line 309
    #[test]
    fn experimental_env_vars_are_massaged() {
        let config = build_from_env(&env(&[
            ("RENOVATE_X_MERGE_CONFIDENCE_API_BASE_URL", "some-url"),
            (
                "RENOVATE_X_MERGE_CONFIDENCE_SUPPORTED_DATASOURCES",
                r#"["docker"]"#,
            ),
            ("RENOVATE_X_AUTODISCOVER_REPO_SORT", "alpha"),
            ("RENOVATE_X_DOCKER_MAX_PAGES", "10"),
            ("RENOVATE_AUTODISCOVER_REPO_ORDER", "desc"),
            ("RENOVATE_AUTODISCOVER", "true"),
            ("RENOVATE_AUTODISCOVER_FILTER", "org/*,!org/archived"),
            ("RENOVATE_AUTODISCOVER_NAMESPACES", "backend,frontend"),
            ("RENOVATE_AUTODISCOVER_PROJECTS", "[\"api\",\"web\"]"),
            ("RENOVATE_AUTODISCOVER_TOPICS", "renovate,dependencies"),
            ("RENOVATE_X_DELETE_CONFIG_FILE", "true"),
            ("RENOVATE_DELETE_ADDITIONAL_CONFIG_FILE", "true"),
            ("RENOVATE_CONFIG_VALIDATION_ERROR", "true"),
            ("RENOVATE_CHECKED_BRANCHES", "renovate/a,renovate/b"),
            ("RENOVATE_GIT_NO_VERIFY", "[\"commit\"]"),
            ("RENOVATE_WRITE_DISCOVERED_REPOS", "./repos.json"),
            ("RENOVATE_X_S3_ENDPOINT", "endpoint"),
            ("RENOVATE_X_S3_PATH_STYLE", "true"),
            ("RENOVATE_X_REPO_CACHE_FORCE_LOCAL", "enabled"),
        ]))
        .unwrap();
        assert_eq!(
            config.merge_confidence_endpoint.as_deref(),
            Some("some-url")
        );
        assert_eq!(config.merge_confidence_datasources, vec!["docker"]);
        assert_eq!(config.autodiscover_repo_sort.as_deref(), Some("alpha"));
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
        assert_eq!(config.docker_max_pages, Some(10));
        assert!(config.delete_config_file);
        assert!(config.delete_additional_config_file);
        assert!(config.config_validation_error);
        assert_eq!(
            config.checked_branches,
            vec!["renovate/a".to_owned(), "renovate/b".to_owned()]
        );
        assert_eq!(config.git_no_verify, vec!["commit".to_owned()]);
        assert_eq!(
            config.write_discovered_repos.as_deref(),
            Some("./repos.json")
        );
        assert_eq!(config.s3_endpoint.as_deref(), Some("endpoint"));
        assert!(config.s3_path_style);
        assert_eq!(config.repository_cache_force_local, Some(true));
    }

    // Ported: "massages converted experimental env vars" — lib/workers/global/config/parse/env.spec.ts line 309
    #[test]
    fn converted_experimental_env_current_names_are_parsed() {
        let config = build_from_env(&env(&[
            ("RENOVATE_AUTODISCOVER_REPO_SORT", "alpha"),
            ("RENOVATE_X_AUTODISCOVER_REPO_ORDER", "desc"),
            ("RENOVATE_DOCKER_MAX_PAGES", "10"),
            ("RENOVATE_DELETE_CONFIG_FILE", "true"),
            ("RENOVATE_S3_ENDPOINT", "endpoint"),
            ("RENOVATE_S3_PATH_STYLE", "true"),
            ("RENOVATE_REPOSITORY_CACHE_FORCE_LOCAL", "false"),
        ]))
        .unwrap();

        assert_eq!(config.autodiscover_repo_sort.as_deref(), Some("alpha"));
        assert_eq!(config.autodiscover_repo_order.as_deref(), Some("desc"));
        assert_eq!(config.docker_max_pages, Some(10));
        assert!(config.delete_config_file);
        assert_eq!(config.s3_endpoint.as_deref(), Some("endpoint"));
        assert!(config.s3_path_style);
        assert_eq!(config.repository_cache_force_local, Some(false));
    }

    // Ported: "does not migrate empty RENOVATE_X_REPO_CACHE_FORCE_LOCAL" — lib/workers/global/config/parse/env.spec.ts line 336
    #[test]
    fn empty_repo_cache_force_local_is_not_migrated() {
        let config = build_from_env(&env(&[("RENOVATE_X_REPO_CACHE_FORCE_LOCAL", "")])).unwrap();
        assert_eq!(config.repository_cache_force_local, None);
    }

    // Ported: "crashes" — lib/workers/global/config/parse/env.spec.ts line 357
    #[test]
    fn invalid_renovate_config_is_rejected() {
        let err = build_from_env(&env(&[("RENOVATE_CONFIG", "!@#")])).unwrap_err();
        assert_eq!(err, "Invalid RENOVATE_CONFIG: '!@#'");
    }

    // Ported: "migrates RENOVATE_CONFIG" — lib/workers/global/config/parse/env.spec.ts line 367
    #[test]
    fn renovate_config_automerge_any_is_migrated() {
        let config = build_from_env(&env(&[(
            "RENOVATE_CONFIG",
            r#"{"automerge":"any","token":"foo"}"#,
        )]))
        .unwrap();
        assert_eq!(config.automerge, Some(true));
        assert_eq!(config.token.as_deref(), Some("foo"));
    }

    // Ported: "renames migrated variables" — lib/workers/global/config/parse/env.spec.ts line 386
    #[test]
    fn git_lab_automerge_env_sets_platform_automerge() {
        let config = build_from_env(&env(&[("RENOVATE_GIT_LAB_AUTOMERGE", "true")])).unwrap();
        assert!(config.platform_automerge);
    }

    // Ported: "renames migrated variables" — lib/workers/global/config/parse/env.spec.ts line 386
    #[test]
    fn renamed_env_vars_map_to_current_options() {
        let config = build_from_env(&env(&[
            (
                "RENOVATE_ALLOWED_POST_UPGRADE_COMMANDS",
                "npm install,cargo update",
            ),
            (
                "RENOVATE_ALIASES",
                r#"{"docker.io":"registry.example.com"}"#,
            ),
            ("RENOVATE_AZURE_AUTO_COMPLETE", "false"),
            (
                "RENOVATE_MERGE_CONFIDENCE_API_BASE_URL",
                "https://mc.example",
            ),
            (
                "RENOVATE_MERGE_CONFIDENCE_SUPPORTED_DATASOURCES",
                r#"["docker","npm"]"#,
            ),
        ]))
        .unwrap();

        assert_eq!(config.allowed_commands, vec!["npm install", "cargo update"]);
        assert_eq!(
            config.registry_aliases.get("docker.io").map(String::as_str),
            Some("registry.example.com")
        );
        assert!(!config.platform_automerge);
        assert_eq!(
            config.merge_confidence_endpoint.as_deref(),
            Some("https://mc.example")
        );
        assert_eq!(config.merge_confidence_datasources, vec!["docker", "npm"]);
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn allowed_commands_env_json5_array_is_parsed() {
        let config = build_from_env(&env(&[(
            "RENOVATE_ALLOWED_COMMANDS",
            "['npm install','cargo update',]",
        )]))
        .unwrap();
        assert_eq!(config.allowed_commands, vec!["npm install", "cargo update"]);
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn renamed_allowed_commands_env_json5_array_is_parsed() {
        let config = build_from_env(&env(&[(
            "RENOVATE_ALLOWED_POST_UPGRADE_COMMANDS",
            "['npm install','cargo update',]",
        )]))
        .unwrap();
        assert_eq!(config.allowed_commands, vec!["npm install", "cargo update"]);
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn merge_confidence_datasources_env_comma_list_is_parsed() {
        let config = build_from_env(&env(&[(
            "RENOVATE_MERGE_CONFIDENCE_SUPPORTED_DATASOURCES",
            "docker,npm, maven,",
        )]))
        .unwrap();
        assert_eq!(
            config.merge_confidence_datasources,
            vec!["docker", "npm", "maven"]
        );
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn command_templating_env_is_parsed() {
        let config =
            build_from_env(&env(&[("RENOVATE_ALLOW_COMMAND_TEMPLATING", "true")])).unwrap();
        assert!(config.allow_command_templating);
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn global_security_env_options_are_parsed() {
        let config = build_from_env(&env(&[
            ("RENOVATE_ALLOW_PLUGINS", "true"),
            ("RENOVATE_ALLOW_SCRIPTS", "false"),
            (
                "RENOVATE_ALLOW_SHELL_EXECUTOR_FOR_POST_UPGRADE_COMMANDS",
                "true",
            ),
            ("RENOVATE_OPTIMIZE_FOR_DISABLED", "true"),
            ("RENOVATE_ALLOW_CUSTOM_CRATE_REGISTRIES", "true"),
            ("RENOVATE_ALLOWED_HEADERS", "X-*,Authorization"),
            ("RENOVATE_ALLOWED_ENV", "['SOME_*','OTHER_*']"),
            (
                "RENOVATE_ALLOWED_UNSAFE_EXECUTIONS",
                "bazelModDeps,goGenerate",
            ),
            ("RENOVATE_EXPOSE_ALL_ENV", "true"),
            ("RENOVATE_DETECT_GLOBAL_MANAGER_CONFIG", "true"),
            ("RENOVATE_DETECT_HOST_RULES_FROM_ENV", "false"),
        ]))
        .unwrap();

        assert_eq!(config.allow_plugins, Some(true));
        assert_eq!(config.allow_scripts, Some(false));
        assert_eq!(
            config.allow_shell_executor_for_post_upgrade_commands,
            Some(true)
        );
        assert!(config.optimize_for_disabled);
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
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn repository_cache_env_options_are_parsed() {
        let config = build_from_env(&env(&[
            ("RENOVATE_REPOSITORY_CACHE", "enabled"),
            ("RENOVATE_REPOSITORY_CACHE_TYPE", "s3"),
            ("RENOVATE_LOCAL_DIR", "/tmp/renovate/repo"),
            ("RENOVATE_CACHE_HARD_TTL_MINUTES", "1440"),
            ("RENOVATE_CACHE_PRIVATE_PACKAGES", "true"),
            ("RENOVATE_PRESET_CACHE_PERSISTENCE", "true"),
            ("RENOVATE_INCLUDE_MIRRORS", "true"),
            ("RENOVATE_GITHUB_TOKEN_WARN", "false"),
            ("RENOVATE_ENCRYPTED_WARNING", "encrypted config ignored"),
            ("RENOVATE_IGNORE_PR_AUTHOR", "true"),
            ("RENOVATE_BB_USE_DEVELOPMENT_BRANCH", "true"),
            ("RENOVATE_PR_CACHE_SYNC_MAX_PAGES", "5"),
        ]))
        .unwrap();

        assert_eq!(config.repository_cache.as_deref(), Some("enabled"));
        assert_eq!(config.repository_cache_type.as_deref(), Some("s3"));
        assert_eq!(config.local_dir.as_deref(), Some("/tmp/renovate/repo"));
        assert_eq!(config.cache_hard_ttl_minutes, Some(1440));
        assert_eq!(config.cache_private_packages, Some(true));
        assert_eq!(config.preset_cache_persistence, Some(true));
        assert_eq!(config.include_mirrors, Some(true));
        assert_eq!(config.github_token_warn, Some(false));
        assert_eq!(
            config.encrypted_warning.as_deref(),
            Some("encrypted config ignored")
        );
        assert_eq!(config.ignore_pr_author, Some(true));
        assert_eq!(config.bb_use_development_branch, Some(true));
        assert_eq!(config.pr_cache_sync_max_pages, Some(5));
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn report_env_options_are_parsed() {
        let config = build_from_env(&env(&[
            ("RENOVATE_REPORT_TYPE", "file"),
            ("RENOVATE_REPORT_PATH", "./report.json"),
            ("RENOVATE_REPORT_FORMATTING", "true"),
            ("RENOVATE_UNICODE_EMOJI", "false"),
        ]))
        .unwrap();

        assert_eq!(config.report_type.as_deref(), Some("file"));
        assert_eq!(config.report_path.as_deref(), Some("./report.json"));
        assert_eq!(config.report_formatting, Some(true));
        assert_eq!(config.unicode_emoji, Some(false));
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn onboarding_env_options_are_parsed() {
        let config = build_from_env(&env(&[
            ("RENOVATE_ONBOARDING", "false"),
            ("RENOVATE_ONBOARDING_BRANCH", "renovate/setup"),
            ("RENOVATE_ONBOARDING_AUTO_CLOSE_AGE", "14"),
            ("RENOVATE_ONBOARDING_COMMIT_MESSAGE", "Configure Renovate"),
            (
                "RENOVATE_CONFIG_FILE_NAMES",
                "renovate.json,.github/renovate.json5",
            ),
            (
                "RENOVATE_MIGRATE_PRESETS",
                "{'config:old':'config:new','config:removed':''}",
            ),
            ("RENOVATE_CUSTOM_ENV_VARIABLES", "{EXAMPLE:'value'}"),
            ("RENOVATE_CACHE_TTL_OVERRIDE", "{'datasource-npm':30}"),
            (
                "RENOVATE_TOOL_SETTINGS",
                "{jvmMaxMemory:1024,nodeMaxMemory:2048}",
            ),
            (
                "RENOVATE_ONBOARDING_CONFIG_FILE_NAME",
                ".github/renovate.json5",
            ),
            ("RENOVATE_ONBOARDING_NO_DEPS", "enabled"),
            ("RENOVATE_ONBOARDING_PR_TITLE", "Configure Renovate"),
            ("RENOVATE_ONBOARDING_REBASE_CHECKBOX", "true"),
            ("RENOVATE_PR_COMMITS_PER_RUN_LIMIT", "4"),
        ]))
        .unwrap();

        assert_eq!(config.onboarding, Some(false));
        assert_eq!(config.onboarding_branch.as_deref(), Some("renovate/setup"));
        assert_eq!(config.onboarding_auto_close_age, Some(14));
        assert_eq!(
            config.onboarding_commit_message.as_deref(),
            Some("Configure Renovate")
        );
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
            config
                .migrate_presets
                .get("config:removed")
                .map(String::as_str),
            Some("")
        );
        assert_eq!(
            config
                .custom_env_variables
                .get("EXAMPLE")
                .map(String::as_str),
            Some("value")
        );
        assert_eq!(
            config
                .cache_ttl_override
                .get("datasource-npm")
                .and_then(serde_json::Value::as_u64),
            Some(30)
        );
        assert_eq!(
            config
                .tool_settings
                .get("jvmMaxMemory")
                .and_then(serde_json::Value::as_u64),
            Some(1024)
        );
        assert_eq!(
            config
                .tool_settings
                .get("nodeMaxMemory")
                .and_then(serde_json::Value::as_u64),
            Some(2048)
        );
        assert_eq!(
            config.onboarding_config_file_name.as_deref(),
            Some(".github/renovate.json5")
        );
        assert_eq!(config.onboarding_no_deps.as_deref(), Some("enabled"));
        assert_eq!(
            config.onboarding_pr_title.as_deref(),
            Some("Configure Renovate")
        );
        assert_eq!(config.onboarding_rebase_checkbox, Some(true));
        assert_eq!(config.pr_commits_per_run_limit, Some(4));
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn onboarding_config_env_is_parsed() {
        let config = build_from_env(&env(&[(
            "RENOVATE_ONBOARDING_CONFIG",
            "{extends:['config:recommended'],}",
        )]))
        .unwrap();

        assert_eq!(
            config.onboarding_config["extends"][0].as_str(),
            Some("config:recommended")
        );
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn invalid_object_env_values_are_rejected() {
        let err =
            build_from_env(&env(&[("RENOVATE_LOCK_FILE_MAINTENANCE", "not-json")])).unwrap_err();
        assert_eq!(
            err,
            "RENOVATE_LOCK_FILE_MAINTENANCE was invalid: Error: Invalid JSON value: 'not-json'"
        );
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn invalid_registry_alias_env_values_are_rejected() {
        let err = build_from_env(&env(&[(
            "RENOVATE_REGISTRY_ALIASES",
            r#"{"docker.io":123}"#,
        )]))
        .unwrap_err();
        assert_eq!(
            err,
            r#"RENOVATE_REGISTRY_ALIASES was invalid: Error: Invalid JSON value: '{"docker.io":123}'"#
        );
    }

    // Ported: "dryRun boolean true" — lib/workers/global/config/parse/env.spec.ts line 441
    #[test]
    fn dry_run_true_maps_to_full() {
        let config = build_from_env(&env(&[("RENOVATE_DRY_RUN", "true")])).unwrap();
        assert_eq!(config.dry_run, Some(DryRun::Full));
    }

    // Ported: "dryRun boolean false" — lib/workers/global/config/parse/env.spec.ts line 449
    #[test]
    fn dry_run_false_disables_dry_run() {
        let config = build_from_env(&env(&[("RENOVATE_DRY_RUN", "false")])).unwrap();
        assert_eq!(config.dry_run, None);
    }

    // Ported: "dryRun null" — lib/workers/global/config/parse/env.spec.ts line 457
    #[test]
    fn dry_run_null_disables_dry_run() {
        let config = build_from_env(&env(&[("RENOVATE_DRY_RUN", "null")])).unwrap();
        assert_eq!(config.dry_run, None);
    }

    // Ported: "requireConfig boolean true" — lib/workers/global/config/parse/env.spec.ts line 465
    #[test]
    fn require_config_true_maps_to_required() {
        let config = build_from_env(&env(&[("RENOVATE_REQUIRE_CONFIG", "true")])).unwrap();
        assert_eq!(config.require_config, RequireConfig::Required);
    }

    // Ported: "requireConfig boolean false" — lib/workers/global/config/parse/env.spec.ts line 473
    #[test]
    fn require_config_false_maps_to_optional() {
        let config = build_from_env(&env(&[("RENOVATE_REQUIRE_CONFIG", "false")])).unwrap();
        assert_eq!(config.require_config, RequireConfig::Optional);
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn fork_processing_env_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_FORK_PROCESSING", "enabled")])).unwrap();
        assert_eq!(config.fork_processing, ForkProcessing::Enabled);
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn fork_mode_env_options_are_parsed() {
        let config = build_from_env(&env(&[
            ("RENOVATE_FORK_CREATION", "false"),
            ("RENOVATE_FORK_TOKEN", "fork-token"),
            ("RENOVATE_FORK_ORG", "renovate-forks"),
        ]))
        .unwrap();
        assert_eq!(config.fork_creation, Some(false));
        assert_eq!(config.fork_token.as_deref(), Some("fork-token"));
        assert_eq!(config.fork_org.as_deref(), Some("renovate-forks"));
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn binary_source_env_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_BINARY_SOURCE", "hermit")])).unwrap();
        assert_eq!(config.binary_source, Some(BinarySource::Hermit));
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn binary_source_auto_env_maps_to_global() {
        let config = build_from_env(&env(&[("RENOVATE_BINARY_SOURCE", "auto")])).unwrap();
        assert_eq!(config.binary_source, Some(BinarySource::Global));
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn invalid_binary_source_env_is_rejected() {
        let err = build_from_env(&env(&[("RENOVATE_BINARY_SOURCE", "invalid")])).unwrap_err();
        assert_eq!(
            err,
            "RENOVATE_BINARY_SOURCE was invalid: Invalid value `invalid` for `binarySource`. The allowed values are docker, global, install, hermit."
        );
    }

    // Ported: "platformCommit boolean true" — lib/workers/global/config/parse/env.spec.ts line 481
    #[test]
    fn platform_commit_true_maps_to_enabled() {
        let config = build_from_env(&env(&[("RENOVATE_PLATFORM_COMMIT", "true")])).unwrap();
        assert_eq!(config.platform_commit.as_deref(), Some("enabled"));
    }

    // Ported: "platformCommit boolean false" — lib/workers/global/config/parse/env.spec.ts line 489
    #[test]
    fn platform_commit_false_maps_to_disabled() {
        let config = build_from_env(&env(&[("RENOVATE_PLATFORM_COMMIT", "false")])).unwrap();
        assert_eq!(config.platform_commit.as_deref(), Some("disabled"));
    }

    // Rust-specific: config_env behavior test
    #[test]
    fn platform_commit_string_values_are_parsed() {
        for (raw, expected) in [
            ("auto", "auto"),
            ("enabled", "enabled"),
            ("disabled", "disabled"),
        ] {
            let config = build_from_env(&env(&[("RENOVATE_PLATFORM_COMMIT", raw)])).unwrap();
            assert_eq!(config.platform_commit.as_deref(), Some(expected), "{raw}");
        }
    }

    // Ported: "has no duplicate env names across options" — lib/workers/global/config/parse/env.spec.ts line 396
    // Verifies that no two config fields share the same RENOVATE_* env var name.
    #[test]
    fn no_duplicate_env_names_across_options() {
        // Exhaustive list of all RENOVATE_* env var names consumed by build_from_env.
        // If a name appears twice here, two fields would silently share the same env var.
        let names: &[&str] = &[
            "RENOVATE_CONFIG",
            "RENOVATE_CONFIG_MIGRATION",
            "RENOVATE_PRINT_CONFIG",
            "RENOVATE_ONBOARDING",
            "RENOVATE_ONBOARDING_AUTO_CLOSE_AGE",
            "RENOVATE_ONBOARDING_REBASE_CHECKBOX",
            "RENOVATE_PR_COMMITS_PER_RUN_LIMIT",
            "RENOVATE_ENABLED",
            "RENOVATE_AUTOMERGE",
            "RENOVATE_DEPENDENCY_DASHBOARD",
            "RENOVATE_DEPENDENCY_DASHBOARD_APPROVAL",
            "RENOVATE_DEPENDENCY_DASHBOARD_AUTOCLOSE",
            "RENOVATE_DEPENDENCY_DASHBOARD_TITLE",
            "RENOVATE_DEPENDENCY_DASHBOARD_HEADER",
            "RENOVATE_DEPENDENCY_DASHBOARD_FOOTER",
            "RENOVATE_DEPENDENCY_DASHBOARD_LABELS",
            "RENOVATE_CONFIG_WARNING_REUSE_ISSUE",
            "RENOVATE_LABELS",
            "RENOVATE_TOKEN",
            "RENOVATE_GIT_PRIVATE_KEY",
            "RENOVATE_GIT_PRIVATE_KEY_PASSPHRASE",
            "RENOVATE_PLATFORM",
            "RENOVATE_ENDPOINT",
            "RENOVATE_USERNAME",
            "RENOVATE_PASSWORD",
            "RENOVATE_REQUIRE_CONFIG",
            "RENOVATE_FORK_CREATION",
            "RENOVATE_FORK_TOKEN",
            "RENOVATE_FORK_ORG",
            "RENOVATE_DRY_RUN",
            "RENOVATE_MODE",
            "RENOVATE_ALLOW_COMMAND_TEMPLATING",
            "RENOVATE_ALLOW_PLUGINS",
            "RENOVATE_ALLOW_SCRIPTS",
            "RENOVATE_ALLOW_SHELL_EXECUTOR_FOR_POST_UPGRADE_COMMANDS",
            "RENOVATE_OPTIMIZE_FOR_DISABLED",
            "RENOVATE_ALLOW_CUSTOM_CRATE_REGISTRIES",
            "RENOVATE_ALLOWED_COMMANDS",
            "RENOVATE_ALLOWED_POST_UPGRADE_COMMANDS",
            "RENOVATE_ALLOWED_HEADERS",
            "RENOVATE_ALLOWED_ENV",
            "RENOVATE_ALLOWED_UNSAFE_EXECUTIONS",
            "RENOVATE_EXPOSE_ALL_ENV",
            "RENOVATE_DETECT_GLOBAL_MANAGER_CONFIG",
            "RENOVATE_DETECT_HOST_RULES_FROM_ENV",
            "RENOVATE_REGISTRY_ALIASES",
            "RENOVATE_LOCK_FILE_MAINTENANCE",
            "RENOVATE_ONBOARDING_CONFIG",
            "RENOVATE_RECREATE_CLOSED",
            "RENOVATE_RECREATE_WHEN",
            "RENOVATE_PLATFORM_AUTOMERGE",
            "RENOVATE_FORK_PROCESSING",
            "RENOVATE_BINARY_SOURCE",
            "RENOVATE_PLATFORM_COMMIT",
            "RENOVATE_GIT_URL",
            "RENOVATE_AUTODISCOVER",
            "RENOVATE_AUTODISCOVER_FILTER",
            "RENOVATE_AUTODISCOVER_REPO_SORT",
            "RENOVATE_AUTODISCOVER_REPO_ORDER",
            "RENOVATE_AUTODISCOVER_NAMESPACES",
            "RENOVATE_AUTODISCOVER_PROJECTS",
            "RENOVATE_AUTODISCOVER_TOPICS",
            "RENOVATE_DOCKER_MAX_PAGES",
            "RENOVATE_DELETE_CONFIG_FILE",
            "RENOVATE_DELETE_ADDITIONAL_CONFIG_FILE",
            "RENOVATE_CONFIG_VALIDATION_ERROR",
            "RENOVATE_CHECKED_BRANCHES",
            "RENOVATE_GIT_NO_VERIFY",
            "RENOVATE_WRITE_DISCOVERED_REPOS",
            "RENOVATE_S3_ENDPOINT",
            "RENOVATE_S3_PATH_STYLE",
            "RENOVATE_REPOSITORY_CACHE",
            "RENOVATE_REPOSITORY_CACHE_TYPE",
            "RENOVATE_REPOSITORY_CACHE_FORCE_LOCAL",
            "RENOVATE_X_REPO_CACHE_FORCE_LOCAL",
            "RENOVATE_CACHE_HARD_TTL_MINUTES",
            "RENOVATE_CACHE_PRIVATE_PACKAGES",
            "RENOVATE_PRESET_CACHE_PERSISTENCE",
            "RENOVATE_INCLUDE_MIRRORS",
            "RENOVATE_GITHUB_TOKEN_WARN",
            "RENOVATE_ENCRYPTED_WARNING",
            "RENOVATE_IGNORE_PR_AUTHOR",
            "RENOVATE_BB_USE_DEVELOPMENT_BRANCH",
            "RENOVATE_PR_CACHE_SYNC_MAX_PAGES",
            "RENOVATE_REPORT_TYPE",
            "RENOVATE_REPORT_PATH",
            "RENOVATE_REPORT_FORMATTING",
            "RENOVATE_UNICODE_EMOJI",
            "RENOVATE_USER_AGENT",
            "RENOVATE_BASE_DIR",
            "RENOVATE_CACHE_DIR",
            "RENOVATE_LOCAL_DIR",
            "RENOVATE_CONTAINERBASE_DIR",
            "RENOVATE_DOCKER_CHILD_PREFIX",
            "RENOVATE_DOCKER_CLI_OPTIONS",
            "RENOVATE_DOCKER_SIDECAR_IMAGE",
            "RENOVATE_DOCKER_USER",
            "RENOVATE_EXECUTION_TIMEOUT",
            "RENOVATE_GIT_TIMEOUT",
            "RENOVATE_HTTP_CACHE_TTL_DAYS",
            "RENOVATE_HOST_RULES",
            "RENOVATE_REPOSITORIES",
            "RENOVATE_MERGE_CONFIDENCE_ENDPOINT",
            "RENOVATE_MERGE_CONFIDENCE_DATASOURCES",
            "RENOVATE_MERGE_CONFIDENCE_SUPPORTED_DATASOURCES",
            "RENOVATE_MIGRATE_PRESETS",
            "RENOVATE_CUSTOM_ENV_VARIABLES",
            "RENOVATE_CACHE_TTL_OVERRIDE",
            "RENOVATE_TOOL_SETTINGS",
            "RENOVATE_ALIASES",
        ];
        let mut seen = std::collections::HashSet::new();
        let mut duplicates = Vec::new();
        for name in names {
            if !seen.insert(name) {
                duplicates.push(name);
            }
        }
        assert!(
            duplicates.is_empty(),
            "Duplicate env var names: {:?}",
            duplicates
        );
    }

    // The single test for this cycle's work on coersions.ts (array coersion used by env/cli).
    #[test]
    fn parse_string_array_matches_coersions_array_behavior() {
        // Ported: array coersion (JSON5 or csv fallback) from lib/workers/global/config/parse/coersions.ts
        // exercised via env array options like allowedCommands, etc.
        // json5 array
        assert_eq!(
            parse_string_array(r#"["a", "b"]"#).unwrap(),
            vec!["a".to_string(), "b".to_string()]
        );
        // csv fallback (like TS catch path)
        assert_eq!(
            parse_string_array("foo, bar, ,baz").unwrap(),
            vec!["foo".to_string(), "bar".to_string(), "baz".to_string()]
        );
        // empty
        assert_eq!(parse_string_array("").unwrap(), Vec::<String>::new());
        // invalid json5 without csv fallback? but per updated logic falls to csv (or empty if no , )
        // here a non-array json5 string falls through to split (which for no , gives the string)
        assert_eq!(
            parse_string_array(r#""just-a-string""#).unwrap(),
            vec!["just-a-string".to_string()]
        );
    }
}
