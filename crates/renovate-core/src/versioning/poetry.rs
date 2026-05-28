//! Poetry versioning — mirrors `lib/modules/versioning/poetry/index.ts`.
//!
//! Poetry versions are a union of SemVer and PEP 440, normalized to SemVer
//! internally. Ranges are expressed as comma-separated comparators and
//! translated to npm-style ranges for matching operations.

use std::sync::OnceLock;

use regex::Regex;
use semver::Version;

// ---------------------------------------------------------------------------
// Patterns
// ---------------------------------------------------------------------------

/// Matches poetry/pep440 version strings.
///
/// Named groups: epoch, release, pre_l, pre_n, post_n1, post_l, post_n2, dev_l, dev_n, local.
fn version_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?x)
            ^
            v?
            (?:
              (?:(?P<epoch>[0-9]+)!)?
              (?P<release>[0-9]+(?:\.[0-9]+){0,2})
              (?P<pre>
                [-_.]?
                (?P<pre_l>a|b|c|rc|alpha|beta|pre|preview)
                [-_.]?
                (?P<pre_n>[0-9]+)?
              )?
              (?P<post>
                (?:-(?P<post_n1>[0-9]+))
                |
                (?:
                  [-_.]?
                  (?P<post_l>post|rev|r)
                  [-_.]?
                  (?P<post_n2>[0-9]+)?
                )
              )?
              (?P<dev>
                [-_.]?
                (?P<dev_l>dev)
                [-_.]?
                (?P<dev_n>[0-9]+)?
              )?
            )
            (?:\+(?P<local>[a-z0-9]+(?:[-_.][a-z0-9]+)*))?
            $",
        )
        .expect("poetry version_pattern")
    })
}

/// Captures range comparator operators so we can split/transform range strings.
fn range_comparator_pattern() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(\s*(?:\^|~|[><!]?=|[><]|\|\|)\s*)").expect("range_comparator_pattern")
    })
}

// ---------------------------------------------------------------------------
// Version transformations
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct LetterTag {
    letter: String,
    number: String,
}

fn parse_letter_tag(letter: Option<&str>, number: Option<&str>) -> Option<LetterTag> {
    if let Some(l) = letter {
        let spellings: &[(&str, &str)] = &[
            ("alpha", "a"),
            ("beta", "b"),
            ("c", "rc"),
            ("pre", "rc"),
            ("preview", "rc"),
            ("r", "post"),
            ("rev", "post"),
        ];
        let normalized = spellings
            .iter()
            .find(|(k, _)| *k == l)
            .map_or(l, |(_, v)| v);
        Some(LetterTag {
            letter: normalized.to_owned(),
            number: number.unwrap_or("0").to_owned(),
        })
    } else if number.is_some() {
        Some(LetterTag {
            letter: "post".to_owned(),
            number: number.unwrap_or("0").to_owned(),
        })
    } else {
        None
    }
}

fn strip_leading_zeros(s: &str) -> String {
    // Trim leading zeros but keep at least one digit: "01" → "1", "0" → "0"
    let trimmed = s.trim_start_matches('0');
    if trimmed.is_empty() { "0".to_owned() } else { trimmed.to_owned() }
}

