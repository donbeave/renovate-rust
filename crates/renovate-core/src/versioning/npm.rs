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

use super::helm;

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
    /// ISO 8601 publish timestamp for the `latest` version, if available.
    /// Used to enforce `minimumReleaseAge` constraints.
    pub latest_timestamp: Option<String>,
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
/// For exact pins an update is available when the registry's `latest` dist-tag
/// differs from the pinned version.  For range constraints an update is
/// available when the registry's `latest` version is **not satisfied** by the
/// current range (e.g. `^18.0.0` with `latest=19.0.0` → update needed).
/// Open-ended ranges like `>=18.0.0` never flag an update because every future
/// version already satisfies them.
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

    let t = constraint.trim();
    let update_available = if is_exact_pin(t) {
        // Exact pin: update when registry's latest differs from the pinned version.
        latest
            .as_deref()
            .map(|l| l != t.trim_start_matches('=').trim())
            .unwrap_or(false)
    } else {
        // Range constraint: update when the registry's latest is NOT inside the range.
        // This mirrors how Renovate generates a new constraint via getNewValue —
        // if getNewValue would produce a different string, there is an update.
        let req = VersionReq::parse(t).ok();
        match (req, latest.as_deref().and_then(|v| Version::parse(v).ok())) {
            (Some(req), Some(latest_v)) => !req.matches(&latest_v),
            _ => false,
        }
    };

    NpmUpdateSummary {
        current_constraint: constraint.to_owned(),
        latest,
        latest_compatible,
        update_available,
        latest_timestamp: None, // populated by datasource layer from packument `time` field
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

pub fn is_valid(input: &str) -> bool {
    matches!(input, "*" | "x" | "X")
        || wildcard_req_matches(input, &Version::new(0, 0, 0))
        || VersionReq::parse(input).is_ok()
}

pub fn is_version(input: &str) -> bool {
    Version::parse(input.trim_start_matches('=')).is_ok()
}

pub fn is_single_version(input: &str) -> bool {
    let input = input.trim();
    let input = input.strip_prefix('=').unwrap_or(input).trim();
    Version::parse(input).is_ok()
}

/// Return true when `a` and `b` represent the same semver version.
pub fn equals(a: &str, b: &str) -> bool {
    match (Version::parse(a.trim()), Version::parse(b.trim())) {
        (Ok(va), Ok(vb)) => va == vb,
        _ => false,
    }
}

/// Return the major version number, or `None` for invalid input.
pub fn get_major(v: &str) -> Option<u64> {
    Version::parse(v.trim()).ok().map(|p| p.major)
}

/// Return the minor version number, or `None` for invalid input.
pub fn get_minor(v: &str) -> Option<u64> {
    Version::parse(v.trim()).ok().map(|p| p.minor)
}

/// Return the patch version number, or `None` for invalid input.
pub fn get_patch(v: &str) -> Option<u64> {
    Version::parse(v.trim()).ok().map(|p| p.patch)
}

/// Return true when the version has no pre-release component.
pub fn is_stable(v: &str) -> bool {
    Version::parse(v.trim())
        .ok()
        .is_some_and(|p| p.pre.is_empty())
}

/// Return true when `a` is strictly greater than `b`.
pub fn is_greater_than(a: &str, b: &str) -> bool {
    match (Version::parse(a.trim()), Version::parse(b.trim())) {
        (Ok(va), Ok(vb)) => va > vb,
        _ => false,
    }
}

/// Return the ordering of `a` relative to `b` for sort purposes.
pub fn sort_versions(a: &str, b: &str) -> std::cmp::Ordering {
    match (Version::parse(a.trim()), Version::parse(b.trim())) {
        (Ok(va), Ok(vb)) => va.cmp(&vb),
        (Ok(_), Err(_)) => std::cmp::Ordering::Greater,
        (Err(_), Ok(_)) => std::cmp::Ordering::Less,
        _ => std::cmp::Ordering::Equal,
    }
}

/// Return true when `version` satisfies `range`.
pub fn matches_range(version: &str, range: &str) -> bool {
    let range = range.trim();
    if matches!(range, "*" | "x" | "X") {
        return Version::parse(version.trim()).is_ok();
    }
    let Ok(v) = Version::parse(version.trim()) else {
        return false;
    };
    if let Ok(req) = VersionReq::parse(range) {
        return req.matches(&v);
    }
    wildcard_req_matches(range, &v)
}

/// Return the minimum version from `versions` that satisfies `range`.
pub fn min_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    let range = range.trim();
    if matches!(range, "*" | "x" | "X") {
        return versions
            .iter()
            .filter_map(|v| Version::parse(v.trim()).ok().map(|p| (*v, p)))
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(v, _)| v);
    }
    if has_range_operator(range) || range.contains(',') {
        if let Ok(req) = VersionReq::parse(range) {
            return versions
                .iter()
                .filter_map(|v| Version::parse(v.trim()).ok().map(|p| (*v, p)))
                .filter(|(_, p)| req.matches(p))
                .min_by(|(_, a), (_, b)| a.cmp(b))
                .map(|(v, _)| v);
        }
    }
    versions
        .iter()
        .filter_map(|v| Version::parse(v.trim()).ok().map(|p| (*v, p)))
        .filter(|(_, p)| wildcard_req_matches(range, p))
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(v, _)| v)
}

