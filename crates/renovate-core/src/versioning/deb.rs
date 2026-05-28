//! Debian package versioning.
//!
//! Ports `lib/modules/versioning/deb/index.ts` and
//! `lib/modules/versioning/debian/common.ts`.
//!
//! Format: `[epoch:]upstream-version[-debian-revision]`
//!
//! Comparison uses the dpkg algorithm:
//! alternating non-digit/digit blocks with a custom character order for
//! non-digit characters.

use std::sync::LazyLock;

use regex::Regex;

// ── Debian container image helpers (lib/modules/versioning/debian/common.ts) ──

/// Regex for dated Debian container image tags like `buster-20220101` or
/// `bookworm-20230816.1`.
static DATED_CODENAME_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<codename>\w+)-(?P<date>\d{8})(?P<suffix>\.\d{1,2})?$").unwrap()
});

/// Known Debian release codenames (series), sourced from
/// `data/debian-distro-info.json`.
const DEBIAN_CODENAMES: &[&str] = &[
    "buzz", "rex", "bo", "hamm", "slink", "potato", "woody", "sarge", "etch", "lenny", "squeeze",
    "wheezy", "jessie", "stretch", "buster", "bullseye", "bookworm", "trixie", "forky", "duke",
];

/// Return `true` when `input` is a dated container image tag whose codename
/// component is a recognised Debian release series.
///
/// Mirrors `isDatedCodeName` from `lib/modules/versioning/debian/common.ts`.
pub fn is_dated_codename(input: &str) -> bool {
    let Some(caps) = DATED_CODENAME_RE.captures(input) else {
        return false;
    };
    let codename = caps.name("codename").expect("named group").as_str();
    DEBIAN_CODENAMES.contains(&codename)
}

/// Extract the codename component from a dated container image tag such as
/// `buster-20220101`.  Returns `None` when the input does not match the
/// expected pattern.
///
/// Mirrors `getDatedContainerImageCodename` from
/// `lib/modules/versioning/debian/common.ts`.
pub fn get_dated_container_image_codename(version: &str) -> Option<&str> {
    let caps = DATED_CODENAME_RE.captures(version)?;
    Some(caps.name("codename")?.as_str())
}

/// Extract the date component from a dated container image tag such as
/// `buster-20220101`, returning it as a `u32` (e.g. `20220101`).  Returns
/// `None` when the input does not match.
///
/// Mirrors `getDatedContainerImageVersion` from
/// `lib/modules/versioning/debian/common.ts`.
pub fn get_dated_container_image_version(version: &str) -> Option<u32> {
    let caps = DATED_CODENAME_RE.captures(version)?;
    caps.name("date")?.as_str().parse().ok()
}

/// Extract the optional suffix component (e.g. `.1`) from a dated container
/// image tag such as `bookworm-20230816.1`.  Returns `None` when absent.
///
/// Mirrors `getDatedContainerImageSuffix` from
/// `lib/modules/versioning/debian/common.ts`.
pub fn get_dated_container_image_suffix(version: &str) -> Option<&str> {
    let caps = DATED_CODENAME_RE.captures(version)?;
    Some(caps.name("suffix")?.as_str())
}

// ── Character ordering ────────────────────────────────────────────────────────

/// Custom character order for non-digit characters in dpkg comparison.
/// `~` < ` ` (empty/digit-treated-as-space) < `A-Z` < `a-z` < `+` < `-` < `.` < `:`
const CHARACTER_ORDER: &str = "~ ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz+-.:";

fn char_priority(c: char, is_empty_or_digit: bool) -> i32 {
    if is_empty_or_digit {
        // digits and empty string are treated like space (position 1)
        1
    } else {
        CHARACTER_ORDER.find(c).map(|i| i as i32).unwrap_or(-1) // unknown chars sort before ~ (shouldn't happen for valid versions)
    }
}

// ── dpkg comparison algorithm ─────────────────────────────────────────────────

