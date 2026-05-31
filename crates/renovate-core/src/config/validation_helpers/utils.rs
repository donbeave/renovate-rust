//! Validation helper utilities.
//!
//! Renovate reference: `lib/config/validation-helpers/utils.ts`.

use regex::Regex;

use super::types::ValidationMessage;

/// Get the parent name from a dotted config path.
///
/// Mirrors `getParentName()` from `lib/config/validation-helpers/utils.ts`.
pub fn get_parent_name(parent_path: Option<&str>) -> String {
    let path = match parent_path {
        Some(p) if !p.is_empty() => p,
        _ => return ".".to_owned(),
    };
    let path = Regex::new(r"\.?encrypted$")
        .unwrap()
        .replace(path, "");
    let path = Regex::new(r"\[\d+\]$")
        .unwrap()
        .replace(&path, "");
    path.split('.')
        .next_back()
        .unwrap_or(".")
        .to_owned()
}

/// Validate that a value is a plain string-keyed object with string values.
///
/// Returns `Ok(())` if valid, or `Err(key)` with the first invalid key.
pub fn validate_plain_object(val: &serde_json::Map<String, serde_json::Value>) -> Result<(), String> {
    for (key, value) in val {
        if !value.is_string() {
            return Err(key.clone());
        }
    }
    Ok(())
}

/// Validate that a number value is a positive integer.
///
/// Mirrors `validateNumber()` from `lib/config/validation-helpers/utils.ts`.
pub fn validate_number(
    key: &str,
    val: &serde_json::Value,
    allows_negative: bool,
    current_path: Option<&str>,
    sub_key: Option<&str>,
) -> Vec<ValidationMessage> {
    let mut errors = Vec::new();
    let path = match (current_path, sub_key) {
        (Some(p), Some(sk)) => format!("{p}.{sk}"),
        (Some(p), None) => p.to_owned(),
        (None, Some(sk)) => sk.to_owned(),
        (None, None) => key.to_owned(),
    };
    if let Some(n) = val.as_i64() {
        if n < 0 && !allows_negative {
            errors.push(ValidationMessage {
                topic: "Configuration Error".to_owned(),
                message: format!("Configuration option `{path}` should be a positive integer. Found negative value instead."),
            });
        }
    } else {
        errors.push(ValidationMessage {
            topic: "Configuration Error".to_owned(),
            message: format!("Configuration option `{path}` should be an integer. Found: {val} ({}).", val_type_name(val)),
        });
    }
    errors
}

/// Format a validation message string.
pub fn get_validation_message(topic: &str, message: &str) -> ValidationMessage {
    ValidationMessage {
        topic: topic.to_owned(),
        message: message.to_owned(),
    }
}

/// Check whether a global-only option is actually a "false global"
/// (i.e., valid in hostRules context).
pub fn is_false_global(option_name: &str, parent_path: Option<&str>) -> bool {
    if let Some(path) = parent_path
        && path.contains("hostRules")
        && (option_name == "token" || option_name == "username" || option_name == "password")
    {
        return true;
    }
    false
}

fn val_type_name(val: &serde_json::Value) -> &'static str {
    match val {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "boolean",
        serde_json::Value::Number(_) => "number",
        serde_json::Value::String(_) => "string",
        serde_json::Value::Array(_) => "array",
        serde_json::Value::Object(_) => "object",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn get_parent_name_with_path() {
        assert_eq!(get_parent_name(Some("packageRules.0")), "0");
    }

    #[test]
    fn get_parent_name_none() {
        assert_eq!(get_parent_name(None), ".");
    }

    #[test]
    fn get_parent_name_strips_encrypted() {
        assert_eq!(get_parent_name(Some("hostRules.0.encrypted")), "0");
    }

    #[test]
    fn get_parent_name_strips_array_index() {
        assert_eq!(get_parent_name(Some("packageRules.0.enabled")), "enabled");
    }

    #[test]
    fn validate_plain_object_all_strings() {
        let map = serde_json::from_str(r#"{"a": "b", "c": "d"}"#).unwrap();
        assert!(validate_plain_object(&map).is_ok());
    }

    #[test]
    fn validate_plain_object_non_string_value() {
        let map = serde_json::from_str(r#"{"a": 1}"#).unwrap();
        assert_eq!(validate_plain_object(&map).unwrap_err(), "a");
    }

    #[test]
    fn validate_number_positive() {
        let errors = validate_number("foo", &json!(5), false, None, None);
        assert!(errors.is_empty());
    }

    #[test]
    fn validate_number_negative_not_allowed() {
        let errors = validate_number("foo", &json!(-1), false, None, None);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("positive integer"));
    }

    #[test]
    fn validate_number_negative_allowed() {
        let errors = validate_number("foo", &json!(-1), true, None, None);
        assert!(errors.is_empty());
    }

    #[test]
    fn validate_number_non_integer() {
        let errors = validate_number("foo", &json!("string"), false, None, None);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("should be an integer"));
    }

    #[test]
    fn is_false_global_token_in_host_rules() {
        assert!(is_false_global("token", Some("hostRules.0")));
        assert!(!is_false_global("token", Some("packageRules.0")));
        assert!(!is_false_global("token", None));
    }
}
