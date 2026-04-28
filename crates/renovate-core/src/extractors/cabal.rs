//! Haskell Cabal `*.cabal` dependency extractor.
//!
//! Finds `build-depends:` fields in Cabal package description files and
//! extracts Hackage package names with their version constraints.
//!
//! Renovate reference:
//! - `lib/modules/manager/haskell-cabal/extract.ts`
//! - Pattern: `/\.cabal$/`
//! - Datasource: Hackage (`https://hackage.haskell.org/`)
//!
//! ## Supported form
//!
//! ```cabal
//! build-depends:
//!     base >= 4.7 && < 5
//!   , text == 2.0.0
//!   , aeson >= 2.0
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// A single extracted Cabal dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CabalDep {
    /// Package name on Hackage (e.g. `"text"`).
    pub package_name: String,
    /// Version constraint string (e.g. `">= 4.7 && < 5"`). Empty if unconstrained.
    pub current_value: String,
}

// ── Compiled regexes ─────────────────────────────────────────────────────────

/// Matches `build-depends:` (case-insensitive) at any indentation.
static BUILD_DEPENDS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)build-depends\s*:(.*)").unwrap());

/// Matches `-- ...` Cabal line comments.
static COMMENT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"--.*$").unwrap());

/// A valid Haskell package name: starts with letter/digit, contains letters/digits/hyphens.
static PKG_NAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([A-Za-z0-9][A-Za-z0-9\-]*)").unwrap());

/// Extract Cabal deps from a `*.cabal` file.
pub fn extract(content: &str) -> Vec<CabalDep> {
    let mut out = Vec::new();
    let mut dep_field: Option<String> = None;
    let mut field_indent: usize = 0;

    for raw in content.lines() {
        // Strip comments.
        let line = COMMENT.replace(raw, "");
        let line = line.trim_end();

        // When not in a field, look for `build-depends:`.
        if dep_field.is_none() {
            if let Some(cap) = BUILD_DEPENDS.captures(line) {
                field_indent = leading_spaces(raw);
                let inline = cap[1].to_string();
                dep_field = Some(inline);
            }
            continue;
        }

        // We're inside a build-depends block.
        let indent = leading_spaces(raw);
        if indent <= field_indent && !line.trim().starts_with(',') && !line.trim().is_empty() {
            // Exited the field — flush.
            if let Some(field) = dep_field.take() {
                parse_field(&field, &mut out);
            }
            // Check if this new line starts another build-depends.
            if let Some(cap) = BUILD_DEPENDS.captures(line) {
                field_indent = indent;
                dep_field = Some(cap[1].to_string());
            }
        } else {
            // Continue collecting field content.
            if let Some(ref mut f) = dep_field {
                f.push('\n');
                f.push_str(line);
            }
        }
    }

    // Flush remaining.
    if let Some(field) = dep_field {
        parse_field(&field, &mut out);
    }

    out
}

/// Parse a `build-depends` field value into individual deps.
fn parse_field(field: &str, out: &mut Vec<CabalDep>) {
    // Split on commas; each entry is `package [constraint]`.
    for entry in field.split(',') {
        let trimmed = entry.trim();
        if trimmed.is_empty() {
            continue;
        }

        let Some(cap) = PKG_NAME.captures(trimmed) else {
            continue;
        };

        let name = cap[1].to_owned();
        // Anything after the name is the constraint.
        let constraint = trimmed[name.len()..].trim().to_owned();

        out.push(CabalDep {
            package_name: name,
            current_value: constraint,
        });
    }
}

fn leading_spaces(s: &str) -> usize {
    s.len() - s.trim_start_matches([' ', '\t']).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
cabal-version:       2.4
name:                my-project
version:             0.1.0.0

library
  build-depends:
      base >= 4.7 && < 5
    , text == 2.0.0
    , aeson >= 2.0
    , containers
  hs-source-dirs: src

executable my-exe
  build-depends:
      base
    , my-project
  main-is: Main.hs
"#;

    #[test]
    fn extracts_library_deps() {
        let deps = extract(SAMPLE);
        let base = deps.iter().find(|d| d.package_name == "base").unwrap();
        assert_eq!(base.current_value, ">= 4.7 && < 5");
        let text = deps.iter().find(|d| d.package_name == "text").unwrap();
        assert_eq!(text.current_value, "== 2.0.0");
    }

    #[test]
    fn extracts_unconstrained_dep() {
        let deps = extract(SAMPLE);
        let containers = deps
            .iter()
            .find(|d| d.package_name == "containers")
            .unwrap();
        assert_eq!(containers.current_value, "");
    }

    #[test]
    fn extracts_from_multiple_sections() {
        let deps = extract(SAMPLE);
        // base appears in both library and executable
        let base_count = deps.iter().filter(|d| d.package_name == "base").count();
        assert!(base_count >= 2);
    }

    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    #[test]
    fn no_build_depends_returns_empty() {
        let content = "cabal-version: 2.4\nname: foo\nversion: 1.0.0\n";
        assert!(extract(content).is_empty());
    }
}
