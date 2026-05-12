//! GitHub Actions versioning.
//!
//! Renovate reference:
//! - `lib/modules/versioning/github-actions/index.ts`

use std::cmp::Ordering;
use std::collections::HashSet;
use std::sync::LazyLock;

use regex::Regex;
use semver::Version;

static FLOATING_TAG_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+(\.\d+)?$").unwrap());
static MAJOR_ONLY_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+$").unwrap());
static MAJOR_MINOR_PRERELEASE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d+\.\d+)(-.+)$").unwrap());
static COERCE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d+)(?:\.(\d+))?(?:\.(\d+))?").unwrap());

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range {
    major: u64,
    minor: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeStrategy {
    Pin,
    Replace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MinLevel {
    Major,
    Minor,
}

#[derive(Debug)]
pub struct NewValueConfig<'a> {
    pub current_value: &'a str,
    pub current_version: &'a str,
    pub range_strategy: RangeStrategy,
    pub new_version: &'a str,
    pub all_versions: Option<HashSet<String>>,
}

fn massage_value(input: &str) -> &str {
    let trimmed = input.trim();
    trimmed
        .strip_prefix('v')
        .or_else(|| trimmed.strip_prefix('V'))
        .unwrap_or(trimmed)
}

fn parse_version(input: &str) -> Option<Version> {
    let stripped = massage_value(input);
    Version::parse(stripped).ok().or_else(|| {
        let normalized = MAJOR_MINOR_PRERELEASE_RE.replace(stripped, "$1.0$2");
        Version::parse(&normalized).ok()
    })
}

fn parse_range(input: &str) -> Option<Range> {
    let stripped = massage_value(input);
    if !FLOATING_TAG_RE.is_match(stripped) {
        return None;
    }

    let mut parts = stripped.split('.');
    let major = parts.next()?.parse().ok()?;
    if MAJOR_ONLY_RE.is_match(stripped) {
        return Some(Range { major, minor: None });
    }

    Some(Range {
        major,
        minor: Some(parts.next()?.parse().ok()?),
    })
}

fn parse_version_coerced(input: &str) -> Option<Version> {
    parse_version(input).or_else(|| {
        let stripped = massage_value(input);
        if !stripped.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            return None;
        }
        let captures = COERCE_RE.captures(stripped)?;
        let major = captures.get(1)?.as_str().parse().ok()?;
        let minor = captures
            .get(2)
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(0);
        let patch = captures
            .get(3)
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(0);
        Some(Version::new(major, minor, patch))
    })
}

pub fn is_valid(input: &str) -> bool {
    parse_version(input).is_some() || parse_range(input).is_some()
}

pub fn is_version(input: Option<&str>) -> bool {
    let Some(input) = input.filter(|value| !value.is_empty()) else {
        return false;
    };

    if parse_version(input).is_some() {
        return true;
    }

    let stripped = massage_value(input);
    stripped.chars().next().is_some_and(|c| c.is_ascii_digit()) && parse_range(input).is_some()
}

pub fn is_stable(version: &str) -> bool {
    parse_version_coerced(version).is_some_and(|version| version.pre.is_empty())
}

pub fn is_single_version(input: &str) -> bool {
    parse_version(input).is_some()
}

pub fn get_major(version: &str) -> Option<u64> {
    Some(parse_version_coerced(version)?.major)
}

pub fn get_minor(version: &str) -> Option<u64> {
    Some(parse_version_coerced(version)?.minor)
}

pub fn get_patch(version: &str) -> Option<u64> {
    Some(parse_version_coerced(version)?.patch)
}

pub fn sort_versions(x: &str, y: &str) -> Ordering {
    let Some(a) = parse_version_coerced(x) else {
        return Ordering::Equal;
    };
    let Some(b) = parse_version_coerced(y) else {
        return Ordering::Equal;
    };

    a.cmp(&b).then_with(|| x.cmp(y))
}

