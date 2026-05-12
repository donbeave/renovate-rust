//! Renovate-compatible global config parsing from environment variables.
//!
//! Renovate reference: `lib/workers/global/config/parse/env.ts` `getConfig`.

use std::collections::BTreeMap;

use renovate_core::config::{GlobalConfig, RecreateWhen};

use crate::config_builder::parse_json_array;

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
    if let Some(value) = env_value(env, prefix, "HOST_RULES") {
        config.host_rules = parse_json_array(value).unwrap_or_default();
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
    use renovate_core::config::RecreateWhen;
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

    // Ported: "supports custom prefixes" — workers/global/config/parse/env.spec.ts line 67
    #[test]
    fn custom_prefix_is_supported() {
        let config =
            build_from_env(&env(&[("ENV_PREFIX", "FOOBAR_"), ("FOOBAR_TOKEN", "abc")])).unwrap();
        assert_eq!(config.token.as_deref(), Some("abc"));
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
}
