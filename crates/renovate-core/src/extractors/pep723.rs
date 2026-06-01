//! PEP 723 inline script metadata extractor.
//!
//! Extracts Python package dependencies from PEP 723 metadata blocks
//! embedded as comments in Python files.
//!
//! Renovate reference:
//! - `lib/modules/manager/pep723/extract.ts`, `utils.ts`, `schema.ts`
//! - Default patterns: `[]` (user-configured — any `.py` file can have this)
//! - Datasource: PyPI
//!
//! ## File format
//!
//! ```python
//! # /// script
//! # requires-python = ">=3.11"
//! # dependencies = [
//! #   "requests>=2.28",
//! #   "rich",
//! # ]
//! # ///
//! ```

use std::sync::LazyLock;

use regex::Regex;
use toml::Value;

/// A single PEP 723 dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pep723Dep {
    /// Normalized package name (e.g. `requests`).
    pub name: String,
    /// Version specifier (e.g. `>=2.28`) or empty string for unpinned.
    pub current_value: String,
    pub skip_reason: Option<Pep723SkipReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pep723SkipReason {
    /// Direct reference (`name @ https://…` or `git+…`).
    DirectReference,
    /// No version specifier — dep is unpinned.
    UnspecifiedVersion,
}

// ── PEP 723 block regex ───────────────────────────────────────────────────────

/// Matches a `# /// script` ... `# ///` block (multiline).
///
/// Adapted from the Python reference implementation:
/// <https://packaging.python.org/en/latest/specifications/inline-script-metadata/#reference-implementation>
static BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"# /// script\n((?:#[^\n]*\n)*?)# ///").expect("valid regex"));

// ── Public API ────────────────────────────────────────────────────────────────

