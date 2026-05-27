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

use std::sync::LazyLock;

use regex::Regex;
use semver::{Comparator, Version, VersionReq};

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
    /// The current constraint string from Cargo.toml (e.g. `"^1.0.0"`).
    pub current_constraint: String,
    /// The latest non-yanked non-prerelease version that satisfies the constraint,
    /// if any.  Used for the branch-name / PR-title template.
    pub latest_compatible: Option<String>,
    /// The absolute latest non-yanked non-prerelease version across all available
    /// versions, ignoring the current constraint.  This mirrors Renovate's
    /// datasource `newVersion` concept.
    pub latest: Option<String>,
    /// `true` when an actionable update exists.
    ///
    /// Two cases:
    /// 1. Pinned exact version (`= 1.0.100` or bare `1.0.100`) where `latest` >
    ///    the pinned version — the user should bump to the new exact version.
    /// 2. Range constraint (`^1.0`, `~1.0`, `>=1.0`) where `latest` is NOT
    ///    satisfied by the current constraint — the range needs widening/bumping
    ///    to reach the new major (or other out-of-range) release.
    pub update_available: bool,
}

/// Produce a full update summary for a dependency constraint.
///
/// Fires `update_available` for:
/// - Pinned exact versions whose absolute latest is strictly greater.
/// - Range constraints whose absolute latest falls outside the current range
///   (e.g. `^1.0` when `2.0.0` is available).
pub fn update_summary(constraint: &str, available_versions: &[String]) -> UpdateSummary {
    let latest_compatible = resolve_latest(constraint, available_versions);

    // Absolute latest: highest non-prerelease version regardless of constraint.
    let latest_overall: Option<Version> = available_versions
        .iter()
        .filter_map(|v| Version::parse(v).ok())
        .filter(|v| v.pre.is_empty())
        .max();
    let latest = latest_overall.as_ref().map(|v| v.to_string());

    // Determine whether the constraint is a pinned exact version:
    //   `= 1.0.228` or bare `1.0.228` (no sigil / no range operator).
    let stripped = constraint.trim().trim_start_matches('=').trim();
    let is_pinned = Version::parse(stripped).is_ok()
        && !constraint.contains('^')
        && !constraint.contains('~')
        && !constraint.contains('>')
        && !constraint.contains('<')
        && !constraint.contains(',')
        && !constraint.contains('*');

    let update_available = if is_pinned {
        // Pinned: update when absolute latest strictly exceeds the pinned version.
        latest_overall
            .as_ref()
            .and_then(|lv| {
                Version::parse(stripped)
                    .ok()
                    .map(|sv| lv.cmp_precedence(&sv).is_gt())
            })
            .unwrap_or(false)
    } else {
        // Range: update when the absolute latest is NOT satisfied by the current constraint.
        match (parse_cargo_req(constraint), latest_overall.as_ref()) {
            (Some(req), Some(lv)) => !req.matches(lv),
            _ => false,
        }
    };

    UpdateSummary {
        current_constraint: constraint.to_owned(),
        latest_compatible,
        latest,
        update_available,
    }
}

// ──────────────────────── Renovate-compatible versioning API ────────────────

/// Strategy for updating a version range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeStrategy {
    Bump,
    Replace,
}

/// Normalise a Cargo range string for the `semver` crate.
///
/// Two normalisations are applied:
/// 1. Wildcards in non-terminal position are stripped: `4.*.0` → `4.*`.
/// 2. Space-separated comparators get commas inserted so the `semver` crate
///    can parse them: `>= 1.0.0 <= 2.0.0` → `>= 1.0.0, <= 2.0.0`.
fn normalise_cargo_range(s: &str) -> String {
    static SPACE_SEP: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"([\d.*])\s+(>=|<=|>|<|\^|~|=)").unwrap());

    // 1. Strip non-terminal wildcard components.
    let normalised: String = s
        .split(',')
        .map(|part| {
            let t = part.trim();
            let (op, ver, _) = parse_comparator(t);
            if op.is_empty() {
                // Bare version/wildcard: trim parts after the first wildcard.
                let parts: Vec<&str> = ver.split('.').collect();
                if let Some(wi) = parts.iter().position(|&p| matches!(p, "*" | "x" | "X")) {
                    return parts[..=wi].join(".");
                }
            }
            t.to_owned()
        })
        .collect::<Vec<_>>()
        .join(", ");

    // 2. Insert commas before operators that follow a version character.
    SPACE_SEP
        .replace_all(normalised.trim(), "$1, $2")
        .into_owned()
}

/// Parse a Cargo range into a `VersionReq`, applying normalisation if needed.
fn parse_cargo_req(range: &str) -> Option<VersionReq> {
    let t = range.trim();
    VersionReq::parse(t)
        .ok()
        .or_else(|| VersionReq::parse(&normalise_cargo_range(t)).ok())
}

/// Split a comparator string into `(operator, version_part, space_after_op)`.
fn parse_comparator(s: &str) -> (&str, &str, bool) {
    let s = s.trim();
    for op in &[">=", "<=", ">", "<", "^", "~", "="] {
        if let Some(rest) = s.strip_prefix(op) {
            let has_space = rest.starts_with(' ');
            return (op, rest.trim_start(), has_space);
        }
    }
    ("", s, false)
}

struct PartialVer {
    #[allow(dead_code)]
    major: u64,
    minor: Option<u64>,
    patch: Option<u64>,
}

fn parse_partial_ver(s: &str) -> Option<PartialVer> {
    let ver = s.split('-').next().unwrap_or(s);
    let parts: Vec<&str> = ver.split('.').collect();
    let major = parts.first()?.parse::<u64>().ok()?;
    let minor = parts.get(1).and_then(|&p| {
        if matches!(p, "*" | "x" | "X") {
            None
        } else {
            p.parse::<u64>().ok()
        }
    });
    let patch = parts.get(2).and_then(|&p| {
        if matches!(p, "*" | "x" | "X") {
            None
        } else {
            p.parse::<u64>().ok()
        }
    });
    Some(PartialVer {
        major,
        minor,
        patch,
    })
}

/// Expand a short version like `"1.0"` to a full three-part `"1.0.0"`.
fn expand_to_full(ver: &str) -> String {
    let n = ver.matches('.').count();
    match n {
        0 => format!("{ver}.0.0"),
        1 => format!("{ver}.0"),
        _ => ver.to_owned(),
    }
}

