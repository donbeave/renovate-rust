//! @parity lib/modules/versioning/pvp/index.ts full
//! @parity lib/modules/versioning/pvp/range.ts full
//!
//! Package Versioning Policy (Haskell) versioning.
//!
//! Renovate reference:
//! - `lib/modules/versioning/pvp/index.ts`

use std::cmp::Ordering;
use std::sync::LazyLock;

use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Range {
    pub lower: String,
    pub upper: String,
}

#[derive(Debug, PartialEq, Eq)]
struct Parts {
    major: Vec<u64>,
    minor: Vec<u64>,
    patch: Vec<u64>,
}

static GTE_AND_LT_RANGE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r">=(?<lower>[\d.]+)&&<(?<upper>[\d.]+)").unwrap());
static LT_AND_GTE_RANGE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<(?<upper>[\d.]+)&&>=(?<lower>[\d.]+)").unwrap());
static DIGITS_AND_DOTS_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[\d.]+$").unwrap());

pub fn extract_all_parts(version: &str) -> Option<Vec<u64>> {
    let mut parts = Vec::new();
    for part in version.split('.') {
        parts.push(part.parse::<u64>().ok()?);
    }
    Some(parts)
}

fn get_parts(version: &str) -> Option<Parts> {
    let parts = extract_all_parts(version)?;
    let major_end = parts.len().min(2);
    let minor_end = parts.len().min(3);
    Some(Parts {
        major: parts[..major_end].to_vec(),
        minor: if parts.len() > 2 {
            parts[2..minor_end].to_vec()
        } else {
            Vec::new()
        },
        patch: if parts.len() > 3 {
            parts[3..].to_vec()
        } else {
            Vec::new()
        },
    })
}

pub fn parse_range(input: &str) -> Option<Range> {
    let no_spaces = input.replace(' ', "");
    let captures = GTE_AND_LT_RANGE_RE
        .captures(&no_spaces)
        .or_else(|| LT_AND_GTE_RANGE_RE.captures(&no_spaces))?;

    Some(Range {
        lower: captures.name("lower")?.as_str().to_owned(),
        upper: captures.name("upper")?.as_str().to_owned(),
    })
}

fn plus_one(major: &[u64]) -> Option<String> {
    Some(format!("{}.{}", major.first()?, major.get(1)? + 1))
}

fn join_numbers(parts: &[u64], separator: &str) -> String {
    parts
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(separator)
}

fn compare_int_array(version_parts: &[u64], other_parts: &[u64]) -> Ordering {
    for index in 0..version_parts.len().min(other_parts.len()) {
        match version_parts[index].cmp(&other_parts[index]) {
            Ordering::Equal => {}
            ordering => return ordering,
        }
    }
    version_parts.len().cmp(&other_parts.len())
}

pub fn is_greater_than(version: &str, other: &str) -> bool {
    let Some(version_parts) = extract_all_parts(version) else {
        return false;
    };
    let Some(other_parts) = extract_all_parts(other) else {
        return false;
    };
    compare_int_array(&version_parts, &other_parts).is_gt()
}

pub fn get_major(version: &str) -> Option<f64> {
    join_numbers(&get_parts(version)?.major, ".").parse().ok()
}

pub fn get_minor(version: &str) -> Option<f64> {
    let minor = get_parts(version)?.minor;
    if minor.is_empty() {
        return None;
    }
    join_numbers(&minor, ".").parse().ok()
}

pub fn get_patch(version: &str) -> Option<f64> {
    let patch = get_parts(version)?.patch;
    let (first, rest) = patch.split_first()?;
    format!("{first}.{}", join_numbers(rest, "")).parse().ok()
}

pub fn matches(version: &str, range: &str) -> bool {
    let Some(parsed) = parse_range(range) else {
        return false;
    };
    let Some(version_parts) = extract_all_parts(version) else {
        return false;
    };
    let Some(lower_parts) = extract_all_parts(&parsed.lower) else {
        return false;
    };
    let Some(upper_parts) = extract_all_parts(&parsed.upper) else {
        return false;
    };

    compare_int_array(&upper_parts, &version_parts).is_gt()
        && !compare_int_array(&lower_parts, &version_parts).is_gt()
}