/// Convert a poetry/pep440 version string to its semver representation.
///
/// `pad_release = true` pads the release segment to 3 components (X.Y.Z).
/// Returns `None` for strings that don't match the version pattern.
///
/// Mirrors `poetry2semver` from `lib/modules/versioning/poetry/transform.ts`.
pub fn poetry2semver(poetry_version: &str, pad_release: bool) -> Option<String> {
    let caps = version_pattern().captures(poetry_version.trim())?;

    let release_str = caps.name("release").map_or("0", |m| m.as_str());
    let mut release_parts: Vec<u64> = release_str
        .split('.')
        .map(|s| s.parse::<u64>().unwrap_or(0))
        .collect();

    // Node-semver normalizes to 3 components regardless of padRelease, so we
    // always pad here to produce valid semver output.
    let _ = pad_release;
    while release_parts.len() < 3 {
        release_parts.push(0);
    }

    let pre = parse_letter_tag(
        caps.name("pre_l").map(|m| m.as_str()),
        caps.name("pre_n").map(|m| m.as_str()),
    );
    // post_n1 is the numeric form (1.9-0); post_l/post_n2 is the labeled form
    // The TypeScript accesses `post_n` (undefined because the group is `post_n2`)
    // so labeled post always gets number "0".
    let post = if let Some(post_n1) = caps.name("post_n1") {
        parse_letter_tag(None, Some(post_n1.as_str()))
    } else {
        parse_letter_tag(
            caps.name("post_l").map(|m| m.as_str()),
            None, // TypeScript bug: uses `post_n` which is undefined (group is post_n2)
        )
    };
    let dev = parse_letter_tag(
        caps.name("dev_l").map(|m| m.as_str()),
        caps.name("dev_n").map(|m| m.as_str()),
    );

    let release_seg = release_parts
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(".");

    let mut result = release_seg;

    if let Some(mut tag) = pre {
        tag.number = strip_leading_zeros(&tag.number);
        result.push_str(&format!("-{}.{}", tag.letter, tag.number));
    }
    if let Some(mut tag) = post {
        tag.number = strip_leading_zeros(&tag.number);
        result.push_str(&format!("-{}.{}", tag.letter, tag.number));
    }
    if let Some(mut tag) = dev {
        tag.number = strip_leading_zeros(&tag.number);
        result.push_str(&format!("-{}.{}", tag.letter, tag.number));
    }

    // Validate with semver crate
    Version::parse(&result).ok()?;
    Some(result)
}

/// Convert a semver version back to poetry format.
///
/// Mirrors `semver2poetry` from `lib/modules/versioning/poetry/transform.ts`.
pub fn semver2poetry(version: &str) -> Option<String> {
    let v = Version::parse(version.trim()).ok()?;

    // Normalize prerelease spellings back to poetry/pep440
    let spellings: &[(&str, &str)] = &[("a", "alpha"), ("b", "beta"), ("c", "rc"), ("dev", "alpha")];

    let pre = if v.pre.is_empty() {
        String::new()
    } else {
        let pre_str = v.pre.as_str();
        // pre might be "alpha.1", "beta.0", "rc.1", "post.0", "dev.0", etc.
        let normalized = spellings
            .iter()
            .fold(pre_str.to_owned(), |acc, (from, to)| {
                // Replace leading component if it matches
                if let Some(rest) = acc.strip_prefix(from) {
                    if rest.is_empty() || rest.starts_with('.') {
                        format!("{to}{rest}")
                    } else {
                        acc
                    }
                } else {
                    acc
                }
            });
        format!("-{normalized}")
    };

    Some(format!("{}.{}.{}{}", v.major, v.minor, v.patch, pre))
}

/// Convert a poetry range string to a format accepted by the Rust `semver` crate.
///
/// Mirrors `poetry2npm` from `lib/modules/versioning/poetry/transform.ts`.
/// Returns `None` only when the range contains unsupported patterns like `!=`.
///
/// The Rust `semver` crate uses commas for AND conditions (unlike npm which uses
/// spaces), so we produce comma-separated output.
pub fn poetry2npm(input: &str) -> Option<String> {
    let input = input.trim();

    // Pass through wildcards unchanged
    if input == "*" {
        return Some("*".to_owned());
    }

    // Handle || (OR) ranges by processing each alternative separately
    if input.contains("||") {
        let alternatives: Vec<&str> = input.split("||").collect();
        let converted: Option<Vec<String>> = alternatives
            .iter()
            .map(|alt| convert_poetry_clause(alt.trim()))
            .collect();
        let converted = converted?;
        return Some(converted.join(" || "));
    }

    // Split on commas (poetry AND separator) and convert each clause
    let clauses: Vec<&str> = input
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();

    // Also handle space-separated AND conditions (no commas)
    // by treating the whole input as one range
    if clauses.len() == 1 {
        return convert_poetry_clause(clauses[0]);
    }

    let mut result_parts: Vec<String> = Vec::new();
    for clause in &clauses {
        let converted = convert_poetry_clause(clause)?;
        result_parts.push(converted);
    }

    let result = result_parts.join(", ");
    let result = result.replace("===", "=");

    if result.split_whitespace().any(|p| p.starts_with("!=")) {
        return None;
    }

    Some(result)
}

