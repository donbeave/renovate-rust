use std::sync::LazyLock;

use regex::Regex;
use semver::{Version, VersionReq};

use super::loose;

struct ConanOptions {
    loose: bool,
    include_prerelease: bool,
}

fn get_options(input: &str) -> ConanOptions {
    let include_prerelease =
        input.contains("include_prerelease") && !input.contains("include_prerelease=False");
    let loose = input.contains("loose=True") || !input.contains("loose=False");
    ConanOptions {
        loose,
        include_prerelease,
    }
}

fn clean_version(input: &str) -> String {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"include_prerelease=|include_prerelease|loose=|,|\[|\]|"|True|False"#).unwrap()
    });
    RE.replace_all(input, "").trim().to_owned()
}

fn coerce_version(s: &str) -> Option<Version> {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(\d+)(?:\.(\d+)(?:\.(\d+))?)?").unwrap());
    let caps = RE.captures(s)?;
    let major: u64 = caps[1].parse().ok()?;
    let minor: u64 = caps
        .get(2)
        .and_then(|m| m.as_str().parse().ok())
        .unwrap_or(0);
    let patch: u64 = caps
        .get(3)
        .and_then(|m| m.as_str().parse().ok())
        .unwrap_or(0);
    Some(Version::new(major, minor, patch))
}

fn parse_version_loose(s: &str) -> Option<Version> {
    let s = s
        .trim()
        .trim_start_matches('v')
        .trim_start_matches('=')
        .trim();
    Version::parse(s).ok().or_else(|| {
        // normalize: strip leading zeros, add hyphen before alpha suffix
        let normalized = normalize_version_str(s);
        Version::parse(&normalized).ok()
    })
}

fn normalize_version_str(s: &str) -> String {
    // Handles: X.Y[.Z][.extra...][-(pre)]|[+(build)]
    static RE_PARTS: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"^(\d+)\.(\d+)(?:\.(\d+))?(?:\.[\d.]+)?(?:-([a-zA-Z0-9._-]*))?(?:\+[a-zA-Z0-9._-]*)?\s*$",
        )
        .unwrap()
    });
    let s = s.trim();
    if let Some(caps) = RE_PARTS.captures(s) {
        let major: u64 = caps[1].parse().unwrap_or(0);
        let minor: u64 = caps[2].parse().unwrap_or(0);
        let patch: u64 = caps
            .get(3)
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(0);
        let pre = caps.get(4).map(|m| m.as_str()).unwrap_or("");
        if pre.is_empty() {
            format!("{major}.{minor}.{patch}")
        } else {
            format!("{major}.{minor}.{patch}-{pre}")
        }
    } else {
        s.to_owned()
    }
}

fn normalize_range_loose(s: &str) -> String {
    static RE_LEADING: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b0+([1-9]\d*)").unwrap());
    static RE_PREREL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d)([a-zA-Z])").unwrap());
    let s = RE_LEADING.replace_all(s, "$1");
    let s = RE_PREREL.replace_all(&s, "$1-$2");
    s.into_owned()
}

// Rust semver requires comma-separated AND comparators; convert "1.0 <2.0" → "1.0,<2.0".
fn normalize_and_range(s: &str) -> String {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(\d)\s+(>=?|<=?|~>|~=|[~^=])").unwrap());
    RE.replace_all(s, "$1,$2").into_owned()
}

// Returns true for versions like "2.0.0b1", "1.0a2" where prerelease is embedded
// without a hyphen separator (Python-style).
fn has_inline_prerelease(s: &str) -> bool {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d+(\.\d+)*[a-zA-Z]").unwrap());
    RE.is_match(s)
}

// Split an AND range into individual comparator parts, grouping each operator
// with its version even when there's a space: ">= 0.0.1 < 1" → [">= 0.0.1", "< 1"].
//
// Rust's regex crate has no lookahead, so we tokenize manually: if a token is a
// pure operator character sequence (e.g. ">=" or "<"), merge it with the next token.
fn split_and_parts(range: &str) -> Vec<String> {
    let tokens: Vec<&str> = range.split_whitespace().collect();
    let mut parts = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        let tok = tokens[i];
        let is_pure_op = !tok.is_empty() && tok.chars().all(|c| "<>=~^".contains(c));
        if is_pure_op && i + 1 < tokens.len() {
            parts.push(format!("{} {}", tok, tokens[i + 1]));
            i += 2;
        } else {
            parts.push(tok.to_owned());
            i += 1;
        }
    }
    parts
}

// Convert npm-style hyphen ranges "1.0.0 - 2.0.0" → ">=1.0.0 <=2.0.0".
fn normalize_hyphen_range(s: &str) -> String {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^([^\s]+)\s+-\s+([^\s]+)$").unwrap());
    if let Some(caps) = RE.captures(s.trim()) {
        format!(">={} <={}", &caps[1], &caps[2])
    } else {
        s.to_owned()
    }
}

fn preprocess_single_range(s: &str) -> String {
    let s = s.trim();
    if s.is_empty() || s == "||" || s == "*" || s == "x" || s == "X" {
        return "*".to_owned();
    }
    if s == ">=*" || s == ">=x" || s == ">=X" {
        return "*".to_owned();
    }
    // ~> → ~
    let s = s.replace("~>", "~");
    // ~= → ~
    let s = s.replace("~=", "~");
    // Plain dotted version (no operator, no wildcard, no spaces, e.g. "3.17.2") → exact match.
    // Rust semver treats bare versions as caret ranges; npm/conan treats them as exact.
    if s.chars()
        .next()
        .map(|c| c.is_ascii_digit())
        .unwrap_or(false)
        && s.contains('.')
        && !s.contains(' ')
        && !s.contains('*')
        && s != "x"
        && s != "X"
        && !s.starts_with(|c: char| "<>=~^".contains(c))
    {
        format!("={s}")
    } else {
        s
    }
}

#[expect(dead_code, reason = "Reserved for future range validation use")]
fn try_parse_range_part(s: &str) -> bool {
    let preprocessed = preprocess_single_range(s);
    if preprocessed == "*" {
        return true;
    }
    if VersionReq::parse(&preprocessed).is_ok() {
        return true;
    }
    let normalized = normalize_range_loose(&preprocessed);
    VersionReq::parse(&normalized).is_ok()
}

fn try_satisfies_range(version_str: &str, range: &str, opts: &ConanOptions) -> bool {
    let cv = version_str
        .trim()
        .trim_start_matches('v')
        .trim_start_matches('=')
        .trim();

    // OR split
    for part in range.split("||") {
        let part = part.trim();
        if part.is_empty() {
            return true; // empty OR part = * = match all
        }
        if try_satisfies_single(cv, part, opts) {
            return true;
        }
    }
    false
}

fn try_satisfies_single(version_str: &str, range: &str, opts: &ConanOptions) -> bool {
    let preprocessed = preprocess_single_range(range);
    if preprocessed == "*" {
        return Version::parse(version_str)
            .map(|v| opts.include_prerelease || v.pre.is_empty())
            .unwrap_or(true);
    }

    let Some(v) = parse_version_loose(version_str).or_else(|| {
        if opts.loose {
            coerce_version(version_str)
        } else {
            None
        }
    }) else {
        return false;
    };

    // No prerelease coercion here — that belongs in matches_with_opts only.
    let v_for_check = v;

    fn check(req_str: &str, v: &Version) -> Option<bool> {
        VersionReq::parse(req_str).ok().map(|req| req.matches(v))
    }

    // 1. Try direct parse.
    if let Some(result) = check(&preprocessed, &v_for_check) {
        return result;
    }
    // 2. Normalize leading zeros / no-hyphen prerelease, try again.
    let normalized = normalize_range_loose(&preprocessed);
    if let Some(result) = check(&normalized, &v_for_check) {
        return result;
    }
    // 3. Hyphen ranges "1.0.0 - 2.0.0" → ">=1.0.0 <=2.0.0", then comma-normalize.
    let hyphen = normalize_hyphen_range(&preprocessed);
    if hyphen != preprocessed {
        let comma = normalize_and_range(&hyphen);
        if let Some(result) = check(&comma, &v_for_check) {
            return result;
        }
    }
    // 4. Rust semver requires commas between AND comparators; convert spaces.
    let comma_range = normalize_and_range(&preprocessed);
    if comma_range != preprocessed {
        if let Some(result) = check(&comma_range, &v_for_check) {
            return result;
        }
        let comma_normalized = normalize_range_loose(&comma_range);
        if let Some(result) = check(&comma_normalized, &v_for_check) {
            return result;
        }
    }
    false
}

