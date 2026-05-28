//! Ruby gem versioning.
//!
//! Mirrors `lib/modules/versioning/ruby/index.ts`.
//! Supports `~>` pessimistic version constraint, dot-separated prereleases,
//! and partial versions (`1`, `1.2`).

use std::cmp::Ordering;
use std::sync::LazyLock;
use regex::Regex;

static VERSION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^v?(\d+)(?:\.(\d+))?(?:\.(\d+))?(?:[.-](.+))?$").unwrap()
});

/// A parsed Ruby gem version.
#[derive(Debug, Clone)]
struct RubyVersion {
    segs: Vec<u64>, // numeric segments (up to 3)
    pre: Option<String>,
}

impl RubyVersion {
    fn parse(input: &str) -> Option<Self> {
        let s = input.trim().trim_start_matches('v');
        if s.is_empty() { return None; }

        let caps = VERSION_RE.captures(s)?;
        let seg1: u64 = caps.get(1)?.as_str().parse().ok()?;
        let mut segs = vec![seg1];
        if let Some(m) = caps.get(2) {
            segs.push(m.as_str().parse().ok()?);
            if let Some(m) = caps.get(3) {
                segs.push(m.as_str().parse().ok()?);
            }
        }
        let pre = caps.get(4).map(|m| m.as_str().to_string());
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
    fn eq(&self, other: &Self) -> bool { self.cmp(other) == Ordering::Equal }
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
    let a_toks: Vec<&str> = a.split(|c| c == '.' || c == '-').collect();
    let b_toks: Vec<&str> = b.split(|c| c == '.' || c == '-').collect();
    let len = a_toks.len().max(b_toks.len());
    for i in 0..len {
        let at = a_toks.get(i).copied().unwrap_or("");
        let bt = b_toks.get(i).copied().unwrap_or("");
        if at == bt { continue; }
        let ord = cmp_prerelease_tokens(at, bt);
        if ord != Ordering::Equal { return ord; }
    }
    Ordering::Equal
}

impl PartialOrd for RubyVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl Ord for RubyVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        // Numeric segments (missing → 0)
        let n = self.max_segs(other);
        for i in 0..n {
            let ord = self.seg(i).cmp(&other.seg(i));
            if ord != Ordering::Equal { return ord; }
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
    if input.is_empty() { return false; }
    RubyVersion::parse(input).is_some()
}

pub fn get_major(input: &str) -> Option<i32> {
    RubyVersion::parse(input).map(|v| v.seg(0) as i32)
}

pub fn get_minor(input: &str) -> Option<i32> {
    RubyVersion::parse(input)?.segs.get(1).copied().map(|v| v as i32)
}

pub fn get_patch(input: &str) -> Option<i32> {
    RubyVersion::parse(input)?.segs.get(2).copied().map(|v| v as i32)
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
    if s.is_empty() { return None; }
    let caps = RE.captures(s)?;
    let op = caps.name("op").map(|m| m.as_str()).unwrap_or("=").to_string();
    let ver_str = caps.name("ver")?.as_str();
    let ver = RubyVersion::parse(ver_str)?;
    let ver_segs = ver.segs.len();
    Some(Constraint { op, ver, ver_segs })
}

fn parse_constraints(range: &str) -> Vec<Constraint> {
    range.split(',').filter_map(|s| parse_constraint(s.trim())).collect()
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
            // Pessimistic:
            // ~> X → >= X, < X+1
            // ~> X.Y → >= X.Y, < X+1
            // ~> X.Y.Z → >= X.Y.Z, < X.Y+1
            if *v < c.ver { return false; }
            if c.ver_segs == 1 {
                v.seg(0) < c.ver.seg(0) + 1
            } else if c.ver_segs == 2 {
                // ~> X.Y → < X+1
                v.seg(0) < c.ver.seg(0) + 1
            } else {
                // ~> X.Y.Z → < X.Y+1
                v.seg(0) == c.ver.seg(0) && v.seg(1) < c.ver.seg(1) + 1
            }
        }
        _ => false,
    }
}

pub fn matches(version: &str, range: &str) -> bool {
    let v = match RubyVersion::parse(version) {
        Some(v) => v,
        None => return false,
    };
    let cs = parse_constraints(range);
    if cs.is_empty() { return false; }
    cs.iter().all(|c| satisfies_constraint(&v, c))
}