/// Convert a single poetry clause (no commas, possibly space-separated comparators).
///
/// Produces a Rust-semver-compatible string using commas for AND conditions.
fn convert_poetry_clause(clause: &str) -> Option<String> {
    let clause = clause.trim();
    if clause.is_empty() {
        return Some(String::new());
    }

    let parts: Vec<&str> = range_comparator_pattern().split(clause).collect();
    let separators: Vec<&str> = range_comparator_pattern()
        .find_iter(clause)
        .map(|m| m.as_str())
        .collect();

    let mut result = String::new();
    let mut sep_iter = separators.iter().peekable();

    for (i, part) in parts.iter().enumerate() {
        let trimmed = part.trim();
        let converted = if trimmed.is_empty() {
            String::new()
        } else {
            let semver_v = poetry2semver(trimmed, false).unwrap_or_else(|| trimmed.to_owned());
            // parts[0] is a bare version (no preceding operator) — treat as exact pin
            // by prepending `=` so Rust semver doesn't apply caret semantics.
            if i == 0 {
                format!("={semver_v}")
            } else {
                semver_v
            }
        };

        // When the result currently ends with a version (digit/dot), and we're
        // about to add another version (after an operator that was already appended),
        // we need to insert a comma to separate AND conditions.
        if !converted.is_empty() && !converted.starts_with('=') {
            let last_non_ws = result.trim_end().chars().last();
            let ends_with_version = last_non_ws.is_some_and(|c| c.is_ascii_digit() || c == '.');
            if ends_with_version {
                result.push_str(", ");
            }
        }

        result.push_str(&converted);

        // Append the operator separator; insert comma before it if result ends with a version
        if let Some(sep) = sep_iter.next() {
            let op = sep.trim();
            let last_non_ws = result.trim_end().chars().last();
            let ends_with_version = last_non_ws.is_some_and(|c| c.is_ascii_digit() || c == '.');
            if ends_with_version && !op.is_empty() {
                result.push_str(", ");
            }
            result.push_str(op);
            // Add space after operator if a version follows
            if i + 1 < parts.len() && !parts[i + 1].trim().is_empty() {
                result.push(' ');
            }
        }
    }

    // Reject unsupported
    if result.split_whitespace().any(|p| p.starts_with("!=")) {
        return None;
    }

    Some(result)
}

/// Convert an npm range string back to poetry format.
///
/// Mirrors `npm2poetry` from `lib/modules/versioning/poetry/transform.ts`.
pub fn npm2poetry(range: &str) -> String {
    let parts: Vec<&str> = range_comparator_pattern().split(range).collect();
    let separators: Vec<&str> = range_comparator_pattern()
        .find_iter(range)
        .map(|m| m.as_str())
        .collect();

    let mut transformed = String::new();
    let mut sep_iter = separators.iter();

    for part in &parts {
        let converted = semver2poetry(part).unwrap_or_else(|| part.to_string());
        transformed.push_str(&converted);
        if let Some(sep) = sep_iter.next() {
            transformed.push_str(sep);
        }
    }

    // Split by whitespace, rejoin with commas, handle || separators
    let res: Vec<String> = transformed
        .split_whitespace()
        .map(str::to_owned)
        .collect();

    let operators = ["^", "~", "=", ">", "<", "<=", ">="];
    let mut merged: Vec<String> = Vec::new();
    let mut i = 0;
    while i < res.len() {
        if operators.contains(&res[i].as_str()) && i + 1 < res.len() {
            merged.push(format!("{} {}", res[i], res[i + 1]));
            i += 2;
        } else {
            merged.push(res[i].clone());
            i += 1;
        }
    }

    // Re-join with comma, preserve || separators
    merged
        .join(", ")
        .replace(" , || , ", " || ")
        .replace(", ||, ", " || ")
        .replace("||,", "||")
}

// ---------------------------------------------------------------------------
// Poetry versioning API
// ---------------------------------------------------------------------------