fn make_version_strict(version: &str, opts: &ConanOptions) -> Option<Version> {
    let v = version
        .trim()
        .trim_start_matches('=')
        .trim_start_matches('v')
        .trim();
    let parsed = Version::parse(v).ok()?;
    if !parsed.pre.is_empty() && !opts.include_prerelease {
        // prerelease starting with digit: use base version
        let first_char = parsed.pre.as_str().chars().next()?;
        if first_char.is_ascii_digit() {
            return Version::parse(&format!(
                "{}.{}.{}",
                parsed.major, parsed.minor, parsed.patch
            ))
            .ok();
        }
        // prerelease starting with alpha: returns false in TS (makeVersion returns false)
        // We return a sentinel: use a special None here but track that it's "prerelease excluded"
        return None;
    }
    Some(parsed)
}

pub fn is_version(input: &str) -> bool {
    if input.is_empty() || input.contains('[') {
        return false;
    }
    let opts = get_options(input);
    let version = clean_version(input);
    if version.is_empty() {
        return false;
    }

    if opts.loose {
        if loose::is_version(&version) {
            return true;
        }
        if coerce_version(&version).is_some() {
            return true;
        }
        return false;
    }

    // Strict mode
    // makeVersion !== null (including the "prerelease excluded" false case)
    let v = version
        .trim()
        .trim_start_matches('=')
        .trim_start_matches('v')
        .trim();
    Version::parse(v).is_ok()
}

fn is_valid_range_str(cleaned: &str, opts: &ConanOptions) -> bool {
    if cleaned.is_empty() || cleaned == "||" {
        return true; // empty = * = valid
    }
    if cleaned == "*" || cleaned == "x" || cleaned == "X" || cleaned == ">=*" {
        return true;
    }

    // Split on ||
    let parts: Vec<&str> = cleaned.split("||").map(str::trim).collect();
    for part in &parts {
        if part.is_empty() {
            continue;
        }
        if !is_valid_range_part(part, opts) {
            return false;
        }
    }
    true
}

fn is_valid_range_part(s: &str, opts: &ConanOptions) -> bool {
    let preprocessed = preprocess_single_range(s);
    if preprocessed == "*" {
        return true;
    }

    // For strict mode: reject leading zeros and no-hyphen prerelease in ranges
    if !opts.loose {
        // Check for leading zeros in version numbers
        static RE_LEADING: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b0+[1-9]").unwrap());
        if RE_LEADING.is_match(s) {
            return false;
        }
        // Check for prerelease without hyphen (e.g. ~1.2.3beta)
        static RE_NOHYPHEN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d[a-zA-Z]").unwrap());
        if RE_NOHYPHEN.is_match(s) {
            return false;
        }
    }

    if VersionReq::parse(&preprocessed).is_ok() {
        return true;
    }
    // Hyphen range support ("1.0.0 - 2.0.0" → ">=1.0.0,<=2.0.0").
    let hyphen = normalize_hyphen_range(&preprocessed);
    if hyphen != preprocessed {
        let comma = normalize_and_range(&hyphen);
        if VersionReq::parse(&comma).is_ok() {
            return true;
        }
    }
    // Space-separated AND comparators need comma conversion.
    let comma = normalize_and_range(&preprocessed);
    if comma != preprocessed && VersionReq::parse(&comma).is_ok() {
        return true;
    }
    if opts.loose {
        let normalized = normalize_range_loose(&preprocessed);
        if VersionReq::parse(&normalized).is_ok() {
            return true;
        }
        let comma_norm = normalize_and_range(&normalized);
        if comma_norm != normalized && VersionReq::parse(&comma_norm).is_ok() {
            return true;
        }
    }
    false
}

pub fn is_valid(input: &str) -> bool {
    let cleaned = clean_version(input);
    let opts = get_options(input);

    // Try as single version first
    if !cleaned.contains('<')
        && !cleaned.contains('>')
        && !cleaned.contains('~')
        && !cleaned.contains('^')
        && !cleaned.contains(' ')
        && !cleaned.contains("||")
    {
        if opts.loose {
            if parse_version_loose(&cleaned).is_some() {
                return true;
            }
        } else if make_version_strict(&cleaned, &opts).is_some() {
            return true;
        }
    }

    // Try as range
    is_valid_range_str(&cleaned, &opts)
}

pub fn is_stable(version: &str) -> bool {
    let cleaned = clean_version(version);
    let opts = get_options(version);
    // If include_prerelease=True → always stable
    if opts.include_prerelease {
        return true;
    }
    // Parse and check for prerelease
    if let Ok(v) = Version::parse(&cleaned) {
        return v.pre.is_empty();
    }
    // For loose versions like "19.00" that aren't standard semver
    if let Some(v) = parse_version_loose(&cleaned) {
        return v.pre.is_empty();
    }
    true
}

pub fn matches(version: &str, range: &str) -> bool {
    // If both are single versions → true
    if is_version(version) && is_version(range) {
        return true;
    }
    let cleaned_version = clean_version(version);
    let opts = get_options(range);
    let clean_range = clean_version(range);
    matches_with_opts(&cleaned_version, &clean_range, &opts)
}

fn matches_with_opts(version: &str, clean_range: &str, opts: &ConanOptions) -> bool {
    let mut cv = version.to_owned();
    // If includePrerelease and version has prerelease: coerce to base version
    if opts.include_prerelease
        && let Ok(v) = Version::parse(version)
        && !v.pre.is_empty()
    {
        cv = format!("{}.{}.{}", v.major, v.minor, v.patch);
    }
    try_satisfies_range(&cv, clean_range, opts)
}

pub fn is_compatible(version: &str, range: &str) -> bool {
    if is_version(version) && is_version(range) {
        return true;
    }
    let opts = get_options(range);
    let cv = clean_version(version);

    // makeVersion: check if version is valid (with options)
    // In TypeScript, makeVersion returns false (not null) for prerelease with includePrerelease=false
    // if false (not null): isCompatible returns false
    // We need: if version has prerelease and !include_prerelease → return false
    let v_parsed = Version::parse(&cv).ok();
    if let Some(ref v) = v_parsed
        && !v.pre.is_empty()
        && !opts.include_prerelease
    {
        let first_char = v.pre.as_str().chars().next();
        if first_char.map(|c| !c.is_ascii_digit()).unwrap_or(false) {
            // prerelease starts with alpha → makeVersion returns false → isCompatible = false
            return false;
        }
    }

    // makeVersion is truthy → check !isLessThanRange
    if v_parsed.is_some() || coerce_version(&cv).is_some() {
        return !is_less_than_range(version, range);
    }

    // makeVersion = null → return false
    false
}

fn lower_bound_of_range_part(s: &str) -> Option<Version> {
    let s = s.trim();
    // No lower bound for upper-bound-only operators.
    if s.starts_with("<=") || (s.starts_with('<') && !s.starts_with("<=")) {
        return None;
    }
    // Wildcards have no specific lower bound.
    if s == "*" || s == "x" || s == "X" || s.ends_with(".*") || s.ends_with(".x") {
        return None;
    }
    let op_len = if s.starts_with(">=") {
        2
    } else if s.starts_with('>') {
        1
    } else if s.starts_with("~>") || s.starts_with("~=") {
        2
    } else if s.starts_with('~') || s.starts_with('^') || s.starts_with('=') {
        1
    } else {
        0 // plain version: lower bound = the version itself
    };
    let rest = s[op_len..].trim();
    parse_version_loose(rest)
}

fn is_lower_bound_strict(s: &str) -> bool {
    s.trim().starts_with('>') && !s.trim().starts_with(">=")
}

pub fn is_less_than_range(version: &str, range: &str) -> bool {
    let cv = clean_version(version);
    let cr = clean_version(range);
    let opts = get_options(range);

    // If satisfies range → not less than
    if try_satisfies_range(&cv, &cr, &opts) {
        return false;
    }

    // Try to parse version
    let Some(v) = parse_version_loose(&cv) else {
        return false;
    };

    // For each OR part, check if there's a part with no lower bound
    // If any OR part has no lower bound (like <X), version can't be less than that part
    // → return false
    let or_parts: Vec<&str> = cr.split("||").map(str::trim).collect();
    for or_part in &or_parts {
        if or_part.is_empty() {
            return false; // empty = * = no lower bound
        }
        // Split AND parts — keep ">= X" as one token.
        let and_parts: Vec<String> = split_and_parts(or_part);
        // Find the highest lower bound
        let mut found_lower_bound = false;
        let mut all_above_lower_bound = false;
        for and_part in &and_parts {
            if let Some(lb) = lower_bound_of_range_part(and_part) {
                found_lower_bound = true;
                let is_strict = is_lower_bound_strict(and_part);
                // Is version >= lb (with strict: strictly > lb means NOT less than range)
                let v_gt_lb = v > lb || (v == lb && !is_strict);
                if v_gt_lb {
                    all_above_lower_bound = true;
                }
            }
        }
        if !found_lower_bound {
            // This OR part has no lower bound (e.g., <X)
            return false;
        }
        if all_above_lower_bound {
            return false;
        }
    }

    true
}

