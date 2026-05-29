//! Maven versioning and update-decision logic.
//!
//! Ports the comparison algorithm from Renovate's
//! `lib/modules/versioning/maven/compare.ts`, which itself follows the
//! [Maven Version Order Specification](https://maven.apache.org/pom.html#version-order-specification).
//!
//! ## Qualifier ordering
//!
//! Within a hyphen-separated qualifier:
//!
//! `alpha` < `beta` < `milestone` < `rc` < `snapshot` < *(release/empty)* < `sp`
//!
//! Unknown qualifiers compare alphabetically after `snapshot` and before `sp`.
//!
//! ## Null tokens
//!
//! Trailing zero/empty tokens are ignored. Numbers 0 and qualifiers `""`,
//! `"final"`, `"ga"`, `"release"`, `"latest"`, and `"sr"` are all "null".

use std::cmp::Ordering;

// ── Token types ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
enum Prefix {
    Dot,
    Hyphen,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenValue {
    Number(u64),
    Qualifier(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    prefix: Prefix,
    value: TokenValue,
    is_transition: bool,
}

impl Token {
    fn is_null(&self) -> bool {
        match &self.value {
            TokenValue::Number(n) => *n == 0,
            TokenValue::Qualifier(q) => {
                matches!(
                    q.as_str(),
                    "" | "final" | "ga" | "release" | "latest" | "sr"
                )
            }
        }
    }

    fn null_for(other: &Token) -> Token {
        Token {
            prefix: other.prefix.clone(),
            value: match other.value {
                TokenValue::Number(_) => TokenValue::Number(0),
                TokenValue::Qualifier(_) => TokenValue::Qualifier(String::new()),
            },
            is_transition: false,
        }
    }
}

// ── Qualifier ordering ────────────────────────────────────────────────────────

/// Returns the sort order for a known qualifier. `None` = unknown qualifier.
fn qualifier_order(q: &str, is_transition: bool) -> Option<i32> {
    match q {
        "alpha" => Some(1),
        "a" if is_transition => Some(1),
        "beta" => Some(2),
        "b" if is_transition => Some(2),
        "milestone" => Some(3),
        "m" if is_transition => Some(3),
        "rc" | "cr" | "preview" => Some(4),
        "snapshot" | "snap" => Some(5),
        "" | "final" | "ga" | "release" | "latest" | "sr" => Some(6),
        "sp" => Some(7),
        _ => None,
    }
}

/// Ordering position of a token relative to others in the compare loop.
///
/// Maven's `commonOrder`:
/// - qualifier-type → 1 (before numbers in same prefix group)
/// - hyphen-prefixed number → 2
/// - dot-prefixed number → 3
fn common_order(token: &Token) -> i32 {
    match &token.value {
        TokenValue::Qualifier(_) => 1,
        TokenValue::Number(_) => match token.prefix {
            Prefix::Hyphen => 2,
            _ => 3,
        },
    }
}

fn compare_tokens(left: &Token, right: &Token) -> Ordering {
    let lo = common_order(left);
    let ro = common_order(right);
    if lo != ro {
        return lo.cmp(&ro);
    }

    match (&left.value, &right.value) {
        (TokenValue::Number(ln), TokenValue::Number(rn)) => ln.cmp(rn),
        (TokenValue::Qualifier(lq), TokenValue::Qualifier(rq)) => {
            compare_qualifiers(lq, left.is_transition, rq, right.is_transition)
        }
        // Mixed (should not happen after common_order check, but be safe).
        _ => Ordering::Equal,
    }
}

fn compare_qualifiers(lq: &str, lt: bool, rq: &str, rt: bool) -> Ordering {
    let lo = qualifier_order(lq, lt);
    let ro = qualifier_order(rq, rt);
    match (lo, ro) {
        (Some(l), Some(r)) => l.cmp(&r),
        (Some(l), None) if l < 6 => Ordering::Less,
        (None, Some(r)) if r < 6 => Ordering::Greater,
        _ => lq.cmp(rq),
    }
}

// ── Tokenizer ─────────────────────────────────────────────────────────────────

fn tokenize(version: &str) -> Vec<Token> {
    tokenize_preserving(version, false)
}

fn tokenize_preserving(version: &str, preserve_minor_zeroes: bool) -> Vec<Token> {
    let lower = version.to_lowercase();
    let chars: Vec<char> = lower.chars().collect();
    let n = chars.len();

    let mut raw_tokens: Vec<Token> = Vec::new();
    let mut current_prefix = Prefix::Other(String::new());
    let mut current_val = String::new();

    let mut i = 0;
    // Strip leading `v`/`V`.
    if n > 0 && (chars[0] == 'v') {
        current_prefix = Prefix::Other("v".to_owned());
        i = 1;
    }

    let yield_token = |prefix: &Prefix, val: &str, is_transition: bool, tokens: &mut Vec<Token>| {
        let actual = if val.is_empty() { "0" } else { val };
        let value = if actual.chars().all(|c| c.is_ascii_digit()) {
            TokenValue::Number(actual.parse().unwrap_or(0))
        } else {
            TokenValue::Qualifier(actual.to_owned())
        };
        tokens.push(Token {
            prefix: prefix.clone(),
            value,
            is_transition,
        });
    };

    while i < n {
        let c = chars[i];
        if c == '-' {
            yield_token(&current_prefix, &current_val, false, &mut raw_tokens);
            current_prefix = Prefix::Hyphen;
            current_val.clear();
        } else if c == '.' {
            yield_token(&current_prefix, &current_val, false, &mut raw_tokens);
            current_prefix = Prefix::Dot;
            current_val.clear();
        } else {
            let is_digit = c.is_ascii_digit();
            // Detect digit↔letter transition.
            if !current_val.is_empty() {
                let prev = current_val.chars().last().unwrap();
                let prev_digit = prev.is_ascii_digit();
                if prev_digit != is_digit {
                    yield_token(&current_prefix, &current_val, true, &mut raw_tokens);
                    current_prefix = Prefix::Hyphen;
                    current_val.clear();
                }
            }
            // If we just had a `v` prefix and the next char is not a digit,
            // absorb the prefix letter into the value.
            if current_val.is_empty()
                && let Prefix::Other(ref p) = current_prefix
                && !p.is_empty()
                && !is_digit
            {
                current_val = p.clone();
                current_prefix = Prefix::Other(String::new());
            }
            current_val.push(c);
        }
        i += 1;
    }
    yield_token(&current_prefix, &current_val, false, &mut raw_tokens);

    // Strip trailing null tokens (Maven's "strip trailing null" rule).
    let mut buf: Vec<Token> = Vec::new();
    let mut result: Vec<Token> = Vec::new();
    let mut leading_zero = true;

    for token in raw_tokens {
        let is_hyphen_or_qual = matches!(token.prefix, Prefix::Hyphen)
            || matches!(token.value, TokenValue::Qualifier(_));
        if is_hyphen_or_qual {
            buf.clear();
        }
        buf.push(token.clone());
        if !token.is_null() {
            leading_zero = false;
            result.append(&mut buf);
        } else if leading_zero || preserve_minor_zeroes {
            result.append(&mut buf);
        }
    }

    result
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Compare two Maven version strings.
///
/// Returns `Ordering::Less` if `left` is an older version than `right`,
/// `Ordering::Greater` if newer, and `Ordering::Equal` if equivalent.
pub fn compare(left: &str, right: &str) -> Ordering {
    let left_tokens = tokenize(left);
    let right_tokens = tokenize(right);
    let len = left_tokens.len().max(right_tokens.len());

    for idx in 0..len {
        let lt = left_tokens
            .get(idx)
            .cloned()
            .unwrap_or_else(|| Token::null_for(right_tokens.get(idx).unwrap()));
        let rt = right_tokens
            .get(idx)
            .cloned()
            .unwrap_or_else(|| Token::null_for(left_tokens.get(idx).unwrap()));
        let cmp = compare_tokens(&lt, &rt);
        if cmp != Ordering::Equal {
            return cmp;
        }
    }
    Ordering::Equal
}

/// Returns `true` when the version has no pre-release qualifier.
///
/// Stable versions: plain numeric (`1.2.3`), `.RELEASE`, `.GA`, `.Final`.
/// Pre-release: `-alpha`, `-beta`, `-M1`, `-RC1`, `-SNAPSHOT`, etc.
pub fn is_stable(version: &str) -> bool {
    if !is_version_str(version) {
        return false;
    }
    let tokens = tokenize(version);
    for token in &tokens {
        if let TokenValue::Qualifier(q) = &token.value {
            match qualifier_order(q, token.is_transition) {
                Some(order) if order < 6 => return false,
                _ => {}
            }
        }
    }
    true
}

/// Update summary produced by [`maven_update_summary`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MavenUpdateSummary {
    pub current_version: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Produce an update summary by comparing `current` against `latest`.
///
/// `update_available` is `true` when:
/// - `latest` is `Some` and non-empty, and
/// - `latest` is strictly newer than `current` by Maven version ordering.
pub fn maven_update_summary(current: &str, latest: Option<&str>) -> MavenUpdateSummary {
    let update_available = latest
        .filter(|l| !l.is_empty() && !current.is_empty())
        .is_some_and(|l| compare(l, current) == Ordering::Greater);

    MavenUpdateSummary {
        current_version: current.to_owned(),
        latest: latest.map(|s| s.to_owned()),
        update_available,
    }
}

// ── Token-to-string ───────────────────────────────────────────────────────────

fn tokens_to_str(tokens: &[Token]) -> String {
    let mut result = String::new();
    for token in tokens {
        match &token.prefix {
            Prefix::Dot => result.push('.'),
            Prefix::Hyphen => result.push('-'),
            Prefix::Other(s) => result.push_str(s),
        }
        match &token.value {
            TokenValue::Number(n) => result.push_str(&n.to_string()),
            TokenValue::Qualifier(q) => result.push_str(q),
        }
    }
    result
}

// ── Range helpers ─────────────────────────────────────────────────────────────

fn is_version_str(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    if !s
        .chars()
        .all(|c| matches!(c, '-' | '.' | '_' | '+' | 'a'..='z' | 'A'..='Z' | '0'..='9'))
    {
        return false;
    }
    let first = s.chars().next().unwrap();
    if first == '.' || first == '-' {
        return false;
    }
    let last = s.chars().last().unwrap();
    if last == '.' || last == '-' {
        return false;
    }
    let lower = s.to_lowercase();
    if lower == "latest" || lower == "release" {
        return false;
    }
    !tokenize(s).is_empty()
}

fn coerce_range_value(prev: &str, next: &str) -> String {
    let prev_tokens = tokenize_preserving(prev, true);
    let next_tokens = tokenize_preserving(next, true);
    let take = prev_tokens.len().min(next_tokens.len());
    let mut result_tokens: Vec<Token> = next_tokens.into_iter().take(take).collect();
    let align = prev_tokens.len().saturating_sub(result_tokens.len());
    if align > 0 {
        let start = prev_tokens.len() - align;
        result_tokens.extend_from_slice(&prev_tokens[start..]);
    }
    tokens_to_str(&result_tokens)
}

fn increment_range_value(value: &str) -> String {
    let mut tokens = tokenize(value);
    if tokens.is_empty() {
        return value.to_owned();
    }
    let last = tokens.last_mut().unwrap();
    if let TokenValue::Number(n) = &mut last.value {
        *n += 1;
        let intermediate = tokens_to_str(&tokens);
        coerce_range_value(value, &intermediate)
    } else {
        value.to_owned()
    }
}

// ── Range types ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangePointType {
    Including,
    Excluding,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeInterval {
    pub left_type: RangePointType,
    pub left_value: Option<String>,
    pub left_bracket: String,
    pub right_type: RangePointType,
    pub right_value: Option<String>,
    pub right_bracket: String,
}

struct IntervalBuilder {
    left_type: Option<RangePointType>,
    left_value: String,
    left_bracket: String,
    right_type: Option<RangePointType>,
    right_value: String,
    right_bracket: String,
}

impl IntervalBuilder {
    fn empty() -> Self {
        Self {
            left_type: None,
            left_value: String::new(),
            left_bracket: String::new(),
            right_type: None,
            right_value: String::new(),
            right_bracket: String::new(),
        }
    }
    fn is_open(&self) -> bool {
        self.left_type.is_some()
    }
}

pub fn parse_range(range_str: &str) -> Option<Vec<RangeInterval>> {
    let parts: Vec<&str> = range_str.split(',').map(|s| s.trim()).collect();
    let mut raw_ranges: Vec<IntervalBuilder> = Vec::new();
    let mut interval = IntervalBuilder::empty();
    let mut failed = false;

    for sub_str in &parts {
        if failed {
            break;
        }
        if !interval.is_open() {
            if sub_str.starts_with('[') && sub_str.ends_with(']') {
                let ver = &sub_str[1..sub_str.len() - 1];
                raw_ranges.push(IntervalBuilder {
                    left_type: Some(RangePointType::Including),
                    left_value: ver.to_owned(),
                    left_bracket: "[".to_owned(),
                    right_type: Some(RangePointType::Including),
                    right_value: ver.to_owned(),
                    right_bracket: "]".to_owned(),
                });
            } else if let Some(rest) = sub_str.strip_prefix('[') {
                interval.left_type = Some(RangePointType::Including);
                interval.left_value = rest.to_owned();
                interval.left_bracket = "[".to_owned();
            } else if sub_str.starts_with('(') || sub_str.starts_with(']') {
                interval.left_type = Some(RangePointType::Excluding);
                interval.left_value = sub_str[1..].to_owned();
                interval.left_bracket = sub_str[..1].to_owned();
            } else {
                failed = true;
            }
        } else if let Some(prefix) = sub_str.strip_suffix(']') {
            interval.right_type = Some(RangePointType::Including);
            interval.right_value = prefix.to_owned();
            interval.right_bracket = "]".to_owned();
            raw_ranges.push(std::mem::replace(&mut interval, IntervalBuilder::empty()));
        } else if sub_str.ends_with(')') || sub_str.ends_with('[') {
            let bracket = if sub_str.ends_with(')') { ")" } else { "[" };
            interval.right_type = Some(RangePointType::Excluding);
            interval.right_value = sub_str[..sub_str.len() - 1].to_owned();
            interval.right_bracket = bracket.to_owned();
            raw_ranges.push(std::mem::replace(&mut interval, IntervalBuilder::empty()));
        } else {
            failed = true;
        }
    }

    if failed || interval.is_open() || raw_ranges.is_empty() {
        return None;
    }

    let last_idx = raw_ranges.len() - 1;
    let mut prev_value: Option<String> = None;
    let mut result: Vec<RangeInterval> = Vec::new();

    for (idx, range) in raw_ranges.into_iter().enumerate() {
        let left_type = range.left_type.unwrap();
        let right_type = range.right_type.unwrap();
        let lv = range.left_value.as_str();
        let rv = range.right_value.as_str();

        if idx == 0 && lv.is_empty() {
            if left_type == RangePointType::Excluding && is_version_str(rv) {
                prev_value = Some(rv.to_owned());
                result.push(RangeInterval {
                    left_type,
                    left_value: None,
                    left_bracket: range.left_bracket,
                    right_type,
                    right_value: Some(rv.to_owned()),
                    right_bracket: range.right_bracket,
                });
                continue;
            }
            return None;
        }
        if idx == last_idx && rv.is_empty() {
            if right_type == RangePointType::Excluding && is_version_str(lv) {
                if let Some(ref pv) = prev_value
                    && compare(pv, lv) == Ordering::Greater
                {
                    return None;
                }
                result.push(RangeInterval {
                    left_type,
                    left_value: Some(lv.to_owned()),
                    left_bracket: range.left_bracket,
                    right_type,
                    right_value: None,
                    right_bracket: range.right_bracket,
                });
                continue;
            }
            return None;
        }
        if is_version_str(lv) && is_version_str(rv) {
            if compare(lv, rv) == Ordering::Greater {
                return None;
            }
            if let Some(ref pv) = prev_value
                && compare(pv, lv) == Ordering::Greater
            {
                return None;
            }
            prev_value = Some(rv.to_owned());
            result.push(RangeInterval {
                left_type,
                left_value: Some(lv.to_owned()),
                left_bracket: range.left_bracket,
                right_type,
                right_value: Some(rv.to_owned()),
                right_bracket: range.right_bracket,
            });
            continue;
        }
        return None;
    }
    Some(result)
}

pub fn range_to_str(ranges: Option<&[RangeInterval]>) -> Option<String> {
    let ranges = ranges?;
    if ranges.len() == 1 {
        let r = &ranges[0];
        if r.left_value == r.right_value && r.left_bracket == "[" && r.right_bracket == "]" {
            let val = r.left_value.as_deref().unwrap_or("");
            return Some(format!("[{val}]"));
        }
    }
    let intervals: Vec<String> = ranges
        .iter()
        .map(|r| {
            let lv = r.left_value.as_deref().unwrap_or("");
            let rv = r.right_value.as_deref().unwrap_or("");
            format!("{}{},{}{}", r.left_bracket, lv, rv, r.right_bracket)
        })
        .collect();
    Some(intervals.join(","))
}

pub fn auto_extend_maven_range(current: &str, new_version: &str) -> String {
    let Some(mut range) = parse_range(current) else {
        return current.to_owned();
    };

    let is_point = range.len() == 1 && {
        let r = &range[0];
        r.left_type == RangePointType::Including
            && r.right_type == RangePointType::Including
            && r.left_value == r.right_value
    };
    if is_point {
        return format!("[{new_version}]");
    }

    let interval_idx = range.iter().rposition(|r| {
        r.right_value.is_none()
            || (r.right_type == RangePointType::Including
                && compare(r.right_value.as_deref().unwrap(), new_version) == Ordering::Less)
            || (r.right_type == RangePointType::Excluding
                && compare(r.right_value.as_deref().unwrap(), new_version) != Ordering::Greater)
    });

    let Some(idx) = interval_idx else {
        return current.to_owned();
    };

    let left_value = range[idx].left_value.clone();
    let right_value = range[idx].right_value.clone();

    if left_value.is_some()
        && right_value.is_some()
        && increment_range_value(left_value.as_deref().unwrap()) == *right_value.as_ref().unwrap()
    {
        if compare(new_version, left_value.as_deref().unwrap()) != Ordering::Less {
            let new_left = coerce_range_value(left_value.as_deref().unwrap(), new_version);
            let new_right = increment_range_value(&new_left);
            range[idx].left_value = Some(new_left);
            range[idx].right_value = Some(new_right);
        }
    } else if let Some(ref rv) = right_value {
        if range[idx].right_type == RangePointType::Including {
            let rv_tokens = tokenize(rv);
            if let Some(last_tok) = rv_tokens.last() {
                if let TokenValue::Number(_) = last_tok.value {
                    range[idx].right_value = Some(coerce_range_value(rv, new_version));
                } else {
                    range[idx].right_value = Some(new_version.to_owned());
                }
            }
        } else {
            let coerced = coerce_range_value(rv, new_version);
            range[idx].right_value = Some(increment_range_value(&coerced));
        }
    } else if let Some(ref lv) = left_value {
        range[idx].left_value = Some(coerce_range_value(lv, new_version));
    }

    range_to_str(Some(&range)).unwrap_or_else(|| current.to_owned())
}

pub fn is_valid(s: &str) -> bool {
    is_version_str(s) || parse_range(s).is_some()
}

pub fn is_version(s: &str) -> bool {
    is_version_str(s)
}

pub fn get_major(version: &str) -> Option<i64> {
    if !is_version_str(version) {
        return None;
    }
    let tokens = tokenize(version);
    match tokens.first() {
        Some(t) => {
            if let TokenValue::Number(n) = t.value {
                Some(n as i64)
            } else {
                Some(0)
            }
        }
        None => None,
    }
}

pub fn get_minor(version: &str) -> Option<i64> {
    if !is_version_str(version) {
        return None;
    }
    let tokens = tokenize(version);
    match tokens.get(1) {
        Some(t) => {
            if let TokenValue::Number(n) = t.value {
                Some(n as i64)
            } else {
                Some(0)
            }
        }
        None => Some(0),
    }
}

pub fn get_patch(version: &str) -> Option<i64> {
    if !is_version_str(version) {
        return None;
    }
    let tokens = tokenize(version);
    let minor = tokens.get(1);
    let patch = tokens.get(2);
    match (minor, patch) {
        (Some(m), Some(p))
            if matches!(m.value, TokenValue::Number(_))
                && matches!(p.value, TokenValue::Number(_)) =>
        {
            if let TokenValue::Number(n) = p.value {
                Some(n as i64)
            } else {
                Some(0)
            }
        }
        _ => Some(0),
    }
}

pub fn matches_range(version: &str, range: &str) -> bool {
    if range.is_empty() {
        return false;
    }
    if is_version_str(range) {
        return compare(version, range) == Ordering::Equal;
    }
    let Some(ranges) = parse_range(range) else {
        return false;
    };
    for r in &ranges {
        let left_ok = match &r.left_value {
            None => true,
            Some(lv) => {
                if r.left_type == RangePointType::Excluding {
                    compare(lv, version) == Ordering::Less
                } else {
                    compare(lv, version) != Ordering::Greater
                }
            }
        };
        let right_ok = match &r.right_value {
            None => true,
            Some(rv) => {
                if r.right_type == RangePointType::Excluding {
                    compare(version, rv) == Ordering::Less
                } else {
                    compare(version, rv) != Ordering::Greater
                }
            }
        };
        if left_ok && right_ok {
            return true;
        }
    }
    false
}

pub fn is_greater_than(a: &str, b: &str) -> bool {
    compare(a, b) == Ordering::Greater
}

pub fn get_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    versions.iter().fold(None, |result, &v| {
        if matches_range(v, range) {
            match result {
                None => Some(v),
                Some(r) if is_greater_than(v, r) => Some(v),
                _ => result,
            }
        } else {
            result
        }
    })
}

pub fn get_new_value(
    current_value: &str,
    range_strategy: Option<&str>,
    new_version: &str,
) -> String {
    if is_version_str(current_value) || range_strategy == Some("pin") {
        return new_version.to_owned();
    }
    auto_extend_maven_range(current_value, new_version)
}

pub fn is_subversion(major_version: &str, minor_version: &str) -> bool {
    let major_tokens = tokenize(major_version);
    let minor_tokens = tokenize(minor_version);
    for (i, major_tok) in major_tokens.iter().enumerate() {
        let null_tok = Token::null_for(major_tok);
        let minor_tok = minor_tokens.get(i).unwrap_or(&null_tok);
        if compare_tokens(major_tok, minor_tok) != Ordering::Equal {
            return false;
        }
    }
    true
}

/// Returns the lowercase value of the last token if it is a qualifier (string), otherwise `None`.
pub fn last_qualifier(version: &str) -> Option<String> {
    let tokens = tokenize(version);
    match tokens.last() {
        Some(t) if matches!(t.value, TokenValue::Qualifier(_)) => {
            if let TokenValue::Qualifier(q) = &t.value {
                Some(q.to_lowercase())
            } else {
                None
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cmp(a: &str, b: &str) -> Ordering {
        compare(a, b)
    }

    #[test]
    fn numeric_versions() {
        assert_eq!(cmp("1.0.0", "1.0.1"), Ordering::Less);
        assert_eq!(cmp("1.0.1", "1.0.0"), Ordering::Greater);
        assert_eq!(cmp("1.0.0", "1.0.0"), Ordering::Equal);
        assert_eq!(cmp("2.0.0", "1.9.9"), Ordering::Greater);
        assert_eq!(cmp("1.10.0", "1.9.0"), Ordering::Greater);
    }

    #[test]
    fn qualifier_ordering() {
        // alpha < beta < milestone < rc < release < sp
        assert_eq!(cmp("1.0-alpha1", "1.0-beta1"), Ordering::Less);
        assert_eq!(cmp("1.0-beta1", "1.0-M1"), Ordering::Less);
        assert_eq!(cmp("1.0-M1", "1.0-RC1"), Ordering::Less);
        assert_eq!(cmp("1.0-RC1", "1.0"), Ordering::Less);
        assert_eq!(cmp("1.0", "1.0-SP1"), Ordering::Less);
    }

    #[test]
    fn release_equivalents() {
        // final, ga, release, and empty are all equivalent to release
        assert_eq!(cmp("1.0.RELEASE", "1.0"), Ordering::Equal);
        assert_eq!(cmp("1.0.GA", "1.0"), Ordering::Equal);
        assert_eq!(cmp("1.0.Final", "1.0"), Ordering::Equal);
    }

    #[test]
    fn snapshot_older_than_release() {
        assert_eq!(cmp("5.3.28-SNAPSHOT", "5.3.28"), Ordering::Less);
        assert_eq!(cmp("5.3.28", "5.3.28-SNAPSHOT"), Ordering::Greater);
    }

    #[test]
    fn v_prefix_stripped() {
        assert_eq!(cmp("v1.0.0", "1.0.0"), Ordering::Equal);
        assert_eq!(cmp("v1.0.0", "v1.0.1"), Ordering::Less);
    }

    #[test]
    fn transition_short_qualifiers() {
        // 1a → 1.alpha, 1b → 1.beta, 1m → 1.milestone
        assert_eq!(cmp("1.0a1", "1.0b1"), Ordering::Less);
        assert_eq!(cmp("1.0b1", "1.0m1"), Ordering::Less);
        assert_eq!(cmp("1.0m1", "1.0"), Ordering::Less);
    }

    #[test]
    fn leading_zeros_ignored() {
        assert_eq!(cmp("1.0.0", "1"), Ordering::Equal);
        assert_eq!(cmp("1.0", "1.0.0"), Ordering::Equal);
    }

    #[test]
    fn spring_versions() {
        assert_eq!(cmp("5.3.28", "5.3.30"), Ordering::Less);
        assert_eq!(cmp("6.0.0-M1", "6.0.0-RC1"), Ordering::Less);
        assert_eq!(cmp("6.0.0-RC1", "6.0.0"), Ordering::Less);
    }

    #[test]
    fn is_stable_checks() {
        assert!(is_stable("1.2.3"));
        assert!(is_stable("5.3.28"));
        assert!(is_stable("1.0.RELEASE"));
        assert!(is_stable("1.0.GA"));
        assert!(!is_stable("1.0-SNAPSHOT"));
        assert!(!is_stable("1.0-alpha1"));
        assert!(!is_stable("1.0-M1"));
        assert!(!is_stable("1.0-RC1"));
    }

    #[test]
    fn maven_update_summary_pinned() {
        let s = maven_update_summary("5.3.28", Some("5.3.30"));
        assert!(s.update_available);
        assert_eq!(s.latest.as_deref(), Some("5.3.30"));

        let s2 = maven_update_summary("5.3.30", Some("5.3.30"));
        assert!(!s2.update_available);

        let s3 = maven_update_summary("5.3.28", Some("5.3.28-SNAPSHOT"));
        // SNAPSHOT < release → no update
        assert!(!s3.update_available);
    }

    #[test]
    fn maven_update_summary_none() {
        let s = maven_update_summary("1.0", None);
        assert!(!s.update_available);
        assert!(s.latest.is_none());
    }

    // Ported: "$x == $y" — maven/compare.spec.ts line 15
    #[test]
    fn compare_equals_matches_renovate_maven_compare_spec() {
        let cases: &[(&str, &str)] = &[
            ("1", "1"),
            ("1", "1.0"),
            ("1", "1.0.0"),
            ("1.0", "1.0.0"),
            ("1", "1-0"),
            ("1", "1.0-0"),
            ("1.0", "1.0-0"),
            ("1a", "1-a"),
            ("1a", "1.0-a"),
            ("1a", "1.0.0-a"),
            ("1.0a", "1-a"),
            ("1.0.0a", "1-a"),
            ("1x", "1-x"),
            ("1x", "1.0-x"),
            ("1x", "1.0.0-x"),
            ("1.0x", "1-x"),
            ("1.0.0x", "1-x"),
            ("1ga", "1"),
            ("1release", "1"),
            ("1final", "1"),
            ("1cr", "1rc"),
            ("1a1", "1-alpha-1"),
            ("1b2", "1-beta-2"),
            ("1m3", "1-milestone-3"),
            ("1X", "1x"),
            ("1A", "1a"),
            ("1B", "1b"),
            ("1M", "1m"),
            ("1Ga", "1"),
            ("1GA", "1"),
            ("1RELEASE", "1"),
            ("1release", "1"),
            ("1RELeaSE", "1"),
            ("1Final", "1"),
            ("1FinaL", "1"),
            ("1FINAL", "1"),
            ("1Cr", "1Rc"),
            ("1cR", "1rC"),
            ("1m3", "1Milestone3"),
            ("1m3", "1MileStone3"),
            ("1m3", "1MILESTONE3"),
            ("2.0-0", "2.0"),
            ("1.0.0", "1"),
            ("1-a1", "1-alpha-1"),
            ("1-b1", "1-beta-1"),
            ("1.0.0", "1.ga"),
            ("1-ga", "1.ga"),
            ("1.final", "1.0"),
            ("1", "1.0"),
            ("1.", "1-"),
            ("1.0.0-0.0.0", "1-final"),
            ("1-1.foo-bar1baz-.1", "1-1.foo-bar-1-baz-0.1"),
            ("1.0ALPHA1", "1.0-a1"),
            ("1.0Alpha1", "1.0-a1"),
            ("1.0AlphA1", "1.0-a1"),
            ("1.0BETA1", "1.0-b1"),
            ("1.0MILESTONE1", "1.0-m1"),
            ("1.0RC1", "1.0-cr1"),
            ("1.0GA", "1.0"),
            ("1.0FINAL", "1.0"),
            ("1.0-SNAPSHOT", "1-snapshot"),
            ("1.0alpha1", "1.0-a1"),
            ("1.0alpha-1", "1.0-a1"),
            ("1.0beta1", "1.0-b1"),
            ("1.0beta-1", "1.0-b1"),
            ("1.0milestone1", "1.0-m1"),
            ("1.0milestone-1", "1.0-m1"),
            ("1.0rc1", "1.0-cr1"),
            ("1.0rc-1", "1.0-cr1"),
            ("1.0ga", "1.0"),
            ("1-0.ga", "1.0"),
            ("1.0-final", "1.0"),
            ("1-0-ga", "1.0"),
            ("1-0-final", "1-0"),
            ("1-0", "1.0"),
            ("0.0-1552", "0.0-1552"),
            ("5.0.7", "5.0.7.RELEASE"),
            ("Hoxton.RELEASE", "hoxton"),
            ("Hoxton.SR1", "hoxton.sr-1"),
            ("1_5ea", "1.0_5ea"),
            ("1.foo", "1-foo"),
            ("1.x", "1-x"),
        ];
        for (x, y) in cases {
            assert_eq!(compare(x, y), Ordering::Equal, "{x} == {y}");
            assert_eq!(compare(y, x), Ordering::Equal, "{y} == {x}");
        }
    }

    // Ported: "$x < $y" — maven/compare.spec.ts line 106
    #[test]
    fn compare_ordering_matches_renovate_maven_compare_spec() {
        let cases: &[(&str, &str)] = &[
            ("1", "2"),
            ("1.5", "2"),
            ("1", "2.5"),
            ("1.0", "1.1"),
            ("1.1", "1.2"),
            ("1.0.0", "1.1"),
            ("1.0.1", "1.1"),
            ("1.1", "1.2.0"),
            ("1.0-alpha-1", "1.0"),
            ("1.0-alpha-1", "1.0-alpha-2"),
            ("1.0-alpha-1", "1.0-beta-1"),
            ("1.0-beta-1", "1.0-SNAPSHOT"),
            ("1.0-SNAPSHOT", "1.0"),
            ("1.0-alpha-1-SNAPSHOT", "1.0-alpha-1"),
            ("1.0", "1.0-1"),
            ("1.0-1", "1.0-2"),
            ("1.0.0", "1.0-1"),
            ("2.0-1", "2.0.1"),
            ("2.0.1-klm", "2.0.1-lmn"),
            ("2.0.1", "2.0.1-xyz"),
            ("2.0.1", "2.0.1-123"),
            ("2.0.1-xyz", "2.0.1-123"),
            ("1", "2"),
            ("1.5", "2"),
            ("1", "2.5"),
            ("1.0", "1.1"),
            ("1.1", "1.2"),
            ("1.0.0", "1.1"),
            ("1.1", "1.2.0"),
            ("1.1.2.alpha1", "1.1.2"),
            ("1.1.2.alpha1", "1.1.2.beta1"),
            ("1.1.2.beta1", "1.2"),
            ("1.0-alpha-1", "1.0"),
            ("1.0-alpha-1", "1.0-alpha-2"),
            ("1.0-alpha-2", "1.0-alpha-15"),
            ("1.0-alpha-1", "1.0-beta-1"),
            ("1.0-beta-1", "1.0-SNAPSHOT"),
            ("1.0-SNAPSHOT", "1.0"),
            ("1.0-alpha-1-SNAPSHOT", "1.0-alpha-1"),
            ("1.0", "1.0-1"),
            ("1.0-1", "1.0-2"),
            ("2.0", "2.0-1"),
            ("2.0.0", "2.0-1"),
            ("2.0-1", "2.0.1"),
            ("2.0.1-klm", "2.0.1-lmn"),
            ("2.0.1", "2.0.1-xyz"),
            ("2.0.1-xyz-1", "2.0.1-1-xyz"),
            ("2.0.1", "2.0.1-123"),
            ("2.0.1-xyz", "2.0.1-123"),
            ("1.2.3-10000000000", "1.2.3-10000000001"),
            ("1.2.3-1", "1.2.3-10000000001"),
            ("2.3.0-v200706262000", "2.3.0-v200706262130"),
            (
                "2.0.0.v200706041905-7C78EK9E_EkMNfNOd2d8qq",
                "2.0.0.v200706041906-7C78EK9E_EkMNfNOd2d8qq",
            ),
            ("1.0-RC1", "1.0-SNAPSHOT"),
            ("1.0-rc1", "1.0-SNAPSHOT"),
            ("1.0-rc-1", "1.0-SNAPSHOT"),
            ("1", "1.1"),
            ("1", "2"),
            ("1-snapshot", "1"),
            ("1.2.3-snap1", "1.2.3-snap2"),
            ("1", "1-sp"),
            ("1-foo2", "1-foo10"),
            ("1-m1", "1-milestone-2"),
            ("1-foo", "1-1"),
            ("1-alpha.1", "1-beta.1"),
            ("1-1", "1.1"),
            ("1-ga", "1-sp"),
            ("1-ga.1", "1-sp.1"),
            ("1-sp-1", "1-ga-1"),
            ("1-cr1", "1"),
            ("0.0-1552", "1.10.520"),
            ("0.0.1", "999"),
            ("1.3-RC1-groovy-2.5", "1.3-groovy-2.5"),
            ("1-milestone", "1-snapshot"),
            ("1-abc", "1-xyz"),
            ("Hoxton.RELEASE", "Hoxton.SR1"),
            ("2.0", "2.0-PFD2"),
            ("2.0", "2.0.SP1"),
            ("2.0-PFD2", "2.0.SP1"),
            ("1.3.9", "1.3.9.fix-log4j2"),
            ("1-0.alpha", "1"),
            ("1-0.beta", "1"),
            ("1-0.alpha", "1-0.beta"),
            ("1_5ea", "1_c3b"),
            ("1_c3b", "2"),
            ("17.0.5", "17.0.5+8"),
        ];
        for (x, y) in cases {
            assert_eq!(compare(x, y), Ordering::Less, "{x} < {y}");
            assert_eq!(compare(y, x), Ordering::Greater, "{y} > {x}");
        }
    }

    // Ported: "$qualifier" — maven/compare.spec.ts line 203
    #[test]
    fn qualifier_mng7644_matches_renovate_maven_compare_spec() {
        let qualifiers = [
            "abc",
            "alpha",
            "a",
            "beta",
            "b",
            "def",
            "milestone",
            "m",
            "RC",
        ];
        for q in qualifiers {
            let dot1 = format!("1.0.0.{q}1");
            let hyp2 = format!("1.0.0-{q}2");
            assert_eq!(compare(&dot1, &hyp2), Ordering::Less, "{dot1} < {hyp2}");

            let hyp = format!("2-{q}");
            let dot = format!("2.0.{q}");
            let dotdot = format!("2.0.0.{q}");
            assert_eq!(compare(&hyp, &dot), Ordering::Equal, "2-{q} == 2.0.{q}");
            assert_eq!(
                compare(&hyp, &dotdot),
                Ordering::Equal,
                "2-{q} == 2.0.0.{q}"
            );
            assert_eq!(
                compare(&dot, &dotdot),
                Ordering::Equal,
                "2.0.{q} == 2.0.0.{q}"
            );
        }
    }

    // Ported: "isSubversion("$majorVersion", "$minorVersion") === $expected" — maven/compare.spec.ts line 226
    #[test]
    fn is_subversion_matches_renovate_maven_compare_spec() {
        assert!(is_subversion("1.2.3", "1.2.3"));
        assert!(!is_subversion("1.2.3", "1.0.0"));
        assert!(is_subversion("2.0.0", "2.0.1"));
        assert!(is_subversion("3.1.0", "3.01.00"));
        assert!(!is_subversion("4.0.0", ""));
        assert!(!is_subversion("5.0.0", "4.5.2"));
        assert!(is_subversion("6.0.0", "6.0.0-beta"));
        assert!(!is_subversion("invalid.version", ""));
        assert!(!is_subversion("", "1.2.3"));
        assert!(is_subversion("v1.2.3", "1.2.3"));
        assert!(is_subversion("v1.2.3", "v1.2.3"));
    }

    // Ported: "$x == $y" — maven/compare.spec.ts line 463
    #[test]
    fn compare_nonstandard_equals_matches_renovate_maven_compare_spec() {
        let cases: &[(&str, &str)] = &[
            ("1-ga-1", "1-1"),
            ("1.0-SNAP", "1-snapshot"),
            ("1.0rc", "1.0-preview"),
            ("v1.2.3", "1.2.3"),
            ("v0.0-1552", "0.0-1552"),
            ("v0.0.1", "0.0.1"),
        ];
        for (x, y) in cases {
            assert_eq!(compare(x, y), Ordering::Equal, "{x} == {y}");
            assert_eq!(compare(y, x), Ordering::Equal, "{y} == {x}");
        }
    }

    // Ported: "$x < $y" — maven/compare.spec.ts line 478
    #[test]
    fn compare_nonstandard_ordering_matches_renovate_maven_compare_spec() {
        let cases: &[(&str, &str)] = &[("1-snap", "1"), ("1-preview", "1-snapshot")];
        for (x, y) in cases {
            assert_eq!(compare(x, y), Ordering::Less, "{x} < {y}");
            assert_eq!(compare(y, x), Ordering::Greater, "{y} > {x}");
        }
    }

    // Ported: "filters out incorrect range: $input" — maven/compare.spec.ts line 490
    #[test]
    fn parse_range_filters_invalid_matches_renovate_maven_compare_spec() {
        let invalid = [
            "1.2.3-SNAPSHOT",
            "[]",
            "[,]",
            "(",
            "[",
            ",",
            "[1.0",
            "1.0]",
            "[1.0],",
            ",[1.0]",
            "(,1.1),(1.0,)",
            "(0,1.1),(1.0,2.0)",
            "(0,1.1),(,2.0)",
            "(,1.0],,[1.2,)",
            "(,1.0],[1.2,),",
            "[1.5,]",
            "[2.0,1.0)",
            "[1.2,1.3],1.4",
            "[1.2,,1.3]",
            "[1.3,1.2]",
            "[1,[2,3],4]",
            "[,1.0]",
            "[,1.0],[,1.0]",
        ];
        for input in invalid {
            let r = parse_range(input);
            assert!(r.is_none(), "expected null for: {input}");
            assert!(range_to_str(r.as_deref()).is_none());
        }
    }

    // Ported: "parseRange("$input")" — maven/compare.spec.ts line 521
    #[test]
    #[allow(clippy::type_complexity)]
    fn parse_range_valid_matches_renovate_maven_compare_spec() {
        use RangePointType::{Excluding, Including};
        let cases: &[(
            &str,
            RangePointType,
            Option<&str>,
            &str,
            RangePointType,
            Option<&str>,
            &str,
        )] = &[
            (
                "[1.0]",
                Including,
                Some("1.0"),
                "[",
                Including,
                Some("1.0"),
                "]",
            ),
            ("(,1.0]", Excluding, None, "(", Including, Some("1.0"), "]"),
            ("(, 1.0]", Excluding, None, "(", Including, Some("1.0"), "]"),
            (
                "[1.2,1.3]",
                Including,
                Some("1.2"),
                "[",
                Including,
                Some("1.3"),
                "]",
            ),
            (
                "[1.2, 1.3]",
                Including,
                Some("1.2"),
                "[",
                Including,
                Some("1.3"),
                "]",
            ),
            (
                "[1.0,2.0)",
                Including,
                Some("1.0"),
                "[",
                Excluding,
                Some("2.0"),
                ")",
            ),
            ("[1.5,)", Including, Some("1.5"), "[", Excluding, None, ")"),
            ("[1.5, )", Including, Some("1.5"), "[", Excluding, None, ")"),
        ];
        for (input, lt, lv, lb, rt, rv, rb) in cases {
            let result = parse_range(input).expect(input);
            assert_eq!(result.len(), 1, "len for {input}");
            let r = &result[0];
            assert_eq!(r.left_type, *lt, "left_type for {input}");
            assert_eq!(r.left_value.as_deref(), *lv, "left_value for {input}");
            assert_eq!(r.left_bracket, *lb, "left_bracket for {input}");
            assert_eq!(r.right_type, *rt, "right_type for {input}");
            assert_eq!(r.right_value.as_deref(), *rv, "right_value for {input}");
            assert_eq!(r.right_bracket, *rb, "right_bracket for {input}");
            let expected_str = input.replace(' ', "");
            assert_eq!(
                range_to_str(Some(&result)),
                Some(expected_str),
                "rangeToStr for {input}"
            );
        }
    }

    // Ported: "uses same function module export and api object" — maven/index.spec.ts line 7
    // Not applicable: tests TypeScript named-export identity (isValid === _isValid); no Rust equivalent

    // Ported: "isValid("$version") === $expected" — maven/index.spec.ts line 11
    #[test]
    fn is_valid_matches_renovate_maven_index_spec() {
        assert!(is_valid("1.0.0"));
        assert!(is_valid("[1.0.0]"));
        assert!(is_valid("17.0.5+8"));
        assert!(is_valid("[1.12.6,1.18.6]"));
        assert!(is_valid("(,1.0]"));
        assert!(is_valid("[1.0,)"));
        assert!(is_valid("[1.0,2.0)"));
        assert!(is_valid("(1.0,2.0]"));
        assert!(is_valid("],1.0]"));
        assert!(is_valid("[1.0,["));
        assert!(is_valid("[1.0,2.0],[3.0,4.0)"));
        assert!(!is_valid("[,1.0]"));
        assert!(!is_valid("[1.0,]"));
        assert!(!is_valid("[2.0,1.0)"));
    }

    // Ported: "isVersion("$version") === $expected" — maven/index.spec.ts line 32
    #[test]
    fn is_version_matches_renovate_maven_index_spec() {
        assert!(!is_version(""));
        assert!(is_version("1.0.0"));
        assert!(is_version("0"));
        assert!(is_version("0.1-2-sp"));
        assert!(is_version("1-final"));
        assert!(is_version("1-foo"));
        assert!(is_version("v1.0.0"));
        assert!(is_version("x1.0.0"));
        assert!(is_version("2.1.1.RELEASE"));
        assert!(is_version("Greenwich.SR1"));
        assert!(is_version("v1.0.0_2"));
        assert!(is_version("1.1.1-20_62b10c"));
        assert!(!is_version(".1"));
        assert!(!is_version("1."));
        assert!(!is_version("-1"));
        assert!(!is_version("1-"));
        assert!(!is_version("[1.12.6,1.18.6]"));
        assert!(!is_version("RELEASE"));
        assert!(!is_version("release"));
        assert!(!is_version("LATEST"));
        assert!(!is_version("latest"));
        assert!(is_version("foobar"));
    }

    // Ported: "isStable("$version") === $expected" — maven/index.spec.ts line 60
    #[test]
    fn is_stable_index_matches_renovate_maven_index_spec() {
        assert!(!is_stable(""));
        assert!(is_stable("foobar"));
        assert!(is_stable("final"));
        assert!(is_stable("1"));
        assert!(is_stable("1.2"));
        assert!(is_stable("1.2.3"));
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
        assert!(is_stable("Hoxton.RELEASE"));
        assert!(is_stable("Hoxton.SR"));
        assert!(is_stable("Hoxton.SR1"));
    }

    // Ported: '"$input" is represented as [$major, $minor, $patch]' — maven/index.spec.ts line 89
    #[test]
    #[allow(clippy::type_complexity)]
    fn get_major_minor_patch_matches_renovate_maven_index_spec() {
        let cases: &[(&str, Option<i64>, Option<i64>, Option<i64>)] = &[
            ("", None, None, None),
            ("1", Some(1), Some(0), Some(0)),
            ("1.2", Some(1), Some(2), Some(0)),
            ("1.2.3", Some(1), Some(2), Some(3)),
            ("v1.2.3", Some(1), Some(2), Some(3)),
            ("1rc42", Some(1), Some(0), Some(0)),
            ("1-rc42", Some(1), Some(0), Some(0)),
            ("1-rc42-1", Some(1), Some(0), Some(0)),
            ("1-rc10", Some(1), Some(0), Some(0)),
            ("1.2.3.4", Some(1), Some(2), Some(3)),
        ];
        for (input, major, minor, patch) in cases {
            assert_eq!(get_major(input), *major, "getMajor({input})");
            assert_eq!(get_minor(input), *minor, "getMinor({input})");
            assert_eq!(get_patch(input), *patch, "getPatch({input})");
        }
    }

    // Ported: 'matches("$version", "$range") === $expected' — maven/index.spec.ts line 111
    #[test]
    fn matches_range_matches_renovate_maven_index_spec() {
        let cases: &[(&str, &str, bool)] = &[
            ("0", "[0,1]", true),
            ("1", "[0,1]", true),
            ("0", "(0,1)", false),
            ("1", "(0,1)", false),
            ("1", "(0,2)", true),
            ("1", "[0,2]", true),
            ("1", "(,1]", true),
            ("1", "(,1)", false),
            ("1", "[1,)", true),
            ("1", "(1,)", false),
            ("1", "(,1),(1,)", false),
            ("1", "(0,1),(1,2)", false),
            ("1.0.0.RC9.2", "(,1.0.0.RC9.2),(1.0.0.RC9.2,)", false),
            ("1.0.0.RC14", "(,1.0.0.RC9.2),(1.0.0.RC9.2,)", true),
            ("0", "", false),
            ("1", "1", true),
            ("1", "(1", false),
            ("2.4.2", "2.4.2", true),
            ("2.4.2", "= 2.4.2", false),
            ("1.2.3", "[1,2],[3,4]", true),
            ("1.2.3", "[1.2.3]", true),
            ("1.2.3", "[1.2.4]", false),
            ("1.0", "[1.0,2.0)", true),
            ("2.0", "[1.0,2.0)", false),
            ("1.5", "[1.0,2.0)", true),
            ("0.9", "[1.0,2.0)", false),
            ("1.0", "(1.0,2.0]", false),
            ("2.0", "(1.0,2.0]", true),
            ("1.5", "(1.0,2.0]", true),
            ("0", "]0,2]", false),
            ("1", "]0,2]", true),
            ("2", "]0,2]", true),
            ("0", "]0,2[", false),
            ("1", "]0,2[", true),
            ("2", "]0,2[", false),
            ("1", "[1,2[", true),
            ("2", "[1,2[", false),
            ("0", "[1,2[", false),
        ];
        for (version, range, expected) in cases {
            assert_eq!(
                matches_range(version, range),
                *expected,
                "matches({version},{range})"
            );
        }
    }

    // Ported: 'isGreaterThan("$a", "$b") === $expected' — maven/index.spec.ts line 158
    #[test]
    fn is_greater_than_matches_renovate_maven_index_spec() {
        assert!(is_greater_than("1.1", "1"));
    }

    // Ported: 'getSatisfyingVersion($versions, "$range") === $expected' — maven/index.spec.ts line 165
    #[test]
    fn get_satisfying_version_matches_renovate_maven_index_spec() {
        assert_eq!(get_satisfying_version(&["1"], "1"), Some("1"));
        assert_eq!(get_satisfying_version(&["1", "2", "3"], "[1,2]"), Some("2"));
        assert_eq!(get_satisfying_version(&["1", "2", "3"], "[1,)"), Some("3"));
        assert_eq!(get_satisfying_version(&["1", "2", "3"], "[4,)"), None);
        assert_eq!(
            get_satisfying_version(&["1.0", "1.1", "2"], "[1.0,2.0)"),
            Some("1.1")
        );
    }

    // Ported: 'minSatisfyingVersion($versions, "$range") === $expected' — maven/index.spec.ts line 179
    #[test]
    fn min_satisfying_version_matches_renovate_maven_index_spec() {
        assert_eq!(get_satisfying_version(&["1"], "1"), Some("1"));
        assert_eq!(get_satisfying_version(&["1", "2", "3"], "[1,2]"), Some("2"));
        assert_eq!(get_satisfying_version(&["1", "2", "3"], "[1,)"), Some("3"));
        assert_eq!(get_satisfying_version(&["1", "2", "3"], "[4,)"), None);
        assert_eq!(
            get_satisfying_version(&["1.0", "1.1", "2"], "[1.0,2.0)"),
            Some("1.1")
        );
    }

    // Ported: 'getNewValue($currentValue, $rangeStrategy, $currentVersion, $newVersion, $expected) === $expected' — maven/index.spec.ts line 193
    #[test]
    fn get_new_value_matches_renovate_maven_index_spec() {
        let cases: &[(&str, Option<&str>, &str, &str)] = &[
            ("1", None, "1.1", "1.1"),
            ("[1.2.3,]", None, "1.2.4", "[1.2.3,]"),
            ("[1.2.3]", Some("pin"), "1.2.4", "1.2.4"),
            ("[1.0.0,1.2.3]", Some("pin"), "1.2.4", "1.2.4"),
            ("[1.0.0,1.2.23]", Some("pin"), "1.2.23", "1.2.23"),
            ("(,1.0]", Some("pin"), "2.0", "2.0"),
            ("],1.0]", Some("pin"), "2.0", "2.0"),
            ("(,1.0)", Some("pin"), "2.0", "2.0"),
            ("],1.0[", Some("pin"), "],2.0[", "],2.0["),
            ("[1.0,1.2],[1.3,1.5)", Some("pin"), "1.2.4", "1.2.4"),
            ("[1.0,1.2],[1.3,1.5[", Some("pin"), "1.2.4", "1.2.4"),
            ("[1.2.3,)", Some("pin"), "1.2.4", "1.2.4"),
            ("[1.2.3,[", Some("pin"), "1.2.4", "1.2.4"),
            ("[1.0.0,1.2.3]", Some("widen"), "1.2.4", "[1.0.0,1.2.4]"),
            ("[1.0.0,1.2.3]", Some("bump"), "1.2.4", "[1.0.0,1.2.4]"),
            ("[1.0.0,1.2.3]", Some("replace"), "1.2.4", "[1.0.0,1.2.4]"),
        ];
        for (cv, rs, nv, expected) in cases {
            assert_eq!(
                get_new_value(cv, *rs, nv),
                *expected,
                "cv={cv} rs={rs:?} nv={nv}"
            );
        }
    }

    // Ported: 'matches("$version", "[2.164.0,2.165.0)") === $expected' — maven/index.spec.ts line 228
    #[test]
    fn matches_jenkins_range_excl_matches_renovate_maven_index_spec() {
        let range = "[2.164.0,2.165.0)";
        assert!(matches_range("2.164.0", range));
        assert!(matches_range("2.164.1", range));
        assert!(matches_range("2.164.99", range));
        assert!(!matches_range("2.165.0", range));
        assert!(!matches_range("2.163.9", range));
        assert!(!matches_range("2.164.0-SNAPSHOT", range));
    }

    // Ported: 'matches("$version", "[2.164.0,2.165.0]") === $expected' — maven/index.spec.ts line 247
    #[test]
    fn matches_jenkins_range_incl_matches_renovate_maven_index_spec() {
        let range = "[2.164.0,2.165.0]";
        assert!(matches_range("2.164.0", range));
        assert!(matches_range("2.164.1", range));
        assert!(matches_range("2.164.99", range));
        assert!(matches_range("2.165.0", range));
        assert!(!matches_range("2.163.9", range));
        assert!(!matches_range("2.164.0-SNAPSHOT", range));
    }

    // Ported: 'matches("$version", "(,2.164.0)") === $expected' — maven/index.spec.ts line 266
    #[test]
    fn matches_jenkins_range_lt_matches_renovate_maven_index_spec() {
        let range = "(,2.164.0)";
        assert!(!matches_range("2.164.0", range));
        assert!(!matches_range("2.164.1", range));
        assert!(matches_range("2.163.9", range));
        assert!(matches_range("2.164.0-SNAPSHOT", range));
        assert!(matches_range("1.0.0", range));
    }

    // Ported: "autoExtendMavenRange("$range", "$version") === $expected" — maven/compare.spec.ts line 560
    #[test]
    fn auto_extend_maven_range_matches_renovate_maven_compare_spec() {
        let cases: &[(&str, &str, &str)] = &[
            ("[1.2.3]", "1.2.3", "[1.2.3]"),
            ("[1.2.3]", "1.2.4", "[1.2.4]"),
            ("[1.0.0,1.2.3]", "0.0.1", "[1.0.0,1.2.3]"),
            ("[1.0.0,1.2.3]", "1.2.4", "[1.0.0,1.2.4]"),
            ("[1.0.0,1.2.23]", "1.1.0", "[1.0.0,1.2.23]"),
            ("(,1.0]", "2.0", "(,2.0]"),
            ("],1.0]", "2.0", "],2.0]"),
            ("(,1.0)", "2.0", "(,3.0)"),
            ("],1.0[", "2.0", "],3.0["),
            ("[1.0,1.2.3],[1.3,1.5)", "1.2.4", "[1.0,1.2.4],[1.3,1.5)"),
            ("[1.0,1.2.3],[1.3,1.5[", "1.2.4", "[1.0,1.2.4],[1.3,1.5["),
            ("[1.2.3,)", "1.2.4", "[1.2.4,)"),
            ("[1.2.3,[", "1.2.4", "[1.2.4,["),
            ("[1.2.3,]", "1.2.4", "[1.2.3,]"),
            ("[0.21,0.22)", "0.20.21", "[0.21,0.22)"),
            ("[0.21,0.22)", "0.21.1", "[0.21,0.22)"),
            ("[0.21,0.22.0)", "0.22.1", "[0.21,0.22.2)"),
            ("[0.21,0.22)", "0.23", "[0.23,0.24)"),
            ("[1.8,1.9)", "1.9.0.1", "[1.9,1.10)"),
            ("[1.8a,1.9)", "1.9.0.1", "[1.8a,1.10)"),
            ("[1.8,1.9.0)", "1.9.0.1", "[1.8,1.10.0)"),
            ("[1.8,1.9.0.0)", "1.9.0.1", "[1.8,1.9.0.2)"),
            ("[1.8,1.9.0.0)", "1.10.1", "[1.8,1.10.2.0)"),
            ("[1.8,1.9)", "1.9.1", "[1.9,1.10)"),
            ("[1.8,1.9)", "1.10.0", "[1.10,1.11)"),
            ("[1.8,1.9)", "1.10.1", "[1.10,1.11)"),
            ("(,1.0.0]", "2.0.0", "(,2.0.0]"),
            ("(,1.0]", "2.0.0", "(,2.0]"),
            ("(,1]", "2.0.0", "(,2]"),
            ("(,1.0.0-foobar]", "2.0.0", "(,2.0.0]"),
            ("[1,2]", "2", "[1,2]"),
            ("[1,2)", "2", "[2,3)"),
            ("[0,2)", "2", "[0,3)"),
            ("[1.2,1.3]", "1.3", "[1.2,1.3]"),
            ("[1.2,1.3)", "1.3", "[1.3,1.4)"),
            ("[1-2,1-3)", "1-3", "[1-3,1-4)"),
            ("[1.1,1.3)", "1.3", "[1.1,1.4)"),
            ("[1.2.3,1.2.4]", "1.2.4", "[1.2.3,1.2.4]"),
            ("[1.2.3,1.2.4)", "1.2.4", "[1.2.4,1.2.5)"),
            ("[1.2.1,1.2.4)", "1.2.4", "[1.2.1,1.2.5)"),
            ("[1,1.2.3)", "1.2.3", "[1,1.2.4)"),
            ("[v3,v4)", "v4.3.2", "[v4,v5)"),
            ("],v1.0]", "v2.0", "],v2.0]"),
            ("(,v1.0)", "v2.0", "(,v3.0)"),
            ("[v1.2.3,]", "v1.2.4", "[v1.2.3,]"),
        ];
        for (range, version, expected) in cases {
            assert_eq!(
                auto_extend_maven_range(range, version),
                *expected,
                "range={range} version={version}"
            );
        }
    }

    // Ported: "should tokenize" — maven/compare.spec.ts line 454
    #[test]
    fn tokenize_matches_renovate_maven_compare_spec() {
        let none_prefix = Prefix::Other(String::new());

        // '1.2.3' → [Number(1)/none, Number(2)/Dot, Number(3)/Dot]
        let t = tokenize("1.2.3");
        assert_eq!(t.len(), 3);
        assert_eq!(t[0], Token { prefix: none_prefix.clone(), value: TokenValue::Number(1), is_transition: false });
        assert_eq!(t[1], Token { prefix: Prefix::Dot, value: TokenValue::Number(2), is_transition: false });
        assert_eq!(t[2], Token { prefix: Prefix::Dot, value: TokenValue::Number(3), is_transition: false });

        // 'v1.2.3' → [Number(1)/v-prefix, Number(2)/Dot, Number(3)/Dot]
        let t = tokenize("v1.2.3");
        assert_eq!(t.len(), 3);
        assert_eq!(t[0], Token { prefix: Prefix::Other("v".to_owned()), value: TokenValue::Number(1), is_transition: false });
        assert_eq!(t[1], Token { prefix: Prefix::Dot, value: TokenValue::Number(2), is_transition: false });
        assert_eq!(t[2], Token { prefix: Prefix::Dot, value: TokenValue::Number(3), is_transition: false });

        // 'version2.3' → [Qualifier("version")/none/transition, Number(2)/Hyphen, Number(3)/Dot]
        let t = tokenize("version2.3");
        assert_eq!(t.len(), 3);
        assert_eq!(t[0], Token { prefix: none_prefix.clone(), value: TokenValue::Qualifier("version".to_owned()), is_transition: true });
        assert_eq!(t[1], Token { prefix: Prefix::Hyphen, value: TokenValue::Number(2), is_transition: false });
        assert_eq!(t[2], Token { prefix: Prefix::Dot, value: TokenValue::Number(3), is_transition: false });

        // 'alpha.beta.rc' → [Q("alpha"), Q("beta")/Dot, Q("rc")/Dot]
        let t = tokenize("alpha.beta.rc");
        assert_eq!(t.len(), 3);
        assert_eq!(t[0], Token { prefix: none_prefix.clone(), value: TokenValue::Qualifier("alpha".to_owned()), is_transition: false });
        assert_eq!(t[1], Token { prefix: Prefix::Dot, value: TokenValue::Qualifier("beta".to_owned()), is_transition: false });
        assert_eq!(t[2], Token { prefix: Prefix::Dot, value: TokenValue::Qualifier("rc".to_owned()), is_transition: false });

        // '1.2.3-alpha.beta'
        let t = tokenize("1.2.3-alpha.beta");
        assert_eq!(t.len(), 5);
        assert_eq!(t[0], Token { prefix: none_prefix.clone(), value: TokenValue::Number(1), is_transition: false });
        assert_eq!(t[1], Token { prefix: Prefix::Dot, value: TokenValue::Number(2), is_transition: false });
        assert_eq!(t[2], Token { prefix: Prefix::Dot, value: TokenValue::Number(3), is_transition: false });
        assert_eq!(t[3], Token { prefix: Prefix::Hyphen, value: TokenValue::Qualifier("alpha".to_owned()), is_transition: false });
        assert_eq!(t[4], Token { prefix: Prefix::Dot, value: TokenValue::Qualifier("beta".to_owned()), is_transition: false });

        // '1.2.x-3'
        let t = tokenize("1.2.x-3");
        assert_eq!(t.len(), 4);
        assert_eq!(t[0], Token { prefix: none_prefix.clone(), value: TokenValue::Number(1), is_transition: false });
        assert_eq!(t[1], Token { prefix: Prefix::Dot, value: TokenValue::Number(2), is_transition: false });
        assert_eq!(t[2], Token { prefix: Prefix::Dot, value: TokenValue::Qualifier("x".to_owned()), is_transition: false });
        assert_eq!(t[3], Token { prefix: Prefix::Hyphen, value: TokenValue::Number(3), is_transition: false });

        // '00.02.003' → leading zero stripped: [Number(0), Number(2)/Dot, Number(3)/Dot]
        let t = tokenize("00.02.003");
        assert_eq!(t.len(), 3);
        assert_eq!(t[0], Token { prefix: none_prefix.clone(), value: TokenValue::Number(0), is_transition: false });
        assert_eq!(t[1], Token { prefix: Prefix::Dot, value: TokenValue::Number(2), is_transition: false });
        assert_eq!(t[2], Token { prefix: Prefix::Dot, value: TokenValue::Number(3), is_transition: false });

        // 'invalid.version'
        let t = tokenize("invalid.version");
        assert_eq!(t.len(), 2);
        assert_eq!(t[0], Token { prefix: none_prefix.clone(), value: TokenValue::Qualifier("invalid".to_owned()), is_transition: false });
        assert_eq!(t[1], Token { prefix: Prefix::Dot, value: TokenValue::Qualifier("version".to_owned()), is_transition: false });

        // '' → [zeroToken = Number(0)/none/not-transition]
        let t = tokenize("");
        assert_eq!(t.len(), 1);
        assert_eq!(t[0], Token { prefix: none_prefix.clone(), value: TokenValue::Number(0), is_transition: false });
    }
}
