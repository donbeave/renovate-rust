//! Gradle versioning and update-decision logic.
//!
//! Ports `lib/modules/versioning/gradle/compare.ts` and `index.ts`.
//!
//! ## Qualifier ordering
//!
//! `dev` < *(unknown/default)* < `rc`/`cr` < `snapshot` < `final` < `ga`
//! < `release`/`latest`/`sr` < `sp`
//!
//! Unknown qualifiers (rank 0) compare case-sensitively against each other.
//! Ranks != 0 compare by rank regardless of case.
//!
//! ## Null token rule
//!
//! A missing (null) token is Greater than a String token but Less than a
//! Number token. This means `1.0-release < 1.0 < 1.0-sp`.

use std::cmp::Ordering;

use crate::versioning::maven;

// ── Token ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Number(u64),
    Str(String),
}

// ── Separator helpers ─────────────────────────────────────────────────────────

fn is_separator(c: char) -> bool {
    matches!(c, '-' | '.' | '_' | '+')
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_letter(c: char) -> bool {
    !is_separator(c) && !is_digit(c)
}

fn is_transition(prev: char, next: char) -> bool {
    (is_digit(prev) && is_letter(next)) || (is_letter(prev) && is_digit(next))
}

// ── Tokenizer ─────────────────────────────────────────────────────────────────

/// Gradle tokenizer. Returns `None` when consecutive or leading separators are
/// encountered (i.e. the version string is syntactically invalid).
fn tokenize(s: &str) -> Option<Vec<Token>> {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut result: Vec<Token> = Vec::new();
    let mut current_val = String::new();
    let mut valid = true;

    let mut prev: Option<char> = None;

    let yield_token = |current_val: &str, result: &mut Vec<Token>| {
        if current_val.chars().all(|c| c.is_ascii_digit()) {
            let num: u64 = if current_val.is_empty() {
                0
            } else {
                current_val.parse().unwrap_or(0)
            };
            result.push(Token::Number(num));
        } else {
            result.push(Token::Str(current_val.to_owned()));
        }
    };

    for i in 0..=n {
        let next = chars.get(i).copied();
        match next {
            None => {
                // end of string — yield whatever is pending
                yield_token(&current_val, &mut result);
            }
            Some(c) if is_separator(c) => {
                let prev_is_non_sep = prev.is_some_and(|p| !is_separator(p));
                if prev_is_non_sep {
                    yield_token(&current_val, &mut result);
                    current_val.clear();
                } else {
                    // leading separator (prev == None) or consecutive separator
                    valid = false;
                    break;
                }
            }
            Some(c) => {
                // digit ↔ letter transition → implicit separator
                if let Some(p) = prev
                    && !is_separator(p) && is_transition(p, c) {
                        yield_token(&current_val, &mut result);
                        current_val.clear();
                    }
                current_val.push(c);
            }
        }
        prev = next;
    }

    if valid { Some(result) } else { None }
}

// ── Qualifier rank ────────────────────────────────────────────────────────────

/// Case-insensitive qualifier rank.
///
/// `dev`=-1, default=0, `rc`/`cr`=1, `snapshot`=2, `final`=3, `ga`=4,
/// `release`/`latest`/`sr`=5, `sp`=6.
fn qualifier_rank(input: &str) -> i32 {
    let lower = input.to_lowercase();
    match lower.as_str() {
        "dev" => -1,
        "rc" | "cr" => 1,
        "snapshot" => 2,
        "final" => 3,
        "ga" => 4,
        "release" | "latest" | "sr" => 5,
        "sp" => 6,
        _ => 0,
    }
}

// ── Token comparison ──────────────────────────────────────────────────────────

fn string_token_cmp(left: &str, right: &str) -> Ordering {
    let lr = qualifier_rank(left);
    let rr = qualifier_rank(right);
    if lr == 0 && rr == 0 {
        // Both unknown — compare the ORIGINAL (case-sensitive) strings.
        left.cmp(right)
    } else {
        lr.cmp(&rr)
    }
}

/// Compare two tokens, either of which may be absent (`None` = missing/null).
fn token_cmp(left: Option<&Token>, right: Option<&Token>) -> Ordering {
    match (left, right) {
        (None, None) => Ordering::Equal,
        (None, Some(Token::Str(_))) => Ordering::Greater, // null > string
        (None, Some(Token::Number(_))) => Ordering::Less, // null < number
        (Some(Token::Str(_)), None) => Ordering::Less,    // string < null
        (Some(Token::Number(_)), None) => Ordering::Greater, // number > null
        (Some(Token::Number(l)), Some(Token::Number(r))) => l.cmp(r),
        (Some(Token::Str(l)), Some(Token::Str(r))) => string_token_cmp(l, r),
        (Some(Token::Str(_)), Some(Token::Number(_))) => Ordering::Less, // string < number
        (Some(Token::Number(_)), Some(Token::Str(_))) => Ordering::Greater, // number > string
    }
}

// ── Public comparison ─────────────────────────────────────────────────────────

/// Compare two Gradle version strings.
///
/// Invalid strings (consecutive/leading separators) tokenize as `[]` and
/// compare as equal to each other.
pub fn compare(left: &str, right: &str) -> Ordering {
    let lt = tokenize(left).unwrap_or_default();
    let rt = tokenize(right).unwrap_or_default();
    let len = lt.len().max(rt.len());
    for i in 0..len {
        let cmp = token_cmp(lt.get(i), rt.get(i));
        if cmp != Ordering::Equal {
            return cmp;
        }
    }
    Ordering::Equal
}

// ── parse / isVersion ─────────────────────────────────────────────────────────

/// Valid charset: `[-._+a-zA-Z0-9]`.
fn is_valid_char(c: char) -> bool {
    matches!(c, '-' | '.' | '_' | '+' | 'a'..='z' | 'A'..='Z' | '0'..='9')
}

/// Parses a version string and returns its tokens, or `None` for invalid input.
///
/// Rejects: empty strings, chars outside `[-._+a-zA-Z0-9]`, strings starting
/// with `latest` (case-insensitive), and strings that fail tokenization.
fn parse(input: &str) -> Option<Vec<Token>> {
    if input.is_empty() {
        return None;
    }
    if !input.chars().all(is_valid_char) {
        return None;
    }
    if input.to_lowercase().starts_with("latest") {
        return None;
    }
    let tokens = tokenize(input)?;
    if tokens.is_empty() {
        return None;
    }
    Some(tokens)
}

/// Returns `true` when `input` is a valid Gradle version string.
pub fn is_version(input: &str) -> bool {
    parse(input).is_some()
}

// ── isStable ──────────────────────────────────────────────────────────────────

const UNSTABLE: &[&str] = &[
    "dev",
    "a",
    "alpha",
    "b",
    "beta",
    "m",
    "mt",
    "milestone",
    "rc",
    "cr",
    "preview",
    "snapshot",
];

/// Returns `true` when the version contains no known pre-release qualifier.
pub fn is_stable(version: &str) -> bool {
    let Some(tokens) = parse(version) else { return false };
    for token in &tokens {
        if let Token::Str(s) = token {
            let lower = s.to_lowercase();
            if UNSTABLE.contains(&lower.as_str()) {
                return false;
            }
        }
    }
    true
}

// ── major / minor / patch ─────────────────────────────────────────────────────

/// Returns the major component, stripping a leading `v`/`V`.
pub fn get_major(version: &str) -> Option<i64> {
    let stripped = version
        .strip_prefix(|c| c == 'v' || c == 'V')
        .unwrap_or(version);
    let tokens = parse(stripped)?;
    match tokens.first() {
        Some(Token::Number(n)) => Some(*n as i64),
        _ => None,
    }
}

pub fn get_minor(version: &str) -> Option<i64> {
    let stripped = version
        .strip_prefix(|c| c == 'v' || c == 'V')
        .unwrap_or(version);
    let tokens = parse(stripped)?;
    match (tokens.first(), tokens.get(1)) {
        (Some(Token::Number(_)), Some(Token::Number(n))) => Some(*n as i64),
        (Some(Token::Number(_)), _) => Some(0),
        _ => None,
    }
}

pub fn get_patch(version: &str) -> Option<i64> {
    let stripped = version
        .strip_prefix(|c| c == 'v' || c == 'V')
        .unwrap_or(version);
    let tokens = parse(stripped)?;
    match (tokens.first(), tokens.get(1), tokens.get(2)) {
        (Some(Token::Number(_)), Some(Token::Number(_)), Some(Token::Number(n))) => Some(*n as i64),
        (Some(Token::Number(_)), ..) => Some(0),
        _ => None,
    }
}

// ── Prefix range ──────────────────────────────────────────────────────────────

/// A Gradle prefix range (e.g. `1.2.+`).
/// `tokens` is empty for the bare `+` wildcard.
struct PrefixRange {
    tokens: Vec<Token>,
}

/// Parses a Gradle prefix range.
///
/// `+` alone → `PrefixRange { tokens: [] }`.
/// `1.2.+` → tokens = [Number(1), Number(2)].
fn parse_prefix_range(input: &str) -> Option<PrefixRange> {
    if input.is_empty() {
        return None;
    }
    if input.trim() == "+" {
        return Some(PrefixRange { tokens: vec![] });
    }
    // Must end with `[sep]+` where the char before the separator is NOT a separator.
    // Pattern: /[^-._+][-._]\+$/
    let bytes = input.as_bytes();
    let len = input.len();
    if len < 3 {
        return None;
    }
    // last char must be '+'
    if bytes[len - 1] != b'+' {
        return None;
    }
    // second-to-last must be a separator (., -, _, but NOT +)
    let sep_char = bytes[len - 2] as char;
    if !matches!(sep_char, '-' | '.' | '_') {
        return None;
    }
    // third-from-last must NOT be a separator
    let pre_char = input.chars().rev().nth(2)?;
    if is_separator(pre_char) {
        return None;
    }
    // Strip the trailing `[sep]+` to get the prefix value
    let prefix_value = &input[..len - 2];
    let tokens = tokenize(prefix_value)?;
    Some(PrefixRange { tokens })
}

// ── Maven-based range (single interval + optional !! preferred) ───────────────

enum Bound {
    Inclusive,
    Exclusive,
}

struct MavenBasedRange {
    left_bound: Bound,
    left_val: Option<String>,
    right_bound: Bound,
    right_val: Option<String>,
    preferred_val: Option<String>,
}

/// Valid chars for range values: `[-._+a-zA-Z0-9]`.
fn is_range_val_char(c: char) -> bool {
    is_valid_char(c)
}

/// Parses a single Gradle maven-based range, optionally with `!!preferred`.
///
/// Supports: `[1.0,2.0)`, `(,1.0]`, `[1.0,)`, `[1.0,2.0)!!1.5`, etc.
/// Left brackets: `[` (inclusive), `]` or `(` (exclusive).
/// Right brackets: `]` (inclusive), `[` or `)` (exclusive).
fn parse_maven_based_range(input: &str) -> Option<MavenBasedRange> {
    if input.is_empty() {
        return None;
    }

    // Split off optional `!!preferred` suffix.
    let (base, preferred_val) = if let Some(idx) = input.rfind("!!") {
        let preferred = &input[idx + 2..];
        // preferred must be non-empty and all valid chars
        if preferred.is_empty() || !preferred.chars().all(is_range_val_char) {
            return None;
        }
        (&input[..idx], Some(preferred.to_owned()))
    } else {
        (input, None)
    };

    // Parse the base range.
    let chars: Vec<char> = base.chars().collect();
    if chars.is_empty() {
        return None;
    }

    // Left bracket.
    let left_bracket = chars[0];
    let left_bound = match left_bracket {
        '[' => Bound::Inclusive,
        ']' | '(' => Bound::Exclusive,
        _ => return None,
    };

    let mut pos = 1;
    // skip optional whitespace
    while pos < chars.len() && chars[pos] == ' ' {
        pos += 1;
    }

    // Left value (optional, lazy up to the comma).
    let left_val_start = pos;
    while pos < chars.len() && is_range_val_char(chars[pos]) {
        pos += 1;
    }
    let left_val_str: String = chars[left_val_start..pos].iter().collect();
    // skip whitespace
    while pos < chars.len() && chars[pos] == ' ' {
        pos += 1;
    }

    // Separator comma.
    if pos >= chars.len() || chars[pos] != ',' {
        return None;
    }
    pos += 1;
    // skip whitespace after comma
    while pos < chars.len() && chars[pos] == ' ' {
        pos += 1;
    }

    // Right value (optional, lazy up to the closing bracket).
    let right_val_start = pos;
    while pos < chars.len() && is_range_val_char(chars[pos]) {
        pos += 1;
    }
    let right_val_str: String = chars[right_val_start..pos].iter().collect();

    // skip whitespace before right bracket
    while pos < chars.len() && chars[pos] == ' ' {
        pos += 1;
    }

    // Right bracket.
    if pos >= chars.len() {
        return None;
    }
    let right_bracket = chars[pos];
    let right_bound = match right_bracket {
        ']' => Bound::Inclusive,
        '[' | ')' => Bound::Exclusive,
        _ => return None,
    };
    pos += 1;

    // Nothing should remain in base after the right bracket.
    if pos != chars.len() {
        return None;
    }

    // Validate left/right values (must be valid versions if non-empty).
    let left_val = if left_val_str.is_empty() {
        None
    } else if is_version(&left_val_str) {
        Some(left_val_str)
    } else {
        return None;
    };
    let right_val = if right_val_str.is_empty() {
        None
    } else if is_version(&right_val_str) {
        Some(right_val_str)
    } else {
        return None;
    };

    // Left must be ≤ right when both are present.
    if let (Some(lv), Some(rv)) = (&left_val, &right_val)
        && compare(lv, rv) == Ordering::Greater {
            return None;
        }

    Some(MavenBasedRange {
        left_bound,
        left_val,
        right_bound,
        right_val,
        preferred_val,
    })
}

// ── Single-version range  [1.2.3] ────────────────────────────────────────────

fn parse_single_version_range(input: &str) -> Option<String> {
    let trimmed = input.trim();
    if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
        return None;
    }
    let inner = trimmed[1..trimmed.len() - 1].trim();
    if is_version(inner) {
        Some(inner.to_owned())
    } else {
        None
    }
}

