//! Node.js versioning.
//!
//! Ports `lib/modules/versioning/node/index.ts`.
//! Wraps npm versioning with Node.js codename support and LTS-based stability.

use chrono::NaiveDate;
use semver::{Version, VersionReq};

use super::npm;

struct NodeSchedule {
    major: u64,
    codename: Option<&'static str>,
    lts: Option<&'static str>,
}

static SCHEDULE: &[NodeSchedule] = &[
    NodeSchedule {
        major: 4,
        codename: Some("Argon"),
        lts: Some("2015-10-12"),
    },
    NodeSchedule {
        major: 6,
        codename: Some("Boron"),
        lts: Some("2016-10-18"),
    },
    NodeSchedule {
        major: 8,
        codename: Some("Carbon"),
        lts: Some("2017-10-31"),
    },
    NodeSchedule {
        major: 10,
        codename: Some("Dubnium"),
        lts: Some("2018-10-30"),
    },
    NodeSchedule {
        major: 12,
        codename: Some("Erbium"),
        lts: Some("2019-10-21"),
    },
    NodeSchedule {
        major: 14,
        codename: Some("Fermium"),
        lts: Some("2020-10-27"),
    },
    NodeSchedule {
        major: 16,
        codename: Some("Gallium"),
        lts: Some("2021-10-26"),
    },
    NodeSchedule {
        major: 18,
        codename: Some("Hydrogen"),
        lts: Some("2022-10-25"),
    },
    NodeSchedule {
        major: 20,
        codename: Some("Iron"),
        lts: Some("2023-10-24"),
    },
    NodeSchedule {
        major: 22,
        codename: Some("Jod"),
        lts: Some("2024-10-29"),
    },
    NodeSchedule {
        major: 24,
        codename: Some("Krypton"),
        lts: Some("2025-10-28"),
    },
];

fn find_by_codename(name: &str) -> Option<&'static NodeSchedule> {
    let upper = name.to_uppercase();
    SCHEDULE
        .iter()
        .find(|s| s.codename.is_some_and(|c| c.to_uppercase() == upper))
}

fn find_by_major(major: u64) -> Option<&'static NodeSchedule> {
    SCHEDULE.iter().find(|s| s.major == major)
}

fn get_major_from_version(version: &str) -> Option<u64> {
    Version::parse(version.trim_start_matches('v'))
        .ok()
        .map(|v| v.major)
}

fn normalize_value(value: &str) -> String {
    if let Some(sched) = find_by_codename(value) {
        return format!("^{}", sched.major);
    }
    value.to_owned()
}

fn npm_is_stable(version: &str) -> bool {
    Version::parse(version.trim_start_matches('v')).is_ok_and(|v| v.pre.is_empty())
}

fn npm_get_new_value(
    current_value: &str,
    range_strategy: &str,
    current_version: &str,
    new_version: &str,
) -> Option<String> {
    let new_stripped = new_version.trim_start_matches('v');
    // Exact version pin → return new version (node-semver behavior)
    if Version::parse(current_value).is_ok() {
        return Some(new_version.to_owned());
    }
    // Tilde range with replace → ~{major}.{minor}.0
    if current_value.starts_with('~')
        && !current_value.starts_with("~>")
        && range_strategy == "replace"
    {
        let new = Version::parse(new_stripped).ok()?;
        return Some(format!("~{}.{}.0", new.major, new.minor));
    }
    npm::get_new_value(current_value, range_strategy, current_version, new_version)
}

pub fn is_valid(version: &str) -> bool {
    npm::is_valid(&normalize_value(version))
}

pub(crate) fn is_stable_at(version: &str, now: NaiveDate) -> bool {
    if !npm_is_stable(version) {
        return false;
    }
    let Some(major) = get_major_from_version(version) else {
        return false;
    };
    let Some(sched) = find_by_major(major) else {
        return false;
    };
    let Some(lts_str) = sched.lts else {
        return false;
    };
    let Ok(lts) = NaiveDate::parse_from_str(lts_str, "%Y-%m-%d") else {
        return false;
    };
    now > lts
}

pub fn is_stable(version: &str) -> bool {
    is_stable_at(version, chrono::Local::now().date_naive())
}

pub fn matches(version: &str, range: &str) -> bool {
    let normalized = normalize_value(range);
    let Ok(ver) = Version::parse(version.trim_start_matches('v')) else {
        return false;
    };
    VersionReq::parse(&normalized).is_ok_and(|req| req.matches(&ver))
}

pub fn get_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    let normalized = normalize_value(range);
    let Ok(req) = VersionReq::parse(&normalized) else {
        return None;
    };
    versions
        .iter()
        .filter_map(|&v| {
            Version::parse(v.trim_start_matches('v'))
                .ok()
                .map(|p| (v, p))
        })
        .filter(|(_, p)| req.matches(p))
        .max_by_key(|(_, p)| p.clone())
        .map(|(v, _)| v)
}

pub fn min_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    let normalized = normalize_value(range);
    let Ok(req) = VersionReq::parse(&normalized) else {
        return None;
    };
    versions
        .iter()
        .filter_map(|&v| {
            Version::parse(v.trim_start_matches('v'))
                .ok()
                .map(|p| (v, p))
        })
        .filter(|(_, p)| req.matches(p))
        .min_by_key(|(_, p)| p.clone())
        .map(|(v, _)| v)
}