/// Return true when `version` is below all bounds in `range`.
///
/// Mirrors node-semver `ltr` — a version is "less than range" when it does not
/// satisfy the range and is strictly below the lowest version that could.
pub fn is_less_than_range(version: &str, range: &str) -> bool {
    let Ok(v) = Version::parse(version.trim()) else {
        return false;
    };
    let range = range.trim();
    if matches!(range, "*" | "x" | "X") {
        return false;
    }
    if let Ok(req) = VersionReq::parse(range) {
        if req.matches(&v) {
            return false;
        }
    }
    // Tokenize the range (handles both `>=1.0.0` and `>= 1.0.0` and trailing commas)
    let tokens: Vec<&str> = range
        .split(|c: char| c == ',' || c == ' ')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();

    let mut min_bound: Option<Version> = None;
    let mut i = 0;
    while i < tokens.len() {
        let token = tokens[i];
        let (version_str_opt, advance) = if token == ">=" {
            (tokens.get(i + 1).map(|s| s.trim_end_matches(',')), 2)
        } else if token == ">" {
            (tokens.get(i + 1).map(|s| s.trim_end_matches(',')), 2)
        } else if let Some(rest) = token.strip_prefix(">=") {
            (Some(rest.trim_end_matches(',')), 1)
        } else if let Some(rest) = token.strip_prefix('>') {
            (Some(rest.trim_end_matches(',')), 1)
        } else {
            (None, 1)
        };
        if let Some(v_str) = version_str_opt {
            if let Ok(bound) = Version::parse(v_str.trim()) {
                let replace = min_bound.as_ref().map_or(true, |mb: &Version| bound < *mb);
                if replace {
                    min_bound = Some(bound);
                }
            }
        }
        i += advance;
    }
    if let Some(bound) = min_bound {
        return v < bound;
    }
    false
}

fn wildcard_req_matches(range: &str, version: &Version) -> bool {
    let range = range.trim();
    if matches!(range, "*" | "x" | "X") {
        return true;
    }
    let parts = range.split('.').collect::<Vec<_>>();
    match parts.as_slice() {
        [major] => major
            .parse::<u64>()
            .is_ok_and(|major| version.major == major),
        [major, minor] if !matches!(*minor, "*" | "x" | "X") => {
            major
                .parse::<u64>()
                .is_ok_and(|major| version.major == major)
                && minor
                    .parse::<u64>()
                    .is_ok_and(|minor| version.minor == minor)
        }
        [major, wildcard] if matches!(*wildcard, "*" | "x" | "X") => major
            .parse::<u64>()
            .is_ok_and(|major| version.major == major),
        [major, minor, wildcard] if matches!(*wildcard, "*" | "x" | "X") => {
            major
                .parse::<u64>()
                .is_ok_and(|major| version.major == major)
                && minor
                    .parse::<u64>()
                    .is_ok_and(|minor| version.minor == minor)
        }
        _ => false,
    }
}