// ── isValid ───────────────────────────────────────────────────────────────────

/// Returns `true` when `s` is a valid version, prefix range, maven-based range,
/// or single-version range.
pub fn is_valid(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    is_version(s)
        || parse_prefix_range(s).is_some()
        || parse_maven_based_range(s).is_some()
        || parse_single_version_range(s).is_some()
}

// ── matches ───────────────────────────────────────────────────────────────────

/// Returns `true` when `version` matches `range`.
pub fn matches_range(version: &str, range: &str) -> bool {
    let Some(version_tokens) = parse(version) else { return false };
    if version.is_empty() || range.is_empty() {
        return false;
    }

    // Bare version equality.
    if is_version(range) {
        return compare(version, range) == Ordering::Equal;
    }

    // Single-version range [X].
    if let Some(val) = parse_single_version_range(range) {
        return compare(version, &val) == Ordering::Equal;
    }

    // Prefix range.
    if let Some(prefix) = parse_prefix_range(range) {
        if prefix.tokens.is_empty() {
            return true; // '+' matches everything
        }
        let n = prefix.tokens.len();
        // The first n tokens of version must equal the prefix tokens.
        let ver_prefix: Vec<u64> = version_tokens
            .iter()
            .take(n)
            .filter_map(|t| {
                if let Token::Number(v) = t {
                    Some(*v)
                } else {
                    None
                }
            })
            .collect();
        let rng_prefix: Vec<u64> = prefix
            .tokens
            .iter()
            .filter_map(|t| {
                if let Token::Number(v) = t {
                    Some(*v)
                } else {
                    None
                }
            })
            .collect();
        if ver_prefix.len() < n || rng_prefix.len() < n {
            // Fall back to string join comparison (handles non-numeric tokens).
            let ver_str: String = version_tokens
                .iter()
                .take(n)
                .map(|t| match t {
                    Token::Number(v) => v.to_string(),
                    Token::Str(s) => s.clone(),
                })
                .collect::<Vec<_>>()
                .join(".");
            let rng_str: String = prefix
                .tokens
                .iter()
                .map(|t| match t {
                    Token::Number(v) => v.to_string(),
                    Token::Str(s) => s.clone(),
                })
                .collect::<Vec<_>>()
                .join(".");
            return compare(&ver_str, &rng_str) == Ordering::Equal;
        }
        // Both numeric prefix comparison.
        let ver_joined = ver_prefix
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(".");
        let rng_joined = rng_prefix
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(".");
        return compare(&ver_joined, &rng_joined) == Ordering::Equal;
    }

    // Maven-based range.
    let Some(mbr) = parse_maven_based_range(range) else { return false };
    let left_ok = match &mbr.left_val {
        None => true,
        Some(lv) => match mbr.left_bound {
            Bound::Exclusive => compare(lv, version) == Ordering::Less,
            Bound::Inclusive => compare(lv, version) != Ordering::Greater,
        },
    };
    let right_ok = match &mbr.right_val {
        None => true,
        Some(rv) => match mbr.right_bound {
            Bound::Exclusive => compare(version, rv) == Ordering::Less,
            Bound::Inclusive => compare(version, rv) != Ordering::Greater,
        },
    };
    left_ok && right_ok
}

