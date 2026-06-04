//! @parity lib/modules/versioning/rpm/index.ts full
//!
//! RPM versioning.
//!
//! Renovate reference:
//! - `lib/modules/versioning/rpm/index.ts`

use std::cmp::Ordering;
use std::sync::LazyLock;

use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct RpmVersion {
    epoch: u64,
    upstream_version: String,
    rpm_release: String,
    rpm_pre_release: String,
    snapshot: String,
    release: Vec<u64>,
}

static ALPHA_NUM_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"([a-zA-Z]+)|(\d+)|(~)").unwrap());
static DIGITS_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").unwrap());

fn parse(version: &str) -> Option<RpmVersion> {
    let mut remaining = version;
    let mut epoch = 0;

    if let Some(epoch_index) = remaining.find(':') {
        let epoch_str = &remaining[..epoch_index];
        if epoch_str.chars().all(|ch| ch.is_ascii_digit()) && !epoch_str.is_empty() {
            epoch = epoch_str.parse().ok()?;
        } else {
            return None;
        }
        remaining = &remaining[epoch_index + 1..];
    }

    let release_index = remaining.find('-');
    let prerelease_index = remaining.find('~');
    let snapshot_index = remaining.find('^');

    let mut rpm_release = String::new();
    let mut rpm_pre_release = String::new();
    let mut snapshot = String::new();

    let upstream_version = if let Some(release_index) = release_index {
        if let Some(prerelease_index) = prerelease_index {
            rpm_release = remaining[release_index..prerelease_index].to_owned();
            if let Some(snapshot_index) = snapshot_index {
                rpm_pre_release = remaining[prerelease_index..snapshot_index].to_owned();
                snapshot = remaining[snapshot_index + 1..].to_owned();
            } else {
                rpm_pre_release = remaining[prerelease_index..].to_owned();
            }
        } else {
            rpm_release = remaining[release_index + 1..].to_owned();
        }
        remaining[..release_index].to_owned()
    } else {
        remaining.to_owned()
    };

    let release = DIGITS_RE
        .find_iter(remaining)
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect();

    Some(RpmVersion {
        epoch,
        upstream_version,
        rpm_release,
        rpm_pre_release,
        snapshot,
        release,
    })
}

pub fn is_valid(version: &str) -> bool {
    parse(version).is_some()
}

fn compare_string(s1: &str, s2: &str) -> Ordering {
    if s1 == s2 {
        return Ordering::Equal;
    }

    for (c1, c2) in s1.chars().zip(s2.chars()) {
        match c1.cmp(&c2) {
            Ordering::Equal => {}
            ordering => return ordering,
        }
    }

    s1.len().cmp(&s2.len())
}

fn numeric_cmp(a: &str, b: &str) -> Ordering {
    let a = a.trim_start_matches('0');
    let b = b.trim_start_matches('0');
    let a = if a.is_empty() { "0" } else { a };
    let b = if b.is_empty() { "0" } else { b };

    match a.len().cmp(&b.len()) {
        Ordering::Equal => a.cmp(b),
        ordering => ordering,
    }
}

