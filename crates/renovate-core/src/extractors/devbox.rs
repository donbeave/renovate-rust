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
                        out.push(DevboxDep {
                            name: name.to_owned(),
                            version: version.to_owned(),
                        });
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
                    out.push(DevboxDep {
                        name: name.clone(),
                        version: version.to_owned(),
                    });
                }
            }
        }
        _ => {}
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn object_form() {
        let content = r#"{"packages": {"node": "18", "python": "3.11.5"}}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(deps.iter().any(|d| d.name == "node"));
        assert!(deps.iter().any(|d| d.name == "python"));
    }

    #[test]
    fn object_with_version_field() {
        let content = r#"{"packages": {"node": {"version": "18.0"}}}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].version, "18.0");
    }

    #[test]
    fn no_packages_key_returns_empty() {
        let content = r#"{"name": "myproject"}"#;
        assert!(extract(content).is_empty());
    }

    #[test]
    fn invalid_json_returns_empty() {
        assert!(extract("not json").is_empty());
    }

    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }
}
