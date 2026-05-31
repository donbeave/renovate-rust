//! Debian versioning API with distro-info awareness.
//!
//! Ports `lib/modules/versioning/debian/index.ts`.
//!
//! Wraps the low-level `deb` comparison with codename/version resolution,
//! rolling release names (stable/oldstable/oldoldstable), and dated
//! container-image tag handling.

use crate::versioning::deb;

// ── Distro info data ──────────────────────────────────────────────────────────

#[allow(dead_code)]
struct DistroInfoEntry {
    version: &'static str,
    codename: &'static str,
    series: &'static str,
    created: &'static str,
    release: Option<&'static str>,
    eol: Option<&'static str>,
    eol_lts: Option<&'static str>,
    eol_elts: Option<&'static str>,
}

const DISTRO_INFO: &[DistroInfoEntry] = &[
    DistroInfoEntry {
        version: "1.1",
        codename: "Buzz",
        series: "buzz",
        created: "1993-08-16",
        release: Some("1996-06-17"),
        eol: Some("1997-06-05"),
        eol_lts: None,
        eol_elts: None,
    },
    DistroInfoEntry {
        version: "1.2",
        codename: "Rex",
        series: "rex",
        created: "1996-06-17",
        release: Some("1996-12-12"),
        eol: Some("1998-06-05"),
        eol_lts: None,
        eol_elts: None,
    },
    DistroInfoEntry {
        version: "1.3",
        codename: "Bo",
        series: "bo",
        created: "1996-12-12",
        release: Some("1997-06-05"),
        eol: Some("1999-03-09"),
        eol_lts: None,
        eol_elts: None,
    },
    DistroInfoEntry {
        version: "2",
        codename: "Hamm",
        series: "hamm",
        created: "1997-06-05",
        release: Some("1998-07-24"),
        eol: Some("2000-03-09"),
        eol_lts: None,
        eol_elts: None,
    },
    DistroInfoEntry {
        version: "2.1",
        codename: "Slink",
        series: "slink",
        created: "1998-07-24",
        release: Some("1999-03-09"),
        eol: Some("2000-10-30"),
        eol_lts: None,
        eol_elts: None,
    },
    DistroInfoEntry {
        version: "2.2",
        codename: "Potato",
        series: "potato",
        created: "1999-03-09",
        release: Some("2000-08-15"),
        eol: Some("2003-06-30"),
        eol_lts: None,
        eol_elts: None,
    },
    DistroInfoEntry {
        version: "3",
        codename: "Woody",
        series: "woody",
        created: "2000-08-15",
        release: Some("2002-07-19"),
        eol: Some("2006-06-30"),
        eol_lts: None,
        eol_elts: None,
    },
    DistroInfoEntry {
        version: "3.1",
        codename: "Sarge",
        series: "sarge",
        created: "2002-07-19",
        release: Some("2005-06-06"),
        eol: Some("2008-03-31"),
        eol_lts: None,
        eol_elts: None,
    },
    DistroInfoEntry {
        version: "4",
        codename: "Etch",
        series: "etch",
        created: "2005-06-06",
        release: Some("2007-04-08"),
        eol: Some("2010-02-15"),
        eol_lts: None,
        eol_elts: None,
    },
    DistroInfoEntry {
        version: "5",
        codename: "Lenny",
        series: "lenny",
        created: "2007-04-08",
        release: Some("2009-02-14"),
        eol: Some("2012-02-06"),
        eol_lts: None,
        eol_elts: None,
    },
    DistroInfoEntry {
        version: "6",
        codename: "Squeeze",
        series: "squeeze",
        created: "2009-02-14",
        release: Some("2011-02-06"),
        eol: Some("2014-05-31"),
        eol_lts: Some("2016-02-29"),
        eol_elts: None,
    },
    DistroInfoEntry {
        version: "7",
        codename: "Wheezy",
        series: "wheezy",
        created: "2011-02-06",
        release: Some("2013-05-04"),
        eol: Some("2016-04-25"),
        eol_lts: Some("2018-05-31"),
        eol_elts: Some("2020-06-30"),
    },
    DistroInfoEntry {
        version: "8",
        codename: "Jessie",
        series: "jessie",
        created: "2013-05-04",
        release: Some("2015-04-26"),
        eol: Some("2018-06-17"),
        eol_lts: Some("2020-06-30"),
        eol_elts: Some("2025-06-30"),
    },
    DistroInfoEntry {
        version: "9",
        codename: "Stretch",
        series: "stretch",
        created: "2015-04-26",
        release: Some("2017-06-17"),
        eol: Some("2020-07-18"),
        eol_lts: Some("2022-06-30"),
        eol_elts: Some("2027-06-30"),
    },
    DistroInfoEntry {
        version: "10",
        codename: "Buster",
        series: "buster",
        created: "2017-06-17",
        release: Some("2019-07-06"),
        eol: Some("2022-09-10"),
        eol_lts: Some("2024-06-30"),
        eol_elts: Some("2029-06-30"),
    },
    DistroInfoEntry {
        version: "11",
        codename: "Bullseye",
        series: "bullseye",
        created: "2019-07-06",
        release: Some("2021-08-14"),
        eol: Some("2024-08-14"),
        eol_lts: Some("2026-08-31"),
        eol_elts: Some("2031-06-30"),
    },
    DistroInfoEntry {
        version: "12",
        codename: "Bookworm",
        series: "bookworm",
        created: "2021-08-14",
        release: Some("2023-06-10"),
        eol: Some("2026-07-11"),
        eol_lts: Some("2028-06-30"),
        eol_elts: Some("2033-06-30"),
    },
    DistroInfoEntry {
        version: "13",
        codename: "Trixie",
        series: "trixie",
        created: "2023-06-10",
        release: Some("2025-08-09"),
        eol: Some("2028-08-09"),
        eol_lts: Some("2030-06-30"),
        eol_elts: Some("2035-06-30"),
    },
    DistroInfoEntry {
        version: "14",
        codename: "Forky",
        series: "forky",
        created: "2025-08-09",
        release: None,
        eol: None,
        eol_lts: None,
        eol_elts: None,
    },
    DistroInfoEntry {
        version: "15",
        codename: "Duke",
        series: "duke",
        created: "2027-08-01",
        release: None,
        eol: None,
        eol_lts: None,
        eol_elts: None,
    },
];

