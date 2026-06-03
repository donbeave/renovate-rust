//! Hex versioning (Elixir/Erlang hex.pm).
//!
//! Renovate reference:
//! - `lib/modules/versioning/hex/index.ts`

use std::sync::LazyLock;

use regex::Regex;
use semver::{Version, VersionReq};

// ── Regex constants ───────────────────────────────────────────────────────────

// ~> X.Y at end of string (2-part)
static TWO_PART_HEX_AT_END: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"~>\s*(\d+\.\d+)$").unwrap());
// ~> X.Y.Z anywhere (3-part)
static THREE_PART_HEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"~>\s*(\d+\.\d+\.\d+)").unwrap());
// != X[.Y[.Z...]]
static NEQ_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"!=\s*(\d+\.\d+(?:\.\d+.*)?)").unwrap());
// First occurrence of == or and (no g flag equiv — use replacen)
// For ~> transforms in getNewValue output
static THREE_PART_TILDE_CARET: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[~^]\s*(\d+\.\d+\.\d+)").unwrap());
static TWO_PART_CARET: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\^\s*(\d+\.\d+)(?:\.\d+)?").unwrap());
static TILDE_THREE_BARE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"~\s*(\d+\.\d+\.\d)").unwrap());
// Match ~> X.Y$ in currentValue
static CV_TWO_PART: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"~>\s*\d+\.\d+$").unwrap());
// Match ~> X.Y.Z$ in currentValue
static CV_THREE_PART: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"~>\s*\d+\.\d+\.\d+$").unwrap());
// Lower bound extractor (>= or > followed by semver)
static LB_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r">=?\s*(\d+\.\d+\.\d+(?:-\S+)?)").unwrap());

// ── Conversion helpers ────────────────────────────────────────────────────────

fn hex2npm(input: &str) -> String {
    // ~> X.Y (2-part, at end) → ^X.Y
    let s = TWO_PART_HEX_AT_END.replace(input, "^$1").into_owned();
    // ~> X.Y.Z (3-part, anywhere) → ~X.Y.Z
    let s = THREE_PART_HEX.replace(&s, "~$1").into_owned();
    // Remove first occurrence of == (bare version; treated as exact) or and
    let s = if let Some(idx) = find_first_eq_or_and(&s) {
        let end = idx + if s[idx..].starts_with("==") { 2 } else { 3 };
        format!("{}{}", &s[..idx], &s[end..])
    } else {
        s
    };
    // or → || (first occurrence only)
    let s = s.replacen(" or ", " || ", 1);
    // != X.Y.Z → >X.Y.Z <X.Y.Z
    let s = NEQ_RE.replace(&s, ">$1 <$1").into_owned();
    s.trim().to_owned()
}

fn find_first_eq_or_and(s: &str) -> Option<usize> {
    static EQAND: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"==|and").unwrap());
    EQAND.find(s).map(|m| m.start())
}

fn npm2hex(input: &str) -> String {
    const OPERATORS: &[&str] = &["^", "=", ">", "<", "<=", ">=", "~>"];
    let tokens: Vec<&str> = input.split_whitespace().filter(|s| !s.is_empty()).collect();
    let mut output = String::new();
    let mut i = 0;
    while i < tokens.len() {
        if i == tokens.len() - 1 {
            output.push_str(tokens[i]);
            break;
        }
        if tokens[i + 1].contains("||") {
            output.push_str(tokens[i]);
            output.push_str(" or ");
            i += 1; // skip to "||"; outer i += 1 then moves past it
        } else if OPERATORS.contains(&tokens[i]) {
            output.push_str(tokens[i]);
            output.push(' ');
        } else {
            output.push_str(tokens[i]);
            output.push_str(" and ");
        }
        i += 1;
    }
    output
}

// ── Semver matching helpers ───────────────────────────────────────────────────

fn semver_satisfies_range(version: &str, range: &str) -> bool {
    let Ok(v) = Version::parse(version) else {
        return false;
    };
    semver_matches_v(&v, range)
}

