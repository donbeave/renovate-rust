//! PEP 440 versioning and update decision logic.
//!
//! Renovate reference: `lib/modules/versioning/pep440/index.ts`
//!
//! Python's PEP 440 version specifiers include:
//! - `==1.2.3`   exact pin
//! - `>=1.0,<2.0` range
//! - `~=1.4`    compatible release ("at least 1.4, same major")
//! - `!=1.5`    exclusion
//!
//! This module handles the common case for the Renovate update planner:
//! - Detect whether a specifier is an exact pin (`==x.y.z`).
//! - For exact pins: flag an update when the registry's latest differs.
//! - For ranges / unconstrained: report latest but don't flag an update.

/// Detailed update summary for a single pip dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pep440UpdateSummary {
    /// The raw specifier string from requirements.txt (e.g. `"==1.2.3"`).
    pub current_specifier: String,
    /// The latest available version from the registry.
    pub latest: Option<String>,
    /// `true` when the specifier is an exact pin (`==X.Y.Z`) and a newer
    /// version is available.
    pub update_available: bool,
}

/// Produce a PEP 440 update summary.
///
/// `available` must be sorted oldest-first with yanked releases already
/// removed.  `latest_stable` is the registry's authoritative "latest stable"
/// answer (typically from the JSON API's `info.version` field).
pub fn pep440_update_summary(specifier: &str, latest_stable: Option<&str>) -> Pep440UpdateSummary {
    let latest = latest_stable.map(str::to_owned);

    // Only flag an update when the specifier is an exact pin.
    let pinned = exact_pin_version(specifier);

    let update_available = pinned
        .as_deref()
        .zip(latest.as_deref())
        .is_some_and(|(pin, lat)| pin != lat);

    Pep440UpdateSummary {
        current_specifier: specifier.to_owned(),
        latest,
        update_available,
    }
}

/// Extract the pinned version from an exact `==X.Y.Z` specifier.
///
/// Returns `Some("X.Y.Z")` if the specifier is a single `==` clause with no
/// other operators; `None` for ranges, unconstrained, or complex specifiers.
pub fn exact_pin_version(specifier: &str) -> Option<String> {
    let s = specifier.trim();
    if s.is_empty() {
        return None;
    }
    // Must be a single clause (no comma separating multiple clauses).
    if s.contains(',') {
        return None;
    }
    // Must start with `==` and nothing else (`!=`, `~=`, `>=`, `<=` are not exact).
    let version = s.strip_prefix("==")?;
    // Wildcard pins like `==1.*` are not strict exact versions.
    if version.contains('*') {
        return None;
    }
    Some(version.trim().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── exact_pin_version ─────────────────────────────────────────────────────

    #[test]
    fn exact_pin_simple() {
        assert_eq!(exact_pin_version("==1.2.3"), Some("1.2.3".to_owned()));
        assert_eq!(exact_pin_version("== 4.2.7"), Some("4.2.7".to_owned()));
    }

    #[test]
    fn ranges_are_not_pins() {
        for s in &[">=1.0", ">=1.0,<2.0", "~=1.4", "!=1.5", ""] {
            assert!(exact_pin_version(s).is_none(), "{s:?} should not be a pin");
        }
    }

    #[test]
    fn wildcard_is_not_pin() {
        assert!(exact_pin_version("==1.*").is_none());
    }

    // ── pep440_update_summary ─────────────────────────────────────────────────

    #[test]
    fn pinned_with_newer_latest_has_update() {
        let s = pep440_update_summary("==4.2.7", Some("4.2.10"));
        assert!(s.update_available);
        assert_eq!(s.latest.as_deref(), Some("4.2.10"));
    }

    #[test]
    fn pinned_already_latest_has_no_update() {
        let s = pep440_update_summary("==4.2.10", Some("4.2.10"));
        assert!(!s.update_available);
    }

    #[test]
    fn range_specifier_has_no_update() {
        let s = pep440_update_summary(">=4.0,<5.0", Some("4.2.10"));
        assert!(!s.update_available);
        assert_eq!(s.latest.as_deref(), Some("4.2.10"));
    }

    #[test]
    fn unconstrained_has_no_update() {
        let s = pep440_update_summary("", Some("4.2.10"));
        assert!(!s.update_available);
    }

    #[test]
    fn no_latest_has_no_update() {
        let s = pep440_update_summary("==4.2.7", None);
        assert!(!s.update_available);
    }
}
