use std::sync::LazyLock;

use regex::Regex;
use semver::{Op, Version, VersionReq};

use crate::versioning::pep440;

// ── Regex patterns ────────────────────────────────────────────────────────────

const VG: &str = r"[0-9a-zA-Z_]+(?:[.\-][0-9a-zA-Z_]+)*";

static RE_MATCH_VERSION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!(r"^(?P<v>{VG})$")).unwrap());

static RE_EXACT_VERSION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!(r"^==(?P<v>{VG})?$")).unwrap());

static RE_INCLUSIVE_BOUND: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!(r"^(?P<lower>{VG})?\.\.(?P<upper>{VG})?$")).unwrap());

static RE_ASC_PLUS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"^(?P<lver>{VG})\+(?P<comma>,?)(?P<upfx><=|<)(?P<uver>{VG})$"
    ))
    .unwrap()
});

static RE_ASC_GTE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"^(?P<lpfx>>|>=)(?P<lver>{VG})(?P<comma>,?)(?P<upfx><=|<)(?P<uver>{VG})$"
    ))
    .unwrap()
});

static RE_DESC: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        r"^(?P<upfx><=|<)(?P<uver>{VG}),(?P<lpfx>>|>=)(?P<lver>{VG})$"
    ))
    .unwrap()
});

static RE_LOWER_BOUND: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(r"^(?P<prefix>>|>=)?(?P<ver>{VG})?(?P<plus>\+)?$")).unwrap()
});

static RE_UPPER_BOUND: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!(r"^(?P<prefix><=|<)(?P<ver>{VG})?$")).unwrap());

static RE_VG_FIRST: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!(r"(?P<v>{VG})")).unwrap());

// ── Range type ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum RezRange {
    MatchVersion(String),
    ExactVersion(String),
    InclusiveBound {
        lower: Option<String>,
        upper: Option<String>,
    },
    LowerBound {
        prefix: String,
        version: Option<String>,
        has_plus: bool,
    },
    UpperBound {
        prefix: String,
        version: Option<String>,
    },
    AscPlus {
        lower_ver: String,
        comma: bool,
        upper_pfx: String,
        upper_ver: String,
    },
    AscGte {
        lower_pfx: String,
        lower_ver: String,
        comma: bool,
        upper_pfx: String,
        upper_ver: String,
    },
    Desc {
        upper_pfx: String,
        upper_ver: String,
        lower_pfx: String,
        lower_ver: String,
    },
}

fn detect_rez_range(input: &str) -> Option<RezRange> {
    let s = input.trim();

    if let Some(caps) = RE_MATCH_VERSION.captures(s) {
        return Some(RezRange::MatchVersion(caps["v"].to_string()));
    }

    if let Some(caps) = RE_EXACT_VERSION.captures(s) {
        let v = caps.name("v").map_or("", |m| m.as_str()).to_owned();
        return Some(RezRange::ExactVersion(v));
    }

    if let Some(caps) = RE_INCLUSIVE_BOUND.captures(s) {
        return Some(RezRange::InclusiveBound {
            lower: caps.name("lower").map(|m| m.as_str().to_owned()),
            upper: caps.name("upper").map(|m| m.as_str().to_owned()),
        });
    }

    if let Some(caps) = RE_ASC_PLUS.captures(s) {
        return Some(RezRange::AscPlus {
            lower_ver: caps["lver"].to_string(),
            comma: !caps["comma"].is_empty(),
            upper_pfx: caps["upfx"].to_string(),
            upper_ver: caps["uver"].to_string(),
        });
    }

    if let Some(caps) = RE_ASC_GTE.captures(s) {
        return Some(RezRange::AscGte {
            lower_pfx: caps["lpfx"].to_string(),
            lower_ver: caps["lver"].to_string(),
            comma: !caps["comma"].is_empty(),
            upper_pfx: caps["upfx"].to_string(),
            upper_ver: caps["uver"].to_string(),
        });
    }

    if let Some(caps) = RE_DESC.captures(s) {
        return Some(RezRange::Desc {
            upper_pfx: caps["upfx"].to_string(),
            upper_ver: caps["uver"].to_string(),
            lower_pfx: caps["lpfx"].to_string(),
            lower_ver: caps["lver"].to_string(),
        });
    }

    if let Some(caps) = RE_LOWER_BOUND.captures(s) {
        let prefix = caps.name("prefix").map_or("", |m| m.as_str()).to_owned();
        let version = caps.name("ver").map(|m| m.as_str().to_owned());
        let has_plus = caps.name("plus").is_some_and(|m| !m.as_str().is_empty());
        if prefix.is_empty() && version.is_none() && !has_plus {
            return None;
        }
        return Some(RezRange::LowerBound {
            prefix,
            version,
            has_plus,
        });
    }

    if let Some(caps) = RE_UPPER_BOUND.captures(s) {
        return Some(RezRange::UpperBound {
            prefix: caps["prefix"].to_string(),
            version: caps.name("ver").map(|m| m.as_str().to_owned()),
        });
    }

    None
}

// ── Transform helpers ─────────────────────────────────────────────────────────

fn pad_zeroes(input: &str) -> String {
    if input.contains(['~', '^', '*']) {
        return input.to_owned();
    }
    let (base, stability) = if let Some(idx) = input.find('-') {
        (&input[..idx], format!("-{}", &input[idx + 1..]))
    } else {
        (input, String::new())
    };
    let mut parts: Vec<&str> = base.split('.').collect();
    while parts.len() < 3 {
        parts.push("0");
    }
    format!("{}{}", parts.join("."), stability)
}

fn rez2npm(input: &str) -> String {
    match detect_rez_range(input) {
        Some(RezRange::MatchVersion(v)) => v,
        Some(RezRange::ExactVersion(v)) => format!("={}", v),
        Some(RezRange::InclusiveBound { lower, upper }) => {
            let lo = lower.as_deref().unwrap_or("");
            let hi = upper.as_deref().unwrap_or("");
            format!(">={} <{}", lo, hi)
        }
        Some(RezRange::LowerBound {
            prefix,
            version,
            has_plus,
        }) => {
            let v = version.as_deref().unwrap_or("");
            if has_plus {
                format!(">={}", v)
            } else {
                format!("{}{}", prefix, v)
            }
        }
        Some(RezRange::UpperBound { prefix, version }) => {
            format!("{}{}", prefix, version.as_deref().unwrap_or(""))
        }
        Some(RezRange::AscPlus {
            lower_ver,
            upper_pfx,
            upper_ver,
            ..
        }) => {
            format!(">={} {}{}", lower_ver, upper_pfx, upper_ver)
        }
        Some(RezRange::AscGte {
            lower_pfx,
            lower_ver,
            upper_pfx,
            upper_ver,
            ..
        }) => {
            format!("{}{} {}{}", lower_pfx, lower_ver, upper_pfx, upper_ver)
        }
        Some(RezRange::Desc {
            upper_pfx,
            upper_ver,
            lower_pfx,
            lower_ver,
        }) => {
            format!("{}{} {}{}", lower_pfx, lower_ver, upper_pfx, upper_ver)
        }
        None => input.to_owned(),
    }
}