fn semver_matches_v(v: &Version, range: &str) -> bool {
    // Convert to comma-separated semver form (Rust crate requirement).
    let normalized = normalize_hex_range(range);
    // Handle || (OR): at least one alternative must match.
    if normalized.contains("||") {
        return normalized.split("||").any(|alt| {
            let alt = alt.trim();
            // A bare version string (from stripped ==) is an exact match.
            if let Ok(exact) = Version::parse(alt) {
                return v == &exact;
            }
            VersionReq::parse(alt).is_ok_and(|req| req.matches(v))
        });
    }
    // A bare version string is an exact match.
    if let Ok(exact) = Version::parse(&normalized) {
        return v == &exact;
    }
    VersionReq::parse(&normalized).is_ok_and(|req| req.matches(v))
}

/// Normalize hex range string: replace ` or ` with ` || ` and ` and ` with `, `.
/// Also converts space-separated compound constraints to comma-separated form
/// so the semver crate can parse them as multiple requirements.
fn normalize_hex_range(range: &str) -> String {
    range
        .replace(" or ", " || ")
        .replace(" and ", ", ")
        .split("||")
        .map(|part| {
            // Within each OR-alternative, normalize space-separated constraints
            // to comma-separated (semver crate requirement).
            normalize_and_part(part.trim())
        })
        .collect::<Vec<_>>()
        .join(" || ")
}

/// Convert a hex AND-range part to semver comma-separated form.
/// e.g. ">= 1.0.0 < 2.0.0" → ">= 1.0.0, < 2.0.0"
fn normalize_and_part(part: &str) -> String {
    // Already comma-separated (from `and` replacement) — normalize whitespace
    let comma_tokens: Vec<&str> = part
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();
    if comma_tokens.len() > 1 {
        return comma_tokens.join(", ");
    }
    // Split space-separated constraints into comma-separated form.
    // Each constraint starts with an operator character.
    let words: Vec<&str> = part.split_whitespace().collect();
    let mut constraints: Vec<String> = Vec::new();
    let mut current: Option<String> = None;
    for word in &words {
        let is_op_start = word.starts_with(['>', '<', '=', '!', '^', '~']);
        if is_op_start {
            if let Some(c) = current.take() {
                constraints.push(c);
            }
            current = Some((*word).to_owned());
        } else if let Some(ref mut c) = current {
            c.push(' ');
            c.push_str(word);
        } else {
            current = Some((*word).to_owned());
        }
    }
    if let Some(c) = current {
        constraints.push(c);
    }
    if constraints.len() > 1 {
        return constraints.join(", ");
    }
    part.to_owned()
}

// ── Operator parsing ──────────────────────────────────────────────────────────

fn constraint_operator(c: &str) -> &str {
    if c.starts_with("~>") {
        "~>"
    } else if c.starts_with(">=") {
        ">="
    } else if c.starts_with("<=") {
        "<="
    } else if c.starts_with("!=") {
        "!="
    } else if c.starts_with('>') {
        ">"
    } else if c.starts_with('<') {
        "<"
    } else if c.starts_with('^') {
        "^"
    } else if c.starts_with('~') {
        "~"
    } else if c.starts_with('=') {
        "="
    } else {
        ""
    }
}

const PURE_OPS: &[&str] = &["~>", ">=", "<=", "!=", ">", "<", "^", "~", "="];

fn is_pure_op(token: &str) -> bool {
    PURE_OPS.contains(&token)
}

fn starts_with_op(token: &str) -> bool {
    PURE_OPS
        .iter()
        .any(|op| token.starts_with(op) && token != *op)
}

/// Split a compound AND range into individual constraint strings.
fn split_constraints(range: &str) -> Vec<String> {
    let tokens: Vec<&str> = range.split_whitespace().collect();
    let mut constraints: Vec<String> = Vec::new();
    let mut current: Option<String> = None;

    for token in tokens {
        if is_pure_op(token) {
            if let Some(c) = current.take() {
                constraints.push(c);
            }
            current = Some(token.to_owned());
        } else if starts_with_op(token) {
            if let Some(c) = current.take() {
                constraints.push(c);
            }
            constraints.push(token.to_owned());
        } else if let Some(ref mut c) = current {
            c.push(' ');
            c.push_str(token);
        } else {
            current = Some(token.to_owned());
        }
    }
    if let Some(c) = current {
        constraints.push(c);
    }
    constraints
}

