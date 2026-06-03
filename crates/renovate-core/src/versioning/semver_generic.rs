//! @parity lib/modules/versioning/semver/common.ts full
//!
//! Generic semver update-decision logic.
//!
//! A shared helper used by datasources that store semver-compatible packages
//! (pub.dev, Packagist/Composer, NuGet, RubyGems, Hex.pm, CocoaPods) where
//! the constraint syntax uses operators like `^`, `~>`, `>=` and the registry
//! returns plain version strings.
//!
//! ## Why this module exists
//!
//! The naive string comparison `l != lower_bound` has a correctness defect:
//! `lower_bound("^6.4")` returns `"6.4"`, but the registry may return
//! `"6.4.0"`, which is semver-equal to `"6.4"` but a different string. This
//! causes a false "update available" report. Padding both sides to three
//! components and using the `semver` crate resolves this.
//!
//! ## Algorithm
//!
//! 1. Strip leading operators (`^`, `~>`, `>=`, `>`, `<=`, `<`, `=`, `!`)
//!    from `current_value` to extract the lower-bound version string.
//! 2. Pad both the lower bound and `latest` to three semver components
//!    (e.g. `"6.4"` → `"6.4.0"`).
//! 3. Parse both with the `semver` crate.
//! 4. `update_available` is `true` when `latest_semver > lower_bound_semver`.

use semver::Version;

/// Update summary produced by [`semver_update_summary`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemverUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Produce an update summary for a dependency using generic semver comparison.
///
/// `current_value` may be a pinned version (`1.2.3`) or a range constraint
/// (`^1.2`, `~> 1.2.3`, `>= 1.0.0`, `>= 1.0, < 2.0`). The lower bound is
/// extracted by stripping leading operators, then compared to `latest` via
/// the `semver` crate to avoid string-equality false positives.
pub fn semver_update_summary(current_value: &str, latest: Option<&str>) -> SemverUpdateSummary {
    let update_available = latest
        .filter(|l| !l.is_empty() && !current_value.is_empty())
        .is_some_and(|latest_str| {
            let lb = lower_bound(current_value);
            if lb.is_empty() {
                return false;
            }
            let Some(lv) = parse_padded(latest_str) else {
                return false;
            };
            let Some(cv) = parse_padded(lb) else {
                // Fall back to string comparison if semver parse fails.
                return latest_str != lb;
            };
            lv > cv
        });

    SemverUpdateSummary {
        current_value: current_value.to_owned(),
        latest: latest.map(|s| s.to_owned()),
        update_available,
    }
}

// ── Update type classification ────────────────────────────────────────────────

/// Semantic update type for a version bump.
///
/// Mirrors Renovate's `UpdateType` enum from `lib/config/types.ts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateType {
    Major,
    Minor,
    Patch,
    /// Package is being replaced by a different package name.
    /// Set via `matchUpdateTypes: ["replacement"]`.
    Replacement,
    /// Docker image or git commit digest is being pinned/updated.
    /// Set via `matchUpdateTypes: ["digest"]`.
    Digest,
    /// Package is being pinned to an exact version from a range.
    /// Set via `matchUpdateTypes: ["pin"]`.
    Pin,
    /// Range bump: dep range was widened to include current version without a
    /// new upstream release.  Used via `matchUpdateTypes: ["bump"]` together
    /// with `isBump: true` on the dep context.
    Bump,
}

impl UpdateType {
    /// Return the Renovate-compatible lowercase string name for this variant.
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateType::Major => "major",
            UpdateType::Minor => "minor",
            UpdateType::Patch => "patch",
            UpdateType::Replacement => "replacement",
            UpdateType::Digest => "digest",
            UpdateType::Pin => "pin",
            UpdateType::Bump => "bump",
        }
    }
}

