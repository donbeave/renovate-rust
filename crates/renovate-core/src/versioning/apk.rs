//! Alpine Package Keeper (APK) versioning.
//!
//! Ports `lib/modules/versioning/apk/index.ts`.
//! APK version format: `[v]major[.minor[.patch[.extra...]]][letter][_prerelease[N]][_pkgfix[N]][-rN]`

use std::cmp::Ordering;
use std::sync::LazyLock;

use regex::Regex;

static VERSION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?x)
        ^v?
        (?P<major>[0-9]+)
        (?:\.(?P<minor>[0-9]+))?
        (?:\.(?P<patch>[0-9]+))?
        (?P<extra>(?:\.[0-9]+)*)
        (?P<letter>[a-z])?
        (?:(?P<prereleaseType>_alpha|_beta|_pre|_rc)(?P<prereleaseNum>[0-9]*))?
        (?:(?P<packageFixType>_cvs|_svn|_git|_hg|_p)(?P<packageFixNum>[0-9]*))?
        (?:-r(?P<releaseNum>[0-9]+))?
        $",
    )
    .unwrap()
});

static ALPHA_NUM_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([a-zA-Z]+)|(\d+)").unwrap());

static OPERATOR_STRIP_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[=><~][=]?").unwrap());

static RANGE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^([><=~]+)(.+)$").unwrap());

static REVISION_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"-r[0-9]+$").unwrap());

#[derive(Debug, Clone)]
struct ParsedApk {
    version: String,
    release: Vec<u64>,
    prerelease: Option<String>,
    release_string: String,
}

fn parse(version: &str) -> Option<ParsedApk> {
    let caps = VERSION_RE.captures(version)?;

    let major = caps.name("major")?.as_str();
    let minor = caps.name("minor").map(|m| m.as_str());
    let patch = caps.name("patch").map(|m| m.as_str());
    let extra = caps.name("extra").map(|m| m.as_str()).unwrap_or("");
    let letter = caps.name("letter").map(|m| m.as_str()).unwrap_or("");
    let prerelease_type = caps.name("prereleaseType").map(|m| m.as_str());
    let prerelease_num = caps.name("prereleaseNum").map(|m| m.as_str()).unwrap_or("");
    let package_fix_type = caps.name("packageFixType").map(|m| m.as_str());
    let package_fix_num = caps.name("packageFixNum").map(|m| m.as_str()).unwrap_or("");
    let release_num = caps.name("releaseNum").map(|m| m.as_str());

    let package_fix_full = match package_fix_type {
        Some(t) => format!("{t}{package_fix_num}"),
        None => String::new(),
    };

    let minor_patch_str = {
        let mut s = String::new();
        if let Some(min) = minor {
            s.push('.');
            s.push_str(min);
        }
        if let Some(pat) = patch {
            s.push('.');
            s.push_str(pat);
        }
        s.push_str(extra);
        s
    };

    let version_str = format!("{major}{minor_patch_str}{letter}{package_fix_full}");

    let prerelease = prerelease_type.map(|t| {
        // strip leading underscore, append num
        format!("{}{prerelease_num}", &t[1..])
    });

    let mut release: Vec<u64> = vec![major.parse().ok()?];
    if let Some(min) = minor {
        release.push(min.parse().ok()?);
    }
    if let Some(pat) = patch {
        release.push(pat.parse().ok()?);
    }
    if !extra.is_empty() {
        for part in extra[1..].split('.').filter(|s| !s.is_empty()) {
            if let Ok(n) = part.parse::<u64>() {
                release.push(n);
            }
        }
    }

    let release_string = release_num.unwrap_or("").to_owned();

    Some(ParsedApk {
        version: version_str,
        release,
        prerelease,
        release_string,
    })
}

fn compare_version_parts(v1: &str, v2: &str) -> i32 {
    if v1 == v2 {
        return 0;
    }

    let parts1: Vec<&str> = ALPHA_NUM_RE.find_iter(v1).map(|m| m.as_str()).collect();
    let parts2: Vec<&str> = ALPHA_NUM_RE.find_iter(v2).map(|m| m.as_str()).collect();
    let min_len = parts1.len().min(parts2.len());

    for i in 0..min_len {
        let p1 = parts1[i];
        let p2 = parts2[i];

        let p1_is_num = p1.chars().all(|c| c.is_ascii_digit());
        let p2_is_num = p2.chars().all(|c| c.is_ascii_digit());

        if p1_is_num {
            if !p2_is_num {
                return 1;
            }
            let n1: u64 = p1.parse().unwrap_or(0);
            let n2: u64 = p2.parse().unwrap_or(0);
            if n1 != n2 {
                return if n1 > n2 { 1 } else { -1 };
            }
        } else if p2_is_num {
            return -1;
        } else if p1 != p2 {
            return match p1.cmp(p2) {
                Ordering::Less => -1,
                Ordering::Greater => 1,
                Ordering::Equal => 0,
            };
        }
    }

    if parts1.len() != parts2.len() {
        let max_len = parts1.len().max(parts2.len());
        for i in min_len..max_len {
            let p1 = parts1.get(i);
            let p2 = parts2.get(i);
            match (p1, p2) {
                (Some(p1), None) => {
                    if p1.chars().all(|c| c.is_ascii_digit()) {
                        return 1;
                    } else {
                        return -1;
                    }
                }
                (None, Some(p2)) => {
                    if p2.chars().all(|c| c.is_ascii_digit()) {
                        return -1;
                    } else {
                        return 1;
                    }
                }
                _ => {}
            }
        }
    }

    0
}