fn extract_min_lower_bound(range: &str) -> Option<Version> {
    LB_RE
        .captures_iter(range)
        .filter_map(|cap| Version::parse(&cap[1]).ok())
        .min()
}

// ── replaceCaretValue ─────────────────────────────────────────────────────────

fn replace_caret_value(old: &Version, new: &Version) -> String {
    let old_t = [old.major, old.minor, old.patch];
    let new_t = [new.major, new.minor, new.patch];
    let mut result = [0u64; 3];
    let mut leading_zero = true;
    let mut need_replace = false;

    for idx in 0..3 {
        let old_v = old_t[idx];
        let new_v = new_t[idx];

        let leading_digit = (old_v != 0 || new_v != 0) && std::mem::take(&mut leading_zero);

        if leading_digit && new_v > old_v {
            need_replace = true;
        }
        if !need_replace && new_v < old_v {
            return format!("{}.{}.{}", new.major, new.minor, new.patch);
        }
        result[idx] = if leading_digit { new_v } else { 0 };
    }

    if need_replace {
        format!("{}.{}.{}", result[0], result[1], result[2])
    } else {
        format!("{}.{}.{}", old.major, old.minor, old.patch)
    }
}

// ── npm getNewValue logic ─────────────────────────────────────────────────────

fn npm_get_new_value(
    current_value: &str,
    range_strategy: &str,
    current_version: &str,
    new_version: &str,
) -> Option<String> {
    let new_ver = Version::parse(new_version).ok()?;

    // Plain exact version
    if Version::parse(current_value).is_ok() {
        return Some(new_version.to_owned());
    }

    // update-lockfile: if satisfied, keep; else fall back to replace
    if range_strategy == "update-lockfile" {
        if semver_satisfies_range(new_version, current_value) {
            return Some(current_value.to_owned());
        }
        return npm_get_new_value(current_value, "replace", current_version, new_version);
    }

    // OR range (||)
    if current_value.contains("||") {
        if range_strategy == "widen" {
            if semver_satisfies_range(new_version, current_value) {
                return Some(current_value.to_owned());
            }
            let replace_val =
                npm_get_new_value(current_value, "replace", current_version, new_version)?;
            return Some(format!("{current_value} || {replace_val}"));
        }
        // replace/bump: last || alternative
        let last_alt = current_value.rsplit("||").next()?.trim();
        return npm_get_new_value(last_alt, range_strategy, current_version, new_version);
    }

    // Compound AND range
    let constraints = split_constraints(current_value);
    if constraints.len() > 1 {
        let last = constraints.last()?;
        if range_strategy == "widen" {
            if semver_satisfies_range(new_version, current_value) {
                return Some(current_value.to_owned());
            }
            let replace_val = npm_get_new_value(last, "replace", current_version, new_version)?;
            let last_op = constraint_operator(last);
            if last_op.starts_with('<') {
                // Replace everything from the last operator occurrence onwards
                if let Some(idx) = current_value.rfind(last_op) {
                    let prefix = &current_value[..idx];
                    return Some(format!("{prefix}{replace_val}"));
                }
            }
            return Some(format!("{current_value} || {replace_val}"));
        }
        // replace/bump: recurse on last constraint only
        return npm_get_new_value(last, "replace", current_version, new_version);
    }

    // Widen strategy for single constraint: if new version is outside the current
    // range, append a new alternative rather than replacing the existing range.
    if range_strategy == "widen" && !semver_satisfies_range(new_version, current_value) {
        let replace_val =
            npm_get_new_value(current_value, "replace", current_version, new_version)?;
        return Some(format!("{current_value} || {replace_val}"));
    }

    // Single constraint — determine operator
    let cv = current_value.trim();
    let has_space = cv.contains("  ") || {
        // space between op and version
        let op = constraint_operator(cv);
        !op.is_empty() && cv[op.len()..].starts_with(' ')
    };

    if cv.starts_with("~>") {
        let _rest = cv.trim_start_matches("~>").trim();
        if range_strategy == "bump" {
            return Some(format!("~> {new_version}"));
        }
        return Some(format!("~> {}.{}.0", new_ver.major, new_ver.minor));
    }

    if cv.starts_with('~') {
        if range_strategy == "bump" {
            return Some(format!("~{new_version}"));
        }
        return Some(format!("~{}.{}.0", new_ver.major, new_ver.minor));
    }

    if cv.starts_with('^') {
        if range_strategy == "bump" {
            return Some(format!("^{new_version}"));
        }
        let cur_ver = Version::parse(current_version).unwrap_or(Version::new(0, 0, 0));
        let res = replace_caret_value(&cur_ver, &new_ver);
        let new_range = format!("^{res}");
        if range_strategy == "widen" && !semver_satisfies_range(new_version, current_value) {
            return Some(format!("{current_value} || {new_range}"));
        }
        return Some(new_range);
    }

    if cv.starts_with("<=") {
        let sep = if has_space { "<= " } else { "<=" };
        let ver_str = cv.trim_start_matches("<=").trim();
        let dot_count = ver_str.matches('.').count();
        return Some(if dot_count >= 2 {
            format!("{sep}{new_version}")
        } else if dot_count == 1 {
            format!("{sep}{}.{}", new_ver.major, new_ver.minor)
        } else {
            format!("{sep}{}", new_ver.major)
        });
    }

    if cv.starts_with(">=") {
        let sep = if has_space { ">= " } else { ">=" };
        return Some(format!("{sep}{new_version}"));
    }

    if cv.starts_with('=') {
        let sep = if has_space { "= " } else { "=" };
        return Some(format!("{sep}{new_version}"));
    }

    Some(new_version.to_owned())
}

