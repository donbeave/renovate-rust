//! Cargo (semver) versioning and update decision logic.
//!
//! Renovate reference: `lib/modules/versioning/cargo/index.ts`
//!
//! Cargo uses a semver dialect with Rust's `semver` crate semantics:
//! - bare `"1.2"` means `"^1.2"` (compatible with 1.2)
//! - `"^1.2"` means `>=1.2.0, <2.0.0`
//! - `"~1.2"` means `>=1.2.0, <1.3.0`
//! - `"1.0, <2"` (comma-separated: intersection of requirements)
//!
//! This module wraps the `semver` crate to provide the decision functions
//! Renovate uses in its update planner.

use semver::{Version, VersionReq};

/// Parse a Cargo version constraint string.
///
/// Returns `None` when the string cannot be parsed as a `VersionReq`.
/// Bare version strings like `"1.2"` are accepted (treated as `^1.2`).
pub fn parse_constraint(constraint: &str) -> Option<VersionReq> {
    // Cargo accepts comma-separated requirements as an intersection.
    // The `semver` crate natively handles this via its `VersionReq::parse`.
    VersionReq::parse(constraint).ok()
}

/// Result of checking whether a new version is available.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateDecision {
    /// A new compatible version exists (satisfies the same constraint family
    /// but is newer than the constraint's lower bound).
    Update { new_version: String },
    /// The current constraint already resolves to the latest non-yanked
    /// version — no update needed.
    UpToDate,
    /// The constraint could not be parsed. The dep should be flagged for
    /// manual review.
    UnparseableConstraint,
    /// No non-yanked versions matched the compatible family.
    NoCompatibleVersions,
}

/// Determine whether `available_versions` contains a version that is:
/// 1. newer than any version currently satisfied by `constraint`, and
/// 2. semver-compatible with the constraint's intent (same major for `^`
///    constraints, etc.)
///
/// `available_versions` must be sorted oldest-first (as the crates.io index
/// provides them). Yanked versions must already be filtered out by the caller.
///
/// This is intentionally a simple "is latest compatible version newer than
/// current upper bound" check. Full Renovate compatibility (respecting range
/// strategies, pinned vs caret, etc.) is a later slice.
pub fn check_update(constraint: &str, available_versions: &[String]) -> UpdateDecision {
    let Some(req) = parse_constraint(constraint) else {
        return UpdateDecision::UnparseableConstraint;
    };

    // Collect all valid, constraint-matching versions.
    let mut compatible: Vec<Version> = available_versions
        .iter()
        .filter_map(|v| Version::parse(v).ok())
        .filter(|v| req.matches(v))
        .collect();

    if compatible.is_empty() {
        return UpdateDecision::NoCompatibleVersions;
    }

    compatible.sort();
    let latest_compatible = compatible.last().unwrap();

    // The last entry in available_versions is the newest published version.
    // If it satisfies the constraint and is newer than our current latest,
    // there is an update available.
    let newest_in_list = available_versions
        .iter()
        .rev()
        .find_map(|v| Version::parse(v).ok());

    match newest_in_list {
        Some(newest) if req.matches(&newest) && &newest > latest_compatible => {
            UpdateDecision::Update {
                new_version: newest.to_string(),
            }
        }
        _ => UpdateDecision::UpToDate,
    }
}

/// Find the newest non-yanked version that satisfies `constraint`.
///
/// Returns `None` when no version matches or the constraint is unparseable.
pub fn resolve_latest(constraint: &str, available_versions: &[String]) -> Option<String> {
    let req = parse_constraint(constraint)?;
    available_versions
        .iter()
        .filter_map(|v| Version::parse(v).ok())
        .filter(|v| req.matches(v))
        .max()
        .map(|v| v.to_string())
}

/// Detailed update summary for a single dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateSummary {
    /// The current constraint string from Cargo.toml (e.g. `"1.0.100"`).
    pub current_constraint: String,
    /// The latest non-yanked version that satisfies the constraint, if any.
    pub latest_compatible: Option<String>,
    /// `true` when the constraint is a fully-specified three-part version
    /// (e.g. `"1.0.100"` or `"=1.0.100"`) and `latest_compatible` is newer.
    /// Range constraints like `"^1"` or `"1.52"` never set this — they already
    /// cover future compatible versions by design.
    pub update_available: bool,
}

