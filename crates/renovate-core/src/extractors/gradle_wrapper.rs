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
    // Split on last `-` to separate version from type.
    let last_dash = without_prefix.rfind('-')?;
    let version_part = &without_prefix[..last_dash];
    // `version_part` should look like `8.4` or `8.4.1`.
    if version_part.is_empty() || !version_part.chars().all(|c| c.is_ascii_digit() || c == '.') {
        return None;
    }
    Some(version_part.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_bin_version() {
        let content =
            "distributionUrl=https\\://services.gradle.org/distributions/gradle-8.4-bin.zip\n";
        let dep = extract(content).unwrap();
        assert_eq!(dep.version, "8.4");
    }

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

    #[test]
    fn no_distribution_url_returns_none() {
        assert!(extract("distributionBase=GRADLE_USER_HOME\n").is_none());
    }

    #[test]
    fn empty_returns_none() {
        assert!(extract("").is_none());
    }
}
