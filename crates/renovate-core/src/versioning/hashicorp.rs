//! HashiCorp versioning and update-decision logic.
//!
//! Used by Terraform providers and modules. The constraint syntax is a superset
//! of semver ranges with an additional `~>` (pessimistic constraint) operator.
//!
//! Renovate reference:
//! - `lib/modules/versioning/hashicorp/index.ts`
//! - `lib/modules/versioning/hashicorp/convertor.ts` — `hashicorp2npm`
//!
//! ## Operator semantics
//!
//! | Constraint | Meaning |
//! |---|---|
//! | `= 5.0.0` | Exactly 5.0.0 |
//! | `!= 5.0.0` | Any version except 5.0.0 (treated as unconstrained here) |
//! | `>= 5.0.0` | At least 5.0.0 |
//! | `<= 5.0.0` | At most 5.0.0 |
//! | `> 5.0.0` | Strictly newer than 5.0.0 |
//! | `< 5.0.0` | Strictly older than 5.0.0 |
//! | `~> 5.0` | `>= 5.0.0, < 6.0.0` (minor-level pessimistic) |
//! | `~> 5.0.1` | `>= 5.0.1, < 5.1.0` (patch-level pessimistic) |
//! | `~> 5` | `>= 5.0.0` (major-only: no upper bound) |
//!
//! ## Update decision
//!
//! For range constraints, the pinned lower-bound version is compared to latest.
//! An update is available when `latest > lower_bound` by semver ordering.

use std::sync::LazyLock;

use regex::Regex;
use semver::{Version, VersionReq};

/// Regex to parse a single HashiCorp constraint element.
/// Group 1 = operator (`~>`, `>=`, `<=`, `>`, `<`, `=`, `!=`, or empty).
/// Group 2 = version string (digits and dots).
static CONSTRAINT_ELEM: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s*(?<op>~>|>=|<=|!=|>|<|=)?\s*v?(?<ver>[\d]+(?:\.[\d]+(?:\.[\d]+(?:[-.][0-9A-Za-z]+)*)?)?)?\s*$").unwrap()
});

/// Parsed single constraint element.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ConstraintElem {
    op: Op,
    version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    Pessimistic, // ~>
    Gte,         // >=
    Lte,         // <=
    Gt,          // >
    Lt,          // <
    Eq,          // = (or no operator)
    Neq,         // !=
}

/// Parse all comma-separated elements of a HashiCorp constraint string.
fn parse_constraint(constraint: &str) -> Vec<ConstraintElem> {
    constraint
        .split(',')
        .filter_map(|part| {
            let cap = CONSTRAINT_ELEM.captures(part.trim())?;
            let op_str = cap.name("op").map(|m| m.as_str()).unwrap_or("");
            let ver = cap.name("ver").map(|m| m.as_str()).unwrap_or("").to_owned();
            if ver.is_empty() {
                return None;
            }
            let op = match op_str {
                "~>" => Op::Pessimistic,
                ">=" => Op::Gte,
                "<=" => Op::Lte,
                ">" => Op::Gt,
                "<" => Op::Lt,
                "!=" => Op::Neq,
                _ => Op::Eq,
            };
            Some(ConstraintElem { op, version: ver })
        })
        .collect()
}

/// Extract the lower-bound version from a HashiCorp constraint.
///
/// For `~> 5.0.1` → `5.0.1`, `>= 2.0.0` → `2.0.0`, `= 3.0` → `3.0`.
/// For multi-element constraints (`>= 1.0, < 2.0`), returns the lower bound
/// of the first `>=`, `>`, or `~>` element, or the first `=` element.
pub fn lower_bound(constraint: &str) -> Option<String> {
    let elems = parse_constraint(constraint);
    // Prefer the element with a lower-bounding operator.
    for elem in &elems {
        match elem.op {
            Op::Pessimistic | Op::Gte | Op::Gt | Op::Eq => return Some(elem.version.clone()),
            _ => {}
        }
    }
    None
}

/// Parse a version string as semver, padding missing components with 0.
fn parse_version(v: &str) -> Option<Version> {
    // Pad to 3 components.
    let parts: Vec<&str> = v.splitn(3, '.').collect();
    let padded = match parts.len() {
        1 => format!("{}.0.0", parts[0]),
        2 => format!("{}.{}.0", parts[0], parts[1]),
        _ => v.to_owned(),
    };
    Version::parse(&padded).ok()
}