/// Produce a full update summary for a dependency constraint.
///
/// This is the primary function for the update planner. It tells callers:
/// - what the latest compatible version is
/// - whether that version represents an actionable change (i.e. the user
///   has a pinned exact version that could be bumped)
///
/// A "pinned" constraint is one where the raw string parses as a valid three-
/// component `semver::Version` with no sigil (e.g. `"1.0.228"`), or an exact
/// match with `=` prefix. Range constraints (`^`, `~`, `>=`, bare `1`, `1.52`)
/// are NOT considered pinned — they automatically cover future compatible
/// versions, so no update is suggested.
pub fn update_summary(constraint: &str, available_versions: &[String]) -> UpdateSummary {
    let latest = resolve_latest(constraint, available_versions);

    // Determine whether the constraint is pinned (exact 3-component version).
    let stripped = constraint.trim().trim_start_matches('=').trim();
    let is_pinned = Version::parse(stripped).is_ok()
        && !constraint.contains('^')
        && !constraint.contains('~')
        && !constraint.contains('>')
        && !constraint.contains('<')
        && !constraint.contains(',')
        && !constraint.contains('*');

    let update_available = is_pinned && latest.as_deref().map(|l| l != stripped).unwrap_or(false);

    UpdateSummary {
        current_constraint: constraint.to_owned(),
        latest_compatible: latest,
        update_available,
    }
}

#[cfg(test)]
mod update_summary_tests {
    use super::*;

    fn v(s: &[&str]) -> Vec<String> {
        s.iter().map(|x| (*x).to_owned()).collect()
    }

    #[test]
    fn pinned_version_with_newer_available_is_update() {
        let avail = v(&["1.0.0", "1.0.100", "1.0.228"]);
        let s = update_summary("1.0.100", &avail);
        assert_eq!(s.latest_compatible.as_deref(), Some("1.0.228"));
        assert!(s.update_available);
    }

    #[test]
    fn pinned_version_already_latest_is_not_update() {
        let avail = v(&["1.0.0", "1.0.228"]);
        let s = update_summary("1.0.228", &avail);
        assert!(!s.update_available);
    }

    #[test]
    fn range_constraint_is_never_flagged_as_update() {
        let avail = v(&["1.0.0", "1.0.228"]);
        for constraint in &["1", "^1", "^1.0", "~1.0", ">=1.0", "1.0"] {
            let s = update_summary(constraint, &avail);
            assert!(
                !s.update_available,
                "expected no update for constraint {constraint:?}"
            );
        }
    }

    #[test]
    fn no_compatible_versions_is_not_update() {
        let avail = v(&["1.0.0"]);
        let s = update_summary("2.0.0", &avail);
        assert!(s.latest_compatible.is_none());
        assert!(!s.update_available);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn versions(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| (*s).to_owned()).collect()
    }

    #[test]
    fn parses_bare_version() {
        assert!(parse_constraint("1.0").is_some());
        assert!(parse_constraint("1.52").is_some());
    }

    #[test]
    fn parses_caret_constraint() {
        assert!(parse_constraint("^1.0").is_some());
    }

    #[test]
    fn parses_tilde_constraint() {
        assert!(parse_constraint("~1.2").is_some());
    }

    #[test]
    fn rejects_invalid_constraint() {
        assert!(parse_constraint("not.a.version.!").is_none());
    }

    #[test]
    fn resolve_latest_returns_highest_matching() {
        let avail = versions(&["1.0.0", "1.1.0", "1.2.0", "2.0.0"]);
        // ^1.0 matches 1.x.x but not 2.x.x
        assert_eq!(resolve_latest("^1.0", &avail), Some("1.2.0".to_owned()));
    }

    #[test]
    fn resolve_latest_exact_match() {
        let avail = versions(&["0.9.0", "1.0.0", "1.0.1"]);
        assert_eq!(resolve_latest("=1.0.0", &avail), Some("1.0.0".to_owned()));
    }

    #[test]
    fn resolve_latest_no_match_returns_none() {
        let avail = versions(&["1.0.0", "1.1.0"]);
        assert_eq!(resolve_latest("^2.0", &avail), None);
    }

    #[test]
    fn check_update_up_to_date_when_latest_is_current() {
        let avail = versions(&["1.0.0", "1.1.0", "1.2.0"]);
        // Constraint ^1 already covers 1.2.0 which is the latest compatible
        assert_eq!(check_update("^1", &avail), UpdateDecision::UpToDate);
    }

    #[test]
    fn check_update_unparseable_constraint() {
        assert_eq!(
            check_update("!not!valid!", &[]),
            UpdateDecision::UnparseableConstraint
        );
    }

    #[test]
    fn check_update_no_compatible_versions() {
        let avail = versions(&["1.0.0", "1.1.0"]);
        assert_eq!(
            check_update("^2.0", &avail),
            UpdateDecision::NoCompatibleVersions
        );
    }
}
