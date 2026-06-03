//! @parity lib/modules/versioning/semver-partial/index.ts full

use semver::Version;

fn massage_value(input: &str) -> &str {
    let s = input.trim();
    if s.starts_with('v') || s.starts_with('V') {
        &s[1..]
    } else {
        s
    }
}

fn parse_version(input: &str) -> Option<Version> {
    Version::parse(massage_value(input)).ok()
}

struct Range {
    major: u64,
    minor: Option<u64>,
}

fn parse_range(input: &str) -> Option<Range> {
    let stripped = massage_value(input);
    // coerce: try to extract a semver from the string
    let coerced = coerce_semver(stripped)?;
    let re_major_only = regex::Regex::new(r"^\d+$").unwrap();
    if re_major_only.is_match(stripped) {
        Some(Range {
            major: coerced.major,
            minor: None,
        })
    } else {
        Some(Range {
            major: coerced.major,
            minor: Some(coerced.minor),
        })
    }
}

fn coerce_semver(input: &str) -> Option<Version> {
    // Try exact parse first
    if let Ok(v) = Version::parse(input) {
        return Some(v);
    }
    // Try X.Y → X.Y.0
    let re_xy = regex::Regex::new(r"^(\d+)\.(\d+)$").unwrap();
    if let Some(caps) = re_xy.captures(input) {
        let major: u64 = caps[1].parse().ok()?;
        let minor: u64 = caps[2].parse().ok()?;
        return Some(Version::new(major, minor, 0));
    }
    // Try X → X.0.0
    let re_x = regex::Regex::new(r"^(\d+)$").unwrap();
    if let Some(caps) = re_x.captures(input) {
        let major: u64 = caps[1].parse().ok()?;
        return Some(Version::new(major, 0, 0));
    }
    None
}

fn is_latest(input: &str) -> bool {
    input == "~latest"
}

pub fn is_valid(input: &str) -> bool {
    is_latest(input) || parse_version(input).is_some() || parse_range(input).is_some()
}

pub fn is_version(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    parse_version(input).is_some()
}

pub fn is_stable(version: &str) -> bool {
    match parse_version(version) {
        Some(v) => v.pre.is_empty(),
        None => false,
    }
}

pub fn is_single_version(input: &str) -> bool {
    is_version(input)
}

pub fn get_major(version: &str) -> Option<u64> {
    parse_version(version).map(|v| v.major)
}

pub fn get_minor(version: &str) -> Option<u64> {
    parse_version(version).map(|v| v.minor)
}

pub fn get_patch(version: &str) -> Option<u64> {
    parse_version(version).map(|v| v.patch)
}

pub fn sort_versions(a: &str, b: &str) -> std::cmp::Ordering {
    match (parse_version(a), parse_version(b)) {
        (Some(av), Some(bv)) => av.cmp(&bv),
        _ => std::cmp::Ordering::Equal,
    }
}

pub fn equals(x: &str, y: &str) -> bool {
    match (parse_version(x), parse_version(y)) {
        (Some(a), Some(b)) => a == b,
        _ => false,
    }
}

pub fn is_greater_than(x: &str, y: &str) -> bool {
    match (parse_version(x), parse_version(y)) {
        (Some(a), Some(b)) => a > b,
        _ => false,
    }
}

pub fn matches(version: &str, range: &str) -> bool {
    let Some(v) = parse_version(version) else {
        return false;
    };
    if is_latest(range) {
        return true;
    }
    if let Some(rv) = parse_version(range) {
        return v == rv;
    }
    let Some(r) = parse_range(range) else {
        return false;
    };
    // prerelease versions don't match partial ranges
    if !v.pre.is_empty() {
        return false;
    }
    if v.major != r.major {
        return false;
    }
    match r.minor {
        None => true,
        Some(m) => v.minor == m,
    }
}

pub fn get_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    let mut matching: Vec<&str> = versions
        .iter()
        .copied()
        .filter(|v| is_version(v) && matches(v, range))
        .collect();
    matching.sort_by(|a, b| sort_versions(b, a));
    matching.into_iter().next()
}

pub fn min_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    let mut matching: Vec<&str> = versions
        .iter()
        .copied()
        .filter(|v| is_version(v) && matches(v, range))
        .collect();
    matching.sort_by(|a, b| sort_versions(a, b));
    matching.into_iter().next()
}

