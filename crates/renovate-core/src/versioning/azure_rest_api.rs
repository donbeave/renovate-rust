//! Azure REST API versioning.
//!
//! Ports `lib/modules/versioning/azure-rest-api/index.ts`.
//! Version format: `YYYY-MM-DD[-prerelease]` (ISO date + optional lowercase suffix).

use std::sync::LazyLock;

use regex::Regex;

static VERSION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})(?P<prerelease>-[a-z]+)?$")
        .unwrap()
});

pub fn is_valid(version: &str) -> bool {
    VERSION_RE.is_match(version)
}

pub fn is_stable(version: &str) -> bool {
    VERSION_RE
        .captures(version)
        .is_some_and(|caps| caps.name("prerelease").is_none())
}

pub fn is_compatible(version: &str) -> bool {
    is_valid(version)
}

pub fn get_major(version: &str) -> Option<u64> {
    VERSION_RE.captures(version).and_then(|caps| {
        let y: u64 = caps["year"].parse().ok()?;
        let m: u64 = caps["month"].parse().ok()?;
        let d: u64 = caps["day"].parse().ok()?;
        Some(y * 10000 + m * 100 + d)
    })
}

pub fn get_minor(version: &str) -> Option<u64> {
    is_valid(version).then_some(0)
}

pub fn get_patch(version: &str) -> Option<u64> {
    is_valid(version).then_some(0)
}

pub fn equals(a: &str, b: &str) -> bool {
    a == b
}

pub fn compare(a: &str, b: &str) -> i32 {
    if a == b {
        return 0;
    }
    if a > b { 1 } else { -1 }
}

pub fn is_greater_than(a: &str, b: &str) -> bool {
    compare(a, b) > 0
}

