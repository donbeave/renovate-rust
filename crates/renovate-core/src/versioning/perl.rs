//! Perl versioning scheme.
//!
//! Supports two formats:
//! - Decimal: `N.DDDDD[_DDD]` — integer part + decimal string in groups of 3
//! - Dotted-decimal: `v?N(.N)*[_N]` — dot-separated components
//!
//! A version containing `_` is a prerelease (unstable).
//!
//! Renovate reference: `lib/modules/versioning/perl/index.ts`

use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
struct PerlVersion {
    release: Vec<u64>,
    /// Non-empty string when version is a prerelease (has `_`).
    prerelease: String,
}

/// `^(\d+)\.(\d+(?:_\d+)?)$` — decimal format
fn parse_decimal(v: &str) -> Option<PerlVersion> {
    let (int_str, dec_str) = v.split_once('.')?;

    // int part: all digits
    if int_str.is_empty() || !int_str.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    // dec part: digits optionally followed by _digits, no other underscores
    let underscore_count = dec_str.chars().filter(|&c| c == '_').count();
    if underscore_count > 1 {
        return None;
    }
    if underscore_count == 1 {
        let (before, after) = dec_str.split_once('_')?;
        if before.is_empty() || after.is_empty() {
            return None;
        }
        if !before.chars().all(|c| c.is_ascii_digit()) || !after.chars().all(|c| c.is_ascii_digit())
        {
            return None;
        }
    } else if !dec_str.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    if dec_str.is_empty() {
        return None;
    }

    let prerelease = if dec_str.contains('_') {
        "alpha".to_owned()
    } else {
        String::new()
    };

    // Expand decimal part in groups of 3 (right-padded)
    let pure_digits = dec_str.replace('_', "");
    let mut components: Vec<u64> = pure_digits
        .as_bytes()
        .chunks(3)
        .map(|chunk| {
            let mut s = std::str::from_utf8(chunk).unwrap().to_owned();
            while s.len() < 3 {
                s.push('0');
            }
            s.parse::<u64>().unwrap_or(0)
        })
        .collect();

    let int_part: u64 = int_str.parse().ok()?;
    let mut release = vec![int_part];
    release.append(&mut components);

    Some(PerlVersion {
        release,
        prerelease,
    })
}

/// `^v?(\d+(?:\.\d+)*(?:_\d+)?)$` — dotted-decimal format
fn parse_dotted(v: &str) -> Option<PerlVersion> {
    let stripped = v.strip_prefix('v').unwrap_or(v);

    if stripped.is_empty() {
        return None;
    }

    // Count underscores — only one allowed, at end
    let underscore_count = stripped.chars().filter(|&c| c == '_').count();
    if underscore_count > 1 {
        return None;
    }

    let prerelease = if underscore_count == 1 {
        "alpha".to_owned()
    } else {
        String::new()
    };

    // Split on `.` and `_`
    let parts: Vec<&str> = stripped.split(['.', '_']).collect();
    for part in &parts {
        if part.is_empty() || !part.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }
    }

    // Validate underscore position: must be the last separator
    if underscore_count == 1 {
        let under_pos = stripped.rfind('_').unwrap();
        // After underscore must be only digits (no further dots)
        let after = &stripped[under_pos + 1..];
        if after.is_empty() || after.contains('.') {
            return None;
        }
    }

    let release: Vec<u64> = parts.iter().map(|p| p.parse().unwrap_or(0)).collect();

    Some(PerlVersion {
        release,
        prerelease,
    })
}

fn parse(v: &str) -> Option<PerlVersion> {
    parse_decimal(v).or_else(|| parse_dotted(v))
}

fn compare_releases(a: &[u64], b: &[u64]) -> Ordering {
    let len = a.len().max(b.len());
    for i in 0..len {
        let av = a.get(i).copied().unwrap_or(0);
        let bv = b.get(i).copied().unwrap_or(0);
        match av.cmp(&bv) {
            Ordering::Equal => {}
            o => return o,
        }
    }
    Ordering::Equal
}

