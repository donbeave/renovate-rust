//! Renovate-compatible global config parsing from environment variables.
//!
//! Renovate reference: `lib/workers/global/config/parse/env.ts` `getConfig`.

use std::collections::BTreeMap;

use renovate_core::config::{
    BinarySource, DryRun, ForkProcessing, GlobalConfig, Platform, RecreateWhen, RequireConfig,
};
use serde_json::{json, Value};

use crate::config_builder::{parse_json_array, parse_json_object};
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
    if let Some(value) = env_value(env, prefix, "ENABLED") {
        config.enabled = Some(parse_bool("RENOVATE_ENABLED", value)?);
    }
    if let Some(value) = env_value(env, prefix, "AUTOMERGE") {
        config.automerge = Some(parse_bool("RENOVATE_AUTOMERGE", value)?);
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
    if let Some(value) = env_value(env, prefix, "USER_AGENT") {
        config.user_agent = Some(value.to_owned());
    }
    if let Some(value) = env_value(env, prefix, "PLATFORM") {
        config.platform = parse_platform(value)?;
    }
    if let Some(value) = env_value(env, prefix, "DRY_RUN") {
        config.dry_run = parse_dry_run(value)?;
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
        config.allow_command_templating =
            parse_bool("RENOVATE_ALLOW_COMMAND_TEMPLATING", value)?;
    }
    if let Some(value) = env_value(env, prefix, "ALLOWED_HEADERS") {
        config.allowed_headers = Some(parse_string_list(value));
    }
    if let Some(value) = env_value(env, prefix, "ALLOWED_ENV") {
        config.allowed_env = Some(parse_string_list(value));
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
    .or_else(|| {
        env_value(env, prefix, "X_MERGE_CONFIDENCE_SUPPORTED_DATASOURCES")
    }) {
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
    if let Some(value) =
        env_converted_experimental_value(env, prefix, "DOCKER_MAX_PAGES", "X_DOCKER_MAX_PAGES")
        && let Ok(pages) = value.parse()
    {
        config.docker_max_pages = Some(pages);
    }
    if let Some(value) = env_converted_experimental_value(
        env,
        prefix,
        "DELETE_CONFIG_FILE",
        "X_DELETE_CONFIG_FILE",
    ) {
        config.delete_config_file = parse_bool("RENOVATE_DELETE_CONFIG_FILE", value)?;
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
        config.execution_timeout =
            Some(parse_u32("RENOVATE_EXECUTION_TIMEOUT", value)?);
    }
    if let Some(value) = env_value(env, prefix, "GIT_TIMEOUT") {
        config.git_timeout = Some(parse_u32("RENOVATE_GIT_TIMEOUT", value)?);
    }
    if let Some(value) = env_value(env, prefix, "HTTP_CACHE_TTL_DAYS") {
        config.http_cache_ttl_days =
            Some(parse_u32("RENOVATE_HTTP_CACHE_TTL_DAYS", value)?);
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

fn parse_u32(env_name: &str, value: &str) -> Result<u32, String> {
    value
        .parse::<u32>()
        .map_err(|_| format!("{env_name} was invalid: {value}"))
}

fn parse_string_array(raw: &str) -> Result<Vec<String>, String> {
    match json5::from_str(raw) {
        Ok(serde_json::Value::Array(values)) => Ok(values
            .into_iter()
            .filter_map(|value| value.as_str().map(str::to_owned))
            .collect()),
        _ => Err(format!("Invalid JSON value: '{raw}'")),
    }
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

    // Ported: "returns empty env" — workers/global/config/parse/env.spec.ts line 11
    #[test]
    fn empty_env_returns_default_config() {
        let config = build_from_env(&env(&[])).unwrap();
        assert!(config.host_rules.is_empty());
        assert_eq!(config, renovate_core::config::GlobalConfig::default());
    }

    // Ported: "supports boolean true" — workers/global/config/parse/env.spec.ts line 15
    #[test]
    fn config_migration_true_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_CONFIG_MIGRATION", "true")])).unwrap();
        assert!(config.config_migration);
    }

    // Ported: "supports boolean false" — workers/global/config/parse/env.spec.ts line 20
    #[test]
    fn config_migration_false_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_CONFIG_MIGRATION", "false")])).unwrap();
        assert!(!config.config_migration);
    }

    #[test]
    fn enabled_env_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_ENABLED", "false")])).unwrap();
        assert_eq!(config.enabled, Some(false));
    }

    #[test]
    fn automerge_env_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_AUTOMERGE", "true")])).unwrap();
        assert_eq!(config.automerge, Some(true));
    }

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

    // Ported: "throws exception for invalid boolean value" — workers/global/config/parse/env.spec.ts line 27
    #[test]
    fn config_migration_invalid_boolean_is_rejected() {
        let err = build_from_env(&env(&[("RENOVATE_CONFIG_MIGRATION", "badvalue")])).unwrap_err();
        assert_eq!(
            err,
            "RENOVATE_CONFIG_MIGRATION was invalid: Error: Invalid boolean value: expected 'true' or 'false', but got 'badvalue'"
        );
    }

    // Ported: "supports list single" — workers/global/config/parse/env.spec.ts line 40
    #[test]
    fn labels_single_value_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_LABELS", "a")])).unwrap();
        assert_eq!(config.labels, vec!["a"]);
    }

    // Ported: "supports list multiple" — workers/global/config/parse/env.spec.ts line 45
    #[test]
    fn labels_multiple_values_are_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_LABELS", "a,b,c")])).unwrap();
        assert_eq!(config.labels, vec!["a", "b", "c"]);
    }

    // Ported: "supports list multiple without blank items" — workers/global/config/parse/env.spec.ts line 50
    #[test]
    fn labels_ignore_blank_items() {
        let config = build_from_env(&env(&[("RENOVATE_LABELS", "a, b, c,")])).unwrap();
        assert_eq!(config.labels, vec!["a", "b", "c"]);
    }

    #[test]
    fn repositories_env_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_REPOSITORIES", "foo, bar,")])).unwrap();
        assert_eq!(config.repositories, vec!["foo", "bar"]);
    }

    #[test]
    fn repositories_env_json_array_is_parsed() {
        let config =
            build_from_env(&env(&[("RENOVATE_REPOSITORIES", r#"["foo","bar"]"#)])).unwrap();
        assert_eq!(config.repositories, vec!["foo", "bar"]);
    }

    // Ported: "supports string" — workers/global/config/parse/env.spec.ts line 55
    #[test]
    fn token_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_TOKEN", "a")])).unwrap();
        assert_eq!(config.token.as_deref(), Some("a"));
    }

    // Ported: "coerces string newlines" — workers/global/config/parse/env.spec.ts line 60
    #[test]
    fn string_newlines_are_coerced() {
        let config = build_from_env(&env(&[("RENOVATE_GIT_PRIVATE_KEY", r"abc\ndef")])).unwrap();
        assert_eq!(config.git_private_key.as_deref(), Some("abc\ndef"));
    }

    #[test]
    fn runtime_global_env_options_are_parsed() {
        let config = build_from_env(&env(&[
            ("RENOVATE_USER_AGENT", "renovate-rust-test"),
            ("RENOVATE_BASE_DIR", "/tmp/renovate"),
            ("RENOVATE_CACHE_DIR", "/tmp/renovate/cache"),
            ("RENOVATE_CONTAINERBASE_DIR", "/tmp/renovate/containerbase"),
            ("RENOVATE_DOCKER_CHILD_PREFIX", "rr_"),
            ("RENOVATE_DOCKER_CLI_OPTIONS", "--network=host"),
            ("RENOVATE_DOCKER_SIDECAR_IMAGE", "example/sidecar:1"),
            ("RENOVATE_DOCKER_USER", "1000:1000"),
            ("RENOVATE_EXECUTION_TIMEOUT", "20"),
            ("RENOVATE_GIT_TIMEOUT", "10000"),
            ("RENOVATE_HTTP_CACHE_TTL_DAYS", "45"),
        ]))
        .unwrap();

        assert_eq!(config.user_agent.as_deref(), Some("renovate-rust-test"));
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

    // Ported: "supports custom prefixes" — workers/global/config/parse/env.spec.ts line 67
    #[test]
    fn custom_prefix_is_supported() {
        let config =
            build_from_env(&env(&[("ENV_PREFIX", "FOOBAR_"), ("FOOBAR_TOKEN", "abc")])).unwrap();
        assert_eq!(config.token.as_deref(), Some("abc"));
    }

    // Ported: "supports json" — workers/global/config/parse/env.spec.ts line 76
    #[test]
    fn lock_file_maintenance_json_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_LOCK_FILE_MAINTENANCE", "{}")])).unwrap();
        assert!(config.lock_file_maintenance.is_empty());
    }

    // Ported: "supports arrays of objects" — workers/global/config/parse/env.spec.ts line 83
    #[test]
    fn host_rules_array_is_parsed() {
        let config =
            build_from_env(&env(&[("RENOVATE_HOST_RULES", r#"[{"foo":"bar"}]"#)])).unwrap();
        assert_eq!(config.host_rules.len(), 1);
        assert_eq!(config.host_rules[0]["foo"], "bar");
    }

    // Ported: "\"$envArg\" -> $config" — workers/global/config/parse/env.spec.ts line 91
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

    // Ported: "skips misconfigured arrays" — workers/global/config/parse/env.spec.ts line 103
    #[test]
    fn host_rules_string_value_is_skipped() {
        let config = build_from_env(&env(&[("RENOVATE_HOST_RULES", r#""foobar""#)])).unwrap();
        assert!(config.host_rules.is_empty());
    }

    #[test]
    fn host_rules_object_value_is_skipped() {
        let config =
            build_from_env(&env(&[("RENOVATE_HOST_RULES", r#"{"matchHost":"github.com"}"#)]))
                .unwrap();
        assert!(config.host_rules.is_empty());
    }

    // Ported: "skips garbage array values" — workers/global/config/parse/env.spec.ts line 117
    #[test]
    fn host_rules_garbage_value_is_skipped() {
        let config = build_from_env(&env(&[("RENOVATE_HOST_RULES", "!@#")])).unwrap();
        assert!(config.host_rules.is_empty());
    }

    // Ported: "supports GitHub token" — workers/global/config/parse/env.spec.ts line 131
    #[test]
    fn github_token_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_TOKEN", "github.com token")])).unwrap();
        assert_eq!(config.platform, Platform::Github);
        assert_eq!(config.token.as_deref(), Some("github.com token"));
    }

    // Ported: "supports GitHub custom endpoint" — workers/global/config/parse/env.spec.ts line 140
    #[test]
    fn github_endpoint_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_ENDPOINT", "a ghe endpoint")])).unwrap();
        assert_eq!(config.platform, Platform::Github);
        assert_eq!(config.endpoint.as_deref(), Some("a ghe endpoint"));
    }

    // Ported: "supports GitHub custom endpoint and github.com" — workers/global/config/parse/env.spec.ts line 149
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

    // Ported: "supports GitHub fine-grained PATs" — workers/global/config/parse/env.spec.ts line 168
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

    // Ported: "supports RENOVATE_ prefixed github com token" — workers/global/config/parse/env.spec.ts line 185
    #[test]
    fn renovate_prefixed_github_com_token_becomes_host_rule() {
        let config = build_from_env(&env(&[
            ("RENOVATE_GITHUB_COM_TOKEN", "github_pat_XXXXXX"),
            ("RENOVATE_TOKEN", "a github.com token"),
        ]))
        .unwrap();
        assert_eq!(config.host_rules[0]["token"], "github_pat_XXXXXX");
    }

    // Ported: "GITHUB_COM_TOKEN takes precedence over RENOVATE_GITHUB_COM_TOKEN" — workers/global/config/parse/env.spec.ts line 202
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

    // Ported: "supports GitHub custom endpoint and gitlab.com" — workers/global/config/parse/env.spec.ts line 220
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

    // Ported: "supports GitLab token" — workers/global/config/parse/env.spec.ts line 231
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

    // Ported: "supports GitLab custom endpoint" — workers/global/config/parse/env.spec.ts line 242
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

    // Ported: "supports Azure DevOps" — workers/global/config/parse/env.spec.ts line 255
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

    // Ported: "supports Bitbucket token" — workers/global/config/parse/env.spec.ts line 268
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

    // Ported: "supports Bitbucket username/password" — workers/global/config/parse/env.spec.ts line 283
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

    // Ported: "merges full config from env" — workers/global/config/parse/env.spec.ts line 299
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

    // Ported: "massages converted experimental env vars" — workers/global/config/parse/env.spec.ts line 309
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
            ("RENOVATE_X_DELETE_CONFIG_FILE", "true"),
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
        assert_eq!(config.docker_max_pages, Some(10));
        assert!(config.delete_config_file);
        assert_eq!(config.s3_endpoint.as_deref(), Some("endpoint"));
        assert!(config.s3_path_style);
        assert_eq!(config.repository_cache_force_local, Some(true));
    }

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

    // Ported: "does not migrate empty RENOVATE_X_REPO_CACHE_FORCE_LOCAL" — workers/global/config/parse/env.spec.ts line 336
    #[test]
    fn empty_repo_cache_force_local_is_not_migrated() {
        let config = build_from_env(&env(&[("RENOVATE_X_REPO_CACHE_FORCE_LOCAL", "")])).unwrap();
        assert_eq!(config.repository_cache_force_local, None);
    }

    // Ported: "crashes" — workers/global/config/parse/env.spec.ts line 357
    #[test]
    fn invalid_renovate_config_is_rejected() {
        let err = build_from_env(&env(&[("RENOVATE_CONFIG", "!@#")])).unwrap_err();
        assert_eq!(err, "Invalid RENOVATE_CONFIG: '!@#'");
    }

    // Ported: "migrates RENOVATE_CONFIG" — workers/global/config/parse/env.spec.ts line 367
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

    // Ported: "renames migrated variables" — workers/global/config/parse/env.spec.ts line 386
    #[test]
    fn git_lab_automerge_env_sets_platform_automerge() {
        let config = build_from_env(&env(&[("RENOVATE_GIT_LAB_AUTOMERGE", "true")])).unwrap();
        assert!(config.platform_automerge);
    }

    // Ported: "renames migrated variables" — workers/global/config/parse/env.spec.ts line 386
    #[test]
    fn renamed_env_vars_map_to_current_options() {
        let config = build_from_env(&env(&[
            (
                "RENOVATE_ALLOWED_POST_UPGRADE_COMMANDS",
                "npm install,cargo update",
            ),
            ("RENOVATE_ALIASES", r#"{"docker.io":"registry.example.com"}"#),
            ("RENOVATE_AZURE_AUTO_COMPLETE", "false"),
            ("RENOVATE_MERGE_CONFIDENCE_API_BASE_URL", "https://mc.example"),
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

    #[test]
    fn allowed_commands_env_json5_array_is_parsed() {
        let config = build_from_env(&env(&[(
            "RENOVATE_ALLOWED_COMMANDS",
            "['npm install','cargo update',]",
        )]))
        .unwrap();
        assert_eq!(config.allowed_commands, vec!["npm install", "cargo update"]);
    }

    #[test]
    fn renamed_allowed_commands_env_json5_array_is_parsed() {
        let config = build_from_env(&env(&[(
            "RENOVATE_ALLOWED_POST_UPGRADE_COMMANDS",
            "['npm install','cargo update',]",
        )]))
        .unwrap();
        assert_eq!(config.allowed_commands, vec!["npm install", "cargo update"]);
    }

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

    #[test]
    fn command_templating_env_is_parsed() {
        let config =
            build_from_env(&env(&[("RENOVATE_ALLOW_COMMAND_TEMPLATING", "true")])).unwrap();
        assert!(config.allow_command_templating);
    }

    #[test]
    fn global_security_env_options_are_parsed() {
        let config = build_from_env(&env(&[
            ("RENOVATE_ALLOWED_HEADERS", "X-*,Authorization"),
            ("RENOVATE_ALLOWED_ENV", "['SOME_*','OTHER_*']"),
            ("RENOVATE_DETECT_GLOBAL_MANAGER_CONFIG", "true"),
            ("RENOVATE_DETECT_HOST_RULES_FROM_ENV", "false"),
        ]))
        .unwrap();

        assert_eq!(
            config.allowed_headers,
            Some(vec!["X-*".to_owned(), "Authorization".to_owned()])
        );
        assert_eq!(
            config.allowed_env,
            Some(vec!["SOME_*".to_owned(), "OTHER_*".to_owned()])
        );
        assert_eq!(config.detect_global_manager_config, Some(true));
        assert_eq!(config.detect_host_rules_from_env, Some(false));
    }

    #[test]
    fn repository_cache_env_options_are_parsed() {
        let config = build_from_env(&env(&[
            ("RENOVATE_REPOSITORY_CACHE", "enabled"),
            ("RENOVATE_REPOSITORY_CACHE_TYPE", "s3"),
        ]))
        .unwrap();

        assert_eq!(config.repository_cache.as_deref(), Some("enabled"));
        assert_eq!(config.repository_cache_type.as_deref(), Some("s3"));
    }

    #[test]
    fn report_env_options_are_parsed() {
        let config = build_from_env(&env(&[
            ("RENOVATE_REPORT_TYPE", "file"),
            ("RENOVATE_REPORT_PATH", "./report.json"),
            ("RENOVATE_REPORT_FORMATTING", "true"),
        ]))
        .unwrap();

        assert_eq!(config.report_type.as_deref(), Some("file"));
        assert_eq!(config.report_path.as_deref(), Some("./report.json"));
        assert_eq!(config.report_formatting, Some(true));
    }

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

    #[test]
    fn invalid_object_env_values_are_rejected() {
        let err =
            build_from_env(&env(&[("RENOVATE_LOCK_FILE_MAINTENANCE", "not-json")])).unwrap_err();
        assert_eq!(
            err,
            "RENOVATE_LOCK_FILE_MAINTENANCE was invalid: Error: Invalid JSON value: 'not-json'"
        );
    }

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

    // Ported: "dryRun boolean true" — workers/global/config/parse/env.spec.ts line 441
    #[test]
    fn dry_run_true_maps_to_full() {
        let config = build_from_env(&env(&[("RENOVATE_DRY_RUN", "true")])).unwrap();
        assert_eq!(config.dry_run, Some(DryRun::Full));
    }

    // Ported: "dryRun boolean false" — workers/global/config/parse/env.spec.ts line 449
    #[test]
    fn dry_run_false_disables_dry_run() {
        let config = build_from_env(&env(&[("RENOVATE_DRY_RUN", "false")])).unwrap();
        assert_eq!(config.dry_run, None);
    }

    // Ported: "dryRun null" — workers/global/config/parse/env.spec.ts line 457
    #[test]
    fn dry_run_null_disables_dry_run() {
        let config = build_from_env(&env(&[("RENOVATE_DRY_RUN", "null")])).unwrap();
        assert_eq!(config.dry_run, None);
    }

    // Ported: "requireConfig boolean true" — workers/global/config/parse/env.spec.ts line 465
    #[test]
    fn require_config_true_maps_to_required() {
        let config = build_from_env(&env(&[("RENOVATE_REQUIRE_CONFIG", "true")])).unwrap();
        assert_eq!(config.require_config, RequireConfig::Required);
    }

    // Ported: "requireConfig boolean false" — workers/global/config/parse/env.spec.ts line 473
    #[test]
    fn require_config_false_maps_to_optional() {
        let config = build_from_env(&env(&[("RENOVATE_REQUIRE_CONFIG", "false")])).unwrap();
        assert_eq!(config.require_config, RequireConfig::Optional);
    }

    #[test]
    fn fork_processing_env_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_FORK_PROCESSING", "enabled")])).unwrap();
        assert_eq!(config.fork_processing, ForkProcessing::Enabled);
    }

    #[test]
    fn binary_source_env_is_parsed() {
        let config = build_from_env(&env(&[("RENOVATE_BINARY_SOURCE", "hermit")])).unwrap();
        assert_eq!(config.binary_source, Some(BinarySource::Hermit));
    }

    #[test]
    fn binary_source_auto_env_maps_to_global() {
        let config = build_from_env(&env(&[("RENOVATE_BINARY_SOURCE", "auto")])).unwrap();
        assert_eq!(config.binary_source, Some(BinarySource::Global));
    }

    #[test]
    fn invalid_binary_source_env_is_rejected() {
        let err = build_from_env(&env(&[("RENOVATE_BINARY_SOURCE", "invalid")])).unwrap_err();
        assert_eq!(
            err,
            "RENOVATE_BINARY_SOURCE was invalid: Invalid value `invalid` for `binarySource`. The allowed values are docker, global, install, hermit."
        );
    }

    // Ported: "platformCommit boolean true" — workers/global/config/parse/env.spec.ts line 481
    #[test]
    fn platform_commit_true_maps_to_enabled() {
        let config = build_from_env(&env(&[("RENOVATE_PLATFORM_COMMIT", "true")])).unwrap();
        assert_eq!(config.platform_commit.as_deref(), Some("enabled"));
    }

    // Ported: "platformCommit boolean false" — workers/global/config/parse/env.spec.ts line 489
    #[test]
    fn platform_commit_false_maps_to_disabled() {
        let config = build_from_env(&env(&[("RENOVATE_PLATFORM_COMMIT", "false")])).unwrap();
        assert_eq!(config.platform_commit.as_deref(), Some("disabled"));
    }

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
}
