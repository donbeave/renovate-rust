//! Hermit versioning.
//!
//! Ports `lib/modules/versioning/hermit/index.ts`.
//!
//! Hermit supports both regular versions (e.g. `17.0.1_12`) and channel
//! references prefixed with `@` (e.g. `@1.2`, `@stable`, `@latest`).
//! Channel versions represent a range of package versions; a shorter channel
//! release array is considered "greater" than a more-specific one.

use std::cmp::Ordering;
use std::sync::LazyLock;

use regex::Regex;
use semver::{Version as SemVer, VersionReq};

// Regex matching hermit version strings (without the @ channel prefix).
// Captures: major, minor, patch, supplement, build, prerelease, compatibility.
static HERMIT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?x)^
        (?P<major>\d+)
        (?:\.(?P<minor>\d+))?
        (?:\.(?P<patch>\d+))?
        (?:\.(?P<supplement>\d+))?
        (?:_(?P<build>\d+))?
        (?:[-]?(?P<prerelease>[^.+][^+]*))?
        (?:[+](?P<compatibility>[^.-][^+]*))?
        $",
    )
    .unwrap()
});

#[derive(Debug, Clone)]
struct ParsedVersion {
    release: Vec<u64>,
    prerelease: Option<String>,
}

fn is_channel(version: &str) -> bool {
    version.starts_with('@')
}

fn get_channel_str(version: &str) -> &str {
    &version[1..]
}

fn parse_channel(version: &str) -> Option<ParsedVersion> {
    let inner = get_channel_str(version);
    let caps = HERMIT_RE.captures(inner)?;

    let mut release = Vec::new();
    if let Some(m) = caps.name("major") {
        release.push(m.as_str().parse::<u64>().ok()?);
    }
    if let Some(m) = caps.name("minor") {
        release.push(m.as_str().parse::<u64>().ok()?);
    }
    if let Some(m) = caps.name("patch") {
        release.push(m.as_str().parse::<u64>().ok()?);
    }
    if let Some(m) = caps.name("supplement") {
        release.push(m.as_str().parse::<u64>().ok()?);
    }
    if let Some(m) = caps.name("build") {
        release.push(m.as_str().parse::<u64>().ok()?);
    }

    Some(ParsedVersion {
        release,
        prerelease: caps.name("prerelease").map(|m| m.as_str().to_owned()),
    })
}

// Non-channel: always 4 elements [major, minor, patch, supplement] plus optional build.
fn parse_version(version: &str) -> Option<ParsedVersion> {
    let caps = HERMIT_RE.captures(version)?;

    let major: u64 = caps.name("major")?.as_str().parse().ok()?;
    let minor: u64 = caps
        .name("minor")
        .and_then(|m| m.as_str().parse().ok())
        .unwrap_or(0);
    let patch: u64 = caps
        .name("patch")
        .and_then(|m| m.as_str().parse().ok())
        .unwrap_or(0);
    let supplement: u64 = caps
        .name("supplement")
        .and_then(|m| m.as_str().parse().ok())
        .unwrap_or(0);

    let mut release = vec![major, minor, patch, supplement];
    if let Some(m) = caps.name("build")
        && let Ok(b) = m.as_str().parse::<u64>()
    {
        release.push(b);
    }

    Some(ParsedVersion {
        release,
        prerelease: caps.name("prerelease").map(|m| m.as_str().to_owned()),
    })
}

fn is_valid_version(version: &str) -> bool {
    !is_channel(version) && parse_version(version).is_some()
}

pub fn is_valid(version: &str) -> bool {
    is_valid_version(version) || is_channel(version)
}

pub fn is_stable(version: &str) -> bool {
    if is_valid_version(version) {
        parse_version(version)
            .map(|p| p.prerelease.is_none())
            .unwrap_or(false)
    } else {
        false
    }
}

fn parse(version: &str) -> Option<ParsedVersion> {
    if is_channel(version) {
        parse_channel(version)
    } else {
        parse_version(version)
    }
}

// Compare two release arrays using hermit channel semantics:
// - if version's slot i is None (version is shorter) → version > other (return 1)
// - if other's slot i is None (other is shorter) → version < other (return -1)
fn compare_release_arrays(a: &[u64], b: &[u64]) -> i32 {
    let max_len = a.len().max(b.len());
    for i in 0..max_len {
        match (a.get(i), b.get(i)) {
            (Some(&av), Some(&bv)) if av != bv => {
                return if av > bv { 1 } else { -1 };
            }
            (None, _) => return 1,
            (_, None) => return -1,
            _ => {}
        }
    }
    0
}