fn compare(a: &str, b: &str) -> i32 {
    let p1 = parse(a);
    let p2 = parse(b);

    match (p1, p2) {
        (Some(p1), Some(p2)) => {
            let vc = compare_version_parts(&p1.version, &p2.version);
            if vc != 0 {
                return vc;
            }

            let pre1 = &p1.prerelease;
            let pre2 = &p2.prerelease;

            if pre1.is_some() || pre2.is_some() {
                if pre1.is_none() {
                    return -1;
                }
                if pre2.is_none() {
                    return 1;
                }
                let pc = pre1.as_deref().unwrap().cmp(pre2.as_deref().unwrap());
                if pc != Ordering::Equal {
                    let pc_i: i32 =
                        compare_version_parts(pre1.as_deref().unwrap(), pre2.as_deref().unwrap());
                    if pc_i != 0 {
                        return pc_i;
                    }
                }
            }

            let r1 = &p1.release_string;
            let r2 = &p2.release_string;

            if !r1.is_empty() && r2.is_empty() {
                return 1;
            }
            if r1.is_empty() && !r2.is_empty() {
                return -1;
            }

            let r1_str = if r1.is_empty() { "0" } else { r1.as_str() };
            let r2_str = if r2.is_empty() { "0" } else { r2.as_str() };

            compare_version_parts(r1_str, r2_str)
        }
        _ => 1,
    }
}

fn strip_operator(version: &str) -> &str {
    OPERATOR_STRIP_RE
        .find(version)
        .map_or(version, |m| &version[m.end()..])
}

pub fn is_valid(version: &str) -> bool {
    if version.is_empty() {
        return false;
    }
    let clean = strip_operator(version);
    parse(clean).is_some()
}

pub fn is_single_version(version: &str) -> bool {
    if version.is_empty() {
        return false;
    }
    if version.starts_with(['>', '<', '~']) {
        return false;
    }
    is_valid(version)
}

pub fn is_stable(version: &str) -> bool {
    if version.is_empty() {
        return false;
    }
    let clean = strip_operator(version);
    parse(clean).is_some_and(|p| p.prerelease.is_none())
}

pub fn get_major(version: &str) -> Option<i64> {
    if version.is_empty() {
        return None;
    }
    let clean = strip_operator(version);
    parse(clean).and_then(|p| p.release.first().map(|&n| n as i64))
}

pub fn get_minor(version: &str) -> Option<i64> {
    if version.is_empty() {
        return None;
    }
    let clean = strip_operator(version);
    parse(clean).and_then(|p| p.release.get(1).map(|&n| n as i64))
}

pub fn get_patch(version: &str) -> Option<i64> {
    if version.is_empty() {
        return None;
    }
    let clean = strip_operator(version);
    parse(clean).and_then(|p| p.release.get(2).map(|&n| n as i64))
}

pub fn sort_versions(a: &str, b: &str) -> i32 {
    let clean_a = a.strip_prefix('=').unwrap_or(a);
    let clean_b = b.strip_prefix('=').unwrap_or(b);
    compare(clean_a, clean_b)
}

pub fn equals(a: &str, b: &str) -> bool {
    sort_versions(a, b) == 0
}

pub fn is_greater_than(a: &str, b: &str) -> bool {
    sort_versions(a, b) > 0
}

pub fn is_less_than_range(version: &str, range: &str) -> bool {
    sort_versions(version, range) < 0
}

