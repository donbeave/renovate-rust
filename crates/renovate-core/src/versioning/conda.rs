//! Conda versioning.
//!
//! Ports `lib/modules/versioning/conda/index.ts`.
//! Uses a custom conda version parser supporting epochs, pre-release, and glob specs.

use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
enum CondaPart {
    Num(u64),
    Str(String),
}

fn extract_epoch(s: &str) -> (u64, &str) {
    if let Some(pos) = s.find('!')
        && let Ok(e) = s[..pos].parse::<u64>() {
            return (e, &s[pos + 1..]);
        }
    (0, s)
}

fn is_valid_version(s: &str) -> bool {
    !s.is_empty()
        && !s.contains('/')
        && !s.contains('#')
        && !s.contains('*')
        && !s.starts_with(['=', '~', '>', '<'])
}

fn is_glob_spec(s: &str) -> bool {
    s.ends_with(".*")
        && s[..s.len() - 2]
            .chars()
            .all(|c| c.is_ascii_digit() || c == '.')
}

fn parse_component_parts(s: &str) -> Vec<CondaPart> {
    let mut result = Vec::new();
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return result;
    }
    let mut start = 0;
    let mut current_is_digit = bytes[0].is_ascii_digit();

    for (i, &b) in bytes.iter().enumerate().skip(1) {
        let is_digit = b.is_ascii_digit();
        if is_digit != current_is_digit {
            let part = &s[start..i];
            if current_is_digit {
                result.push(CondaPart::Num(part.parse().unwrap_or(0)));
            } else {
                result.push(CondaPart::Str(part.to_lowercase()));
            }
            start = i;
            current_is_digit = is_digit;
        }
    }
    let part = &s[start..];
    if current_is_digit {
        result.push(CondaPart::Num(part.parse().unwrap_or(0)));
    } else {
        result.push(CondaPart::Str(part.to_lowercase()));
    }
    result
}

fn parse_for_compare(s: &str) -> Option<(u64, Vec<CondaPart>)> {
    if !is_valid_version(s) {
        return None;
    }
    let (epoch, rest) = extract_epoch(s);
    let parts: Vec<CondaPart> = rest
        .split(['.', '-', '_'])
        .filter(|p| !p.is_empty())
        .flat_map(parse_component_parts)
        .collect();
    Some((epoch, parts))
}

fn compare_parts(a: &CondaPart, b: &CondaPart) -> i32 {
    match (a, b) {
        (CondaPart::Num(av), CondaPart::Num(bv)) => {
            if av > bv {
                1
            } else if av < bv {
                -1
            } else {
                0
            }
        }
        (CondaPart::Str(as_), CondaPart::Str(bs)) => match as_.cmp(bs) {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => 0,
        },
        (CondaPart::Num(_), CondaPart::Str(_)) => 1,
        (CondaPart::Str(_), CondaPart::Num(_)) => -1,
    }
}

fn compare_conda(a: &(u64, Vec<CondaPart>), b: &(u64, Vec<CondaPart>)) -> i32 {
    if a.0 != b.0 {
        return if a.0 > b.0 { 1 } else { -1 };
    }
    let max_len = a.1.len().max(b.1.len());
    for i in 0..max_len {
        let ap = a.1.get(i).cloned().unwrap_or(CondaPart::Num(0));
        let bp = b.1.get(i).cloned().unwrap_or(CondaPart::Num(0));
        let cmp = compare_parts(&ap, &bp);
        if cmp != 0 {
            return cmp;
        }
    }
    0
}

fn compare_versions_str(a: &str, b: &str) -> i32 {
    match (parse_for_compare(a), parse_for_compare(b)) {
        (Some(va), Some(vb)) => compare_conda(&va, &vb),
        _ => 1,
    }
}

fn extract_leading_num(s: &str) -> Option<u64> {
    let digits: String = s.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        None
    } else {
        digits.parse().ok()
    }
}

fn get_numeric_parts(s: &str) -> Option<Vec<u64>> {
    if !is_valid_version(s) {
        return None;
    }
    let (_, rest) = extract_epoch(s);
    let nums: Vec<u64> = rest
        .split(['.', '-', '_'])
        .filter_map(extract_leading_num)
        .collect();
    if nums.is_empty() { None } else { Some(nums) }
}