/// Whether `input` is a valid poetry version or range.
pub fn is_valid(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    // Reject URLs / git refs
    if input.contains("://") || input.contains('#') {
        return false;
    }
    // `~=X.Y` is a PEP 440 compatible-release operator — always valid when
    // the version part is valid
    if let Some(rest) = input.strip_prefix("~=").or_else(|| {
        // handle with spaces: "~= 1.9"
        input.strip_prefix("~=")
    }) {
        if is_version(rest.trim()) || Version::parse(rest.trim()).is_ok() {
            return true;
        }
    }
    // `===X.Y.Z` is a PEP 440 arbitrary equality operator
    if let Some(rest) = input.strip_prefix("===") {
        if is_version(rest.trim()) {
            return true;
        }
    }
    // `==X.Y.*` or `==X.Y.Z` is a PEP 440 matching operator
    if let Some(rest) = input.strip_prefix("==") {
        let r = rest.trim();
        if is_version(r) || Version::parse(r).is_ok() {
            return true;
        }
        // wildcard form: ==1.9.* or ==1.*
        if r.ends_with(".*") {
            return true;
        }
    }
    // Multi-value PEP 440 constraint with commas and != or spaces — not supported
    if input.contains("!=") {
        return false;
    }
    match poetry2npm(input) {
        Some(npm) => {
            // Wildcard
            if matches!(npm.trim(), "*") {
                return true;
            }
            // Valid npm range
            semver::VersionReq::parse(&npm).is_ok()
                || Version::parse(npm.trim()).is_ok()
        }
        None => false,
    }
}

/// Whether `input` is a valid single version (no range operators).
pub fn is_version(input: &str) -> bool {
    version_pattern().is_match(input.trim())
}

/// Return the major component of a poetry version, or `None`.
pub fn get_major(v: &str) -> Option<u64> {
    poetry2semver(v, true).and_then(|s| Version::parse(&s).ok().map(|p| p.major))
}

/// Return the minor component of a poetry version, or `None`.
pub fn get_minor(v: &str) -> Option<u64> {
    poetry2semver(v, true).and_then(|s| Version::parse(&s).ok().map(|p| p.minor))
}

/// Return the patch component of a poetry version, or `None`.
pub fn get_patch(v: &str) -> Option<u64> {
    poetry2semver(v, true).and_then(|s| Version::parse(&s).ok().map(|p| p.patch))
}

/// Whether `a` equals `b` (poetry normalization applied first).
pub fn equals(a: &str, b: &str) -> bool {
    match (poetry2semver(a, true), poetry2semver(b, true)) {
        (Some(sa), Some(sb)) => super::npm::equals(&sa, &sb),
        _ => false,
    }
}

/// Whether `a` is strictly greater than `b` (PEP 440 ordering).
pub fn is_greater_than(a: &str, b: &str) -> bool {
    // Poetry delegates isGreaterThan to pep440
    super::pep440::is_greater_than(a, b)
}

/// Whether the version has no pre-release component.
pub fn is_stable(v: &str) -> bool {
    poetry2semver(v, true)
        .as_deref()
        .map_or(false, super::npm::is_stable)
}

/// Whether `version` satisfies `range`.
pub fn matches(version: &str, range: &str) -> bool {
    let Some(semver_version) = poetry2semver(version, false) else {
        return false;
    };
    if !is_version(version) {
        return false;
    }
    let range = range.trim();

    // `===X.Y.Z` — PEP 440 arbitrary equality (exact string match after normalization)
    if let Some(exact) = range.strip_prefix("===") {
        let exact = exact.trim();
        if let (Some(sv), Some(se)) = (poetry2semver(version, false), poetry2semver(exact, false)) {
            return sv == se;
        }
        return false;
    }

    // `==X.Y` — PEP 440 compatible version match (treats X.Y as X.Y.*)
    if let Some(spec) = range.strip_prefix("==") {
        let spec = spec.trim();
        // Check if it's a wildcard form: ==1.9.* — means patch can be anything
        if spec.ends_with(".*") {
            let prefix = spec.trim_end_matches(".*");
            let parts: Vec<&str> = prefix.split('.').collect();
            if let Some(sv) = poetry2semver(version, false) {
                let Ok(v) = Version::parse(&sv) else { return false };
                match parts.as_slice() {
                    [major] => {
                        return v.major == major.parse::<u64>().unwrap_or(u64::MAX);
                    }
                    [major, minor] => {
                        return v.major == major.parse::<u64>().unwrap_or(u64::MAX)
                            && v.minor == minor.parse::<u64>().unwrap_or(u64::MAX);
                    }
                    _ => {}
                }
            }
            return false;
        }
        // `==X.Y` means version is in X.Y.* (same as X.Y.*) in PEP 440
        // But in poetry, it means the version starts with X.Y
        if let (Some(sv), Some(se)) = (
            poetry2semver(version, false),
            poetry2semver(spec, false),
        ) {
            // Check if major.minor match (wildcard on patch)
            let spec_dots = spec.split('.').count();
            let Ok(vv) = Version::parse(&sv) else { return false };
            let Ok(sv_spec) = Version::parse(&se) else { return false };
            return match spec_dots {
                1 => vv.major == sv_spec.major,
                2 => vv.major == sv_spec.major && vv.minor == sv_spec.minor,
                _ => sv == se,
            };
        }
        return false;
    }

    let Some(npm_range) = poetry2npm(range) else {
        return false;
    };
    super::npm::matches_range(&semver_version, &npm_range)
}

