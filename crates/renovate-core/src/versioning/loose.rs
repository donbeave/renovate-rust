//! Loose versioning scheme.
//!
//! Accepts any `[vV]?MAJOR(.MINOR)*SUFFIX` string.
//! Commit-hash-like strings (7–40 lowercase hex chars that are not pure
//! numeric) are rejected.  Versions with more than 6 numeric components
//! are also rejected.
//!
//! Renovate reference: `lib/modules/versioning/loose/index.ts`

use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
struct LooseVersion {
    release: Vec<u64>,
    suffix: String,
}

/// Regex-free parse matching the TypeScript:
/// `versionPattern = /^[vV]?(\d+(?:\.\d+)*)(.*)$/`
/// `commitHashPattern = /^[a-f0-9]{7,40}$/`
/// `numericPattern = /^[0-9]+$/`
fn parse(v: &str) -> Option<LooseVersion> {
    let len = v.len();
    let is_commit_hash =
        (7..=40).contains(&len) && v.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f'));
    let is_pure_numeric = v.chars().all(|c| c.is_ascii_digit());
    if is_commit_hash && !is_pure_numeric {
        return None;
    }

    let stripped = v.strip_prefix(|c| c == 'v' || c == 'V').unwrap_or(v);

    if stripped.is_empty() || !stripped.starts_with(|c: char| c.is_ascii_digit()) {
        return None;
    }

    // Find the end of the numeric prefix: `\d+(\.\d+)*`
    let mut pos = 0;
    loop {
        let start = pos;
        while pos < stripped.len() && stripped.as_bytes()[pos].is_ascii_digit() {
            pos += 1;
        }
        if pos == start {
            break;
        }
        if pos < stripped.len() && stripped.as_bytes()[pos] == b'.' {
            let next = pos + 1;
            if next < stripped.len() && stripped.as_bytes()[next].is_ascii_digit() {
                pos = next;
                continue;
            }
        }
        break;
    }

    let numeric_part = &stripped[..pos];
    let suffix = stripped[pos..].to_owned();

    let release: Vec<u64> = numeric_part
        .split('.')
        .map(|p| p.parse::<u64>().unwrap_or(0))
        .collect();

    if release.len() > 6 {
        return None;
    }

    Some(LooseVersion { release, suffix })
}

fn natural_compare(a: &str, b: &str) -> Ordering {
    let (mut ai, mut bi) = (a, b);
    loop {
        if ai.is_empty() && bi.is_empty() {
            return Ordering::Equal;
        }
        if ai.is_empty() {
            return Ordering::Less;
        }
        if bi.is_empty() {
            return Ordering::Greater;
        }
        let a_digit = ai.starts_with(|c: char| c.is_ascii_digit());
        let b_digit = bi.starts_with(|c: char| c.is_ascii_digit());
        if a_digit && b_digit {
            let ae = ai.find(|c: char| !c.is_ascii_digit()).unwrap_or(ai.len());
            let be = bi.find(|c: char| !c.is_ascii_digit()).unwrap_or(bi.len());
            let an: u64 = ai[..ae].parse().unwrap_or(0);
            let bn: u64 = bi[..be].parse().unwrap_or(0);
            match an.cmp(&bn) {
                Ordering::Equal => {
                    ai = &ai[ae..];
                    bi = &bi[be..];
                }
                o => return o,
            }
        } else {
            let ac = ai.chars().next().unwrap();
            let bc = bi.chars().next().unwrap();
            match ac.to_ascii_lowercase().cmp(&bc.to_ascii_lowercase()) {
                Ordering::Equal => {
                    ai = &ai[ac.len_utf8()..];
                    bi = &bi[bc.len_utf8()..];
                }
                o => return o,
            }
        }
    }
}

fn compare(left: &LooseVersion, right: &LooseVersion) -> Ordering {
    let len = left.release.len().max(right.release.len());
    for i in 0..len {
        match (left.release.get(i), right.release.get(i)) {
            (None, _) => return Ordering::Less,
            (_, None) => return Ordering::Greater,
            (Some(a), Some(b)) => {
                let c = a.cmp(b);
                if c != Ordering::Equal {
                    return c;
                }
            }
        }
    }

    match (!left.suffix.is_empty(), !right.suffix.is_empty()) {
        (true, true) => natural_compare(&left.suffix, &right.suffix),
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        (false, false) => Ordering::Equal,
    }
}

pub fn is_version(version: &str) -> bool {
    parse(version).is_some()
}

