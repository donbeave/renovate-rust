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

use crate::extractors::dockerfile;

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
    /// Renovate datasource used for lookup.
    pub datasource: &'static str,
    /// `groupId:artifactId`
    pub dep_name: String,
    /// Datasource package name when it differs from or supplements `dep_name`.
    pub package_name: Option<String>,
    /// Raw version string (empty = unversioned).
    pub current_value: String,
    /// Current digest for container-style dependencies.
    pub current_digest: Option<String>,
    /// Which POM section this dep came from.
    pub dep_type: MavenDepType,
    /// Maven scope value for `<dependency>` elements (e.g. "compile", "test").
    /// `None` for plugins, extensions, and parent POM entries.
    pub scope: Option<String>,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<MavenSkipReason>,
    /// Registry URLs extracted from Maven settings files or related metadata.
    pub registry_urls: Vec<String>,
    /// Full string to replace for container-style dependencies.
    pub replace_string: Option<String>,
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

/// Extract Maven registry URLs from a `settings.xml` file.
pub fn extract_registries(content: &str) -> Vec<String> {
    let cursor = BufReader::new(content.as_bytes());
    let mut reader = Reader::from_reader(cursor);
    reader.config_mut().trim_text(true);

    let mut stack: Vec<String> = Vec::new();
    let mut urls = Vec::new();
    let mut saw_settings = false;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                if stack.is_empty() && name != "settings" {
                    return Vec::new();
                }
                if stack.is_empty() {
                    saw_settings = true;
                }
                stack.push(name);
            }
            Ok(Event::End(_)) => {
                stack.pop();
            }
            Ok(Event::Text(e)) => {
                if is_settings_registry_url_path(&stack) {
                    let text = e.decode().map(|s| s.trim().to_owned()).unwrap_or_default();
                    if !text.is_empty() && !urls.iter().any(|url| url == &text) {
                        urls.push(text);
                    }
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(_) => return Vec::new(),
        }
        buf.clear();
    }

    if saw_settings { urls } else { Vec::new() }
}

/// Parse a Maven `.mvn/extensions.xml` file.
pub fn extract_extensions(content: &str) -> Option<Vec<MavenExtractedDep>> {
    let cursor = BufReader::new(content.as_bytes());
    let mut reader = Reader::from_reader(cursor);
    reader.config_mut().trim_text(true);

    let mut stack: Vec<String> = Vec::new();
    let mut current: Option<CurrentDep> = None;
    let mut deps = Vec::new();
    let mut saw_extensions = false;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                if stack.is_empty() && name != "extensions" {
                    return None;
                }
                if stack.is_empty() {
                    saw_extensions = true;
                }
                if stack.len() == 1 && stack[0] == "extensions" && name == "extension" {
                    current = Some(CurrentDep::new(MavenDepType::Extension));
                }
                stack.push(name);
            }
            Ok(Event::End(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                if name == "extension"
                    && let Some(dep) = current.take()
                    && let Some(extracted) = build_dep(&dep)
                {
                    deps.push(extracted);
                }
                stack.pop();
            }
            Ok(Event::Text(e)) => {
                if let Some(ref mut dep) = current
                    && stack.len() == 3
                {
                    let text = e.decode().map(|s| s.trim().to_owned()).unwrap_or_default();
                    if text.is_empty() {
                        continue;
                    }
                    match stack.last().map(String::as_str) {
                        Some("groupId") => dep.group_id = text,
                        Some("artifactId") => dep.artifact_id = text,
                        Some("version") => dep.version = text,
                        _ => {}
                    }
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(_) => return None,
        }
        buf.clear();
    }

    (saw_extensions && !deps.is_empty()).then_some(deps)
}