fn compare_glob(v1: &str, v2: &str) -> Ordering {
    if v1 == v2 {
        return Ordering::Equal;
    }

    let matches_v1 = ALPHA_NUM_RE
        .find_iter(v1)
        .map(|m| m.as_str())
        .collect::<Vec<_>>();
    let matches_v2 = ALPHA_NUM_RE
        .find_iter(v2)
        .map(|m| m.as_str())
        .collect::<Vec<_>>();
    let matches = matches_v1.len().min(matches_v2.len());

    for index in 0..matches {
        let match_v1 = matches_v1[index];
        let match_v2 = matches_v2[index];

        if match_v1.starts_with('~') || match_v2.starts_with('~') {
            if !match_v1.starts_with('~') {
                return Ordering::Greater;
            }
            if !match_v2.starts_with('~') {
                return Ordering::Less;
            }
        }

        if match_v1.starts_with(|ch: char| ch.is_ascii_digit()) {
            if !match_v2.starts_with(|ch: char| ch.is_ascii_digit()) {
                return Ordering::Greater;
            }
            match numeric_cmp(match_v1, match_v2) {
                Ordering::Equal => continue,
                ordering => return ordering,
            }
        } else if match_v2.starts_with(|ch: char| ch.is_ascii_digit()) {
            return Ordering::Less;
        }

        match compare_string(match_v1, match_v2) {
            Ordering::Equal => {}
            ordering => return ordering,
        }
    }

    if matches_v1.len() == matches_v2.len() {
        return Ordering::Equal;
    }

    if matches_v1
        .get(matches)
        .is_some_and(|segment| segment.starts_with('~'))
    {
        return Ordering::Less;
    }
    if matches_v2
        .get(matches)
        .is_some_and(|segment| segment.starts_with('~'))
    {
        return Ordering::Greater;
    }

    matches_v1.len().cmp(&matches_v2.len())
}

fn compare(version: &str, other: &str) -> Ordering {
    let Some(parsed1) = parse(version) else {
        return Ordering::Greater;
    };
    let Some(parsed2) = parse(other) else {
        return Ordering::Greater;
    };

    match parsed1.epoch.cmp(&parsed2.epoch) {
        Ordering::Equal => {}
        ordering => return ordering,
    }

    match compare_glob(&parsed1.upstream_version, &parsed2.upstream_version) {
        Ordering::Equal => {}
        ordering => return ordering,
    }

    let release_version_difference = compare_glob(&parsed1.rpm_release, &parsed2.rpm_release);
    if !release_version_difference.is_eq() {
        return release_version_difference;
    }

    if parsed1.rpm_pre_release.is_empty() && !parsed2.rpm_pre_release.is_empty() {
        return Ordering::Greater;
    }
    if !parsed1.rpm_pre_release.is_empty() && parsed2.rpm_pre_release.is_empty() {
        return Ordering::Less;
    }

    let pre_release_difference = compare_glob(&parsed1.rpm_pre_release, &parsed2.rpm_pre_release);
    if !pre_release_difference.is_eq() {
        return release_version_difference;
    }

    compare_glob(&parsed1.snapshot, &parsed2.snapshot)
}

pub fn equals(a: &str, b: &str) -> bool {
    compare(a, b).is_eq()
}

pub fn is_greater_than(a: &str, b: &str) -> bool {
    compare(a, b).is_gt()
}

pub fn get_major(version: &str) -> Option<u64> {
    parse(version)?.release.first().copied()
}

pub fn get_minor(version: &str) -> Option<u64> {
    parse(version)?.release.get(1).copied()
}