/// Compare two Debian version sub-strings (upstream or revision) using
/// the dpkg algorithm.
fn compare_string(a: &str, b: &str) -> i32 {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let len = a_chars.len().max(b_chars.len());
    let mut pos = 0;

    while pos < len {
        let ac = a_chars.get(pos).copied();
        let bc = b_chars.get(pos).copied();

        let a_is_digit = ac.is_some_and(|c| c.is_ascii_digit());
        let b_is_digit = bc.is_some_and(|c| c.is_ascii_digit());

        if a_is_digit && b_is_digit {
            // Numeric block comparison: find end of numeric block for both
            let mut a_end = pos + 1;
            while a_chars.get(a_end).is_some_and(|c| c.is_ascii_digit()) {
                a_end += 1;
            }
            let mut b_end = pos + 1;
            while b_chars.get(b_end).is_some_and(|c| c.is_ascii_digit()) {
                b_end += 1;
            }
            let a_num: u64 = a_chars[pos..a_end]
                .iter()
                .collect::<String>()
                .parse()
                .unwrap_or(0);
            let b_num: u64 = b_chars[pos..b_end]
                .iter()
                .collect::<String>()
                .parse()
                .unwrap_or(0);
            if a_num != b_num {
                return if a_num < b_num { -1 } else { 1 };
            }
            // Both blocks equal; advance to the end of whichever is longer.
            pos = a_end.max(b_end);
            continue;
        }

        let a_char_or_empty = ac;
        let b_char_or_empty = bc;

        if a_char_or_empty != b_char_or_empty {
            let a_prio = char_priority(
                a_char_or_empty.unwrap_or(' '),
                a_char_or_empty.is_none() || a_is_digit,
            );
            let b_prio = char_priority(
                b_char_or_empty.unwrap_or(' '),
                b_char_or_empty.is_none() || b_is_digit,
            );
            return (a_prio - b_prio).signum();
        }
        pos += 1;
    }
    0
}

// ── Parsing ───────────────────────────────────────────────────────────────────

fn valid_epoch(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

fn valid_upstream(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| matches!(c, '-' | '+' | '.' | ':' | '~' | 'A'..='Z' | 'a'..='z' | '0'..='9'))
}

fn valid_revision(s: &str) -> bool {
    s.chars()
        .all(|c| matches!(c, '+' | '.' | '~' | 'A'..='Z' | 'a'..='z' | '0'..='9'))
}

#[derive(Debug, Clone)]
struct DebVersion {
    epoch: u32,
    upstream: String,
    revision: String,
    release: Vec<i64>,
}

fn parse(version: &str) -> Option<DebVersion> {
    if version.is_empty() {
        return None;
    }

    // Split epoch on first `:`.
    let (epoch_str, remainder) = if let Some(pos) = version.find(':') {
        (&version[..pos], &version[pos + 1..])
    } else {
        ("0", version)
    };

    // Epoch must be all digits.
    if !valid_epoch(epoch_str) {
        return None;
    }
    let epoch: u32 = epoch_str.parse().ok()?;

    // Forbid trailing `-` (would produce empty debian-revision).
    if remainder.ends_with('-') {
        return None;
    }

    // Split debian revision on LAST `-`.
    let (upstream, revision) = if let Some(pos) = remainder.rfind('-') {
        (&remainder[..pos], &remainder[pos + 1..])
    } else {
        (remainder, "")
    };

    if !valid_upstream(upstream) || !valid_revision(revision) {
        return None;
    }

    // Extract all numeric sequences from the full remainder as the "release" array.
    let release: Vec<i64> = {
        let mut nums = Vec::new();
        let mut in_num = false;
        let mut start = 0;
        for (i, c) in remainder.char_indices() {
            if c.is_ascii_digit() {
                if !in_num {
                    start = i;
                    in_num = true;
                }
            } else if in_num {
                let n: i64 = remainder[start..i].parse().unwrap_or(0);
                nums.push(n);
                in_num = false;
            }
        }
        if in_num {
            let n: i64 = remainder[start..].parse().unwrap_or(0);
            nums.push(n);
        }
        nums
    };

    Some(DebVersion {
        epoch,
        upstream: upstream.to_owned(),
        revision: revision.to_owned(),
        release,
    })
}

// ── Core comparison ───────────────────────────────────────────────────────────

/// Compare two Debian version strings.
///
/// Returns 1 when either version is invalid (mirrors the TypeScript base class).
fn compare(a: &str, b: &str) -> i32 {
    let va = parse(a);
    let vb = parse(b);
    match (va, vb) {
        (Some(av), Some(bv)) => {
            // Compare epochs.
            if av.epoch != bv.epoch {
                return if av.epoch < bv.epoch { -1 } else { 1 };
            }
            // Compare upstream versions.
            let up_cmp = compare_string(&av.upstream, &bv.upstream);
            if up_cmp != 0 {
                return up_cmp;
            }
            // Compare debian revisions.
            compare_string(&av.revision, &bv.revision)
        }
        _ => 1, // invalid version → always returns 1 (matches TS behaviour)
    }
}

