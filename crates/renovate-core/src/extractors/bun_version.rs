//! Bun version-file extractor (``.bun-version``).
//!
//! Renovate reference: `lib/modules/manager/bun-version/index.ts`

/// A single extracted dependency from a `.bun-version` file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BunVersionDep {
    pub dep_name: &'static str,
    pub package_name: &'static str,
    pub current_value: String,
    pub datasource: &'static str,
    pub skip_reason: Option<&'static str>,
}

/// Extract the Bun version from a `.bun-version` file.
///
/// Returns `None` for empty content or files with more than one version line.
pub fn extract_package_file(content: &str) -> Option<Vec<BunVersionDep>> {
    if content.is_empty() {
        return None;
    }
    if content.split('\n').count() > 2 {
        return None;
    }
    let current_value = content.trim().to_owned();
    let skip_reason = semver::VersionReq::parse(&current_value)
        .is_err()
        .then_some("invalid-version");
    Some(vec![BunVersionDep {
        dep_name: "Bun",
        package_name: "bun",
        current_value,
        datasource: "npm",
        skip_reason,
    }])
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "returns a result" — lib/modules/manager/bun-version/index.spec.ts line 5
    #[test]
    fn returns_a_result() {
        let deps = extract_package_file("1.1.15\n").unwrap();
        assert_eq!(deps.len(), 1);
        let dep = &deps[0];
        assert_eq!(dep.dep_name, "Bun");
        assert_eq!(dep.package_name, "bun");
        assert_eq!(dep.current_value, "1.1.15");
        assert_eq!(dep.datasource, "npm");
        assert!(dep.skip_reason.is_none());
    }

    // Ported: "handles empty files" — lib/modules/manager/bun-version/index.spec.ts line 17
    #[test]
    fn handles_empty_files() {
        assert!(extract_package_file("").is_none());
    }

    // Ported: "handles no newline at the end" — lib/modules/manager/bun-version/index.spec.ts line 22
    #[test]
    fn handles_no_newline_at_end() {
        assert!(extract_package_file("1.1.15").is_some());
    }

    // Ported: "handles multiple lines" — lib/modules/manager/bun-version/index.spec.ts line 27
    #[test]
    fn handles_multiple_lines() {
        assert!(extract_package_file("1.1.15\n1.1.16\n").is_none());
    }

    // Ported: "handles invalid versions" — lib/modules/manager/bun-version/index.spec.ts line 32
    #[test]
    fn handles_invalid_versions() {
        let deps = extract_package_file("notaversion\n").unwrap();
        assert_eq!(deps[0].current_value, "notaversion");
        assert_eq!(deps[0].skip_reason, Some("invalid-version"));
    }

    // Ported: "handles ranges" — lib/modules/manager/bun-version/index.spec.ts line 45
    #[test]
    fn handles_ranges() {
        let deps = extract_package_file("1.0\n").unwrap();
        assert_eq!(deps[0].current_value, "1.0");
        assert!(deps[0].skip_reason.is_none());
    }
}
