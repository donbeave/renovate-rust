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
        } else if leading_zero {
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
}
