//! PEP 440 versioning and update decision logic.
//! @parity lib/modules/versioning/pep440/index.ts full
//! @parity lib/modules/versioning/pep440/range.ts full
//!
//! Renovate reference: `lib/modules/versioning/pep440/index.ts` and
//! `lib/modules/versioning/pep440/range.ts`.

use std::str::FromStr;

use pep440_rs::{Operator, Version, VersionSpecifier, VersionSpecifiers};

// ── Existing helpers ──────────────────────────────────────────────────────────

/// Detailed update summary for a single pip dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pep440UpdateSummary {
    pub current_specifier: String,
    pub latest: Option<String>,
    pub update_available: bool,
    pub latest_timestamp: Option<String>,
    pub current_version_timestamp: Option<String>,
}

pub fn pep440_update_summary(specifier: &str, latest_stable: Option<&str>) -> Pep440UpdateSummary {
    let latest = latest_stable.map(str::to_owned);
    let pinned = exact_pin_version(specifier);
    let update_available = pinned
        .as_deref()
        .zip(latest.as_deref())
        .is_some_and(|(pin, lat)| pin != lat);
    Pep440UpdateSummary {
        current_specifier: specifier.to_owned(),
        latest,
        update_available,
        latest_timestamp: None,
        current_version_timestamp: None,
    }
}

/// Extract the pinned version from an exact `==X.Y.Z` specifier.
pub fn exact_pin_version(specifier: &str) -> Option<String> {
    let s = specifier.trim();
    if s.is_empty() {
        return None;
    }
    if s.contains(',') {
        return None;
    }
    let version = s.strip_prefix("==")?;
    if version.contains('*') {
        return None;
    }
    Some(version.trim().to_owned())
}

// ── New API functions (ported from TypeScript) ────────────────────────────────

/// Whether `input` is a valid PEP 440 version or range specifier.
pub fn is_valid(input: &str) -> bool {
    Version::from_str(input).is_ok() || VersionSpecifiers::from_str(input).is_ok()
}

/// Whether `input` is a stable (non-pre-release, non-dev) version.
pub fn is_stable(input: &str) -> bool {
    Version::from_str(input)
        .map(|v| v.is_stable())
        .unwrap_or(false)
}

/// Whether `a` is strictly greater than `b` (PEP 440 ordering).
pub fn is_greater_than(a: &str, b: &str) -> bool {
    match (Version::from_str(a), Version::from_str(b)) {
        (Ok(va), Ok(vb)) => va > vb,
        _ => false,
    }
}

/// Return the ordering of `a` relative to `b` for sort purposes.
pub fn sort_versions(a: &str, b: &str) -> std::cmp::Ordering {
    match (Version::from_str(a), Version::from_str(b)) {
        (Ok(va), Ok(vb)) => va.cmp(&vb),
        (Ok(_), Err(_)) => std::cmp::Ordering::Greater,
        (Err(_), Ok(_)) => std::cmp::Ordering::Less,
        _ => std::cmp::Ordering::Equal,
    }
}

/// Whether two version strings are semantically equal.
pub fn equals(a: &str, b: &str) -> bool {
    match (Version::from_str(a), Version::from_str(b)) {
        (Ok(va), Ok(vb)) => va == vb,
        _ => false,
    }
}

/// Whether `version` satisfies `range` (range can be a bare version or specifiers).
pub fn matches_range(version: &str, range: &str) -> bool {
    let Ok(ver) = Version::from_str(version) else {
        return false;
    };
    if let Ok(range_ver) = Version::from_str(range) {
        return ver == range_ver;
    }
    if let Ok(specs) = VersionSpecifiers::from_str(range) {
        return specs.contains(&ver);
    }
    false
}

/// Whether `constraint` represents exactly one version.
pub fn is_single_version(constraint: &str) -> bool {
    if Version::from_str(constraint).is_ok() {
        return true;
    }
    let trimmed = constraint.trim();
    if let Some(rest) = trimmed.strip_prefix("==") {
        let rest = rest.trim();
        if !rest.contains('*') {
            return Version::from_str(rest).is_ok();
        }
    }
    false
}

/// Highest version from `versions` that satisfies `range`, or `None`.
pub fn get_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    let Ok(specs) = VersionSpecifiers::from_str(range) else {
        return None;
    };
    let mut satisfying: Vec<(&str, Version)> = versions
        .iter()
        .filter_map(|&v| {
            Version::from_str(v)
                .ok()
                .filter(|ver| specs.contains(ver))
                .map(|ver| (v, ver))
        })
        .collect();
    satisfying.sort_by(|a, b| a.1.cmp(&b.1));
    satisfying.last().map(|(s, _)| *s)
}

/// Lowest version from `versions` that satisfies `range`, or `None`.
pub fn min_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    let Ok(specs) = VersionSpecifiers::from_str(range) else {
        return None;
    };
    let mut satisfying: Vec<(&str, Version)> = versions
        .iter()
        .filter_map(|&v| {
            Version::from_str(v)
                .ok()
                .filter(|ver| specs.contains(ver))
                .map(|ver| (v, ver))
        })
        .collect();
    satisfying.sort_by(|a, b| a.1.cmp(&b.1));
    satisfying.first().map(|(s, _)| *s)
}

/// Whether `version` is below the lower bound of `range`.
///
/// Ported from `isLessThanRange` in `lib/modules/versioning/pep440/range.ts`.
pub fn is_less_than_range(version: &str, range: &str) -> bool {
    let Ok(input_ver) = Version::from_str(version) else {
        return false;
    };

    let mut invert_result = true;
    let mut results: Vec<bool> = Vec::new();

    for part in range.split(',') {
        let part: String = part.chars().filter(|c| !c.is_whitespace()).collect();
        let Some((op_str, ver_str)) = split_op_version(&part) else {
            return false;
        };
        match op_str {
            "!=" | "<=" | "<" => {
                results.push(true);
            }
            "~=" | "==" | ">=" | "===" => {
                invert_result = false;
                let Ok(ver) = Version::from_str(ver_str) else {
                    return false;
                };
                results.push(input_ver < ver);
            }
            ">" => {
                invert_result = false;
                let Ok(ver) = Version::from_str(ver_str) else {
                    return false;
                };
                results.push(input_ver <= ver);
            }
            _ => results.push(false),
        }
    }

    let result = results.iter().all(|&r| r);
    if invert_result { !result } else { result }
}

