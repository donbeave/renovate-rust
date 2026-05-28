//! Docker versioning.
//!
//! Ports `lib/modules/versioning/docker/index.ts`.
//! Version format: `numeric-parts[prerelease][-suffix]` (e.g. `3.8.0b1-alpine`).
//! Shorter release array is considered greater: `3.7 > 3.7.0` (broader tag covers range).

use std::sync::LazyLock;

use regex::Regex;

static COMMIT_HASH_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-f0-9]{7,40}$").unwrap());

static ALL_NUMERIC_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+$").unwrap());

static VERSION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?P<version>\d+(?:\.\d+)*)(?P<prerelease>\w*)$").unwrap());

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedVersion {
    release: Vec<u64>,
    prerelease: String,
    suffix: String,
}

fn parse(version: &str) -> Option<ParsedVersion> {
    if version.is_empty() {
        return None;
    }
    // Commit hash: 7-40 hex chars that are NOT purely numeric
    if COMMIT_HASH_RE.is_match(version) && !ALL_NUMERIC_RE.is_match(version) {
        return None;
    }
    let stripped = version.strip_prefix('v').unwrap_or(version);
    let mut pieces = stripped.splitn(2, '-');
    let prefix = pieces.next().unwrap_or("");
    let suffix = pieces.next().unwrap_or("").to_owned();

    let caps = VERSION_RE.captures(prefix)?;
    let ver_str = &caps["version"];
    let prerelease = caps["prerelease"].to_owned();
    let release: Vec<u64> = ver_str
        .split('.')
        .map(|p| p.parse::<u64>().unwrap_or(0))
        .collect();
    Some(ParsedVersion {
        release,
        prerelease,
        suffix,
    })
}

pub fn is_valid(version: &str) -> bool {
    parse(version).is_some()
}

pub fn is_stable(version: &str) -> bool {
    parse(version).is_some_and(|p| p.prerelease.is_empty())
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

pub fn compare(a: &str, b: &str) -> i32 {
    let (Some(pa), Some(pb)) = (parse(a), parse(b)) else {
        return 1;
    };
    let max_len = pa.release.len().max(pb.release.len());
    for i in 0..max_len {
        let av = pa.release.get(i);
        let bv = pb.release.get(i);
        match (av, bv) {
            (Some(&av), Some(&bv)) if av != bv => {
                return if av > bv { 1 } else { -1 };
            }
            (None, _) => return 1,
            (_, None) => return -1,
            _ => {}
        }
    }
    // Release arrays equal; compare prerelease
    if pa.prerelease != pb.prerelease {
        if pa.prerelease.is_empty() && !pb.prerelease.is_empty() {
            return 1;
        }
        if !pa.prerelease.is_empty() && pb.prerelease.is_empty() {
            return -1;
        }
        return match pa.prerelease.cmp(&pb.prerelease) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Equal => 0,
        };
    }
    // Suffix comparison: suffix_of_b.localeCompare(suffix_of_a)
    match pb.suffix.as_str().cmp(pa.suffix.as_str()) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Equal => 0,
    }
}

pub fn equals(a: &str, b: &str) -> bool {
    compare(a, b) == 0
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

pub fn is_compatible(version: &str, current: &str) -> bool {
    let (Some(pv), Some(pc)) = (parse(version), parse(current)) else {
        return false;
    };
    pv.suffix == pc.suffix && pv.release.len() == pc.release.len()
}

pub fn value_to_version(value: &str) -> &str {
    value.split('-').next().unwrap_or(value)
}

pub fn get_new_value<'a>(
    _current_value: Option<&str>,
    _range_strategy: Option<&str>,
    new_version: &'a str,
) -> &'a str {
    new_version
}

pub fn get_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    versions.iter().find(|&&v| equals(v, range)).copied()
}

