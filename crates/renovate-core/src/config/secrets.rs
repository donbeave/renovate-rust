//! Config secrets and variables interpolation.
//!
//! Renovate reference: `lib/config/secrets.ts`.

use std::collections::BTreeMap;

use regex::Regex;
use serde_json::Value;

/// Error raised while validating or applying secrets and variables.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum SecretsError {
    /// Invalid `secrets` config shape or name.
    #[error("config secrets invalid")]
    InvalidSecrets,
    /// Invalid `variables` config shape or name.
    #[error("config variables invalid")]
    InvalidVariables,
    /// Missing secret/variable referenced by a template.
    #[error("config validation failed")]
    ConfigValidation,
}

/// Validate top-level and repository-level `secrets` / `variables` maps.
pub fn validate_config_secrets_and_variables(config: &Value) -> Result<(), SecretsError> {
    validate_nested(config, "secrets", SecretsError::InvalidSecrets)?;
    validate_nested(config, "variables", SecretsError::InvalidVariables)?;
    Ok(())
}

/// Apply `secrets` and `variables` template replacements to config.
pub fn apply_secrets_and_variables_to_config(
    config: &Value,
    delete_secrets: bool,
    delete_variables: bool,
) -> Result<Value, SecretsError> {
    let secrets = collect_named_values(config.get("secrets"), SecretsError::InvalidSecrets)?;
    let variables = collect_named_values(config.get("variables"), SecretsError::InvalidVariables)?;

    let mut result = config.clone();
    replace_values(&mut result, "variables", &variables)?;
    replace_values(&mut result, "secrets", &secrets)?;

    if let Value::Object(map) = &mut result {
        if delete_secrets {
            map.remove("secrets");
        }
        if delete_variables {
            map.remove("variables");
        }
    }

    Ok(result)
}

fn validate_nested(
    config: &Value,
    key: &'static str,
    err: SecretsError,
) -> Result<(), SecretsError> {
    validate_named_values(config.get(key), err)?;
    if let Some(Value::Array(repositories)) = config.get("repositories") {
        for repository in repositories {
            if repository.is_object() {
                validate_named_values(repository.get(key), err)?;
            }
        }
    }
    Ok(())
}

fn validate_named_values(value: Option<&Value>, err: SecretsError) -> Result<(), SecretsError> {
    let Some(value) = value else {
        return Ok(());
    };
    let Value::Object(values) = value else {
        return Err(err);
    };
    for (name, value) in values {
        if !is_valid_name(name) || !matches!(value, Value::String(_)) {
            return Err(err);
        }
    }
    Ok(())
}

fn collect_named_values(
    value: Option<&Value>,
    err: SecretsError,
) -> Result<BTreeMap<String, String>, SecretsError> {
    validate_named_values(value, err)?;
    let Some(Value::Object(values)) = value else {
        return Ok(BTreeMap::new());
    };
    Ok(values
        .iter()
        .filter_map(|(name, value)| value.as_str().map(|value| (name.clone(), value.to_owned())))
        .collect())
}

fn replace_values(
    value: &mut Value,
    namespace: &'static str,
    replacements: &BTreeMap<String, String>,
) -> Result<(), SecretsError> {
    match value {
        Value::String(s) => {
            *s = replace_string(s, namespace, replacements)?;
        }
        Value::Array(values) => {
            for value in values {
                replace_values(value, namespace, replacements)?;
            }
        }
        Value::Object(values) => {
            for value in values.values_mut() {
                replace_values(value, namespace, replacements)?;
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) => {}
    }
    Ok(())
}

fn replace_string(
    input: &str,
    namespace: &'static str,
    replacements: &BTreeMap<String, String>,
) -> Result<String, SecretsError> {
    let pattern = format!(
        r"\{{\{{ *{}\.(?P<name>[A-Za-z][A-Za-z0-9_]*) *\}}\}}",
        namespace
    );
    let regex = Regex::new(&pattern).expect("valid template regex");
    let mut missing = false;
    let result = regex.replace_all(input, |captures: &regex::Captures<'_>| {
        let name = captures.name("name").expect("name capture").as_str();
        match replacements.get(name) {
            Some(value) => value.clone(),
            None => {
                missing = true;
                captures.get(0).expect("whole match").as_str().to_owned()
            }
        }
    });
    if missing {
        Err(SecretsError::ConfigValidation)
    } else {
        Ok(result.into_owned())
    }
}

