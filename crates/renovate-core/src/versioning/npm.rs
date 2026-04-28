//! npm (node-semver) versioning and update decision logic.
//!
//! Renovate reference: `lib/modules/versioning/npm/index.ts`
//!
//! npm uses node-semver which is broadly compatible with semver but has one
//! key difference from Cargo: a bare `"1.2.3"` constraint is an *exact* pin
//! (`=1.2.3`), not a compatible-range.  Range operators (`^`, `~`, `>=`, etc.)
//! behave as in standard semver.
//!
//! This module wraps the `semver` crate (which handles node-semver ranges) and
//! adds npm-specific update decisions.

use semver::{Version, VersionReq};

/// Detailed update summary for a single npm dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NpmUpdateSummary {
    /// The current constraint string from package.json.
    pub current_constraint: String,
    /// What npm's `latest` dist-tag points to (the registry's "stable" answer).
    pub latest: Option<String>,
    /// The newest version that satisfies the current constraint.
    pub latest_compatible: Option<String>,
    /// `true` when the constraint is an exact pin and a newer compatible
    /// version exists in the registry.
    pub update_available: bool,
}

/// Parse an npm constraint string into a `VersionReq`.
///
/// Returns `None` when the string is not a valid node-semver range.
pub fn parse_constraint(constraint: &str) -> Option<VersionReq> {
    VersionReq::parse(constraint).ok()
}

/// Return the newest version from `available` that satisfies `constraint`.
///
/// `available` must be sorted oldest-first; deprecated/yanked versions must
/// already be removed by the caller.
///
/// For npm exact pins (`"1.2.3"` without sigil), the Rust `semver` crate
/// would apply Cargo-style `^` semantics. This function detects that case and
/// prepends an explicit `=` so only the exact version matches.
pub fn resolve_latest_compatible(constraint: &str, available: &[String]) -> Option<String> {
    // Convert bare exact pins to "=version" so the semver crate matches only
    // that version (node-semver treats bare "1.2.3" as exact, unlike Cargo).
    let effective: String;
    let t = constraint.trim();
    let req_str = if is_exact_pin(t) && !t.starts_with('=') {
        effective = format!("={t}");
        &effective
    } else {
        t
    };
    let req = VersionReq::parse(req_str).ok()?;
    available
        .iter()
        .filter_map(|v| Version::parse(v).ok().map(|p| (v, p)))
        .filter(|(_, p)| req.matches(p))
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(s, _)| s.clone())
}

/// Produce an npm update summary for a single dependency.
///
/// `latest_tag` is the value of the npm `latest` dist-tag, used as the
/// registry's "current stable" anchor independent of the user's constraint.
///
/// For pinned exact versions an update is available when the registry's
/// `latest` dist-tag (or newest available version) differs from the current
/// pin — the user needs to bump the exact version string.  For range
/// constraints (`^`, `~`, `>=`, etc.) no update is flagged because the range
/// already covers future compatible versions.
pub fn npm_update_summary(
    constraint: &str,
    available: &[String],
    latest_tag: Option<&str>,
) -> NpmUpdateSummary {
    let latest_compatible = resolve_latest_compatible(constraint, available);

    // The overall "latest" tracks the registry's stable recommendation,
    // regardless of what the user's current constraint covers.
    let latest = latest_tag
        .map(str::to_owned)
        .or_else(|| available.last().cloned());

    // An exact pin is updatable when the registry's latest differs from it.
    // Range constraints are never flagged — they already cover future versions.
    let update_available = is_exact_pin(constraint)
        && latest
            .as_deref()
            .map(|l| l != constraint.trim())
            .unwrap_or(false);

    NpmUpdateSummary {
        current_constraint: constraint.to_owned(),
        latest,
        latest_compatible,
        update_available,
    }
}

/// Return `true` when `constraint` is an exact semver pin (`"1.2.3"` or
/// `"=1.2.3"`) with no range operators.
///
/// Unlike Cargo, npm treats a bare three-component version as exact, so
/// `"4.17.21"` means `=4.17.21` rather than `^4.17.21`.
fn is_exact_pin(constraint: &str) -> bool {
    let t = constraint.trim();
    // Strip a single leading `=` (explicit exact-match sigil).
    let stripped = t.strip_prefix('=').unwrap_or(t).trim();
    // Any remaining range operator disqualifies the string as an exact pin.
    !has_range_operator(stripped) && Version::parse(stripped).is_ok()
}

