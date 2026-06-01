//! Exact string versioning.
//!
//! Renovate reference:
//! - `lib/modules/versioning/exact/index.ts`

use std::cmp::Ordering;

pub fn is_valid(input: &str) -> bool {
    !input.is_empty()
}

pub fn is_version(input: Option<&str>) -> bool {
    input.is_some_and(is_valid)
}

pub fn is_single_version(input: &str) -> bool {
    is_valid(input)
}

pub fn is_stable(_input: &str) -> bool {
    true
}

pub fn is_compatible(version: &str, current: &str) -> bool {
    version == current
}

pub fn get_major(_input: &str) -> Option<u64> {
    None
}

pub fn get_minor(_input: &str) -> Option<u64> {
    None
}

pub fn get_patch(_input: &str) -> Option<u64> {
    None
}

pub fn equals(a: &str, b: &str) -> bool {
    a == b
}

pub fn is_greater_than(_a: &str, _b: &str) -> bool {
    false
}

pub fn matches(version: &str, range: &str) -> bool {
    version == range
}

pub fn get_satisfying_version<'a>(versions: &'a [&'a str], range: &str) -> Option<&'a str> {
    versions.iter().copied().find(|version| *version == range)
}

pub fn min_satisfying_version<'a>(versions: &'a [&'a str], range: &str) -> Option<&'a str> {
    get_satisfying_version(versions, range)
}

pub fn get_new_value(current_value: &str) -> &str {
    current_value
}

pub fn sort_versions(_a: &str, _b: &str) -> Ordering {
    Ordering::Equal
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isValid(\"$input\") === $expected" — versioning/exact/index.spec.ts line 5
    #[test]
    fn is_valid_matches_renovate_exact_spec() {
        let cases = [
            ("", false),
            ("v1", true),
            ("1.0.0", true),
            ("any-string", true),
            ("abc123", true),
            ("sha256:abcdef", true),
        ];

        for (input, expected) in cases {
            assert_eq!(is_valid(input), expected, "is_valid({input})");
        }
    }

    // Ported: "isVersion($input) === $expected" — versioning/exact/index.spec.ts line 19
    #[test]
    fn is_version_matches_renovate_exact_spec() {
        let cases = [
            (None, false),
            (Some(""), false),
            (Some("v1"), true),
            (Some("1.0.0"), true),
        ];

        for (input, expected) in cases {
            assert_eq!(is_version(input), expected, "is_version({input:?})");
        }
    }

    // Ported: "returns true for any valid version" — versioning/exact/index.spec.ts line 32
    #[test]
    fn is_single_version_returns_true_for_any_valid_version() {
        assert!(is_single_version("1.0.0"));
        assert!(is_single_version("any-string"));
    }

    // Ported: "returns true for any version" — versioning/exact/index.spec.ts line 39
    #[test]
    fn is_stable_returns_true_for_any_version() {
        assert!(is_stable("1.0.0-alpha"));
        assert!(is_stable("1.0.0"));
    }

    // Ported: "returns true when version equals current" — versioning/exact/index.spec.ts line 46
    #[test]
    fn is_compatible_returns_true_when_version_equals_current() {
        assert!(is_compatible("v1", "v1"));
    }

    // Ported: "returns false when version differs from current" — versioning/exact/index.spec.ts line 50
    #[test]
    fn is_compatible_returns_false_when_version_differs_from_current() {
        assert!(!is_compatible("v1.0.0", "v1.1.0"));
    }

    // Ported: "returns null for all" — versioning/exact/index.spec.ts line 56
    #[test]
    fn component_accessors_return_none() {
        assert_eq!(get_major("1.2.3"), None);
        assert_eq!(get_minor("1.2.3"), None);
        assert_eq!(get_patch("1.2.3"), None);
    }

    // Ported: "equals(\"$a\", \"$b\") === $expected" — versioning/exact/index.spec.ts line 64
    #[test]
    fn equals_matches_renovate_exact_spec() {
        let cases = [
            ("1.0.0", "1.0.0", true),
            ("v1", "v1", true),
            ("1.0.0", "1.0", false),
            ("v1", "v2", false),
        ];

        for (a, b, expected) in cases {
            assert_eq!(equals(a, b), expected, "equals({a}, {b})");
        }
    }

    // Ported: "isGreaterThan(\"$a\", \"$b\") === $expected" — versioning/exact/index.spec.ts line 76
    #[test]
    fn is_greater_than_matches_renovate_exact_spec() {
        for (a, b) in [("2.0", "1.0"), ("1.0", "2.0"), ("a", "b")] {
            assert!(!is_greater_than(a, b), "is_greater_than({a}, {b})");
        }
    }

    // Ported: "matches(\"$version\", \"$range\") === $expected" — versioning/exact/index.spec.ts line 87
    #[test]
    fn matches_matches_renovate_exact_spec() {
        let cases = [
            ("1.0.0", "1.0.0", true),
            ("1.0.0", "1.0", false),
            ("v1", "v1", true),
            ("v1", "v2", false),
        ];

        for (version, range, expected) in cases {
            assert_eq!(
                matches(version, range),
                expected,
                "matches({version}, {range})"
            );
        }
    }

    // Ported: "returns exact match only" — versioning/exact/index.spec.ts line 102
    #[test]
    fn get_satisfying_version_returns_exact_match_only() {
        let versions = ["1.0.0", "1.0.1", "2.0.0"];
        assert_eq!(get_satisfying_version(&versions, "1.0.0"), Some("1.0.0"));
        assert_eq!(get_satisfying_version(&versions, "1.0.2"), None);
    }

    // Ported: "returns exact match only" — versioning/exact/index.spec.ts line 110
    #[test]
    fn min_satisfying_version_returns_exact_match_only() {
        let versions = ["1.0.0", "1.0.1", "2.0.0"];
        assert_eq!(min_satisfying_version(&versions, "1.0.1"), Some("1.0.1"));
        assert_eq!(min_satisfying_version(&versions, "3.0.0"), None);
    }

    // Ported: "returns currentValue unchanged" — versioning/exact/index.spec.ts line 118
    #[test]
    fn get_new_value_returns_current_value_unchanged() {
        assert_eq!(get_new_value("v1"), "v1");
    }

    // Ported: "returns 0 for any comparison" — versioning/exact/index.spec.ts line 131
    #[test]
    fn sort_versions_returns_equal_for_any_comparison() {
        assert_eq!(sort_versions("1.0", "2.0"), Ordering::Equal);
        assert_eq!(sort_versions("a", "b"), Ordering::Equal);
    }

    #[test]
    fn get_major_minor_patch_exact() {
        assert_eq!(get_major("1.2.3"), None);
        assert_eq!(get_minor("1.2.3"), None);
        assert_eq!(get_patch("1.2.3"), None);
    }
}