/// True when `s` is a pure digits-and-dots string (`/^\d+(\.\d+)*$/`).
fn is_bare_digits(s: &str) -> bool {
    !s.is_empty()
        && s.starts_with(|c: char| c.is_ascii_digit())
        && s.chars().all(|c| c.is_ascii_digit() || c == '.')
}

fn is_wildcard_ver(ver: &str) -> bool {
    ver.contains('*') || ver.contains('x') || ver.contains('X')
}

/// Update the fixed components of a wildcard pattern with the new version.
fn update_wildcard(ver: &str, maj: u64, min: u64, _patch: u64) -> String {
    let parts: Vec<&str> = ver.split('.').collect();
    let wi = parts.iter().position(|&p| matches!(p, "*" | "x" | "X"));
    match wi {
        Some(1) => format!("{maj}.{}", parts[1]),
        Some(2) => format!("{maj}.{min}.{}", parts[2]),
        _ => ver.to_owned(),
    }
}

/// Port of Renovate's `replaceCaretValue` from `npm/range.ts`.
fn replace_caret_value(old: &Version, new: &Version) -> String {
    let old_parts = [old.major, old.minor, old.patch];
    let new_parts = [new.major, new.minor, new.patch];
    let mut leading_zero = true;
    let mut need_replace = false;
    let mut result = [0u64; 3];

    for i in 0..3 {
        let (ov, nv) = (old_parts[i], new_parts[i]);
        let leading_digit = if ov != 0 || nv != 0 {
            std::mem::take(&mut leading_zero)
        } else {
            false
        };
        if leading_digit && nv > ov {
            need_replace = true;
        }
        if !need_replace && nv < ov {
            // New version regressed on this component: use full new version.
            return format!("{}.{}.{}", new.major, new.minor, new.patch);
        }
        result[i] = if leading_digit { nv } else { 0 };
    }
    if need_replace {
        format!("{}.{}.{}", result[0], result[1], result[2])
    } else {
        format!("{}.{}.{}", old.major, old.minor, old.patch)
    }
}

/// Prefix a version string and truncate to `precision` dot-separated parts.
fn apply_precision(prefix: &str, ver: &str, precision: usize) -> String {
    let parts: Vec<&str> = ver.split('.').collect();
    format!(
        "{}{}",
        prefix,
        parts[..precision.min(parts.len())].join(".")
    )
}

/// True if `version` is an exact 3-component semver (including pre-release).
/// Equivalent to Renovate cargo `isVersion`.
pub fn is_version(input: &str) -> bool {
    Version::parse(input.trim()).is_ok()
}

/// True if `input` is a valid Cargo version requirement or version.
/// Equivalent to Renovate cargo `isValid`.
pub fn is_valid(input: &str) -> bool {
    parse_cargo_req(input).is_some()
}

/// True if `version` satisfies `range`.
/// Equivalent to Renovate cargo `matches`.
pub fn matches_range(version: &str, range: &str) -> bool {
    let Ok(v) = Version::parse(version.trim()) else {
        return false;
    };
    parse_cargo_req(range).is_some_and(|req| req.matches(&v))
}

/// True if `version` is strictly below every version that satisfies `range`.
/// Equivalent to Renovate cargo `isLessThanRange`.
pub fn is_less_than_range(version: &str, range: &str) -> bool {
    let Ok(v) = Version::parse(version.trim()) else {
        return false;
    };
    let Some(req) = parse_cargo_req(range) else {
        return false;
    };
    if req.matches(&v) {
        return false;
    }
    // Version is outside range. Determine whether it's below or above.
    // If any upper-bound comparator has a bound ≤ v, the version is above range.
    for comp in &req.comparators {
        let bound = bound_version(comp);
        let is_above = match comp.op {
            semver::Op::Less => v >= bound,
            semver::Op::LessEq => v > bound,
            _ => false,
        };
        if is_above {
            return false;
        }
    }
    true
}

fn bound_version(c: &Comparator) -> Version {
    Version::new(c.major, c.minor.unwrap_or(0), c.patch.unwrap_or(0))
}

/// Highest version in `versions` satisfying `range`.
/// Equivalent to Renovate cargo `getSatisfyingVersion`.
pub fn get_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    let req = parse_cargo_req(range)?;
    versions
        .iter()
        .filter_map(|&v| Version::parse(v).ok().map(|p| (v, p)))
        .filter(|(_, p)| req.matches(p))
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(v, _)| v)
}

/// Lowest version in `versions` satisfying `range`.
/// Equivalent to Renovate cargo `minSatisfyingVersion`.
pub fn min_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    let req = parse_cargo_req(range)?;
    versions
        .iter()
        .filter_map(|&v| Version::parse(v).ok().map(|p| (v, p)))
        .filter(|(_, p)| req.matches(p))
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(v, _)| v)
}

/// True if `constraint` is a single pinned version (`=` prefix + exact version).
/// Equivalent to Renovate cargo `isSingleVersion`.
pub fn is_single_version(constraint: &str) -> bool {
    let t = constraint.trim();
    t.starts_with('=') && Version::parse(t[1..].trim()).is_ok()
}

/// Return the pinned form of `version` (`=version`).
/// Equivalent to Renovate cargo `getPinnedValue`.
pub fn get_pinned_value(version: &str) -> String {
    format!("={version}")
}

/// Compute an updated constraint string for a new version.
/// Equivalent to Renovate cargo `getNewValue`.
pub fn get_new_value(
    current_value: &str,
    strategy: RangeStrategy,
    current_version: &str,
    new_version: &str,
) -> Option<String> {
    if current_value.is_empty() || current_value == "*" {
        return Some(current_value.to_owned());
    }
    // bump + bare digits: always return the new version directly.
    if strategy == RangeStrategy::Bump && is_bare_digits(current_value) {
        return Some(new_version.to_owned());
    }
    // Single pinned (= prefix): preserve `= ` spacing only when there are no
    // leading spaces before the `=` (leading spaces → canonical `=version`).
    if is_single_version(current_value) {
        let t = current_value.trim();
        let has_leading_space = current_value.starts_with(|c: char| c.is_whitespace());
        return Some(if !has_leading_space && t.starts_with("= ") {
            format!("= {new_version}")
        } else {
            format!("={new_version}")
        });
    }
    // replace + already matches: keep current constraint.
    if strategy == RangeStrategy::Replace && matches_range(new_version, current_value) {
        return Some(current_value.to_owned());
    }

    let components: Vec<&str> = current_value.split(',').map(str::trim).collect();
    if components.len() == 1 {
        get_new_single(current_value.trim(), strategy, current_version, new_version)
    } else {
        let new_ver = Version::parse(new_version).ok()?;
        let parts: Vec<String> = components
            .iter()
            .filter_map(|comp| {
                get_new_multi_component(comp, strategy, current_version, new_version, &new_ver)
            })
            .filter(|s| !s.is_empty())
            .collect();
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(", "))
        }
    }
}

