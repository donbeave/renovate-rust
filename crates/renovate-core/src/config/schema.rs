//! Config schema validation.
//!
//! Renovate reference: `lib/config/schema.ts`.

use serde_json::Value;

/// Error from config schema validation.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ConfigSchemaError {
    #[error("Config must be an object")]
    NotObject,
    #[error("Invalid value for key `{key}`: {message}")]
    InvalidValue { key: String, message: String },
    #[error("Unknown config key: {key}")]
    UnknownKey { key: String },
}

/// Validate a config value against basic schema rules.
///
/// This performs structural validation (type checks, required fields).
/// Full JSON schema validation would require a JSON Schema library.
///
/// Mirrors the validation intent from `lib/config/schema.ts`.
pub fn validate_config_schema(config: &Value) -> Result<(), Vec<ConfigSchemaError>> {
    let mut errors = Vec::new();

    if !config.is_object() {
        errors.push(ConfigSchemaError::NotObject);
        return Err(errors);
    }

    let obj = config.as_object().unwrap();

    if let Some(extends) = obj.get("extends") {
        if !extends.is_array() && !extends.is_string() {
            errors.push(ConfigSchemaError::InvalidValue {
                key: "extends".to_owned(),
                message: "must be an array or string".to_owned(),
            });
        }
    }

    if let Some(enabled) = obj.get("enabled") {
        if !enabled.is_boolean() {
            errors.push(ConfigSchemaError::InvalidValue {
                key: "enabled".to_owned(),
                message: format!("must be a boolean, got {}", value_type(enabled)),
            });
        }
    }

    if let Some(schedule) = obj.get("schedule") {
        if !schedule.is_array() && !schedule.is_string() {
            errors.push(ConfigSchemaError::InvalidValue {
                key: "schedule".to_owned(),
                message: "must be an array or string".to_owned(),
            });
        }
    }

    if let Some(package_rules) = obj.get("packageRules") {
        if !package_rules.is_array() {
            errors.push(ConfigSchemaError::InvalidValue {
                key: "packageRules".to_owned(),
                message: "must be an array".to_owned(),
            });
        }
    }

    if let Some(labels) = obj.get("labels") {
        if !labels.is_array() {
            errors.push(ConfigSchemaError::InvalidValue {
                key: "labels".to_owned(),
                message: "must be an array".to_owned(),
            });
        }
    }

    if let Some(pr_hourly_limit) = obj.get("prHourlyLimit") {
        if !pr_hourly_limit.is_number() {
            errors.push(ConfigSchemaError::InvalidValue {
                key: "prHourlyLimit".to_owned(),
                message: "must be a number".to_owned(),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn value_type(val: &Value) -> &'static str {
    match val {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn valid_config_passes() {
        let config = json!({"enabled": true, "extends": ["config:recommended"]});
        assert!(validate_config_schema(&config).is_ok());
    }

    #[test]
    fn non_object_fails() {
        let config = json!("not an object");
        let errors = validate_config_schema(&config).unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(matches!(errors[0], ConfigSchemaError::NotObject));
    }

    #[test]
    fn invalid_enabled_type() {
        let config = json!({"enabled": "yes"});
        let errors = validate_config_schema(&config).unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, ConfigSchemaError::InvalidValue { key, .. } if key == "enabled")));
    }

    #[test]
    fn invalid_package_rules_type() {
        let config = json!({"packageRules": "not array"});
        let errors = validate_config_schema(&config).unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, ConfigSchemaError::InvalidValue { key, .. } if key == "packageRules")));
    }
}