pub fn get_satisfying_version(versions: &[&str], range: &str) -> Option<String> {
    // Try range operator
    if let Some(caps) = RANGE_RE.captures(range) {
        let operator = caps.get(1)?.as_str();
        let target = caps.get(2)?.as_str();

        if !is_valid(target) && operator != "=" && operator != "==" {
            // For operators that need valid target, return null if target invalid
            // But = / == also need valid target to match, so check below
        }

        let mut satisfying: Vec<&str> = versions
            .iter()
            .copied()
            .filter(|v| {
                if !is_valid(v) || !is_valid(target) {
                    return false;
                }
                match operator {
                    ">" => is_greater_than(v, target),
                    ">=" => is_greater_than(v, target) || equals(v, target),
                    "<" => is_less_than_range(v, target),
                    "<=" => is_less_than_range(v, target) || equals(v, target),
                    "=" | "==" => equals(v, target),
                    "~" => {
                        let tv = parse(target);
                        let vv = parse(v);
                        match (tv, vv) {
                            (Some(tp), Some(vp)) => {
                                let t_maj = tp.release.first().copied().unwrap_or(0);
                                let t_min = tp.release.get(1).copied().unwrap_or(0);
                                let v_maj = vp.release.first().copied().unwrap_or(0);
                                let v_min = vp.release.get(1).copied().unwrap_or(0);
                                if t_maj != v_maj || t_min != v_min {
                                    return false;
                                }
                                is_greater_than(v, target) || equals(v, target)
                            }
                            _ => false,
                        }
                    }
                    _ => false,
                }
            })
            .collect();

        if satisfying.is_empty() {
            return None;
        }

        satisfying.sort_by(|a, b| {
            let cmp = sort_versions(b, a); // reverse for max
            if cmp < 0 {
                Ordering::Less
            } else if cmp > 0 {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        return satisfying.first().map(|s| (*s).to_owned());
    }

    // Exact match fallback
    versions
        .iter()
        .find(|&&v| equals(v, range))
        .map(|s| (*s).to_owned())
}

pub fn get_new_value(current_value: &str, new_version: &str) -> String {
    let has_revision = REVISION_RE.is_match(current_value);
    if has_revision {
        new_version.to_owned()
    } else {
        REVISION_RE.replace(new_version, "").into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isValid($version) === $expected" — lib/modules/versioning/apk/index.spec.ts line 5
    #[test]
    fn is_valid_matches_renovate_apk_index_spec() {
        assert!(is_valid("2.39.0-r0"));
        assert!(is_valid("2.39.0"));
        assert!(is_valid("2.39.0_rc1"));
        assert!(!is_valid("foo"));
        assert!(!is_valid("a.39.0-"));
        assert!(is_valid("6.5_p20250503-r0"));
    }

    // Ported: "isStable($version) === $expected" — lib/modules/versioning/apk/index.spec.ts line 19
    #[test]
    fn is_stable_matches_renovate_apk_index_spec() {
        assert!(is_stable("2.39.0-r0"));
        assert!(!is_stable("2.39.0_rc1-r0"));
        assert!(is_stable("2.39.0"));
        assert!(is_stable("2.39.0-r0"));
        assert!(!is_stable("2.39.0_rc2"));
        assert!(!is_stable("2.39.0_rc10-r0"));
        assert!(!is_stable("2.39.0_rc1"));
        assert!(!is_stable("2.39.0_rc0"));
        assert!(is_stable("6.5_p20250503-r0"));
        assert!(is_stable("1.0_p1-r0"));
        assert!(is_stable("2.0_cvs-r0"));
        assert!(is_stable("3.0_git-r0"));
        assert!(!is_stable("4.0_alpha-r0"));
        assert!(!is_stable("5.0_beta-r0"));
    }

    // Ported: "getMajor($version) === $expected" — lib/modules/versioning/apk/index.spec.ts line 41
    #[test]
    fn get_major_matches_renovate_apk_index_spec() {
        assert_eq!(get_major("2.39.0-r0"), Some(2));
        assert_eq!(get_major("2.39.0_rc1-r0"), Some(2));
    }

    // Ported: "getMinor($version) === $expected" — lib/modules/versioning/apk/index.spec.ts line 51
    #[test]
    fn get_minor_matches_renovate_apk_index_spec() {
        assert_eq!(get_minor("2.39.0-r0"), Some(39));
        assert_eq!(get_minor("2.39.0_rc1-r0"), Some(39));
    }

    // Ported: "getPatch($version) === $expected" — lib/modules/versioning/apk/index.spec.ts line 61
    #[test]
    fn get_patch_matches_renovate_apk_index_spec() {
        assert_eq!(get_patch("2.39.0-r0"), Some(0));
        assert_eq!(get_patch("2.39.0_rc1-r0"), Some(0));
        assert_eq!(get_patch("6.5_p20250503-r0"), None);
        assert_eq!(get_patch("3.9_pre20060124"), None);
        assert_eq!(get_patch("0.3.4_pre20061029"), Some(4));
    }

    // Ported: "compare($a, $b) === $expected" — lib/modules/versioning/apk/index.spec.ts line 74
    #[test]
    fn compare_matches_renovate_apk_index_spec() {
        assert_eq!(sort_versions("2.39.0-r1", "2.39.0-r0").signum(), 1);
        assert_eq!(sort_versions("2.39.1-r0", "2.39.0-r0").signum(), 1);
        assert_eq!(sort_versions("2.39.0-r0", "2.39.1-r0").signum(), -1);
        assert_eq!(sort_versions("2.39.0-r0", "2.39.0-r1").signum(), -1);
        assert_eq!(sort_versions("2.39.0", "2.39.0").signum(), 0);
        assert_eq!(sort_versions("2.39.0", "2.39.1").signum(), -1);
        assert_eq!(sort_versions("2.39.1", "2.39.0").signum(), 1);
        assert_eq!(sort_versions("2.39.0-r0", "2.39.0").signum(), 1);
        assert_eq!(sort_versions("2.39.0", "2.39.0-r0").signum(), -1);
        assert_eq!(sort_versions("2.39.0_beta", "2.39.0").signum(), 1);
        assert_eq!(sort_versions("2.39.0", "2.39.0_beta").signum(), -1);
        assert_eq!(
            sort_versions("0.3.4_pre20061029", "0.3.4_pre20061030").signum(),
            -1
        );
        assert_eq!(
            sort_versions("0.3.4_pre20061029", "0.3.4_pre20061028").signum(),
            1
        );
        assert_eq!(
            sort_versions("0.3.4_pre20061029", "0.3.4_alpha").signum(),
            1
        );
        assert_eq!(
            sort_versions("0.3.4_alpha", "0.3.4_pre20061029").signum(),
            -1
        );
        assert_eq!(sort_versions("0.3.4_pre20061029", "0.4.0").signum(), -1);
        assert_eq!(sort_versions("0.4.0", "0.3.4_pre20061029").signum(), 1);
        assert_eq!(
            sort_versions("2.9.11_pre20061021-r2", "5.36-r1").signum(),
            -1
        );
        assert_eq!(sort_versions("0.3.4_alpha", "0.3.4_beta").signum(), -1);
        assert_eq!(sort_versions("0.3.4_beta", "0.3.4_alpha").signum(), 1);
    }

    // Ported: "isGreaterThan($a, $b) === $expected" — lib/modules/versioning/apk/index.spec.ts line 102
    #[test]
    fn is_greater_than_matches_renovate_apk_index_spec() {
        assert!(is_greater_than("2.39.1-r0", "2.39.0-r0"));
        assert!(is_greater_than("2.39.0-r1", "2.39.0-r0"));
        assert!(!is_greater_than("2.39.0-r0", "2.39.1-r0"));
        assert!(!is_greater_than("2.39.0-r0", "2.39.0-r1"));
        assert!(is_greater_than("1.4_p12-r5", "1.4_p12-r2"));
    }

    // Ported: "equals($a, $b) === $expected" — lib/modules/versioning/apk/index.spec.ts line 115
    #[test]
    fn equals_matches_renovate_apk_index_spec() {
        assert!(equals("2.39.0-r0", "2.39.0-r0"));
        assert!(equals("2.39.0", "2.39.0"));
        assert!(!equals("2.39.0-r0", "2.39.0-r1"));
        assert!(!equals("2.39.0", "2.39.1"));
    }

    // Ported: "getSatisfyingVersion with exact match ($range) === $expected" — lib/modules/versioning/apk/index.spec.ts line 136
    #[test]
    fn get_satisfying_version_exact_matches_renovate_apk_index_spec() {
        let versions = &[
            "2.39.0-r0",
            "2.39.0-r1",
            "2.39.1-r0",
            "2.40.0-r0",
            "2.40.0-r1",
            "3.0.0-r0",
        ];
        assert_eq!(
            get_satisfying_version(versions, "2.39.0-r0").as_deref(),
            Some("2.39.0-r0")
        );
        assert_eq!(
            get_satisfying_version(versions, "2.39.0-r1").as_deref(),
            Some("2.39.0-r1")
        );
        assert_eq!(
            get_satisfying_version(versions, "2.40.0-r0").as_deref(),
            Some("2.40.0-r0")
        );
        assert_eq!(get_satisfying_version(versions, "nonexistent"), None);
    }

    // Ported: "getSatisfyingVersion with range operator ($range) === $expected" — lib/modules/versioning/apk/index.spec.ts line 149
    #[test]
    fn get_satisfying_version_range_operator_matches_renovate_apk_index_spec() {
        let versions = &[
            "2.39.0-r0",
            "2.39.0-r1",
            "2.39.1-r0",
            "2.40.0-r0",
            "2.40.0-r1",
            "3.0.0-r0",
        ];
        assert_eq!(
            get_satisfying_version(versions, ">2.39.0-r0").as_deref(),
            Some("3.0.0-r0")
        );
        assert_eq!(
            get_satisfying_version(versions, ">=2.39.0-r0").as_deref(),
            Some("3.0.0-r0")
        );
        assert_eq!(
            get_satisfying_version(versions, "<2.40.0-r0").as_deref(),
            Some("2.39.1-r0")
        );
        assert_eq!(
            get_satisfying_version(versions, "<=2.40.0-r0").as_deref(),
            Some("2.40.0-r0")
        );
        assert_eq!(
            get_satisfying_version(versions, "=2.39.0-r0").as_deref(),
            Some("2.39.0-r0")
        );
        assert_eq!(
            get_satisfying_version(versions, "==2.39.0-r0").as_deref(),
            Some("2.39.0-r0")
        );
    }

    // Ported: "getSatisfyingVersion with tilde range ($range) === $expected" — lib/modules/versioning/apk/index.spec.ts line 164
    #[test]
    fn get_satisfying_version_tilde_matches_renovate_apk_index_spec() {
        let versions = &[
            "2.39.0-r0",
            "2.39.0-r1",
            "2.39.1-r0",
            "2.40.0-r0",
            "2.40.0-r1",
            "3.0.0-r0",
        ];
        assert_eq!(
            get_satisfying_version(versions, "~2.39.0-r0").as_deref(),
            Some("2.39.1-r0")
        );
        assert_eq!(
            get_satisfying_version(versions, "~2.40.0-r0").as_deref(),
            Some("2.40.0-r1")
        );
    }

    // Ported: "should return null for invalid range operators" — lib/modules/versioning/apk/index.spec.ts line 175
    #[test]
    fn get_satisfying_version_null_invalid_range_matches_renovate_apk_index_spec() {
        let versions = &[
            "2.39.0-r0",
            "2.39.0-r1",
            "2.39.1-r0",
            "2.40.0-r0",
            "2.40.0-r1",
            "3.0.0-r0",
        ];
        assert_eq!(get_satisfying_version(versions, "invalid-range"), None);
    }

    // Ported: "should return null for empty versions array" — lib/modules/versioning/apk/index.spec.ts line 179
    #[test]
    fn get_satisfying_version_null_empty_array_matches_renovate_apk_index_spec() {
        assert_eq!(get_satisfying_version(&[], "2.39.0-r0"), None);
    }

    // Ported: "should filter out invalid versions" — lib/modules/versioning/apk/index.spec.ts line 183
    #[test]
    fn get_satisfying_version_filter_invalid_matches_renovate_apk_index_spec() {
        let versions = &["2.39.0-r0", "invalid", "2.40.0-r0"];
        assert_eq!(
            get_satisfying_version(versions, ">2.39.0-r0").as_deref(),
            Some("2.40.0-r0")
        );
    }

    // Ported: "isSingleVersion($version) === $expected" — lib/modules/versioning/apk/index.spec.ts line 192
    #[test]
    fn is_single_version_matches_renovate_apk_index_spec() {
        assert!(is_single_version("2.39.0-r0"));
        assert!(is_single_version("2.39.0"));
        assert!(!is_single_version("~2.39.0-r0"));
        assert!(!is_single_version(">2.39.0-r0"));
    }

    // Ported: "should return false for empty versions" — lib/modules/versioning/apk/index.spec.ts line 202
    #[test]
    fn is_single_version_empty_matches_renovate_apk_index_spec() {
        assert!(!is_single_version(""));
    }

    // Ported: "isLessThanRange($version, $range) === $expected" — lib/modules/versioning/apk/index.spec.ts line 210
    #[test]
    fn is_less_than_range_matches_renovate_apk_index_spec() {
        assert!(is_less_than_range("2.39.0-r0", "2.39.0-r1"));
        assert!(!is_less_than_range("2.39.0-r1", "2.39.0-r0"));
        assert!(!is_less_than_range("2.39.0-r0", "2.39.0-r0"));
        assert!(is_less_than_range("2.38.0-r0", "2.39.0-r0"));
    }

    // Ported: "should sort versions correctly" — lib/modules/versioning/apk/index.spec.ts line 225
    #[test]
    fn sort_versions_sort_correctly_matches_renovate_apk_index_spec() {
        let mut versions = vec!["2.40.0-r0", "2.39.0-r1", "2.39.0-r0", "2.39.1-r0"];
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
            vec!["2.39.0-r0", "2.39.0-r1", "2.39.1-r0", "2.40.0-r0"]
        );
    }

    // Ported: "should compare release numbers when version parts are equal" — lib/modules/versioning/apk/index.spec.ts line 236
    #[test]
    fn sort_versions_release_numbers_matches_renovate_apk_index_spec() {
        assert!(sort_versions("2.39.0-r1", "2.39.0-r2") < 0);
        assert!(sort_versions("2.39.0-r2", "2.39.0-r1") > 0);
        assert_eq!(sort_versions("2.39.0", "2.39.0"), 0);
    }

    // Ported: "should parse complex versions ($version) === $expected" — lib/modules/versioning/apk/index.spec.ts line 246
    #[test]
    fn complex_version_parsing_is_valid_matches_renovate_apk_index_spec() {
        assert!(is_valid("v2.39.0-r0"));
        assert!(is_valid("2.39.0_rc1"));
        assert!(is_valid("2.39.0_beta"));
        assert!(is_valid("6.5_p20250503-r0"));
        assert!(is_valid("2.39.0_cvs-r0"));
        assert!(is_valid("2.39.0_git-r0"));
    }

    // Ported: "should identify stable versions ($version) === $expected" — lib/modules/versioning/apk/index.spec.ts line 261
    #[test]
    fn complex_version_parsing_is_stable_matches_renovate_apk_index_spec() {
        assert!(is_stable("v2.39.0-r0"));
        assert!(!is_stable("2.39.0_rc1"));
        assert!(!is_stable("2.39.0_beta"));
        assert!(is_stable("6.5_p20250503-r0"));
        assert!(is_stable("2.39.0_cvs-r0"));
        assert!(is_stable("2.39.0_git-r0"));
    }

    // Ported: "should compare versions with prerelease identifiers ($a, $b) === $expected" — lib/modules/versioning/apk/index.spec.ts line 278
    #[test]
    fn version_comparison_edge_cases_matches_renovate_apk_index_spec() {
        assert_eq!(sort_versions("2.39.0-r0", "2.39.0").signum(), 1);
        assert_eq!(sort_versions("2.39.0", "2.39.0-r0").signum(), -1);
        assert_eq!(sort_versions("2.39.0_beta", "2.39.0").signum(), 1);
        assert_eq!(sort_versions("2.39.0", "2.39.0_beta").signum(), -1);
        assert_eq!(
            sort_versions("2.39.0_rc1-r0", "2.39.0_alpha-r0").signum(),
            1
        );
        assert_eq!(
            sort_versions("2.39.0_alpha-r0", "2.39.0_rc1-r0").signum(),
            -1
        );
    }

    // Ported: "should handle invalid version parsing gracefully" — lib/modules/versioning/apk/index.spec.ts line 295
    #[test]
    fn error_handling_invalid_gracefully_matches_renovate_apk_index_spec() {
        assert!(!is_valid(""));
        assert!(!is_valid("invalid"));
        assert!(!is_valid("a.39.0-"));
        assert_eq!(get_major("invalid"), None);
        assert_eq!(get_minor("invalid"), None);
        assert_eq!(get_patch("invalid"), None);
        assert!(!is_stable("invalid"));
    }

    // Ported: "should handle null/undefined inputs" — lib/modules/versioning/apk/index.spec.ts line 305
    #[test]
    fn error_handling_null_inputs_matches_renovate_apk_index_spec() {
        // Rust has no null/undefined; we test empty string behavior
        assert!(!is_valid(""));
        assert_eq!(get_major(""), None);
        assert_eq!(get_minor(""), None);
        assert_eq!(get_patch(""), None);
    }

    // Ported: "should return false for unstable versions with prerelease" — lib/modules/versioning/apk/index.spec.ts line 315
    #[test]
    fn error_handling_unstable_prerelease_matches_renovate_apk_index_spec() {
        assert!(!is_stable("=2.39.0_rc1-r0"));
        assert!(!is_stable(">2.39.0_beta-r0"));
        assert!(!is_stable("~2.39.0_alpha-r0"));
    }

    // Ported: "should return false for empty versions in isStable" — lib/modules/versioning/apk/index.spec.ts line 321
    #[test]
    fn error_handling_empty_is_stable_matches_renovate_apk_index_spec() {
        assert!(!is_stable(""));
    }

    // Ported: "should handle versions with different major versions in tilde range" — lib/modules/versioning/apk/index.spec.ts line 329
    #[test]
    fn get_satisfying_version_tilde_major_matches_renovate_apk_index_spec() {
        let versions = &["1.0.0-r0", "2.0.0-r0", "2.1.0-r0"];
        assert_eq!(
            get_satisfying_version(versions, "~1.0.0-r0").as_deref(),
            Some("1.0.0-r0")
        );
        assert_eq!(
            get_satisfying_version(versions, "~2.0.0-r0").as_deref(),
            Some("2.0.0-r0")
        );
    }

    // Ported: "should handle versions with different minor versions in tilde range" — lib/modules/versioning/apk/index.spec.ts line 335
    #[test]
    fn get_satisfying_version_tilde_minor_matches_renovate_apk_index_spec() {
        let versions = &["2.0.0-r0", "2.1.0-r0", "2.2.0-r0", "3.0.0-r0"];
        assert_eq!(
            get_satisfying_version(versions, "~2.1.0-r0").as_deref(),
            Some("2.1.0-r0")
        );
    }

    // Ported: "should handle invalid target versions in ranges" — lib/modules/versioning/apk/index.spec.ts line 340
    #[test]
    fn get_satisfying_version_invalid_target_matches_renovate_apk_index_spec() {
        let versions = &["2.39.0-r0", "2.40.0-r0"];
        assert_eq!(get_satisfying_version(versions, ">invalid"), None);
        assert_eq!(get_satisfying_version(versions, "~invalid"), None);
    }

    // Ported: "should handle versions with prerelease identifiers in ranges" — lib/modules/versioning/apk/index.spec.ts line 346
    #[test]
    fn get_satisfying_version_prerelease_ranges_matches_renovate_apk_index_spec() {
        let versions = &["2.39.0-r0", "2.39.0_rc1-r0", "2.40.0-r0"];
        assert_eq!(
            get_satisfying_version(versions, ">2.39.0-r0").as_deref(),
            Some("2.40.0-r0")
        );
        assert_eq!(
            get_satisfying_version(versions, ">=2.39.0_rc1-r0").as_deref(),
            Some("2.40.0-r0")
        );
    }

    // Ported: "should return null for versions with _p package fix suffix" — lib/modules/versioning/apk/index.spec.ts line 358
    #[test]
    fn get_patch_edge_cases_p_suffix_matches_renovate_apk_index_spec() {
        assert_eq!(get_patch("6.5_p20250503-r0"), None);
        assert_eq!(get_patch("1.0_p1-r0"), None);
    }

    // Ported: "should return null for invalid versions" — lib/modules/versioning/apk/index.spec.ts line 364
    #[test]
    fn get_patch_edge_cases_invalid_matches_renovate_apk_index_spec() {
        assert_eq!(get_patch("invalid"), None);
        assert_eq!(get_patch("2.0_package-r0"), None);
    }

    // Ported: "should return patch version for non-_p patterns" — lib/modules/versioning/apk/index.spec.ts line 370
    #[test]
    fn get_patch_edge_cases_non_p_matches_renovate_apk_index_spec() {
        assert_eq!(get_patch("2.39.0-r0"), Some(0));
        assert_eq!(get_patch("2.39.1-r0"), Some(1));
        assert_eq!(get_patch("2.39.0_rc1-r0"), Some(0));
    }

    // Ported: "should handle versions with operators" — lib/modules/versioning/apk/index.spec.ts line 376
    #[test]
    fn get_patch_edge_cases_operators_matches_renovate_apk_index_spec() {
        assert_eq!(get_patch("=2.39.0-r0"), Some(0));
        assert_eq!(get_patch(">2.39.1-r0"), Some(1));
        assert_eq!(get_patch("~2.39.2-r0"), Some(2));
    }

    // Ported: "should strip revision from newVersion when currentValue has no revision" — lib/modules/versioning/apk/index.spec.ts line 384
    #[test]
    fn get_new_value_strip_revision_matches_renovate_apk_index_spec() {
        assert_eq!(get_new_value("2.50.0", "2.51.1-r1"), "2.51.1");
    }

    // Ported: "should keep revision in newVersion when currentValue has revision" — lib/modules/versioning/apk/index.spec.ts line 394
    #[test]
    fn get_new_value_keep_revision_matches_renovate_apk_index_spec() {
        assert_eq!(get_new_value("2.50.0-r0", "2.51.1-r1"), "2.51.1-r1");
    }

    // Ported: "should handle newVersion without revision when currentValue has no revision" — lib/modules/versioning/apk/index.spec.ts line 404
    #[test]
    fn get_new_value_no_revision_matches_renovate_apk_index_spec() {
        assert_eq!(get_new_value("2.50.0", "2.51.1"), "2.51.1");
    }

    // Ported: "should handle newVersion without revision when currentValue has revision" — lib/modules/versioning/apk/index.spec.ts line 414
    #[test]
    fn get_new_value_has_revision_no_new_revision_matches_renovate_apk_index_spec() {
        assert_eq!(get_new_value("2.50.0-r0", "2.51.1"), "2.51.1");
    }

    // Ported: "should handle complex prerelease identifier comparisons" — lib/modules/versioning/apk/index.spec.ts line 426
    #[test]
    fn version_comparison_prerelease_complex_matches_renovate_apk_index_spec() {
        assert!(sort_versions("2.39.0_alpha-r0", "2.39.0_beta-r0") < 0);
        assert!(sort_versions("2.39.0_beta-r0", "2.39.0_alpha-r0") > 0);
        assert!(sort_versions("2.39.0_rc1-r0", "2.39.0_rc2-r0") < 0);
    }

    // Ported: "should handle versions with different prerelease patterns" — lib/modules/versioning/apk/index.spec.ts line 438
    #[test]
    fn version_comparison_prerelease_patterns_matches_renovate_apk_index_spec() {
        assert!(sort_versions("2.39.0-r0", "2.39.0_rc1-r0") < 0);
        assert!(sort_versions("2.39.0_rc1-r0", "2.39.0-r0") > 0);
    }

    // Ported: "should handle unknown range operators" — lib/modules/versioning/apk/index.spec.ts line 445
    #[test]
    fn get_satisfying_version_unknown_operators_matches_renovate_apk_index_spec() {
        let versions = &["2.39.0-r0", "2.40.0-r0"];
        assert_eq!(get_satisfying_version(versions, "!2.39.0-r0"), None);
        assert_eq!(get_satisfying_version(versions, "?2.39.0-r0"), None);
        assert_eq!(get_satisfying_version(versions, "*2.39.0-r0"), None);
        assert_eq!(get_satisfying_version(versions, "@2.39.0-r0"), None);
        assert_eq!(get_satisfying_version(versions, "#2.39.0-r0"), None);
    }

    // Ported: "should handle unhandled range operators that match regex" — lib/modules/versioning/apk/index.spec.ts line 456
    #[test]
    fn get_satisfying_version_unhandled_operators_matches_renovate_apk_index_spec() {
        let versions = &["2.39.0-r0", "2.40.0-r0"];
        assert_eq!(get_satisfying_version(versions, ">>2.39.0-r0"), None);
        assert_eq!(get_satisfying_version(versions, "<>2.39.0-r0"), None);
        assert_eq!(get_satisfying_version(versions, "><2.39.0-r0"), None);
        assert_eq!(get_satisfying_version(versions, "~~~2.39.0-r0"), None);
    }

    // Ported: "should handle tilde range with invalid target version" — lib/modules/versioning/apk/index.spec.ts line 467
    #[test]
    fn get_satisfying_version_tilde_invalid_target_matches_renovate_apk_index_spec() {
        let versions = &["2.39.0-r0", "2.40.0-r0"];
        assert_eq!(get_satisfying_version(versions, "~invalid"), None);
    }

    // Ported: "should handle tilde range with invalid version in list" — lib/modules/versioning/apk/index.spec.ts line 474
    #[test]
    fn get_satisfying_version_tilde_invalid_in_list_matches_renovate_apk_index_spec() {
        let versions = &["2.39.0-r0", "invalid", "2.40.0-r0"];
        assert_eq!(
            get_satisfying_version(versions, "~2.39.0-r0").as_deref(),
            Some("2.39.0-r0")
        );
    }

    // Ported: "should handle major-only versions without minor/patch" — lib/modules/versioning/apk/index.spec.ts line 485
    #[test]
    fn version_comparison_major_only_matches_renovate_apk_index_spec() {
        assert!(is_valid("1"));
        assert!(is_valid("42"));
        assert!(sort_versions("1", "2") < 0);
        assert!(sort_versions("2", "1") > 0);
        assert_eq!(sort_versions("1", "1"), 0);
    }

    // Ported: "should handle letter vs number at same position in version parts" — lib/modules/versioning/apk/index.spec.ts line 494
    #[test]
    fn version_comparison_letter_vs_number_matches_renovate_apk_index_spec() {
        assert!(sort_versions("1a", "1.1") < 0);
        assert!(sort_versions("1.1", "1a") > 0);
    }

    // Ported: "should handle number vs letter comparison in version parts" — lib/modules/versioning/apk/index.spec.ts line 499
    #[test]
    fn version_comparison_number_vs_letter_matches_renovate_apk_index_spec() {
        assert!(sort_versions("2.39.0.1-r0", "2.39.0a-r0") > 0);
        assert!(sort_versions("2.39.0a-r0", "2.39.0.1-r0") < 0);
    }

    // Ported: "should handle extra numeric parts in remaining segments" — lib/modules/versioning/apk/index.spec.ts line 504
    #[test]
    fn version_comparison_extra_numeric_matches_renovate_apk_index_spec() {
        assert!(sort_versions("2.39.0.1-r0", "2.39.0-r0") > 0);
        assert!(sort_versions("2.39.0-r0", "2.39.0.1-r0") < 0);
    }

    // Ported: "should handle lexicographic string comparison in version parts" — lib/modules/versioning/apk/index.spec.ts line 509
    #[test]
    fn version_comparison_lexicographic_matches_renovate_apk_index_spec() {
        assert!(sort_versions("2.39.0a-r0", "2.39.0b-r0") < 0);
        assert!(sort_versions("2.39.0b-r0", "2.39.0a-r0") > 0);
    }

    // Ported: "should handle equal letter parts continuing to next segment" — lib/modules/versioning/apk/index.spec.ts line 514
    #[test]
    fn version_comparison_equal_letter_continues_matches_renovate_apk_index_spec() {
        assert!(sort_versions("1.0a_p1-r0", "1.0a_p2-r0") < 0);
        assert!(sort_versions("1.0a_p2-r0", "1.0a_p1-r0") > 0);
    }

    // Ported: "should handle trailing letter in remaining segments" — lib/modules/versioning/apk/index.spec.ts line 519
    #[test]
    fn version_comparison_trailing_letter_matches_renovate_apk_index_spec() {
        assert!(sort_versions("1", "1a") > 0);
        assert!(sort_versions("1a", "1") < 0);
    }

    // Ported: "should return 0 for numerically equal but string-different versions" — lib/modules/versioning/apk/index.spec.ts line 524
    #[test]
    fn version_comparison_numeric_equal_string_diff_matches_renovate_apk_index_spec() {
        assert_eq!(sort_versions("1.0", "1.00"), 0);
    }

    // Ported: "should handle versions with different extra segment lengths" — lib/modules/versioning/apk/index.spec.ts line 528
    #[test]
    fn version_comparison_extra_segments_matches_renovate_apk_index_spec() {
        assert!(sort_versions("2.39.0.1.2", "2.39.0.1") > 0);
        assert!(sort_versions("2.39.0.1", "2.39.0.1.2") < 0);
    }
}
