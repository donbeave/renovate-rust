//! OSGi feature model (`.json`) Maven bundle dependency extractor.
//!
//! Parses OSGi Compendium R8 feature model files and extracts Maven GAV
//! identifiers from the `bundles` array and custom artifact list sections.
//!
//! Renovate reference:
//! - `lib/modules/manager/osgi/extract.ts`
//! - Pattern: `/(^|/)src/main/features/.+\.json$/`
//! - Datasource: Maven
//!
//! ## File format
//!
//! ```json
//! {
//!   "feature-resource-version": "1.0",
//!   "bundles": [
//!     "org.apache.felix:org.apache.felix.framework:7.0.5",
//!     { "id": "org.osgi:osgi.cmpn:8.0.0" }
//!   ]
//! }
//! ```
//!
//! The version must be the last `:` separated field.
//! Both `group/artifact/version` (slash) and `group:artifact:version` (colon)
//! are accepted separators, but we always normalize to colon internally.

use serde::Deserialize;
use serde_json::Value;

/// Why an OSGi dep is skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OsgiSkipReason {
    /// The version field contains a property placeholder.
    ContainsVariable,
    /// The GAV has fewer than 3 or more than 5 parts.
    InvalidValue,
}

/// A single extracted OSGi Maven bundle dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OsgiDep {
    /// Maven coordinates `groupId:artifactId`.
    pub dep_name: String,
    /// Version string.
    pub current_value: String,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<OsgiSkipReason>,
}

#[derive(Debug, Deserialize)]
struct FeatureModel {
    #[serde(rename = "feature-resource-version")]
    feature_resource_version: Option<String>,
    bundles: Option<Vec<Value>>,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, Value>,
}

/// Extract Maven bundle deps from an OSGi feature model JSON file.
pub fn extract(content: &str) -> Vec<OsgiDep> {
    let model: FeatureModel = match json5::from_str(content.trim()) {
        Ok(m) => m,
        Err(_) => return Vec::new(),
    };

    // Validate feature-resource-version: must be 1.x
    if let Some(ref ver) = model.feature_resource_version {
        let major = ver
            .split('.')
            .next()
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);
        if major != 1 {
            return Vec::new();
        }
    }

    let mut all_bundles: Vec<String> = Vec::new();

    // Standard bundles array
    if let Some(bundles) = &model.bundles {
        for entry in bundles {
            if let Some(id) = bundle_id(entry) {
                all_bundles.push(id);
            }
        }
    }

    // `execution-environment:JSON|false` framework bundle
    if let Some(exec_env) = model.extra.get("execution-environment:JSON|false")
        && let Some(framework) = exec_env.get("framework")
        && let Some(id) = bundle_id(framework)
    {
        all_bundles.push(id);
    }

    // Custom artifact list sections (keys ending with `:JSON|false` or containing
    // arrays of bundle-like strings are checked heuristically).
    for (key, value) in &model.extra {
        // Skip known non-artifact sections
        if key == "execution-environment:JSON|false" {
            continue;
        }
        if let Some(arr) = value.as_array() {
            for entry in arr {
                if let Some(id) = bundle_id(entry) {
                    // Only include if it looks like a GAV (has colons or slashes)
                    if id.contains(':') || id.contains('/') {
                        all_bundles.push(id);
                    }
                }
            }
        }
    }

    all_bundles
        .iter()
        .filter_map(|raw| parse_gav(raw))
        .collect()
}

fn bundle_id(entry: &Value) -> Option<String> {
    match entry {
        Value::String(s) => Some(s.clone()),
        Value::Object(obj) => obj.get("id")?.as_str().map(|s| s.to_owned()),
        _ => None,
    }
}

