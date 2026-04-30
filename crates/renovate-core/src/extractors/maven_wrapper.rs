//! Maven Wrapper `maven-wrapper.properties` extractor.
//!
//! Reads the `distributionUrl` and `wrapperUrl`/`wrapperVersion` properties
//! and extracts the Maven version and optionally the Maven Wrapper version.
//!
//! Also handles `mvnw` / `mvnw.cmd` shell-script files, where the wrapper
//! version appears in the startup comment line.
//!
//! Renovate reference:
//! - `lib/modules/manager/maven-wrapper/extract.ts`
//! - Patterns: `/(^|/)\.mvn/wrapper/maven-wrapper\.properties$/`
//!
//! ## Supported forms
//!
//! ```properties
//! distributionUrl=https://repo.maven.apache.org/maven2/org/apache/maven/apache-maven/3.9.4/apache-maven-3.9.4-bin.zip
//! wrapperUrl=https://repo.maven.apache.org/maven2/org/apache/maven/wrapper/maven-wrapper/3.2.0/maven-wrapper-3.2.0.jar
//! ```
//!
//! ```sh
//! # Apache Maven Wrapper startup batch script, version 3.3.0
//! ```

const MVNW_MARKER: &str = "Apache Maven Wrapper startup batch script, version ";

/// A dependency extracted from a Maven Wrapper properties file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MavenWrapperDep {
    /// Display name (`"maven"` or `"maven-wrapper"`).
    pub dep_name: String,
    /// Maven Central package name (`groupId:artifactId`).
    pub package_name: String,
    /// Current version string.
    pub version: String,
    /// String to replace when updating (full URL or bare version).
    pub replace_string: String,
}

/// Parse `maven-wrapper.properties` (or an `mvnw` script) and extract deps.
pub fn extract(content: &str) -> Vec<MavenWrapperDep> {
    // Detect mvnw/mvnw.cmd: look for the startup comment line.
    for raw in content.lines() {
        let trimmed = raw.trim();
        // Unix: `# Apache Maven Wrapper startup batch script, version X`
        // Windows: `@REM Apache Maven Wrapper startup batch script, version X`
        let version_part = if let Some(rest) = trimmed.strip_prefix("# ") {
            rest.strip_prefix(MVNW_MARKER)
        } else if let Some(rest) = trimmed.strip_prefix("@REM ") {
            rest.strip_prefix(MVNW_MARKER)
        } else {
            None
        };
        if let Some(version) = version_part {
            let version = version.trim().to_owned();
            if !version.is_empty() && is_version_like(&version) {
                return vec![MavenWrapperDep {
                    dep_name: "maven-wrapper".into(),
                    package_name: "org.apache.maven.wrapper:maven-wrapper".into(),
                    replace_string: version.clone(),
                    version,
                }];
            }
        }
    }

    // Otherwise parse as a `.properties` file.
    let mut maven_version: Option<String> = None;
    let mut maven_url: Option<String> = None;
    let mut wrapper_version: Option<String> = None;
    let mut wrapper_replace: Option<String> = None;

    for line in content.lines() {
        let trimmed = line.trim();

        if let Some(val) = trimmed.strip_prefix("distributionUrl=") {
            let url = val.trim();
            if let Some(v) = extract_version_from_url(url) {
                maven_version = Some(v);
                maven_url = Some(url.to_owned());
            }
        } else if let Some(val) = trimmed.strip_prefix("wrapperUrl=") {
            let url = val.trim();
            if let Some(v) = extract_version_from_url(url) {
                // wrapperUrl takes precedence over wrapperVersion
                wrapper_version = Some(v);
                wrapper_replace = Some(url.to_owned());
            }
        } else if let Some(val) = trimmed.strip_prefix("wrapperVersion=") {
            let v = val.trim();
            // Only set if wrapperUrl hasn't already set wrapper_version
            if wrapper_version.is_none() && !v.is_empty() && is_version_like(v) {
                wrapper_version = Some(v.to_owned());
                wrapper_replace = Some(v.to_owned());
            }
        }
    }

    let mut out = Vec::new();
    if let (Some(version), Some(url)) = (maven_version, maven_url) {
        out.push(MavenWrapperDep {
            dep_name: "maven".into(),
            package_name: "org.apache.maven:apache-maven".into(),
            replace_string: url,
            version,
        });
    }
    if let (Some(version), Some(replace)) = (wrapper_version, wrapper_replace) {
        out.push(MavenWrapperDep {
            dep_name: "maven-wrapper".into(),
            package_name: "org.apache.maven.wrapper:maven-wrapper".into(),
            replace_string: replace,
            version,
        });
    }
    out
}

/// Extract a version string from a Maven distribution or wrapper URL.
fn extract_version_from_url(url: &str) -> Option<String> {
    let parts: Vec<&str> = url.split('/').collect();
    let end = parts.len().saturating_sub(1);
    parts[1..end]
        .iter()
        .find(|&&s| is_version_like(s))
        .map(|s| (*s).to_owned())
}

