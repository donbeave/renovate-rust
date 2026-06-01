//! Unity3D editor versioning.
//!
//! Renovate reference:
//! - `lib/modules/versioning/unity3d/index.ts`

use std::{cmp::Ordering, sync::LazyLock};

use regex::Regex;

static UNITY_VERSION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?<major>\d+)\.(?<minor>\d+)\.(?<patch>\d+)(?<stream>[abfp])(?<stream_version>\d+) \([0-9a-f]{12}\)$",
    )
    .unwrap()
});

#[derive(Debug, Clone, PartialEq, Eq)]
struct UnityVersion {
    major: u64,
    minor: u64,
    patch: u64,
    stream: u8,
    stream_version: u64,
}

pub fn is_valid(input: &str) -> bool {
    parse(input).is_some()
}

pub fn is_stable(input: &str) -> bool {
    parse(input).is_some_and(|version| version.stream == stream_rank("f"))
}

pub fn equals(a: &str, b: &str) -> bool {
    parse(a)
        .zip(parse(b))
        .is_some_and(|(a, b)| a.cmp(&b) == Ordering::Equal)
}

pub fn is_greater_than(a: &str, b: &str) -> bool {
    parse(a)
        .zip(parse(b))
        .is_some_and(|(a, b)| a.cmp(&b) == Ordering::Greater)
}

fn parse(input: &str) -> Option<UnityVersion> {
    let caps = UNITY_VERSION_RE.captures(input)?;
    Some(UnityVersion {
        major: caps.name("major")?.as_str().parse().ok()?,
        minor: caps.name("minor")?.as_str().parse().ok()?,
        patch: caps.name("patch")?.as_str().parse().ok()?,
        stream: stream_rank(caps.name("stream")?.as_str()),
        stream_version: caps.name("stream_version")?.as_str().parse().ok()?,
    })
}

fn stream_rank(stream: &str) -> u8 {
    match stream {
        "a" => 0,
        "b" => 1,
        "f" => 2,
        "p" => 3,
        _ => 0,
    }
}

impl Ord for UnityVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        (
            self.major,
            self.minor,
            self.patch,
            self.stream,
            self.stream_version,
        )
            .cmp(&(
                other.major,
                other.minor,
                other.patch,
                other.stream,
                other.stream_version,
            ))
    }
}

impl PartialOrd for UnityVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isValid(\"$input\") === $expected" — versioning/unity3d/index.spec.ts line 4
    #[test]
    fn is_valid_matches_renovate_unity3d_spec() {
        let cases = [
            ("9.0.3", false),
            ("1.2019.3.22", false),
            ("3.0.0-beta", false),
            ("2.0.2-pre20191018090318", false),
            ("1.0.0+c30d7625", false),
            ("2.3.4-beta+1990ef74", false),
            ("17.04", false),
            ("3.0.0.beta", false),
            ("5.1.2-+", false),
            ("2022.2.12f1 (1234567890ab)", true),
            ("2022.2.11 (1234567890ab)", false),
            ("2021.1.10p1 (1234567890ab)", true),
            ("2021.1.9b1 (1234567890ab)", true),
            ("2021.1.1a1 (1234567890ab)", true),
        ];

        for (input, expected) in cases {
            assert_eq!(is_valid(input), expected, "is_valid({input})");
        }
    }

    // Ported: "isStable(\"$input\") === $expected" — versioning/unity3d/index.spec.ts line 25
    #[test]
    fn is_stable_matches_renovate_unity3d_spec() {
        let cases = [
            ("2022.2.12f1 (1234567890ab)", true),
            ("2021.1.10p1 (1234567890ab)", false),
            ("2021.1.9b1 (1234567890ab)", false),
            ("2021.1.1a1 (1234567890ab)", false),
        ];

        for (input, expected) in cases {
            assert_eq!(is_stable(input), expected, "is_stable({input})");
        }
    }

    // Ported: "equals($a, $b) === $expected" — versioning/unity3d/index.spec.ts line 35
    #[test]
    fn equals_matches_renovate_unity3d_spec() {
        let cases = [
            (
                "2022.2.12f1 (1234567890ab)",
                "2022.2.12f1 (1234567890ab)",
                true,
            ),
            (
                "2021.1.10p1 (1234567890ab)",
                "2021.1.10p1 (1234567890ab)",
                true,
            ),
            (
                "2021.1.9b1 (1234567890ab)",
                "2021.1.9b1 (1234567890ab)",
                true,
            ),
            (
                "2021.1.1a1 (1234567890ab)",
                "2021.1.1a1 (1234567890ab)",
                true,
            ),
        ];

        for (a, b, expected) in cases {
            assert_eq!(equals(a, b), expected, "equals({a}, {b})");
        }
    }

    // Ported: "isGreaterThan($a, $b) === $expected" — versioning/unity3d/index.spec.ts line 45
    #[test]
    fn is_greater_than_matches_renovate_unity3d_spec() {
        let cases = [
            (
                "2022.2.12f1 (1234567890ab)",
                "2022.2.12b1 (1234567890ab)",
                true,
            ),
            (
                "2022.2.12f1 (1234567890ab)",
                "2021.1.10p1 (1234567890ab)",
                true,
            ),
            (
                "2021.1.10p1 (1234567890ab)",
                "2021.1.9b1 (1234567890ab)",
                true,
            ),
            (
                "2021.1.9b1 (1234567890ab)",
                "2021.1.1a1 (1234567890ab)",
                true,
            ),
            (
                "2021.1.10p1 (1234567890ab)",
                "2022.2.12f1 (1234567890ab)",
                false,
            ),
            (
                "2021.1.9b1 (1234567890ab)",
                "2021.1.10p1 (1234567890ab)",
                false,
            ),
            (
                "2021.1.1a1 (1234567890ab)",
                "2021.1.9b1 (1234567890ab)",
                false,
            ),
            (
                "2022.2.12b1 (1234567890ab)",
                "2022.2.12f1 (1234567890ab)",
                false,
            ),
            (
                "2021.1.10p1 (1234567890ab)",
                "2022.2.12f1 (1234567890ab)",
                false,
            ),
            (
                "2021.1.9b1 (1234567890ab)",
                "2021.1.10p1 (1234567890ab)",
                false,
            ),
            (
                "2021.1.1a1 (1234567890ab)",
                "2021.1.9b1 (1234567890ab)",
                false,
            ),
            (
                "2022.2.12f1 (1234567890ab)",
                "2021.1.10p1 (1234567890ab)",
                true,
            ),
            (
                "2021.1.10p1 (1234567890ab)",
                "2021.1.9b1 (1234567890ab)",
                true,
            ),
            (
                "2021.1.9b1 (1234567890ab)",
                "2021.1.1a1 (1234567890ab)",
                true,
            ),
        ];

        for (a, b, expected) in cases {
            assert_eq!(is_greater_than(a, b), expected, "is_greater_than({a}, {b})");
        }
    }
}
