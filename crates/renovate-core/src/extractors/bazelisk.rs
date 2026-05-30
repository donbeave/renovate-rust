//! Bazelisk `.bazelversion` extractor.
//!
//! Reads the first non-empty line of a `.bazelversion` file and treats it as
//! the Bazel version to track via GitHub Releases.
//!
//! Renovate reference:
//! - `lib/modules/manager/bazelisk/extract.ts`
//! - `lib/modules/manager/bazelisk/index.ts`

/// A single dependency extracted from a `.bazelversion` file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BazeliskDep {
    pub dep_name: &'static str,
    pub package_name: &'static str,
    pub current_value: String,
    pub datasource: &'static str,
    pub versioning: &'static str,
}

/// Extract the Bazel version from a `.bazelversion` file.
///
/// Returns `None` for empty content.
pub fn extract(content: &str) -> Option<Vec<BazeliskDep>> {
    let current_value = content.lines().next()?.trim().to_owned();
    if current_value.is_empty() {
        return None;
    }
    Some(vec![BazeliskDep {
        dep_name: "bazel",
        package_name: "bazelbuild/bazel",
        current_value,
        datasource: "github-releases",
        versioning: "semver",
    }])
}

#[cfg(test)]
mod tests {
    use super::*;

    // Rust-specific: bazelisk behavior test
    #[test]
    fn extracts_version_from_first_line() {
        let deps = extract("6.4.0\n").unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "bazel");
        assert_eq!(deps[0].package_name, "bazelbuild/bazel");
        assert_eq!(deps[0].current_value, "6.4.0");
        assert_eq!(deps[0].datasource, "github-releases");
        assert_eq!(deps[0].versioning, "semver");
    }

    // Rust-specific: bazelisk behavior test
    #[test]
    fn trims_whitespace() {
        let deps = extract("  7.0.0  \n").unwrap();
        assert_eq!(deps[0].current_value, "7.0.0");
    }

    // Rust-specific: bazelisk behavior test
    #[test]
    fn returns_none_for_empty() {
        assert!(extract("").is_none());
    }

    // Rust-specific: bazelisk behavior test
    #[test]
    fn returns_none_for_whitespace_only() {
        assert!(extract("   \n  \n").is_none());
    }
}