fn is_valid_name(name: &str) -> bool {
    let mut chars = name.chars();
    matches!(chars.next(), Some(ch) if ch.is_ascii_alphabetic())
        && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    // Ported: "works with default config" — config/secrets.spec.ts line 14
    #[test]
    fn validate_config_secrets_and_variables_works_with_default_config() {
        assert!(validate_config_secrets_and_variables(&json!({})).is_ok());
    }

    // Ported: "returns if no secrets/variables" — config/secrets.spec.ts line 20
    #[test]
    fn validate_config_secrets_and_variables_returns_without_entries() {
        assert!(validate_config_secrets_and_variables(&json!({})).is_ok());
    }

    // Ported: "throws for invalid secret name" — config/secrets.spec.ts line 24
    #[test]
    fn validate_config_secrets_and_variables_rejects_invalid_secret_name() {
        assert_eq!(
            validate_config_secrets_and_variables(&json!({"secrets": {"123": "abc"}})),
            Err(SecretsError::InvalidSecrets)
        );
    }

    // Ported: "throws for invalid variable name" — config/secrets.spec.ts line 32
    #[test]
    fn validate_config_secrets_and_variables_rejects_invalid_variable_name() {
        assert_eq!(
            validate_config_secrets_and_variables(&json!({"variables": {"123": "abc"}})),
            Err(SecretsError::InvalidVariables)
        );
    }

    // Ported: "throws for secrets in repositories" — config/secrets.spec.ts line 40
    #[test]
    fn validate_config_secrets_and_variables_rejects_repository_secrets() {
        assert_eq!(
            validate_config_secrets_and_variables(
                &json!({"repositories": [{"repository": "x/y", "secrets": {"abc": 123}}]})
            ),
            Err(SecretsError::InvalidSecrets)
        );
    }

    // Ported: "throws for variables in repositories" — config/secrets.spec.ts line 48
    #[test]
    fn validate_config_secrets_and_variables_rejects_repository_variables() {
        assert_eq!(
            validate_config_secrets_and_variables(
                &json!({"repositories": [{"repository": "x/y", "variables": {"abc": 123}}]})
            ),
            Err(SecretsError::InvalidVariables)
        );
    }

    // Ported: "replaces both secrets and variables" — config/secrets.spec.ts line 58
    #[test]
    fn apply_secrets_and_variables_replaces_both() {
        let result = apply_secrets_and_variables_to_config(
            &json!({
                "secrets": {"TOKEN": "secret123"},
                "variables": {"MANAGER": "npm"},
                "hostRules": [{"hostType": "{{ variables.MANAGER }}", "token": "{{ secrets.TOKEN }}"}]
            }),
            true,
            true,
        )
        .unwrap();

        assert_eq!(
            result,
            json!({"hostRules": [{"hostType": "npm", "token": "secret123"}]})
        );
    }

    // Ported: "replaces all secrets and variables" — config/secrets.spec.ts line 75
    #[test]
    fn apply_secrets_and_variables_replaces_all() {
        let result = apply_secrets_and_variables_to_config(
            &json!({
                "secrets": {"FOO": "foo", "BAR": "bar", "BAZ": "baz"},
                "variables": {"FOO": "foo", "BAR": "bar", "BAZ": "baz"},
                "customEnvVariables": {
                    "SECRETS": "{{ secrets.FOO }} {{ secrets.BAR }} {{ secrets.BAZ }}",
                    "VARIABLES": "{{ variables.FOO }} {{ variables.BAR }} {{ variables.BAZ }}"
                }
            }),
            true,
            true,
        )
        .unwrap();

        assert_eq!(
            result,
            json!({"customEnvVariables": {"SECRETS": "foo bar baz", "VARIABLES": "foo bar baz"}})
        );
    }

    // Ported: "handles a mix of space characters around the curly braces" — config/secrets.spec.ts line 94
    #[test]
    fn apply_secrets_and_variables_handles_spaces_around_braces() {
        let result = apply_secrets_and_variables_to_config(
            &json!({
                "secrets": {"TOKEN": "secret123"},
                "variables": {"MANAGER": "npm"},
                "hostRules": [{"hostType": "{{variables.MANAGER   }}", "token": "{{secrets.TOKEN}}"}]
            }),
            true,
            true,
        )
        .unwrap();

        assert_eq!(
            result,
            json!({"hostRules": [{"hostType": "npm", "token": "secret123"}]})
        );
    }

    // Ported: "does not handle non-space characters around the curly braces" — config/secrets.spec.ts line 111
    #[test]
    fn apply_secrets_and_variables_does_not_handle_non_space_characters() {
        let result = apply_secrets_and_variables_to_config(
            &json!({
                "secrets": {"TOKEN": "secret123"},
                "variables": {"MANAGER": "npm"},
                "hostRules": [{"hostType": "{{variables.MANAGER   }}", "token": "{{\tsecrets.token\t}}"}]
            }),
            true,
            true,
        )
        .unwrap();

        assert_eq!(
            result,
            json!({"hostRules": [{"hostType": "npm", "token": "{{\tsecrets.token\t}}"}]})
        );
    }

    // Ported: "preserves secrets and variables if delete flags are false" — config/secrets.spec.ts line 128
    #[test]
    fn apply_secrets_and_variables_preserves_sources_when_delete_flags_are_false() {
        let config = json!({
            "secrets": {"TOKEN": "secret123"},
            "variables": {"MANAGER": "npm"},
            "hostRules": [{"hostType": "{{ variables.MANAGER }}", "token": "{{ secrets.TOKEN }}"}]
        });
        let result = apply_secrets_and_variables_to_config(&config, false, false).unwrap();

        assert_eq!(
            result,
            json!({
                "secrets": {"TOKEN": "secret123"},
                "variables": {"MANAGER": "npm"},
                "hostRules": [{"hostType": "npm", "token": "secret123"}]
            })
        );
    }

    // Ported: "throws if secret is missing" — config/secrets.spec.ts line 151
    #[test]
    fn apply_secrets_and_variables_errors_if_secret_missing() {
        assert_eq!(
            apply_secrets_and_variables_to_config(
                &json!({"hostRules": [{"token": "{{ secrets.MISSING_SECRET }}"}]}),
                true,
                true
            ),
            Err(SecretsError::ConfigValidation)
        );
    }

    // Ported: "throws if variable is missing" — config/secrets.spec.ts line 160
    #[test]
    fn apply_secrets_and_variables_errors_if_variable_missing() {
        assert_eq!(
            apply_secrets_and_variables_to_config(
                &json!({"hostRules": [{"hostType": "{{ variables.MISSING_VAR }}"}]}),
                true,
                true
            ),
            Err(SecretsError::ConfigValidation)
        );
    }

    // ── interpolator.spec.ts — replaceInterpolatedValuesInObject ─────────────

    // Ported: "replaces values and deletes secrets" — util/interpolator.spec.ts line 48
    #[test]
    fn replaces_values_and_deletes_secrets() {
        let config = json!({
            "mode": "{{ secrets.SECRET_MODE }}",
            "labels": ["{{ secrets.SECRET_LABEL }}", "renovate"],
            "prBodyDefinitions": {
                "Package": "{{ secrets.SECRET_PACKAGE }}",
                "Type": "peer"
            },
            "hostRules": [{"matchHost": "{{ secrets.SECRET_HOST }}"}],
            "secrets": {
                "SECRET_HOST": "host",
                "SECRET_MODE": "silent",
                "SECRET_LABEL": "secret",
                "SECRET_PACKAGE": "package"
            }
        });
        let result = apply_secrets_and_variables_to_config(&config, true, false).unwrap();
        assert_eq!(result["mode"], json!("silent"));
        assert_eq!(result["labels"], json!(["secret", "renovate"]));
        assert_eq!(result["prBodyDefinitions"]["Package"], json!("package"));
        assert_eq!(result["hostRules"][0]["matchHost"], json!("host"));
        assert!(result.get("secrets").is_none(), "secrets should be deleted");
    }

    // Ported: "replaces values and keeps secrets" — util/interpolator.spec.ts line 97
    #[test]
    fn replaces_values_and_keeps_secrets() {
        let config = json!({
            "mode": "{{ secrets.SECRET_MODE }}",
            "secrets": {"SECRET_MODE": "silent"}
        });
        let result = apply_secrets_and_variables_to_config(&config, false, false).unwrap();
        assert_eq!(result["mode"], json!("silent"));
        assert!(result.get("secrets").is_some(), "secrets should be kept");
    }

    // Ported: "throws error if secret key is not present in config" — util/interpolator.spec.ts line 175
    #[test]
    fn errors_if_secret_key_is_not_present_in_config() {
        let config = json!({
            "mode": "{{ secrets.SECRET_MODE }}",
            "secrets": {"SECRET_NOT_MODE": "silent"}
        });
        let result = apply_secrets_and_variables_to_config(&config, false, false);
        assert_eq!(result, Err(SecretsError::ConfigValidation));
    }
}
