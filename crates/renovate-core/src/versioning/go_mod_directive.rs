//! @parity lib/modules/versioning/go-mod-directive/index.ts full
//! Go Modules Directive versioning.
//!
//! Ports `lib/modules/versioning/go-mod-directive/index.ts`.
//! Delegates to npm/semver with `^` prefix added to ranges.
//! Valid versions: `X.Y` or `X.Y.Z` (no pre-release, no build metadata).

use semver::{Version, VersionReq};

fn to_npm_range(range: &str) -> String {
    format!("^{range}")
}

fn shorten(version: &str) -> String {
    version.splitn(3, '.').take(2).collect::<Vec<_>>().join(".")
}

fn pad_to_semver(partial: &str) -> Option<Version> {
    match partial.matches('.').count() {
        1 => Version::parse(&format!("{partial}.0")).ok(),
        2 => Version::parse(partial).ok(),
        _ => None,
    }
}

pub fn is_valid(input: &str) -> bool {
    let parts: Vec<&str> = input.split('.').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return false;
    }
    parts
        .iter()
        .all(|p| p.chars().all(|c| c.is_ascii_digit()) && !p.is_empty())
}

pub fn is_version(input: &str) -> bool {
    Version::parse(input).is_ok()
}

pub fn matches(version: &str, range: &str) -> bool {
    let Ok(v) = Version::parse(version) else {
        return false;
    };
    let Ok(req) = VersionReq::parse(&to_npm_range(range)) else {
        return false;
    };
    req.matches(&v)
}

pub fn get_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    let req = VersionReq::parse(&to_npm_range(range)).ok()?;
    versions
        .iter()
        .filter_map(|&v| Version::parse(v).ok().map(|p| (v, p)))
        .filter(|(_, p)| req.matches(p))
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(s, _)| s)
}

pub fn min_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    let req = VersionReq::parse(&to_npm_range(range)).ok()?;
    versions
        .iter()
        .filter_map(|&v| Version::parse(v).ok().map(|p| (v, p)))
        .filter(|(_, p)| req.matches(p))
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(s, _)| s)
}

pub fn is_less_than_range(version: &str, range: &str) -> bool {
    let Ok(v) = Version::parse(version) else {
        return false;
    };
    let Some(floor) = pad_to_semver(range) else {
        return false;
    };
    v < floor
}

pub fn get_new_value(current_value: &str, range_strategy: &str, new_version: &str) -> String {
    if range_strategy == "bump" {
        let v_1_20 = Version::new(1, 20, 0);
        if let Ok(nv) = Version::parse(new_version)
            && nv >= v_1_20
        {
            return new_version.to_owned();
        }
        return shorten(new_version);
    }
    if range_strategy == "replace" && !matches(new_version, current_value) {
        return new_version.to_owned();
    }
    current_value.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "matches("$version", "$range") === "$expected"" — lib/modules/versioning/go-mod-directive/index.spec.ts line 4
    #[test]
    fn matches_matches_renovate_go_mod_directive_index_spec() {
        let cases = [
            ("1.16.0", "1.16", true),
            ("1.16.1", "1.16", true),
            ("1.15.0", "1.16", false),
            ("1.19.1", "1.16", true),
            ("2.0.0", "1.16", false),
            ("1.22.2", "1.21.9", true),
        ];
        for (version, range, expected) in cases {
            assert_eq!(
                matches(version, range),
                expected,
                "matches({version:?}, {range:?})"
            );
        }
    }

    // Ported: "getSatisfyingVersion($versions, "$range") === "$expected"" — lib/modules/versioning/go-mod-directive/index.spec.ts line 19
    #[test]
    fn get_satisfying_version_matches_renovate_go_mod_directive_index_spec() {
        assert_eq!(
            get_satisfying_version(&["1.16.0", "1.16.1", "1.17.0"], "1.16"),
            Some("1.17.0")
        );
    }

    // Ported: "isValid("$version") === $expected" — lib/modules/versioning/go-mod-directive/index.spec.ts line 29
    #[test]
    fn is_valid_matches_renovate_go_mod_directive_index_spec() {
        let cases = [("1", false), ("1.2", true), ("1.2.3", true)];
        for (version, expected) in cases {
            assert_eq!(is_valid(version), expected, "is_valid({version:?})");
        }
    }

    // Ported: "isVersion("$version") === $expected" — lib/modules/versioning/go-mod-directive/index.spec.ts line 38
    #[test]
    fn is_version_matches_renovate_go_mod_directive_index_spec() {
        let cases = [("1", false), ("1.2", false), ("1.2.3", true)];
        for (version, expected) in cases {
            assert_eq!(is_version(version), expected, "is_version({version:?})");
        }
    }

    // Ported: "isLessThanRange("$version", "$range") === "$expected"" — lib/modules/versioning/go-mod-directive/index.spec.ts line 47
    #[test]
    fn is_less_than_range_matches_renovate_go_mod_directive_index_spec() {
        let cases = [("1.15.0", "1.16", true), ("1.19.0", "1.16", false)];
        for (version, range, expected) in cases {
            assert_eq!(
                is_less_than_range(version, range),
                expected,
                "is_less_than_range({version:?}, {range:?})"
            );
        }
    }

    // Ported: "minSatisfyingVersion($versions, "$range") === "$expected"" — lib/modules/versioning/go-mod-directive/index.spec.ts line 58
    #[test]
    fn min_satisfying_version_matches_renovate_go_mod_directive_index_spec() {
        assert_eq!(
            min_satisfying_version(&["1.15.0", "1.16.0", "1.16.1"], "1.16"),
            Some("1.16.0")
        );
        assert_eq!(
            min_satisfying_version(&["0.4.0", "0.5.0", "4.2.0", "5.0.0"], "1.16"),
            None
        );
    }

    // Ported: "getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected"" — lib/modules/versioning/go-mod-directive/index.spec.ts line 69
    #[test]
    fn get_new_value_matches_renovate_go_mod_directive_index_spec() {
        let cases = [
            ("1.16", "bump", "1.16.4", "1.17.0", "1.17"),
            ("1.16", "bump", "1.16.4", "1.16.4", "1.16"),
            ("1.16", "replace", "1.16.4", "1.16.4", "1.16"),
            ("1.16", "replace", "1.21.2", "1.21.2", "1.16"),
            ("1.16", "widen", "1.16.4", "1.16.4", "1.16"),
            ("1.16", "bump", "1.16.4", "1.21.3", "1.21.3"),
            ("1.21.2", "bump", "1.21.2", "1.21.3", "1.21.3"),
            ("1.21.2", "replace", "1.21.2", "1.22.2", "1.21.2"),
            ("1.21.2", "replace", "1.21.2", "2.0.0", "2.0.0"),
        ];
        for (current, strategy, _current_version, new_version, expected) in cases {
            assert_eq!(
                get_new_value(current, strategy, new_version),
                expected,
                "get_new_value({current:?}, {strategy:?}, {new_version:?})"
            );
        }
    }
}
