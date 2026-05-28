//! Ubuntu versioning.
//!
//! Renovate reference:
//! - `lib/modules/versioning/ubuntu/index.ts`

use std::cmp::Ordering;
use std::sync::LazyLock;

use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct UbuntuRelease {
    version: &'static str,
    codename: &'static str,
    release: &'static str,
}

static VERSION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(0[4-5]|[6-9]|[1-9][0-9])\.[0-9][0-9](\.[0-9]{1,2})?$").unwrap()
});
static DATED_CODENAME_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?<codename>\w+)-(?<date>\d{8})(?<suffix>\.\d{1,2})?$").unwrap()
});

const RELEASES: &[UbuntuRelease] = &[
    UbuntuRelease {
        version: "04.10",
        codename: "warty",
        release: "2004-10-20",
    },
    UbuntuRelease {
        version: "05.04",
        codename: "hoary",
        release: "2005-04-08",
    },
    UbuntuRelease {
        version: "05.10",
        codename: "breezy",
        release: "2005-10-12",
    },
    UbuntuRelease {
        version: "6.06",
        codename: "dapper",
        release: "2006-06-01",
    },
    UbuntuRelease {
        version: "6.10",
        codename: "edgy",
        release: "2006-10-26",
    },
    UbuntuRelease {
        version: "7.04",
        codename: "feisty",
        release: "2007-04-19",
    },
    UbuntuRelease {
        version: "7.10",
        codename: "gutsy",
        release: "2007-10-18",
    },
    UbuntuRelease {
        version: "8.04",
        codename: "hardy",
        release: "2008-04-24",
    },
    UbuntuRelease {
        version: "8.10",
        codename: "intrepid",
        release: "2008-10-30",
    },
    UbuntuRelease {
        version: "9.04",
        codename: "jaunty",
        release: "2009-04-23",
    },
    UbuntuRelease {
        version: "9.10",
        codename: "karmic",
        release: "2009-10-29",
    },
    UbuntuRelease {
        version: "10.04",
        codename: "lucid",
        release: "2010-04-29",
    },
    UbuntuRelease {
        version: "10.10",
        codename: "maverick",
        release: "2010-10-10",
    },
    UbuntuRelease {
        version: "11.04",
        codename: "natty",
        release: "2011-04-28",
    },
    UbuntuRelease {
        version: "11.10",
        codename: "oneiric",
        release: "2011-10-13",
    },
    UbuntuRelease {
        version: "12.04",
        codename: "precise",
        release: "2012-04-26",
    },
    UbuntuRelease {
        version: "12.10",
        codename: "quantal",
        release: "2012-10-18",
    },
    UbuntuRelease {
        version: "13.04",
        codename: "raring",
        release: "2013-04-25",
    },
    UbuntuRelease {
        version: "13.10",
        codename: "saucy",
        release: "2013-10-17",
    },
    UbuntuRelease {
        version: "14.04",
        codename: "trusty",
        release: "2014-04-17",
    },
    UbuntuRelease {
        version: "14.10",
        codename: "utopic",
        release: "2014-10-23",
    },
    UbuntuRelease {
        version: "15.04",
        codename: "vivid",
        release: "2015-04-23",
    },
    UbuntuRelease {
        version: "15.10",
        codename: "wily",
        release: "2015-10-22",
    },
    UbuntuRelease {
        version: "16.04",
        codename: "xenial",
        release: "2016-04-21",
    },
    UbuntuRelease {
        version: "16.10",
        codename: "yakkety",
        release: "2016-10-13",
    },
    UbuntuRelease {
        version: "17.04",
        codename: "zesty",
        release: "2017-04-13",
    },
    UbuntuRelease {
        version: "17.10",
        codename: "artful",
        release: "2017-10-19",
    },
    UbuntuRelease {
        version: "18.04",
        codename: "bionic",
        release: "2018-04-26",
    },
    UbuntuRelease {
        version: "18.10",
        codename: "cosmic",
        release: "2018-10-18",
    },
    UbuntuRelease {
        version: "19.04",
        codename: "disco",
        release: "2019-04-18",
    },
    UbuntuRelease {
        version: "19.10",
        codename: "eoan",
        release: "2019-10-17",
    },
    UbuntuRelease {
        version: "20.04",
        codename: "focal",
        release: "2020-04-23",
    },
    UbuntuRelease {
        version: "20.10",
        codename: "groovy",
        release: "2020-10-22",
    },
    UbuntuRelease {
        version: "21.04",
        codename: "hirsute",
        release: "2021-04-22",
    },
    UbuntuRelease {
        version: "21.10",
        codename: "impish",
        release: "2021-10-14",
    },
    UbuntuRelease {
        version: "22.04",
        codename: "jammy",
        release: "2022-04-21",
    },
    UbuntuRelease {
        version: "22.10",
        codename: "kinetic",
        release: "2022-10-20",
    },
    UbuntuRelease {
        version: "23.04",
        codename: "lunar",
        release: "2023-04-20",
    },
    UbuntuRelease {
        version: "23.10",
        codename: "mantic",
        release: "2023-10-12",
    },
    UbuntuRelease {
        version: "24.04",
        codename: "noble",
        release: "2024-04-25",
    },
    UbuntuRelease {
        version: "24.10",
        codename: "oracular",
        release: "2024-10-10",
    },
];

