//! Regex/glob matcher validation.
//!
//! Renovate reference: `lib/config/validation-helpers/regex-glob-matchers.ts`.

use regex::Regex;

use super::types::{CheckMatcherArgs, ValidationMessage};

/// Check if a string is a regex pattern (enclosed in `/`).
fn is_regex_match(s: &str) -> bool {
    s.starts_with('/') && s.ends_with('/') && s.len() >= 2
}

/// Validate regex/glob patterns in matchers.
///
/// Mirrors `check()` from `lib/config/validation-helpers/regex-glob-matchers.ts`.
pub fn check_regex_glob_matchers(args: &CheckMatcherArgs) -> Vec<ValidationMessage> {
    let mut result = Vec::new();

    if let Some(matchers) = args.val.as_array() {
        let string_matchers: Vec<&str> = matchers.iter().filter_map(|v| v.as_str()).collect();

        if (string_matchers.iter().any(|m| *m == "*" || *m == "**")) && matchers.len() > 1 {
            result.push(ValidationMessage {
                topic: "Configuration Error".to_owned(),
                message: format!(
                    "{}: Your input contains * or ** along with other patterns. Please remove them, as * or ** matches all patterns.",
                    args.current_path
                ),
            });
        }

        for matcher in &string_matchers {
            if is_regex_match(matcher) {
                let pattern = matcher.trim_start_matches('/').trim_end_matches('/');
                if Regex::new(pattern).is_err() {
                    result.push(ValidationMessage {
                        topic: "Configuration Error".to_owned(),
                        message: format!(
                            "Failed to parse regex pattern for {}: {}",
                            args.current_path, matcher
                        ),
                    });
                }
            }
        }
    } else {
        result.push(ValidationMessage {
            topic: "Configuration Error".to_owned(),
            message: format!(
                "{}: should be an array of strings. You have included {}.",
                args.current_path,
                val_type_name(&args.val)
            ),
        });
    }

    result
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
    fn check_warns_on_wildcard_with_other_patterns() {
        let args = CheckMatcherArgs {
            val: json!(["*", "other"]),
            current_path: "matchPackageNames".to_owned(),
        };
        let result = check_regex_glob_matchers(&args);
        assert_eq!(result.len(), 1);
        assert!(result[0].message.contains("* or **"));
    }

    #[test]
    fn check_passes_on_wildcard_alone() {
        let args = CheckMatcherArgs {
            val: json!(["*"]),
            current_path: "matchPackageNames".to_owned(),
        };
        assert!(check_regex_glob_matchers(&args).is_empty());
    }

    #[test]
    fn check_warns_on_invalid_regex() {
        let args = CheckMatcherArgs {
            val: json!(["/invalid regex [/", "valid"]),
            current_path: "matchPackageNames".to_owned(),
        };
        let result = check_regex_glob_matchers(&args);
        assert_eq!(result.len(), 1);
        assert!(result[0].message.contains("Failed to parse regex"));
    }

    #[test]
    fn check_warns_on_non_array() {
        let args = CheckMatcherArgs {
            val: json!("not an array"),
            current_path: "matchPackageNames".to_owned(),
        };
        let result = check_regex_glob_matchers(&args);
        assert_eq!(result.len(), 1);
        assert!(result[0].message.contains("should be an array"));
    }

    #[test]
    fn check_passes_on_valid_patterns() {
        let args = CheckMatcherArgs {
            val: json!(["/^@scope/", "plain-string"]),
            current_path: "matchPackageNames".to_owned(),
        };
        assert!(check_regex_glob_matchers(&args).is_empty());
    }
}