pub fn is_less_than_range(version: &str, range: &str) -> Option<bool> {
    let v = RubyVersion::parse(version)?;
    let cs = parse_constraints(range);
    if cs.is_empty() { return Some(false); }
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
                if v >= c.ver { return false; }
                // Also check that version <= bump (upper bound)
                let bump = if c.ver_segs <= 1 {
                    RubyVersion { segs: vec![c.ver.seg(0) + 1], pre: None }
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
    let mut matches: Vec<(&'a str, RubyVersion)> = versions.iter()
        .filter_map(|&v| {
            let rv = RubyVersion::parse(v)?;
            if cs.iter().all(|c| satisfies_constraint(&rv, c)) { Some((v, rv)) } else { None }
        })
        .collect();
    matches.sort_by(|a, b| a.1.cmp(&b.1));
    matches.into_iter().next().map(|(v, _)| v)
}

pub fn get_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    let cs = parse_constraints(range);
    let mut ms: Vec<(&'a str, RubyVersion)> = versions.iter()
        .filter_map(|&v| {
            let rv = RubyVersion::parse(v)?;
            if cs.iter().all(|c| satisfies_constraint(&rv, c)) { Some((v, rv)) } else { None }
        })
        .collect();
    ms.sort_by(|a, b| a.1.cmp(&b.1));
    ms.into_iter().last().map(|(v, _)| v)
}

pub fn is_valid(input: &str) -> bool {
    let s = input.trim();
    if s.is_empty() { return false; }
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
    if s.is_empty() { return false; }
    RE.is_match(s)
}

pub fn get_pinned_value(input: &str) -> String {
    input.trim_start_matches('v').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "equals(\"$a\", \"$b\") === $expected" — modules/versioning/ruby/index.spec.ts line 4
    #[test]
    fn ruby_equals_cases() {
        let trues = [("1.0.0", "1"), ("1.2.0", "1.2"), ("1.2.0", "1.2.0"), ("1.0.0.rc1", "1.0.0.rc1")];
        for (a, b) in trues { assert!(equals(a, b), "equals({a:?}, {b:?})"); }
        let falses = [("1.2.0", "2"), ("1.2.0", "1.1"), ("1.2.0", "1.2.1"), ("1.0.0.rc1", "1.0.0.rc2")];
        for (a, b) in falses { assert!(!equals(a, b), "!equals({a:?}, {b:?})"); }
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
        for v in ["0","v0","v1","v1.2","v1.2.3","1","1.1","1.1.2","1.1.2.3","1.1.2-4","1.1.2.pre.4","v1.1.2.pre.4"] {
            assert!(is_version(v), "is_version({v:?})");
        }
        for v in ["","v","tottally-not-a-version"] {
            assert!(!is_version(v), "!is_version({v:?})");
        }
    }

    // Ported: "isGreaterThan(\"$a\", \"$b\") === $expected" — ruby/index.spec.ts line 62
    #[test]
    fn ruby_is_greater_than_cases() {
        for (a,b) in [("2","1"),("2.2","2.1"),("2.2.1","2.2.0"),
                      ("3.0.0.rc2","3.0.0.rc1"),("3.0.0-rc.2","3.0.0-rc.1"),
                      ("3.0.0.rc1","3.0.0.beta"),("3.0.0-rc.1","3.0.0-beta"),
                      ("3.0.0.beta","3.0.0.alpha"),("3.0.0-beta","3.0.0-alpha"),
                      ("5.0.1.rc1","5.0.1.beta1"),("5.0.1-rc.1","5.0.1-beta.1")] {
            assert!(is_greater_than(a, b), "{a:?} > {b:?}");
        }
        for (a,b) in [("1","2"),("2.1","2.2"),("2.2.0","2.2.1"),
                      ("3.0.0.rc1","3.0.0.rc2"),("3.0.0-rc.1","3.0.0-rc.2"),
                      ("3.0.0.beta","3.0.0.rc1"),("3.0.0-beta","3.0.0-rc.1"),
                      ("3.0.0.alpha","3.0.0.beta"),("3.0.0-alpha","3.0.0-beta"),
                      ("5.0.1.beta1","5.0.1.rc1"),("5.0.1-beta.1","5.0.1-rc.1"),
                      ("1","1"),("2.1","2.1"),("2.2.0","2.2.0")] {
            assert!(!is_greater_than(a, b), "!({a:?} > {b:?})");
        }
    }

    // Ported: "isStable(\"$version\") === $expected" — ruby/index.spec.ts line 106
    #[test]
    fn ruby_is_stable() {
        for v in ["1","1.2","1.2.3"] { assert!(is_stable(v), "is_stable({v:?})"); }
        for v in ["1.2.0.alpha","1.2.0.alpha1","1.2.0-alpha.1"] { assert!(!is_stable(v), "!is_stable({v:?})"); }
    }

    // Ported: "$versions -> sortVersions -> $expected" — ruby/index.spec.ts line 122
    #[test]
    fn ruby_sort_versions() {
        let mut v = vec!["1.2.3-beta","2.0.1","1.3.4","1.2.3"];
        v.sort_by(|a,b| sort_versions(a,b));
        assert_eq!(v, vec!["1.2.3-beta","1.2.3","1.3.4","2.0.1"]);
    }

    // Ported: "minSatisfyingVersion($versions, \"$range\") === \"$expected\"" — ruby/index.spec.ts line 129
    #[test]
    fn ruby_min_satisfying_version() {
        let cases: &[(&[&str], &str, Option<&str>)] = &[
            (&["2.1.5","2.1.6"], "~> 2.1", Some("2.1.5")),
            (&["2.1.6","2.1.5"], "~> 2.1.6", Some("2.1.6")),
            (&["4.7.3","4.7.4","4.7.5","4.7.9"], "~> 4.7, >= 4.7.4", Some("4.7.4")),
            (&["2.5.3","2.5.4","2.5.5","2.5.6"], "~>2.5.3", Some("2.5.3")),
            (&["2.1.0","3.0.0.beta","2.3","3.0.0-rc.1","3.0.0","3.1.1"], "~> 3.0", Some("3.0.0")),
            (&["1.2.3","1.2.4"], ">= 3.5.0", None),
        ];
        for (vs, r, exp) in cases {
            assert_eq!(min_satisfying_version(vs, r), *exp, "min({vs:?}, {r:?})");
        }
    }

    // Ported: "getSatisfyingVersion($versions, \"$range\") === \"$expected\"" — ruby/index.spec.ts line 147
    #[test]
    fn ruby_get_satisfying_version() {
        let cases: &[(&[&str], &str, Option<&str>)] = &[
            (&["2.1.5","2.1.6"], "~> 2.1", Some("2.1.6")),
            (&["2.1.6","2.1.5"], "~> 2.1.6", Some("2.1.6")),
            (&["4.7.3","4.7.4","4.7.5","4.7.9"], "~> 4.7, >= 4.7.4", Some("4.7.9")),
            (&["2.5.3","2.5.4","2.5.5","2.5.6"], "~>2.5.3", Some("2.5.6")),
            (&["2.1.0","3.0.0.beta","2.3","3.0.0-rc.1","3.0.0","3.1.1"], "~> 3.0", Some("3.1.1")),
            (&["1.2.3","1.2.4"], ">= 3.5.0", None),
        ];
        for (vs, r, exp) in cases {
            assert_eq!(get_satisfying_version(vs, r), *exp, "max({vs:?}, {r:?})");
        }
    }

    // Ported: "matches(\"$version\", \"$range\") === \"$expected\"" — ruby/index.spec.ts line 165
    #[test]
    fn ruby_matches() {
        for (v,r) in [("1.2",">= 1.2"),("1.2.3","~> 1.2.1"),("1.2.7","1.2.7"),("1.1.6",">= 1.1.5, < 2.0")] {
            assert!(matches(v, r), "matches({v:?}, {r:?})");
        }
        for (v,r) in [("1.2",">= 1.3"),("1.3.8","~> 1.2.1"),("1.3.9","1.3.8"),("2.0.0",">= 1.1.5, < 2.0")] {
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
        for v in ["1","1.2","1.2.3"] { assert!(is_valid(v), "is_valid({v:?})"); }
        for v in ["^1.2.3","~1.2.3","1.2.*","< 3.0, >= 1.0.0 <= 2.0.0"] { assert!(!is_valid(v), "!is_valid({v:?})"); }
    }

    // Ported: "isValid(\"$version\") === $expected" (range form) — ruby/index.spec.ts line 224
    #[test]
    fn ruby_is_valid_range_form() {
        for v in ["= 1","!= 1.1","> 1.1.2","< 1.0.0-beta",">= 1.0.0.beta","<= 1.2.0.alpha1","~> 1.2.0-alpha.1",">= 3.0.5, < 3.2"] {
            assert!(is_valid(v), "is_valid({v:?})");
        }
        for v in ["+ 1","- 1.1","=== 1.1.2","! 1.0.0-beta","& 1.0.0.beta"] {
            assert!(!is_valid(v), "!is_valid({v:?})");
        }
    }

    // Ported: "isSingleVersion(\"$version\") === $expected" — ruby/index.spec.ts line 247
    #[test]
    fn ruby_is_single_version() {
        for v in ["1","1.2","1.2.1","=1","=1.2","=1.2.1","= 1","= 1.2","= 1.2.1","1.2.1.rc1","1.2.1-rc.1","= 1.2.0.alpha","= 1.2.0-alpha"] {
            assert!(is_single_version(v), "is_single({v:?})");
        }
        for v in ["!= 1","> 1.2","< 1.2.1",">= 1","<= 1.2","~> 1.2.1","","tottally-not-a-version"] {
            assert!(!is_single_version(v), "!is_single({v:?})");
        }
    }

    // Ported: "returns a pinned value" — ruby/index.spec.ts line 276
    #[test]
    fn ruby_get_pinned_value() {
        assert_eq!(get_pinned_value("1.2.3"), "1.2.3");
        assert_eq!(get_pinned_value("v1.2.3"), "1.2.3");
    }
}