fn rez2pep440(input: &str) -> String {
    match detect_rez_range(input) {
        Some(RezRange::MatchVersion(v)) => v,
        Some(RezRange::ExactVersion(v)) => format!("=={}", v),
        Some(RezRange::InclusiveBound { lower, upper }) => {
            let lo = lower.as_deref().unwrap_or("");
            let hi = upper.as_deref().unwrap_or("");
            format!(">={}, <{}", lo, hi)
        }
        Some(RezRange::LowerBound {
            prefix,
            version,
            has_plus,
        }) => {
            let v = version.as_deref().unwrap_or("");
            if has_plus {
                format!(">={}", v)
            } else {
                format!("{}{}", prefix, v)
            }
        }
        Some(RezRange::UpperBound { prefix, version }) => {
            format!("{}{}", prefix, version.as_deref().unwrap_or(""))
        }
        Some(RezRange::AscPlus {
            lower_ver,
            upper_pfx,
            upper_ver,
            ..
        }) => {
            format!(">={}, {}{}", lower_ver, upper_pfx, upper_ver)
        }
        Some(RezRange::AscGte {
            lower_pfx,
            lower_ver,
            upper_pfx,
            upper_ver,
            ..
        }) => {
            format!("{}{}, {}{}", lower_pfx, lower_ver, upper_pfx, upper_ver)
        }
        Some(RezRange::Desc {
            upper_pfx,
            upper_ver,
            lower_pfx,
            lower_ver,
        }) => {
            // Descending in rez = ascending order for pep440
            format!("{}{}, {}{}", lower_pfx, lower_ver, upper_pfx, upper_ver)
        }
        None => input.to_owned(),
    }
}

fn pep4402rez_inclusive_bound(pep440_val: &str) -> String {
    pep440_val
        .split(',')
        .map(|part| {
            part.trim()
                .trim_start_matches(['<', '>', '='])
        })
        .collect::<Vec<_>>()
        .join("..")
}

fn npm2rezplus(input: &str) -> String {
    format!(
        "{}+",
        input.trim().strip_prefix(">=").unwrap_or(input.trim())
    )
}

fn extract_first_version_group(s: &str) -> String {
    RE_VG_FIRST
        .find(s)
        .map(|m| m.as_str().to_owned())
        .unwrap_or_default()
}

// ── Semver matching helpers ───────────────────────────────────────────────────

fn pad_partial_version(v: &str) -> String {
    
    match v.split('.').count() {
        1 => format!("{}.0.0", v),
        2 => format!("{}.0", v),
        _ => v.to_owned(),
    }
}

fn normalize_constraint(c: &str) -> Option<String> {
    let c = c.trim();
    if c.is_empty() {
        return None;
    }
    let (op, ver) = if let Some(rest) = c.strip_prefix(">=") {
        (">=", rest)
    } else if let Some(rest) = c.strip_prefix('>') {
        (">", rest)
    } else if let Some(rest) = c.strip_prefix("<=") {
        ("<=", rest)
    } else if let Some(rest) = c.strip_prefix('<') {
        ("<", rest)
    } else if let Some(rest) = c.strip_prefix('=') {
        ("=", rest)
    } else {
        ("", c)
    };
    Some(format!("{}{}", op, pad_partial_version(ver)))
}

fn npm_to_version_req(npm_range: &str) -> Option<VersionReq> {
    let trimmed = npm_range.trim();

    // Bare version (no operators) → wildcard range
    if !trimmed.contains('<')
        && !trimmed.contains('>')
        && !trimmed.contains('=')
        && !trimmed.contains(' ')
        && !trimmed.contains(',')
    {
        let parts: Vec<&str> = trimmed.split('.').collect();
        let req_str = match parts.len() {
            1 => {
                let m: u64 = parts[0].parse().ok()?;
                format!(">={}.0.0,<{}.0.0", m, m + 1)
            }
            2 => {
                let m: u64 = parts[0].parse().ok()?;
                let n: u64 = parts[1].parse().ok()?;
                format!(">={}.{}.0,<{}.{}.0", m, n, m, n + 1)
            }
            _ => format!("={}", pad_partial_version(trimmed)),
        };
        return VersionReq::parse(&req_str).ok();
    }

    // Split on whitespace and/or commas
    let constraints: Vec<&str> = trimmed
        .split([' ', ','])
        .filter(|s| !s.is_empty())
        .collect();

    if constraints.is_empty() {
        return None;
    }

    let processed: Option<Vec<String>> = constraints
        .iter()
        .map(|c| normalize_constraint(c))
        .collect();

    let processed = processed?;
    VersionReq::parse(&processed.join(",")).ok()
}

fn version_satisfies_npm_range(version: &Version, npm_range: &str) -> bool {
    match npm_to_version_req(npm_range) {
        Some(req) => req.matches(version),
        None => false,
    }
}

// ── Validity helpers ──────────────────────────────────────────────────────────

fn is_valid_rez_version_str(v: &str) -> bool {
    let stripped = v.strip_prefix(|c: char| c == 'v' || c == 'V').unwrap_or(v);

    if Version::parse(stripped).is_ok() {
        return true;
    }

    // Allow partial versions (1, 1.2) and pre-release tags — but no leading zeros
    let (base, _stability) = if let Some(idx) = stripped.find('-') {
        (&stripped[..idx], Some(&stripped[idx + 1..]))
    } else {
        (stripped, None)
    };

    let parts: Vec<&str> = base.split('.').collect();
    if parts.is_empty() {
        return false;
    }
    parts.iter().all(|p| {
        if let Ok(n) = p.parse::<u64>() {
            n.to_string() == *p // no leading zeros
        } else {
            // Non-numeric component must start with a letter (not a digit like "3foo")
            p.starts_with(|c: char| c.is_alphabetic() || c == '_')
                && p.chars()
                    .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
                && !p.is_empty()
        }
    })
}

fn is_valid_range_content(range: &RezRange) -> bool {
    match range {
        RezRange::MatchVersion(v) => is_valid_rez_version_str(v),
        RezRange::ExactVersion(v) => v.is_empty() || is_valid_rez_version_str(v),
        RezRange::InclusiveBound { lower, upper } => {
            lower.as_deref().is_none_or(is_valid_rez_version_str)
                && upper.as_deref().is_none_or(is_valid_rez_version_str)
        }
        RezRange::LowerBound { version, .. } => {
            version.as_deref().is_none_or(is_valid_rez_version_str)
        }
        RezRange::UpperBound { version, .. } => {
            version.as_deref().is_none_or(is_valid_rez_version_str)
        }
        RezRange::AscPlus {
            lower_ver,
            upper_ver,
            ..
        } => is_valid_rez_version_str(lower_ver) && is_valid_rez_version_str(upper_ver),
        RezRange::AscGte {
            lower_ver,
            upper_ver,
            ..
        } => is_valid_rez_version_str(lower_ver) && is_valid_rez_version_str(upper_ver),
        RezRange::Desc {
            upper_ver,
            lower_ver,
            ..
        } => is_valid_rez_version_str(upper_ver) && is_valid_rez_version_str(lower_ver),
    }
}

// ── getNewValue helpers ───────────────────────────────────────────────────────

fn compute_asc_new_value(
    pep440_value: &str,
    lower_current: &str,
    lower_ver: &str,
    upper_current: &str,
    upper_ver: &str,
    comma: bool,
) -> Option<String> {
    let parts: Vec<&str> = pep440_value.split(", ").collect();
    if parts.len() != 2 {
        return None;
    }
    let lower_ver_new = extract_first_version_group(parts[0]);
    let upper_ver_new = extract_first_version_group(parts[1]);

    let lower_new = lower_current.replacen(lower_ver, &lower_ver_new, 1);
    let upper_new = upper_current.replacen(upper_ver, &upper_ver_new, 1);
    let sep = if comma { "," } else { "" };
    Some(format!("{}{}{}", lower_new, sep, upper_new))
}

