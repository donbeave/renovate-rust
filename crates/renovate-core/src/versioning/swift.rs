//! Swift Package Manager versioning.
//!
//! Ports `lib/modules/versioning/swift/range.ts` and `index.ts`.
//!
//! Version comparison uses semver. Ranges use Swift Package Manager syntax:
//! - `from: "X.Y.Z"` → `>=X.Y.Z <(major+1).0.0`
//! - `"X.Y.Z"...` → `>=X.Y.Z`
//! - `"X.Y.Z"..."A.B.C"` → `>=X.Y.Z <=A.B.C`
//! - `"X.Y.Z"..<"A.B.C"` → `>=X.Y.Z <A.B.C`
//! - `..."A.B.C"` → `<=A.B.C`
//! - `..<"A.B.C"` → `<A.B.C`

use semver::{Op, Version, VersionReq};

// ── Range conversion ──────────────────────────────────────────────────────────

fn strip_v(s: &str) -> &str {
    s.strip_prefix('v').unwrap_or(s)
}

/// Convert a Swift Package Manager range expression to a semver range string
/// parseable by the Rust `semver` crate.
pub fn to_semver_range(range: &str) -> Option<String> {
    let t = range.trim();

    // from: "X.Y.Z"
    if let Some(rest) = t.strip_prefix("from").map(|r| r.trim()) {
        if let Some(rest) = rest.strip_prefix(':').map(|r| r.trim())
            && rest.starts_with('"')
            && rest.ends_with('"')
        {
            let ver_str = &rest[1..rest.len() - 1];
            let v = Version::parse(ver_str).ok()?;
            let next_major = v.major + 1;
            return Some(format!(">={ver_str}, <{next_major}.0.0"));
        }
        return None;
    }

    // "X.Y.Z"... or "X.Y.Z"..<"A.B.C" or "X.Y.Z"..."A.B.C"
    if let Some(after_quote) = t.strip_prefix('"') {
        // binary range: "X"..op"Y" or "X"...
        // find end of first quoted version
        let end = after_quote.find('"')? + 1;
        let left_ver = &t[1..end];
        let rest = t[end + 1..].trim();

        if rest.is_empty() || rest == "..." {
            // "X.Y.Z"...
            Version::parse(left_ver).ok()?;
            return Some(format!(">={left_ver}"));
        }

        if let Some(right) = rest.strip_prefix("...").map(|r| r.trim()) {
            // "X.Y.Z"..."A.B.C"
            if right.starts_with('"') && right.ends_with('"') {
                let right_ver = &right[1..right.len() - 1];
                Version::parse(left_ver).ok()?;
                Version::parse(right_ver).ok()?;
                return Some(format!(">={left_ver}, <={right_ver}"));
            }
        }

        if let Some(right) = rest.strip_prefix("..<").map(|r| r.trim()) {
            // "X.Y.Z"..<"A.B.C"
            if right.starts_with('"') && right.ends_with('"') {
                let right_ver = &right[1..right.len() - 1];
                Version::parse(left_ver).ok()?;
                Version::parse(right_ver).ok()?;
                return Some(format!(">={left_ver}, <{right_ver}"));
            }
        }

        return None;
    }

    // ..."X.Y.Z" or ..<"X.Y.Z"
    if let Some(rest) = t.strip_prefix("...").map(|r| r.trim())
        && rest.starts_with('"')
        && rest.ends_with('"')
    {
        let ver_str = &rest[1..rest.len() - 1];
        Version::parse(ver_str).ok()?;
        return Some(format!("<={ver_str}"));
    }

    if let Some(rest) = t.strip_prefix("..<").map(|r| r.trim())
        && rest.starts_with('"')
        && rest.ends_with('"')
    {
        let ver_str = &rest[1..rest.len() - 1];
        Version::parse(ver_str).ok()?;
        return Some(format!("<{ver_str}"));
    }

    None
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Returns `true` when `input` is a valid semver version or a valid Swift range.
pub fn is_valid(input: &str) -> bool {
    Version::parse(strip_v(input.trim())).is_ok() || to_semver_range(input).is_some()
}

/// Returns `true` only for plain semver versions (not ranges).
pub fn is_version(input: &str) -> bool {
    Version::parse(strip_v(input.trim())).is_ok()
}

pub fn is_stable(version: &str) -> bool {
    Version::parse(strip_v(version.trim())).is_ok_and(|v| v.pre.is_empty())
}

pub fn get_major(version: &str) -> Option<i64> {
    Version::parse(strip_v(version))
        .ok()
        .map(|v| v.major as i64)
}

pub fn get_minor(version: &str) -> Option<i64> {
    Version::parse(strip_v(version))
        .ok()
        .map(|v| v.minor as i64)
}

pub fn get_patch(version: &str) -> Option<i64> {
    Version::parse(strip_v(version))
        .ok()
        .map(|v| v.patch as i64)
}

pub fn equals(a: &str, b: &str) -> bool {
    match (Version::parse(strip_v(a)), Version::parse(strip_v(b))) {
        (Ok(va), Ok(vb)) => va == vb,
        _ => false,
    }
}

pub fn is_greater_than(a: &str, b: &str) -> bool {
    match (Version::parse(strip_v(a)), Version::parse(strip_v(b))) {
        (Ok(va), Ok(vb)) => va > vb,
        _ => false,
    }
}

pub fn sort_versions(a: &str, b: &str) -> i32 {
    match (Version::parse(strip_v(a)), Version::parse(strip_v(b))) {
        (Ok(va), Ok(vb)) => va.cmp(&vb) as i32,
        _ => 0,
    }
}

pub fn matches_range(version: &str, range: &str) -> bool {
    if range.is_empty() {
        return false;
    }
    let clean_v = strip_v(version.trim());
    // Plain version equality.
    if is_version(range) {
        return equals(clean_v, strip_v(range.trim()));
    }
    let Some(semver_range) = to_semver_range(range) else {
        return false;
    };
    let Ok(v) = Version::parse(clean_v) else {
        return false;
    };
    let Ok(req) = VersionReq::parse(&semver_range) else {
        return false;
    };
    req.matches(&v)
}

pub fn is_less_than_range(version: &str, range: &str) -> bool {
    let Some(semver_range) = to_semver_range(range) else {
        return false;
    };
    let clean_v = strip_v(version.trim());
    let Ok(v) = Version::parse(clean_v) else {
        return false;
    };
    let Ok(req) = VersionReq::parse(&semver_range) else {
        return false;
    };
    if req.matches(&v) {
        return false;
    }
    // Version doesn't satisfy. Check if any lower bound (>= or >) in the range > v.
    for comp in &req.comparators {
        if matches!(comp.op, Op::GreaterEq | Op::Greater) {
            let lower = Version::new(comp.major, comp.minor.unwrap_or(0), comp.patch.unwrap_or(0));
            if v < lower {
                return true;
            }
        }
    }
    false
}

pub fn get_satisfying_version(versions: &[&str], range: &str) -> Option<String> {
    if range.is_empty() {
        return None;
    }
    let semver_range = to_semver_range(range)?;
    let req = VersionReq::parse(&semver_range).ok()?;
    versions
        .iter()
        .filter_map(|&v| {
            let clean = strip_v(v.trim());
            Version::parse(clean)
                .ok()
                .filter(|pv| req.matches(pv))
                .map(|pv| (pv, clean.to_owned()))
        })
        .max_by(|(a, _), (b, _)| a.cmp(b))
        .map(|(_, s)| s)
}

pub fn min_satisfying_version(versions: &[&str], range: &str) -> Option<String> {
    if range.is_empty() {
        return None;
    }
    let semver_range = to_semver_range(range)?;
    let req = VersionReq::parse(&semver_range).ok()?;
    versions
        .iter()
        .filter_map(|&v| {
            let clean = strip_v(v.trim());
            Version::parse(clean)
                .ok()
                .filter(|pv| req.matches(pv))
                .map(|pv| (pv, clean.to_owned()))
        })
        .min_by(|(a, _), (b, _)| a.cmp(b))
        .map(|(_, s)| s)
}

pub fn get_new_value(current_value: &str, new_version: &str) -> String {
    // Strip leading v from new_version.
    let clean_new = new_version
        .strip_prefix('v')
        .filter(|rest| rest.chars().next().is_some_and(|c| c.is_ascii_digit()))
        .unwrap_or(new_version);

    // from: "X.Y.Z" pattern
    if current_value.trim().starts_with("from") {
        // Replace the first quoted version.
        return replace_first_quoted(current_value, clean_new);
    }

    // "X.Y.Z"... pattern
    if let Some(end) = current_value[1..].find('"') {
        let left_end = end + 1;
        if current_value.starts_with('"') {
            let left_ver = &current_value[1..left_end];
            let rest = &current_value[left_end + 1..];
            if rest.trim().is_empty()
                || rest.trim().starts_with("...") && !rest.trim().contains('"')
            {
                // "X.Y.Z"...  — replace left ver
                let new_cv = current_value.replacen(left_ver, clean_new, 1);
                return new_cv;
            }
            // binary range: replace LAST quoted version
            if let (Some(_op), Some(_right)) = (
                rest.find("...").or_else(|| rest.find("..<")),
                rest.rfind('"'),
            ) {
                let last_quote_end = rest.rfind('"').unwrap();
                let last_quote_start = rest[..last_quote_end].rfind('"').unwrap_or(0) + 1;
                let last_ver = &rest[last_quote_start..last_quote_end];
                let new_cv = current_value.replacen(last_ver, clean_new, 1);
                return new_cv;
            }
        }
    }

    // to_range: ..."X" or ..<"X" — replace the quoted version
    if current_value.trim().starts_with("...") || current_value.trim().starts_with("..<") {
        return replace_first_quoted(current_value, clean_new);
    }

    clean_new.to_owned()
}

fn replace_first_quoted(s: &str, replacement: &str) -> String {
    let start = s.find('"').map(|i| i + 1);
    let end = start.and_then(|i| s[i..].find('"').map(|j| i + j));
    match (start, end) {
        (Some(s_idx), Some(e_idx)) => {
            format!("{}\"{}\"{}", &s[..s_idx - 1], replacement, &s[e_idx + 1..])
        }
        _ => replacement.to_owned(),
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: 'isVersion("$version") === $expected' — versioning/swift/index.spec.ts line 14
    #[test]
    fn swift_is_version() {
        assert!(!is_version(r#"from: "1.2.3""#));
        assert!(is_version("1.2.3"));
        assert!(is_version("v1.2.3"));
        assert!(!is_version("a"));
    }

    // Ported: 'isValid("$version") === $expected' — versioning/swift/index.spec.ts line 24
    #[test]
    fn swift_is_valid() {
        let valid = [
            r#"from: "1.2.3""#,
            r#"from : "1.2.3""#,
            r#"from:"1.2.3""#,
            r#" from:"1.2.3" "#,
            r#" from : "1.2.3" "#,
            r#""1.2.3"..."1.2.4""#,
            r#" "1.2.3" ... "1.2.4" "#,
            r#""1.2.3"..."#,
            r#" "1.2.3" ... "#,
            r#"..."1.2.4""#,
            r#" ... "1.2.4" "#,
            r#""1.2.3"..<"1.2.4""#,
            r#" "1.2.3" ..< "1.2.4" "#,
            r#"..<"1.2.4""#,
            r#" ..< "1.2.4" "#,
            "1.2.3",
            "v1.2.3",
            "1.2.3-foo",
            r#"from: "1.2.3""#,
        ];
        let invalid = [
            r#"from : "1.2.3.4.5""#,
            r#""1.2.3.4.5"..."#,
            r#""1.2.3.4.5"..<"1.2.4""#,
            "17.04.0",
            "1.2.3foo",
            "~1.2.3",
            "^1.2.3",
        ];
        for s in &valid {
            assert!(is_valid(s), "isValid({s:?}) should be true");
        }
        for s in &invalid {
            assert!(!is_valid(s), "isValid({s:?}) should be false");
        }
    }

    // Ported: 'minSatisfyingVersion($versions, "$range") === "$expected"' — versioning/swift/index.spec.ts line 62
    #[test]
    fn swift_min_satisfying_version() {
        assert_eq!(
            min_satisfying_version(&["1.2.3", "1.2.4", "1.2.5"], r#"..<"1.2.4""#),
            Some("1.2.3".to_owned())
        );
        assert_eq!(
            min_satisfying_version(&["v1.2.3", "v1.2.4", "v1.2.5"], r#"..<"1.2.4""#),
            Some("1.2.3".to_owned())
        );
        assert_eq!(
            min_satisfying_version(&["v1.2.3", "v1.2.4", "v1.2.5"], ""),
            None
        );
    }

    // Ported: 'getSatisfyingVersion($versions, "$range") === "$expected"' — versioning/swift/index.spec.ts line 74
    #[test]
    fn swift_get_satisfying_version() {
        assert_eq!(
            get_satisfying_version(&["1.2.3", "1.2.4", "1.2.5"], r#"..<"1.2.4""#),
            Some("1.2.3".to_owned())
        );
        assert_eq!(
            get_satisfying_version(&["v1.2.3", "v1.2.4", "v1.2.5"], r#"..<"1.2.4""#),
            Some("1.2.3".to_owned())
        );
        assert_eq!(
            get_satisfying_version(&["1.2.3", "1.2.4", "1.2.5"], r#"..."1.2.4""#),
            Some("1.2.4".to_owned())
        );
        assert_eq!(
            get_satisfying_version(&["1.2.3", "1.2.4", "1.2.5"], ""),
            None
        );
    }

    // Ported: 'isLessThanRange("$version", "$range") === "$expected"' — versioning/swift/index.spec.ts line 87
    #[test]
    fn swift_is_less_than_range() {
        assert!(!is_less_than_range("1.2.3", r#"..."1.2.4""#));
        assert!(!is_less_than_range("v1.2.3", r#"..."1.2.4""#));
        assert!(is_less_than_range("1.2.3", r#""1.2.4"..."#));
        assert!(is_less_than_range("v1.2.3", r#""1.2.4"..."#));
        assert!(!is_less_than_range("v1.2.3", ""));
    }

    // Ported: 'matches("$version", "$range") === "$expected"' — versioning/swift/index.spec.ts line 101
    #[test]
    fn swift_matches() {
        assert!(matches_range("1.2.3", "1.2.3"));
        assert!(matches_range("v1.2.3", "1.2.3"));
        assert!(matches_range("1.2.4", r#"..."1.2.4""#));
        assert!(matches_range("v1.2.4", r#"..."1.2.4""#));
        assert!(!matches_range("1.2.4", r#"..."1.2.3""#));
        assert!(!matches_range("v1.2.4", r#"..."1.2.3""#));
        assert!(!matches_range("v1.2.4", ""));
    }

    #[test]
    fn swift_is_stable() {
        assert!(is_stable("1.2.3"));
        assert!(is_stable("v1.2.3"));
        assert!(!is_stable("1.2.3-alpha"));
        assert!(!is_stable("1.2.3-beta.1"));
    }

    #[test]
    fn swift_get_major_minor_patch() {
        assert_eq!(get_major("1.2.3"), Some(1));
        assert_eq!(get_minor("1.2.3"), Some(2));
        assert_eq!(get_patch("1.2.3"), Some(3));
        assert_eq!(get_major("v2.0.0"), Some(2));
        assert_eq!(get_minor("v2.0.0"), Some(0));
        assert_eq!(get_patch("v2.0.0"), Some(0));
    }

    #[test]
    fn swift_equals() {
        assert!(equals("1.2.3", "1.2.3"));
        assert!(equals("1.2.3", "v1.2.3"));
        assert!(!equals("1.2.3", "1.2.4"));
    }

    #[test]
    fn swift_is_greater_than() {
        assert!(is_greater_than("1.2.4", "1.2.3"));
        assert!(!is_greater_than("1.2.3", "1.2.4"));
        assert!(!is_greater_than("1.2.3", "1.2.3"));
    }

    #[test]
    fn swift_sort_versions() {
        assert_eq!(sort_versions("1.2.3", "1.2.4"), -1);
        assert_eq!(sort_versions("1.2.4", "1.2.3"), 1);
        assert_eq!(sort_versions("1.2.3", "1.2.3"), 0);
    }

    #[test]
    fn swift_to_semver_range() {
        assert_eq!(
            to_semver_range(r#"from: "1.2.3""#),
            Some(">=1.2.3, <2.0.0".to_owned())
        );
        assert_eq!(to_semver_range(r#""1.2.3"..."#), Some(">=1.2.3".to_owned()));
        assert_eq!(
            to_semver_range(r#""1.2.3"..."1.2.4""#),
            Some(">=1.2.3, <=1.2.4".to_owned())
        );
        assert_eq!(
            to_semver_range(r#""1.2.3"..<"1.2.4""#),
            Some(">=1.2.3, <1.2.4".to_owned())
        );
        assert_eq!(to_semver_range(r#"..."1.2.4""#), Some("<=1.2.4".to_owned()));
        assert_eq!(to_semver_range(r#"..<"1.2.4""#), Some("<1.2.4".to_owned()));
        assert_eq!(to_semver_range("invalid"), None);
    }

    // Ported: 'getNewValue("$currentValue", ...) === "$expected"' — versioning/swift/index.spec.ts line 117
    #[test]
    fn swift_get_new_value() {
        assert_eq!(get_new_value("1.2.3", "1.2.4"), "1.2.4");
        assert_eq!(get_new_value("1.2.3", "v1.2.4"), "1.2.4");
        assert_eq!(
            get_new_value(r#"from: "1.2.3""#, "1.2.4"),
            r#"from: "1.2.4""#
        );
        assert_eq!(
            get_new_value(r#"from: "1.2.3""#, "v1.2.4"),
            r#"from: "1.2.4""#
        );
        assert_eq!(
            get_new_value(r#"from: "1.2.2""#, "1.2.4"),
            r#"from: "1.2.4""#
        );
        assert_eq!(
            get_new_value(r#"from: "1.2.2""#, "v1.2.4"),
            r#"from: "1.2.4""#
        );
        assert_eq!(get_new_value(r#""1.2.3"..."#, "1.2.4"), r#""1.2.4"..."#);
        assert_eq!(get_new_value(r#""1.2.3"..."#, "v1.2.4"), r#""1.2.4"..."#);
        assert_eq!(
            get_new_value(r#""1.2.3"..."1.2.4""#, "1.2.5"),
            r#""1.2.3"..."1.2.5""#
        );
        assert_eq!(
            get_new_value(r#""1.2.3"..."1.2.4""#, "v1.2.5"),
            r#""1.2.3"..."1.2.5""#
        );
        assert_eq!(
            get_new_value(r#""1.2.3"..<"1.2.4""#, "1.2.5"),
            r#""1.2.3"..<"1.2.5""#
        );
        assert_eq!(
            get_new_value(r#""1.2.3"..<"1.2.4""#, "v1.2.5"),
            r#""1.2.3"..<"1.2.5""#
        );
        assert_eq!(get_new_value(r#"..."1.2.4""#, "1.2.5"), r#"..."1.2.5""#);
        assert_eq!(get_new_value(r#"..."1.2.4""#, "v1.2.5"), r#"..."1.2.5""#);
        assert_eq!(get_new_value(r#"..<"1.2.4""#, "1.2.5"), r#"..<"1.2.5""#);
        assert_eq!(get_new_value(r#"..<"1.2.4""#, "v1.2.5"), r#"..<"1.2.5""#);
    }
}
