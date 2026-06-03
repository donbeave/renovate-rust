//! @parity lib/modules/versioning/semver-coerced/index.ts full
//!
//! Coerced semantic versioning.

use std::cmp::Ordering;
use std::sync::LazyLock;

use regex::Regex;
use semver::Version;

static COERCE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(\d+)(?:\.(\d+|x))?(?:\.(\d+|x))?").unwrap());
static STABLE_PREFIX_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^v?(?<major>\d+)(?<minor>\.\d+)?(?<patch>\.\d+)?(?<others>.+)?").unwrap()
});
static STARTS_WITH_NUMBER_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d").unwrap());

fn coerce(input: &str) -> Option<Version> {
    let captures = COERCE_RE.captures(input)?;
    let component = |index| match captures.get(index).map(|m| m.as_str()) {
        Some(value) if value.eq_ignore_ascii_case("x") => Some(0),
        Some(value) if value.len() > 1 && value.starts_with('0') => None,
        Some(value) => value.parse::<u64>().ok(),
        None => Some(0),
    };
    let major = component(1)?;
    let minor = component(2)?;
    let patch = component(3)?;
    Version::parse(&format!("{major}.{minor}.{patch}")).ok()
}

fn parse_strict(input: &str) -> Option<Version> {
    Version::parse(input.trim_start_matches('v')).ok()
}

fn lower_bound_for(range: &str) -> Option<Version> {
    let range = range.trim();
    let version = range
        .trim_start_matches(['^', '~', '>', '<', '=', ' '])
        .trim();
    coerce(version)
}

fn satisfies_single(version: &Version, range: &str) -> bool {
    let range = range.trim();
    if range.is_empty() {
        return false;
    }

    // Compound range: ">=1.0.0 <2" — all parts must match
    if range.contains(' ') {
        return range
            .split_whitespace()
            .all(|part| satisfies_single(version, part));
    }

    if let Some(rest) = range.strip_prefix('^') {
        let Some(lower) = coerce(rest) else {
            return false;
        };
        let upper = if lower.major > 0 {
            Version::new(lower.major + 1, 0, 0)
        } else if lower.minor > 0 {
            Version::new(0, lower.minor + 1, 0)
        } else {
            Version::new(0, 0, lower.patch + 1)
        };
        return version >= &lower && version < &upper && version.pre.is_empty();
    }

    if let Some(rest) = range.strip_prefix('~') {
        let Some(lower) = coerce(rest) else {
            return false;
        };
        let upper = Version::new(lower.major, lower.minor + 1, 0);
        return version >= &lower && version < &upper && version.pre.is_empty();
    }

    if let Some(rest) = range.strip_prefix(">=") {
        let Some(lower) = coerce(rest) else {
            return false;
        };
        return version >= &lower && version.pre.is_empty();
    }

    if let Some(rest) = range.strip_prefix('>') {
        let Some(lower) = coerce(rest) else {
            return false;
        };
        return version > &lower && version.pre.is_empty();
    }

    if let Some(rest) = range.strip_prefix("<=") {
        let Some(upper) = coerce(rest) else {
            return false;
        };
        return version <= &upper && version.pre.is_empty();
    }

    if let Some(rest) = range.strip_prefix('<') {
        let Some(upper) = coerce(rest) else {
            return false;
        };
        return version < &upper && version.pre.is_empty();
    }

    let Some(exact) = coerce(range) else {
        return false;
    };
    version == &exact
}

fn satisfies(version: &Version, range: &str) -> bool {
    range
        .split("||")
        .any(|single| satisfies_single(version, single))
}

pub fn equals(a: &str, b: &str) -> bool {
    match (coerce(a), coerce(b)) {
        (Some(a), Some(b)) => a == b,
        _ => false,
    }
}

pub fn get_major(input: &str) -> Option<u64> {
    coerce(input).map(|version| version.major)
}

