//! Devbox versioning scheme.
//!
//! Accepts numeric-only versions of 1–3 components (no leading zeros).
//! The special string "latest" is valid but not a concrete version.
//! Comparison treats missing trailing components as equal (1.0.0 == 1.0 == 1).
//!
//! Renovate reference: `lib/modules/versioning/devbox/index.ts`

use std::cmp::Ordering;

/// Parses a 1-3 component version without leading zeros.
/// Returns `None` for 'latest', invalid formats, or leading-zero components.
fn parse_numeric(v: &str) -> Option<Vec<u32>> {
    if v == "latest" {
        return None;
    }
    let parts: Vec<&str> = v.split('.').collect();
    if parts.len() > 3 {
        return None;
    }
    let mut release = Vec::with_capacity(parts.len());
    for part in parts {
        if part.is_empty() {
            return None;
        }
        // No leading zeros: '01' is invalid, '0' is valid.
        if part.len() > 1 && part.starts_with('0') {
            return None;
        }
        let n: u32 = part.parse().ok()?;
        release.push(n);
    }
    Some(release)
}

/// A version is considered a concrete "pin" only when it has exactly 3 components.
fn parse_version(v: &str) -> Option<Vec<u32>> {
    let r = parse_numeric(v)?;
    if r.len() == 3 { Some(r) } else { None }
}

pub fn is_version(version: &str) -> bool {
    parse_version(version).is_some()
}

pub fn is_valid(version: &str) -> bool {
    version == "latest" || parse_numeric(version).is_some()
}

fn compare(version: &str, other: &str) -> Ordering {
    // Special: any valid version equals 'latest'
    if other == "latest" {
        if parse_numeric(version).is_some() {
            return Ordering::Equal;
        }
        return Ordering::Greater;
    }
    let (Some(a), Some(b)) = (parse_numeric(version), parse_numeric(other)) else {
        return Ordering::Greater;
    };
    let len = a.len().max(b.len());
    for i in 0..len {
        match (a.get(i), b.get(i)) {
            (Some(x), Some(y)) => match x.cmp(y) {
                Ordering::Equal => {}
                o => return o,
            },
            // missing component treated as equal
            _ => {}
        }
    }
    Ordering::Equal
}

pub fn equals(version: &str, range: &str) -> bool {
    compare(version, range) == Ordering::Equal
}

pub fn matches(version: &str, range: &str) -> bool {
    is_version(version) && equals(version, range)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isVersion(\"$version\") === $expected" — versioning/devbox/index.spec.ts line 4
    #[test]
    fn is_version_matches_renovate_devbox_spec() {
        let cases = [
            ("1", false),
            ("01", false),
            ("1.01", false),
            ("1.1", false),
            ("1.3.0", true),
            ("2.1.20", true),
            ("v1.4", false),
            ("V0.5", false),
            ("3.5.0", true),
            ("4.2.21.Final", false),
            ("1234", false),
            ("foo", false),
            ("latest", false),
            ("", false),
            ("3.5.0-beta.3", false),
            ("*", false),
            ("x", false),
            ("X", false),
            ("~1.2.3", false),
            (">1.2.3", false),
            ("^1.2.3", false),
            ("1.2.3-foo", false),
            ("1.2.3foo", false),
        ];
        for (version, expected) in cases {
            assert_eq!(is_version(version), expected, "is_version({version:?})");
        }
    }

    // Ported: "isValid(\"$version\") === $isValid" — versioning/devbox/index.spec.ts line 34
    #[test]
    fn is_valid_matches_renovate_devbox_spec() {
        let cases = [
            ("1", true),
            ("01", false),
            ("1.01", false),
            ("1.1", true),
            ("1.3.0", true),
            ("2.1.20", true),
            ("v1.4", false),
            ("V0.5", false),
            ("3.5.0", true),
            ("4.2.21.Final", false),
            ("1234", true),
            ("foo", false),
            ("latest", true),
            ("", false),
            ("3.5.0-beta.3", false),
            ("*", false),
            ("x", false),
            ("X", false),
            ("~1.2.3", false),
            (">1.2.3", false),
            ("^1.2.3", false),
            ("1.2.3-foo", false),
            ("1.2.3foo", false),
        ];
        for (version, expected) in cases {
            assert_eq!(is_valid(version), expected, "is_valid({version:?})");
        }
    }

    // Ported: "matches(\"$version\", \"$range\") === $expected" — versioning/devbox/index.spec.ts line 64
    #[test]
    fn matches_matches_renovate_devbox_spec() {
        let cases = [
            ("1", "1", false),
            ("1", "0", false),
            ("1.2.3", "1", true),
            ("1.2", "1", false),
            ("1.0.0", "1", true),
            ("1.2.0", "1.2", true),
            ("1.2.3", "1.2", true),
            ("0", "latest", false),
            ("1.2.3", "latest", true),
            ("1.2.3.5", "1.2.3.5", false),
            ("1.2", "1.2.3", false),
        ];
        for (version, range, expected) in cases {
            assert_eq!(
                matches(version, range),
                expected,
                "matches({version:?}, {range:?})"
            );
        }
    }

    // Ported: "equals(\"$version\", \"$range\") === $expected" — versioning/devbox/index.spec.ts line 84
    #[test]
    fn equals_matches_renovate_devbox_spec() {
        let cases = [
            ("1", "1", true),
            ("1", "0", false),
            ("1.2.3", "1", true),
            ("1.2", "1", true),
            ("1.0.0", "1", true),
            ("1.2.0", "1.2", true),
            ("1.2.3", "1.2", true),
            ("0", "latest", true),
            ("1.2.3", "latest", true),
            ("1.2.3.5", "1.2.3.5", false),
            ("latest", "latest", false),
            ("latest", "1.2.3", false),
            ("1.2", "1.2.3", true),
        ];
        for (version, range, expected) in cases {
            assert_eq!(
                equals(version, range),
                expected,
                "equals({version:?}, {range:?})"
            );
        }
    }
}
