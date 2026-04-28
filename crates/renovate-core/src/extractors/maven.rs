//! Maven `pom.xml` dependency extractor.
//!
//! Parses Maven POM files and returns the set of dependencies with their
//! version specifiers, ready for Maven Central version lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/maven/extract.ts` — `extractAllPackageFiles`
//! - `lib/modules/manager/maven/dep-types.ts` — dep type taxonomy
//!
//! ## Supported sections
//!
//! | POM element | Dep type |
//! |---|---|
//! | `<dependencies>` (project root) | `Regular` |
//! | `<dependencyManagement><dependencies>` | `Management` |
//! | `<build><plugins><plugin>` | `Plugin` |
//! | `<build><extensions><extension>` | `Extension` |
//! | `<parent>` | `Parent` |
//! | `<profiles><profile><dependencies>` | `Profile` |
//!
//! ## Property references
//!
//! Versions of the form `${property}` are skipped with
//! `MavenSkipReason::PropertyRef`; the version cannot be updated without
//! resolving cross-file property values (deferred).

use std::io::BufReader;

use quick_xml::Reader;
use quick_xml::events::Event;
use thiserror::Error;

/// Which POM section the dep came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MavenDepType {
    /// `<dependencies>` directly under `<project>`
    Regular,
    /// `<dependencyManagement><dependencies>`
    Management,
    /// `<build><plugins><plugin>`
    Plugin,
    /// `<build><extensions><extension>`
    Extension,
    /// `<parent>`
    Parent,
    /// `<profiles><profile><dependencies>`
    Profile,
}

/// Why a Maven dependency is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MavenSkipReason {
    /// Version is a property reference: `${…}`.
    PropertyRef,
}

/// A single extracted Maven dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MavenExtractedDep {
    /// `groupId:artifactId`
    pub dep_name: String,
    /// Raw version string (empty = unversioned).
    pub current_value: String,
    /// Which POM section this dep came from.
    pub dep_type: MavenDepType,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<MavenSkipReason>,
}

/// Errors from parsing a `pom.xml`.
#[derive(Debug, Error)]
pub enum MavenExtractError {
    #[error("XML parse error: {0}")]
    Xml(#[from] quick_xml::Error),
}

// ── Public API ─────────────────────────────────────────────────────────────

/// Parse a `pom.xml` string and extract all Maven dependencies.
pub fn extract(content: &str) -> Result<Vec<MavenExtractedDep>, MavenExtractError> {
    let cursor = BufReader::new(content.as_bytes());
    let mut reader = Reader::from_reader(cursor);
    reader.config_mut().trim_text(true);

    let mut deps: Vec<MavenExtractedDep> = Vec::new();

    // Element name stack so we can track the current XML path.
    let mut stack: Vec<String> = Vec::new();

    // Currently accumulating a dep record.
    let mut current: Option<CurrentDep> = None;

    // Track nesting depth at which we started collecting, so we know when the
    // closing tag of the container is reached.
    let mut collect_start_depth: usize = 0;

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                stack.push(name.clone());

                match name.as_str() {
                    "dependency" if current.is_none() => {
                        if let Some(dep_type) = infer_dep_type(&stack) {
                            current = Some(CurrentDep::new(dep_type));
                            collect_start_depth = stack.len();
                        }
                    }
                    "plugin" if current.is_none() && is_plugin_context(&stack) => {
                        current = Some(CurrentDep::new(MavenDepType::Plugin));
                        collect_start_depth = stack.len();
                    }
                    "extension" if current.is_none() && is_extension_context(&stack) => {
                        current = Some(CurrentDep::new(MavenDepType::Extension));
                        collect_start_depth = stack.len();
                    }
                    "parent" if current.is_none() && stack.len() == 2 => {
                        // <project><parent>
                        current = Some(CurrentDep::new(MavenDepType::Parent));
                        collect_start_depth = stack.len();
                    }
                    _ => {}
                }
            }

            Event::End(e) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();

                // Check if we just closed the container we were collecting.
                if let Some(ref dep) = current
                    && stack.len() == collect_start_depth
                {
                    // Closing the container tag — emit if we have groupId + artifactId.
                    let container = match dep.dep_type {
                        MavenDepType::Plugin => "plugin",
                        MavenDepType::Extension => "extension",
                        MavenDepType::Parent => "parent",
                        _ => "dependency",
                    };
                    if name == container
                        && !dep.artifact_id.is_empty()
                        && let Some(d) = build_dep(dep)
                    {
                        deps.push(d);
                    }
                    current = None;
                }

                stack.pop();
            }

            Event::Text(e) => {
                if let Some(ref mut dep) = current {
                    let text = e.decode().map(|s| s.trim().to_owned()).unwrap_or_default();
                    if text.is_empty() {
                        continue;
                    }
                    // Only capture direct children of the container.
                    if stack.len() == collect_start_depth + 1 {
                        match stack.last().map(String::as_str) {
                            Some("groupId") => dep.group_id = text,
                            Some("artifactId") => dep.artifact_id = text,
                            Some("version") => dep.version = text,
                            _ => {}
                        }
                    }
                }
            }

            Event::Eof => break,

            _ => {}
        }
        buf.clear();
    }

    Ok(deps)
}