fn version_has_prefix(version: &str, prefix: &str) -> bool {
    let Some(v_nums) = get_numeric_parts(version) else {
        return false;
    };
    let Some(p_nums) = get_numeric_parts(prefix) else {
        return false;
    };
    p_nums
        .iter()
        .enumerate()
        .all(|(i, &pn)| v_nums.get(i).copied().unwrap_or(0) == pn)
}

fn contains_dev_component(s: &str) -> bool {
    let (_, rest) = extract_epoch(s);
    rest.split(['.', '-', '_'])
        .flat_map(parse_component_parts)
        .any(|part| {
            if let CondaPart::Str(ref name) = part {
                name.eq_ignore_ascii_case("dev")
            } else {
                false
            }
        })
}

fn compatible_release_match(version: &str, ver_str: &str) -> bool {
    let parts: Vec<&str> = ver_str.rsplitn(2, '.').collect();
    if parts.len() < 2 {
        return false;
    }
    let prefix = parts[1];
    is_valid_version(version)
        && compare_versions_str(version, ver_str) >= 0
        && version_has_prefix(version, prefix)
}

fn parse_spec_op(s: &str) -> Option<(&str, &str)> {
    for op in ["==", "!=", "~=", ">=", "<=", ">", "<"] {
        if let Some(rest) = s.strip_prefix(op) {
            return Some((op, rest.trim()));
        }
    }
    None
}

fn matches_spec(version: &str, spec: &str) -> bool {
    if spec.is_empty() {
        return false;
    }
    if spec == "*" {
        return is_valid_version(version);
    }
    if is_glob_spec(spec) {
        let prefix = &spec[..spec.len() - 2];
        return is_valid_version(version) && version_has_prefix(version, prefix);
    }
    if let Some((op, ver_str)) = parse_spec_op(spec) {
        if let Some(prefix) = ver_str.strip_suffix(".*") {
            return op == "==" && is_valid_version(version) && version_has_prefix(version, prefix);
        }
        match op {
            "==" => {
                let (av, bv) = (parse_for_compare(version), parse_for_compare(ver_str));
                matches!((av, bv), (Some(a), Some(b)) if compare_conda(&a, &b) == 0)
            }
            ">=" => is_valid_version(version) && compare_versions_str(version, ver_str) >= 0,
            "<=" => is_valid_version(version) && compare_versions_str(version, ver_str) <= 0,
            ">" => is_valid_version(version) && compare_versions_str(version, ver_str) > 0,
            "<" => is_valid_version(version) && compare_versions_str(version, ver_str) < 0,
            "!=" => {
                let (av, bv) = (parse_for_compare(version), parse_for_compare(ver_str));
                matches!((av, bv), (Some(a), Some(b)) if compare_conda(&a, &b) != 0)
            }
            "~=" => compatible_release_match(version, ver_str),
            _ => false,
        }
    } else {
        // Bare version — exact match
        let (av, bv) = (parse_for_compare(version), parse_for_compare(spec));
        matches!((av, bv), (Some(a), Some(b)) if compare_conda(&a, &b) == 0)
    }
}

fn update_glob_for_new_version(glob: &str, new_version: &str) -> Option<String> {
    let prefix = &glob[..glob.len() - 2];
    let depth = prefix.split('.').count();
    let new_nums = get_numeric_parts(new_version)?;
    let new_prefix: Vec<String> = new_nums.iter().take(depth).map(|n| n.to_string()).collect();
    if new_prefix.len() < depth {
        return None;
    }
    Some(format!("{}.*", new_prefix.join(".")))
}

fn is_valid_spec(s: &str) -> bool {
    if s == "*" {
        return true;
    }
    if is_glob_spec(s) {
        return true;
    }
    if let Some((_, rest)) = parse_spec_op(s) {
        if rest.is_empty() {
            return false;
        }
        if rest == "*" || rest.ends_with(".*") {
            return true;
        }
        return is_valid_version(rest);
    }
    false
}

