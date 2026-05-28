//! Ivy versioning (Apache Ivy / Gradle-compatible dynamic revision syntax).
//!
//! Ports `lib/modules/versioning/ivy/parse.ts` and `index.ts`.
//!
//! Plain version comparison and range semantics are delegated to the Maven
//! versioning module. Ivy extends Maven with three additional revision types:
//!
//! - **Latest**: `latest`, `latest.release`, `latest.milestone`, `latest.integration`
//! - **Subrevision**: `1.0.+` — matches any version whose first N tokens equal `1.0`
//! - **Range**: `[1.0,2.0]`, `(0,1)`, `]0,1[` etc. — single Maven-style intervals

use std::cmp::Ordering;

use crate::versioning::maven;

// ── Revision types ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RevType {
    Latest,
    Subrevision,
    Range,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Revision {
    pub rev_type: RevType,
    pub value: String,
}

// ── parseDynamicRevision ──────────────────────────────────────────────────────

fn is_latest_pattern(s: &str) -> bool {
    let lower = s.to_lowercase();
    lower == "latest" || lower.starts_with("latest.")
}

/// Parses an Ivy dynamic revision string.
///
/// Returns `None` for: empty strings, invalid sub-revision prefixes, and
/// multi-interval range strings (`[0,1),(1,2]`).
pub fn parse_dynamic_revision(s: &str) -> Option<Revision> {
    if s.is_empty() {
        return None;
    }

    // latest / latest.X
    let lower = s.to_lowercase();
    if lower == "latest" || lower.starts_with("latest.") {
        let value_part = lower.strip_prefix("latest.").unwrap_or("").to_owned();
        // "integration" is equivalent to "" (unqualified latest)
        let value = if value_part == "integration" {
            String::new()
        } else {
            value_part
        };
        return Some(Revision {
            rev_type: RevType::Latest,
            value,
        });
    }

    // Subrevision: ends with ".+"
    if let Some(prefix) = s.strip_suffix(".+") {
        if maven::is_version(prefix) {
            return Some(Revision {
                rev_type: RevType::Subrevision,
                value: prefix.to_owned(),
            });
        }
        return None;
    }

    // Range (single interval only)
    let ranges = maven::parse_range(s)?;
    if ranges.len() == 1 {
        let range_str = maven::range_to_str(Some(&ranges))?;
        return Some(Revision {
            rev_type: RevType::Range,
            value: range_str,
        });
    }

    None
}

// ── isValid / isVersion ───────────────────────────────────────────────────────

/// Returns `true` for valid plain versions, dynamic revisions, and `latest.*`.
pub fn is_valid(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    // latest.* shortcut (also covered by parseDynamicRevision)
    if is_latest_pattern(s) {
        return true;
    }
    is_version(s) || parse_dynamic_revision(s).is_some()
}

/// Returns `true` only for plain Maven-compatible version strings.
///
/// Excludes: `latest.*`, subrevision patterns (`+`), and range strings.
pub fn is_version(s: &str) -> bool {
    if s.is_empty() || is_latest_pattern(s) || s.contains('+') {
        return false;
    }
    maven::is_version(s)
}

// ── matches ───────────────────────────────────────────────────────────────────

/// Returns `true` when `version` satisfies `range`.
pub fn matches_range(version: &str, range: &str) -> bool {
    if version.is_empty() || range.is_empty() {
        return false;
    }
    let Some(dynamic) = parse_dynamic_revision(range) else {
        return maven::compare(version, range) == Ordering::Equal;
    };

    match dynamic.rev_type {
        RevType::Latest => {
            let qualifier = &dynamic.value;
            if qualifier.is_empty() {
                // bare "latest" or "latest.integration" → any version matches
                return true;
            }
            // The version must end with a qualifier token equal to `qualifier`
            match maven::last_qualifier(version) {
                Some(q) => q == *qualifier,
                None => false,
            }
        }
        RevType::Subrevision => {
            // version must have the subrev prefix as its leading tokens
            maven::is_subversion(&dynamic.value, version)
        }
        RevType::Range => maven::matches_range(version, &dynamic.value),
    }
}

