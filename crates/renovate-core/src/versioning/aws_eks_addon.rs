//! AWS EKS Addon versioning.
//!
//! Ports `lib/modules/versioning/aws-eks-addon/index.ts`.
//! Version format: `v?major.minor.patch-eksbuild.build`

use std::sync::LazyLock;

use regex::Regex;

static VERSION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?x)^
        v?
        (?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)
        (?P<compatibility>-eksbuild\.)
        (?P<build>\d+)
        $",
    )
    .unwrap()
});

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedVersion {
    major: u64,
    minor: u64,
    patch: u64,
    build: u64,
    compatibility: String,
}

fn parse(version: &str) -> Option<ParsedVersion> {
    let caps = VERSION_RE.captures(version)?;
    Some(ParsedVersion {
        major: caps["major"].parse().ok()?,
        minor: caps["minor"].parse().ok()?,
        patch: caps["patch"].parse().ok()?,
        build: caps["build"].parse().ok()?,
        compatibility: caps["compatibility"].to_string(),
    })
}

pub fn is_valid(version: &str) -> bool {
    parse(version).is_some()
}

pub fn get_major(version: &str) -> Option<u64> {
    parse(version).map(|p| p.major)
}

pub fn get_minor(version: &str) -> Option<u64> {
    parse(version).map(|p| p.minor)
}

pub fn get_patch(version: &str) -> Option<u64> {
    parse(version).map(|p| p.patch)
}

pub fn is_compatible(version: &str, current: Option<&str>) -> bool {
    let Some(current) = current else {
        return false;
    };
    let (Some(pv), Some(pc)) = (parse(version), parse(current)) else {
        return false;
    };
    pv.compatibility == pc.compatibility
}

fn compare(a: &str, b: &str) -> i32 {
    match (parse(a), parse(b)) {
        (Some(pa), Some(pb)) => {
            for (va, vb) in [
                (pa.major, pb.major),
                (pa.minor, pb.minor),
                (pa.patch, pb.patch),
                (pa.build, pb.build),
            ] {
                if va != vb {
                    return if va > vb { 1 } else { -1 };
                }
            }
            0
        }
        (Some(_), None) => 1,
        (None, Some(_)) => -1,
        (None, None) => 0,
    }
}

pub fn is_greater_than(a: &str, b: &str) -> bool {
    compare(a, b) > 0
}