// ── Public API ────────────────────────────────────────────────────────────────

pub fn is_valid(version: &str) -> bool {
    parse(version).is_some()
}

pub fn equals(a: &str, b: &str) -> bool {
    compare(a, b) == 0
}

pub fn is_greater_than(a: &str, b: &str) -> bool {
    compare(a, b) > 0
}

/// Every valid deb version is a single version (no range support).
pub fn is_single_version(version: &str) -> bool {
    is_valid(version)
}

pub fn get_major(version: &str) -> Option<i64> {
    parse(version)?.release.first().copied()
}

pub fn get_minor(version: &str) -> Option<i64> {
    let v = parse(version)?;
    if v.release.len() >= 2 {
        Some(v.release[1])
    } else {
        None
    }
}

pub fn get_patch(version: &str) -> Option<i64> {
    let v = parse(version)?;
    if v.release.len() >= 3 {
        Some(v.release[2])
    } else {
        None
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: 'isValid("$version") === $expected' — versioning/deb/index.spec.ts line 4
    #[test]
    fn deb_is_valid() {
        let valid = [
            "1.1",
            "1.3.RC2",
            "0:1.1-1",
            "1:1:1:2-1",
            "1:a:b:c:2-1",
            "2:1.1-1",
            "1.1.1-0debian1",
            "1.1.1+really1.1.2-0debian1",
            "2.31-13+deb11u5",
            "1:4.14+20190211-1ubuntu1",
            "2.7.7+dfsg-12",
            "9.5.0-1ubuntu1~22.04",
            "5:20.10.17~3-0~ubuntu-focal",
            "1:6.0.1r16-1.1build1",
            "2:102.12+LibO7.3.7-0ubuntu0.22.04.1",
            "1:2.20.1-1~bpo9+1",
            "v1.4",
            "3.5.0",
            "4.2.21.Final",
            "0.6.5.1",
            "20100527",
            "2.1.0-M3",
            "4.3.20.RELEASE",
            "1.1-groovy-2.4",
            "0.8a",
            "3.1.0.GA",
            "3.0.0-beta.3",
            "foo",
            "1.2.3.4.5.6.7",
            "0a1b2c3",
            "0a1b2c3d",
            "0a1b2c3d4e5f6a7b8c9d0a1b2c3d4e5f6a7b8c9d",
            "0a1b2c3d4e5f6a7b8c9d0a1b2c3d4e5f6a7b8c9d0",
            "0a1b2C3",
            "0z1b2c3",
            "0A1b2c3d4e5f6a7b8c9d0a1b2c3d4e5f6a7b8c9d",
            "123098140293",
        ];
        let invalid = [
            "a:1.1-1",
            "1.1:1.3-1",
            "1.1a:1.3-1",
            "1a:1.3-1",
            "-1:1.3-1",
            "1:3_3.2-1",
            "1:3!3.2-1",
            "1:3/3.2-1",
            "1.0-3_2",
            "1.0-3!3",
            "1.0-3/3",
            "1.0+ä1-1",
            "1,0-1",
        ];
        for s in &valid {
            assert!(is_valid(s), "isValid({s:?}) should be true");
        }
        for s in &invalid {
            assert!(!is_valid(s), "isValid({s:?}) should be false");
        }
    }

    // Ported: 'equals("$a", "$b") === $expected' — versioning/deb/index.spec.ts line 60
    #[test]
    fn deb_equals() {
        let cases: &[(&str, &str, bool)] = &[
            ("2.4", "2.4", true),
            ("2.4.0", "2.4.0", true),
            ("2.4.0", "2.4", false),
            ("2.4.1", "2.4", false),
            ("2.4.2", "2.4.1", false),
            ("0.8a", "0.8a", true),
            ("9.5.0-1ubuntu1~22.04", "9.5.0-1ubuntu1", false),
            ("9.5.0-1ubuntu1~22.04", "9.5.0-1ubuntu1~20.04", false),
            ("9.5.0-1ubuntu1~22.04", "9.5.0-1ubuntu1~22.04", true),
            ("2.31-13+deb11u5", "2.31-13+deb11u5", true),
            ("2.31-13+deb11u5", "2.31-13+deb11u4", false),
            ("1.4-", "1.4", false),  // invalid → 1 != 0
            ("v1.4", "1.4", false),  // different upstream versions
            ("0:1.4", "1.4", true),  // epoch 0 == default epoch 0
            ("1:1.4", "1.4", false), // epoch 1 != 0
            ("1.4-1", "1.4-2", false),
            ("0:1.4", "a:1.4", false), // a:1.4 invalid
            ("a:1.4", "0:1.4", false), // a:1.4 invalid
        ];
        for (a, b, expected) in cases {
            assert_eq!(equals(a, b), *expected, "equals({a:?}, {b:?})");
        }
    }

    // Ported: 'isGreaterThan("$a", "$b") === $expected' — versioning/deb/index.spec.ts line 84
    #[test]
    fn deb_is_greater_than() {
        let cases: &[(&str, &str, bool)] = &[
            ("2.4.0", "2.4", true),
            ("2.4.2", "2.4.1", true),
            ("2.4.beta", "2.4.alpha", true),
            ("1.9", "2", false),
            ("1.9", "1.9.1", false),
            ("2.4", "2.4.beta", false),
            ("2.4.0", "2.4.beta", false),
            ("2.4.beta", "2.4", true),
            ("2.4.beta", "2.4.0", true),
            ("2.4~", "2.4~~", true),
            ("2.4", "2.4~", true),
            ("2.4a", "2.4", true),
            ("2.31-13+deb11u5", "2.31-9", true),
            ("2.31-13+deb11u5", "2.31-13+deb10u5", true),
            ("2.31-13+deb11u5", "2.31-13+deb11u4", true),
            ("1.9", "1:1.7", false), // epoch 0 < epoch 1
            ("1.9", "1.12", false),
            ("1.12", "1.9", true),
            ("1:1.9", "1:1.7", true),
            ("2.4.0.beta1", "2.4.0.Beta1", true), // b > B in char order
            ("1:1.0", "1:1.0~", true),
            ("1:1.0Z0-0", "1:1.0", true),
            ("1:1.0Z0-0", "1:1.0A0-0", true),
            ("1:1.0a0-0", "1:1.0Z0-0", true), // a > Z
            ("1:1.0z0-0", "1:1.0a0-0", true),
            ("1:1.0+0-0", "1:1.0z0-0", true), // + > z
            ("1:1.0-0-0", "1:1.0+0-0", true), // - > +
            ("1:1.0.0-0", "1:1.0-0-0", true), // . > -
            ("1:1.0:0-0", "1:1.0.0-0", true), // : > .
            ("a:1.4", "0:1.4", true),         // a:1.4 invalid → 1
            ("0:1.4", "a:1.4", true),         // a:1.4 invalid → 1
            ("a:1.4", "a:1.4", true),         // both invalid → 1
            ("a1", "a~", true),               // 1/digit (pos 1) > ~ (pos 0)
            ("a0", "a~", true),
            ("aa", "a1", true), // a (pos 28) > digit-treated-as-space (pos 1)
            ("ab", "a0", true),
            ("10", "1.", true), // digit block: 10 vs 1 then '.'. Actually...
            ("10", "1a", true), // 10 > 1 in numeric; then '' vs 'a'
        ];
        for (a, b, expected) in cases {
            assert_eq!(
                is_greater_than(a, b),
                *expected,
                "isGreaterThan({a:?}, {b:?})"
            );
        }
    }

    // Ported: 'isSingleVersion("$version") === $expected' — versioning/deb/index.spec.ts line 128
    #[test]
    fn deb_is_single_version() {
        assert!(is_single_version("1.2.0"));
        assert!(!is_single_version("^1.2.0")); // ^ not valid in deb charset
    }

    // Ported: 'getMajor("$version") === $expected' — versioning/deb/index.spec.ts line 136
    #[test]
    fn deb_get_major() {
        assert_eq!(get_major("v1.3.0"), Some(1));
        assert_eq!(get_major("2-0-1"), Some(2));
        assert_eq!(get_major("2.31-13+deb11u5"), Some(2));
        assert_eq!(get_major("1:2.3.1"), Some(2));
        assert_eq!(get_major("foo"), None);
        assert_eq!(get_major("8"), Some(8));
        assert_eq!(get_major("1.0"), Some(1));
    }

    // Ported: 'getMinor("$version") === $expected' — versioning/deb/index.spec.ts line 149
    #[test]
    fn deb_get_minor() {
        assert_eq!(get_minor("v1.3.0"), Some(3));
        assert_eq!(get_minor("2-0-1"), Some(0));
        assert_eq!(get_minor("2.31-13+deb11u5"), Some(31));
        assert_eq!(get_minor("1:2.3.1"), Some(3));
        assert_eq!(get_minor("foo"), None);
        assert_eq!(get_minor("8"), None);
        assert_eq!(get_minor("1.0"), Some(0));
    }

    // Ported: 'getPatch("$version") === $expected' — versioning/deb/index.spec.ts line 162
    #[test]
    fn deb_get_patch() {
        assert_eq!(get_patch("v1.3.0"), Some(0));
        assert_eq!(get_patch("2-0-1"), Some(1));
        assert_eq!(get_patch("2.31-13+deb11u5"), Some(13));
        assert_eq!(get_patch("1:2.3.1"), Some(1));
        assert_eq!(get_patch("foo"), None);
        assert_eq!(get_patch("8"), None);
        assert_eq!(get_patch("1.0"), None);
    }

    // Ported: 'isDatedCodeName("$input") === $expected' — versioning/debian/common.spec.ts line 31
    #[test]
    fn debian_is_dated_codename() {
        assert!(!is_dated_codename("buster"));
        assert!(is_dated_codename("buster-20220101"));
        assert!(is_dated_codename("bullseye-20220101"));
        assert!(is_dated_codename("bookworm-20230816"));
        assert!(is_dated_codename("bookworm-20230816.1"));
        assert!(!is_dated_codename("invalid-20220101"));
        assert!(!is_dated_codename("buster-2022010"));
        assert!(!is_dated_codename("buster-202201011"));
        assert!(!is_dated_codename("buster-20220101.123"));
    }

    // Ported: 'getDatedContainerImageCodename("$input") === $expected' — versioning/debian/common.spec.ts line 48
    #[test]
    fn debian_get_dated_container_image_codename() {
        assert_eq!(
            get_dated_container_image_codename("buster-20220101"),
            Some("buster")
        );
        assert_eq!(
            get_dated_container_image_codename("bullseye-20220101"),
            Some("bullseye")
        );
        assert_eq!(
            get_dated_container_image_codename("bookworm-20230816"),
            Some("bookworm")
        );
        assert_eq!(
            get_dated_container_image_codename("bookworm-20230816.1"),
            Some("bookworm")
        );
        assert_eq!(get_dated_container_image_codename("buster"), None);
        assert_eq!(
            get_dated_container_image_codename("invalid-20220101"),
            Some("invalid")
        );
        assert_eq!(get_dated_container_image_codename("buster-2022010"), None);
        assert_eq!(
            get_dated_container_image_codename("buster-20220101.123"),
            None
        );
        assert_eq!(get_dated_container_image_codename("buster-20220101a"), None);
        assert_eq!(get_dated_container_image_codename("buster-20220101-"), None);
    }

    // Ported: 'getDatedContainerImageVersion("$input") === $expected' — versioning/debian/common.spec.ts line 69
    #[test]
    fn debian_get_dated_container_image_version() {
        assert_eq!(
            get_dated_container_image_version("buster-20220101"),
            Some(20220101)
        );
        assert_eq!(
            get_dated_container_image_version("bullseye-20220101"),
            Some(20220101)
        );
        assert_eq!(
            get_dated_container_image_version("bookworm-20230816"),
            Some(20230816)
        );
        assert_eq!(
            get_dated_container_image_version("bookworm-20230816.1"),
            Some(20230816)
        );
        assert_eq!(get_dated_container_image_version("buster"), None);
        assert_eq!(
            get_dated_container_image_version("invalid-20220101"),
            Some(20220101)
        );
        assert_eq!(get_dated_container_image_version("buster-2022010"), None);
    }

    // Ported: 'getDatedContainerImageSuffix("$input") === $expected' — versioning/debian/common.spec.ts line 87
    #[test]
    fn debian_get_dated_container_image_suffix() {
        assert_eq!(get_dated_container_image_suffix("buster-20220101"), None);
        assert_eq!(get_dated_container_image_suffix("bullseye-20220101"), None);
        assert_eq!(get_dated_container_image_suffix("bookworm-20230816"), None);
        assert_eq!(
            get_dated_container_image_suffix("bookworm-20230816.1"),
            Some(".1")
        );
        assert_eq!(
            get_dated_container_image_suffix("buster-20220101.2"),
            Some(".2")
        );
        assert_eq!(get_dated_container_image_suffix("buster"), None);
        assert_eq!(get_dated_container_image_suffix("invalid-20220101"), None);
        assert_eq!(get_dated_container_image_suffix("buster-2022010"), None);
    }
}
