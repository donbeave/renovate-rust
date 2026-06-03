//! Red Hat versioning.
//!
//! Renovate reference:
//! - `lib/modules/versioning/redhat/index.ts`

use std::sync::LazyLock;

use regex::Regex;

static REDHAT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^v?(?<major>\d+)(?:\.(?<minor>\d+))?(?:\.(?<patch>\d+))?(?:\.GA)?(?:-(?<release_major>\d+)(?:\.(?<release_minor>\d+))?)?$",
    )
    .unwrap()
});

fn parse(input: &str) -> Option<[u64; 5]> {
    let captures = REDHAT_RE.captures(input)?;

    let component = |name| {
        captures
            .name(name)
            .map_or(Some(0), |value| value.as_str().parse::<u64>().ok())
    };

    Some([
        component("major")?,
        component("minor")?,
        component("patch")?,
        component("release_major")?,
        component("release_minor")?,
    ])
}

pub fn is_valid(input: &str) -> bool {
    parse(input).is_some()
}

pub fn is_greater_than(a: Option<&str>, b: &str) -> bool {
    let Some(a) = a else {
        return true;
    };

    let Some(a) = parse(a) else {
        return false;
    };
    let Some(b) = parse(b) else {
        return false;
    };

    a > b
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "isValid(\"$input\") === $expected" — lib/modules/versioning/redhat/index.spec.ts line 4
    #[test]
    fn is_valid_matches_renovate_redhat_spec() {
        let cases = [
            ("1", true),
            ("17.04", true),
            ("9.0.0", true),
            ("1-632", true),
            ("1.2-62", true),
            ("1.2.3-62", true),
            ("8.5-239.1651231664", true),
            ("8.5.0-239.1651231664", true),
            ("9.0.0-62", true),
            ("1.0.0.GA-20.1770236070", true),
            ("v0.4.0-383", true),
            ("1.2.3.4-62", false),
            ("1.2.3 -62", false),
            ("8.5-", false),
            ("3.0.0-beta", false),
            ("2.0.2-pre2019100318", false),
            ("1.0.0+c30d7625", false),
            ("2.3.4-beta+1990ef74", false),
            ("5.1.2-+", false),
            ("5.1.2+1", false),
        ];

        for (input, expected) in cases {
            assert_eq!(is_valid(input), expected, "is_valid({input})");
        }
    }

    // Ported: "isGreaterThan($a, $b) === $expected" — lib/modules/versioning/redhat/index.spec.ts line 30
    #[test]
    fn is_greater_than_matches_renovate_redhat_spec() {
        let cases = [
            (Some("3-57"), "3-2", true),
            (Some("8.6"), "8.4", true),
            (Some("3.2.32-12"), "2.3.7-15", true),
            (Some("7.9-49"), "7.9-46", true),
            (Some("1-121"), "1-81.1618436879", true),
            (Some("7.7-18.1575996389"), "7.7-18", true),
            (Some("7.9-698.1655292976"), "7.9-628", true),
            (Some("7.9-698.1655292976"), "7.9-628.1645808164", true),
            (Some("7.9-698.1655292976"), "7.9-698.1645808164", true),
            (Some("8.6-754.1655117782"), "8.5-239.1651231664", true),
            (Some("8.6-754.1655117782"), "8.5-754.1651231664", true),
            (Some("8.6-754.1652117782"), "8.5-754.1651231664", true),
            (Some("8.45-754.1652117782"), "8.5-754.1651231664", true),
            (Some("8.6-754.1655117782"), "8.6-754", true),
            (Some("9.0.0-1471"), "9.0.0", true),
            (Some("9.0.0-1471.1655190711"), "9.0.0-1471", true),
            (None, "1.2.0", true),
            (Some("1.2.0"), "1.2.0-2", false),
            (Some("1.9"), "2", false),
            (Some("1.9"), "1.9.1", false),
            (Some("2.4.0-50"), "2.4.0-51", false),
            (Some("2.4.0-51"), "2.4.0-51", false),
            (Some("1-19"), "1-19.1655193074", false),
            (Some("8.5-754.1651117782"), "8.5-754.1652231664", false),
        ];

        for (a, b, expected) in cases {
            assert_eq!(
                is_greater_than(a, b),
                expected,
                "is_greater_than({a:?}, {b})"
            );
        }
    }
}
