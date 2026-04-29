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
//! Versions of the form `${property}` are resolved against the POM's own
//! `<properties>` section during extraction.  A version that fully resolves
//! (no remaining `${…}` after substitution) is treated as actionable.
//! Versions that reference properties not defined in this file are skipped
//! with `MavenSkipReason::PropertyRef`; cross-file resolution is deferred.
//!
//! Renovate reference: `applyProps` / `applyPropsInternal` in
//! `lib/modules/manager/maven/extract.ts`.

use std::collections::HashMap;
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

impl MavenDepType {
    /// Return the Renovate-canonical `depType` string for this Maven dep type.
    ///
    /// For `Regular` deps the actual scope (compile/test/provided/runtime/system)
    /// is more accurate but requires the scope string.  Use `renovate_dep_type_str`
    /// for the full string including scope.
    ///
    /// Renovate reference: `lib/modules/manager/maven/extract.ts`
    pub fn as_renovate_str(&self) -> &'static str {
        match self {
            // Regular deps are "compile" by Maven default; scope overrides this.
            MavenDepType::Regular => "compile",
            // dependencyManagement section.
            MavenDepType::Management => "dependency-management",
            // Plugins and extensions map to Renovate's "build" dep type.
            MavenDepType::Plugin | MavenDepType::Extension => "build",
            MavenDepType::Parent => "parent",
            MavenDepType::Profile => "compile",
        }
    }
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
    /// Maven scope value for `<dependency>` elements (e.g. "compile", "test").
    /// `None` for plugins, extensions, and parent POM entries.
    pub scope: Option<String>,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<MavenSkipReason>,
}

impl MavenExtractedDep {
    /// Return the Renovate-canonical `depType` string, incorporating the
    /// Maven scope when available.
    ///
    /// Mirrors Renovate's behavior where `depType` is set to the scope value
    /// (e.g. `"compile"`, `"test"`, `"provided"`) for `<dependency>` elements,
    /// and `"build"` for plugins/extensions, `"parent"` for parent POM.
    pub fn renovate_dep_type(&self) -> &str {
        match self.dep_type {
            MavenDepType::Regular | MavenDepType::Profile => {
                self.scope.as_deref().unwrap_or("compile")
            }
            _ => self.dep_type.as_renovate_str(),
        }
    }
}

/// Errors from parsing a `pom.xml`.
#[derive(Debug, Error)]
pub enum MavenExtractError {
    #[error("XML parse error: {0}")]
    Xml(#[from] quick_xml::Error),
}

// ── Public API ─────────────────────────────────────────────────────────────

/// Parse a `pom.xml` string and extract all Maven dependencies.
///
/// Property references in version strings (e.g. `${spring.version}`) are
/// resolved against the POM's own `<properties>` section.  Unresolvable
/// references remain marked with [`MavenSkipReason::PropertyRef`].
pub fn extract(content: &str) -> Result<Vec<MavenExtractedDep>, MavenExtractError> {
    let (mut deps, properties) = parse_pom(content)?;

    // Resolve ${property} references using the POM's own <properties> section.
    // groupId and artifactId can also be property refs (e.g. ${quuxGroup}).
    for dep in &mut deps {
        // Resolve dep_name (${groupId}:${artifactId}).
        if dep.dep_name.contains("${") {
            dep.dep_name = apply_props(&dep.dep_name, &properties);
        }
        // Resolve version.
        if dep.skip_reason == Some(MavenSkipReason::PropertyRef) {
            let resolved = apply_props(&dep.current_value, &properties);
            if !resolved.contains("${") {
                dep.current_value = resolved;
                dep.skip_reason = None;
            }
            // Otherwise leave as PropertyRef — cross-file resolution is deferred.
        }
    }

    Ok(deps)
}

/// SAX parse a POM and return (deps, properties).
fn parse_pom(
    content: &str,
) -> Result<(Vec<MavenExtractedDep>, HashMap<String, String>), MavenExtractError> {
    let cursor = BufReader::new(content.as_bytes());
    let mut reader = Reader::from_reader(cursor);
    reader.config_mut().trim_text(true);

    let mut deps: Vec<MavenExtractedDep> = Vec::new();
    let mut properties: HashMap<String, String> = HashMap::new();

    // Element name stack — tracks current XML path.
    let mut stack: Vec<String> = Vec::new();

    // Currently accumulating a dep record.
    let mut current: Option<CurrentDep> = None;
    let mut collect_start_depth: usize = 0;

    // Currently accumulating a <properties> child value.
    // `Some(key)` when we are inside <project><properties><key>.
    let mut prop_key: Option<String> = None;

    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                stack.push(name.clone());

                // Detect entry into a <properties> child element.
                // Stack must be exactly [project, properties, <key>].
                if stack.len() == 3
                    && stack[1] == "properties"
                    && current.is_none()
                    && prop_key.is_none()
                {
                    prop_key = Some(name.clone());
                }

                if current.is_none() && prop_key.is_none() {
                    match name.as_str() {
                        "dependency" => {
                            if let Some(dep_type) = infer_dep_type(&stack) {
                                current = Some(CurrentDep::new(dep_type));
                                collect_start_depth = stack.len();
                            }
                        }
                        "plugin" if is_plugin_context(&stack) => {
                            current = Some(CurrentDep::new(MavenDepType::Plugin));
                            collect_start_depth = stack.len();
                        }
                        "extension" if is_extension_context(&stack) => {
                            current = Some(CurrentDep::new(MavenDepType::Extension));
                            collect_start_depth = stack.len();
                        }
                        "parent" if stack.len() == 2 => {
                            current = Some(CurrentDep::new(MavenDepType::Parent));
                            collect_start_depth = stack.len();
                        }
                        _ => {}
                    }
                }
            }

