//! Pattern matching utilities mirroring Renovate's `lib/util/string-match.ts`.
//!
//! Supports three pattern forms (same as Renovate):
//! - `/regex/` or `/regex/flags` — compiled and tested as a regex.
//! - Glob patterns (`*`, `?`, `**`, `{a,b}`, character classes) — matched via
//!   [`globset`].
//! - Bare strings — exact equality.
//!
//! `match_regex_or_glob_list` additionally supports Renovate's negation
//! semantics: patterns starting with `!` are negative exclusions.
//! The function returns `true` when:
//! 1. Every positive pattern that exists: at least one matches.
//! 2. Every negative pattern that exists: none of them match the input.
//!
//! Renovate reference: `lib/util/string-match.ts`.

use globset::Glob;
use regex::Regex;

/// Match `input` against a single pattern.
///
/// Pattern forms:
/// - `"*"` → always true
/// - `"/pattern/"` or `"/pattern/i"` → compiled regex
/// - Contains `*`, `?`, `[`, `{`, or `**` → glob
/// - Otherwise → exact equality (case-sensitive)
pub fn match_regex_or_glob(input: &str, pattern: &str) -> bool {
    if pattern == "*" {
        return true;
    }

    // Negated form: !expr  — invert the result of matching the inner expression.
    if let Some(inner) = pattern.strip_prefix('!') {
        return !match_regex_or_glob(input, inner);
    }

    // Inline regex: /pattern/ or /pattern/flags
    if let Some(regex_body) = extract_regex_pattern(pattern) {
        return Regex::new(&regex_body)
            .map(|re| re.is_match(input))
            .unwrap_or(false);
    }

    // Glob: contains special characters
    if is_glob_pattern(pattern) {
        return Glob::new(pattern)
            .map(|g| g.compile_matcher().is_match(input))
            .unwrap_or(false);
    }

    // Exact match
    input == pattern
}

/// Match `input` against a list of patterns with Renovate's positive/negative
/// semantics.
///
/// - Patterns NOT starting with `!` are positive; at least one must match if
///   any exist.
/// - Patterns starting with `!` are negative; none must match if any exist.
/// - An empty list returns `false` (no patterns → no match).
pub fn match_regex_or_glob_list(input: &str, patterns: &[String]) -> bool {
    if patterns.is_empty() {
        return false;
    }

    let mut has_positive = false;
    let mut positive_matched = false;

    for pattern in patterns {
        if let Some(pos_pat) = pattern.strip_prefix('!') {
            // Negative pattern: if it matches, exclude the input.
            if match_regex_or_glob(input, pos_pat) {
                return false;
            }
        } else {
            has_positive = true;
            if match_regex_or_glob(input, pattern) {
                positive_matched = true;
            }
        }
    }

    // If there were positive patterns, at least one must have matched.
    if has_positive {
        return positive_matched;
    }

    // All patterns were negative and none matched → include.
    true
}

/// Extract a Rust-compatible regex string from a `/pattern/` or `/pattern/flags` literal.
///
/// Supported flags: `i` (case-insensitive), `m` (multi-line), `s` (dot-all).
/// Unknown flags are ignored.  Returns `None` if `s` is not a regex literal.
fn extract_regex_pattern(s: &str) -> Option<String> {
    let inner = s.strip_prefix('/')?;
    let close = inner.rfind('/')?;
    let body = &inner[..close];
    let flags = &inner[close + 1..];

    if body.is_empty() {
        return None;
    }

    // Build a prefix of embedded flags recognised by the `regex` crate.
    let mut prefix = String::new();
    for ch in flags.chars() {
        match ch {
            'i' | 'm' | 's' | 'x' => {
                prefix.push('(');
                prefix.push('?');
                prefix.push(ch);
                prefix.push(')');
            }
            _ => {}
        }
    }

    Some(format!("{prefix}{body}"))
}