pub fn get_satisfying_version<'a>(versions: &[&'a str], range: &str) -> Option<&'a str> {
    versions.iter().find(|&&v| v == range).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "should return 1.23.7 and release version" — versioning/aws-eks-addon/index.spec.ts line 5
    #[test]
    fn get_major_minor_patch_matches_renovate_aws_eks_addon_index_spec() {
        assert_eq!(get_major("v1.20.7-eksbuild.1"), Some(1));
        assert_eq!(get_minor("v1.23.7-eksbuild.1"), Some(23));
        assert_eq!(get_patch("v1.20.7-eksbuild.1"), Some(7));
    }

    // Ported: "isValid("$input") === $expected" — versioning/aws-eks-addon/index.spec.ts line 13
    #[test]
    fn is_valid_matches_renovate_aws_eks_addon_index_spec() {
        let cases = [
            ("", false),
            (".1..", false),
            ("abrakadabra", false),
            ("v1", false),
            ("v1.", false),
            ("v1...-eksbuild.1", false),
            ("v1-eksbuild.1", false),
            ("v1.a-eksbuild.1", false),
            ("v1.23-eksbuild.1", false),
            ("1.23.1-eksbuild.a", false),
            ("v1.11.7", false),
            ("v1.11.7.6", false),
            ("v1.11.7-noneksbuild", false),
            ("v1.11.7-noneksbuild.1", false),
            ("v1.11.7-eksbuild", false),
            ("v1.11.7.3-eksbuild.1", false),
            ("v1.23.1-eksbuild.1", true),
            ("1.23.1-eksbuild.1", true),
            ("v1.23.1-eksbuild.11", true),
        ];
        for (input, expected) in cases {
            assert_eq!(is_valid(input), expected, "is_valid({input:?})");
        }
    }

    // Ported: "isValid("$input") === $expected" — versioning/aws-eks-addon/index.spec.ts line 41
    #[test]
    fn is_version_matches_renovate_aws_eks_addon_index_spec() {
        // isVersion has identical behavior to isValid for this module
        let cases = [
            ("", false),
            ("abrakadabra", false),
            ("v1", false),
            ("v1.", false),
            ("v1-eksbuild.1", false),
            ("v1.a-eksbuild.1", false),
            ("v1.23-eksbuild.1", false),
            ("1.23.1-eksbuild.a", false),
            ("v1.11.7", false),
            ("v1.11.7.6", false),
            ("v1.11.7-noneksbuild", false),
            ("v1.11.7-noneksbuild.1", false),
            ("v1.11.7-eksbuild", false),
            ("v1.11.7.3-eksbuild.1", false),
            ("v1.23.1-eksbuild.1", true),
            ("1.23.1-eksbuild.1", true),
            ("v1.23.1-eksbuild.11", true),
        ];
        for (input, expected) in cases {
            assert_eq!(is_valid(input), expected, "is_version({input:?})");
        }
    }

    // Ported: "isCompatible("$input") === $expected" — versioning/aws-eks-addon/index.spec.ts line 67
    #[test]
    fn is_compatible_single_arg_matches_renovate_aws_eks_addon_index_spec() {
        let cases = [
            ("", false),
            ("abrakadabra", false),
            ("v1", false),
            ("v1.", false),
            ("v1-eksbuild.1", false),
            ("v1.a-eksbuild.1", false),
            ("v1.23-eksbuild.1", false),
            ("1.23.1-eksbuild.1", false),
            ("1.23.1-eksbuild.a", false),
            ("v1.11.7", false),
            ("v1.11.7.6", false),
            ("v1.11.7-noneksbuild", false),
            ("v1.11.7-noneksbuild.1", false),
            ("v1.11.7-eksbuild", false),
            ("v1.11.7.3-eksbuild.1", false),
        ];
        for (input, expected) in cases {
            assert_eq!(
                is_compatible(input, None),
                expected,
                "is_compatible({input:?}, None)"
            );
        }
    }

    // Ported: "isCompatible($version, $current) === $expected" — versioning/aws-eks-addon/index.spec.ts line 91
    #[test]
    fn is_compatible_two_args_matches_renovate_aws_eks_addon_index_spec() {
        let cases = [
            ("1.23.1-eksbuild.1", "1.23.1-eksbuild.2", true),
            ("v1.23.1-eksbuild.1", "1.23.1-eksbuild.2", true),
            ("v1.23.1-eksbuild.1", "1.23.1-eksbuild.21", true),
            ("v1.11.7-eksbuild.1", "v1.11.7-noneksbuild.1", false),
            ("v1.11.7", "v1.11.7-noneksbuild.1", false),
            ("v1-eksbuild.1", "artful", false),
            ("v1.11.7.1-eksbuild.1", "v1.11.7-eksbuild.1", false),
        ];
        for (version, current, expected) in cases {
            assert_eq!(
                is_compatible(version, Some(current)),
                expected,
                "is_compatible({version:?}, Some({current:?}))"
            );
        }
    }

    // Ported: "isGreaterThan($version, $other) === $expected" — versioning/aws-eks-addon/index.spec.ts line 110
    #[test]
    fn is_greater_than_matches_renovate_aws_eks_addon_index_spec() {
        let cases = [
            ("v1.11.7-eksbuild.1", "v1.11.7-eksbuild.0", true),
            ("v1.11.7-eksbuild.11", "v1.11.7-eksbuild.1", true),
            ("v1.22.7-eksbuild.2", "v1.20.7-eksbuild.1", true),
            ("v1.22.7-eksbuild.2", "v1.22.7", true),
            ("v1.20.7-eksbuild.1", "v2.0.0", true),
            ("v1.20.7-eksbuild.1", "v1.20.7-eksbuild.2", false),
            ("v1.20.6-eksbuild.1", "v1.20.7-eksbuild.2", false),
            ("v1.20.7-eksbuild.1", "v2.0.0-eksbuild.1", false),
        ];
        for (version, other, expected) in cases {
            assert_eq!(
                is_greater_than(version, other),
                expected,
                "is_greater_than({version:?}, {other:?})"
            );
        }
    }

    // Ported: "getSatisfyingVersion" — versioning/aws-eks-addon/index.spec.ts line 129
    #[test]
    fn get_satisfying_version_matches_renovate_aws_eks_addon_index_spec() {
        assert_eq!(
            get_satisfying_version(&["v1.20.7-eksbuild.1"], "v1.20.7-eksbuild.1"),
            Some("v1.20.7-eksbuild.1")
        );
        assert_eq!(
            get_satisfying_version(
                &["v1.20.7-eksbuild.1", "v1.20.7-eksbuild.2", "v1.20.7-eksbuild.7"],
                "v1.20.7-eksbuild.3"
            ),
            None
        );
        assert_eq!(
            get_satisfying_version(
                &["v1.20.7-eksbuild.1", "v1.20.7-eksbuild.2"],
                "v1.20.7-eksbuild.3"
            ),
            None
        );
    }
}