pub fn equals(x: &str, y: &str) -> bool {
    let Some(a) = parse_version_coerced(x) else {
        return false;
    };
    let Some(b) = parse_version_coerced(y) else {
        return false;
    };
    a == b
}

pub fn is_greater_than(x: &str, y: &str) -> bool {
    let Some(a) = parse_version_coerced(x) else {
        return false;
    };
    let Some(b) = parse_version_coerced(y) else {
        return false;
    };
    a > b
}

pub fn matches(version: &str, range: &str) -> bool {
    if parse_version_coerced(version).is_some() && massage_value(version) == massage_value(range) {
        return true;
    }

    let Some(version) = parse_version(version) else {
        return false;
    };

    if let Some(range_version) = parse_version(range) {
        return version == range_version;
    }

    let Some(range) = parse_range(range) else {
        return false;
    };

    if !version.pre.is_empty() || version.major != range.major {
        return false;
    }

    range.minor.is_none_or(|minor| version.minor == minor)
}

pub fn get_satisfying_version<'a>(versions: &'a [&'a str], range: &str) -> Option<&'a str> {
    let mut sorted = versions.to_vec();
    sorted.sort_by(|a, b| sort_versions(a, b));
    sorted.reverse();
    sorted.into_iter().find(|version| matches(version, range))
}

pub fn min_satisfying_version<'a>(versions: &'a [&'a str], range: &str) -> Option<&'a str> {
    let mut sorted = versions.to_vec();
    sorted.sort_by(|a, b| sort_versions(a, b));
    sorted.into_iter().find(|version| matches(version, range))
}

pub fn is_less_than_range(version: &str, range: &str) -> bool {
    let Some(version) = parse_version_coerced(version) else {
        return false;
    };
    let Some(range) = parse_range(range) else {
        return false;
    };

    if version.major != range.major {
        return version.major < range.major;
    }

    range.minor.is_some_and(|minor| version.minor < minor)
}

pub fn get_new_value(config: &NewValueConfig<'_>) -> Option<String> {
    if config.range_strategy == RangeStrategy::Pin {
        return Some(config.new_version.to_owned());
    }

    let Some(range) = parse_range(config.current_value) else {
        return Some(config.new_version.to_owned());
    };

    let min_level = if range.minor.is_none() {
        MinLevel::Major
    } else {
        MinLevel::Minor
    };
    let current_tag = massage_value(config.current_value);
    let prefix = config
        .current_value
        .split_once(current_tag)
        .map(|(prefix, _)| prefix)
        .unwrap_or_default();

    let Some(new_parsed) = parse_version(config.new_version) else {
        if let Some(new_coerced) = parse_version_coerced(config.new_version) {
            let empty = HashSet::new();
            let all_versions = config.all_versions.as_ref().unwrap_or(&empty);
            if let Some(shortest) =
                get_shortest_matching_version(prefix, &new_coerced, all_versions, min_level)
            {
                return Some(shortest);
            }
        }
        return Some(config.new_version.to_owned());
    };

    if parse_version(config.current_value).is_some() {
        return Some(config.new_version.to_owned());
    }

    let Some(all_versions) = config
        .all_versions
        .as_ref()
        .filter(|versions| !versions.is_empty())
    else {
        if range.minor.is_none() {
            return Some(format!("{prefix}{}", new_parsed.major));
        }

        return Some(format!("{prefix}{}.{}", new_parsed.major, new_parsed.minor));
    };

    if range.minor.is_none() && new_parsed.major == range.major {
        return Some(format!("{prefix}{}", new_parsed.major));
    }

    get_shortest_matching_version(prefix, &new_parsed, all_versions, min_level)
        .or_else(|| Some(config.new_version.to_owned()))
}

fn get_shortest_matching_version(
    prefix: &str,
    new_parsed: &Version,
    all_versions: &HashSet<String>,
    min_level: MinLevel,
) -> Option<String> {
    if min_level == MinLevel::Major {
        let version = format!("{prefix}{}", new_parsed.major);
        if all_versions.contains(&version) {
            return Some(version);
        }
    }

    let minor = format!("{prefix}{}.{}", new_parsed.major, new_parsed.minor);
    if all_versions.contains(&minor) {
        return Some(minor);
    }

    let patch = format!(
        "{prefix}{}.{}.{}",
        new_parsed.major, new_parsed.minor, new_parsed.patch
    );
    if all_versions.contains(&patch) {
        return Some(patch);
    }

    let full = format!("{prefix}{new_parsed}");
    if all_versions.contains(&full) {
        return Some(full);
    }

    None
}

pub fn is_compatible(version: &str) -> bool {
    is_valid(version)
}

pub fn is_breaking(version: &str, current: &str) -> bool {
    let Some(version) = parse_version(version) else {
        return false;
    };
    let Some(current) = parse_version(current) else {
        return false;
    };

    if current.major == 0 {
        return version.major > 0 || version.minor > current.minor;
    }

    version.major > current.major
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(values: &[&str]) -> HashSet<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }

    // Ported: "isValid(\"$version\") === $expected" — versioning/github-actions/index.spec.ts line 6
    #[test]
    fn is_valid_matches_renovate_github_actions_spec() {
        let cases = [
            ("1", true),
            ("1.2", true),
            ("1.2.3", true),
            ("~latest", false),
            ("1.2.3-alpha", true),
            ("v1", true),
            ("v1.2", true),
            ("v1.2.3", true),
            ("v1.2.3-alpha", true),
            ("v2.2-rc.1", true),
            ("invalid", false),
            ("", false),
            ("<6", false),
            (">=5", false),
            ("~4", false),
            ("^3", false),
        ];

        for (version, expected) in cases {
            assert_eq!(is_valid(version), expected, "is_valid({version})");
        }
    }

    // Ported: "isVersion(\"$version\") === $expected" — versioning/github-actions/index.spec.ts line 30
    #[test]
    fn is_version_matches_renovate_github_actions_spec() {
        let cases = [
            ("1", true),
            ("1.2", true),
            ("1.2.3", true),
            ("~latest", false),
            ("1.2.3-alpha", true),
            ("1.2.3-rc.1", true),
            ("v1", true),
            ("v1.2", true),
            ("v1.2.3", true),
            ("v1.2.3-alpha", true),
            ("v1.2.3-rc.1", true),
            ("v2.2-rc.1", true),
            ("invalid", false),
            ("", false),
            ("#1.0.0", false),
            ("x1.0.0", false),
        ];

        for (version, expected) in cases {
            assert_eq!(is_version(Some(version)), expected, "is_version({version})");
        }
        assert!(!is_version(None));
    }

    // Ported: "isStable(\"$version\") === $expected" — versioning/github-actions/index.spec.ts line 54
    #[test]
    fn is_stable_matches_renovate_github_actions_spec() {
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
            ("1.2", true),
            ("v1.2", true),
            ("1", true),
            ("v1", true),
            ("v2.2-rc.1", false),
            ("not-a-version", false),
        ];

        for (version, expected) in cases {
            assert_eq!(is_stable(version), expected, "is_stable({version})");
        }
    }

    // Ported: "isSingleVersion(\"$version\") === $expected" — versioning/github-actions/index.spec.ts line 81
    #[test]
    fn is_single_version_matches_renovate_github_actions_spec() {
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
            ("v2.2-rc.1", true),
        ];

        for (version, expected) in cases {
            assert_eq!(
                is_single_version(version),
                expected,
                "is_single_version({version})"
            );
        }
    }

    // Ported: "matches(\"$version\", \"$range\") === $expected" — versioning/github-actions/index.spec.ts line 99
    #[test]
    fn matches_floating_ranges_and_versions_like_renovate() {
        let cases = [
            ("1.1.0", "1.0", false),
            ("1.0.0", "1", true),
            ("1.2.0", "1", true),
            ("2.0.0", "1", false),
            ("1.1.5", "1.1", true),
            ("1.2.0", "1.1", false),
            ("1.0.0", "~latest", false),
            ("1.0.0-rc", "1", false),
            ("invalid", "1", false),
            ("1", "v1", true),
            ("v1.2", "1.2", true),
            ("1.2.4", "v1.2.3", false),
            ("v1.2.3", "1.2.3", true),
            ("1.2.3", "v1.2.3", true),
        ];

        for (version, range, expected) in cases {
            assert_eq!(
                matches(version, range),
                expected,
                "matches({version}, {range})"
            );
        }
    }

    // Ported: "should not handle invalid range that is not ~latest or valid version" — versioning/github-actions/index.spec.ts line 158
    #[test]
    fn matches_rejects_invalid_ranges() {
        assert!(!matches("1.0.0", "completely-invalid-range"));
    }

    // Ported: "getSatisfyingVersion($versions, \"$range\") === $expected" — versioning/github-actions/index.spec.ts line 166
    #[test]
    fn get_satisfying_version_matches_renovate_github_actions_spec() {
        let cases = [
            (
                ["1.0.0", "1.1.0", "1.1.1", "1.2.0", "2.0.0", "2.0.1"].as_slice(),
                "1",
                Some("1.2.0"),
            ),
            (
                ["1.0.0", "1.1.0", "1.1.1", "1.2.0", "2.0.0", "2.0.1"].as_slice(),
                "1.1",
                Some("1.1.1"),
            ),
            (
                ["1.0.0", "1.0.1-alpha", "1.0.2", "1.1.0-beta", "1.1.1"].as_slice(),
                "1",
                Some("1.1.1"),
            ),
            (
                ["v1.0.0", "v1.1.0", "v1.1.1", "v1.2.0", "v2.0.0"].as_slice(),
                "v1",
                Some("v1.2.0"),
            ),
            (["1.0", "1.1", "1.2"].as_slice(), "1", None),
            (["not-valid", "also-bad"].as_slice(), "1", None),
        ];

        for (versions, range, expected) in cases {
            assert_eq!(get_satisfying_version(versions, range), expected);
        }
    }

    // Ported: "minSatisfyingVersion($versions, \"$range\") === $expected" — versioning/github-actions/index.spec.ts line 202
    #[test]
    fn min_satisfying_version_matches_renovate_github_actions_spec() {
        let cases = [
            (
                ["1.0.0", "1.1.0", "1.1.1", "1.2.0", "2.0.0"].as_slice(),
                "1",
                Some("1.0.0"),
            ),
            (
                ["1.0.0", "1.1.0", "1.1.1", "1.2.0", "2.0.0"].as_slice(),
                "1.1",
                Some("1.1.0"),
            ),
            (
                ["v1.0.0", "v1.1.0", "1.2.0", "v2.0.0"].as_slice(),
                "v1",
                Some("v1.0.0"),
            ),
            (["v0.5.0", "v1.0.0", "v2.0.0"].as_slice(), "v3", None),
        ];

        for (versions, range, expected) in cases {
            assert_eq!(min_satisfying_version(versions, range), expected);
        }
    }

    // Ported: "isLessThanRange(\"$version\", \"$range\") === $expected" — versioning/github-actions/index.spec.ts line 226
    #[test]
    fn is_less_than_range_matches_renovate_github_actions_spec() {
        let cases = [
            ("0.9.0", "1", true),
            ("1.0.0", "1", false),
            ("1.0.0", "1.1", true),
            ("1.2.0", "1.1", false),
            ("invalid", "1", false),
            ("v1.2", "v1.3", true),
            ("v1.3", "v1.2", false),
            ("v1.2", "v1.2", false),
        ];

        for (version, range, expected) in cases {
            assert_eq!(is_less_than_range(version, range), expected);
        }
    }

    // Ported: "equals(\"$version\", \"$other\") === $expected" — versioning/github-actions/index.spec.ts line 260
    #[test]
    fn equals_matches_renovate_github_actions_spec() {
        let cases = [
            ("1.0.0", "1.0.0", true),
            ("1.0.0", "1.0.1", false),
            ("invalid", "invalid", false),
            ("v1.0.0", "1.0.0", true),
            ("v1.2", "1.2", true),
            ("v1", "v1", true),
            ("v6", "v5", false),
        ];

        for (version, other, expected) in cases {
            assert_eq!(equals(version, other), expected);
        }
    }

    // Ported: "getMajor(\"$version\") === $expected" — versioning/github-actions/index.spec.ts line 287
    // Ported: "getMinor(\"$version\") === $expected" — versioning/github-actions/index.spec.ts line 302
    // Ported: "getPatch(\"$version\") === $expected" — versioning/github-actions/index.spec.ts line 316
    #[test]
    fn component_getters_match_renovate_github_actions_spec() {
        assert_eq!(get_major("v2.3.4"), Some(2));
        assert_eq!(get_major("v1"), Some(1));
        assert_eq!(get_major("invalid"), None);
        assert_eq!(get_minor("v2.3.4"), Some(3));
        assert_eq!(get_minor("v1.2"), Some(2));
        assert_eq!(get_minor("invalid"), None);
        assert_eq!(get_patch("v2.3.4"), Some(4));
        assert_eq!(get_patch("v1.2"), Some(0));
        assert_eq!(get_patch("invalid"), None);
    }

    // Ported: "isGreaterThan(\"$version\", \"$other\") === $expected" — versioning/github-actions/index.spec.ts line 330
    #[test]
    fn is_greater_than_matches_renovate_github_actions_spec() {
        let cases = [
            ("1.0.1", "1.0.0", true),
            ("1.0.0", "1.0.1", false),
            ("v2.0.0", "1.0.0", true),
            ("v1.3", "v1.2", true),
            ("v1", "v1.2", false),
            ("v6", "v5", true),
            ("v6.0.1", "v6", true),
            ("invalid", "1.0.0", false),
        ];

        for (version, other, expected) in cases {
            assert_eq!(is_greater_than(version, other), expected);
        }
    }

    // Ported: "sortVersions(\"$a\", \"$b\") === $expected" — versioning/github-actions/index.spec.ts line 364
    #[test]
    fn sort_versions_matches_renovate_github_actions_spec() {
        let cases = [
            ("1.0.0", "1.0.0", Ordering::Equal),
            ("1.0.0", "1.0.1", Ordering::Less),
            ("1.0.1", "1.0.0", Ordering::Greater),
            ("invalid", "1.0.0", Ordering::Equal),
            ("v1.0.0", "1.0.0", Ordering::Greater),
            ("v1.3", "v1.3.0", Ordering::Less),
            ("v6", "v6.0.0", Ordering::Less),
        ];

        for (a, b, expected) in cases {
            assert_eq!(sort_versions(a, b), expected, "sort_versions({a}, {b})");
        }
    }

    // Ported: "isBreaking(\"$version\", \"$current\") === $expected" — versioning/github-actions/index.spec.ts line 394
    #[test]
    fn is_breaking_matches_renovate_github_actions_spec() {
        let cases = [
            ("2.0.0", "1.0.0", true),
            ("1.1.0", "1.0.0", false),
            ("0.2.0", "0.1.0", true),
            ("0.1.1", "0.1.0", false),
            ("invalid", "1.0.0", false),
            ("v2.0.0", "1.0.0", true),
            ("1.1.0", "v1.0.0", false),
        ];

        for (version, current, expected) in cases {
            assert_eq!(is_breaking(version, current), expected);
        }
    }

    // Ported: "isCompatible(\"$version\") === $expected" — versioning/github-actions/index.spec.ts line 422
    #[test]
    fn is_compatible_matches_renovate_github_actions_spec() {
        let cases = [
            ("1.0.0", true),
            ("1", true),
            ("~latest", false),
            ("v1.0.0", true),
            ("v1", true),
            ("invalid", false),
        ];

        for (version, expected) in cases {
            assert_eq!(is_compatible(version), expected);
        }
    }

    // Ported: "getNewValue(\"$currentValue\", \"$rangeStrategy\", \"$currentVersion\", \"$newVersion\") === \"$expected\"" — versioning/github-actions/index.spec.ts line 436
    #[test]
    fn get_new_value_matches_renovate_github_actions_spec() {
        let cases = [
            ("1", RangeStrategy::Pin, "1.0.0", "1.1.0", "1.1.0"),
            ("v1", RangeStrategy::Pin, "v1.0.0", "1.1.0", "1.1.0"),
            ("1", RangeStrategy::Replace, "1.0.0", "1.1.0", "1"),
            ("1", RangeStrategy::Replace, "1.0.0", "2.0.0", "2"),
            ("1.2", RangeStrategy::Replace, "1.2.0", "1.3.0", "1.3"),
            ("1.2.3", RangeStrategy::Replace, "1.2.3", "1.2.4", "1.2.4"),
            ("~latest", RangeStrategy::Replace, "1.0.0", "2.0.0", "2.0.0"),
            ("v1", RangeStrategy::Replace, "v1.0.0", "v2.0.0", "v2"),
            ("v1", RangeStrategy::Replace, "v1.0.0", "2.0.0", "v2"),
            ("1", RangeStrategy::Replace, "1.0.0", "v2.0.0", "2"),
        ];

        for (current_value, range_strategy, current_version, new_version, expected) in cases {
            assert_eq!(
                get_new_value(&NewValueConfig {
                    current_value,
                    current_version,
                    range_strategy,
                    new_version,
                    all_versions: None,
                })
                .as_deref(),
                Some(expected)
            );
        }
    }

    // Ported: "does not determine if the proposed newVersion exists, if allVersions is not set" — versioning/github-actions/index.spec.ts line 502
    #[test]
    fn get_new_value_without_all_versions_returns_floating_major() {
        assert_eq!(
            get_new_value(&NewValueConfig {
                current_value: "v7",
                current_version: "v7.6.0",
                new_version: "v8.1.0",
                range_strategy: RangeStrategy::Replace,
                all_versions: None,
            })
            .as_deref(),
            Some("v8")
        );
    }

    // Ported: "does not determine if the proposed newVersion exists, if allVersions is an empty array: %s -> %s" — versioning/github-actions/index.spec.ts line 514
    #[test]
    fn get_new_value_empty_all_versions_behaves_like_absent_all_versions() {
        assert_eq!(
            get_new_value(&NewValueConfig {
                current_value: "v7",
                current_version: "v7.6.0",
                new_version: "v8.1.0",
                range_strategy: RangeStrategy::Replace,
                all_versions: Some(HashSet::new()),
            })
            .as_deref(),
            Some("v8")
        );
    }

    // Ported: "when a major version exists" / "when a minor version exists" / "when a patch version exists" — versioning/github-actions/index.spec.ts line 532
    #[test]
    fn get_new_value_uses_shortest_existing_matching_version() {
        assert_eq!(
            get_new_value(&NewValueConfig {
                current_value: "v7",
                current_version: "v7.6.0",
                new_version: "v8.0.0",
                range_strategy: RangeStrategy::Replace,
                all_versions: Some(set(&["v7.5.0", "v7.6.0", "v8.0"])),
            })
            .as_deref(),
            Some("v8.0")
        );
    }

    // Ported: "preserves floating major for non-major updates ($description)" — versioning/github-actions/index.spec.ts line 562
    #[test]
    fn get_new_value_preserves_floating_major_for_non_major_updates() {
        for (new_version, all_versions) in [
            ("v7.6.0", set(&["v7.5.0", "v7.6.0"])),
            ("v7.6", set(&["v7.5.0", "v7.6", "v7.6.0", "v7"])),
        ] {
            assert_eq!(
                get_new_value(&NewValueConfig {
                    current_value: "v7",
                    current_version: "v7.0.0",
                    new_version,
                    range_strategy: RangeStrategy::Replace,
                    all_versions: Some(all_versions),
                })
                .as_deref(),
                Some("v7")
            );
        }
    }

    // Ported: "migrates from a floating major to a floating major.minor if the floating major no longer exists" — versioning/github-actions/index.spec.ts line 614
    #[test]
    fn get_new_value_migrates_to_floating_minor_when_floating_major_missing() {
        assert_eq!(
            get_new_value(&NewValueConfig {
                current_value: "v7",
                current_version: "v7.0.0",
                new_version: "v7.6",
                range_strategy: RangeStrategy::Replace,
                all_versions: Some(set(&["v7.5.0", "v7.6", "v7.6.0"])),
            })
            .as_deref(),
            Some("v7.6")
        );
    }

    // Ported: "preserves floating minor for non-major updates ($description)" — versioning/github-actions/index.spec.ts line 625
    #[test]
    fn get_new_value_preserves_floating_minor_for_non_major_updates() {
        assert_eq!(
            get_new_value(&NewValueConfig {
                current_value: "v7.5",
                current_version: "v7.5.0",
                new_version: "v7.6.0",
                range_strategy: RangeStrategy::Replace,
                all_versions: Some(set(&["v7", "v7.5.0", "v7.6", "v7.6.0"])),
            })
            .as_deref(),
            Some("v7.6")
        );
    }

    // Ported: "when a release candidate version exists, that exact version is used" — versioning/github-actions/index.spec.ts line 658
    #[test]
    fn get_new_value_uses_existing_release_candidate() {
        assert_eq!(
            get_new_value(&NewValueConfig {
                current_value: "v7",
                current_version: "v7.6.0",
                new_version: "v8.0.0-rc3",
                range_strategy: RangeStrategy::Replace,
                all_versions: Some(set(&["v8.0.0-rc1", "v8.0.0-rc2", "v8.0.0-rc3"])),
            })
            .as_deref(),
            Some("v8.0.0-rc3")
        );
    }

    // Ported: "returns newVersion when newVersion is a floating tag and allVersions is not set" — versioning/github-actions/index.spec.ts line 675
    #[test]
    fn get_new_value_returns_floating_new_version_without_all_versions() {
        assert_eq!(
            get_new_value(&NewValueConfig {
                current_value: "v7",
                current_version: "v7.6.0",
                new_version: "v8",
                range_strategy: RangeStrategy::Replace,
                all_versions: None,
            })
            .as_deref(),
            Some("v8")
        );
    }

    // Ported: "returns the floating newVersion when it exists in allVersions" — versioning/github-actions/index.spec.ts line 685
    #[test]
    fn get_new_value_returns_existing_floating_new_version() {
        assert_eq!(
            get_new_value(&NewValueConfig {
                current_value: "v7",
                current_version: "v7.6.0",
                new_version: "v8",
                range_strategy: RangeStrategy::Replace,
                all_versions: Some(set(&["v7.6.0", "v8"])),
            })
            .as_deref(),
            Some("v8")
        );
    }

    // Ported: "newVersion is returned anyway" — versioning/github-actions/index.spec.ts line 698
    #[test]
    fn get_new_value_returns_missing_new_version_anyway() {
        assert_eq!(
            get_new_value(&NewValueConfig {
                current_value: "v7",
                current_version: "v7.6.0",
                new_version: "v8.5.0",
                range_strategy: RangeStrategy::Replace,
                all_versions: Some(set(&["v7.5.0", "v7.6.0"])),
            })
            .as_deref(),
            Some("v8.5.0")
        );
    }

    // Ported: "debug logs" — versioning/github-actions/index.spec.ts line 709
    #[test]
    fn get_new_value_missing_new_version_logging_is_not_applicable() {
        assert_eq!(
            get_new_value(&NewValueConfig {
                current_value: "v7",
                current_version: "v7.6.0",
                new_version: "v8.5.0",
                range_strategy: RangeStrategy::Replace,
                all_versions: Some(set(&["v7.5.0", "v7.6.0"])),
            })
            .as_deref(),
            Some("v8.5.0")
        );
    }
}