/// Update summary produced by [`hashicorp_update_summary`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashicorpUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Produce an update summary for a HashiCorp-versioned dependency.
///
/// `update_available` is `true` when `latest` is strictly newer than the
/// lower bound extracted from `current_value`, by semver comparison.
pub fn hashicorp_update_summary(
    current_value: &str,
    latest: Option<&str>,
) -> HashicorpUpdateSummary {
    let update_available = latest
        .filter(|l| !l.is_empty() && !current_value.is_empty())
        .is_some_and(|latest_ver| {
            let Some(lb) = lower_bound(current_value) else {
                return false;
            };
            let Some(lv) = parse_version(latest_ver) else {
                return false;
            };
            let Some(cv) = parse_version(&lb) else {
                return false;
            };
            lv > cv
        });

    HashicorpUpdateSummary {
        current_value: current_value.to_owned(),
        latest: latest.map(|s| s.to_owned()),
        update_available,
    }
}

// ── Public API matching lib/modules/versioning/hashicorp/index.ts ──────────

/// Pad a version string to 3 dot-separated components.
fn pad_to_semver(v: &str) -> String {
    let parts: Vec<&str> = v.splitn(3, '.').collect();
    match parts.len() {
        1 => format!("{}.0.0", parts[0]),
        2 => format!("{}.{}.0", parts[0], parts[1]),
        _ => v.to_owned(),
    }
}

/// Parse a version string into (major, minor, patch).
fn parse_ver(v: &str) -> Option<(u64, u64, u64)> {
    let sv = Version::parse(&pad_to_semver(v.trim_start_matches('v'))).ok()?;
    Some((sv.major, sv.minor, sv.patch))
}

/// Parse operator and version from a single npm range element.
fn npm_split_op_ver(part: &str) -> (&str, &str) {
    for op in &[">=", "<=", ">", "<", "^", "~", "="] {
        if let Some(rest) = part.strip_prefix(op) {
            return (op, rest);
        }
    }
    ("", part)
}

/// Convert hashicorp constraint to npm range string.
///
/// Returns `None` on invalid hashicorp input (e.g. contains `!=` or an
/// unrecognised operator like `^`) or on a version string that does not
/// start with a digit.  Returns `Some("")` for an empty input.
pub fn hashicorp2npm(input: &str) -> Option<String> {
    if input.is_empty() {
        return Some(String::new());
    }
    let mut parts = Vec::new();
    for segment in input.split(',') {
        let segment = segment.trim();
        // Parse operator
        let (op, ver) = {
            let ops = ["~>", ">=", "<=", "!=", ">", "<", "="];
            let mut found = ("", segment);
            for o in ops {
                if let Some(rest) = segment.strip_prefix(o) {
                    found = (o, rest.trim().trim_start_matches('v'));
                    break;
                }
            }
            if found.0.is_empty() {
                ("", segment.trim().trim_start_matches('v'))
            } else {
                found
            }
        };
        if op == "!=" {
            return None;
        }
        let npm_part = match op {
            "=" | "" => {
                // Bare version: must start with a digit (e.g. "4.2.0").
                // Strings like "^4" or "4.x.x" are not valid hashicorp input.
                if !ver.starts_with(|c: char| c.is_ascii_digit()) {
                    return None;
                }
                ver.to_owned()
            }
            "~>" => {
                let n_dots = ver.chars().filter(|&c| c == '.').count();
                match n_dots {
                    0 => format!(">={ver}"),
                    1 => format!("^{ver}"),
                    _ => format!("~{ver}"),
                }
            }
            _ => format!("{op}{ver}"),
        };
        parts.push(npm_part);
    }
    Some(parts.join(" "))
}

/// Convert npm range string back to hashicorp constraint syntax.
///
/// Returns `None` for unsupported npm ranges (e.g. `"4.x.x"`).
/// Returns `Some("")` for an empty input.
pub fn npm2hashicorp(input: &str) -> Option<String> {
    if input.is_empty() {
        return Some(String::new());
    }
    let parts: Vec<&str> = input.split_whitespace().collect();
    let mut result = Vec::new();
    for part in parts {
        let (op, ver) = npm_split_op_ver(part);
        let hashi = match op {
            "^" => {
                let n_dots = ver.chars().filter(|&c| c == '.').count();
                if n_dots == 0 {
                    format!("~> {ver}.0")
                } else if n_dots == 1 {
                    // ^X.Y → ~> X.Y
                    format!("~> {ver}")
                } else {
                    // ^X.Y.Z
                    let parts: Vec<&str> = ver.splitn(3, '.').collect();
                    if parts[2] == "0" {
                        // ^X.Y.0 → ~> X.Y
                        format!("~> {}.{}", parts[0], parts[1])
                    } else {
                        // ^X.Y.Z (non-zero patch) → ~> X.Y
                        format!("~> {}.{}", parts[0], parts[1])
                    }
                }
            }
            "~" => {
                let n_dots = ver.chars().filter(|&c| c == '.').count();
                if n_dots == 0 {
                    format!("~> {ver}.0")
                } else if n_dots == 1 {
                    // ~X.Y → ~> X.Y.0
                    format!("~> {ver}.0")
                } else {
                    // ~X.Y.Z → ~> X.Y.Z
                    format!("~> {ver}")
                }
            }
            "" => {
                // Bare version: strip leading `v`, reject wildcard ranges.
                let v = ver.trim_start_matches('v');
                if v.contains('x') || v.contains('X') || v.contains('*') {
                    return None;
                }
                v.to_owned()
            }
            ">=" | "<=" | ">" | "<" => format!("{op} {ver}"),
            _ => return None,
        };
        result.push(hashi);
    }
    Some(result.join(", "))
}

