//! Clojure `deps.edn` dependency extractor.
//!
//! Parses the `:mvn/version` map entries from Clojure Tools Deps (`deps.edn`)
//! and Babashka (`bb.edn`) files.
//!
//! Renovate reference:
//! - `lib/modules/manager/deps-edn/extract.ts`
//! - Pattern: `/(^|/)(?:deps|bb)\.edn$/`
//! - Datasource: Clojure (Maven Central + Clojars)
//!
//! ## Supported forms
//!
//! ```edn
//! {:deps {org.clojure/clojure {:mvn/version "1.11.1"}
//!         ring {:mvn/version "1.9.6"}}}
//! ```
//!
//! ## Skip reasons
//!
//! - `:git/url` deps — uses git rev, not a Maven version
//! - `:local/root` deps — local path
//! - `:deps/root` deps — local root

use std::sync::LazyLock;

use regex::Regex;

/// A single extracted deps.edn Maven dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DepsEdnDep {
    /// Maven `group:artifact` coordinates (e.g. `"org.clojure:clojure"`).
    pub dep_name: String,
    /// Version string (e.g. `"1.11.1"`).
    pub current_value: String,
}

// ── Compiled regex ────────────────────────────────────────────────────────────

/// Matches `:mvn/version "x.y.z"`.
static MVN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#":mvn/version\s+"([^"]+)""#).unwrap());

/// Expand `group/artifact` or bare `group` to `group:artifact`.
fn expand_name(s: &str) -> String {
    if let Some((g, a)) = s.split_once('/') {
        format!("{g}:{a}")
    } else {
        format!("{s}:{s}")
    }
}

/// Extract Maven deps from a `deps.edn` or `bb.edn` file.
///
/// Strategy: scan line by line. For each line that contains `:mvn/version`,
/// look backwards (same line or previous tracked line) for the most recent
/// EDN dep symbol followed by ` {`.
pub fn extract(content: &str) -> Vec<DepsEdnDep> {
    let mut out = Vec::new();
    let mut pending: Option<String> = None; // dep name waiting for version

    for raw in content.lines() {
        // Strip line comments.
        let line = {
            let mut in_str = false;
            let mut end = raw.len();
            let mut prev = ' ';
            for (i, ch) in raw.char_indices() {
                if ch == '"' && prev != '\\' {
                    in_str = !in_str;
                }
                if ch == ';' && !in_str {
                    end = i;
                    break;
                }
                prev = ch;
            }
            &raw[..end]
        };

        // Skip git/local deps — they may span multiple lines so reset pending.
        if line.contains(":git/") || line.contains(":local/") || line.contains(":deps/root") {
            pending = None;
            continue;
        }

        let has_mvn = MVN.is_match(line);

        // Find the last `dep-symbol {` pattern on this line.
        // We look for the rightmost `sym {` where `sym` is a valid dep symbol
        // (starts with a letter, contains alphanumeric/dot/hyphen/slash).
        let dep_on_line = find_last_dep_sym(line);

        if has_mvn {
            let version = MVN.captures(line).map(|c| c[1].to_owned());
            // Use dep on this same line if present; otherwise use pending from previous line.
            let name = dep_on_line.or_else(|| pending.take());
            if let (Some(n), Some(v)) = (name, version) {
                out.push(DepsEdnDep {
                    dep_name: expand_name(&n),
                    current_value: v,
                });
            }
            pending = None;
        } else if let Some(sym) = dep_on_line {
            // This line has a dep symbol but no :mvn/version — version is on next line.
            pending = Some(sym);
        } else {
            // No dep symbol on this line — keep pending only if it's from previous line
            // (don't clear it mid-block).
        }
    }
    out
}

/// Find the last `dep-symbol {` occurrence on a line, returning the symbol.
///
/// Returns `None` if no valid dep-symbol-space-brace pattern is found.
/// Filters out EDN keywords (`:deps`, `:aliases`, etc.).
fn find_last_dep_sym(line: &str) -> Option<String> {
    let mut last: Option<String> = None;
    // Scan for every symbol followed by whitespace and `{`.
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];
        // Start of a potential symbol: letter.
        if ch.is_ascii_alphabetic() {
            let start = i;
            // Consume the symbol: letters, digits, `.`, `-`, `_`, `/`.
            while i < chars.len()
                && (chars[i].is_alphanumeric()
                    || chars[i] == '.'
                    || chars[i] == '-'
                    || chars[i] == '_'
                    || chars[i] == '/')
            {
                i += 1;
            }
            let sym: String = chars[start..i].iter().collect();
            // Skip if the symbol is a keyword word (like `mvn`, `git`, `local`).
            // We allow anything that is a valid Clojure dep symbol.
            // Next must be whitespace then `{`.
            let rest: String = chars[i..].iter().collect();
            if rest.trim_start().starts_with('{') {
                last = Some(sym);
            }
        } else {
            i += 1;
        }
    }
    last
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
{:deps {org.clojure/clojure {:mvn/version "1.11.1"}
        ring/ring-core {:mvn/version "1.9.6"}
        compojure {:mvn/version "1.6.3"}
        nrepl/nrepl {:git/url "https://github.com/nrepl/nrepl"
                     :git/sha "abc123"}
        local-lib {:local/root "../local-lib"}}

 :aliases
 {:dev {:extra-deps {ring/ring-mock {:mvn/version "0.4.0"}}}}}
"#;

    #[test]
    fn extracts_deps() {
        let deps = extract(SAMPLE);
        let clojure = deps
            .iter()
            .find(|d| d.dep_name == "org.clojure:clojure")
            .unwrap();
        assert_eq!(clojure.current_value, "1.11.1");

        let ring = deps
            .iter()
            .find(|d| d.dep_name == "ring:ring-core")
            .unwrap();
        assert_eq!(ring.current_value, "1.9.6");

        // Bare `compojure` → `compojure:compojure`
        let compojure = deps
            .iter()
            .find(|d| d.dep_name == "compojure:compojure")
            .unwrap();
        assert_eq!(compojure.current_value, "1.6.3");
    }

    #[test]
    fn skips_git_deps() {
        let deps = extract(SAMPLE);
        assert!(!deps.iter().any(|d| d.dep_name == "nrepl:nrepl"));
    }

    #[test]
    fn skips_local_deps() {
        let deps = extract(SAMPLE);
        assert!(!deps.iter().any(|d| d.dep_name == "local-lib:local-lib"));
    }

    #[test]
    fn extracts_alias_deps() {
        let deps = extract(SAMPLE);
        let mock = deps.iter().find(|d| d.dep_name == "ring:ring-mock");
        assert!(mock.is_some());
        assert_eq!(mock.unwrap().current_value, "0.4.0");
    }

    #[test]
    fn empty_returns_empty() {
        assert!(extract("{}").is_empty());
    }
}
