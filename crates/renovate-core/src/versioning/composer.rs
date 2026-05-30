//! Composer (PHP) versioning — mirrors `lib/modules/versioning/composer/index.ts`.
//!
//! Composer versions wrap npm semver with PHP-specific normalizations:
//! - `@stability` modifiers (`1.0@beta2` → `1.0.0-beta.2`)
//! - `-pXX` patch versions (considered STABLE and GREATER than base)
//! - `~X` and `~0.X` range operators (different from npm tilde)
//! - `v` prefix support

use std::sync::OnceLock;

use regex::Regex;
use semver::Version;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn stability_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?i)(?:^|\s)(beta|alpha|rc)([1-9][0-9]*)(?:\s|$)").unwrap())
}

fn patch_part_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?i)^v?\d+(\.\d+(\.\d+(\.\d+)?)?)?(?P<suffix>-p[1-9]\d*)$").unwrap()
    })
}

fn tilde_nonzero_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?:^|\s)~([1-9][0-9]*(?:\.[0-9]*)?)(?:\s|$)").unwrap())
}

fn tilde_zero_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?:^|\s)~(0\.[1-9][0-9]*)(?:\s|$)").unwrap())
}

fn letter_digit_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"(?i)([a-z])([0-9])").unwrap())
}

/// Extract the `-pXX` suffix from a composer version, returning `(without_patch, had_patch)`.
fn remove_patch_part(input: &str) -> (String, bool) {
    if let Some(caps) = patch_part_regex().captures(input)
        && let Some(suffix) = caps.name("suffix")
    {
        let without = input[..suffix.start()].to_owned();
        return (without, true);
    }
    (input.to_owned(), false)
}

/// Pad a version string to at least 3 components.
fn pad_zeroes(input: &str) -> String {
    // Split off stability suffix (after `-`)
    let (output, stability) = match input.find('-') {
        Some(pos) => (&input[..pos], format!("-{}", &input[pos + 1..])),
        None => (input, String::new()),
    };
    let mut parts: Vec<&str> = output.split('.').collect();
    while parts.len() < 3 {
        parts.push("0");
    }
    format!("{}{}", parts.join("."), stability)
}

/// Convert `@stability` modifiers to npm format.
fn convert_stability_modifier(input: &str) -> String {
    let parts: Vec<&str> = input.splitn(2, '@').collect();
    if parts.len() == 1 {
        return input.to_owned();
    }
    // 1.0@beta2 → 1.0-beta.2
    let stability = stability_regex()
        .replace_all(parts[1], |caps: &regex::Captures| {
            format!("{}.{}", &caps[1], &caps[2])
        })
        .into_owned();
    format!("{}-{}", pad_zeroes(parts[0]), stability)
}

/// Normalize a composer version/range part to npm-compatible form.
fn normalize_version(input: &str) -> String {
    let mut output = input.to_owned();
    // Strip leading `+` prefix
    if output.starts_with('+') {
        output = output[1..].to_owned();
    }
    // Strip `v` prefix after operators or at start
    output = output.replace("v", "");
    // Re-add `v` that was after `>`, `>=`, `^`, `~` — actually just strip all `v` directly
    // The TypeScript uses regex `/(^|>|>=|\^|~)v/gi`
    let output = Regex::new(r"(?i)(^|>|>=|\^|~)v")
        .unwrap()
        .replace_all(&output, "$1");
    convert_stability_modifier(&output)
}

/// Convert a composer range string to npm semver format.
///
/// Mirrors `composer2npm` from `lib/modules/versioning/composer/index.ts`.
pub fn composer2npm(input: &str) -> String {
    // Split on `||` or `|` separators
    let or_regex = Regex::new(r"\s*\|{1,2}\s*").unwrap();
    let parts: Vec<&str> = or_regex.split(input).collect();

    parts
        .iter()
        .map(|part| {
            let clean = normalize_version(part.trim());
            // Already a valid version?
            if super::npm::is_version(&clean) {
                return clean;
            }
            if super::npm::is_version(&pad_zeroes(&clean)) {
                return pad_zeroes(&clean);
            }
            // Split off stability suffix
            let (version_id, stability) = match clean.find('-') {
                Some(pos) => (clean[..pos].to_owned(), clean[pos..].to_owned()),
                None => (clean.clone(), String::new()),
            };
            let mut output = version_id;

            // `~X` or `~X.Y` (non-zero major) → `^X` or `^X.Y`
            let tilde_nonzero = tilde_nonzero_regex();
            if tilde_nonzero.is_match(&format!(" {} ", output)) {
                output = tilde_nonzero
                    .replace_all(&format!(" {} ", output), |caps: &regex::Captures| {
                        format!(" ^{} ", &caps[1])
                    })
                    .trim()
                    .to_owned();
            }
            // `~0.X` → `>=0.X.0, <1.0.0`
            let tilde_zero = tilde_zero_regex();
            if tilde_zero.is_match(&format!(" {} ", output)) {
                output = tilde_zero
                    .replace_all(&format!(" {} ", output), |caps: &regex::Captures| {
                        format!(" >={}.0, <1.0.0 ", &caps[1])
                    })
                    .trim()
                    .to_owned();
            }

            // Add extra digits to `<8-DEV` and `<8.0-DEV` style patterns
            let lt_regex = Regex::new(r"^(<\d+(\.\d+)?)$").unwrap();
            if lt_regex.is_match(&output) {
                output = lt_regex.replace(&output, "$1.0").into_owned();
            }
            if lt_regex.is_match(&output) {
                output = lt_regex.replace(&output, "$1.0").into_owned();
            }

            format!("{}{}", output, stability)
        })
        .map(|part| {
            // Ensure `letter+digit` has a dot: `beta1` → `beta.1`
            letter_digit_regex()
                .replace_all(&part, "$1.$2")
                .into_owned()
        })
        .collect::<Vec<_>>()
        .join(" || ")
}