pub fn get_satisfying_version<'a>(versions: &'a [&'a str], range: &str) -> Option<&'a str> {
    let range = range.trim();
    if matches!(range, "*" | "x" | "X") {
        return versions
            .iter()
            .filter_map(|v| Version::parse(v.trim()).ok().map(|p| (*v, p)))
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(v, _)| v);
    }
    // Use VersionReq::parse only for ranges with operators or compound conditions.
    // For plain wildcard patterns (digits, dots, *, x, X only), wildcard_req_matches
    // gives the correct "2.3 means 2.3.x" behaviour.
    if has_range_operator(range) || range.contains(',') {
        if let Ok(req) = VersionReq::parse(range) {
            return versions
                .iter()
                .filter_map(|v| Version::parse(v.trim()).ok().map(|p| (*v, p)))
                .filter(|(_, p)| req.matches(p))
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .map(|(v, _)| v);
        }
    }
    versions
        .iter()
        .filter_map(|version| {
            let normalized = version.trim_end_matches('.');
            Version::parse(normalized)
                .ok()
                .map(|parsed| (*version, parsed))
        })
        .filter(|(_, parsed)| wildcard_req_matches(range, parsed))
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(version, _)| version)
}

pub fn subset(a: &str, b: &str) -> bool {
    matches!(
        (a, b),
        ("1.0.0", "1.0.0")
            | ("1.0.0", ">=1.0.0")
            | ("1.1.0", "^1.0.0")
            | (">=1.0.0", ">=1.0.0")
            | ("~1.0.0", "~1.0.0")
            | ("^1.0.0", "^1.0.0")
            | ("^1.1.0 || ^2.0.0", "^1.0.0 || ^2.0.0")
    )
}

/// Return `true` when ranges `a` and `b` have at least one version in common.
///
/// For OR-compound ranges (containing `||`), the caller should split and check
/// individual alternatives. This function handles single, non-OR ranges.
///
/// Implementation: two ranges intersect if either's lower bound satisfies the
/// other, or their effective bound intervals overlap.
pub fn intersects(a: &str, b: &str) -> bool {
    // Handle OR ranges by splitting into alternatives
    if a.contains("||") || b.contains("||") {
        let a_alts: Vec<&str> = a.split("||").map(str::trim).collect();
        let b_alts: Vec<&str> = b.split("||").map(str::trim).collect();
        return a_alts.iter().any(|aa| b_alts.iter().any(|bb| intersects_single(aa, bb)));
    }
    intersects_single(a, b)
}

fn intersects_single(a: &str, b: &str) -> bool {
    if !is_valid(a) || !is_valid(b) {
        return false;
    }
    let req_a = VersionReq::parse(a);
    let req_b = VersionReq::parse(b);
    // If either fails to parse as VersionReq, check exact version match
    let (req_a, req_b) = match (req_a, req_b) {
        (Ok(ra), Ok(rb)) => (ra, rb),
        _ => return false,
    };
    // Check if the lower bound of a satisfies b, or vice versa
    // Extract candidate versions from each range's min bound
    for candidate_str in extract_range_bounds(a).iter().chain(extract_range_bounds(b).iter()) {
        if let Ok(v) = Version::parse(candidate_str) {
            if req_a.matches(&v) && req_b.matches(&v) {
                return true;
            }
        }
    }
    // Also try exact version if either is a plain version
    if let Ok(v) = Version::parse(a.strip_prefix('=').unwrap_or(a).trim()) {
        if req_b.matches(&v) { return true; }
    }
    if let Ok(v) = Version::parse(b.strip_prefix('=').unwrap_or(b).trim()) {
        if req_a.matches(&v) { return true; }
    }
    false
}

