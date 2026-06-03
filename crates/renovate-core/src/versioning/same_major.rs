//! @parity lib/modules/versioning/same-major/index.ts full
//!
//! Same-major versioning.
//! Delegates to semver-coerced, converting a version input to `>=X.Y.Z <(major+1)`.

use super::semver_coerced;

fn massage_version(input: &str) -> String {
    if !semver_coerced::is_single_version(input) {
        return input.to_owned();
    }
    match semver_coerced::get_major(input) {
        Some(major) => format!(">={input} <{}", major + 1),
        None => input.to_owned(),
    }
}

pub fn is_greater_than(version: &str, other: &str) -> bool {
    let vm = semver_coerced::get_major(version);
    let om = semver_coerced::get_major(other);
    match (vm, om) {
        (Some(v), Some(o)) if v > 0 && o > 0 => v > o,
        _ => false,
    }
}

pub fn matches(version: &str, range: &str) -> bool {
    semver_coerced::matches(version, &massage_version(range))
}

pub fn get_satisfying_version(versions: &[&str], range: &str) -> Option<String> {
    semver_coerced::get_satisfying_version(versions, &massage_version(range))
}

pub fn min_satisfying_version(versions: &[&str], range: &str) -> Option<String> {
    semver_coerced::min_satisfying_version(versions, &massage_version(range))
}

pub fn is_less_than_range(version: &str, range: &str) -> bool {
    semver_coerced::is_less_than_range(version, &massage_version(range))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "should return true" — lib/modules/versioning/same-major/index.spec.ts line 5
    #[test]
    fn is_greater_than_true_matches_renovate_same_major_index_spec() {
        assert!(is_greater_than("4.0.0", "3.0.0"));
    }

    // Ported: "should return false" — lib/modules/versioning/same-major/index.spec.ts line 9
    #[test]
    fn is_greater_than_false_matches_renovate_same_major_index_spec() {
        assert!(!is_greater_than("2.0.2", "3.1.0"));
        assert!(!is_greater_than("3.1.0", "3.0.0")); // same major
        assert!(!is_greater_than("3.0.0", "3.0.0")); // equal
        assert!(!is_greater_than("a", "3.0.0")); // invalid
    }

    // Ported: "should return true when version has same major" — lib/modules/versioning/same-major/index.spec.ts line 18
    #[test]
    fn matches_true_matches_renovate_same_major_index_spec() {
        assert!(matches("1.0.1", "1.0.0"));
        assert!(matches("1.0.0", "1.0.0"));
    }

    // Ported: "should return false when version has different major" — lib/modules/versioning/same-major/index.spec.ts line 23
    #[test]
    fn matches_diff_major_matches_renovate_same_major_index_spec() {
        assert!(!matches("2.0.1", "1.0.0"));
    }

    // Ported: "should return false when version is out of range" — lib/modules/versioning/same-major/index.spec.ts line 27
    #[test]
    fn matches_out_of_range_matches_renovate_same_major_index_spec() {
        assert!(!matches("1.2.3", "1.2.4"));
        assert!(!matches("2.0.0", "1.2.4"));
        assert!(!matches("3.2.4", "1.2.4"));
    }

    // Ported: "should return false when version is invalid" — lib/modules/versioning/same-major/index.spec.ts line 33
    #[test]
    fn matches_invalid_matches_renovate_same_major_index_spec() {
        assert!(!matches("1.0.0", "xxx"));
    }

    // Ported: "should return max satisfying version in range" — lib/modules/versioning/same-major/index.spec.ts line 39
    #[test]
    fn get_satisfying_version_matches_renovate_same_major_index_spec() {
        assert_eq!(
            get_satisfying_version(&["1.0.0", "1.0.4", "1.3.0", "2.0.0"], "1.0.3"),
            Some("1.3.0".to_owned())
        );
    }

    // Ported: "should return min satisfying version in range" — lib/modules/versioning/same-major/index.spec.ts line 50
    #[test]
    fn min_satisfying_version_matches_renovate_same_major_index_spec() {
        assert_eq!(
            min_satisfying_version(&["1.0.0", "1.0.4", "1.3.0", "2.0.0"], "1.0.3"),
            Some("1.0.4".to_owned())
        );
    }

    // Ported: "should return true" — lib/modules/versioning/same-major/index.spec.ts line 61
    #[test]
    fn is_less_than_range_true_matches_renovate_same_major_index_spec() {
        assert!(is_less_than_range("2.0.2", "3.0.0"));
    }

    // Ported: "should return false" — lib/modules/versioning/same-major/index.spec.ts line 65
    #[test]
    fn is_less_than_range_false_matches_renovate_same_major_index_spec() {
        assert!(!is_less_than_range("4.0.0", "3.0.0"));
        assert!(!is_less_than_range("3.1.0", "3.0.0"));
    }
}
