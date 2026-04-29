//! Gleam `gleam.toml` dependency extractor.
//!
//! Parses `dependencies` and `dev-dependencies` sections of `gleam.toml`
//! files and maps each entry to the Hex.pm datasource.
//!
//! Renovate reference:
//! - `lib/modules/manager/gleam/extract.ts`
//! - Pattern: `/(^|/)gleam\.toml$/`
//!
//! ## Supported form
//!
//! ```toml
//! [dependencies]
//! gleam_stdlib = "~> 0.34"
//! lustre = ">= 4.0.0, < 5.0.0"
//!
//! [dev-dependencies]
//! gleeunit = "~> 1.0"
//! ```

/// A single extracted Gleam dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GleamDep {
    pub name: String,
    pub version: String,
    pub dev: bool,
}

/// Parse `gleam.toml` and extract all Hex.pm dependencies.
pub fn extract(content: &str) -> Vec<GleamDep> {
    let mut out = Vec::new();
    let mut in_deps = false;
    let mut in_dev_deps = false;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        // Section headers.
        if trimmed.starts_with('[') {
            in_deps = trimmed == "[dependencies]";
            in_dev_deps = trimmed == "[dev-dependencies]";
            continue;
        }

        if !in_deps && !in_dev_deps {
            continue;
        }

        // Parse `name = "version"` entries.
        if let Some((name_raw, val_raw)) = trimmed.split_once('=') {
            let name = name_raw.trim();
            let version = val_raw.trim().trim_matches('"').trim_matches('\'').trim();
            if !name.is_empty() && !version.is_empty() {
                out.push(GleamDep {
                    name: name.to_owned(),
                    version: version.to_owned(),
                    dev: in_dev_deps,
                });
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_dependencies() {
        let content = r#"
[dependencies]
gleam_stdlib = "~> 0.34"
lustre = ">= 4.0.0, < 5.0.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(deps.iter().any(|d| d.name == "gleam_stdlib" && !d.dev));
        assert!(deps.iter().any(|d| d.name == "lustre" && !d.dev));
    }

    #[test]
    fn extracts_dev_dependencies() {
        let content = r#"
[dev-dependencies]
gleeunit = "~> 1.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert!(deps[0].dev);
        assert_eq!(deps[0].name, "gleeunit");
    }

    #[test]
    fn both_sections() {
        let content = r#"
[dependencies]
gleam_stdlib = "~> 0.34"

[dev-dependencies]
gleeunit = "~> 1.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
    }

    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    #[test]
    fn no_deps_section_returns_empty() {
        // Ported: "should return null when no dependencies are found" — gleam/extract.spec.ts line 65
        let content = r#"name = "test"\nversion = "1.0.0"\n\n[unknown]\ngleam_http = "~> 3.6.0""#;
        assert!(extract(content).is_empty());
    }

    #[test]
    fn invalid_toml_returns_empty() {
        // Ported: "should return null when gleam.toml is invalid" — gleam/extract.spec.ts line 82
        assert!(extract("foo").is_empty());
    }
}
