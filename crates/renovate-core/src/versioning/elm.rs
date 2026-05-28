use semver::Version;

fn parse_elm_range(input: &str) -> Option<(String, String)> {
    let re =
        regex::Regex::new(r"^(?P<lower>\d+\.\d+\.\d+)\s*<=\s*v\s*<\s*(?P<upper>\d+\.\d+\.\d+)$")
            .unwrap();
    let caps = re.captures(input.trim())?;
    let lower = caps["lower"].to_string();
    let upper = caps["upper"].to_string();
    let lv = Version::parse(&lower).ok()?;
    let uv = Version::parse(&upper).ok()?;
    if lv > uv {
        return None;
    }
    Some((lower, upper))
}

pub fn is_version(input: &str) -> bool {
    Version::parse(input).is_ok()
}

pub fn is_valid(input: &str) -> bool {
    if is_version(input) {
        return true;
    }
    parse_elm_range(input).is_some()
}

pub fn is_single_version(input: &str) -> bool {
    is_version(input)
}

pub fn is_stable(input: &str) -> bool {
    match Version::parse(input) {
        Ok(v) => v.pre.is_empty(),
        Err(_) => false,
    }
}

pub fn is_compatible(input: &str) -> bool {
    is_version(input)
}

pub fn get_major(input: &str) -> Option<u64> {
    Version::parse(input).ok().map(|v| v.major)
}

pub fn get_minor(input: &str) -> Option<u64> {
    Version::parse(input).ok().map(|v| v.minor)
}

pub fn get_patch(input: &str) -> Option<u64> {
    Version::parse(input).ok().map(|v| v.patch)
}

pub fn equals(a: &str, b: &str) -> bool {
    match (Version::parse(a), Version::parse(b)) {
        (Ok(av), Ok(bv)) => av == bv,
        _ => false,
    }
}

pub fn is_greater_than(a: &str, b: &str) -> bool {
    match (Version::parse(a), Version::parse(b)) {
        (Ok(av), Ok(bv)) => av > bv,
        _ => false,
    }
}

pub fn sort_versions(a: &str, b: &str) -> std::cmp::Ordering {
    match (Version::parse(a), Version::parse(b)) {
        (Ok(av), Ok(bv)) => av.cmp(&bv),
        _ => std::cmp::Ordering::Equal,
    }
}

pub fn matches(version: &str, range: &str) -> bool {
    if !is_version(version) {
        return false;
    }
    if is_version(range) {
        return equals(version, range);
    }
    match parse_elm_range(range) {
        None => false,
        Some((lower, upper)) => {
            let v = Version::parse(version).unwrap();
            let lv = Version::parse(&lower).unwrap();
            let uv = Version::parse(&upper).unwrap();
            v >= lv && v < uv
        }
    }
}

pub fn is_less_than_range(version: &str, range: &str) -> bool {
    if !is_version(version) {
        return false;
    }
    if is_version(range) {
        return is_greater_than(range, version);
    }
    match parse_elm_range(range) {
        None => false,
        Some((lower, _)) => {
            let v = Version::parse(version).unwrap();
            let lv = Version::parse(&lower).unwrap();
            v < lv
        }
    }
}

pub fn get_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    let mut matching: Vec<&str> = versions
        .iter()
        .copied()
        .filter(|v| is_version(v) && matches(v, range))
        .collect();
    matching.sort_by(|a, b| sort_versions(b, a));
    matching.into_iter().next()
}

pub fn min_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    let mut matching: Vec<&str> = versions
        .iter()
        .copied()
        .filter(|v| is_version(v) && matches(v, range))
        .collect();
    matching.sort_by(|a, b| sort_versions(a, b));
    matching.into_iter().next()
}

fn next_major(version: &str) -> String {
    let v = Version::parse(version).unwrap();
    format!("{}.0.0", v.major + 1)
}

#[derive(Debug)]
pub struct NewValueParams {
    pub current_value: String,
    pub range_strategy: String,
    pub new_version: String,
}

