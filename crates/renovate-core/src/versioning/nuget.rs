//! NuGet versioning.
//!
//! NuGet uses a 4-part version scheme: `Major.Minor.Patch[.Revision][-PreRelease]`.
//! The 4th component (Revision) is optional and defaults to 0. Two versions are
//! equal if all four numeric components match, regardless of whether Revision was
//! written explicitly.
//!
//! Renovate reference:
//! - `lib/modules/versioning/nuget/version.ts` — `compare`, `parseVersion`
//! - `lib/modules/versioning/nuget/index.ts` — `isStable`
//!
//! ## Algorithm
//!
//! 1. Split the version string on `-` to separate the numeric part from any
//!    pre-release label.
//! 2. Split the numeric part on `.` to get up to 4 components; pad with 0s.
//! 3. Compare component-by-component; if all match, a version with a pre-release
//!    label is considered LESS THAN one without.
//! 4. `update_available` is `true` when `latest > current`.

use std::cmp::Ordering;

/// Update summary produced by [`nuget_update_summary`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NuGetUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Produce an update summary for a NuGet dependency.
pub fn nuget_update_summary(current_value: &str, latest: Option<&str>) -> NuGetUpdateSummary {
    let update_available = latest
        .filter(|l| !l.is_empty() && !current_value.is_empty())
        .is_some_and(|latest_str| compare(latest_str, current_value) == Ordering::Greater);

    NuGetUpdateSummary {
        current_value: current_value.to_owned(),
        latest: latest.map(|s| s.to_owned()),
        update_available,
    }
}

/// Compare two NuGet version strings.
///
/// Returns `Ordering::Greater` if `a > b`, `Ordering::Less` if `a < b`,
/// `Ordering::Equal` otherwise.
pub fn compare(a: &str, b: &str) -> Ordering {
    let pa = parse(a);
    let pb = parse(b);

    // Compare the 4 numeric components first.
    for i in 0..4 {
        let cmp = pa.components[i].cmp(&pb.components[i]);
        if cmp != Ordering::Equal {
            return cmp;
        }
    }

    // Numeric parts equal: stable (no pre-release) > pre-release.
    match (&pa.prerelease, &pb.prerelease) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (Some(la), Some(lb)) => compare_prerelease(la, lb),
    }
}

/// Returns `true` when the version is a stable release (no pre-release label).
pub fn is_stable(version: &str) -> bool {
    if version.trim().is_empty() || version.contains('*') {
        return false;
    }

    parse(version).prerelease.is_none()
}

// ── Internal ──────────────────────────────────────────────────────────────────

struct ParsedVersion {
    /// Exactly 4 components, padded with 0 for missing parts.
    components: [u64; 4],
    prerelease: Option<String>,
}

fn parse(version: &str) -> ParsedVersion {
    let version = version.trim();
    let version = version.split_once('+').map_or(version, |(base, _)| base);

    // Split off pre-release label at first `-`.
    let (numeric, prerelease) = if let Some(pos) = version.find('-') {
        (
            &version[..pos],
            Some(version[pos + 1..].to_ascii_lowercase()),
        )
    } else {
        (version, None)
    };

    let parts: Vec<&str> = numeric.split('.').collect();
    let mut components = [0u64; 4];
    for (i, part) in parts.iter().take(4).enumerate() {
        components[i] = part.parse().unwrap_or(0);
    }

    ParsedVersion {
        components,
        prerelease,
    }
}

