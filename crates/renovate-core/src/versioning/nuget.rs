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
        (Some(la), Some(lb)) => la.cmp(lb),
    }
}

/// Returns `true` when the version is a stable release (no pre-release label).
pub fn is_stable(version: &str) -> bool {
    parse(version).prerelease.is_none()
}

// ── Internal ──────────────────────────────────────────────────────────────────

struct ParsedVersion {
    /// Exactly 4 components, padded with 0 for missing parts.
    components: [u64; 4],
    prerelease: Option<String>,
}

fn parse(version: &str) -> ParsedVersion {
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
}