fn release_by_codename(codename: &str) -> Option<UbuntuRelease> {
    RELEASES
        .iter()
        .copied()
        .find(|release| release.codename == codename)
}

fn release_by_version(version: &str) -> Option<UbuntuRelease> {
    let base = base_version(version);
    RELEASES
        .iter()
        .copied()
        .find(|release| release.version == base)
}

fn base_version(version: &str) -> &str {
    version
        .split_once('.')
        .and_then(|(major, rest)| rest.split_once('.').map(|(minor, _)| (major, minor)))
        .map_or(version, |(major, minor)| {
            let end = major.len() + 1 + minor.len();
            &version[..end]
        })
}

fn is_codename(input: &str) -> bool {
    release_by_codename(input).is_some()
}

fn dated_captures(input: &str) -> Option<regex::Captures<'_>> {
    DATED_CODENAME_RE.captures(input)
}

fn dated_codename(input: &str) -> Option<&str> {
    dated_captures(input).and_then(|captures| captures.name("codename").map(|m| m.as_str()))
}

fn dated_version(input: &str) -> Option<u64> {
    dated_captures(input)
        .and_then(|captures| captures.name("date"))
        .and_then(|m| m.as_str().parse().ok())
}

fn dated_suffix(input: &str) -> Option<&str> {
    dated_captures(input).and_then(|captures| captures.name("suffix").map(|m| m.as_str()))
}

pub fn is_valid(input: Option<&str>) -> bool {
    let Some(input) = input else {
        return false;
    };
    if input.is_empty() {
        return false;
    }
    VERSION_RE.is_match(input) || is_codename(input) || DATED_CODENAME_RE.is_match(input)
}

pub fn is_version(input: Option<&str>) -> bool {
    is_valid(input)
}

pub fn is_compatible(version: Option<&str>, _current: Option<&str>) -> bool {
    is_valid(version)
}

pub fn is_single_version(version: Option<&str>) -> bool {
    is_valid(version)
}

fn version_by_codename(version: &str) -> &str {
    if let Some(codename) = dated_codename(version) {
        return release_by_codename(codename).map_or(codename, |release| release.version);
    }
    release_by_codename(version).map_or(version, |release| release.version)
}

fn codename_by_version(version: &str) -> &str {
    if is_codename(version) {
        return version;
    }
    release_by_version(version).map_or(version, |release| release.codename)
}

fn is_released(version: &str, now: &str) -> bool {
    release_by_version(version).is_some_and(|release| release.release <= now)
}

pub fn is_stable_at(version: Option<&str>, now: &str) -> bool {
    let Some(version) = version else {
        return false;
    };
    let version = version_by_codename(version);
    if !is_valid(Some(version)) {
        return false;
    }
    if !is_released(base_version(version), now) {
        return false;
    }
    let Some((major, rest)) = version.split_once('.') else {
        return false;
    };
    let minor = rest.split('.').next().unwrap_or("");
    minor == "04"
        && major
            .chars()
            .last()
            .is_some_and(|digit| matches!(digit, '0' | '2' | '4' | '6' | '8'))
}

pub fn get_major(version: Option<&str>) -> Option<u64> {
    let version = version_by_codename(version?);
    if !is_valid(Some(version)) {
        return None;
    }
    version.split('.').next()?.parse().ok()
}

pub fn get_minor(version: Option<&str>) -> Option<u64> {
    let version = version_by_codename(version?);
    if !is_valid(Some(version)) {
        return None;
    }
    version.split('.').nth(1)?.parse().ok()
}

