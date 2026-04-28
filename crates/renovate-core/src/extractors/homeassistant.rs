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
    requirements: Option<Vec<String>>,
}

/// Extract PyPI deps from a Home Assistant `manifest.json` file.
pub fn extract(content: &str) -> Vec<PipExtractedDep> {
    let manifest: HaManifest = match serde_json::from_str(content.trim()) {
        Ok(m) => m,
        Err(_) => return Vec::new(),
    };

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
        assert!(extract("not json").is_empty());
    }

    #[test]
    fn empty_requirements_returns_empty() {
        assert!(extract(r#"{"domain": "x", "name": "y"}"#).is_empty());
    }
}