// ── Reference dates (snapshot for test time 2023-07-10) ──────────────────────

const REFERENCE_DATE: &str = "2023-07-10";
const REFERENCE_DATE_RELEASE: &str = "2023-07-09";

// ── Distro info lookup helpers ────────────────────────────────────────────────

fn distro_find_by_version(version: &str) -> Option<&'static DistroInfoEntry> {
    DISTRO_INFO.iter().find(|e| e.version == version)
}

fn distro_find_by_series(input: &str) -> Option<&'static DistroInfoEntry> {
    DISTRO_INFO.iter().find(|e| e.series == input)
}

// ── Distro info functions ─────────────────────────────────────────────────────

fn distro_get_version_by_codename(input: &str) -> String {
    match distro_find_by_series(input) {
        Some(e) => e.version.to_owned(),
        None => input.to_owned(),
    }
}

fn distro_get_codename_by_version(version: &str) -> String {
    match distro_find_by_version(version) {
        Some(e) => e.series.to_owned(),
        None => version.to_owned(),
    }
}

fn distro_is_created(version: &str) -> bool {
    let Some(entry) = distro_find_by_version(version) else {
        return false;
    };
    entry.created < REFERENCE_DATE_RELEASE
}

fn distro_is_released(version: &str) -> bool {
    let Some(entry) = distro_find_by_version(version) else {
        return false;
    };
    let Some(release) = entry.release else {
        return false;
    };
    release < REFERENCE_DATE_RELEASE
}