fn compute_desc_new_value(
    pep440_value: &str,
    upper_current: &str,
    upper_ver: &str,
    lower_current: &str,
    lower_ver: &str,
) -> Option<String> {
    let parts: Vec<&str> = pep440_value.split(", ").collect();
    if parts.len() != 2 {
        return None;
    }
    // pep440 is in lower, upper order (ascending)
    let lower_ver_new = extract_first_version_group(parts[0]);
    let upper_ver_new = extract_first_version_group(parts[1]);

    let upper_new = upper_current.replacen(upper_ver, &upper_ver_new, 1);
    let lower_new = lower_current.replacen(lower_ver, &lower_ver_new, 1);
    Some(format!("{},{}", upper_new, lower_new))
}

// ── Public API ────────────────────────────────────────────────────────────────

pub fn is_valid(input: &str) -> bool {
    detect_rez_range(input.trim())
        .as_ref()
        .is_some_and(is_valid_range_content)
}

pub fn is_version(input: &str) -> bool {
    let padded = pad_zeroes(&rez2npm(input));
    let stripped = padded
        .strip_prefix(|c: char| c == 'v' || c == 'V')
        .unwrap_or(&padded);
    Version::parse(stripped).is_ok()
}

pub fn is_single_version(constraint: &str) -> bool {
    let c = constraint.trim();
    if let Some(rest) = c.strip_prefix("==") {
        return is_version(rest.trim());
    }
    is_version(c)
}

pub fn is_stable(version: &str) -> bool {
    let padded = pad_zeroes(version);
    match Version::parse(&padded) {
        Ok(v) => v.pre.is_empty(),
        Err(_) => false,
    }
}

pub fn equals(a: &str, b: &str) -> bool {
    let pa = pad_zeroes(a);
    let pb = pad_zeroes(b);
    match (Version::parse(&pa), Version::parse(&pb)) {
        (Ok(va), Ok(vb)) => va == vb,
        _ => false,
    }
}

pub fn get_major(version: &str) -> Option<i64> {
    let padded = pad_zeroes(version);
    Version::parse(&padded).ok().map(|v| v.major as i64)
}

pub fn get_minor(version: &str) -> Option<i64> {
    let padded = pad_zeroes(version);
    Version::parse(&padded).ok().map(|v| v.minor as i64)
}

pub fn get_patch(version: &str) -> Option<i64> {
    let padded = pad_zeroes(version);
    Version::parse(&padded).ok().map(|v| v.patch as i64)
}

pub fn is_greater_than(a: &str, b: &str) -> bool {
    let pa = pad_zeroes(a);
    let pb = pad_zeroes(b);
    match (Version::parse(&pa), Version::parse(&pb)) {
        (Ok(va), Ok(vb)) => va > vb,
        _ => false,
    }
}

pub fn is_less_than_range(version: &str, range: &str) -> bool {
    let padded = pad_zeroes(version);
    let Ok(ver) = Version::parse(&padded) else { return false };
    let npm = rez2npm(range);
    let Some(req) = npm_to_version_req(&npm) else { return false };
    // If version satisfies range, it is not less than it
    if req.matches(&ver) {
        return false;
    }
    // Find the lower bound to determine which side of the range we're on
    for comp in &req.comparators {
        if matches!(comp.op, Op::GreaterEq | Op::Greater) {
            let lower = Version {
                major: comp.major,
                minor: comp.minor.unwrap_or(0),
                patch: comp.patch.unwrap_or(0),
                pre: comp.pre.clone(),
                build: semver::BuildMetadata::EMPTY,
            };
            return ver < lower;
        }
    }
    false
}

pub fn matches_range(version: &str, range: &str) -> bool {
    let padded = pad_zeroes(version);
    let Ok(ver) = Version::parse(&padded) else { return false };
    let npm = rez2npm(range);
    version_satisfies_npm_range(&ver, &npm)
}

pub fn sort_versions(a: &str, b: &str) -> i32 {
    let pa = pad_zeroes(a);
    let pb = pad_zeroes(b);
    match (Version::parse(&pa), Version::parse(&pb)) {
        (Ok(va), Ok(vb)) => match va.cmp(&vb) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        },
        _ => 0,
    }
}

pub fn get_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    let npm = rez2npm(range);
    versions
        .iter()
        .filter_map(|v| {
            let padded = pad_zeroes(v);
            Version::parse(&padded).ok().map(|parsed| (*v, parsed))
        })
        .filter(|(_, parsed)| version_satisfies_npm_range(parsed, &npm))
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(v, _)| v)
}

pub fn min_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    let npm = rez2npm(range);
    versions
        .iter()
        .filter_map(|v| {
            let padded = pad_zeroes(v);
            Version::parse(&padded).ok().map(|parsed| (*v, parsed))
        })
        .filter(|(_, parsed)| version_satisfies_npm_range(parsed, &npm))
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(v, _)| v)
}

pub fn is_compatible(version: &str) -> bool {
    is_version(version)
}