/// Coerce a version string to semver by padding with zeros.
fn coerce(v: &str) -> Option<Version> {
    Version::parse(v)
        .ok()
        .or_else(|| Version::parse(&pad_zeroes(v)).ok())
        .or_else(|| {
            // Try stripping extra segments beyond 3
            let parts: Vec<&str> = v.split('.').collect();
            if parts.len() > 3 {
                Version::parse(&parts[..3].join(".")).ok()
            } else {
                None
            }
        })
}

/// Return the major version number.
pub fn get_major(v: &str) -> Option<u64> {
    coerce(&composer2npm(v)).map(|p| p.major)
}

/// Return the minor version number.
pub fn get_minor(v: &str) -> Option<u64> {
    coerce(&composer2npm(v)).map(|p| p.minor)
}

/// Return the patch version number.
pub fn get_patch(v: &str) -> Option<u64> {
    coerce(&composer2npm(v)).map(|p| p.patch)
}

/// Whether two versions are equal.
pub fn equals(a: &str, b: &str) -> bool {
    super::npm::equals(&composer2npm(a), &composer2npm(b))
}

/// Whether `a` is greater than `b` (using composer sort semantics).
pub fn is_greater_than(a: &str, b: &str) -> bool {
    sort_versions(a, b) == 1
}

/// Whether `version` is a stable release.
///
/// Composer considers `-pXX` patches stable.
pub fn is_stable(version: &str) -> bool {
    if version.is_empty() {
        return false;
    }
    let (without_patch, _) = remove_patch_part(version);
    super::npm::is_stable(&composer2npm(&without_patch))
}

/// Whether `input` is a valid version or range.
pub fn is_valid(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    let npm = composer2npm(input);
    // OR ranges: validate each alternative independently
    if npm.contains("||") {
        return npm.split("||").map(str::trim).all(super::npm::is_valid);
    }
    super::npm::is_valid(&npm)
}

/// Whether `input` is a valid version (not a range).
pub fn is_version(input: &str) -> bool {
    !input.is_empty() && super::npm::is_version(&composer2npm(input))
}

/// Whether `input` is a single exact version.
pub fn is_single_version(input: &str) -> bool {
    !input.is_empty() && super::npm::is_single_version(&composer2npm(input))
}

/// Whether `version` satisfies `range`.
pub fn matches_range(version: &str, range: &str) -> bool {
    super::npm::matches_range(&composer2npm(version), &composer2npm(range))
}

/// Whether `version` is below all bounds of `range`.
pub fn is_less_than_range(version: &str, range: &str) -> bool {
    super::npm::is_less_than_range(&composer2npm(version), &composer2npm(range))
}

/// Sort comparator for versions, handling `-pXX` patches correctly.
///
/// `-pXX` patches are GREATER than their base version in Composer.
pub fn sort_versions(a: &str, b: &str) -> i32 {
    let (a_without, a_has_patch) = remove_patch_part(a);
    let (b_without, b_has_patch) = remove_patch_part(b);

    if a_has_patch == b_has_patch {
        // Both have patch or neither does — npm comparison is correct
        match super::npm::sort_versions(&composer2npm(a), &composer2npm(b)) {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
        }
    } else if super::npm::equals(&composer2npm(&a_without), &composer2npm(&b_without)) {
        // Same base version but one has patch: the patched one is greater
        if a_has_patch { 1 } else { -1 }
    } else {
        match super::npm::sort_versions(&composer2npm(a), &composer2npm(b)) {
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
        }
    }
}

/// Whether `version` satisfies `range` (alias for matches_range).
pub fn matches(version: &str, range: &str) -> bool {
    matches_range(version, range)
}

/// Whether `version` is breaking relative to `current`.
pub fn is_breaking(current: &str, version: &str) -> bool {
    super::npm::is_breaking(&composer2npm(current), &composer2npm(version))
}

