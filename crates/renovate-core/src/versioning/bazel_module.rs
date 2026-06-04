//! @parity lib/modules/versioning/bazel-module/index.ts full
//! Bazel module (Bzlmod) versioning API.
//!
//! Ports `lib/modules/versioning/bazel-module/index.ts`.
//!
//! No range syntax — every "range" is an exact version match.
//! Version comparison delegates to `BzlmodVersion`.

pub mod bzlmod_version;

use bzlmod_version::BzlmodVersion;

fn parse(version: &str) -> Option<BzlmodVersion> {
    BzlmodVersion::new(version).ok()
}

// ── Public API ────────────────────────────────────────────────────────────────

pub fn get_major(version: &str) -> Option<i64> {
    Some(parse(version)?.release.major() as i64)
}

pub fn get_minor(version: &str) -> Option<i64> {
    Some(parse(version)?.release.minor() as i64)
}

pub fn get_patch(version: &str) -> Option<i64> {
    Some(parse(version)?.release.patch() as i64)
}

pub fn equals(a: &str, b: &str) -> bool {
    match (parse(a), parse(b)) {
        (Some(av), Some(bv)) => av.equals(&bv, true),
        _ => false,
    }
}

pub fn is_greater_than(a: &str, b: &str) -> bool {
    match (parse(a), parse(b)) {
        (Some(av), Some(bv)) => av.is_greater_than(&bv),
        _ => false,
    }
}

pub fn is_less_than_range(version: &str, range: &str) -> bool {
    match (parse(version), parse(range)) {
        (Some(av), Some(bv)) => av.is_less_than(&bv),
        _ => false,
    }
}

/// Find the highest version in `versions` that equals `range` (exact match).
pub fn get_satisfying_version(versions: &[&str], range: &str) -> Option<String> {
    let target = parse(range)?;
    versions
        .iter()
        .find(|&&v| parse(v).is_some_and(|bv| target.equals(&bv, true)))
        .map(|_| range.to_owned())
}

pub fn min_satisfying_version(versions: &[&str], range: &str) -> Option<String> {
    get_satisfying_version(versions, range)
}

pub fn sort_versions(a: &str, b: &str) -> i32 {
    match (parse(a), parse(b)) {
        (Some(av), Some(bv)) => BzlmodVersion::default_compare(&av, &bv),
        _ => 0,
    }
}

pub fn is_stable(version: &str) -> bool {
    parse(version).is_some_and(|v| !v.is_prerelease())
}

pub fn is_valid(input: &str) -> bool {
    parse(input).is_some()
}

/// Same as `is_valid` — no distinction between versions and ranges in Bzlmod.
pub fn is_version(input: &str) -> bool {
    is_valid(input)
}

pub fn matches_range(version: &str, range: &str) -> bool {
    equals(version, range)
}