/// Params for `get_new_value`.
#[derive(Debug, Clone)]
pub struct NewValueParams {
    pub current_value: String,
    pub range_strategy: String,
    pub current_version: String,
    pub new_version: String,
    pub is_replacement: bool,
}

/// Compute an updated version specifier string.
///
/// Ported from `getNewValue` in `lib/modules/versioning/pep440/range.ts`.
pub fn get_new_value(params: &NewValueParams) -> Option<String> {
    let NewValueParams {
        current_value,
        range_strategy,
        current_version,
        new_version,
        is_replacement,
    } = params;

    if current_value == current_version || *is_replacement {
        return Some(new_version.clone());
    }

    if Version::from_str(current_value).is_ok() {
        let first = current_value.chars().next().unwrap_or(' ');
        if first == 'v' || first == 'V' {
            return Some(format!("{}{}", first, new_version));
        }
        return Some(new_version.clone());
    }

    let Ok(clauses) = parse_current_range(current_value) else {
        return None;
    };

    if clauses.is_empty() {
        return Some(current_value.clone());
    }

    let Ok(new_ver) = Version::from_str(new_version) else {
        return None;
    };

    // Return early if newVersion is excluded from range.
    if clauses.iter().any(|c| {
        matches!(c.op, Operator::NotEqual | Operator::NotEqualStar) && c.version == new_ver
    }) {
        tracing::debug!(
            "Cannot calculate new value as the newVersion:`{}` is excluded from range: `{}`",
            new_version,
            current_value
        );
        return None;
    }

    let updated: Vec<Option<String>> = match range_strategy.as_str() {
        "auto" | "replace" => {
            handle_replace_strategy(current_value, new_version, &new_ver, &clauses)
        }
        "widen" => handle_widen_strategy(current_value, new_version, &new_ver, &clauses),
        "bump" => handle_bump_strategy(new_version, &new_ver, &clauses),
        _ => {
            tracing::debug!(
                "Unsupported rangeStrategy: {}. Using \"replace\" instead.",
                range_strategy
            );
            handle_replace_strategy(current_value, new_version, &new_ver, &clauses)
        }
    };

    let joined: Vec<String> = updated.into_iter().flatten().collect();
    let mut result = joined.join(", ");

    if result.contains(", ") && !current_value.contains(", ") {
        result = result.replace(", ", ",");
    }

    let result = check_range_and_remove_unnecessary_range_limit(&result, new_version);

    if let Ok(specs) = VersionSpecifiers::from_str(&result)
        && !specs.contains(&new_ver)
    {
        tracing::warn!(
            result,
            new_version,
            current_value,
            "pep440: failed to calculate newValue"
        );
        return None;
    }

    Some(result)
}

/// Remove unnecessary lower-bound from `==X.*,>=Y` patterns.
///
/// Ported from `checkRangeAndRemoveUnnecessaryRangeLimit` in range.ts.
pub fn check_range_and_remove_unnecessary_range_limit(range: &str, new_version: &str) -> String {
    if !range.contains(',') {
        return range.to_owned();
    }
    let parts: Vec<&str> = range.splitn(2, ',').collect();
    if parts.len() == 2
        && parts[0].contains(".*")
        && parts[0].contains("==")
        && parts[1].contains(">=")
        && let (Ok(specs), Ok(ver)) = (
            VersionSpecifiers::from_str(parts[0]),
            Version::from_str(new_version),
        )
        && specs.contains(&ver)
    {
        return parts[0].to_owned();
    }
    range.to_owned()
}

// ── Internal helpers ──────────────────────────────────────────────────────────

/// A single parsed range clause (`>=1.2.0`, `<2.0`, `==1.*`, etc.).
#[derive(Debug, Clone)]
struct RangeClause {
    op: Operator,
    /// True when the clause uses `==X.*` or `!=X.*` wildcard syntax.
    is_star: bool,
    version: Version,
}

fn fmt_parts(parts: &[u64]) -> String {
    parts
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(".")
}

fn format_clause_with_parts(op: Operator, is_star: bool, parts: &[u64]) -> String {
    let op_str = match op {
        Operator::Equal | Operator::EqualStar => "==",
        Operator::NotEqual | Operator::NotEqualStar => "!=",
        Operator::TildeEqual => "~=",
        Operator::LessThan => "<",
        Operator::LessThanEqual => "<=",
        Operator::GreaterThan => ">",
        Operator::GreaterThanEqual => ">=",
        Operator::ExactEqual => "===",
    };
    if is_star {
        format!("{}{}.*", op_str, fmt_parts(parts))
    } else {
        format!("{}{}", op_str, fmt_parts(parts))
    }
}

fn format_clause(clause: &RangeClause) -> String {
    format_clause_with_parts(clause.op, clause.is_star, clause.version.release())
}

/// Parse `currentValue` into ordered range clauses.
fn parse_current_range(current_value: &str) -> Result<Vec<RangeClause>, ()> {
    if current_value.trim().is_empty() {
        return Ok(Vec::new());
    }
    let mut clauses = Vec::new();
    for part in current_value.split(',') {
        let part = part.trim();
        let spec = VersionSpecifier::from_str(part).map_err(|_| ())?;
        if matches!(spec.operator(), Operator::ExactEqual) {
            return Err(());
        }
        clauses.push(RangeClause {
            is_star: spec.operator().is_star(),
            op: *spec.operator(),
            version: spec.version().clone(),
        });
    }
    Ok(clauses)
}

/// Split a specifier string (no whitespace) into `(operator, version_str)`.
fn split_op_version(s: &str) -> Option<(&str, &str)> {
    for op in &["===", "~=", "==", "!=", "<=", ">=", "<", ">"] {
        if let Some(rest) = s.strip_prefix(op) {
            return Some((op, rest));
        }
    }
    None
}

/// Compute precision (0=Major, 1=Minor, 2=Micro, 3+) from a clause list.
fn get_range_precision(clauses: &[RangeClause]) -> usize {
    let bound = if clauses.len() >= 2 {
        clauses[1].version.release()
    } else {
        clauses[0].version.release()
    };

    let raw = if clauses.len() == 1 {
        Some(bound.len().saturating_sub(1))
    } else {
        let lower = clauses[0].version.release();
        bound.iter().enumerate().find_map(|(i, &el)| {
            let lo = lower.get(i).copied().unwrap_or(0);
            if el > lo { Some(i) } else { None }
        })
    };

    let precision = raw.unwrap_or_else(|| bound.len().saturating_sub(1));

    // Tune down Major if followed by zero.
    if precision == 0 && bound.len() > 1 && bound[1] == 0 {
        1
    } else {
        precision
    }
}

