//! Config parsing.
//!
//! Mirrors `lib/workers/global/config/parse/index.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParsedConfig {
    pub config: serde_json::Value,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub config_file: Option<String>,
}

pub fn parse_config(input: &str) -> ParsedConfig {
    let (config, errors) = match serde_json::from_str::<serde_json::Value>(input) {
        Ok(v) => (v, Vec::new()),
        Err(e) => (
            serde_json::Value::Null,
            vec![format!("JSON parse error: {e}")],
        ),
    };

    ParsedConfig {
        config,
        errors,
        warnings: Vec::new(),
        config_file: None,
    }
}

pub fn parse_config_file(contents: &str, file_name: &str) -> ParsedConfig {
    let mut result = parse_config(contents);
    result.config_file = Some(file_name.to_owned());
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsed_config_default() {
        let c = ParsedConfig::default();
        assert!(c.config.is_null());
        assert!(c.errors.is_empty());
        assert!(c.warnings.is_empty());
        assert!(c.config_file.is_none());
    }

    #[test]
    fn parse_config_valid_json() {
        let result = parse_config(r#"{"enabled": true}"#);
        assert!(result.errors.is_empty());
        assert_eq!(result.config["enabled"], true);
    }

    #[test]
    fn parse_config_invalid_json() {
        let result = parse_config("not json");
        assert!(!result.errors.is_empty());
        assert!(result.config.is_null());
    }

    #[test]
    fn parse_config_complex() {
        let result = parse_config(
            r#"{"repositories": ["org/repo"], "enabled": true, "packageRules": []}"#,
        );
        assert!(result.errors.is_empty());
        assert_eq!(result.config["repositories"][0], "org/repo");
    }

    #[test]
    fn parse_config_file_sets_name() {
        let result = parse_config_file("{}", "renovate.json");
        assert_eq!(result.config_file, Some("renovate.json".to_owned()));
    }

    #[test]
    fn parse_config_empty_object() {
        let result = parse_config("{}");
        assert!(result.errors.is_empty());
        assert!(result.config.is_object());
    }

    #[test]
    fn parsed_config_serialization_roundtrip() {
        let c = ParsedConfig {
            config: serde_json::json!({"key": "value"}),
            errors: vec![],
            warnings: vec!["warn".into()],
            config_file: Some("config.json".into()),
        };
        let json = serde_json::to_string(&c).unwrap();
        let back: ParsedConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(back.config_file, Some("config.json".into()));
    }
}
