//! npm-semver versioning scheme.
//!
//! Strict SemVer: `MAJOR.MINOR.PATCH[-prerelease][+build]`.
//! Leading zeros and ranges are not valid versions.
//!
//! Renovate reference: `lib/modules/versioning/semver/index.ts`

use semver::Version;

/// A version string is valid iff it can be parsed as a strict SemVer version.
pub fn is_valid(input: &str) -> bool {
    Version::parse(input.trim()).is_ok()
}

/// True for any valid SemVer version (pinned or prerelease).
pub fn is_single_version(version: &str) -> bool {
    is_valid(version)
}

/// Whether upgrading from `current` to `version` is a breaking change.
///
/// A change is breaking if:
/// - Either version is a prerelease (unstable), or
/// - `current` has major version 0 (all 0.x changes are potentially breaking), or
/// - The major version increases.
pub fn is_breaking(current: &str, version: &str) -> bool {
    let Ok(cur) = Version::parse(current.trim()) else {
        return true;
    };
    let Ok(ver) = Version::parse(version.trim()) else {
        return true;
    };
    if !cur.pre.is_empty() || !ver.pre.is_empty() {
        return true;
    }
    if cur.major == 0 {
        return true;
    }
    cur.major != ver.major
}

/// A valid SemVer version is compatible with itself.
pub fn is_compatible(version: &str) -> bool {
    is_valid(version)
}

/// Compute the new pinned value.
///
/// If `current_version` equals `"v{current_value}"`, the caller pinned without
/// the v-prefix, so we strip `v` from `new_version` to match.
pub fn get_new_value(current_value: &str, current_version: &str, new_version: &str) -> String {
    if current_version == format!("v{current_value}") {
        new_version.trim_start_matches('v').to_owned()
    } else {
        new_version.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isValid(\"$version\") === $expected" — lib/modules/versioning/semver/index.spec.ts line 4
    #[test]
    fn is_valid_matches_renovate_semver_spec() {
        let cases = [
            ("17.04.0", false),
            ("1.2.3", true),
            ("1.2.3-foo", true),
            ("1.2.3foo", false),
            ("~1.2.3", false),
            ("^1.2.3", false),
            (">1.2.3", false),
            ("renovatebot/renovate", false),
            ("renovatebot/renovate#master", false),
            ("https://github.com/renovatebot/renovate.git", false),
        ];
        for (version, expected) in cases {
            assert_eq!(is_valid(version), expected, "is_valid({version:?})");
        }
    }

    // Ported: "isSingleVersion(\"$version\") === $expected" — lib/modules/versioning/semver/index.spec.ts line 20
    #[test]
    fn is_single_version_matches_renovate_semver_spec() {
        let cases = [
            ("1.2.3", true),
            ("1.2.3-alpha.1", true),
            ("=1.2.3", false),
            ("= 1.2.3", false),
            ("1.x", false),
        ];
        for (version, expected) in cases {
            assert_eq!(
                is_single_version(version),
                expected,
                "is_single_version({version:?})"
            );
        }
    }

    // Ported: "getNewValue(\"$currentValue\", \"$rangeStrategy\", \"$currentVersion\", \"$newVersion\") === \"$expected\"" — lib/modules/versioning/semver/index.spec.ts line 31
    #[test]
    fn get_new_value_matches_renovate_semver_spec() {
        assert_eq!(
            get_new_value("=1.0.0", "1.0.0", "1.1.0"),
            "1.1.0",
            "strip-prefix case"
        );
        assert_eq!(
            get_new_value("1.0.0", "v1.0.0", "v2.0.0"),
            "2.0.0",
            "v-prefix stripped"
        );
    }

    // Ported: "isBreaking(\"$currentVersion\", \"$newVersion\") === $expected" — lib/modules/versioning/semver/index.spec.ts line 48
    #[test]
    fn is_breaking_matches_renovate_semver_spec() {
        let cases = [
            ("0.0.1", "0.0.2", true),
            ("0.0.1", "0.2.0", true),
            ("0.0.1", "1.0.0", true),
            ("1.0.0-alpha.1", "1.0.0", true),
            ("1.0.0-alpha.1", "1.0.0-alpha.2", true),
            ("1.0.0", "2.0.0-alpha.1", true),
            ("1.0.0", "1.0.0", false),
            ("1.0.0", "2.0.0", true),
            ("2.0.0", "2.0.1", false),
            ("2.0.0", "2.1.0", false),
        ];
        for (cur, ver, expected) in cases {
            assert_eq!(
                is_breaking(cur, ver),
                expected,
                "is_breaking({cur:?}, {ver:?})"
            );
        }
    }

    // Ported: "isCompatible(\"$version\") === $expected" — lib/modules/versioning/semver/index.spec.ts line 67
    #[test]
    fn is_compatible_matches_renovate_semver_spec() {
        assert!(is_compatible("1.2.0"));
    }
}