fn compare_prerelease(a: &str, b: &str) -> Ordering {
    let mut a_parts = a.split('.');
    let mut b_parts = b.split('.');

    loop {
        match (a_parts.next(), b_parts.next()) {
            (Some(a_part), Some(b_part)) => {
                let cmp = match (a_part.parse::<u64>(), b_part.parse::<u64>()) {
                    (Ok(a_num), Ok(b_num)) => a_num.cmp(&b_num),
                    _ => a_part.cmp(b_part),
                };
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return Ordering::Equal,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    // ── compare ──────────────────────────────────────────────────────────────

    #[test]
    fn equal_versions() {
        assert_eq!(compare("1.2.3", "1.2.3"), Ordering::Equal);
    }

    #[test]
    fn revision_zero_equals_three_part() {
        // 1.2.3.0 == 1.2.3 (revision defaults to 0)
        assert_eq!(compare("1.2.3.0", "1.2.3"), Ordering::Equal);
        assert_eq!(compare("1.2.3", "1.2.3.0"), Ordering::Equal);
    }

    #[test]
    fn newer_patch() {
        assert_eq!(compare("1.2.4", "1.2.3"), Ordering::Greater);
        assert_eq!(compare("1.2.3", "1.2.4"), Ordering::Less);
    }

    #[test]
    fn newer_minor() {
        assert_eq!(compare("1.3.0", "1.2.9"), Ordering::Greater);
    }

    #[test]
    fn newer_major() {
        assert_eq!(compare("2.0.0", "1.9.9"), Ordering::Greater);
    }

    #[test]
    fn revision_bump() {
        assert_eq!(compare("1.2.3.1", "1.2.3.0"), Ordering::Greater);
        assert_eq!(compare("1.2.3.1", "1.2.3"), Ordering::Greater);
    }

    #[test]
    fn stable_greater_than_prerelease() {
        assert_eq!(compare("1.2.3", "1.2.3-alpha"), Ordering::Greater);
        assert_eq!(compare("1.2.3-rc1", "1.2.3"), Ordering::Less);
    }

    #[test]
    fn prerelease_ordering() {
        // alpha < beta < rc alphabetically
        assert_eq!(compare("1.0.0-beta", "1.0.0-alpha"), Ordering::Greater);
        assert_eq!(compare("1.0.0-alpha", "1.0.0-beta"), Ordering::Less);
    }

    // Ported: "compare($x, $y) === $expected" — versioning/nuget/version.spec.ts line 4
    #[test]
    fn compare_matches_renovate_version_spec() {
        let cases = [
            ("17.4", "17.04", Ordering::Equal),
            ("1.4", "1.4.0", Ordering::Equal),
            ("1.0.110", "1.0.110.0", Ordering::Equal),
            ("1.0.0", "1.0.0+c30d7625", Ordering::Equal),
            ("1.022", "1.22.0.0", Ordering::Equal),
            ("23.2.3", "23.2.3.0", Ordering::Equal),
            ("1.3.42.10133", "1.3.42.10133", Ordering::Equal),
            ("1.0", "1.0.0.0", Ordering::Equal),
            ("1.23.01", "1.23.1", Ordering::Equal),
            ("1.45.6", "1.45.6.0", Ordering::Equal),
            ("1.45.6-Alpha", "1.45.6-Alpha", Ordering::Equal),
            ("1.6.2-BeTa", "1.6.02-beta", Ordering::Equal),
            ("22.3.07     ", "22.3.07", Ordering::Equal),
            ("1.0", "1.0.0.0+beta", Ordering::Equal),
            ("1.0.0.0+beta.2", "1.0.0.0+beta.1", Ordering::Equal),
            ("1.0.0", "1.0.0", Ordering::Equal),
            ("1.0.0-BETA", "1.0.0-beta", Ordering::Equal),
            ("1.0.0-BETA+AA", "1.0.0-beta+aa", Ordering::Equal),
            (
                "1.0.0-BETA.X.y.5.77.0+AA",
                "1.0.0-beta.x.y.5.77.0+aa",
                Ordering::Equal,
            ),
            ("1.0.0", "1.0.0+beta", Ordering::Equal),
            ("1.0", "1.0.0.0", Ordering::Equal),
            ("1.0+test", "1.0.0.0", Ordering::Equal),
            ("1.0.0.1-1.2.A", "1.0.0.1-1.2.a+A", Ordering::Equal),
            ("1.0.01", "1.0.1.0", Ordering::Equal),
            ("0.0.0", "1.0.0", Ordering::Less),
            ("1.1.0", "1.0.0", Ordering::Greater),
            ("1.0.1", "1.0.0", Ordering::Greater),
            ("1.0.0-BETA", "1.0.0-beta2", Ordering::Less),
            ("1.0.0+AA", "1.0.0-beta+aa", Ordering::Greater),
            ("1.0.0-BETA+AA", "1.0.0-beta", Ordering::Equal),
            (
                "1.0.0-BETA.X.y.5.77.0+AA",
                "1.0.0-beta.x.y.5.79.0+aa",
                Ordering::Less,
            ),
            ("1.2.3.4-RC+99", "1.2.3.4-RC+99", Ordering::Equal),
            ("1.2.3", "1.2.3", Ordering::Equal),
            ("1.2.3-Pre.2", "1.2.3-Pre.2", Ordering::Equal),
            ("1.2.3+99", "1.2.3+99", Ordering::Equal),
            ("1.2-Pre", "1.2.0-Pre", Ordering::Equal),
            ("2.4.2", "2.4.1", Ordering::Greater),
            ("2.4-beta", "2.4-alpha", Ordering::Greater),
            ("1.9", "2", Ordering::Less),
            ("1.9", "1.9.1", Ordering::Less),
            ("2.4.0", "2.4.0-beta", Ordering::Greater),
            ("2.4.0-alpha", "2.4.0", Ordering::Less),
            ("1.2.0-beta.333", "1.2.0-beta.66", Ordering::Greater),
            ("1.2.0-beta2", "1.2.0-beta10", Ordering::Greater),
            ("1.2.0.1", "1.2.0", Ordering::Greater),
            ("1.2.0.1", "1.2.0.1-beta", Ordering::Greater),
            ("1.2.0.1-beta", "1.2.0.1", Ordering::Less),
            ("1.2.0+1", "1.2.0", Ordering::Equal),
            ("1.2.0", "1.2.0+1", Ordering::Equal),
            ("1-a", "1-0", Ordering::Greater),
            ("1-a.b", "1-a", Ordering::Greater),
            ("1-a", "1-a.b", Ordering::Less),
            ("1.0.1", "1.0", Ordering::Greater),
            ("1.231", "1.23", Ordering::Greater),
            ("1.45.6", "1.4.5.6", Ordering::Greater),
            ("1.4.5.60", "1.4.5.6", Ordering::Greater),
            ("1.10", "1.01", Ordering::Greater),
            ("1.10-beta", "1.01-alpha", Ordering::Greater),
            ("1.10.0-rc-2", "1.01.0-RC-1", Ordering::Greater),
            ("1.01", "1.01-RC-1", Ordering::Greater),
            ("1.2-preview", "1.01", Ordering::Greater),
            ("1.0.0", "0.0.0", Ordering::Greater),
            ("1.1.0", "1.0.0", Ordering::Greater),
            ("1.0.1", "1.0.0", Ordering::Greater),
            ("2.1.1", "1.999.9999", Ordering::Greater),
            ("1.0.0-beta2", "1.0.0-BETA", Ordering::Greater),
            ("1.0.0+aa", "1.0.0-beta+AA", Ordering::Greater),
            ("1.0.0-beta.1+AA", "1.0.0-BETA", Ordering::Greater),
            (
                "1.0.0-beta.x.y.5.79.0+aa",
                "1.0.0-BETA.X.y.5.77.0+AA",
                Ordering::Greater,
            ),
            (
                "1.0.0-beta.x.y.5.790.0+abc",
                "1.0.0-BETA.X.y.5.79.0+AA",
                Ordering::Greater,
            ),
        ];

        for (x, y, expected) in cases {
            assert_eq!(compare(x, y), expected, "compare({x}, {y})");
        }
    }

    // ── nuget_update_summary ─────────────────────────────────────────────────

    #[test]
    fn same_version_no_update() {
        let s = nuget_update_summary("13.0.3", Some("13.0.3"));
        assert!(!s.update_available);
    }

    #[test]
    fn revision_zero_no_false_positive() {
        // Registry returns "13.0.3.0"; current is "13.0.3" → no update.
        let s = nuget_update_summary("13.0.3", Some("13.0.3.0"));
        assert!(!s.update_available);
        let s = nuget_update_summary("13.0.3.0", Some("13.0.3"));
        assert!(!s.update_available);
    }

    #[test]
    fn newer_patch_triggers_update() {
        let s = nuget_update_summary("13.0.1", Some("13.0.3"));
        assert!(s.update_available);
        assert_eq!(s.latest.as_deref(), Some("13.0.3"));
    }

    #[test]
    fn no_latest_no_update() {
        let s = nuget_update_summary("1.0.0", None);
        assert!(!s.update_available);
    }

    #[test]
    fn empty_current_no_update() {
        let s = nuget_update_summary("", Some("1.0.0"));
        assert!(!s.update_available);
    }

    #[test]
    fn older_latest_no_update() {
        let s = nuget_update_summary("2.0.0", Some("1.9.9"));
        assert!(!s.update_available);
    }

    // ── is_stable ────────────────────────────────────────────────────────────

    #[test]
    fn stable_versions() {
        assert!(is_stable("1.2.3"));
        assert!(is_stable("13.0.3"));
        assert!(is_stable("1.2.3.4"));
    }

    #[test]
    fn prerelease_versions() {
        assert!(!is_stable("1.2.3-preview1"));
        assert!(!is_stable("1.0.0-alpha"));
        assert!(!is_stable("1.0.0-rc.1"));
    }

    // Ported: "isStable(\"$input\") === $expected" — versioning/nuget/index.spec.ts line 16
    #[test]
    fn is_stable_matches_renovate_index_spec() {
        let cases = [
            ("9.0.3", true),
            ("1.2019.3.22", true),
            ("3.0.0-beta", false),
            ("2.0.2-pre20191018090318", false),
            ("1.0.0+c30d7625", true),
            ("2.3.4-beta+1990ef74", false),
            ("[1.2.3]", true),
            ("[1.2.3-beta]", false),
            ("1.0.0+Metadata", true),
            ("1.0.0", true),
            ("1.0.0-Beta", false),
            ("1.0.0-Beta+Meta", false),
            ("1.0.0-RC.X+Meta", false),
            ("1.0.0-RC.X.35.A.3455+Meta", false),
            ("*", false),
            ("1.0.*", false),
            ("1.0.*-*", false),
        ];

        for (input, expected) in cases {
            assert_eq!(is_stable(input), expected, "is_stable({input})");
        }
    }
}