fn parse_gav(raw: &str) -> Option<OsgiDep> {
    // Normalize slashes to colons
    let gav = raw.replace('/', ":");

    let parts: Vec<&str> = gav.split(':').collect();
    if parts.len() < 3 || parts.len() > 5 {
        return Some(OsgiDep {
            dep_name: gav.clone(),
            current_value: String::new(),
            skip_reason: Some(OsgiSkipReason::InvalidValue),
        });
    }

    let dep_name = format!("{}:{}", parts[0], parts[1]);
    let current_value = parts[parts.len() - 1].to_owned();

    if current_value.contains("${") {
        return Some(OsgiDep {
            dep_name,
            current_value,
            skip_reason: Some(OsgiSkipReason::ContainsVariable),
        });
    }

    Some(OsgiDep {
        dep_name,
        current_value,
        skip_reason: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "extracts the bundles from a file with string bundles defintions" — osgi/extract.spec.ts line 193
    #[test]
    fn extracts_string_bundle() {
        let content = r#"{
  "feature-resource-version": "1.0",
  "bundles": [
    "org.apache.felix:org.apache.felix.framework:7.0.5"
  ]
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].dep_name,
            "org.apache.felix:org.apache.felix.framework"
        );
        assert_eq!(deps[0].current_value, "7.0.5");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "extracts the bundles from a file with object bundles definitions" — osgi/extract.spec.ts line 171
    #[test]
    fn extracts_object_bundle() {
        let content = r#"{
  "feature-resource-version": "1.0",
  "bundles": [
    { "id": "org.osgi:osgi.cmpn:8.0.0" }
  ]
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "org.osgi:osgi.cmpn");
        assert_eq!(deps[0].current_value, "8.0.0");
    }

    // Ported: "extracts the bundles from a file with string bundles defintions" — osgi/extract.spec.ts line 193
    #[test]
    fn slash_separator_normalized() {
        let content = r#"{
  "feature-resource-version": "1.0",
  "bundles": [
    "org.apache.felix/org.apache.felix.gogo.shell/1.1.4"
  ]
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].dep_name,
            "org.apache.felix:org.apache.felix.gogo.shell"
        );
        assert_eq!(deps[0].current_value, "1.1.4");
    }

    // Ported: "skips artifacts with variables in version" — osgi/extract.spec.ts line 297
    #[test]
    fn variable_version_skipped() {
        let content = r#"{
  "feature-resource-version": "1.0",
  "bundles": [
    "org.foo:bar:${project.version}"
  ]
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(OsgiSkipReason::ContainsVariable));
    }

    // Ported: "returns null for unsupported version of feature model definition" — osgi/extract.spec.ts line 151
    #[test]
    fn unsupported_version_skipped() {
        let content = r#"{
  "feature-resource-version": "2.0",
  "bundles": ["org.foo:bar:1.0.0"]
}"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "returns null for invalid file" — osgi/extract.spec.ts line 147
    #[test]
    fn invalid_json_returns_empty() {
        assert!(extract("not json").is_empty());
    }

    // Ported: "extracts the bundles from a file with comments" — osgi/extract.spec.ts line 215
    #[test]
    fn json_with_comments() {
        let content = r#"{
  // feature-resource-version check
  "feature-resource-version": "1.0",
  "bundles": [
    "org.apache.felix:framework:7.0.5" // felix framework
  ]
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "7.0.5");
    }

    // Ported: "returns null for a null string passed in as a feature model definition" — osgi/extract.spec.ts line 163
    #[test]
    fn null_string_returns_empty() {
        assert!(extract("null").is_empty());
    }

    // Ported: "returns null for empty file" — osgi/extract.spec.ts line 143
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "returns null for a valid file with no artifact definitions" — osgi/extract.spec.ts line 167
    #[test]
    fn no_bundles_returns_empty() {
        let content = r#"{"feature-resource-version": "1.0"}"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "returns null for an invalid version of feature model definition" — osgi/extract.spec.ts line 157
    #[test]
    fn invalid_feature_version_returns_empty() {
        let content = r#"{
  "feature-resource-version": "unknown",
  "bundles": [{"id": "commons-codec:commons-codec:1.15", "start-order": "5"}]
}"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "extracts the artifacts from an extension section" — osgi/extract.spec.ts line 228
    #[test]
    fn extracts_from_extension_section() {
        let content = r#"{
  "content-packages:ARTIFACTS|true": [
    "com.day.cq:core.wcm.components.all:zip:2.21.0"
  ]
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "com.day.cq:core.wcm.components.all");
        assert_eq!(deps[0].current_value, "2.21.0");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "extracts the artifacts a file with a double slash" — osgi/extract.spec.ts line 241
    #[test]
    fn double_slash_in_value_not_treated_as_comment() {
        let content = r#"{
  "bundles": [
    {"id": "com.h2database:h2-mvstore:2.1.214", "start-order": "15"},
    {"id": "org.mongodb:mongo-java-driver:3.12.11", "start-order": "15"}
  ],
  "configurations": {
    "org.apache.jackrabbit.oak.plugins.document.DocumentNodeStoreService": {
      "mongouri": "mongodb://localhost:27017"
    }
  }
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(
            deps.iter()
                .any(|d| d.dep_name == "com.h2database:h2-mvstore")
        );
        assert!(
            deps.iter()
                .any(|d| d.dep_name == "org.mongodb:mongo-java-driver")
        );
    }

    // Ported: "extracts the artifacts from the framework artifact section" — osgi/extract.spec.ts line 263
    #[test]
    fn extracts_from_framework_artifact_section() {
        let content = r#"{
  "execution-environment:JSON|false": {
    "framework": {
      "id": "org.apache.felix:org.apache.felix.framework:7.0.5"
    }
  }
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].dep_name,
            "org.apache.felix:org.apache.felix.framework"
        );
        assert_eq!(deps[0].current_value, "7.0.5");
    }

    // Ported: "skips depedencies with with malformed definitions" — osgi/extract.spec.ts line 276
    #[test]
    fn malformed_definitions_skipped_with_valid_kept() {
        let content = r##"{
  "bundles": [
    {"#": "missing id", "not-id": "commons-codec:commons-codec:1.15"},
    {"id": "commons-codec:1.15"},
    {"id": "commons-codec:commons-codec:1.15"}
  ]
}"##;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        let invalid = deps.iter().find(|d| d.dep_name == "commons-codec:1.15");
        assert!(invalid.is_some());
        assert_eq!(
            invalid.unwrap().skip_reason,
            Some(OsgiSkipReason::InvalidValue)
        );
        let valid = deps
            .iter()
            .find(|d| d.dep_name == "commons-codec:commons-codec");
        assert!(valid.is_some());
        assert!(valid.unwrap().skip_reason.is_none());
    }
}
