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
use std::sync::LazyLock;

use regex::Regex;

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

/// Return true when `version` is a syntactically valid NuGet version.
///
/// Mirrors `lib/modules/versioning/nuget/index.ts` `isValid()`/`isVersion()`.
/// A valid NuGet version has 1–4 dot-separated numeric components, optionally
/// followed by a pre-release label (`-anything`).
pub fn is_valid(version: &str) -> bool {
    let v = version.trim();
    if v.is_empty() {
        return false;
    }
    // Strip build metadata suffix.
    let v = v.split_once('+').map_or(v, |(base, _)| base);
    let (numeric, _) = v.split_once('-').unwrap_or((v, ""));
    let parts: Vec<&str> = numeric.split('.').collect();
    if parts.is_empty() || parts.len() > 4 {
        return false;
    }
    parts
        .iter()
        .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
}

/// Sort comparator for NuGet version strings — mirrors
/// `lib/modules/datasource/nuget/common.ts` `sortNugetVersions()`.
pub fn sort_nuget_versions(a: &str, b: &str) -> std::cmp::Ordering {
    match (is_valid(a), is_valid(b)) {
        (true, true) => compare(a, b),
        (true, false) => std::cmp::Ordering::Greater,
        (false, true) => std::cmp::Ordering::Less,
        (false, false) => std::cmp::Ordering::Equal,
    }
}

// ── Parser types ─────────────────────────────────────────────────────────────

/// Parsed representation of a NuGet version string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NugetVersionData {
    pub major: u64,
    pub minor: Option<u64>,
    pub patch: Option<u64>,
    pub revision: Option<u64>,
    pub prerelease: Option<String>,
    pub metadata: Option<String>,
}

/// Which component of a floating range is wildcard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NugetFloatingField {
    Major,
    Minor,
    Patch,
    Revision,
}

/// Parsed representation of a NuGet floating range (e.g. `1.2.*`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NugetFloatingRangeData {
    pub major: u64,
    pub minor: Option<u64>,
    pub patch: Option<u64>,
    pub revision: Option<u64>,
    pub floating: Option<NugetFloatingField>,
    pub prerelease: Option<String>,
}

/// Parsed representation of an exact NuGet range (e.g. `[1.2.3]`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NugetExactRangeData {
    pub version: NugetVersionData,
}

/// Min bound of a bracket range — either a fixed version or a floating range.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NugetBracketMin {
    Version(NugetVersionData),
    Floating(NugetFloatingRangeData),
}

/// Parsed representation of a NuGet bracket range (e.g. `[1.0,2.0)`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NugetBracketRangeData {
    pub min: Option<NugetBracketMin>,
    pub max: Option<NugetVersionData>,
    pub min_inclusive: bool,
    pub max_inclusive: bool,
}

/// Any valid NuGet range expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NugetRangeData {
    Exact(NugetExactRangeData),
    Floating(NugetFloatingRangeData),
    Bracket(NugetBracketRangeData),
}

// ── Parser functions ──────────────────────────────────────────────────────────

/// Parse a NuGet version string into [`NugetVersionData`].
///
/// Mirrors `lib/modules/versioning/nuget/parser.ts` `parseVersion()`.
pub fn parse_version(input: &str) -> Option<NugetVersionData> {
    static VERSION_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"^(?P<major>\d+)(?:\s*\.\s*(?P<minor>\d+)(?:\s*\.\s*(?P<patch>\d+)(?:\s*\.\s*(?P<revision>\d+))?)?)?\s*(?:-(?P<prerelease>[-a-zA-Z0-9]+(?:\.[-a-zA-Z0-9]+)*))?(?:\+(?P<metadata>[-a-zA-Z0-9]+(?:\.[-a-zA-Z0-9]+)*))?$",
        ).unwrap()
    });
    let cap = VERSION_RE.captures(input.trim())?;
    let major: u64 = cap.name("major")?.as_str().parse().ok()?;
    let minor = cap.name("minor").and_then(|m| m.as_str().parse().ok());
    let patch = cap.name("patch").and_then(|m| m.as_str().parse().ok());
    let revision = cap.name("revision").and_then(|m| m.as_str().parse().ok());
    let prerelease = cap.name("prerelease").map(|m| m.as_str().to_owned());
    let metadata = cap.name("metadata").map(|m| m.as_str().to_owned());
    Some(NugetVersionData { major, minor, patch, revision, prerelease, metadata })
}

