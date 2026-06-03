//! Lambda Node.js runtime versioning.
//! @parity lib/modules/versioning/lambda-node/index.ts full
//! @parity lib/modules/versioning/lambda-node/schedule.ts full
//!
//! Ports `lib/modules/versioning/lambda-node/index.ts`.
//! Extends Node.js versioning with AWS Lambda support-window stability checks.

use chrono::NaiveDate;
use semver::Version;

use super::node;

pub use node::{get_new_value, get_satisfying_version, is_valid, matches, min_satisfying_version};

enum LambdaEntry {
    #[allow(
        dead_code,
        reason = "Constructed under cfg(test) and reserved for runtimes without a sunset date"
    )]
    Always,
    Until(&'static str),
}

static LAMBDA_SCHEDULE: &[(u64, LambdaEntry)] = &[
    (10, LambdaEntry::Until("2021-07-30")),
    (12, LambdaEntry::Until("2023-03-31")),
    (14, LambdaEntry::Until("2023-12-04")),
    (16, LambdaEntry::Until("2024-06-12")),
    (18, LambdaEntry::Until("2025-09-01")),
    (20, LambdaEntry::Until("2026-04-30")),
    (22, LambdaEntry::Until("2027-04-30")),
    (24, LambdaEntry::Until("2028-04-30")),
];

fn is_stable_in_schedule(version: &str, now: NaiveDate, schedule: &[(u64, LambdaEntry)]) -> bool {
    if !node::is_stable_at(version, now) {
        return false;
    }
    let Some(major) = Version::parse(version.trim_start_matches('v'))
        .ok()
        .map(|v| v.major)
    else {
        return false;
    };
    match schedule.iter().find(|(m, _)| *m == major).map(|(_, e)| e) {
        None => false,
        Some(LambdaEntry::Always) => true,
        Some(LambdaEntry::Until(date_str)) => {
            NaiveDate::parse_from_str(date_str, "%Y-%m-%d").is_ok_and(|end| now < end)
        }
    }
}

pub fn is_stable(version: &str) -> bool {
    is_stable_in_schedule(version, chrono::Local::now().date_naive(), LAMBDA_SCHEDULE)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(s: &str) -> NaiveDate {
        NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }

    // Test schedule mirrors the vi.mock() setup in index.spec.ts:
    // v20.support = true (Always), v22 removed entirely.
    static TEST_SCHEDULE: &[(u64, LambdaEntry)] = &[
        (10, LambdaEntry::Until("2021-07-30")),
        (12, LambdaEntry::Until("2023-03-31")),
        (14, LambdaEntry::Until("2023-12-04")),
        (16, LambdaEntry::Until("2024-06-12")),
        (18, LambdaEntry::Until("2025-09-01")),
        (20, LambdaEntry::Always),
        // v22 not present (removed in mock)
        (24, LambdaEntry::Until("2028-04-30")),
    ];

    fn stable(version: &str, now: &str) -> bool {
        is_stable_in_schedule(version, d(now), TEST_SCHEDULE)
    }

    // Ported: "getNewValue($currentValue, $rangeStrategy, $currentVersion, $newVersion, $expected) === $expected" — lib/modules/versioning/lambda-node/index.spec.ts line 47
    #[test]
    fn get_new_value_matches_renovate_lambda_node_index_spec() {
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
                "get_new_value({current_value:?}, {range_strategy:?})"
            );
        }
    }

    // Ported: "isStable("$version") === $expected" — lib/modules/versioning/lambda-node/index.spec.ts line 71
    #[test]
    fn is_stable_matches_renovate_lambda_node_index_spec() {
        let t1 = "2025-03-01";
        let t2 = "2024-03-01";
        let cases: &[(&str, &str, bool)] = &[
            ("v22.0.0", t1, false),
            ("v20.0.0", t1, true),
            ("Iron", t1, false),
            ("v18.0.3", t1, true),
            ("v18.0.0", t1, true),
            ("18.0.0", t1, true),
            ("18.0.0a", t1, false),
            ("16.0.0", t2, true),
            ("16.0.0", t1, false),
            ("15.0.0", t1, false),
            ("14.9.0", t1, false),
            ("14.0.0", t1, false),
            ("12.0.3", t1, false),
            ("v12.0.3", t1, false),
            ("12.0.3a", t1, false),
            ("11.0.0", t1, false),
            ("10.0.0", t1, false),
            ("10.0.999", t1, false),
            ("10.1.0", t1, false),
            ("10.0.0a", t1, false),
            ("9.0.0", t1, false),
        ];
        for &(version, now, expected) in cases {
            assert_eq!(
                stable(version, now),
                expected,
                "is_stable({version:?}, {now:?})"
            );
        }
    }

    // Ported: "isValid("$version") === $expected" — lib/modules/versioning/lambda-node/index.spec.ts line 100
    #[test]
    fn is_valid_matches_renovate_lambda_node_index_spec() {
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

    // Ported: "matches("$version", "$range") === $expected" — lib/modules/versioning/lambda-node/index.spec.ts line 112
    #[test]
    fn matches_matches_renovate_lambda_node_index_spec() {
        let cases = [("16.0.0", "gallium", true), ("16.0.0", "fermium", false)];
        for (version, range, expected) in cases {
            assert_eq!(
                matches(version, range),
                expected,
                "matches({version:?}, {range:?})"
            );
        }
    }

    // Ported: "getSatisfyingVersion("$versions", "$range") === $expected" — lib/modules/versioning/lambda-node/index.spec.ts line 125
    #[test]
    fn get_satisfying_version_matches_renovate_lambda_node_index_spec() {
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

    // Ported: "minSatisfyingVersion("$versions", "$range") === $expected" — lib/modules/versioning/lambda-node/index.spec.ts line 139
    #[test]
    fn min_satisfying_version_matches_renovate_lambda_node_index_spec() {
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