/// Extract `!=` versions from a hashicorp constraint.
pub fn get_excluded_versions(range: &str) -> Vec<String> {
    range
        .split(',')
        .map(str::trim)
        .filter(|p| p.starts_with("!="))
        .map(|p| p.trim_start_matches("!=").trim().to_owned())
        .collect()
}

/// Return constraint with `!=` elements removed.
pub fn get_filtered_range(range: &str) -> String {
    range
        .split(',')
        .map(str::trim)
        .filter(|p| !p.starts_with("!="))
        .collect::<Vec<_>>()
        .join(",")
}

/// Check if `version` satisfies an npm range string.
fn npm_satisfies(version: &str, npm_range: &str) -> bool {
    let ver_str = pad_to_semver(version.trim_start_matches('v'));
    let Ok(ver) = Version::parse(&ver_str) else {
        return false;
    };
    // Normalize npm range: space-separated comparators → comma-separated
    let req_str = npm_range.split_whitespace().collect::<Vec<_>>().join(", ");
    let Ok(req) = VersionReq::parse(&req_str) else {
        return false;
    };
    req.matches(&ver)
}

/// `replaceCaretValue` from npm/range.ts: compute new caret lower bound.
fn replace_caret_value(old_ver: &str, new_ver: &str) -> String {
    let (Some((old_maj, old_min, old_pat)), Some((new_maj, new_min, new_pat))) =
        (parse_ver(old_ver), parse_ver(new_ver))
    else {
        return new_ver.to_owned();
    };
    let old_t = [old_maj, old_min, old_pat];
    let new_t = [new_maj, new_min, new_pat];
    let mut result = [0u64; 3];
    let mut leading_zero = true;
    let mut need_replace = false;
    for i in 0..3 {
        let ov = old_t[i];
        let nv = new_t[i];
        let leading_digit = (ov != 0 || nv != 0) && std::mem::take(&mut leading_zero);
        if leading_digit && nv > ov {
            need_replace = true;
        }
        if !need_replace && nv < ov {
            return new_ver.to_owned();
        }
        result[i] = if leading_digit { nv } else { 0 };
    }
    if need_replace {
        format!("{}.{}.{}", result[0], result[1], result[2])
    } else {
        old_ver.to_owned()
    }
}

/// npm getNewValue for `replace` strategy (single-element range).
fn npm_replace_single(op: &str, _ver: &str, current_ver: &str, new_ver: &str) -> Option<String> {
    let (new_maj, new_min, _new_pat) = parse_ver(new_ver)?;
    match op {
        "^" => {
            let result = replace_caret_value(current_ver, new_ver);
            Some(format!("^{result}"))
        }
        "~" => Some(format!("~{new_maj}.{new_min}.0")),
        "" | "=" => {
            // bare version: X.Y → major.minor
            let n_dots = _ver.chars().filter(|&c| c == '.').count();
            match n_dots {
                0 => Some(format!("{new_maj}")),
                1 => Some(format!("{new_maj}.{new_min}")),
                _ => Some(new_ver.trim_start_matches('v').to_owned()),
            }
        }
        "<=" => {
            let n_dots = _ver.chars().filter(|&c| c == '.').count();
            if n_dots >= 2 {
                Some(format!("<={}", new_ver.trim_start_matches('v')))
            } else if n_dots == 1 {
                Some(format!("<={new_maj}.{new_min}"))
            } else {
                Some(format!("<={new_maj}"))
            }
        }
        _ => None,
    }
}

