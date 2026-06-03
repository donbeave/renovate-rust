//! Git hash versioning.
//!
//! Renovate reference:
//! - `lib/modules/versioning/git/index.ts`

use std::sync::LazyLock;

use regex::Regex;

static GIT_HASH_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^[0-9a-f]{7,40}$").unwrap());

pub fn is_valid(input: &str) -> bool {
    GIT_HASH_RE.is_match(input)
}

pub fn is_compatible(version: &str, _range: &str) -> bool {
    is_valid(version)
}

pub fn is_greater_than(_a: &str, _b: &str) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isValid(\"$input\") === $expected" — lib/modules/versioning/git/index.spec.ts line 4
    #[test]
    fn is_valid_matches_renovate_git_spec() {
        let cases = [
            ("", false),
            ("2", false),
            ("29", false),
            ("29c", false),
            ("29c7", false),
            ("29c79", false),
            ("29c792", false),
            ("29c7921", true),
            ("29c792109259545157f4bc3f8d43f47ffcf34e20", true),
            ("foobar", false),
        ];

        for (input, expected) in cases {
            assert_eq!(is_valid(input), expected, "is_valid({input})");
        }
    }

    // Ported: "isCompatible(\"$version\") === $expected" — lib/modules/versioning/git/index.spec.ts line 20
    #[test]
    fn is_compatible_matches_renovate_git_spec() {
        let cases = [("", "", false), ("1234567890aBcDeF", "", true)];

        for (version, range, expected) in cases {
            assert_eq!(
                is_compatible(version, range),
                expected,
                "is_compatible({version})"
            );
        }
    }

    // Ported: "isGreaterThan(\"$a\", \"$b\") === $expected" — lib/modules/versioning/git/index.spec.ts line 32
    #[test]
    fn is_greater_than_matches_renovate_git_spec() {
        let cases = [
            ("", "", false),
            ("abc", "bca", false),
            ("123", "321", false),
        ];

        for (a, b, expected) in cases {
            assert_eq!(is_greater_than(a, b), expected, "is_greater_than({a}, {b})");
        }
    }
}