pub fn is_valid(version: &str) -> bool {
    parse(version).is_some()
}

pub fn equals(a: &str, b: &str) -> bool {
    parse(a)
        .zip(parse(b))
        .is_some_and(|(a, b)| compare(&a, &b) == Ordering::Equal)
}

pub fn is_greater_than(a: &str, b: &str) -> bool {
    parse(a)
        .zip(parse(b))
        .is_some_and(|(a, b)| compare(&a, &b) == Ordering::Greater)
}

pub fn is_compatible(version: &str) -> bool {
    is_valid(version)
}

pub fn is_single_version(version: &str) -> bool {
    is_valid(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isVersion(\"$version\") === $expected" — lib/modules/versioning/loose/index.spec.ts line 4
    #[test]
    fn is_version_matches_renovate_loose_spec() {
        let cases = [("1.1", true), ("1.3.RC2", true), ("2.1-rc2", true)];
        for (version, expected) in cases {
            assert_eq!(is_version(version), expected, "is_version({version:?})");
        }
    }

    // Ported: "isValid(\"$version\") === $expected" — lib/modules/versioning/loose/index.spec.ts line 13
    #[test]
    fn is_valid_matches_renovate_loose_spec() {
        let cases = [
            ("v1.4", true),
            ("V0.5", true),
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
            ("foo", false),
            ("1.2.3.4.5.6.7", false),
            ("0a1b2c3", false),
            ("0a1b2c3d", false),
            ("0a1b2c3d4e5f6a7b8c9d0a1b2c3d4e5f6a7b8c9d", false),
            ("0a1b2c3d4e5f6a7b8c9d0a1b2c3d4e5f6a7b8c9d0", true),
            ("0a1b2C3", true),
            ("0z1b2c3", true),
            ("0A1b2c3d4e5f6a7b8c9d0a1b2c3d4e5f6a7b8c9d", true),
            ("123098140293", true),
        ];
        for (version, expected) in cases {
            assert_eq!(is_valid(version), expected, "is_valid({version:?})");
        }
    }

    // Ported: "equals(\"$a\", \"$b\") === $expected" — lib/modules/versioning/loose/index.spec.ts line 41
    #[test]
    fn equals_matches_renovate_loose_spec() {
        let cases = [
            ("2.4", "2.4", true),
            ("2.4.0", "2.4.0", true),
            ("2.4.0", "2.4", false),
            ("2.4.1", "2.4", false),
            ("2.4.2", "2.4.1", false),
        ];
        for (a, b, expected) in cases {
            assert_eq!(equals(a, b), expected, "equals({a:?}, {b:?})");
        }
    }

    // Ported: "isGreaterThan(\"$a\", \"$b\") === $expected" — lib/modules/versioning/loose/index.spec.ts line 52
    #[test]
    fn is_greater_than_matches_renovate_loose_spec() {
        let cases = [
            ("2.4.0", "2.4", true),
            ("2.4.2", "2.4.1", true),
            ("2.4.100", "2.4.99", true),
            ("2.4.beta", "2.4.alpha", true),
            ("1.9", "2", false),
            ("1.9", "1.9.1", false),
            ("2.4", "2.4.beta", true),
            ("2.4.0", "2.4.beta", true),
            ("2.4.beta", "2.4", false),
            ("2.4.beta", "2.4.0", false),
            (
                "2024-07-21T11-33-05.abc123",
                "2023-06-21T11-33-05.abc123",
                true,
            ),
            (
                "2023-07-21T11-33-05.abc123",
                "2023-07-21T11-33-04.abc123",
                true,
            ),
            ("2023-07-21-113305-abc123", "2023-07-21-113304-abc123", true),
            ("1.1.5-100", "1.1.5-99", true),
        ];
        for (a, b, expected) in cases {
            assert_eq!(
                is_greater_than(a, b),
                expected,
                "is_greater_than({a:?}, {b:?})"
            );
        }
    }

    // Ported: "isCompatible(\"$version\") === $expected" — lib/modules/versioning/loose/index.spec.ts line 72
    #[test]
    fn is_compatible_matches_renovate_loose_spec() {
        assert!(is_compatible("1.2.0"));
    }

    // Ported: "isSingleVersion(\"$version\") === $expected" — lib/modules/versioning/loose/index.spec.ts line 79
    #[test]
    fn is_single_version_matches_renovate_loose_spec() {
        assert!(is_single_version("1.2.0"));
        assert!(!is_single_version("^1.2.0"));
    }
}