// ── Helpers ───────────────────────────────────────────────────────────────

struct CurrentDep {
    dep_type: MavenDepType,
    group_id: String,
    artifact_id: String,
    version: String,
}

impl CurrentDep {
    fn new(dep_type: MavenDepType) -> Self {
        Self {
            dep_type,
            group_id: String::new(),
            artifact_id: String::new(),
            version: String::new(),
        }
    }
}

fn build_dep(dep: &CurrentDep) -> Option<MavenExtractedDep> {
    let group_id = if dep.group_id.is_empty() && dep.dep_type == MavenDepType::Plugin {
        // Maven default for plugins without a groupId.
        "org.apache.maven.plugins".to_owned()
    } else {
        dep.group_id.clone()
    };

    if group_id.is_empty() || dep.artifact_id.is_empty() {
        return None;
    }

    let dep_name = format!("{}:{}", group_id, dep.artifact_id);
    let current_value = dep.version.clone();

    // Skip versions that are unresolved property references.
    let skip_reason = if current_value.contains("${") {
        Some(MavenSkipReason::PropertyRef)
    } else {
        None
    };

    Some(MavenExtractedDep {
        dep_name,
        current_value,
        dep_type: dep.dep_type,
        skip_reason,
    })
}

/// Infer the dep type from the current element stack when we encounter a
/// `<dependency>` open tag.
fn infer_dep_type(stack: &[String]) -> Option<MavenDepType> {
    let len = stack.len();
    if len < 3 {
        return None;
    }
    // Last element is "dependency"; look at parent path.
    let parent = stack[len - 2].as_str();
    let grandparent = stack[len - 3].as_str();

    match (grandparent, parent) {
        ("project", "dependencies") => Some(MavenDepType::Regular),
        ("dependencyManagement", "dependencies") => Some(MavenDepType::Management),
        ("profile", "dependencies") => Some(MavenDepType::Profile),
        // Nested inside plugin or reporting — skip (we only capture the plugin
        // itself, not its own sub-dependencies).
        _ => None,
    }
}

/// True when `<plugin>` appears in `<build><plugins>` or
/// `<profile><build><plugins>` or `<reporting><plugins>`.
fn is_plugin_context(stack: &[String]) -> bool {
    let len = stack.len();
    if len < 3 {
        return false;
    }
    let parent = stack[len - 2].as_str();
    let grandparent = stack[len - 3].as_str();
    // <plugins><plugin> where plugins is under build or reporting
    if parent == "plugins" && (grandparent == "build" || grandparent == "reporting") {
        return true;
    }
    false
}