pub fn get_new_value(
    current_value: &str,
    range_strategy: &str,
    current_version: &str,
    new_version: &str,
) -> Option<String> {
    // Codename → find codename for new major version
    if range_strategy != "pin" && find_by_codename(current_value).is_some() {
        let new_major = get_major_from_version(new_version)?;
        let new_sched = find_by_major(new_major)?;
        let codename = new_sched.codename?;
        return Some(codename.to_lowercase());
    }
    let normalized = normalize_value(current_value);
    let res = npm_get_new_value(&normalized, range_strategy, current_version, new_version)?;
    // Strip 'v' prefix if result is a plain version
    if Version::parse(res.trim_start_matches('v')).is_ok() {
        Some(res.trim_start_matches('v').to_owned())
    } else {
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }

    // Ported: "getNewValue($currentValue, $rangeStrategy, $currentVersion, $newVersion, $expected) === $expected" — versioning/node/index.spec.ts line 14
    #[test]
    fn get_new_value_matches_renovate_node_index_spec() {
        let cases = [
            ("1.0.0", "replace", "1.0.0", "v1.1.0", "1.1.0"),
            ("~8.0.0", "replace", "8.0.2", "v8.2.0", "~8.2.0"),
            ("erbium", "replace", "12.0.0", "v14.1.4", "fermium"),
            ("Fermium", "replace", "14.0.0", "v16.1.6", "gallium"),
            ("gallium", "bump", "16.0.0", "v16.1.6", "gallium"),
            ("gallium", "auto", "16.1.6", "v16.1.6", "gallium"),
        ];
        for (current_value, range_strategy, current_version, new_version, expected) in cases {
            assert_eq!(
                get_new_value(current_value, range_strategy, current_version, new_version)
                    .as_deref(),
                Some(expected),
                "get_new_value({current_value:?}, {range_strategy:?}, {current_version:?}, {new_version:?})"
            );
        }
    }

    // Ported: "isStable("$version") === $expected" — versioning/node/index.spec.ts line 43
    #[test]
    fn is_stable_matches_renovate_node_index_spec() {
        let t1 = d("2020-09-01");
        let t2 = d("2021-06-01");
        let cases: &[(&str, NaiveDate, bool)] = &[
            ("16.0.0", t1, false),
            ("15.0.0", t1, false),
            ("14.9.0", t1, false),
            ("14.0.0", t2, true),
            ("12.0.3", t1, true),
            ("v12.0.3", t1, true),
            ("12.0.3a", t1, false),
            ("11.0.0", t1, false),
            ("10.0.0", t1, true),
            ("10.0.999", t1, true),
            ("10.1.0", t1, true),
            ("10.0.0a", t1, false),
            ("9.0.0", t1, false),
        ];
        for &(version, now, expected) in cases {
            assert_eq!(
                is_stable_at(version, now),
                expected,
                "is_stable_at({version:?}, {now})"
            );
        }
    }

    // Ported: "isValid("$version") === $expected" — versioning/node/index.spec.ts line 64
    #[test]
    fn is_valid_matches_renovate_node_index_spec() {
        let cases = [
            ("16.0.0", true),
            ("erbium", true),
            ("bogus", false),
            ("^10.0.0", true),
            ("10.x", true),
            ("10.9.8.7", false),
        ];
        for (version, expected) in cases {
            assert_eq!(is_valid(version), expected, "is_valid({version:?})");
        }
    }

    // Ported: "matches("$version", "$range") === $expected" — versioning/node/index.spec.ts line 75
    #[test]
    fn matches_matches_renovate_node_index_spec() {
        let cases = [("16.0.0", "gallium", true), ("16.0.0", "fermium", false)];
        for (version, range, expected) in cases {
            assert_eq!(
                matches(version, range),
                expected,
                "matches({version:?}, {range:?})"
            );
        }
    }

    // Ported: "getSatisfyingVersion("$versions", "$range") === $expected" — versioning/node/index.spec.ts line 87
    #[test]
    fn get_satisfying_version_matches_renovate_node_index_spec() {
        let cases: &[(&[&str], &str, Option<&str>)] = &[
            (&["16.0.0"], "gallium", Some("16.0.0")),
            (&["16.0.0", "14.0.0", "16.9.9"], "gallium", Some("16.9.9")),
            (&["15.0.0", "14.0.0"], "gallium", None),
        ];
        for &(versions, range, expected) in cases {
            assert_eq!(
                get_satisfying_version(versions, range),
                expected,
                "get_satisfying_version({versions:?}, {range:?})"
            );
        }
    }

    // Ported: "minSatisfyingVersion("$versions", "$range") === $expected" — versioning/node/index.spec.ts line 102
    #[test]
    fn min_satisfying_version_matches_renovate_node_index_spec() {
        let cases: &[(&[&str], &str, Option<&str>)] = &[
            (&["16.0.0"], "gallium", Some("16.0.0")),
            (&["16.0.0", "14.0.0", "16.9.9"], "gallium", Some("16.0.0")),
            (&["15.0.0", "14.0.0"], "gallium", None),
        ];
        for &(versions, range, expected) in cases {
            assert_eq!(
                min_satisfying_version(versions, range),
                expected,
                "min_satisfying_version({versions:?}, {range:?})"
            );
        }
    }
}