pub fn min_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    versions.iter().find(|&&v| equals(v, range)).copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    // Ported: "isValid("$version") === $expected" — versioning/docker/index.spec.ts line 5
    #[test]
    fn is_valid_matches_renovate_docker_index_spec() {
        let cases = [
            ("", false),
            ("1.2.3", true),
            ("18.04", true),
            ("10.1", true),
            ("3", true),
            ("foo", false),
            ("0a1b2c3", false),
            ("0a1b2c3d", false),
            ("0a1b2c3d4e5f6a7b8c9d0a1b2c3d4e5f6a7b8c9d", false),
            ("0a1b2c3d4e5f6a7b8c9d0a1b2c3d4e5f6a7b8c9d0", true),
            ("0a1b2C3", true),
            ("0z1b2c3", true),
            ("0A1b2c3d4e5f6a7b8c9d0a1b2c3d4e5f6a7b8c9d", true),
            ("123098140293", true),
            ("01aecc#v2.1.0", false),
        ];
        for (version, expected) in cases {
            assert_eq!(is_valid(version), expected, "is_valid({version:?})");
        }
    }

    // Ported: "getMajor, getMinor, getPatch for "$version"" — versioning/docker/index.spec.ts line 27
    #[test]
    #[allow(clippy::type_complexity)]
    fn get_major_minor_patch_matches_renovate_docker_index_spec() {
        let cases: &[(&str, Option<u64>, Option<u64>, Option<u64>)] = &[
            ("1.2.3", Some(1), Some(2), Some(3)),
            ("18.04", Some(18), Some(4), None),
            ("10.1", Some(10), Some(1), None),
            ("3", Some(3), None, None),
            ("foo", None, None, None),
        ];
        for &(version, major, minor, patch) in cases {
            assert_eq!(get_major(version), major, "get_major({version:?})");
            assert_eq!(get_minor(version), minor, "get_minor({version:?})");
            assert_eq!(get_patch(version), patch, "get_patch({version:?})");
        }
    }

    // Ported: "isGreaterThan($a, $b) === $expected" — versioning/docker/index.spec.ts line 43
    #[test]
    fn is_greater_than_matches_renovate_docker_index_spec() {
        let cases = [
            ("1.2.3", "1.2", false),
            ("18.04", "18.1", true),
            ("10.1", "10.1.2", true),
            ("3", "2", true),
            ("1.2.3", "1.2.3", false),
        ];
        for (a, b, expected) in cases {
            assert_eq!(
                is_greater_than(a, b),
                expected,
                "is_greater_than({a:?}, {b:?})"
            );
        }
    }

    // Ported: "isLessThanRange($version, $range) === $expected" — versioning/docker/index.spec.ts line 54
    #[test]
    fn is_less_than_range_matches_renovate_docker_index_spec() {
        let cases = [
            ("1.2.3", "2.0", true),
            ("18.04", "18.1", false),
            ("10.1", "10.0.4", false),
            ("3", "4.0", true),
            ("1.2", "1.3.4", true),
        ];
        for (version, range, expected) in cases {
            assert_eq!(
                is_less_than_range(version, range),
                expected,
                "is_less_than_range({version:?}, {range:?})"
            );
        }
    }

    // Ported: "equals($a, $b) === $expected" — versioning/docker/index.spec.ts line 68
    #[test]
    fn equals_matches_renovate_docker_index_spec() {
        let cases = [
            ("1.2.3", "1.2.3", true),
            ("18.04", "18.4", true),
            ("10.0", "10.0.4", false),
            ("3", "4.0", false),
            ("1.2", "1.2.3", false),
        ];
        for (a, b, expected) in cases {
            assert_eq!(equals(a, b), expected, "equals({a:?}, {b:?})");
        }
    }

    // Ported: "satisfying for $version -> $expected" — versioning/docker/index.spec.ts line 92
    #[test]
    fn satisfying_matches_renovate_docker_index_spec() {
        let versions = &[
            "0.9.8", "1.1.1", "1.1", "1.2.3", "1.2", "1", "2.2.2", "2.2", "2",
        ];
        let cases = [
            ("1.2.3", Some("1.2.3")),
            ("1.2", Some("1.2")),
            ("1", Some("1")),
            ("1.3", None),
            ("0.9", None),
        ];
        for (version, expected) in cases {
            assert_eq!(
                get_satisfying_version(versions, version),
                expected,
                "get_satisfying({version:?})"
            );
            assert_eq!(
                min_satisfying_version(versions, version),
                expected,
                "min_satisfying({version:?})"
            );
        }
    }

    // Ported: "docker.sortVersions("$a", "$b") === semver.sortVersions("$a", "$b")" — versioning/docker/index.spec.ts line 108
    #[test]
    fn sort_versions_semver_matches_renovate_docker_index_spec() {
        let cases = [
            ("1.1.1", "1.2.3"),
            ("1.2.3", "1.3.4"),
            ("2.0.1", "1.2.3"),
            ("1.2.3", "0.9.5"),
        ];
        for (a, b) in cases {
            let docker_cmp = sort_versions(a, b);
            // For standard 3-part numeric versions, compare == simple numeric compare
            let pv_a: Vec<u64> = a.split('.').map(|p| p.parse().unwrap()).collect();
            let pv_b: Vec<u64> = b.split('.').map(|p| p.parse().unwrap()).collect();
            let semver_cmp = pv_a.cmp(&pv_b);
            let expected = match semver_cmp {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            };
            assert_eq!(docker_cmp, expected, "sort_versions({a:?}, {b:?})");
        }
    }

    // Ported: "sorts unstable" — versioning/docker/index.spec.ts line 123
    #[test]
    fn sort_unstable_matches_renovate_docker_index_spec() {
        let mut versions = vec![
            "3.7.0",
            "3.7-alpine",
            "3.7.0b1",
            "3.7.0b5",
            "3.8.0b1-alpine",
            "3.8.0-alpine",
            "3.8.2",
            "3.8.0",
        ];
        versions.sort_by(|a, b| match sort_versions(a, b) {
            n if n < 0 => Ordering::Less,
            n if n > 0 => Ordering::Greater,
            _ => Ordering::Equal,
        });
        assert_eq!(
            versions,
            vec![
                "3.7.0b1",
                "3.7.0b5",
                "3.7.0",
                "3.7-alpine",
                "3.8.0b1-alpine",
                "3.8.0-alpine",
                "3.8.0",
                "3.8.2",
            ]
        );
    }

    // Ported: "getNewValue($currentValue, $rangeStrategy, $currentVersion, $newVersion, $expected) === $expected" — versioning/docker/index.spec.ts line 148
    #[test]
    fn get_new_value_matches_renovate_docker_index_spec() {
        assert_eq!(get_new_value(None, None, "1.2.3"), "1.2.3");
    }

    // Ported: "isStable("$version") === $expected" — versioning/docker/index.spec.ts line 164
    #[test]
    fn is_stable_matches_renovate_docker_index_spec() {
        let cases = [
            ("3.7.0", true),
            ("3.7.0b1", false),
            ("3.7-alpine", true),
            ("3.8.0-alpine", true),
            ("3.8.0b1-alpine", false),
            ("3.8.2", true),
        ];
        for (version, expected) in cases {
            assert_eq!(is_stable(version), expected, "is_stable({version:?})");
        }
    }

    // Ported: "isCompatible("$version") === $expected" — versioning/docker/index.spec.ts line 177
    #[test]
    fn is_compatible_matches_renovate_docker_index_spec() {
        let cases = [
            ("3.7.0", "3.7.0", true),
            ("3.7.0b1", "3.7.0", true),
            ("3.7-alpine", "3.7.0", false),
            ("3.8.0-alpine", "3.7.0", false),
            ("3.8.0b1-alpine", "3.7.0", false),
            ("3.8.2", "3.7.0", true),
            ("3.7.0", "3.7.0-alpine", false),
            ("3.7.0b1", "3.7.0-alpine", false),
            ("3.7-alpine", "3.7.0-alpine", false),
            ("3.8.0-alpine", "3.7.0-alpine", true),
            ("3.8.0b1-alpine", "3.7.0-alpine", true),
            ("3.8.2", "3.7.0-alpine", false),
        ];
        for (version, range, expected) in cases {
            assert_eq!(
                is_compatible(version, range),
                expected,
                "is_compatible({version:?}, {range:?})"
            );
        }
    }

    // Ported: "valueToVersion("$value") === $expected" — versioning/docker/index.spec.ts line 199
    #[test]
    fn value_to_version_matches_renovate_docker_index_spec() {
        let cases = [
            ("3.7.0", "3.7.0"),
            ("3.7.0b1", "3.7.0b1"),
            ("3.7-alpine", "3.7"),
            ("3.8.0-alpine", "3.8.0"),
            ("3.8.0b1-alpine", "3.8.0b1"),
            ("3.8.2", "3.8.2"),
        ];
        for (value, expected) in cases {
            assert_eq!(
                value_to_version(value),
                expected,
                "value_to_version({value:?})"
            );
        }
    }
}
