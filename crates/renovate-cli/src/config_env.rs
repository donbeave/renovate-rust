//! Renovate-compatible global config parsing from environment variables.
//!
//! Renovate reference: `lib/workers/global/config/parse/env.ts` `getConfig`.

use std::collections::BTreeMap;

use renovate_core::config::{DryRun, GlobalConfig, Platform, RecreateWhen, RequireConfig};
use serde_json::json;

use crate::config_builder::{parse_json_array, parse_json_object};

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

    if let Some(value) = env_value(env, prefix, "CONFIG_MIGRATION") {
        config.config_migration = parse_bool("RENOVATE_CONFIG_MIGRATION", value)?;
    }
    if let Some(value) = env_value(env, prefix, "LABELS") {
        config.labels = split_list(value);
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
    if let Some(value) = env_value(env, prefix, "PLATFORM_COMMIT") {
        config.platform_commit = Some(if parse_bool("RENOVATE_PLATFORM_COMMIT", value)? {
            "enabled".to_owned()
        } else {
            "disabled".to_owned()
        });
    }
    if let Some(value) = env_value(env, prefix, "HOST_RULES") {
        config.host_rules = parse_json_array(value).unwrap_or_default();
    }
    if let Some(value) = env_value(env, prefix, "LOCK_FILE_MAINTENANCE") {
        config.lock_file_maintenance = parse_json_object(value).unwrap_or_default();
    }
    if let Some(token) = env
        .get("GITHUB_COM_TOKEN")
        .or_else(|| env.get("RENOVATE_GITHUB_COM_TOKEN"))
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
    if let Some(value) = env_value(env, prefix, "GIT_LAB_AUTOMERGE") {
        config.platform_automerge = parse_bool("RENOVATE_GIT_LAB_AUTOMERGE", value)?;
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
    env.get(&key).map(String::as_str)
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

fn split_list(value: &str) -> Vec<String> {
    value
        .split(',')
        .filter(|item| !item.is_empty())
        .map(str::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::build_from_env;
    use renovate_core::config::{DryRun, Platform, RecreateWhen, RequireConfig};
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
        let config = build_from_env(&env(&[("RENOVATE_LABELS", "a,b,c,")])).unwrap();
        assert_eq!(config.labels, vec!["a", "b", "c"]);
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

    // Ported: "renames migrated variables" — workers/global/config/parse/env.spec.ts line 386
    #[test]
    fn git_lab_automerge_env_sets_platform_automerge() {
        let config = build_from_env(&env(&[("RENOVATE_GIT_LAB_AUTOMERGE", "true")])).unwrap();
        assert!(config.platform_automerge);
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
}