/// Return `true` if `pattern` contains any glob metacharacters.
fn is_glob_pattern(pattern: &str) -> bool {
    pattern.contains(['*', '?', '[', '{'])
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── match_regex_or_glob ───────────────────────────────────────────────────

    #[test]
    fn wildcard_star_always_matches() {
        assert!(match_regex_or_glob("anything", "*"));
    }

    #[test]
    fn exact_match() {
        assert!(match_regex_or_glob("npm", "npm"));
        assert!(!match_regex_or_glob("npm", "cargo"));
    }

    #[test]
    fn regex_pattern_match() {
        assert!(match_regex_or_glob("npm", "/^npm/"));
        assert!(match_regex_or_glob("npm-check", "/^npm/"));
        assert!(!match_regex_or_glob("cargo", "/^npm/"));
    }

    #[test]
    fn regex_pattern_with_flags_match() {
        assert!(match_regex_or_glob("NPM", "/^npm/i"));
    }

    #[test]
    fn glob_star_prefix() {
        assert!(match_regex_or_glob("npm", "n*"));
        assert!(match_regex_or_glob("npm-check", "npm*"));
        assert!(!match_regex_or_glob("cargo", "npm*"));
    }

    #[test]
    fn glob_double_star() {
        assert!(match_regex_or_glob("docker/nginx", "docker/**"));
    }

    // ── match_regex_or_glob_list ──────────────────────────────────────────────

    #[test]
    fn empty_list_returns_false() {
        assert!(!match_regex_or_glob_list("npm", &[]));
    }

    #[test]
    fn positive_list_matches() {
        let pats: Vec<String> = vec!["npm".into(), "cargo".into()];
        assert!(match_regex_or_glob_list("npm", &pats));
        assert!(match_regex_or_glob_list("cargo", &pats));
        assert!(!match_regex_or_glob_list("pip", &pats));
    }

    #[test]
    fn negation_excludes_input() {
        let pats: Vec<String> = vec!["npm".into(), "!cargo".into()];
        assert!(match_regex_or_glob_list("npm", &pats));
        assert!(!match_regex_or_glob_list("cargo", &pats));
    }

    #[test]
    fn all_negative_patterns_allow_non_matching() {
        // Only negative patterns: any input that doesn't match the negation passes.
        let pats: Vec<String> = vec!["!cargo".into()];
        assert!(match_regex_or_glob_list("npm", &pats));
        assert!(!match_regex_or_glob_list("cargo", &pats));
    }

    #[test]
    fn glob_in_list() {
        let pats: Vec<String> = vec!["npm*".into()];
        assert!(match_regex_or_glob_list("npm", &pats));
        assert!(match_regex_or_glob_list("npm-check", &pats));
        assert!(!match_regex_or_glob_list("cargo", &pats));
    }

    #[test]
    fn regex_in_list() {
        let pats: Vec<String> = vec!["/^(npm|pip)/".into()];
        assert!(match_regex_or_glob_list("npm", &pats));
        assert!(match_regex_or_glob_list("pip", &pats));
        assert!(!match_regex_or_glob_list("cargo", &pats));
    }

    #[test]
    fn negation_glob() {
        let pats: Vec<String> = vec!["!npm*".into()];
        assert!(match_regex_or_glob_list("cargo", &pats));
        assert!(!match_regex_or_glob_list("npm", &pats));
        assert!(!match_regex_or_glob_list("npm-check", &pats));
    }

    // ── match_regex_or_glob negation support ─────────────────────────────────

    #[test]
    fn single_negated_regex_inverts_match() {
        // !/^0/ should match strings NOT starting with "0"
        assert!(match_regex_or_glob("1.2.3", "!/^0/"));
        assert!(!match_regex_or_glob("0.5.0", "!/^0/"));
    }

    #[test]
    fn single_negated_exact_inverts_match() {
        assert!(match_regex_or_glob("cargo", "!npm"));
        assert!(!match_regex_or_glob("npm", "!npm"));
    }

    #[test]
    fn single_negated_glob_inverts_match() {
        assert!(match_regex_or_glob("cargo", "!npm*"));
        assert!(!match_regex_or_glob("npm-check", "!npm*"));
    }

    #[test]
    fn brace_expansion_in_glob() {
        // globset supports brace expansion: {/,} matches '/' or ','
        assert!(match_regex_or_glob(
            "@opentelemetry/http",
            "@opentelemetry{/,}**"
        ));
        assert!(!match_regex_or_glob(
            "@opentelemetry-http",
            "@opentelemetry{/,}**"
        ));
    }
}