pub fn get_new_value(
    current_value: &str,
    current_version: Option<&str>,
    new_version: &str,
) -> String {
    // If currentVersion == v{currentValue}, strip the leading v from newVersion.
    if current_version == Some(&format!("v{current_value}")) {
        return new_version.trim_start_matches('v').to_owned();
    }
    new_version.to_owned()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "getMajor()" — lib/modules/versioning/bazel-module/index.spec.ts line 5
    #[test]
    fn bzlmod_get_major() {
        assert_eq!(get_major("1.2.3"), Some(1));
    }

    // Ported: "getMinor()" — lib/modules/versioning/bazel-module/index.spec.ts line 9
    #[test]
    fn bzlmod_get_minor() {
        assert_eq!(get_minor("1.2.3"), Some(2));
    }

    // Ported: "getPatch()" — lib/modules/versioning/bazel-module/index.spec.ts line 13
    #[test]
    fn bzlmod_get_patch() {
        assert_eq!(get_patch("1.2.3"), Some(3));
    }

    // Ported: "equals($a, $b)" — lib/modules/versioning/bazel-module/index.spec.ts line 17
    #[test]
    fn bzlmod_equals() {
        assert!(equals("1.2.3", "1.2.3"));
        assert!(!equals("1.2.3", "1.2.4"));
        // matches is alias for equals
        assert!(matches_range("1.2.3", "1.2.3"));
        assert!(!matches_range("1.2.3", "1.2.4"));
    }

    // Ported: "isGreaterThan($a, $b)" — lib/modules/versioning/bazel-module/index.spec.ts line 27
    #[test]
    fn bzlmod_is_greater_than() {
        assert!(is_greater_than("1.2.4", "1.2.3"));
        assert!(!is_greater_than("1.2.3", "1.2.3"));
        assert!(!is_greater_than("1.2.2", "1.2.3"));
    }

    // Ported: "isLessThanRange($a, $b)" — lib/modules/versioning/bazel-module/index.spec.ts line 36
    #[test]
    fn bzlmod_is_less_than_range() {
        assert!(!is_less_than_range("1.2.4", "1.2.3"));
        assert!(!is_less_than_range("1.2.3", "1.2.3"));
        assert!(is_less_than_range("1.2.2", "1.2.3"));
    }

    // Ported: "getSatisfyingVersion(vers, rng)" — lib/modules/versioning/bazel-module/index.spec.ts line 45
    #[test]
    fn bzlmod_get_satisfying_version() {
        assert_eq!(get_satisfying_version(&[], "1.2.3"), None);
        assert_eq!(
            get_satisfying_version(&["1.1.0", "1.2.0", "2.0.0"], "1.2.0"),
            Some("1.2.0".to_owned())
        );
        assert_eq!(
            get_satisfying_version(&["1.1.0", "1.2.0", "2.0.0"], "1.2.3"),
            None
        );
        // minSatisfyingVersion is alias
        assert_eq!(
            min_satisfying_version(&["1.1.0", "1.2.0", "2.0.0"], "1.2.0"),
            Some("1.2.0".to_owned())
        );
    }

    // Ported: "sortVersions($a, $b)" — lib/modules/versioning/bazel-module/index.spec.ts line 56
    #[test]
    fn bzlmod_sort_versions() {
        assert_eq!(sort_versions("1.2.3", "1.2.3"), 0);
        assert_eq!(sort_versions("1.2.3", "1.2.4"), -1);
        assert_eq!(sort_versions("1.2.4", "1.2.3"), 1);
    }

    // Ported: "isStable" — lib/modules/versioning/bazel-module/index.spec.ts line 65
    #[test]
    fn bzlmod_is_stable() {
        assert!(is_stable("1.2.3"));
        assert!(!is_stable("1.2.3-pre"));
        assert!(is_stable("1.2.3+build"));
    }

    // Ported: "isValid($a)" — lib/modules/versioning/bazel-module/index.spec.ts line 74
    #[test]
    fn bzlmod_is_valid() {
        assert!(is_valid("1.2.3"));
        assert!(is_valid("1.2.3-pre"));
        assert!(is_valid("1.2.3+build"));
        assert!(is_valid("1.2.3-pre+build"));
        assert!(!is_valid("-abc"));
        assert!(!is_valid("1_2"));
        // isCompatible and isSingleVersion are aliases
        assert!(is_version("1.2.3"));
        assert!(!is_version("-abc"));
    }

    // Ported: "isVersion($a)" — lib/modules/versioning/bazel-module/index.spec.ts line 90
    #[test]
    fn bzlmod_is_version_null() {
        // null/undefined handled by Option in Rust
        assert!(is_version("1.2.3"));
        assert!(!is_version("-abc"));
    }

    // Ported: "getNewValue()" — lib/modules/versioning/bazel-module/index.spec.ts line 100
    #[test]
    fn bzlmod_get_new_value() {
        assert_eq!(get_new_value("1.0.0", None, "1.0.1"), "1.0.1");
        // currentVersion == v{currentValue} → strip leading v
        assert_eq!(get_new_value("1.0.0", Some("v1.0.0"), "v1.0.1"), "1.0.1");
    }
}
