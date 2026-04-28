//! Maven Wrapper `maven-wrapper.properties` extractor.
//!
//! Reads the `distributionUrl` and `wrapperUrl`/`wrapperVersion` properties
//! and extracts the Maven version and optionally the Maven Wrapper version.
//!
//! Renovate reference:
//! - `lib/modules/manager/maven-wrapper/extract.ts`
//! - Patterns: `/(^|/)\.mvn/wrapper/maven-wrapper\.properties$/`
//!
//! ## Supported form
//!
//! ```properties
//! distributionUrl=https://repo.maven.apache.org/maven2/org/apache/maven/apache-maven/3.9.4/apache-maven-3.9.4-bin.zip
//! wrapperUrl=https://repo.maven.apache.org/maven2/org/apache/maven/wrapper/maven-wrapper/3.2.0/maven-wrapper-3.2.0.jar
//! ```

/// A dependency extracted from a Maven Wrapper properties file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MavenWrapperDep {
    /// Display name (`"maven"` or `"maven-wrapper"`).
    pub dep_name: String,
    /// Maven Central package name (`groupId:artifactId`).
    pub package_name: String,
    /// Current version string.
    pub version: String,
}

/// Parse `maven-wrapper.properties` and extract Maven + optional wrapper deps.
pub fn extract(content: &str) -> Vec<MavenWrapperDep> {
    let mut maven_version: Option<String> = None;
    let mut wrapper_version: Option<String> = None;

    for line in content.lines() {
        let trimmed = line.trim();

        if let Some(val) = trimmed.strip_prefix("distributionUrl=") {
            if let Some(v) = extract_version_from_url(val.trim()) {
                maven_version = Some(v);
            }
        } else if let Some(val) = trimmed.strip_prefix("wrapperUrl=") {
            if let Some(v) = extract_version_from_url(val.trim()) {
                wrapper_version = Some(v);
            }
        } else if let Some(val) = trimmed.strip_prefix("wrapperVersion=") {
            let v = val.trim();
            if !v.is_empty() && is_version_like(v) {
                wrapper_version = Some(v.to_owned());
            }
        }
    }

    let mut out = Vec::new();
    if let Some(version) = maven_version {
        out.push(MavenWrapperDep {
            dep_name: "maven".into(),
            package_name: "org.apache.maven:apache-maven".into(),
            version,
        });
    }
    if let Some(version) = wrapper_version {
        out.push(MavenWrapperDep {
            dep_name: "maven-wrapper".into(),
            package_name: "org.apache.maven.wrapper:maven-wrapper".into(),
            version,
        });
    }
    out
}

/// Extract a version string from a Maven distribution or wrapper URL.
///
/// URL examples:
/// - `https://repo.maven.apache.org/.../apache-maven/3.9.4/apache-maven-3.9.4-bin.zip`
/// - `https://repo.maven.apache.org/.../maven-wrapper/3.2.0/maven-wrapper-3.2.0.jar`
fn extract_version_from_url(url: &str) -> Option<String> {
    // Maven URL pattern: `.../artifactId/{version}/artifactId-{version}-type.ext`
    // The version is a standalone path segment before the filename.
    // Skip first segment (empty before leading `/` or scheme) and last (filename).
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

    #[test]
    fn extracts_maven_version() {
        let content = "distributionUrl=https://repo.maven.apache.org/maven2/org/apache/maven/apache-maven/3.9.4/apache-maven-3.9.4-bin.zip\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "maven");
        assert_eq!(deps[0].package_name, "org.apache.maven:apache-maven");
        assert_eq!(deps[0].version, "3.9.4");
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
    fn wrapper_version_key_parsed() {
        let content = "wrapperVersion=3.2.0\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "maven-wrapper");
        assert_eq!(deps[0].version, "3.2.0");
    }

    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    #[test]
    fn no_version_key_skipped() {
        let content = "distributionUrl=https://example.com/no-version-here\n";
        let deps = extract(content);
        assert!(deps.is_empty());
    }
}