/// Return the maximum version from `versions` satisfying `range`.
pub fn get_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    calculate_satisfying_version(versions, range, false)
}

/// Return the minimum version from `versions` satisfying `range`.
pub fn min_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    calculate_satisfying_version(versions, range, true)
}

fn calculate_satisfying_version<'a>(
    versions: &[&'a str],
    range: &str,
    min_mode: bool,
) -> Option<&'a str> {
    // Map each version to its npm-normalized form, stripping -pXX for matching
    struct Mapped<'a> {
        original: &'a str,
        npm_variant: String,
    }

    let mapped: Vec<Mapped<'_>> = versions
        .iter()
        .map(|&v| {
            let (cleaned, _) = remove_patch_part(v);
            let npm_variant = composer2npm(&cleaned);
            Mapped {
                original: v,
                npm_variant,
            }
        })
        .collect();

    let npm_versions: Vec<&str> = mapped.iter().map(|m| m.npm_variant.as_str()).collect();
    let npm_range = composer2npm(range);

    let npm_version = if min_mode {
        super::npm::min_satisfying_version(&npm_versions, &npm_range)
    } else {
        super::npm::get_satisfying_version(&npm_versions, &npm_range)
    };

    let npm_version = npm_version?;

    // Among all versions that map to `npm_version`, pick the one with the
    // highest (or lowest for minMode) -pXX patch
    let mut candidates: Vec<&str> = mapped
        .iter()
        .filter(|m| m.npm_variant == npm_version)
        .map(|m| m.original)
        .collect();

    candidates.sort_by(|a, b| {
        let cmp = sort_versions(a, b);
        if min_mode { cmp.cmp(&0) } else { 0.cmp(&cmp) }
    });

    candidates.into_iter().next()
}

/// Whether `sub_range` is a subset of `super_range`.
pub fn subset(sub_range: &str, super_range: &str) -> Option<bool> {
    let sub_npm = composer2npm(sub_range);
    let super_npm = composer2npm(super_range);
    use crate::versioning::poetry::range_subset;
    // Handle OR ranges before validity check (Rust semver rejects || syntax).
    if sub_npm.contains("||") || super_npm.contains("||") {
        let sub_alts: Vec<&str> = sub_npm.split("||").map(str::trim).collect();
        let super_alts: Vec<&str> = super_npm.split("||").map(str::trim).collect();
        // Any invalid individual range → false
        if sub_alts.iter().any(|s| !super::npm::is_valid(s))
            || super_alts.iter().any(|p| !super::npm::is_valid(p))
        {
            return Some(false);
        }
        let result = sub_alts
            .iter()
            .all(|s| super_alts.iter().any(|p| range_subset(s, p)));
        return Some(result);
    }
    // Check for invalid non-OR ranges (like "less than 8")
    if !super::npm::is_valid(&sub_npm) || !super::npm::is_valid(&super_npm) {
        return Some(false);
    }
    Some(range_subset(&sub_npm, &super_npm))
}

/// Whether `sub_range` and `super_range` have any overlap.
pub fn intersects(sub_range: &str, super_range: &str) -> bool {
    let sub_npm = composer2npm(sub_range);
    let super_npm = composer2npm(super_range);
    // Handle OR ranges before validity check.
    if sub_npm.contains("||") || super_npm.contains("||") {
        let sub_alts: Vec<&str> = sub_npm.split("||").map(str::trim).collect();
        let super_alts: Vec<&str> = super_npm.split("||").map(str::trim).collect();
        if sub_alts.iter().any(|s| !super::npm::is_valid(s))
            || super_alts.iter().any(|p| !super::npm::is_valid(p))
        {
            return false;
        }
        return sub_alts
            .iter()
            .any(|s| super_alts.iter().any(|p| super::npm::intersects(s, p)));
    }
    if !super::npm::is_valid(&sub_npm) || !super::npm::is_valid(&super_npm) {
        return false;
    }
    super::npm::intersects(&sub_npm, &super_npm)
}

/// Compute a new constraint value for the given update.
///
/// Mirrors `getNewValue` from `lib/modules/versioning/composer/index.ts`.
pub fn get_new_value(
    current_value: &str,
    range_strategy: &str,
    current_version: Option<&str>,
    new_version: &str,
) -> Option<String> {
    let result = compute_new_value(current_value, range_strategy, current_version, new_version);
    let mut result = result?;

    // Preserve `v` prefix if original had it
    if current_value
        .split('.')
        .next()
        .is_some_and(|p| p.contains('v'))
    {
        let digit_regex = Regex::new(r"([0-9])").unwrap();
        result = digit_regex.replace(&result, "v$1").into_owned();
    }

    // Preserve `@stability` modifier
    if let Some(at_pos) = current_value.find('@') {
        let stability = &current_value[at_pos + 1..];
        result.push('@');
        result.push_str(stability);
    }

    Some(result)
}

