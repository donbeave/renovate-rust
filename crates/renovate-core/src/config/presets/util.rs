//! Preset utility functions and error constants.
//!
//! Renovate reference: `lib/config/presets/util.ts`.

use serde_json::Value;

pub const PRESET_DEP_NOT_FOUND: &str = "dep not found";
pub const PRESET_INVALID: &str = "invalid preset";
pub const PRESET_INVALID_JSON: &str = "invalid preset JSON";
pub const PRESET_NOT_FOUND: &str = "preset not found";
pub const PRESET_PROHIBITED_SUBPRESET: &str = "prohibited sub-preset";
pub const PRESET_RENOVATE_CONFIG_NOT_FOUND: &str = "preset renovate-config not found";

/// Parse preset content (JSON string) into a config value.
///
/// Mirrors `parsePreset()` from `lib/config/presets/util.ts`.
pub fn parse_preset_content(content: Option<&str>, file_name: &str) -> Result<Value, String> {
    let Some(content) = content else {
        return Err(PRESET_DEP_NOT_FOUND.to_owned());
    };
    let trimmed = content.trim();
    if trimmed.is_empty() {
        return Err(PRESET_DEP_NOT_FOUND.to_owned());
    }
    if file_name.ends_with(".json5") {
        json5::from_str(trimmed).map_err(|e| format!("{PRESET_INVALID_JSON}: {e}"))
    } else {
        serde_json::from_str(trimmed).map_err(|e| format!("{PRESET_INVALID_JSON}: {e}"))
    }
}

/// Ensure a URL-like endpoint string has a trailing slash.
pub fn ensure_trailing_slash(s: &str) -> String {
    if s.ends_with('/') {
        s.to_owned()
    } else {
        format!("{s}/")
    }
}

/// Build a file path for a preset within a repository.
pub fn build_preset_file_path(path_prefix: &str, file_name: &str) -> String {
    let prefix = if path_prefix.is_empty() {
        String::new()
    } else if path_prefix.ends_with('/') {
        path_prefix.to_owned()
    } else {
        format!("{path_prefix}/")
    };
    format!("{prefix}{file_name}")
}

/// Normalize a preset reference string by stripping the source prefix.
pub fn normalize_preset_reference(input: &str) -> String {
    let prefixes = ["github>", "gitlab>", "gitea>", "forgejo>", "local>", "npm>"];
    let mut result = input.to_owned();
    for prefix in prefixes {
        if result.starts_with(prefix) {
            result = result[prefix.len()..].to_owned();
            break;
        }
    }
    result
}

/// Check if a preset source type is supported.
pub fn is_preset_supported(preset_source: &str) -> bool {
    matches!(
        preset_source,
        "github" | "gitlab" | "npm" | "http" | "local" | "forgejo" | "gitea" | "internal"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_preset_content_valid_json() {
        let result = parse_preset_content(Some(r#"{"extends": ["config:base"]}"#), "default.json");
        assert!(result.is_ok());
        assert_eq!(result.unwrap()["extends"][0], "config:base");
    }

    #[test]
    fn parse_preset_content_invalid_json() {
        let result = parse_preset_content(Some("{invalid"), "default.json");
        assert!(result.is_err());
    }

    #[test]
    fn parse_preset_content_none_returns_error() {
        let result = parse_preset_content(None, "default.json");
        assert_eq!(result.unwrap_err(), PRESET_DEP_NOT_FOUND);
    }

    #[test]
    fn ensure_trailing_slash_adds_slash() {
        assert_eq!(
            ensure_trailing_slash("https://api.github.com"),
            "https://api.github.com/"
        );
    }

    #[test]
    fn ensure_trailing_slash_keeps_existing() {
        assert_eq!(
            ensure_trailing_slash("https://api.github.com/"),
            "https://api.github.com/"
        );
    }

    #[test]
    fn normalize_strips_prefix() {
        assert_eq!(
            normalize_preset_reference("github>owner/repo"),
            "owner/repo"
        );
        assert_eq!(normalize_preset_reference("npm>package"), "package");
    }

    #[test]
    fn is_preset_supported_checks() {
        assert!(is_preset_supported("github"));
        assert!(is_preset_supported("internal"));
        assert!(!is_preset_supported("unknown"));
    }
}