pub fn get_minor(input: &str) -> Option<u64> {
    coerce(input).map(|version| version.minor)
}

pub fn get_patch(input: &str) -> Option<u64> {
    coerce(input).map(|version| version.patch)
}

pub fn is_breaking(current: &str, version: &str) -> bool {
    let Some(current) = coerce(current) else {
        return false;
    };
    let Some(version) = coerce(version) else {
        return false;
    };

    version.major > current.major || (current.major == 0 && version.minor > current.minor)
}

pub fn is_compatible(input: &str) -> bool {
    is_version(input)
}

pub fn is_greater_than(a: &str, b: &str) -> bool {
    match (coerce(a), coerce(b)) {
        (Some(a), Some(b)) => a > b,
        _ => false,
    }
}

pub fn is_less_than_range(version: &str, range: &str) -> bool {
    let Some(version) = coerce(version) else {
        return false;
    };
    let Some(lower) = lower_bound_for(range) else {
        return false;
    };
    version < lower
}

pub fn is_single_version(input: &str) -> bool {
    (input.starts_with('v') || STARTS_WITH_NUMBER_RE.is_match(input)) && coerce(input).is_some()
}

pub fn is_stable(input: &str) -> bool {
    let Some(captures) = STABLE_PREFIX_RE.captures(input) else {
        return false;
    };
    let major = captures.name("major").map(|m| m.as_str()).unwrap_or("");
    let minor = captures.name("minor").map(|m| m.as_str()).unwrap_or(".0");
    let patch = captures.name("patch").map(|m| m.as_str()).unwrap_or(".0");
    let others = captures.name("others").map(|m| m.as_str()).unwrap_or("");
    Version::parse(&format!("{major}{minor}{patch}{others}"))
        .is_ok_and(|version| version.pre.is_empty())
}

pub fn is_valid(input: &str) -> bool {
    coerce(input).is_some()
}

pub fn is_version(input: &str) -> bool {
    is_valid(input)
}

pub fn matches(version: &str, range: &str) -> bool {
    coerce(version).is_some_and(|version| satisfies(&version, range))
}

pub fn get_satisfying_version(versions: &[&str], range: &str) -> Option<String> {
    versions
        .iter()
        .filter_map(|version| parse_strict(version).or_else(|| coerce(version)))
        .filter(|version| satisfies(version, range))
        .max()
        .map(|version| version.to_string())
}

pub fn min_satisfying_version(versions: &[&str], range: &str) -> Option<String> {
    versions
        .iter()
        .filter_map(|version| coerce(version))
        .filter(|version| satisfies(version, range))
        .min()
        .map(|version| version.to_string())
}

pub fn get_new_value(current_value: &str, current_version: &str, new_version: &str) -> String {
    if current_version == format!("v{current_value}") {
        new_version.trim_start_matches('v').to_owned()
    } else {
        new_version.to_owned()
    }
}