pub fn compare(a: &str, b: &str) -> i32 {
    if is_valid_version(a) && is_valid_version(b) {
        let pa = parse_version(a).unwrap();
        let pb = parse_version(b).unwrap();
        return compare_release_arrays(&pa.release, &pb.release);
    }

    let pa = parse(a);
    let pb = parse(b);

    match (pa, pb) {
        (None, None) => match a.cmp(b) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        },
        (None, Some(_)) => -1,
        (Some(_), None) => 1,
        (Some(pa), Some(pb)) => compare_release_arrays(&pa.release, &pb.release),
    }
}

pub fn equals(a: &str, b: &str) -> bool {
    a == b
}

pub fn is_greater_than(a: &str, b: &str) -> bool {
    compare(a, b) > 0
}

pub fn is_less_than_range(version: &str, range: &str) -> bool {
    compare(version, range) < 0
}

pub fn sort_versions(a: &str, b: &str) -> i32 {
    compare(a, b)
}

pub fn get_major(version: &str) -> Option<u64> {
    parse(version).and_then(|p| p.release.first().copied())
}

pub fn get_minor(version: &str) -> Option<u64> {
    parse(version).and_then(|p| p.release.get(1).copied())
}

pub fn get_patch(version: &str) -> Option<u64> {
    parse(version).and_then(|p| p.release.get(2).copied())
}

fn semver_matches(version: &str, range: &str) -> bool {
    let Ok(ver) = SemVer::parse(version) else {
        return false;
    };

    // Hyphen range: "0.6.0 - 0.6.3" → >=lower <=upper
    if let Some(idx) = range.find(" - ") {
        let lower = &range[..idx];
        let upper = &range[idx + 3..];
        let (Ok(lo), Ok(hi)) = (
            VersionReq::parse(&format!(">={lower}")),
            VersionReq::parse(&format!("<={upper}")),
        ) else {
            return false;
        };
        return lo.matches(&ver) && hi.matches(&ver);
    }

    // Space-separated AND: ">0.6.0 <0.7.0" — each part must match separately
    if range.contains(' ') {
        return range
            .split_whitespace()
            .all(|part| VersionReq::parse(part).is_ok_and(|req| req.matches(&ver)));
    }

    VersionReq::parse(range).is_ok_and(|req| req.matches(&ver))
}

pub fn matches(version: &str, range: &str) -> bool {
    if is_channel(version) || is_channel(range) {
        return equals(version, range);
    }
    semver_matches(version, range)
}

pub fn get_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    versions
        .iter()
        .filter(|&&v| matches(v, range))
        .max_by(|&&a, &&b| compare(a, b).cmp(&0))
        .copied()
}