/// Extract PEP 723 dependencies from a Python file.
pub fn extract(content: &str) -> Vec<Pep723Dep> {
    let Some(cap) = BLOCK_RE.captures(content) else {
        return Vec::new();
    };
    let block = cap[1].to_owned();

    // Strip `# ` or `#` prefix from each line to recover the TOML text.
    let toml_text: String = block
        .lines()
        .map(|line| {
            if let Some(rest) = line.strip_prefix("# ") {
                rest
            } else if let Some(rest) = line.strip_prefix('#') {
                rest
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    let parsed: Value = match toml::from_str(&toml_text) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let Some(deps_array) = parsed.get("dependencies").and_then(|v| v.as_array()) else {
        return Vec::new();
    };

    deps_array
        .iter()
        .filter_map(|v| v.as_str())
        .map(parse_pep508)
        .collect()
}

/// Parse a single PEP 508 dependency specifier into a `Pep723Dep`.
fn parse_pep508(raw: &str) -> Pep723Dep {
    let raw = raw.trim();

    // Direct references.
    if raw.contains(" @ ") || raw.starts_with("git+") || raw.starts_with("https://") {
        let name = raw
            .split_whitespace()
            .next()
            .unwrap_or("unknown")
            .to_owned();
        return Pep723Dep {
            name,
            current_value: raw.to_owned(),
            skip_reason: Some(Pep723SkipReason::DirectReference),
        };
    }

    // Strip environment markers (`;…`).
    let without_markers = raw.split(';').next().unwrap_or("").trim();

    // Extract name (ends at first non-name char).
    let name_end = without_markers
        .find(|c: char| !c.is_ascii_alphanumeric() && c != '.' && c != '-' && c != '_')
        .unwrap_or(without_markers.len());

    let raw_name = &without_markers[..name_end];
    if raw_name.is_empty() {
        return Pep723Dep {
            name: String::new(),
            current_value: raw.to_owned(),
            skip_reason: Some(Pep723SkipReason::UnspecifiedVersion),
        };
    }

    let name = normalize_name(raw_name);
    let specifier = without_markers[name_end..].trim_start();

    // Strip extras `[…]` before the version specifier.
    let specifier = if specifier.starts_with('[') {
        specifier
            .find(']')
            .map(|i| specifier[i + 1..].trim())
            .unwrap_or(specifier)
    } else {
        specifier
    };

    Pep723Dep {
        name,
        current_value: specifier.to_owned(),
        skip_reason: if specifier.is_empty() {
            Some(Pep723SkipReason::UnspecifiedVersion)
        } else {
            None
        },
    }
}

/// A PEP 723 dependency with full metadata (for `extract_pep723`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pep723FullDep {
    pub dep_name: String,
    pub current_value: String,
    /// Extracted version number (for `==X.Y.Z` specifiers only).
    pub current_version: Option<String>,
}

/// Full result of `extract_pep723`, analogous to TypeScript's `PackageFileContent`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pep723Result {
    pub deps: Vec<Pep723FullDep>,
    /// From `requires-python` in the metadata block.
    pub python_constraint: Option<String>,
}

/// Extract PEP 723 metadata including `requires-python` and deps.
///
/// Mirrors `lib/modules/manager/pep723/utils.ts` `extractPep723()`.
/// Returns `None` when no metadata block is found, TOML is invalid,
/// or no valid dependencies are present.
pub fn extract_pep723(content: &str) -> Option<Pep723Result> {
    let block = BLOCK_RE.captures(content)?;
    let raw_block = &block[1];

    let toml_text: String = raw_block
        .lines()
        .map(|line| {
            if let Some(rest) = line.strip_prefix("# ") {
                rest
            } else if let Some(rest) = line.strip_prefix('#') {
                rest
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    let parsed: Value = toml::from_str(&toml_text).ok()?;

    let python_constraint = parsed
        .get("requires-python")
        .and_then(|v| v.as_str())
        .map(str::to_owned);

    let deps_array = parsed.get("dependencies")?.as_array()?;

    let deps: Vec<Pep723FullDep> = deps_array
        .iter()
        .filter_map(|v| v.as_str())
        .map(parse_pep508)
        .filter(|d| !d.name.is_empty() && d.skip_reason.is_none())
        .map(|d| {
            let current_version = if d.current_value.starts_with("==") {
                Some(d.current_value.trim_start_matches("==").to_owned())
            } else {
                None
            };
            Pep723FullDep {
                dep_name: d.name,
                current_value: d.current_value,
                current_version,
            }
        })
        .collect();

    if deps.is_empty() {
        return None;
    }

    Some(Pep723Result {
        deps,
        python_constraint,
    })
}

/// Normalize a Python package name per PEP 503.
fn normalize_name(name: &str) -> String {
    let lower = name.to_lowercase();
    let mut result = String::with_capacity(lower.len());
    let mut prev_sep = false;
    for ch in lower.chars() {
        if ch == '-' || ch == '_' || ch == '.' {
            if !prev_sep {
                result.push('-');
            }
            prev_sep = true;
        } else {
            result.push(ch);
            prev_sep = false;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "should extract dependencies" — manager/pep723/extract.spec.ts line 10
    #[test]
    fn extracts_script_block_with_version() {
        let content = r#"#!/usr/bin/env python3
# /// script
# requires-python = ">=3.11"
# dependencies = [
#   "requests>=2.28",
#   "rich",
# ]
# ///

import requests
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);

        assert_eq!(deps[0].name, "requests");
        assert_eq!(deps[0].current_value, ">=2.28");
        assert!(deps[0].skip_reason.is_none());

        assert_eq!(deps[1].name, "rich");
        assert_eq!(
            deps[1].skip_reason,
            Some(Pep723SkipReason::UnspecifiedVersion)
        );
    }

    // Rust-specific: pep723 behavior test
    #[test]
    fn returns_empty_for_no_block() {
        let content = "import requests\nprint('hello')\n";
        assert!(extract(content).is_empty());
    }

    // Rust-specific: pep723 behavior test
    #[test]
    fn returns_empty_for_non_script_block() {
        let content = r#"# /// readme
# This is not a script block
# ///
"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "should extract dependencies" — manager/pep723/extract.spec.ts line 10
    #[test]
    fn handles_direct_reference() {
        let content = r#"# /// script
# dependencies = ["my-pkg @ https://example.com/pkg.tar.gz"]
# ///
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(Pep723SkipReason::DirectReference));
    }

    // Ported: "should extract dependencies" — manager/pep723/extract.spec.ts line 10
    #[test]
    fn normalizes_package_name() {
        let content = r#"# /// script
# dependencies = ["Flask_Utils>=1.0"]
# ///
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "flask-utils");
    }

    // Ported: "should extract dependencies" — manager/pep723/extract.spec.ts line 10
    #[test]
    fn extracts_pinned_version() {
        let content = r#"# /// script
# dependencies = ["numpy==1.26.0"]
# ///
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "numpy");
        assert_eq!(deps[0].current_value, "==1.26.0");
    }

    // Ported: "should extract dependencies" — modules/manager/pep723/utils.spec.ts line 6
    #[test]
    fn pep723_extract_should_extract_dependencies() {
        let content = "# /// script\n# requires-python = \">=3.11\"\n# dependencies = [\n#   \"requests==2.32.3\",\n#   \"rich>=13.8.0\",\n# ]\n# ///\n";
        let result = extract_pep723(content).unwrap();
        assert_eq!(result.python_constraint.as_deref(), Some(">=3.11"));
        assert_eq!(result.deps.len(), 2);
        assert_eq!(result.deps[0].dep_name, "requests");
        assert_eq!(result.deps[0].current_value, "==2.32.3");
        assert_eq!(result.deps[0].current_version.as_deref(), Some("2.32.3"));
        assert_eq!(result.deps[1].dep_name, "rich");
        assert_eq!(result.deps[1].current_value, ">=13.8.0");
        assert!(result.deps[1].current_version.is_none());
    }

    // Ported: "should skip invalid dependencies" — modules/manager/pep723/utils.spec.ts line 42
    #[test]
    fn pep723_extract_should_skip_invalid_dependencies() {
        let content = "# /// script\n# requires-python = \"==3.11\"\n# dependencies = [\n#   \"requests==2.32.3\",\n#   \"==1.2.3\",\n# ]\n# ///\n";
        let result = extract_pep723(content).unwrap();
        assert_eq!(result.python_constraint.as_deref(), Some("==3.11"));
        assert_eq!(result.deps.len(), 1);
        assert_eq!(result.deps[0].dep_name, "requests");
    }

    // Ported: "should return null on missing dependencies" — modules/manager/pep723/utils.spec.ts line 71
    #[test]
    fn pep723_extract_returns_none_on_missing_dependencies() {
        let content = "# /// script\n# requires-python = \">=3.11\"\n# ///\n";
        assert!(extract_pep723(content).is_none());
    }

    // Ported: "should return null on invalid TOML" — modules/manager/pep723/utils.spec.ts line 84
    #[test]
    fn pep723_extract_returns_none_on_invalid_toml() {
        let content = "# /// script\n# requires-python\n# dependencies = [\n#   \"requests==2.32.3\",\n# ]\n# ///\n";
        assert!(extract_pep723(content).is_none());
    }

    // Ported: "should return null if there is no PEP 723 metadata" — modules/manager/pep723/utils.spec.ts line 101
    #[test]
    fn pep723_extract_returns_none_if_no_metadata_block() {
        let content = "if True:\n    print(\"requires-python>=3.11\")\n";
        assert!(extract_pep723(content).is_none());
    }
}
