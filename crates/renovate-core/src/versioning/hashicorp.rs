//! HashiCorp versioning and update-decision logic.
//!
//! Used by Terraform providers and modules. The constraint syntax is a superset
//! of semver ranges with an additional `~>` (pessimistic constraint) operator.
//!
//! Renovate reference:
//! - `lib/modules/versioning/hashicorp/index.ts`
//! - `lib/modules/versioning/hashicorp/convertor.ts` — `hashicorp2npm`
//!
//! ## Operator semantics
//!
//! | Constraint | Meaning |
//! |---|---|
//! | `= 5.0.0` | Exactly 5.0.0 |
//! | `!= 5.0.0` | Any version except 5.0.0 (treated as unconstrained here) |
//! | `>= 5.0.0` | At least 5.0.0 |
//! | `<= 5.0.0` | At most 5.0.0 |
//! | `> 5.0.0` | Strictly newer than 5.0.0 |
//! | `< 5.0.0` | Strictly older than 5.0.0 |
//! | `~> 5.0` | `>= 5.0.0, < 6.0.0` (minor-level pessimistic) |
//! | `~> 5.0.1` | `>= 5.0.1, < 5.1.0` (patch-level pessimistic) |
//! | `~> 5` | `>= 5.0.0` (major-only: no upper bound) |
//!
//! ## Update decision
//!
//! For range constraints, the pinned lower-bound version is compared to latest.
//! An update is available when `latest > lower_bound` by semver ordering.

use std::sync::LazyLock;

use regex::Regex;
use semver::Version;

/// Regex to parse a single HashiCorp constraint element.
/// Group 1 = operator (`~>`, `>=`, `<=`, `>`, `<`, `=`, `!=`, or empty).
/// Group 2 = version string (digits and dots).
static CONSTRAINT_ELEM: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s*(?<op>~>|>=|<=|!=|>|<|=)?\s*v?(?<ver>[\d]+(?:\.[\d]+(?:\.[\d]+(?:[-.][0-9A-Za-z]+)*)?)?)?\s*$").unwrap()
});

/// Parsed single constraint element.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ConstraintElem {
    op: Op,
    version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    Pessimistic, // ~>
    Gte,         // >=
    Lte,         // <=
    Gt,          // >
    Lt,          // <
    Eq,          // = (or no operator)
    Neq,         // !=
}

/// Parse all comma-separated elements of a HashiCorp constraint string.
fn parse_constraint(constraint: &str) -> Vec<ConstraintElem> {
    constraint
        .split(',')
        .filter_map(|part| {
            let cap = CONSTRAINT_ELEM.captures(part.trim())?;
            let op_str = cap.name("op").map(|m| m.as_str()).unwrap_or("");
            let ver = cap.name("ver").map(|m| m.as_str()).unwrap_or("").to_owned();
            if ver.is_empty() {
                return None;
            }
            let op = match op_str {
                "~>" => Op::Pessimistic,
                ">=" => Op::Gte,
                "<=" => Op::Lte,
                ">" => Op::Gt,
                "<" => Op::Lt,
                "!=" => Op::Neq,
                _ => Op::Eq,
            };
            Some(ConstraintElem { op, version: ver })
        })
        .collect()
}

/// Extract the lower-bound version from a HashiCorp constraint.
///
/// For `~> 5.0.1` → `5.0.1`, `>= 2.0.0` → `2.0.0`, `= 3.0` → `3.0`.
/// For multi-element constraints (`>= 1.0, < 2.0`), returns the lower bound
/// of the first `>=`, `>`, or `~>` element, or the first `=` element.
pub fn lower_bound(constraint: &str) -> Option<String> {
    let elems = parse_constraint(constraint);
    // Prefer the element with a lower-bounding operator.
    for elem in &elems {
        match elem.op {
            Op::Pessimistic | Op::Gte | Op::Gt | Op::Eq => return Some(elem.version.clone()),
            _ => {}
        }
    }
    None
}