pub fn get_patch(version: &str) -> Option<u64> {
    parse(version)?.release.get(2).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isValid(\"$version\") === $expected" — lib/modules/versioning/rpm/index.spec.ts line 4
    #[test]
    fn is_valid_matches_renovate_rpm_spec() {
        let cases = [
            ("1.1", true),
            ("1.3.RC2", true),
            ("0:1.1-1", true),
            ("a:1.1-1", false),
            ("1.1:1.3-1", false),
            ("1.1a:1.3-1", false),
            ("1a:1.3-1", false),
            ("-1:1.3-1", false),
            ("1:1:1:2-1", true),
            ("1:a:b:c:2-1", true),
            ("1:3_3.2-1", true),
            ("1:3!3.2-1", true),
            ("1:3/3.2-1", true),
            ("1.0-3_2", true),
            ("1.0-3!3", true),
            ("1.0-3/3", true),
            ("1.0+ä1-1", true),
            ("1,0-1", true),
            ("2:1.1-1", true),
            ("1.1.1-0rpmian1", true),
            ("1.1.1+really1.1.2-0rpmian1", true),
            ("2.31-13+rpm11u5", true),
            ("1:0.17.20140318svn632.el7", true),
            ("2.7.7+dfsg-12", true),
            ("8.20140605hgacf1c26e3029.el7", true),
            ("5:0.5.20120830CVS.el7", true),
            ("1:6.0.1r16-1.1build1", true),
            ("1.el6", true),
            ("1:2.20.1-1~bpo9+1", true),
            ("v1.4", true),
            ("3.5.0", true),
            ("4.2.21.Final", true),
            ("0.6.5.1", true),
            ("20100527", true),
            ("2.1.0-M3", true),
            ("4.3.20.RELEASE", true),
            ("1.1-groovy-2.4", true),
            ("0.8a", true),
            ("3.1.0.GA", true),
            ("3.0.0-beta.3", true),
            ("foo", true),
            ("1.2.3.4.5.6.7", true),
            ("0a1b2c3", true),
            ("0a1b2c3d", true),
            ("0a1b2c3d4e5f6a7b8c9d0a1b2c3d4e5f6a7b8c9d", true),
            ("0a1b2c3d4e5f6a7b8c9d0a1b2c3d4e5f6a7b8c9d0", true),
            ("0a1b2C3", true),
            ("0z1b2c3", true),
            ("0A1b2c3d4e5f6a7b8c9d0a1b2c3d4e5f6a7b8c9d", true),
            ("123098140293", true),
            ("3.12.0-1~a1^20231001", true),
            ("1.2.3^20231001", true),
        ];

        for (version, expected) in cases {
            assert_eq!(is_valid(version), expected, "is_valid({version})");
        }
    }

    // Ported: "equals(\"$a\", \"$b\") === $expected" — lib/modules/versioning/rpm/index.spec.ts line 62
    #[test]
    fn equals_matches_renovate_rpm_spec() {
        let cases = [
            ("", "", true),
            ("~a", "~~", false),
            ("~", "~", true),
            ("~", "1", false),
            ("1~", "~", false),
            ("1", "a", false),
            ("2.4", "2.4", true),
            ("2.4.~", "2.4", false),
            ("2.4", "2.4.~", false),
            ("2.4.0", "2.4.0", true),
            ("2.4.0", "2.4", false),
            ("2.4.1", "2.4", false),
            ("2.4.2", "2.4.1", false),
            ("0.8a", "0.8a", true),
            ("90.5.20120830CVS.el6", "0.5.20120830CVS.el7", false),
            ("0.5.20120830CVS.el7", "0.5.20120830CVS.el6", false),
            ("0.5.20120830CVS.el7", "0.5.20120830CVS.el7", true),
            ("2.31-13+rpm11u5", "2.31-13+rpm11u5", true),
            ("2.31-13+rpm11u5", "2.31-13+rpm11u4", false),
            ("1.4-", "1.4", true),
            ("v1.4", "1.4", false),
            ("0:1.4", "1.4", true),
            ("1:1.4", "1.4", false),
            ("1.4-1", "1.4-2", false),
            ("0:1.4", "a:1.4", false),
            ("a:1.4", "0:1.4", false),
            ("3.12.0-1~^2023", "3.12.0-1^2023", false),
        ];

        for (a, b, expected) in cases {
            assert_eq!(equals(a, b), expected, "equals({a}, {b})");
        }
    }

    // Ported: "isGreaterThan(\"$a\", \"$b\") === $expected" — lib/modules/versioning/rpm/index.spec.ts line 95
    #[test]
    fn is_greater_than_matches_renovate_rpm_spec() {
        let cases = [
            ("2.4.0", "2.4", true),
            ("2.4.2", "2.4.1", true),
            ("2.4.beta", "2.4.alpha", true),
            ("1.9", "2", false),
            ("1.9", "1.9.1", false),
            ("2.4", "2.4.beta", false),
            ("2.4.0", "2.4.beta", true),
            ("2.4.beta", "2.4", true),
            ("2.4.beta", "2.4.0", false),
            ("2.4~", "2.4~~", true),
            ("2.4", "2.4~", true),
            ("2.4a", "2.4", true),
            ("2.31-13+rpm11u5", "2.31-9", true),
            ("2.31-13+rpm11u5", "2.31-13+rpm10u5", true),
            ("2.31-13+rpm11u5", "2.31-13+rpm11u4", true),
            ("1.9", "1:1.7", false),
            ("1.9", "1.12", false),
            ("1.12", "1.9", true),
            ("1:1.9", "1:1.7", true),
            ("2.4.0.beta1", "2.4.0.Beta1", true),
            ("1:1.0", "1:1.0~", true),
            ("1:1.0Z0-0", "1:1.0", true),
            ("1:1.0Z0-0", "1:1.0A0-0", true),
            ("1:1.0a0-0", "1:1.0Z0-0", true),
            ("1:1.0z0-0", "1:1.0a0-0", true),
            ("1:1.0+0-0", "1:1.0z0-0", true),
            ("1:1.0-0-0", "1:1.0+0-0", false),
            ("1:1.0.0-0", "1:1.0-0-0", true),
            ("1:1.0:0-0", "1:1.0.0-0", false),
            ("a:1.4", "0:1.4", true),
            ("0:1.4", "a:1.4", true),
            ("a:1.4", "a:1.4", true),
            ("a1", "a~", true),
            ("a0", "a~", true),
            ("aa", "a1", true),
            ("ab", "a0", true),
            ("10", "1.", true),
            ("10", "1a", true),
            ("a", "A", true),
            ("A", "a", false),
            ("A1", "Aa", false),
            ("aaaaa1", "aaaaaaaaaaaa2", false),
            ("a-1~^20231001", "a-1^20231001", false),
            ("1", "2", false),
            ("a-1~pre2^20231001", "a-1~pre2^20231002", false),
            ("a-1", "a-1~pre1", true),
            ("4.20-4~beta4", "4.20-4", false),
            ("1.2.3~beta2", "1.2.3~alpha1", true),
            ("1.2.3-4~alpha1", "1.2.3-4~beta2", false),
            ("}}}", "{{{", false),
        ];

        for (a, b, expected) in cases {
            assert_eq!(is_greater_than(a, b), expected, "is_greater_than({a}, {b})");
        }
    }

    // Ported: "getMajor(\"$version\") === $expected" — lib/modules/versioning/rpm/index.spec.ts line 151
    #[test]
    fn get_major_matches_renovate_rpm_spec() {
        let cases = [
            ("v1.3.0", Some(1)),
            ("2-0-1", Some(2)),
            ("2.31-13+rpm11u5", Some(2)),
            ("1:2.3.1", Some(2)),
            ("foo", None),
            ("8", Some(8)),
            ("1.0", Some(1)),
        ];

        for (version, expected) in cases {
            assert_eq!(get_major(version), expected, "get_major({version})");
        }
    }

    // Ported: "getMinor(\"$version\") === $expected" — lib/modules/versioning/rpm/index.spec.ts line 164
    #[test]
    fn get_minor_matches_renovate_rpm_spec() {
        let cases = [
            ("v1.3.0", Some(3)),
            ("2-0-1", Some(0)),
            ("2.31-13+rpm11u5", Some(31)),
            ("1:2.3.1", Some(3)),
            ("foo", None),
            ("8", None),
            ("1.0", Some(0)),
        ];

        for (version, expected) in cases {
            assert_eq!(get_minor(version), expected, "get_minor({version})");
        }
    }

    // Ported: "getPatch(\"$version\") === $expected" — lib/modules/versioning/rpm/index.spec.ts line 177
    #[test]
    fn get_patch_matches_renovate_rpm_spec() {
        let cases = [
            ("v1.3.0", Some(0)),
            ("2-0-1", Some(1)),
            ("2.31-13+rpm11u5", Some(13)),
            ("1:2.3.1", Some(1)),
            ("foo", None),
            ("8", None),
            ("1.0", None),
        ];

        for (version, expected) in cases {
            assert_eq!(get_patch(version), expected, "get_patch({version})");
        }
    }
}