// ── Public API ────────────────────────────────────────────────────────────────

pub fn is_valid(input: &str) -> bool {
    let npm = hex2npm(input);
    let normalized = normalize_hex_range(&npm);
    normalized.split("||").all(|alt| {
        let alt = alt.trim();
        if alt.is_empty() {
            return false;
        }
        // Bare version (after == removal) is valid
        Version::parse(alt.trim_start_matches('=')).is_ok() || VersionReq::parse(alt).is_ok()
    })
}

pub fn is_single_version(constraint: &str) -> bool {
    let c = constraint.trim();
    if Version::parse(c).is_ok() {
        return true;
    }
    if let Some(rest) = c.strip_prefix("==") {
        return Version::parse(rest.trim()).is_ok();
    }
    false
}

pub fn get_pinned_value(version: &str) -> String {
    format!("== {version}")
}

pub fn matches_range(version: &str, range: &str) -> bool {
    let Ok(v) = Version::parse(&hex2npm(version)) else {
        return false;
    };
    semver_matches_v(&v, &hex2npm(range))
}

pub fn is_less_than_range(version: &str, range: &str) -> bool {
    let npm_range = hex2npm(range);
    let Ok(v) = Version::parse(version) else {
        return false;
    };
    if semver_matches_v(&v, &npm_range) {
        return false;
    }
    match extract_min_lower_bound(&npm_range) {
        Some(bound) => v < bound,
        None => false,
    }
}

pub fn get_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    let npm_range = hex2npm(range);
    versions
        .iter()
        .filter(|&&v| Version::parse(v).is_ok_and(|pv| semver_matches_v(&pv, &npm_range)))
        .max_by(|&&a, &&b| Version::parse(a).unwrap().cmp(&Version::parse(b).unwrap()))
        .copied()
}

pub fn min_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    let npm_range = hex2npm(range);
    versions
        .iter()
        .filter(|&&v| Version::parse(v).is_ok_and(|pv| semver_matches_v(&pv, &npm_range)))
        .min_by(|&&a, &&b| Version::parse(a).unwrap().cmp(&Version::parse(b).unwrap()))
        .copied()
}