pub fn get_new_value(params: &NewValueParams) -> Option<String> {
    let new_ver = &params.new_version;
    if !is_version(new_ver) {
        return None;
    }
    if is_version(&params.current_value) {
        return Some(new_ver.clone());
    }
    let (lower, upper) = parse_elm_range(&params.current_value)?;
    match params.range_strategy.as_str() {
        "pin" => Some(new_ver.clone()),
        "bump" => {
            if matches(new_ver, &params.current_value) {
                Some(format!("{new_ver} <= v < {upper}"))
            } else {
                Some(format!("{new_ver} <= v < {}", next_major(new_ver)))
            }
        }
        "widen" => {
            if matches(new_ver, &params.current_value) {
                return Some(params.current_value.clone());
            }
            let nv = Version::parse(new_ver).unwrap();
            let uv = Version::parse(&upper).unwrap();
            let new_upper = if nv >= uv {
                next_major(new_ver)
            } else {
                upper
            };
            Some(format!("{lower} <= v < {new_upper}"))
        }
        "replace" => Some(format!("{new_ver} <= v < {}", next_major(new_ver))),
        "update-lockfile" => {
            if matches(new_ver, &params.current_value) {
                Some(params.current_value.clone())
            } else {
                Some(format!("{new_ver} <= v < {}", next_major(new_ver)))
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isVersion("$input") === $expected" — versioning/elm/index.spec.ts line 5
    #[test]
    fn is_version_table() {
        let cases = [
            ("1.0.0", true),
            ("0.19.1", true),
            ("2.0.0", true),
            ("1.2.3", true),
            ("invalid", false),
            ("1.0", false),
            ("1", false),
            ("", false),
        ];
        for (input, expected) in cases {
            assert_eq!(
                is_version(input),
                expected,
                "is_version({input:?}) should be {expected}"
            );
        }
    }

    // Ported: "isValid("$input") === $expected" — versioning/elm/index.spec.ts line 23
    #[test]
    fn is_valid_table() {
        let cases = [
            ("1.0.0", true),
            ("1.0.0 <= v < 2.0.0", true),
            ("0.19.0 <= v < 0.20.0", true),
            ("1.0.0 <= v < 1.0.1", true),
            ("invalid", false),
            ("1.0.0 <= v", false),
            ("<= v < 2.0.0", false),
            ("1.0.0 < v < 2.0.0", false),
            ("1.0.0 <= v <= 2.0.0", false),
            (">=1.0.0 <2.0.0", false),
            ("", false),
            ("2.0.0 <= v < 1.0.0", false),
        ];
        for (input, expected) in cases {
            assert_eq!(
                is_valid(input),
                expected,
                "is_valid({input:?}) should be {expected}"
            );
        }
    }

    // Ported: "isSingleVersion("$input") === $expected" — versioning/elm/index.spec.ts line 43
    #[test]
    fn is_single_version_table() {
        let cases = [
            ("1.0.0", true),
            ("0.19.1", true),
            ("1.0.0 <= v < 2.0.0", false),
            ("invalid", false),
        ];
        for (input, expected) in cases {
            assert_eq!(
                is_single_version(input),
                expected,
                "is_single_version({input:?}) should be {expected}"
            );
        }
    }

    // Ported: "isStable("$input") === $expected" — versioning/elm/index.spec.ts line 55
    #[test]
    fn is_stable_table() {
        let cases = [
            ("1.0.0", true),
            ("2.3.4", true),
            ("1.0.0-alpha", false),
            ("1.0.0-beta.1", false),
        ];
        for (input, expected) in cases {
            assert_eq!(
                is_stable(input),
                expected,
                "is_stable({input:?}) should be {expected}"
            );
        }
    }

    // Ported: "returns false for invalid version" — versioning/elm/index.spec.ts line 65
    #[test]
    fn is_stable_invalid_returns_false() {
        assert!(!is_stable("invalid"));
    }

    // Ported: "isCompatible("$input") === $expected" — versioning/elm/index.spec.ts line 71
    #[test]
    fn is_compatible_table() {
        assert!(is_compatible("1.0.0"));
        assert!(!is_compatible("invalid"));
    }

    // Ported: "extracts version components" — versioning/elm/index.spec.ts line 81
    #[test]
    fn extracts_version_components() {
        assert_eq!(get_major("1.2.3"), Some(1));
        assert_eq!(get_minor("1.2.3"), Some(2));
        assert_eq!(get_patch("1.2.3"), Some(3));
    }

    // Ported: "equals("$a", "$b") === $expected" — versioning/elm/index.spec.ts line 89
    #[test]
    fn equals_table() {
        assert!(equals("1.0.0", "1.0.0"));
        assert!(!equals("1.0.0", "1.0.1"));
        assert!(!equals("2.0.0", "1.0.0"));
    }

    // Ported: "isGreaterThan("$a", "$b") === $expected" — versioning/elm/index.spec.ts line 100
    #[test]
    fn is_greater_than_table() {
        assert!(is_greater_than("2.0.0", "1.0.0"));
        assert!(is_greater_than("1.0.1", "1.0.0"));
        assert!(!is_greater_than("1.0.0", "1.0.0"));
        assert!(!is_greater_than("1.0.0", "2.0.0"));
    }

    // Ported: "sorts versions correctly" — versioning/elm/index.spec.ts line 112
    #[test]
    fn sorts_versions_correctly() {
        assert!(sort_versions("1.0.0", "2.0.0") == std::cmp::Ordering::Less);
        assert!(sort_versions("2.0.0", "1.0.0") == std::cmp::Ordering::Greater);
        assert!(sort_versions("1.0.0", "1.0.0") == std::cmp::Ordering::Equal);
    }

    // Ported: "matches("$version", "$range") === $expected" — versioning/elm/index.spec.ts line 120
    #[test]
    fn matches_table() {
        let cases = [
            ("1.0.0", "1.0.0", true),
            ("1.0.1", "1.0.0", false),
            ("1.0.0", "1.0.0 <= v < 2.0.0", true),
            ("1.5.0", "1.0.0 <= v < 2.0.0", true),
            ("1.9.9", "1.0.0 <= v < 2.0.0", true),
            ("2.0.0", "1.0.0 <= v < 2.0.0", false),
            ("0.9.0", "1.0.0 <= v < 2.0.0", false),
            ("0.19.0", "0.19.0 <= v < 0.20.0", true),
            ("0.19.1", "0.19.0 <= v < 0.20.0", true),
            ("0.20.0", "0.19.0 <= v < 0.20.0", false),
        ];
        for (version, range, expected) in cases {
            assert_eq!(
                matches(version, range),
                expected,
                "matches({version:?}, {range:?}) should be {expected}"
            );
        }
    }

    // Ported: "returns false for invalid version" — versioning/elm/index.spec.ts line 139
    #[test]
    fn matches_invalid_version_returns_false() {
        assert!(!matches("invalid", "1.0.0 <= v < 2.0.0"));
    }

    // Ported: "returns false for invalid range" — versioning/elm/index.spec.ts line 143
    #[test]
    fn matches_invalid_range_returns_false() {
        assert!(!matches("1.0.0", "invalid"));
    }

    // Ported: "returns false for malformed range where lower > upper" — versioning/elm/index.spec.ts line 147
    #[test]
    fn matches_lower_gt_upper_returns_false() {
        assert!(!matches("1.5.0", "2.0.0 <= v < 1.0.0"));
    }

    // Ported: "isLessThanRange("$version", "$range") === $expected" — versioning/elm/index.spec.ts line 153
    #[test]
    fn is_less_than_range_table() {
        let cases = [
            ("0.9.0", "1.0.0 <= v < 2.0.0", true),
            ("0.5.0", "1.0.0 <= v < 2.0.0", true),
            ("1.0.0", "1.0.0 <= v < 2.0.0", false),
            ("1.5.0", "1.0.0 <= v < 2.0.0", false),
            ("2.0.0", "1.0.0 <= v < 2.0.0", false),
            ("0.9.0", "1.0.0", true),
            ("1.0.0", "1.0.0", false),
            ("1.1.0", "1.0.0", false),
        ];
        for (version, range, expected) in cases {
            assert_eq!(
                is_less_than_range(version, range),
                expected,
                "is_less_than_range({version:?}, {range:?}) should be {expected}"
            );
        }
    }

    // Ported: "returns false for invalid version" — versioning/elm/index.spec.ts line 170
    #[test]
    fn is_less_than_range_invalid_version_returns_false() {
        assert!(!is_less_than_range("invalid", "1.0.0 <= v < 2.0.0"));
    }

    // Ported: "returns false for invalid range" — versioning/elm/index.spec.ts line 176
    #[test]
    fn is_less_than_range_invalid_range_returns_false() {
        assert!(!is_less_than_range("1.0.0", "invalid"));
    }

    // Ported: "getSatisfyingVersion($versions, "$range") === $expected" — versioning/elm/index.spec.ts line 182
    #[test]
    fn get_satisfying_version_table() {
        let cases: Vec<(Vec<&str>, &str, Option<&str>)> = vec![
            (
                vec!["1.0.0", "1.5.0", "2.0.0"],
                "1.0.0 <= v < 2.0.0",
                Some("1.5.0"),
            ),
            (
                vec!["1.0.0", "1.0.1", "1.0.2"],
                "1.0.0 <= v < 2.0.0",
                Some("1.0.2"),
            ),
            (vec!["0.5.0", "0.9.0"], "1.0.0 <= v < 2.0.0", None),
            (vec!["2.0.0", "3.0.0"], "1.0.0 <= v < 2.0.0", None),
            (vec!["1.0.0"], "1.0.0", Some("1.0.0")),
            (vec!["1.0.1"], "1.0.0", None),
        ];
        for (versions, range, expected) in cases {
            assert_eq!(
                get_satisfying_version(&versions, range),
                expected,
                "get_satisfying_version({versions:?}, {range:?}) should be {expected:?}"
            );
        }
    }

    // Ported: "minSatisfyingVersion($versions, "$range") === $expected" — versioning/elm/index.spec.ts line 199
    #[test]
    fn min_satisfying_version_table() {
        let cases: Vec<(Vec<&str>, &str, Option<&str>)> = vec![
            (
                vec!["1.0.0", "1.5.0", "2.0.0"],
                "1.0.0 <= v < 2.0.0",
                Some("1.0.0"),
            ),
            (
                vec!["1.5.0", "1.6.0", "1.7.0"],
                "1.0.0 <= v < 2.0.0",
                Some("1.5.0"),
            ),
            (vec!["0.5.0", "0.9.0"], "1.0.0 <= v < 2.0.0", None),
            (vec!["2.0.0", "3.0.0"], "1.0.0 <= v < 2.0.0", None),
        ];
        for (versions, range, expected) in cases {
            assert_eq!(
                min_satisfying_version(&versions, range),
                expected,
                "min_satisfying_version({versions:?}, {range:?}) should be {expected:?}"
            );
        }
    }

    // Ported: "replaces exact version with new version" — versioning/elm/index.spec.ts line 215
    #[test]
    fn get_new_value_exact_replace() {
        let result = get_new_value(&NewValueParams {
            current_value: "1.0.0".to_owned(),
            range_strategy: "replace".to_owned(),
            new_version: "1.0.5".to_owned(),
        });
        assert_eq!(result, Some("1.0.5".to_owned()));
    }

    // Ported: "handles bump strategy for exact version" — versioning/elm/index.spec.ts line 225
    #[test]
    fn get_new_value_exact_bump() {
        let result = get_new_value(&NewValueParams {
            current_value: "1.0.0".to_owned(),
            range_strategy: "bump".to_owned(),
            new_version: "2.0.0".to_owned(),
        });
        assert_eq!(result, Some("2.0.0".to_owned()));
    }

    // Ported: "getNewValue("$currentValue", "$rangeStrategy", "$newVersion") === "$expected"" — versioning/elm/index.spec.ts line 237
    #[test]
    fn get_new_value_range_table() {
        let cases = [
            ("1.0.0 <= v < 2.0.0", "bump", "1.0.5", "1.0.5 <= v < 2.0.0"),
            ("1.0.0 <= v < 2.0.0", "bump", "2.0.0", "2.0.0 <= v < 3.0.0"),
            ("1.0.0 <= v < 2.0.0", "widen", "1.5.0", "1.0.0 <= v < 2.0.0"),
            ("1.0.0 <= v < 2.0.0", "widen", "2.0.0", "1.0.0 <= v < 3.0.0"),
            ("1.0.0 <= v < 2.0.0", "widen", "2.5.0", "1.0.0 <= v < 3.0.0"),
            (
                "1.0.0 <= v < 2.0.0",
                "replace",
                "1.5.0",
                "1.5.0 <= v < 2.0.0",
            ),
            (
                "1.0.0 <= v < 2.0.0",
                "replace",
                "2.0.0",
                "2.0.0 <= v < 3.0.0",
            ),
            (
                "0.19.0 <= v < 0.20.0",
                "bump",
                "0.19.1",
                "0.19.1 <= v < 0.20.0",
            ),
            (
                "0.19.0 <= v < 0.20.0",
                "replace",
                "0.20.0",
                "0.20.0 <= v < 1.0.0",
            ),
            (
                "1.0.0 <= v < 2.0.0",
                "update-lockfile",
                "1.5.0",
                "1.0.0 <= v < 2.0.0",
            ),
            (
                "1.0.0 <= v < 2.0.0",
                "update-lockfile",
                "2.0.0",
                "2.0.0 <= v < 3.0.0",
            ),
            ("1.0.0 <= v < 2.0.0", "pin", "1.5.0", "1.5.0"),
        ];
        for (current_value, range_strategy, new_version, expected) in cases {
            let result = get_new_value(&NewValueParams {
                current_value: current_value.to_owned(),
                range_strategy: range_strategy.to_owned(),
                new_version: new_version.to_owned(),
            });
            assert_eq!(
                result,
                Some(expected.to_owned()),
                "get_new_value({current_value:?}, {range_strategy:?}, {new_version:?})"
            );
        }
    }

    // Ported: "returns null for invalid new version" — versioning/elm/index.spec.ts line 266
    #[test]
    fn get_new_value_invalid_new_version_returns_none() {
        let result = get_new_value(&NewValueParams {
            current_value: "1.0.0 <= v < 2.0.0".to_owned(),
            range_strategy: "bump".to_owned(),
            new_version: "invalid".to_owned(),
        });
        assert_eq!(result, None);
    }

    // Ported: "returns null for invalid current value" — versioning/elm/index.spec.ts line 276
    #[test]
    fn get_new_value_invalid_current_value_returns_none() {
        let result = get_new_value(&NewValueParams {
            current_value: "invalid".to_owned(),
            range_strategy: "bump".to_owned(),
            new_version: "1.5.0".to_owned(),
        });
        assert_eq!(result, None);
    }

    // Ported: "returns null for unknown range strategy" — versioning/elm/index.spec.ts line 286
    #[test]
    fn get_new_value_unknown_strategy_returns_none() {
        let result = get_new_value(&NewValueParams {
            current_value: "1.0.0 <= v < 2.0.0".to_owned(),
            range_strategy: "auto".to_owned(),
            new_version: "1.5.0".to_owned(),
        });
        assert_eq!(result, None);
    }

    // Ported: "handles widen when newVersion equals upper bound exactly" — versioning/elm/index.spec.ts line 296
    #[test]
    fn get_new_value_widen_equals_upper() {
        let result = get_new_value(&NewValueParams {
            current_value: "1.0.0 <= v < 2.0.0".to_owned(),
            range_strategy: "widen".to_owned(),
            new_version: "2.0.0".to_owned(),
        });
        assert_eq!(result, Some("1.0.0 <= v < 3.0.0".to_owned()));
    }

    // Ported: "widens elm-version range for new compiler release" — versioning/elm/index.spec.ts line 307
    #[test]
    fn get_new_value_widen_elm_compiler() {
        let result = get_new_value(&NewValueParams {
            current_value: "0.19.0 <= v < 0.20.0".to_owned(),
            range_strategy: "widen".to_owned(),
            new_version: "0.20.0".to_owned(),
        });
        assert_eq!(result, Some("0.19.0 <= v < 1.0.0".to_owned()));
    }

    // Ported: "keeps elm-version range unchanged when version is already satisfied" — versioning/elm/index.spec.ts line 318
    #[test]
    fn get_new_value_update_lockfile_satisfied() {
        let result = get_new_value(&NewValueParams {
            current_value: "0.19.0 <= v < 0.20.0".to_owned(),
            range_strategy: "update-lockfile".to_owned(),
            new_version: "0.19.1".to_owned(),
        });
        assert_eq!(result, Some("0.19.0 <= v < 0.20.0".to_owned()));
    }

    // Ported: "replaces elm-version range when explicitly requested" — versioning/elm/index.spec.ts line 328
    #[test]
    fn get_new_value_replace_elm_version() {
        let result = get_new_value(&NewValueParams {
            current_value: "0.19.0 <= v < 0.20.0".to_owned(),
            range_strategy: "replace".to_owned(),
            new_version: "0.19.1".to_owned(),
        });
        assert_eq!(result, Some("0.19.1 <= v < 1.0.0".to_owned()));
    }

    // Ported: "finds highest satisfying version for elm-version range" — versioning/elm/index.spec.ts line 341
    #[test]
    fn get_satisfying_version_elm_compiler() {
        let versions = vec!["0.18.0", "0.19.0", "0.19.1", "0.20.0", "0.21.0"];
        assert_eq!(
            get_satisfying_version(&versions, "0.19.0 <= v < 0.20.0"),
            Some("0.19.1")
        );
    }

    // Ported: "returns null when no compiler version satisfies range" — versioning/elm/index.spec.ts line 355
    #[test]
    fn get_satisfying_version_none_satisfies() {
        let versions = vec!["0.18.0", "0.20.0"];
        assert_eq!(
            get_satisfying_version(&versions, "0.19.0 <= v < 0.20.0"),
            None
        );
    }
}