fn normalize_range_ops(range: &str) -> String {
    let mut tokens = range.split_whitespace().peekable();
    let mut out = Vec::new();
    let ops = [">=", ">", "<=", "<", "=", "~", "^"];
    while let Some(token) = tokens.next() {
        if ops.contains(&token) {
            if let Some(ver) = tokens.next() {
                out.push(format!("{}{}", token, ver));
            } else {
                out.push(token.to_owned());
            }
        } else {
            out.push(token.to_owned());
        }
    }
    out.join(" ")
}

/// Whether `version` is below all bounds of `range`.
pub fn is_less_than_range(version: &str, range: &str) -> bool {
    let Some(semver_version) = poetry2semver(version, true) else {
        return false;
    };
    if !is_version(version) {
        return false;
    }
    let Some(npm_range) = poetry2npm(range) else {
        return false;
    };
    let normalized = normalize_range_ops(&npm_range);
    super::npm::is_less_than_range(&semver_version, &normalized)
}

/// Return the maximum version from `versions` satisfying `range`, in poetry format.
pub fn get_satisfying_version(versions: &[&str], range: &str) -> Option<String> {
    let npm_range = poetry2npm(range)?;
    let semver_versions: Vec<String> = versions
        .iter()
        .filter_map(|&v| poetry2semver(v, false))
        .collect();
    let semver_strs: Vec<&str> = semver_versions.iter().map(String::as_str).collect();
    let satisfying = super::npm::get_satisfying_version(&semver_strs, &npm_range)?;
    semver2poetry(satisfying).or_else(|| Some(satisfying.to_owned()))
}

/// Return the minimum version from `versions` satisfying `range`, in poetry format.
pub fn min_satisfying_version(versions: &[&str], range: &str) -> Option<String> {
    let npm_range = poetry2npm(range)?;
    let semver_versions: Vec<String> = versions
        .iter()
        .filter_map(|&v| poetry2semver(v, false))
        .collect();
    let semver_strs: Vec<&str> = semver_versions.iter().map(String::as_str).collect();
    let satisfying = super::npm::min_satisfying_version(&semver_strs, &npm_range)?;
    semver2poetry(satisfying).or_else(|| Some(satisfying.to_owned()))
}

/// Whether `constraint` is a single version (not a range).
pub fn is_single_version(constraint: &str) -> bool {
    let c = constraint.trim();
    let inner = c.strip_prefix('=').map(str::trim).unwrap_or(c);
    let inner = inner.strip_prefix(' ').unwrap_or(inner);
    is_version(inner.trim())
}

/// Sort comparator: returns positive when `a > b`, negative when `a < b`, 0 when equal.
pub fn sort_versions(a: &str, b: &str) -> i32 {
    match super::pep440::sort_versions(a, b) {
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
    }
}

/// Whether `sub_range` is a subset of `super_range`.
pub fn subset(sub_range: &str, super_range: &str) -> Option<bool> {
    let sub_npm = poetry2npm(sub_range)?;
    let super_npm = poetry2npm(super_range)?;
    Some(super::npm::subset(&sub_npm, &super_npm))
}