pub fn get_new_value(
    current_value: &str,
    range_strategy: &str,
    current_version: &str,
    new_version: &str,
) -> Option<String> {
    let npm_current = hex2npm(current_value);
    let new_semver = npm_get_new_value(&npm_current, range_strategy, current_version, new_version)?;
    let mut result = npm2hex(&new_semver);

    // Apply ~> patterns based on original currentValue
    if CV_THREE_PART.is_match(current_value) {
        result = THREE_PART_TILDE_CARET
            .replace_all(&result, "~> $1")
            .into_owned();
    } else if CV_TWO_PART.is_match(current_value) {
        result = TWO_PART_CARET
            .replace_all(&result, |caps: &regex::Captures| format!("~> {}", &caps[1]))
            .into_owned();
    } else {
        result = TILDE_THREE_BARE.replace_all(&result, "~> $1").into_owned();
    }

    if Version::parse(&result).is_ok() {
        result = format!("== {result}");
    }

    Some(result)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "matches(\"$version\", \"$range\") === $expected" — lib/modules/versioning/hex/index.spec.ts line 4
    #[test]
    fn hex_matches_parametrized() {
        let cases = [
            ("4.2.0", "~> 4.0", true),
            ("2.1.0", "~> 2.0.0", false),
            ("2.0.0", ">= 2.0.0 and < 2.1.0", true),
            ("2.1.0", "== 2.0.0 or < 2.1.0", false),
            ("1.9.4", "== 1.9.4", true),
            ("1.9.5", "== 1.9.4", false),
        ];
        for (version, range, expected) in cases {
            assert_eq!(
                matches_range(version, range),
                expected,
                "matches({version:?}, {range:?})"
            );
        }
    }

    // Ported: "getSatisfyingVersion($versions, \"$range\") === $expected" — lib/modules/versioning/hex/index.spec.ts line 19
    #[test]
    fn hex_get_satisfying_version_parametrized() {
        let cases: Vec<(Vec<&str>, &str, Option<&str>)> = vec![
            (
                vec!["0.4.0", "0.5.0", "4.0.0", "4.2.0", "5.0.0"],
                "~> 4.0",
                Some("4.2.0"),
            ),
            (
                vec!["0.4.0", "0.5.0", "4.0.0", "4.2.0", "5.0.0"],
                "~> 4.0.0",
                Some("4.0.0"),
            ),
        ];
        for (versions, range, expected) in cases {
            assert_eq!(
                get_satisfying_version(&versions, range),
                expected,
                "getSatisfyingVersion({versions:?}, {range:?})"
            );
        }
    }

    // Ported: "isValid(\"$input\") === $expected" — lib/modules/versioning/hex/index.spec.ts line 30
    #[test]
    fn hex_is_valid_parametrized() {
        let cases = [
            (">= 1.0.0 and <= 2.0.0", true),
            (">= 1.0.0 or <= 2.0.0", true),
            ("!= 1.0.0", true),
            ("== 1.0.0", true),
        ];
        for (input, expected) in cases {
            assert_eq!(is_valid(input), expected, "isValid({input:?})");
        }
    }

    // Ported: "isSingleVersion(\"$version\") === $expected" — lib/modules/versioning/hex/index.spec.ts line 41
    #[test]
    fn hex_is_single_version_parametrized() {
        let cases = [
            ("1.2.3", true),
            ("== 1.2.3", true),
            ("~> 1.2", false),
            ("~> 1.2.0", false),
            (">= 1.0.0", false),
        ];
        for (version, expected) in cases {
            assert_eq!(
                is_single_version(version),
                expected,
                "isSingleVersion({version:?})"
            );
        }
    }

    // Ported: "getPinnedValue returns == prefixed version" — lib/modules/versioning/hex/index.spec.ts line 52
    #[test]
    fn hex_get_pinned_value() {
        assert_eq!(get_pinned_value("1.2.3"), "== 1.2.3");
    }

    // Ported: "isLessThanRange($version, $range) === $expected" — lib/modules/versioning/hex/index.spec.ts line 56
    #[test]
    fn hex_is_less_than_range_parametrized() {
        let cases = [
            ("0.1.0", ">= 1.0.0 and <= 2.0.0", true),
            ("1.9.0", ">= 1.0.0 and <= 2.0.0", false),
            ("0.9.0", ">= 1.0.0 or >= 2.0.0", true),
            ("1.9.0", ">= 1.0.0 or >= 2.0.0", false),
        ];
        for (version, range, expected) in cases {
            assert_eq!(
                is_less_than_range(version, range),
                expected,
                "isLessThanRange({version:?}, {range:?})"
            );
        }
    }

    // Ported: "minSatisfyingVersion($versions, \"$range\") === $expected" — lib/modules/versioning/hex/index.spec.ts line 69
    #[test]
    fn hex_min_satisfying_version_parametrized() {
        let cases: Vec<(Vec<&str>, &str, Option<&str>)> = vec![
            (
                vec!["0.4.0", "0.5.0", "4.2.0", "5.0.0"],
                "~> 4.0",
                Some("4.2.0"),
            ),
            (vec!["0.4.0", "0.5.0", "4.2.0", "5.0.0"], "~> 4.0.0", None),
        ];
        for (versions, range, expected) in cases {
            assert_eq!(
                min_satisfying_version(&versions, range),
                expected,
                "minSatisfyingVersion({versions:?}, {range:?})"
            );
        }
    }

    // Ported: "getNewValue(\"$currentValue\", \"$rangeStrategy\", \"$currentVersion\", \"$newVersion\") === \"$expected\"" — lib/modules/versioning/hex/index.spec.ts line 80
    #[test]
    fn hex_get_new_value_parametrized() {
        let cases: &[(&str, &str, &str, &str, &str)] = &[
            ("== 3.6.1", "bump", "3.6.1", "3.6.2", "== 3.6.2"),
            ("== 3.6.1", "replace", "3.6.1", "3.6.2", "== 3.6.2"),
            ("~> 1.2", "replace", "1.2.3", "2.0.7", "~> 2.0"),
            ("~> 1.2", "bump", "1.2.3", "2.0.7", "~> 2.0"),
            ("~> 1.2", "bump", "1.2.3", "1.3.1", "~> 1.3"),
            ("~> 1.1", "update-lockfile", "1.2.0", "1.3.0", "~> 1.1"),
            ("~> 1.1", "update-lockfile", "1.2.0", "2.0.0", "~> 2.0"),
            ("~> 1.2.0", "replace", "1.2.3", "2.0.7", "~> 2.0.0"),
            ("~> 1.2.0", "bump", "1.2.3", "2.0.7", "~> 2.0.7"),
            (
                "~> 0.2 and <= 0.2.6",
                "widen",
                "0.2.6",
                "0.2.8",
                "~> 0.2 and <= 0.2.8",
            ),
            (
                ">= 1.0.0 and <= 2.0.0",
                "widen",
                "1.2.3",
                "2.0.7",
                ">= 1.0.0 and <= 2.0.7",
            ),
            (
                ">= 1.0.0 and <= 2.0.0",
                "replace",
                "1.2.3",
                "2.0.7",
                "<= 2.0.7",
            ),
            (
                ">= 1.0.0 or <= 2.0.0",
                "widen",
                "1.2.3",
                "2.0.7",
                ">= 1.0.0 or <= 2.0.0",
            ),
            (
                ">= 1.0.0 or <= 2.0.0",
                "replace",
                "1.2.3",
                "2.0.7",
                "<= 2.0.7",
            ),
            ("~> 0.4", "replace", "0.4.2", "0.6.0", "~> 0.6"),
            ("~> 1.0", "widen", "1.0.0", "2.0.0", "~> 1.0 or ~> 2.0"),
            (
                "~> 1.0.0",
                "widen",
                "1.0.0",
                "2.0.0",
                "~> 1.0.0 or ~> 2.0.0",
            ),
        ];
        for &(cv, strat, cur, new, expected) in cases {
            let result = get_new_value(cv, strat, cur, new);
            assert_eq!(
                result.as_deref(),
                Some(expected),
                "getNewValue({cv:?}, {strat:?}, {cur:?}, {new:?})"
            );
        }
    }
}
