//! Unity3D Packages versioning.
//!
//! Versions follow `MAJOR.MINOR.PATCH[-LABEL]`. A label starting with
//! `exp.`, `pre.`, or `preview.` marks the version as a pre-release;
//! all other labels (including numeric suffixes like `-4`) are stable.
//!
//! Renovate reference: `lib/modules/versioning/unity3d-packages/index.ts`

use std::cmp::Ordering;

/// Parsed unity3d-packages version.
#[derive(Debug, Clone, PartialEq, Eq)]
struct U3dPkgVersion {
    major: u32,
    minor: u32,
    patch: u32,
    /// Non-empty only when the label starts with `exp.`, `pre.`, or `preview.`.
    prerelease: String,
}

fn parse(v: &str) -> Option<U3dPkgVersion> {
    let (numeric_part, label) = match v.split_once('-') {
        Some((n, l)) => (n, l),
        None => (v, ""),
    };

    let mut parts = numeric_part.split('.');
    let major: u32 = parts.next()?.parse().ok()?;
    let minor: u32 = parts.next()?.parse().ok()?;
    let patch: u32 = parts.next()?.parse().ok()?;
    if parts.next().is_some() {
        return None;
    }

    let is_unstable =
        label.starts_with("exp.") || label.starts_with("pre.") || label.starts_with("preview.");

    Some(U3dPkgVersion {
        major,
        minor,
        patch,
        prerelease: if is_unstable {
            label.to_owned()
        } else {
            String::new()
        },
    })
}

/// Compare two prerelease label strings with numeric-aware ordering,
/// mirroring JavaScript `localeCompare(…, { numeric: true })`.
fn natural_compare(a: &str, b: &str) -> Ordering {
    let mut ai = a;
    let mut bi = b;
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
            let a_end = ai.find(|c: char| !c.is_ascii_digit()).unwrap_or(ai.len());
            let b_end = bi.find(|c: char| !c.is_ascii_digit()).unwrap_or(bi.len());
            let a_n: u64 = ai[..a_end].parse().unwrap_or(0);
            let b_n: u64 = bi[..b_end].parse().unwrap_or(0);
            match a_n.cmp(&b_n) {
                Ordering::Equal => {
                    ai = &ai[a_end..];
                    bi = &bi[b_end..];
                }
                other => return other,
            }
        } else {
            let ac = ai.chars().next().unwrap();
            let bc = bi.chars().next().unwrap();
            match ac.to_ascii_lowercase().cmp(&bc.to_ascii_lowercase()) {
                Ordering::Equal => {
                    ai = &ai[ac.len_utf8()..];
                    bi = &bi[bc.len_utf8()..];
                }
                other => return other,
            }
        }
    }
}

fn compare(a: &U3dPkgVersion, b: &U3dPkgVersion) -> Ordering {
    let rel = (a.major, a.minor, a.patch).cmp(&(b.major, b.minor, b.patch));
    if rel != Ordering::Equal {
        return rel;
    }

    match (!a.prerelease.is_empty(), !b.prerelease.is_empty()) {
        (true, true) => natural_compare(&a.prerelease, &b.prerelease),
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        (false, false) => Ordering::Equal,
    }
}

pub fn is_valid(input: &str) -> bool {
    parse(input).is_some()
}

pub fn is_stable(input: &str) -> bool {
    parse(input).is_some_and(|v| v.prerelease.is_empty())
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

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isValid(\"$input\") === $expected" — lib/modules/versioning/unity3d-packages/index.spec.ts line 4
    #[test]
    fn is_valid_matches_renovate_unity3d_packages_spec() {
        let cases = [
            ("1", false),
            ("1.2", false),
            ("1.2.3", true),
            ("1.2.3-4", true),
            ("1.2.3-exp.1", true),
            ("1.2.3-pre.1", true),
            ("1.2.3-preview.1", true),
        ];
        for (input, expected) in cases {
            assert_eq!(is_valid(input), expected, "is_valid({input:?})");
        }
    }

    // Ported: "isStable(\"$input\") === $expected" — lib/modules/versioning/unity3d-packages/index.spec.ts line 18
    #[test]
    fn is_stable_matches_renovate_unity3d_packages_spec() {
        let cases = [
            ("1.2.3", true),
            ("1.2.3-4", true),
            ("1.2.3-exp.1", false),
            ("1.2.3-pre.1", false),
            ("1.2.3-preview.1", false),
        ];
        for (input, expected) in cases {
            assert_eq!(is_stable(input), expected, "is_stable({input:?})");
        }
    }

    // Ported: "equals($a, $b) === $expected" — lib/modules/versioning/unity3d-packages/index.spec.ts line 29
    #[test]
    fn equals_matches_renovate_unity3d_packages_spec() {
        let cases = [
            ("1.2.3", "1.2.3", true),
            ("1.2.3-4", "1.2.3-4", true),
            ("1.2.3-exp.1", "1.2.3-exp.1", true),
            ("1.2.3-pre.1", "1.2.3-pre.1", true),
            ("1.2.3-preview.1", "1.2.3-preview.1", true),
        ];
        for (a, b, expected) in cases {
            assert_eq!(equals(a, b), expected, "equals({a:?}, {b:?})");
        }
    }

    // Ported: "isGreaterThan($a, $b) === $expected" — lib/modules/versioning/unity3d-packages/index.spec.ts line 40
    #[test]
    fn is_greater_than_matches_renovate_unity3d_packages_spec() {
        let cases = [
            ("1.2.4", "1.2.3", true),
            ("1.2.3-exp.1", "1.2.3", false),
            ("1.2.3", "1.2.3-1", false),
            ("1.2.3-exp.10", "1.2.3-exp.2", true),
            ("1.2.3-exp.2", "1.2.3-exp.1", true),
            ("1.2.3-pre.1", "1.2.3-exp.2", true),
            ("1.2.3-pre.10", "1.2.3-pre.2", true),
            ("1.2.3-pre.2", "1.2.3-pre.1", true),
            ("1.2.3", "1.2.3-pre.2", true),
        ];
        for (a, b, expected) in cases {
            assert_eq!(
                is_greater_than(a, b),
                expected,
                "is_greater_than({a:?}, {b:?})"
            );
        }
    }
}
