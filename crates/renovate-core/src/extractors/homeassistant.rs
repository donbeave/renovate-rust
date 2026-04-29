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
    /// Required: HA domain key (absent in Chrome/browser extension manifests).
    domain: Option<String>,
    /// Required: integration name.
    name: Option<String>,
    /// Present in Chrome Extension Manifest V3 — signals this is NOT a HA file.
    manifest_version: Option<serde_json::Value>,
    requirements: Option<Vec<String>>,
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

    // Must be a Home Assistant manifest: requires `domain` and `name`.
    if manifest.domain.is_none() || manifest.name.is_none() {
        return Vec::new();
    }
    // Chrome extension manifests have `manifest_version` — skip them.
    if manifest.manifest_version.is_some() {
        return Vec::new();
    }

    let reqs = manifest.requirements.unwrap_or_default();
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

    #[test]
    fn extracts_requirements() {
        let deps = extract(SAMPLE);
        let req = deps.iter().find(|d| d.name == "requests").unwrap();
        assert_eq!(req.current_value, "==2.28.0");
    }

    #[test]
    fn extracts_range_version() {
        let deps = extract(SAMPLE);
        let aio = deps.iter().find(|d| d.name == "aiohttp").unwrap();
        assert_eq!(aio.current_value, ">=3.8.0");
    }

    #[test]
    fn invalid_json_returns_empty() {
        // Ported: "returns null for invalid JSON" — homeassistant-manifest/extract.spec.ts line 9
        assert!(extract("not valid json").is_empty());
    }

    #[test]
    fn empty_requirements_returns_empty() {
        // Ported: "returns null for empty requirements" — homeassistant-manifest/extract.spec.ts line 45
        let content = r#"{"domain": "test", "name": "Test Integration", "requirements": []}"#;
        assert!(extract(content).is_empty());
    }

    #[test]
    fn no_requirements_field_returns_empty() {
        // Ported: "returns null when no requirements field" — homeassistant-manifest/extract.spec.ts line 55
        let content = r#"{"domain": "test", "name": "Test Integration"}"#;
        assert!(extract(content).is_empty());
    }

    #[test]
    fn missing_domain_returns_empty() {
        // Ported: "returns null for non-Home Assistant manifest (missing domain)" — spec line 14
        let content = r#"{"name": "My Extension", "version": "1.0.0", "requirements": ["some-package==1.0.0"]}"#;
        assert!(extract(content).is_empty());
    }

    #[test]
    fn missing_name_returns_empty() {
        // Ported: "returns null for non-Home Assistant manifest (missing name)" — spec line 24
        let content =
            r#"{"domain": "test", "version": "1.0.0", "requirements": ["some-package==1.0.0"]}"#;
        assert!(extract(content).is_empty());
    }

    #[test]
    fn chrome_extension_manifest_returns_empty() {
        // Ported: "returns null for chrome extension manifest" — spec line 34
        let content = r#"{"manifest_version": 3, "name": "My Extension", "version": "1.0.0", "permissions": ["storage"]}"#;
        assert!(extract(content).is_empty());
    }

    #[test]
    fn extracts_single_requirement_exact_version() {
        // Ported: "extracts single requirement with exact version" — spec line 64
        let content =
            r#"{"domain": "hue", "name": "Philips Hue", "requirements": ["aiohue==1.9.1"]}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "aiohue");
        assert_eq!(deps[0].current_value, "==1.9.1");
    }

    #[test]
    fn extracts_multiple_requirements() {
        // Ported: "extracts multiple requirements" — spec line 84
        let content = r#"{"domain": "hue", "name": "Philips Hue", "requirements": ["aiohue==1.9.1", "aiohttp==3.8.1", "pyyaml==6.0"]}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
        assert_eq!(deps[0].name, "aiohue");
        assert_eq!(deps[1].name, "aiohttp");
        assert_eq!(deps[2].name, "pyyaml");
    }

    #[test]
    fn handles_requirements_with_extras() {
        // Ported: "handles requirements with extras" — spec line 118
        let content = r#"{"domain": "test", "name": "Test", "requirements": ["package[extra1,extra2]==1.0.0"]}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "package");
        assert_eq!(deps[0].current_value, "==1.0.0");
    }
}