            Event::End(e) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();

                // Close of a <properties> child element.
                if prop_key.is_some() && stack.len() == 3 && stack[1] == "properties" {
                    prop_key = None;
                }

                // Check if we just closed the dep container.
                if let Some(ref dep) = current
                    && stack.len() == collect_start_depth
                {
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
                let text = e.decode().map(|s| s.trim().to_owned()).unwrap_or_default();
                if text.is_empty() {
                    continue;
                }

                // Capture a <properties><key>value</key> entry.
                if let Some(ref key) = prop_key {
                    properties.insert(key.clone(), text.clone());
                }

                // Capture dep fields.
                if let Some(ref mut dep) = current
                    && stack.len() == collect_start_depth + 1
                {
                    match stack.last().map(String::as_str) {
                        Some("groupId") => dep.group_id = text,
                        Some("artifactId") => dep.artifact_id = text,
                        Some("version") => dep.version = text,
                        Some("scope") => dep.scope = Some(text),
                        _ => {}
                    }
                }
            }

            Event::Eof => break,

            _ => {}
        }
        buf.clear();
    }

    Ok((deps, properties))
}

/// Substitute `${key}` references in `value` using `properties`.
/// Applies up to 3 passes to handle one level of indirection.
fn apply_props(value: &str, properties: &HashMap<String, String>) -> String {
    let mut result = value.to_owned();
    for _ in 0..3 {
        let next = substitute_props(&result, properties);
        if next == result {
            break;
        }
        result = next;
    }
    result
}

fn substitute_props(value: &str, properties: &HashMap<String, String>) -> String {
    let mut out = String::with_capacity(value.len());
    let mut rest = value;
    while let Some(start) = rest.find("${") {
        out.push_str(&rest[..start]);
        let after_open = &rest[start + 2..];
        if let Some(close) = after_open.find('}') {
            let key = &after_open[..close];
            if let Some(val) = properties.get(key) {
                out.push_str(val);
            } else {
                out.push_str("${");
                out.push_str(key);
                out.push('}');
            }
            rest = &after_open[close + 1..];
        } else {
            // Unclosed ${ — emit as-is and stop scanning.
            out.push_str("${");
            rest = after_open;
        }
    }
    out.push_str(rest);
    out
}

// ── Helpers ───────────────────────────────────────────────────────────────

struct CurrentDep {
    dep_type: MavenDepType,
    group_id: String,
    artifact_id: String,
    version: String,
    /// Maven `<scope>` value for `<dependency>` elements (e.g. "compile", "test").
    scope: Option<String>,
}