/// Whether any clause has a version with fewer than 3 components.
fn has_zero_specifier(clauses: &[RangeClause]) -> bool {
    clauses.iter().any(|c| c.version.release().len() < 3)
}

/// Compute the next version boundary.
///
/// `policy` controls which component increments; `usize::MAX` copies all.
/// `base_release` provides the shape (length) of the output; when `None`
/// the shape comes from `new_release` and the policy index is NOT incremented.
fn get_future_version(
    policy: usize,
    new_release: &[u64],
    base_release: Option<&[u64]>,
) -> Vec<u64> {
    let (effective_base, increment) = match base_release {
        Some(b) => (b, true),
        None => (new_release, false),
    };
    effective_base
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let to = new_release.get(i).copied().unwrap_or(0);
            if policy == usize::MAX || i < policy {
                to
            } else if i == policy {
                if increment { to + 1 } else { to }
            } else {
                0
            }
        })
        .collect()
}

fn trim_trailing_zeros(v: &[u64]) -> Vec<u64> {
    let mut end = v.len();
    while end > 0 && v[end - 1] == 0 {
        end -= 1;
    }
    v[..end].to_vec()
}

/// Expand `~=X.Y.Z` into `[>=X.Y.Z, <X.{Y+1}]`.
fn divide_compatible_release(clause: &RangeClause) -> Vec<RangeClause> {
    let mut upper = clause.version.release().to_vec();
    if upper.len() > 1 {
        upper.pop();
    }
    *upper.last_mut().unwrap() += 1;
    let upper_ver =
        Version::from_str(&fmt_parts(&upper)).unwrap_or_else(|_| clause.version.clone());
    vec![
        RangeClause {
            op: Operator::GreaterThanEqual,
            is_star: false,
            version: clause.version.clone(),
        },
        RangeClause {
            op: Operator::LessThan,
            is_star: false,
            version: upper_ver,
        },
    ]
}

/// Shared single-clause update logic (used by bump and as fallback).
fn update_range_value(
    new_version: &str,
    new_ver: &Version,
    clause: &RangeClause,
) -> Option<String> {
    let op = clause.op;

    // !=  → preserve unchanged
    if matches!(op, Operator::NotEqual | Operator::NotEqualStar) {
        return Some(format_clause(clause));
    }

    // ==1.2.* wildcard → update the non-star components
    if clause.is_star {
        let future = get_future_version(
            usize::MAX,
            new_ver.release(),
            Some(clause.version.release()),
        );
        return Some(format_clause_with_parts(Operator::EqualStar, true, &future));
    }

    // ~= compatible release
    if matches!(op, Operator::TildeEqual) {
        let base_len = clause.version.release().len();
        let new_release = new_ver.release();
        let new_len = new_release.len();
        if base_len < new_len {
            let sliced: Vec<u64> = new_release[..base_len].to_vec();
            return Some(format!("~={}", fmt_parts(&sliced)));
        }
        if base_len > new_len {
            let mut padded = new_release.to_vec();
            while padded.len() < base_len {
                padded.push(0);
            }
            return Some(format!("~={}", fmt_parts(&padded)));
        }
        return Some(format!("~={}", fmt_parts(new_release)));
    }

    // == and <=
    if matches!(op, Operator::Equal | Operator::LessThanEqual) {
        let op_str = if matches!(op, Operator::Equal) {
            "=="
        } else {
            "<="
        };
        if new_ver <= &clause.version {
            return Some(format_clause(clause));
        }
        return Some(format!("{}{}", op_str, fmt_parts(new_ver.release())));
    }

    // < upper bound
    if matches!(op, Operator::LessThan) {
        if new_ver >= &clause.version {
            let precision = get_range_precision(std::slice::from_ref(clause));
            let future =
                get_future_version(precision, new_ver.release(), Some(clause.version.release()));
            return Some(format!("<{}", fmt_parts(&future)));
        }
        return Some(format_clause(clause));
    }

    // > and >= lower bound
    if matches!(op, Operator::GreaterThan | Operator::GreaterThanEqual) {
        if new_ver <= &clause.version {
            return Some(format!(">={}", new_version));
        }
        return Some(format_clause(clause));
    }

    None
}

fn handle_bump_strategy(
    new_version: &str,
    new_ver: &Version,
    clauses: &[RangeClause],
) -> Vec<Option<String>> {
    clauses
        .iter()
        .map(|c| {
            if matches!(c.op, Operator::GreaterThanEqual) {
                Some(format!(">={}", new_version))
            } else {
                update_range_value(new_version, new_ver, c)
            }
        })
        .collect()
}

fn handle_replace_strategy(
    current_value: &str,
    new_version: &str,
    new_ver: &Version,
    clauses: &[RangeClause],
) -> Vec<Option<String>> {
    if satisfies_range(new_ver, current_value) {
        return vec![Some(current_value.to_owned())];
    }

    let trim_zeros = has_zero_specifier(clauses);

    clauses
        .iter()
        .map(|c| {
            // < upper bound that new version exceeds
            if matches!(c.op, Operator::LessThan) && new_ver >= &c.version {
                let precision = get_range_precision(clauses);
                let mut future =
                    get_future_version(precision, new_ver.release(), Some(c.version.release()));
                if trim_zeros {
                    future = trim_trailing_zeros(&future);
                }
                return Some(format!("<{}", fmt_parts(&future)));
            }

            // > and >= lower bound
            if matches!(c.op, Operator::GreaterThan | Operator::GreaterThanEqual) {
                if new_ver <= &c.version {
                    return Some(format!(">={}", new_version));
                }
                let lower_len = c.version.release().len();
                let lp = lower_len.saturating_sub(1);
                let mut new_base = get_future_version(lp, new_ver.release(), None);
                if trim_zeros {
                    new_base = trim_trailing_zeros(&new_base);
                }
                if matches!(c.op, Operator::GreaterThan)
                    && new_version == fmt_parts(&new_base)
                    && new_base.len() > 1
                {
                    new_base.pop();
                }
                return Some(format!(
                    "{}{}",
                    if matches!(c.op, Operator::GreaterThan) {
                        ">"
                    } else {
                        ">="
                    },
                    fmt_parts(&new_base)
                ));
            }

            update_range_value(new_version, new_ver, c)
        })
        .collect()
}