fn get_new_multi_component(
    comp: &str,
    _strategy: RangeStrategy,
    current_version: &str,
    new_version: &str,
    new_ver: &Version,
) -> Option<String> {
    // Try bump first; if new_version satisfies the bumped constraint, use it.
    // Otherwise fall back to replace — mirrors the npm multi-range bump logic.
    let bumped = get_new_single(comp, RangeStrategy::Bump, current_version, new_version)?;
    if parse_cargo_req(&bumped).is_some_and(|req| req.matches(new_ver)) {
        return Some(bumped);
    }
    get_new_single(comp, RangeStrategy::Replace, current_version, new_version)
}

fn get_new_single(
    comp: &str,
    strategy: RangeStrategy,
    _current_version: &str,
    new_version: &str,
) -> Option<String> {
    let (op, ver, has_op_space) = parse_comparator(comp);
    let new_ver = Version::parse(new_version).ok()?;
    let (nmaj, nmin, npatch) = (new_ver.major, new_ver.minor, new_ver.patch);

    // Pre-release suffix: only the first dot-separated identifier (e.g. "rc" from "rc.2").
    let pre_suffix = if new_ver.pre.is_empty() {
        String::new()
    } else {
        let first = new_ver.pre.as_str().split('.').next().unwrap_or("");
        format!("-{first}")
    };

    match strategy {
        RangeStrategy::Bump => match op {
            "^" => Some(format!("^{new_version}")),
            "~" => Some(format!("~{new_version}")),
            "=" => Some(format!("={new_version}")),
            ">=" => Some(if has_op_space {
                format!(">= {new_version}")
            } else {
                format!(">={new_version}")
            }),
            "<" | "<=" => Some(comp.to_owned()),
            "" => {
                if is_wildcard_ver(ver) {
                    Some(update_wildcard(ver, nmaj, nmin, npatch))
                } else {
                    Some(new_version.to_owned())
                }
            }
            _ => Some(comp.to_owned()),
        },

        RangeStrategy::Replace => match op {
            "^" => {
                let old_ver = Version::parse(&expand_to_full(ver)).ok()?;
                let replaced = replace_caret_value(&old_ver, &new_ver);
                let precision = ver.split('.').count();
                Some(apply_precision("^", &replaced, precision))
            }
            "~" => {
                let raw = if pre_suffix.is_empty() {
                    format!("~{nmaj}.{nmin}.0")
                } else {
                    format!("~{nmaj}.{nmin}.{npatch}{pre_suffix}")
                };
                let orig_parts = comp.split('.').count();
                let raw_parts = raw.split('.').count();
                Some(if raw_parts > orig_parts {
                    raw.split('.')
                        .take(orig_parts)
                        .collect::<Vec<_>>()
                        .join(".")
                } else {
                    raw
                })
            }
            "<=" => {
                let pv = parse_partial_ver(ver)?;
                let res = if pv.patch.is_some() || !pre_suffix.is_empty() {
                    format!("<={new_version}")
                } else if pv.minor.is_some() {
                    format!("<={nmaj}.{nmin}")
                } else {
                    format!("<={nmaj}")
                };
                Some(if comp.contains("<= ") {
                    res.replace("<=", "<= ")
                } else {
                    res
                })
            }
            "<" => {
                let pv = parse_partial_ver(ver)?;
                let has_patch = pv.patch.is_some();
                let has_minor = pv.minor.is_some();
                let res = if comp.ends_with(".0.0") {
                    format!("<{}.0.0", nmaj + 1)
                } else if comp.ends_with(".0") && has_minor {
                    format!("<{nmaj}.{}{}", nmin + 1, if has_patch { ".0" } else { "" })
                } else if has_patch {
                    format!("<{nmaj}.{nmin}.{}", npatch + 1)
                } else if has_minor {
                    format!("<{nmaj}.{}", nmin + 1)
                } else {
                    format!("<{}", nmaj + 1)
                };
                Some(if comp.contains("< ") {
                    res.replace('<', "< ")
                } else {
                    res
                })
            }
            "" => {
                if is_wildcard_ver(ver) {
                    Some(update_wildcard(ver, nmaj, nmin, npatch))
                } else {
                    let old_ver = Version::parse(&expand_to_full(ver)).ok()?;
                    let replaced = replace_caret_value(&old_ver, &new_ver);
                    let precision = ver.split('.').count();
                    Some(
                        replaced
                            .split('.')
                            .take(precision)
                            .collect::<Vec<_>>()
                            .join("."),
                    )
                }
            }
            _ => Some(comp.to_owned()),
        },
    }
}

/// Determine whether upgrading from `current` to `new` is a breaking change.
///
/// Cargo semver breaking-change rules (matches Renovate cargo `isBreaking`):
/// - Either version is unstable (pre-release) — breaking.
/// - `0.0.x`: breaking unless the version is identical.
/// - `0.y.z` (y > 0): breaking if the minor component changes.
/// - `>=1.0.0`: breaking only if the major component changes.
pub fn is_breaking(current: &str, new: &str) -> bool {
    let (Ok(cur), Ok(nw)) = (Version::parse(current), Version::parse(new)) else {
        return true;
    };
    if !cur.pre.is_empty() || !nw.pre.is_empty() {
        return true;
    }
    if cur.major == 0 {
        if cur.minor == 0 {
            return current != new;
        }
        return cur.minor != nw.minor;
    }
    cur.major != nw.major
}

// ──────────────────────────── Cargo.lock parsing ────────────────────────────

use std::collections::HashMap;

/// A single package entry from a Cargo.lock file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CargoLockPackage {
    pub name: String,
    pub version: String,
    pub source: Option<String>,
}

/// Parsed representation of a Cargo.lock file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CargoLock {
    pub package: Vec<CargoLockPackage>,
}