/// Extract Maven package files from already-read path/content pairs.
pub fn extract_all_package_files(files: &[(&str, &str)]) -> Vec<Vec<MavenExtractedDep>> {
    let mut package_files = Vec::new();
    let registry_urls = extract_package_registry_urls(files);
    for (path, content) in files {
        if path.ends_with(".mvn/extensions.xml") || *path == ".mvn/extensions.xml" {
            if let Some(mut deps) = extract_extensions(content)
                && !deps.is_empty()
            {
                apply_registry_urls(&mut deps, &registry_urls);
                package_files.push(deps);
            }
            continue;
        }

        if path.ends_with(".xml")
            && !is_settings_xml_path(path)
            && let Ok(mut deps) = extract(content)
            && !deps.is_empty()
        {
            apply_registry_urls(&mut deps, &registry_urls);
            package_files.push(deps);
        }
    }
    package_files
}

fn extract_package_registry_urls(files: &[(&str, &str)]) -> Vec<String> {
    let mut urls = Vec::new();
    for (path, content) in files {
        if is_settings_xml_path(path) {
            for url in extract_registries(content) {
                if !urls.iter().any(|existing| existing == &url) {
                    urls.push(url);
                }
            }
        }
    }
    if !urls.is_empty()
        && !urls
            .iter()
            .any(|url| url == "https://repo.maven.apache.org/maven2")
    {
        urls.push("https://repo.maven.apache.org/maven2".to_owned());
    }
    urls
}

fn is_settings_xml_path(path: &str) -> bool {
    path.ends_with("settings.xml")
}

fn apply_registry_urls(deps: &mut [MavenExtractedDep], registry_urls: &[String]) {
    if registry_urls.is_empty() {
        return;
    }
    for dep in deps {
        if dep.datasource == "maven" {
            dep.registry_urls = registry_urls.to_vec();
        }
    }
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
                        Some("groupId") => dep.group_id = text.clone(),
                        Some("artifactId") => dep.artifact_id = text.clone(),
                        Some("version") => dep.version = text.clone(),
                        Some("scope") => dep.scope = Some(text.clone()),
                        _ => {}
                    }
                }

                if let Some(ref dep) = current
                    && dep.dep_type == MavenDepType::Plugin
                    && is_spring_boot_plugin(dep)
                    && let Some(image_dep) = spring_boot_image_dep(&stack, &text)
                {
                    deps.push(image_dep);
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
        datasource: "maven",
        dep_name,
        package_name: None,
        current_value,
        current_digest: None,
        dep_type: dep.dep_type,
        scope: dep.scope.clone(),
        skip_reason,
        registry_urls: Vec::new(),
        replace_string: None,
    })
}

fn is_spring_boot_plugin(dep: &CurrentDep) -> bool {
    dep.group_id == "org.springframework.boot" && dep.artifact_id == "spring-boot-maven-plugin"
}

fn spring_boot_image_dep(stack: &[String], text: &str) -> Option<MavenExtractedDep> {
    let tag = stack.last().map(String::as_str)?;
    match tag {
        "builder" | "runImage" if stack_ends_with(stack, &["configuration", "image", tag]) => {
            docker_image_dep(text)
        }
        "buildpack"
            if stack_ends_with(
                stack,
                &["configuration", "image", "buildpacks", "buildpack"],
            ) =>
        {
            buildpack_dep(text)
        }
        _ => None,
    }
}

fn buildpack_dep(text: &str) -> Option<MavenExtractedDep> {
    let reference = text.trim();
    if let Some(stripped) = reference.strip_prefix("docker://") {
        return docker_image_dep(stripped);
    }
    if reference.contains("://") || reference.starts_with("urn:") {
        return None;
    }
    if let Some((package_name, version)) = reference.split_once('@')
        && package_name.contains('/')
        && !package_name.is_empty()
        && !version.is_empty()
    {
        return Some(container_style_dep(
            "buildpacks-registry",
            package_name,
            Some(package_name),
            version,
            None,
            None,
        ));
    }
    docker_image_dep(reference)
}