/// Returns true if `s` looks like a version string (starts with digit, contains `.`).
fn is_version_like(s: &str) -> bool {
    s.starts_with(|c: char| c.is_ascii_digit()) && s.contains('.')
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "extracts version for property file with distribution type "bin" in distributionUrl" — maven-wrapper/extract.spec.ts line 14
    #[test]
    fn extracts_wrapper_and_maven_properties() {
        let content = "distributionUrl=https://internal.artifactory.acme.org/artifactory/maven-bol/org/apache/maven/apache-maven/3.8.4/apache-maven-3.8.4-bin.zip\nwrapperUrl=https://internal.artifactory.acme.org/artifactory/maven-bol/org/apache/maven/wrapper/maven-wrapper/3.1.0/maven-wrapper-3.1.0.jar";
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        let maven = deps.iter().find(|d| d.dep_name == "maven").unwrap();
        assert_eq!(maven.version, "3.8.4");
        assert_eq!(maven.package_name, "org.apache.maven:apache-maven");
        assert_eq!(
            maven.replace_string,
            "https://internal.artifactory.acme.org/artifactory/maven-bol/org/apache/maven/apache-maven/3.8.4/apache-maven-3.8.4-bin.zip"
        );
        let wrapper = deps.iter().find(|d| d.dep_name == "maven-wrapper").unwrap();
        assert_eq!(wrapper.version, "3.1.0");
        assert_eq!(
            wrapper.replace_string,
            "https://internal.artifactory.acme.org/artifactory/maven-bol/org/apache/maven/wrapper/maven-wrapper/3.1.0/maven-wrapper-3.1.0.jar"
        );
    }

    // Ported: "extracts version for property file with only a wrapper url" — maven-wrapper/extract.spec.ts line 37
    #[test]
    fn extracts_only_wrapper_url() {
        let content = "wrapperUrl=https://repo.maven.apache.org/maven2/io/takari/maven-wrapper/0.5.6/maven-wrapper-0.5.6.jar";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "maven-wrapper");
        assert_eq!(deps[0].version, "0.5.6");
        assert_eq!(
            deps[0].package_name,
            "org.apache.maven.wrapper:maven-wrapper"
        );
        assert_eq!(
            deps[0].replace_string,
            "https://repo.maven.apache.org/maven2/io/takari/maven-wrapper/0.5.6/maven-wrapper-0.5.6.jar"
        );
    }

    // Ported: "extracts version for property file with only a wrapper version" — maven-wrapper/extract.spec.ts line 51
    #[test]
    fn extracts_only_wrapper_version_key() {
        let content = "wrapperVersion=3.3.1";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "maven-wrapper");
        assert_eq!(deps[0].version, "3.3.1");
        assert_eq!(deps[0].replace_string, "3.3.1");
    }

    // Ported: "extracts wrapper information from wrapperUrl in precedence to wrapperVersion" — maven-wrapper/extract.spec.ts line 64
    #[test]
    fn wrapper_url_takes_precedence_over_wrapper_version() {
        let content = "wrapperVersion=3.1.0\nwrapperUrl=https://internal.artifactory.acme.org/artifactory/maven-bol/org/apache/maven/wrapper/maven-wrapper/3.1.2/maven-wrapper-3.1.2.jar";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "maven-wrapper");
        assert_eq!(deps[0].version, "3.1.2");
        assert_eq!(
            deps[0].replace_string,
            "https://internal.artifactory.acme.org/artifactory/maven-bol/org/apache/maven/wrapper/maven-wrapper/3.1.2/maven-wrapper-3.1.2.jar"
        );
    }

    // Ported: "extracts maven warapper version from mvnw file" — maven-wrapper/extract.spec.ts line 80
    #[test]
    fn extracts_version_from_mvnw_unix() {
        let content = "# Apache Maven Wrapper startup batch script, version 3.3.0";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "maven-wrapper");
        assert_eq!(deps[0].version, "3.3.0");
        assert_eq!(deps[0].replace_string, "3.3.0");
    }

    // Ported: "extracts maven warapper version from mvnw file - Windows" — maven-wrapper/extract.spec.ts line 93
    #[test]
    fn extracts_version_from_mvnw_windows() {
        let content = "@REM Apache Maven Wrapper startup batch script, version 3.3.0";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "maven-wrapper");
        assert_eq!(deps[0].version, "3.3.0");
        assert_eq!(deps[0].replace_string, "3.3.0");
    }

    // Ported: "returns null for invalid wrapper version string in from mvnw file" — maven-wrapper/extract.spec.ts line 106
    #[test]
    fn invalid_mvnw_prefix_returns_empty() {
        let content = "invalid Apache Maven Wrapper startup batch script, version 3.3.0";
        assert!(extract(content).is_empty());
    }

    // Ported: "extracts version for property file with only a maven url" — maven-wrapper/extract.spec.ts line 111
    #[test]
    fn extracts_maven_version() {
        let content = "distributionUrl=https://repo.maven.apache.org/maven2/org/apache/maven/apache-maven/3.5.4/apache-maven-3.5.4-bin.zip\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "maven");
        assert_eq!(deps[0].package_name, "org.apache.maven:apache-maven");
        assert_eq!(deps[0].version, "3.5.4");
        assert_eq!(
            deps[0].replace_string,
            "https://repo.maven.apache.org/maven2/org/apache/maven/apache-maven/3.5.4/apache-maven-3.5.4-bin.zip"
        );
    }

    // Ported: "should return null when there is no string matching the maven properties regex" — maven-wrapper/extract.spec.ts line 125
    #[test]
    fn no_matching_key_returns_empty() {
        assert!(extract("nowrapper").is_empty());
    }

    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    #[test]
    fn extracts_maven_and_wrapper_versions() {
        let content = r#"
distributionUrl=https://repo.maven.apache.org/maven2/org/apache/maven/apache-maven/3.9.4/apache-maven-3.9.4-bin.zip
wrapperUrl=https://repo.maven.apache.org/maven2/org/apache/maven/wrapper/maven-wrapper/3.2.0/maven-wrapper-3.2.0.jar
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(
            deps.iter()
                .any(|d| d.dep_name == "maven" && d.version == "3.9.4")
        );
        assert!(
            deps.iter()
                .any(|d| d.dep_name == "maven-wrapper" && d.version == "3.2.0")
        );
    }

    #[test]
    fn no_version_key_skipped() {
        let content = "distributionUrl=https://example.com/no-version-here\n";
        assert!(extract(content).is_empty());
    }
}