/// npm getNewValue — `rangeStrategy` is one of replace/bump/widen/update-lockfile.
fn npm_get_new_value(
    npm_range: &str,
    range_strategy: &str,
    current_ver: &str,
    new_ver: &str,
) -> Option<String> {
    // Split npm_range into whitespace-separated elements to find last operator
    let elements: Vec<&str> = npm_range.split_whitespace().collect();
    let last_elem = elements.last()?;
    let (last_op, last_ver) = npm_split_op_ver(last_elem);

    match range_strategy {
        "update-lockfile" => {
            if npm_satisfies(new_ver, npm_range) {
                return Some(npm_range.to_owned());
            }
            npm_get_new_value(npm_range, "replace", current_ver, new_ver)
        }
        "bump" => {
            let stripped = new_ver.trim_start_matches('v');
            match last_op {
                "~" => Some(format!("~{stripped}")),
                "^" => Some(format!("^{stripped}")),
                ">=" => {
                    if npm_range.contains(' ') {
                        Some(npm_range.to_owned())
                    } else {
                        Some(format!(">={stripped}"))
                    }
                }
                "" => Some(stripped.to_owned()),
                _ => None,
            }
        }
        "widen" => {
            if npm_satisfies(new_ver, npm_range) {
                return Some(npm_range.to_owned());
            }
            // For multi-element ranges with a `<=` last element
            if last_op.starts_with('<') && elements.len() > 1 {
                let replace_result = npm_replace_single(last_op, last_ver, current_ver, new_ver)?;
                // Rebuild: drop last element, append new result
                let prefix_elems = &elements[..elements.len() - 1];
                let prefix = prefix_elems.join(" ");
                return Some(format!("{prefix} {replace_result}"));
            }
            npm_get_new_value(npm_range, "replace", current_ver, new_ver)
        }
        _ => {
            // replace
            if elements.len() == 1 {
                npm_replace_single(last_op, last_ver, current_ver, new_ver)
            } else {
                // Multi-element: handle the last element
                let replace_result = npm_replace_single(last_op, last_ver, current_ver, new_ver)?;
                let prefix_elems = &elements[..elements.len() - 1];
                let prefix = prefix_elems.join(" ");
                Some(format!("{prefix} {replace_result}"))
            }
        }
    }
}

/// Validate a hashicorp constraint string.
pub fn is_valid(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    // Check each comma-separated element
    for segment in input.split(',') {
        let segment = segment.trim();
        let ops = ["~>", ">=", "<=", "!=", ">", "<", "="];
        let mut found_op = false;
        let mut ver = segment;
        for op in ops {
            if let Some(rest) = segment.strip_prefix(op) {
                if op == "!=" {
                    return false; // != alone is unsupported
                }
                found_op = true;
                ver = rest.trim().trim_start_matches('v');
                break;
            }
        }
        if !found_op {
            ver = segment.trim().trim_start_matches('v');
        }
        // Validate the version part
        if pad_to_semver(ver).parse::<Version>().is_err() {
            return false;
        }
    }
    true
}

/// Check if `version` satisfies a hashicorp constraint.
pub fn matches(version: &str, range: &str) -> bool {
    let excluded = get_excluded_versions(range);
    if excluded.iter().any(|e| e == version) {
        return false;
    }
    let filtered = get_filtered_range(range);
    let Some(npm) = hashicorp2npm(&filtered) else {
        return false;
    };
    npm_satisfies(version, &npm)
}

/// Return the maximum version from `versions` that satisfies `range`.
pub fn get_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    let excluded = get_excluded_versions(range);
    let filtered = get_filtered_range(range);
    let npm = hashicorp2npm(&filtered)?;
    versions
        .iter()
        .filter(|&&v| !excluded.iter().any(|e| e == v))
        .filter(|&&v| npm_satisfies(v, &npm))
        .max_by(|&&a, &&b| {
            let va = Version::parse(&pad_to_semver(a)).unwrap_or(Version::new(0, 0, 0));
            let vb = Version::parse(&pad_to_semver(b)).unwrap_or(Version::new(0, 0, 0));
            va.cmp(&vb)
        })
        .copied()
}

/// Return the minimum version from `versions` that satisfies `range`.
pub fn min_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    let excluded = get_excluded_versions(range);
    let filtered = get_filtered_range(range);
    let npm = hashicorp2npm(&filtered)?;
    versions
        .iter()
        .filter(|&&v| !excluded.iter().any(|e| e == v))
        .filter(|&&v| npm_satisfies(v, &npm))
        .min_by(|&&a, &&b| {
            let va = Version::parse(&pad_to_semver(a)).unwrap_or(Version::new(0, 0, 0));
            let vb = Version::parse(&pad_to_semver(b)).unwrap_or(Version::new(0, 0, 0));
            va.cmp(&vb)
        })
        .copied()
}