fn docker_image_dep(reference: &str) -> Option<MavenExtractedDep> {
    let dep = dockerfile::classify_image_ref(reference.trim());
    if dep.skip_reason.is_some() || (dep.tag.is_none() && dep.digest.is_none()) {
        return None;
    }
    let replace_string = image_ref(&dep.image, dep.tag.as_deref(), dep.digest.as_deref());
    Some(container_style_dep(
        "docker",
        &dep.image,
        Some(&dep.image),
        dep.tag.as_deref().unwrap_or_default(),
        dep.digest,
        Some(replace_string),
    ))
}

fn container_style_dep(
    datasource: &'static str,
    dep_name: &str,
    package_name: Option<&str>,
    current_value: &str,
    current_digest: Option<String>,
    replace_string: Option<String>,
) -> MavenExtractedDep {
    MavenExtractedDep {
        datasource,
        dep_name: dep_name.to_owned(),
        package_name: package_name.map(str::to_owned),
        current_value: current_value.to_owned(),
        current_digest,
        dep_type: MavenDepType::Plugin,
        scope: None,
        skip_reason: None,
        registry_urls: Vec::new(),
        replace_string,
    }
}

fn image_ref(image: &str, tag: Option<&str>, digest: Option<&str>) -> String {
    let mut out = image.to_owned();
    if let Some(tag) = tag
        && !tag.is_empty()
    {
        out.push(':');
        out.push_str(tag);
    }
    if let Some(digest) = digest
        && !digest.is_empty()
    {
        out.push('@');
        out.push_str(digest);
    }
    out
}