fn parse_floating_component(input: &str) -> u64 {
    let int_str = input.split('*').next().unwrap_or("");
    if int_str.is_empty() { 0 } else { int_str.parse::<u64>().unwrap_or(0) * 10 }
}

/// Parse a NuGet floating range string (e.g. `1.2.*`) into [`NugetFloatingRangeData`].
///
/// Mirrors `lib/modules/versioning/nuget/parser.ts` `parseFloatingRange()`.
pub fn parse_floating_range(input: &str) -> Option<NugetFloatingRangeData> {
    static FLOATING_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"^(?:(?:(?P<floating_major>\d*\*)|(?P<major>\d+)(?:\.(?:(?P<floating_minor>\d*\*)|(?P<minor>\d+)(?:\.(?:(?P<floating_patch>\d*\*)|(?P<patch>\d+)(?:\.(?:(?P<floating_revision>\d*\*)|(?P<revision>\d+)))?))?))?)(?:-(?P<floating_prerelease>\*|[-a-zA-Z0-9]+(?:\.[-a-zA-Z0-9]+)*\.?\*))?)$",
        ).unwrap()
    });
    let cap = FLOATING_RE.captures(input)?;
    let prerelease = cap.name("floating_prerelease").map(|m| m.as_str().to_owned());
    let mut res = NugetFloatingRangeData {
        major: 0,
        minor: None,
        patch: None,
        revision: None,
        floating: None,
        prerelease: prerelease.clone(),
    };
    if let Some(fm) = cap.name("floating_major") {
        res.major = parse_floating_component(fm.as_str());
        res.floating = Some(NugetFloatingField::Major);
        return Some(res);
    }
    if let Some(m) = cap.name("major") {
        res.major = m.as_str().parse().unwrap_or(0);
    }
    if let Some(fm) = cap.name("floating_minor") {
        res.minor = Some(parse_floating_component(fm.as_str()));
        res.floating = Some(NugetFloatingField::Minor);
        return Some(res);
    }
    if let Some(m) = cap.name("minor") {
        res.minor = Some(m.as_str().parse().unwrap_or(0));
    }
    if let Some(fp) = cap.name("floating_patch") {
        res.patch = Some(parse_floating_component(fp.as_str()));
        res.floating = Some(NugetFloatingField::Patch);
        return Some(res);
    }
    if let Some(p) = cap.name("patch") {
        res.patch = Some(p.as_str().parse().unwrap_or(0));
    }
    if let Some(fr) = cap.name("floating_revision") {
        res.revision = Some(parse_floating_component(fr.as_str()));
        res.floating = Some(NugetFloatingField::Revision);
        return Some(res);
    }
    if let Some(r) = cap.name("revision") {
        res.revision = Some(r.as_str().parse().unwrap_or(0));
    }
    if prerelease.is_some() { Some(res) } else { None }
}

/// Compute the lower bound [`NugetVersionData`] for a floating range.
///
/// Mirrors `lib/modules/versioning/nuget/range.ts` `getFloatingRangeLowerBound()`.
pub fn get_floating_range_lower_bound(range: &NugetFloatingRangeData) -> NugetVersionData {
    let prerelease = range.prerelease.as_ref().map(|pr| {
        let mut parts: Vec<String> = pr.split('.').map(str::to_owned).collect();
        let last_idx = parts.len() - 1;
        if parts[last_idx] == "*" {
            parts[last_idx] = "0".to_owned();
        } else {
            parts[last_idx] = parts[last_idx].trim_end_matches('*').to_owned();
        }
        parts.join(".")
    });
    NugetVersionData {
        major: range.major,
        minor: Some(range.minor.unwrap_or(0)),
        patch: Some(range.patch.unwrap_or(0)),
        revision: Some(range.revision.unwrap_or(0)),
        prerelease,
        metadata: None,
    }
}