/// Parse a version string as semver, padding missing components with 0.
fn parse_version(v: &str) -> Option<Version> {
    // Pad to 3 components.
    let parts: Vec<&str> = v.splitn(3, '.').collect();
    let padded = match parts.len() {
        1 => format!("{}.0.0", parts[0]),
        2 => format!("{}.{}.0", parts[0], parts[1]),
        _ => v.to_owned(),
    };
    Version::parse(&padded).ok()
}

/// Update summary produced by [`hashicorp_update_summary`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashicorpUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Produce an update summary for a HashiCorp-versioned dependency.
///
/// `update_available` is `true` when `latest` is strictly newer than the
/// lower bound extracted from `current_value`, by semver comparison.
pub fn hashicorp_update_summary(
    current_value: &str,
    latest: Option<&str>,
) -> HashicorpUpdateSummary {
    let update_available = latest
        .filter(|l| !l.is_empty() && !current_value.is_empty())
        .is_some_and(|latest_ver| {
            let Some(lb) = lower_bound(current_value) else {
                return false;
            };
            let Some(lv) = parse_version(latest_ver) else {
                return false;
            };
            let Some(cv) = parse_version(&lb) else {
                return false;
            };
            lv > cv
        });

    HashicorpUpdateSummary {
        current_value: current_value.to_owned(),
        latest: latest.map(|s| s.to_owned()),
        update_available,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lower_bound_pessimistic_patch() {
        assert_eq!(lower_bound("~> 5.0.1"), Some("5.0.1".to_owned()));
    }

    #[test]
    fn lower_bound_pessimistic_minor() {
        assert_eq!(lower_bound("~> 5.0"), Some("5.0".to_owned()));
    }

    #[test]
    fn lower_bound_pessimistic_major_only() {
        assert_eq!(lower_bound("~> 5"), Some("5".to_owned()));
    }

    #[test]
    fn lower_bound_gte() {
        assert_eq!(lower_bound(">= 2.0.0"), Some("2.0.0".to_owned()));
    }

    #[test]
    fn lower_bound_exact() {
        assert_eq!(lower_bound("= 3.1.4"), Some("3.1.4".to_owned()));
    }

    #[test]
    fn lower_bound_bare_version() {
        assert_eq!(lower_bound("3.1.4"), Some("3.1.4".to_owned()));
    }

    #[test]
    fn lower_bound_range() {
        // `>= 1.0, < 2.0` — lower bound is 1.0
        assert_eq!(lower_bound(">= 1.0, < 2.0"), Some("1.0".to_owned()));
    }

    #[test]
    fn update_available_when_newer() {
        let s = hashicorp_update_summary("~> 5.0", Some("5.7.3"));
        assert!(s.update_available);
        assert_eq!(s.latest.as_deref(), Some("5.7.3"));
    }

    #[test]
    fn no_update_when_same_lower_bound() {
        // Lower bound is 5.0.1; latest is also 5.0.1
        let s = hashicorp_update_summary("~> 5.0.1", Some("5.0.1"));
        assert!(!s.update_available);
    }

    #[test]
    fn no_update_when_older() {
        // Lower bound 5.0, latest 4.9.9 (unlikely but defensive)
        let s = hashicorp_update_summary("~> 5.0", Some("4.9.9"));
        assert!(!s.update_available);
    }

    #[test]
    fn no_update_when_latest_none() {
        let s = hashicorp_update_summary("~> 5.0", None);
        assert!(!s.update_available);
    }

    #[test]
    fn exact_pinned_update() {
        let s = hashicorp_update_summary("= 5.0.0", Some("5.1.0"));
        assert!(s.update_available);
    }

    #[test]
    fn gte_constraint_update() {
        let s = hashicorp_update_summary(">= 4.0.0", Some("5.0.0"));
        assert!(s.update_available);
    }

    #[test]
    fn multi_element_constraint() {
        // >= 2.0.0, < 3.0.0 — lower bound 2.0.0
        let s = hashicorp_update_summary(">= 2.0.0, < 3.0.0", Some("2.5.0"));
        assert!(s.update_available);
    }

    #[test]
    fn version_padding() {
        // ~> 5 → lower bound 5.0.0; latest 5.1.0 → update
        let s = hashicorp_update_summary("~> 5", Some("5.1.0"));
        assert!(s.update_available);
    }
}
