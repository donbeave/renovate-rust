//! Typst (`.typ`) package import extractor.
//!
//! Scans Typst source files for `#import "@preview/pkg:version"` lines and
//! extracts the package name and version for Typst registry lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/typst/extract.ts`
//! - Pattern: `/\.typ$/`
//! - Datasource: Typst
//!
//! ## File format
//!
//! ```typst
//! #import "@preview/cetz:0.2.0"
//! #import "@preview/fletcher:0.4.5": diagram, node, edge
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// Why a Typst dep is skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypstSkipReason {
    /// Package uses the `local` namespace (not in the registry).
    Local,
    /// Package uses an unknown namespace.
    Unsupported,
}

/// A single extracted Typst package dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypstDep {
    /// Package name (e.g. `cetz`).
    pub package_name: String,
    /// Namespace (e.g. `preview`).
    pub namespace: String,
    /// Version string (e.g. `0.2.0`).
    pub current_value: String,
    /// Set when the dep should be skipped.
    pub skip_reason: Option<TypstSkipReason>,
}

/// Matches `#import "@namespace/pkg:version"` (with optional trailing content).
static IMPORT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"#import\s+"@(?P<namespace>[^/]+)/(?P<pkg>[^:]+):(?P<version>[^"]+)""#).unwrap()
});

/// Extract all Typst package deps from a `.typ` file.
///
/// Lines starting with `//` are treated as comments and skipped.
pub fn extract(content: &str) -> Vec<TypstDep> {
    let mut deps = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        // Skip line comments
        if trimmed.starts_with("//") {
            continue;
        }

        for cap in IMPORT_RE.captures_iter(trimmed) {
            let namespace = cap["namespace"].to_owned();
            let package_name = cap["pkg"].to_owned();
            let current_value = cap["version"].to_owned();

            let skip_reason = if namespace == "preview" {
                None
            } else if namespace == "local" {
                Some(TypstSkipReason::Local)
            } else {
                Some(TypstSkipReason::Unsupported)
            };

            deps.push(TypstDep {
                package_name,
                namespace,
                current_value,
                skip_reason,
            });
        }
    }

    deps
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "extracts single import" — typst/extract.spec.ts line 21
    #[test]
    fn extracts_preview_import() {
        let content = r#"#import "@preview/cetz:0.2.0""#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].package_name, "cetz");
        assert_eq!(deps[0].namespace, "preview");
        assert_eq!(deps[0].current_value, "0.2.0");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "extracts single import" — typst/extract.spec.ts line 21
    #[test]
    fn extracts_import_with_trailing_colon_import() {
        let content = r#"#import "@preview/fletcher:0.4.5": diagram, node, edge"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].package_name, "fletcher");
        assert_eq!(deps[0].current_value, "0.4.5");
    }

    // Ported: "adds skipReason for non-preview namespaces" — typst/extract.spec.ts line 167
    #[test]
    fn local_namespace_skipped() {
        let content = r#"#import "@local/mylib:1.0.0""#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(TypstSkipReason::Local));
    }

    // Ported: "adds skipReason for non-preview namespaces" — typst/extract.spec.ts line 167
    #[test]
    fn unknown_namespace_skipped() {
        let content = r#"#import "@custom/mypkg:2.0.0""#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(TypstSkipReason::Unsupported));
    }

    // Ported: "strips JSON comments before parsing" — typst/extract.spec.ts line 98
    #[test]
    fn comment_line_skipped() {
        let content = r#"// #import "@preview/cetz:0.2.0""#;
        assert!(extract(content).is_empty());
    }

    // Ported: "extracts multiple imports" — typst/extract.spec.ts line 36
    #[test]
    fn multiple_imports() {
        let content = r#"
#import "@preview/cetz:0.2.0"
#import "@preview/fletcher:0.4.5"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].package_name, "cetz");
        assert_eq!(deps[1].package_name, "fletcher");
    }

    // Ported: "returns empty deps when no imports found" — typst/extract.spec.ts line 10
    #[test]
    fn no_imports_returns_empty() {
        assert!(extract("= Hello, World!\n\nThis is a Typst document.").is_empty());
    }

    // Ported: "ignores invalid import formats" — typst/extract.spec.ts line 147
    #[test]
    fn ignores_invalid_import_formats() {
        let content = "#import \"regular/path\": *\nimport \"@preview/pkg:1.0.0\": *\n#import @preview/pkg:1.0.0: *\n#import \"@preview/valid:1.0.0\": *\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].package_name, "valid");
        assert_eq!(deps[0].current_value, "1.0.0");
    }

    // Ported: "adds skipReason for non-preview namespaces" — typst/extract.spec.ts line 167
    #[test]
    fn non_preview_namespaces_get_skip_reasons() {
        let content = "#import \"@preview/valid:1.0.0\": *\n#import \"@local/local-pkg:2.0.0\": *\n#import \"@custom/custom-pkg:3.0.0\": *\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
        assert!(deps[0].skip_reason.is_none());
        assert_eq!(deps[1].skip_reason, Some(TypstSkipReason::Local));
        assert_eq!(deps[2].skip_reason, Some(TypstSkipReason::Unsupported));
    }
}