fn has_range_operator(s: &str) -> bool {
    s.contains('^')
        || s.contains('~')
        || s.contains('>')
        || s.contains('<')
        || s.contains('=')
        || s.contains('*')
        || s.contains('|')
        || s.contains(' ')
        || s.contains(',')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn avail(vs: &[&str]) -> Vec<String> {
        vs.iter().map(|s| (*s).to_owned()).collect()
    }

    // ── is_exact_pin ─────────────────────────────────────────────────────────

    #[test]
    fn bare_three_component_is_pinned() {
        assert!(is_exact_pin("4.17.21"));
        assert!(is_exact_pin("18.0.0"));
        assert!(is_exact_pin("0.0.1"));
    }

    #[test]
    fn explicit_equals_is_pinned() {
        assert!(is_exact_pin("=4.17.21"));
    }

    #[test]
    fn range_constraints_are_not_pinned() {
        for c in &["^4.17.21", "~4.17.21", ">=4.0.0", "4.17", "4", "*"] {
            assert!(!is_exact_pin(c), "{c:?} should not be a pin");
        }
    }

    // ── resolve_latest_compatible ─────────────────────────────────────────────

    #[test]
    fn resolve_caret_range() {
        let versions = avail(&["17.0.0", "17.0.2", "18.0.0", "18.2.0", "19.0.0"]);
        assert_eq!(
            resolve_latest_compatible("^18.0.0", &versions),
            Some("18.2.0".to_owned())
        );
    }

    #[test]
    fn resolve_gte_range() {
        let versions = avail(&["15.0.0", "16.0.0", "17.0.0"]);
        assert_eq!(
            resolve_latest_compatible(">=16", &versions),
            Some("17.0.0".to_owned())
        );
    }

    #[test]
    fn resolve_exact_pin() {
        let versions = avail(&["4.17.21", "4.18.0"]);
        assert_eq!(
            resolve_latest_compatible("4.17.21", &versions),
            Some("4.17.21".to_owned())
        );
    }

    // ── npm_update_summary ────────────────────────────────────────────────────

    #[test]
    fn pinned_with_newer_version_has_update() {
        let versions = avail(&["4.17.20", "4.17.21", "4.18.2"]);
        let s = npm_update_summary("4.17.21", &versions, Some("4.18.2"));
        // "=4.17.21" only matches 4.17.21 exactly.
        assert_eq!(s.latest_compatible.as_deref(), Some("4.17.21"));
        // The registry's latest is 4.18.2 ≠ pin → Renovate should bump the pin.
        assert!(s.update_available);
        assert_eq!(s.latest.as_deref(), Some("4.18.2"));
    }

    #[test]
    fn pinned_with_exact_newer_patch_has_update() {
        let versions = avail(&["1.0.0", "1.0.1", "1.0.2"]);
        let s = npm_update_summary("1.0.0", &versions, Some("1.0.2"));
        // latest is 1.0.2 ≠ pin 1.0.0 → update available.
        assert!(s.update_available);
        assert_eq!(s.latest_compatible.as_deref(), Some("1.0.0"));
    }

    #[test]
    fn range_constraint_never_has_update() {
        let versions = avail(&["18.0.0", "18.2.0", "19.0.0"]);
        for c in &["^18.0.0", ">=18", "~18.0.0"] {
            let s = npm_update_summary(c, &versions, Some("19.0.0"));
            assert!(
                !s.update_available,
                "range {c:?} should never flag an update"
            );
        }
    }

    #[test]
    fn latest_tag_is_reported_even_for_ranges() {
        let versions = avail(&["1.0.0", "2.0.0"]);
        let s = npm_update_summary("^1.0.0", &versions, Some("1.0.0"));
        assert_eq!(s.latest.as_deref(), Some("1.0.0"));
    }

    #[test]
    fn no_versions_returns_none_fields() {
        let s = npm_update_summary("^1.0.0", &[], None);
        assert!(s.latest_compatible.is_none());
        assert!(s.latest.is_none());
        assert!(!s.update_available);
    }
}