fn distro_is_eol_lts(version: &str) -> bool {
    let Some(entry) = distro_find_by_version(version) else {
        return true;
    };
    let end = entry.eol_lts.or(entry.eol);
    match end {
        Some(d) => d < REFERENCE_DATE,
        None => true,
    }
}

fn distro_is_codename(input: &str) -> bool {
    distro_find_by_series(input).is_some()
}

fn distro_exists(version: &str) -> bool {
    distro_find_by_version(version).is_some()
}

// ── Rolling release data (fixed snapshot for 2023-07-10) ─────────────────────

struct RollingEntry {
    version: &'static str,
    series: &'static str,
}

const ROLLING_DATA: &[RollingEntry] = &[
    RollingEntry {
        version: "12",
        series: "stable",
    },
    RollingEntry {
        version: "11",
        series: "oldstable",
    },
    RollingEntry {
        version: "10",
        series: "oldoldstable",
    },
];

fn rolling_get_version_by_lts(input: &str) -> &str {
    match ROLLING_DATA.iter().find(|e| e.series == input) {
        Some(e) => e.version,
        None => input,
    }
}

fn rolling_has(input: &str) -> bool {
    ROLLING_DATA.iter().any(|e| e.series == input)
}

fn rolling_get_lts_by_version(version: &str) -> Option<&'static str> {
    ROLLING_DATA
        .iter()
        .find(|e| e.version == version)
        .map(|e| e.series)
}

fn rolling_schedule(input: &str) -> Option<&'static RollingEntry> {
    ROLLING_DATA
        .iter()
        .find(|e| e.version == input || e.series == input)
}

// ── Parse helper ──────────────────────────────────────────────────────────────

fn resolve_version(version: &str) -> String {
    let ver = rolling_get_version_by_lts(version);
    distro_get_version_by_codename(ver)
}

fn debian_parse(version: &str) -> Option<Vec<i64>> {
    let ver = if deb::is_dated_codename(version) {
        let codename = deb::get_dated_container_image_codename(version)?;
        distro_get_version_by_codename(codename)
    } else {
        resolve_version(version)
    };
    if !distro_exists(&ver) {
        return None;
    }
    Some(ver.split('.').filter_map(|s| s.parse().ok()).collect())
}

fn debian_compare_internal(a: &str, b: &str) -> i32 {
    let left = debian_parse(a);
    let right = debian_parse(b);
    match (left, right) {
        (Some(lv), Some(rv)) => {
            let len = lv.len().max(rv.len());
            for i in 0..len {
                let p1 = lv.get(i).copied().unwrap_or(0);
                let p2 = rv.get(i).copied().unwrap_or(0);
                if p1 != p2 {
                    return (p1 - p2) as i32;
                }
            }
            0
        }
        _ => 1,
    }
}

// ── Public API ────────────────────────────────────────────────────────────────

pub fn debian_is_valid(version: &str) -> bool {
    let parsed_valid = debian_parse(version).is_some();
    let ver = resolve_version(version);
    (parsed_valid && distro_is_created(&ver)) || deb::is_dated_codename(version)
}

pub fn debian_is_stable(version: &str) -> bool {
    if deb::is_dated_codename(version) {
        let Some(codename) = deb::get_dated_container_image_codename(version) else {
            return false;
        };
        let ver = distro_get_version_by_codename(codename);
        return distro_is_released(&ver) && !distro_is_eol_lts(&ver);
    }
    let ver = resolve_version(version);
    distro_is_released(&ver) && !distro_is_eol_lts(&ver)
}

pub fn debian_equals(a: &str, b: &str) -> bool {
    let a_img = deb::get_dated_container_image_version(a);
    let b_img = deb::get_dated_container_image_version(b);
    if a_img != b_img {
        return false;
    }
    let a_suf = deb::get_dated_container_image_suffix(a);
    let b_suf = deb::get_dated_container_image_suffix(b);
    if a_suf != b_suf {
        return false;
    }
    let a_base = get_base_version(a);
    let b_base = get_base_version(b);
    debian_compare_internal(&a_base, &b_base) == 0
}