pub fn get_new_value(
    current_value: &str,
    range_strategy: &str,
    current_version: &str,
    new_version: &str,
) -> Option<String> {
    let pep440_range = rez2pep440(current_value);
    let pep440_opt = pep440::get_new_value(&pep440::NewValueParams {
        current_value: pep440_range,
        range_strategy: range_strategy.to_owned(),
        current_version: current_version.to_owned(),
        new_version: new_version.to_owned(),
        is_replacement: false,
    });

    match detect_rez_range(current_value)? {
        RezRange::ExactVersion(_) | RezRange::MatchVersion(_) => {
            // For exact ==X.Y.Z and simple versions: return pep440 result directly
            pep440_opt
        }
        RezRange::InclusiveBound { .. } => {
            let pep440_value = pep440_opt?;
            Some(pep4402rez_inclusive_bound(&pep440_value))
        }
        RezRange::LowerBound { has_plus, .. } => {
            let pep440_value = pep440_opt?;
            if has_plus {
                Some(npm2rezplus(&pep440_value))
            } else {
                Some(pep440_value)
            }
        }
        RezRange::UpperBound { .. } => pep440_opt,
        RezRange::AscPlus {
            lower_ver,
            comma,
            upper_pfx,
            upper_ver,
        } => {
            let pep440_value = pep440_opt?;
            let lower_current = format!("{}+", lower_ver);
            let upper_current = format!("{}{}", upper_pfx, upper_ver);
            compute_asc_new_value(
                &pep440_value,
                &lower_current,
                &lower_ver,
                &upper_current,
                &upper_ver,
                comma,
            )
        }
        RezRange::AscGte {
            lower_pfx,
            lower_ver,
            comma,
            upper_pfx,
            upper_ver,
        } => {
            let pep440_value = pep440_opt?;
            let lower_current = format!("{}{}", lower_pfx, lower_ver);
            let upper_current = format!("{}{}", upper_pfx, upper_ver);
            compute_asc_new_value(
                &pep440_value,
                &lower_current,
                &lower_ver,
                &upper_current,
                &upper_ver,
                comma,
            )
        }
        RezRange::Desc {
            upper_pfx,
            upper_ver,
            lower_pfx,
            lower_ver,
        } => {
            let pep440_value = pep440_opt?;
            let upper_current = format!("{}{}", upper_pfx, upper_ver);
            let lower_current = format!("{}{}", lower_pfx, lower_ver);
            compute_desc_new_value(
                &pep440_value,
                &upper_current,
                &upper_ver,
                &lower_current,
                &lower_ver,
            )
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "equals("$version", "$equal") === $expected" — versioning/rez/index.spec.ts line 5
    #[test]
    fn equals_table() {
        let cases = [
            ("1", "1", true),
            ("1.0", "1", true),
            ("1.0.0", "1", true),
            ("1.9.0", "1.9", true),
            ("1", "2", false),
            ("1.9.1", "1.9", false),
            ("1.9-beta", "1.9", false),
        ];
        for (version, equal, expected) in cases {
            assert_eq!(
                equals(version, equal),
                expected,
                "equals({version:?}, {equal:?})"
            );
        }
    }

    // Ported: "getMajor("$version") === $expected" — versioning/rez/index.spec.ts line 21
    #[test]
    fn get_major_table() {
        let cases = [("1", 1i64), ("1.9", 1), ("1.9.0", 1)];
        for (version, expected) in cases {
            assert_eq!(get_major(version), Some(expected), "getMajor({version:?})");
        }
    }

    // Ported: "getMinor("$version") === $expected" — versioning/rez/index.spec.ts line 30
    #[test]
    fn get_minor_table() {
        let cases = [("1", 0i64), ("1.9", 9), ("1.9.0", 9)];
        for (version, expected) in cases {
            assert_eq!(get_minor(version), Some(expected), "getMinor({version:?})");
        }
    }

    // Ported: "getPatch("$version") === $expected" — versioning/rez/index.spec.ts line 39
    #[test]
    fn get_patch_table() {
        let cases = [("1", 0i64), ("1.9", 0), ("1.9.0", 0), ("1.9.4", 4)];
        for (version, expected) in cases {
            assert_eq!(get_patch(version), Some(expected), "getPatch({version:?})");
        }
    }

    // Ported: "isGreaterThan("$version", "$other") === $expected" — versioning/rez/index.spec.ts line 49
    #[test]
    fn is_greater_than_table() {
        let cases = [
            ("2", "1", true),
            ("2.0", "1", true),
            ("2.0.0", "1", true),
            ("1.10.0", "1.9", true),
            ("1.9", "1.9-beta", true),
            ("1", "1", false),
            ("1.0", "1", false),
            ("1.0.0", "1", false),
            ("1.9.0", "1.9", false),
        ];
        for (version, other, expected) in cases {
            assert_eq!(
                is_greater_than(version, other),
                expected,
                "isGreaterThan({version:?}, {other:?})"
            );
        }
    }

    // Ported: "isStable("$version") === $expected" — versioning/rez/index.spec.ts line 67
    #[test]
    fn is_stable_table() {
        let cases = [
            ("1", true),
            ("1.9", true),
            ("1.9.0", true),
            ("1.9.4", true),
            ("1.9.4-beta", false),
        ];
        for (version, expected) in cases {
            assert_eq!(is_stable(version), expected, "isStable({version:?})");
        }
    }

    // Ported: "isValid("$input") === $expected" — versioning/rez/index.spec.ts line 78
    #[test]
    fn is_valid_table() {
        let cases = [
            ("1.2.3..1.2.4", true),
            ("1.2..1.3", true),
            ("1.2..2", true),
            ("1..3", true),
            ("17.04.0", false),
            ("1.2.3", true),
            ("v1.2.3", true),
            ("1.2.3-foo", true),
            ("1.2.3foo", false),
            ("1.2.3+", true),
            ("1.2.3+<2", true),
            ("1.2.3..1.2.4", true),
            ("<=1.2.3", true),
            ("<=2.0.0,>1.0.0", true),
            ("==1.2.3", true),
        ];
        for (input, expected) in cases {
            assert_eq!(is_valid(input), expected, "isValid({input:?})");
        }
    }

    // Ported: "isVersion("$input") === $expected" — versioning/rez/index.spec.ts line 100
    #[test]
    fn is_version_table() {
        let cases = [("1.2.3", true)];
        for (input, expected) in cases {
            assert_eq!(is_version(input), expected, "isVersion({input:?})");
        }
    }

    // Ported: "isSingleVersion("$input") === $expected" — versioning/rez/index.spec.ts line 108
    #[test]
    fn is_single_version_table() {
        let cases = [
            ("1.2.3", true),
            ("1.2.3-alpha.1", true),
            ("==1.2.3", true),
            ("1.*", false),
        ];
        for (input, expected) in cases {
            assert_eq!(
                is_single_version(input),
                expected,
                "isSingleVersion({input:?})"
            );
        }
    }

    // Ported: "minSatisfyingVersion($versions, "$range") === $expected" — versioning/rez/index.spec.ts line 119
    #[test]
    fn min_satisfying_version_table() {
        let cases: &[(&[&str], &str, &str)] = &[
            (&["1.2.3", "1.2.4", "1.2.5"], "1.2.3..1.2.4", "1.2.3"),
            (&["0.4.0", "0.5.0", "4.2.0", "5.0.0"], "4", "4.2.0"),
            (&["0.4.0", "0.5.0", "4.2.0", "5.0.0"], "4..5", "4.2.0"),
            (&["0.4.0", "0.5.0", "4.2.0", "5.0.0"], "4..5.0", "4.2.0"),
            (&["0.4.0", "0.5.0", "4.2.0", "5.0.0"], "4.2..5.0", "4.2.0"),
            (&["0.4.0", "0.5.0", "4.2.0", "5.0.0"], "4.2.0..5.0", "4.2.0"),
            (
                &["0.4.0", "0.5.0", "4.2.0", "5.0.0"],
                "4.2.0..5.0.0",
                "4.2.0",
            ),
        ];
        for (versions, range, expected) in cases {
            assert_eq!(
                min_satisfying_version(versions, range),
                Some(*expected),
                "minSatisfyingVersion({versions:?}, {range:?})"
            );
        }
    }

    // Ported: "getSatisfyingVersion($versions, "$range") === $expected" — versioning/rez/index.spec.ts line 135
    #[test]
    fn get_satisfying_version_table() {
        let cases: &[(&[&str], &str, &str)] =
            &[(&["1.2.3", "1.2.4", "1.2.5"], "1.2.3..1.2.4", "1.2.3")];
        for (versions, range, expected) in cases {
            assert_eq!(
                get_satisfying_version(versions, range),
                Some(*expected),
                "getSatisfyingVersion({versions:?}, {range:?})"
            );
        }
    }

    // Ported: "isLessThanRange($version, "$range") === $expected" — versioning/rez/index.spec.ts line 145
    #[test]
    fn is_less_than_range_table() {
        let cases = [
            ("1.2.3", "1.2.3..1.2.4", false),
            ("1.2.3", "1.2.4..1.2.5", true),
            ("0.9.0", "1.0.0..2.0.0", true),
            ("1.9.0", "1.0.0..2.0.0", false),
        ];
        for (version, range, expected) in cases {
            assert_eq!(
                is_less_than_range(version, range),
                expected,
                "isLessThanRange({version:?}, {range:?})"
            );
        }
    }

    // Ported: "matches($version, "$range") === $expected" — versioning/rez/index.spec.ts line 158
    #[test]
    fn matches_range_table() {
        let cases = [
            ("1.2.3", "1.2.3..1.2.4", true),
            ("1.2.4", "1.2.2..1.2.3", false),
            ("4.2.0", "4.2.0..5.0.0", true),
            ("4.2", "4.2.0..5.0.0", true),
            ("4.2", "4.2..5", true),
            ("4.2.0", "4.2..5", true),
            ("4.2.0", "4.2..5.0", true),
            ("4.2.0", "4.2..5.0.0", true),
            ("4.2.0", "2.0..3.0", false),
            ("4.2.2", "4.2.0..4.2.4", true),
            ("1.4", "1.4", true),
        ];
        for (version, range, expected) in cases {
            assert_eq!(
                matches_range(version, range),
                expected,
                "matches({version:?}, {range:?})"
            );
        }
    }

    // Ported: "rez.sortVersions("$a", "$b") === semver.sortVersions("$a", "$b")" — versioning/rez/index.spec.ts line 178
    #[test]
    fn sort_versions_table() {
        let cases = [
            ("1.1.1", "1.2.3"),
            ("1.2.3", "1.3.4"),
            ("2.0.1", "1.2.3"),
            ("1.2.3", "0.9.5"),
        ];
        for (a, b) in cases {
            let rez_sorted = sort_versions(a, b);
            // Compare with direct semver sort
            let va = Version::parse(a).unwrap();
            let vb = Version::parse(b).unwrap();
            let semver_sorted = match va.cmp(&vb) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            };
            assert_eq!(rez_sorted, semver_sorted, "sortVersions({a:?}, {b:?})");
        }
    }

    // Ported: "getNewValue($currentValue, $rangeStrategy, $currentVersion, $newVersion, $expected) === $expected" — versioning/rez/index.spec.ts line 193
    #[test]
    fn get_new_value_table() {
        let cases: &[(&str, &str, &str, &str, Option<&str>)] = &[
            // exactVersion
            ("==1.2.3", "replace", "1.2.3", "1.2.4", Some("==1.2.4")),
            // simple version
            ("1.2.3", "auto", "1.2.3", "1.2.4", Some("1.2.4")),
            ("1.2.3", "bump", "1.2.3", "1.2.4", Some("1.2.4")),
            ("1.2.3", "replace", "1.2.3", "1.2.4", Some("1.2.4")),
            ("1.2.3", "widen", "1.2.3", "1.2.4", Some("1.2.4")),
            // inclusive bound replace
            ("7..8", "replace", "7.2.3", "8.2.5", Some("8..9")),
            ("7.2..8", "replace", "7.2.3", "8.2.5", Some("8.2..9")),
            ("7.2.3..8", "replace", "7.2.3", "8.2.5", Some("8.2.5..9")),
            ("7..8.0", "replace", "7.2.3", "8.2.5", Some("8..8.3")),
            ("7.2..8.0", "replace", "7.2.3", "8.2.5", Some("8.2..8.3")),
            (
                "7.2.3..8.0",
                "replace",
                "7.2.3",
                "8.2.5",
                Some("8.2.5..8.3"),
            ),
            ("7..8.0.0", "replace", "7.2.3", "8.2.5", Some("8..8.3")),
            ("7.2..8.0.0", "replace", "7.2.3", "8.2.5", Some("8.2..8.3")),
            (
                "7.2.3..8.0.0",
                "replace",
                "7.2.3",
                "8.2.5",
                Some("8.2.5..8.3.0"),
            ),
            // inclusive bound bump
            ("5..6", "bump", "5.2.3", "5.2.5", Some("5.2.5..6")),
            ("5.2..6", "bump", "5.2.3", "5.2.5", Some("5.2.5..6")),
            ("5.2.3..6", "bump", "5.2.3", "5.2.5", Some("5.2.5..6")),
            ("5..6.0", "bump", "5.2.3", "6.2.5", Some("6.2.5..6.3")),
            ("5.2..6.0", "bump", "5.2.3", "6.2.5", Some("6.2.5..6.3")),
            ("5.2.3..6.0", "bump", "5.2.3", "5.2.5", Some("5.2.5..6.0")),
            ("5..6.0.0", "bump", "5.2.3", "5.2.5", Some("5.2.5..6.0.0")),
            ("5.2..6.0.0", "bump", "5.2.3", "5.2.5", Some("5.2.5..6.0.0")),
            (
                "5.2.3..6.0.0",
                "bump",
                "5.2.3",
                "5.2.5",
                Some("5.2.5..6.0.0"),
            ),
            // inclusive bound widen
            ("1..2", "widen", "1.2.3", "2.2.5", Some("1..3")),
            ("1.2..2", "widen", "1.2.3", "2.2.5", Some("1.2..3")),
            ("1.2..2.0", "widen", "1.2.3", "2.2.5", Some("1.2..2.3")),
            ("1.2.3..2.0", "widen", "1.2.3", "2.2.5", Some("1.2.3..2.3")),
            (
                "1.2.3..2.0.0",
                "widen",
                "1.2.3",
                "2.2.5",
                Some("1.2.3..2.3.0"),
            ),
            // lower bound + (plus suffix) — replace/widen keep, bump changes
            ("7+", "replace", "7.2.3", "8.2.5", Some("7+")),
            ("7.2+", "replace", "7.2.3", "8.2.5", Some("7.2+")),
            ("7.2.3+", "replace", "7.2.3", "8.2.5", Some("7.2.3+")),
            ("5+", "bump", "5.2.3", "6.2.5", Some("6.2.5+")),
            ("5.2+", "bump", "5.2.3", "6.2.5", Some("6.2.5+")),
            ("5.2.3+", "bump", "5.2.3", "6.2.5", Some("6.2.5+")),
            ("1+", "widen", "1.2.3", "2.2.5", Some("1+")),
            ("1.2+", "widen", "1.2.3", "2.2.5", Some("1.2+")),
            ("1.2.3+", "widen", "1.2.3", "2.2.5", Some("1.2.3+")),
            // lower bound >= — replace/widen keep, bump changes
            (">=7", "replace", "7.2.3", "8.2.5", Some(">=7")),
            (">=7.2", "replace", "7.2.3", "8.2.5", Some(">=7.2")),
            (">=7.2.3", "replace", "7.2.3", "8.2.5", Some(">=7.2.3")),
            (">=5", "bump", "5.2.3", "6.2.5", Some(">=6.2.5")),
            (">=5.2", "bump", "5.2.3", "6.2.5", Some(">=6.2.5")),
            (">=5.2.3", "bump", "5.2.3", "6.2.5", Some(">=6.2.5")),
            (">=1", "widen", "1.2.3", "2.2.5", Some(">=1")),
            (">=1.2", "widen", "1.2.3", "2.2.5", Some(">=1.2")),
            (">=1.2.3", "widen", "1.2.3", "2.2.5", Some(">=1.2.3")),
            // lower bound > — replace/widen keep, bump keeps
            (">7", "replace", "7.2.3", "8.2.5", Some(">7")),
            (">7.2", "replace", "7.2.3", "8.2.5", Some(">7.2")),
            (">7.2.2", "replace", "7.2.3", "8.2.5", Some(">7.2.2")),
            (">5", "bump", "5.2.3", "6.2.5", Some(">5")),
            (">5.2", "bump", "5.2.3", "6.2.5", Some(">5.2")),
            (">5.2.3", "bump", "5.2.3", "6.2.5", Some(">5.2.3")),
            (">1", "widen", "1.2.3", "2.2.5", Some(">1")),
            (">1.2", "widen", "1.2.3", "2.2.5", Some(">1.2")),
            (">1.2.3", "widen", "1.2.3", "2.2.5", Some(">1.2.3")),
            // upper bound <=
            ("<=8", "replace", "7.2.3", "8.2.5", Some("<=8.2.5")),
            ("<=7.3", "replace", "7.2.3", "8.2.5", Some("<=8.2.5")),
            ("<=7.2.3", "replace", "7.2.3", "8.2.5", Some("<=8.2.5")),
            ("<=6", "bump", "5.2.3", "6.2.5", Some("<=6.2.5")),
            ("<=5.3", "bump", "5.2.3", "6.2.5", Some("<=6.2.5")),
            ("<=5.2.3", "bump", "5.2.3", "6.2.5", Some("<=6.2.5")),
            ("<=2", "widen", "1.2.3", "2.2.5", Some("<=2.2.5")),
            ("<=1.3", "widen", "1.2.3", "2.2.5", Some("<=2.2.5")),
            ("<=1.2.3", "widen", "1.2.3", "2.2.5", Some("<=2.2.5")),
            // upper bound <
            ("<8", "replace", "7.2.3", "8.2.5", Some("<9")),
            ("<7.3", "replace", "7.2.3", "8.2.5", Some("<8.3")),
            ("<7.2.4", "replace", "7.2.3", "8.2.5", Some("<8.2.6")),
            ("<6", "bump", "5.2.3", "6.2.5", Some("<7")),
            ("<5.3", "bump", "5.2.3", "6.2.5", Some("<6.3")),
            ("<5.2.4", "bump", "5.2.3", "6.2.5", Some("<6.2.6")),
            ("<2", "widen", "1.2.3", "2.2.5", Some("<3")),
            ("<1.3", "widen", "1.2.3", "2.2.5", Some("<2.3")),
            ("<1.2.4", "widen", "1.2.3", "2.2.5", Some("<2.2.6")),
            // ascending range, + lower
            ("7+<8", "replace", "7.2.3", "8.2.5", Some("8+<9")),
            ("7.2+<8", "replace", "7.2.3", "8.2.5", Some("8.2+<9")),
            ("7.2.3+<8", "replace", "7.2.3", "8.2.5", Some("8.2.5+<9")),
            ("7+<8.0", "replace", "7.2.3", "8.2.5", Some("8+<8.3")),
            ("7.2+<8.0", "replace", "7.2.3", "8.2.5", Some("8.2+<8.3")),
            (
                "7.2.3+<8.0",
                "replace",
                "7.2.3",
                "8.2.5",
                Some("8.2.5+<8.3"),
            ),
            ("7+<8.0.0", "replace", "7.2.3", "8.2.5", Some("8+<8.3")),
            ("7.2+<8.0.0", "replace", "7.2.3", "8.2.5", Some("8.2+<8.3")),
            (
                "7.2.3+<8.0.0",
                "replace",
                "7.2.3",
                "8.2.5",
                Some("8.2.5+<8.3.0"),
            ),
            ("5+<6", "bump", "5.2.3", "5.2.5", Some("5.2.5+<6")),
            ("5.2+<6", "bump", "5.2.3", "5.2.5", Some("5.2.5+<6")),
            ("5.2.3+<6", "bump", "5.2.3", "5.2.5", Some("5.2.5+<6")),
            ("5+<6.0", "bump", "5.2.3", "6.2.5", Some("6.2.5+<6.3")),
            ("5.2+<6.0", "bump", "5.2.3", "6.2.5", Some("6.2.5+<6.3")),
            ("5.2.3+<6.0", "bump", "5.2.3", "5.2.5", Some("5.2.5+<6.0")),
            ("5+<6.0.0", "bump", "5.2.3", "5.2.5", Some("5.2.5+<6.0.0")),
            ("5.2+<6.0.0", "bump", "5.2.3", "5.2.5", Some("5.2.5+<6.0.0")),
            (
                "5.2.3+<6.0.0",
                "bump",
                "5.2.3",
                "5.2.5",
                Some("5.2.5+<6.0.0"),
            ),
            ("1+<2", "widen", "1.2.3", "2.2.5", Some("1+<3")),
            ("1.2+<2", "widen", "1.2.3", "2.2.5", Some("1.2+<3")),
            ("1.2+<2.0", "widen", "1.2.3", "2.2.5", Some("1.2+<2.3")),
            ("1.2.3+<2.0", "widen", "1.2.3", "2.2.5", Some("1.2.3+<2.3")),
            (
                "1.2.3+<2.0.0",
                "widen",
                "1.2.3",
                "2.2.5",
                Some("1.2.3+<2.3.0"),
            ),
            // ascending range, >= lower, with comma
            (">=7,<8", "replace", "7.2.3", "8.2.5", Some(">=8,<9")),
            (">=7.2,<8", "replace", "7.2.3", "8.2.5", Some(">=8.2,<9")),
            (
                ">=7.2.3,<8",
                "replace",
                "7.2.3",
                "8.2.5",
                Some(">=8.2.5,<9"),
            ),
            (">=7,<8.0", "replace", "7.2.3", "8.2.5", Some(">=8,<8.3")),
            (
                ">=7.2,<8.0",
                "replace",
                "7.2.3",
                "8.2.5",
                Some(">=8.2,<8.3"),
            ),
            (
                ">=7.2.3,<8.0",
                "replace",
                "7.2.3",
                "8.2.5",
                Some(">=8.2.5,<8.3"),
            ),
            (">=7,<8.0.0", "replace", "7.2.3", "8.2.5", Some(">=8,<8.3")),
            (
                ">=7.2,<8.0.0",
                "replace",
                "7.2.3",
                "8.2.5",
                Some(">=8.2,<8.3"),
            ),
            (
                ">=7.2.3,<8.0.0",
                "replace",
                "7.2.3",
                "8.2.5",
                Some(">=8.2.5,<8.3.0"),
            ),
            (">=5,<6", "bump", "5.2.3", "5.2.5", Some(">=5.2.5,<6")),
            (">=5.2,<6", "bump", "5.2.3", "5.2.5", Some(">=5.2.5,<6")),
            (">=5.2.3,<6", "bump", "5.2.3", "5.2.5", Some(">=5.2.5,<6")),
            (">=5,<6.0", "bump", "5.2.3", "6.2.5", Some(">=6.2.5,<6.3")),
            (">=5.2,<6.0", "bump", "5.2.3", "6.2.5", Some(">=6.2.5,<6.3")),
            (
                ">=5.2.3,<6.0",
                "bump",
                "5.2.3",
                "5.2.5",
                Some(">=5.2.5,<6.0"),
            ),
            (
                ">=5,<6.0.0",
                "bump",
                "5.2.3",
                "5.2.5",
                Some(">=5.2.5,<6.0.0"),
            ),
            (
                ">=5.2,<6.0.0",
                "bump",
                "5.2.3",
                "5.2.5",
                Some(">=5.2.5,<6.0.0"),
            ),
            (
                ">=5.2.3,<6.0.0",
                "bump",
                "5.2.3",
                "5.2.5",
                Some(">=5.2.5,<6.0.0"),
            ),
            (">=1,<2", "widen", "1.2.3", "2.2.5", Some(">=1,<3")),
            (">=1.2,<2", "widen", "1.2.3", "2.2.5", Some(">=1.2,<3")),
            (">=1.2,<2.0", "widen", "1.2.3", "2.2.5", Some(">=1.2,<2.3")),
            (
                ">=1.2.3,<2.0",
                "widen",
                "1.2.3",
                "2.2.5",
                Some(">=1.2.3,<2.3"),
            ),
            (
                ">=1.2.3,<2.0.0",
                "widen",
                "1.2.3",
                "2.2.5",
                Some(">=1.2.3,<2.3.0"),
            ),
            // ascending range, >= lower, without comma
            (">=7<8", "replace", "7.2.3", "8.2.5", Some(">=8<9")),
            (">=7.2<8", "replace", "7.2.3", "8.2.5", Some(">=8.2<9")),
            (">=7.2.3<8", "replace", "7.2.3", "8.2.5", Some(">=8.2.5<9")),
            (">=7<8.0", "replace", "7.2.3", "8.2.5", Some(">=8<8.3")),
            (">=7.2<8.0", "replace", "7.2.3", "8.2.5", Some(">=8.2<8.3")),
            (
                ">=7.2.3<8.0",
                "replace",
                "7.2.3",
                "8.2.5",
                Some(">=8.2.5<8.3"),
            ),
            (">=7<8.0.0", "replace", "7.2.3", "8.2.5", Some(">=8<8.3")),
            (
                ">=7.2<8.0.0",
                "replace",
                "7.2.3",
                "8.2.5",
                Some(">=8.2<8.3"),
            ),
            (
                ">=7.2.3<8.0.0",
                "replace",
                "7.2.3",
                "8.2.5",
                Some(">=8.2.5<8.3.0"),
            ),
            (">=5<6", "bump", "5.2.3", "5.2.5", Some(">=5.2.5<6")),
            (">=5.2<6", "bump", "5.2.3", "5.2.5", Some(">=5.2.5<6")),
            (">=5.2.3<6", "bump", "5.2.3", "5.2.5", Some(">=5.2.5<6")),
            (">=5<6.0", "bump", "5.2.3", "6.2.5", Some(">=6.2.5<6.3")),
            (">=5.2<6.0", "bump", "5.2.3", "6.2.5", Some(">=6.2.5<6.3")),
            (">=5.2.3<6.0", "bump", "5.2.3", "5.2.5", Some(">=5.2.5<6.0")),
            (">=5<6.0.0", "bump", "5.2.3", "5.2.5", Some(">=5.2.5<6.0.0")),
            (
                ">=5.2<6.0.0",
                "bump",
                "5.2.3",
                "5.2.5",
                Some(">=5.2.5<6.0.0"),
            ),
            (
                ">=5.2.3<6.0.0",
                "bump",
                "5.2.3",
                "5.2.5",
                Some(">=5.2.5<6.0.0"),
            ),
            (">=1<2", "widen", "1.2.3", "2.2.5", Some(">=1<3")),
            (">=1.2<2", "widen", "1.2.3", "2.2.5", Some(">=1.2<3")),
            (">=1.2<2.0", "widen", "1.2.3", "2.2.5", Some(">=1.2<2.3")),
            (
                ">=1.2.3<2.0",
                "widen",
                "1.2.3",
                "2.2.5",
                Some(">=1.2.3<2.3"),
            ),
            (
                ">=1.2.3<2.0.0",
                "widen",
                "1.2.3",
                "2.2.5",
                Some(">=1.2.3<2.3.0"),
            ),
            // ascending range, > lower, with comma
            (">6,<8", "replace", "7.2.3", "8.2.5", Some(">8,<9")),
            (">7.1,<8", "replace", "7.2.3", "8.2.5", Some(">8.2,<9")),
            (">7.2.0,<8", "replace", "7.2.3", "8.2.5", Some(">8.2,<9")),
            (">6,<8.0", "replace", "7.2.3", "8.2.5", Some(">8,<8.3")),
            (">7.1,<8.0", "replace", "7.2.3", "8.2.5", Some(">8.2,<8.3")),
            (
                ">7.2.0,<8.0",
                "replace",
                "7.2.3",
                "8.2.5",
                Some(">8.2,<8.3"),
            ),
            (">6,<8.0.0", "replace", "7.2.3", "8.2.5", Some(">8,<8.3")),
            (
                ">7.1,<8.0.0",
                "replace",
                "7.2.3",
                "8.2.5",
                Some(">8.2,<8.3"),
            ),
            (
                ">7.2.0,<8.0.0",
                "replace",
                "7.2.3",
                "8.2.5",
                Some(">8.2,<8.3.0"),
            ),
            (">4,<6", "bump", "5.2.3", "5.2.5", Some(">4,<6")),
            (">5.1,<6", "bump", "5.2.3", "5.2.5", Some(">5.1,<6")),
            (">5.2.0,<6", "bump", "5.2.3", "5.2.5", Some(">5.2.0,<6")),
            (">5,<6.0", "bump", "5.2.3", "6.2.5", Some(">5,<6.3")),
            (">5.1,<6.0", "bump", "5.2.3", "6.2.5", Some(">5.1,<6.3")),
            (">5.2.0,<6.0", "bump", "5.2.3", "5.2.5", Some(">5.2.0,<6.0")),
            (">5,<6.0.0", "bump", "5.2.3", "5.2.5", Some(">5,<6.0.0")),
            (">5.1,<6.0.0", "bump", "5.2.3", "5.2.5", Some(">5.1,<6.0.0")),
            (
                ">5.2.0,<6.0.0",
                "bump",
                "5.2.3",
                "5.2.5",
                Some(">5.2.0,<6.0.0"),
            ),
            (">1,<2", "widen", "1.2.3", "2.2.5", Some(">1,<3")),
            (">1.1,<2", "widen", "1.2.3", "2.2.5", Some(">1.1,<3")),
            (">1.1,<2.0", "widen", "1.2.3", "2.2.5", Some(">1.1,<2.3")),
            (
                ">1.2.0,<2.0",
                "widen",
                "1.2.3",
                "2.2.5",
                Some(">1.2.0,<2.3"),
            ),
            (
                ">1.2.0,<2.0.0",
                "widen",
                "1.2.3",
                "2.2.5",
                Some(">1.2.0,<2.3.0"),
            ),
            // ascending range, > lower, without comma
            (">6<8", "replace", "7.2.3", "8.2.5", Some(">8<9")),
            (">7.1<8", "replace", "7.2.3", "8.2.5", Some(">8.2<9")),
            (">7.2.0<8", "replace", "7.2.3", "8.2.5", Some(">8.2<9")),
            (">6<8.0", "replace", "7.2.3", "8.2.5", Some(">8<8.3")),
            (">7.1<8.0", "replace", "7.2.3", "8.2.5", Some(">8.2<8.3")),
            (">7.2.0<8.0", "replace", "7.2.3", "8.2.5", Some(">8.2<8.3")),
            (">6<8.0.0", "replace", "7.2.3", "8.2.5", Some(">8<8.3")),
            (">7.1<8.0.0", "replace", "7.2.3", "8.2.5", Some(">8.2<8.3")),
            (
                ">7.2.0<8.0.0",
                "replace",
                "7.2.3",
                "8.2.5",
                Some(">8.2<8.3.0"),
            ),
            (">4<6", "bump", "5.2.3", "5.2.5", Some(">4<6")),
            (">5.1<6", "bump", "5.2.3", "5.2.5", Some(">5.1<6")),
            (">5.2.0<6", "bump", "5.2.3", "5.2.5", Some(">5.2.0<6")),
            (">5<6.0", "bump", "5.2.3", "6.2.5", Some(">5<6.3")),
            (">5.1<6.0", "bump", "5.2.3", "6.2.5", Some(">5.1<6.3")),
            (">5.2.0<6.0", "bump", "5.2.3", "5.2.5", Some(">5.2.0<6.0")),
            (">4<6.0.0", "bump", "5.2.3", "5.2.5", Some(">4<6.0.0")),
            (">5.1<6.0.0", "bump", "5.2.3", "5.2.5", Some(">5.1<6.0.0")),
            (
                ">5.2.0<6.0.0",
                "bump",
                "5.2.3",
                "5.2.5",
                Some(">5.2.0<6.0.0"),
            ),
            (">1<2", "widen", "1.2.3", "2.2.5", Some(">1<3")),
            (">1.1<2", "widen", "1.2.3", "2.2.5", Some(">1.1<3")),
            (">1.1<2.0", "widen", "1.2.3", "2.2.5", Some(">1.1<2.3")),
            (">1.2.0<2.0", "widen", "1.2.3", "2.2.5", Some(">1.2.0<2.3")),
            (
                ">1.2.0<2.0.0",
                "widen",
                "1.2.3",
                "2.2.5",
                Some(">1.2.0<2.3.0"),
            ),
            // descending range (upper,lower), with comma
            ("<8,>=7", "replace", "7.2.3", "8.2.5", Some("<9,>=8")),
            ("<8,>=7.2", "replace", "7.2.3", "8.2.5", Some("<9,>=8.2")),
            (
                "<8,>=7.2.3",
                "replace",
                "7.2.3",
                "8.2.5",
                Some("<9,>=8.2.5"),
            ),
            ("<8.0,>=7", "replace", "7.2.3", "8.2.5", Some("<8.3,>=8")),
            (
                "<8.0,>=7.2",
                "replace",
                "7.2.3",
                "8.2.5",
                Some("<8.3,>=8.2"),
            ),
            (
                "<8.0,>=7.2.3",
                "replace",
                "7.2.3",
                "8.2.5",
                Some("<8.3,>=8.2.5"),
            ),
            ("<8.0.0,>=7", "replace", "7.2.3", "8.2.5", Some("<8.3,>=8")),
            (
                "<8.0.0,>=7.2",
                "replace",
                "7.2.3",
                "8.2.5",
                Some("<8.3,>=8.2"),
            ),
            (
                "<8.0.0,>=7.2.3",
                "replace",
                "7.2.3",
                "8.2.5",
                Some("<8.3.0,>=8.2.5"),
            ),
            ("<6,>=5", "bump", "5.2.3", "5.2.5", Some("<6,>=5.2.5")),
            ("<6,>=5.2", "bump", "5.2.3", "5.2.5", Some("<6,>=5.2.5")),
            ("<6,>=5.2.3", "bump", "5.2.3", "5.2.5", Some("<6,>=5.2.5")),
            ("<6.0,>=5", "bump", "5.2.3", "6.2.5", Some("<6.3,>=6.2.5")),
            ("<6.0,>=5.2", "bump", "5.2.3", "6.2.5", Some("<6.3,>=6.2.5")),
            (
                "<6.0,>=5.2.3",
                "bump",
                "5.2.3",
                "5.2.5",
                Some("<6.0,>=5.2.5"),
            ),
            (
                "<6.0.0,>=5",
                "bump",
                "5.2.3",
                "5.2.5",
                Some("<6.0.0,>=5.2.5"),
            ),
            (
                "<6.0.0,>=5.2",
                "bump",
                "5.2.3",
                "5.2.5",
                Some("<6.0.0,>=5.2.5"),
            ),
            (
                "<6.0.0,>=5.2.3",
                "bump",
                "5.2.3",
                "5.2.5",
                Some("<6.0.0,>=5.2.5"),
            ),
            ("<2,>=1", "widen", "1.2.3", "2.2.5", Some("<3,>=1")),
            ("<2,>=1.2", "widen", "1.2.3", "2.2.5", Some("<3,>=1.2")),
            ("<2.0,>=1.2", "widen", "1.2.3", "2.2.5", Some("<2.3,>=1.2")),
            (
                "<2.0,>=1.2.3",
                "widen",
                "1.2.3",
                "2.2.5",
                Some("<2.3,>=1.2.3"),
            ),
            (
                "<2.0.0,>=1.2.3",
                "widen",
                "1.2.3",
                "2.2.5",
                Some("<2.3.0,>=1.2.3"),
            ),
            ("<8,>6", "replace", "7.2.3", "8.2.5", Some("<9,>8")),
            ("<8,>7.1", "replace", "7.2.3", "8.2.5", Some("<9,>8.2")),
            ("<8,>7.2.0", "replace", "7.2.3", "8.2.5", Some("<9,>8.2")),
            ("<8.0,>6", "replace", "7.2.3", "8.2.5", Some("<8.3,>8")),
            ("<8.0,>7.1", "replace", "7.2.3", "8.2.5", Some("<8.3,>8.2")),
            (
                "<8.0,>7.2.0",
                "replace",
                "7.2.3",
                "8.2.5",
                Some("<8.3,>8.2"),
            ),
            ("<8.0.0,>6", "replace", "7.2.3", "8.2.5", Some("<8.3,>8")),
            (
                "<8.0.0,>7.1",
                "replace",
                "7.2.3",
                "8.2.5",
                Some("<8.3,>8.2"),
            ),
            (
                "<8.0.0,>7.2.0",
                "replace",
                "7.2.3",
                "8.2.5",
                Some("<8.3.0,>8.2"),
            ),
            ("<6,>4", "bump", "5.2.3", "5.2.5", Some("<6,>4")),
            ("<6,>5.1", "bump", "5.2.3", "5.2.5", Some("<6,>5.1")),
            ("<6,>5.2.0", "bump", "5.2.3", "5.2.5", Some("<6,>5.2.0")),
            ("<6.0,>5", "bump", "5.2.3", "6.2.5", Some("<6.3,>5")),
            ("<6.0,>5.1", "bump", "5.2.3", "6.2.5", Some("<6.3,>5.1")),
            ("<6.0,>5.2.0", "bump", "5.2.3", "5.2.5", Some("<6.0,>5.2.0")),
            ("<6.0.0,>5", "bump", "5.2.3", "5.2.5", Some("<6.0.0,>5")),
            ("<6.0.0,>5.1", "bump", "5.2.3", "5.2.5", Some("<6.0.0,>5.1")),
            (
                "<6.0.0,>5.2.0",
                "bump",
                "5.2.3",
                "5.2.5",
                Some("<6.0.0,>5.2.0"),
            ),
            ("<2,>1", "widen", "1.2.3", "2.2.5", Some("<3,>1")),
            ("<2,>1.1", "widen", "1.2.3", "2.2.5", Some("<3,>1.1")),
            ("<2.0,>1.1", "widen", "1.2.3", "2.2.5", Some("<2.3,>1.1")),
            (
                "<2.0,>1.2.0",
                "widen",
                "1.2.3",
                "2.2.5",
                Some("<2.3,>1.2.0"),
            ),
            (
                "<2.0.0,>1.2.0",
                "widen",
                "1.2.3",
                "2.2.5",
                Some("<2.3.0,>1.2.0"),
            ),
            // null case (range with space after comma doesn't match any rez pattern)
            ("<=1.2.5, >1.2.0", "widen", "1.2.3", "1.2.4", None),
        ];
        for (cv, strategy, current_ver, new_ver, expected) in cases {
            assert_eq!(
                get_new_value(cv, strategy, current_ver, new_ver).as_deref(),
                *expected,
                "getNewValue({cv:?}, {strategy:?}, {current_ver:?}, {new_ver:?})"
            );
        }
    }

    // Ported: "isCompatible("$version") === $expected" — versioning/rez/index.spec.ts line 443
    #[test]
    fn is_compatible_table() {
        let cases = [("1.2.0", true)];
        for (version, expected) in cases {
            assert_eq!(
                is_compatible(version),
                expected,
                "isCompatible({version:?})"
            );
        }
    }
}
