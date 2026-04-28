//! Cake build script (`.cake` / `.csx`) dependency extractor.
//!
//! Parses `#addin`, `#tool`, `#module`, `#load`, and `#l` directives that
//! reference NuGet packages in the `nuget:?package=Name&version=X` format.
//!
//! Renovate reference:
//! - `lib/modules/manager/cake/index.ts`
//! - Pattern: `/\.cake$/`
//! - Datasource: NuGet
//!
//! ## Supported forms
//!
//! ```
//! #addin nuget:?package=Foo.Bar&version=1.2.3
//! #tool nuget:https://api.nuget.org/v3/index.json?package=Foo.Bar&version=1.2.3
//! #load nuget:?package=Foo.Bar&version=1.0.0
//! ```
//!
//! ## Skip reasons
//!
//! - `nuget:file:///...` — local file path registry
//! - No `package=` query parameter present
//! - Inside `//` or `/* */` comments

use std::sync::LazyLock;

use regex::Regex;

/// A single extracted Cake NuGet dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CakeDep {
    /// NuGet package name (e.g. `"Cake.Git"`).
    pub package_name: String,
    /// Version string, if specified (e.g. `"2.2.3"`). Empty when omitted.
    pub current_value: String,
    /// Registry URL, if non-default. Empty means use NuGet default.
    pub registry_url: String,
}

// ── Compiled regexes ─────────────────────────────────────────────────────────

/// Matches `#addin`, `#tool`, `#module`, `#load`, `#l` followed by `nuget:...`.
static DIRECTIVE_LINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^\s*#(?:addin|tool|module|load|l)\s+"?nuget:([^"]*)"?"#).unwrap()
});

/// Extracts `package=Name` from a query string.
static PACKAGE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[?&]package=([^&\s]+)").unwrap());

/// Extracts `version=X.Y.Z` from a query string.
static VERSION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[?&]version=([^&\s]+)").unwrap());

/// Extracts the registry URL (the part before the `?`).
/// `nuget:https://example.com?package=Foo` → `https://example.com`.
/// `nuget:?package=Foo` → `""` (empty, use default).
static REGISTRY_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(https?://[^?]+)\?").unwrap());

/// Find the start of a `//` line comment, ignoring `://` (URL scheme separators).
fn find_comment_start(line: &str) -> Option<usize> {
    let bytes = line.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b'/' && bytes[i + 1] == b'/' {
            // Not a URL — `://` would have `:` before the first `/`.
            if i == 0 || bytes[i - 1] != b':' {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

/// Extract Cake NuGet deps from a `.cake` or `.csx` file.
pub fn extract(content: &str) -> Vec<CakeDep> {
    let mut out = Vec::new();
    let mut in_block_comment = false;

    for raw in content.lines() {
        // Track `/* */` block comments.
        if in_block_comment {
            if raw.contains("*/") {
                in_block_comment = false;
            }
            continue;
        }
        if raw.contains("/*") {
            in_block_comment = true;
            continue;
        }

        // Strip `//` line comments — but only if `//` appears before any directive.
        // Split on whitespace-only `//` patterns to avoid breaking URLs like `https://`.
        let line = if let Some(idx) = find_comment_start(raw) {
            &raw[..idx]
        } else {
            raw
        };

        let Some(cap) = DIRECTIVE_LINE.captures(line) else {
            continue;
        };
        let nuget_ref = &cap[1]; // everything after `nuget:`

        // Skip file:// local references.
        if nuget_ref.starts_with("file:") {
            continue;
        }

        let Some(pkg_cap) = PACKAGE_RE.captures(nuget_ref) else {
            continue;
        };
        let package_name = pkg_cap[1].to_owned();

        let current_value = VERSION_RE
            .captures(nuget_ref)
            .map(|c| c[1].to_owned())
            .unwrap_or_default();

        let registry_url = REGISTRY_RE
            .captures(nuget_ref)
            .map(|c| c[1].trim_end_matches('/').to_owned())
            .unwrap_or_default();

        out.push(CakeDep {
            package_name,
            current_value,
            registry_url,
        });
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
foo
#addin nuget:?package=Foo.Foo
#addin "nuget:?package=Bim.Bim&version=6.6.6"
#tool nuget:https://example.com?package=Bar.Bar&version=2.2.2
#module nuget:file:///tmp/?package=Baz.Baz&version=3.3.3
#load nuget:?package=Cake.7zip&version=1.0.3
// #module nuget:?package=Qux.Qux&version=4.4.4
/*
#module nuget:?package=Quux.Quux&version=5.5.5
*/
bar
"#;

    #[test]
    fn extracts_package_with_version() {
        let deps = extract(SAMPLE);
        let bim = deps.iter().find(|d| d.package_name == "Bim.Bim").unwrap();
        assert_eq!(bim.current_value, "6.6.6");
        assert_eq!(bim.registry_url, "");
    }

    #[test]
    fn extracts_package_without_version() {
        let deps = extract(SAMPLE);
        let foo = deps.iter().find(|d| d.package_name == "Foo.Foo").unwrap();
        assert_eq!(foo.current_value, "");
    }

    #[test]
    fn extracts_custom_registry() {
        let deps = extract(SAMPLE);
        let bar = deps.iter().find(|d| d.package_name == "Bar.Bar").unwrap();
        assert_eq!(bar.registry_url, "https://example.com");
    }

    #[test]
    fn skips_local_file_registry() {
        let deps = extract(SAMPLE);
        assert!(!deps.iter().any(|d| d.package_name == "Baz.Baz"));
    }

    #[test]
    fn skips_line_comment() {
        let deps = extract(SAMPLE);
        assert!(!deps.iter().any(|d| d.package_name == "Qux.Qux"));
    }

    #[test]
    fn skips_block_comment() {
        let deps = extract(SAMPLE);
        assert!(!deps.iter().any(|d| d.package_name == "Quux.Quux"));
    }

    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }
}