fn handle_widen_strategy(
    current_value: &str,
    new_version: &str,
    new_ver: &Version,
    clauses: &[RangeClause],
) -> Vec<Option<String>> {
    if satisfies_range(new_ver, current_value) {
        return vec![Some(current_value.to_owned())];
    }

    let mut range_precision = get_range_precision(clauses);
    let trim_zeros = has_zero_specifier(clauses);

    // Expand ~= into its two-bound equivalent for widen.
    let effective: Vec<RangeClause> =
        if clauses.len() == 1 && matches!(clauses[0].op, Operator::TildeEqual) {
            divide_compatible_release(&clauses[0])
        } else {
            clauses.to_vec()
        };

    effective
        .iter()
        .map(|c| {
            if matches!(c.op, Operator::LessThan) && new_ver >= &c.version {
                let upper = c.version.release();
                let len = upper.len();
                if upper[len - 1] != 0 {
                    let key = len.saturating_sub(1).min(3);
                    range_precision = key;
                }
                let mut future =
                    get_future_version(range_precision, new_ver.release(), Some(upper));
                if trim_zeros {
                    future = trim_trailing_zeros(&future);
                }
                return Some(format!("<{}", fmt_parts(&future)));
            }

            update_range_value(new_version, new_ver, c)
        })
        .collect()
}