pub fn get_major(version: &str) -> Option<u64> {
    let cv = clean_version(version);
    // Try standard semver
    if let Ok(v) = Version::parse(&cv) {
        return Some(v.major);
    }
    let v = parse_version_loose(&cv)?;
    Some(v.major)
}

pub fn get_minor(version: &str) -> Option<u64> {
    let cv = clean_version(version);
    if let Ok(v) = Version::parse(&cv) {
        return Some(v.minor);
    }
    let parts: Vec<&str> = cv.split('.').collect();
    if parts.len() >= 2 {
        let minor_str = parts[1];
        let digits: String = minor_str
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect();
        if !digits.is_empty() {
            return digits.parse().ok();
        }
    }
    None
}

pub fn get_patch(version: &str) -> Option<u64> {
    let cv = clean_version(version);
    // parse_version_loose handles 4-component versions (4.1.3.2 → patch=3).
    let v = parse_version_loose(&cv)?;
    Some(v.patch)
}

pub fn equals(version: &str, other: &str) -> bool {
    let cv = clean_version(version);
    let co = clean_version(other);
    let opts = ConanOptions {
        loose: true,
        include_prerelease: true,
    };

    let v1 = parse_version_loose(&cv);
    let v2 = parse_version_loose(&co);

    if let (Some(a), Some(b)) = (v1, v2) {
        // Compare ignoring build metadata
        a.major == b.major && a.minor == b.minor && a.patch == b.patch && a.pre == b.pre
    } else {
        loose::equals(&cv, &co) || try_satisfies_range(&cv, &co, &opts)
    }
}

pub fn is_greater_than(version: &str, other: &str) -> bool {
    let cv = clean_version(version);
    let co = clean_version(other);

    let v1 = parse_version_loose(&cv);
    let v2 = parse_version_loose(&co);

    if let (Some(a), Some(b)) = (v1, v2) {
        a > b
    } else {
        loose::is_greater_than(&cv, &co)
    }
}

pub fn sort_versions(version: &str, other: &str) -> i32 {
    let cv = clean_version(version);
    let co = clean_version(other);

    let v1 = parse_version_loose(&cv);
    let v2 = parse_version_loose(&co);

    if let (Some(a), Some(b)) = (v1, v2) {
        match a.cmp(&b) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    } else if loose::is_greater_than(&cv, &co) {
        1
    } else if loose::equals(&cv, &co) {
        0
    } else {
        -1
    }
}

pub fn get_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    find_satisfying(versions, range, true)
}

pub fn min_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    find_satisfying(versions, range, false)
}

fn find_satisfying<'a>(versions: &[&'a str], range: &str, want_max: bool) -> Option<&'a str> {
    let opts = get_options(range);
    let clean_range = clean_version(range);

    let mut best_idx: Option<usize> = None;
    let mut best_v: Option<Version> = None;

    for (idx, &ver) in versions.iter().enumerate() {
        let cv_str = clean_version(ver);
        let parsed_proper = parse_version_loose(&cv_str);
        // Versions like "2.0.0b1" can't be parsed by parse_version_loose but look like
        // prereleases; coerce_version would strip the pre marker giving a false positive.
        let is_inline_pre = parsed_proper.is_none() && has_inline_prerelease(&cv_str);
        let parsed = parsed_proper.or_else(|| coerce_version(&cv_str));
        if let Some(v) = parsed {
            // Skip inline-prerelease versions (e.g. "2.0.0b1") when not including prereleases.
            if is_inline_pre && !opts.include_prerelease {
                continue;
            }
            // Check stability per makeVersion logic
            if !opts.include_prerelease && !v.pre.is_empty() {
                let first_char = v.pre.as_str().chars().next();
                if first_char.map(|c| !c.is_ascii_digit()).unwrap_or(false) {
                    // Non-numeric prerelease → skip (makeVersion returns false)
                    continue;
                }
                // Numeric prerelease → strip it, check if base satisfies
                let base = Version::new(v.major, v.minor, v.patch);
                if !try_satisfies_single(
                    &format!("{}.{}.{}", base.major, base.minor, base.patch),
                    &clean_range,
                    &opts,
                ) {
                    continue;
                }
                // Satisfies: track this version
                let is_better = match &best_v {
                    None => true,
                    Some(bv) => {
                        if want_max {
                            base > *bv
                        } else {
                            base < *bv
                        }
                    }
                };
                if is_better {
                    best_idx = Some(idx);
                    best_v = Some(base);
                }
                continue;
            }

            // When include_prerelease=true and version has prerelease: coerce for
            // range check (matchesWithOptions semantics), but compare by original version.
            let check_str = if opts.include_prerelease && !v.pre.is_empty() {
                format!("{}.{}.{}", v.major, v.minor, v.patch)
            } else {
                cv_str.clone()
            };
            if !try_satisfies_range(&check_str, &clean_range, &opts) {
                continue;
            }

            let is_better = match &best_v {
                None => true,
                Some(bv) => {
                    if want_max {
                        v > *bv
                    } else {
                        v < *bv
                    }
                }
            };
            if is_better {
                best_idx = Some(idx);
                best_v = Some(v);
            }
        }
    }

    best_idx.map(|i| versions[i])
}

// ── getNewValue helpers ──────────────────────────────────────────────────────

fn inc_patch_str(version: &str) -> Option<String> {
    let v = Version::parse(version.trim_start_matches('v')).ok()?;
    Some(format!("{}.{}.{}", v.major, v.minor, v.patch + 1))
}

fn replace_range(clean_range: &str, new_version: &str) -> Option<String> {
    let cv = clean_range.trim();
    let nv = new_version.trim_start_matches('v');
    let new_v = Version::parse(nv)
        .ok()
        .or_else(|| parse_version_loose(nv))
        .or_else(|| coerce_version(nv));
    let new_major = new_v.as_ref().map(|v| v.major).unwrap_or(0);
    let new_minor = new_v.as_ref().map(|v| v.minor).unwrap_or(0);
    let new_patch = new_v.as_ref().map(|v| v.patch).unwrap_or(0);
    let suffix = new_v
        .as_ref()
        .filter(|v| !v.pre.is_empty())
        .map(|v| {
            format!(
                "-{}",
                v.pre.as_str().split('.').next().unwrap_or(v.pre.as_str())
            )
        })
        .unwrap_or_default();

    // ~> operator
    if cv.starts_with("~>") {
        return Some(format!("~> {new_major}.{new_minor}.0"));
    }
    // ~= operator
    if let Some(after) = cv.strip_prefix("~=") {
        let after = after.trim();
        let dots = after.matches('.').count();
        return Some(if dots == 0 {
            format!("~={new_major}")
        } else if dots == 1 {
            format!("~={new_major}.{new_minor}")
        } else {
            format!("~={new_major}.{new_minor}.{new_patch}")
        });
    }
    // ~ operator (not ~> or ~=) — TypeScript always returns 3-component when no suffix.
    if let Some(after_tilde) = cv.strip_prefix('~') {
        let op = if cv.starts_with("~ ") { "~ " } else { "~" };
        if !suffix.is_empty() {
            return Some(format!("{op}{new_major}.{new_minor}.{new_patch}{suffix}"));
        }
        let after = after_tilde.trim_start_matches(' ');
        let dots = after.matches('.').count();
        return Some(if dots == 0 {
            format!("{op}{new_major}")
        } else {
            format!("{op}{new_major}.{new_minor}.0")
        });
    }
    // = operator (but not >= or <=)
    if cv.starts_with('=') && !cv.starts_with(">=") {
        return Some(format!("={nv}"));
    }
    // <= operator
    if cv.starts_with("<=") {
        let sep = if cv.starts_with("<= ") { "<= " } else { "<=" };
        let after = cv.trim_start_matches("<=").trim();
        let dots = after.matches('.').count();
        return Some(if !suffix.is_empty() || dots >= 2 {
            format!("{sep}{nv}")
        } else if dots == 1 {
            format!("{sep}{new_major}.{new_minor}")
        } else {
            format!("{sep}{new_major}")
        });
    }
    // < operator
    if cv.starts_with('<') {
        let after = cv.trim_start_matches('<').trim();
        let has_space = cv.starts_with("< ");
        let sp = if has_space { "< " } else { "<" };
        let dots = after.matches('.').count();
        return Some(if cv.ends_with(".0.0") {
            format!("{sp}{}.0.0", new_major + 1)
        } else if dots >= 2 {
            format!("{sp}{}", inc_patch_str(nv)?)
        } else if dots == 1 {
            format!("{sp}{new_major}.{}", new_minor + 1)
        } else {
            format!("{sp}{}", new_major + 1)
        });
    }
    // >= operator
    if cv.starts_with(">=") {
        let sep = if cv.starts_with(">= ") { ">= " } else { ">=" };
        return Some(format!("{sep}{nv}"));
    }
    // > operator
    if cv.starts_with('>') {
        let has_space = cv.starts_with("> ");
        let after = cv.trim_start_matches('>').trim();
        let dots = after.matches('.').count();
        return Some(if cv.ends_with(".0.0") {
            let sp = if has_space { "> " } else { ">" };
            format!("{sp}{}.0.0", new_major + 1)
        } else if dots >= 2 {
            format!(">{new_major}.{new_minor}.{new_patch}")
        } else if dots == 1 {
            format!(">{new_major}.{new_minor}")
        } else {
            format!(">{new_major}")
        });
    }

    // No operator: wildcard or plain version
    if cv.ends_with(".*") || cv.ends_with(".x") {
        let wildcard = if cv.ends_with(".*") { ".*" } else { ".x" };
        let dots = cv.matches('.').count();
        return Some(if dots == 2 {
            format!("{new_major}.{new_minor}{wildcard}")
        } else {
            format!("{new_major}{wildcard}")
        });
    }
    if cv == "x" || cv == "X" {
        return Some(cv.to_owned());
    }

    // Plain version or number

    Some(match cv.split('.').count() {
        1 => format!("{new_major}"),
        2 => format!("{new_major}.{new_minor}"),
        _ => nv.to_owned(),
    })
}