pub fn min_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    versions
        .iter()
        .filter(|&&v| matches(v, range))
        .min_by(|&&a, &&b| compare(a, b).cmp(&0))
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isStable("$version") === $expected" — lib/modules/versioning/hermit/index.spec.ts line 6
    #[test]
    fn is_stable_matches_renovate_hermit_index_spec() {
        let cases = [
            ("1", true),
            ("1.2", true),
            ("@1", false),
            ("@1.2", false),
            ("@1.2.3", false),
            ("@latest", false),
            ("@stable", false),
        ];
        for (version, expected) in cases {
            assert_eq!(is_stable(version), expected, "is_stable({version:?})");
        }
    }

    // Ported: "isValid("$version") === $expected" — lib/modules/versioning/hermit/index.spec.ts line 19
    #[test]
    fn is_valid_matches_renovate_hermit_index_spec() {
        let cases = [
            ("1", true),
            ("1rc1", true),
            ("1-foo", true),
            ("1+bar", true),
            ("1.2", true),
            ("1.2-foo", true),
            ("1.2+bar", true),
            ("1.2.3", true),
            ("1.2.3rc1", true),
            ("1.2.3-foo", true),
            ("1.2.3+bar", true),
            ("17.0.1_12", true),
            ("17.0.1_12+m1", true),
            ("11.0.11_9-zulu11.48.21", true),
            ("1.2-kotlin.3", true),
            ("@1", true),
            ("@1.2", true),
            ("@1.2.3", true),
            ("@latest", true),
            ("@stable", true),
        ];
        for (version, expected) in cases {
            assert_eq!(is_valid(version), expected, "is_valid({version:?})");
        }
    }

    // Ported: "getMajor, getMinor, getPatch for "$version"" — lib/modules/versioning/hermit/index.spec.ts line 46
    #[test]
    #[allow(clippy::type_complexity)]
    fn get_major_minor_patch_matches_renovate_hermit_index_spec() {
        let cases: &[(&str, Option<u64>, Option<u64>, Option<u64>)] = &[
            ("17", Some(17), Some(0), Some(0)),
            ("17.2", Some(17), Some(2), Some(0)),
            ("17.2.3a1", Some(17), Some(2), Some(3)),
            ("17.2.3-foo", Some(17), Some(2), Some(3)),
            ("17.2.3+m1", Some(17), Some(2), Some(3)),
            ("@17", Some(17), None, None),
            ("@17.2", Some(17), Some(2), None),
            ("@stable", None, None, None),
        ];
        for &(version, major, minor, patch) in cases {
            assert_eq!(get_major(version), major, "get_major({version:?})");
            assert_eq!(get_minor(version), minor, "get_minor({version:?})");
            assert_eq!(get_patch(version), patch, "get_patch({version:?})");
        }
    }

    // Ported: "equals("$version", "$other") === $expected" — lib/modules/versioning/hermit/index.spec.ts line 65
    #[test]
    fn equals_matches_renovate_hermit_index_spec() {
        let cases = [
            ("1", "1.2", false),
            ("@1", "@1.2", false),
            ("@1.2", "@1.2", true),
            ("@1.2", "@1.3", false),
            ("@1.2.3", "@1.2", false),
            ("@1.2.3_4", "@1.2.3", false),
            ("@latest", "@1", false),
            ("@stable", "@stable", true),
            ("stable", "stable", true),
        ];
        for (version, other, expected) in cases {
            assert_eq!(
                equals(version, other),
                expected,
                "equals({version:?}, {other:?})"
            );
        }
    }

    // Ported: "matches("$version", "$range") === $expected" — lib/modules/versioning/hermit/index.spec.ts line 83
    #[test]
    fn matches_matches_renovate_hermit_index_spec() {
        let cases = [
            ("0.6.1", ">0.6.0 <0.7.0", true),
            ("0.6.1", "<0.7.0", true),
            ("0.6.1", "<=0.7.0", true),
            ("0.6.1", ">=0.6.0", true),
            ("0.6.1", ">0.6.0", true),
            ("0.6.1", "0.6.x", true),
            ("0.6.1", "0.6.*", true),
            ("0.6.1", "0.6.0 - 0.6.3", true),
            ("0.6.1", "~0.6", true),
            ("0.6.1", "0.6.1", true),
            ("0.0.6", "^0.0.6", true),
            ("0.0.6", "@0.0.6", false),
            ("@0.0.6", "0.0.6", false),
            ("@1", "@1.2", false),
            ("@1.2", "@1.2", true),
            ("@1.2.3", "@1.2", false),
            ("@latest", "@1", false),
            ("@stable", "@stable", true),
        ];
        for (version, range, expected) in cases {
            assert_eq!(
                matches(version, range),
                expected,
                "matches({version:?}, {range:?})"
            );
        }
    }

    // Ported: "isGreaterThan("$version", "$other") === $expected" — lib/modules/versioning/hermit/index.spec.ts line 110
    #[test]
    fn is_greater_than_matches_renovate_hermit_index_spec() {
        let cases = [
            ("@1", "@1.2", true),
            ("@1.2", "@1.2", false),
            ("@1.2", "@1.3", false),
            ("@1.2.3", "@1.2", false),
            ("@11.0.10_9", "@11.0.10.1_1", true),
            ("@11.0.10_9", "@11.0.14.1_1", false),
            ("@11.0.10_9", "@11.0.14_1", false),
            ("@11.0.10.1_9", "@11.0.10.2_8", false),
            ("@11.0.10.2_9", "@11.0.14_1", false),
            ("@11.0.10.2_9", "@11.0.14.1_1", false),
            ("1.2.3", "@latest", true),
            ("@latest", "@1", false),
            ("@stable", "@latest", true),
            ("@latest", "@stable", false),
            ("11.0.10_9", "11.0.10.2_1", false),
            ("11.0.10_9", "11.0.14.1_1", false),
            ("11.0.10_9", "11.0.14_1", false),
            ("11.0.10.1_9", "11.0.10.2_8", false),
            ("11.0.10.2_9", "11.0.14_1", false),
            ("11.0.10.2_9", "11.0.14.1_1", false),
        ];
        for (version, other, expected) in cases {
            assert_eq!(
                is_greater_than(version, other),
                expected,
                "is_greater_than({version:?}, {other:?})"
            );
        }
    }

    // Ported: "isLessThanRange("$version", "$other") === $expected" — lib/modules/versioning/hermit/index.spec.ts line 139
    #[test]
    fn is_less_than_range_matches_renovate_hermit_index_spec() {
        let cases = [
            ("@1", "@1.2", false),
            ("@1.2", "@1.2", false),
            ("@1.2.3", "@1.2", true),
            ("@11.0.10_9", "@11.0.10.1_1", false),
            ("@11.0.10_9", "@11.0.14.1_1", true),
            ("@11.0.10_9", "@11.0.14_1", true),
            ("@11.0.10.1_9", "@11.0.10.2_8", true),
            ("@11.0.10.1_9", "@11.0.14_1", true),
            ("@11.0.10.1_9", "@11.0.14.1_1", true),
            ("@latest", "@1", true),
            ("@stable", "@latest", false),
            ("@latest", "@stable", true),
            ("11.0.10_9", "11.0.10.2_1", true),
            ("11.0.10_9", "11.0.14.1_1", true),
            ("11.0.10_9", "11.0.14_1", true),
            ("11.0.10.1_9", "11.0.10.2_8", true),
            ("11.0.10.2_9", "11.0.14_1", true),
            ("11.0.10.2_9", "11.0.14.1_1", true),
        ];
        for (version, range, expected) in cases {
            assert_eq!(
                is_less_than_range(version, range),
                expected,
                "is_less_than_range({version:?}, {range:?})"
            );
        }
    }

    // Ported: "getSatisfyingVersion" — lib/modules/versioning/hermit/index.spec.ts line 166
    #[test]
    fn get_satisfying_version_matches_renovate_hermit_index_spec() {
        assert_eq!(
            get_satisfying_version(&["@1.1.1", "1.2.3"], "1.2.3"),
            Some("1.2.3")
        );
        assert_eq!(
            get_satisfying_version(&["1.1.1", "@2.2.1", "2.2.2", "3.3.3"], "2.2.2"),
            Some("2.2.2")
        );
        assert_eq!(
            get_satisfying_version(&["1.1.1", "@1.3.3", "2.2.2", "3.3.3"], "1.2.3"),
            None
        );
    }

    // Ported: "minSatisfyingVersion" — lib/modules/versioning/hermit/index.spec.ts line 184
    #[test]
    fn min_satisfying_version_matches_renovate_hermit_index_spec() {
        assert_eq!(
            min_satisfying_version(&["@1.1.1", "1.2.3"], "1.2.3"),
            Some("1.2.3")
        );
        assert_eq!(
            min_satisfying_version(&["1.1.1", "@1.2.3", "2.2.2", "3.3.3"], "2.2.2"),
            Some("2.2.2")
        );
        assert_eq!(
            min_satisfying_version(&["1.1.1", "@1.2.2", "2.2.2", "3.3.3"], "1.2.3"),
            None
        );
    }

    // Ported: "sorts versions in an ascending order" — lib/modules/versioning/hermit/index.spec.ts line 203
    #[test]
    fn sort_versions_ascending_matches_renovate_hermit_index_spec() {
        let mut versions = vec![
            "@1", "1.1", "1.2", "1.2.3", "1.3", "@1.2", "@2", "2", "2.1", "@stable", "@latest",
        ];
        versions.sort_by(|a, b| {
            let cmp = sort_versions(a, b);
            if cmp < 0 {
                Ordering::Less
            } else if cmp > 0 {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        assert_eq!(
            versions,
            vec![
                "@latest", "@stable", "1.1", "1.2", "1.2.3", "@1.2", "1.3", "@1", "2", "2.1", "@2"
            ]
        );
    }

    #[test]
    fn compare_direct() {
        assert_eq!(compare("1.2.3", "1.2.3"), 0);
        assert!(compare("1.2.3", "1.2.2") > 0);
        assert!(compare("1.2.2", "1.2.3") < 0);
    }
}
