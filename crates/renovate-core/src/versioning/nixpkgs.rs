//! NixPkgs versioning scheme.
//!
//! Valid versions match:
//!   `^(nixos|nixpkgs|release)-((\d{2})\.(\d{2})|unstable)(-(small|aarch64|darwin))?$`
//!
//! Unstable variants compare as newer than any stable version.
//! Compatibility is determined by matching prefix + suffix (ignoring version number).
//!
//! Renovate reference: `lib/modules/versioning/nixpkgs/index.ts`

use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
struct NixpkgsVersion {
    prefix: String,
    /// YY * 100 + MM for stable, None for unstable.
    release: Option<[u32; 2]>,
    suffix: String,
    /// Prefix + optional suffix, used for compatibility check.
    compatibility: String,
}

fn parse(v: &str) -> Option<NixpkgsVersion> {
    let valid_prefixes = ["nixos", "nixpkgs", "release"];
    let valid_suffixes = ["small", "aarch64", "darwin"];

    // Find prefix
    let (prefix, rest) = valid_prefixes.iter().find_map(|p| {
        v.strip_prefix(*p)
            .and_then(|r| r.strip_prefix('-'))
            .map(|r| (*p, r))
    })?;

    // Parse version part (either XX.YY or "unstable"), optionally followed by -suffix
    let (release, suffix_part) = if rest == "unstable" || rest.starts_with("unstable-") {
        let after = &rest["unstable".len()..];
        (None, after)
    } else if let Some((major_str, after_major)) = rest.split_once('.') {
        if major_str.len() != 2 || !major_str.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }
        // after_major should be minor (2 digits) optionally followed by -suffix
        let (minor_str, after_minor) = if let Some(pos) = after_major.find('-') {
            (&after_major[..pos], &after_major[pos..])
        } else {
            (after_major, "")
        };
        if minor_str.len() != 2 || !minor_str.chars().all(|c| c.is_ascii_digit()) {
            return None;
        }
        let major: u32 = major_str.parse().ok()?;
        let minor: u32 = minor_str.parse().ok()?;
        (Some([major, minor]), after_minor)
    } else {
        return None;
    };

    // Parse optional suffix
    let suffix = if suffix_part.is_empty() {
        String::new()
    } else {
        let suf = suffix_part.strip_prefix('-')?;
        if !valid_suffixes.contains(&suf) {
            return None;
        }
        suf.to_owned()
    };

    let compatibility = if suffix.is_empty() {
        prefix.to_owned()
    } else {
        format!("{prefix}-{suffix}")
    };

    Some(NixpkgsVersion {
        prefix: prefix.to_owned(),
        release,
        suffix,
        compatibility,
    })
}

fn compare(a: &NixpkgsVersion, b: &NixpkgsVersion) -> Ordering {
    match (a.release.as_ref(), b.release.as_ref()) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Greater, // unstable > stable
        (Some(_), None) => Ordering::Less,
        (Some(ar), Some(br)) => ar.cmp(br),
    }
}

pub fn is_valid(version: &str) -> bool {
    parse(version).is_some()
}

pub fn is_stable(version: &str) -> bool {
    parse(version).is_some_and(|v| v.release.is_some())
}

pub fn equals(a: &str, b: &str) -> bool {
    parse(a).zip(parse(b)).is_some_and(|(a, b)| {
        compare(&a, &b) == Ordering::Equal && a.compatibility == b.compatibility
    })
}

pub fn sort_versions(a: &str, b: &str) -> Ordering {
    parse(a)
        .zip(parse(b))
        .map(|(a, b)| compare(&a, &b))
        .unwrap_or(Ordering::Equal)
}