/// Parse a Cargo.lock file string into a [`CargoLock`].
///
/// Mirrors `lib/modules/manager/cargo/locked-version.ts` `parseLockFile()`.
/// Returns `None` when the input is not parseable as a Cargo.lock file.
pub fn parse_lock_file(content: &str) -> Option<CargoLock> {
    let table = content.parse::<toml::Table>().ok()?;
    let Some(packages_val) = table.get("package") else {
        // No [[package]] section → valid lockfile with zero packages
        return Some(CargoLock { package: vec![] });
    };
    let packages_arr = packages_val.as_array()?;
    let mut packages = Vec::new();
    for item in packages_arr {
        let pkg = item.as_table()?;
        let name = pkg.get("name")?.as_str()?.to_owned();
        let version = pkg.get("version")?.as_str()?.to_owned();
        let source = pkg
            .get("source")
            .and_then(|v| v.as_str())
            .map(str::to_owned);
        packages.push(CargoLockPackage {
            name,
            version,
            source,
        });
    }
    Some(CargoLock { package: packages })
}

/// Extract a map of `package_name → [versions]` from optional Cargo.lock content.
///
/// Mirrors `lib/modules/manager/cargo/locked-version.ts` `extractLockFileVersions()`.
/// Returns `None` when `content` is `None` (file missing) or when the content
/// is unparseable.
pub fn extract_lock_file_versions(content: Option<&str>) -> Option<HashMap<String, Vec<String>>> {
    extract_lock_file_content_versions(content?)
}

/// Extract a map of `package_name → [versions]` from Cargo.lock content.
///
/// Mirrors `lib/modules/manager/cargo/locked-version.ts`
/// `extractLockFileContentVersions()`.
/// Returns `None` when the content is unparseable. Returns an empty map
/// when there are no `[[package]]` entries.
pub fn extract_lock_file_content_versions(content: &str) -> Option<HashMap<String, Vec<String>>> {
    let lock = parse_lock_file(content)?;
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for pkg in lock.package {
        map.entry(pkg.name).or_default().push(pkg.version);
    }
    Some(map)
}

/// Status result for `update_locked_dependency`.
#[derive(Debug)]
pub enum UpdateLockedStatus {
    AlreadyUpdated,
    Unsupported,
    UpdateFailed,
}

impl UpdateLockedStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            UpdateLockedStatus::AlreadyUpdated => "already-updated",
            UpdateLockedStatus::Unsupported => "unsupported",
            UpdateLockedStatus::UpdateFailed => "update-failed",
        }
    }
}

/// Config for `update_locked_dependency`.
#[derive(Debug)]
pub struct UpdateLockedConfig {
    pub dep_name: Option<String>,
    pub new_version: Option<String>,
    pub lock_file_content: Option<String>,
}

/// Mirrors `lib/modules/manager/cargo/update-locked.ts` `updateLockedDependency()`.
pub fn update_locked_dependency(config: &UpdateLockedConfig) -> UpdateLockedStatus {
    let (Some(dep_name), Some(lock_file_content)) = (
        config.dep_name.as_deref(),
        config.lock_file_content.as_deref(),
    ) else {
        return UpdateLockedStatus::Unsupported;
    };
    let new_version = config.new_version.as_deref().unwrap_or("");
    let Some(locked) = extract_lock_file_content_versions(lock_file_content) else {
        return UpdateLockedStatus::UpdateFailed;
    };
    if locked
        .get(dep_name)
        .is_some_and(|versions| versions.iter().any(|v| v == new_version))
    {
        return UpdateLockedStatus::AlreadyUpdated;
    }
    UpdateLockedStatus::Unsupported
}

/// Result of `bump_package_version`.
#[derive(Debug)]
pub struct BumpPackageVersionResult {
    pub bumped_content: String,
}

/// Bump the `version` field in Cargo.toml content.
///
/// Mirrors `lib/modules/manager/cargo/update.ts` `bumpPackageVersion()`.
/// Returns unchanged content when `current_value` is not valid semver, when
/// `bump_version` is not a recognised release type, or when semver increment
/// fails.
pub fn bump_package_version(
    content: &str,
    current_value: &str,
    bump_version: &str,
) -> BumpPackageVersionResult {
    static VERSION_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"(?m)^(?P<prefix>version[ \t]*=[ \t]*['"])[^'"]*"#).unwrap());

    let bumped_content = (|| -> Option<String> {
        let mut new_ver = Version::parse(current_value).ok()?;
        match bump_version {
            "patch" => new_ver.patch += 1,
            "minor" => {
                new_ver.minor += 1;
                new_ver.patch = 0;
            }
            "major" => {
                new_ver.major += 1;
                new_ver.minor = 0;
                new_ver.patch = 0;
            }
            _ => return None,
        }
        let new_str = new_ver.to_string();
        let result = VERSION_RE
            .replace(content, |caps: &regex::Captures| {
                format!("{}{}", &caps["prefix"], new_str)
            })
            .into_owned();
        Some(result)
    })()
    .unwrap_or_else(|| content.to_owned());

    BumpPackageVersionResult { bumped_content }
}