pub fn is_less_than_range(version: &str, range: &str) -> bool {
    let Some(v) = parse_version(version) else {
        return false;
    };
    let Some(r) = parse_range(range) else {
        return false;
    };
    if v.major != r.major {
        return v.major < r.major;
    }
    match r.minor {
        None => false,
        Some(m) => v.minor < m,
    }
}

pub fn is_compatible(version: &str) -> bool {
    is_valid(version)
}

pub fn is_breaking(version: &str, current: &str) -> bool {
    let Some(v) = parse_version(version) else {
        return false;
    };
    let Some(c) = parse_version(current) else {
        return false;
    };
    if c.major == 0 {
        return v.major > 0 || v.minor > c.minor;
    }
    v.major > c.major
}

#[derive(Debug)]
pub struct NewValueParams {
    pub current_value: String,
    pub range_strategy: String,
    pub new_version: String,
}

pub fn get_new_value(params: &NewValueParams) -> Option<String> {
    let current_value = &params.current_value;
    let new_version = &params.new_version;

    if params.range_strategy == "pin" {
        return Some(new_version.clone());
    }
    if is_latest(current_value) {
        return Some(current_value.clone());
    }
    let Some(range) = parse_range(current_value) else {
        return Some(new_version.clone());
    };
    let Some(new_parsed) = parse_version(new_version) else {
        return Some(new_version.clone());
    };
    // if currentValue is a full version, return newVersion directly
    if parse_version(current_value).is_some() {
        return Some(new_version.clone());
    }
    // extract prefix (e.g. "v" from "v1")
    let massaged = massage_value(current_value);
    let prefix = &current_value[..current_value.len() - massaged.len()];

    match range.minor {
        None => Some(format!("{}{}", prefix, new_parsed.major)),
        Some(_) => Some(format!(
            "{}{}.{}",
            prefix, new_parsed.major, new_parsed.minor
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isValid("$version") === $expected" — lib/modules/versioning/semver-partial/index.spec.ts line 5
    #[test]
    fn is_valid_table() {
        let cases = [
            ("1", true),
            ("1.2", true),
            ("1.2.3", true),
            ("~latest", true),
            ("1.2.3-alpha", true),
            ("v1", true),
            ("v1.2", true),
            ("v1.2.3", true),
            ("v1.2.3-alpha", true),
            ("invalid", false),
            ("", false),
        ];
        for (version, expected) in cases {
            assert_eq!(
                is_valid(version),
                expected,
                "is_valid({version:?}) should be {expected}"
            );
        }
    }

    // Ported: "isVersion("$version") === $expected" — lib/modules/versioning/semver-partial/index.spec.ts line 24
    #[test]
    fn is_version_table() {
        let cases = [
            ("1", false),
            ("1.2", false),
            ("1.2.3", true),
            ("~latest", false),
            ("1.2.3-alpha", true),
            ("1.2.3-rc.1", true),
            ("v1", false),
            ("v1.2", false),
            ("v1.2.3", true),
            ("v1.2.3-alpha", true),
            ("v1.2.3-rc.1", true),
            ("invalid", false),
            ("", false),
            ("#1.0.0", false),
            ("x1.0.0", false),
        ];
        for (version, expected) in cases {
            assert_eq!(
                is_version(version),
                expected,
                "is_version({version:?}) should be {expected}"
            );
        }
    }

    // Ported: "isStable("$version") === $expected" — lib/modules/versioning/semver-partial/index.spec.ts line 47
    #[test]
    fn is_stable_table() {
        let cases = [
            ("1.0.0-alpha", false),
            ("1.0.0-beta", false),
            ("1.0.0-rc", false),
            ("1.0.0-pre", false),
            ("1.0.0-dev", false),
            ("1.0.0-snapshot", false),
            ("1.0.0-unstable", false),
            ("1.0.0-Alpha", false),
            ("1.0.0-1", false),
            ("1.0.0-build.1", false),
            ("1.0.0", true),
            ("v1.0.0", true),
            ("v1.0.0-alpha", false),
            ("1", false),
            ("not-a-version", false),
        ];
        for (version, expected) in cases {
            assert_eq!(
                is_stable(version),
                expected,
                "is_stable({version:?}) should be {expected}"
            );
        }
    }

    // Ported: "isSingleVersion("$version") === $expected" — lib/modules/versioning/semver-partial/index.spec.ts line 70
    #[test]
    fn is_single_version_table() {
        let cases = [
            ("1", false),
            ("1.2", false),
            ("1.2.3", true),
            ("~latest", false),
            ("1.2.3-alpha", true),
            ("v1", false),
            ("v1.2", false),
            ("v1.2.3", true),
            ("v1.2.3-alpha", true),
        ];
        for (version, expected) in cases {
            assert_eq!(
                is_single_version(version),
                expected,
                "is_single_version({version:?}) should be {expected}"
            );
        }
    }

    // Ported: "matches("$version", "$range") === $expected" — lib/modules/versioning/semver-partial/index.spec.ts line 87
    #[test]
    fn matches_table() {
        let cases = [
            ("1.1.0", "1.0", false),
            ("1.0.0", "1", true),
            ("1.2.0", "1", true),
            ("1.2.3", "1", true),
            ("2.0.0", "1", false),
            ("1.1.0", "1.1", true),
            ("1.1.5", "1.1", true),
            ("1.2.0", "1.1", false),
            ("1.0.0", "1.1", false),
            ("1.2.3", "1.2", true),
            ("1.2.0", "1.2", true),
            ("1.3.0", "1.2", false),
            ("1.0.0", "~latest", true),
            ("2.1.0", "~latest", true),
            ("1.0.0-rc", "1", false),
            ("1.0.0-rc", "1.0", false),
            ("invalid", "1", false),
            ("~latest", "1", false),
            ("1", "1", false),
            ("1.2", "1.2", false),
            ("1.2.3", "1.2.3", true),
            ("1.2.4", "1.2.3", false),
            ("not-semver-ver", "1", false),
            ("1.0.0-alpha", "1", false),
            ("1.0.0-beta", "1.0", false),
            ("v1.0.0", "v1", true),
            ("v1.2.0", "v1", true),
            ("v1.1.0", "v1.1", true),
            ("v1.2.3", "v1.2.3", true),
            ("v2.0.0", "v1", false),
            ("v1.0.0", "1", true),
            ("1.0.0", "v1", true),
            ("v1.1.5", "1.1", true),
            ("1.1.5", "v1.1", true),
            ("v2.1.0", "2", true),
            ("2.1.0", "v2", true),
            ("v1.2.0", "1.1", false),
            ("1.2.0", "v1.1", false),
            ("v1.0.0-rc", "v1", false),
            ("1.0.0-rc", "v1", false),
            ("v1.0.0-rc", "1", false),
            ("v1.2.4", "1.2.3", false),
            ("1.2.4", "v1.2.3", false),
            ("v1.2.3", "1.2.3", true),
            ("1.2.3", "v1.2.3", true),
        ];
        for (version, range, expected) in cases {
            assert_eq!(
                matches(version, range),
                expected,
                "matches({version:?}, {range:?}) should be {expected}"
            );
        }
    }

    // Ported: "should handle invalid range that is not ~latest or valid version" — lib/modules/versioning/semver-partial/index.spec.ts line 141
    #[test]
    fn matches_completely_invalid_range() {
        assert!(!matches("1.0.0", "completely-invalid-range"));
    }

    // Ported: "getSatisfyingVersion($versions, "$range") === $expected" — lib/modules/versioning/semver-partial/index.spec.ts line 149
    #[test]
    fn get_satisfying_version_table() {
        let cases: Vec<(Vec<&str>, &str, Option<&str>)> = vec![
            (
                vec!["1.0.0", "1.1.0", "1.1.1", "1.2.0", "2.0.0", "2.0.1"],
                "1",
                Some("1.2.0"),
            ),
            (
                vec!["1.0.0", "1.1.0", "1.1.1", "1.2.0", "2.0.0", "2.0.1"],
                "1.1",
                Some("1.1.1"),
            ),
            (
                vec!["1.0.0", "1.1.0", "1.1.1", "1.2.0", "2.0.0", "2.0.1"],
                "2",
                Some("2.0.1"),
            ),
            (
                vec!["1.0.0", "1.1.0", "1.1.1", "1.2.0", "2.0.0", "2.1.0"],
                "~latest",
                Some("2.1.0"),
            ),
            (
                vec!["1.0.0", "1.1.0", "1.2.0", "2.0.0", "2.0.1", "2.1.0-rc"],
                "2",
                Some("2.0.1"),
            ),
            (vec!["1.0.0", "1.0.1-rc", "1.1.0"], "1.0", Some("1.0.0")),
            (vec!["0.5.0", "1.0.0", "2.0.0"], "3", None),
            (vec!["invalid-version", "1.0.0"], "1", Some("1.0.0")),
            (vec!["1.0", "1.1", "1.2"], "1", None),
            (
                vec!["0.9.0-alpha", "0.9.0-beta", "0.9.0"],
                "~latest",
                Some("0.9.0"),
            ),
            (vec!["some-text", "another-text"], "1", None),
            (vec!["not-valid", "also-bad"], "1", None),
            (
                vec!["1.0.0", "1.0.1-alpha", "1.0.2", "1.1.0-beta", "1.1.1"],
                "1",
                Some("1.1.1"),
            ),
            (
                vec!["1.0.0", "1.0.1-alpha", "1.0.2", "1.1.0-beta", "1.1.1"],
                "1.0",
                Some("1.0.2"),
            ),
            (
                vec!["v1.0.0", "v1.1.0", "v1.1.1", "v1.2.0", "v2.0.0"],
                "v1",
                Some("v1.2.0"),
            ),
            (vec!["v1.0.0", "v1.1.0", "v1.1.1"], "v1.1", Some("v1.1.1")),
            (vec!["v1.0.0", "v2.0.0", "v3.0.0"], "v2", Some("v2.0.0")),
            (
                vec!["1.0.0", "v1.1.0", "1.1.1", "v1.2.0", "2.0.0"],
                "1",
                Some("v1.2.0"),
            ),
            (vec!["v1.0.0", "1.1.0", "v1.1.1"], "v1.1", Some("v1.1.1")),
            (vec!["1.0.0", "v1.1.0", "1.1.1"], "1", Some("1.1.1")),
            (vec!["v1.0.0", "1.1.0", "v1.1.1"], "1", Some("v1.1.1")),
            (vec!["1.0.0", "1.1.0", "1.2.0"], "v1", Some("1.2.0")),
            (vec!["v1.0.0", "v1.1.0", "1.2.0"], "1", Some("1.2.0")),
        ];
        for (versions, range, expected) in &cases {
            assert_eq!(
                get_satisfying_version(versions, range),
                *expected,
                "get_satisfying_version({versions:?}, {range:?}) should be {expected:?}"
            );
        }
    }

    // Ported: "minSatisfyingVersion($versions, "$range") === $expected" — lib/modules/versioning/semver-partial/index.spec.ts line 185
    #[test]
    fn min_satisfying_version_table() {
        let cases: Vec<(Vec<&str>, &str, Option<&str>)> = vec![
            (
                vec!["1.0.0", "1.1.0", "1.1.1", "1.2.0", "2.0.0"],
                "1",
                Some("1.0.0"),
            ),
            (
                vec!["1.0.0", "1.1.0", "1.1.1", "1.2.0", "2.0.0"],
                "1.1",
                Some("1.1.0"),
            ),
            (
                vec!["1.0.0", "1.1.0", "1.1.1", "1.2.0", "2.0.0", "2.0.1"],
                "2",
                Some("2.0.0"),
            ),
            (
                vec!["1.0.0", "1.1.0", "1.2.0", "2.0.0", "2.0.1", "2.1.0"],
                "~latest",
                Some("1.0.0"),
            ),
            (vec!["1.0.0", "1.0.1-rc", "1.1.0"], "1.0", Some("1.0.0")),
            (vec!["0.5.0", "1.0.0", "2.0.0"], "3", None),
            (vec!["v0.5.0", "v1.0.0", "v2.0.0"], "v3", None),
            (vec!["v1.0.0", "1.1.0", "v1.2.0"], "1", Some("v1.0.0")),
            (vec!["1.0.0", "v1.1.0", "1.2.0"], "v1", Some("1.0.0")),
            (
                vec!["v1.0.0", "v1.1.0", "1.2.0", "v2.0.0"],
                "v1",
                Some("v1.0.0"),
            ),
            (
                vec!["1.0.0", "1.1.0", "v1.2.0", "2.0.0"],
                "1",
                Some("1.0.0"),
            ),
        ];
        for (versions, range, expected) in &cases {
            assert_eq!(
                min_satisfying_version(versions, range),
                *expected,
                "min_satisfying_version({versions:?}, {range:?}) should be {expected:?}"
            );
        }
    }

    // Ported: "isLessThanRange("$version", "$range") === $expected" — lib/modules/versioning/semver-partial/index.spec.ts line 209
    #[test]
    fn is_less_than_range_table() {
        let cases = [
            ("0.9.0", "1", true),
            ("1.0.0", "1", false),
            ("1.5.0", "1", false),
            ("2.0.0", "1", false),
            ("1.0.0", "1.1", true),
            ("1.1.0", "1.1", false),
            ("1.2.0", "1.1", false),
            ("0.9.0", "~latest", false),
            ("1.0.0", "~latest", false),
            ("1.5.0", "1", false),
            ("invalid", "1", false),
            ("v0.9.0", "v1", true),
            ("v1.0.0", "v1", false),
            ("v1.0.0", "v1.1", true),
            ("0.9.0", "v1", true),
            ("v0.9.0", "1", true),
            ("1.0.0", "v1.1", true),
            ("v1.0.0", "1.1", true),
            ("v2.0.0", "1", false),
            ("2.0.0", "v1", false),
        ];
        for (version, range, expected) in cases {
            assert_eq!(
                is_less_than_range(version, range),
                expected,
                "is_less_than_range({version:?}, {range:?}) should be {expected}"
            );
        }
    }

    // Ported: "equals("$version", "$other") === $expected" — lib/modules/versioning/semver-partial/index.spec.ts line 240
    #[test]
    fn equals_table() {
        let cases = [
            ("1.0.0", "1.0.0", true),
            ("1.0.0", "1.0.1", false),
            ("invalid", "1.0.0", false),
            ("1.0.0", "invalid", false),
            ("invalid", "invalid", false),
            ("v1.0.0", "v1.0.0", true),
            ("v1.0.0", "v1.0.1", false),
            ("v1.0.0", "1.0.0", true),
            ("1.0.0", "v1.0.0", true),
            ("v1.0.0", "1.0.1", false),
            ("1.0.1", "v1.0.0", false),
        ];
        for (a, b, expected) in cases {
            assert_eq!(
                equals(a, b),
                expected,
                "equals({a:?}, {b:?}) should be {expected}"
            );
        }
    }

    // Ported: "getMajor("$version") === $expected" — lib/modules/versioning/semver-partial/index.spec.ts line 262
    #[test]
    fn get_major_table() {
        assert_eq!(get_major("1.0.0"), Some(1));
        assert_eq!(get_major("2.3.4"), Some(2));
        assert_eq!(get_major("v1.0.0"), Some(1));
        assert_eq!(get_major("v2.3.4"), Some(2));
        assert_eq!(get_major("invalid"), None);
    }

    // Ported: "getMinor("$version") === $expected" — lib/modules/versioning/semver-partial/index.spec.ts line 275
    #[test]
    fn get_minor_table() {
        assert_eq!(get_minor("1.0.0"), Some(0));
        assert_eq!(get_minor("2.3.4"), Some(3));
        assert_eq!(get_minor("v1.0.0"), Some(0));
        assert_eq!(get_minor("v2.3.4"), Some(3));
        assert_eq!(get_minor("invalid"), None);
    }

    // Ported: "getPatch("$version") === $expected" — lib/modules/versioning/semver-partial/index.spec.ts line 288
    #[test]
    fn get_patch_table() {
        assert_eq!(get_patch("1.0.0"), Some(0));
        assert_eq!(get_patch("2.3.4"), Some(4));
        assert_eq!(get_patch("v1.0.0"), Some(0));
        assert_eq!(get_patch("v2.3.4"), Some(4));
        assert_eq!(get_patch("invalid"), None);
    }

    // Ported: "isGreaterThan("$version", "$other") === $expected" — lib/modules/versioning/semver-partial/index.spec.ts line 301
    #[test]
    fn is_greater_than_table() {
        let cases = [
            ("1.0.1", "1.0.0", true),
            ("1.0.0", "1.0.1", false),
            ("2.0.0", "1.9.9", true),
            ("invalid", "1.0.0", false),
            ("1.0.0", "invalid", false),
            ("v1.0.1", "v1.0.0", true),
            ("v1.0.0", "v1.0.1", false),
            ("v2.0.0", "v1.9.9", true),
            ("v1.0.0", "1.0.1", false),
            ("1.0.1", "v1.0.0", true),
            ("v2.0.0", "1.0.0", true),
            ("2.0.0", "v1.0.0", true),
            ("v1.9.9", "1.9.8", true),
            ("1.9.9", "v1.9.8", true),
        ];
        for (a, b, expected) in cases {
            assert_eq!(
                is_greater_than(a, b),
                expected,
                "is_greater_than({a:?}, {b:?}) should be {expected}"
            );
        }
    }

    // Ported: "sortVersions("$a", "$b") === $expected" — lib/modules/versioning/semver-partial/index.spec.ts line 326
    #[test]
    fn sort_versions_table() {
        use std::cmp::Ordering;
        let cases = [
            ("1.0.0", "1.0.0", Ordering::Equal),
            ("1.0.0", "1.0.1", Ordering::Less),
            ("1.0.1", "1.0.0", Ordering::Greater),
            ("2.0.0", "1.9.9", Ordering::Greater),
            ("invalid", "1.0.0", Ordering::Equal),
            ("1.0.0", "invalid", Ordering::Equal),
            ("invalid", "invalid", Ordering::Equal),
            ("v1.0.0", "v1.0.0", Ordering::Equal),
            ("v1.0.0", "v1.0.1", Ordering::Less),
            ("v1.0.1", "v1.0.0", Ordering::Greater),
            ("v1.0.0", "1.0.0", Ordering::Equal),
            ("1.0.0", "v1.0.0", Ordering::Equal),
            ("v1.0.0", "1.0.1", Ordering::Less),
            ("1.0.1", "v1.0.0", Ordering::Greater),
        ];
        for (a, b, expected) in cases {
            assert_eq!(
                sort_versions(a, b),
                expected,
                "sort_versions({a:?}, {b:?}) should be {expected:?}"
            );
        }
    }

    // Ported: "isBreaking("$version", "$current") === $expected" — lib/modules/versioning/semver-partial/index.spec.ts line 348
    #[test]
    fn is_breaking_table() {
        let cases = [
            ("2.0.0", "1.0.0", true),
            ("1.1.0", "1.0.0", false),
            ("1.0.1", "1.0.0", false),
            ("0.2.0", "0.1.0", true),
            ("0.1.1", "0.1.0", false),
            ("1.0.0", "0.9.0", true),
            ("invalid", "1.0.0", false),
            ("1.0.0", "invalid", false),
            ("v2.0.0", "v1.0.0", true),
            ("v1.1.0", "v1.0.0", false),
            ("v0.2.0", "v0.1.0", true),
            ("v2.0.0", "1.0.0", true),
            ("2.0.0", "v1.0.0", true),
            ("1.1.0", "v1.0.0", false),
            ("v1.1.0", "1.0.0", false),
            ("v1.0.0", "0.9.0", true),
            ("1.0.0", "v0.9.0", true),
        ];
        for (version, current, expected) in cases {
            assert_eq!(
                is_breaking(version, current),
                expected,
                "is_breaking({version:?}, {current:?}) should be {expected}"
            );
        }
    }

    // Ported: "isCompatible("$version") === $expected" — lib/modules/versioning/semver-partial/index.spec.ts line 376
    #[test]
    fn is_compatible_table() {
        assert!(is_compatible("1.0.0"));
        assert!(is_compatible("1"));
        assert!(is_compatible("~latest"));
        assert!(is_compatible("v1.0.0"));
        assert!(is_compatible("v1"));
        assert!(!is_compatible("invalid"));
    }

    // Ported: "getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected"" — lib/modules/versioning/semver-partial/index.spec.ts line 390
    #[test]
    fn get_new_value_table() {
        let cases = [
            // pin strategy
            ("1", "pin", "1.0.0", "1.1.0", "1.1.0"),
            ("1.2", "pin", "1.2.0", "1.2.1", "1.2.1"),
            ("1.2.3", "pin", "1.2.3", "1.2.4", "1.2.4"),
            ("2", "pin", "2.0.0", "2.1.0", "2.1.0"),
            ("2.5", "pin", "2.5.0", "2.5.3", "2.5.3"),
            ("10", "pin", "10.0.0", "10.1.0", "10.1.0"),
            ("~latest", "pin", "1.0.0", "1.1.0", "1.1.0"),
            ("1.0.0", "pin", "1.0.0", "1.1.0", "1.1.0"),
            ("v1", "pin", "v1.0.0", "v1.1.0", "v1.1.0"),
            ("v1.2", "pin", "v1.2.0", "v1.2.1", "v1.2.1"),
            ("v1.2.3", "pin", "v1.2.3", "v1.2.4", "v1.2.4"),
            ("v1", "pin", "v1.0.0", "1.1.0", "1.1.0"),
            ("v1.2.3", "pin", "v1.2.3", "1.2.4", "1.2.4"),
            ("1", "pin", "1.0.0", "v1.1.0", "v1.1.0"),
            ("1.2.3", "pin", "1.2.3", "v1.2.4", "v1.2.4"),
            // replace strategy
            ("1", "replace", "1.0.0", "1.1.0", "1"),
            ("1", "replace", "1.0.0", "2.0.0", "2"),
            ("1.2", "replace", "1.2.0", "1.2.1", "1.2"),
            ("1.2", "replace", "1.2.0", "1.3.0", "1.3"),
            ("1.2.3", "replace", "1.2.3", "1.2.4", "1.2.4"),
            ("1.2.3", "replace", "1.2.3", "1.3.0", "1.3.0"),
            ("1", "replace", "1.0.0", "1.2.0", "1"),
            ("1.2", "replace", "1.2.0", "2.0.0", "2.0"),
            ("2", "replace", "2.0.0", "3.0.0", "3"),
            ("2.1", "replace", "2.1.0", "2.2.0", "2.2"),
            ("10.5", "replace", "10.5.0", "10.6.0", "10.6"),
            ("~latest", "replace", "1.0.0", "2.0.0", "~latest"),
            ("1.0.0", "replace", "1.0.0", "1.1.0", "1.1.0"),
            ("1", "replace", "1.0.0", "invalid", "invalid"),
            ("v1", "replace", "v1.0.0", "v1.1.0", "v1"),
            ("v1", "replace", "v1.0.0", "v2.0.0", "v2"),
            ("v1.2", "replace", "v1.2.0", "v1.2.1", "v1.2"),
            ("v1.2", "replace", "v1.2.0", "v1.3.0", "v1.3"),
            ("v1.2.3", "replace", "v1.2.3", "v1.2.4", "v1.2.4"),
            ("v1.2.3", "replace", "v1.2.3", "v1.3.0", "v1.3.0"),
            ("v1", "replace", "v1.0.0", "v1.2.0", "v1"),
            ("v2", "replace", "v2.0.0", "v3.0.0", "v3"),
            ("v1", "replace", "v1.0.0", "1.1.0", "v1"),
            ("v1", "replace", "v1.0.0", "2.0.0", "v2"),
            ("v1.2.3", "replace", "v1.2.3", "1.2.4", "1.2.4"),
            ("1", "replace", "1.0.0", "v1.1.0", "1"),
            ("1", "replace", "1.0.0", "v2.0.0", "2"),
            ("1.2.3", "replace", "1.2.3", "v1.2.4", "v1.2.4"),
            ("not-a-version", "replace", "1.0.0", "2.0.0", "2.0.0"),
        ];
        for (current_value, range_strategy, _current_version, new_version, expected) in cases {
            let result = get_new_value(&NewValueParams {
                current_value: current_value.to_owned(),
                range_strategy: range_strategy.to_owned(),
                new_version: new_version.to_owned(),
            });
            assert_eq!(
                result,
                Some(expected.to_owned()),
                "get_new_value({current_value:?}, {range_strategy:?}, {new_version:?})"
            );
        }
    }
}