/// Parse an exact NuGet range (e.g. `[1.2.3]`) into [`NugetExactRangeData`].
///
/// Mirrors `lib/modules/versioning/nuget/parser.ts` `parseExactRange()`.
pub fn parse_exact_range(input: &str) -> Option<NugetExactRangeData> {
    static EXACT_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^\s*\[\s*(?P<version>[^,]+?)\s*\]\s*$").unwrap()
    });
    let version_str = EXACT_RE.captures(input)?.name("version")?.as_str();
    let version = parse_version(version_str)?;
    Some(NugetExactRangeData { version })
}

/// Parse a NuGet bracket range (e.g. `[1.0,2.0)`) into [`NugetBracketRangeData`].
///
/// Mirrors `lib/modules/versioning/nuget/parser.ts` `parseBracketRange()`.
pub fn parse_bracket_range(input: &str) -> Option<NugetBracketRangeData> {
    static MAX_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^\s*(?P<left_bracket>\(|\[)\s*,\s*(?P<max_version>[^\s\])]+)\s*(?P<right_bracket>\)|\])\s*$").unwrap()
    });
    static MIN_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^\s*(?P<left_bracket>\(|\[)\s*(?P<min_version>[^\s,]+)\s*,\s*(?P<right_bracket>\)|\])\s*$").unwrap()
    });
    static BOTH_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^\s*(?P<left_bracket>\(|\[)\s*(?P<min_version>[^\s,]+)\s*,\s*(?P<max_version>[^\s\])]+)\s*(?P<right_bracket>\)|\])\s*$").unwrap()
    });
    if let Some(cap) = MAX_RE.captures(input) {
        let max = parse_version(cap.name("max_version")?.as_str())?;
        return Some(NugetBracketRangeData {
            min: None,
            max: Some(max),
            min_inclusive: cap.name("left_bracket")?.as_str() == "[",
            max_inclusive: cap.name("right_bracket")?.as_str() == "]",
        });
    }
    if let Some(cap) = MIN_RE.captures(input) {
        let min_str = cap.name("min_version")?.as_str();
        let min = parse_version(min_str).map(NugetBracketMin::Version)
            .or_else(|| parse_floating_range(min_str).map(NugetBracketMin::Floating))?;
        return Some(NugetBracketRangeData {
            min: Some(min),
            max: None,
            min_inclusive: cap.name("left_bracket")?.as_str() == "[",
            max_inclusive: cap.name("right_bracket")?.as_str() == "]",
        });
    }
    if let Some(cap) = BOTH_RE.captures(input) {
        let min_str = cap.name("min_version")?.as_str();
        let min = parse_version(min_str).map(NugetBracketMin::Version)
            .or_else(|| parse_floating_range(min_str).map(NugetBracketMin::Floating))?;
        let max = parse_version(cap.name("max_version")?.as_str())?;
        return Some(NugetBracketRangeData {
            min: Some(min),
            max: Some(max),
            min_inclusive: cap.name("left_bracket")?.as_str() == "[",
            max_inclusive: cap.name("right_bracket")?.as_str() == "]",
        });
    }
    None
}

/// Parse any NuGet range expression into [`NugetRangeData`].
///
/// Mirrors `lib/modules/versioning/nuget/parser.ts` `parseRange()`.
pub fn parse_range(input: &str) -> Option<NugetRangeData> {
    parse_exact_range(input).map(NugetRangeData::Exact)
        .or_else(|| parse_bracket_range(input).map(NugetRangeData::Bracket))
        .or_else(|| parse_floating_range(input).map(NugetRangeData::Floating))
}