fn stack_ends_with(stack: &[String], suffix: &[&str]) -> bool {
    stack.len() >= suffix.len()
        && stack[stack.len() - suffix.len()..]
            .iter()
            .map(String::as_str)
            .eq(suffix.iter().copied())
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

fn is_settings_registry_url_path(stack: &[String]) -> bool {
    matches!(
        stack,
        [settings, mirrors, mirror, url]
            if settings == "settings" && mirrors == "mirrors" && mirror == "mirror" && url == "url"
    ) || matches!(
        stack,
        [settings, profiles, profile, repositories, repository, url]
            if settings == "settings"
                && profiles == "profiles"
                && profile == "profile"
                && repositories == "repositories"
                && repository == "repository"
                && url == "url"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_ok(content: &str) -> Vec<MavenExtractedDep> {
        extract(content).expect("parse should succeed")
    }

    fn cnb_deps(deps: &[MavenExtractedDep]) -> Vec<&MavenExtractedDep> {
        deps.iter()
            .filter(|dep| dep.datasource == "docker" || dep.datasource == "buildpacks-registry")
            .collect()
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

    // Ported: "extract dependencies from any XML position" — maven/extract.spec.ts line 29
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

    // Ported: "extract dependencies from any XML position" — maven/extract.spec.ts line 29
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

    // Ported: "extract dependencies from any XML position" — maven/extract.spec.ts line 29
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

    // Ported: "extract dependencies from any XML position" — maven/extract.spec.ts line 29
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

    // Ported: "extract dependencies from any XML position" — maven/extract.spec.ts line 29
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

    // Ported: "returns null for invalid XML" — maven/extract.spec.ts line 471
    #[test]
    fn settings_registries_invalid_xml_returns_empty() {
        assert!(extract_registries("").is_empty());
        assert!(extract_registries("invalid xml content").is_empty());
        assert!(extract_registries("<foobar></foobar>").is_empty());
        assert!(extract_registries("<settings></settings>").is_empty());
    }

    // Ported: "extract registries from a simple mirror settings file" — maven/extract.spec.ts line 478
    #[test]
    fn settings_registries_extracts_simple_mirror() {
        let content = r#"<settings xmlns="http://maven.apache.org/SETTINGS/1.0.0">
  <mirrors>
    <mirror>
      <id>my-maven-repo</id>
      <url>https://artifactory.company.com/artifactory/my-maven-repo</url>
      <mirrorOf>*</mirrorOf>
    </mirror>
  </mirrors>
</settings>"#;
        assert_eq!(
            extract_registries(content),
            vec!["https://artifactory.company.com/artifactory/my-maven-repo"]
        );
    }

    // Ported: "extract registries from a simple profile settings file" — maven/extract.spec.ts line 485
    #[test]
    fn settings_registries_extracts_simple_profile_repository() {
        let content = r#"<settings xmlns="http://maven.apache.org/SETTINGS/1.0.0">
  <profiles>
    <profile>
      <id>adobe-public</id>
      <repositories>
        <repository>
          <id>adobe-public-releases</id>
          <url>https://repo.adobe.com/nexus/content/groups/public</url>
        </repository>
      </repositories>
    </profile>
  </profiles>
</settings>"#;
        assert_eq!(
            extract_registries(content),
            vec!["https://repo.adobe.com/nexus/content/groups/public"]
        );
    }

    // Ported: "extract registries from a complex profile settings file" — maven/extract.spec.ts line 492
    #[test]
    fn settings_registries_extracts_complex_settings() {
        let content = r#"<settings xmlns="http://maven.apache.org/SETTINGS/1.0.0">
  <mirrors>
    <mirror>
      <id>my-maven-repo</id>
      <url>https://artifactory.company.com/artifactory/my-maven-repo</url>
      <mirrorOf>*</mirrorOf>
    </mirror>
    <mirror>
      <id>my-maven-repo-v2</id>
      <url>https://repo.adobe.com/nexus/content/groups/public</url>
      <mirrorOf>custom-repo</mirrorOf>
    </mirror>
  </mirrors>
  <profiles>
    <profile>
      <id>adobe-public</id>
      <repositories>
        <repository>
          <id>adobe-public-releases</id>
          <url>https://repo.adobe.com/nexus/content/groups/public</url>
        </repository>
        <repository>
          <id>adobe-public-releases-v2</id>
          <url>https://repo.adobe.com/v2/nexus/content/groups/public</url>
        </repository>
      </repositories>
    </profile>
    <profile>
      <id>adobe-public-v2</id>
      <repositories>
        <repository>
          <id>adobe-public-releases-v3</id>
          <url>https://repo.adobe.com/v3/nexus/content/groups/public</url>
        </repository>
        <repository>
          <id>adobe-public-releases-v4</id>
          <url>https://repo.adobe.com/v4/nexus/content/groups/public</url>
        </repository>
      </repositories>
    </profile>
  </profiles>
</settings>"#;
        assert_eq!(
            extract_registries(content),
            vec![
                "https://artifactory.company.com/artifactory/my-maven-repo",
                "https://repo.adobe.com/nexus/content/groups/public",
                "https://repo.adobe.com/v2/nexus/content/groups/public",
                "https://repo.adobe.com/v3/nexus/content/groups/public",
                "https://repo.adobe.com/v4/nexus/content/groups/public",
            ]
        );
    }

    // Ported: "extract registries from a settings file that uses a newer schema" — maven/extract.spec.ts line 503
    #[test]
    fn settings_registries_extracts_newer_schema() {
        let content = r#"<settings xmlns="http://maven.apache.org/SETTINGS/1.2.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
  xsi:schemaLocation="http://maven.apache.org/SETTINGS/1.2.0 http://maven.apache.org/xsd/settings-1.2.0.xsd">
  <mirrors>
    <mirror>
      <id>Test-Internal-repository</id>
      <name>Proxy Repository Manager</name>
      <url>https://proxy-repo.com/artifactory/apache-maven</url>
      <mirrorOf>central</mirrorOf>
    </mirror>
  </mirrors>
  <profiles/>
  <activeProfiles/>
</settings>"#;
        assert_eq!(
            extract_registries(content),
            vec!["https://proxy-repo.com/artifactory/apache-maven"]
        );
    }

    // Ported: "returns null for invalid xml files" — maven/extract.spec.ts line 527
    #[test]
    fn extensions_invalid_xml_returns_none() {
        assert!(extract_extensions("").is_none());
        assert!(extract_extensions("invalid xml content").is_none());
        assert!(extract_extensions("<foobar></foobar>").is_none());
        assert!(extract_extensions("<extensions></extensions>").is_none());
        assert!(
            extract_extensions(
                r#"<extensions xmlns="http://maven.apache.org/EXTENSIONS/1.0.0"></extensions>"#
            )
            .is_none()
        );
    }

    // Ported: "should return empty if package has no content" — maven/extract.spec.ts line 548
    #[test]
    fn extract_all_package_files_empty_content_returns_empty() {
        assert!(extract_all_package_files(&[("random.pom.xml", "")]).is_empty());
    }

    // Ported: "should return empty for packages with invalid content" — maven/extract.spec.ts line 554
    #[test]
    fn extract_all_package_files_invalid_content_returns_empty() {
        assert!(extract_all_package_files(&[("random.pom.xml", "invalid content")]).is_empty());
    }

    // Ported: "should return packages with urls from a settings file" — maven/extract.spec.ts line 560
    #[test]
    fn extract_all_package_files_applies_settings_registry_urls() {
        let settings = r#"<settings>
  <mirrors>
    <mirror>
      <url>https://artifactory.company.com/artifactory/my-maven-repo</url>
    </mirror>
    <mirror>
      <url>https://maven.atlassian.com/content/repositories/atlassian-public/</url>
    </mirror>
  </mirrors>
</settings>"#;
        let pom = r#"<project>
  <dependencies>
    <dependency>
      <groupId>org.example</groupId>
      <artifactId>demo</artifactId>
      <version>1.2.3</version>
    </dependency>
  </dependencies>
</project>"#;

        let packages = extract_all_package_files(&[
            ("mirror.settings.xml", settings),
            ("simple.pom.xml", pom),
        ]);
        let expected = vec![
            "https://artifactory.company.com/artifactory/my-maven-repo".to_owned(),
            "https://maven.atlassian.com/content/repositories/atlassian-public/".to_owned(),
            "https://repo.maven.apache.org/maven2".to_owned(),
        ];
        assert_eq!(packages.len(), 1);
        assert_eq!(packages[0][0].registry_urls, expected);
    }

    // Ported: "should include registryUrls in the correct order" — maven/extract.spec.ts line 791
    #[test]
    fn extract_all_package_files_preserves_settings_registry_url_order() {
        let pom = r#"<project>
  <dependencies>
    <dependency>
      <groupId>org.example</groupId>
      <artifactId>demo</artifactId>
      <version>1.2.3</version>
    </dependency>
  </dependencies>
</project>"#;
        let settings = r#"<settings>
  <profiles>
    <profile>
      <repositories>
        <repository>
          <url>https://repo.adobe.com/nexus/content/groups/public</url>
        </repository>
        <repository>
          <url>https://maven.atlassian.com/content/repositories/atlassian-public/</url>
        </repository>
      </repositories>
    </profile>
  </profiles>
</settings>"#;

        let packages = extract_all_package_files(&[
            ("simple.pom.xml", pom),
            ("profile.settings.xml", settings),
        ]);
        let expected = vec![
            "https://repo.adobe.com/nexus/content/groups/public".to_owned(),
            "https://maven.atlassian.com/content/repositories/atlassian-public/".to_owned(),
            "https://repo.maven.apache.org/maven2".to_owned(),
        ];
        assert_eq!(packages.len(), 1);
        assert_eq!(packages[0][0].registry_urls, expected);
    }

    // Ported: "should extract from .mvn/extensions.xml file" — maven/extract.spec.ts line 888
    #[test]
    fn extract_all_package_files_extracts_extensions_xml() {
        let content = r#"<extensions xmlns="http://maven.apache.org/EXTENSIONS/1.0.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <extension>
    <groupId>io.jenkins.tools.incrementals</groupId>
    <artifactId>git-changelist-maven-extension</artifactId>
    <version>1.6</version>
  </extension>
</extensions>"#;
        let packages = extract_all_package_files(&[(".mvn/extensions.xml", content)]);
        assert_eq!(packages.len(), 1);
        assert_eq!(packages[0].len(), 1);
        assert_eq!(
            packages[0][0].dep_name,
            "io.jenkins.tools.incrementals:git-changelist-maven-extension"
        );
        assert_eq!(packages[0][0].current_value, "1.6");
        assert_eq!(packages[0][0].dep_type, MavenDepType::Extension);
    }

    // Ported: "should return empty array if extensions file is invalid or empty" — maven/extract.spec.ts line 917
    #[test]
    fn extract_all_package_files_invalid_extensions_return_empty() {
        assert!(
            extract_all_package_files(&[
                (".mvn/extensions.xml", ""),
                ("grp/.mvn/extensions.xml", "invalid xml content"),
            ])
            .is_empty()
        );
    }

    // Ported: "extract dependencies from any XML position" — maven/extract.spec.ts line 29
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

    // Ported: "extract dependencies from any XML position" — maven/extract.spec.ts line 29
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

    // Ported: "extract dependencies from any XML position" — maven/extract.spec.ts line 29
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

    // Ported: "should apply props recursively" — maven/extract.spec.ts line 418
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

    // Ported: "extract dependencies from any XML position" — maven/extract.spec.ts line 29
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

    // Ported: "extract dependencies from any XML position" — maven/extract.spec.ts line 29
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

    // Ported: "extract dependencies from any XML position" — maven/extract.spec.ts line 29
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

    // Ported: "returns null for invalid XML" — maven/extract.spec.ts line 22
    #[test]
    fn empty_pom_returns_empty() {
        let content = r#"<project>
  <modelVersion>4.0.0</modelVersion>
</project>"#;
        let deps = extract_ok(content);
        assert!(deps.is_empty());
    }

    // Ported: "extract dependencies from any XML position" — maven/extract.spec.ts line 29
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

    // Ported: "extract dependencies with windows line endings" — maven/extract.spec.ts line 237
    #[test]
    fn windows_line_endings_are_tolerated() {
        let content = "<project>\r\n  <dependencies>\r\n    <dependency>\r\n      <groupId>org.example</groupId>\r\n      <artifactId>demo</artifactId>\r\n      <version>1.2.3</version>\r\n    </dependency>\r\n  </dependencies>\r\n</project>\r\n";
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "org.example:demo");
        assert_eq!(deps[0].current_value, "1.2.3");
    }

    // Ported: "extracts builder and buildpack images from spring-boot plugin" — maven/extract.spec.ts line 279
    #[test]
    fn spring_boot_plugin_extracts_builder_run_image_and_buildpacks() {
        let content = r#"<project>
  <parent>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-starter-parent</artifactId>
    <version>3.2.2</version>
  </parent>
  <build>
    <plugins>
      <plugin>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-maven-plugin</artifactId>
        <configuration>
          <image>
            <builder>paketobuildpacks/builder-jammy-base:0.4.316</builder>
            <runImage>paketobuildpacks/run-noble-full:0.0.28</runImage>
            <buildpacks>
              <buildpack>paketo-buildpacks/nodejs@6.1.1</buildpack>
              <buildpack>urn:cnb:builder:paketo-buildpacks/php@2.13.1</buildpack>
              <buildpack>gcr.io/paketo-buildpacks/nodejs:1.8.0</buildpack>
              <buildpack>docker://docker.io/paketobuildpacks/python:2.22.1@sha256:2c27cd0b4482a4aa5aeb38104f6d934511cd87c1af34a10d1d6cdf2d9d16f138</buildpack>
              <buildpack>docker://docker.io/paketobuildpacks/ruby@sha256:080f4cfa5c8fe43837b2b83f69ae16e320ea67c051173e4934a015590b2ca67a</buildpack>
              <buildpack>paketobuildpacks/java:12.1.0</buildpack>
              <buildpack>paketobuildpacks/go</buildpack>
            </buildpacks>
          </image>
        </configuration>
      </plugin>
    </plugins>
  </build>
</project>"#;
        let deps = extract_ok(content);
        assert!(deps.iter().any(|dep| {
            dep.datasource == "maven"
                && dep.dep_name == "org.springframework.boot:spring-boot-starter-parent"
                && dep.current_value == "3.2.2"
        }));

        let cnb_deps = cnb_deps(&deps);
        assert_eq!(
            cnb_deps
                .iter()
                .map(|dep| (
                    dep.datasource,
                    dep.dep_name.as_str(),
                    dep.current_value.as_str(),
                    dep.current_digest.as_deref()
                ))
                .collect::<Vec<_>>(),
            vec![
                (
                    "docker",
                    "paketobuildpacks/builder-jammy-base",
                    "0.4.316",
                    None
                ),
                ("docker", "paketobuildpacks/run-noble-full", "0.0.28", None),
                (
                    "buildpacks-registry",
                    "paketo-buildpacks/nodejs",
                    "6.1.1",
                    None
                ),
                ("docker", "gcr.io/paketo-buildpacks/nodejs", "1.8.0", None),
                (
                    "docker",
                    "docker.io/paketobuildpacks/python",
                    "2.22.1",
                    Some("sha256:2c27cd0b4482a4aa5aeb38104f6d934511cd87c1af34a10d1d6cdf2d9d16f138")
                ),
                (
                    "docker",
                    "docker.io/paketobuildpacks/ruby",
                    "",
                    Some("sha256:080f4cfa5c8fe43837b2b83f69ae16e320ea67c051173e4934a015590b2ca67a")
                ),
                ("docker", "paketobuildpacks/java", "12.1.0", None),
            ]
        );
        assert_eq!(
            cnb_deps[4].replace_string.as_deref(),
            Some(
                "docker.io/paketobuildpacks/python:2.22.1@sha256:2c27cd0b4482a4aa5aeb38104f6d934511cd87c1af34a10d1d6cdf2d9d16f138"
            )
        );
    }

    // Ported: "extracts only builder if defaults are used in spring-boot plugin" — maven/extract.spec.ts line 370
    #[test]
    fn spring_boot_plugin_extracts_only_configured_builder() {
        let content = r#"<project>
  <build>
    <plugins>
      <plugin>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-maven-plugin</artifactId>
        <configuration>
          <image>
            <builder>paketobuildpacks/builder-jammy-base:0.4.316</builder>
          </image>
        </configuration>
      </plugin>
    </plugins>
  </build>
</project>"#;
        let deps = extract_ok(content);
        let cnb_deps = cnb_deps(&deps);
        assert_eq!(cnb_deps.len(), 1);
        assert_eq!(cnb_deps[0].datasource, "docker");
        assert_eq!(cnb_deps[0].dep_name, "paketobuildpacks/builder-jammy-base");
        assert_eq!(cnb_deps[0].current_value, "0.4.316");
    }

    // Ported: "returns no buildpack dependencies when image tag is missing in spring boot plugin configuration" — maven/extract.spec.ts line 398
    #[test]
    fn spring_boot_plugin_skips_missing_image_tag() {
        let content = r#"<project>
  <build>
    <plugins>
      <plugin>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-maven-plugin</artifactId>
        <configuration>
          <image-tag-missing></image-tag-missing>
        </configuration>
      </plugin>
    </plugins>
  </build>
</project>"#;
        let deps = extract_ok(content);
        assert!(cnb_deps(&deps).is_empty());
    }

    // Ported: "returns no buildpack dependencies when dependencies are invalid in spring boot plugin" — maven/extract.spec.ts line 407
    #[test]
    fn spring_boot_plugin_skips_invalid_buildpack_dependencies() {
        let content = r#"<project>
  <build>
    <plugins>
      <plugin>
        <groupId>org.springframework.boot</groupId>
        <artifactId>spring-boot-maven-plugin</artifactId>
        <configuration>
          <image>
            <builder>invalid-builder</builder>
            <runImage>invalid-run</runImage>
            <buildpacks>
              <buildpack>invalid-image</buildpack>
              <buildpack>urn:cnb:builder:buildpacks:invalid@2.13.1</buildpack>
              <buildpack>invalid://identifier/type:1.8.0</buildpack>
            </buildpacks>
          </image>
        </configuration>
      </plugin>
    </plugins>
  </build>
</project>"#;
        let deps = extract_ok(content);
        assert!(cnb_deps(&deps).is_empty());
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
            datasource: "maven",
            dep_name: "org.example:lib".to_owned(),
            package_name: None,
            current_value: "1.0.0".to_owned(),
            current_digest: None,
            dep_type: MavenDepType::Regular,
            scope: Some("test".to_owned()),
            skip_reason: None,
            registry_urls: Vec::new(),
            replace_string: None,
        };
        assert_eq!(dep.renovate_dep_type(), "test");
    }

    #[test]
    fn renovate_dep_type_defaults_to_compile_without_scope() {
        let dep = MavenExtractedDep {
            datasource: "maven",
            dep_name: "org.example:lib".to_owned(),
            package_name: None,
            current_value: "1.0.0".to_owned(),
            current_digest: None,
            dep_type: MavenDepType::Regular,
            scope: None,
            skip_reason: None,
            registry_urls: Vec::new(),
            replace_string: None,
        };
        assert_eq!(dep.renovate_dep_type(), "compile");
    }

    // Ported: "returns null for invalid XML" — maven/extract.spec.ts line 22
    #[test]
    fn invalid_xml_returns_empty() {
        // Empty string, invalid XML, and documents with wrong root element all
        // return no dependencies (mirrors extractPackage returning null).
        assert!(extract_ok("").is_empty());
        assert!(extract_ok("invalid xml content").is_empty());
        assert!(extract_ok("<foobar></foobar>").is_empty());
        assert!(extract_ok("<project></project>").is_empty());
    }

    // Ported: "tries minimum manifests" — maven/extract.spec.ts line 249
    #[test]
    fn minimum_manifest_returns_empty_deps() {
        // A minimal valid POM with modelVersion and version but no dependencies.
        let content = r#"<project>
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.example</groupId>
  <artifactId>minimum</artifactId>
  <version>1</version>
</project>"#;
        let deps = extract_ok(content);
        assert!(deps.is_empty());
    }

    // Ported: "tries minimum snapshot manifests" — maven/extract.spec.ts line 264
    #[test]
    fn minimum_snapshot_manifest_returns_empty_deps() {
        let content = r#"<project>
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.example</groupId>
  <artifactId>minimum-snapshot</artifactId>
  <version>0.0.1-SNAPSHOT</version>
</project>"#;
        let deps = extract_ok(content);
        assert!(deps.is_empty());
    }

    // Ported: "should apply props multiple times" — maven/extract.spec.ts line 432
    #[test]
    fn props_applied_with_multiple_usages() {
        // ${lucene.version} used in both groupId-like suffix and version.
        let content = r#"<project>
  <properties>
    <lucene.version>1.2.3</lucene.version>
  </properties>
  <dependencies>
    <dependency>
      <groupId>org.apache.lucene</groupId>
      <artifactId>lucene-core-${lucene.version}.${lucene.version}</artifactId>
      <version>${lucene.version}</version>
    </dependency>
  </dependencies>
</project>"#;
        let deps = extract_ok(content);
        // The version reference should be fully resolved.
        assert_eq!(deps[0].current_value, "1.2.3");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "should detect props infinitely recursing props" — maven/extract.spec.ts line 448
    #[test]
    fn infinite_recursing_props_left_as_placeholder() {
        // foo -> bar -> foo: circular reference — apply_props caps at 3 passes.
        let content = r#"<project>
  <properties>
    <foo>${bar}</foo>
    <bar>${foo}</bar>
  </properties>
  <dependencies>
    <dependency>
      <groupId>org.apache.lucene</groupId>
      <artifactId>lucene-core</artifactId>
      <version>${foo}</version>
    </dependency>
  </dependencies>
</project>"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 1);
        // After max iterations the property remains unresolved — skip_reason is set.
        assert_eq!(deps[0].skip_reason, Some(MavenSkipReason::PropertyRef));
    }
}
