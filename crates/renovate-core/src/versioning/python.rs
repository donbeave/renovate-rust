//! Python versioning — mirrors `lib/modules/versioning/python/index.ts`.
//!
//! Delegates to Poetry for poetry-style ranges and PEP 440 otherwise.

use super::{pep440, poetry};

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Whether `input` is a valid Python version or range (poetry OR pep440).
pub fn is_valid(input: &str) -> bool {
    poetry::is_valid(input) || pep440::is_valid(input)
}

/// Whether `version` satisfies `range`.
pub fn matches(version: &str, range: &str) -> bool {
    if poetry::is_valid(range) {
        poetry::matches(version, range)
    } else {
        pep440::matches_range(version, range)
    }
}

/// Whether `version` is strictly below all bounds of `range`.
pub fn is_less_than_range(version: &str, range: &str) -> bool {
    if poetry::is_valid(range) {
        poetry::is_less_than_range(version, range)
    } else {
        pep440::is_less_than_range(version, range)
    }
}

/// Return the minimum version from `versions` that satisfies `range`.
pub fn min_satisfying_version(versions: &[&str], range: &str) -> Option<String> {
    if poetry::is_valid(range) {
        poetry::min_satisfying_version(versions, range)
    } else {
        pep440::min_satisfying_version(versions, range).map(ToOwned::to_owned)
    }
}

/// Return the maximum version from `versions` that satisfies `range`.
pub fn get_satisfying_version(versions: &[&str], range: &str) -> Option<String> {
    if poetry::is_valid(range) {
        poetry::get_satisfying_version(versions, range)
    } else {
        pep440::get_satisfying_version(versions, range).map(ToOwned::to_owned)
    }
}

/// Return a new constraint value for the given update strategy.
/// Delegates entirely to poetry.
pub fn get_new_value(
    current_value: &str,
    range_strategy: &str,
    current_version: Option<&str>,
    new_version: &str,
) -> Option<String> {
    poetry::get_new_value(current_value, range_strategy, current_version, new_version)
}

/// Whether `sub_range` is a subset of `super_range`.
/// Returns `None` when either range is not valid poetry syntax.
pub fn subset(sub_range: &str, super_range: &str) -> Option<bool> {
    if poetry::is_valid(sub_range) && poetry::is_valid(super_range) {
        poetry::subset(sub_range, super_range)
    } else {
        None
    }
}

