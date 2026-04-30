//! Home Assistant `manifest.json` dependency extractor.
//!
//! Parses the `requirements` array from Home Assistant custom integration
//! manifests and returns PyPI package deps.
//!
//! Renovate reference:
//! - `lib/modules/manager/homeassistant-manifest/extract.ts`
//! - Pattern: `/(^|/)manifest\.json$/`
//! - Datasource: PyPI
//!
//! ## File format
//!
//! ```json
//! {
//!   "domain": "my_integration",
//!   "name": "My Integration",
//!   "requirements": ["requests==2.28.0", "aiohttp>=3.8.0"]
//! }
//! ```

use serde::Deserialize;

use crate::extractors::pip::PipExtractedDep;

#[derive(Debug, Deserialize)]
struct HaManifest {
    domain: Option<String>,
    name: Option<String>,
    /// Chrome Extension Manifest V3 marker — signals this is NOT a HA file.
    manifest_version: Option<serde_json::Value>,
    /// Flexible type to handle mixed arrays (strings, numbers, null).
    requirements: Option<serde_json::Value>,
}

/// Extract PyPI deps from a Home Assistant `manifest.json` file.
///
/// Returns empty if the file is not a valid HA manifest (missing `domain`
/// or `name`, or has `manifest_version` which marks a Chrome extension).
pub fn extract(content: &str) -> Vec<PipExtractedDep> {
    let manifest: HaManifest = match serde_json::from_str(content.trim()) {
        Ok(m) => m,
        Err(_) => return Vec::new(),
    };

    if manifest.domain.is_none() || manifest.name.is_none() {
        return Vec::new();
    }
    if manifest.manifest_version.is_some() {
        return Vec::new();
    }

    let reqs_val = match manifest.requirements {
        Some(serde_json::Value::Array(arr)) => arr,
        _ => return Vec::new(),
    };

    // Filter to string entries only (skip numbers, null, etc.).
    let reqs: Vec<String> = reqs_val
        .into_iter()
        .filter_map(|v| v.as_str().map(str::to_owned))
        .collect();

    if reqs.is_empty() {
        return Vec::new();
    }

    let joined = reqs.join("\n");
    crate::extractors::pip::extract(&joined).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{
  "domain": "my_integration",
  "name": "My Integration",
  "requirements": ["requests==2.28.0", "aiohttp>=3.8.0", "no-version"]
}"#;

    // Ported: "extracts multiple requirements" — homeassistant-manifest/extract.spec.ts line 84
    #[test]
    fn extracts_requirements() {
        let deps = extract(SAMPLE);
        let req = deps.iter().find(|d| d.name == "requests").unwrap();
        assert_eq!(req.current_value, "==2.28.0");
    }

    // Ported: "supports requirements with other operators" — homeassistant-manifest/extract.spec.ts line 168
    #[test]
    fn extracts_range_version() {
        let deps = extract(SAMPLE);
        let aio = deps.iter().find(|d| d.name == "aiohttp").unwrap();
        assert_eq!(aio.current_value, ">=3.8.0");
    }

    // Ported: "returns null for invalid JSON" — homeassistant-manifest/extract.spec.ts line 9
    #[test]
    fn invalid_json_returns_empty() {
        assert!(extract("not valid json").is_empty());
    }

    // Ported: "returns null for empty requirements" — homeassistant-manifest/extract.spec.ts line 45
    #[test]
    fn empty_requirements_returns_empty() {
        let content = r#"{"domain": "test", "name": "Test Integration", "requirements": []}"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "returns null when no requirements field" — homeassistant-manifest/extract.spec.ts line 55
    #[test]
    fn no_requirements_field_returns_empty() {
        let content = r#"{"domain": "test", "name": "Test Integration"}"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "returns null for non-Home Assistant manifest (missing domain)" — homeassistant-manifest/extract.spec.ts line 14
    #[test]
    fn missing_domain_returns_empty() {
        let content = r#"{"name": "My Extension", "version": "1.0.0", "requirements": ["some-package==1.0.0"]}"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "returns null for non-Home Assistant manifest (missing name)" — homeassistant-manifest/extract.spec.ts line 24
    #[test]
    fn missing_name_returns_empty() {
        let content =
            r#"{"domain": "test", "version": "1.0.0", "requirements": ["some-package==1.0.0"]}"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "returns null for chrome extension manifest" — homeassistant-manifest/extract.spec.ts line 34
    #[test]
    fn chrome_extension_manifest_returns_empty() {
        let content = r#"{"manifest_version": 3, "name": "My Extension", "version": "1.0.0", "permissions": ["storage"]}"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "extracts single requirement with exact version" — homeassistant-manifest/extract.spec.ts line 64
    #[test]
    fn extracts_single_requirement_exact_version() {
        let content =
            r#"{"domain": "hue", "name": "Philips Hue", "requirements": ["aiohue==1.9.1"]}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "aiohue");
        assert_eq!(deps[0].current_value, "==1.9.1");
    }

    // Ported: "extracts multiple requirements" — homeassistant-manifest/extract.spec.ts line 84
    #[test]
    fn extracts_multiple_requirements() {
        let content = r#"{"domain": "hue", "name": "Philips Hue", "requirements": ["aiohue==1.9.1", "aiohttp==3.8.1", "pyyaml==6.0"]}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
        assert_eq!(deps[0].name, "aiohue");
        assert_eq!(deps[1].name, "aiohttp");
        assert_eq!(deps[2].name, "pyyaml");
    }

    // Ported: "handles requirements with extras" — homeassistant-manifest/extract.spec.ts line 118
    #[test]
    fn handles_requirements_with_extras() {
        let content = r#"{"domain": "test", "name": "Test", "requirements": ["package[extra1,extra2]==1.0.0"]}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "package");
        assert_eq!(deps[0].current_value, "==1.0.0");
    }

    // Ported: "handles requirements without version" — homeassistant-manifest/extract.spec.ts line 211
    #[test]
    fn handles_requirements_without_version() {
        let content =
            r#"{"domain": "test", "name": "Test", "requirements": ["package", "aiohue==1.9.1"]}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        let aiohue = deps.iter().find(|d| d.name == "aiohue").unwrap();
        assert_eq!(aiohue.current_value, "==1.9.1");
        // bare package name with no version specifier — pip extractor returns it with empty value
        let pkg = deps.iter().find(|d| d.name == "package").unwrap();
        assert!(pkg.current_value.is_empty() || pkg.skip_reason.is_some());
    }

    // Ported: "extracts from real-world ASUSWRT manifest" — homeassistant-manifest/extract.spec.ts line 237
    #[test]
    fn extracts_asuswrt_manifest() {
        let content = r#"{
  "domain": "asuswrt",
  "name": "ASUSWRT",
  "requirements": ["aioasuswrt==1.5.1", "asusrouter==1.21.3"]
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        let a = deps.iter().find(|d| d.name == "aioasuswrt").unwrap();
        assert_eq!(a.current_value, "==1.5.1");
        let b = deps.iter().find(|d| d.name == "asusrouter").unwrap();
        assert_eq!(b.current_value, "==1.21.3");
    }

    // Ported: "handles invalid requirement types in array" — homeassistant-manifest/extract.spec.ts line 272
    #[test]
    fn skips_non_string_entries_in_requirements_array() {
        let content = r#"{"domain": "test", "name": "Test", "requirements": ["aiohue==1.9.1", 123, null, "valid==2.0.0"]}"#;
        let deps = extract(content);
        // Non-string entries (123, null) are filtered out
        assert_eq!(deps.len(), 2);
        assert!(deps.iter().any(|d| d.name == "aiohue"));
        assert!(deps.iter().any(|d| d.name == "valid"));
    }

    // Ported: "returns null when requirements is not an array" — homeassistant-manifest/extract.spec.ts line 299
    #[test]
    fn requirements_not_an_array_returns_empty() {
        let content = r#"{"domain": "test", "name": "Test", "requirements": "not-an-array"}"#;
        assert!(extract(content).is_empty());
    }
}