/// Returns true when `a` is compatible with `b` — they share the same prefix and suffix.
pub fn is_compatible(a: &str, b: &str) -> bool {
    parse(a)
        .zip(parse(b))
        .is_some_and(|(a, b)| a.compatibility == b.compatibility)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isValid(\"$version\") === $expected" — versioning/nixpkgs/index.spec.ts line 6
    #[test]
    fn is_valid_matches_renovate_nixpkgs_spec() {
        let cases = [
            ("1.2.3", false),
            ("22.05", false),
            ("release-22.05", true),
            ("nixos-22.05", true),
            ("nixos-22.05-small", true),
            ("nixos-22.05-aarch64", true),
            ("nixos-22.05-aarch64-small", false),
            ("nixpkgs-22.05-darwin", true),
            ("nixpkgs-22.05-darwin-aarch64", false),
            ("nixos-unstable", true),
            ("nixos-unstable-small", true),
            ("nixpkgs-unstable", true),
            ("nixos-22.05.1234", false),
            ("nixos-22.05-1234", false),
            ("nixos-22.05-unknown", false),
            ("unknown-22.05", false),
            ("nixos-nixpkgs-22.05", false),
        ];
        for (version, expected) in cases {
            assert_eq!(is_valid(version), expected, "is_valid({version:?})");
        }
    }

    // Ported: "isStable(\"$version\") === $expected" — versioning/nixpkgs/index.spec.ts line 32
    #[test]
    fn is_stable_matches_renovate_nixpkgs_spec() {
        let cases = [
            ("release-22.05", true),
            ("nixos-22.05", true),
            ("nixos-22.05-small", true),
            ("nixos-22.05-aarch64", true),
            ("nixpkgs-22.05-darwin", true),
            ("nixos-unstable", false),
            ("nixos-unstable-small", false),
            ("nixpkgs-unstable", false),
        ];
        for (version, expected) in cases {
            assert_eq!(is_stable(version), expected, "is_stable({version:?})");
        }
    }

    // Ported: "equals($a, $b) === $expected" — versioning/nixpkgs/index.spec.ts line 49
    #[test]
    fn equals_matches_renovate_nixpkgs_spec() {
        let cases = [
            ("nixos-22.05", "nixos-22.05", true),
            ("nixos-22.05", "nixos-21.11", false),
            ("nixos-22.05", "nixos-unstable", false),
            ("nixos-unstable", "nixos-unstable", true),
            ("nixos-unstable-small", "nixos-unstable-small", true),
        ];
        for (a, b, expected) in cases {
            assert_eq!(equals(a, b), expected, "equals({a:?}, {b:?})");
        }
    }

    // Ported: "$versions -> sortVersions -> $expected" — versioning/nixpkgs/index.spec.ts line 60
    #[test]
    fn sort_versions_matches_renovate_nixpkgs_spec() {
        let mut versions = vec![
            "nixos-21.11",
            "nixos-22.05",
            "nixos-22.05-small",
            "nixos-unstable",
            "nixos-unstable-small",
        ];
        versions.sort_by(|a, b| sort_versions(a, b));
        assert_eq!(
            versions,
            [
                "nixos-21.11",
                "nixos-22.05",
                "nixos-22.05-small",
                "nixos-unstable",
                "nixos-unstable-small"
            ]
        );
    }

    // Ported: "equals($a, $b) === $expected" — versioning/nixpkgs/index.spec.ts line 72
    #[test]
    fn is_compatible_matches_renovate_nixpkgs_spec() {
        let cases = [
            ("nixos-22.05", "nixos-22.05", true),
            ("nixos-22.05", "nixpkgs-22.05", false),
            ("nixos-22.05", "nixos-21.11", true),
            ("nixos-22.05", "nixos-unstable", true),
            ("nixos-22.05", "nixos-22.05-small", false),
            ("nixos-22.05", "nixos-22.05-aarch64", false),
            ("nixos-22.05", "nixos-22.05-darwin", false),
        ];
        for (a, b, expected) in cases {
            assert_eq!(is_compatible(a, b), expected, "is_compatible({a:?}, {b:?})");
        }
    }
}