/// Render a [`NugetVersionData`] back to its canonical string form.
///
/// Mirrors `lib/modules/versioning/nuget/version.ts` `versionToString()`.
pub fn version_to_string(v: &NugetVersionData) -> String {
    let mut s = v.major.to_string();
    if let Some(n) = v.minor { s.push('.'); s.push_str(&n.to_string()); }
    if let Some(n) = v.patch { s.push('.'); s.push_str(&n.to_string()); }
    if let Some(n) = v.revision { s.push('.'); s.push_str(&n.to_string()); }
    if let Some(ref pr) = v.prerelease { s.push('-'); s.push_str(pr); }
    if let Some(ref md) = v.metadata { s.push('+'); s.push_str(md); }
    s
}

fn floating_component_to_string(component: u64) -> String {
    let int = component / 10;
    if int == 0 { "*".to_owned() } else { format!("{int}*") }
}

/// Render a [`NugetRangeData`] back to its canonical string form.
///
/// Mirrors `lib/modules/versioning/nuget/range.ts` `rangeToString()`.
pub fn range_to_string(range: &NugetRangeData) -> String {
    match range {
        NugetRangeData::Exact(r) => format!("[{}]", version_to_string(&r.version)),
        NugetRangeData::Floating(r) => {
            let mut s = String::new();
            if let Some(ref pr) = r.prerelease { s = format!("-{pr}"); }
            if let Some(rev) = r.revision {
                let part = if r.floating == Some(NugetFloatingField::Revision) {
                    floating_component_to_string(rev)
                } else { rev.to_string() };
                s = format!(".{part}{s}");
            }
            if let Some(pat) = r.patch {
                let part = if r.floating == Some(NugetFloatingField::Patch) {
                    floating_component_to_string(pat)
                } else { pat.to_string() };
                s = format!(".{part}{s}");
            }
            if let Some(min) = r.minor {
                let part = if r.floating == Some(NugetFloatingField::Minor) {
                    floating_component_to_string(min)
                } else { min.to_string() };
                s = format!(".{part}{s}");
            }
            let maj = if r.floating == Some(NugetFloatingField::Major) {
                floating_component_to_string(r.major)
            } else { r.major.to_string() };
            format!("{maj}{s}")
        }
        NugetRangeData::Bracket(r) => {
            let l = if r.min_inclusive { '[' } else { '(' };
            let rr = if r.max_inclusive { ']' } else { ')' };
            let min_str = |m: &NugetBracketMin| match m {
                NugetBracketMin::Version(v) => version_to_string(v),
                NugetBracketMin::Floating(f) => range_to_string(&NugetRangeData::Floating(f.clone())),
            };
            match (&r.min, &r.max) {
                (Some(mn), Some(mx)) => format!("{l}{},{}{rr}", min_str(mn), version_to_string(mx)),
                (Some(mn), None) => format!("{l}{},{rr}", min_str(mn)),
                (None, Some(mx)) => format!("{l},{}{rr}", version_to_string(mx)),
                (None, None) => format!("{l},{rr}"),
            }
        }
    }
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

    // Ported: "sortNugetVersions(\"$version\", \"$other\") === $result" — datasource/nuget/common.spec.ts line 4
    #[test]
    fn sort_nuget_versions_matches_renovate_spec() {
        assert_eq!(sort_nuget_versions("invalid1", "invalid2"), Ordering::Equal);
        assert_eq!(sort_nuget_versions("invalid", "1.0.0"), Ordering::Less);
        assert_eq!(sort_nuget_versions("1.0.0", "invalid"), Ordering::Greater);
        assert_eq!(sort_nuget_versions("1.0.0-rc.1", "1.0.0"), Ordering::Less);
        assert_eq!(
            sort_nuget_versions("1.0.0", "1.0.0-rc.1"),
            Ordering::Greater
        );
        assert_eq!(sort_nuget_versions("1.0.0", "1.0.0"), Ordering::Equal);
    }

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

    // ── parse_version ─────────────────────────────────────────────────────────

    // Ported: "returns null for invalid input" — versioning/nuget/parser.spec.ts line 13
    #[test]
    fn parse_version_rejects_invalid_input() {
        assert_eq!(parse_version("!@#"), None);
        assert_eq!(parse_version("abc"), None);
    }

    // Ported: "parses version" — versioning/nuget/parser.spec.ts line 18
    #[test]
    fn parse_version_parses_full_version() {
        assert_eq!(
            parse_version("1.2.3.4-foo+bar"),
            Some(NugetVersionData {
                major: 1,
                minor: Some(2),
                patch: Some(3),
                revision: Some(4),
                prerelease: Some("foo".to_owned()),
                metadata: Some("bar".to_owned()),
            })
        );
    }

    // ── parse_floating_range ──────────────────────────────────────────────────

    // Ported: "rejects invalid input" — versioning/nuget/parser.spec.ts line 32
    #[test]
    fn parse_floating_range_rejects_invalid_input() {
        assert_eq!(parse_floating_range("!@#"), None);
        assert_eq!(parse_floating_range("abc"), None);
        assert_eq!(parse_floating_range("1.2.*-foo"), None);
        assert_eq!(parse_floating_range("1.2.3"), None);
    }

    // Ported: "$input" — versioning/nuget/parser.spec.ts line 39
    #[test]
    fn parse_floating_range_parametrized() {
        macro_rules! fr {
            ($major:expr) => {
                NugetFloatingRangeData { major: $major, minor: None, patch: None, revision: None, floating: None, prerelease: None }
            };
            ($major:expr, floating=$f:expr) => {
                NugetFloatingRangeData { major: $major, minor: None, patch: None, revision: None, floating: Some($f), prerelease: None }
            };
            ($major:expr, floating=$f:expr, pre=$p:expr) => {
                NugetFloatingRangeData { major: $major, minor: None, patch: None, revision: None, floating: Some($f), prerelease: Some($p.to_owned()) }
            };
            ($major:expr, minor=$minor:expr, floating=$f:expr) => {
                NugetFloatingRangeData { major: $major, minor: Some($minor), patch: None, revision: None, floating: Some($f), prerelease: None }
            };
            ($major:expr, minor=$minor:expr, floating=$f:expr, pre=$p:expr) => {
                NugetFloatingRangeData { major: $major, minor: Some($minor), patch: None, revision: None, floating: Some($f), prerelease: Some($p.to_owned()) }
            };
            ($major:expr, minor=$minor:expr, patch=$patch:expr, floating=$f:expr) => {
                NugetFloatingRangeData { major: $major, minor: Some($minor), patch: Some($patch), revision: None, floating: Some($f), prerelease: None }
            };
            ($major:expr, minor=$minor:expr, patch=$patch:expr, floating=$f:expr, pre=$p:expr) => {
                NugetFloatingRangeData { major: $major, minor: Some($minor), patch: Some($patch), revision: None, floating: Some($f), prerelease: Some($p.to_owned()) }
            };
            ($major:expr, minor=$minor:expr, patch=$patch:expr, rev=$rev:expr, floating=$f:expr) => {
                NugetFloatingRangeData { major: $major, minor: Some($minor), patch: Some($patch), revision: Some($rev), floating: Some($f), prerelease: None }
            };
            ($major:expr, minor=$minor:expr, patch=$patch:expr, rev=$rev:expr, floating=$f:expr, pre=$p:expr) => {
                NugetFloatingRangeData { major: $major, minor: Some($minor), patch: Some($patch), revision: Some($rev), floating: Some($f), prerelease: Some($p.to_owned()) }
            };
            ($major:expr, minor=$minor:expr, patch=$patch:expr, rev=$rev:expr, pre=$p:expr) => {
                NugetFloatingRangeData { major: $major, minor: Some($minor), patch: Some($patch), revision: Some($rev), floating: None, prerelease: Some($p.to_owned()) }
            };
            ($major:expr, minor=$minor:expr, pre=$p:expr) => {
                NugetFloatingRangeData { major: $major, minor: Some($minor), patch: None, revision: None, floating: None, prerelease: Some($p.to_owned()) }
            };
            ($major:expr, minor=$minor:expr, patch=$patch:expr, pre=$p:expr) => {
                NugetFloatingRangeData { major: $major, minor: Some($minor), patch: Some($patch), revision: None, floating: None, prerelease: Some($p.to_owned()) }
            };
            ($major:expr, pre=$p:expr) => {
                NugetFloatingRangeData { major: $major, minor: None, patch: None, revision: None, floating: None, prerelease: Some($p.to_owned()) }
            };
        }
        use NugetFloatingField::*;
        let cases: &[(&str, NugetFloatingRangeData)] = &[
            ("*-*",          fr!(0, floating=Major, pre="*")),
            ("*-foo*",       fr!(0, floating=Major, pre="foo*")),
            ("*-foo.bar*",   fr!(0, floating=Major, pre="foo.bar*")),
            ("*",            fr!(0, floating=Major)),
            ("1.*",          fr!(1, minor=0, floating=Minor)),
            ("1.*-*",        fr!(1, minor=0, floating=Minor, pre="*")),
            ("1.*-foo*",     fr!(1, minor=0, floating=Minor, pre="foo*")),
            ("1.2.*",        fr!(1, minor=2, patch=0, floating=Patch)),
            ("1.2.*-*",      fr!(1, minor=2, patch=0, floating=Patch, pre="*")),
            ("1.2.*-foo*",   fr!(1, minor=2, patch=0, floating=Patch, pre="foo*")),
            ("1.2.3.*",      fr!(1, minor=2, patch=3, rev=0, floating=Revision)),
            ("1.2.3.*-*",    fr!(1, minor=2, patch=3, rev=0, floating=Revision, pre="*")),
            ("1.2.3.*-foo*", fr!(1, minor=2, patch=3, rev=0, floating=Revision, pre="foo*")),
            ("1.2.3.4-*",    fr!(1, minor=2, patch=3, rev=4, pre="*")),
            ("1.2.3.4-foo*", fr!(1, minor=2, patch=3, rev=4, pre="foo*")),
            ("123*",         fr!(1230, floating=Major)),
            ("1-*",          fr!(1, pre="*")),
            ("1.2-*",        fr!(1, minor=2, pre="*")),
            ("1.2.3-*",      fr!(1, minor=2, patch=3, pre="*")),
        ];
        for (input, expected) in cases {
            assert_eq!(parse_floating_range(input).as_ref(), Some(expected), "parse_floating_range({input})");
        }
    }

    // ── get_floating_range_lower_bound ────────────────────────────────────────

    // Ported: "$input" — versioning/nuget/parser.spec.ts line 78
    #[test]
    fn get_floating_range_lower_bound_parametrized() {
        macro_rules! vd {
            ($major:expr, $minor:expr, $patch:expr, $rev:expr) => {
                NugetVersionData { major: $major, minor: Some($minor), patch: Some($patch), revision: Some($rev), prerelease: None, metadata: None }
            };
            ($major:expr, $minor:expr, $patch:expr, $rev:expr, pre=$p:expr) => {
                NugetVersionData { major: $major, minor: Some($minor), patch: Some($patch), revision: Some($rev), prerelease: Some($p.to_owned()), metadata: None }
            };
        }
        let cases: &[(&str, NugetVersionData)] = &[
            ("*-*",          vd!(0, 0, 0, 0, pre="0")),
            ("*-foo*",       vd!(0, 0, 0, 0, pre="foo")),
            ("*-foo.bar*",   vd!(0, 0, 0, 0, pre="foo.bar")),
            ("*",            vd!(0, 0, 0, 0)),
            ("1.*",          vd!(1, 0, 0, 0)),
            ("1.*-*",        vd!(1, 0, 0, 0, pre="0")),
            ("1.*-foo*",     vd!(1, 0, 0, 0, pre="foo")),
            ("1.2.*",        vd!(1, 2, 0, 0)),
            ("1.2.*-*",      vd!(1, 2, 0, 0, pre="0")),
            ("1.2.*-foo*",   vd!(1, 2, 0, 0, pre="foo")),
            ("1.2.3.*",      vd!(1, 2, 3, 0)),
            ("1.2.3.*-*",    vd!(1, 2, 3, 0, pre="0")),
            ("1.2.3.*-foo*", vd!(1, 2, 3, 0, pre="foo")),
            ("1.2.3.4-*",    vd!(1, 2, 3, 4, pre="0")),
            ("1.2.3.4-foo*", vd!(1, 2, 3, 4, pre="foo")),
            ("1234*",        vd!(12340, 0, 0, 0)),
            ("1.234*",       vd!(1, 2340, 0, 0)),
            ("1.2.34*",      vd!(1, 2, 340, 0)),
            ("1.2.3.4*",     vd!(1, 2, 3, 40)),
            ("1.2.3-4.5.*",  vd!(1, 2, 3, 0, pre="4.5.0")),
        ];
        for (input, expected) in cases {
            let range = parse_floating_range(input).unwrap_or_else(|| panic!("parse_floating_range({input}) returned None"));
            assert_eq!(get_floating_range_lower_bound(&range), *expected, "lower_bound({input})");
        }
    }

    // ── parse_exact_range ─────────────────────────────────────────────────────

    // Ported: "rejects invalid input" — versioning/nuget/parser.spec.ts line 115
    #[test]
    fn parse_exact_range_rejects_invalid_input() {
        assert_eq!(parse_exact_range("!@#"), None);
        assert_eq!(parse_exact_range("abc"), None);
        assert_eq!(parse_exact_range("1.2.*"), None);
        assert_eq!(parse_exact_range("[1.2.*]"), None);
        assert_eq!(parse_exact_range("[foobar]"), None);
    }

    // Ported: "parses exact range" — versioning/nuget/parser.spec.ts line 123
    #[test]
    fn parse_exact_range_parses() {
        assert_eq!(
            parse_exact_range("[1.2.3]"),
            Some(NugetExactRangeData {
                version: NugetVersionData {
                    major: 1, minor: Some(2), patch: Some(3),
                    revision: None, prerelease: None, metadata: None,
                }
            })
        );
    }

    // ── parse_bracket_range ───────────────────────────────────────────────────

    // Ported: "rejects invalid input" — versioning/nuget/parser.spec.ts line 137
    #[test]
    fn parse_bracket_range_rejects_invalid_input() {
        assert_eq!(parse_bracket_range("!@#"), None);
        assert_eq!(parse_bracket_range("abc"), None);
        assert_eq!(parse_bracket_range("[1.2.*"), None);
        assert_eq!(parse_bracket_range("[foo,)"), None);
        assert_eq!(parse_bracket_range("[,bar]"), None);
        assert_eq!(parse_bracket_range("[foo,bar]"), None);
        assert_eq!(parse_bracket_range("[1.2.3,bar]"), None);
    }

    // Ported: "parses range without lower bound" — versioning/nuget/parser.spec.ts line 147
    #[test]
    fn parse_bracket_range_no_lower_bound() {
        assert_eq!(
            parse_bracket_range("(,1.2.3]"),
            Some(NugetBracketRangeData {
                min: None,
                max: Some(NugetVersionData { major: 1, minor: Some(2), patch: Some(3), revision: None, prerelease: None, metadata: None }),
                min_inclusive: false,
                max_inclusive: true,
            })
        );
    }

    // Ported: "parses range without upper bound" — versioning/nuget/parser.spec.ts line 157
    #[test]
    fn parse_bracket_range_no_upper_bound() {
        assert_eq!(
            parse_bracket_range("[1.2.3,)"),
            Some(NugetBracketRangeData {
                min: Some(NugetBracketMin::Version(NugetVersionData { major: 1, minor: Some(2), patch: Some(3), revision: None, prerelease: None, metadata: None })),
                max: None,
                min_inclusive: true,
                max_inclusive: false,
            })
        );
    }

    // Ported: "$input" — versioning/nuget/parser.spec.ts line 168
    #[test]
    fn parse_bracket_range_bounds_inclusivity() {
        let v1 = || NugetVersionData { major: 1, minor: None, patch: None, revision: None, prerelease: None, metadata: None };
        let v2 = || NugetVersionData { major: 2, minor: None, patch: None, revision: None, prerelease: None, metadata: None };
        let cases = [
            ("(1,2)", false, false),
            ("[1,2)", true, false),
            ("(1,2]", false, true),
            ("[1,2]", true, true),
        ];
        for (input, min_incl, max_incl) in cases {
            assert_eq!(
                parse_bracket_range(input),
                Some(NugetBracketRangeData {
                    min: Some(NugetBracketMin::Version(v1())),
                    max: Some(v2()),
                    min_inclusive: min_incl,
                    max_inclusive: max_incl,
                }),
                "parse_bracket_range({input})"
            );
        }
    }

    // Ported: "handles whitespaces" — versioning/nuget/parser.spec.ts line 185
    #[test]
    fn parse_bracket_range_handles_whitespace() {
        assert_eq!(
            parse_bracket_range(" [ 1 , 2 ] "),
            Some(NugetBracketRangeData {
                min: Some(NugetBracketMin::Version(NugetVersionData { major: 1, minor: None, patch: None, revision: None, prerelease: None, metadata: None })),
                max: Some(NugetVersionData { major: 2, minor: None, patch: None, revision: None, prerelease: None, metadata: None }),
                min_inclusive: true,
                max_inclusive: true,
            })
        );
    }

    // Ported: "handles floating ranges as lower bounds" — versioning/nuget/parser.spec.ts line 195
    #[test]
    fn parse_bracket_range_floating_lower_bound() {
        let float_1_minor = NugetFloatingRangeData {
            major: 1, minor: Some(0), patch: None, revision: None,
            floating: Some(NugetFloatingField::Minor), prerelease: None,
        };
        assert_eq!(
            parse_bracket_range("[1.*,2]"),
            Some(NugetBracketRangeData {
                min: Some(NugetBracketMin::Floating(float_1_minor.clone())),
                max: Some(NugetVersionData { major: 2, minor: None, patch: None, revision: None, prerelease: None, metadata: None }),
                min_inclusive: true,
                max_inclusive: true,
            })
        );
        assert_eq!(
            parse_bracket_range("[1.*,)"),
            Some(NugetBracketRangeData {
                min: Some(NugetBracketMin::Floating(float_1_minor)),
                max: None,
                min_inclusive: true,
                max_inclusive: false,
            })
        );
    }

    // ── version_to_string ─────────────────────────────────────────────────────

    // Ported: "$version" — versioning/nuget/parser.spec.ts line 224
    #[test]
    fn version_to_string_roundtrip() {
        let cases = [
            "1", "1.2", "1.2.3", "1.2.3.4",
            "1-beta", "1.2-beta", "1.2.3-beta", "1.2.3.4-beta",
            "1.2.3.4-beta+ABC",
        ];
        for v in cases {
            let parsed = parse_version(v).unwrap_or_else(|| panic!("parse_version({v}) returned None"));
            assert_eq!(version_to_string(&parsed), v, "version_to_string(parse_version({v}))");
        }
    }

    // ── range_to_string ───────────────────────────────────────────────────────

    // Ported: "$version" — versioning/nuget/parser.spec.ts line 242
    #[test]
    fn range_to_string_roundtrip() {
        let cases = [
            "[1]", "[1.2]", "[1.2.3]", "[1.2.3.4]",
            "[1-foo]", "[1.2-bar]", "[1.2.3-baz]", "[1.2.3.4-qux]",
            "*", "1.*", "1.2.*", "1.2.3.*",
            "1.2.3.4-*", "1.2.3.*-*", "1.2.*-*", "1.*-*", "*-*",
            "1234*", "1.234*", "1.2.34*", "1.2.3.4*",
            "(1,2)", "[1,2)", "(1,2]", "[1,2]",
            "(*,2)", "[*,2)", "(*,2]", "[*,2]",
            "(1,)", "(1,]", "[1,]", "[1,)",
            "(*,)", "(*,]", "[*,]", "[*,)",
            "(,1)", "(,1]", "[,1]", "[,1)",
        ];
        for v in cases {
            let parsed = parse_range(v).unwrap_or_else(|| panic!("parse_range({v}) returned None"));
            assert_eq!(range_to_string(&parsed), v, "range_to_string(parse_range({v}))");
        }
    }
}