/// Classify a version bump as major, minor, or patch using semver comparison.
///
/// Returns `None` when either version string cannot be parsed as semver.
/// Both strings are padded to three components and leading `v` is stripped.
pub fn classify_semver_update(current: &str, latest: &str) -> Option<UpdateType> {
    let current_v = parse_padded(lower_bound(current))?;
    let latest_v = parse_padded(latest)?;
    if latest_v.major > current_v.major {
        Some(UpdateType::Major)
    } else if latest_v.minor > current_v.minor {
        Some(UpdateType::Minor)
    } else if latest_v.patch > current_v.patch {
        Some(UpdateType::Patch)
    } else {
        None
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Extract the lower-bound version string from a constraint.
///
/// Examples:
/// - `"^1.2.3"` → `"1.2.3"`
/// - `"~> 1.2"` → `"1.2"`
/// - `">= 1.0.0"` → `"1.0.0"`
/// - `">= 1.0, < 2.0"` → `"1.0"` (first component)
/// - `"1.2.3"` → `"1.2.3"` (bare version)
pub fn lower_bound(constraint: &str) -> &str {
    let stripped = constraint
        .trim()
        .trim_start_matches(['~', '>', '<', '=', '!', ' ', '^']);
    stripped.split(',').next().unwrap_or("").trim()
}

/// Return true when `range` is a SemVer X-range wildcard (`*`, `x`, `X`, or empty).
///
/// Mirrors `lib/modules/versioning/semver/common.ts` `isSemVerXRange()`.
pub fn is_semver_x_range(range: &str) -> bool {
    matches!(range, "*" | "x" | "X" | "")
}

/// Parse a version string, padding missing minor/patch components with 0.
pub fn parse_padded(v: &str) -> Option<Version> {
    let v = v.trim().trim_start_matches('v');
    let parts: Vec<&str> = v.splitn(3, '.').collect();
    let padded = match parts.len() {
        0 => return None,
        1 => format!("{}.0.0", parts[0]),
        2 => format!("{}.{}.0", parts[0], parts[1]),
        _ => v.to_owned(),
    };
    Version::parse(&padded).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isSemVerXRange(\"range\") === $expected" — lib/modules/versioning/semver/common.spec.ts line 4
    #[test]
    fn is_semver_x_range_matches_renovate_spec() {
        for r in ["*", "x", "X", ""] {
            assert!(is_semver_x_range(r), "{r:?} should be X range");
        }
        for r in ["1", "1.2", "1.2.3"] {
            assert!(!is_semver_x_range(r), "{r:?} should not be X range");
        }
    }

    // Rust-specific: unit tests for semver_update_summary edge cases
    #[test]
    fn same_version_no_update() {
        let s = semver_update_summary("1.2.3", Some("1.2.3"));
        assert!(!s.update_available);
    }

    // Rust-specific: unit tests for semver_update_summary edge cases
    #[test]
    fn newer_version_update() {
        let s = semver_update_summary("1.2.3", Some("1.2.4"));
        assert!(s.update_available);
    }

    // Rust-specific: verifies padded semver comparison avoids false positives
    #[test]
    fn caret_range_lower_bound_match() {
        // "^6.4" lower bound is "6.4" = semver 6.4.0
        // latest "6.4.0" should NOT trigger update (false positive with string compare)
        let s = semver_update_summary("^6.4", Some("6.4.0"));
        assert!(!s.update_available);
    }

    // Rust-specific: unit tests for semver_update_summary edge cases
    #[test]
    fn caret_range_newer() {
        let s = semver_update_summary("^6.4", Some("6.5.0"));
        assert!(s.update_available);
    }

    // Rust-specific: unit tests for semver_update_summary edge cases
    #[test]
    fn pessimistic_tilde_range() {
        // "~> 1.7.0" lower bound "1.7.0"
        let s = semver_update_summary("~> 1.7.0", Some("1.7.3"));
        assert!(s.update_available);
    }

    // Rust-specific: unit tests for semver_update_summary edge cases
    #[test]
    fn gte_constraint() {
        let s = semver_update_summary(">= 1.0.0", Some("1.5.0"));
        assert!(s.update_available);
    }

    // Rust-specific: verifies padded semver comparison avoids false positives
    #[test]
    fn two_component_vs_three() {
        // "1.7" lower bound → semver 1.7.0; latest "1.7.0" → equal → no update
        let s = semver_update_summary("1.7", Some("1.7.0"));
        assert!(!s.update_available);
    }

    // Rust-specific: unit tests for semver_update_summary edge cases
    #[test]
    fn unconstrained_no_update() {
        let s = semver_update_summary("", Some("1.0.0"));
        assert!(!s.update_available);
    }

    // Rust-specific: unit tests for semver_update_summary edge cases
    #[test]
    fn no_latest_no_update() {
        let s = semver_update_summary("1.0.0", None);
        assert!(!s.update_available);
    }

    // Rust-specific: unit tests for semver_update_summary edge cases
    #[test]
    fn multi_constraint_lower_bound() {
        // ">= 1.0, < 2.0" → lower bound "1.0" → semver 1.0.0
        let s = semver_update_summary(">= 1.0, < 2.0", Some("1.5.0"));
        assert!(s.update_available);
    }

    // Rust-specific: unit tests for parse_padded helper
    #[test]
    fn parse_padded_one_component() {
        let v = parse_padded("6").unwrap();
        assert_eq!(v.to_string(), "6.0.0");
    }

    // Rust-specific: unit tests for parse_padded helper
    #[test]
    fn parse_padded_two_components() {
        let v = parse_padded("6.4").unwrap();
        assert_eq!(v.to_string(), "6.4.0");
    }

    // Rust-specific: unit tests for parse_padded helper
    #[test]
    fn parse_padded_three_components() {
        let v = parse_padded("6.4.1").unwrap();
        assert_eq!(v.to_string(), "6.4.1");
    }

    // Rust-specific: unit tests for lower_bound helper
    #[test]
    fn lower_bound_caret() {
        assert_eq!(lower_bound("^1.2.3"), "1.2.3");
    }

    // Rust-specific: unit tests for lower_bound helper
    #[test]
    fn lower_bound_tilde_gt() {
        assert_eq!(lower_bound("~> 1.7"), "1.7");
    }

    // Rust-specific: unit tests for lower_bound helper
    #[test]
    fn lower_bound_gte() {
        assert_eq!(lower_bound(">= 1.0.0"), "1.0.0");
    }

    // Rust-specific: unit tests for lower_bound helper
    #[test]
    fn lower_bound_exact() {
        assert_eq!(lower_bound("1.2.3"), "1.2.3");
    }

    // Rust-specific: defensive test for semver_update_summary edge case
    #[test]
    fn older_version_no_update() {
        // Latest is older than lower bound (shouldn't happen but defensive)
        let s = semver_update_summary("~> 5.0", Some("4.9.9"));
        assert!(!s.update_available);
    }

    // ── classify_semver_update tests ─────────────────────────────────────────

    // Rust-specific: unit tests for classify_semver_update helper
    #[test]
    fn classify_major_bump() {
        assert_eq!(
            classify_semver_update("1.2.3", "2.0.0"),
            Some(UpdateType::Major)
        );
    }

    // Rust-specific: unit tests for classify_semver_update helper
    #[test]
    fn classify_minor_bump() {
        assert_eq!(
            classify_semver_update("1.2.3", "1.3.0"),
            Some(UpdateType::Minor)
        );
    }

    // Rust-specific: unit tests for classify_semver_update helper
    #[test]
    fn classify_patch_bump() {
        assert_eq!(
            classify_semver_update("1.2.3", "1.2.4"),
            Some(UpdateType::Patch)
        );
    }

    // Rust-specific: unit tests for classify_semver_update helper
    #[test]
    fn classify_same_version_returns_none() {
        assert_eq!(classify_semver_update("1.2.3", "1.2.3"), None);
    }

    // Rust-specific: unit tests for classify_semver_update helper
    #[test]
    fn classify_strips_v_prefix() {
        assert_eq!(
            classify_semver_update("v1.2.3", "v2.0.0"),
            Some(UpdateType::Major)
        );
    }

    // Rust-specific: unit tests for classify_semver_update helper
    #[test]
    fn classify_caret_range_to_major() {
        assert_eq!(
            classify_semver_update("^1.2", "2.0.0"),
            Some(UpdateType::Major)
        );
    }

    // Rust-specific: unit tests for classify_semver_update helper
    #[test]
    fn classify_non_semver_returns_none() {
        assert_eq!(classify_semver_update("latest", "2.0.0"), None);
        assert_eq!(classify_semver_update("1.0.0", "next"), None);
    }

    #[test]
    fn update_type_as_str() {
        assert_eq!(UpdateType::Major.as_str(), "major");
        assert_eq!(UpdateType::Minor.as_str(), "minor");
        assert_eq!(UpdateType::Patch.as_str(), "patch");
    }
}