/// Compute a new version string for the given update.
///
/// Mirrors `getNewValue` from `lib/modules/versioning/poetry/index.ts`.
pub fn get_new_value(
    current_value: &str,
    range_strategy: &str,
    current_version: Option<&str>,
    new_version: &str,
) -> Option<String> {
    // replace strategy: if new version already satisfies current, keep current
    if range_strategy == "replace" {
        let npm_current_value = poetry2npm(current_value)?;
        if let Some(massaged_new) = poetry2semver(new_version, false) {
            if Version::parse(&massaged_new).is_ok()
                && is_version(&massaged_new)
                && super::npm::matches_range(&massaged_new, &npm_current_value)
            {
                return Some(current_value.to_owned());
            }
        }
        // Check for single-comparator caret/tilde
        let parsed_range = parse_range_elements(&npm_current_value);
        if let Some(element) = parsed_range.last() {
            if parsed_range.len() == 1 {
                if element.starts_with('^') {
                    if let Some(v) = handle_short("^", &npm_current_value, new_version) {
                        return Some(npm2poetry(&v));
                    }
                }
                if element.starts_with('~') {
                    if let Some(v) = handle_short("~", &npm_current_value, new_version) {
                        return Some(npm2poetry(&v));
                    }
                }
            }
        }
    }

    // Must have a 3-component release
    let release_parts = version_pattern()
        .captures(new_version.trim())
        .and_then(|c| c.name("release").map(|m| m.as_str().to_owned()));
    if release_parts
        .as_deref()
        .map(|r| r.split('.').count())
        != Some(3)
    {
        return Some(current_value.to_owned());
    }

    let current_semver = current_version.and_then(|cv| poetry2semver(cv, false));
    let new_semver = poetry2semver(new_version, false)?;
    let npm_current_value = poetry2npm(current_value)?;

    let new_npm = super::npm::get_new_value(
        &npm_current_value,
        range_strategy,
        current_semver.as_deref().unwrap_or(""),
        &new_semver,
    )?;
    Some(npm2poetry(&new_npm))
}

/// Parse a semver range string into its comparator elements.
fn parse_range_elements(range: &str) -> Vec<String> {
    range
        .split_whitespace()
        .map(str::to_owned)
        .collect()
}