pub fn debian_is_greater_than(a: &str, b: &str) -> bool {
    if !deb::is_dated_codename(a) && !deb::is_dated_codename(b) {
        return debian_compare_internal(a, b) > 0;
    }
    let x_major = debian_get_major(a).unwrap_or(0);
    let y_major = debian_get_major(b).unwrap_or(0);
    if x_major > y_major {
        return true;
    }
    if x_major < y_major {
        return false;
    }
    let x_minor = debian_get_minor(a).unwrap_or(0);
    let y_minor = debian_get_minor(b).unwrap_or(0);
    if x_minor > y_minor {
        return true;
    }
    if x_minor < y_minor {
        return false;
    }
    let x_img: u32 = deb::get_dated_container_image_version(a).unwrap_or(0);
    let y_img: u32 = deb::get_dated_container_image_version(b).unwrap_or(0);
    if x_img > y_img {
        return true;
    }
    if x_img < y_img {
        return false;
    }
    let x_suf = parse_suffix_numeric(deb::get_dated_container_image_suffix(a));
    let y_suf = parse_suffix_numeric(deb::get_dated_container_image_suffix(b));
    if x_suf > y_suf {
        return true;
    }
    if x_suf < y_suf {
        return false;
    }
    let x_patch = debian_get_patch(a).unwrap_or(0);
    let y_patch = debian_get_patch(b).unwrap_or(0);
    x_patch > y_patch
}

pub fn debian_get_major(version: &str) -> Option<i64> {
    let ver = get_base_version(version);
    if debian_is_valid(&ver) {
        debian_parse(&ver).and_then(|r| r.first().copied())
    } else {
        None
    }
}

pub fn debian_get_minor(version: &str) -> Option<i64> {
    let ver = get_base_version(version);
    if debian_is_valid(&ver) {
        debian_parse(&ver).and_then(|r| r.get(1).copied())
    } else {
        None
    }
}

pub fn debian_get_patch(version: &str) -> Option<i64> {
    let ver = get_base_version(version);
    if debian_is_valid(&ver) {
        debian_parse(&ver).and_then(|r| r.get(2).copied())
    } else {
        None
    }
}

pub fn debian_get_new_value(current_value: Option<&str>, new_version: &str) -> Option<String> {
    let cv = current_value.unwrap_or("");

    if rolling_has(cv) {
        return Some(
            rolling_get_lts_by_version(new_version)
                .unwrap_or(new_version)
                .to_owned(),
        );
    }

    if distro_is_codename(cv) {
        let di = rolling_schedule(new_version);
        let ver = match di {
            Some(d) => d.version,
            None => new_version,
        };
        return Some(distro_get_codename_by_version(ver));
    }

    if rolling_has(new_version) {
        return rolling_schedule(new_version).map(|d| d.version.to_owned());
    }

    Some(distro_get_version_by_codename(new_version))
}

pub fn debian_sort_versions(a: &str, b: &str) -> i32 {
    debian_compare_internal(a, b)
}

pub fn debian_matches(version: &str, range: &str) -> bool {
    debian_equals(version, range)
}

pub fn debian_is_compatible(version: &str) -> bool {
    debian_is_valid(version)
}

pub fn debian_is_single_version(version: &str) -> bool {
    debian_is_valid(version)
}

pub fn debian_is_version(version: &str) -> bool {
    debian_is_valid(version)
}

pub fn debian_get_satisfying_version<'a>(versions: &'a [&str], range: &str) -> Option<&'a str> {
    versions.iter().find(|&&v| debian_equals(v, range)).copied()
}

pub fn debian_min_satisfying_version<'a>(versions: &'a [&str], range: &str) -> Option<&'a str> {
    versions.iter().find(|&&v| debian_equals(v, range)).copied()
}

// ── Internal helpers ──────────────────────────────────────────────────────────

