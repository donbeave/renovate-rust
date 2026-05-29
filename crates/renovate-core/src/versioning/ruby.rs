//! Ruby gem versioning.
//!
//! Mirrors `lib/modules/versioning/ruby/index.ts`.
//! Supports `~>` pessimistic version constraint, dot-separated prereleases,
//! and partial versions (`1`, `1.2`).

use regex::Regex;
use std::cmp::Ordering;
use std::sync::LazyLock;


/// A parsed Ruby gem version.
#[derive(Debug, Clone)]
struct RubyVersion {
    segs: Vec<u64>, // numeric segments (arbitrary count)
    pre: Option<String>,
}

impl RubyVersion {
    fn parse(input: &str) -> Option<Self> {
        let s = input.trim().trim_start_matches('v');
        if s.is_empty() {
            return None;
        }

        // Split on '.' or '-' to tokenize
        // Rule: consume leading all-digit tokens as numeric segments;
        // once we hit a non-numeric token (or a '-' separator), the rest is prerelease.
        // This matches ruby gem semantics where "4.2.5.1" is 4 numeric segs
        // and "1.0.0.alpha" is [1,0,0] + pre("alpha").
        let mut segs: Vec<u64> = Vec::new();
        let mut pre_parts: Vec<&str> = Vec::new();
        let mut in_pre = false;

        // First split by '-' to separate the dash-prerelease portion
        let (numeric_part, dash_pre) = if let Some(pos) = s.find('-') {
            (&s[..pos], Some(&s[pos + 1..]))
        } else {
            (s, None)
        };

        // Now split numeric_part by '.'
        for token in numeric_part.split('.') {
            if in_pre {
                pre_parts.push(token);
            } else if let Ok(n) = token.parse::<u64>() {
                segs.push(n);
            } else {
                // Non-numeric dot-segment → prerelease starts here
                in_pre = true;
                pre_parts.push(token);
            }
        }

        let pre = if !pre_parts.is_empty() {
            let mut p = pre_parts.join(".");
            if let Some(d) = dash_pre {
                p.push('-');
                p.push_str(d);
            }
            Some(p)
        } else {
            dash_pre.map(|d| d.to_owned())
        };

        if segs.is_empty() {
            return None;
        }

        Some(RubyVersion { segs, pre })
    }

    fn seg(&self, i: usize) -> u64 {
        self.segs.get(i).copied().unwrap_or(0)
    }

    fn max_segs(&self, other: &Self) -> usize {
        self.segs.len().max(other.segs.len())
    }
}

impl PartialEq for RubyVersion {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for RubyVersion {}

/// Compare two prerelease token strings (e.g., "rc1" vs "beta").
fn cmp_prerelease_tokens(a: &str, b: &str) -> Ordering {
    let an: Option<u64> = a.parse().ok();
    let bn: Option<u64> = b.parse().ok();
    match (an, bn) {
        (Some(a), Some(b)) => a.cmp(&b),
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (None, None) => a.cmp(b),
    }
}

fn cmp_prerelease_str(a: &str, b: &str) -> Ordering {
    let a_toks: Vec<&str> = a.split(['.', '-']).collect();
    let b_toks: Vec<&str> = b.split(['.', '-']).collect();
    let len = a_toks.len().max(b_toks.len());
    for i in 0..len {
        let at = a_toks.get(i).copied().unwrap_or("");
        let bt = b_toks.get(i).copied().unwrap_or("");
        if at == bt {
            continue;
        }
        let ord = cmp_prerelease_tokens(at, bt);
        if ord != Ordering::Equal {
            return ord;
        }
    }
    Ordering::Equal
}

impl PartialOrd for RubyVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RubyVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        // Numeric segments (missing → 0)
        let n = self.max_segs(other);
        for i in 0..n {
            let ord = self.seg(i).cmp(&other.seg(i));
            if ord != Ordering::Equal {
                return ord;
            }
        }
        // Prerelease: pre < stable
        match (&self.pre, &other.pre) {
            (None, None) => Ordering::Equal,
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (Some(a), Some(b)) => cmp_prerelease_str(a, b),
        }
    }
}

pub fn is_version(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }
    RubyVersion::parse(input).is_some()
}

pub fn get_major(input: &str) -> Option<i32> {
    RubyVersion::parse(input).map(|v| v.seg(0) as i32)
}

pub fn get_minor(input: &str) -> Option<i32> {
    RubyVersion::parse(input)?
        .segs
        .get(1)
        .copied()
        .map(|v| v as i32)
}

pub fn get_patch(input: &str) -> Option<i32> {
    RubyVersion::parse(input)?
        .segs
        .get(2)
        .copied()
        .map(|v| v as i32)
}

pub fn equals(a: &str, b: &str) -> bool {
    match (RubyVersion::parse(a), RubyVersion::parse(b)) {
        (Some(av), Some(bv)) => av == bv,
        _ => false,
    }
}

pub fn is_greater_than(a: &str, b: &str) -> bool {
    match (RubyVersion::parse(a), RubyVersion::parse(b)) {
        (Some(av), Some(bv)) => av > bv,
        _ => false,
    }
}

pub fn is_stable(input: &str) -> bool {
    match RubyVersion::parse(input) {
        Some(v) => v.pre.is_none(),
        None => false,
    }
}

pub fn sort_versions(a: &str, b: &str) -> Ordering {
    match (RubyVersion::parse(a), RubyVersion::parse(b)) {
        (Some(av), Some(bv)) => av.cmp(&bv),
        _ => a.cmp(b),
    }
}

/// One parsed range constraint: operator + version string.
#[derive(Debug, Clone)]
struct Constraint {
    op: String,
    ver: RubyVersion,
    ver_segs: usize, // original number of numeric segments
}

/// Parse a constraint like `>= 1.2`, `~> 1.2.3`, `1.2.3`.
fn parse_constraint(s: &str) -> Option<Constraint> {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^(?<op>~>|[!<>]=?|=)?\s*(?<ver>v?[\d][0-9a-zA-Z._-]*)$").unwrap()
    });
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    let caps = RE.captures(s)?;
    let op = caps
        .name("op")
        .map(|m| m.as_str())
        .unwrap_or("=")
        .to_owned();
    let ver_str = caps.name("ver")?.as_str();
    let ver = RubyVersion::parse(ver_str)?;
    let ver_segs = ver.segs.len();
    Some(Constraint { op, ver, ver_segs })
}

fn parse_constraints(range: &str) -> Vec<Constraint> {
    range
        .split(',')
        .filter_map(|s| parse_constraint(s.trim()))
        .collect()
}