/// Handle short-form `^X` / `~X` / `^X.Y` / `~X.Y` ranges for replace strategy.
fn handle_short(operator: &str, current_value: &str, new_version: &str) -> Option<String> {
    let major = get_major(new_version)?;
    let minor = get_minor(new_version)?;
    let split: Vec<&str> = current_value
        .trim_start_matches(operator)
        .split('.')
        .collect();
    if split.len() == 1 {
        return Some(format!("{operator}{major}"));
    }
    if split.len() == 2 {
        return Some(format!("{operator}{major}.{minor}"));
    }
    None
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ── poetry2semver ────────────────────────────────────────────────────────

    #[test]
    fn poetry2semver_basic_versions() {
        assert_eq!(poetry2semver("1", true).as_deref(), Some("1.0.0"));
        assert_eq!(poetry2semver("1.0", true).as_deref(), Some("1.0.0"));
        assert_eq!(poetry2semver("1.9", true).as_deref(), Some("1.9.0"));
        assert_eq!(poetry2semver("1.9.4", true).as_deref(), Some("1.9.4"));
        assert_eq!(poetry2semver("17.04.01", true).as_deref(), Some("17.4.1"));
        assert_eq!(poetry2semver("!@#", true), None);
    }

    #[test]
    fn poetry2semver_pre_release() {
        // Always pads to 3 components (matching node-semver normalization)
        assert_eq!(
            poetry2semver("1.9b0", false).as_deref(),
            Some("1.9.0-b.0")
        );
        assert_eq!(
            poetry2semver("1.9.4b0", false).as_deref(),
            Some("1.9.4-b.0")
        );
        assert_eq!(
            poetry2semver("1.9.01b01", false).as_deref(),
            Some("1.9.1-b.1")
        );
        assert_eq!(
            poetry2semver("1.9.4a0", false).as_deref(),
            Some("1.9.4-a.0")
        );
    }

    #[test]
    fn poetry2semver_post_release() {
        // Always pads to 3 components
        assert_eq!(
            poetry2semver("1.9-0", false).as_deref(),
            Some("1.9.0-post.0")
        );
        assert_eq!(
            poetry2semver("1.9.0-post", false).as_deref(),
            Some("1.9.0-post.0")
        );
    }

    #[test]
    fn poetry2semver_dev_release() {
        assert_eq!(
            poetry2semver("1.9.0dev0", false).as_deref(),
            Some("1.9.0-dev.0")
        );
    }

    // ── equals ───────────────────────────────────────────────────────────────

    // Ported: "equals("$a", "$b") === $expected" — poetry/index.spec.ts line 5
    #[test]
    fn equals_basic() {
        assert!(equals("1", "1"));
        assert!(equals("1.0", "1"));
        assert!(equals("1.0.0", "1"));
        assert!(equals("1.9.0", "1.9"));
        assert!(!equals("1", "2"));
        assert!(!equals("1.9.1", "1.9"));
        assert!(!equals("1.9-beta", "1.9"));
        assert!(!equals("1.9b0", "1.9"));
        assert!(equals("1.9b0", "1.9.0-beta.0"));
        assert!(equals("1.9.01b01", "1.9.1-beta.1"));
        assert!(equals("1.9-0", "1.9.0-post.0"));
        assert!(equals("1.9.0-post", "1.9.0-post.0"));
        assert!(equals("1.9.01-post", "1.9.1-post.0"));
        assert!(equals("1.9.0dev0", "1.9.0-dev.0"));
    }

    // ── getMajor/getMinor/getPatch ────────────────────────────────────────────

    // Ported: "getMajor, getMinor, getPatch for "$version"" — poetry/index.spec.ts line 28
    #[test]
    fn get_version_parts() {
        assert_eq!(get_major("1"), Some(1));
        assert_eq!(get_minor("1"), Some(0));
        assert_eq!(get_patch("1"), Some(0));
        assert_eq!(get_major("1.9"), Some(1));
        assert_eq!(get_minor("1.9"), Some(9));
        assert_eq!(get_patch("1.9"), Some(0));
        assert_eq!(get_major("1.9.4"), Some(1));
        assert_eq!(get_minor("1.9.4"), Some(9));
        assert_eq!(get_patch("1.9.4"), Some(4));
        assert_eq!(get_major("1.9.4b0"), Some(1));
        assert_eq!(get_major("17.04.01"), Some(17));
        assert_eq!(get_minor("17.04.01"), Some(4));
        assert_eq!(get_patch("17.04.01"), Some(1));
        assert_eq!(get_major("!@#"), None);
        assert_eq!(get_minor("!@#"), None);
        assert_eq!(get_patch("!@#"), None);
    }

    // ── isGreaterThan ────────────────────────────────────────────────────────

    // Ported: "isGreaterThan("$a", "$b") === $expected" — poetry/index.spec.ts line 47
    #[test]
    fn is_greater_than_cases() {
        assert!(is_greater_than("2", "1"));
        assert!(is_greater_than("2.0", "1"));
        assert!(is_greater_than("2.0.0", "1"));
        assert!(is_greater_than("1.10.0", "1.9"));
        assert!(is_greater_than("1.9", "1.9-beta"));
        assert!(is_greater_than("1.9", "1.9a0"));
        assert!(!is_greater_than("1", "1"));
        assert!(!is_greater_than("1.0", "1"));
        assert!(!is_greater_than("1.0.0", "1"));
        assert!(!is_greater_than("1.9.0", "1.9"));
    }

    // ── isStable ─────────────────────────────────────────────────────────────

    // Ported: "isStable("$version") === $expected" — poetry/index.spec.ts line 82
    #[test]
    fn is_stable_cases() {
        assert!(is_stable("1"));
        assert!(is_stable("1.9"));
        assert!(is_stable("1.9.0"));
        assert!(is_stable("1.9.4"));
        assert!(!is_stable("1.9.4-beta"));
        assert!(!is_stable("1.9.4a0"));
    }

    // ── isVersion ───────────────────────────────────────────────────────────

    // Ported: "isVersion("$version") === $expected" — poetry/index.spec.ts line 95
    #[test]
    fn is_version_cases() {
        assert!(is_version("1.2.3a0"));
        assert!(is_version("1.2.3b1"));
        assert!(is_version("1.2.3rc23"));
        assert!(is_version("17.04.01"));
        assert!(!is_version("17.b4.0"));
        assert!(!is_version("0.98.5.1"));
    }

    // ── isValid ─────────────────────────────────────────────────────────────

    // Ported: "isValid("$version") === $expected" — poetry/index.spec.ts line 107
    #[test]
    fn is_valid_cases() {
        assert!(!is_valid(""));
        assert!(is_valid("17.04.00"));
        assert!(!is_valid("17.b4.0"));
        assert!(is_valid("1.2.3"));
        assert!(is_valid("1.2.3-foo"));
        assert!(!is_valid("1.2.3foo"));
        assert!(is_valid("1.2.3a0"));
        assert!(is_valid("1.2.3b1"));
        assert!(is_valid("1.2.3rc23"));
        assert!(is_valid("*"));
        assert!(is_valid("~1.2.3"));
        assert!(is_valid("^1.2.3"));
        assert!(is_valid(">1.2.3"));
        assert!(is_valid("~=1.9"));
        assert!(is_valid("==1.9"));
        assert!(is_valid("===1.9.4"));
        assert!(!is_valid("renovatebot/renovate"));
        assert!(!is_valid("renovatebot/renovate#master"));
        assert!(!is_valid("https://github.com/renovatebot/renovate.git"));
        // The >=..., !=... multi-part pep440 range is not supported by poetry
        // (poetry uses commas, not spaces+commas mixed)
        assert!(!is_valid(">=2.6, !=3.0.*, !=3.1.*, !=3.2.*, <4"));
    }

    // ── isSingleVersion ─────────────────────────────────────────────────────

    // Ported: "isSingleVersion("$version") === $expected" — poetry/index.spec.ts line 134
    #[test]
    fn is_single_version_cases() {
        assert!(is_single_version("1.2.3"));
        assert!(is_single_version("1.2.3-alpha.1"));
        assert!(is_single_version("=1.2.3"));
        assert!(is_single_version("= 1.2.3"));
        assert!(!is_single_version("1.*"));
    }

    // ── matches ─────────────────────────────────────────────────────────────

    // Ported: "matches("$version", "$range") === "$expected"" — poetry/index.spec.ts line 145
    #[test]
    fn matches_cases() {
        assert!(matches("4.2.0", "4.2, >= 3.0, < 5.0.0"));
        assert!(!matches("4.2.0", "2.0, >= 3.0, < 5.0.0"));
        assert!(!matches("4.2.2", "4.2.0, < 4.2.4"));
        assert!(matches("4.2.2", "^4.2.0, < 4.2.4"));
        assert!(!matches("4.2.0", "4.3.0, 3.0.0"));
        assert!(!matches("4.2.0", "> 5.0.0, <= 6.0.0"));
        assert!(matches("4.2.0", "*"));
        assert!(matches("1.9.4", "==1.9"));
        assert!(matches("1.9.4", "===1.9.4"));
        assert!(!matches("1.9.4", "===1.9.3"));
        assert!(matches("0.8.0a1", "^0.8.0-alpha.0"));
        assert!(!matches("0.7.4", "^0.8.0-alpha.0"));
        assert!(matches("1.4", "1.4"));
    }

    // ── isLessThanRange ──────────────────────────────────────────────────────

    // Ported: "isLessThanRange("$version", "$range") === "$expected"" — poetry/index.spec.ts line 167
    #[test]
    fn is_less_than_range_cases() {
        assert!(is_less_than_range("0.9.0", ">= 1.0.0 <= 2.0.0"));
        assert!(!is_less_than_range("1.9.0", ">= 1.0.0 <= 2.0.0"));
    }

    // ── sortVersions ────────────────────────────────────────────────────────

    // Ported: "sortVersions("$a", "$b") === $expected" — poetry/index.spec.ts line 269
    #[test]
    fn sort_versions_cases() {
        assert_eq!(sort_versions("2", "1"), 1);
        assert_eq!(sort_versions("2.0", "1"), 1);
        assert_eq!(sort_versions("2.0.0", "1"), 1);
        assert_eq!(sort_versions("1.10.0", "1.9"), 1);
        assert_eq!(sort_versions("1.9", "1.9-beta"), 1);
        assert_eq!(sort_versions("1", "1"), 0);
        assert_eq!(sort_versions("1.0", "1"), 0);
        assert_eq!(sort_versions("1.0.0", "1"), 0);
        assert_eq!(sort_versions("1.9.0", "1.9"), 0);
        assert_eq!(sort_versions("1.9", "1.9b"), 1);
        assert_eq!(sort_versions("1.9", "1.9rc0"), 1);
    }
}
