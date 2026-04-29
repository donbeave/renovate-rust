//! Pattern matching utilities mirroring Renovate's `lib/util/string-match.ts`.
//!
//! Supports three pattern forms (same as Renovate):
//! - `/regex/` or `/regex/flags` — compiled and tested as a regex.
//! - Glob patterns (`*`, `?`, `**`, `{a,b}`, character classes) — matched via
//!   [`globset`] with case-insensitive mode (matching Renovate's minimatch `nocase:true`).
//! - Bare strings — case-insensitive exact match, matching minimatch `nocase:true` behavior.
//!
//! `match_regex_or_glob_list` additionally supports Renovate's negation
//! semantics: patterns starting with `!` are negative exclusions.
//! The function returns `true` when:
//! 1. Every positive pattern that exists: at least one matches.
//! 2. Every negative pattern that exists: none of them match the input.
//!
//! Renovate reference: `lib/util/string-match.ts`.

use globset::GlobBuilder;
use regex::Regex;

/// Match `input` against a single pattern.
///
/// Pattern forms:
/// - `"*"` → always true
/// - `"/pattern/"` or `"/pattern/i"` → compiled regex (case depends on flags)
/// - Contains `*`, `?`, `[`, `{`, or `**` → case-insensitive glob
/// - Otherwise → case-insensitive exact match (mirrors minimatch `nocase:true`)
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

    // Glob: contains special characters.
    // - case_insensitive: matches Renovate's minimatch {nocase: true}
    // - literal_separator: makes bare `*` not cross `/`, so `**` must be a full
    //   path component to span directories — same as minimatch path-boundary rules
    if is_glob_pattern(pattern) {
        return GlobBuilder::new(pattern)
            .case_insensitive(true)
            .literal_separator(true)
            .build()
            .map(|g| g.compile_matcher().is_match(input))
            .unwrap_or(false);
    }

    // Exact match — case-insensitive to mirror minimatch's nocase:true behavior.
    // Renovate routes ALL non-regex patterns through minimatch({nocase: true}),
    // so bare-string patterns like "https://github.com/Foo/bar" match
    // "https://github.com/foo/Bar" (verified in index.spec.ts).
    input.eq_ignore_ascii_case(pattern)
}

