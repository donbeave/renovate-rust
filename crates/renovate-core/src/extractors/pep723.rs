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
    let block = match BLOCK_RE.captures(content) {
        Some(cap) => cap[1].to_owned(),
        None => return Vec::new(),
    };

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

    #[test]
    fn returns_empty_for_no_block() {
        let content = "import requests\nprint('hello')\n";
        assert!(extract(content).is_empty());
    }

    #[test]
    fn returns_empty_for_non_script_block() {
        let content = r#"# /// readme
# This is not a script block
# ///
"#;
        assert!(extract(content).is_empty());
    }

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
}