// ── getSatisfyingVersion ──────────────────────────────────────────────────────

pub fn get_satisfying_version(versions: &[&str], range: &str) -> Option<String> {
    versions.iter().fold(None, |best: Option<String>, &v| {
        if matches_range(v, range) {
            match best {
                None => Some(v.to_owned()),
                Some(ref b) if maven::is_greater_than(v, b) => Some(v.to_owned()),
                _ => best,
            }
        } else {
            best
        }
    })
}

pub fn min_satisfying_version(versions: &[&str], range: &str) -> Option<String> {
    versions.iter().fold(None, |best: Option<String>, &v| {
        if matches_range(v, range) {
            match best {
                None => Some(v.to_owned()),
                Some(ref b) if maven::compare(v, b) == Ordering::Less => Some(v.to_owned()),
                _ => best,
            }
        } else {
            best
        }
    })
}

// ── getNewValue ───────────────────────────────────────────────────────────────

pub fn get_new_value(
    current_value: &str,
    _range_strategy: Option<&str>,
    new_version: &str,
) -> Option<String> {
    if is_version(current_value) {
        return Some(new_version.to_owned());
    }
    Some(maven::auto_extend_maven_range(current_value, new_version))
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: 'parseDynamicRevision("$input") === { type: "$type", value: "$value" }' — versioning/ivy/index.spec.ts line 10
    #[test]
    fn ivy_parse_dynamic_revision_ok() {
        let cases = [
            ("latest", RevType::Latest, ""),
            ("latest.release", RevType::Latest, "release"),
            ("latest.milestone", RevType::Latest, "milestone"),
            ("latest.integration", RevType::Latest, ""),
            ("1.0.+", RevType::Subrevision, "1.0"),
            ("1.2.3.+", RevType::Subrevision, "1.2.3"),
            ("[1.0,2.0]", RevType::Range, "[1.0,2.0]"),
            ("[1.0,2.0[", RevType::Range, "[1.0,2.0["),
            ("]1.0,2.0]", RevType::Range, "]1.0,2.0]"),
            ("]1.0,2.0[", RevType::Range, "]1.0,2.0["),
            ("[1.0,)", RevType::Range, "[1.0,)"),
            ("]1.0,)", RevType::Range, "]1.0,)"),
            ("(,2.0]", RevType::Range, "(,2.0]"),
            ("(,2.0[", RevType::Range, "(,2.0["),
        ];
        for (input, expected_type, expected_value) in &cases {
            let result = parse_dynamic_revision(input);
            assert!(
                result.is_some(),
                "parseDynamicRevision({input:?}) should be Some"
            );
            let rev = result.unwrap();
            assert_eq!(rev.rev_type, *expected_type, "type mismatch for {input:?}");
            assert_eq!(rev.value, *expected_value, "value mismatch for {input:?}");
        }
    }

    // Ported: 'parseDynamicRevision("$input") === null' — versioning/ivy/index.spec.ts line 33
    #[test]
    fn ivy_parse_dynamic_revision_null() {
        assert!(parse_dynamic_revision("").is_none());
        assert!(parse_dynamic_revision(".+").is_none());
        assert!(parse_dynamic_revision("[0,1),(1,)").is_none()); // multi-interval → null
    }

    // Ported: 'isValid("$input") === $expected' — versioning/ivy/index.spec.ts line 43
    #[test]
    fn ivy_is_valid() {
        let true_cases = [
            "1.0.0",
            "0",
            "0.1-2-sp",
            "1-final",
            "v1.0.0",
            "x1.0.0",
            "2.1.1.RELEASE",
            "Greenwich.SR1",
            "latest",
            "latest.release",
            "latest.milestone",
            "latest.integration",
            "1.0.+",
            "]0,1[",
            "[0,1]",
        ];
        let false_cases = ["", ".1", "1.", "-1", "1-", "1.0+", "[0,1),(1,2]"];
        for s in &true_cases {
            assert!(is_valid(s), "isValid({s:?}) should be true");
        }
        for s in &false_cases {
            assert!(!is_valid(s), "isValid({s:?}) should be false");
        }
    }

    // Ported: 'isVersion("$input") === $expected' — versioning/ivy/index.spec.ts line 72
    #[test]
    fn ivy_is_version() {
        let true_cases = [
            "1.0.0",
            "0",
            "0.1-2-sp",
            "1-final",
            "v1.0.0",
            "x1.0.0",
            "2.1.1.RELEASE",
            "Greenwich.SR1",
        ];
        let false_cases = [
            "",
            ".1",
            "1.",
            "-1",
            "1-",
            "latest",
            "latest.release",
            "latest.milestone",
            "latest.integration",
            "1.0.+",
            "1.0+",
            "]0,1[",
            "[0,1]",
            "[0,1),(1,2]",
        ];
        for s in &true_cases {
            assert!(is_version(s), "isVersion({s:?}) should be true");
        }
        for s in &false_cases {
            assert!(!is_version(s), "isVersion({s:?}) should be false");
        }
    }

    // Ported: 'matches("$version", "$range") === $expected' — versioning/ivy/index.spec.ts line 100
    #[test]
    fn ivy_matches() {
        let cases: &[(&str, &str, bool)] = &[
            ("", "latest", false),
            ("0", "", false),
            ("0", "latest", true),
            ("0", "latest.integration", true),
            ("0", "latest.release", false),
            ("release", "latest.release", true),
            ("0.release", "latest.release", true),
            ("0-release", "latest.release", true),
            ("0release", "latest.release", true),
            ("0.RELEASE", "latest.release", true),
            ("0", "latest.milestone", false),
            ("milestone", "latest.milestone", true),
            ("0.milestone", "latest.milestone", true),
            ("0-milestone", "latest.milestone", true),
            ("0milestone", "latest.milestone", true),
            ("0.MILESTONE", "latest.milestone", true),
            ("0", "1.0.+", false),
            ("1.1.0", "1.2.+", false),
            ("1.2.0", "1.2.+", true),
            ("1.2.milestone", "1.2.+", true),
            ("1.3", "1.2.+", false),
            ("1", "1", true),
            ("1", "0", false),
            ("1", "[0,1]", true),
            ("0", "(0,1)", false),
            ("0", "(0,1[", false),
            ("0", "]0,1)", false),
            ("1", "(0,1)", false),
            ("1", "(0,2)", true),
            ("1", "[0,2]", true),
            ("1", "(,1]", true),
            ("1", "(,1)", false),
            ("1", "[1,)", true),
            ("1", "(1,)", false),
        ];
        for (version, range, expected) in cases {
            assert_eq!(
                matches_range(version, range),
                *expected,
                "matches({version:?}, {range:?})"
            );
        }
    }

    // Ported: 'getNewValue("$currentValue", ...) === "$expected"' — versioning/ivy/index.spec.ts line 143
    #[test]
    fn ivy_get_new_value() {
        assert_eq!(
            get_new_value("1", Some("auto"), "1.1"),
            Some("1.1".to_owned())
        );
        assert_eq!(
            get_new_value("[1.2.3,]", Some("auto"), "1.2.4"),
            Some("[1.2.3,]".to_owned())
        );
    }

    // Ported: 'getSatisfyingVersion($versions, "$range") === $expected' — versioning/ivy/index.spec.ts line 160
    #[test]
    fn ivy_get_satisfying_version() {
        assert_eq!(
            get_satisfying_version(&["0", "1", "2"], "(,2)"),
            Some("1".to_owned())
        );
    }

    // Ported: 'isCompatible("$version") === $expected' — versioning/ivy/index.spec.ts line 170
    #[test]
    fn ivy_is_compatible() {
        assert!(is_version("1.2.0"));
    }

    // Ported: 'isSingleVersion("$version") === $expected' — versioning/ivy/index.spec.ts line 177
    #[test]
    fn ivy_is_single_version() {
        assert!(is_version("1.2.0"));
        assert!(!is_version("^1.2.0")); // ^ not a valid Maven version char
    }
}