fn satisfies_constraint(v: &RubyVersion, c: &Constraint) -> bool {
    match c.op.as_str() {
        "=" | "" => *v == c.ver,
        "!=" => *v != c.ver,
        ">" => *v > c.ver,
        ">=" => *v >= c.ver,
        "<" => *v < c.ver,
        "<=" => *v <= c.ver,
        "~>" => {
            // Pessimistic constraint `~> X.Y...Z`:
            // - Lower bound: v >= X.Y...Z
            // - Upper bound: all segments up to second-to-last are fixed equal,
            //   and the second-to-last segment < c.ver.second-to-last + 1.
            // ~> X → >= X, < X+1 (only 1 seg: bump seg 0)
            // ~> X.Y → >= X.Y, < X+1 (2 segs: bump seg 0)
            // ~> X.Y.Z → >= X.Y.Z, < X.Y+1 (3+ segs: fix segs 0..n-2, bump seg n-2)
            if *v < c.ver {
                return false;
            }
            if c.ver_segs <= 2 {
                // Upper bound: v.seg(0) < c.ver.seg(0) + 1
                v.seg(0) < c.ver.seg(0) + 1
            } else {
                // Fix segments 0..n-2, bump segment n-2
                let bump_idx = c.ver_segs - 2;
                for i in 0..bump_idx {
                    if v.seg(i) != c.ver.seg(i) {
                        return false;
                    }
                }
                v.seg(bump_idx) < c.ver.seg(bump_idx) + 1
            }
        }
        _ => false,
    }
}

pub fn matches(version: &str, range: &str) -> bool {
    let Some(v) = RubyVersion::parse(version) else {
        return false;
    };
    let cs = parse_constraints(range);
    if cs.is_empty() {
        return false;
    }
    cs.iter().all(|c| satisfies_constraint(&v, c))
}

pub fn is_less_than_range(version: &str, range: &str) -> Option<bool> {
    let v = RubyVersion::parse(version)?;
    let cs = parse_constraints(range);
    if cs.is_empty() {
        return Some(false);
    }
    // Mirror ltr() from range.ts:
    // For each constraint, check if version is "less than or at" the constraint's version.
    // GT/LT: version <= constraint_version (compare <= 0)
    // GTE/LTE/EQ/NEQ: version < constraint_version (compare < 0)
    // ~>: version < constraint_version AND in same major series
    let all_ltr = cs.iter().all(|c| {
        match c.op.as_str() {
            ">" | "<" => v <= c.ver,
            ">=" | "<=" | "=" | "" | "!=" => v < c.ver,
            "~>" => {
                // version < constraint AND version < bump of constraint
                if v >= c.ver {
                    return false;
                }
                // Also check that version <= bump (upper bound)
                let bump = if c.ver_segs <= 1 {
                    RubyVersion {
                        segs: vec![c.ver.seg(0) + 1],
                        pre: None,
                    }
                } else {
                    // bump penultimate segment
                    let mut segs = c.ver.segs[..c.ver_segs - 1].to_vec();
                    *segs.last_mut().unwrap() += 1;
                    RubyVersion { segs, pre: None }
                };
                v <= bump
            }
            _ => false,
        }
    });
    Some(all_ltr)
}

pub fn min_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    let cs = parse_constraints(range);
    let mut matches: Vec<(&'a str, RubyVersion)> = versions
        .iter()
        .filter_map(|&v| {
            let rv = RubyVersion::parse(v)?;
            if cs.iter().all(|c| satisfies_constraint(&rv, c)) {
                Some((v, rv))
            } else {
                None
            }
        })
        .collect();
    matches.sort_by(|a, b| a.1.cmp(&b.1));
    matches.into_iter().next().map(|(v, _)| v)
}

pub fn get_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    let cs = parse_constraints(range);
    let mut ms: Vec<(&'a str, RubyVersion)> = versions
        .iter()
        .filter_map(|&v| {
            let rv = RubyVersion::parse(v)?;
            if cs.iter().all(|c| satisfies_constraint(&rv, c)) {
                Some((v, rv))
            } else {
                None
            }
        })
        .collect();
    ms.sort_by(|a, b| a.1.cmp(&b.1));
    ms.into_iter().last().map(|(v, _)| v)
}

pub fn is_valid(input: &str) -> bool {
    let s = input.trim();
    if s.is_empty() {
        return false;
    }
    // All comma-separated parts must parse successfully
    let parts: Vec<&str> = s.split(',').map(|p| p.trim()).collect();
    let valid_ops = ["=", "!=", ">", ">=", "<", "<=", "~>", ""];
    for part in &parts {
        match parse_constraint(part) {
            Some(c) if valid_ops.contains(&c.op.as_str()) => {}
            _ => return false,
        }
    }
    !parts.is_empty()
}

pub fn is_single_version(input: &str) -> bool {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"^(?:=\s*)?v?(\d+)(?:\.(\d+))?(?:\.(\d+))?(?:[.-](.+))?$").unwrap()
    });
    let s = input.trim();
    if s.is_empty() {
        return false;
    }
    RE.is_match(s)
}

pub fn get_pinned_value(input: &str) -> String {
    input.trim_start_matches('v').to_owned()
}

// ---------------------------------------------------------------------------
// get_new_value — Ruby range strategy helpers
// ---------------------------------------------------------------------------

/// A parsed single range part (operator + delimiter + version), with an
/// optional companion (the `>=` part of a `~> X, >= Y` pair).
#[derive(Debug, Clone)]
struct RubyRange {
    operator: String,
    delimiter: String,
    version: String,
    companion: Option<Box<RubyRange>>,
}

static RANGE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<operator>[^\d\s]*)(?P<delimiter>\s*)(?P<version>[0-9a-zA-Z.\-]+)$").unwrap()
});

/// Parse a single range string into a `RubyRange`.
fn parse_ruby_range(s: &str) -> RubyRange {
    let value = s.trim();
    if let Some(caps) = RANGE_RE.captures(value) {
        let operator = caps.name("operator").map(|m| m.as_str()).unwrap_or("").to_owned();
        let delimiter = caps.name("delimiter").map(|m| m.as_str()).unwrap_or(" ").to_owned();
        let version = caps.name("version").map(|m| m.as_str()).unwrap_or("").to_owned();
        RubyRange { operator, delimiter, version, companion: None }
    } else {
        RubyRange {
            operator: String::new(),
            delimiter: " ".to_owned(),
            version: String::new(),
            companion: None,
        }
    }
}

