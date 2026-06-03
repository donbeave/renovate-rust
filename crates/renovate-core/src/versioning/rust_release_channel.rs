//! Rust release channel versioning.
//!
//! Ports `lib/modules/versioning/rust-release-channel/index.ts` and
//! `lib/modules/versioning/rust-release-channel/parse.ts`.
//! Supports channel names (stable/beta/nightly), versioned releases (1.82.0),
//! partial versions (1.82), beta prereleases (1.83.0-beta.5), dated nightlies,
//! and host triples.

use std::cmp::Ordering;
use std::sync::LazyLock;

use regex::Regex;

// First Rust 1.0.0 stable release date (2015-05-15)
const RUST1_DATE: (i32, u32, u32) = (2015, 5, 15);

static TOOLCHAIN_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?x)
        ^
        (?P<channel>
            stable | beta | nightly
            |
            (?P<major>\d+) \. (?P<minor>\d+)
            (?: \. (?P<patch>\d+) )?
            (?: - (?P<beta>beta) (?: \. (?P<betaNumber>\d+) )? )?
        )
        (?: - (?P<year>\d{4}) - (?P<month>\d{2}) - (?P<day>\d{2}) )?
        (?: - (?P<host>.+) )?
        $",
    )
    .unwrap()
});

#[derive(Debug, Clone, PartialEq)]
pub struct Prerelease {
    pub name: &'static str,
    pub number: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VersionChannel {
    pub major: u32,
    pub minor: u32,
    pub patch: Option<u32>,
    pub prerelease: Option<Prerelease>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Channel {
    Stable,
    Beta,
    Nightly,
    Version(VersionChannel),
}

#[derive(Debug, Clone, PartialEq)]
pub struct DateObj {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Toolchain {
    pub channel: Channel,
    pub date: Option<DateObj>,
    pub host: Option<String>,
}

pub fn parse(input: &str) -> Option<Toolchain> {
    if input.is_empty() {
        return None;
    }

    let caps = TOOLCHAIN_RE.captures(input)?;

    let channel_str = caps.name("channel")?.as_str();

    let channel = match channel_str {
        "stable" => Channel::Stable,
        "beta" => Channel::Beta,
        "nightly" => Channel::Nightly,
        _ => {
            let major: u32 = caps.name("major")?.as_str().parse().ok()?;
            let minor: u32 = caps.name("minor")?.as_str().parse().ok()?;
            let patch = caps
                .name("patch")
                .and_then(|m| m.as_str().parse::<u32>().ok());
            let prerelease = if caps.name("beta").is_some() {
                let number = caps
                    .name("betaNumber")
                    .and_then(|m| m.as_str().parse::<u32>().ok());
                Some(Prerelease {
                    name: "beta",
                    number,
                })
            } else {
                None
            };
            Channel::Version(VersionChannel {
                major,
                minor,
                patch,
                prerelease,
            })
        }
    };

    let date = match (caps.name("year"), caps.name("month"), caps.name("day")) {
        (Some(y), Some(mo), Some(d)) => {
            let year: i32 = y.as_str().parse().ok()?;
            let month: u32 = mo.as_str().parse().ok()?;
            let day: u32 = d.as_str().parse().ok()?;
            Some(DateObj { year, month, day })
        }
        _ => None,
    };

    let host = caps.name("host").map(|m| m.as_str().to_owned());

    Some(Toolchain {
        channel,
        date,
        host,
    })
}

fn sort_parsed(a: &Toolchain, b: &Toolchain) -> i32 {
    let is_a_nightly = a.channel == Channel::Nightly;
    let is_b_nightly = b.channel == Channel::Nightly;

    if is_a_nightly && !is_b_nightly {
        return 1;
    } else if !is_a_nightly && is_b_nightly {
        return -1;
    }

    if is_a_nightly && is_b_nightly {
        if let (Some(da), Some(db)) = (&a.date, &b.date) {
            if da.year != db.year {
                return da.year - db.year;
            }
            if da.month != db.month {
                return da.month as i32 - db.month as i32;
            }
            if da.day != db.day {
                return da.day as i32 - db.day as i32;
            }
        }
        return 0;
    }

    match (&a.channel, &b.channel) {
        (Channel::Version(va), Channel::Version(vb)) => {
            if va.major != vb.major {
                return va.major as i32 - vb.major as i32;
            }
            if va.minor != vb.minor {
                return va.minor as i32 - vb.minor as i32;
            }
            let pa = va.patch.unwrap_or(0);
            let pb = vb.patch.unwrap_or(0);
            if pa != pb {
                return pa as i32 - pb as i32;
            }
            let has_pre_a = va.prerelease.is_some();
            let has_pre_b = vb.prerelease.is_some();
            if has_pre_a && !has_pre_b {
                return -1;
            } else if !has_pre_a && has_pre_b {
                return 1;
            }
            if has_pre_a && has_pre_b {
                let num_a = va.prerelease.as_ref().and_then(|p| p.number).unwrap_or(0);
                let num_b = vb.prerelease.as_ref().and_then(|p| p.number).unwrap_or(0);
                if num_a != num_b {
                    return num_a as i32 - num_b as i32;
                }
            }
            0
        }
        _ => 0,
    }
}

pub fn is_valid(input: &str) -> bool {
    parse(input).is_some()
}

pub fn is_version(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    let Some(parsed) = parse(input) else {
        return false;
    };
    match &parsed.channel {
        Channel::Nightly => parsed.date.is_some(),
        Channel::Version(v) => {
            if v.patch.is_none() {
                return false;
            }
            match &v.prerelease {
                None => true,
                Some(pre) => pre.number.is_some(),
            }
        }
        _ => false,
    }
}

pub fn is_single_version(input: &str) -> bool {
    is_version(input)
}

pub fn is_stable(version: &str) -> bool {
    let Some(parsed) = parse(version) else {
        return false;
    };
    match &parsed.channel {
        Channel::Version(v) => v.patch.is_some() && v.prerelease.is_none(),
        _ => false,
    }
}

pub fn is_compatible(version: &str, current: Option<&str>) -> bool {
    let Some(current) = current else {
        return true;
    };
    if current.is_empty() {
        return true;
    }
    let Some(pv) = parse(version) else {
        return false;
    };
    let Some(pc) = parse(current) else {
        return false;
    };
    if pv.host != pc.host {
        return false;
    }
    let is_v_nightly = pv.channel == Channel::Nightly;
    let is_c_nightly = pc.channel == Channel::Nightly;
    is_v_nightly == is_c_nightly
}

pub fn get_major(version: &str) -> Option<i64> {
    let parsed = parse(version)?;
    match &parsed.channel {
        Channel::Nightly => {
            if let Some(date) = &parsed.date {
                let rust1 = RUST1_DATE;
                let after = (date.year, date.month, date.day) >= ({ rust1.0 }, rust1.1, rust1.2);
                Some(if after { 1 } else { 0 })
            } else {
                None
            }
        }
        Channel::Version(v) => Some(v.major as i64),
        _ => Some(1),
    }
}

pub fn get_minor(version: &str) -> Option<i64> {
    let parsed = parse(version)?;
    match &parsed.channel {
        Channel::Version(v) => Some(v.minor as i64),
        _ => None,
    }
}

pub fn get_patch(version: &str) -> Option<i64> {
    let parsed = parse(version)?;
    match &parsed.channel {
        Channel::Version(v) => v.patch.map(|p| p as i64),
        _ => None,
    }
}

pub fn sort_versions(a: &str, b: &str) -> i32 {
    match (parse(a), parse(b)) {
        (Some(pa), Some(pb)) => sort_parsed(&pa, &pb),
        _ => {
            // localeCompare-like: lexicographic fallback
            match a.cmp(b) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            }
        }
    }
}

pub fn equals(a: &str, b: &str) -> bool {
    match (parse(a), parse(b)) {
        (Some(pa), Some(pb)) => sort_parsed(&pa, &pb) == 0,
        _ => false,
    }
}

pub fn is_greater_than(version: &str, other: &str) -> bool {
    sort_versions(version, other) > 0
}

pub fn matches(version: &str, range: &str) -> bool {
    let Some(pv) = parse(version) else {
        return false;
    };
    let Some(pr) = parse(range) else {
        return false;
    };

    let vc = &pv.channel;
    let rc = &pr.channel;

    match rc {
        Channel::Nightly => matches!(vc, Channel::Nightly) && pv.date.is_some(),
        Channel::Beta => {
            matches!(vc, Channel::Version(v) if v.prerelease.as_ref().map(|p| p.name) == Some("beta"))
        }
        Channel::Stable => {
            matches!(vc, Channel::Version(v) if v.patch.is_some() && v.prerelease.is_none())
        }
        Channel::Version(rv) => {
            if let Channel::Version(vv) = vc {
                if vv.major != rv.major || vv.minor != rv.minor {
                    return false;
                }
                if rv.patch.is_none() {
                    return true;
                }
                if vv.patch != rv.patch {
                    return false;
                }
                match &rv.prerelease {
                    None => vv.prerelease.is_none(),
                    Some(rp) => {
                        if rp.number.is_none() {
                            vv.prerelease.is_some()
                        } else {
                            vv.prerelease.as_ref().map(|p| p.number) == Some(rp.number)
                        }
                    }
                }
            } else {
                false
            }
        }
    }
}

pub fn get_satisfying_version(versions: &[&str], range: &str) -> Option<String> {
    let mut matching: Vec<&str> = versions
        .iter()
        .copied()
        .filter(|v| matches(v, range))
        .collect();
    if matching.is_empty() {
        return None;
    }
    matching.sort_by(|a, b| {
        let cmp = sort_versions(a, b);
        if cmp < 0 {
            Ordering::Less
        } else if cmp > 0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    matching.last().map(|s| (*s).to_owned())
}

pub fn min_satisfying_version(versions: &[&str], range: &str) -> Option<String> {
    let mut matching: Vec<&str> = versions
        .iter()
        .copied()
        .filter(|v| matches(v, range))
        .collect();
    if matching.is_empty() {
        return None;
    }
    matching.sort_by(|a, b| {
        let cmp = sort_versions(a, b);
        if cmp < 0 {
            Ordering::Less
        } else if cmp > 0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    matching.first().map(|s| (*s).to_owned())
}

pub fn get_new_value(
    current_value: &str,
    range_strategy: &str,
    new_version: &str,
) -> Option<String> {
    let parsed_current = parse(current_value)?;
    let parsed_new = parse(new_version)?;

    let current_channel = &parsed_current.channel;
    let new_channel = &parsed_new.channel;

    if range_strategy == "pin" {
        return Some(new_version.to_owned());
    }

    // Dated nightlies replaced by new dated nightly
    if *current_channel == Channel::Nightly && parsed_current.date.is_some() {
        return Some(new_version.to_owned());
    }

    // Channel names without dates stay as channel names
    match current_channel {
        Channel::Stable | Channel::Beta | Channel::Nightly => Some(current_value.to_owned()),
        Channel::Version(cv) => {
            if let Channel::Version(nv) = new_channel {
                if cv.patch.is_none() {
                    return Some(format!("{}.{}", nv.major, nv.minor));
                }
                if let Some(cp) = &cv.prerelease
                    && cp.number.is_none()
                {
                    return Some(format!(
                        "{}.{}.{}-beta",
                        nv.major,
                        nv.minor,
                        nv.patch.unwrap_or(0)
                    ));
                }
            }
            Some(new_version.to_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "parses "$input" correctly" — lib/modules/versioning/rust-release-channel/parse.spec.ts line 6
    #[test]
    fn parse_channel_names_matches_renovate_rust_release_channel_parse_spec() {
        let s = parse("stable").unwrap();
        assert_eq!(s.channel, Channel::Stable);
        assert!(s.date.is_none());

        let b = parse("beta").unwrap();
        assert_eq!(b.channel, Channel::Beta);

        let n = parse("nightly").unwrap();
        assert_eq!(n.channel, Channel::Nightly);
        assert!(n.date.is_none());
    }

    // Ported: "parses "$input" correctly" — lib/modules/versioning/rust-release-channel/parse.spec.ts line 17
    #[test]
    fn parse_full_versions_matches_renovate_rust_release_channel_parse_spec() {
        let p = parse("1.82.0").unwrap();
        assert_eq!(
            p.channel,
            Channel::Version(VersionChannel {
                major: 1,
                minor: 82,
                patch: Some(0),
                prerelease: None
            })
        );

        let p2 = parse("1.0.0").unwrap();
        assert_eq!(
            p2.channel,
            Channel::Version(VersionChannel {
                major: 1,
                minor: 0,
                patch: Some(0),
                prerelease: None
            })
        );

        let p3 = parse("2.5.10").unwrap();
        assert_eq!(
            p3.channel,
            Channel::Version(VersionChannel {
                major: 2,
                minor: 5,
                patch: Some(10),
                prerelease: None
            })
        );
    }

    // Ported: "parses "$input" correctly" — lib/modules/versioning/rust-release-channel/parse.spec.ts line 28
    #[test]
    fn parse_partial_versions_matches_renovate_rust_release_channel_parse_spec() {
        let p = parse("1.82").unwrap();
        assert_eq!(
            p.channel,
            Channel::Version(VersionChannel {
                major: 1,
                minor: 82,
                patch: None,
                prerelease: None
            })
        );

        let p2 = parse("1.0").unwrap();
        assert_eq!(
            p2.channel,
            Channel::Version(VersionChannel {
                major: 1,
                minor: 0,
                patch: None,
                prerelease: None
            })
        );
    }

    // Ported: "parses "$input" correctly" — lib/modules/versioning/rust-release-channel/parse.spec.ts line 39
    #[test]
    fn parse_beta_versions_with_number_matches_renovate_rust_release_channel_parse_spec() {
        let p = parse("1.83.0-beta.5").unwrap();
        assert_eq!(
            p.channel,
            Channel::Version(VersionChannel {
                major: 1,
                minor: 83,
                patch: Some(0),
                prerelease: Some(Prerelease {
                    name: "beta",
                    number: Some(5)
                })
            })
        );

        let p2 = parse("2.0.0-beta.10").unwrap();
        assert_eq!(
            p2.channel,
            Channel::Version(VersionChannel {
                major: 2,
                minor: 0,
                patch: Some(0),
                prerelease: Some(Prerelease {
                    name: "beta",
                    number: Some(10)
                })
            })
        );
    }

    // Ported: "parses "$input" correctly" — lib/modules/versioning/rust-release-channel/parse.spec.ts line 50
    #[test]
    fn parse_beta_ranges_without_number_matches_renovate_rust_release_channel_parse_spec() {
        let p = parse("1.83.0-beta").unwrap();
        assert_eq!(
            p.channel,
            Channel::Version(VersionChannel {
                major: 1,
                minor: 83,
                patch: Some(0),
                prerelease: Some(Prerelease {
                    name: "beta",
                    number: None
                })
            })
        );
    }

    // Ported: "parses "$input" correctly" — lib/modules/versioning/rust-release-channel/parse.spec.ts line 60
    #[test]
    fn parse_dated_channels_matches_renovate_rust_release_channel_parse_spec() {
        let p = parse("stable-2025-11-24").unwrap();
        assert_eq!(p.channel, Channel::Stable);
        assert_eq!(
            p.date,
            Some(DateObj {
                year: 2025,
                month: 11,
                day: 24
            })
        );

        let n = parse("nightly-2025-11-24").unwrap();
        assert_eq!(n.channel, Channel::Nightly);
        assert_eq!(
            n.date,
            Some(DateObj {
                year: 2025,
                month: 11,
                day: 24
            })
        );

        let n2 = parse("nightly-2015-05-15").unwrap();
        assert_eq!(
            n2.date,
            Some(DateObj {
                year: 2015,
                month: 5,
                day: 15
            })
        );

        let n3 = parse("nightly-2025-01-01").unwrap();
        assert_eq!(
            n3.date,
            Some(DateObj {
                year: 2025,
                month: 1,
                day: 1
            })
        );
    }

    // Ported: "parses "$input" correctly" — lib/modules/versioning/rust-release-channel/parse.spec.ts line 74
    #[test]
    fn parse_host_triples_matches_renovate_rust_release_channel_parse_spec() {
        let p = parse("stable-x86_64-pc-windows-msvc").unwrap();
        assert_eq!(p.channel, Channel::Stable);
        assert_eq!(p.host.as_deref(), Some("x86_64-pc-windows-msvc"));

        let p2 = parse("1.82.0-x86_64-pc-windows-msvc").unwrap();
        assert_eq!(
            p2.channel,
            Channel::Version(VersionChannel {
                major: 1,
                minor: 82,
                patch: Some(0),
                prerelease: None
            })
        );
        assert_eq!(p2.host.as_deref(), Some("x86_64-pc-windows-msvc"));

        let p3 = parse("nightly-2025-11-24-x86_64-pc-windows-msvc").unwrap();
        assert_eq!(p3.channel, Channel::Nightly);
        assert_eq!(
            p3.date,
            Some(DateObj {
                year: 2025,
                month: 11,
                day: 24
            })
        );
        assert_eq!(p3.host.as_deref(), Some("x86_64-pc-windows-msvc"));
    }

    // Ported: "returns null for "$input" ($reason)" — lib/modules/versioning/rust-release-channel/parse.spec.ts line 87
    #[test]
    fn parse_invalid_inputs_matches_renovate_rust_release_channel_parse_spec() {
        assert!(parse("").is_none());
        assert!(parse("invalid").is_none());
        assert!(parse("1.82.0.0").is_none());
        assert!(parse("1.-1.0").is_none());
        assert!(parse("1.82.-1").is_none());
        assert!(parse("a.b.c").is_none());
    }

    // Ported: "isValid("$input") === $expected" — lib/modules/versioning/rust-release-channel/index.spec.ts line 4
    #[test]
    fn is_valid_matches_renovate_rust_release_channel_index_spec() {
        assert!(is_valid("stable"));
        assert!(is_valid("beta"));
        assert!(is_valid("nightly"));
        assert!(is_valid("1.82.0"));
        assert!(is_valid("1.82"));
        assert!(is_valid("1.83.0-beta.5"));
        assert!(is_valid("1.83.0-beta"));
        assert!(is_valid("nightly-2025-11-24"));
        assert!(is_valid("stable-x86_64-pc-windows-msvc"));
        assert!(!is_valid(""));
        assert!(!is_valid("invalid"));
        assert!(!is_valid("1.82.0.0"));
        assert!(!is_valid("a.b.c"));
    }

    // Ported: "isVersion("$input") === $expected" — lib/modules/versioning/rust-release-channel/index.spec.ts line 23
    #[test]
    fn is_version_matches_renovate_rust_release_channel_index_spec() {
        assert!(is_version("1.82.0"));
        assert!(is_version("1.83.0-beta.5"));
        assert!(is_version("nightly-2025-11-24"));
        assert!(!is_version("stable"));
        assert!(!is_version("beta"));
        assert!(!is_version("nightly"));
        assert!(!is_version("1.82"));
        assert!(!is_version("1.83.0-beta"));
        assert!(!is_version(""));
        assert!(!is_version("invalid"));
    }

    // Ported: "isSingleVersion("$input") === $expected" — lib/modules/versioning/rust-release-channel/index.spec.ts line 40
    #[test]
    fn is_single_version_matches_renovate_rust_release_channel_index_spec() {
        assert!(is_single_version("1.82.0"));
        assert!(is_single_version("1.83.0-beta.5"));
        assert!(is_single_version("nightly-2025-11-24"));
        assert!(!is_single_version("stable"));
        assert!(!is_single_version("beta"));
        assert!(!is_single_version("nightly"));
        assert!(!is_single_version("1.82"));
        assert!(!is_single_version("1.83.0-beta"));
    }

    // Ported: "isStable("$version") === $expected" — lib/modules/versioning/rust-release-channel/index.spec.ts line 54
    #[test]
    fn is_stable_matches_renovate_rust_release_channel_index_spec() {
        assert!(is_stable("1.82.0"));
        assert!(is_stable("1.0.0"));
        assert!(is_stable("2.5.10"));
        assert!(!is_stable("1.83.0-beta.5"));
        assert!(!is_stable("1.83.0-beta"));
        assert!(!is_stable("nightly-2025-11-24"));
        assert!(!is_stable("stable"));
        assert!(!is_stable("1.82"));
        assert!(!is_stable("invalid"));
    }

    // Ported: "equals("$a", "$b") === $expected" — lib/modules/versioning/rust-release-channel/index.spec.ts line 69
    #[test]
    fn equals_matches_renovate_rust_release_channel_index_spec() {
        assert!(equals("1.82", "1.82"));
        assert!(equals("1.82.0", "1.82.0"));
        assert!(equals("1.83.0-beta", "1.83.0-beta"));
        assert!(equals("1.83.0-beta.5", "1.83.0-beta.5"));
        assert!(equals("nightly-2025-11-24", "nightly-2025-11-24"));
        assert!(equals("stable", "stable"));
        assert!(!equals("invalid", "invalid"));
        assert!(!equals("1.82.0", "1.83.0"));
        assert!(!equals("1.83.0-beta.5", "1.83.0-beta.1"));
        assert!(!equals("nightly-2025-11-24", "nightly-2025-11-23"));
    }

    // Ported: "isGreaterThan("$a", "$b") === $expected" — lib/modules/versioning/rust-release-channel/index.spec.ts line 85
    #[test]
    fn is_greater_than_matches_renovate_rust_release_channel_index_spec() {
        assert!(is_greater_than("nightly-2025-11-24", "1.82.0"));
        assert!(is_greater_than("nightly-2025-11-24", "nightly-2025-11-23"));
        assert!(is_greater_than("nightly-2025-11-24", "nightly-2024-11-24"));
        assert!(is_greater_than("nightly-2025-11-24", "nightly-2025-10-24"));
        assert!(is_greater_than("1.83.0", "1.82.0"));
        assert!(is_greater_than("1.83.0-beta.5", "1.83.0-beta.1"));
        assert!(is_greater_than("1.83.0", "1.83.0-beta.1"));
        assert!(is_greater_than("1.84.0-beta.1", "1.83.0-beta.1"));
        assert!(is_greater_than("2.0.0", "1.99.0"));
        assert!(is_greater_than("1.83", "1.82"));
        assert!(is_greater_than("1.82.1", "1.82"));
        assert!(!is_greater_than("1.82.0", "1.83.0"));
        assert!(!is_greater_than("1.82", "1.83"));
        assert!(!is_greater_than("1.82", "1.82.0"));
        assert!(!is_greater_than("1.82", "1.82.1"));
        assert!(!is_greater_than("1.83.0-beta.1", "1.83.0-beta.5"));
        assert!(!is_greater_than("nightly-2025-11-23", "nightly-2025-11-24"));
        assert!(!is_greater_than("nightly-2024-11-24", "nightly-2025-11-24"));
        assert!(!is_greater_than("nightly-2025-10-24", "nightly-2025-11-24"));
        assert!(!is_greater_than("1.82.0", "nightly-2025-11-24"));
        assert!(!is_greater_than("1.83.0-beta.1", "1.83.0"));
        assert!(!is_greater_than("1.99.0", "2.0.0"));
    }

    // Ported: "sortVersions("$a", "$b") === $expected" — lib/modules/versioning/rust-release-channel/index.spec.ts line 113
    #[test]
    fn sort_versions_matches_renovate_rust_release_channel_index_spec() {
        assert_eq!(sort_versions("1.82.0", "1.82.0"), 0);
        assert_eq!(sort_versions("1.83.0-beta.5", "1.83.0-beta.5"), 0);
        assert_eq!(sort_versions("nightly-2025-11-24", "nightly-2025-11-24"), 0);
        assert_eq!(sort_versions("foo", "foo"), 0);
        assert_eq!(sort_versions("1.83.0", "1.82.0"), 1);
        assert_eq!(sort_versions("2.0.0", "1.99.0"), 1);
        assert_eq!(sort_versions("1.83.0-beta.5", "1.83.0-beta.1"), 4);
        assert_eq!(sort_versions("1.83.0", "1.83.0-beta.1"), 1);
        assert_eq!(sort_versions("nightly-2025-11-24", "1.82.0"), 1);
        assert_eq!(sort_versions("nightly-2025-11-24", "nightly-2025-11-23"), 1);
        assert_eq!(sort_versions("foo", "bar"), 1);
        assert_eq!(sort_versions("1.82.0", "1.83.0"), -1);
        assert_eq!(sort_versions("1.99.0", "2.0.0"), -1);
        assert_eq!(sort_versions("1.83.0-beta.1", "1.83.0-beta.5"), -4);
        assert_eq!(sort_versions("1.83.0-beta.1", "1.83.0"), -1);
        assert_eq!(sort_versions("1.82.0", "nightly-2025-11-24"), -1);
        assert_eq!(
            sort_versions("nightly-2025-11-23", "nightly-2025-11-24"),
            -1
        );
        assert_eq!(sort_versions("bar", "foo"), -1);
    }

    // Ported: "getMajor("$version") === $expected" — lib/modules/versioning/rust-release-channel/index.spec.ts line 137
    #[test]
    fn get_major_matches_renovate_rust_release_channel_index_spec() {
        assert_eq!(get_major("1.82.0"), Some(1));
        assert_eq!(get_major("1.0.0"), Some(1));
        assert_eq!(get_major("2.5.10"), Some(2));
        assert_eq!(get_major("1.83.0-beta.5"), Some(1));
        assert_eq!(get_major("nightly-2025-11-24"), Some(1));
        assert_eq!(get_major("nightly-2014-12-18"), Some(0));
        assert_eq!(get_major("stable"), Some(1));
        assert_eq!(get_major("invalid"), None);
    }

    // Ported: "getMinor("$version") === $expected" — lib/modules/versioning/rust-release-channel/index.spec.ts line 151
    #[test]
    fn get_minor_matches_renovate_rust_release_channel_index_spec() {
        assert_eq!(get_minor("1.82.0"), Some(82));
        assert_eq!(get_minor("1.0.0"), Some(0));
        assert_eq!(get_minor("2.5.10"), Some(5));
        assert_eq!(get_minor("1.83.0-beta.5"), Some(83));
        assert_eq!(get_minor("nightly-2025-11-24"), None);
        assert_eq!(get_minor("invalid"), None);
    }

    // Ported: "getPatch("$version") === $expected" — lib/modules/versioning/rust-release-channel/index.spec.ts line 163
    #[test]
    fn get_patch_matches_renovate_rust_release_channel_index_spec() {
        assert_eq!(get_patch("1.82.0"), Some(0));
        assert_eq!(get_patch("1.0.0"), Some(0));
        assert_eq!(get_patch("2.5.10"), Some(10));
        assert_eq!(get_patch("1.83.0-beta.5"), Some(0));
        assert_eq!(get_patch("1.82"), None);
        assert_eq!(get_patch("nightly-2025-11-24"), None);
        assert_eq!(get_patch("invalid"), None);
    }

    // Ported: "matches("$version", "$range") === $expected" — lib/modules/versioning/rust-release-channel/index.spec.ts line 176
    #[test]
    fn matches_matches_renovate_rust_release_channel_index_spec() {
        assert!(matches("1.82.0", "stable"));
        assert!(matches("1.82.0", "1.82"));
        assert!(matches("1.82.1", "1.82"));
        assert!(matches("1.82.0", "1.82.0"));
        assert!(matches("1.83.0-beta.5", "beta"));
        assert!(matches("1.83.0-beta.5", "1.83.0-beta"));
        assert!(matches("1.83.0-beta.1", "1.83.0-beta.1"));
        assert!(matches("nightly-2025-11-24", "nightly"));
        assert!(matches("nightly-2025-11-24", "nightly-2025-11-24"));
        assert!(!matches("1.83.0-beta.5", "stable"));
        assert!(!matches("1.82.0", "beta"));
        assert!(!matches("1.82.0", "nightly"));
        assert!(!matches("1.83.0", "1.82"));
        assert!(!matches("1.82.0", "1.83.0-beta"));
        assert!(!matches("nightly-2025-11-24", "stable"));
        assert!(!matches("nightly-2025-11-24", "1.82"));
        assert!(!matches("1.82.1", "1.82.0"));
        assert!(!matches("invalid", "1.82"));
        assert!(!matches("1.82.0", "invalid"));
    }

    // Ported: "isCompatible("$version", "$current") === $expected" — lib/modules/versioning/rust-release-channel/index.spec.ts line 204
    #[test]
    fn is_compatible_matches_renovate_rust_release_channel_index_spec() {
        assert!(is_compatible(
            "nightly-2025-11-24",
            Some("nightly-2025-11-23")
        ));
        assert!(is_compatible(
            "nightly-2025-11-25",
            Some("nightly-2025-11-24")
        ));
        assert!(is_compatible("1.83.0", Some("1.82.0")));
        assert!(is_compatible("1.83.0-beta.5", Some("1.82.0")));
        assert!(is_compatible("1.83.0", Some("1.82.0-beta.1")));
        assert!(!is_compatible("nightly-2025-11-24", Some("1.82.0")));
        assert!(!is_compatible("1.82.0", Some("nightly-2025-11-24")));
        assert!(is_compatible("1.82.0", None));
        assert!(!is_compatible("invalid", Some("1.82.0")));
        assert!(!is_compatible("1.82.0", Some("invalid")));
        assert!(is_compatible(
            "1.83.0-x86_64-unknown-linux-gnu",
            Some("1.82.0-x86_64-unknown-linux-gnu")
        ));
        assert!(!is_compatible(
            "1.83.0-x86_64-unknown-linux-gnu",
            Some("1.82.0-aarch64-apple-darwin")
        ));
        assert!(!is_compatible(
            "1.83.0-x86_64-unknown-linux-gnu",
            Some("1.82.0")
        ));
        assert!(!is_compatible(
            "1.83.0",
            Some("1.82.0-x86_64-unknown-linux-gnu")
        ));
        assert!(is_compatible(
            "nightly-2025-11-24-x86_64-unknown-linux-gnu",
            Some("nightly-2025-11-23-x86_64-unknown-linux-gnu")
        ));
        assert!(!is_compatible(
            "nightly-2025-11-24-x86_64-unknown-linux-gnu",
            Some("nightly-2025-11-23-aarch64-apple-darwin")
        ));
    }

    // Ported: "getSatisfyingVersion($versions, "$range") === $expected" — lib/modules/versioning/rust-release-channel/index.spec.ts line 229
    #[test]
    fn get_satisfying_version_matches_renovate_rust_release_channel_index_spec() {
        assert_eq!(
            get_satisfying_version(&["1.82.0", "1.83.0", "1.84.0"], "stable").as_deref(),
            Some("1.84.0")
        );
        assert_eq!(
            get_satisfying_version(&["1.82.0", "1.83.0-beta.1", "1.83.0-beta.5"], "beta")
                .as_deref(),
            Some("1.83.0-beta.5")
        );
        assert_eq!(
            get_satisfying_version(&["1.82.0", "1.82.1", "1.83.0"], "1.82").as_deref(),
            Some("1.82.1")
        );
        assert_eq!(
            get_satisfying_version(
                &["1.83.0-beta.1", "1.83.0-beta.5", "1.84.0-beta.10"],
                "1.83.0-beta"
            )
            .as_deref(),
            Some("1.83.0-beta.5")
        );
        assert_eq!(
            get_satisfying_version(
                &["nightly-2025-11-22", "nightly-2025-11-23", "1.82.0"],
                "nightly"
            )
            .as_deref(),
            Some("nightly-2025-11-23")
        );
        assert_eq!(
            get_satisfying_version(&["1.82.0", "1.83.0-beta.5", "nightly-2025-11-24"], "stable")
                .as_deref(),
            Some("1.82.0")
        );
        assert_eq!(get_satisfying_version(&["1.82.0", "1.83.0"], "beta"), None);
        assert_eq!(get_satisfying_version(&[], "stable"), None);
    }

    // Ported: "minSatisfyingVersion($versions, "$range") === $expected" — lib/modules/versioning/rust-release-channel/index.spec.ts line 248
    #[test]
    fn min_satisfying_version_matches_renovate_rust_release_channel_index_spec() {
        assert_eq!(
            min_satisfying_version(&["1.82.0", "1.83.0", "1.84.0"], "stable").as_deref(),
            Some("1.82.0")
        );
        assert_eq!(
            min_satisfying_version(&["1.82.0", "1.83.0-beta.1", "1.83.0-beta.5"], "beta")
                .as_deref(),
            Some("1.83.0-beta.1")
        );
        assert_eq!(
            min_satisfying_version(&["1.82.0", "1.82.1", "1.83.0"], "1.82").as_deref(),
            Some("1.82.0")
        );
        assert_eq!(
            min_satisfying_version(
                &["1.82.0-beta.1", "1.83.0-beta.5", "1.83.0-beta.10"],
                "1.83.0-beta"
            )
            .as_deref(),
            Some("1.83.0-beta.5")
        );
        assert_eq!(
            min_satisfying_version(
                &["nightly-2025-11-22", "nightly-2025-11-23", "1.82.0"],
                "nightly"
            )
            .as_deref(),
            Some("nightly-2025-11-22")
        );
        assert_eq!(
            min_satisfying_version(&["1.82.0", "1.83.0-beta.5", "nightly-2025-11-24"], "stable")
                .as_deref(),
            Some("1.82.0")
        );
        assert_eq!(min_satisfying_version(&["1.82.0", "1.83.0"], "beta"), None);
        assert_eq!(min_satisfying_version(&[], "stable"), None);
    }

    // Ported: "getNewValue({ currentValue: "$currentValue", rangeStrategy: "$rangeStrategy", newVersion: "$newVersion" }) === $expected" — lib/modules/versioning/rust-release-channel/index.spec.ts line 267
    #[test]
    fn get_new_value_matches_renovate_rust_release_channel_index_spec() {
        assert_eq!(
            get_new_value("stable", "replace", "1.83.0").as_deref(),
            Some("stable")
        );
        assert_eq!(
            get_new_value("beta", "replace", "1.83.0-beta.5").as_deref(),
            Some("beta")
        );
        assert_eq!(
            get_new_value("nightly", "replace", "nightly-2025-11-24").as_deref(),
            Some("nightly")
        );
        assert_eq!(
            get_new_value("nightly-2025-11-23", "replace", "nightly-2025-11-24").as_deref(),
            Some("nightly-2025-11-24")
        );
        assert_eq!(
            get_new_value("1.82", "replace", "1.83.0").as_deref(),
            Some("1.83")
        );
        assert_eq!(
            get_new_value("1.82.0", "replace", "1.83.0").as_deref(),
            Some("1.83.0")
        );
        assert_eq!(
            get_new_value("1.83.0-beta", "replace", "1.83.0-beta.5").as_deref(),
            Some("1.83.0-beta")
        );
        assert_eq!(
            get_new_value("1.83.0-beta.1", "replace", "1.83.0-beta.5").as_deref(),
            Some("1.83.0-beta.5")
        );
        assert_eq!(
            get_new_value("1.83.0-beta", "replace", "1.84.0-beta.5").as_deref(),
            Some("1.84.0-beta")
        );
        assert_eq!(
            get_new_value("1.83.0-beta", "replace", "1.84-beta.1").as_deref(),
            Some("1.84.0-beta")
        );
        assert_eq!(
            get_new_value("stable", "pin", "1.83.0").as_deref(),
            Some("1.83.0")
        );
        assert_eq!(
            get_new_value("beta", "pin", "1.83.0-beta.5").as_deref(),
            Some("1.83.0-beta.5")
        );
        assert_eq!(
            get_new_value("nightly", "pin", "nightly-2025-11-24").as_deref(),
            Some("nightly-2025-11-24")
        );
        assert_eq!(
            get_new_value("1.82", "pin", "1.82.0").as_deref(),
            Some("1.82.0")
        );
        assert_eq!(
            get_new_value("1.82.0", "pin", "1.82.0").as_deref(),
            Some("1.82.0")
        );
        assert_eq!(get_new_value("invalid", "replace", "1.83.0"), None);
        assert_eq!(get_new_value("1.82.0", "replace", "invalid"), None);
    }
}