fn compare(a: &PerlVersion, b: &PerlVersion) -> Ordering {
    let rel = compare_releases(&a.release, &b.release);
    if rel != Ordering::Equal {
        return rel;
    }
    match (!a.prerelease.is_empty(), !b.prerelease.is_empty()) {
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        _ => Ordering::Equal,
    }
}

pub fn is_valid(version: &str) -> bool {
    parse(version).is_some()
}

pub fn is_stable(version: &str) -> bool {
    parse(version).is_some_and(|v| v.prerelease.is_empty())
}

pub fn equals(a: &str, b: &str) -> bool {
    parse(a)
        .zip(parse(b))
        .is_some_and(|(a, b)| compare(&a, &b) == Ordering::Equal)
}

pub fn is_greater_than(a: &str, b: &str) -> bool {
    // undefined (None) for `a` is treated as greater (consistent with TS: `undefined > b → true`)
    match (parse(a), parse(b)) {
        (None, _) => true,
        (_, None) => false,
        (Some(pa), Some(pb)) => compare(&pa, &pb) == Ordering::Greater,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isValid(\"$input\") === $expected" — versioning/perl/index.spec.ts line 4
    #[test]
    fn is_valid_matches_renovate_perl_spec() {
        let cases = [
            ("1", true),
            ("1.2", true),
            ("1.02", true),
            ("1.002", true),
            ("1.0023", true),
            ("1.00203", true),
            ("1.002003", true),
            ("1.002_003", true),
            ("1._002003", false),
            ("1.002003_", false),
            ("1.00_20_03", false),
            ("v1", true),
            ("v1.200", true),
            ("v1.20.0", true),
            ("v1.2.3", true),
            ("1.2.3", true),
            ("v1.2_3", true),
            ("v1._23", false),
            ("v1.23_", false),
            ("v1.2_3_4", false),
        ];
        for (input, expected) in cases {
            assert_eq!(is_valid(input), expected, "is_valid({input:?})");
        }
    }

    // Ported: "isStable(\"$input\") === $expected" — versioning/perl/index.spec.ts line 31
    #[test]
    fn is_stable_matches_renovate_perl_spec() {
        let cases = [
            ("1", true),
            ("1.234", true),
            ("1.2_34", false),
            ("v1", true),
            ("v1.2", true),
            ("v1.2.3", true),
            ("v1.2.3_4", false),
        ];
        for (input, expected) in cases {
            assert_eq!(is_stable(input), expected, "is_stable({input:?})");
        }
    }

    // Ported: "equals($a, $b) === $expected" — versioning/perl/index.spec.ts line 44
    #[test]
    fn equals_matches_renovate_perl_spec() {
        let cases = [
            ("1.2", "v1.200.0", true),
            ("1.02", "v1.20.0", true),
            ("1.002", "v1.2.0", true),
            ("1.0023", "v1.2.300", true),
            ("1.00203", "v1.2.30", true),
            ("1.002003", "v1.2.3", true),
            ("1.02_03", "1.020_3", true),
            ("1.02_03", "v1.20_300", true),
        ];
        for (a, b, expected) in cases {
            assert_eq!(equals(a, b), expected, "equals({a:?}, {b:?})");
        }
    }

    // Ported: "isGreaterThan($a, $b) === $expected" — versioning/perl/index.spec.ts line 58
    #[test]
    fn is_greater_than_matches_renovate_perl_spec() {
        let cases = [
            ("2.4.2", "2.4.1", true),
            ("0.1301", "0.13_01", true),
            ("0.13_01", "0.1301", false),
            ("1.900", "2.000", false),
            ("1.900", "1.901", false),
            ("1.2.0.1", "1.2.0", true),
            ("1.2.0", "1.2.0.1", false),
            // undefined (None parse) > any valid version
        ];
        for (a, b, expected) in cases {
            assert_eq!(
                is_greater_than(a, b),
                expected,
                "is_greater_than({a:?}, {b:?})"
            );
        }
        // special: undefined > "1.2.0"
        assert!(is_greater_than("not_a_version_at_all_xyz", "1.2.0"));
    }
}