impl CurrentDep {
    fn new(dep_type: MavenDepType) -> Self {
        Self {
            dep_type,
            group_id: String::new(),
            artifact_id: String::new(),
            version: String::new(),
            scope: None,
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
        scope: dep.scope.clone(),
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
    fn property_ref_skipped_when_not_defined() {
        // No <properties> section — ${spring.version} cannot be resolved.
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
    fn property_resolved_from_properties_section() {
        let content = r#"<project>
  <properties>
    <spring.version>5.3.28</spring.version>
    <junit.version>4.13.2</junit.version>
  </properties>
  <dependencies>
    <dependency>
      <groupId>org.springframework</groupId>
      <artifactId>spring-core</artifactId>
      <version>${spring.version}</version>
    </dependency>
    <dependency>
      <groupId>junit</groupId>
      <artifactId>junit</artifactId>
      <version>${junit.version}</version>
    </dependency>
    <dependency>
      <groupId>org.example</groupId>
      <artifactId>unknown-dep</artifactId>
      <version>${unknown.version}</version>
    </dependency>
  </dependencies>
</project>"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 3);

        let spring = deps
            .iter()
            .find(|d| d.dep_name == "org.springframework:spring-core")
            .unwrap();
        assert_eq!(spring.current_value, "5.3.28");
        assert!(
            spring.skip_reason.is_none(),
            "resolved property should have no skip reason"
        );

        let junit = deps.iter().find(|d| d.dep_name == "junit:junit").unwrap();
        assert_eq!(junit.current_value, "4.13.2");
        assert!(junit.skip_reason.is_none());

        let unknown = deps
            .iter()
            .find(|d| d.dep_name == "org.example:unknown-dep")
            .unwrap();
        assert_eq!(unknown.skip_reason, Some(MavenSkipReason::PropertyRef));
    }

    #[test]
    fn recursive_property_resolution() {
        // ${alias} = ${actual}, ${actual} = 1.2.3 — two-level indirection.
        let content = r#"<project>
  <properties>
    <alias.version>${actual.version}</alias.version>
    <actual.version>1.2.3</actual.version>
  </properties>
  <dependencies>
    <dependency>
      <groupId>org.example</groupId>
      <artifactId>alias-dep</artifactId>
      <version>${alias.version}</version>
    </dependency>
  </dependencies>
</project>"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "1.2.3");
        assert!(deps[0].skip_reason.is_none());
    }

    #[test]
    fn pdm_style_pom_with_properties() {
        // Based on the simple.pom.xml fixture from the Renovate test suite.
        let content = r#"<project xmlns="http://maven.apache.org/POM/4.0.0">
  <properties>
    <quuxGroup>org.example</quuxGroup>
    <quuxId>quux</quuxId>
    <quuxVersion>1.2.3.4</quuxVersion>
  </properties>
  <dependencies>
    <dependency>
      <groupId>${quuxGroup}</groupId>
      <artifactId>${quuxId}</artifactId>
      <version>${quuxVersion}</version>
    </dependency>
    <dependency>
      <groupId>org.example</groupId>
      <artifactId>quuz</artifactId>
      <version>1.2.3</version>
    </dependency>
  </dependencies>
</project>"#;
        let deps = extract_ok(content);
        // quux: groupId and artifactId are properties — dep_name will use resolved values.
        let quux = deps.iter().find(|d| d.dep_name == "org.example:quux");
        assert!(
            quux.is_some(),
            "quux should be extracted with resolved groupId/artifactId"
        );
        let quux = quux.unwrap();
        assert_eq!(quux.current_value, "1.2.3.4");
        assert!(quux.skip_reason.is_none());

        let quuz = deps
            .iter()
            .find(|d| d.dep_name == "org.example:quuz")
            .unwrap();
        assert_eq!(quuz.current_value, "1.2.3");
    }

    #[test]
    fn substitute_props_handles_unknown_key() {
        let mut props = HashMap::new();
        props.insert("known".to_owned(), "1.0".to_owned());
        let result = substitute_props("${known}-${unknown}", &props);
        assert_eq!(result, "1.0-${unknown}");
    }

    #[test]
    fn substitute_props_unclosed_brace() {
        let props = HashMap::new();
        let result = substitute_props("${unclosed", &props);
        assert_eq!(result, "${unclosed");
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

    #[test]
    fn dep_type_as_renovate_str() {
        // Renovate uses scope-based dep types for Maven (compile, test, etc.)
        // The as_renovate_str() returns the default when no scope is available.
        assert_eq!(MavenDepType::Regular.as_renovate_str(), "compile");
        assert_eq!(
            MavenDepType::Management.as_renovate_str(),
            "dependency-management"
        );
        // Plugins and extensions both map to "build" in Renovate's maven extractor.
        assert_eq!(MavenDepType::Plugin.as_renovate_str(), "build");
        assert_eq!(MavenDepType::Extension.as_renovate_str(), "build");
        assert_eq!(MavenDepType::Parent.as_renovate_str(), "parent");
        assert_eq!(MavenDepType::Profile.as_renovate_str(), "compile");
    }

    #[test]
    fn renovate_dep_type_uses_scope() {
        let dep = MavenExtractedDep {
            dep_name: "org.example:lib".to_owned(),
            current_value: "1.0.0".to_owned(),
            dep_type: MavenDepType::Regular,
            scope: Some("test".to_owned()),
            skip_reason: None,
        };
        assert_eq!(dep.renovate_dep_type(), "test");
    }

    #[test]
    fn renovate_dep_type_defaults_to_compile_without_scope() {
        let dep = MavenExtractedDep {
            dep_name: "org.example:lib".to_owned(),
            current_value: "1.0.0".to_owned(),
            dep_type: MavenDepType::Regular,
            scope: None,
            skip_reason: None,
        };
        assert_eq!(dep.renovate_dep_type(), "compile");
    }
}