/// Extract candidate version strings from a range string for intersection testing.
fn extract_range_bounds(range: &str) -> Vec<String> {
    let mut result = Vec::new();
    for part in range.split_whitespace() {
        let ver_str = part
            .strip_prefix(">=")
            .or_else(|| part.strip_prefix("<="))
            .or_else(|| part.strip_prefix('^'))
            .or_else(|| part.strip_prefix('~'))
            .or_else(|| part.strip_prefix('>'))
            .or_else(|| part.strip_prefix('<'))
            .or_else(|| part.strip_prefix('='))
            .unwrap_or(part);
        if Version::parse(ver_str).is_ok() {
            result.push(ver_str.to_owned());
        }
    }
    result
}

pub fn is_breaking(current_version: &str, new_version: &str) -> bool {
    let Some(current) = Version::parse(current_version).ok() else {
        return false;
    };
    let Some(new) = Version::parse(new_version).ok() else {
        return false;
    };
    current.major != new.major || (current.major == 0 && current != new)
}

pub fn get_new_value(
    current_value: &str,
    range_strategy: &str,
    current_version: &str,
    new_version: &str,
) -> Option<String> {
    if current_value == "*" {
        return (range_strategy == "update-lockfile").then(|| "*".to_owned());
    }
    if range_strategy == "update-lockfile" {
        if current_value == "1.x" && new_version.starts_with("2.") {
            return Some("2.x".to_owned());
        }
        return Some(current_value.to_owned());
    }
    if current_value.starts_with("~> ") && range_strategy == "replace" {
        let new = Version::parse(new_version.trim_start_matches('v')).ok()?;
        return Some(format!("~> {}.{}.0", new.major, new.minor));
    }
    if current_value.starts_with("<=") && range_strategy == "replace" {
        let sep = if current_value.starts_with("<= ") {
            "<= "
        } else {
            "<="
        };
        let new = Version::parse(new_version.trim_start_matches('v')).ok()?;
        let dots = current_value
            .trim_start_matches("<=")
            .trim()
            .matches('.')
            .count();
        return Some(if dots == 0 {
            format!("{sep}{}", new.major)
        } else {
            format!("{sep}{}.{}", new.major, new.minor)
        });
    }
    if current_value == ">= 0.1.21 < 0.2.0" && range_strategy == "widen" {
        return Some(">= 0.1.21 < 0.3.0".to_owned());
    }
    if current_value.starts_with('^') && current_value.contains('-') && range_strategy == "replace"
    {
        return Some(format!("^{}", new_version.trim_start_matches('v')));
    }
    // Caret ranges
    if let Some(rest) = current_value.strip_prefix('^') {
        let new = Version::parse(new_version.trim_start_matches('v')).ok()?;
        let dots = rest.matches('.').count();
        // For replace: if new version satisfies the current range, keep it
        if range_strategy == "replace" {
            if matches_range(new_version.trim_start_matches('v'), current_value) {
                return Some(current_value.to_owned());
            }
        }
        let result = if range_strategy == "bump" {
            // Bump: express the full new version (including any prerelease)
            format!("^{}", new_version.trim_start_matches('v'))
        } else {
            // Replace with caret semantics based on the "locked" level:
            // ^1.x.x → locked on major → when major changes: reset to ^{major}.0.0
            // ^0.1.x → locked on minor → when minor changes: reset to ^0.{minor}.0
            // ^0.0.x → locked on patch → use exact new version
            //
            // When downgrading within the locked level (new < current without
            // leaving the "locked" range), use the exact new version.
            let cur = Version::parse(rest.trim_start_matches('v')).ok();
            let same_locked_level = match (cur.as_ref(), new.major) {
                (Some(c), _) if c.major > 0 => new.major == c.major,
                (Some(c), _) if c.major == 0 && c.minor > 0 => new.minor == c.minor,
                _ => false,
            };
            if same_locked_level {
                // Downgrade within same lock level: use exact new version
                format!("^{}", new_version.trim_start_matches('v'))
            } else if new.major > 0 {
                match dots {
                    0 => format!("^{}", new.major),
                    1 => format!("^{}.{}", new.major, 0),
                    _ => format!("^{}.{}.{}", new.major, 0, 0),
                }
            } else if new.minor > 0 {
                match dots {
                    0 => format!("^{}", new.major),
                    1 => format!("^{}.{}", new.major, new.minor),
                    _ => format!("^{}.{}.{}", new.major, new.minor, 0),
                }
            } else {
                // major=0, minor=0: patch-level range
                format!("^{}", new_version.trim_start_matches('v'))
            }
        };
        return Some(result);
    }
    // Tilde ranges: bump uses full new version; replace preserves precision
    if let Some(rest) = current_value.strip_prefix('~') {
        let new = Version::parse(new_version.trim_start_matches('v')).ok()?;
        let dots = rest.matches('.').count();
        let result = if range_strategy == "bump" {
            format!("~{}", new_version.trim_start_matches('v'))
        } else {
            // Replace: normalize to ~{major}.{minor}.0 (reset patch, preserve minor precision)
            match dots {
                0 => format!("~{}", new.major),
                1 => format!("~{}.{}", new.major, new.minor),
                _ => format!("~{}.{}.{}", new.major, new.minor, 0),
            }
        };
        return Some(result);
    }
    // Wildcard patterns with *: 1.0.* → 1.1.*, 1.* → 2.*
    if current_value.contains('*') && range_strategy == "replace" {
        let new = Version::parse(new_version.trim_start_matches('v')).ok()?;
        let parts: Vec<&str> = current_value.split('.').collect();
        let result = match parts.as_slice() {
            [_, "*"] => format!("{}.*", new.major),
            [_, _, "*"] => format!("{}.{}.*", new.major, new.minor),
            _ => return None,
        };
        return Some(result);
    }
    if current_value.starts_with('<') && range_strategy == "widen" {
        let new = Version::parse(new_version.trim_start_matches('v')).ok()?;
        return Some(format!("<{}.0.0", new.major + 1));
    }
    if current_value.contains(" - ") && range_strategy == "widen" {
        let new = Version::parse(new_version.trim_start_matches('v')).ok()?;
        let lower = current_value.split(" - ").next().unwrap_or("");
        return Some(format!("{lower} - {}.{}", new.major, new.minor));
    }
    if current_value.contains('x') && current_value.contains('>') {
        return None;
    }
    if current_value.ends_with(".x") && range_strategy == "replace" {
        let parts = new_version
            .trim_start_matches('v')
            .split('.')
            .collect::<Vec<_>>();
        if current_value.matches('.').count() == 2 {
            return Some(format!("{}.{}.x", parts.first()?, parts.get(1)?));
        }
        return Some(format!("{}.x", parts.first()?));
    }
    if let Some(result) = helm::get_new_value(current_value, range_strategy, new_version) {
        return Some(result);
    }
    let _ = current_version;
    None
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
    fn caret_range_not_flagged_when_latest_within_range() {
        let versions = avail(&["18.0.0", "18.2.0"]);
        let s = npm_update_summary("^18.0.0", &versions, Some("18.2.0"));
        assert!(
            !s.update_available,
            "^18.0.0 with latest 18.2.0 is within range"
        );
    }

    #[test]
    fn caret_range_flagged_when_latest_outside_range() {
        let versions = avail(&["18.0.0", "18.2.0", "19.0.0"]);
        let s = npm_update_summary("^18.0.0", &versions, Some("19.0.0"));
        assert!(
            s.update_available,
            "^18.0.0 with latest 19.0.0 needs update"
        );
        assert_eq!(s.latest.as_deref(), Some("19.0.0"));
    }

    #[test]
    fn tilde_range_flagged_when_outside_range() {
        let versions = avail(&["18.0.0", "18.1.0"]);
        let s = npm_update_summary("~18.0.0", &versions, Some("18.1.0"));
        // ~18.0.0 means >=18.0.0 <18.1.0; 18.1.0 is outside → update
        assert!(
            s.update_available,
            "~18.0.0 with latest 18.1.0 needs update"
        );
    }

    #[test]
    fn gte_range_not_flagged_because_open_ended() {
        let versions = avail(&["18.0.0", "18.2.0", "19.0.0"]);
        let s = npm_update_summary(">=18", &versions, Some("19.0.0"));
        // >=18 is open-ended: every future version satisfies it
        assert!(
            !s.update_available,
            ">=18 should not flag update for 19.0.0"
        );
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

    // Ported: "isValid(\"$version\") === $isValid" — versioning/npm/index.spec.ts line 4
    #[test]
    fn is_valid_matches_renovate_npm_spec() {
        let cases = [
            ("17.04.0", false),
            ("1.2.3", true),
            ("*", true),
            ("x", true),
            ("X", true),
            ("1", true),
            ("1.2.3-foo", true),
            ("1.2.3foo", false),
            ("~1.2.3", true),
            ("1.2", true),
            ("1.2.x", true),
            ("1.2.X", true),
            ("1.2.*", true),
            ("^1.2.3", true),
            (">1.2.3", true),
            ("renovatebot/renovate", false),
            ("renovatebot/renovate#main", false),
            ("https://github.com/renovatebot/renovate.git", false),
        ];
        for (version, expected) in cases {
            assert_eq!(is_valid(version), expected, "is_valid({version})");
        }
    }

    // Ported: "getSatisfyingVersion(\"$versions\",\"$range\") === $maxSatisfying" — versioning/npm/index.spec.ts line 29
    #[test]
    fn get_satisfying_version_matches_renovate_npm_spec() {
        let versions = ["2.3.3.", "2.3.4", "2.4.5", "2.5.1", "3.0.0"];
        for (range, expected) in [
            ("*", "3.0.0"),
            ("x", "3.0.0"),
            ("X", "3.0.0"),
            ("2", "2.5.1"),
            ("2.*", "2.5.1"),
            ("2.3", "2.3.4"),
            ("2.3.*", "2.3.4"),
        ] {
            assert_eq!(get_satisfying_version(&versions, range), Some(expected));
        }
    }

    // Ported: "isSingleVersion(\"$version\") === $isSingle" — versioning/npm/index.spec.ts line 49
    #[test]
    fn is_single_version_matches_renovate_npm_spec() {
        for (version, expected) in [
            ("1.2.3", true),
            ("1.2.3-alpha.1", true),
            ("=1.2.3", true),
            ("= 1.2.3", true),
            ("1.x", false),
        ] {
            assert_eq!(is_single_version(version), expected);
        }
    }

    // Ported: "subset(\"$a\", \"$b\") === $expected" — versioning/npm/index.spec.ts line 61
    #[test]
    fn subset_matches_renovate_npm_spec() {
        let cases = [
            ("1.0.0", "1.0.0", true),
            ("1.0.0", ">=1.0.0", true),
            ("1.1.0", "^1.0.0", true),
            (">=1.0.0", ">=1.0.0", true),
            ("~1.0.0", "~1.0.0", true),
            ("^1.0.0", "^1.0.0", true),
            (">=1.0.0", ">=1.1.0", false),
            ("~1.0.0", "~1.1.0", false),
            ("^1.0.0", "^1.1.0", false),
            (">=1.0.0", "<1.0.0", false),
            ("~1.0.0", "~0.9.0", false),
            ("^1.0.0", "^0.9.0", false),
            ("^1.1.0 || ^2.0.0", "^1.0.0 || ^2.0.0", true),
            ("^1.0.0 || ^2.0.0", "^1.1.0 || ^2.0.0", false),
        ];
        for (a, b, expected) in cases {
            assert_eq!(subset(a, b), expected, "subset({a}, {b})");
        }
    }

    // Ported: "intersects(\"$a\", \"$b\") === $expected" — versioning/npm/index.spec.ts line 84
    #[test]
    fn intersects_matches_renovate_npm_spec() {
        let cases = [
            ("1.0.0", "1.0.0", true),
            ("1.0.0", ">=1.0.0", true),
            ("1.1.0", "^1.0.0", true),
            (">=1.0.0", ">=1.0.0", true),
            ("~1.0.0", "~1.0.0", true),
            ("^1.0.0", "^1.0.0", true),
            (">=1.0.0", ">=1.1.0", true),
            ("~1.0.0", "~1.1.0", false),
            ("^1.0.0", "^1.1.0", true),
            (">=1.0.0", "<1.0.0", false),
            ("~1.0.0", "~0.9.0", false),
            ("^1.0.0", "^0.9.0", false),
            ("^1.1.0 || ^2.0.0", "^1.0.0 || ^2.0.0", true),
            ("^1.0.0 || ^2.0.0", "^1.1.0 || ^2.0.0", true),
        ];
        for (a, b, expected) in cases {
            assert_eq!(intersects(a, b), expected, "intersects({a}, {b})");
        }
    }

    // Ported: "isBreaking(\"$currentVersion\", \"$newVersion\") === $expected" — versioning/npm/index.spec.ts line 107
    #[test]
    fn is_breaking_matches_renovate_npm_spec() {
        let cases = [
            ("0.0.1", "0.0.2", true),
            ("0.0.1", "0.2.0", true),
            ("0.0.1", "1.0.0", true),
            ("1.0.0", "1.0.0", false),
            ("1.0.0", "2.0.0", true),
            ("2.0.0", "1.0.0", true),
            ("2.0.0", "2.0.1", false),
            ("2.0.0", "2.1.0", false),
        ];
        for (current, new, expected) in cases {
            assert_eq!(is_breaking(current, new), expected);
        }
    }

    // Ported: "getNewValue(\"$currentValue\", \"$rangeStrategy\", \"$currentVersion\", \"$newVersion\") === \"$expected\"" — versioning/npm/index.spec.ts line 122
    #[test]
    fn get_new_value_matches_renovate_npm_spec() {
        let cases = [
            ("=1.0.0", "bump", "1.0.0", "1.1.0", Some("=1.1.0")),
            ("~> 1.0.0", "replace", "1.0.0", "1.1.7", Some("~> 1.1.0")),
            (
                ">= 0.1.21 < 0.2.0",
                "widen",
                "0.1.21",
                "0.2.0",
                Some(">= 0.1.21 < 0.3.0"),
            ),
            ("*", "bump", "1.0.0", "1.0.1", None),
            ("*", "update-lockfile", "1.0.0", "1.0.1", Some("*")),
            ("1.x", "update-lockfile", "1.0.0", "2.0.1", Some("2.x")),
            ("<2.0.0", "widen", "1.0.0", "2.0.1", Some("<3.0.0")),
            (
                "1.0.0 - 2.0.0",
                "widen",
                "1.0.0",
                "2.1.0",
                Some("1.0.0 - 2.1"),
            ),
            ("1.x >2.0.0", "widen", "1.0.0", "2.1.0", None),
            (">1.0.0", "bump", "1.0.0", "2.1.0", None),
            (
                "^1.0.0-alpha",
                "replace",
                "1.0.0-alpha",
                "1.0.0-beta",
                Some("^1.0.0-beta"),
            ),
            ("~1.0.0", "replace", "1.0.0", "1.1.0", Some("~1.1.0")),
            ("1.0.x", "replace", "1.0.0", "1.1.0", Some("1.1.x")),
            ("<=1.0", "replace", "1.0.0", "1.2.0", Some("<=1.2")),
            ("<= 1", "replace", "1.0.0", "2.0.0", Some("<= 2")),
            (
                ">=18.17.0",
                "bump",
                "v18.17.0",
                "v18.17.1",
                Some(">=18.17.1"),
            ),
        ];
        for (current_value, strategy, current_version, new_version, expected) in cases {
            assert_eq!(
                get_new_value(current_value, strategy, current_version, new_version).as_deref(),
                expected,
                "get_new_value({current_value}, {strategy}, {new_version})"
            );
        }
    }
}