pub fn sort_versions(a: &str, b: &str) -> i32 {
    compare(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isValid("$version") === $expected" — versioning/azure-rest-api/index.spec.ts line 4
    #[test]
    fn is_valid_matches_renovate_azure_rest_api_index_spec() {
        let cases = [
            ("0000-00-00", true),
            ("2023-01-01", true),
            ("2023-01-01-preview", true),
            ("2023-01-01-alpha", true),
            ("2023-01-01-beta", true),
            ("2023-01-01-rc", true),
            ("2023-01-01-privatepreview", true),
            ("2023-01-01preview", false),
            ("2023-01-01 ", false),
            (" 2023-01-01", false),
            ("2023-01-01-", false),
            ("2023 01 01", false),
            ("2023-01-01-23", false),
            ("2023.01.01", false),
            ("2023_01_01", false),
            ("2023/01/01", false),
        ];
        for (version, expected) in cases {
            assert_eq!(is_valid(version), expected, "is_valid({version:?})");
        }
    }

    // Ported: "isCompatible("$version") === $expected" — versioning/azure-rest-api/index.spec.ts line 26
    #[test]
    fn is_compatible_matches_renovate_azure_rest_api_index_spec() {
        let cases = [("2023-01-01", true), ("2023-01-01-preview", true)];
        for (version, expected) in cases {
            assert_eq!(
                is_compatible(version),
                expected,
                "is_compatible({version:?})"
            );
        }
    }

    // Ported: "isStable("$version") === $expected" — versioning/azure-rest-api/index.spec.ts line 34
    #[test]
    fn is_stable_matches_renovate_azure_rest_api_index_spec() {
        let cases = [
            ("2023-01-01", true),
            ("2023-01-01-preview", false),
            ("2023-01-01-rc", false),
            ("2023-01-01-alpha", false),
            ("2023-01-01-beta", false),
            ("2023-01-01-privatepreview", false),
        ];
        for (version, expected) in cases {
            assert_eq!(is_stable(version), expected, "is_stable({version:?})");
        }
    }

    // Ported: "isSingleVersion("$version") === $expected" — versioning/azure-rest-api/index.spec.ts line 46
    #[test]
    fn is_single_version_matches_renovate_azure_rest_api_index_spec() {
        let cases = [("2023-01-01", true), ("2023-01-01-preview", true)];
        for (version, expected) in cases {
            assert_eq!(
                is_valid(version),
                expected,
                "is_single_version({version:?})"
            );
        }
    }

    // Ported: "isVersion("$version") === $expected" — versioning/azure-rest-api/index.spec.ts line 54
    #[test]
    fn is_version_matches_renovate_azure_rest_api_index_spec() {
        let cases = [
            ("2023-01-01", true),
            ("2023-01-01-preview", true),
            ("1.2.3", false),
        ];
        for (version, expected) in cases {
            assert_eq!(is_valid(version), expected, "is_version({version:?})");
        }
    }

    // Ported: "getMajor("$version") === 1" — versioning/azure-rest-api/index.spec.ts line 66
    #[test]
    fn get_major_matches_renovate_azure_rest_api_index_spec() {
        let cases = [
            ("2023-01-01", Some(20230101u64)),
            ("2023-01-01-preview", Some(20230101)),
        ];
        for (version, expected) in cases {
            assert_eq!(get_major(version), expected, "get_major({version:?})");
        }
    }

    // Ported: "getMinor("$version") === 0" — versioning/azure-rest-api/index.spec.ts line 74
    #[test]
    fn get_minor_matches_renovate_azure_rest_api_index_spec() {
        let cases = [("2023-01-01", Some(0u64)), ("2023-01-01-preview", Some(0))];
        for (version, expected) in cases {
            assert_eq!(get_minor(version), expected, "get_minor({version:?})");
        }
    }

    // Ported: "getPatch("$version") === 0" — versioning/azure-rest-api/index.spec.ts line 82
    #[test]
    fn get_patch_matches_renovate_azure_rest_api_index_spec() {
        let cases = [("2023-01-01", Some(0u64)), ("2023-01-01-preview", Some(0))];
        for (version, expected) in cases {
            assert_eq!(get_patch(version), expected, "get_patch({version:?})");
        }
    }

    // Ported: "equals("$version", "$other") === $expected" — versioning/azure-rest-api/index.spec.ts line 90
    #[test]
    fn equals_matches_renovate_azure_rest_api_index_spec() {
        let cases = [
            ("2023-01-01", "2023-01-01", true),
            ("2023-01-01-preview", "2023-01-01-preview", true),
            ("2023-01-01", "2023-01-02", false),
            ("2023-01-01", "2023-02-01", false),
            ("2023-01-01", "2024-01-01", false),
        ];
        for (version, other, expected) in cases {
            assert_eq!(
                equals(version, other),
                expected,
                "equals({version:?}, {other:?})"
            );
        }
    }

    // Ported: "isGreaterThan("$version", "$other") === $expected" — versioning/azure-rest-api/index.spec.ts line 104
    #[test]
    fn is_greater_than_matches_renovate_azure_rest_api_index_spec() {
        let cases = [
            ("2023-01-01", "2023-01-02", false),
            ("2023-01-01", "2023-02-01", false),
            ("2023-01-01", "2024-01-01", false),
            ("2023-01-01", "2023-01-01", false),
            ("2023-01-01-preview", "2023-01-01-preview", false),
            ("2023-01-02", "2023-01-01", true),
            ("2023-02-01", "2023-01-01", true),
            ("2024-01-01", "2023-01-01", true),
        ];
        for (version, other, expected) in cases {
            assert_eq!(
                is_greater_than(version, other),
                expected,
                "is_greater_than({version:?}, {other:?})"
            );
        }
    }

    // Ported: "sortVersions("$version", "$other") === $expected" — versioning/azure-rest-api/index.spec.ts line 121
    #[test]
    fn sort_versions_matches_renovate_azure_rest_api_index_spec() {
        let cases = [
            ("2023-01-01", "2023-01-01", 0),
            ("2023-01-01-preview", "2023-01-01-preview", 0),
            ("2023-01-01", "2023-01-02", -1),
            ("2023-01-01", "2023-02-01", -1),
            ("2023-01-01", "2024-01-01", -1),
            ("2023-01-02", "2023-01-01", 1),
            ("2023-02-01", "2023-01-01", 1),
            ("2024-01-01", "2023-01-01", 1),
        ];
        for (version, other, expected) in cases {
            assert_eq!(
                sort_versions(version, other),
                expected,
                "sort_versions({version:?}, {other:?})"
            );
        }
    }

    #[test]
    fn compare_direct() {
        assert_eq!(compare("2023-01-01", "2023-01-01"), 0);
        assert_eq!(compare("2023-01-02", "2023-01-01"), 1);
        assert_eq!(compare("2023-01-01", "2023-01-02"), -1);
    }
}
