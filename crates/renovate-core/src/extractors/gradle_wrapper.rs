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

use std::sync::LazyLock;

use regex::Regex;

/// Matches the version at the end of a stem after stripping `-bin` or `-all`.
/// Pattern mirrors the TypeScript: `\d+\.\d+(?:\.\d+)?(?:-\w+)*`
static VERSION_SUFFIX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\d+\.\d+(?:\.\d+)?(?:-\w+)*)$").unwrap());

/// The extracted Gradle wrapper dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GradleWrapperDep {
    /// The Gradle version string (e.g. `"8.4"`, `"8.4.0"`).
    pub version: String,
}

/// Result of [`extract_gradle_version`]: the full URL and the parsed version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GradleVersionExtract {
    pub url: String,
    pub version: String,
}

// Matches `distributionUrl = <url>` where url ends with `-{version}-{bin|all}.zip`.
static DISTRIBUTION_URL_FULL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?m)^(?:distributionUrl\s*=\s*)(?P<url>\S*-(?P<version>\d+\.\d+(?:\.\d+)?(?:-\w+)*)-(?:bin|all)\.zip)\s*$",
    )
    .unwrap()
});

/// Extract the Gradle distribution URL and version from properties file content.
///
/// Mirrors TypeScript `extractGradleVersion` in
/// `lib/modules/manager/gradle-wrapper/utils.ts`.
pub fn extract_gradle_version(content: &str) -> Option<GradleVersionExtract> {
    let cap = DISTRIBUTION_URL_FULL_RE.captures(content)?;
    Some(GradleVersionExtract {
        url: cap["url"].to_owned(),
        version: cap["version"].to_owned(),
    })
}

/// Compute the Java constraint for a given Gradle version string.
///
/// This is the pure (no I/O) part of `getJavaConstraint` from
/// `lib/modules/manager/gradle-wrapper/utils.ts`. When the gradlewFile is
/// empty (as in the test suite's `it.each` cases), file reads return null and
/// the result is determined solely by the Gradle major/minor version.
pub fn java_constraint_from_gradle_version(gradle_version: &str) -> &'static str {
    if gradle_version.is_empty() {
        return "^11.0.0";
    }
    let mut parts = gradle_version.split('.');
    let Some(major) = parts.next().and_then(|s| s.parse::<u64>().ok()) else {
        return "^11.0.0";
    };
    // TypeScript treats `0` as falsy for minor version comparison, so we map 0 → None.
    let minor: Option<u64> = parts.next().and_then(|s| {
        let n: u64 = s.parse().ok()?;
        if n > 0 { Some(n) } else { None }
    });

    if major > 9 || (major == 9 && minor.is_some_and(|m| m >= 1)) {
        return "^25.0.0";
    }
    if major > 8 || (major == 8 && minor.is_some_and(|m| m >= 5)) {
        return "^21.0.0";
    }
    if major > 7 || (major == 7 && minor.is_some_and(|m| m >= 3)) {
        return "^17.0.0";
    }
    if major == 7 {
        return "^16.0.0";
    }
    if major > 0 && major < 5 {
        return "^8.0.0";
    }
    "^11.0.0"
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
/// Matches the TypeScript regex: `\S*-{version}-{type}.zip` where type is `bin` or `all`.
/// Supports both standard (`gradle-8.4-bin.zip`) and custom (`custom-wrapper-1.3.7-bin.zip`)
/// as well as prerelease versions (`gradle-7.0-milestone-1-bin.zip`).
fn parse_distribution_url(url: &str) -> Option<String> {
    // Unescape `\:` → `:` (Gradle properties syntax).
    let url = url.replace("\\:", ":");

    let zip_name = url.split('/').next_back()?;

    // Must end in `-bin.zip` or `-all.zip`.
    let stem = zip_name
        .strip_suffix("-bin.zip")
        .or_else(|| zip_name.strip_suffix("-all.zip"))?;
    // Find the version at the end of the stem using the same pattern as TypeScript.
    // The version is `\d+\.\d+(?:\.\d+)?(?:-\w+)*` at the tail of the stem.
    let cap = VERSION_SUFFIX.captures(stem)?;
    let version = cap[1].to_owned();
    if version.contains('.') {
        Some(version)
    } else {
        None
    }
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

    // Ported: "extracts version for property file with distribution type \"bin\" in distributionUrl" — gradle-wrapper/extract.spec.ts line 33
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

    // Ported: "returns null for property file without distributionUrl" — gradle-wrapper/extract.spec.ts line 24
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

    // Ported: "extracts version for property file with custom distribution of type \"bin\" in distributionUrl" — gradle-wrapper/extract.spec.ts line 89
    #[test]
    fn custom_distribution_bin_extracted() {
        let content = r"distributionUrl=https\://domain.tld/repository/maven-releases/tld/domain/gradle-wrapper/custom-gradle-wrapper/1.3.7/custom-gradle-wrapper-1.3.7-bin.zip
";
        let dep = extract(content).unwrap();
        assert_eq!(dep.version, "1.3.7");
    }

    // Ported: "extracts version for property file with custom distribution of type \"all\" in distributionUrl" — gradle-wrapper/extract.spec.ts line 103
    #[test]
    fn custom_distribution_all_extracted() {
        let content = r"distributionUrl=https\://domain.tld/repository/maven-releases/tld/domain/gradle-wrapper/custom-gradle-wrapper/6.6.6/custom-gradle-wrapper-6.6.6-all.zip
";
        let dep = extract(content).unwrap();
        assert_eq!(dep.version, "6.6.6");
    }

    // --- extractGradleVersion tests ---

    // Ported: "returns null" — gradle-wrapper/util.spec.ts line 113
    #[test]
    fn extract_gradle_version_returns_none_without_distribution_url() {
        let properties = "distributionSha256Sum=038794feef1f4745c6347107b6726279d1c824f3fc634b60f86ace1e9fbd1768\nzipStoreBase=GRADLE_USER_HOME\n";
        assert!(extract_gradle_version(properties).is_none());
    }

    // Ported: "returns gradle version" — gradle-wrapper/util.spec.ts line 121
    #[test]
    fn extract_gradle_version_returns_url_and_version() {
        let properties = "distributionSha256Sum=038794feef1f4745c6347107b6726279d1c824f3fc634b60f86ace1e9fbd1768\ndistributionUrl=https\\://services.gradle.org/distributions/gradle-6.3-bin.zip\nzipStoreBase=GRADLE_USER_HOME\n";
        let r = extract_gradle_version(properties).unwrap();
        assert_eq!(
            r.url,
            "https\\://services.gradle.org/distributions/gradle-6.3-bin.zip"
        );
        assert_eq!(r.version, "6.3");
    }

    // --- getJavaConstraint pure tests ---

    // Ported: "$gradleVersion | $javaConstraint" — gradle-wrapper/util.spec.ts line 20
    #[test]
    fn java_constraint_from_gradle_version_cases() {
        let cases = [
            ("", "^11.0.0"),
            ("4", "^8.0.0"),
            ("4.9", "^8.0.0"),
            ("6.0", "^11.0.0"),
            ("7.0.1", "^16.0.0"),
            ("7.3.0", "^17.0.0"),
            ("8.0.1", "^17.0.0"),
            ("8.5.0", "^21.0.0"),
            ("9.0.1", "^21.0.0"),
            ("9.1.0", "^25.0.0"),
            ("10.0.1", "^25.0.0"),
        ];
        for (gradle, expected) in cases {
            assert_eq!(
                java_constraint_from_gradle_version(gradle),
                expected,
                "gradle version: {gradle}"
            );
        }
    }
}