/// Return `true` if any element of `inputs` matches at least one of `patterns`
/// using [`match_regex_or_glob_list`] semantics.
///
/// Mirrors Renovate's `anyMatchRegexOrGlobList`.
pub fn any_match_regex_or_glob_list(inputs: &[&str], patterns: &[String]) -> bool {
    inputs
        .iter()
        .any(|input| match_regex_or_glob_list(input, patterns))
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

/// Return `true` when an inline comment should cause the dep to be skipped.
///
/// Mirrors Renovate's `isSkipComment()` from `lib/util/ignore.ts`.
/// Returns `true` when `comment` starts with `renovate:` or `pyup:` and the
/// command after the colon is `"ignore"`.  Case-sensitive (matches Renovate's
/// `regEx(/^(renovate|pyup):/)` + `command === 'ignore'` check).
///
/// Renovate reference: `lib/util/ignore.ts`
pub fn is_skip_comment(comment: &str) -> bool {
    let comment = comment.trim();
    let body = if let Some(rest) = comment.strip_prefix("renovate:") {
        rest
    } else if let Some(rest) = comment.strip_prefix("pyup:") {
        rest
    } else {
        return false;
    };
    // The command ends at the first '#' (nested comment) or at whitespace.
    let cmd = body.split('#').next().unwrap_or(body).trim();
    cmd == "ignore"
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
    fn exact_match_is_case_insensitive() {
        // Mirrors Renovate's minimatch({nocase:true}) — even bare string patterns are case-insensitive.
        // Verified in Renovate index.spec.ts: "matches matchSourceUrls(case-insensitive)"
        assert!(match_regex_or_glob(
            "https://github.com/renovatebot/Renovate",
            "https://github.com/Renovatebot/renovate"
        ));
        // Package names: npm != NPM in strict sense but our impl follows Renovate's minimatch.
        assert!(match_regex_or_glob("NPM", "npm"));
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

    // ── Ported from Renovate dep-names.spec.ts ────────────────────────────────
    // These verify our glob/regex semantics match Renovate's behavior.

    #[test]
    fn dep_names_exact_match() {
        // Exact string match
        let pats: Vec<String> = vec!["@opentelemetry/http".into()];
        assert!(match_regex_or_glob_list("@opentelemetry/http", &pats));
        assert!(!match_regex_or_glob_list("@opentelemetry/trace", &pats));
    }

    #[test]
    fn dep_names_regex_prefix() {
        // Regex: /^@opentelemetry/ should match @opentelemetry/<anything>
        let pats: Vec<String> = vec!["/^@opentelemetry/".into()];
        assert!(match_regex_or_glob_list("@opentelemetry/http", &pats));
        assert!(match_regex_or_glob_list("@opentelemetry/trace", &pats));
        assert!(!match_regex_or_glob_list("@other/http", &pats));
    }

    #[test]
    fn dep_names_negated_regex_prefix() {
        // Negated regex: !/^@opentelemetry/ should exclude @opentelemetry scope
        let pats: Vec<String> = vec!["!/^@opentelemetry/".into()];
        assert!(!match_regex_or_glob_list("@opentelemetry/http", &pats));
        assert!(match_regex_or_glob_list("@other/http", &pats));
    }

    #[test]
    fn dep_names_scoped_package_glob() {
        // @typescript-eslint/** should match all packages under the scope
        let pats: Vec<String> = vec!["@typescript-eslint/**".into()];
        assert!(match_regex_or_glob_list(
            "@typescript-eslint/eslint-plugin",
            &pats
        ));
        assert!(match_regex_or_glob_list("@typescript-eslint/parser", &pats));
        assert!(!match_regex_or_glob_list("typescript-eslint", &pats));
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

    // ── Ported from Renovate string-match.spec.ts ─────────────────────────────

    #[test]
    fn glob_is_case_insensitive_matching_renovate_nocase() {
        // Renovate uses minimatch({nocase: true}) for all glob matching.
        // "TEST" must match "t*".
        let pats: Vec<String> = vec!["t*".into()];
        assert!(match_regex_or_glob_list("TEST", &pats));
    }

    #[test]
    fn all_negative_patterns_both_must_not_match() {
        // returns false if not matching every negative pattern (regex)
        let pats: Vec<String> = vec!["!/test3/".into(), "!/test/".into()];
        assert!(!match_regex_or_glob_list("test", &pats));
    }

    #[test]
    fn all_negative_patterns_both_must_not_match_glob() {
        // returns false if not matching every negative pattern (glob)
        let pats: Vec<String> = vec!["!test3".into(), "!te*".into()];
        assert!(!match_regex_or_glob_list("test", &pats));
    }

    #[test]
    fn negative_regex_positive_pattern_returns_true() {
        // returns true if matching positive and negative patterns
        let pats: Vec<String> = vec!["test".into(), "!/test3/".into()];
        assert!(match_regex_or_glob_list("test", &pats));
    }

    #[test]
    fn negative_glob_positive_pattern_returns_true() {
        // returns true if matching every negative pattern (glob)
        let pats: Vec<String> = vec!["test".into(), "!test3".into(), "!test4".into()];
        assert!(match_regex_or_glob_list("test", &pats));
    }

    // ── any_match_regex_or_glob_list ──────────────────────────────────────────

    #[test]
    fn any_match_empty_patterns_returns_false() {
        assert!(!any_match_regex_or_glob_list(&["test"], &[]));
    }

    #[test]
    fn any_match_empty_inputs_returns_false() {
        let pats: Vec<String> = vec!["/test2/".into()];
        assert!(!any_match_regex_or_glob_list(&[], &pats));
    }

    #[test]
    fn any_match_positive_list_matches() {
        let pats: Vec<String> = vec!["b".into()];
        assert!(any_match_regex_or_glob_list(&["a", "b"], &pats));
    }

    #[test]
    fn any_match_negative_list_matches_non_excluded() {
        // any_match with negative pattern: if any input passes the negative filter, returns true
        let pats: Vec<String> = vec!["!b".into()];
        assert!(any_match_regex_or_glob_list(&["a", "b"], &pats));
    }

    // ── Ported from Renovate dep-names.spec.ts (additional) ──────────────────

    #[test]
    fn dep_names_no_slash_double_star_does_not_cross_slash() {
        // "@opentelemetry**" (without path separator) should NOT match "@opentelemetry/http"
        // Renovate minimatch: ** without path boundary doesn't cross "/"
        // globset: same behavior — ** must be a whole path segment to be path-spanning
        assert!(!match_regex_or_glob(
            "@opentelemetry/http",
            "@opentelemetry**"
        ));
    }

    // ── is_skip_comment (ported from Renovate util/ignore.spec.ts) ────────────

    #[test]
    fn skip_comment_renovate_ignore_returns_true() {
        // Ported: "returns true for 'renovate:ignore' comments" — util/ignore.spec.ts
        assert!(is_skip_comment("renovate:ignore"));
    }

    #[test]
    fn skip_comment_pyup_ignore_returns_true() {
        assert!(is_skip_comment("pyup:ignore"));
    }

    #[test]
    fn skip_comment_other_prefix_returns_false() {
        // Ported: "returns false for comments not starting with 'renovate:' or 'pyup:'"
        assert!(!is_skip_comment("other:ignore"));
    }

    #[test]
    fn skip_comment_renovate_non_ignore_returns_false() {
        // Ported: "returns false for 'renovate:' comments without 'ignore'"
        assert!(!is_skip_comment("renovate:update"));
    }

    #[test]
    fn skip_comment_empty_returns_false() {
        // Ported: "returns false when comment is undefined"
        assert!(!is_skip_comment(""));
    }

    #[test]
    fn skip_comment_with_leading_whitespace() {
        // Inline comment: "  renovate:ignore  " (with spaces)
        assert!(is_skip_comment("  renovate:ignore  "));
    }
}
