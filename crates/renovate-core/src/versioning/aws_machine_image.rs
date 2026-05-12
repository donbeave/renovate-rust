//! AWS Machine Image versioning.
//!
//! Renovate reference:
//! - `lib/modules/versioning/aws-machine-image/index.ts`

use std::sync::LazyLock;

use regex::Regex;

static AMI_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^ami-[0-9a-f]{17}$").unwrap());

pub fn is_valid(input: &str) -> bool {
    AMI_RE.is_match(input)
}

pub fn is_version(input: &str) -> bool {
    is_valid(input)
}

pub fn is_compatible(version: &str, _range: Option<&str>) -> bool {
    is_valid(version)
}

pub fn get_major(_version: &str) -> u64 {
    1
}

pub fn get_minor(_version: &str) -> u64 {
    0
}

pub fn get_patch(_version: &str) -> u64 {
    0
}

pub fn is_greater_than(_version: &str, _other: &str) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "should return 1.0.0" — versioning/aws-machine-image/index.spec.ts line 5
    #[test]
    fn parse_returns_fixed_components() {
        assert_eq!(get_major("ami-00e1b2c30011d4e5f"), 1);
        assert_eq!(get_minor("ami-00e1b2c30011d4e5f"), 0);
        assert_eq!(get_patch("ami-00e1b2c30011d4e5f"), 0);
    }

    // Ported: "should return true" — versioning/aws-machine-image/index.spec.ts line 13
    #[test]
    fn is_valid_returns_true_for_ami_id() {
        assert!(is_valid("ami-00e1b2c30011d4e5f"));
    }

    // Ported: "should return false" — versioning/aws-machine-image/index.spec.ts line 17
    #[test]
    fn is_valid_returns_false_for_short_ami_id() {
        assert!(!is_valid("ami-1"));
    }

    // Ported: "should return true" — versioning/aws-machine-image/index.spec.ts line 23
    #[test]
    fn is_version_returns_true_for_ami_id() {
        assert!(is_version("ami-00e1b2c30011d4e5f"));
    }

    // Ported: "should return false" — versioning/aws-machine-image/index.spec.ts line 27
    #[test]
    fn is_version_returns_false_for_short_ami_id() {
        assert!(!is_version("ami-1"));
    }

    // Ported: "should return true" — versioning/aws-machine-image/index.spec.ts line 33
    #[test]
    fn is_compatible_returns_true_for_ami_id_without_range() {
        assert!(is_compatible("ami-00e1b2c30011d4e5f", None));
    }

    // Ported: "should return false" — versioning/aws-machine-image/index.spec.ts line 37
    #[test]
    fn is_compatible_returns_false_for_short_ami_id_without_range() {
        assert!(!is_compatible("ami-1", None));
    }

    // Ported: "should return true" — versioning/aws-machine-image/index.spec.ts line 43
    #[test]
    fn is_compatible_returns_true_for_ami_id_with_range() {
        assert!(is_compatible("ami-00e1b2c30011d4e5f", Some("anything")));
    }

    // Ported: "should return false" — versioning/aws-machine-image/index.spec.ts line 51
    #[test]
    fn is_compatible_returns_false_for_short_ami_id_with_range() {
        assert!(!is_compatible("ami-1", Some("anything")));
    }

    // Ported: "should return true" — versioning/aws-machine-image/index.spec.ts line 58
    #[test]
    fn is_greater_than_returns_true_for_any_ami_pair() {
        assert!(is_greater_than("ami-00", "ami-99"));
        assert!(is_greater_than("ami-99", "ami-00"));
    }
}