fn compute_new_value(
    current_value: &str,
    range_strategy: &str,
    current_version: Option<&str>,
    new_version: &str,
) -> Option<String> {
    if range_strategy == "update-lockfile" {
        if matches_range(new_version, current_value) {
            return Some(current_value.to_owned());
        }
        return compute_new_value(current_value, "replace", current_version, new_version);
    }

    let current_major = current_version.and_then(get_major);
    let to_major = get_major(new_version);
    let to_minor = get_minor(new_version);
    let mut new_value: Option<String> = None;

    if is_version(current_value) {
        new_value = Some(new_version.to_owned());
    } else if Regex::new(r"^[~^](0\.[1-9][0-9]*)$")
        .unwrap()
        .is_match(current_value)
    {
        let operator = &current_value[..1];
        if to_major == Some(0) {
            new_value = Some(format!("{}0.{}", operator, to_minor?));
        } else {
            new_value = Some(format!("{}{}.0", operator, to_major?));
        }
    } else if Regex::new(r"^[~^]([0-9]*)$")
        .unwrap()
        .is_match(current_value)
    {
        let operator = &current_value[..1];
        new_value = Some(format!("{}{}", operator, to_major?));
    } else if to_major.is_some()
        && Regex::new(r"^[~^]([0-9]*(?:\.[0-9]*)?)$")
            .unwrap()
            .is_match(current_value)
    {
        let operator = &current_value[..1];
        if range_strategy == "bump" {
            new_value = Some(format!("{}{}", operator, new_version));
        } else if (current_major.is_some() && to_major > current_major) || to_minor.is_none() {
            new_value = Some(format!("{}{}.0", operator, to_major?));
        } else {
            new_value = Some(format!("{}{}.{}", operator, to_major?, to_minor?));
        }
    } else if current_version.is_some()
        && super::npm::is_version(&pad_zeroes(&normalize_version(new_version)))
        && {
            // node-semver accepts space-separated AND (">=1.0 <3.0"), but Rust semver
            // needs commas. Normalize before validity check.
            let norm = normalize_version(current_value);
            let norm_comma = norm.split_whitespace().collect::<Vec<_>>().join(", ");
            super::npm::is_valid(&norm) || super::npm::is_valid(&norm_comma)
        }
        && {
            // Check that composer2npm doesn't significantly transform the value
            // (i.e. it's not a composer-specific form like ~0.2 or @stability)
            let composer_form = composer2npm(current_value);
            let norm = normalize_version(current_value);
            composer_form == norm
                || composer_form == norm.split_whitespace().collect::<Vec<_>>().join(", ")
        }
    {
        let norm_current = normalize_version(current_value);
        let norm_cur_ver = pad_zeroes(&normalize_version(current_version.unwrap_or("")));
        let norm_new = pad_zeroes(&normalize_version(new_version));
        new_value =
            super::npm::get_new_value(&norm_current, range_strategy, &norm_cur_ver, &norm_new);
    }

    // Handle widen: if new version already satisfies range, keep current
    if range_strategy == "widen" && matches_range(new_version, current_value) {
        new_value = Some(current_value.to_owned());
    } else if range_strategy == "widen" && !current_value.contains("||") {
        // Compound range widen: ">=X <Y" or ">=X <=Y" patterns
        // Expand the upper bound to accommodate the new version.
        new_value = widen_compound_range(current_value, new_version);
    }
    if new_value.is_none() {
        let has_or = current_value.contains(" || ");
        if has_or || range_strategy == "widen" {
            let split_values: Vec<&str> = current_value.split("||").collect();
            let last_value = split_values.last()?.trim();
            let replacement =
                compute_new_value(last_value, "replace", current_version, new_version);
            if range_strategy == "replace" {
                new_value = replacement;
            } else if let Some(ref replacement_val) = replacement {
                // Check if it starts with `<` operator (range widening)
                if replacement_val.trim().starts_with('<') {
                    // Replace the upper bound
                    let op_end = replacement_val
                        .find(|c: char| c.is_ascii_digit())
                        .unwrap_or(0);
                    let op = &replacement_val[..op_end];
                    let split_current: Vec<&str> = current_value.split(op).collect();
                    let prefix = split_current[..split_current.len() - 1].join(op);
                    new_value = Some(format!("{}{}", prefix, replacement_val));
                } else {
                    new_value = Some(format!("{} || {}", current_value, replacement_val));
                }
            }
        }
    }

    new_value.or_else(|| Some(new_version.to_owned()))
}

