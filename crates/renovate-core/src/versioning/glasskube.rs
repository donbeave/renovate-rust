//! Glasskube versioning scheme.
//!
//! SemVer-based; build metadata component is included in ordering.
//! A non-empty prerelease identifier marks the version as unstable.
//!
//! Renovate reference: `lib/modules/versioning/glasskube/index.ts`

use std::cmp::Ordering;

use semver::Version;

#[derive(Debug, Clone)]
struct GlasskubeVersion {
    major: u64,
    minor: u64,
    patch: u64,
    /// First numeric build-metadata component (0 when absent).
    build_num: u64,
    /// Non-empty when version has a prerelease identifier.
    prerelease: String,
}

fn parse(v: &str) -> Option<GlasskubeVersion> {
    let stripped = v.strip_prefix('v').unwrap_or(v);
    let sv = Version::parse(stripped).ok()?;
    let build_num = if sv.build.is_empty() {
        0
    } else {
        sv.build
            .as_str()
            .split('.')
            .next()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0)
    };
    let prerelease = if sv.pre.is_empty() {
        String::new()
    } else {
        sv.pre.to_string()
    };
    Some(GlasskubeVersion {
        major: sv.major,
        minor: sv.minor,
        patch: sv.patch,
        build_num,
        prerelease,
    })
}

fn compare(a: &GlasskubeVersion, b: &GlasskubeVersion) -> Ordering {
    let rel =
        (a.major, a.minor, a.patch, a.build_num).cmp(&(b.major, b.minor, b.patch, b.build_num));
    if rel != Ordering::Equal {
        return rel;
    }
    match (!a.prerelease.is_empty(), !b.prerelease.is_empty()) {
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        _ => Ordering::Equal,
    }
}

pub fn is_valid(version: &str) -> bool {
    parse(version).is_some()
}

pub fn is_stable(version: &str) -> bool {
    parse(version).is_some_and(|v| v.prerelease.is_empty())
}

pub fn get_major(version: &str) -> Option<u64> {
    parse(version).map(|v| v.major)
}

pub fn get_minor(version: &str) -> Option<u64> {
    parse(version).map(|v| v.minor)
}

pub fn get_patch(version: &str) -> Option<u64> {
    parse(version).map(|v| v.patch)
}

pub fn is_greater_than(a: &str, b: &str) -> bool {
    parse(a)
        .zip(parse(b))
        .is_some_and(|(a, b)| compare(&a, &b) == Ordering::Greater)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isStable(\"$version\") === $expected" — versioning/glasskube/index.spec.ts line 6
    #[test]
    fn is_stable_matches_renovate_glasskube_spec() {
        let cases = [
            ("v1.2.3", true),
            ("v1.2.3+1", true),
            ("v1.2.3-1", false),
            ("v1.2.3-1+1", false),
        ];
        for (version, expected) in cases {
            assert_eq!(is_stable(version), expected, "is_stable({version:?})");
        }
    }

    // Ported: "isValid(\"$version\") === $expected" — versioning/glasskube/index.spec.ts line 16
    #[test]
    fn is_valid_matches_renovate_glasskube_spec() {
        let cases = [
            ("alpha", false),
            ("v1", false),
            ("v1.2", false),
            ("v1.2.3", true),
            ("v1.2.3+1", true),
            ("v1.2.3-1", true),
            ("v1.2.3-1+1", true),
            ("1.2.3-1+1", true),
        ];
        for (version, expected) in cases {
            assert_eq!(is_valid(version), expected, "is_valid({version:?})");
        }
    }

    // Ported: "getMajor, getMinor, getPatch for \"$version\"" — versioning/glasskube/index.spec.ts line 30
    #[test]
    fn get_components_matches_renovate_glasskube_spec() {
        let cases = [
            ("v1.2.3", 1u64, 2u64, 3u64),
            ("v1.2.3+1", 1, 2, 3),
            ("v1.2.3-1", 1, 2, 3),
        ];
        for (version, major, minor, patch) in cases {
            assert_eq!(get_major(version), Some(major), "get_major({version:?})");
            assert_eq!(get_minor(version), Some(minor), "get_minor({version:?})");
            assert_eq!(get_patch(version), Some(patch), "get_patch({version:?})");
        }
    }

    // Ported: "getMajor, getMinor, getPatch for \"$version\"" — versioning/glasskube/index.spec.ts line 44
    #[test]
    fn is_greater_than_matches_renovate_glasskube_spec() {
        let cases = [
            ("v1.2.3+1", "v1.2.3"),
            ("v1.2.3+2", "v1.2.3+1"),
            ("v1.2.3+1", "v1.2.3-1"),
            ("v1.2.3+1", "v1.2.3-1+1"),
            ("v1.2.3-1+1", "v1.2.3-1"),
        ];
        for (version_b, version_a) in cases {
            assert!(
                is_greater_than(version_b, version_a),
                "is_greater_than({version_b:?}, {version_a:?})"
            );
        }
    }

    #[test]
    fn get_components_invalid_returns_none() {
        assert_eq!(get_major("invalid"), None);
        assert_eq!(get_minor("invalid"), None);
        assert_eq!(get_patch("invalid"), None);
    }

    #[test]
    fn is_greater_than_invalid_returns_false() {
        assert!(!is_greater_than("invalid", "v1.2.3"));
        assert!(!is_greater_than("v1.2.3", "invalid"));
        assert!(!is_greater_than("invalid", "invalid"));
    }

    #[test]
    fn is_stable_invalid_returns_false() {
        assert!(!is_stable("invalid"));
    }

    #[test]
    fn is_valid_invalid_returns_false() {
        assert!(!is_valid(""));
        assert!(!is_valid("not-a-version"));
    }
}