fn bump_range_single(clean_range: &str, new_version: &str, _opts: &ConanOptions) -> Option<String> {
    let cv = clean_range.trim();
    let nv = new_version.trim_start_matches('v');
    let new_v = Version::parse(nv).ok();
    let new_major = new_v.as_ref().map(|v| v.major).unwrap_or(0);
    let new_minor = new_v.as_ref().map(|v| v.minor).unwrap_or(0);
    let suffix = new_v
        .as_ref()
        .filter(|v| !v.pre.is_empty())
        .map(|v| {
            format!(
                "-{}",
                v.pre.as_str().split('.').next().unwrap_or(v.pre.as_str())
            )
        })
        .unwrap_or_default();

    // Wildcard ranges: return unchanged
    if cv == "*" || cv == "x" || cv == "X" || cv == ">=*" {
        return Some(cv.to_owned());
    }

    // ~= operator
    if let Some(after) = cv.strip_prefix("~=") {
        let after = after.trim();
        let dots = after.matches('.').count();
        return Some(if dots == 0 {
            format!("~={new_major}")
        } else if dots == 1 {
            format!("~={new_major}.{new_minor}")
        } else {
            format!(
                "~={new_major}.{new_minor}.{}",
                new_v.as_ref().map(|v| v.patch).unwrap_or(0)
            )
        });
    }

    // ~ operator
    if cv.starts_with('~') && !cv.starts_with("~>") && !cv.starts_with("~=") {
        if !suffix.is_empty() {
            return Some(format!("~{nv}"));
        }
        let after = cv.trim_start_matches('~').trim_start_matches(' ');
        let dots = after.matches('.').count();
        return Some(if dots == 0 {
            format!("~{new_major}")
        } else if dots == 1 {
            format!("~{new_major}.{new_minor}")
        } else {
            let patch = new_v.as_ref().map(|v| v.patch).unwrap_or(0);
            format!("~{new_major}.{new_minor}.{patch}")
        });
    }

    // = operator
    if cv.starts_with('=') && !cv.starts_with(">=") {
        return Some(format!("={nv}"));
    }

    // >= operator
    if cv.starts_with(">=") {
        let sep = if cv.starts_with(">= ") { ">= " } else { ">=" };
        return Some(format!("{sep}{nv}"));
    }

    // < operator: return unchanged (don't bump upper bound in bump strategy)
    if cv.starts_with('<') {
        return Some(cv.to_owned());
    }

    // No operator: use replace
    replace_range(cv, new_version)
}

fn bump_range(clean_range: &str, new_version: &str, opts: &ConanOptions) -> Option<String> {
    let cv = clean_range.trim();

    // Wildcard check
    if cv == "*" || cv == "x" || cv == "X" || cv == ">=*" {
        return Some(cv.to_owned());
    }

    // OR ranges with no operators: use widen
    let or_parts: Vec<&str> = cv.split("||").map(str::trim).collect();
    let all_pinned = or_parts
        .iter()
        .all(|p| !p.starts_with(|c: char| "<>=~^".contains(c)));
    if or_parts.len() > 1 && all_pinned {
        return widen_range(cv, new_version, opts);
    }

    // Split AND parts — use split_and_parts to keep ">= 0.0.1" together.
    let and_parts: Vec<String> = if or_parts.len() == 1 {
        split_and_parts(cv)
    } else {
        vec![]
    };

    // If OR range with operators
    if or_parts.len() > 1 {
        let bumped_parts: Vec<String> = or_parts
            .iter()
            .map(|part| {
                if part.is_empty() {
                    return "".to_owned();
                }
                // Check if this part has operators
                let has_op = part.starts_with(|c: char| "<>=~^".contains(c));
                if !has_op {
                    // Pinned version: preserve
                    return (*part).to_owned();
                }
                // Handle multi-part AND within this OR part
                let and_sub = split_and_parts(part);
                if and_sub.len() == 1 {
                    let bumped = bump_range_single(part, new_version, opts);
                    if let Some(b) = &bumped
                        && try_satisfies_single(new_version, b, opts)
                    {
                        return b.clone();
                    }
                    replace_range(part, new_version).unwrap_or_else(|| (*part).to_owned())
                } else {
                    // AND sub-range
                    bump_and_range(&and_sub, new_version, opts)
                }
            })
            .collect();
        let result = bumped_parts
            .iter()
            .filter(|p| !p.is_empty())
            .cloned()
            .collect::<Vec<_>>()
            .join(" || ");
        return Some(result);
    }

    // Single or AND range
    if and_parts.len() <= 1 {
        let bumped = bump_range_single(cv, new_version, opts)?;
        // If the range didn't change (e.g. < operator returns unchanged): keep it.
        if bumped == cv {
            return Some(bumped);
        }
        let satisfies = try_satisfies_single(new_version, &bumped, opts);
        if satisfies {
            Some(bumped)
        } else {
            replace_range(cv, new_version)
        }
    } else {
        Some(bump_and_range(&and_parts, new_version, opts))
    }
}

fn bump_and_range(and_parts: &[String], new_version: &str, opts: &ConanOptions) -> String {
    let result: Vec<String> = and_parts
        .iter()
        .map(|part| {
            let part = part.trim();
            let bumped = bump_range_single(part, new_version, opts);
            if let Some(b) = bumped {
                let satisfies = try_satisfies_single(new_version, &b, opts);
                if satisfies {
                    return b;
                }
            }
            replace_range(part, new_version).unwrap_or_else(|| (*part).to_owned())
        })
        .collect();
    result.join(" ")
}

fn widen_range(clean_range: &str, new_version: &str, opts: &ConanOptions) -> Option<String> {
    let cv = clean_range.trim();

    // Wildcard
    if cv == "*" || cv == "x" || cv == "X" || cv == ">=*" {
        return Some(cv.to_owned());
    }

    // If newVersion already satisfies → return unchanged
    if try_satisfies_range(new_version, cv, opts) {
        return Some(cv.to_owned());
    }

    let nv_clean = new_version.trim_start_matches('v');
    let replaced_whole = replace_range(cv, new_version);

    // Find the last AND element — use split_and_parts to keep ">= X" together.
    let parts: Vec<String> = split_and_parts(cv);
    let last = parts.last().map(String::as_str).unwrap_or(cv);

    // If entire range starts with < (e.g. "<= 1.2.3"): replace the whole range.
    if cv.starts_with('<') {
        return replace_range(cv, new_version);
    }

    // If last element starts with <: replace upper bound
    if last.starts_with('<') {
        let replaced = replace_range(last, new_version)?;
        // Rebuild: everything before last + replaced
        let without_last = &parts[..parts.len() - 1];
        if without_last.is_empty() {
            return Some(replaced);
        }
        return Some(format!("{} {replaced}", without_last.join(" ")));
    }

    // If multiple AND parts and contains " - ": hyphen range
    if cv.contains(" - ") {
        let replaced_last =
            replaced_whole.unwrap_or_else(|| new_version.trim_start_matches('v').to_owned());
        // Split on ' - ', pop last, rejoin and append
        let segments: Vec<&str> = cv.split(" - ").collect();
        if segments.len() >= 2 {
            let without_last = &segments[..segments.len() - 1];
            return Some(format!("{} - {}", without_last.join(" - "), replaced_last));
        }
    }

    // If last has > operator and there are multiple parts: warn, return null
    if parts.len() > 1 && last.starts_with('>') {
        return None;
    }

    // Default: append || newVersion
    Some(format!("{cv} || {nv_clean}"))
}

