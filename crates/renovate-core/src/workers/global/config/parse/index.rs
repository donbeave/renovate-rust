//! Config parsing.
//!
//! Mirrors `lib/workers/global/config/parse/index.ts`.
//! @parity lib/workers/global/config/parse/index.ts partial — low-level `parse_config` / `parse_config_file` (now with JSON5 support for trailing commas/comments to match upstream config file flexibility and the usage inside the `parseConfigs` composition). The high-level `parseConfigs` (merging of defaults + file + additional + cli + env, globalExtends resolution, detectGlobalManagerConfig, detectHostRulesFromEnv, repository override warning, various massaging, private key loading, secrets/variables application, configFileNames, etc.) is implemented in the CLI layer (`config_builder.rs`, `main.rs`, and the sub-parsers) in the current Rust architecture.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParsedConfig {
    pub config: serde_json::Value,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub config_file: Option<String>,
}

pub fn parse_config(input: &str) -> ParsedConfig {
    // Use json5 (like the rest of the port and upstream config file support) so that
    // the low-level primitive matches the flexibility expected by the composition in
    // parseConfigs (file + additional + etc.).
    let (config, errors) = match json5::from_str::<serde_json::Value>(input) {
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
        let result =
            parse_config(r#"{"repositories": ["org/repo"], "enabled": true, "packageRules": []}"#);
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

    // The single test added for this cycle (proves the JSON5 support added to the
    // low-level parse primitive that is used by the composition equivalent to parseConfigs).
    #[test]
    fn parse_config_supports_json5() {
        // Ported: JSON5 support in low-level config text parsing (trailing commas, comments)
        // as used by fileParser / the overall parseConfigs flow in lib/workers/global/config/parse/index.ts.
        let result = parse_config(
            r#"{
                "enabled": true,
                "repositories": ["a/b"], // comment
            }"#,
        );
        assert!(result.errors.is_empty());
        assert_eq!(result.config["enabled"], true);
        assert_eq!(result.config["repositories"][0], "a/b");
    }
}