pub fn sort_versions(a: &str, b: &str) -> Ordering {
    match (coerce(a), coerce(b)) {
        (Some(a), Some(b)) => a.cmp(&b),
        _ => Ordering::Equal,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "should return true for strictly equal versions" — lib/modules/versioning/semver-coerced/index.spec.ts line 5
    #[test]
    fn equals_returns_true_for_strictly_equal_versions() {
        assert!(equals("1.0.0", "1.0.0"));
    }

    // Ported: "should return true for non-strictly equal versions" — lib/modules/versioning/semver-coerced/index.spec.ts line 9
    #[test]
    fn equals_returns_true_for_non_strictly_equal_versions() {
        assert!(equals("v1.0", "1.0.0"));
        assert!(equals("v1.0", "v1.x"));
    }

    // Ported: "should return false for non-equal versions" — lib/modules/versioning/semver-coerced/index.spec.ts line 14
    #[test]
    fn equals_returns_false_for_non_equal_versions() {
        assert!(!equals("2.0.1", "2.3.0"));
    }

    // Ported: "invalid version" — lib/modules/versioning/semver-coerced/index.spec.ts line 18
    #[test]
    fn equals_returns_false_for_invalid_version() {
        assert!(!equals("xxx", "1.2.3"));
    }

    // Ported: "should return major version number for strict semver" — lib/modules/versioning/semver-coerced/index.spec.ts line 24
    #[test]
    fn get_major_returns_major_for_strict_semver() {
        assert_eq!(get_major("1.0.2"), Some(1));
    }

    // Ported: "should return major version number for non-strict semver" — lib/modules/versioning/semver-coerced/index.spec.ts line 28
    #[test]
    fn get_major_returns_major_for_non_strict_semver() {
        assert_eq!(get_major("v3.1"), Some(3));
    }

    // Ported: "invalid version" — lib/modules/versioning/semver-coerced/index.spec.ts line 32
    #[test]
    fn get_major_returns_none_for_invalid_version() {
        assert_eq!(get_major("xxx"), None);
    }

    // Ported: "should return minor version number for strict semver" — lib/modules/versioning/semver-coerced/index.spec.ts line 38
    #[test]
    fn get_minor_returns_minor_for_strict_semver() {
        assert_eq!(get_minor("1.0.2"), Some(0));
    }

    // Ported: "should return minor version number for non-strict semver" — lib/modules/versioning/semver-coerced/index.spec.ts line 42
    #[test]
    fn get_minor_returns_minor_for_non_strict_semver() {
        assert_eq!(get_minor("v3.1"), Some(1));
    }

    // Ported: "invalid version" — lib/modules/versioning/semver-coerced/index.spec.ts line 46
    #[test]
    fn get_minor_returns_none_for_invalid_version() {
        assert_eq!(get_minor("xxx"), None);
    }

    // Ported: "getPatch(\"$version\") === $expected" — lib/modules/versioning/semver-coerced/index.spec.ts line 52
    #[test]
    fn get_patch_matches_renovate_semver_coerced_spec() {
        let cases = [
            ("1.0.2", Some(2)),
            ("v3.1.2-foo", Some(2)),
            ("v1.3.5", Some(5)),
            ("v2.1", Some(0)),
            ("3.4", Some(0)),
            ("v2", Some(0)),
            ("2", Some(0)),
            ("v1.0.4-alpha", Some(4)),
            ("1.0.3-Beta.1", Some(3)),
            ("1.0.0-rc2", Some(0)),
            ("v1.0.8-rc2", Some(8)),
            ("1.0-Beta.0", Some(0)),
            ("two1.0", Some(0)),
            ("ver1.2.3", Some(3)),
            ("r3.0", Some(0)),
            ("abc", None),
        ];

        for (version, expected) in cases {
            assert_eq!(get_patch(version), expected, "get_patch({version})");
        }
    }

    // Ported: "should return false for patch updates" — lib/modules/versioning/semver-coerced/index.spec.ts line 76
    #[test]
    fn is_breaking_returns_false_for_patch_updates() {
        assert!(!is_breaking("1.0", "1.0.1"));
    }

    // Ported: "should return false for minor updates" — lib/modules/versioning/semver-coerced/index.spec.ts line 80
    #[test]
    fn is_breaking_returns_false_for_minor_updates() {
        assert!(!is_breaking("1.0", "1.1"));
    }

    // Ported: "should return true for major updates" — lib/modules/versioning/semver-coerced/index.spec.ts line 84
    #[test]
    fn is_breaking_returns_true_for_major_updates() {
        assert!(is_breaking("1.0.0", "2"));
    }

    // Ported: "should return true for major updates from v0.x" — lib/modules/versioning/semver-coerced/index.spec.ts line 88
    #[test]
    fn is_breaking_returns_true_for_major_updates_from_v0() {
        assert!(is_breaking("0.0.0", "1.0.0"));
    }

    // Ported: "should return true for major updates within v0.x" — lib/modules/versioning/semver-coerced/index.spec.ts line 92
    #[test]
    fn is_breaking_returns_true_for_major_updates_within_v0() {
        assert!(is_breaking("0.1", "0.2.1"));
    }

    // Ported: "should return true for strict semver" — lib/modules/versioning/semver-coerced/index.spec.ts line 98
    #[test]
    fn is_compatible_returns_true_for_strict_semver() {
        assert!(is_compatible("1.0.2"));
    }

    // Ported: "should return true for non-strict semver" — lib/modules/versioning/semver-coerced/index.spec.ts line 102
    #[test]
    fn is_compatible_returns_true_for_non_strict_semver() {
        assert!(is_compatible("v3.1.2-foo"));
    }

    // Ported: "should return false for non-semver" — lib/modules/versioning/semver-coerced/index.spec.ts line 106
    #[test]
    fn is_compatible_returns_false_for_non_semver() {
        assert!(!is_compatible("foo"));
    }

    // Ported: "should return true for a greater version in strict semver" — lib/modules/versioning/semver-coerced/index.spec.ts line 112
    #[test]
    fn is_greater_than_returns_true_for_greater_strict_semver() {
        assert!(is_greater_than("1.0.2", "1.0.0"));
    }

    // Ported: "should return false for lower version in strict semver" — lib/modules/versioning/semver-coerced/index.spec.ts line 116
    #[test]
    fn is_greater_than_returns_false_for_lower_strict_semver() {
        assert!(!is_greater_than("3.1.2", "4.1.0"));
    }

    // Ported: "should return false if version cannot be coerced" — lib/modules/versioning/semver-coerced/index.spec.ts line 120
    #[test]
    fn is_greater_than_returns_false_if_version_cannot_be_coerced() {
        assert!(!is_greater_than("e.e.e", "4.1.0"));
    }

    // Ported: "should return true for a lower version in strict semver" — lib/modules/versioning/semver-coerced/index.spec.ts line 126
    #[test]
    fn is_less_than_range_returns_true_for_lower_strict_semver() {
        assert!(is_less_than_range("1.0.2", "~2.0"));
    }

    // Ported: "should return false for in-range version in strict semver" — lib/modules/versioning/semver-coerced/index.spec.ts line 130
    #[test]
    fn is_less_than_range_returns_false_for_in_range_strict_semver() {
        assert!(!is_less_than_range("3.0.2", "~3.0"));
    }

    // Ported: "invalid version" — lib/modules/versioning/semver-coerced/index.spec.ts line 134
    #[test]
    fn is_less_than_range_returns_false_for_invalid_version() {
        assert!(!is_less_than_range("xxx", "1.2.3"));
    }

    // Ported: "returns true if naked version" — lib/modules/versioning/semver-coerced/index.spec.ts line 140
    #[test]
    fn is_single_version_returns_true_for_naked_version() {
        assert!(is_single_version("1.2.3"));
        assert!(is_single_version("1.2.3-alpha.1"));
    }

    // Ported: "returns false if equals" — lib/modules/versioning/semver-coerced/index.spec.ts line 145
    #[test]
    fn is_single_version_returns_false_if_equals() {
        assert!(!is_single_version("=1.2.3"));
        assert!(!is_single_version("= 1.2.3"));
    }

    // Ported: "returns false when not version" — lib/modules/versioning/semver-coerced/index.spec.ts line 150
    #[test]
    fn is_single_version_returns_false_when_not_version() {
        assert!(!is_single_version("~1.0"));
    }

    // Ported: "isStable(\"$version\") === $expected" — lib/modules/versioning/semver-coerced/index.spec.ts line 156
    #[test]
    fn is_stable_matches_renovate_semver_coerced_spec() {
        let cases = [
            ("1.0.0", true),
            ("v1.3.5", true),
            ("v2.1", true),
            ("3.4", true),
            ("v2", true),
            ("2", true),
            ("v1.0.0-alpha", false),
            ("1.0.0-Beta.1", false),
            ("1.0.0-rc2", false),
            ("v1.0.0-rc2", false),
            ("1.0-Beta.0", false),
            ("v1.0-alpha", false),
            ("two1.0", false),
            ("ver1.2.3", false),
            ("r3.0", false),
        ];

        for (version, expected) in cases {
            assert_eq!(is_stable(version), expected, "is_stable({version})");
        }
    }

    // Ported: "should return null for non-digit version strings" — lib/modules/versioning/semver-coerced/index.spec.ts line 179
    #[test]
    fn is_valid_returns_false_for_non_digit_version_strings() {
        assert!(!is_valid("version two"));
    }

    // Ported: "should return null for irregular version strings" — lib/modules/versioning/semver-coerced/index.spec.ts line 183
    #[test]
    fn is_valid_returns_false_for_irregular_version_strings() {
        assert!(!is_valid("17.04.0"));
    }

    // Ported: "should support strict semver" — lib/modules/versioning/semver-coerced/index.spec.ts line 187
    #[test]
    fn is_valid_supports_strict_semver() {
        assert!(is_valid("1.2.3"));
    }

    // Ported: "should treat semver with dash as a valid version" — lib/modules/versioning/semver-coerced/index.spec.ts line 191
    #[test]
    fn is_valid_treats_semver_with_dash_as_valid_version() {
        assert!(is_valid("1.2.3-foo"));
    }

    // Ported: "should treat semver without dash as a valid version" — lib/modules/versioning/semver-coerced/index.spec.ts line 195
    #[test]
    fn is_valid_treats_semver_without_dash_as_valid_version() {
        assert!(is_valid("1.2.3foo"));
    }

    // Ported: "should treat ranges as valid versions" — lib/modules/versioning/semver-coerced/index.spec.ts line 199
    #[test]
    fn is_valid_treats_ranges_as_valid_versions() {
        assert!(is_valid("~1.2.3"));
        assert!(is_valid("^1.2.3"));
        assert!(is_valid(">1.2.3"));
    }

    // Ported: "should reject github repositories" — lib/modules/versioning/semver-coerced/index.spec.ts line 205
    #[test]
    fn is_valid_rejects_github_repositories() {
        assert!(!is_valid("renovatebot/renovate"));
        assert!(!is_valid("renovatebot/renovate#master"));
        assert!(!is_valid("https://github.com/renovatebot/renovate.git"));
    }

    // Ported: "should return null for non-digit versions" — lib/modules/versioning/semver-coerced/index.spec.ts line 215
    #[test]
    fn is_version_returns_false_for_non_digit_versions() {
        assert!(!is_version("version one"));
    }

    // Ported: "should support strict semver versions" — lib/modules/versioning/semver-coerced/index.spec.ts line 219
    #[test]
    fn is_version_supports_strict_semver_versions() {
        assert!(is_version("1.2.3"));
    }

    // Ported: "should support non-strict versions" — lib/modules/versioning/semver-coerced/index.spec.ts line 223
    #[test]
    fn is_version_supports_non_strict_versions() {
        assert!(is_version("v1.2"));
    }

    // Ported: "should return true when version is in range" — lib/modules/versioning/semver-coerced/index.spec.ts line 229
    #[test]
    fn matches_returns_true_when_version_is_in_range() {
        assert!(matches("1.0.0", "1.0.0 || 1.0.1"));
    }

    // Ported: "should return true with non-strict version in range" — lib/modules/versioning/semver-coerced/index.spec.ts line 233
    #[test]
    fn matches_returns_true_with_non_strict_version_in_range() {
        assert!(matches("v1.0", "1.0.0 || 1.0.1"));
    }

    // Ported: "should return false when version is not in range" — lib/modules/versioning/semver-coerced/index.spec.ts line 237
    #[test]
    fn matches_returns_false_when_version_is_not_in_range() {
        assert!(!matches("1.2.3", "1.4.1 || 1.4.2"));
    }

    // Ported: "invalid version" — lib/modules/versioning/semver-coerced/index.spec.ts line 241
    #[test]
    fn matches_returns_false_for_invalid_version() {
        assert!(!matches("xxx", "1.2.3"));
    }

    // Ported: "should return max satisfying version in range" — lib/modules/versioning/semver-coerced/index.spec.ts line 247
    #[test]
    fn get_satisfying_version_returns_max_satisfying_version_in_range() {
        assert_eq!(
            get_satisfying_version(&["1.0.0", "1.0.4"], "^1.0").as_deref(),
            Some("1.0.4")
        );
    }

    // Ported: "should support coercion" — lib/modules/versioning/semver-coerced/index.spec.ts line 253
    #[test]
    fn get_satisfying_version_supports_coercion() {
        assert_eq!(
            get_satisfying_version(&["v1.0", "1.0.4-foo"], "^1.0").as_deref(),
            Some("1.0.0")
        );
    }

    // Ported: "should return min satisfying version in range" — lib/modules/versioning/semver-coerced/index.spec.ts line 261
    #[test]
    fn min_satisfying_version_returns_min_satisfying_version_in_range() {
        assert_eq!(
            min_satisfying_version(&["1.0.0", "1.0.4"], "^1.0").as_deref(),
            Some("1.0.0")
        );
    }

    // Ported: "should support coercion" — lib/modules/versioning/semver-coerced/index.spec.ts line 267
    #[test]
    fn min_satisfying_version_supports_coercion() {
        assert_eq!(
            min_satisfying_version(&["v1.0", "1.0.4-foo"], "^1.0").as_deref(),
            Some("1.0.0")
        );
    }

    // Ported: "uses newVersion" — lib/modules/versioning/semver-coerced/index.spec.ts line 275
    #[test]
    fn get_new_value_uses_new_version() {
        assert_eq!(get_new_value("=1.0.0", "1.0.0", "1.1.0"), "1.1.0");
        assert_eq!(get_new_value("1.0.0", "v1.0.0", "v1.1.0"), "1.1.0");
        assert_eq!(get_new_value("1.0.0", "v1.0.0", "1.1.0"), "1.1.0");
    }

    // Ported: "should return zero for equal versions" — lib/modules/versioning/semver-coerced/index.spec.ts line 304
    #[test]
    fn sort_versions_returns_zero_for_equal_versions() {
        assert_eq!(sort_versions("1.0.0", "1.0.0"), Ordering::Equal);
    }

    // Ported: "should return -1 for a < b" — lib/modules/versioning/semver-coerced/index.spec.ts line 308
    #[test]
    fn sort_versions_returns_less_for_a_less_than_b() {
        assert_eq!(sort_versions("1.0.0", "1.0.1"), Ordering::Less);
    }

    // Ported: "should return 1 for a > b" — lib/modules/versioning/semver-coerced/index.spec.ts line 312
    #[test]
    fn sort_versions_returns_greater_for_a_greater_than_b() {
        assert_eq!(sort_versions("1.0.1", "1.0.0"), Ordering::Greater);
    }

    // Ported: "should return zero for equal non-strict versions" — lib/modules/versioning/semver-coerced/index.spec.ts line 316
    #[test]
    fn sort_versions_returns_zero_for_equal_non_strict_versions() {
        assert_eq!(sort_versions("v1.0", "1.x"), Ordering::Equal);
    }

    // Ported: "works with invalid version" — lib/modules/versioning/semver-coerced/index.spec.ts line 320
    #[test]
    fn sort_versions_works_with_invalid_version() {
        assert_eq!(sort_versions("v1.0", "xx"), Ordering::Equal);
    }
}