fn satisfies_range(ver: &Version, range: &str) -> bool {
    VersionSpecifiers::from_str(range)
        .map(|s| s.contains(ver))
        .unwrap_or(false)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── exact_pin_version ─────────────────────────────────────────────────────

    // Rust-specific: unit tests for exact_pin_version helper
    #[test]
    fn exact_pin_simple() {
        assert_eq!(exact_pin_version("==1.2.3"), Some("1.2.3".to_owned()));
        assert_eq!(exact_pin_version("== 4.2.7"), Some("4.2.7".to_owned()));
    }

    // Rust-specific: unit tests for exact_pin_version helper
    #[test]
    fn ranges_are_not_pins() {
        for s in &[">=1.0", ">=1.0,<2.0", "~=1.4", "!=1.5", ""] {
            assert!(exact_pin_version(s).is_none(), "{s:?} should not be a pin");
        }
    }

    // Rust-specific: unit tests for exact_pin_version helper
    #[test]
    fn wildcard_is_not_pin() {
        assert!(exact_pin_version("==1.*").is_none());
    }

    // ── pep440_update_summary ─────────────────────────────────────────────────

    // Rust-specific: unit tests for pep440_update_summary edge cases
    #[test]
    fn pinned_with_newer_latest_has_update() {
        let s = pep440_update_summary("==4.2.7", Some("4.2.10"));
        assert!(s.update_available);
        assert_eq!(s.latest.as_deref(), Some("4.2.10"));
    }

    // Rust-specific: unit tests for pep440_update_summary edge cases
    #[test]
    fn pinned_already_latest_has_no_update() {
        let s = pep440_update_summary("==4.2.10", Some("4.2.10"));
        assert!(!s.update_available);
    }

    // Rust-specific: unit tests for pep440_update_summary edge cases
    #[test]
    fn range_specifier_has_no_update() {
        let s = pep440_update_summary(">=4.0,<5.0", Some("4.2.10"));
        assert!(!s.update_available);
        assert_eq!(s.latest.as_deref(), Some("4.2.10"));
    }

    // Rust-specific: unit tests for pep440_update_summary edge cases
    #[test]
    fn unconstrained_has_no_update() {
        let s = pep440_update_summary("", Some("4.2.10"));
        assert!(!s.update_available);
    }

    // Rust-specific: unit tests for pep440_update_summary edge cases
    #[test]
    fn no_latest_has_no_update() {
        let s = pep440_update_summary("==4.2.7", None);
        assert!(!s.update_available);
    }

    // ── is_valid ──────────────────────────────────────────────────────────────

    // Ported: "isValid("$input") === $expected" — lib/modules/versioning/pep440/index.spec.ts line 4
    #[test]
    fn is_valid_table() {
        let cases = [
            ("0.750", true),
            ("1.2.3", true),
            ("1.9", true),
            ("17.04.0", true),
            ("==1.2.3", true),
            ("==1.2.3.0", true),
            ("==1.2.3rc0", true),
            ("==1.2.3b0", true),
            ("~=1.2.3", true),
            ("==1.2.*", true),
            (">1.2.3", true),
            ("renovatebot/renovate", false),
            ("renovatebot/renovate#master", false),
            ("https://github.com/renovatebot/renovate.git", false),
        ];
        for (input, expected) in cases {
            assert_eq!(is_valid(input), expected, "is_valid({input:?})");
        }
    }

    // ── is_stable ─────────────────────────────────────────────────────────────

    // Ported: "isStable("$input") === $expected" — lib/modules/versioning/pep440/index.spec.ts line 25
    #[test]
    fn is_stable_table() {
        assert!(is_stable("1.2.3"));
        assert!(!is_stable("1.2.3rc0"));
        assert!(!is_stable("not_version"));
    }

    // ── equals ────────────────────────────────────────────────────────────────

    // Ported: "equals($a, $b) === $expected" — lib/modules/versioning/pep440/index.spec.ts line 34
    #[test]
    fn equals_table() {
        assert!(equals("1.0", "1.0.0"));
        assert!(!equals("1.0.0", "1.0..foo"));
    }

    // ── matches ───────────────────────────────────────────────────────────────

    // Ported: "matches($a, $b) === $expected" — lib/modules/versioning/pep440/index.spec.ts line 42
    #[test]
    fn matches_table() {
        assert!(matches_range("1.0", ">=1.0.0"));
        assert!(matches_range("3.0.0", "3.0.0"));
        assert!(matches_range("1.6.2", "<2.2.1.0"));
        assert!(!matches_range(">=3.8", ">=3.9"));
        assert!(matches_range("0.43b0", ">=0.43b0, <1.0"));
    }

    // ── is_single_version ─────────────────────────────────────────────────────

    // Ported: "isSingleVersion("$version") === $isSingle" — lib/modules/versioning/pep440/index.spec.ts line 53
    #[test]
    fn is_single_version_table() {
        assert!(is_single_version("1.2.3"));
        assert!(is_single_version("1.2.3rc0"));
        assert!(is_single_version("1.2.3b0"));
        assert!(is_single_version("==1.2.3"));
        assert!(is_single_version("==1.2"));
        assert!(is_single_version("== 1.2.3"));
        assert!(!is_single_version("==1.*"));
    }

    // ── get_satisfying_version ────────────────────────────────────────────────

    // Ported: "getSatisfyingVersion($versions, "$range") === $expected" — lib/modules/versioning/pep440/index.spec.ts line 78
    #[test]
    fn get_satisfying_version_table() {
        let versions = &[
            "0.9.4", "1.0.0", "1.1.5", "1.2.1", "1.2.2", "1.2.3", "1.3.4", "2.0.3",
        ];
        assert_eq!(get_satisfying_version(versions, "~=1.2.1"), Some("1.2.3"));
        assert_eq!(get_satisfying_version(versions, "~=2.1"), None);
    }

    // ── min_satisfying_version ────────────────────────────────────────────────

    // Ported: "minSatisfyingVersion($versions, "$range") === $expected" — lib/modules/versioning/pep440/index.spec.ts line 89
    #[test]
    fn min_satisfying_version_table() {
        let versions = &[
            "0.9.4", "1.0.0", "1.1.5", "1.2.1", "1.2.2", "1.2.3", "1.3.4", "2.0.3",
        ];
        assert_eq!(min_satisfying_version(versions, "~=1.2.1"), Some("1.2.1"));
        assert_eq!(min_satisfying_version(versions, "~=2.1"), None);
    }

    // ── get_new_value (non-replacement) ───────────────────────────────────────

    fn gnv(cv: &str, strategy: &str, cur: &str, nv: &str) -> Option<String> {
        get_new_value(&NewValueParams {
            current_value: cv.to_owned(),
            range_strategy: strategy.to_owned(),
            current_version: cur.to_owned(),
            new_version: nv.to_owned(),
            is_replacement: false,
        })
    }

    // Ported: "getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected"" — lib/modules/versioning/pep440/index.spec.ts line 100
    #[test]
    fn get_new_value_table() {
        let cases: &[(&str, &str, &str, &str, Option<&str>)] = &[
            ("1.0.0", "bump", "1.0.0", "1.2.3", Some("1.2.3")),
            ("1.0.0", "replace", "1.0.0", "1.2.3", Some("1.2.3")),
            ("v1.0.0", "bump", "1.0.0", "1.2.3", Some("v1.2.3")),
            ("v1.0.0", "replace", "1.0.0", "1.2.3", Some("v1.2.3")),
            ("==1.0.3", "bump", "1.0.0", "1.2.3", Some("==1.2.3")),
            ("==1.0.3", "replace", "1.0.0", "1.2.3", Some("==1.2.3")),
            (">=1.2.0", "bump", "1.0.0", "1.2.3", Some(">=1.2.3")),
            (">=1.2.0", "replace", "1.0.0", "1.2.3", Some(">=1.2.0")),
            ("~=1.2.0", "bump", "1.0.0", "1.2.3", Some("~=1.2.3")),
            ("~=1.2.0", "replace", "1.0.0", "1.2.3", Some("~=1.2.0")),
            ("~=1.0.3", "bump", "1.0.0", "1.2.3", Some("~=1.2.3")),
            ("~=1.0.3", "replace", "1.0.0", "1.2.3", Some("~=1.2.3")),
            ("==1.2.*", "bump", "1.0.0", "1.2.3", Some("==1.2.*")),
            ("==1.2.*", "replace", "1.0.0", "1.2.3", Some("==1.2.*")),
            ("==1.0.*", "bump", "1.0.0", "1.2.3", Some("==1.2.*")),
            ("==1.0.*", "replace", "1.0.0", "1.2.3", Some("==1.2.*")),
            ("<1.2.2.3", "bump", "1.0.0", "1.2.3", Some("<1.2.3.1")),
            ("<1.2.2.3", "replace", "1.0.0", "1.2.3", Some("<1.2.3.1")),
            ("<1.2.3", "bump", "1.0.0", "1.2.3", Some("<1.2.4")),
            ("<1.2.3", "replace", "1.0.0", "1.2.3", Some("<1.2.4")),
            ("<1.2", "bump", "1.0.0", "1.2.3", Some("<1.3")),
            ("<1.2", "replace", "1.0.0", "1.2.3", Some("<1.3")),
            ("<1", "bump", "1.0.0", "1.2.3", Some("<2")),
            ("<1", "replace", "1.0.0", "1.2.3", Some("<2")),
            ("<2.0.0", "bump", "1.0.0", "1.2.3", Some("<2.0.0")),
            ("<2.0.0", "replace", "1.0.0", "1.2.3", Some("<2.0.0")),
            (">0.9.8", "bump", "1.0.0", "1.2.3", Some(">0.9.8")),
            (">0.9.8", "replace", "1.0.0", "1.2.3", Some(">0.9.8")),
            (">2.0.0", "bump", "1.0.0", "1.2.3", Some(">=1.2.3")),
            (">2.0.0", "replace", "1.0.0", "1.2.3", Some(">=1.2.3")),
            (">=2.0.0", "bump", "1.0.0", "1.2.3", Some(">=1.2.3")),
            (">=2.0.0", "replace", "1.0.0", "1.2.3", Some(">=1.2.3")),
            (
                "~=1.1.0, !=1.1.1",
                "bump",
                "1.0.0",
                "1.2.3",
                Some("~=1.2.3, !=1.1.1"),
            ),
            (
                "~=1.1.0, !=1.1.1",
                "replace",
                "1.0.0",
                "1.2.3",
                Some("~=1.2.3, !=1.1.1"),
            ),
            (
                "~=1.1.0,!=1.1.1",
                "bump",
                "1.0.0",
                "1.2.3",
                Some("~=1.2.3,!=1.1.1"),
            ),
            (
                "~=1.1.0,!=1.1.1",
                "replace",
                "1.0.0",
                "1.2.3",
                Some("~=1.2.3,!=1.1.1"),
            ),
            (" ", "bump", "1.0.0", "1.2.3", Some(" ")),
            (" ", "replace", "1.0.0", "1.2.3", Some(" ")),
            ("invalid", "bump", "1.0.0", "1.2.3", None),
            ("invalid", "replace", "1.0.0", "1.2.3", None),
            ("===1.0.3", "bump", "1.0.0", "1.2.3", None),
            ("===1.0.3", "replace", "1.0.0", "1.2.3", None),
            ("!=1.2.3", "bump", "1.0.0", "1.2.3", None),
            ("!=1.2.3", "replace", "1.0.0", "1.2.3", None),
            (
                "~=1.1.0,!=1.1.1",
                "unsupported",
                "1.0.0",
                "1.2.3",
                Some("~=1.2.3,!=1.1.1"),
            ),
            (
                ">=19.12.2,<20.13.9",
                "replace",
                "19.12.2",
                "21.3.1",
                Some(">=21.3.1,<22.0.0"),
            ),
            (
                ">=19.12.2,<19.13.9",
                "replace",
                "19.12.2",
                "20.3.1",
                Some(">=20.3.1,<20.4.0"),
            ),
            (
                ">=19.12.2,<19.13.0",
                "replace",
                "19.12.2",
                "20.3.1",
                Some(">=20.3.1,<20.4.0"),
            ),
            (
                ">=19.12.2,<19.13.0",
                "replace",
                "19.12.2",
                "20.3.0",
                Some(">=20.3.0,<20.4.0"),
            ),
            (
                ">=19.12.2,<19.13.0",
                "replace",
                "19.12.2",
                "19.13.1",
                Some(">=19.13.1,<19.14.0"),
            ),
            (
                ">=19.12.2,<19.13.0",
                "replace",
                "19.12.2",
                "19.13.0",
                Some(">=19.13.0,<19.14.0"),
            ),
            (
                ">=19.12.2,<19.13.0",
                "auto",
                "19.12.2",
                "19.13.0",
                Some(">=19.13.0,<19.14.0"),
            ),
            (
                ">=19.12.2,<20.13.9",
                "widen",
                "19.12.2",
                "21.3.1",
                Some(">=19.12.2,<21.3.2"),
            ),
            (
                ">=19.12.2,<19.13.9",
                "widen",
                "19.12.2",
                "20.3.1",
                Some(">=19.12.2,<20.3.2"),
            ),
            (
                ">=19.12.2,<19.13.0",
                "widen",
                "19.12.2",
                "20.3.1",
                Some(">=19.12.2,<20.4.0"),
            ),
            (
                ">=19.12.2,<19.13.0",
                "widen",
                "19.12.2",
                "20.3.0",
                Some(">=19.12.2,<20.4.0"),
            ),
            (
                ">=19.12.2,<19.13.0",
                "widen",
                "19.12.2",
                "19.13.1",
                Some(">=19.12.2,<19.14.0"),
            ),
            (
                ">=19.12.2,<19.13.0",
                "widen",
                "19.12.2",
                "19.13.0",
                Some(">=19.12.2,<19.14.0"),
            ),
            ("~=7.2", "replace", "7.2.0", "8.0.1", Some("~=8.0")),
            ("~=7.2", "replace", "7.2.0", "8", Some("~=8.0")),
            ("~=7.2.0", "replace", "7.2.0", "8.2", Some("~=8.2.0")),
            ("~=7.2", "widen", "7.2.0", "8.0.1", Some(">=7.2,<9")),
            ("~=7.2", "widen", "7.2.0", "8", Some(">=7.2,<9")),
            ("~=7.2.0", "widen", "7.2.0", "8.2", Some(">=7.2.0,<8.3")),
            (
                "==3.2.*,>=3.2.2",
                "replace",
                "3.2.2",
                "4.1.1",
                Some("==4.1.*"),
            ),
            (
                "==3.2.*,>=3.2.2",
                "replace",
                "3.2.2",
                "4.0.0",
                Some("==4.0.*"),
            ),
            (
                ">=1.0.0,<1.1.0",
                "replace",
                "1.0.0",
                "1.2.0",
                Some(">=1.2.0,<1.3.0"),
            ),
            ("<1.3.0", "bump", "1.3.0", "0.9.2", Some("<1.3.0")),
            ("<1.3.0", "bump", "0.9.0", "0.9.2", Some("<1.3.0")),
            ("<=1.3.0", "bump", "0.9.0", "0.9.2", Some("<=1.3.0")),
            ("<=1.3.0", "bump", "1.3.0", "0.9.2", Some("<=1.3.0")),
            ("<1.3.0", "bump", "1.3.0", "1.6.0", Some("<1.6.1")),
            ("<1.3.0", "bump", "0.9.0", "1.6.0", Some("<1.6.1")),
            ("<=1.3.0", "bump", "0.9.0", "1.6.0", Some("<=1.6.0")),
            ("<=1.3.0", "bump", "1.3.0", "1.6.0", Some("<=1.6.0")),
        ];
        for &(cv, strategy, cur, nv, expected) in cases {
            let result = gnv(cv, strategy, cur, nv);
            assert_eq!(
                result.as_deref(),
                expected,
                "getNewValue({cv:?}, {strategy:?}, {cur:?}, {nv:?})"
            );
        }
    }

    // ── get_new_value (with isReplacement=true) ────────────────────────────────

    fn gnv_replace(cv: &str, strategy: &str, cur: &str, nv: &str) -> Option<String> {
        get_new_value(&NewValueParams {
            current_value: cv.to_owned(),
            range_strategy: strategy.to_owned(),
            current_version: cur.to_owned(),
            new_version: nv.to_owned(),
            is_replacement: true,
        })
    }

    // Ported: "getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected"" — lib/modules/versioning/pep440/index.spec.ts line 190
    #[test]
    fn get_new_value_replacement_table() {
        let cases: &[(&str, &str, &str, &str)] = &[
            ("1.0.0", "bump", "1.0.0", "1.2.3"),
            ("1.0.0", "replace", "1.0.0", "1.2.3"),
            ("1.0.0", "pin", "1.0.0", "1.2.3"),
            ("==1.0.3", "bump", "1.0.0", "1.2.3"),
            ("==1.0.3", "replace", "1.0.0", "1.2.3"),
            ("==1.0.3", "pin", "1.0.0", "1.2.3"),
            (">=1.2.0", "bump", "1.0.0", "1.2.3"),
            (">=1.2.0", "replace", "1.0.0", "1.2.3"),
            ("~=1.2.0", "bump", "1.0.0", "1.2.3"),
            ("~=1.2.0", "replace", "1.0.0", "1.2.3"),
            (">=1.2.0", "pin", "1.0.0", "1.2.3"),
            ("~=1.2.0", "pin", "1.0.0", "1.2.3"),
            ("~=1.0.3", "bump", "1.0.0", "1.2.3"),
            ("~=1.0.3", "replace", "1.0.0", "1.2.3"),
            ("~=1.0.3", "pin", "1.0.0", "1.2.3"),
            ("==1.2.*", "bump", "1.0.0", "1.2.3"),
            ("==1.2.*", "replace", "1.0.0", "1.2.3"),
            ("==1.2.*", "pin", "1.0.0", "1.2.3"),
            ("==1.0.*", "bump", "1.0.0", "1.2.3"),
            ("==1.0.*", "replace", "1.0.0", "1.2.3"),
            ("==1.0.*", "pin", "1.0.0", "1.2.3"),
            ("<1.2.2.3", "bump", "1.0.0", "1.2.3"),
            ("<1.2.2.3", "pin", "1.0.0", "1.2.3"),
            ("<1.2.3", "bump", "1.0.0", "1.2.3"),
            ("<1.2.3", "replace", "1.0.0", "1.2.3"),
            ("<1.2.3", "pin", "1.0.0", "1.2.3"),
            ("<1.2", "bump", "1.0.0", "1.2.3"),
            ("<1.2", "replace", "1.0.0", "1.2.3"),
            ("<1.2", "pin", "1.0.0", "1.2.3"),
            ("<1", "bump", "1.0.0", "1.2.3"),
            ("<1", "replace", "1.0.0", "1.2.3"),
            ("<1", "pin", "1.0.0", "1.2.3"),
            ("<2.0.0", "bump", "1.0.0", "1.2.3"),
            ("<2.0.0", "replace", "1.0.0", "1.2.3"),
            ("<2.0.0", "pin", "1.0.0", "1.2.3"),
            (">0.9.8", "bump", "1.0.0", "1.2.3"),
            (">0.9.8", "replace", "1.0.0", "1.2.3"),
            (">0.9.8", "pin", "1.0.0", "1.2.3"),
            (">2.0.0", "bump", "1.0.0", "1.2.3"),
            (">2.0.0", "replace", "1.0.0", "1.2.3"),
            (">2.0.0", "pin", "1.0.0", "1.2.3"),
            (">=2.0.0", "bump", "1.0.0", "1.2.3"),
            (">=2.0.0", "replace", "1.0.0", "1.2.3"),
            (">=2.0.0", "pin", "1.0.0", "1.2.3"),
            ("~=1.1.0, !=1.1.1", "bump", "1.0.0", "1.2.3"),
            ("~=1.1.0, !=1.1.1", "replace", "1.0.0", "1.2.3"),
            ("~=1.1.0, !=1.1.1", "pin", "1.0.0", "1.2.3"),
            ("~=1.1.0,!=1.1.1", "bump", "1.0.0", "1.2.3"),
            ("~=1.1.0,!=1.1.1", "replace", "1.0.0", "1.2.3"),
            ("~=1.1.0,!=1.1.1", "pin", "1.0.0", "1.2.3"),
            (" ", "bump", "1.0.0", "1.2.3"),
            (" ", "replace", "1.0.0", "1.2.3"),
            (" ", "pin", "1.0.0", "1.2.3"),
            ("invalid", "bump", "1.0.0", "1.2.3"),
            ("invalid", "replace", "1.0.0", "1.2.3"),
            ("invalid", "pin", "1.0.0", "1.2.3"),
            ("===1.0.3", "bump", "1.0.0", "1.2.3"),
            ("===1.0.3", "replace", "1.0.0", "1.2.3"),
            ("===1.0.3", "pin", "1.0.0", "1.2.3"),
            ("!=1.2.3", "bump", "1.0.0", "1.2.3"),
            ("!=1.2.3", "replace", "1.0.0", "1.2.3"),
            ("!=1.2.3", "pin", "1.0.0", "1.2.3"),
            ("~=1.1.0,!=1.1.1", "unsupported", "1.0.0", "1.2.3"),
            (">=19.12.2,<20.13.9", "replace", "19.12.2", "21.3.1"),
            (">=19.12.2,<19.13.9", "replace", "19.12.2", "20.3.1"),
            (">=19.12.2,<19.13.0", "replace", "19.12.2", "20.3.1"),
            (">=19.12.2,<19.13.0", "replace", "19.12.2", "20.3.0"),
            (">=19.12.2,<19.13.0", "replace", "19.12.2", "19.13.1"),
            (">=19.12.2,<19.13.0", "replace", "19.12.2", "19.13.0"),
            (">=19.12.2,<19.13.0", "auto", "19.12.2", "19.13.0"),
            (">=19.12.2,<20.13.9", "widen", "19.12.2", "21.3.1"),
            (">=19.12.2,<19.13.9", "widen", "19.12.2", "20.3.1"),
            (">=19.12.2,<19.13.0", "widen", "19.12.2", "20.3.1"),
            (">=19.12.2,<19.13.0", "widen", "19.12.2", "20.3.0"),
            (">=19.12.2,<19.13.0", "widen", "19.12.2", "19.13.1"),
            (">=19.12.2,<19.13.0", "widen", "19.12.2", "19.13.0"),
            ("~=7.2", "replace", "7.2.0", "8.0.1"),
            ("~=7.2", "replace", "7.2.0", "8"),
            ("~=7.2.0", "replace", "7.2.0", "8.2"),
            ("~=7.2", "widen", "7.2.0", "8.0.1"),
            ("~=7.2", "widen", "7.2.0", "8"),
            ("~=7.2.0", "widen", "7.2.0", "8.2"),
            ("==3.2.*,>=3.2.2", "replace", "3.2.2", "4.1.1"),
            ("==3.2.*,>=3.2.2", "replace", "3.2.2", "4.0.0"),
            (">=1.0.0,<1.1.0", "replace", "1.0.0", "1.2.0"),
            ("<1.3.0", "bump", "1.3.0", "0.9.2"),
            ("<1.3.0", "bump", "0.9.0", "0.9.2"),
            ("<=1.3.0", "bump", "0.9.0", "0.9.2"),
            ("<=1.3.0", "bump", "1.3.0", "0.9.2"),
            ("<1.3.0", "bump", "1.3.0", "1.6.0"),
            ("<1.3.0", "bump", "0.9.0", "1.6.0"),
            ("<=1.3.0", "bump", "0.9.0", "1.6.0"),
            ("<=1.3.0", "bump", "1.3.0", "1.6.0"),
            ("1.0.0", "bump", "1.0.0", "==1.2.3"),
            ("1.0.0", "bump", "1.0.0", ">=1.2.3"),
            ("1.0.0", "bump", "1.0.0", "<=1.2.3"),
            ("1.0.0", "bump", "1.0.0", "~=1.2.3"),
            ("1.0.0", "bump", "1.0.0", "!=1.2.3"),
            ("1.0.0", "bump", "1.0.0", ">1.2.3"),
            ("1.0.0", "bump", "1.0.0", "<1.2.3"),
        ];
        for &(cv, strategy, cur, nv) in cases {
            let result = gnv_replace(cv, strategy, cur, nv);
            assert_eq!(
                result.as_deref(),
                Some(nv),
                "getNewValue(isReplacement=true, {cv:?}, {strategy:?}, {cur:?}, {nv:?})"
            );
        }
    }

    // ── is_less_than_range ────────────────────────────────────────────────────

    // Ported: "isLessThanRange("$version", "$range") === "$expected"" — lib/modules/versioning/pep440/index.spec.ts line 307
    #[test]
    fn is_less_than_range_table() {
        let cases: &[(&str, &str, bool)] = &[
            ("0.9.9.9", ">= 1.0.0, < 2.0.0", true),
            ("1.0.0a0", ">= 1.0.0, < 2.0.0", true),
            ("1.0.0.0", "> 1.0.0, < 2.0.0", true),
            ("2.0.1.0", "> 1.0.0, < 2.0.0", false),
            ("2.0.0.0", "> 1.0.0, < 2.0.0", false),
            ("2.0.0a0", "> 1.0.0, < 2.0.0", false),
            ("1.2.2.9", "== 1.2.3", true),
            ("1.2.3a0", "== 1.2.3", true),
            ("1.2.3.0", "== 1.2.3", false),
            ("1.2.3.1", "== 1.2.3", false),
            ("1.2.4a0", "== 1.2.3", false),
            ("1.2.2.9", "!= 1.2.3", false),
            ("1.2.3.0", "!= 1.2.3", false),
            ("1.2.3.1", "!= 1.2.3", false),
            ("0.0.1", "< 1.0.0", false),
            ("1.0.0", "< 1.0.0", false),
            ("2.0.0", "< 1.0.0", false),
            ("0.0.1", "<= 1.0.0", false),
            ("1.0.0", "<= 1.0.0", false),
            ("2.0.0", "<= 1.0.0", false),
            ("0.0.1", "< 1.0.0, > 2.0.0", true),
            ("3.0.0", "< 1.0.0, > 2.0.0", false),
        ];
        for &(ver, range, expected) in cases {
            assert_eq!(
                is_less_than_range(ver, range),
                expected,
                "isLessThanRange({ver:?}, {range:?})"
            );
        }
    }

    // ── check_range_and_remove_unnecessary_range_limit ────────────────────────

    // Ported: "checkRange("$rangeInput, "$newVersion"") === "$expected"" — lib/modules/versioning/pep440/range.spec.ts line 8
    #[test]
    fn check_range_table() {
        assert_eq!(
            check_range_and_remove_unnecessary_range_limit("==4.1.*,>=3.2.2", "4.1.1"),
            "==4.1.*"
        );
        assert_eq!(
            check_range_and_remove_unnecessary_range_limit("==4.0.*,>=3.2.2", "4.0.0"),
            "==4.0.*"
        );
        assert_eq!(
            check_range_and_remove_unnecessary_range_limit("==7.2.*", "7.2.0"),
            "==7.2.*"
        );
    }

    // Ported: "returns null without warning if new version is excluded from range" — lib/modules/versioning/pep440/range.spec.ts line 24
    #[test]
    fn excluded_version_returns_none() {
        let result = get_new_value(&NewValueParams {
            current_value: ">=1.25.0,<2,!=1.32.0".to_owned(),
            range_strategy: "auto".to_owned(),
            new_version: "1.32.0".to_owned(),
            current_version: "1.25.0".to_owned(),
            is_replacement: false,
        });
        assert!(result.is_none());
    }

    // Ported: "handles v-prefixed version as currentValue" — lib/modules/versioning/pep440/range.spec.ts line 39
    #[test]
    fn v_prefix_preserved() {
        let result = get_new_value(&NewValueParams {
            current_value: "v0.7.15".to_owned(),
            range_strategy: "auto".to_owned(),
            new_version: "0.8.0".to_owned(),
            current_version: "0.7.15".to_owned(),
            is_replacement: false,
        });
        assert_eq!(result.as_deref(), Some("v0.8.0"));
    }

    // Ported: "handles bare version that differs from currentVersion without v-prefix" — lib/modules/versioning/pep440/range.spec.ts line 49
    #[test]
    fn bare_version_differs_from_current() {
        let result = get_new_value(&NewValueParams {
            current_value: "1.0.0.0".to_owned(),
            range_strategy: "auto".to_owned(),
            new_version: "1.2.3".to_owned(),
            current_version: "1.0.0".to_owned(),
            is_replacement: false,
        });
        assert_eq!(result.as_deref(), Some("1.2.3"));
    }

    #[test]
    fn is_greater_than_pep440() {
        assert!(is_greater_than("2.0.0", "1.0.0"));
        assert!(!is_greater_than("1.0.0", "2.0.0"));
        assert!(!is_greater_than("1.0.0", "1.0.0"));
    }

    #[test]
    fn sort_versions_pep440() {
        use std::cmp::Ordering;
        assert_eq!(sort_versions("1.0.0", "1.0.0"), Ordering::Equal);
        assert_eq!(sort_versions("2.0.0", "1.0.0"), Ordering::Greater);
        assert_eq!(sort_versions("1.0.0", "2.0.0"), Ordering::Less);
    }
}