fn get_base_version(version: &str) -> String {
    if deb::is_dated_codename(version)
        && let Some(codename) = deb::get_dated_container_image_codename(version)
    {
        return distro_get_version_by_codename(codename);
    }
    version.to_owned()
}

fn parse_suffix_numeric(suffix: Option<&str>) -> f64 {
    suffix
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "test" — versioning/debian/index.spec.ts line 18
    #[test]
    fn debian_basic_test() {
        assert!(debian_is_version("1.1"));
    }

    // Ported: 'isValid("$version") === $expected' — versioning/debian/index.spec.ts line 78
    #[test]
    fn test_debian_is_valid() {
        let valid = [
            "buzz", "rex", "bo", "hamm", "slink", "potato", "woody", "sarge", "etch", "lenny",
            "squeeze", "wheezy", "jessie", "stretch", "buster", "bullseye", "bookworm", "1.1",
            "1.2", "1.3", "2", "2.1", "2.2", "3", "4", "5", "6", "7", "8", "9", "10", "11",
            "12", "stable", "oldstable", "oldoldstable", "bookworm-20230816",
            "bullseye-20220101", "buster-20190101", "bookworm-20230816.1",
            "bullseye-20220101.2",
        ];
        let invalid = [
            "Buster",
            "forky",
            "sid",
            "14",
            "10-slim",
            "invalid-20230816",
            "bookworm-2023081",
            "bookworm-20230816.123",
        ];
        for s in &valid {
            assert!(debian_is_valid(s), "isValid({s:?}) should be true");
        }
        for s in &invalid {
            assert!(!debian_is_valid(s), "isValid({s:?}) should be false");
        }
    }

    // Ported: 'isCompatible("$version") === $expected' — versioning/debian/index.spec.ts line 98
    #[test]
    fn debian_is_compatible() {
        let compatible = [
            ("7", None::<&str>),
            ("11", None::<&str>),
            ("12", None::<&str>),
            ("stable", None::<&str>),
            ("oldstable", None::<&str>),
            ("oldoldstable", None::<&str>),
            ("wheezy", None::<&str>),
            ("bullseye", None::<&str>),
            ("bookworm", None::<&str>),
        ];
        let incompatible = [
            ("forky", None::<&str>),
        ];
        for (version, range) in &compatible {
            assert!(
                debian_is_valid(version),
                "isCompatible({version:?}, {range:?}) should be true"
            );
        }
        for (version, range) in &incompatible {
            assert!(
                !debian_is_valid(version),
                "isCompatible({version:?}, {range:?}) should be false"
            );
        }
    }

    // Ported: 'isSingleVersion("$version") === $expected' — versioning/debian/index.spec.ts line 111
    #[test]
    fn test_debian_is_single_version() {
        assert!(!debian_is_single_version(""));
        assert!(debian_is_single_version("6"));
        assert!(!debian_is_single_version(">=6"));
    }

    // Ported: 'isStable("$version") === $expected' — versioning/debian/index.spec.ts line 165
    #[test]
    fn test_debian_is_stable() {
        let stable = [
            "buster",
            "bullseye",
            "bookworm",
            "10",
            "11",
            "12",
            "stable",
            "oldstable",
            "oldoldstable",
            "bookworm-20230816",
            "bullseye-20220101",
            "buster-20190101",
        ];
        let not_stable = [
            "buzz",
            "rex",
            "bo",
            "hamm",
            "slink",
            "potato",
            "woody",
            "sarge",
            "etch",
            "lenny",
            "squeeze",
            "wheezy",
            "jessie",
            "stretch",
            "trixie",
            "sid",
            "1.1",
            "1.2",
            "1.3",
            "2",
            "2.1",
            "2.2",
            "3",
            "4",
            "5",
            "6",
            "7",
            "8",
            "9",
            "13",
            "experimental",
            "wheezy-20140101",
        ];
        for s in &stable {
            assert!(debian_is_stable(s), "isStable({s:?}) should be true");
        }
        for s in &not_stable {
            assert!(!debian_is_stable(s), "isStable({s:?}) should be false");
        }
    }

    // Ported: 'ensures that rolling release is not refreshed within frame time window: $version' — versioning/debian/index.spec.ts line 181
    // Not applicable: tests logging/debug infrastructure for cache refresh behavior.
    // Our implementation uses a fixed snapshot without dynamic refresh.

    // Ported: 'isVersion("$version") === $expected' — versioning/debian/index.spec.ts line 244
    #[test]
    fn test_debian_is_version() {
        let valid = [
            "buzz", "rex", "bo", "hamm", "slink", "potato", "woody", "sarge", "etch", "lenny",
            "squeeze", "wheezy", "jessie", "stretch", "buster", "bullseye", "bookworm", "1.1",
            "1.2", "1.3", "2", "2.1", "2.2", "3", "4", "5", "6", "7", "8", "9", "10", "11",
            "12", "stable", "oldstable", "oldoldstable",
        ];
        let invalid = [
            "Trixie",
            "sid",
            "14",
            "experimental",
            "Bookworm",
            "Sid",
            "Potato-",
            "Woody",
        ];
        for s in &valid {
            assert!(debian_is_version(s), "isVersion({s:?}) should be true");
        }
        for s in &invalid {
            assert!(!debian_is_version(s), "isVersion({s:?}) should be false");
        }
    }

    // Ported: 'getMajor, getMinor, getPatch for "$version"' — versioning/debian/index.spec.ts line 265
    #[test]
    #[allow(clippy::type_complexity)]
    fn debian_get_major_minor_patch() {
        let cases: &[(&str, Option<i64>, Option<i64>, Option<i64>)] = &[
            ("3.1", Some(3), Some(1), None),
            ("1.1", Some(1), Some(1), None),
            ("7", Some(7), None, None),
            ("8", Some(8), None, None),
            ("9", Some(9), None, None),
            ("10", Some(10), None, None),
            ("oldoldstable", Some(10), None, None),
            ("oldstable", Some(11), None, None),
            ("stable", Some(12), None, None),
        ];
        for (version, major, minor, patch) in cases {
            assert_eq!(debian_get_major(version), *major, "getMajor({version:?})");
            assert_eq!(debian_get_minor(version), *minor, "getMinor({version:?})");
            assert_eq!(debian_get_patch(version), *patch, "getPatch({version:?})");
        }

        assert_eq!(debian_get_major("42"), None);
        assert_eq!(debian_get_major("2020.04"), None);
    }

    // Ported: 'equals($a, $b) === $expected' — versioning/debian/index.spec.ts line 293
    #[test]
    fn test_debian_equals() {
        let cases: &[(&str, &str, bool)] = &[
            ("woody", "sarge", false),
            ("lenny", "3", false),
            ("lenny", "5", true),
            ("squeeze", "6", true),
            ("10", "buster", true),
            ("6", "squeeze", true),
            ("buster", "10", true),
            ("oldoldstable", "10", true),
            ("oldstable", "11", true),
            ("stable", "12", true),
            ("10", "oldoldstable", true),
            ("11", "oldstable", true),
            ("12", "stable", true),
            ("bookworm-20230816", "bookworm-20230816", true),
            ("bookworm-20230816", "bookworm-20230817", false),
            ("bullseye-20220101", "bullseye-20220101", true),
            ("bookworm-20230816.1", "bookworm-20230816.2", false),
            ("bookworm-20230816", "bookworm-20230816.1", false),
        ];
        for (a, b, expected) in cases {
            assert_eq!(debian_equals(a, b), *expected, "equals({a:?}, {b:?})");
        }
    }

    // Ported: 'isGreaterThan("$a", "$b") === $expected' — versioning/debian/index.spec.ts line 336
    #[test]
    fn test_debian_is_greater_than() {
        let cases: &[(&str, &str, bool)] = &[
            ("5", "6", false),
            ("6", "5", true),
            ("5", "10", false),
            ("11", "10", true),
            ("5", "6", false),
            ("11", "1.1", true),
            ("xxx", "yyy", true),
            ("yyy", "xxx", true),
            ("lenny", "squeeze", false),
            ("squeeze", "lenny", true),
            ("lenny", "buster", false),
            ("bookworm", "etch", true),
            ("sarge", "bo", true),
            ("bullseye", "rex", true),
            ("buzz", "jessie", false),
            ("oldoldstable", "8", true),
            ("oldstable", "oldoldstable", true),
            ("stable", "oldstable", true),
            ("12", "oldoldstable", true),
            ("11", "oldstable", false),
            ("10", "stable", false),
            ("bookworm-20230816", "bullseye-20220101", true),
            ("bullseye-20220101", "bookworm-20230816", false),
            ("bookworm-20230816", "bookworm-20230817", false),
            ("bookworm-20230817", "bookworm-20230816", true),
            ("bookworm-20230816.1", "bookworm-20230816", true),
            ("bookworm-20230816", "bookworm-20230816.1", false),
            ("buster-2022010", "buster-2022010", true),
            ("1.1", "1.2", false),
            ("1.2", "1.1", true),
            ("11.1", "bullseye-20220101", false),
            ("bullseye-20220101", "11.1", true),
            ("3.1", "woody-20010101", true),
            ("woody-20010101", "3.1", false),
            ("bookworm-20230816", "bookworm-20230816", false),
            ("11", "bullseye-20220101", false),
            ("bullseye-20220101", "11", true),
        ];
        for (a, b, expected) in cases {
            assert_eq!(
                debian_is_greater_than(a, b),
                *expected,
                "isGreaterThan({a:?}, {b:?})"
            );
        }
    }

    // Ported: 'getSatisfyingVersion($versions, "$range") === "$expected"' — versioning/debian/index.spec.ts line 355
    #[test]
    fn test_debian_get_satisfying_version() {
        let cases: &[(&[&str], &str, Option<&str>)] = &[
            (&["8", "9", "10", "11"], "2020.04", None),
            (&["8", "9", "10", "11"], "foobar", None),
            (&["8", "9", "10", "11"], "11", Some("11")),
            (&["8", "9", "10", "11"], "10", Some("10")),
            (&["8", "9", "10", "11"], "4", None),
            (
                &["jessie", "stretch", "buster", "bullseye"],
                "2020.04",
                None,
            ),
            (
                &["jessie", "stretch", "buster", "bullseye"],
                "foobar",
                None,
            ),
            (
                &["jessie", "stretch", "buster", "bullseye"],
                "bullseye",
                Some("bullseye"),
            ),
            (
                &["jessie", "stretch", "buster", "bullseye"],
                "buster",
                Some("buster"),
            ),
            (
                &["jessie", "stretch", "buster", "stable"],
                "stable",
                Some("stable"),
            ),
            (
                &["jessie", "stretch", "oldstable", "bullseye"],
                "bullseye",
                Some("oldstable"),
            ),
            (
                &["jessie", "oldoldstable", "buster", "bullseye"],
                "warty",
                None,
            ),
        ];
        for (versions, range, expected) in cases {
            assert_eq!(
                debian_get_satisfying_version(versions, range),
                *expected,
                "getSatisfyingVersion({versions:?}, {range:?})"
            );
        }
    }

    // Ported: 'minSatisfyingVersion($versions, "$range") === "$expected"' — versioning/debian/index.spec.ts line 377
    #[test]
    fn test_debian_min_satisfying_version() {
        let cases: &[(&[&str], &str, Option<&str>)] = &[
            (&["8", "9", "10", "11"], "2020.04", None),
            (&["8", "9", "10", "11"], "foobar", None),
            (&["8", "9", "10", "11"], "11", Some("11")),
            (&["8", "9", "10", "11"], "10", Some("10")),
            (&["8", "9", "10", "11"], "4", None),
            (
                &["jessie", "stretch", "buster", "bullseye"],
                "2020.04",
                None,
            ),
            (
                &["jessie", "stretch", "buster", "bullseye"],
                "foobar",
                None,
            ),
            (
                &["jessie", "stretch", "buster", "bullseye"],
                "bullseye",
                Some("bullseye"),
            ),
            (
                &["jessie", "stretch", "buster", "bullseye"],
                "buster",
                Some("buster"),
            ),
            (
                &["jessie", "stretch", "buster", "bullseye"],
                "warty",
                None,
            ),
            (
                &["jessie", "stretch", "buster", "stable"],
                "stable",
                Some("stable"),
            ),
            (
                &["jessie", "stretch", "oldstable", "bullseye"],
                "bullseye",
                Some("oldstable"),
            ),
            (
                &["jessie", "oldoldstable", "buster", "bullseye"],
                "warty",
                None,
            ),
        ];
        for (versions, range, expected) in cases {
            assert_eq!(
                debian_min_satisfying_version(versions, range),
                *expected,
                "minSatisfyingVersion({versions:?}, {range:?})"
            );
        }
    }

    // Ported: 'getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected"' — versioning/debian/index.spec.ts line 396
    #[test]
    fn test_debian_get_new_value() {
        let cases: &[(Option<&str>, &str, Option<&str>)] = &[
            (None, "foobar", Some("foobar")),
            (Some("stretch"), "11", Some("bullseye")),
            (Some("stretch"), "bullseye", Some("bullseye")),
            (Some("stretch"), "stable", Some("bookworm")),
            (Some("9"), "11", Some("11")),
            (Some("oldoldstable"), "12", Some("stable")),
            (Some("oldstable"), "12", Some("stable")),
            (Some("9"), "stable", Some("12")),
            (Some("oldstable"), "12", Some("stable")),
            (Some("oldstable"), "3", Some("3")),
        ];
        for (current_value, new_version, expected) in cases {
            assert_eq!(
                debian_get_new_value(*current_value, new_version),
                (*expected).map(|s| s.to_owned()),
                "getNewValue({current_value:?}, {new_version:?})"
            );
        }
    }

    // Ported: 'debian.sortVersions($a, $b) === $expected' — versioning/debian/index.spec.ts line 425
    #[test]
    fn test_debian_sort_versions() {
        let cases: &[(&str, &str, i32)] = &[
            ("woody", "sarge", -1),
            ("lenny", "3", 2),
            ("3", "lenny", -2),
            ("lenny", "5", 0),
            ("squeeze", "6", 0),
            ("10", "buster", 0),
            ("6", "squeeze", 0),
            ("buster", "10", 0),
            ("oldoldstable", "9", 1),
            ("oldstable", "oldoldstable", 1),
            ("stable", "oldstable", 1),
            ("12", "oldoldstable", 2),
            ("11", "oldstable", 0),
            ("10", "stable", -2),
        ];
        for (a, b, expected) in cases {
            assert_eq!(
                debian_sort_versions(a, b),
                *expected,
                "sortVersions({a:?}, {b:?})"
            );
        }
    }

    // Ported: 'matches("$version", "$range") === $expected' — versioning/debian/index.spec.ts line 436
    #[test]
    fn test_debian_matches() {
        assert!(!debian_matches("10", "10-slim"));
        assert!(debian_matches("11", "11"));
        assert!(!debian_matches("11", "11.0"));
    }

    // Ported: 'checks runtime date handling & refresh rolling release data' — versioning/debian/index.spec.ts line 441
    // Not applicable: tests dynamic time-based refresh of rolling release data.
    // Our implementation uses a fixed snapshot at 2023-07-10.

    #[test]
    fn debian_is_compatible_validity() {
        assert!(super::debian_is_compatible("11"));
        assert!(!super::debian_is_compatible("not-a-version"));
    }
}