/// True when `<extension>` appears in `<build><extensions>`.
fn is_extension_context(stack: &[String]) -> bool {
    let len = stack.len();
    if len < 3 {
        return false;
    }
    let parent = stack[len - 2].as_str();
    let grandparent = stack[len - 3].as_str();
    parent == "extensions" && grandparent == "build"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_ok(content: &str) -> Vec<MavenExtractedDep> {
        extract(content).expect("parse should succeed")
    }

    const SIMPLE_POM: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0">
  <modelVersion>4.0.0</modelVersion>
  <groupId>com.example</groupId>
  <artifactId>myapp</artifactId>
  <version>1.0.0</version>

  <dependencies>
    <dependency>
      <groupId>org.springframework</groupId>
      <artifactId>spring-core</artifactId>
      <version>5.3.28</version>
    </dependency>
    <dependency>
      <groupId>junit</groupId>
      <artifactId>junit</artifactId>
      <version>4.13.2</version>
      <scope>test</scope>
    </dependency>
  </dependencies>
</project>"#;

    #[test]
    fn extracts_regular_dependencies() {
        let deps = extract_ok(SIMPLE_POM);
        let regular: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == MavenDepType::Regular)
            .collect();
        assert_eq!(regular.len(), 2);
        assert!(regular.iter().any(
            |d| d.dep_name == "org.springframework:spring-core" && d.current_value == "5.3.28"
        ));
        assert!(
            regular
                .iter()
                .any(|d| d.dep_name == "junit:junit" && d.current_value == "4.13.2")
        );
    }

    #[test]
    fn extracts_parent() {
        let content = r#"<project>
  <parent>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-starter-parent</artifactId>
    <version>3.1.4</version>
  </parent>
  <modelVersion>4.0.0</modelVersion>
</project>"#;
        let deps = extract_ok(content);
        let parents: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == MavenDepType::Parent)
            .collect();
        assert_eq!(parents.len(), 1);
        assert_eq!(
            parents[0].dep_name,
            "org.springframework.boot:spring-boot-starter-parent"
        );
        assert_eq!(parents[0].current_value, "3.1.4");
    }

    #[test]
    fn extracts_dependency_management() {
        let content = r#"<project>
  <dependencyManagement>
    <dependencies>
      <dependency>
        <groupId>org.example</groupId>
        <artifactId>bom-artifact</artifactId>
        <version>2.0.0</version>
        <type>pom</type>
        <scope>import</scope>
      </dependency>
    </dependencies>
  </dependencyManagement>
</project>"#;
        let deps = extract_ok(content);
        let mgmt: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == MavenDepType::Management)
            .collect();
        assert_eq!(mgmt.len(), 1);
        assert_eq!(mgmt[0].dep_name, "org.example:bom-artifact");
        assert_eq!(mgmt[0].current_value, "2.0.0");
    }

    #[test]
    fn extracts_build_plugins() {
        let content = r#"<project>
  <build>
    <plugins>
      <plugin>
        <groupId>org.apache.maven.plugins</groupId>
        <artifactId>maven-compiler-plugin</artifactId>
        <version>3.11.0</version>
      </plugin>
    </plugins>
  </build>
</project>"#;
        let deps = extract_ok(content);
        let plugins: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == MavenDepType::Plugin)
            .collect();
        assert_eq!(plugins.len(), 1);
        assert_eq!(
            plugins[0].dep_name,
            "org.apache.maven.plugins:maven-compiler-plugin"
        );
        assert_eq!(plugins[0].current_value, "3.11.0");
    }

    #[test]
    fn plugin_default_group_id() {
        let content = r#"<project>
  <build>
    <plugins>
      <plugin>
        <artifactId>maven-release-plugin</artifactId>
        <version>2.4.2</version>
      </plugin>
    </plugins>
  </build>
</project>"#;
        let deps = extract_ok(content);
        let plugins: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == MavenDepType::Plugin)
            .collect();
        assert_eq!(plugins.len(), 1);
        assert_eq!(
            plugins[0].dep_name,
            "org.apache.maven.plugins:maven-release-plugin"
        );
    }

    #[test]
    fn extracts_build_extensions() {
        let content = r#"<project>
  <build>
    <extensions>
      <extension>
        <groupId>org.example</groupId>
        <artifactId>extension-artefact</artifactId>
        <version>1.0</version>
      </extension>
    </extensions>
  </build>
</project>"#;
        let deps = extract_ok(content);
        let exts: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == MavenDepType::Extension)
            .collect();
        assert_eq!(exts.len(), 1);
        assert_eq!(exts[0].dep_name, "org.example:extension-artefact");
        assert_eq!(exts[0].current_value, "1.0");
    }

    #[test]
    fn property_ref_version_is_skipped() {
        let content = r#"<project>
  <dependencies>
    <dependency>
      <groupId>org.example</groupId>
      <artifactId>prop-dep</artifactId>
      <version>${spring.version}</version>
    </dependency>
  </dependencies>
</project>"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(MavenSkipReason::PropertyRef));
    }

    #[test]
    fn plugin_nested_dependencies_not_captured_as_regular() {
        // Dependencies nested inside a <plugin> block should not appear as
        // Regular deps — only the plugin itself should be extracted.
        let content = r#"<project>
  <build>
    <plugins>
      <plugin>
        <groupId>org.apache.maven.plugins</groupId>
        <artifactId>maven-release-plugin</artifactId>
        <version>2.4.2</version>
        <dependencies>
          <dependency>
            <groupId>org.apache.maven.scm</groupId>
            <artifactId>maven-scm-provider-gitexe</artifactId>
            <version>1.8.1</version>
          </dependency>
        </dependencies>
      </plugin>
    </plugins>
  </build>
</project>"#;
        let deps = extract_ok(content);
        // Only the plugin itself should be present.
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_type, MavenDepType::Plugin);
        assert_eq!(
            deps[0].dep_name,
            "org.apache.maven.plugins:maven-release-plugin"
        );
    }

    #[test]
    fn profile_dependencies_extracted() {
        let content = r#"<project>
  <profiles>
    <profile>
      <id>dev</id>
      <dependencies>
        <dependency>
          <groupId>org.example</groupId>
          <artifactId>profile-artifact</artifactId>
          <version>2.17</version>
        </dependency>
      </dependencies>
    </profile>
  </profiles>
</project>"#;
        let deps = extract_ok(content);
        let profile: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == MavenDepType::Profile)
            .collect();
        assert_eq!(profile.len(), 1);
        assert_eq!(profile[0].dep_name, "org.example:profile-artifact");
    }

    #[test]
    fn empty_pom_returns_empty() {
        let content = r#"<project>
  <modelVersion>4.0.0</modelVersion>
</project>"#;
        let deps = extract_ok(content);
        assert!(deps.is_empty());
    }

    #[test]
    fn multiline_element_values_trimmed() {
        let content = r#"<project>
  <dependencyManagement>
    <dependencies>
      <dependency>
        <groupId>
          org.example
        </groupId>
        <artifactId>
          bar
        </artifactId>
        <version>
          1.0.0
        </version>
      </dependency>
    </dependencies>
  </dependencyManagement>
</project>"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "org.example:bar");
        assert_eq!(deps[0].current_value, "1.0.0");
    }
}