/// Return `true` if `version` is less than all versions in the hashicorp `range`.
pub fn is_less_than_range(version: &str, range: &str) -> bool {
    let excluded = get_excluded_versions(range);
    if excluded.iter().any(|e| e == version) {
        return false;
    }
    let filtered = get_filtered_range(range);
    let Some(npm) = hashicorp2npm(&filtered) else {
        return false;
    };
    if npm_satisfies(version, &npm) {
        return false;
    }
    // Version doesn't satisfy range. Check if it's below the lower bound.
    let lb = {
        let elems: Vec<&str> = npm.split_whitespace().collect();
        // Find first gte/gt element
        let mut bound = None;
        for e in &elems {
            let (op, ver) = npm_split_op_ver(e);
            if op == ">=" || op == ">" || op.is_empty() {
                bound = Version::parse(&pad_to_semver(ver)).ok();
                break;
            }
        }
        bound
    };
    let ver_padded = pad_to_semver(version.trim_start_matches('v'));
    let Ok(ver) = Version::parse(&ver_padded) else {
        return false;
    };
    lb.map(|lb| ver < lb).unwrap_or(false)
}

/// Compute a new hashicorp constraint value.
pub fn get_new_value(
    current_value: &str,
    range_strategy: &str,
    current_version: &str,
    new_version: &str,
) -> Option<String> {
    let has_v = current_value.starts_with('v');
    let cv = current_value.trim_start_matches('v');
    let nv = new_version.trim_start_matches('v');
    let cur = current_version.trim_start_matches('v');

    let npm_range = hashicorp2npm(cv)?;
    let npm_result = npm_get_new_value(&npm_range, range_strategy, cur, nv)?;
    let mut hashi = npm2hashicorp(&npm_result)?;

    if has_v && !hashi.starts_with('v') {
        hashi = format!("v{hashi}");
    }
    Some(hashi)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Rust-specific: unit tests for lower_bound helper
    #[test]
    fn lower_bound_pessimistic_patch() {
        assert_eq!(lower_bound("~> 5.0.1"), Some("5.0.1".to_owned()));
    }

    // Rust-specific: unit tests for lower_bound helper
    #[test]
    fn lower_bound_pessimistic_minor() {
        assert_eq!(lower_bound("~> 5.0"), Some("5.0".to_owned()));
    }

    // Rust-specific: unit tests for lower_bound helper
    #[test]
    fn lower_bound_pessimistic_major_only() {
        assert_eq!(lower_bound("~> 5"), Some("5".to_owned()));
    }

    // Rust-specific: unit tests for lower_bound helper
    #[test]
    fn lower_bound_gte() {
        assert_eq!(lower_bound(">= 2.0.0"), Some("2.0.0".to_owned()));
    }

    // Rust-specific: unit tests for lower_bound helper
    #[test]
    fn lower_bound_exact() {
        assert_eq!(lower_bound("= 3.1.4"), Some("3.1.4".to_owned()));
    }

    // Rust-specific: unit tests for lower_bound helper
    #[test]
    fn lower_bound_bare_version() {
        assert_eq!(lower_bound("3.1.4"), Some("3.1.4".to_owned()));
    }

    // Rust-specific: unit tests for lower_bound helper
    #[test]
    fn lower_bound_range() {
        // `>= 1.0, < 2.0` — lower bound is 1.0
        assert_eq!(lower_bound(">= 1.0, < 2.0"), Some("1.0".to_owned()));
    }

    // Rust-specific: unit tests for hashicorp_update_summary edge cases
    #[test]
    fn update_available_when_newer() {
        let s = hashicorp_update_summary("~> 5.0", Some("5.7.3"));
        assert!(s.update_available);
        assert_eq!(s.latest.as_deref(), Some("5.7.3"));
    }

    // Rust-specific: unit tests for hashicorp_update_summary edge cases
    #[test]
    fn no_update_when_same_lower_bound() {
        // Lower bound is 5.0.1; latest is also 5.0.1
        let s = hashicorp_update_summary("~> 5.0.1", Some("5.0.1"));
        assert!(!s.update_available);
    }

    // Rust-specific: unit tests for hashicorp_update_summary edge cases
    #[test]
    fn no_update_when_older() {
        // Lower bound 5.0, latest 4.9.9 (unlikely but defensive)
        let s = hashicorp_update_summary("~> 5.0", Some("4.9.9"));
        assert!(!s.update_available);
    }

    // Rust-specific: unit tests for hashicorp_update_summary edge cases
    #[test]
    fn no_update_when_latest_none() {
        let s = hashicorp_update_summary("~> 5.0", None);
        assert!(!s.update_available);
    }

    // Rust-specific: unit tests for hashicorp_update_summary edge cases
    #[test]
    fn exact_pinned_update() {
        let s = hashicorp_update_summary("= 5.0.0", Some("5.1.0"));
        assert!(s.update_available);
    }

    // Rust-specific: unit tests for hashicorp_update_summary edge cases
    #[test]
    fn gte_constraint_update() {
        let s = hashicorp_update_summary(">= 4.0.0", Some("5.0.0"));
        assert!(s.update_available);
    }

    // Rust-specific: unit tests for hashicorp_update_summary edge cases
    #[test]
    fn multi_element_constraint() {
        // >= 2.0.0, < 3.0.0 — lower bound 2.0.0
        let s = hashicorp_update_summary(">= 2.0.0, < 3.0.0", Some("2.5.0"));
        assert!(s.update_available);
    }

    // Rust-specific: unit tests for hashicorp_update_summary edge cases
    #[test]
    fn version_padding() {
        // ~> 5 → lower bound 5.0.0; latest 5.1.0 → update
        let s = hashicorp_update_summary("~> 5", Some("5.1.0"));
        assert!(s.update_available);
    }

    // Ported: "matches(\"$version\", \"$range\") === $expected" — versioning/hashicorp/index.spec.ts line 4
    #[test]
    fn matches_matches_renovate_hashicorp_spec() {
        let cases = [
            ("4.2.0", "~> 4.0", true),
            ("4.2.0", "~> 4.0.0", false),
            ("4.2.0", "~> 4.0, != 4.2.0", false),
            ("4.2.6", "~> 4.0, != 4.2.0", true),
        ];
        for (version, range, expected) in cases {
            assert_eq!(
                matches(version, range),
                expected,
                "matches({version:?}, {range:?})"
            );
        }
    }

    // Ported: "getSatisfyingVersion($versions, \"$range\") === $expected" — versioning/hashicorp/index.spec.ts line 17
    #[test]
    fn get_satisfying_version_matches_renovate_hashicorp_spec() {
        let versions = ["0.4.0", "0.5.0", "4.0.0", "4.2.0", "5.0.0"];
        assert_eq!(get_satisfying_version(&versions, "~> 4.0"), Some("4.2.0"));
        assert_eq!(get_satisfying_version(&versions, "~> 4.0.0"), Some("4.0.0"));
        assert_eq!(
            get_satisfying_version(&versions, "!=4.2.0, > 4.0.0"),
            Some("5.0.0")
        );
    }

    // Ported: "isValid(\"$input\") === $expected" — versioning/hashicorp/index.spec.ts line 29
    #[test]
    fn is_valid_matches_renovate_hashicorp_spec() {
        let cases = [
            (">= 1.0.0, <= 2.0.0", true),
            ("~> 4", true),
            ("~> 4.0", true),
            ("~> 4.1", true),
            ("~> 4.1.2", true),
            ("=4", true),
            ("=4.0", true),
            ("!=4.0", false),
            (">=4.1", true),
            ("<=4.1.2", true),
            ("", false),
            ("0.1.0-beta.0", true),
        ];
        for (input, expected) in cases {
            assert_eq!(is_valid(input), expected, "is_valid({input:?})");
        }
    }

    // Ported: "isLessThanRange($version, $range) === $expected" — versioning/hashicorp/index.spec.ts line 48
    #[test]
    fn is_less_than_range_matches_renovate_hashicorp_spec() {
        assert!(is_less_than_range("0.9.0", ">= 1.0.0, <= 2.0.0"));
        assert!(!is_less_than_range("1.9.0", ">= 1.0.0, <= 2.0.0"));
    }

    // Ported: "minSatisfyingVersion($versions, \"$range\") === $expected" — versioning/hashicorp/index.spec.ts line 59
    #[test]
    fn min_satisfying_version_matches_renovate_hashicorp_spec() {
        let v1 = ["0.4.0", "0.5.0", "4.2.0", "5.0.0"];
        assert_eq!(min_satisfying_version(&v1, "~> 4.0"), Some("4.2.0"));
        assert_eq!(min_satisfying_version(&v1, "~> 4.0.0"), None);
        assert_eq!(min_satisfying_version(&v1, "~> 4.0, != 4.2.0"), None);
        let v2 = ["0.4.0", "0.5.0", "4.2.0", "4.1.0"];
        assert_eq!(
            min_satisfying_version(&v2, "~> 4.0, != 4.2.0"),
            Some("4.1.0")
        );
    }

    // Ported: "getNewValue(\"$currentValue\", \"$rangeStrategy\", \"$currentVersion\", \"$newVersion\") === \"$expected\"" — versioning/hashicorp/index.spec.ts line 72
    #[test]
    fn get_new_value_matches_renovate_hashicorp_spec() {
        let cases: &[(&str, &str, &str, &str, Option<&str>)] = &[
            ("~> 1.2", "replace", "1.2.3", "2.0.7", Some("~> 2.0")),
            ("~> 1.2.0", "replace", "1.2.3", "2.0.7", Some("~> 2.0.0")),
            ("~> 1.2", "replace", "1.2.3", "1.2.3", Some("~> 1.2")),
            ("~> 1.2", "replace", "1.2.3", "1.2.4", Some("~> 1.2")),
            ("~> 1.2.0", "replace", "1.2.3", "1.2.3", Some("~> 1.2.0")),
            (
                "~> 0.14.0",
                "replace",
                "0.14.1",
                "0.15.0",
                Some("~> 0.15.0"),
            ),
            (
                "~> 0.14.0",
                "replace",
                "0.14.1",
                "0.15.1",
                Some("~> 0.15.0"),
            ),
            (
                "~> 0.14.6",
                "replace",
                "0.14.6",
                "0.15.0",
                Some("~> 0.15.0"),
            ),
            (
                "~> 0.14.0",
                "replace",
                "0.14.1",
                "0.14.2",
                Some("~> 0.14.0"),
            ),
            (
                "~> 0.14.6",
                "replace",
                "0.14.6",
                "0.14.7",
                Some("~> 0.14.0"),
            ),
            ("~> 2.3.4", "replace", "2.3.4", "2.3.5", Some("~> 2.3.0")),
            ("~> 0.14.0", "bump", "0.14.1", "0.14.2", Some("~> 0.14.2")),
            ("~> 0.14.6", "bump", "0.14.6", "0.14.7", Some("~> 0.14.7")),
            ("~> 0.14.6", "bump", "0.14.6", "0.15.1", Some("~> 0.15.1")),
            ("~> 0.14.6", "bump", "0.14.6", "2.0.7", Some("~> 2.0.7")),
            (
                ">= 1.0.0, <= 2.0.0",
                "widen",
                "1.2.3",
                "2.0.7",
                Some(">= 1.0.0, <= 2.0.7"),
            ),
            ("0.14", "replace", "0.14.2", "0.15.0", Some("0.15")),
            ("~> 0.14", "replace", "0.14.2", "0.15.0", Some("~> 0.15")),
            (
                "~> 0.14",
                "update-lockfile",
                "0.14.2",
                "0.14.6",
                Some("~> 0.14"),
            ),
            (
                "~> 0.14",
                "update-lockfile",
                "0.14.2",
                "0.15.0",
                Some("~> 0.15"),
            ),
            (
                "~> 2.62.0",
                "update-lockfile",
                "2.62.0",
                "2.62.1",
                Some("~> 2.62.0"),
            ),
            (
                "~> 2.62.0",
                "update-lockfile",
                "2.62.0",
                "2.67.0",
                Some("~> 2.67.0"),
            ),
            ("v0.14", "replace", "v0.14.2", "v0.15.0", Some("v0.15")),
        ];
        for &(cv, strategy, cur, nv, expected) in cases {
            let result = get_new_value(cv, strategy, cur, nv);
            assert_eq!(
                result.as_deref(),
                expected,
                "get_new_value({cv:?}, {strategy:?}, {cur:?}, {nv:?})"
            );
        }
    }

    // Ported: "hashicorp2npm(\"$hashicorp\") === $npm && npm2hashicorp(\"$npm\") === $hashicorp" — versioning/hashicorp/convertor.spec.ts line 4
    #[test]
    fn hashicorp2npm_and_npm2hashicorp_roundtrip_matches_renovate_hashicorp_convertor_spec() {
        let cases: &[(&str, &str)] = &[
            ("", ""),
            ("4.2.0", "4.2.0"),
            ("4.2.0-alpha", "4.2.0-alpha"),
            ("~> 4.0", "^4.0"),
            ("~> 4.1", "^4.1"),
            ("~> 4.0.0", "~4.0.0"),
            ("~> 4.0.1", "~4.0.1"),
            ("~> 4.1.0", "~4.1.0"),
            ("~> 4.1.1", "~4.1.1"),
            ("~> 4.0.0-alpha", "~4.0.0-alpha"),
            (">= 4.0", ">=4.0"),
            ("<= 4.0", "<=4.0"),
            ("> 4.0", ">4.0"),
            ("< 4.0", "<4.0"),
            ("> 4.0, < 5.0", ">4.0 <5.0"),
            ("~> 2.3.4", "~2.3.4"),
            ("0.1.0-beta.0", "0.1.0-beta.0"),
        ];
        for &(hashi, npm) in cases {
            assert_eq!(
                hashicorp2npm(hashi).as_deref(),
                Some(npm),
                "hashicorp2npm({hashi:?})"
            );
            assert_eq!(
                npm2hashicorp(npm).as_deref(),
                Some(hashi),
                "npm2hashicorp({npm:?})"
            );
        }
    }

    // Ported: "hashicorp2npm(\"$version\") === $version && npm2hashicorp(\"$version\") === $version" — versioning/hashicorp/convertor.spec.ts line 32
    #[test]
    fn hashicorp2npm_and_npm2hashicorp_identity_matches_renovate_hashicorp_convertor_spec() {
        let versions = [
            "1.0.0-0",
            "1.0.0-1",
            "1.0.0-1.1",
            "1.0.0-10.21.32",
            "1.0.0-1.alpha.2",
            "1.0.0-alpha.beta",
            "1.0.0-alpha.beta.1",
            "1.0.0-alpha0.valid",
            "1.0.0-alpha.0valid",
            "1.0.0-alpha1test",
            "1.0.0-a.b",
            "1.0.0-a-b",
            "1.0.0-a1.-1-0-.09-9-",
            "1.0.0-a--.b",
        ];
        for v in versions {
            assert_eq!(hashicorp2npm(v).as_deref(), Some(v), "hashicorp2npm({v:?})");
            assert_eq!(npm2hashicorp(v).as_deref(), Some(v), "npm2hashicorp({v:?})");
        }
    }

    // Ported: "hashicorp2npm(\"$hashicorp\") === $npm" — versioning/hashicorp/convertor.spec.ts line 57
    #[test]
    fn hashicorp2npm_nonreflective_matches_renovate_hashicorp_convertor_spec() {
        let cases: &[(&str, &str)] = &[
            ("~> 4", ">=4"),
            ("~> v4", ">=4"),
            (">= v4.0", ">=4.0"),
            (">=4.0", ">=4.0"),
            ("<=4.0", "<=4.0"),
            ("= 4.0", "4.0"),
            ("> 4.0,< 5.0", ">4.0 <5.0"),
        ];
        for &(hashi, npm) in cases {
            assert_eq!(
                hashicorp2npm(hashi).as_deref(),
                Some(npm),
                "hashicorp2npm({hashi:?})"
            );
        }
    }

    // Ported: "npm2hashicorp(\"$npm\") === $hashicorp" — versioning/hashicorp/convertor.spec.ts line 71
    #[test]
    fn npm2hashicorp_nonreflective_matches_renovate_hashicorp_convertor_spec() {
        let cases: &[(&str, &str)] = &[
            ("^4", "~> 4.0"),
            ("^4.0.0", "~> 4.0"),
            ("^4.1.0", "~> 4.1"),
            ("^4.1.1", "~> 4.1"),
            ("~4", "~> 4.0"),
            ("~4.0", "~> 4.0.0"),
            ("~4.1", "~> 4.1.0"),
            ("v4.1.0", "4.1.0"),
        ];
        for &(npm, hashi) in cases {
            assert_eq!(
                npm2hashicorp(npm).as_deref(),
                Some(hashi),
                "npm2hashicorp({npm:?})"
            );
        }
    }

    // Ported: "hashicorp2npm doesnt support !=" — versioning/hashicorp/convertor.spec.ts line 85
    #[test]
    fn hashicorp2npm_doesnt_support_neq_matches_renovate_hashicorp_convertor_spec() {
        assert_eq!(hashicorp2npm("!= 4"), None);
    }

    // Ported: "hashicorp2npm throws on invalid" — versioning/hashicorp/convertor.spec.ts line 89
    #[test]
    fn hashicorp2npm_throws_on_invalid_matches_renovate_hashicorp_convertor_spec() {
        assert_eq!(hashicorp2npm("^4"), None);
    }

    // Ported: "npm2hashicorp throws on unsupported" — versioning/hashicorp/convertor.spec.ts line 93
    #[test]
    fn npm2hashicorp_throws_on_unsupported_matches_renovate_hashicorp_convertor_spec() {
        assert_eq!(npm2hashicorp("4.x.x"), None);
    }
}