pub fn get_patch(version: Option<&str>) -> Option<u64> {
    let version = version_by_codename(version?);
    if !is_valid(Some(version)) {
        return None;
    }
    version.split('.').nth(2)?.parse().ok()
}

pub fn equals(version: &str, other: &str) -> bool {
    if dated_version(version) != dated_version(other) {
        return false;
    }
    if dated_suffix(version) != dated_suffix(other) {
        return false;
    }
    let version = version_by_codename(version);
    let other = version_by_codename(other);
    is_version(Some(version)) && is_version(Some(other)) && version == other
}

pub fn is_greater_than(version: &str, other: &str) -> bool {
    for ordering in [
        get_major(Some(version))
            .unwrap_or(0)
            .cmp(&get_major(Some(other)).unwrap_or(0)),
        get_minor(Some(version))
            .unwrap_or(0)
            .cmp(&get_minor(Some(other)).unwrap_or(0)),
        dated_version(version)
            .unwrap_or(0)
            .cmp(&dated_version(other).unwrap_or(0)),
        dated_suffix(version)
            .unwrap_or("0")
            .cmp(dated_suffix(other).unwrap_or("0")),
        get_patch(Some(version))
            .unwrap_or(0)
            .cmp(&get_patch(Some(other)).unwrap_or(0)),
    ] {
        match ordering {
            Ordering::Greater => return true,
            Ordering::Less => return false,
            Ordering::Equal => {}
        }
    }
    false
}

pub fn get_satisfying_version<'a>(versions: &[&str], range: &'a str) -> Option<&'a str> {
    versions
        .iter()
        .copied()
        .find(|version| equals(version, range))
        .map(|_| range)
}

pub fn min_satisfying_version<'a>(versions: &[&str], range: &'a str) -> Option<&'a str> {
    get_satisfying_version(versions, range)
}

pub fn get_new_value(current_value: Option<&str>, new_version: &str) -> String {
    if current_value.is_some_and(is_codename) {
        codename_by_version(new_version).to_owned()
    } else {
        version_by_codename(new_version).to_owned()
    }
}