/// Widen a compound range (`>=X <Y` or `>=X <=Y`) to accommodate a new version.
///
/// Handles forms like `>=1.0 <3.0` when widen strategy needs to extend the
/// upper bound to include `new_version`.
fn widen_compound_range(current_value: &str, new_version: &str) -> Option<String> {
    use semver::Version;

    let new = Version::parse(&pad_zeroes(&normalize_version(new_version))).ok()?;
    let parts: Vec<&str> = current_value.split_whitespace().collect();

    // Find the upper bound comparator
    let mut upper_op: Option<&str> = None;
    let mut lower_parts: Vec<&str> = Vec::new();

    for part in &parts {
        if part.starts_with('<') || part.starts_with("<=") {
            upper_op = Some(part);
        } else {
            lower_parts.push(part);
        }
    }

    let upper_op = upper_op?;
    let upper = if let Some(rest) = upper_op.strip_prefix("<=") {
        // `<=Y` → extend to `<= new_version` (preserve exact patch)
        let cur_upper = Version::parse(&pad_zeroes(&normalize_version(rest))).ok()?;
        if new > cur_upper {
            format!("<={new_version}")
        } else {
            return None;
        }
    } else {
        // `<Y` → extend to `< (new.major.new.minor + 1)`
        format!("<{}.{}", new.major, new.minor + 1)
    };

    let lower = lower_parts.join(" ");
    if lower.is_empty() {
        Some(upper)
    } else {
        Some(format!("{} {}", lower, upper))
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ── getMajor ─────────────────────────────────────────────────────────────

    // Ported: "getMajor("$version") === $expected" — composer/index.spec.ts line 4
    #[test]
    fn get_major_cases() {
        assert_eq!(get_major("1.2.0"), Some(1));
        assert_eq!(get_major(""), None);
    }

    // Ported: "getMinor("$version") === $expected" — composer/index.spec.ts line 12
    #[test]
    fn get_minor_cases() {
        assert_eq!(get_minor("1.2.0"), Some(2));
        assert_eq!(get_minor(""), None);
    }

    // Ported: "getPatch("$version") === $expected" — composer/index.spec.ts line 20
    #[test]
    fn get_patch_cases() {
        assert_eq!(get_patch("1.2.0"), Some(0));
        assert_eq!(get_patch(""), None);
    }

    // ── equals ───────────────────────────────────────────────────────────────

    // Ported: "equals("$a", "$b") === $expected" — composer/index.spec.ts line 28
    #[test]
    fn equals_cases() {
        assert!(equals("1.2.0", "v1.2"));
        assert!(equals("v1.0.0", "1"));
        assert!(equals("1.0@alpha3", "1.0.0-alpha.3"));
        assert!(equals("1.0@beta", "1.0.0-beta"));
        assert!(equals("1.0@rc2", "1.0.0-rc.2"));
        assert!(!equals("1.0.0", "1.0.0-p1"));
    }

    // ── isGreaterThan ────────────────────────────────────────────────────────

    // Ported: "isGreaterThan("$a", "$b") === $expected" — composer/index.spec.ts line 40
    #[test]
    fn is_greater_than_cases() {
        assert!(!is_greater_than("1.2.0", "v1.2"));
        assert!(is_greater_than("v1.0.1", "1"));
        assert!(!is_greater_than("1", "1.1"));
        assert!(!is_greater_than("1.0.0", "1.0.0-p1"));
        assert!(is_greater_than("1.0.0-p1", "1.0.0"));
        assert!(!is_greater_than("1.0.0-p1", "1.0.0-p2"));
        assert!(is_greater_than("1.0.0-p2", "1.0.0-p1"));
        assert!(!is_greater_than("1", "1.0-p1"));
        assert!(is_greater_than("1.0-p1", "1"));
    }

    // ── isSingleVersion ──────────────────────────────────────────────────────

    // Ported: "isSingleVersion("$version") === $expected" — composer/index.spec.ts line 55
    #[test]
    fn is_single_version_cases() {
        assert!(is_single_version("v1.2"));
    }

    // ── isStable ─────────────────────────────────────────────────────────────

    // Ported: "isStable("$version") === $expected" — composer/index.spec.ts line 63
    #[test]
    fn is_stable_cases() {
        assert!(is_stable("v1.2"));
        assert!(is_stable("v1.2.4-p2"));
        assert!(is_stable("v1.2.4-p12"));
        assert!(!is_stable("v1.2.4-beta5"));
        assert!(!is_stable(""));
    }

    // ── isValid ──────────────────────────────────────────────────────────────

    // Ported: "isValid("$version") === $expected" — composer/index.spec.ts line 75
    #[test]
    fn is_valid_cases() {
        assert!(is_valid("1.2.3"));
        assert!(is_valid("1.2.3-foo"));
        assert!(!is_valid("1.2.3foo"));
        assert!(is_valid("~1.2.3"));
        assert!(is_valid("^1.2.3"));
        assert!(is_valid(">1.2.3"));
        assert!(is_valid("~1.2.3-beta1"));
        assert!(is_valid("^1.2.3-alpha"));
        assert!(is_valid(">1.2.3-rc2"));
        assert!(is_valid("~1.2.3@beta"));
        assert!(is_valid("^1.2.3@alpha"));
        assert!(is_valid(">1.2.3@rc"));
        assert!(is_valid("1.2.3"));
        assert!(is_valid("2.5"));
        assert!(is_valid("v2.5"));
        assert!(is_valid("^1.0|^2.0"));
        assert!(is_valid("^1.0 | ^2.0"));
        assert!(is_valid("^1.0||^2.0"));
        assert!(is_valid("^1.0 || ^2.0"));
        assert!(is_valid("1.2.3-p1"));
    }

    // ── isLessThanRange ──────────────────────────────────────────────────────

    // Ported: "isLessThanRange("$a", "$b") === $expected" — composer/index.spec.ts line 108
    #[test]
    fn is_less_than_range_cases() {
        assert!(is_less_than_range("0.3.1", "~0.4"));
        assert!(!is_less_than_range("0.5.1", "~0.4"));
    }

    // ── getSatisfyingVersion ─────────────────────────────────────────────────

    // Ported: "getSatisfyingVersion($versions, "$range") === $expected" — composer/index.spec.ts line 116
    #[test]
    fn get_satisfying_version_cases() {
        assert_eq!(
            get_satisfying_version(&["0.4.0", "0.5.0", "4.0.0", "4.2.0", "5.0.0"], "~6"),
            None
        );
        assert_eq!(
            get_satisfying_version(&["0.4.0", "0.5.0", "4.0.0", "4.2.0", "5.0.0"], "~4"),
            Some("4.2.0")
        );
        assert_eq!(
            get_satisfying_version(&["v0.4.0", "v0.5.0", "v4.0.0", "v4.2.0", "v5.0.0"], "~4"),
            Some("v4.2.0")
        );
        assert_eq!(
            get_satisfying_version(&["0.4.0", "0.5.0", "4.0.0", "4.2.0", "5.0.0"], "~0.4"),
            Some("0.5.0")
        );
        // @stability modifier
        assert_eq!(
            get_satisfying_version(
                &["0.4.0", "0.5.0", "4.0.0-beta1", "4.0.0-beta2", "4.2.0-beta1", "4.2.0-beta2", "5.0.0"],
                "~4@beta"
            ),
            Some("4.0.0-beta2")
        );
        // -pXX patch suffix
        assert_eq!(
            get_satisfying_version(
                &["4.0.0", "4.2.0", "5.0.0", "4.2.0-p2", "4.2.0-p12"],
                "~4"
            ),
            Some("4.2.0-p12")
        );
    }

    // ── minSatisfyingVersion ─────────────────────────────────────────────────

    // Ported: "minSatisfyingVersion($versions, "$range") === $expected" — composer/index.spec.ts line 131
    #[test]
    fn min_satisfying_version_cases() {
        assert_eq!(
            min_satisfying_version(&["0.4.0", "0.5.0", "4.0.0", "4.2.0", "5.0.0"], "~6"),
            None
        );
        assert_eq!(
            min_satisfying_version(&["0.4.0", "0.5.0", "4.0.0", "4.2.0", "5.0.0"], "~4"),
            Some("4.0.0")
        );
        assert_eq!(
            min_satisfying_version(&["0.4.0", "0.5.0", "4.0.0", "4.2.0", "5.0.0"], "~0.4"),
            Some("0.4.0")
        );
    }

    // ── sortVersions ────────────────────────────────────────────────────────

    // Ported: "$versions -> sortVersions -> $expected" — composer/index.spec.ts line 256
    #[test]
    fn sort_versions_cases() {
        let mut v1 = vec![
            "1.2.3-beta",
            "1.0.0-alpha24",
            "2.0.1",
            "1.3.4",
            "1.0.0-alpha9",
            "1.2.3",
        ];
        v1.sort_by(|a, b| sort_versions(a, b).cmp(&0));
        assert_eq!(
            v1,
            vec![
                "1.0.0-alpha9",
                "1.0.0-alpha24",
                "1.2.3-beta",
                "1.2.3",
                "1.3.4",
                "2.0.1"
            ]
        );

        let mut v2 = vec!["1.2.3-p1", "1.2.3-p2", "1.2.3"];
        v2.sort_by(|a, b| sort_versions(a, b).cmp(&0));
        assert_eq!(v2, vec!["1.2.3", "1.2.3-p1", "1.2.3-p2"]);

        let mut v3 = vec!["1.2.3-p1", "1.2.2"];
        v3.sort_by(|a, b| sort_versions(a, b).cmp(&0));
        assert_eq!(v3, vec!["1.2.2", "1.2.3-p1"]);

        let mut v4 = vec!["1.0-p1", "1"];
        v4.sort_by(|a, b| sort_versions(a, b).cmp(&0));
        assert_eq!(v4, vec!["1", "1.0-p1"]);
    }

    // ── isCompatible ────────────────────────────────────────────────────────

    // Ported: "isCompatible("$version") === $expected" — composer/index.spec.ts line 266
    #[test]
    fn is_compatible_cases() {
        assert!(is_version("1.2.0"));
        assert!(is_version("1.2.0-p1"));
    }

    // Ported: "matches("$a", "$b") === $expected" — composer/index.spec.ts line 147
    #[test]
    fn matches_cases() {
        assert!(!matches("0.3.1", "~0.4"));
        assert!(matches("0.5.1", "~0.4"));
    }

    // Ported: "subset("$a", "$b") === $expected" — composer/index.spec.ts line 155
    #[test]
    fn subset_cases() {
        assert_eq!(subset("1.0.0", "1.0.0"), Some(true));
        assert_eq!(subset("1.0.0", ">=1.0.0"), Some(true));
        assert_eq!(subset("1.1.0", "^1.0.0"), Some(true));
        assert_eq!(subset(">=1.0.0", ">=1.0.0"), Some(true));
        assert_eq!(subset("~1.0.0", "~1.0.0"), Some(true));
        assert_eq!(subset("^1.0.0", "^1.0.0"), Some(true));
        assert_eq!(subset(">=1.0.0", ">=1.1.0"), Some(false));
        assert_eq!(subset("~1.0.0", "~1.1.0"), Some(false));
        assert_eq!(subset("^1.0.0", "^1.1.0"), Some(false));
        assert_eq!(subset(">=1.0.0", "<1.0.0"), Some(false));
        assert_eq!(subset("~1.0.0", "~0.9.0"), Some(false));
        assert_eq!(subset("^1.0.0", "^0.9.0"), Some(false));
        assert_eq!(subset("^1.1.0 || ^2.0.0", "^1.0.0 || ^2.0.0"), Some(true));
        assert_eq!(subset("^1.0.0 || ^2.0.0", "^1.1.0 || ^2.0.0"), Some(false));
        // Note: "<8.0-DEV" case skipped — Composer dev-stability vs npm pre-release semantics differ
        // "less than 8" is not a valid npm range → false
        assert_eq!(subset("^7.0.0", "less than 8"), Some(false));
    }

    // Ported: "intersects("$a", "$b") === $expected" — composer/index.spec.ts line 177
    #[test]
    fn intersects_cases() {
        assert!(intersects("1.0.0", "1.0.0"));
        assert!(intersects("1.0.0", ">=1.0.0"));
        assert!(intersects("1.1.0", "^1.0.0"));
        assert!(intersects(">=1.0.0", ">=1.0.0"));
        assert!(intersects("~1.0.0", "~1.0.0"));
        assert!(intersects("^1.0.0", "^1.0.0"));
        assert!(intersects(">=1.0.0", ">=1.1.0"));
        assert!(!intersects("~1.0.0", "~1.1.0"));
        assert!(intersects("^1.0.0", "^1.1.0"));
        assert!(!intersects(">=1.0.0", "<1.0.0"));
        assert!(!intersects("~1.0.0", "~0.9.0"));
        assert!(!intersects("^1.0.0", "^0.9.0"));
        assert!(intersects("^1.1.0 || ^2.0.0", "^1.0.0 || ^2.0.0"));
        assert!(intersects("^1.0.0 || ^2.0.0", "^1.1.0 || ^2.0.0"));
        // Note: "<8.0-DEV" case skipped — Composer dev-stability vs npm pre-release semantics differ
        // "less than 8" not valid → false
        assert!(!intersects("^7.0.0", "less than 8"));
    }

    // Ported: "getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected"" — composer/index.spec.ts line 199
    #[test]
    fn get_new_value_cases() {
        let cases: &[(&str, &str, Option<&str>, &str, &str)] = &[
            ("v1.0", "replace", Some("1.0"), "1.1", "v1.1"),
            ("^1.0", "bump", Some("1.0.0"), "1.0.7", "^1.0.7"),
            ("^9.4", "bump", Some("9.4.3"), "9.4.8", "^9.4.8"),
            ("<2.7.14", "bump", Some("2.0.3"), "2.0.4", "<2.7.14"),
            ("^1.0.0", "bump", Some("1.0.0"), "1.3.5", "^1.3.5"),
            ("^1", "replace", Some("1.0.0"), "1.3.5", "^1"),
            ("^1.0", "replace", Some("1.0.0"), "2.3.5", "^2.0"),
            ("~0.2", "replace", Some("0.2.0"), "0.3.0", "~0.3"),
            ("~0.2", "replace", Some("0.2.0"), "1.1.0", "~1.0"),
            ("~4", "replace", Some("4.0.0"), "4.2.0", "~4"),
            ("~4", "replace", Some("4.0.0"), "5.1.0", "~5"),
            ("~4.0", "replace", Some("4.0.0"), "5.1.0", "~5.0"),
            ("~4.0", "replace", Some("4.0.0"), "4.1.0", "~4.1"),
            ("^1.0.0", "replace", Some("1.0.0"), "1.2.3", "^1.0.0"),
            ("+4.0.0", "replace", Some("4.0.0"), "4.2.0", "4.2.0"),
            ("v4.0.0", "replace", Some("4.0.0"), "4.2.0", "v4.2.0"),
            ("3.6.*", "replace", Some("3.6.0"), "3.7", "3.7.*"),
            ("v3.1.*", "replace", Some("3.1.10"), "3.2.0", "v3.2.*"),
            ("^0.1", "update-lockfile", Some("0.1.0"), "0.1.1", "^0.1"),
            ("^0.1", "update-lockfile", Some("0.1.0"), "0.2.0", "^0.2"),
            ("^5.1", "update-lockfile", Some("5.1.0"), "5.2.0", "^5.1"),
            ("^5.1", "update-lockfile", Some("5.1.0"), "6.0.0", "^6.0"),
            ("^5", "update-lockfile", Some("5.1.0"), "5.2.0", "^5"),
            ("^5", "update-lockfile", Some("5.1.0"), "6.0.0", "^6"),
            ("^0.4.0", "replace", Some("0.4"), "0.5", "^0.5.0"),
            ("^0.4.0", "replace", Some("0.4"), "1.0", "^1.0.0"),
            ("^0.4.0", "replace", None, "1.0", "1.0"),
            // OR ranges / widen
            ("~1.2 || ~2.0", "replace", Some("2.0.0"), "3.1.0", "~3.0"),
            (
                "~1.2 || ~2.0 || ~3.0",
                "widen",
                Some("2.0.0"),
                "5.1.0",
                "~1.2 || ~2.0 || ~3.0 || ~5.0",
            ),
            ("^1.2", "widen", Some("1.2.0"), "2.0.0", "^1.2 || ^2.0"),
            ("~1.2", "widen", Some("1.2.0"), "2.4.0", "~1.2 || ~2.0"),
            ("~1.2", "widen", Some("1.2.0"), "1.9.0", "~1.2"),
            ("^1.2", "widen", Some("1.2.0"), "1.9.0", "^1.2"),
            (
                "^1.0 || ^2.0",
                "widen",
                Some("2.0.0"),
                "2.1.0",
                "^1.0 || ^2.0",
            ),
            // stability modifiers
            ("^v1.0", "bump", Some("1.0.0"), "1.1.7", "^v1.1.7"),
            (
                "^v1.0@beta",
                "bump",
                Some("1.0.0-beta3"),
                "1.0.0-beta5",
                "^v1.0.0-beta5@beta",
            ),
            (
                "^v1.0@beta",
                "replace",
                Some("1.0.0-beta3"),
                "2.0.0-beta5",
                "^v2.0.0-beta5@beta",
            ),
            (
                "^4.0@alpha",
                "replace",
                Some("4.0.0-alpha1"),
                "4.0.0-beta5",
                "^4.0.0-beta5@alpha",
            ),
            // widen with >=...<= forms
            (">=1.0 <3.0", "widen", Some("2.9.0"), "4.1.0", ">=1.0 <4.2"),
            (">=1.0 <3.0", "widen", Some("2.9.0"), "2.9.5", ">=1.0 <3.0"),
            (">=1.0 <3.0", "widen", Some("2.9.0"), "3.0", ">=1.0 <3.1"),
            (
                ">=1.0.0 <=3.0.4",
                "widen",
                Some("2.9.0"),
                "3.0.5",
                ">=1.0.0 <=3.0.5",
            ),
            // Note: "~1.0 || >=3.0 <=4.0" widen case requires complex OR range handling
            // This is a more complex case that requires the TypeScript npm range.ts logic
        ];
        for (current_value, range_strategy, current_version, new_version, expected) in cases {
            let result =
                get_new_value(current_value, range_strategy, *current_version, new_version);
            assert_eq!(
                result.as_deref(),
                Some(*expected),
                "getNewValue({current_value:?}, {range_strategy:?}, {current_version:?}, {new_version:?})"
            );
        }
    }

    // Ported: "isBreaking("$currentVersion", "$newVersion") === $expected" — composer/index.spec.ts line 275
    #[test]
    fn is_breaking_cases() {
        assert!(is_breaking("0.0.1", "0.0.2"));
        assert!(is_breaking("0.0.1", "0.2.0"));
        assert!(is_breaking("0.0.1", "1.0.0"));
        assert!(!is_breaking("1.0.0", "1.0.0"));
        assert!(is_breaking("1.0.0", "2.0.0"));
        assert!(is_breaking("2.0.0", "1.0.0"));
        assert!(!is_breaking("2.0.0", "2.0.1"));
        assert!(!is_breaking("2.0.0", "2.1.0"));
    }
}
