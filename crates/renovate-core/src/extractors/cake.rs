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
//! ```text
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

/// Matches an `InstallTool(...)` or `InstallTools(...)` block (potentially multiline).
static INSTALL_TOOLS_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?s)InstallTools?\s*\([^)]+\)\s*;").unwrap()
});

/// Matches a `"dotnet:..."` string inside an InstallTools block.
static DOTNET_REF_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#""dotnet:([^"]+)""#).unwrap());

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

/// Parse a `dotnet:URL` or `nuget:URL` reference into a `CakeDep`.
fn parse_dep_url(url_str: &str) -> Option<CakeDep> {
    if url_str.starts_with("file:") {
        return None;
    }
    let pkg_cap = PACKAGE_RE.captures(url_str)?;
    let package_name = pkg_cap[1].to_owned();
    let current_value = VERSION_RE
        .captures(url_str)
        .map(|c| c[1].to_owned())
        .unwrap_or_default();
    let registry_url = REGISTRY_RE
        .captures(url_str)
        .map(|c| c[1].trim_end_matches('/').to_owned())
        .unwrap_or_default();
    Some(CakeDep { package_name, current_value, registry_url })
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

        if let Some(dep) = parse_dep_url(nuget_ref) {
            out.push(dep);
        }
    }

    // Also scan InstallTool/InstallTools(...) blocks for `dotnet:` references.
    for block_match in INSTALL_TOOLS_RE.find_iter(content) {
        let block = block_match.as_str();
        for dotnet_cap in DOTNET_REF_RE.captures_iter(block) {
            let url_str = &dotnet_cap[1];
            if let Some(dep) = parse_dep_url(url_str) {
                out.push(dep);
            }
        }
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

    // Ported: "returns null for empty" — lib/modules/manager/woodpecker/extract.spec.ts line 8
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "extracts" — modules/manager/cake/index.spec.ts line 21
    // Note: file:// references (Baz.Baz) are silently dropped in Rust (TS returns them with skipReason)
    #[test]
    fn extracts_build_cake_fixture() {
        let content = concat!(
            "foo\n",
            "#addin nuget:?package=Foo.Foo\n",
            "#addin \"nuget:?package=Bim.Bim&version=6.6.6\"\n",
            "#tool nuget:https://example.com?package=Bar.Bar&version=2.2.2\n",
            "#tool nuget:https://example.com/feed/v3/?package=Cake.Git&version=2.2.3\n",
            "#tool nuget:https://example.com/feed/v3/index.json?package=Cake.MinVer&version=2.2.4\n",
            "#module nuget:file:///tmp/?package=Baz.Baz&version=3.3.3\n",
            "#load nuget:?package=Cake.7zip&version=1.0.3\n",
            "#l nuget:?package=Cake.asciidoctorj&version=1.0.0\n",
            "// #module nuget:?package=Qux.Qux&version=4.4.4\n",
            "/*\n",
            "#module nuget:?package=Quux.Quux&version=5.5.5\n",
            "*/\n",
            "bar\n",
        );
        let deps = extract(content);
        let find = |name: &str| deps.iter().find(|d| d.package_name == name);

        // Foo.Foo — no version, no registry
        let foo = find("Foo.Foo").expect("Foo.Foo");
        assert!(foo.current_value.is_empty());
        assert!(foo.registry_url.is_empty());

        // Bim.Bim — version but no registry
        let bim = find("Bim.Bim").expect("Bim.Bim");
        assert_eq!(bim.current_value, "6.6.6");

        // Bar.Bar — custom registry
        let bar = find("Bar.Bar").expect("Bar.Bar");
        assert_eq!(bar.registry_url, "https://example.com");

        // Cake.Git — custom registry with v3 feed
        let git = find("Cake.Git").expect("Cake.Git");
        assert_eq!(git.registry_url, "https://example.com/feed/v3");

        // Cake.MinVer — custom registry with index.json
        let minver = find("Cake.MinVer").expect("Cake.MinVer");
        assert_eq!(
            minver.registry_url,
            "https://example.com/feed/v3/index.json"
        );

        // Baz.Baz — file:// → skipped in Rust (TypeScript returns with skipReason unsupported-url)
        assert!(find("Baz.Baz").is_none());

        // Cake.7zip — #load directive
        let zip = find("Cake.7zip").expect("Cake.7zip");
        assert_eq!(zip.current_value, "1.0.3");

        // Cake.asciidoctorj — #l shorthand
        let ascii = find("Cake.asciidoctorj").expect("Cake.asciidoctorj");
        assert_eq!(ascii.current_value, "1.0.0");

        // Commented-out and block-comment packages should be absent
        assert!(find("Qux.Qux").is_none());
        assert!(find("Quux.Quux").is_none());
    }

    // Ported: "extracts dotnet tools from single sdk style build file" — modules/manager/cake/index.spec.ts line 45
    #[test]
    fn extracts_install_tools_dotnet() {
        let content = concat!(
            "#:sdk Cake.Sdk\n",
            "\n",
            "// Install single tool\n",
            "InstallTool(\"dotnet:https://api.nuget.org/v3/index.json?package=SingleTool.Install.First&version=1.0.0\");\n",
            "InstallTool(\"dotnet:?package=SingleTool.Install.Second&version=1.2.0\");\n",
            "\n",
            "// Install multiple tools at once\n",
            "InstallTools(\n",
            "    \"dotnet:https://api.nuget.org/v3/index.json?package=MultipleTools.Install.First&version=2.0.0\",\n",
            "    \"dotnet:?package=MultipleTools.Install.Second&version=2.1.1\"\n",
            ");\n",
            "\n",
            "var installTools = \"dotnet:?Should.Not.Match&version=1.0.0\";\n",
        );
        let deps = extract(content);
        let find = |name: &str| deps.iter().find(|d| d.package_name == name);

        let first = find("SingleTool.Install.First").expect("SingleTool.Install.First");
        assert_eq!(first.current_value, "1.0.0");
        assert_eq!(first.registry_url, "https://api.nuget.org/v3/index.json");

        let second = find("SingleTool.Install.Second").expect("SingleTool.Install.Second");
        assert_eq!(second.current_value, "1.2.0");
        assert!(second.registry_url.is_empty());

        let multi_first = find("MultipleTools.Install.First").expect("MultipleTools.Install.First");
        assert_eq!(multi_first.current_value, "2.0.0");
        assert_eq!(multi_first.registry_url, "https://api.nuget.org/v3/index.json");

        let multi_second = find("MultipleTools.Install.Second").expect("MultipleTools.Install.Second");
        assert_eq!(multi_second.current_value, "2.1.1");

        // Variable assignment outside InstallTool() should not be extracted
        assert!(find("Should.Not.Match").is_none());
    }

    // Ported: "skips invalid entries in InstallTools" — modules/manager/cake/index.spec.ts line 101
    #[test]
    fn skips_invalid_install_tools_entries() {
        let content = concat!(
            "#:sdk Cake.Sdk\n",
            "\n",
            "// One invalid and one valid tool entry\n",
            "InstallTools(\n",
            "  \"dotnet:bad uri\",\n",
            "  \"dotnet:?package=Good.Tool&version=1.2.3\"\n",
            ");\n",
        );
        let deps = extract(content);
        let find = |name: &str| deps.iter().find(|d| d.package_name == name);

        let good = find("Good.Tool").expect("Good.Tool");
        assert_eq!(good.current_value, "1.2.3");
        // "bad uri" has no package= param, so it's skipped
        assert_eq!(deps.len(), 1);
    }
}
