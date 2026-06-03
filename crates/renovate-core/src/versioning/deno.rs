//! Deno versioning.
//!
//! Renovate reference: `lib/modules/versioning/deno/index.ts`
//!
//! Deno versioning extends npm versioning with two additions:
//! - `"latest"` is a valid version string
//! - `"latest"` and `"*"` have special getNewValue semantics

use super::npm;

pub fn is_valid(input: &str) -> bool {
    input == "latest" || npm::is_valid(input)
}

pub fn get_new_value(
    current_value: &str,
    range_strategy: &str,
    current_version: &str,
    new_version: &str,
) -> Option<String> {
    if current_value == "latest" {
        return match range_strategy {
            "replace" | "pin" => Some(new_version.to_owned()),
            "update-lockfile" => Some("latest".to_owned()),
            _ => None,
        };
    }
    if current_value == "*" {
        return match range_strategy {
            "pin" => Some(new_version.to_owned()),
            "update-lockfile" => Some("*".to_owned()),
            _ => None,
        };
    }
    npm::get_new_value(current_value, range_strategy, current_version, new_version)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isValid(\"$version\") === $isValid" — lib/modules/versioning/deno/index.spec.ts line 4
    #[test]
    fn is_valid_matches_renovate_deno_spec() {
        let cases = [
            ("17.04.0", false),
            ("1.2.3", true),
            ("*", true),
            ("x", true),
            ("X", true),
            ("1", true),
            ("1.2.3-foo", true),
            ("1.2.3foo", false),
            ("~1.2.3", true),
            ("1.2", true),
            ("1.2.x", true),
            ("1.2.X", true),
            ("1.2.*", true),
            ("^1.2.3", true),
            (">1.2.3", true),
            ("latest", true),
            ("renovatebot/renovate", false),
            ("renovatebot/renovate#main", false),
            ("https://github.com/renovatebot/renovate.git", false),
        ];
        for (version, expected) in cases {
            assert_eq!(is_valid(version), expected, "is_valid({version:?})");
        }
    }

    // Ported: "getSatisfyingVersion(\"$versions\",\"$range\") === $maxSatisfying" — lib/modules/versioning/deno/index.spec.ts line 31
    #[test]
    fn get_satisfying_version_matches_renovate_deno_spec() {
        let versions = ["2.3.3.", "2.3.4", "2.4.5", "2.5.1", "3.0.0"];
        for (range, expected) in [
            ("*", "3.0.0"),
            ("x", "3.0.0"),
            ("X", "3.0.0"),
            ("2", "2.5.1"),
            ("2.*", "2.5.1"),
            ("2.3", "2.3.4"),
            ("2.3.*", "2.3.4"),
        ] {
            assert_eq!(
                npm::get_satisfying_version(&versions, range),
                Some(expected),
                "getSatisfyingVersion({range:?})"
            );
        }
    }

    // Ported: "isSingleVersion(\"$version\") === $isSingle" — lib/modules/versioning/deno/index.spec.ts line 47
    #[test]
    fn is_single_version_matches_renovate_deno_spec() {
        for (version, expected) in [
            ("1.2.3", true),
            ("1.2.3-alpha.1", true),
            ("1.x", false),
            ("latest", false),
        ] {
            assert_eq!(
                npm::is_single_version(version),
                expected,
                "is_single_version({version:?})"
            );
        }
    }

    // Ported: "subset(\"$a\", \"$b\") === $expected" — lib/modules/versioning/deno/index.spec.ts line 58
    #[test]
    fn subset_matches_renovate_deno_spec() {
        let cases = [
            ("1.0.0", "1.0.0", true),
            ("1.1.0", "^1.0.0", true),
            ("~1.0.0", "~1.0.0", true),
            ("^1.0.0", "^1.0.0", true),
            ("~1.0.0", "~1.1.0", false),
            ("^1.0.0", "^1.1.0", false),
            ("~1.0.0", "~0.9.0", false),
            ("^1.0.0", "^0.9.0", false),
        ];
        for (a, b, expected) in cases {
            assert_eq!(npm::subset(a, b), expected, "subset({a:?}, {b:?})");
        }
    }

    // Ported: "getNewValue(\"$currentValue\", \"$rangeStrategy\", \"$currentVersion\", \"$newVersion\") === \"$expected\"" — lib/modules/versioning/deno/index.spec.ts line 72
    #[test]
    fn get_new_value_matches_renovate_deno_spec() {
        let cases: &[(&str, &str, &str, &str, Option<&str>)] = &[
            ("^1.0", "bump", "1.0.0", "1.0.7", Some("^1.0.7")),
            (
                "^1",
                "bump",
                "1.0.0",
                "1.0.7-prerelease.1",
                Some("^1.0.7-prerelease.1"),
            ),
            ("^1.0", "bump", "1.0.0", "1.1.7", Some("^1.1.7")),
            ("~1.0", "bump", "1.0.0", "1.1.7", Some("~1.1.7")),
            (
                "~1.0",
                "bump",
                "1.0.0",
                "1.0.7-prerelease.1",
                Some("~1.0.7-prerelease.1"),
            ),
            ("^1", "bump", "1.0.0", "2.1.7", Some("^2.1.7")),
            ("~1", "bump", "1.0.0", "1.1.7", Some("~1.1.7")),
            ("5", "bump", "5.0.0", "5.1.7", Some("5.1.7")),
            ("5", "bump", "5.0.0", "6.1.7", Some("6.1.7")),
            ("5.0", "bump", "5.0.0", "5.0.7", Some("5.0.7")),
            ("5.0", "bump", "5.0.0", "5.1.7", Some("5.1.7")),
            ("5.0", "bump", "5.0.0", "6.1.7", Some("6.1.7")),
            ("1.0.*", "replace", "1.0.0", "1.1.0", Some("1.1.*")),
            ("1.*", "replace", "1.0.0", "2.1.0", Some("2.*")),
            ("*", "bump", "1.0.0", "1.0.1", None),
            ("*", "replace", "1.0.0", "1.0.1", None),
            ("*", "widen", "1.0.0", "1.0.1", None),
            ("*", "pin", "1.0.0", "1.0.1", Some("1.0.1")),
            ("*", "update-lockfile", "1.0.0", "1.0.1", Some("*")),
            ("^0.0.3", "replace", "0.0.3", "0.0.6", Some("^0.0.6")),
            ("^0.0.3", "replace", "0.0.3", "0.5.0", Some("^0.5.0")),
            ("^0.0.3", "replace", "0.0.3", "0.5.6", Some("^0.5.0")),
            ("^0.0.3", "replace", "0.0.3", "4.0.0", Some("^4.0.0")),
            ("^0.0.3", "replace", "0.0.3", "4.0.6", Some("^4.0.0")),
            ("^0.0.3", "replace", "0.0.3", "4.5.6", Some("^4.0.0")),
            ("^0.2.0", "replace", "0.2.0", "0.5.6", Some("^0.5.0")),
            ("^0.2.3", "replace", "0.2.3", "0.5.0", Some("^0.5.0")),
            ("^0.2.3", "replace", "0.2.3", "0.5.6", Some("^0.5.0")),
            ("^1.2.3", "replace", "1.2.3", "4.0.0", Some("^4.0.0")),
            ("^1.2.3", "replace", "1.2.3", "4.5.6", Some("^4.0.0")),
            ("^1.0.0", "replace", "1.0.0", "4.5.6", Some("^4.0.0")),
            ("^0.2.3", "replace", "0.2.3", "0.2.4", Some("^0.2.3")),
            ("^2.3.0", "replace", "2.3.0", "2.4.0", Some("^2.3.0")),
            ("^2.3.4", "replace", "2.3.4", "2.4.5", Some("^2.3.4")),
            ("^2.3.4", "replace", "2.3.4", "2.3.5", Some("^2.3.4")),
            ("~2.3.4", "replace", "2.3.4", "2.3.5", Some("~2.3.0")),
            ("^0.0.1", "replace", "0.0.1", "0.0.2", Some("^0.0.2")),
            ("^0.0.1", "replace", "v0.0.1", "v0.0.2", Some("^0.0.2")),
            ("^1.0.1", "replace", "1.0.1", "2.0.2", Some("^2.0.0")),
            ("^1.2.3", "replace", "1.2.3", "1.2.3", Some("^1.2.3")),
            ("^1.2.3", "replace", "1.2.3", "1.2.2", Some("^1.2.2")),
            ("^0.9.21", "replace", "0.9.21", "0.9.22", Some("^0.9.21")),
            ("1.0.0", "pin", "1.0.0", "1.0.1", Some("1.0.1")),
            ("1.x", "update-lockfile", "1.0.0", "1.0.1", Some("1.x")),
            ("1.x", "update-lockfile", "1.0.0", "2.0.1", Some("2.x")),
            ("^1.0.0", "bump", "1.0.0", "2.0.0", Some("^2.0.0")),
            ("~1.0.0", "bump", "1.0.0", "2.0.0", Some("~2.0.0")),
            ("~1.0.0", "bump", "v1.0.0", "v2.0.0", Some("~2.0.0")),
            (
                "^1.0.0-alpha",
                "replace",
                "1.0.0-alpha",
                "1.0.0-beta",
                Some("^1.0.0-beta"),
            ),
            ("~1.0.0", "replace", "1.0.0", "1.1.0", Some("~1.1.0")),
            ("1.0.x", "replace", "1.0.0", "1.1.0", Some("1.1.x")),
            ("latest", "bump", "1.0.0", "1.0.1", None),
            ("latest", "replace", "1.0.0", "1.0.1", Some("1.0.1")),
            ("latest", "widen", "1.0.0", "1.0.1", None),
            ("latest", "pin", "1.0.0", "1.0.1", Some("1.0.1")),
            (
                "latest",
                "update-lockfile",
                "1.0.0",
                "1.0.1",
                Some("latest"),
            ),
            ("latest", "auto", "1.0.0", "1.0.1", None),
            ("latest", "future", "1.0.0", "1.0.1", None),
            ("latest", "in-range-only", "1.0.0", "1.0.1", None),
            ("*", "auto", "1.0.0", "1.0.1", None),
            ("*", "future", "1.0.0", "1.0.1", None),
            ("*", "in-range-only", "1.0.0", "1.0.1", None),
        ];
        for &(cv, strategy, cur_ver, new_ver, expected) in cases {
            assert_eq!(
                get_new_value(cv, strategy, cur_ver, new_ver).as_deref(),
                expected,
                "get_new_value({cv:?}, {strategy:?}, {cur_ver:?}, {new_ver:?})"
            );
        }
    }
}