pub fn get_new_value(
    current_value: &str,
    range_strategy: &str,
    _current_version: &str,
    new_version: &str,
) -> Option<String> {
    // Plain version (no brackets)
    if is_version(current_value) {
        return Some(new_version.trim_start_matches('v').to_owned());
    }

    let clean_range = clean_version(current_value);
    let opts = get_options(current_value);

    // Wildcard: return unchanged
    if clean_range == "*"
        || clean_range == "x"
        || clean_range == "X"
        || clean_range == ">=*"
        || clean_range == ">=x"
    {
        return Some(current_value.to_owned());
    }

    // Verify validRange is not * (also check semver)
    if let Ok(req) = VersionReq::parse(&clean_range) {
        // VersionReq::parse("*") returns STAR; check matches all
        if req == VersionReq::STAR {
            return Some(current_value.to_owned());
        }
    }

    let new_value = match range_strategy {
        "widen" => widen_range(&clean_range, new_version, &opts),
        "bump" => bump_range(&clean_range, new_version, &opts),
        _ => replace_range(&clean_range, new_version),
    };

    new_value.map(|nv| {
        // Replace cleanRange in currentValue with newValue
        current_value.replacen(&clean_range, &nv, 1)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isValid("$version") === $result" — lib/modules/versioning/conan/index.spec.ts line 5
    #[test]
    fn is_valid_matches_renovate_conan_index_spec() {
        let cases = vec![
            ("[1.2.3.4, loose=False]", false),
            ("[NOT VALID, loose=False]", false),
            ("[1.2, loose=False]", true),
            ("[1.a.2, loose=False]", false),
            ("[Infinity.NaN.Infinity, loose=False]", false),
            ("1.2.3.4", true),
            ("NOT VALID", false),
            ("1.2", true),
            ("1.a.2", false),
            ("", true),
            ("Infinity.NaN.Infinity", false),
            ("17.04.0", true),
            ("1.2.3", true),
            ("1.2.3-foo", true),
            ("[>1.1 <2.0]", true),
            ("1.2.3foo", true),
            ("[~1.2.3]", true),
            ("[^1.2.3]", true),
            ("1.x", true),
            ("[>1.2.3]", true),
            ("[>1.1 <2.1]", true),
            ("[~=3.0]", true),
            ("[>1.1 || 0.8]", true),
            ("[1.2.7 || >=1.2.9 <2.0.0]", true),
            ("[>1.1 <2.1, include_prerelease=True]", true),
            ("[~1.2.3, loose=False]", true),
            ("[~1.2.3, loose=False, include_prerelease=True]", true),
            ("renovatebot/renovate", false),
            ("renovatebot/renovate#main", false),
            ("https://github.com/renovatebot/renovate.git", false),
            ("[>=01.02.03]", true),
            ("[~1.02.03beta]", true),
            (
                r#"[">1.0.0 <1.0.2", loose=False, include_prerelease=True]"#,
                true,
            ),
            ("[1.0.0 - 2.0.0, loose=False]", true),
            ("[1.0.0, loose=False]", true),
            ("[>=*, loose=False]", true),
            ("[, loose=False]", true),
            ("[*, loose=False]", true),
            ("[>=1.0.0, loose=False]", true),
            ("[>1.0.0, loose=False]", true),
            ("[<=2.0.0, loose=False]", true),
            ("[1, loose=False]", true),
            ("[<2.0.0, loose=False]", true),
            ("[>= 1.0.0, loose=False]", true),
            ("[>=  1.0.0, loose=False]", true),
            ("[>=   1.0.0, loose=False]", true),
            ("[> 1.0.0, loose=False]", true),
            ("[>  1.0.0, loose=False]", true),
            ("[<=   2.0.0, loose=False]", true),
            ("[<= 2.0.0, loose=False]", true),
            ("[<=  2.0.0, loose=False]", true),
            ("[<    2.0.0, loose=False]", true),
            ("[>=0.1.97, loose=False]", true),
            ("[0.1.20 || 1.2.4, loose=False]", true),
            ("[>=0.2.3 || <0.0.1, loose=False]", true),
            ("[||, loose=False]", true),
            ("[2.x.x, loose=False]", true),
            ("[1.2.x, loose=False]", true),
            ("[1.2.x || 2.x, loose=False]", true),
            ("[x, loose=False]", true),
            ("[2.*.*, loose=False]", true),
            ("[1.2.*, loose=False]", true),
            ("[1.2.* || 2.*, loose=False]", true),
            ("[*, loose=False]", true),
            ("[2, loose=False]", true),
            ("[2.3, loose=False]", true),
            ("[~2.4, loose=False]", true),
            ("[~>3.2.1, loose=False]", true),
            ("[~1, loose=False]", true),
            ("[~>1, loose=False]", true),
            ("[~> 1, loose=False]", true),
            ("[~1.0, loose=False]", true),
            ("[~ 1.0, loose=False]", true),
            ("[^0, loose=False]", true),
            ("[^ 1, loose=False]", true),
            ("[^0.1, loose=False]", true),
            ("[^1.0, loose=False]", true),
            ("[^1.2, loose=False]", true),
            ("[^0.0.1, loose=False]", true),
            ("[^0.0.1-beta, loose=False]", true),
            ("[^0.1.2, loose=False]", true),
            ("[^1.2.3, loose=False]", true),
            ("[^1.2.3-beta.4, loose=False]", true),
            ("[<1, loose=False]", true),
            ("[< 1, loose=False]", true),
            ("[>=1, loose=False]", true),
            (">= 1, loose=False]", true),
            ("[<1.2, loose=False]", true),
            ("[< 1.2, loose=False]", true),
            ("[>01.02.03, loose=True]", true),
            ("[>01.02.03, loose=False]", false),
            ("[~1.2.3beta, loose=True]", true),
            ("[~1.2.3beta, loose=False]", false),
            ("[^ 1.2 ^ 1, loose=False]", true),
        ];
        for (input, expected) in cases {
            assert_eq!(
                is_valid(input),
                expected,
                "is_valid({input:?}) should be {expected}"
            );
        }
    }

    // Ported: "isVersion("$version") === $result" — lib/modules/versioning/conan/index.spec.ts line 117
    #[test]
    fn is_version_matches_renovate_conan_index_spec() {
        let cases: Vec<(&str, bool)> = vec![
            ("1.0.7-prerelease.1", true),
            ("1.0.7-prerelease.1, include_prerelease=True", true),
            ("NOT VALID, loose=False", false),
            ("NOT VALID", false),
            ("1.a.2, loose=False", false),
            ("1.a.2", true),
            ("1.2, loose=False", false),
            ("1.2", true),
            ("1.2.3.4, loose=False", false),
            ("1.2.3.4", true),
            ("1.2.23.4", true),
            ("4.1.3-pre, include_prerelease=True", true),
            ("X.2, loose=False", false),
            ("X.2", true),
            ("Infinity.NaN.Infinity, loose=False", false),
            ("Infinity.NaN.Infinity", false),
            ("1.2.3", true),
            (r#""1.2.3", loose=False"#, true),
            (r#""1.2.3", loose=False, include_prerelease=True"#, true),
            (r#""1.2.3", include_prerelease=True"#, true),
            ("1.2.3-alpha.1", true),
            (r#""1.2.3-alpha.1", include_prerelease=True"#, true),
            (r#""1.2.6-pre.1", include_prerelease=True"#, true),
            (r#""1.2.3-dev.1+abc", include_prerelease=True"#, true),
            ("1.2.3-dev.1+abc", true),
            ("1.2.6-pre.1", true),
            ("=1.2.3", true),
            ("= 1.2.3", true),
            ("1.x", true),
            (r#""1.x", loose=False"#, false),
            ("01.02.03", true),
            ("1.2.3-beta.01, include_prerelease=True", true),
            ("   =1.2.3", true),
            ("1.2.3foo, include_prerelease=True", true),
            ("5.0.20210712-T1759Z+b563c1478", true),
            ("0.2", true),
            ("16.00", true),
        ];
        for (input, expected) in cases {
            assert_eq!(
                is_version(input),
                expected,
                "is_version({input:?}) should be {expected}"
            );
        }
    }

    // Ported: "isCompatible("$version", "$range") === $result" — lib/modules/versioning/conan/index.spec.ts line 163
    #[test]
    fn is_compatible_matches_renovate_conan_index_spec() {
        // Sample of the test cases (the spec has ~200 rows, we test a representative subset)
        let cases: Vec<(&str, &str, bool)> = vec![
            ("[>1.1 <2.0]", "1.2.3", true),
            (
                r#"["~1.2.3", loose=False, include_prerelease=True]"#,
                "1.2.3-pre.1",
                false,
            ),
            (
                r#"["1.0.0 - 2.0.0", loose=False, include_prerelease=False]"#,
                "1.2.3",
                true,
            ),
            (
                r#"["1.0.0", loose=False, include_prerelease=False]"#,
                "1.0.0",
                true,
            ),
            (
                r#"[">=*", loose=False, include_prerelease=False]"#,
                "0.2.4",
                true,
            ),
            (
                r#"["", loose=False, include_prerelease=False]"#,
                "1.0.0",
                true,
            ),
            (
                r#"["*", loose=False, include_prerelease=False]"#,
                "1.2.3",
                true,
            ),
            (
                r#"[">=1.0.0", loose=False, include_prerelease=False]"#,
                "1.0.0",
                true,
            ),
            (
                r#"[">=1.0.0", loose=False, include_prerelease=False]"#,
                "1.0.1",
                true,
            ),
            (
                r#"[">1.0.0", loose=False, include_prerelease=True]"#,
                "1.0.1-pre.1",
                true,
            ),
            (
                r#"["<=2.0.0", loose=False, include_prerelease=False]"#,
                "3.0.0",
                true,
            ),
            (r#"["1.0.0 - 2.0.0", loose=False]"#, "2.2.3", true),
            (r#"[">=1.0.0", loose=False]"#, "0.0.0", false),
            (r#"[">=1.0.0", loose=False]"#, "0.0.1", false),
            (r#"[">=1.0.0", loose=False]"#, "0.1.0", false),
            (r#"[">1.0.0", loose=False]"#, "0.0.1", false),
            (r#"[">1.0.0", loose=False]"#, "0.1.0", false),
            (r#"[">=0.2.3 || <0.0.1", loose=False]"#, "0.0.3", true),
            (r#"[">=0.2.3 || <0.0.1", loose=False]"#, "0.2.2", true),
            (r#"["0.1.20 || 1.2.4", loose=False]"#, "1.2.3", true),
            ("[~=1.18]", "1.20.0", true),
            ("[0.2.0]", "0.3.0", true),
            ("[~8.4.0, loose=False]", "8.5.0", true),
            ("[~=1.0 include_prerelease=True]", "1.21.2", true),
            ("1.0.7", "1.21.2", true),
            ("16.00", "19.00", true),
        ];
        for (range, version, expected) in cases {
            assert_eq!(
                is_compatible(version, range),
                expected,
                "is_compatible({version:?}, {range:?}) should be {expected}"
            );
        }
    }

    // Ported: "matches("$version", "$range") === $result" — lib/modules/versioning/conan/index.spec.ts line 358
    #[test]
    fn matches_matches_renovate_conan_index_spec() {
        let cases: Vec<(&str, &str, bool)> = vec![
            ("[>1.1 <2.0]", "1.2.3", true),
            (
                r#"["~1.2.3", loose=False, include_prerelease=True]"#,
                "1.2.3-pre.1",
                true,
            ),
            (
                r#"["1.0.0 - 2.0.0", loose=False, include_prerelease=False]"#,
                "1.2.3",
                true,
            ),
            (
                r#"["*", loose=False, include_prerelease=False]"#,
                "1.2.3",
                true,
            ),
            (
                r#"[">=1.0.0", loose=False, include_prerelease=False]"#,
                "1.0.0",
                true,
            ),
            (
                r#"[">1.0.0", loose=False, include_prerelease=True]"#,
                "1.0.1-pre.1",
                true,
            ),
            (
                r#"[">=0.2.3 || <0.0.1", loose=False, include_prerelease=False]"#,
                "0.0.0",
                true,
            ),
            (
                r#"[">=0.2.3 || <0.0.1", loose=False, include_prerelease=False]"#,
                "0.2.3",
                true,
            ),
            (
                r#"["^1.2.3-alpha", loose=False, include_prerelease=False]"#,
                "1.2.3-pre",
                true,
            ),
            (
                r#"["^1.2.3-alpha", loose=False, include_prerelease=True]"#,
                "1.2.4-pre",
                true,
            ),
            ("[>1.1.0 <2.0.0]", "1.2.3-dev.1+abc", false),
            (
                r#"[">1.0.0 <1.0.2", loose=False, include_prerelease=True]"#,
                "1.0.2-beta",
                false,
            ),
            (
                r#"[">1.1.0 <2.0.0", include_prerelease=False]"#,
                "1.2.3-dev.1+abc",
                false,
            ),
            (r#"["1.0.0 - 2.0.0", loose=False]"#, "2.2.3", false),
            (r#"["^1.2.3+build", loose=False]"#, "2.0.0", false),
            (r#"["^1.2.3+build", loose=False]"#, "1.2.0", false),
            (r#"["^1.2.3", loose=False]"#, "1.2.3-pre", false),
            (r#"[">=1.0.0", loose=False]"#, "0.0.0", false),
            (r#"[">=0.1.97", loose=False]"#, "0.1.93", false),
            (r#"["0.1.20 || 1.2.4", loose=False]"#, "1.2.3", false),
            ("[~=1.18]", "1.20.0", false),
            ("[0.2.0]", "0.3.0", false),
            ("[~8.4.0, loose=False]", "8.5.0", false),
            ("[~=1.0 include_prerelease=True]", "1.21.2", false),
            ("1.0.7", "1.21.2", true),
            ("16.00", "19.00", true),
        ];
        for (range, version, expected) in cases {
            assert_eq!(
                matches(version, range),
                expected,
                "matches({version:?}, {range:?}) should be {expected}"
            );
        }
    }

    // Ported: "isStable("$version") === $result" — lib/modules/versioning/conan/index.spec.ts line 553
    #[test]
    fn is_stable_matches_renovate_conan_index_spec() {
        assert!(is_stable("5.0.1"));
        assert!(is_stable("19.00"));
        assert!(!is_stable("1.0.7-prerelease.1"));
        assert!(is_stable("1.0.7-prerelease.1, include_prerelease=True"));
    }

    // Ported: "getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$result""
    //         — lib/modules/versioning/conan/index.spec.ts line 565
    #[test]
    fn get_new_value_matches_renovate_conan_index_spec() {
        let cases: Vec<(&str, &str, &str, &str, Option<&str>)> = vec![
            ("[<=1.2.3]", "widen", "1.0.0", "1.2.3", Some("[<=1.2.3]")),
            ("[<1.2.3]", "widen", "1.5.5", "1.5.6", Some("[<1.5.7]")),
            (
                "[>1.2.7 >3.0.0 5.0]",
                "widen",
                "0.1.21",
                "0.1.24",
                Some("[>1.2.7 >3.0.0 5.0 || 0.1.24]"),
            ),
            ("[>=1.2.7 >3.0.0 >5.0]", "widen", "0.1.21", "0.1.24", None),
            (
                "[>=1.2.7]",
                "widen",
                "0.1.21",
                "0.1.24",
                Some("[>=1.2.7 || 0.1.24]"),
            ),
            ("[<= 1.2.3]", "widen", "1.0.0", "1.2.4", Some("[<= 1.2.4]")),
            (
                "[4.5.5 - 1.2.3 - 2.0]",
                "widen",
                "1.0.0",
                "1.4.8",
                Some("[4.5.5 - 1.2.3 - 1.4.8]"),
            ),
            ("[*]", "widen", "1.0.0", "2.0.0", Some("[*]")),
            ("[>=*]", "widen", "1.0.0", "2.0.0", Some("[>=*]")),
            ("[x]", "widen", "1.0.0", "2.0.0", Some("[x]")),
            ("1.0.0", "replace", "1.0.0", "1.1.0", Some("1.1.0")),
            ("[<1.0.0]", "replace", "1.0.0", "2.1.0", Some("[<3.0.0]")),
            ("[<1.1]", "replace", "1.0.0", "2.1.0", Some("[<2.2]")),
            ("[1.0.*]", "replace", "1.0.0", "1.1.0", Some("[1.1.*]")),
            ("[1.*]", "replace", "1.0.0", "2.1.0", Some("[2.*]")),
            ("[1.0.x]", "replace", "1.0.0", "1.1.0", Some("[1.1.x]")),
            ("[1.x]", "replace", "1.0.0", "2.1.0", Some("[2.x]")),
            ("[~0.6]", "replace", "0.6.8", "0.7.0", Some("[~0.7.0]")),
            (
                "[~0.6.1]",
                "replace",
                "0.7.0",
                "0.7.0-rc.2",
                Some("[~0.7.0-rc]"),
            ),
            (
                "[~>0.6.1]",
                "replace",
                "0.7.0",
                "0.7.0-rc.2",
                Some("[~> 0.7.0]"),
            ),
            ("[<=1.2]", "replace", "1.0.0", "1.2.3", Some("[<=1.2]")),
            ("[<=1]", "replace", "1.0.0", "1.2.3", Some("[<=1]")),
            (
                "[<1.6.11]",
                "replace",
                "0.6.14",
                "1.6.14",
                Some("[<1.6.15]"),
            ),
            ("[0.2.0]", "replace", "0.6.14", "0.3.0", Some("[0.3.0]")),
            ("[< 1]", "replace", "1.0.0", "1.0.1", Some("[< 2]")),
            (
                "[<3.6 loose=False, include_prerelease=True]",
                "replace",
                "0.1",
                "3.7.0",
                Some("[<3.8 loose=False, include_prerelease=True]"),
            ),
            (
                "[<1.8 loose=False]",
                "replace",
                "0.2",
                "1.17.1",
                Some("[<1.18 loose=False]"),
            ),
            ("[=8.4.0]", "replace", "0.6.14", "8.5.0", Some("[=8.5.0]")),
            ("[>8.0.0]", "replace", "0.6.14", "8.5.0", Some("[>9.0.0]")),
            ("[>8]", "replace", "0.6.14", "8.5.0", Some("[>8]")),
            ("[> 8]", "replace", "0.6.14", "8.5.0", Some("[>8]")),
            ("[*]", "replace", "1.0.0", "2.0.0", Some("[*]")),
            ("[>=*]", "replace", "1.0.0", "2.0.0", Some("[>=*]")),
            ("[x]", "replace", "1.0.0", "2.0.0", Some("[x]")),
            ("[=8.4.0]", "bump", "0.6.14", "8.5.0", Some("[=8.5.0]")),
            (
                "[~8.4.0, loose=False]",
                "bump",
                "0.6.14",
                "8.5.0",
                Some("[~8.5.0, loose=False]"),
            ),
            (
                "[~0.7.15, loose=False, include_prerelease=True]",
                "bump",
                "0.6.14",
                "0.9.7",
                Some("[~0.9.7, loose=False, include_prerelease=True]"),
            ),
            (
                "[~=1.0 include_prerelease=True]",
                "bump",
                "0.2",
                "1.21.1",
                Some("[~=1.21 include_prerelease=True]"),
            ),
            ("[~=1.18]", "bump", "0.6.14", "1.20.0", Some("[~=1.20]")),
            ("[0.2.0]", "bump", "0.6.14", "0.3.0", Some("[0.3.0]")),
            ("[~1]", "bump", "2", "1.1.7", Some("[~1]")),
            ("[~1]", "bump", "1.0.0", "2.1.7", Some("[~2]")),
            ("[~1.0]", "bump", "1.0.0", "1.1.7", Some("[~1.1]")),
            ("[~1.0.0]", "bump", "1.0.0", "1.1.7", Some("[~1.1.7]")),
            (
                "[~1.0]",
                "bump",
                "1.0.0",
                "1.0.7-prerelease.1",
                Some("[~1.0.7-prerelease.1]"),
            ),
            (
                "[~1.0.7-prerelease.1]",
                "bump",
                "1.0.0",
                "1.0.7-prerelease.1",
                Some("[~1.0.7-prerelease.1]"),
            ),
            ("[5]", "bump", "5.0.0", "6.1.7", Some("[6]")),
            ("[>=1.0.0]", "bump", "1.0.0", "1.1.0", Some("[>=1.1.0]")),
            ("[<1.0.0]", "bump", "1.0.0", "1.1.0", Some("[<1.0.0]")),
            (
                "[>1.1 <3.0, include_prerelease=True]",
                "bump",
                "0.6.14",
                "1.1.1l",
                Some("[>1.1 <3.0, include_prerelease=True]"),
            ),
            (
                "[>= 0.0.1 < 1]",
                "bump",
                "1.0.0",
                "1.0.1",
                Some("[>= 1.0.1 < 2]"),
            ),
            (
                "[>3.0 <3.6 loose=False, include_prerelease=True]",
                "bump",
                "0.1",
                "3.7.0",
                Some("[>3.7 <3.8 loose=False, include_prerelease=True]"),
            ),
            (
                "[>1.0 <1.8 loose=False]",
                "bump",
                "0.2",
                "1.17.1",
                Some("[>1.17 <1.18 loose=False]"),
            ),
            (
                "[>0.6.7 <1.6.11]",
                "bump",
                "0.6.14",
                "1.6.14",
                Some("[>1.6.14 <1.6.15]"),
            ),
            (
                "[3.17.2 || 3.15.7]",
                "bump",
                "0.6.14",
                "3.21.3",
                Some("[3.17.2 || 3.15.7 || 3.21.3]"),
            ),
            (
                "[>3.17.2 || 3.15.7]",
                "bump",
                "0.6.14",
                "3.21.3",
                Some("[>3.21.3 || 3.15.7]"),
            ),
            (
                "[1.69.0 || >=1.71.0 <1.76.0]",
                "bump",
                "0.6.14",
                "1.76.0",
                Some("[1.69.0 || >=1.76.0 <1.76.1]"),
            ),
            ("[*]", "bump", "1.0.0", "2.0.0", Some("[*]")),
            ("[>=*]", "bump", "1.0.0", "2.0.0", Some("[>=*]")),
            ("[x]", "bump", "1.0.0", "2.0.0", Some("[x]")),
        ];
        for (cv, strategy, cur_ver, new_ver, expected) in &cases {
            let result = get_new_value(cv, strategy, cur_ver, new_ver);
            assert_eq!(
                result.as_deref(),
                *expected,
                "get_new_value({cv:?}, {strategy:?}, {cur_ver:?}, {new_ver:?}) should be {expected:?}"
            );
        }
    }

    // Ported: "getSatisfyingVersion("$versions", "$range") === "$result"" — lib/modules/versioning/conan/index.spec.ts line 641
    #[test]
    fn get_satisfying_version_matches_renovate_conan_index_spec() {
        fn v<'a>(vs: &'a [&'a str]) -> Vec<&'a str> {
            vs.to_vec()
        }
        let cases: Vec<(Vec<&str>, &str, Option<&str>)> = vec![
            (
                v(&["1.2.4", "1.2.3", "1.2.5-beta"]),
                r#"["~1.2.3", loose=False, include_prerelease=True]"#,
                Some("1.2.5-beta"),
            ),
            (
                v(&["1.2.4", "1.2.3", "1.2.5-beta"]),
                r#"["~1.2.3", loose=False, include_prerelease=False]"#,
                Some("1.2.4"),
            ),
            (
                v(&["1.2.3", "1.2.4"]),
                r#"["1.2", loose=False, include_prerelease=False]"#,
                Some("1.2.4"),
            ),
            (
                v(&["1.2.4", "1.2.3"]),
                r#"["1.2", loose=False, include_prerelease=False]"#,
                Some("1.2.4"),
            ),
            (
                v(&["1.2.3", "1.2.4", "1.2.5", "1.2.6"]),
                r#"["~1.2.3", loose=False, include_prerelease=False]"#,
                Some("1.2.6"),
            ),
            (
                v(&["1.0.1-beta"]),
                r#"["1.x", loose=False, include_prerelease=False]"#,
                None,
            ),
            (
                v(&["1.0.1-beta"]),
                r#"["1.x", loose=False, include_prerelease=True]"#,
                Some("1.0.1-beta"),
            ),
            (
                v(&["1.2.3", "1.2.4", "1.2.5", "1.2.6", "2.0.1"]),
                r#"["~1.2.3", loose=False]"#,
                Some("1.2.6"),
            ),
            (
                v(&[
                    "1.1.0", "1.2.0", "1.3.0", "2.0.0b1", "2.0.0b3", "2.0.0", "2.1.0",
                ]),
                "[~2.0.0]",
                Some("2.0.0"),
            ),
            (
                v(&["1.1.0", "1.2.0", "1.3.0", "2.0.0b1", "2.0.0b3", "2.1.0"]),
                r#"["~2.0.0", loose=False]"#,
                None,
            ),
            (
                v(&[
                    "1.1.0", "1.2.0", "1.3.0", "2.0.0b1", "2.0.0b3", "2.0.1", "2.1.0",
                ]),
                r#"["~2.0.0", loose=False]"#,
                Some("2.0.1"),
            ),
            (
                v(&["1.2.3", "1.2.4", "1.2.5", "1.2.6-pre.1", "2.0.1"]),
                r#"["~1.2.3", loose=False, include_prerelease=True]"#,
                Some("1.2.6-pre.1"),
            ),
            (
                v(&["1.2.3", "1.2.4", "1.2.5", "1.2.6-pre.1", "2.0.1"]),
                r#"["~1.2.3", loose=False, include_prerelease=False]"#,
                Some("1.2.5"),
            ),
            (
                v(&["1.1.1", "1.2.0-pre", "1.2.0", "1.1.1-111", "1.1.1-21"]),
                "[<=1.2]",
                Some("1.2.0"),
            ),
        ];
        for (versions, range, expected) in &cases {
            let result = get_satisfying_version(versions, range);
            assert_eq!(
                result, *expected,
                "get_satisfying_version({versions:?}, {range:?}) should be {expected:?}"
            );
        }
    }

    // Ported: "minSatisfyingVersion("$versions", "$range") === "$result"" — lib/modules/versioning/conan/index.spec.ts line 699
    #[test]
    fn min_satisfying_version_matches_renovate_conan_index_spec() {
        fn v<'a>(vs: &'a [&'a str]) -> Vec<&'a str> {
            vs.to_vec()
        }
        let cases: Vec<(Vec<&str>, &str, Option<&str>)> = vec![
            (
                v(&["1.2.3", "1.2.4", "1.2.5", "1.2.6", "2.0.1"]),
                r#"["~1.2.3", loose=False]"#,
                Some("1.2.3"),
            ),
            (
                v(&[
                    "1.1.0", "1.2.0", "1.3.0", "2.0.0b1", "2.0.0b3", "2.0.0", "2.1.0",
                ]),
                "[~2.0.0]",
                Some("2.0.0"),
            ),
            (
                v(&["1.1.0", "1.2.0", "1.3.0", "2.0.0b1", "2.0.0b3", "2.1.0"]),
                r#"["~2.0.0", loose=False]"#,
                None,
            ),
            (
                v(&[
                    "1.1.0", "1.2.0", "1.3.0", "2.0.0b1", "2.0.0b3", "2.0.1", "2.1.0",
                ]),
                r#"["~2.0.0", loose=False]"#,
                Some("2.0.1"),
            ),
            (
                v(&["1.2.3-pre.1", "1.2.4", "1.2.5", "1.2.6-pre.1", "2.0.1"]),
                r#"["~1.2.3", loose=False, include_prerelease=True]"#,
                Some("1.2.3-pre.1"),
            ),
            (
                v(&["1.2.3-pre.1", "1.2.4", "1.2.5", "1.2.6-pre.1", "2.0.1"]),
                r#"["~1.2.3", loose=False, include_prerelease=False]"#,
                Some("1.2.4"),
            ),
            (
                v(&["1.2.3", "1.2.4"]),
                r#"["1.2", loose=False]"#,
                Some("1.2.3"),
            ),
            (
                v(&["1.2.4", "1.2.3"]),
                r#"["1.2", loose=False]"#,
                Some("1.2.3"),
            ),
            (
                v(&["1.2.3", "1.2.4", "1.2.5", "1.2.6"]),
                "[~1.2.3, loose=False]",
                Some("1.2.3"),
            ),
            (
                v(&[
                    "1.1.0", "1.2.0", "1.2.1", "1.3.0", "2.0.0b1", "2.0.0b2", "2.0.0b3", "2.0.0",
                    "2.1.0",
                ]),
                "[~2.0.0, loose=True]",
                Some("2.0.0"),
            ),
        ];
        for (versions, range, expected) in &cases {
            let result = min_satisfying_version(versions, range);
            assert_eq!(
                result, *expected,
                "min_satisfying_version({versions:?}, {range:?}) should be {expected:?}"
            );
        }
    }

    // Ported: "getMajor("$version") === $major getMinor("$version") === $minor getPatch("$version") === $patch"
    //         — lib/modules/versioning/conan/index.spec.ts line 720
    #[test]
    #[allow(clippy::type_complexity)]
    fn get_major_minor_patch_matches_renovate_conan_index_spec() {
        let cases: Vec<(&str, Option<u64>, Option<u64>, Option<u64>)> = vec![
            ("4.1.3", Some(4), Some(1), Some(3)),
            ("4.1.3+jenkins", Some(4), Some(1), Some(3)),
            ("4.1.3-pre", Some(4), Some(1), Some(3)),
            ("4.1.3.2", Some(4), Some(1), Some(3)),
            ("4.1.3.2+jenkins", Some(4), Some(1), Some(3)),
            ("4.1.3.2-pre", Some(4), Some(1), Some(3)),
            ("4.1.3.2-pre2", Some(4), Some(1), Some(3)),
            ("4.1.3.2-pre.2", Some(4), Some(1), Some(3)),
            ("4.1.3.2-pre.2+xxx", Some(4), Some(1), Some(3)),
            ("4.1.33.2", Some(4), Some(1), Some(33)),
            ("1.a.2", None, None, None),
        ];
        for (v, major, minor, patch) in cases {
            assert_eq!(get_major(v), major, "getMajor({v:?})");
            assert_eq!(get_minor(v), minor, "getMinor({v:?})");
            assert_eq!(get_patch(v), patch, "getPatch({v:?})");
        }
    }

    // Ported: "getMajor("$version") === "$result"" — lib/modules/versioning/conan/index.spec.ts line 743
    #[test]
    fn get_major_matches_renovate_conan_index_spec() {
        assert_eq!(get_major("4.1.33.2"), Some(4));
    }

    // Ported: "getMinor("$version") === "$result"" — lib/modules/versioning/conan/index.spec.ts line 752
    #[test]
    fn get_minor_matches_renovate_conan_index_spec() {
        assert_eq!(get_minor("1.2.3"), Some(2));
        assert_eq!(get_minor("5.2.1"), Some(2));
        assert_eq!(get_minor("4.1.33.2"), Some(1));
    }

    // Ported: "getPatch("$version") === "$result"" — lib/modules/versioning/conan/index.spec.ts line 763
    #[test]
    fn get_patch_matches_renovate_conan_index_spec() {
        assert_eq!(get_patch("1.2.3"), Some(3));
        assert_eq!(get_patch("5.2.1"), Some(1));
        assert_eq!(get_patch("4.1.33.2"), Some(33));
    }

    // Ported: "equals("$version", "$other) === "$result"" — lib/modules/versioning/conan/index.spec.ts line 774
    #[test]
    fn equals_matches_renovate_conan_index_spec() {
        assert!(equals("1.2.3", "1.2.3"));
        assert!(!equals("2.3.1", "1.2.3"));
        assert!(equals("1.2.3", "v1.2.3, loose=True"));
        assert!(equals("1.2.3", "=1.2.3, loose=True"));
        assert!(equals("1.2.3", "v 1.2.3, loose=True"));
        assert!(equals("1.2.3", "= 1.2.3, loose=True"));
        assert!(equals(
            "1.2.3-beta+build",
            " = 1.2.3-beta+otherbuild, loose=True"
        ));
        assert!(equals("1.2.3+build", " = 1.2.3+otherbuild, loose=True"));
        assert!(equals(
            "1.2.3-beta+build",
            "1.2.3-beta+otherbuild, loose=False"
        ));
        assert!(equals("1.2.3+build", "1.2.3+otherbuild, loose=False"));
        assert!(equals("  v1.2.3+build", "1.2.3+otherbuild, loose=False"));
        assert!(!equals("1.3", "1.2"));
    }

    // Ported: "isGreaterThan("$version", "$other) === "$result"" — lib/modules/versioning/conan/index.spec.ts line 825
    #[test]
    fn is_greater_than_matches_renovate_conan_index_spec() {
        assert!(!is_greater_than("1.2.3", "1.2.3"));
        assert!(is_greater_than("19.00", "16.00"));
        assert!(is_greater_than("1.2", "1.0"));
        assert!(is_greater_than("2.3.1", "1.2.3"));
        assert!(is_greater_than(
            "0.0.0, loose=False",
            "0.0.0-foo, loose=False"
        ));
        assert!(is_greater_than("0.0.1, loose=False", "0.0.0, loose=False"));
        assert!(is_greater_than("1.0.0, loose=False", "0.9.9, loose=False"));
        assert!(is_greater_than("2.0.0, loose=False", "1.2.3, loose=False"));
        assert!(is_greater_than("v0.0.0", "0.0.0-foo"));
        assert!(is_greater_than(
            "1.2.3, loose=False",
            "1.2.3-asdf, loose=False"
        ));
        assert!(is_greater_than(
            "1.2.3-5-foo, loose=False",
            "1.2.3-5, loose=False"
        ));
        assert!(is_greater_than(
            "1.2.3-5, loose=False",
            "1.2.3-4, loose=False"
        ));
        assert!(is_greater_than(
            "3.0.0, loose=False",
            "2.7.2+asdf, loose=False"
        ));
    }

    // Ported: "sortVersions("$version", "$other) === "$result"" — lib/modules/versioning/conan/index.spec.ts line 871
    #[test]
    fn sort_versions_matches_renovate_conan_index_spec() {
        assert_eq!(sort_versions("1.2", "1.3"), -1);
        assert_eq!(sort_versions("1.2.3", "1.2.3"), 0);
        assert_eq!(sort_versions("2.3.1", "1.2.3"), 1);
        assert_eq!(sort_versions("1.2.3", "2.3.1"), -1);
    }

    // Ported: "isLessThanRange("$version", "$range") === "$result"" — lib/modules/versioning/conan/index.spec.ts line 886
    #[test]
    fn is_less_than_range_matches_renovate_conan_index_spec() {
        assert!(is_less_than_range("1.2.3", "[>1.2.3]"));
        assert!(!is_less_than_range("2.3.1", "[>1.2.3]"));
        assert!(is_less_than_range("1.2.3", "[>2.3.1]"));
    }
}