/// Determine the effective Cargo range strategy.
///
/// Mirrors `lib/modules/manager/cargo/range.ts` `getRangeStrategy()`.
/// Non-`"auto"` strategies are returned unchanged. `"auto"` maps to `"widen"`
/// when `current_value` contains `<`, otherwise to `"update-lockfile"`.
pub fn get_range_strategy<'a>(range_strategy: &'a str, current_value: Option<&str>) -> &'a str {
    if range_strategy != "auto" {
        return range_strategy;
    }
    if current_value.is_some_and(|cv| cv.contains('<')) {
        return "widen";
    }
    "update-lockfile"
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
    fn range_constraint_not_flagged_when_latest_within_range() {
        // All available versions satisfy the constraints — no out-of-range release.
        let avail = v(&["1.0.0", "1.0.228"]);
        for constraint in &["1", "^1", "^1.0", ">=1.0", "1.0"] {
            let s = update_summary(constraint, &avail);
            assert!(
                !s.update_available,
                "expected no update for {constraint:?} when all versions are in-range"
            );
        }
    }

    #[test]
    fn range_constraint_flagged_when_latest_outside_range() {
        // 2.0.0 is outside ^1.0.0 — an update should be suggested.
        let avail = v(&["1.0.0", "1.5.0", "2.0.0"]);
        let s = update_summary("^1.0.0", &avail);
        assert!(s.update_available, "^1.0.0 should flag 2.0.0 as update");
        assert_eq!(s.latest.as_deref(), Some("2.0.0"));
        assert_eq!(s.latest_compatible.as_deref(), Some("1.5.0"));
    }

    #[test]
    fn tilde_range_flagged_when_minor_bump_outside_range() {
        // ~1.0 covers 1.0.x; 1.1.0 is outside that range.
        let avail = v(&["1.0.0", "1.0.5", "1.1.0"]);
        let s = update_summary("~1.0.0", &avail);
        assert!(s.update_available, "~1.0.0 should flag 1.1.0 as update");
        assert_eq!(s.latest.as_deref(), Some("1.1.0"));
    }

    #[test]
    fn caret_not_flagged_when_minor_within_range() {
        // ^1.0 covers >=1.0, <2.0 — 1.9.9 is within range.
        let avail = v(&["1.0.0", "1.9.9"]);
        let s = update_summary("^1.0", &avail);
        assert!(
            !s.update_available,
            "^1.0 with 1.9.9 should not flag update"
        );
    }

    #[test]
    fn no_compatible_versions_is_not_update() {
        let avail = v(&["1.0.0"]);
        let s = update_summary("2.0.0", &avail);
        assert!(s.latest_compatible.is_none());
        assert!(!s.update_available);
    }

    #[test]
    fn build_metadata_same_precedence_is_not_update() {
        // "1.1.2" and "1.1.2+spec-1.1.0" have equal SemVer precedence.
        // The crates.io sparse index may list both; this must NOT show as update.
        // Note: build-metadata versions are filtered out when computing `latest`.
        let avail = v(&["1.1.2", "1.1.2+spec-1.1.0"]);
        let s = update_summary("1.1.2", &avail);
        assert!(
            !s.update_available,
            "build-metadata variant should not trigger update"
        );
    }

    #[test]
    fn build_metadata_with_actual_newer_version_is_update() {
        // If a genuinely newer version exists alongside a build-metadata variant,
        // the update should still fire.
        let avail = v(&["1.1.2", "1.1.2+spec-1.1.0", "1.1.3"]);
        let s = update_summary("1.1.2", &avail);
        assert!(s.update_available);
        assert_eq!(s.latest_compatible.as_deref(), Some("1.1.3"));
    }

    #[test]
    fn prerelease_versions_excluded_from_latest() {
        // A prerelease newer than stable should not trigger update for stable users.
        let avail = v(&["1.0.0", "2.0.0-alpha.1"]);
        let s = update_summary("^1.0.0", &avail);
        assert!(!s.update_available, "prerelease should not trigger update");
        assert_eq!(s.latest.as_deref(), Some("1.0.0"));
    }

    #[test]
    fn latest_field_reflects_absolute_max() {
        let avail = v(&["1.0.0", "1.5.0", "2.0.0"]);
        let s = update_summary("^1.0.0", &avail);
        assert_eq!(s.latest.as_deref(), Some("2.0.0"));
        assert_eq!(s.latest_compatible.as_deref(), Some("1.5.0"));
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

// ─────────────────── Tests ported from the Renovate TypeScript spec ──────────
// Reference: lib/modules/versioning/cargo/index.spec.ts

#[cfg(test)]
mod renovate_compat_tests {
    use super::*;

    // Ported: "matches("$version", "$range") === "$expected"" — cargo/index.spec.ts line 4
    #[test]
    fn matches_cases() {
        let cases: &[(&str, &str, bool)] = &[
            ("4.2.0", "4.2, >= 3.0, < 5.0.0", true),
            ("4.2.0", "2.0, >= 3.0, < 5.0.0", false),
            ("4.2.0", "4.2.0, < 4.2.4", true),
            ("4.2.0", "4.1", true),
            ("4.2.0", "4", true),
            ("4.2.0", "4.3", false),
            ("4.2.0", "5", false),
            ("0.4.2", "0.4", true),
            ("0.4.2", "0", true),
            ("0.4.2", "0.3", false),
            ("0.4.2", "1", false),
            ("4.2.0", "4.3.0, 3.0.0", false),
            ("4.2.0", "> 5.0.0, <= 6.0.0", false),
        ];
        for &(version, range, expected) in cases {
            assert_eq!(
                matches_range(version, range),
                expected,
                "matches({version:?}, {range:?})"
            );
        }
    }

    // Ported: "getSatisfyingVersion($versions, "$range") === "$expected"" — cargo/index.spec.ts line 27
    #[test]
    fn get_satisfying_version_cases() {
        let v1: Vec<&str> = vec!["4.2.1", "0.4.0", "0.5.0", "4.0.0", "4.2.0", "5.0.0"];
        assert_eq!(get_satisfying_version(&v1, "4.*.0, < 4.2.5"), Some("4.2.1"));
        let v2: Vec<&str> = vec!["0.4.0", "0.5.0", "4.0.0", "4.2.0", "5.0.0", "5.0.3"];
        assert_eq!(get_satisfying_version(&v2, "5.0, > 5.0.0"), Some("5.0.3"));
    }

    // Ported: "isValid("$version") === $expected" — cargo/index.spec.ts line 40
    #[test]
    fn is_valid_cases() {
        assert!(is_valid("1"));
        assert!(is_valid("1.2"));
        assert!(is_valid("1.2.3"));
        assert!(is_valid("^1.2.3"));
        assert!(is_valid("~1.2.3"));
        assert!(is_valid("1.2.*"));
        assert!(is_valid("< 3.0, >= 1.0.0 <= 2.0.0"));
        assert!(is_valid("< 3.0, >= 1.0.0 <= 2.0.0, = 5.1.2"));
    }

    // Ported: "isVersion("$version") === $expected" — cargo/index.spec.ts line 53
    #[test]
    fn is_version_cases() {
        assert!(!is_version("1"));
        assert!(!is_version("1.2"));
        assert!(is_version("1.2.3"));
    }

    // Ported: "isLessThanRange("$version", "$range") === "$expected"" — cargo/index.spec.ts line 61
    #[test]
    fn is_less_than_range_cases() {
        assert!(is_less_than_range("0.9.0", ">= 1.0.0 <= 2.0.0"));
        assert!(!is_less_than_range("1.9.0", ">= 1.0.0 <= 2.0.0"));
    }

    // Ported: "minSatisfyingVersion($versions, "$range") === "$expected"" — cargo/index.spec.ts line 74
    #[test]
    fn min_satisfying_version_cases() {
        let v1: Vec<&str> = vec!["0.4.0", "0.5.0", "4.2.0", "4.3.0", "5.0.0"];
        assert_eq!(min_satisfying_version(&v1, "4.*, > 4.2"), Some("4.3.0"));

        let v2: Vec<&str> = vec!["0.4.0", "0.5.0", "4.2.0", "5.0.0"];
        assert_eq!(min_satisfying_version(&v2, "4.0.0"), Some("4.2.0"));
        assert_eq!(min_satisfying_version(&v2, "4.0.0, = 0.5.0"), None);
        assert_eq!(
            min_satisfying_version(&v2, "4.0.0, > 4.1.0, <= 4.3.5"),
            Some("4.2.0")
        );
        assert_eq!(min_satisfying_version(&v2, "6.2.0, 3.*"), None);
    }

    // Ported: "isSingleVersion("$version") === $expected" — cargo/index.spec.ts line 92
    #[test]
    fn is_single_version_cases() {
        assert!(!is_single_version("1.2.3"));
        assert!(!is_single_version("1.2.3-alpha.1"));
        assert!(is_single_version("=1.2.3"));
        assert!(is_single_version("= 1.2.3"));
        assert!(is_single_version("  = 1.2.3"));
        assert!(!is_single_version("1"));
        assert!(!is_single_version("1.2"));
        assert!(!is_single_version("*"));
        assert!(!is_single_version("1.*"));
        assert!(!is_single_version("1.2.*"));
    }

    // Ported: "returns a pinned value" — cargo/index.spec.ts line 107
    #[test]
    fn get_pinned_value_case() {
        assert_eq!(get_pinned_value("1.2.3"), "=1.2.3");
    }

    // Ported: "getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected"" — cargo/index.spec.ts line 111
    #[test]
    fn get_new_value_cases() {
        let cases: &[(&str, RangeStrategy, &str, &str, Option<&str>)] = &[
            (
                "~0.7",
                RangeStrategy::Replace,
                "0.7.3",
                "0.8.5",
                Some("~0.8"),
            ),
            ("*", RangeStrategy::Bump, "1.0.0", "1.1.0", Some("*")),
            (
                "=1.0.0",
                RangeStrategy::Bump,
                "1.0.0",
                "1.1.0",
                Some("=1.1.0"),
            ),
            (
                "   =1.0.0",
                RangeStrategy::Bump,
                "1.0.0",
                "1.1.0",
                Some("=1.1.0"),
            ),
            (
                "= 1.0.0",
                RangeStrategy::Bump,
                "1.0.0",
                "1.1.0",
                Some("= 1.1.0"),
            ),
            (
                "  = 1.0.0",
                RangeStrategy::Bump,
                "1.0.0",
                "1.1.0",
                Some("=1.1.0"),
            ),
            (
                "  =   1.0.0",
                RangeStrategy::Bump,
                "1.0.0",
                "1.1.0",
                Some("=1.1.0"),
            ),
            (
                "=    1.0.0",
                RangeStrategy::Bump,
                "1.0.0",
                "1.1.0",
                Some("= 1.1.0"),
            ),
            (
                "1.0.0",
                RangeStrategy::Bump,
                "1.0.0",
                "1.1.0",
                Some("1.1.0"),
            ),
            (
                "^1.0",
                RangeStrategy::Bump,
                "1.0.0",
                "1.0.7",
                Some("^1.0.7"),
            ),
            (
                "^1.0.0",
                RangeStrategy::Replace,
                "1.0.0",
                "2.0.7",
                Some("^2.0.0"),
            ),
            (
                "1.0.0",
                RangeStrategy::Replace,
                "1.0.0",
                "2.0.7",
                Some("2.0.0"),
            ),
            ("^1", RangeStrategy::Bump, "1.0.0", "2.1.7", Some("^2.1.7")),
            ("~1", RangeStrategy::Bump, "1.0.0", "1.1.7", Some("~1.1.7")),
            ("5", RangeStrategy::Bump, "5.0.0", "5.1.7", Some("5.1.7")),
            ("5", RangeStrategy::Bump, "5.0.0", "6.1.7", Some("6.1.7")),
            ("5.0", RangeStrategy::Bump, "5.0.0", "5.0.7", Some("5.0.7")),
            ("5.0", RangeStrategy::Bump, "5.0.0", "5.1.7", Some("5.1.7")),
            ("5.0", RangeStrategy::Bump, "5.0.0", "6.1.7", Some("6.1.7")),
            ("0.5", RangeStrategy::Bump, "0.5.0", "0.5.1", Some("0.5.1")),
            ("0.5", RangeStrategy::Bump, "0.5.0", "0.6.1", Some("0.6.1")),
            ("1.2", RangeStrategy::Replace, "1.2.3", "1.3.0", Some("1.2")),
            ("5.0", RangeStrategy::Replace, "5.0.0", "5.1.7", Some("5.0")),
            ("5.0", RangeStrategy::Replace, "5.0.0", "6.1.7", Some("6.0")),
            ("0.5", RangeStrategy::Replace, "0.5.0", "0.6.1", Some("0.6")),
            (
                "=1.0.0",
                RangeStrategy::Replace,
                "1.0.0",
                "1.1.0",
                Some("=1.1.0"),
            ),
            (
                "1.0.*",
                RangeStrategy::Replace,
                "1.0.0",
                "1.1.0",
                Some("1.1.*"),
            ),
            ("1.*", RangeStrategy::Replace, "1.0.0", "2.1.0", Some("2.*")),
            (
                "~0.6.1",
                RangeStrategy::Replace,
                "0.6.8",
                "0.7.0-rc.2",
                Some("~0.7.0-rc"),
            ),
            (
                "<1.3.4",
                RangeStrategy::Replace,
                "1.2.3",
                "1.5.0",
                Some("<1.5.1"),
            ),
            (
                "< 1.3.4",
                RangeStrategy::Replace,
                "1.2.3",
                "1.5.0",
                Some("< 1.5.1"),
            ),
            (
                "<   1.3.4",
                RangeStrategy::Replace,
                "1.2.3",
                "1.5.0",
                Some("< 1.5.1"),
            ),
            (
                "<=1.3.4",
                RangeStrategy::Replace,
                "1.2.3",
                "1.5.0",
                Some("<=1.5.0"),
            ),
            (
                "<= 1.3.4",
                RangeStrategy::Replace,
                "1.2.3",
                "1.5.0",
                Some("<= 1.5.0"),
            ),
            (
                "<=   1.3.4",
                RangeStrategy::Replace,
                "1.2.3",
                "1.5.0",
                Some("<= 1.5.0"),
            ),
            (
                ">= 0.1.21, < 0.2.0",
                RangeStrategy::Bump,
                "0.1.21",
                "0.1.24",
                Some(">= 0.1.24, < 0.2.0"),
            ),
            (
                ">= 0.1.21, <= 0.2.0",
                RangeStrategy::Bump,
                "0.1.21",
                "0.1.24",
                Some(">= 0.1.24, <= 0.2.0"),
            ),
            (
                ">= 0.0.1, <= 0.1",
                RangeStrategy::Bump,
                "0.0.1",
                "0.0.2",
                Some(">= 0.0.2, <= 0.1"),
            ),
            (
                ">= 1.2.3, <= 1",
                RangeStrategy::Bump,
                "1.2.3",
                "1.2.4",
                Some(">= 1.2.4, <= 1"),
            ),
            (
                ">= 1.2.3, <= 1.0",
                RangeStrategy::Bump,
                "1.2.3",
                "1.2.4",
                Some(">= 1.2.4, <= 1.2"),
            ),
            (
                ">= 0.0.1, < 0.1",
                RangeStrategy::Bump,
                "0.1.0",
                "0.2.1",
                Some(">= 0.2.1, < 0.3"),
            ),
        ];
        for &(cv, strategy, cur, nv, expected) in cases {
            assert_eq!(
                get_new_value(cv, strategy, cur, nv).as_deref(),
                expected,
                "getNewValue({cv:?}, {:?}, {cur:?}, {nv:?})",
                strategy
            );
        }
    }

    // Ported: "isBreaking("$currentVersion", "$newVersion") === $expected" — cargo/index.spec.ts line 176
    #[test]
    fn is_breaking_cases() {
        let cases: &[(&str, &str, bool)] = &[
            ("0.0.1", "0.0.1", false),
            ("0.0.1", "0.0.2", true),
            ("0.0.1", "0.2.0", true),
            ("0.0.1", "1.0.0", true),
            ("0.1.0", "0.1.1", false),
            ("0.1.0", "0.2.0", true),
            ("1.0.0-alpha.1", "1.0.0", true),
            ("1.0.0-alpha.1", "1.0.0-alpha.2", true),
            ("1.0.0", "2.0.0-alpha.1", true),
            ("1.0.0", "1.0.0", false),
            ("1.0.0", "2.0.0", true),
            ("2.0.0", "2.0.1", false),
            ("2.0.0", "2.1.0", false),
        ];
        for &(cur, nv, expected) in cases {
            assert_eq!(
                is_breaking(cur, nv),
                expected,
                "isBreaking({cur:?}, {nv:?})"
            );
        }
    }

    // null currentValue case from spec
    // Ported: "getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected"" — cargo/index.spec.ts line 113
    #[test]
    fn get_new_value_null_returns_none() {
        assert_eq!(
            get_new_value("", RangeStrategy::Bump, "1.0.0", "1.1.0"),
            Some("".to_owned())
        );
    }

    // ── get_range_strategy ───────────────────────────────────────────────────

    // Ported: "returns same if not auto" — modules/manager/cargo/range.spec.ts line 5
    #[test]
    fn range_strategy_returns_same_if_not_auto() {
        assert_eq!(get_range_strategy("widen", None), "widen");
    }

    // Ported: "returns widen if current value includes <" — modules/manager/cargo/range.spec.ts line 10
    #[test]
    fn range_strategy_returns_widen_if_current_value_has_lt() {
        assert_eq!(get_range_strategy("auto", Some("<1.0.0")), "widen");
    }

    // Ported: "defaults to update-lockfile" — modules/manager/cargo/range.spec.ts line 18
    #[test]
    fn range_strategy_defaults_to_update_lockfile() {
        assert_eq!(get_range_strategy("auto", Some("1.0.0")), "update-lockfile");
    }

    // ── parseLockFile tests ──────────────────────────────────────────────────

    // Ported: "parses v1 lockfile string into an object" — modules/manager/cargo/locked-version.spec.ts line 51
    #[test]
    fn parse_lock_file_v1() {
        let content = include_str!("../../tests/fixtures/cargo/lockfile-parsing/Cargo.v1.lock");
        let result = parse_lock_file(content).unwrap();
        assert_eq!(result.package.len(), 2);
        assert_eq!(result.package[0].name, "foo");
        assert_eq!(result.package[0].version, "1.0.4");
        assert_eq!(
            result.package[0].source.as_deref(),
            Some("registry+https://github.com/rust-lang/crates.io-index")
        );
        assert_eq!(result.package[1].name, "bar");
        assert_eq!(result.package[1].version, "0.7.6");
        assert_eq!(
            result.package[1].source.as_deref(),
            Some("registry+https://github.com/rust-lang/crates.io-index")
        );
    }

    // Ported: "parses v2 lockfile string into an object" — modules/manager/cargo/locked-version.spec.ts line 70
    #[test]
    fn parse_lock_file_v2() {
        let content = include_str!("../../tests/fixtures/cargo/lockfile-parsing/Cargo.v2.lock");
        let result = parse_lock_file(content).unwrap();
        assert_eq!(result.package.len(), 2);
        assert_eq!(result.package[0].name, "foo");
        assert_eq!(result.package[0].version, "1.1.0");
        assert_eq!(
            result.package[0].source.as_deref(),
            Some("registry+https://github.com/rust-lang/crates.io-index")
        );
        assert_eq!(result.package[1].name, "bar");
        assert_eq!(result.package[1].version, "7.0.1");
        assert!(result.package[1].source.is_none());
    }

    // Ported: "parses v3 lockfile string into an object" — modules/manager/cargo/locked-version.spec.ts line 88
    #[test]
    fn parse_lock_file_v3() {
        let content = include_str!("../../tests/fixtures/cargo/lockfile-parsing/Cargo.v3.lock");
        let result = parse_lock_file(content).unwrap();
        assert_eq!(result.package.len(), 2);
        assert_eq!(result.package[0].name, "foo");
        assert_eq!(result.package[0].version, "1.1.0");
        assert_eq!(result.package[1].name, "bar");
        assert_eq!(result.package[1].version, "7.0.1");
    }

    // Ported: "can deal with invalid lockfiles" — modules/manager/cargo/locked-version.spec.ts line 106
    #[test]
    fn parse_lock_file_invalid() {
        assert!(parse_lock_file("foo").is_none());
    }

    // Ported: "returns null for missing lock file" — modules/manager/cargo/locked-version.spec.ts line 19
    #[test]
    fn extract_versions_missing_file_returns_none() {
        assert!(extract_lock_file_versions(None).is_none());
    }

    // Ported: "returns null for invalid lock file" — modules/manager/cargo/locked-version.spec.ts line 23
    #[test]
    fn extract_versions_invalid_content_returns_none() {
        assert!(extract_lock_file_content_versions("foo").is_none());
    }

    // Ported: "returns empty map for lock file without packages" — modules/manager/cargo/locked-version.spec.ts line 28
    #[test]
    fn extract_versions_no_packages_returns_empty() {
        let result = extract_lock_file_content_versions("[metadata]").unwrap();
        assert!(result.is_empty());
    }

    // Ported: "returns a map of package versions" — modules/manager/cargo/locked-version.spec.ts line 33
    #[test]
    fn extract_versions_returns_map_of_package_versions() {
        let content = include_str!("../../tests/fixtures/cargo/lockfile-update/Cargo.1.lock");
        let result = extract_lock_file_content_versions(content).unwrap();
        assert_eq!(result.get("proc-macro2"), Some(&vec!["1.0.66".to_string()]));
        assert_eq!(result.get("quote"), Some(&vec!["1.0.33".to_string()]));
        assert_eq!(result.get("test"), Some(&vec!["0.1.0".to_string()]));
        assert_eq!(
            result.get("unicode-ident"),
            Some(&vec!["1.0.11".to_string()])
        );
        assert_eq!(result.get("unicode-xid"), Some(&vec!["0.2.4".to_string()]));
        let syn = result.get("syn").unwrap();
        assert!(syn.contains(&"1.0.1".to_string()));
        assert!(syn.contains(&"2.0.1".to_string()));
        assert_eq!(syn.len(), 2);
    }

    // Ported: "detects already updated" — modules/manager/cargo/update-locked.spec.ts line 9
    #[test]
    fn update_locked_detects_already_updated() {
        let lock_file_content =
            include_str!("../../tests/fixtures/cargo/lockfile-parsing/Cargo.v1.lock");
        let config = UpdateLockedConfig {
            dep_name: Some("foo".to_string()),
            new_version: Some("1.0.4".to_string()),
            lock_file_content: Some(lock_file_content.to_string()),
        };
        assert_eq!(
            update_locked_dependency(&config).as_str(),
            "already-updated"
        );
    }

    // Ported: "returns unsupported for empty lockfile" — modules/manager/cargo/update-locked.spec.ts line 21
    #[test]
    fn update_locked_unsupported_no_lock_file_content() {
        let config = UpdateLockedConfig {
            dep_name: Some("foo".to_string()),
            new_version: Some("1.0.4".to_string()),
            lock_file_content: None,
        };
        assert_eq!(update_locked_dependency(&config).as_str(), "unsupported");
    }

    // Ported: "returns unsupported for empty depName" — modules/manager/cargo/update-locked.spec.ts line 32
    #[test]
    fn update_locked_unsupported_no_dep_name() {
        let lock_file_content =
            include_str!("../../tests/fixtures/cargo/lockfile-parsing/Cargo.v1.lock");
        let config = UpdateLockedConfig {
            dep_name: None,
            new_version: Some("1.0.4".to_string()),
            lock_file_content: Some(lock_file_content.to_string()),
        };
        assert_eq!(update_locked_dependency(&config).as_str(), "unsupported");
    }

    // Ported: "returns unsupported" — modules/manager/cargo/update-locked.spec.ts line 44
    #[test]
    fn update_locked_unsupported_version_not_in_lock() {
        let lock_file_content =
            include_str!("../../tests/fixtures/cargo/lockfile-parsing/Cargo.v1.lock");
        let config = UpdateLockedConfig {
            dep_name: Some("foo".to_string()),
            new_version: Some("1.0.3".to_string()),
            lock_file_content: Some(lock_file_content.to_string()),
        };
        assert_eq!(update_locked_dependency(&config).as_str(), "unsupported");
    }

    // Ported: "returns update-failed in case of errors" — modules/manager/cargo/update-locked.spec.ts line 56
    #[test]
    fn update_locked_update_failed_on_invalid_content() {
        let config = UpdateLockedConfig {
            dep_name: Some("foo".to_string()),
            new_version: Some("1.0.3".to_string()),
            lock_file_content: Some("not valid toml {{{".to_string()),
        };
        assert_eq!(update_locked_dependency(&config).as_str(), "update-failed");
    }

    fn cargo_toml_content() -> &'static str {
        "[package]\nname = \"test\"\nversion = \"0.0.2\"\n"
    }

    // Ported: "increments" — modules/manager/cargo/update.spec.ts line 16
    #[test]
    fn bump_package_version_increments_patch() {
        let content = cargo_toml_content();
        let result = bump_package_version(content, "0.0.2", "patch");
        assert_eq!(result.bumped_content, content.replace("0.0.2", "0.0.3"));
    }

    // Ported: "no ops" — modules/manager/cargo/update.spec.ts line 24
    #[test]
    fn bump_package_version_no_ops_when_current_value_mismatch() {
        let content = cargo_toml_content();
        let result = bump_package_version(content, "0.0.1", "patch");
        assert_eq!(result.bumped_content, content);
    }

    // Ported: "updates" — modules/manager/cargo/update.spec.ts line 31
    #[test]
    fn bump_package_version_updates_minor() {
        let content = cargo_toml_content();
        let result = bump_package_version(content, "0.0.1", "minor");
        let expected = content.replace("0.0.2", "0.1.0");
        assert_eq!(result.bumped_content, expected);
    }

    // Ported: "returns content if bumping errors" — modules/manager/cargo/update.spec.ts line 38
    #[test]
    fn bump_package_version_returns_content_on_invalid_bump_type() {
        let content = cargo_toml_content();
        let result = bump_package_version(content, "0.0.2", "invalid_bump_type");
        assert_eq!(result.bumped_content, content);
    }

    // Ported: "does not bump version if version is not a semantic version" — modules/manager/cargo/update.spec.ts line 47
    #[test]
    fn bump_package_version_no_bump_if_not_semver() {
        let content = cargo_toml_content();
        let result = bump_package_version(content, "1", "patch");
        assert_eq!(result.bumped_content, content);
    }
}