pub fn sort_versions(version: &str, other: &str) -> Ordering {
    if equals(version, other) {
        Ordering::Equal
    } else if is_greater_than(version, other) {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

pub fn matches(version: &str, range: &str) -> bool {
    equals(version, range)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isValid(\"$version\") === $expected" — versioning/ubuntu/index.spec.ts line 7
    #[test]
    fn is_valid_matches_renovate_ubuntu_spec() {
        let cases = [
            (None, false),
            (Some(""), false),
            (Some("xenial"), true),
            (Some("04.10"), true),
            (Some("05.04"), true),
            (Some("05.10"), true),
            (Some("6.06"), true),
            (Some("6.10"), true),
            (Some("7.04"), true),
            (Some("7.10"), true),
            (Some("8.04"), true),
            (Some("8.10"), true),
            (Some("9.04"), true),
            (Some("9.10"), true),
            (Some("10.04.4"), true),
            (Some("10.10"), true),
            (Some("11.04"), true),
            (Some("11.10"), true),
            (Some("12.04.5"), true),
            (Some("12.10"), true),
            (Some("13.04"), true),
            (Some("13.10"), true),
            (Some("14.04.6"), true),
            (Some("14.10"), true),
            (Some("15.04"), true),
            (Some("15.10"), true),
            (Some("16.04.7"), true),
            (Some("16.10"), true),
            (Some("17.04"), true),
            (Some("17.10"), true),
            (Some("18.04.5"), true),
            (Some("18.10"), true),
            (Some("19.04"), true),
            (Some("19.10"), true),
            (Some("20.04"), true),
            (Some("20.10"), true),
            (Some("2020.04"), false),
            (Some("warty"), true),
            (Some("hoary"), true),
            (Some("breezy"), true),
            (Some("dapper"), true),
            (Some("edgy"), true),
            (Some("feisty"), true),
            (Some("gutsy"), true),
            (Some("hardy"), true),
            (Some("intrepid"), true),
            (Some("jaunty"), true),
            (Some("karmic"), true),
            (Some("lucid.4"), false),
            (Some("maverick"), true),
            (Some("natty"), true),
            (Some("oneiric"), true),
            (Some("precise.5"), false),
            (Some("quantal"), true),
            (Some("raring"), true),
            (Some("saucy"), true),
            (Some("trusty.6"), false),
            (Some("utopic"), true),
            (Some("vivid"), true),
            (Some("wily"), true),
            (Some("xenial.7"), false),
            (Some("yakkety"), true),
            (Some("zesty"), true),
            (Some("artful"), true),
            (Some("bionic.5"), false),
            (Some("cosmic"), true),
            (Some("disco"), true),
            (Some("eoan"), true),
            (Some("focal"), true),
            (Some("groovy"), true),
            (Some("hirsute"), true),
            (Some("impish"), true),
            (Some("jammy"), true),
            (Some("jammy-20230816"), true),
            (Some("yakkety-20160806.1"), true),
            (Some("utopic-20150228.11"), true),
            (Some("utopic-20150228.11.1"), false),
            (Some("oracular-20240811."), false),
        ];

        for (version, expected) in cases {
            assert_eq!(is_valid(version), expected, "is_valid({version:?})");
        }
        assert!(!is_valid(None));
    }

    // Ported: "isCompatible(\"$version\") === $expected" — versioning/ubuntu/index.spec.ts line 90
    #[test]
    fn is_compatible_matches_renovate_ubuntu_spec() {
        for (version, expected) in [
            (None, false),
            (Some(""), false),
            (Some("04.10"), true),
            (Some("20.10"), true),
            (Some("warty"), true),
            (Some("groovy"), true),
        ] {
            assert_eq!(is_compatible(version, None), expected);
        }
    }

    // Ported: "isSingleVersion(\"$version\") === $expected" — versioning/ubuntu/index.spec.ts line 108
    #[test]
    fn is_single_version_matches_renovate_ubuntu_spec() {
        for (version, expected) in [
            (None, false),
            (Some(""), false),
            (Some("20.04"), true),
            (Some(">=20.04"), false),
        ] {
            assert_eq!(is_single_version(version), expected);
        }
    }

    // Ported: "isStable(\"$version\") === $expected" — versioning/ubuntu/index.spec.ts line 118
    #[test]
    fn is_stable_matches_renovate_ubuntu_spec() {
        let cases = [
            (None, false),
            (Some(""), false),
            (Some("04.10"), false),
            (Some("05.04"), false),
            (Some("05.10"), false),
            (Some("6.06"), false),
            (Some("6.10"), false),
            (Some("7.04"), false),
            (Some("7.10"), false),
            (Some("8.04"), true),
            (Some("8.10"), false),
            (Some("9.04"), false),
            (Some("9.10"), false),
            (Some("10.04.4"), true),
            (Some("10.10"), false),
            (Some("11.04"), false),
            (Some("11.10"), false),
            (Some("12.04.5"), true),
            (Some("12.10"), false),
            (Some("13.04"), false),
            (Some("13.10"), false),
            (Some("14.04.6"), true),
            (Some("14.10"), false),
            (Some("15.04"), false),
            (Some("15.10"), false),
            (Some("16.04.7"), true),
            (Some("16.10"), false),
            (Some("17.04"), false),
            (Some("17.10"), false),
            (Some("18.04.5"), true),
            (Some("18.10"), false),
            (Some("19.04"), false),
            (Some("19.10"), false),
            (Some("20.04"), true),
            (Some("20.10"), false),
            (Some("22.04"), false),
            (Some("2020.04"), false),
            (Some("warty"), false),
            (Some("hoary"), false),
            (Some("breezy"), false),
            (Some("dapper"), false),
            (Some("edgy"), false),
            (Some("feisty"), false),
            (Some("gutsy"), false),
            (Some("hardy"), true),
            (Some("intrepid"), false),
            (Some("jaunty"), false),
            (Some("karmic"), false),
            (Some("lucid"), true),
            (Some("maverick"), false),
            (Some("natty"), false),
            (Some("oneiric"), false),
            (Some("precise"), true),
            (Some("quantal"), false),
            (Some("raring"), false),
            (Some("saucy"), false),
            (Some("trusty"), true),
            (Some("utopic"), false),
            (Some("vivid"), false),
            (Some("wily"), false),
            (Some("xenial"), true),
            (Some("yakkety"), false),
            (Some("zesty"), false),
            (Some("artful"), false),
            (Some("bionic"), true),
            (Some("cosmic"), false),
            (Some("disco"), false),
            (Some("eoan"), false),
            (Some("focal"), true),
            (Some("groovy"), false),
            (Some("hirsute"), false),
            (Some("impish"), false),
            (Some("jammy"), false),
        ];

        for (version, expected) in cases {
            assert_eq!(
                is_stable_at(version, "2022-04-20"),
                expected,
                "is_stable({version:?})"
            );
        }
    }

    // Ported: "isVersion(\"$version\") === $expected" — versioning/ubuntu/index.spec.ts line 195
    #[test]
    fn is_version_matches_renovate_ubuntu_spec() {
        let cases = [
            (None, false),
            (Some(""), false),
            (Some("02.10"), false),
            (Some("04.10"), true),
            (Some("05.04"), true),
            (Some("6.06"), true),
            (Some("8.04"), true),
            (Some("9.04"), true),
            (Some("10.04.4"), true),
            (Some("12.04.5"), true),
            (Some("13.04"), true),
            (Some("14.04.6"), true),
            (Some("15.04"), true),
            (Some("16.04.7"), true),
            (Some("16.10"), true),
            (Some("17.04"), true),
            (Some("18.04.5"), true),
            (Some("18.10"), true),
            (Some("20.04"), true),
            (Some("20.10"), true),
            (Some("30.11"), true),
            (Some("2020.04"), false),
            (Some("warty"), true),
            (Some("hoary"), true),
            (Some("dapper"), true),
            (Some("hardy"), true),
            (Some("jaunty"), true),
            (Some("lucid"), true),
            (Some("precise"), true),
            (Some("raring"), true),
            (Some("trusty"), true),
            (Some("vivid"), true),
            (Some("xenial"), true),
            (Some("yakkety"), true),
            (Some("zesty"), true),
            (Some("bionic"), true),
            (Some("cosmic"), true),
            (Some("focal"), true),
            (Some("groovy"), true),
            (Some("hirsute"), true),
            (Some("impish"), true),
            (Some("jammy"), true),
            (Some("Groovy"), false),
            (Some("Hirsute"), false),
            (Some("impish-"), false),
            (Some("JAMMY"), false),
        ];

        for (version, expected) in cases {
            assert_eq!(is_version(version), expected, "is_version({version:?})");
        }
    }

    // Ported: "getMajor, getMinor, getPatch for \"$version\"" — versioning/ubuntu/index.spec.ts line 244
    #[test]
    fn component_accessors_match_renovate_ubuntu_spec() {
        let cases = [
            (None, None, None, None),
            (Some(""), None, None, None),
            (Some("42"), None, None, None),
            (Some("2020.04"), None, None, None),
            (Some("04.10"), Some(4), Some(10), None),
            (Some("18.04.5"), Some(18), Some(4), Some(5)),
            (Some("20.04"), Some(20), Some(4), None),
            (Some("intrepid"), Some(8), Some(10), None),
            (Some("bionic"), Some(18), Some(4), None),
            (Some("focal"), Some(20), Some(4), None),
            (Some("jammy-20230816"), Some(22), Some(4), None),
        ];

        for (version, major, minor, patch) in cases {
            assert_eq!(get_major(version), major);
            assert_eq!(get_minor(version), minor);
            assert_eq!(get_patch(version), patch);
        }
    }

    // Ported: "equals($a, $b) === $expected" — versioning/ubuntu/index.spec.ts line 265
    #[test]
    fn equals_matches_renovate_ubuntu_spec() {
        let cases = [
            ("20.04", "2020.04", false),
            ("17.10", "artful", true),
            ("xenial", "artful", false),
            ("17.04", "artful", false),
            ("artful", "17.10", true),
            ("16.04", "xenial", true),
            ("focal", "20.04", true),
            ("20.04", "focal", true),
            ("19.10", "19.10", true),
            ("jammy", "jammy-20230816", false),
            ("jammy-20230816", "jammy-20230816", true),
            ("jammy-20230716", "jammy-20230816", false),
            ("jammy-20230716.1", "jammy-20230716.1", true),
            ("jammy-20230716.1", "jammy-20230716.2", false),
            ("jammy-20230716.1", "jammy-20230816.11", false),
        ];

        for (a, b, expected) in cases {
            assert_eq!(equals(a, b), expected, "equals({a}, {b})");
        }
    }

    // Ported: "isGreaterThan(\"$a\", \"$b\") === $expected" — versioning/ubuntu/index.spec.ts line 287
    #[test]
    fn is_greater_than_matches_renovate_ubuntu_spec() {
        let cases = [
            ("20.04", "20.10", false),
            ("20.10", "20.04", true),
            ("19.10", "20.04", false),
            ("20.04", "19.10", true),
            ("16.04", "16.04.7", false),
            ("16.04.7", "16.04", true),
            ("16.04.1", "16.04.7", false),
            ("16.04.7", "16.04.1", true),
            ("19.10.1", "20.04.1", false),
            ("20.04.1", "19.10.1", true),
            ("xxx", "yyy", false),
            ("focal", "groovy", false),
            ("groovy", "focal", true),
            ("eoan", "focal", false),
            ("focal", "eoan", true),
            ("vivid", "saucy", true),
            ("impish", "focal", true),
            ("eoan", "quantal", true),
            ("focal", "lucid", true),
            ("jammy", "focal", true),
            ("jammy-20230816", "focal", true),
            ("jammy-20230816", "jammy-20230716", true),
            ("jammy-20230716", "jammy-20230816", false),
            ("focal-20230816", "jammy-20230716", false),
            ("zesty-20170517.1", "jammy-20240627.1", false),
            ("jammy-20240627.3", "jammy-20240627.1", true),
            ("jammy-20240627.3", "jammy-20240627.4", false),
            ("jammy-20240627.1", "precise-20150228.11", true),
            ("jammy-20240627", "precise-20150228.11", true),
        ];

        for (a, b, expected) in cases {
            assert_eq!(is_greater_than(a, b), expected, "is_greater_than({a}, {b})");
        }
    }

    // Ported: "getSatisfyingVersion($versions, \"$range\") === \"$expected\"" — versioning/ubuntu/index.spec.ts line 326
    #[test]
    fn get_satisfying_version_matches_renovate_ubuntu_spec() {
        let numeric = ["18.10", "19.04", "19.10", "20.04"];
        let names = ["cosmic", "disco", "eoan", "focal"];
        assert_eq!(get_satisfying_version(&numeric, "2020.04"), None);
        assert_eq!(get_satisfying_version(&numeric, "foobar"), None);
        assert_eq!(get_satisfying_version(&numeric, "20.04"), Some("20.04"));
        assert_eq!(get_satisfying_version(&numeric, "19.10"), Some("19.10"));
        assert_eq!(get_satisfying_version(&numeric, "04.10"), None);
        assert_eq!(get_satisfying_version(&names, "2020.04"), None);
        assert_eq!(get_satisfying_version(&names, "foobar"), None);
        assert_eq!(get_satisfying_version(&names, "focal"), Some("focal"));
        assert_eq!(get_satisfying_version(&names, "eoan"), Some("eoan"));
        assert_eq!(get_satisfying_version(&names, "warty"), None);
    }

    // Ported: "minSatisfyingVersion($versions, \"$range\") === \"$expected\"" — versioning/ubuntu/index.spec.ts line 354
    #[test]
    fn min_satisfying_version_matches_renovate_ubuntu_spec() {
        let numeric = ["18.10", "19.04", "19.10", "20.04"];
        let names = ["cosmic", "disco", "eoan", "focal"];
        assert_eq!(min_satisfying_version(&numeric, "2020.04"), None);
        assert_eq!(min_satisfying_version(&numeric, "foobar"), None);
        assert_eq!(min_satisfying_version(&numeric, "20.04"), Some("20.04"));
        assert_eq!(min_satisfying_version(&numeric, "19.10"), Some("19.10"));
        assert_eq!(min_satisfying_version(&numeric, "04.10"), None);
        assert_eq!(min_satisfying_version(&names, "2020.04"), None);
        assert_eq!(min_satisfying_version(&names, "foobar"), None);
        assert_eq!(min_satisfying_version(&names, "focal"), Some("focal"));
        assert_eq!(min_satisfying_version(&names, "eoan"), Some("eoan"));
        assert_eq!(min_satisfying_version(&names, "warty"), None);
    }

    // Ported: "getNewValue(\"$currentValue\", \"$rangeStrategy\", \"$currentVersion\", \"$newVersion\") === \"$expected\"" — versioning/ubuntu/index.spec.ts line 382
    #[test]
    fn get_new_value_matches_renovate_ubuntu_spec() {
        assert_eq!(get_new_value(None, "foobar"), "foobar");
        assert_eq!(get_new_value(Some("xenial"), "20.04"), "focal");
        assert_eq!(get_new_value(Some("xenial"), "focal"), "focal");
        assert_eq!(get_new_value(Some("16.04"), "20.04"), "20.04");
        assert_eq!(get_new_value(Some("16.04"), "focal"), "20.04");
    }

    // Ported: "$versions -> sortVersions -> $expected" — versioning/ubuntu/index.spec.ts line 404
    #[test]
    fn sort_versions_matches_renovate_ubuntu_spec() {
        let mut numeric = ["17.03", "18.04", "18.04", "6.10", "19.10"];
        numeric.sort_by(|a, b| sort_versions(a, b));
        assert_eq!(numeric, ["6.10", "17.03", "18.04", "18.04", "19.10"]);

        let mut names = ["17.03", "zesty", "bionic", "bionic", "edgy", "eoan"];
        names.sort_by(|a, b| sort_versions(a, b));
        assert_eq!(
            names,
            ["edgy", "17.03", "zesty", "bionic", "bionic", "eoan"]
        );
    }

    // Ported: "matches(\"$version\", \"$range\") === \"$expected\"" — versioning/ubuntu/index.spec.ts line 411
    #[test]
    fn matches_matches_renovate_ubuntu_spec() {
        assert!(!matches("20.04", "2020.04"));
        assert!(matches("20.04", "20.04"));
        assert!(!matches("20.04", "20.04.0"));
    }

    // ── distro.spec.ts — Ubuntu DistroInfo ────────────────────────────────

    // Ported: "isCodename("$version") === $expected" — versioning/distro.spec.ts line 12
    #[test]
    fn distro_is_codename() {
        assert!(is_codename("jammy"));
        assert!(is_codename("impish"));
        assert!(is_codename("hirsute"));
        assert!(is_codename("groovy"));
        assert!(is_codename("focal"));
        assert!(is_codename("eoan"));
        assert!(!is_codename("Wily Werewolf")); // full name, not lowercase series
        assert!(!is_codename("asdf"));
        assert!(!is_codename("Yakkety")); // capitalized
    }

    // Ported: "getVersionByCodename("$version") === $expected" — versioning/distro.spec.ts line 27
    #[test]
    fn distro_get_version_by_codename() {
        assert_eq!(version_by_codename("jammy"), "22.04");
        assert_eq!(version_by_codename("impish"), "21.10");
        assert_eq!(version_by_codename("hirsute"), "21.04");
        assert_eq!(version_by_codename("groovy"), "20.10");
        assert_eq!(version_by_codename("focal"), "20.04");
        assert_eq!(version_by_codename("eoan"), "19.10");
        // Non-codenames return as-is
        assert_eq!(version_by_codename("asd"), "asd");
        assert_eq!(version_by_codename("16.06"), "16.06");
    }

    // Ported: "getCodenameByVersion("$version") === $expected" — versioning/distro.spec.ts line 44
    #[test]
    fn distro_get_codename_by_version() {
        assert_eq!(codename_by_version("22.04"), "jammy");
        assert_eq!(codename_by_version("21.10"), "impish");
        assert_eq!(codename_by_version("21.04"), "hirsute");
        assert_eq!(codename_by_version("20.10"), "groovy");
        assert_eq!(codename_by_version("20.04"), "focal");
        assert_eq!(codename_by_version("19.10"), "eoan");
        // Non-versions return as-is
        assert_eq!(codename_by_version("asd"), "asd");
        assert_eq!(codename_by_version("16.06"), "16.06");
    }

    // Ported: "isReleased("$version") === $expected" — versioning/distro.spec.ts line 98
    // Fixed date: 2021-03-20
    // DistroInfo.isReleased() resolves codenames first via getVersionByCodename().
    #[test]
    fn distro_is_released() {
        let now = "2021-03-20";
        // Helper: convert codename → version, then check is_released
        let check = |input: &str| {
            let ver = version_by_codename(input);
            is_released(ver, now)
        };
        assert!(check("focal"));    // 2020-04-23 < 2021-03-20
        assert!(check("groovy"));   // 2020-10-22 < 2021-03-20
        assert!(!check("hirsute")); // 2021-04-22 > 2021-03-20
        assert!(!check("impish"));  // 2021-10-14 > 2021-03-20
        assert!(!check("jammy"));   // 2022-04-21 > 2021-03-20
        assert!(check("20.04"));
        assert!(check("20.10"));
        assert!(!check("21.04"));
        assert!(!check("21.10"));
        assert!(!check("22.04"));
        assert!(!check("24.04"));   // not in releases list
    }
}
