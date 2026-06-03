//! JSON writer with configurable indentation.
//!
//! Renovate reference: `lib/util/json-writer/json-writer.ts`

use serde::Serialize;
use serde_json::ser::PrettyFormatter;

/// Indentation type for JSON output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndentationType {
    Space,
    Tab,
}

/// Configuration for the JSON writer.
#[derive(Debug, Clone)]
pub struct JsonWriterConfig {
    pub indentation_type: IndentationType,
    pub indentation_size: usize,
}

impl Default for JsonWriterConfig {
    fn default() -> Self {
        JsonWriterConfig {
            indentation_type: IndentationType::Space,
            indentation_size: 2,
        }
    }
}

/// Write JSON with configurable indentation.
///
/// Mirrors `lib/util/json-writer/json-writer.ts` `JSONWriter`.
pub fn write_json<T: Serialize>(
    value: &T,
    config: &JsonWriterConfig,
    trailing_newline: bool,
) -> String {
    let indent = match config.indentation_type {
        IndentationType::Tab => "\t".to_owned(),
        IndentationType::Space => " ".repeat(config.indentation_size),
    };

    let mut out = Vec::new();
    let formatter = PrettyFormatter::with_indent(indent.as_bytes());
    let mut ser = serde_json::Serializer::with_formatter(&mut out, formatter);
    value.serialize(&mut ser).expect("serialization failed");

    let mut result = String::from_utf8(out).expect("valid utf8");
    if trailing_newline {
        result.push('\n');
    }
    result
}

/// Strip whitespace from a JSON string by parsing and re-serializing compactly.
///
/// Returns `None` if the input is empty or cannot be parsed.
///
/// Mirrors `lib/workers/repository/config-migration/branch/rebase.ts`
/// `jsonStripWhitespaces()`.
pub fn json_strip_whitespaces(json: &str) -> Option<String> {
    if json.is_empty() {
        return None;
    }
    let value: serde_json::Value = serde_json::from_str(json).ok()?;
    serde_json::to_string(&value).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // Ported: "should apply 2 spaces indentation by default" — lib/util/json-writer/json-writer.spec.ts line 8
    #[test]
    fn json_writer_default_2_space_indent() {
        let data = json!({ "value": 1 });
        let result = write_json(&data, &JsonWriterConfig::default(), true);
        assert_eq!(result, "{\n  \"value\": 1\n}\n");
    }

    // Ported: "should apply indentation size" — lib/util/json-writer/json-writer.spec.ts line 14
    #[test]
    fn json_writer_custom_indent_size() {
        let data = json!({ "value": 1 });
        let config = JsonWriterConfig {
            indentation_type: IndentationType::Space,
            indentation_size: 10,
        };
        let result = write_json(&data, &config, true);
        assert_eq!(result, "{\n          \"value\": 1\n}\n");
    }

    // Ported: "should apply indentation type" — lib/util/json-writer/json-writer.spec.ts line 23
    #[test]
    fn json_writer_tab_indent() {
        let data = json!({ "value": 1 });
        let config = JsonWriterConfig {
            indentation_type: IndentationType::Tab,
            indentation_size: 1,
        };
        let result = write_json(&data, &config, true);
        assert_eq!(result, "{\n\t\"value\": 1\n}\n");
    }

    // Ported: "new line at the end should be optional" — lib/util/json-writer/json-writer.spec.ts line 31
    #[test]
    fn json_writer_optional_trailing_newline() {
        let data = json!({ "value": 1 });
        let config = JsonWriterConfig {
            indentation_type: IndentationType::Space,
            indentation_size: 10,
        };
        let result = write_json(&data, &config, false);
        assert_eq!(result, "{\n          \"value\": 1\n}");
    }

    // Ported: "should strip white spaces from json" — lib/workers/repository/config-migration/branch/rebase.spec.ts line 138
    #[test]
    fn json_strip_whitespaces_removes_formatting() {
        let data = json!({ "name": "renovate", "enabled": true, "count": 3 });
        let formatted = serde_json::to_string_pretty(&data).unwrap();
        let stripped = json_strip_whitespaces(&formatted).unwrap();
        assert_eq!(stripped, serde_json::to_string(&data).unwrap());
    }
}