/// Parse a comma-separated range string into `Vec<RubyRange>`, combining
/// consecutive `~>` + `>=` pairs into a single range with companion.
fn parse_ruby_ranges(range: &str) -> Vec<RubyRange> {
    let raw: Vec<RubyRange> = range.split(',').map(parse_ruby_range).collect();
    let mut result: Vec<RubyRange> = Vec::new();
    let mut i = 0;
    while i < raw.len() {
        if i + 1 < raw.len() && raw[i].operator == "~>" && raw[i + 1].operator == ">=" {
            let mut combined = raw[i].clone();
            combined.companion = Some(Box::new(raw[i + 1].clone()));
            result.push(combined);
            i += 2;
        } else {
            result.push(raw[i].clone());
            i += 1;
        }
    }
    result
}

/// Stringify a list of `RubyRange` back to a comma-separated string.
fn stringify_ruby_ranges(ranges: &[RubyRange]) -> String {
    ranges
        .iter()
        .map(|r| {
            if let Some(comp) = &r.companion {
                format!(
                    "{}{}{}, {}{}{}",
                    r.operator, r.delimiter, r.version,
                    comp.operator, comp.delimiter, comp.version
                )
            } else {
                format!("{}{}{}", r.operator, r.delimiter, r.version)
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

// ---------------------------------------------------------------------------
// Version arithmetic helpers
// ---------------------------------------------------------------------------

/// `adapt(left, right)`: truncate `left` to the same number of dot-segments as `right`.
fn ruby_adapt(left: &str, right: &str) -> String {
    let count = right.split('.').count();
    left.split('.').take(count).collect::<Vec<_>>().join(".")
}

/// `floor(v)`: set the last segment to "0" (for >= 2 segments).
fn ruby_floor(v: &str) -> String {
    let segs: Vec<&str> = v.split('.').collect();
    if segs.len() <= 1 {
        return v.to_owned();
    }
    let mut s: Vec<&str> = segs[..segs.len() - 1].to_vec();
    s.push("0");
    s.join(".")
}

/// `trim_zeroes(v)`: remove trailing ".0" segments.
fn ruby_trim_zeroes(v: &str) -> String {
    let mut segs: Vec<&str> = v.split('.').collect();
    while segs.len() > 1 && segs.last() == Some(&"0") {
        segs.pop();
    }
    segs.join(".")
}

/// `increment_last_segment(v)`: increment the last numeric segment.
fn ruby_increment_last_segment(v: &str) -> String {
    let mut segs: Vec<String> = v.split('.').map(|s| s.to_owned()).collect();
    if let Some(last) = segs.last_mut() {
        let n: u64 = last.parse().unwrap_or(0);
        *last = (n + 1).to_string();
    }
    segs.join(".")
}

/// `pgte_upper_bound(v)`: upper bound of the `~>` operator.
/// If more than 1 segment, pop the last then increment the new last.
fn ruby_pgte_upper_bound(v: &str) -> String {
    let segs: Vec<&str> = v.split('.').collect();
    if segs.len() > 1 {
        let without_last = segs[..segs.len() - 1].join(".");
        ruby_increment_last_segment(&without_last)
    } else {
        ruby_increment_last_segment(v)
    }
}

/// `increment(from, to)`: find the smallest version > `from` that still
/// reaches `to` when adapted.  Mirrors the TypeScript `increment` function.
fn ruby_increment(from: &str, to: &str) -> String {
    let adapted = ruby_adapt(to, from);
    if from == adapted {
        return ruby_increment_last_segment(from);
    }
    // TypeScript always uses 3 segments (major.minor.patch) for the next candidate.
    // We use max(from.len(), 3) to match that behavior.
    let from_segs: Vec<i64> = from.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    let adapted_segs: Vec<i64> = adapted.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    let n = from_segs.len().max(3);

    for i in 0..from_segs.len() {
        let from_seg = from_segs.get(i).copied().unwrap_or(0);
        let adapted_seg = adapted_segs.get(i).copied().unwrap_or(0);
        if from_seg != adapted_seg {
            // Increment from's value at this level (matches TypeScript incrementMajor/Minor/Patch),
            // set everything below to 0, pad to n segments.
            let mut next: Vec<i64> = from_segs[..i].to_vec();
            next.push(from_seg + 1);
            while next.len() < n {
                next.push(0);
            }
            let next_str = next.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(".");
            return ruby_increment(&next_str, to);
        }
    }
    ruby_increment_last_segment(from)
}

/// `decrement(v)`: decrement the last segment, borrowing from higher segments
/// as needed.
fn ruby_decrement(v: &str) -> String {
    let segs: Vec<i64> = v.split('.').map(|s| s.parse().unwrap_or(0)).collect();
    let n = segs.len();
    let mut result = segs;
    let mut i = n;
    while i > 0 {
        i -= 1;
        result[i] -= 1;
        if result[i] >= 0 {
            break;
        }
        result[i] = 0;
        // borrow: continue to next higher segment
    }
    result.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(".")
}

// ---------------------------------------------------------------------------
// Range satisfaction
// ---------------------------------------------------------------------------

/// Check whether `ver` satisfies a single `RubyRange` (including its companion).
fn satisfies_ruby_range(ver: &str, r: &RubyRange) -> bool {
    let constraint_str = format!("{}{}{}", r.operator, r.delimiter, r.version);
    if !matches(ver, &constraint_str) {
        return false;
    }
    if let Some(comp) = &r.companion {
        satisfies_ruby_range(ver, comp)
    } else {
        true
    }
}

// ---------------------------------------------------------------------------
// replacePart — mirrors TypeScript replacePart
// ---------------------------------------------------------------------------

fn replace_part(part: &RubyRange, to: &str) -> RubyRange {
    match part.operator.as_str() {
        "<" => RubyRange {
            operator: part.operator.clone(),
            delimiter: part.delimiter.clone(),
            version: ruby_increment(&part.version, to),
            companion: None,
        },
        "<=" => RubyRange {
            operator: part.operator.clone(),
            delimiter: part.delimiter.clone(),
            version: to.to_owned(),
            companion: None,
        },
        "~>" => {
            let new_ver = ruby_floor(ruby_adapt(to, &part.version).as_str());
            if let Some(comp) = &part.companion {
                RubyRange {
                    operator: part.operator.clone(),
                    delimiter: part.delimiter.clone(),
                    version: new_ver,
                    companion: Some(Box::new(RubyRange {
                        operator: comp.operator.clone(),
                        delimiter: comp.delimiter.clone(),
                        version: to.to_owned(),
                        companion: None,
                    })),
                }
            } else {
                RubyRange {
                    operator: part.operator.clone(),
                    delimiter: part.delimiter.clone(),
                    version: new_ver,
                    companion: None,
                }
            }
        }
        ">" => RubyRange {
            operator: part.operator.clone(),
            delimiter: part.delimiter.clone(),
            version: ruby_decrement(to),
            companion: None,
        },
        ">=" | "=" | "" => RubyRange {
            operator: part.operator.clone(),
            delimiter: part.delimiter.clone(),
            version: to.to_owned(),
            companion: None,
        },
        _ => part.clone(), // "!=" and unknown: no change
    }
}

// ---------------------------------------------------------------------------
// Strategy implementations
// ---------------------------------------------------------------------------

fn bump_ranges(ranges: &[RubyRange], to: &str) -> Vec<RubyRange> {
    ranges
        .iter()
        .map(|part| {
            match part.operator.as_str() {
                "<" => {
                    if is_greater_than(to, &part.version) || equals(to, &part.version) {
                        replace_part(part, to)
                    } else {
                        part.clone()
                    }
                }
                "<=" => {
                    if is_greater_than(to, &part.version) {
                        replace_part(part, to)
                    } else {
                        part.clone()
                    }
                }
                "~>" => {
                    let trimmed = ruby_adapt(to, &part.version);
                    if ruby_trim_zeroes(&trimmed) == ruby_trim_zeroes(to) {
                        RubyRange {
                            operator: part.operator.clone(),
                            delimiter: part.delimiter.clone(),
                            version: trimmed,
                            companion: None,
                        }
                    } else {
                        RubyRange {
                            operator: part.operator.clone(),
                            delimiter: part.delimiter.clone(),
                            version: trimmed,
                            companion: Some(Box::new(RubyRange {
                                operator: ">=".to_owned(),
                                delimiter: " ".to_owned(),
                                version: to.to_owned(),
                                companion: None,
                            })),
                        }
                    }
                }
                "!=" => {
                    if is_greater_than(to, &part.version) {
                        RubyRange {
                            operator: ">=".to_owned(),
                            delimiter: part.delimiter.clone(),
                            version: to.to_owned(),
                            companion: None,
                        }
                    } else {
                        part.clone()
                    }
                }
                _ => replace_part(part, to),
            }
        })
        .collect()
}

fn replace_ranges(ranges: &[RubyRange], to: &str) -> Vec<RubyRange> {
    ranges
        .iter()
        .map(|part| {
            if satisfies_ruby_range(to, part) {
                return part.clone();
            }
            let part_seg_count = part.version.split('.').count();
            let to_seg_count = to.split('.').count();
            if part_seg_count > to_seg_count {
                let diff = part_seg_count - to_seg_count;
                let mut padded_to_segs: Vec<&str> = to.split('.').collect();
                padded_to_segs.extend(std::iter::repeat_n("0", diff));
                let padded_to = padded_to_segs.join(".");
                let replacement = replace_part(part, &padded_to);
                // Shorten version by removing last `diff` segments
                let ver_segs: Vec<&str> = replacement.version.split('.').collect();
                let shortened = if ver_segs.len() > diff {
                    ver_segs[..ver_segs.len() - diff].join(".")
                } else {
                    replacement.version.clone()
                };
                RubyRange {
                    operator: replacement.operator,
                    delimiter: replacement.delimiter,
                    version: shortened,
                    companion: replacement.companion,
                }
            } else {
                replace_part(part, to)
            }
        })
        .collect()
}

fn widen_ranges(ranges: &[RubyRange], to: &str) -> Vec<RubyRange> {
    ranges
        .iter()
        .flat_map(|part| {
            if satisfies_ruby_range(to, part) {
                return vec![part.clone()];
            }
            match part.operator.as_str() {
                "~>" => {
                    let base_version = if let Some(comp) = &part.companion {
                        comp.version.clone()
                    } else {
                        part.version.clone()
                    };
                    let upper = ruby_pgte_upper_bound(&part.version);
                    let limit = ruby_increment(&upper, to);
                    vec![
                        RubyRange {
                            operator: ">=".to_owned(),
                            delimiter: " ".to_owned(),
                            version: base_version,
                            companion: None,
                        },
                        RubyRange {
                            operator: "<".to_owned(),
                            delimiter: " ".to_owned(),
                            version: limit,
                            companion: None,
                        },
                    ]
                }
                _ => vec![replace_part(part, to)],
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Public get_new_value entry point
// ---------------------------------------------------------------------------

pub fn get_new_value(
    current_value: &str,
    range_strategy: &str,
    current_version: &str,
    new_version: &str,
) -> Option<String> {
    // Detect quote delimiter (single or double quote wrapping each part)
    let first_char = current_value.chars().next();
    let delimiter: Option<char> = match first_char {
        Some('\'') | Some('"') => first_char,
        _ => None,
    };

    // Strip quotes from content: strip leading/trailing quote from each comma-part.
    // Mirrors vtrim() in TypeScript which strips all quote chars from a string.
    let strip_quotes_from = |s: &str| -> String {
        if delimiter.is_some() {
            s.replace(['\'', '"'], "")
        } else {
            s.to_owned()
        }
    };

    // `content` is the unquoted version of `current_value`
    let content = strip_quotes_from(current_value);
    let cv: &str = content.trim_start_matches('v');
    let nv: &str = new_version.trim_start_matches('v');

    // Compute new_value (unquoted)
    let new_value: Option<String> = if is_version(cv) {
        // Case 1: currentValue is a plain version
        let new_val = if content.starts_with('v') && !new_version.starts_with('v') {
            format!("v{}", nv)
        } else {
            new_version.to_owned()
        };
        Some(new_val)
    } else {
        let cv_stripped = cv.trim_start_matches('=').trim();
        let cur_vtrim = current_version.trim_start_matches('v');
        if cv_stripped == cur_vtrim {
            // Case 2: currentValue stripped of `= ` equals currentVersion
            Some(content.replace(current_version, new_version))
        } else {
            // Case 3: range strategies
            match range_strategy {
                "update-lockfile" => {
                    if matches(nv, cv) {
                        Some(cv.to_owned())
                    } else {
                        // Recurse with replace strategy
                        return get_new_value(current_value, "replace", current_version, new_version);
                    }
                }
                "bump" => Some(stringify_ruby_ranges(&bump_ranges(&parse_ruby_ranges(cv), nv))),
                "auto" | "replace" => Some(stringify_ruby_ranges(&replace_ranges(&parse_ruby_ranges(cv), nv))),
                "widen" => Some(stringify_ruby_ranges(&widen_ranges(&parse_ruby_ranges(cv), nv))),
                _ => None,
            }
        }
    };

    // Re-apply quotes if needed (mirrors TypeScript's end-of-function quote wrapping)
    if let (Some(result), Some(q)) = (&new_value, delimiter) {
        // TypeScript splits on ',' and adds delimiter around each part
        // For multi-part ranges like ">= 3.0.5, < 3.3" it wraps each part
        let quoted = result
            .split(',')
            .map(|part| {
                // Preserve leading whitespace, add quote, then trailing quote
                let trimmed = part.trim();
                format!("{}{}{}", q, trimmed, q)
            })
            .collect::<Vec<_>>()
            .join(", ");
        return Some(quoted);
    }

    new_value
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "equals(\"$a\", \"$b\") === $expected" — modules/versioning/ruby/index.spec.ts line 4
    #[test]
    fn ruby_equals_cases() {
        let trues = [
            ("1.0.0", "1"),
            ("1.2.0", "1.2"),
            ("1.2.0", "1.2.0"),
            ("1.0.0.rc1", "1.0.0.rc1"),
        ];
        for (a, b) in trues {
            assert!(equals(a, b), "equals({a:?}, {b:?})");
        }
        let falses = [
            ("1.2.0", "2"),
            ("1.2.0", "1.1"),
            ("1.2.0", "1.2.1"),
            ("1.0.0.rc1", "1.0.0.rc2"),
        ];
        for (a, b) in falses {
            assert!(!equals(a, b), "!equals({a:?}, {b:?})");
        }
    }

    // Ported: "getMajor, getMinor, getPatch for \"$version\"" — ruby/index.spec.ts line 21
    #[test]
    fn ruby_get_components() {
        assert_eq!(get_major("1"), Some(1));
        assert_eq!(get_minor("1"), None);
        assert_eq!(get_patch("1"), None);
        assert_eq!(get_major("1.2"), Some(1));
        assert_eq!(get_minor("1.2"), Some(2));
        assert_eq!(get_patch("1.2"), None);
        assert_eq!(get_major("1.2.0"), Some(1));
        assert_eq!(get_minor("1.2.0"), Some(2));
        assert_eq!(get_patch("1.2.0"), Some(0));
        assert_eq!(get_major("1.2.0.alpha.4"), Some(1));
        assert_eq!(get_minor("1.2.0.alpha.4"), Some(2));
        assert_eq!(get_patch("1.2.0.alpha.4"), Some(0));
    }

    // Ported: "isVersion(\"$version\") === $expected" — ruby/index.spec.ts line 38
    #[test]
    fn ruby_is_version_cases() {
        for v in [
            "0",
            "v0",
            "v1",
            "v1.2",
            "v1.2.3",
            "1",
            "1.1",
            "1.1.2",
            "1.1.2.3",
            "1.1.2-4",
            "1.1.2.pre.4",
            "v1.1.2.pre.4",
        ] {
            assert!(is_version(v), "is_version({v:?})");
        }
        for v in ["", "v", "tottally-not-a-version"] {
            assert!(!is_version(v), "!is_version({v:?})");
        }
    }

    // Ported: "isGreaterThan(\"$a\", \"$b\") === $expected" — ruby/index.spec.ts line 62
    #[test]
    fn ruby_is_greater_than_cases() {
        for (a, b) in [
            ("2", "1"),
            ("2.2", "2.1"),
            ("2.2.1", "2.2.0"),
            ("3.0.0.rc2", "3.0.0.rc1"),
            ("3.0.0-rc.2", "3.0.0-rc.1"),
            ("3.0.0.rc1", "3.0.0.beta"),
            ("3.0.0-rc.1", "3.0.0-beta"),
            ("3.0.0.beta", "3.0.0.alpha"),
            ("3.0.0-beta", "3.0.0-alpha"),
            ("5.0.1.rc1", "5.0.1.beta1"),
            ("5.0.1-rc.1", "5.0.1-beta.1"),
        ] {
            assert!(is_greater_than(a, b), "{a:?} > {b:?}");
        }
        for (a, b) in [
            ("1", "2"),
            ("2.1", "2.2"),
            ("2.2.0", "2.2.1"),
            ("3.0.0.rc1", "3.0.0.rc2"),
            ("3.0.0-rc.1", "3.0.0-rc.2"),
            ("3.0.0.beta", "3.0.0.rc1"),
            ("3.0.0-beta", "3.0.0-rc.1"),
            ("3.0.0.alpha", "3.0.0.beta"),
            ("3.0.0-alpha", "3.0.0-beta"),
            ("5.0.1.beta1", "5.0.1.rc1"),
            ("5.0.1-beta.1", "5.0.1-rc.1"),
            ("1", "1"),
            ("2.1", "2.1"),
            ("2.2.0", "2.2.0"),
        ] {
            assert!(!is_greater_than(a, b), "!({a:?} > {b:?})");
        }
    }

    // Ported: "isStable(\"$version\") === $expected" — ruby/index.spec.ts line 106
    #[test]
    fn ruby_is_stable() {
        for v in ["1", "1.2", "1.2.3"] {
            assert!(is_stable(v), "is_stable({v:?})");
        }
        for v in ["1.2.0.alpha", "1.2.0.alpha1", "1.2.0-alpha.1"] {
            assert!(!is_stable(v), "!is_stable({v:?})");
        }
    }

    // Ported: "$versions -> sortVersions -> $expected" — ruby/index.spec.ts line 122
    #[test]
    fn ruby_sort_versions() {
        let mut v = vec!["1.2.3-beta", "2.0.1", "1.3.4", "1.2.3"];
        v.sort_by(|a, b| sort_versions(a, b));
        assert_eq!(v, vec!["1.2.3-beta", "1.2.3", "1.3.4", "2.0.1"]);
    }

    // Ported: "minSatisfyingVersion($versions, \"$range\") === \"$expected\"" — ruby/index.spec.ts line 129
    #[test]
    fn ruby_min_satisfying_version() {
        let cases: &[(&[&str], &str, Option<&str>)] = &[
            (&["2.1.5", "2.1.6"], "~> 2.1", Some("2.1.5")),
            (&["2.1.6", "2.1.5"], "~> 2.1.6", Some("2.1.6")),
            (
                &["4.7.3", "4.7.4", "4.7.5", "4.7.9"],
                "~> 4.7, >= 4.7.4",
                Some("4.7.4"),
            ),
            (
                &["2.5.3", "2.5.4", "2.5.5", "2.5.6"],
                "~>2.5.3",
                Some("2.5.3"),
            ),
            (
                &["2.1.0", "3.0.0.beta", "2.3", "3.0.0-rc.1", "3.0.0", "3.1.1"],
                "~> 3.0",
                Some("3.0.0"),
            ),
            (&["1.2.3", "1.2.4"], ">= 3.5.0", None),
        ];
        for (vs, r, exp) in cases {
            assert_eq!(min_satisfying_version(vs, r), *exp, "min({vs:?}, {r:?})");
        }
    }

    // Ported: "getSatisfyingVersion($versions, \"$range\") === \"$expected\"" — ruby/index.spec.ts line 147
    #[test]
    fn ruby_get_satisfying_version() {
        let cases: &[(&[&str], &str, Option<&str>)] = &[
            (&["2.1.5", "2.1.6"], "~> 2.1", Some("2.1.6")),
            (&["2.1.6", "2.1.5"], "~> 2.1.6", Some("2.1.6")),
            (
                &["4.7.3", "4.7.4", "4.7.5", "4.7.9"],
                "~> 4.7, >= 4.7.4",
                Some("4.7.9"),
            ),
            (
                &["2.5.3", "2.5.4", "2.5.5", "2.5.6"],
                "~>2.5.3",
                Some("2.5.6"),
            ),
            (
                &["2.1.0", "3.0.0.beta", "2.3", "3.0.0-rc.1", "3.0.0", "3.1.1"],
                "~> 3.0",
                Some("3.1.1"),
            ),
            (&["1.2.3", "1.2.4"], ">= 3.5.0", None),
        ];
        for (vs, r, exp) in cases {
            assert_eq!(get_satisfying_version(vs, r), *exp, "max({vs:?}, {r:?})");
        }
    }

    // Ported: "matches(\"$version\", \"$range\") === \"$expected\"" — ruby/index.spec.ts line 165
    #[test]
    fn ruby_matches() {
        for (v, r) in [
            ("1.2", ">= 1.2"),
            ("1.2.3", "~> 1.2.1"),
            ("1.2.7", "1.2.7"),
            ("1.1.6", ">= 1.1.5, < 2.0"),
        ] {
            assert!(matches(v, r), "matches({v:?}, {r:?})");
        }
        for (v, r) in [
            ("1.2", ">= 1.3"),
            ("1.3.8", "~> 1.2.1"),
            ("1.3.9", "1.3.8"),
            ("2.0.0", ">= 1.1.5, < 2.0"),
        ] {
            assert!(!matches(v, r), "!matches({v:?}, {r:?})");
        }
    }

    // Ported: "isLessThanRange(\"$version\", \"$range\") === \"$expected\"" — ruby/index.spec.ts line 185
    #[test]
    fn ruby_is_less_than_range() {
        let cases: &[(&str, &str, Option<bool>)] = &[
            ("1.2.2", "< 1.2.2", Some(true)),
            ("1.1.4", ">= 1.1.5, < 2.0", Some(true)),
            ("1.2.0-alpha", "1.2.0-beta", Some(true)),
            ("1.2.2", "> 1.2.2, ~> 2.0.0", Some(true)),
            ("1.2.2", "<= 1.2.2", Some(false)),
            ("2.0.0", ">= 1.1.5, < 2.0", Some(false)),
            ("1.2.0-beta", "1.2.0-alpha", Some(false)),
            ("2.0.0", "> 1.2.2, ~> 2.0.0", Some(false)),
            ("asdf", "> 1.2.2, ~> 2.0.0", None),
        ];
        for (v, r, exp) in cases {
            assert_eq!(is_less_than_range(v, r), *exp, "isLTR({v:?}, {r:?})");
        }
    }

    // Ported: "isValid(\"$version\") === $expected" (version form) — ruby/index.spec.ts line 209
    #[test]
    fn ruby_is_valid_version_form() {
        for v in ["1", "1.2", "1.2.3"] {
            assert!(is_valid(v), "is_valid({v:?})");
        }
        for v in ["^1.2.3", "~1.2.3", "1.2.*", "< 3.0, >= 1.0.0 <= 2.0.0"] {
            assert!(!is_valid(v), "!is_valid({v:?})");
        }
    }

    // Ported: "isValid(\"$version\") === $expected" (range form) — ruby/index.spec.ts line 224
    #[test]
    fn ruby_is_valid_range_form() {
        for v in [
            "= 1",
            "!= 1.1",
            "> 1.1.2",
            "< 1.0.0-beta",
            ">= 1.0.0.beta",
            "<= 1.2.0.alpha1",
            "~> 1.2.0-alpha.1",
            ">= 3.0.5, < 3.2",
        ] {
            assert!(is_valid(v), "is_valid({v:?})");
        }
        for v in ["+ 1", "- 1.1", "=== 1.1.2", "! 1.0.0-beta", "& 1.0.0.beta"] {
            assert!(!is_valid(v), "!is_valid({v:?})");
        }
    }

    // Ported: "isSingleVersion(\"$version\") === $expected" — ruby/index.spec.ts line 247
    #[test]
    fn ruby_is_single_version() {
        for v in [
            "1",
            "1.2",
            "1.2.1",
            "=1",
            "=1.2",
            "=1.2.1",
            "= 1",
            "= 1.2",
            "= 1.2.1",
            "1.2.1.rc1",
            "1.2.1-rc.1",
            "= 1.2.0.alpha",
            "= 1.2.0-alpha",
        ] {
            assert!(is_single_version(v), "is_single({v:?})");
        }
        for v in [
            "!= 1",
            "> 1.2",
            "< 1.2.1",
            ">= 1",
            "<= 1.2",
            "~> 1.2.1",
            "",
            "tottally-not-a-version",
        ] {
            assert!(!is_single_version(v), "!is_single({v:?})");
        }
    }

    // Ported: "returns a pinned value" — ruby/index.spec.ts line 276
    #[test]
    fn ruby_get_pinned_value() {
        assert_eq!(get_pinned_value("1.2.3"), "1.2.3");
        assert_eq!(get_pinned_value("v1.2.3"), "1.2.3");
    }

    // Ported: "getNewValue(\"$currentValue\", \"$rangeStrategy\", \"$currentVersion\", \"$newVersion\") === \"$expected\"" — modules/versioning/ruby/index.spec.ts line 281
    #[test]
    fn ruby_get_new_value_cases() {
        // (currentValue, rangeStrategy, currentVersion, newVersion, expected)
        let cases: &[(&str, &str, &str, &str, &str)] = &[
            ("1.0.3", "pin", "1.0.3", "1.2.3", "1.2.3"),
            ("v1.0.3", "auto", "v1.0.3", "v1.2.3", "v1.2.3"),
            ("'>= 3.0.5', '< 3.2'", "replace", "3.1.5", "3.2.1", "'>= 3.0.5', '< 3.3'"),
            ("'0.0.10'", "auto", "0.0.10", "0.0.11", "'0.0.11'"),
            ("'0.0.10'", "replace", "0.0.10", "0.0.11", "'0.0.11'"),
            (">= 3.2, < 5.0", "bump", "4.0.2", "6.0.1", ">= 6.0.1, < 6.0.2"),
            ("~> 5.2, >= 5.2.5", "bump", "5.3.0", "6.0.0", "~> 6.0"),
            ("~> 5.2, >= 5.2.5", "bump", "5.3.0", "6.0.1", "~> 6.0, >= 6.0.1"),
            ("~> 5.2.0, >= 5.2.5", "bump", "5.2.5", "5.3.1", "~> 5.3.1"),
            ("4.2.0", "bump", "4.2.0", "4.2.5.1", "4.2.5.1"),
            ("4.2.5.1", "bump", "0.1", "4.3.0", "4.3.0"),
            ("~> 1", "bump", "1.2.0", "2.0.3", "~> 2, >= 2.0.3"),
            ("'~> 1'", "bump", "1.2.0", "2.0.3", "'~> 2', '>= 2.0.3'"),
            ("= 5.2.2", "bump", "5.2.2", "5.2.2.1", "= 5.2.2.1"),
            ("1.0.3", "bump", "1.0.3", "1.2.3", "1.2.3"),
            ("v1.0.3", "bump", "1.0.3", "1.2.3", "v1.2.3"),
            ("= 1.0.3", "bump", "1.0.3", "1.2.3", "= 1.2.3"),
            ("!= 1.0.3", "bump", "1.0.0", "1.2.3", ">= 1.2.3"),
            ("!= 1.0.3", "bump", "1.0.0", "1.0.2", "!= 1.0.3"),
            ("!= 1.0.3", "bump", "1.0.0", "1.0.3", "!= 1.0.3"),
            ("> 1.0.3", "bump", "1.0.4", "1.2.3", "> 1.2.2"),
            ("> 1.2.3", "bump", "1.0.0", "1.0.3", "> 1.0.2"),
            ("< 1.0.3", "bump", "1.0.0", "1.2.3", "< 1.2.4"),
            ("< 1.2.3", "bump", "1.0.0", "1.0.3", "< 1.2.3"),
            ("< 1.2.2", "bump", "1.0.0", "1.2.3", "< 1.2.4"),
            ("< 1.2.3", "bump", "1.0.0", "1.2.3", "< 1.2.4"),
            ("< 1.2", "bump", "1.0.0", "1.2.3", "< 1.3"),
            ("< 1", "bump", "0.9.0", "1.2.3", "< 2"),
            ("< 1.2.3", "bump", "1.0.0", "1.2.2", "< 1.2.3"),
            (">= 1.0.3", "bump", "1.0.3", "1.2.3", ">= 1.2.3"),
            (">= 1.0.3", "bump", "1.0.3", "1.0.2", ">= 1.0.2"),
            ("<= 1.0.3", "bump", "1.0.3", "1.2.3", "<= 1.2.3"),
            ("<= 1.0.3", "bump", "1.0.0", "1.0.2", "<= 1.0.3"),
            ("~> 1.0.3", "bump", "1.0.3", "1.2.3", "~> 1.2.3"),
            ("~> 1.0.3", "bump", "1.0.3", "1.0.4", "~> 1.0.4"),
            ("~> 4.7, >= 4.7.4", "bump", "4.7.5", "4.7.9", "~> 4.7, >= 4.7.9"),
            ("~> 4.7, >= 4.7.4", "bump", "4.7.5", "4.8.0", "~> 4.8"),
            (">= 2.0.0, <= 2.15", "bump", "2.15.0", "2.20.1", ">= 2.20.1, <= 2.20.1"),
            ("~> 5.2.0", "bump", "5.2.4.1", "6.0.2.1", "~> 6.0.2, >= 6.0.2.1"),
            ("~> 4.0, < 5", "bump", "4.7.5", "5.0.0", "~> 5.0, < 6"),
            ("~> 4.0, < 5", "bump", "4.7.5", "5.0.1", "~> 5.0, >= 5.0.1, < 6"),
            ("~> 4.0, < 5", "bump", "4.7.5", "5.1.0", "~> 5.1, < 6"),
            (">= 3.2, < 5.0", "replace", "4.0.2", "6.0.1", ">= 3.2, < 6.0.2"),
            ("~> 5.2, >= 5.2.5", "replace", "5.3.0", "6.0.0", "~> 6.0, >= 6.0.0"),
            ("~> 5.2, >= 5.2.5", "replace", "5.3.0", "6.0.1", "~> 6.0, >= 6.0.1"),
            ("~> 5.2.0, >= 5.2.5", "replace", "5.2.5", "5.3.1", "~> 5.3.0, >= 5.3.1"),
            ("4.2.0", "replace", "4.2.0", "4.2.5.1", "4.2.5.1"),
            ("4.2.5.1", "replace", "0.1", "4.3.0", "4.3.0"),
            ("4.2.5.1", "replace", "0.1", "4.2.6", "4.2.6"),
            ("~> 4.2", "replace", "0.1", "4.2.5.1", "~> 4.2"),
            ("~> 4.2", "replace", "4.2.5.2", "4.2.5.1", "~> 4.2"),
            ("~> 4.2.5", "replace", "0.1", "4.2.5.1", "~> 4.2.5"),
            ("~> 4.2.5", "replace", "0.1", "4.3.0.1", "~> 4.3.0"),
            ("~> 4.2.5.1", "replace", "0.1", "4.2.6", "~> 4.2.6"),
            ("~> 4.2.5.1", "replace", "4.2.5.2", "4.2.6", "~> 4.2.6"),
            ("~> 1", "replace", "1.2.0", "2.0.3", "~> 2"),
            ("= 5.2.2", "replace", "5.2.2", "5.2.2.1", "= 5.2.2.1"),
            ("1.0.3", "replace", "1.0.3", "1.2.3", "1.2.3"),
            ("v1.0.3", "replace", "1.0.3", "1.2.3", "v1.2.3"),
            ("= 1.0.3", "replace", "1.0.3", "1.2.3", "= 1.2.3"),
            ("!= 1.0.3", "replace", "1.0.0", "1.2.3", "!= 1.0.3"),
            ("!= 1.0.3", "replace", "1.0.0", "1.0.2", "!= 1.0.3"),
            ("!= 1.0.3", "replace", "1.0.0", "1.0.3", "!= 1.0.3"),
            ("> 1.0.3", "replace", "1.0.4", "1.2.3", "> 1.0.3"),
            ("> 1.2.3", "replace", "1.0.0", "1.0.3", "> 1.0.2"),
            ("< 1.0.3", "replace", "1.0.0", "1.2.3", "< 1.2.4"),
            ("< 1.2.3", "replace", "1.0.0", "1.0.3", "< 1.2.3"),
            ("< 1.2.2", "replace", "1.0.0", "1.2.3", "< 1.2.4"),
            ("< 1.2.3", "replace", "1.0.0", "1.2.3", "< 1.2.4"),
            ("< 1.2", "replace", "1.0.0", "1.2.3", "< 1.3"),
            ("< 1", "replace", "0.9.0", "1.2.3", "< 2"),
            ("< 1.2.3", "replace", "1.0.0", "1.2.2", "< 1.2.3"),
            (">= 1.0.3", "replace", "1.0.3", "1.2.3", ">= 1.0.3"),
            (">= 1.0.3", "replace", "1.0.3", "1.0.2", ">= 1.0.2"),
            ("<= 1.0.3", "replace", "1.0.0", "1.2.3", "<= 1.2.3"),
            ("<= 1.0.3", "replace", "1.0.0", "1.0.2", "<= 1.0.3"),
            ("~> 1.0.3", "replace", "1.0.0", "1.2.3", "~> 1.2.0"),
            ("~> 1.0.3", "replace", "1.0.0", "1.0.4", "~> 1.0.3"),
            ("~> 4.7, >= 4.7.4", "replace", "1.0.0", "4.7.9", "~> 4.7, >= 4.7.4"),
            ("~> 4.7, >= 4.7.4", "replace", "4.7.5", "4.8.0", "~> 4.7, >= 4.7.4"),
            (">= 2.0.0, <= 2.15", "replace", "2.15.0", "2.20.1", ">= 2.0.0, <= 2.20.1"),
            ("~> 5.2.0", "replace", "5.2.4.1", "6.0.2.1", "~> 6.0.0"),
            ("~> 4.0, < 5", "replace", "4.7.5", "5.0.0", "~> 5.0, < 6"),
            ("~> 4.0, < 5", "replace", "4.7.5", "5.0.1", "~> 5.0, < 6"),
            ("~> 4.0, < 5", "replace", "4.7.5", "5.1.0", "~> 5.0, < 6"),
            (">= 3.2, < 5.0", "widen", "4.0.2", "6.0.1", ">= 3.2, < 6.0.2"),
            ("~> 5.2, >= 5.2.5", "widen", "5.3.0", "6.0.0", ">= 5.2.5, < 7"),
            ("~> 5.2, >= 5.2.5", "widen", "5.3.0", "6.0.1", ">= 5.2.5, < 7"),
            ("~> 5.2.0, >= 5.2.5", "widen", "5.2.5", "5.3.1", ">= 5.2.5, < 5.4"),
            ("4.2.0", "widen", "4.2.0", "4.2.5.1", "4.2.5.1"),
            ("4.2.5.1", "widen", "0.1", "4.3.0", "4.3.0"),
            ("~> 1", "widen", "1.2.0", "2.0.3", ">= 1, < 3"),
            ("= 5.2.2", "widen", "5.2.2", "5.2.2.1", "= 5.2.2.1"),
            ("1.0.3", "widen", "1.0.3", "1.2.3", "1.2.3"),
            ("v1.0.3", "widen", "1.0.3", "1.2.3", "v1.2.3"),
            ("= 1.0.3", "widen", "1.0.3", "1.2.3", "= 1.2.3"),
            ("!= 1.0.3", "widen", "1.0.0", "1.2.3", "!= 1.0.3"),
            ("!= 1.0.3", "widen", "1.0.0", "1.0.2", "!= 1.0.3"),
            ("!= 1.0.3", "widen", "1.0.0", "1.0.3", "!= 1.0.3"),
            ("> 1.0.3", "widen", "1.0.4", "1.2.3", "> 1.0.3"),
            ("> 1.2.3", "widen", "1.0.0", "1.0.3", "> 1.0.2"),
            ("< 1.0.3", "widen", "1.0.0", "1.2.3", "< 1.2.4"),
            ("< 1.2.3", "widen", "1.0.0", "1.0.3", "< 1.2.3"),
            ("< 1.2.2", "widen", "1.0.0", "1.2.3", "< 1.2.4"),
            ("< 1.2.3", "widen", "1.0.0", "1.2.3", "< 1.2.4"),
            ("< 1.2", "widen", "1.0.0", "1.2.3", "< 1.3"),
            ("< 1", "widen", "0.9.0", "1.2.3", "< 2"),
            ("< 1.2.3", "widen", "1.0.0", "1.2.2", "< 1.2.3"),
            (">= 1.0.3", "widen", "1.0.3", "1.2.3", ">= 1.0.3"),
            (">= 1.0.3", "widen", "1.0.3", "1.0.2", ">= 1.0.2"),
            ("<= 1.0.3", "widen", "1.0.0", "1.2.3", "<= 1.2.3"),
            ("<= 1.0.3", "widen", "1.0.0", "1.0.2", "<= 1.0.3"),
            ("~> 1.0.3", "widen", "1.0.0", "1.2.3", ">= 1.0.3, < 1.2.4"),
            ("~> 1.0.3", "widen", "1.0.0", "1.0.4", "~> 1.0.3"),
            ("~> 4.7, >= 4.7.4", "widen", "1.0.0", "4.7.9", "~> 4.7, >= 4.7.4"),
            ("~> 4.7, >= 4.7.4", "widen", "4.7.5", "4.8.0", "~> 4.7, >= 4.7.4"),
            (">= 2.0.0, <= 2.15", "widen", "2.15.0", "2.20.1", ">= 2.0.0, <= 2.20.1"),
            ("~> 5.2.0", "widen", "5.2.4.1", "6.0.2.1", ">= 5.2.0, < 6.0.3"),
            ("~> 4.0, < 5", "widen", "4.7.5", "5.0.0", ">= 4.0, < 6, < 6"),
            ("~> 4.0, < 5", "widen", "4.7.5", "5.0.1", ">= 4.0, < 6, < 6"),
            ("~> 4.0, < 5", "widen", "4.7.5", "5.1.0", ">= 4.0, < 6, < 6"),
            ("< 1.0.3", "auto", "1.0.3", "1.2.4", "< 1.2.5"),
            ("< 1.0.3", "replace", "1.0.3", "1.2.4", "< 1.2.5"),
            ("< 1.0.3", "widen", "1.0.3", "1.2.4", "< 1.2.5"),
            ("< 1.0.3", "replace", "1.0.3", "1.2.4", "< 1.2.5"),
            ("~> 6.0.0", "update-lockfile", "6.0.2", "6.0.3", "~> 6.0.0"),
            ("~> 6.0.0", "update-lockfile", "6.0.2", "7.0.0", "~> 7.0.0"),
            ("\"~> 6.0.0\"", "update-lockfile", "6.0.2", "7.0.0", "\"~> 7.0.0\""),
        ];
        for &(cv, strat, cur, nv, expected) in cases {
            let got = get_new_value(cv, strat, cur, nv).unwrap_or_default();
            assert_eq!(got, expected, "get_new_value({cv:?}, {strat:?}, {cur:?}, {nv:?})");
        }
    }
}
