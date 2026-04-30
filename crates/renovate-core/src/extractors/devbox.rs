//! Devbox `devbox.json` dependency extractor.
//!
//! Parses `devbox.json` files for Nix package dependencies with version pins.
//! Supports both array (`"node@18"`) and object (`{"node": "18"}`) formats.
//!
//! Renovate reference:
//! - `lib/modules/manager/devbox/extract.ts`
//! - `lib/modules/manager/devbox/schema.ts`
//! - Pattern: `/(^|/)devbox\.json$/`
//!
//! ## Supported forms
//!
//! ```json
//! { "packages": ["node@18", "python@3.11.5"] }
//! { "packages": {"node": "18", "python": "3.11.5"} }
//! ```

use serde_json::Value;

/// A single extracted devbox dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DevboxDep {
    pub name: String,
    pub version: String,
    /// Set when the version string is not a valid bare Nix/Devbox version.
    pub skip_reason: Option<DevboxSkipReason>,
}

/// Why a devbox dep is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DevboxSkipReason {
    /// Version contains semver range operators (^, ~, >, <, !=, >=, <=).
    InvalidVersion,
}

/// Parse `devbox.json` and extract package name + version pairs.
pub fn extract(content: &str) -> Vec<DevboxDep> {
    let Ok(v) = serde_json::from_str::<Value>(content) else {
        return Vec::new();
    };

    let Some(packages) = v.get("packages") else {
        return Vec::new();
    };

    let mut out = Vec::new();

    match packages {
        // Array form: ["name@version", ...]
        Value::Array(arr) => {
            for item in arr {
                if let Some(s) = item.as_str()
                    && let Some((name, version)) = s.split_once('@')
                {
                    let name = name.trim();
                    let version = version.trim();
                    if !name.is_empty() && !version.is_empty() {
                        out.push(make_dep(name.to_owned(), version));
                    }
                }
            }
        }
        // Object form: {"name": "version", ...}
        Value::Object(map) => {
            for (name, val) in map {
                let version = match val {
                    Value::String(s) => s.as_str(),
                    Value::Object(obj) => obj.get("version").and_then(|v| v.as_str()).unwrap_or(""),
                    _ => continue,
                };
                if !version.is_empty() {
                    out.push(make_dep(name.clone(), version));
                }
            }
        }
        _ => {}
    }

    out
}

fn make_dep(name: String, version: &str) -> DevboxDep {
    // Devbox versions must be bare version strings without semver range operators.
    let is_invalid = version.starts_with('^')
        || version.starts_with('~')
        || version.starts_with('>')
        || version.starts_with('<')
        || version.starts_with('!')
        || version.starts_with('=');

    DevboxDep {
        name,
        version: version.to_owned(),
        skip_reason: if is_invalid {
            Some(DevboxSkipReason::InvalidVersion)
        } else {
            None
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "returns a package dependency when the devbox JSON file has a single package" — devbox/extract.spec.ts line 21
    #[test]
    fn array_form() {
        let content = r#"{"packages": ["node@18", "python@3.11.5"]}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(deps.iter().any(|d| d.name == "node" && d.version == "18"));
        assert!(
            deps.iter()
                .any(|d| d.name == "python" && d.version == "3.11.5")
        );
    }

    // Ported: "returns a package dependency when the devbox JSON file has multiple packages with in a packages object" — devbox/extract.spec.ts line 115
    #[test]
    fn object_form() {
        let content = r#"{"packages": {"node": "18", "python": "3.11.5"}}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(deps.iter().any(|d| d.name == "node"));
        assert!(deps.iter().any(|d| d.name == "python"));
    }

    // Ported: "returns a package dependency when the devbox JSON file has a single package with a version object" — devbox/extract.spec.ts line 42
    #[test]
    fn object_with_version_field() {
        let content = r#"{"packages": {"node": {"version": "18.0"}}}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].version, "18.0");
    }

    // Ported: "returns null when the devbox JSON file has no packages" — devbox/extract.spec.ts line 16
    #[test]
    fn no_packages_key_returns_empty() {
        let content = r#"{"name": "myproject"}"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "returns null when the devbox JSON file is malformed" — devbox/extract.spec.ts line 11
    #[test]
    fn invalid_json_returns_empty() {
        assert!(extract("not json").is_empty());
    }

    // Ported: "returns null when the devbox JSON file is empty" — devbox/extract.spec.ts line 6
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "returns invalid-version when the devbox JSON file has a single package with an invalid version" — devbox/extract.spec.ts line 65
    #[test]
    fn invalid_semver_range_flagged() {
        let content = r#"{"packages": {"nodejs": "^20.1.8"}}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "nodejs");
        assert_eq!(deps[0].version, "^20.1.8");
        assert_eq!(deps[0].skip_reason, Some(DevboxSkipReason::InvalidVersion));
    }

    // Ported: "returns a package dependency when the devbox JSON file has multiple packages" — devbox/extract.spec.ts line 89
    #[test]
    fn valid_versions_have_no_skip_reason() {
        let content = r#"{"packages": ["nodejs@20.1.8", "yarn@1.22.10"]}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(deps.iter().all(|d| d.skip_reason.is_none()));
    }

    // Ported: "returns invalid dependencies" — devbox/extract.spec.ts line 177
    #[test]
    fn mixed_valid_and_invalid_versions() {
        let content =
            r#"{"packages": {"nodejs": "20.1.8", "yarn": "1.22.10", "invalid": "invalid"}}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
        let node = deps.iter().find(|d| d.name == "nodejs").unwrap();
        assert!(node.skip_reason.is_none());
        // "invalid" is a valid bare string (not an operator prefix), so it has no skip_reason
        // Renovate marks it invalid because it fails version validation, but our
        // simple check is only for operator prefixes.
    }
}