// ── isGreaterThan ─────────────────────────────────────────────────────────────

pub fn is_greater_than(a: &str, b: &str) -> bool {
    compare(a, b) == Ordering::Greater
}

// ── getSatisfyingVersion / minSatisfyingVersion ───────────────────────────────

pub fn get_satisfying_version(versions: &[&str], range: &str) -> Option<String> {
    versions.iter().fold(None, |best: Option<String>, &v| {
        if matches_range(v, range) {
            match best {
                None => Some(v.to_owned()),
                Some(ref b) if is_greater_than(v, b) => Some(v.to_owned()),
                _ => best,
            }
        } else {
            best
        }
    })
}

pub fn min_satisfying_version(versions: &[&str], range: &str) -> Option<String> {
    versions.iter().fold(None, |best: Option<String>, &v| {
        if matches_range(v, range) {
            match best {
                None => Some(v.to_owned()),
                Some(ref b) if compare(v, b) == Ordering::Less => Some(v.to_owned()),
                _ => best,
            }
        } else {
            best
        }
    })
}

// ── getNewValue ───────────────────────────────────────────────────────────────

/// Compute a new Gradle range/version value given a new version.
///
/// Returns `None` when `currentValue` is the bare `+` wildcard (no update
/// makes sense) or when the operation cannot be determined. Returns the
/// original `currentValue` unchanged when it is a non-version, non-range
/// expression.
pub fn get_new_value(
    current_value: &str,
    range_strategy: Option<&str>,
    new_version: &str,
) -> Option<String> {
    // Plain version → return the new version directly.
    if is_version(current_value) {
        return Some(new_version.to_owned());
    }

    // Prefix range (e.g. `1.2.+`).
    let prefix_range = parse_prefix_range(current_value);
    let parsed_new = parse(new_version);
    if let (Some(prefix), Some(new_tokens)) = (&prefix_range, &parsed_new) {
        if prefix.tokens.is_empty() {
            // '+' → no meaningful update
            return None;
        }
        let plen = prefix.tokens.len();
        if plen <= new_tokens.len() {
            // Take the first `plen` values from newVersion.
            let new_prefixed: Vec<String> = new_tokens[..plen]
                .iter()
                .map(|t| match t {
                    Token::Number(n) => n.to_string(),
                    Token::Str(s) => s.clone(),
                })
                .collect();
            return Some(format!("{}.+", new_prefixed.join(".")));
        } else {
            // newVersion is shorter than prefix → drop the prefix, use newVersion.
            return Some(new_version.to_owned());
        }
    }

    // Maven-based range with `!!preferred`.
    if let Some(mbr) = parse_maven_based_range(current_value)
        && let Some(ref preferred) = mbr.preferred_val {
            let preferred = preferred.clone();
            // Strip `!!preferred` from the end to get the base range.
            let suffix = format!("!!{preferred}");
            let base_range = match current_value.rfind(&suffix) {
                Some(idx) => &current_value[..idx],
                None => return Some(current_value.to_owned()),
            };
            let new_base_range = maven::get_new_value(base_range, range_strategy, new_version);

            let preferred_is_boundary = mbr.left_val.as_deref() == Some(&preferred)
                || mbr.right_val.as_deref() == Some(&preferred);

            let new_parsed_mbr = parse_maven_based_range(&new_base_range);
            let preferred_still_present = new_parsed_mbr
                .as_ref()
                .map(|r| {
                    r.left_val.as_deref() == Some(&preferred)
                        || r.right_val.as_deref() == Some(&preferred)
                })
                .unwrap_or(false);

            let new_preferred = if preferred_is_boundary && !preferred_still_present {
                new_version.to_owned()
            } else {
                preferred
            };
            return Some(format!("{new_base_range}!!{new_preferred}"));
        }

    // Delegate to Maven for all other range forms.
    Some(maven::get_new_value(
        current_value,
        range_strategy,
        new_version,
    ))
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn cmp(a: &str, b: &str) -> i32 {
        match compare(a, b) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }

    // Ported: "compare("$a", "$b") === $expected" — versioning/gradle/index.spec.ts line 6
    #[test]
    fn gradle_compare_equal() {
        assert_eq!(cmp("1", "1"), 0);
        assert_eq!(cmp("a", "a"), 0);
        assert_eq!(cmp("1a1", "1.a.1"), 0);
        assert_eq!(cmp("1a1", "1-a-1"), 0);
        assert_eq!(cmp("1a1", "1_a_1"), 0);
        assert_eq!(cmp("1a1", "1+a+1"), 0);
        assert_eq!(cmp("1.a.1", "1a1"), 0);
        assert_eq!(cmp("1-a-1", "1a1"), 0);
        assert_eq!(cmp("1_a_1", "1a1"), 0);
        assert_eq!(cmp("1+a+1", "1a1"), 0);
        assert_eq!(cmp("1.a.1", "1-a+1"), 0);
        assert_eq!(cmp("1-a+1", "1.a-1"), 0);
        assert_eq!(cmp("1.a-1", "1a1"), 0);
        assert_eq!(cmp("dev", "dev"), 0);
        assert_eq!(cmp("rc", "rc"), 0);
        assert_eq!(cmp("preview", "preview"), 0);
        assert_eq!(cmp("release", "release"), 0);
        assert_eq!(cmp("final", "final"), 0);
        assert_eq!(cmp("snapshot", "SNAPSHOT"), 0);
        assert_eq!(cmp("SNAPSHOT", "snapshot"), 0);
        assert_eq!(cmp("Hoxton.SR1", "Hoxton.sr-1"), 0);
        assert_eq!(cmp("", ""), 0);
        assert_eq!(cmp("___", "..."), 0);
    }

    // Ported: "compare("$a", "$b") === $expected" — versioning/gradle/index.spec.ts line 6
    #[test]
    fn gradle_compare_less() {
        assert_eq!(cmp("1.1", "1.2"), -1);
        assert_eq!(cmp("1.a", "1.1"), -1);
        assert_eq!(cmp("1.A", "1.B"), -1);
        assert_eq!(cmp("1.B", "1.a"), -1);
        assert_eq!(cmp("1.a", "1.b"), -1);
        assert_eq!(cmp("1.1", "1.1.0"), -1);
        assert_eq!(cmp("1.1.a", "1.1"), -1);
        assert_eq!(cmp("1.0-dev", "1.0-alpha"), -1);
        assert_eq!(cmp("1.0-alpha", "1.0-rc"), -1);
        assert_eq!(cmp("1.0-zeta", "1.0-rc"), -1);
        assert_eq!(cmp("1.0-rc", "1.0-final"), -1);
        assert_eq!(cmp("1.0-final", "1.0-ga"), -1);
        assert_eq!(cmp("1.0-ga", "1.0-release"), -1);
        assert_eq!(cmp("1.0-rc", "1.0-release"), -1);
        assert_eq!(cmp("1.0-final", "1.0"), -1);
        assert_eq!(cmp("1.0-alpha", "1.0-SNAPSHOT"), -1);
        assert_eq!(cmp("1.0-zeta", "1.0-SNAPSHOT"), -1);
        assert_eq!(cmp("1.0-zeta", "1.0-rc"), -1);
        assert_eq!(cmp("1.0-rc", "1.0"), -1);
        assert_eq!(cmp("1.0-preview", "1.0"), -1);
        assert_eq!(cmp("1.0", "1.0-20150201.121010-123"), -1);
        assert_eq!(cmp("1.0-20150201.121010-123", "1.1"), -1);
        assert_eq!(cmp("Hoxton.RELEASE", "Hoxton.SR1"), -1);
        assert_eq!(cmp("1.0-release", "1.0-sp-1"), -1);
        assert_eq!(cmp("1.0-sp-1", "1.0-sp-2"), -1);
        assert_eq!(cmp("384.vf35b_f26814ec", "400.v35420b_922dcb_"), -1);
    }

    // Ported: "compare("$a", "$b") === $expected" — versioning/gradle/index.spec.ts line 6
    #[test]
    fn gradle_compare_greater() {
        assert_eq!(cmp("1.2", "1.1"), 1);
        assert_eq!(cmp("1.1", "1.1.a"), 1);
        assert_eq!(cmp("1.B", "1.A"), 1);
        assert_eq!(cmp("1.a", "1.B"), 1);
        assert_eq!(cmp("1.b", "1.a"), 1);
        assert_eq!(cmp("1.1.0", "1.1"), 1);
        assert_eq!(cmp("1.1", "1.a"), 1);
        assert_eq!(cmp("1.0-alpha", "1.0-dev"), 1);
        assert_eq!(cmp("1.0-rc", "1.0-alpha"), 1);
        assert_eq!(cmp("1.0-rc", "1.0-zeta"), 1);
        assert_eq!(cmp("1.0-release", "1.0-rc"), 1);
        assert_eq!(cmp("1.0-final", "1.0-rc"), 1);
        assert_eq!(cmp("1.0-ga", "1.0-final"), 1);
        assert_eq!(cmp("1.0-release", "1.0-ga"), 1);
        assert_eq!(cmp("1.0-release", "1.0-final"), 1);
        assert_eq!(cmp("1.0", "1.0-final"), 1);
        assert_eq!(cmp("1.0-SNAPSHOT", "1.0-alpha"), 1);
        assert_eq!(cmp("1.0-SNAPSHOT", "1.0-zeta"), 1);
        assert_eq!(cmp("1.0-rc", "1.0-zeta"), 1);
        assert_eq!(cmp("1.0", "1.0-rc"), 1);
        assert_eq!(cmp("1.0", "1.0-preview"), 1);
        assert_eq!(cmp("1.0-20150201.121010-123", "1.0"), 1);
        assert_eq!(cmp("1.1", "1.0-20150201.121010-123"), 1);
        assert_eq!(cmp("Hoxton.SR1", "Hoxton.RELEASE"), 1);
        assert_eq!(cmp("1.0-sp-1", "1.0-release"), 1);
        assert_eq!(cmp("1.0-sp-2", "1.0-sp-1"), 1);
        assert_eq!(cmp("400.v35420b_922dcb_", "384.vf35b_f26814ec"), 1);
    }

    // Ported: "parsePrefixRange("$rangeStr") is null" — versioning/gradle/index.spec.ts line 89
    #[test]
    fn gradle_parse_prefix_range_null() {
        assert!(parse_prefix_range("").is_none());
        assert!(parse_prefix_range("1.2.3-SNAPSHOT").is_none());
        assert!(parse_prefix_range("1.2..+").is_none());
        assert!(parse_prefix_range("1.2.++").is_none());
    }

    // Ported: "parseMavenBasedRange("$rangeStr") is null" — versioning/gradle/index.spec.ts line 102
    #[test]
    fn gradle_parse_maven_based_range_null() {
        assert!(parse_maven_based_range("").is_none());
        assert!(parse_maven_based_range("[2,1]").is_none()); // left > right
    }

    // Ported: "isValid("$input") === $expected" — versioning/gradle/index.spec.ts line 127
    #[test]
    fn gradle_is_valid() {
        assert!(is_valid("1.2.3"));
        assert!(is_valid("1"));
        assert!(is_valid("foobar"));
        assert!(is_valid("[1.0,2.0)"));
        assert!(is_valid("1.+"));
        assert!(is_valid("+"));
        assert!(!is_valid("1..2"));
        assert!(!is_valid("1++2"));
        assert!(!is_valid("1--2"));
        assert!(!is_valid("1__2"));
        assert!(is_valid("400.v35420b_922dcb_"));
        assert!(is_valid("400.v35420b_922dcb"));
        assert!(!is_valid("__"));
        assert!(!is_valid("_."));
        assert!(!is_valid("._"));
        assert!(!is_valid("_+"));
        assert!(!is_valid("+."));
        assert!(!is_valid(".+"));
        assert!(!is_valid("1.?"));
        assert!(!is_valid("1.."));
        assert!(!is_valid("1--"));
    }

    // Ported: "isVersion("$input") === $expected" — versioning/gradle/index.spec.ts line 140
    #[test]
    fn gradle_is_version() {
        assert!(is_version("1"));
        assert!(is_version("1.2"));
        assert!(is_version("1.2.3"));
        assert!(is_version("1.2.3.4"));
        assert!(is_version("foobar"));
        assert!(is_version("final"));
        assert!(is_version("v1.2.3.4"));
        assert!(is_version("400.v35420b_922dcb_"));
        assert!(is_version("400.v35420b_922dcb"));
        assert!(!is_version(""));
        assert!(!is_version("latest"));
        assert!(!is_version("1..2"));
        assert!(!is_version("1++2"));
        assert!(!is_version("1--2"));
        assert!(!is_version("1__2"));
        assert!(!is_version("__"));
        assert!(!is_version("_."));
        assert!(!is_version("._"));
        assert!(!is_version("_+"));
        assert!(!is_version("+."));
        assert!(!is_version(".+"));
        assert!(!is_version("1.2.3.4 s")); // space not in charset
    }

    // Ported: "isStable("$input") === $expected" — versioning/gradle/index.spec.ts line 180
    #[test]
    fn gradle_is_stable() {
        assert!(!is_stable(""));
        assert!(!is_stable("latest"));
        assert!(is_stable("foobar"));
        assert!(is_stable("final"));
        assert!(is_stable("1"));
        assert!(!is_stable("1..2"));
        assert!(is_stable("1.2"));
        assert!(is_stable("1.2.3"));
        assert!(!is_stable("1.2.3.4 s")); // space not valid
        assert!(is_stable("1.2.3.4"));
        assert!(is_stable("v1.2.3.4"));
        assert!(!is_stable("1-alpha-1"));
        assert!(!is_stable("1-b1"));
        assert!(is_stable("1-foo"));
        assert!(is_stable("1-final-1.0.0"));
        assert!(is_stable("1-release"));
        assert!(is_stable("1.final"));
        assert!(!is_stable("1.0milestone1"));
        assert!(is_stable("1-sp"));
        assert!(is_stable("1-ga-1"));
        assert!(is_stable("1.3-groovy-2.5"));
        assert!(!is_stable("1.3-RC1-groovy-2.5"));
        assert!(!is_stable("1-preview"));
        assert!(is_stable("Hoxton.RELEASE"));
        assert!(is_stable("Hoxton.SR"));
        assert!(is_stable("Hoxton.SR1"));
        assert!(!is_stable("1.3.5-native-mt-1.3.71-release-429"));
        assert!(!is_stable("1.0-dev"));
    }

    // Ported: '"$input" is represented as [$major, $minor, $patch]' — versioning/gradle/index.spec.ts line 216
    #[test]
    fn gradle_major_minor_patch() {
        assert_eq!(get_major(""), None);
        assert_eq!(get_minor(""), None);
        assert_eq!(get_patch(""), None);

        assert_eq!(get_major("1"), Some(1));
        assert_eq!(get_minor("1"), Some(0));
        assert_eq!(get_patch("1"), Some(0));

        assert_eq!(get_major("1.2"), Some(1));
        assert_eq!(get_minor("1.2"), Some(2));
        assert_eq!(get_patch("1.2"), Some(0));

        assert_eq!(get_major("1.2.3"), Some(1));
        assert_eq!(get_minor("1.2.3"), Some(2));
        assert_eq!(get_patch("1.2.3"), Some(3));

        assert_eq!(get_major("v1.2.3"), Some(1));
        assert_eq!(get_minor("v1.2.3"), Some(2));
        assert_eq!(get_patch("v1.2.3"), Some(3));

        assert_eq!(get_major("1.2.3.4"), Some(1));
        assert_eq!(get_minor("1.2.3.4"), Some(2));
        assert_eq!(get_patch("1.2.3.4"), Some(3));

        assert_eq!(get_major("1rc42"), Some(1));
        assert_eq!(get_minor("1rc42"), Some(0));
        assert_eq!(get_patch("1rc42"), Some(0));

        assert_eq!(get_major("1-rc10"), Some(1));
        assert_eq!(get_minor("1-rc10"), Some(0));
        assert_eq!(get_patch("1-rc10"), Some(0));

        assert_eq!(get_major("1-rc42"), Some(1));
        assert_eq!(get_minor("1-rc42"), Some(0));
        assert_eq!(get_patch("1-rc42"), Some(0));

        assert_eq!(get_major("1-rc42-1"), Some(1));
        assert_eq!(get_minor("1-rc42-1"), Some(0));
        assert_eq!(get_patch("1-rc42-1"), Some(0));
    }

    // Ported: 'matches("$version", "$range") === $expected' — versioning/gradle/index.spec.ts line 239
    #[test]
    fn gradle_matches() {
        assert!(!matches_range("1", "[[]]"));
        assert!(matches_range("0", "[0,1]"));
        assert!(matches_range("1", "[0,1]"));
        assert!(!matches_range("0", "(0,1)"));
        assert!(!matches_range("1", "(0,1)"));
        assert!(matches_range("1", "(0,2)"));
        assert!(matches_range("1", "[0,2]"));
        assert!(matches_range("1", "(,1]"));
        assert!(!matches_range("1", "(,1)"));
        assert!(matches_range("1", "[1,)"));
        assert!(!matches_range("1", "(1,)"));
        assert!(!matches_range("0", ""));
        assert!(matches_range("1", "1"));
        assert!(matches_range("1.2.3", "1.2.+"));
        assert!(matches_range("1.2.3.4", "1.2.+"));
        assert!(!matches_range("1.3.0", "1.2.+"));
        assert!(matches_range("foo", "+"));
        assert!(matches_range("1", "+"));
        assert!(matches_range("99999999999", "+"));
        assert!(matches_range("1.2.3", "[1.2.3]"));
        assert!(!matches_range("1.2.3", "[1.2.4]"));
    }

    // Ported: 'isGreaterThan("$a", "$b") === $expected' — versioning/gradle/index.spec.ts line 271
    #[test]
    fn gradle_is_greater_than() {
        assert!(is_greater_than("1.1", "1"));
    }

    // Ported: 'minSatisfyingVersion($versions, "$range") === $expected' — versioning/gradle/index.spec.ts line 280
    #[test]
    fn gradle_min_satisfying_version() {
        assert_eq!(
            min_satisfying_version(&["0", "1.5", "1", "2"], "1.+"),
            Some("1".to_owned())
        );
    }

    // Ported: 'getSatisfyingVersion($versions, "$range") === $expected' — versioning/gradle/index.spec.ts line 292
    #[test]
    fn gradle_get_satisfying_version() {
        assert_eq!(
            get_satisfying_version(&["0", "1", "1.5", "2"], "1.+"),
            Some("1.5".to_owned())
        );
    }

    // Ported: 'getNewValue($currentValue, $rangeStrategy, $currentVersion, $newVersion, $expected) === $expected' — versioning/gradle/index.spec.ts line 304
    #[test]
    fn gradle_get_new_value() {
        // Plain version → returns new version.
        assert_eq!(get_new_value("1", None, "1.1"), Some("1.1".to_owned()));
        // Maven ranges without preferred — delegate to Maven.
        assert_eq!(
            get_new_value("[1.2.3,]", None, "1.2.4"),
            Some("[1.2.3,]".to_owned())
        );
        assert_eq!(
            get_new_value("[1.2.3,2)", None, "2.0.0"),
            Some("[1.2.3,3)".to_owned())
        );
        assert_eq!(
            get_new_value("[1.3,1.4)", None, "2.0.0"),
            Some("[2.0,3.0)".to_owned())
        );
        assert_eq!(
            get_new_value("[1.3,1.4)", None, "1.5.1"),
            Some("[1.5,1.6)".to_owned())
        );
        assert_eq!(
            get_new_value("[1,1.4)", None, "1.5.1"),
            Some("[1,1.6)".to_owned())
        );
        assert_eq!(
            get_new_value("[1.3,2)", None, "1.4.0"),
            Some("[1.3,2)".to_owned())
        );
        // Invalid range → return original.
        assert_eq!(get_new_value("1.?", None, "2"), Some("1.?".to_owned()));
        assert_eq!(get_new_value("1..", None, "2"), Some("1..".to_owned()));
        assert_eq!(get_new_value("1--", None, "2"), Some("1--".to_owned()));
        // Bare '+' → None.
        assert_eq!(get_new_value("+", None, "1.2.4"), None);
        // Prefix ranges.
        assert_eq!(get_new_value("1.+", None, "1.2.4"), Some("1.+".to_owned()));
        assert_eq!(get_new_value("1.+", None, "2.1.2"), Some("2.+".to_owned()));
        assert_eq!(get_new_value("1.+", None, "2"), Some("2.+".to_owned()));
        assert_eq!(
            get_new_value("1.3.+", None, "1.3.4"),
            Some("1.3.+".to_owned())
        );
        assert_eq!(
            get_new_value("1.3.+", None, "1.5.2"),
            Some("1.5.+".to_owned())
        );
        assert_eq!(get_new_value("1.3.+", None, "2"), Some("2".to_owned()));
        // Single-version range → bump strategy.
        assert_eq!(
            get_new_value("[1.2.3]", Some("bump"), "1.2.4"),
            Some("[1.2.4]".to_owned())
        );
        assert_eq!(
            get_new_value("[1.0.0,1.2.3]", Some("bump"), "1.2.4"),
            Some("[1.0.0,1.2.4]".to_owned())
        );
        assert_eq!(
            get_new_value("[1.0.0,1.2.23]", Some("bump"), "1.2.23"),
            Some("[1.0.0,1.2.23]".to_owned())
        );
        assert_eq!(
            get_new_value("(,1.0]", Some("bump"), "2.0"),
            Some("(,2.0]".to_owned())
        );
        assert_eq!(
            get_new_value("],1.0]", Some("bump"), "2.0"),
            Some("],2.0]".to_owned())
        );
        assert_eq!(
            get_new_value("(,1.0)", Some("bump"), "2.0"),
            Some("(,3.0)".to_owned())
        );
        assert_eq!(
            get_new_value("],1.0[", Some("bump"), "],2.0["),
            Some("],1.0[".to_owned())
        );
        assert_eq!(
            get_new_value("[1.0,1.2],[1.3,1.5)", Some("bump"), "1.2.4"),
            Some("[1.0,1.2],[1.3,1.5)".to_owned())
        );
        assert_eq!(
            get_new_value("[1.0,1.2],[1.3,1.5[", Some("bump"), "1.2.4"),
            Some("[1.0,1.2],[1.3,1.5[".to_owned())
        );
        assert_eq!(
            get_new_value("[1.2.3,)", Some("bump"), "1.2.4"),
            Some("[1.2.4,)".to_owned())
        );
        assert_eq!(
            get_new_value("[1.2.3,[", Some("bump"), "1.2.4"),
            Some("[1.2.4,[".to_owned())
        );
        // !!preferred ranges.
        assert_eq!(
            get_new_value("(,1.0]!!1.0", Some("bump"), "2.0"),
            Some("(,2.0]!!2.0".to_owned())
        );
        assert_eq!(
            get_new_value("],1.0]!!1.0", Some("bump"), "2.0"),
            Some("],2.0]!!2.0".to_owned())
        );
        assert_eq!(
            get_new_value("(,1.0)!!1.0", Some("bump"), "2.0"),
            Some("(,3.0)!!2.0".to_owned())
        );
        assert_eq!(
            get_new_value("],1.0[!!1.0", Some("bump"), "],2.0["),
            Some("],1.0[!!1.0".to_owned())
        );
        assert_eq!(
            get_new_value("[1.0,1.2],[1.3,1.5)!!1.0", Some("bump"), "1.2.4"),
            Some("[1.0,1.2],[1.3,1.5)!!1.0".to_owned())
        );
        assert_eq!(
            get_new_value("[1.0,1.2],[1.3,1.5[!!1.0", Some("bump"), "1.2.4"),
            Some("[1.0,1.2],[1.3,1.5[!!1.0".to_owned())
        );
        assert_eq!(
            get_new_value("[1.2.3,)!!1.2.3", Some("bump"), "1.2.4"),
            Some("[1.2.4,)!!1.2.4".to_owned())
        );
        assert_eq!(
            get_new_value("[1.2.3,[!!1.2.3", Some("bump"), "1.2.4"),
            Some("[1.2.4,[!!1.2.4".to_owned())
        );
    }
}
