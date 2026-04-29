//! Gradle Wrapper `gradle/wrapper/gradle-wrapper.properties` extractor.
//!
//! Reads the `distributionUrl` property and extracts the Gradle version number.
//!
//! Renovate reference:
//! - `lib/modules/manager/gradle-wrapper/extract.ts`
//! - `lib/modules/manager/gradle-wrapper/utils.ts` — `extractGradleVersion`
//! - Pattern: `/(^|/)gradle/wrapper/gradle-wrapper\.properties$/`
//!
//! ## Supported form
//!
//! ```properties
//! distributionBase=GRADLE_USER_HOME
//! distributionPath=wrapper/dists
//! distributionUrl=https\://services.gradle.org/distributions/gradle-8.4-bin.zip
//! zipStoreBase=GRADLE_USER_HOME
//! zipStorePath=wrapper/dists
//! ```

/// The extracted Gradle wrapper dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GradleWrapperDep {
    /// The Gradle version string (e.g. `"8.4"`, `"8.4.0"`).
    pub version: String,
}

/// Parse `gradle-wrapper.properties` and extract the Gradle version.
///
/// Returns `None` if no `distributionUrl` with a recognizable version is found.
pub fn extract(content: &str) -> Option<GradleWrapperDep> {
    for line in content.lines() {
        let trimmed = line.trim();
        // Gradle properties escape colons in URLs with backslash.
        if let Some(val) = trimmed
            .strip_prefix("distributionUrl=")
            .or_else(|| trimmed.strip_prefix("distributionUrl ="))
            && let Some(version) = parse_distribution_url(val.trim())
        {
            return Some(GradleWrapperDep { version });
        }
    }
    None
}

/// Extract the Gradle version from a `distributionUrl` value.
///
/// Handles:
/// - `https\://...gradle-8.4-bin.zip` (escaped colon)
/// - `https://...gradle-8.4.1-all.zip`
fn parse_distribution_url(url: &str) -> Option<String> {
    // Unescape `\:` → `:` (Gradle properties syntax).
    let url = url.replace("\\:", ":");

    // Pattern: `gradle-{version}-{type}.zip` at end of URL.
    // version = `\d+\.\d+(\.\d+)?(-\w+)?`
    let zip_name = url.split('/').next_back()?;

    // `gradle-8.4-bin.zip` or `gradle-8.4.1-all.zip`
    let stem = zip_name.strip_suffix(".zip")?;
    // stem = `gradle-8.4-bin` or `gradle-8.4.1-all`
    let without_prefix = stem.strip_prefix("gradle-")?;
    // without_prefix = `8.4-bin` or `8.4.1-all`
    // Split on last `-` to separate version from type (e.g. "bin", "all").
    let last_dash = without_prefix.rfind('-')?;
    let version_part = &without_prefix[..last_dash];
    // version_part must start with a digit (semver or prerelease like "7.0-milestone-1").
    if version_part.is_empty() || !version_part.starts_with(|c: char| c.is_ascii_digit()) {
        return None;
    }
    // Must contain at least one dot (e.g. "8.4") to distinguish from garbage.
    if !version_part.contains('.') {
        return None;
    }
    Some(version_part.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "extracts version for property file with distribution type \"bin\" in distributionUrl" — gradle-wrapper/extract.spec.ts line 33
    #[test]
    fn extracts_bin_version() {
        let content =
            "distributionUrl=https\\://services.gradle.org/distributions/gradle-8.4-bin.zip\n";
        let dep = extract(content).unwrap();
        assert_eq!(dep.version, "8.4");
    }

    // Ported: "extracts version for property file with distribution type \"all\" in distributionUrl" — gradle-wrapper/extract.spec.ts line 47
    #[test]
    fn extracts_all_version() {
        let content =
            "distributionUrl=https\\://services.gradle.org/distributions/gradle-8.4.1-all.zip\n";
        let dep = extract(content).unwrap();
        assert_eq!(dep.version, "8.4.1");
    }

    #[test]
    fn full_properties_file() {
        let content = r#"
distributionBase=GRADLE_USER_HOME
distributionPath=wrapper/dists
distributionUrl=https\://services.gradle.org/distributions/gradle-8.3-bin.zip
zipStoreBase=GRADLE_USER_HOME
zipStorePath=wrapper/dists
"#;
        let dep = extract(content).unwrap();
        assert_eq!(dep.version, "8.3");
    }

    // Ported: "returns null for property file without distributionUrl" — gradle-wrapper/extract.spec.ts line 24
    #[test]
    fn no_distribution_url_returns_none() {
        assert!(extract("distributionBase=GRADLE_USER_HOME\n").is_none());
    }

    #[test]
    fn empty_returns_none() {
        assert!(extract("").is_none());
    }

    // Ported: "extracts version for property file with prerelease version in distributionUrl" — gradle-wrapper/extract.spec.ts line 61
    #[test]
    fn prerelease_version_extracted() {
        let content = "distributionUrl=https\\://services.gradle.org/distributions/gradle-7.0-milestone-1-bin.zip\n";
        let dep = extract(content).unwrap();
        assert_eq!(dep.version, "7.0-milestone-1");
    }

    // Ported: "extracts version for property file with unnecessary whitespace in distributionUrl" — gradle-wrapper/extract.spec.ts line 75
    #[test]
    fn whitespace_around_value_handled() {
        let content =
            "distributionUrl= https\\://services.gradle.org/distributions/gradle-4.10.3-all.zip \n";
        let dep = extract(content).unwrap();
        assert_eq!(dep.version, "4.10.3");
    }

    // Ported: "returns null for property file with unsupported distributionUrl format" — gradle-wrapper/extract.spec.ts line 28
    #[test]
    fn unsupported_url_format_returns_none() {
        let content = "distributionUrl=https://example.com/gradle/custom-gradle.zip\n";
        assert!(extract(content).is_none());
    }
}