pub fn is_version(s: &str) -> bool {
    is_valid_version(s)
}

pub fn is_valid(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    is_valid_version(s) || is_valid_spec(s)
}

pub fn is_stable(s: &str) -> bool {
    if !is_valid_version(s) {
        return false;
    }
    !contains_dev_component(s)
}

pub fn equals(a: &str, b: &str) -> bool {
    match (parse_for_compare(a), parse_for_compare(b)) {
        (Some(va), Some(vb)) => compare_conda(&va, &vb) == 0,
        _ => false,
    }
}

pub fn matches(version: &str, spec: &str) -> bool {
    matches_spec(version, spec)
}

pub fn get_major(s: &str) -> Option<u64> {
    get_numeric_parts(s).and_then(|n| n.first().copied())
}

pub fn get_minor(s: &str) -> Option<u64> {
    get_numeric_parts(s).and_then(|n| n.get(1).copied())
}

pub fn get_patch(s: &str) -> Option<u64> {
    get_numeric_parts(s).map(|n| n.get(2).copied().unwrap_or(0))
}

pub fn is_single_version(s: &str) -> bool {
    if !s.starts_with('=') {
        return false;
    }
    let stripped = if s.starts_with("==") {
        &s[2..]
    } else {
        &s[1..]
    };
    let rest = stripped.trim_start();
    is_valid_version(rest)
}

pub fn is_compatible(_version: &str, _current: &str) -> bool {
    true
}

pub fn is_greater_than(a: &str, b: &str) -> bool {
    compare_versions_str(a, b) > 0
}

pub fn get_pinned_value(version: &str) -> String {
    format!("=={version}")
}

pub fn get_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    versions
        .iter()
        .filter(|&&v| matches_spec(v, range))
        .copied()
        .max_by(|&a, &b| match compare_versions_str(a, b) {
            n if n < 0 => Ordering::Less,
            n if n > 0 => Ordering::Greater,
            _ => Ordering::Equal,
        })
}

pub fn min_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    versions
        .iter()
        .filter(|&&v| matches_spec(v, range))
        .copied()
        .min_by(|&a, &b| match compare_versions_str(a, b) {
            n if n < 0 => Ordering::Less,
            n if n > 0 => Ordering::Greater,
            _ => Ordering::Equal,
        })
}