fn satisfying_version<'a>(versions: &'a [&'a str], range: &str, reverse: bool) -> Option<&'a str> {
    let mut versions = versions.to_vec();
    versions.sort_by(|a, b| {
        let ordering = sort_versions(a, b);
        if reverse {
            ordering
        } else {
            ordering.reverse()
        }
    });
    versions.into_iter().find(|version| matches(version, range))
}

pub fn get_satisfying_version<'a>(versions: &'a [&'a str], range: &str) -> Option<&'a str> {
    satisfying_version(versions, range, false)
}

pub fn min_satisfying_version<'a>(versions: &'a [&'a str], range: &str) -> Option<&'a str> {
    satisfying_version(versions, range, true)
}

pub fn is_less_than_range(version: &str, range: &str) -> bool {
    let Some(parsed) = parse_range(range) else {
        return false;
    };
    let Some(version_parts) = extract_all_parts(version) else {
        return false;
    };
    let Some(lower_parts) = extract_all_parts(&parsed.lower) else {
        return false;
    };
    compare_int_array(&version_parts, &lower_parts).is_lt()
}

pub fn get_new_value(
    current_value: &str,
    new_version: &str,
    range_strategy: &str,
) -> Option<String> {
    if range_strategy != "widen" {
        return None;
    }
    let parsed = parse_range(current_value)?;
    if is_less_than_range(new_version, current_value) || matches(new_version, current_value) {
        return None;
    }
    let parts = get_parts(new_version)?;
    let major_plus_one = plus_one(&parts.major)?;
    if !matches(
        new_version,
        &format!(">={} && <{}", parsed.lower, major_plus_one),
    ) {
        return None;
    }
    Some(format!(">={} && <{}", parsed.lower, major_plus_one))
}

pub fn is_same(component: &str, a: &str, b: &str) -> bool {
    let Some(a_parts) = get_parts(a) else {
        return false;
    };
    let Some(b_parts) = get_parts(b) else {
        return false;
    };

    let (a, b) = match component {
        "major" => (&a_parts.major, &b_parts.major),
        "minor" => (&a_parts.minor, &b_parts.minor),
        "patch" => (&a_parts.patch, &b_parts.patch),
        _ => return false,
    };
    compare_int_array(a, b).is_eq()
}

pub fn subset(sub_range: &str, super_range: &str) -> Option<bool> {
    let sub = parse_range(sub_range)?;
    let sup = parse_range(super_range)?;
    let sub_lower = extract_all_parts(&sub.lower)?;
    let sub_upper = extract_all_parts(&sub.upper)?;
    let sup_lower = extract_all_parts(&sup.lower)?;
    let sup_upper = extract_all_parts(&sup.upper)?;

    if compare_int_array(&sub_lower, &sup_lower).is_lt() {
        return Some(false);
    }
    if compare_int_array(&sub_upper, &sup_upper).is_gt() {
        return Some(false);
    }
    Some(true)
}

pub fn is_version(input: Option<&str>) -> bool {
    input.is_some_and(|value| parse_range(value).is_none())
}

pub fn is_valid(input: &str) -> bool {
    extract_all_parts(input).is_some() || parse_range(input).is_some()
}

pub fn is_single_version(input: &str) -> bool {
    let input = input.trim();
    input
        .strip_prefix("==")
        .is_some_and(|version| DIGITS_AND_DOTS_RE.is_match(version))
}

pub fn equals(a: &str, b: &str) -> bool {
    let Some(a_parts) = extract_all_parts(a) else {
        return false;
    };
    let Some(b_parts) = extract_all_parts(b) else {
        return false;
    };
    compare_int_array(&a_parts, &b_parts).is_eq()
}

pub fn sort_versions(a: &str, b: &str) -> Ordering {
    if equals(a, b) {
        Ordering::Equal
    } else if is_greater_than(a, b) {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

pub fn is_stable(_version: &str) -> bool {
    true
}

pub fn is_compatible(_version: &str) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "should parse >=1.0 && <1.1" — lib/modules/versioning/pvp/range.spec.ts line 5
    #[test]
    fn parse_range_parses_lower_and_upper_bounds() {
        let parsed = parse_range(">=1.0 && <1.1").expect("range should parse");
        assert_eq!(parsed.lower, "1.0");
        assert_eq!(parsed.upper, "1.1");
    }

    // Ported: "should return null when there are no numbers" — lib/modules/versioning/pvp/util.spec.ts line 5
    #[test]
    fn extract_all_parts_returns_none_when_there_are_no_numbers() {
        assert_eq!(extract_all_parts(""), None);
    }

    // Ported: "should parse 3.0" — lib/modules/versioning/pvp/util.spec.ts line 9
    #[test]
    fn extract_all_parts_parses_numeric_components() {
        assert_eq!(extract_all_parts("3.0"), Some(vec![3, 0]));
    }

    // Ported: "\"0\" is valid major version" — lib/modules/versioning/pvp/util.spec.ts line 15
    #[test]
    fn get_parts_accepts_zero_major_version() {
        assert_eq!(get_parts("0").map(|parts| parts.major), Some(vec![0]));
    }

    // Ported: "returns null when no parts could be extracted" — lib/modules/versioning/pvp/util.spec.ts line 19
    #[test]
    fn get_parts_returns_none_when_no_parts_can_be_extracted() {
        assert_eq!(get_parts(""), None);
    }

    // Ported: "pvp.isGreaterThan($first, $second)" — lib/modules/versioning/pvp/index.spec.ts line 5
    #[test]
    fn is_greater_than_matches_renovate_pvp_spec() {
        let cases = [
            ("1.23.1", "1.9.6", true),
            ("4.0.0", "3.0.0", true),
            ("3.0.1", "3.0.0", true),
            ("4.10", "4.1", true),
            ("1.0.0", "1.0", true),
            ("2.0.2", "3.1.0", false),
            ("3.0.0", "3.0.0", false),
            ("4.1", "4.10", false),
            ("1.0", "1.0.0", false),
            ("", "1.0", false),
            ("1.0", "", false),
        ];

        for (first, second, expected) in cases {
            assert_eq!(
                is_greater_than(first, second),
                expected,
                "is_greater_than({first}, {second})"
            );
        }
    }

    // Ported: "pvp.getMajor(\"$version\") === $expected" — lib/modules/versioning/pvp/index.spec.ts line 24
    #[test]
    fn get_major_matches_renovate_pvp_spec() {
        let cases = [
            ("1.0.0", Some(1.0)),
            ("1.0.1", Some(1.0)),
            ("1.1.1", Some(1.1)),
            ("", None),
        ];

        for (version, expected) in cases {
            assert_eq!(get_major(version), expected, "get_major({version})");
        }
    }

    // Ported: "pvp.getMinor(\"$version\") === $expected" — lib/modules/versioning/pvp/index.spec.ts line 36
    #[test]
    fn get_minor_matches_renovate_pvp_spec() {
        let cases = [
            ("1.0", None),
            ("1.0.0", Some(0.0)),
            ("1.0.1", Some(1.0)),
            ("1.1.2", Some(2.0)),
        ];

        for (version, expected) in cases {
            assert_eq!(get_minor(version), expected, "get_minor({version})");
        }
    }

    // Ported: "pvp.getPatch(\"$version\") === $expected" — lib/modules/versioning/pvp/index.spec.ts line 48
    #[test]
    fn get_patch_matches_renovate_pvp_spec() {
        let cases = [
            ("1.0.0", None),
            ("1.0.0.5.1", Some(5.1)),
            ("1.0.1.6", Some(6.0)),
            ("1.1.2.7", Some(7.0)),
            ("0.0.0.0.1", Some(0.1)),
            ("0.0.0.0.10", Some(0.1)),
        ];

        for (version, expected) in cases {
            assert_eq!(get_patch(version), expected, "get_patch({version})");
        }
    }

    // Ported: "pvp.matches(\"$version\", \"$range\") === $expected" — lib/modules/versioning/pvp/index.spec.ts line 62
    #[test]
    fn matches_matches_renovate_pvp_spec() {
        let cases = [
            ("1.0.1", ">=1.0 && <1.1", true),
            ("4.1", ">=4.0 && <4.10", true),
            ("4.1", ">=4.1 && <4.10", true),
            ("4.1.0", ">=4.1 && <4.10", true),
            ("4.1.0", "<4.10 && >=4.1", true),
            ("4.10", ">=4.1 && <4.10.0", true),
            ("4.10", ">=4.0 && <4.10.1", true),
            ("1.0.0", ">=2.0 && <2.1", false),
            ("4", ">=4.0 && <4.10", false),
            ("4.10", ">=4.1 && <4.10", false),
            ("4", "gibberish", false),
            ("", ">=1.0 && <1.1", false),
        ];

        for (version, range, expected) in cases {
            assert_eq!(
                matches(version, range),
                expected,
                "matches({version}, {range})"
            );
        }
    }

    // Ported: "pvp.getSatisfyingVersion($versions, \"$range\") === $expected" — lib/modules/versioning/pvp/index.spec.ts line 85
    #[test]
    fn get_satisfying_version_matches_renovate_pvp_spec() {
        let cases = [
            (
                vec!["1.0.0", "1.0.4", "1.3.0", "2.0.0"],
                ">=1.0 && <1.1",
                Some("1.0.4"),
            ),
            (
                vec!["2.0.0", "1.0.0", "1.0.4", "1.3.0"],
                ">=1.0 && <1.1",
                Some("1.0.4"),
            ),
            (
                vec!["1.0.0", "1.0.4", "1.3.0", "2.0.0"],
                ">=3.0 && <4.0",
                None,
            ),
        ];

        for (versions, range, expected) in cases {
            assert_eq!(get_satisfying_version(&versions, range), expected);
        }
    }

    // Ported: "should return min satisfying version in range" — lib/modules/versioning/pvp/index.spec.ts line 99
    #[test]
    fn min_satisfying_version_returns_min_satisfying_version_in_range() {
        let versions = ["0.9", "1.0.0", "1.0.4", "1.3.0", "2.0.0"];
        assert_eq!(
            min_satisfying_version(&versions, ">=1.0 && <1.1"),
            Some("1.0.0")
        );
    }

    // Ported: "pvp.isLessThanRange?.(\"$version\", \"$range\") === $expected" — lib/modules/versioning/pvp/index.spec.ts line 110
    #[test]
    fn is_less_than_range_matches_renovate_pvp_spec() {
        let cases = [
            ("2.0.2", ">=3.0 && <3.1", true),
            ("3", ">=3.0 && <3.1", true),
            ("3", ">=3 && <3.1", false),
            ("3.0", ">=3.0 && <3.1", false),
            ("3.0.0", ">=3.0 && <3.1", false),
            ("4.0.0", ">=3.0 && <3.1", false),
            ("3.1.0", ">=3.0 && <3.1", false),
            ("3", "gibberish", false),
            ("", ">=3.0 && <3.1", false),
        ];

        for (version, range, expected) in cases {
            assert_eq!(
                is_less_than_range(version, range),
                expected,
                "is_less_than_range({version}, {range})"
            );
        }
    }

    // Ported: "pvp.isValid(\"$version\") === $expected" — lib/modules/versioning/pvp/index.spec.ts line 130
    #[test]
    fn is_valid_matches_renovate_pvp_spec() {
        let cases = [
            ("", false),
            ("1.0.0.0", true),
            ("1.0", true),
            (">=1.0 && <1.1", true),
        ];

        for (version, expected) in cases {
            assert_eq!(is_valid(version), expected, "is_valid({version})");
        }
    }

    // Ported: "pvp.getNewValue({currentValue: \"$currentValue\", newVersion: \"$newVersion\", rangeStrategy: \"$rangeStrategy\"}) === $expected" — lib/modules/versioning/pvp/index.spec.ts line 142
    #[test]
    fn get_new_value_matches_renovate_pvp_spec() {
        let cases = [
            (">=1.0 && <1.1", "1.1", "widen", Some(">=1.0 && <1.2")),
            (">=1.2 && <1.3", "1.2.3", "widen", None),
            (">=1.0 && <1.1", "1.2.3", "update-lockfile", None),
            ("gibberish", "1.2.3", "widen", None),
            (">=1.0 && <1.1", "0.9", "widen", None),
            (">=1.0 && <1.1", "", "widen", None),
        ];

        for (current_value, new_version, range_strategy, expected) in cases {
            assert_eq!(
                get_new_value(current_value, new_version, range_strategy).as_deref(),
                expected,
                "get_new_value({current_value}, {new_version}, {range_strategy})"
            );
        }
    }

    // Ported: "pvp.isSame(\"$type\", \"$a\", \"$b\") === $expected" — lib/modules/versioning/pvp/index.spec.ts line 161
    #[test]
    fn is_same_matches_renovate_pvp_spec() {
        let cases = [
            ("major", "4.10", "4.1", false),
            ("major", "4.1.0", "5.1.0", false),
            ("major", "4.1", "5.1", false),
            ("major", "0", "1", false),
            ("major", "4.1", "4.1.0", true),
            ("major", "4.1.1", "4.1.2", true),
            ("major", "0", "0", true),
            ("minor", "4.1.0", "5.1.0", true),
            ("minor", "4.1", "4.1", true),
            ("minor", "4.1", "5.1", true),
            ("minor", "4.1.0", "4.1.1", false),
            ("minor", "", "0", false),
            ("patch", "1.0.0.0", "1.0.0.0", true),
            ("patch", "1.0.0.0", "2.0.0.0", true),
            ("patch", "1.0.0.0", "1.0.0.1", false),
            ("patch", "0.0.0.0.1", "0.0.0.0.10", false),
        ];

        for (component, a, b, expected) in cases {
            assert_eq!(
                is_same(component, a, b),
                expected,
                "is_same({component}, {a}, {b})"
            );
        }
    }

    // Ported: "pvp.isVersion(\"$version\") === $expected" — lib/modules/versioning/pvp/index.spec.ts line 188
    #[test]
    fn is_version_matches_renovate_pvp_spec() {
        let cases = [("1.0", true), (">=1.0 && <1.1", false)];

        for (version, expected) in cases {
            assert_eq!(is_version(Some(version)), expected, "is_version({version})");
        }
    }

    // Ported: "pvp.equals(\"$a\", \"$b\") === $expected" — lib/modules/versioning/pvp/index.spec.ts line 198
    #[test]
    fn equals_matches_renovate_pvp_spec() {
        let cases = [
            ("1.01", "1.1", true),
            ("1.01", "1.0", false),
            ("", "1.0", false),
            ("1.0", "", false),
        ];

        for (a, b, expected) in cases {
            assert_eq!(equals(a, b), expected, "equals({a}, {b})");
        }
    }

    // Ported: "pvp.isSingleVersion(\"$version\") === $expected" — lib/modules/versioning/pvp/index.spec.ts line 210
    #[test]
    fn is_single_version_matches_renovate_pvp_spec() {
        let cases = [("==1.0", true), (">=1.0 && <1.1", false)];

        for (version, expected) in cases {
            assert_eq!(
                is_single_version(version),
                expected,
                "is_single_version({version})"
            );
        }
    }

    // Ported: "pvp.subbet(\"$subRange\", \"$superRange\") === $expected" — lib/modules/versioning/pvp/index.spec.ts line 223
    #[test]
    fn subset_matches_renovate_pvp_spec() {
        let cases = [
            (">=1.0 && <1.1", ">=1.0 && <2.0", Some(true)),
            (">=1.0 && <2.0", ">=1.0 && <2.0", Some(true)),
            (">=1.0 && <2.1", ">=1.0 && <2.0", Some(false)),
            (">=0.9 && <2.1", ">=1.0 && <2.0", Some(false)),
            ("gibberish", "", None),
            (">=. && <.", ">=. && <.", None),
        ];

        for (sub_range, super_range, expected) in cases {
            assert_eq!(
                subset(sub_range, super_range),
                expected,
                "subset({sub_range}, {super_range})"
            );
        }
    }

    // Ported: "pvp.sortVersions(\"$a\", \"$b\") === $expected" — lib/modules/versioning/pvp/index.spec.ts line 240
    #[test]
    fn sort_versions_matches_renovate_pvp_spec() {
        let cases = [
            ("1.0", "1.1", Ordering::Less),
            ("1.1", "1.0", Ordering::Greater),
            ("1.0", "1.0", Ordering::Equal),
        ];

        for (a, b, expected) in cases {
            assert_eq!(sort_versions(a, b), expected, "sort_versions({a}, {b})");
        }
    }

    // Ported: "should consider 0.0.0 stable" — lib/modules/versioning/pvp/index.spec.ts line 251
    #[test]
    fn is_stable_considers_all_versions_stable() {
        assert!(is_stable("0.0.0"));
    }

    // Ported: "should consider 0.0.0 compatible" — lib/modules/versioning/pvp/index.spec.ts line 259
    #[test]
    fn is_compatible_considers_all_versions_compatible() {
        assert!(is_compatible("0.0.0"));
    }
}