/// Whether going from `current_version` to `new_version` is a breaking change.
pub fn is_breaking(current_version: &str, new_version: &str) -> bool {
    let current_major = poetry::get_major(current_version);
    let current_minor = poetry::get_minor(current_version);
    let new_major = poetry::get_major(new_version);
    let new_minor = poetry::get_minor(new_version);
    !(current_major == new_major && current_minor == new_minor)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isValid("$version") === $expected" — versioning/python/index.spec.ts line 4
    #[test]
    fn is_valid_cases() {
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
    }

    // Ported: "matches("$version", "$range") === "$expected"" — versioning/python/index.spec.ts line 28
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
        assert!(matches("1.4.5", "== 1.4.*"));
        assert!(!matches("1.5.5", "== 1.4.*"));
        assert!(matches("1.4.5", "== 1.4.5"));
        assert!(!matches("1.4.6", "== 1.4.5"));
    }

    // Ported: "isLessThanRange("$version", "$range") === "$expected"" — versioning/python/index.spec.ts line 54
    #[test]
    fn is_less_than_range_cases() {
        assert!(is_less_than_range("0.9.0", ">= 1.0.0 <= 2.0.0"));
        assert!(!is_less_than_range("1.9.0", ">= 1.0.0 <= 2.0.0"));
        assert!(!is_less_than_range("1.9.0", "== 2.7.*"));
    }

    // Ported: "minSatisfyingVersion($versions, "$range") === $expected" — versioning/python/index.spec.ts line 66
    #[test]
    fn min_satisfying_version_cases() {
        assert_eq!(
            min_satisfying_version(&["0.4.0", "0.5.0", "4.2.0", "4.3.0", "5.0.0"], "4.*, > 4.2"),
            Some("4.3.0".to_owned())
        );
        assert_eq!(
            min_satisfying_version(&["0.4.0", "0.5.0", "4.2.0", "5.0.0"], "^4.0.0"),
            Some("4.2.0".to_owned())
        );
        assert_eq!(
            min_satisfying_version(&["0.4.0", "0.5.0", "4.2.0", "5.0.0"], "^4.0.0, = 0.5.0"),
            None
        );
        assert_eq!(
            min_satisfying_version(
                &["0.4.0", "0.5.0", "4.2.0", "5.0.0"],
                "^4.0.0, > 4.1.0, <= 4.3.5"
            ),
            Some("4.2.0".to_owned())
        );
        assert_eq!(
            min_satisfying_version(&["0.4.0", "0.5.0", "4.2.0", "5.0.0"], "^6.2.0, 3.*"),
            None
        );
        assert_eq!(
            min_satisfying_version(&["0.8.0a2", "0.8.0a7"], "^0.8.0-alpha.0"),
            Some("0.8.0-alpha.2".to_owned())
        );
        assert_eq!(min_satisfying_version(&["1.0.0", "2.0.0"], "^3.0.0"), None);
        assert_eq!(
            min_satisfying_version(&["1.0.0", "2.0.0"], "== 3.7.*"),
            None
        );
    }

    // Ported: "getSatisfyingVersion($versions, "$range") === $expected" — versioning/python/index.spec.ts line 83
    #[test]
    fn get_satisfying_version_cases() {
        assert_eq!(
            get_satisfying_version(
                &["4.2.1", "0.4.0", "0.5.0", "4.0.0", "4.2.0", "5.0.0"],
                "4.*.0, < 4.2.5"
            ),
            Some("4.2.1".to_owned())
        );
        assert_eq!(
            get_satisfying_version(
                &["0.4.0", "0.5.0", "4.0.0", "4.2.0", "5.0.0", "5.0.3"],
                "5.0, > 5.0.0"
            ),
            Some("5.0.3".to_owned())
        );
        assert_eq!(
            get_satisfying_version(&["0.8.0a2", "0.8.0a7"], "^0.8.0-alpha.0"),
            Some("0.8.0-alpha.7".to_owned())
        );
        assert_eq!(get_satisfying_version(&["1.0.0", "2.0.0"], "^3.0.0"), None);
        assert_eq!(
            get_satisfying_version(&["1.0.0", "2.0.0"], "== 3.7.*"),
            None
        );
    }

    // Ported: "getNewValue("$currentValue", "$rangeStrategy", "$currentVersion", "$newVersion") === "$expected"" — versioning/python/index.spec.ts line 97
    // Delegates entirely to poetry::get_new_value; same cases as poetry spec.
    #[test]
    #[allow(clippy::type_complexity)]
    fn get_new_value_cases() {
        let cases: &[(&str, &str, Option<&str>, &str, Option<&str>)] = &[
            ("1.0.0", "bump", Some("1.0.0"), "1.1.0", Some("1.1.0")),
            ("=1.0.0", "bump", Some("1.0.0"), "1.1.0", Some("=1.1.0")),
            ("^1.0.0", "replace", Some("1.0.0"), "2.0.7", Some("^2.0.0")),
            ("1.0.0", "replace", Some("1.0.0"), "2.0.7", Some("2.0.7")),
        ];
        for (cv, rs, curv, nv, expected) in cases {
            let result = get_new_value(cv, rs, *curv, nv);
            assert_eq!(
                result.as_deref(),
                *expected,
                "get_new_value({cv:?}, {rs:?}, {curv:?}, {nv:?})"
            );
        }
    }

    // Ported: "subset("$a", "$b") === $expected" — versioning/python/index.spec.ts line 160
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
        // Invalid: "1.2.3foo" is not valid poetry → returns None
        assert_eq!(subset("1.2.3foo", "~1.1.0"), None);
    }

    // Ported: "isBreaking("$currentVersion", "$newVersion") === $expected" — versioning/python/index.spec.ts line 182
    #[test]
    fn is_breaking_cases() {
        assert!(is_breaking("3.7", "3.8"));
        assert!(is_breaking("3.7.0", "3.8.0"));
        assert!(!is_breaking("3.8.0", "3.8.1"));
        assert!(is_breaking("3.8.0", "4.0.0"));
    }
}