pub fn get_new_value(
    current_value: &str,
    range_strategy: &str,
    _current_version: &str,
    new_version: &str,
) -> Option<String> {
    if current_value == "*" {
        return if range_strategy == "bump" {
            Some(format!(">={new_version}"))
        } else {
            None
        };
    }
    if is_glob_spec(current_value) {
        match range_strategy {
            "widen" => {
                if matches_spec(new_version, current_value) {
                    return Some(current_value.to_owned());
                }
                update_glob_for_new_version(current_value, new_version)
            }
            _ => update_glob_for_new_version(current_value, new_version),
        }
    } else if let Some((op, _)) = parse_spec_op(current_value) {
        match (op, range_strategy) {
            (">=", "bump") => Some(format!(">={new_version}")),
            _ => None,
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isVersion("$input") === $expected" — versioning/conda/index.spec.ts line 4
    #[test]
    fn is_version_matches_renovate_conda_index_spec() {
        let cases = [
            ("0.750", true),
            ("1.2.3", true),
            ("1.0.1a", true),
            ("1.9", true),
            ("17.04.0", true),
            ("", false),
            ("==1.2.3", false),
            ("==1.2.3.0", false),
            ("==1.2.3rc0", false),
            ("~=1.2.3", false),
            ("1.2.*", false),
            (">1.2.3", false),
            ("renovatebot/renovate", false),
            ("renovatebot/renovate#master", false),
            ("https://github.com/renovatebot/renovate.git", false),
        ];
        for (input, expected) in cases {
            assert_eq!(is_version(input), expected, "is_version({input:?})");
        }
    }

    // Ported: "isValid("$input") === $expected" — versioning/conda/index.spec.ts line 26
    #[test]
    fn is_valid_matches_renovate_conda_index_spec() {
        let cases = [
            ("0.750", true),
            ("1.2.3", true),
            ("1.9", true),
            ("17.04.0", true),
            ("==1.2.3", true),
            ("==1.2.3.0", true),
            ("==1.2.3rc0", true),
            ("~=1.2.3", true),
            ("1.2.*", true),
            (">1.2.3", true),
            ("", false),
            ("renovatebot/renovate", false),
            ("renovatebot/renovate#master", false),
            ("https://github.com/renovatebot/renovate.git", false),
        ];
        for (input, expected) in cases {
            assert_eq!(is_valid(input), expected, "is_valid({input:?})");
        }
    }

    // Ported: "isStable("$input") === $expected" — versioning/conda/index.spec.ts line 47
    #[test]
    fn is_stable_matches_renovate_conda_index_spec() {
        let cases = [
            ("1.2.3", true),
            ("1.2.3rc0", true),
            ("1.2.3a", true),
            ("not./version..1", false),
        ];
        for (input, expected) in cases {
            assert_eq!(is_stable(input), expected, "is_stable({input:?})");
        }
    }

    // Ported: "equals("$a", "$b") === $expected" — versioning/conda/index.spec.ts line 57
    #[test]
    fn equals_matches_renovate_conda_index_spec() {
        let cases = [
            ("1.0", "1.0.0", true),
            ("1.0.0", "1.0.foo", false),
            ("non-pep440-1", "non-pep440-2", false),
            ("broken/version", "broken/version", false),
            ("1.0.0", "broken/version", false),
            ("broken/version", "1.0.0", false),
        ];
        for (a, b, expected) in cases {
            assert_eq!(equals(a, b), expected, "equals({a:?}, {b:?})");
        }
    }

    // Ported: "matches("$a", "$b") === $expected" — versioning/conda/index.spec.ts line 69
    #[test]
    fn matches_matches_renovate_conda_index_spec() {
        let cases = [
            ("1.0", ">=1.0.0", true),
            ("3.0.0", "==3.0.0", true),
            ("1.6.2", "<2.2.1.0", true),
            ("3.8", ">=3.9", false),
            ("not-pep440-version", "*", true),
            ("not/conda/version", "*", false),
            ("not/conda/version", "", false),
        ];
        for (a, b, expected) in cases {
            assert_eq!(matches(a, b), expected, "matches({a:?}, {b:?})");
        }
    }

    // Ported: "getMajor("$a") === $expected" — versioning/conda/index.spec.ts line 82
    #[test]
    fn get_major_matches_renovate_conda_index_spec() {
        let cases: &[(&str, Option<u64>)] = &[
            ("1.0", Some(1)),
            ("3.0.0", Some(3)),
            ("1.6.2", Some(1)),
            ("3.8", Some(3)),
            ("not-pep440-version", None),
        ];
        for &(a, expected) in cases {
            assert_eq!(get_major(a), expected, "get_major({a:?})");
        }
    }

    // Ported: "getMinor($a) === $expected" — versioning/conda/index.spec.ts line 93
    #[test]
    fn get_minor_matches_renovate_conda_index_spec() {
        let cases: &[(&str, Option<u64>)] = &[
            ("1.0", Some(0)),
            ("3.0.0", Some(0)),
            ("1.6.2", Some(6)),
            ("3.8", Some(8)),
            ("1!3.8", Some(8)),
            ("non-pep440-string", None),
        ];
        for &(a, expected) in cases {
            assert_eq!(get_minor(a), expected, "get_minor({a:?})");
        }
    }

    // Ported: "getPatch("$a") === $expected" — versioning/conda/index.spec.ts line 105
    #[test]
    fn get_patch_matches_renovate_conda_index_spec() {
        let cases: &[(&str, Option<u64>)] = &[
            ("1.0", Some(0)),
            ("3.0.0", Some(0)),
            ("1.6.2", Some(2)),
            ("3.8", Some(0)),
            ("not-pep440-version", None),
        ];
        for &(a, expected) in cases {
            assert_eq!(get_patch(a), expected, "get_patch({a:?})");
        }
    }

    // Ported: "isSingleVersion("$version") === $isSingle" — versioning/conda/index.spec.ts line 116
    #[test]
    fn is_single_version_matches_renovate_conda_index_spec() {
        let cases = [
            ("==1.2.3", true),
            ("==1.2.3rc0", true),
            ("==1.2.3", true),
            ("==1.2", true),
            ("== 1.2.3", true),
            ("==1.*", false),
            ("*", false),
            (">=1.0", false),
        ];
        for (version, expected) in cases {
            assert_eq!(
                is_single_version(version),
                expected,
                "is_single_version({version:?})"
            );
        }
    }

    // Ported: "always compatible" — versioning/conda/index.spec.ts line 131
    #[test]
    fn always_compatible_matches_renovate_conda_index_spec() {
        assert!(is_compatible("a", "b"));
    }

    // Ported: "getSatisfyingVersion($versions, "$range") === $expected" — versioning/conda/index.spec.ts line 146
    #[test]
    fn get_satisfying_version_matches_renovate_conda_index_spec() {
        let versions = &[
            "0.9.4", "1.0.0", "1.1.5", "1.2.1", "1.2.2", "1.2.3", "1.3.4", "2.0.3",
        ];
        let cases: &[(&str, Option<&str>)] = &[("~=1.2.1", Some("1.2.3")), ("~=2.1", None)];
        for &(range, expected) in cases {
            assert_eq!(
                get_satisfying_version(versions, range),
                expected,
                "get_satisfying_version({range:?})"
            );
        }
    }

    // Ported: "minSatisfyingVersion($versions, "$range") === $expected" — versioning/conda/index.spec.ts line 157
    #[test]
    fn min_satisfying_version_matches_renovate_conda_index_spec() {
        let versions = &[
            "0.9.4", "1.0.0", "1.1.5", "1.2.1", "1.2.2", "1.2.3", "1.3.4", "2.0.3",
        ];
        let cases: &[(&str, Option<&str>)] = &[("~=1.2.1", Some("1.2.1")), ("~=2.1", None)];
        for &(range, expected) in cases {
            assert_eq!(
                min_satisfying_version(versions, range),
                expected,
                "min_satisfying_version({range:?})"
            );
        }
    }

    // Ported: "isGreaterThan("$a", "$b") === $result" — versioning/conda/index.spec.ts line 168
    #[test]
    fn is_greater_than_matches_renovate_conda_index_spec() {
        let cases = [("1.2.1", "1.2.0", true), ("1!1.0.0", "3.1.2", true)];
        for (a, b, expected) in cases {
            assert_eq!(
                is_greater_than(a, b),
                expected,
                "is_greater_than({a:?}, {b:?})"
            );
        }
    }

    // Ported: "returns a pinned value" — versioning/conda/index.spec.ts line 176
    #[test]
    fn get_pinned_value_matches_renovate_conda_index_spec() {
        assert_eq!(get_pinned_value("1.2.3"), "==1.2.3");
    }

    // Ported: "getNewValue($currentValue, $rangeStrategy, $currentVersion, $newVersion) === $expected" — versioning/conda/index.spec.ts line 180
    #[test]
    fn get_new_value_matches_renovate_conda_index_spec() {
        let cases: &[(&str, &str, &str, &str, Option<&str>)] = &[
            ("*", "bump", "1.0.0", "1.2.3", Some(">=1.2.3")),
            ("*", "widen", "1.0.0", "1.2.3", None),
            ("*", "widen", "1.0.0", "1.2.3", None),
            ("1.0.*", "bump", "1.0.0", "1.2.3", Some("1.2.*")),
            ("1.2.*", "widen", "1.0.0", "1.2.3", Some("1.2.*")),
            (">=1.0.0", "bump", "1.0.0", "1.2.3", Some(">=1.2.3")),
        ];
        for &(cv, rs, cur, nv, expected) in cases {
            assert_eq!(
                get_new_value(cv, rs, cur, nv).as_deref(),
                expected,
                "get_new_value({cv:?}, {rs:?}, {cur:?}, {nv:?})"
            );
        }
    }
}
