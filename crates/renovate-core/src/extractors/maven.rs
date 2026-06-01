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
    /// `<parent>` pointing at the repository root `pom.xml`
    ParentRoot,
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
            MavenDepType::ParentRoot => "parent-root",
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

/// A single Maven `<properties>` entry with its file position.
///
/// Mirrors upstream `MavenProp`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MavenProp {
    /// Resolved property value.
    pub val: String,
    /// Byte offset of the property text content, relative to the first `<`
    /// in the file (matching `maven_update_dependency` expectations).
    pub file_replace_position: usize,
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
    /// Maven property name that supplied the version, when the version is shared.
    pub shared_variable_name: Option<String>,
    /// Byte offset of the `<version>` text content in the XML file (after leading whitespace).
    pub file_replace_position: Option<usize>,
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

/// Extracted Maven package-file data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MavenPackageFile {
    /// Path of the source package file.
    pub package_file: String,
    /// Dependencies extracted from this package file.
    pub deps: Vec<MavenExtractedDep>,
    /// Top-level `<project><version>` value, when present.
    pub package_file_version: Option<String>,
    /// Properties declared in this POM's `<properties>` section.
    pub maven_props: HashMap<String, MavenProp>,
    /// Path to the parent POM file, if this POM declares a `<parent>`.
    pub parent: Option<String>,
}

/// Errors from parsing a `pom.xml`.
#[derive(Debug, Error)]
pub enum MavenExtractError {
    #[error("XML parse error: {0}")]
    Xml(#[from] quick_xml::Error),
    #[error("Not a valid Maven POM: {0}")]
    InvalidPom(String),
}

// ── Public API ─────────────────────────────────────────────────────────────

/// Parse a `pom.xml` string and extract all Maven dependencies.
///
/// Property references in version strings (e.g. `${spring.version}`) are
/// resolved against the POM's own `<properties>` section.  Unresolvable
/// references remain marked with [`MavenSkipReason::PropertyRef`].
pub fn extract(content: &str) -> Result<Vec<MavenExtractedDep>, MavenExtractError> {
    let (mut deps, prop_map, _parent) = parse_pom(content, "")?;

    // Flatten MavenProp values into plain strings for local resolution.
    let properties: HashMap<String, String> = prop_map
        .into_iter()
        .map(|(k, v)| (k, v.val))
        .collect();

    // Adjust file_replace_position to be relative to the first '<' in content,
    // matching what maven_update_dependency expects.
    let offset = content.find('<').unwrap_or(0);

    // Resolve ${property} references using the POM's own <properties> section.
    // groupId and artifactId can also be property refs (e.g. ${quuxGroup}).
    for dep in &mut deps {
        if let Some(pos) = dep.file_replace_position {
            dep.file_replace_position = Some(pos - offset);
        }
        // Resolve dep_name (${groupId}:${artifactId}).
        if dep.dep_name.contains("${") {
            dep.dep_name = apply_props(&dep.dep_name, &properties);
        }
        // Resolve version.
        if dep.skip_reason == Some(MavenSkipReason::PropertyRef) {
            dep.shared_variable_name = exact_property_ref(&dep.current_value)
                .filter(|key| properties.contains_key(*key))
                .map(str::to_owned);
            let resolved = apply_props(&dep.current_value, &properties);
            if !resolved.contains("${") {
                dep.current_value = resolved;
                // Go template expressions ({{...}}) are unresolvable version placeholders.
                if dep.current_value.contains("{{") {
                    dep.skip_reason = Some(MavenSkipReason::PropertyRef);
                } else {
                    dep.skip_reason = None;
                }
            }
            // Otherwise leave as PropertyRef — cross-file resolution is deferred.
        }
    }

    Ok(deps)
}

/// Extract a single Maven package file, returning raw deps, properties, and
/// parent path without cross-file resolution.
/// Mirrors upstream `extractPackageFile`.
pub fn extract_package_file(content: &str, package_file: &str) -> Result<MavenPackageFile, MavenExtractError> {
    let (mut deps, properties, parent) = parse_pom(content, package_file)?;

    // Adjust file_replace_position to be relative to the first '<' in content.
    let offset = content.find('<').unwrap_or(0);
    for dep in &mut deps {
        if let Some(pos) = dep.file_replace_position {
            dep.file_replace_position = Some(pos - offset);
        }
    }

    Ok(MavenPackageFile {
        package_file: package_file.to_owned(),
        deps,
        package_file_version: project_version(content),
        maven_props: properties,
        parent,
    })
}

/// Resolve cross-file properties and registry URLs for Maven multi-module
/// projects.  Mirrors upstream `resolveParents`.
pub fn resolve_parents(package_files: &mut [MavenPackageFile]) {
    if package_files.is_empty() {
        return;
    }

    // Map package file path -> index.
    let path_to_index: std::collections::HashMap<String, usize> = package_files
        .iter()
        .enumerate()
        .map(|(i, pkg)| (pkg.package_file.clone(), i))
        .collect();

    // For each package, build merged properties by walking up the parent chain.
    // Child properties override parent properties.
    let mut merged_props: Vec<HashMap<String, MavenProp>> = Vec::with_capacity(package_files.len());
    let mut merged_registry_urls: Vec<Vec<String>> = Vec::with_capacity(package_files.len());

    for pkg in package_files.iter() {
        let mut props = HashMap::new();
        let mut registry_urls = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut current = Some(pkg);

        while let Some(current_pkg) = current {
            // Insert parent props first (they get overridden by child props).
            for (key, prop) in &current_pkg.maven_props {
                props.entry(key.clone()).or_insert_with(|| prop.clone());
            }

            // Collect registry URLs from parent deps.
            for dep in &current_pkg.deps {
                for url in &dep.registry_urls {
                    if !registry_urls.contains(url) {
                        registry_urls.push(url.clone());
                    }
                }
            }

            let parent_path = current_pkg.parent.as_deref();
            if let Some(parent_path) = parent_path {
                if visited.insert(parent_path) {
                    current = path_to_index
                        .get(parent_path)
                        .and_then(|&idx| package_files.get(idx));
                } else {
                    break; // Cycle detected.
                }
            } else {
                break;
            }
        }

        merged_props.push(props);
        merged_registry_urls.push(registry_urls);
    }

    // Apply merged properties to each dep and merge registry URLs.
    for (i, pkg) in package_files.iter_mut().enumerate() {
        let props = &merged_props[i];
        let parent_registry_urls = &merged_registry_urls[i];

        for dep in &mut pkg.deps {
            // Merge registry URLs (prepend parent URLs, keep unique).
            let mut new_urls = parent_registry_urls.clone();
            for url in &dep.registry_urls {
                if !new_urls.contains(url) {
                    new_urls.push(url.clone());
                }
            }
            dep.registry_urls = new_urls;

            // Resolve dep_name property references.
            if dep.dep_name.contains("${") {
                dep.dep_name = substitute_props(&dep.dep_name, &string_props(props));
            }

            // Resolve version property references.
            if dep.skip_reason == Some(MavenSkipReason::PropertyRef) {
                let prop_key = exact_property_ref(&dep.current_value);
                let prop_value = prop_key.and_then(|key| props.get(key));

                if let (Some(key), Some(prop)) = (prop_key, prop_value) {
                    dep.shared_variable_name = Some(key.to_owned());
                    dep.file_replace_position = Some(prop.file_replace_position);
                    dep.current_value = prop.val.clone();
                    // Go template expressions ({{...}}) are unresolvable version placeholders.
                    if dep.current_value.contains("{{") {
                        dep.skip_reason = Some(MavenSkipReason::PropertyRef);
                    } else {
                        dep.skip_reason = None;
                    }
                }
            }
        }
    }

    // Mark parent-root deps.
    // A parent dep is "root" when its parent POM either has no parent, or
    // its parent is not in the current file set.
    let root_dep_names: std::collections::HashSet<String> = package_files
        .iter()
        .filter_map(|pkg| {
            let parent_path = pkg.parent.as_deref()?;
            let parent_idx = path_to_index.get(parent_path)?;
            let parent_pkg = package_files.get(*parent_idx)?;
            let is_root = parent_pkg.parent.is_none()
                || path_to_index.get(parent_pkg.parent.as_deref()?).is_none();
            if is_root {
                pkg.deps
                    .iter()
                    .find(|d| d.dep_type == MavenDepType::Parent)
                    .map(|d| d.dep_name.clone())
            } else {
                None
            }
        })
        .collect();

    for pkg in package_files.iter_mut() {
        for dep in &mut pkg.deps {
            if dep.dep_type == MavenDepType::Parent && root_dep_names.contains(&dep.dep_name) {
                dep.dep_type = MavenDepType::ParentRoot;
            }
        }
    }
}

/// Convert a `HashMap<String, MavenProp>` to `HashMap<String, String>`
/// for use with `substitute_props`.
fn string_props(props: &HashMap<String, MavenProp>) -> HashMap<String, String> {
    props.iter().map(|(k, v)| (k.clone(), v.val.clone())).collect()
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
    extract_all_package_file_infos(files)
        .into_iter()
        .map(|package_file| package_file.deps)
        .collect()
}

/// Extract Maven package-file data from already-read path/content pairs.
///
/// This function performs cross-file parent resolution (`resolve_parents`),
/// merging properties and registry URLs from parent POMs into child POMs.
pub fn extract_all_package_file_infos(files: &[(&str, &str)]) -> Vec<MavenPackageFile> {
    let mut package_files = Vec::new();

    // Extract raw package files (no cross-file resolution yet).
    for (path, content) in files {
        if path.ends_with(".mvn/extensions.xml") || *path == ".mvn/extensions.xml" {
            if let Some(deps) = extract_extensions(content)
                && !deps.is_empty()
            {
                package_files.push(MavenPackageFile {
                    package_file: (*path).to_owned(),
                    deps,
                    package_file_version: None,
                    maven_props: HashMap::new(),
                    parent: None,
                });
            }
            continue;
        }

        if path.ends_with(".xml")
            && !is_settings_xml_path(path)
            && !content.is_empty()
        {
            if let Ok(pkg) = extract_package_file(content, path) {
                package_files.push(pkg);
            }
        }
    }

    // Cross-file property / registry resolution.
    resolve_parents(&mut package_files);

    // Apply settings-level registry URLs to all packages.
    let settings_registry_urls = extract_package_registry_urls(files);
    for pkg in &mut package_files {
        apply_registry_urls(&mut pkg.deps, &settings_registry_urls);
    }

    package_files
}

fn extract_package_registry_urls(files: &[(&str, &str)]) -> Vec<String> {
    let mut urls = Vec::new();
    let properties = package_file_properties(files);
    for (path, content) in files {
        if is_settings_xml_path(path) {
            for url in extract_registries(content) {
                if !urls.iter().any(|existing| existing == &url) {
                    urls.push(url);
                }
            }
        } else if path.ends_with(".xml") && !path.ends_with(".mvn/extensions.xml") {
            for url in extract_pom_registries(content, &properties) {
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

fn package_file_properties(files: &[(&str, &str)]) -> HashMap<String, String> {
    let mut properties = HashMap::new();
    for (path, content) in files {
        if path.ends_with(".xml")
            && !is_settings_xml_path(path)
            && !path.ends_with(".mvn/extensions.xml")
            && let Ok((_, pom_properties, _)) = parse_pom(content, path)
        {
            for (key, prop) in pom_properties {
                properties.entry(key).or_insert(prop.val);
            }
        }
    }
    properties
}

fn extract_pom_registries(content: &str, properties: &HashMap<String, String>) -> Vec<String> {
    let cursor = BufReader::new(content.as_bytes());
    let mut reader = Reader::from_reader(cursor);
    reader.config_mut().trim_text(true);

    let mut stack: Vec<String> = Vec::new();
    let mut urls = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                stack.push(String::from_utf8_lossy(e.name().as_ref()).into_owned());
            }
            Ok(Event::End(_)) => {
                stack.pop();
            }
            Ok(Event::Text(e)) => {
                if is_pom_registry_url_path(&stack) {
                    let text = e.decode().map(|s| s.trim().to_owned()).unwrap_or_default();
                    let url = apply_props(&text, properties);
                    if !url.is_empty() && !url.contains("${") && !urls.iter().any(|u| u == &url) {
                        urls.push(url);
                    }
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(_) => return Vec::new(),
        }
        buf.clear();
    }

    urls
}

fn is_pom_registry_url_path(stack: &[String]) -> bool {
    matches!(
        stack,
        [project, repositories, repository, url]
            if project == "project"
                && repositories == "repositories"
                && repository == "repository"
                && url == "url"
    )
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

/// Resolve a parent POM file path from a `<relativePath>` value.
/// Mirrors upstream `resolveParentFile`.
fn resolve_parent_file(package_file: &str, parent_path: &str) -> String {
    let (parent_file, parent_dir) = if parent_path.ends_with("pom.xml")
        || parent_path.ends_with(".pom.xml")
    {
        let file_name = std::path::Path::new(parent_path)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "pom.xml".to_owned());
        let dir = std::path::Path::new(parent_path)
            .parent()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        (file_name, dir)
    } else {
        ("pom.xml".to_owned(), parent_path.to_owned())
    };

    let dir = std::path::Path::new(package_file)
        .parent()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();

    let raw = if dir.is_empty() {
        format!("{}/{}", parent_dir, parent_file)
    } else {
        format!("{}/{}/{}", dir, parent_dir, parent_file)
    };
    normalize_path(&raw)
}

/// Normalize a path by resolving `.` and `..` components.
fn normalize_path(path: &str) -> String {
    use std::path::Component;
    let mut comps = Vec::new();
    for comp in std::path::Path::new(path).components() {
        match comp {
            Component::Normal(c) => comps.push(c.to_string_lossy().into_owned()),
            Component::ParentDir if !comps.is_empty() => {
                comps.pop();
            }
            Component::ParentDir => comps.push("..".to_owned()),
            Component::CurDir => {}
            Component::RootDir | Component::Prefix(_) => {
                comps.clear();
                comps.push(comp.as_os_str().to_string_lossy().into_owned());
            }
        }
    }
    comps.join("/")
}

/// SAX parse a POM and return (deps, properties, parent_file_path).
fn parse_pom(
    content: &str,
    package_file: &str,
) -> Result<(Vec<MavenExtractedDep>, HashMap<String, MavenProp>, Option<String>), MavenExtractError>
{
    let offset = content.find('<').unwrap_or(0);
    let cursor = BufReader::new(content.as_bytes());
    let mut reader = Reader::from_reader(cursor);
    reader.config_mut().trim_text(true);

    let mut deps: Vec<MavenExtractedDep> = Vec::new();
    let mut properties: HashMap<String, MavenProp> = HashMap::new();

    // Element name stack — tracks current XML path.
    let mut stack: Vec<String> = Vec::new();

    // Currently accumulating a dep record.
    let mut current: Option<CurrentDep> = None;
    let mut collect_start_depth: usize = 0;

    // Currently accumulating a <properties> child value.
    // `Some(key)` when we are inside <project><properties><key>.
    let mut prop_key: Option<String> = None;

    // Tracks <relativePath> text when inside <parent>.
    let mut parent_relative_path: Option<String> = None;
    let mut inside_parent: bool = false;

    let mut buf = Vec::new();
    let mut saw_project_root = false;

    loop {
        let pos_before = reader.buffer_position() as usize;
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                if stack.is_empty() && name != "project" {
                    // Root element is not <project> → not a valid Maven POM.
                    return Err(MavenExtractError::InvalidPom(format!(
                        "root element is <{name}>, expected <project>"
                    )));
                }
                saw_project_root = true;
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
                            inside_parent = true;
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
                        MavenDepType::Parent | MavenDepType::ParentRoot => "parent",
                        _ => "dependency",
                    };
                    if name == container
                        && !dep.artifact_id.is_empty()
                        && let Some(d) = build_dep(dep)
                    {
                        deps.push(d);
                    }
                    if dep.dep_type == MavenDepType::Parent {
                        inside_parent = false;
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
                    // Position relative to first '<' (matching extract/update expectations).
                    let file_replace_position = pos_before.saturating_sub(offset);
                    properties.insert(
                        key.clone(),
                        MavenProp {
                            val: text.clone(),
                            file_replace_position,
                        },
                    );
                }

                // Capture dep fields.
                if let Some(ref mut dep) = current
                    && stack.len() == collect_start_depth + 1
                {
                    match stack.last().map(String::as_str) {
                        Some("groupId") => dep.group_id = text.clone(),
                        Some("artifactId") => dep.artifact_id = text.clone(),
                        Some("version") => {
                            dep.version = text.clone();
                            dep.file_replace_position = Some(pos_before);
                        }
                        Some("scope") => dep.scope = Some(text.clone()),
                        Some("relativePath") => {
                            dep.relative_path = Some(text.clone());
                            if inside_parent {
                                parent_relative_path = Some(text.clone());
                            }
                        }
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

    // If we never saw a <project> start, this isn't a valid Maven POM.
    if !saw_project_root {
        return Err(MavenExtractError::InvalidPom(
            "no <project> element found".to_owned(),
        ));
    }

    // Compute parent file path from the parent dep's relativePath.
    // Default to ../pom.xml when <parent> exists but <relativePath> is absent.
    let parent = deps
        .iter()
        .find(|d| d.dep_type == MavenDepType::Parent)
        .map(|_| {
            let relative_path = parent_relative_path.as_deref().unwrap_or("../pom.xml");
            resolve_parent_file(package_file, relative_path)
        });

    Ok((deps, properties, parent))
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

fn exact_property_ref(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    trimmed
        .strip_prefix("${")
        .and_then(|rest| rest.strip_suffix('}'))
        .filter(|key| !key.contains("${") && !key.contains('}'))
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
    /// Byte offset of the `<version>` text content in the XML file.
    file_replace_position: Option<usize>,
    /// `<relativePath>` text when this is a `<parent>` element.
    relative_path: Option<String>,
}

impl CurrentDep {
    fn new(dep_type: MavenDepType) -> Self {
        Self {
            dep_type,
            group_id: String::new(),
            artifact_id: String::new(),
            version: String::new(),
            scope: None,
            file_replace_position: None,
            relative_path: None,
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
        shared_variable_name: None,
        file_replace_position: dep.file_replace_position,
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
        shared_variable_name: None,
        file_replace_position: None,
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

fn project_version(content: &str) -> Option<String> {
    let cursor = BufReader::new(content.as_bytes());
    let mut reader = Reader::from_reader(cursor);
    reader.config_mut().trim_text(true);

    let mut stack: Vec<String> = Vec::new();
    let mut version = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                stack.push(String::from_utf8_lossy(e.name().as_ref()).into_owned());
            }
            Ok(Event::End(_)) => {
                stack.pop();
            }
            Ok(Event::Text(e)) => {
                if stack.len() == 2 && stack[0] == "project" && stack[1] == "version" {
                    let text = e.decode().map(|s| s.trim().to_owned()).unwrap_or_default();
                    if !text.is_empty() {
                        version = Some(text);
                    }
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(_) => return None,
        }
        buf.clear();
    }

    version
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

// ═══════════════════════════════════════════════════════════════════════════
// maven update — lib/modules/manager/maven/update.ts
// ═══════════════════════════════════════════════════════════════════════════

/// Upgrade parameters for `maven_update_dependency`.
/// Mirrors the relevant fields from `Upgrade` in Renovate's `types.ts`.
#[derive(Debug, Clone, Default)]
pub struct MavenUpdateUpgrade {
    pub dep_name: Option<String>,
    pub new_name: Option<String>,
    pub current_value: Option<String>,
    pub new_value: Option<String>,
    pub current_digest: Option<String>,
    pub new_digest: Option<String>,
    pub datasource: Option<String>,
    pub shared_variable_name: Option<String>,
    /// Byte offset from the start of the XML content (after leading whitespace).
    pub file_replace_position: usize,
}

/// Replace the text value of a named XML element.
/// Mirrors `updateValue()` from `lib/modules/manager/maven/update.ts`.
fn maven_update_value(content: &str, node_name: &str, old_value: &str, new_value: &str) -> String {
    let element_start = format!("<{}", node_name);
    let element_end_tag = format!("</{}", node_name);
    let Some(start_idx) = content.find(&element_start) else {
        return content.to_owned();
    };
    let Some(gt_idx) = content[start_idx..].find('>') else {
        return content.to_owned();
    };
    let value_start = start_idx + gt_idx + 1;
    let Some(end_idx) = content[value_start..].find(&element_end_tag) else {
        return content.to_owned();
    };
    let value_end = value_start + end_idx;
    let element_content = &content[value_start..value_end];
    if element_content.trim() == old_value {
        let replaced = element_content.replacen(old_value, new_value, 1);
        format!(
            "{}{}{}",
            &content[..value_start],
            replaced,
            &content[value_end..]
        )
    } else {
        content.to_owned()
    }
}

/// Update a Maven POM value at the given byte position.
/// Mirrors `updateAtPosition()` from `lib/modules/manager/maven/update.ts`.
///
/// `ending_anchor` is usually `"</"` (the start of the closing tag).
pub fn maven_update_at_position(
    file_content: &str,
    upgrade: &MavenUpdateUpgrade,
    ending_anchor: &str,
) -> Option<String> {
    let pos = upgrade.file_replace_position;
    if pos > file_content.len() {
        return None;
    }
    let left_part = &file_content[..pos];
    let right_part = &file_content[pos..];

    let version_close_pos = right_part.find(ending_anchor)?;
    let mut rest_part = right_part[version_close_pos..].to_owned();
    let version_part = &right_part[..version_close_pos];
    let version = version_part.trim();

    let new_value = upgrade.new_value.as_deref().unwrap_or("");
    let current_value = upgrade.current_value.as_deref().unwrap_or("");

    if let Some(new_name) = upgrade.new_name.as_deref() {
        // Rename dep: update groupId and/or artifactId in the enclosing block.
        let block_tags = ["<parent", "<dependency", "<plugin", "<extension"];
        let close_tags = ["</parent", "</dependency", "</plugin", "</extension"];

        // Find the start of the enclosing block in left_part.
        let block_start = block_tags
            .iter()
            .filter_map(|tag| left_part.rfind(tag))
            .max()
            .unwrap_or(0);

        // Find the end of the enclosing block in rest_part.
        let block_end_relative = close_tags
            .iter()
            .filter_map(|tag| rest_part.find(tag))
            .min();

        let block_end = block_end_relative?;

        let mut left_block = left_part[block_start..].to_owned();
        let mut right_block = rest_part[..block_end].to_owned();

        let dep_name = upgrade.dep_name.as_deref().unwrap_or("");
        let (group_id, artifact_id) = dep_name.split_once(':').unwrap_or((dep_name, ""));
        let (new_group_id, new_artifact_id) = new_name.split_once(':').unwrap_or((new_name, ""));

        if left_block.contains("<groupId") {
            left_block = maven_update_value(&left_block, "groupId", group_id, new_group_id);
        } else {
            right_block = maven_update_value(&right_block, "groupId", group_id, new_group_id);
        }

        if left_block.contains("<artifactId") {
            left_block =
                maven_update_value(&left_block, "artifactId", artifact_id, new_artifact_id);
        } else {
            right_block =
                maven_update_value(&right_block, "artifactId", artifact_id, new_artifact_id);
        }

        let left_prefix = &left_part[..block_start];
        rest_part = right_block + &rest_part[block_end..];

        // Also update the version if new_value differs from current.
        let (final_left, final_right) = if !new_value.is_empty() && version != new_value {
            let replaced = version_part.replacen(version, new_value, 1);
            (
                format!("{}{}{}", left_prefix, left_block, replaced),
                rest_part,
            )
        } else {
            (format!("{}{}", left_prefix, left_block), rest_part)
        };
        Some(format!("{}{}", final_left, final_right))
    } else {
        // Version-only update.
        let is_docker = matches!(
            upgrade.datasource.as_deref(),
            Some("docker") | Some("buildpacks-registry")
        );

        if version == new_value && upgrade.shared_variable_name.is_none() {
            // Already at desired version.
            return Some(file_content.to_owned());
        }

        if version == current_value || upgrade.shared_variable_name.is_some() {
            let replaced = version_part.replacen(version, new_value, 1);
            Some(format!("{}{}{}", left_part, replaced, rest_part))
        } else if is_docker {
            let mut replaced = version.to_owned();
            if !current_value.is_empty() {
                replaced = replaced.replacen(current_value, new_value, 1);
            }
            if let (Some(cur_digest), Some(new_digest)) = (
                upgrade.current_digest.as_deref(),
                upgrade.new_digest.as_deref(),
            ) {
                replaced = replaced.replacen(cur_digest, new_digest, 1);
            }
            if replaced != version {
                Some(format!("{}{}{}", left_part, replaced, rest_part))
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// Update a Maven POM dependency entry in a format-preserving way.
/// Mirrors `updateDependency()` from `lib/modules/manager/maven/update.ts`.
pub fn maven_update_dependency(file_content: &str, upgrade: &MavenUpdateUpgrade) -> Option<String> {
    let offset = file_content.find('<').unwrap_or(0);
    let spaces = &file_content[..offset];
    let rest_content = &file_content[offset..];
    let updated = maven_update_at_position(rest_content, upgrade, "</")?;
    if updated == rest_content {
        Some(file_content.to_owned())
    } else {
        Some(format!("{}{}", spaces, updated))
    }
}

/// The result of `maven_bump_package_version`.
#[derive(Debug)]
pub struct MavenBumpResult {
    pub bumped_content: String,
}

/// Bump the `<version>` element in a Maven POM file.
/// Mirrors `bumpPackageVersion()` from `lib/modules/manager/maven/update.ts`.
///
/// Handles SNAPSHOT qualifiers: a SNAPSHOT version like `0.0.1-SNAPSHOT` keeps
/// its qualifier through the bump.  A release version gets `-SNAPSHOT` appended
/// when `bump_version = "prerelease"`.
pub fn maven_bump_package_version(
    content: &str,
    current_value: &str,
    bump_version: &str,
) -> MavenBumpResult {
    let bumped_content = try_bump_pom_version(content, current_value, bump_version)
        .unwrap_or_else(|| content.to_owned());
    MavenBumpResult { bumped_content }
}

fn try_bump_pom_version(content: &str, current_value: &str, bump_version: &str) -> Option<String> {
    // Must be a valid semver to bump.
    let parsed = semver::Version::parse(current_value).ok()?;

    let new_version = compute_bumped_pom_version(&parsed, current_value, bump_version)?;

    // Find the root-level <version> element and replace its text content.
    // We look for the text content of the first <version> element that is a
    // direct child of the root (depth 1).
    let version_pos = find_root_version_position(content)?;
    let (val_start, val_end) = version_pos;
    let found = &content[val_start..val_end];
    if found != current_value {
        return None;
    }
    Some(format!(
        "{}{}{}",
        &content[..val_start],
        new_version,
        &content[val_end..]
    ))
}

/// Compute the new POM version string given the current semver and bump type.
fn compute_bumped_pom_version(
    parsed: &semver::Version,
    _current_str: &str,
    bump_version: &str,
) -> Option<String> {
    let pre_str = if parsed.pre.is_empty() {
        None
    } else {
        Some(parsed.pre.as_str())
    };

    let is_snapshot = pre_str.map(|p| p.ends_with("SNAPSHOT")).unwrap_or(false);
    let is_prerelease_non_snapshot = pre_str.is_some() && !is_snapshot;

    if is_snapshot {
        // Keep the same prerelease qualifier, bump the numeric component.
        let qualifier = pre_str.unwrap_or("SNAPSHOT");
        let pre_bump = if !bump_version.starts_with("pre") {
            format!("pre{}", bump_version)
        } else {
            bump_version.to_owned()
        };
        let (new_major, new_minor, new_patch) =
            bump_numeric(parsed.major, parsed.minor, parsed.patch, &pre_bump)?;
        Some(format!(
            "{}.{}.{}-{}",
            new_major, new_minor, new_patch, qualifier
        ))
    } else if is_prerelease_non_snapshot {
        // Prerelease with a non-SNAPSHOT qualifier: increment the pre identifier.
        // e.g. "1.0.0-1" + prerelease → "1.0.0-2"
        if bump_version == "prerelease" {
            let pre = pre_str.unwrap_or("0");
            // Try to increment the last numeric component of the pre string.
            let new_pre = increment_prerelease_identifier(pre)?;
            Some(format!(
                "{}.{}.{}-{}",
                parsed.major, parsed.minor, parsed.patch, new_pre
            ))
        } else {
            let (major, minor, patch) =
                bump_numeric(parsed.major, parsed.minor, parsed.patch, bump_version)?;
            Some(format!("{}.{}.{}", major, minor, patch))
        }
    } else {
        // Release version.
        if bump_version == "prerelease" {
            // Add -SNAPSHOT after bumping patch.
            let (major, minor, patch) =
                bump_numeric(parsed.major, parsed.minor, parsed.patch, "prepatch")?;
            Some(format!("{}.{}.{}-SNAPSHOT", major, minor, patch))
        } else {
            let (major, minor, patch) =
                bump_numeric(parsed.major, parsed.minor, parsed.patch, bump_version)?;
            Some(format!("{}.{}.{}", major, minor, patch))
        }
    }
}

fn bump_numeric(major: u64, minor: u64, patch: u64, bump: &str) -> Option<(u64, u64, u64)> {
    match bump {
        "patch" | "prepatch" => Some((major, minor, patch + 1)),
        "minor" | "preminor" => Some((major, minor + 1, 0)),
        "major" | "premajor" => Some((major + 1, 0, 0)),
        _ => None,
    }
}

fn increment_prerelease_identifier(pre: &str) -> Option<String> {
    // Try to parse the last dot-separated segment as a number.
    let parts: Vec<&str> = pre.split('.').collect();
    let last = parts.last()?;
    if let Ok(n) = last.parse::<u64>() {
        let mut new_parts = parts[..parts.len() - 1].to_vec();
        new_parts.push(&*Box::leak(format!("{}", n + 1).into_boxed_str()));
        Some(new_parts.join("."))
    } else if let Ok(n) = pre.parse::<u64>() {
        // Single numeric identifier like "1".
        Some(format!("{}", n + 1))
    } else {
        None
    }
}

/// Find the byte range of the text content in the root-level `<version>` element.
/// Returns `(start, end)` byte offsets into `content`.
fn find_root_version_position(content: &str) -> Option<(usize, usize)> {
    use quick_xml::{Reader, events::Event};

    let mut reader = Reader::from_str(content);
    reader.config_mut().trim_text(false);
    reader.config_mut().check_end_names = false;

    let mut depth: usize = 0;
    let mut in_version_at_depth_one = false;

    loop {
        let pos_before = reader.buffer_position() as usize;
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                depth += 1;
                if depth == 2 {
                    // Direct child of root.
                    let name_bytes = e.name().as_ref().to_owned();
                    let name = std::str::from_utf8(&name_bytes).unwrap_or("");
                    if name == "version" {
                        in_version_at_depth_one = true;
                    }
                }
            }
            Ok(Event::Text(e)) if in_version_at_depth_one => {
                let start = pos_before;
                let end = reader.buffer_position() as usize;
                let text = std::str::from_utf8(e.as_ref()).unwrap_or("");
                // Verify it spans the right range.
                if end > start && start + text.len() == end {
                    return Some((start, end));
                }
                // Fallback: search for text in content from pos_before.
                let text_in_content = &content[start..];
                if text_in_content.starts_with(text) {
                    return Some((start, start + text.len()));
                }
                return None;
            }
            Ok(Event::End(_)) => {
                if in_version_at_depth_one && depth == 2 {
                    in_version_at_depth_one = false;
                }
                depth = depth.saturating_sub(1);
            }
            Ok(Event::Eof) | Err(_) => break,
            _ => {}
        }
    }
    None
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

    // Ported: "should return package files info" — maven/extract.spec.ts line 812
    #[test]
    fn extract_all_package_file_infos_returns_package_file_metadata() {
        let pom = r#"<project>
  <parent>
    <groupId>org.example</groupId>
    <artifactId>parent</artifactId>
    <version>42</version>
  </parent>
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.example</groupId>
  <artifactId>ExamplePomFile</artifactId>
  <version>0.0.1</version>
  <properties>
    <quuxVersion>1.2.3.4</quuxVersion>
  </properties>
  <dependencies>
    <dependency>
      <groupId>org.example</groupId>
      <artifactId>foo</artifactId>
      <version>0.0.1</version>
    </dependency>
    <dependency>
      <groupId>org.example</groupId>
      <artifactId>quux</artifactId>
      <version>${quuxVersion}</version>
    </dependency>
  </dependencies>
</project>"#;

        let packages = extract_all_package_file_infos(&[("random.pom.xml", pom)]);
        assert_eq!(packages.len(), 1);
        let package = &packages[0];
        assert_eq!(package.package_file, "random.pom.xml");
        assert_eq!(package.package_file_version.as_deref(), Some("0.0.1"));
        assert!(
            package
                .deps
                .iter()
                .any(|dep| dep.dep_name == "org.example:parent"
                    && dep.current_value == "42"
                    && dep.dep_type == MavenDepType::Parent)
        );
        assert!(
            package
                .deps
                .iter()
                .any(|dep| dep.dep_name == "org.example:foo" && dep.current_value == "0.0.1")
        );
        let quux = package
            .deps
            .iter()
            .find(|dep| dep.dep_name == "org.example:quux")
            .unwrap();
        assert_eq!(quux.current_value, "1.2.3.4");
        assert_eq!(quux.shared_variable_name.as_deref(), Some("quuxVersion"));
    }

    // Ported: "should include registryUrls from parent pom files" — maven/extract.spec.ts line 581
    #[test]
    fn extract_all_package_files_includes_registry_urls_from_parent_poms() {
        let parent = r#"<project>
  <parent>
    <groupId>org.example</groupId>
    <artifactId>child</artifactId>
    <version>42</version>
    <relativePath>child.pom.xml</relativePath>
  </parent>
  <properties>
    <repoUrl>http://example.com/</repoUrl>
  </properties>
  <repositories>
    <repository>
      <url>http://example.com/nexus/xyz</url>
    </repository>
  </repositories>
</project>"#;
        let child = r#"<project>
  <parent>
    <groupId>org.example</groupId>
    <artifactId>parent</artifactId>
    <version>42</version>
    <relativePath>parent.pom.xml</relativePath>
  </parent>
  <dependencies>
    <dependency>
      <groupId>org.example</groupId>
      <artifactId>foo</artifactId>
      <version>0.0.1</version>
    </dependency>
  </dependencies>
  <repositories>
    <repository>
      <url>${repoUrl}</url>
    </repository>
  </repositories>
</project>"#;

        let packages =
            extract_all_package_files(&[("parent.pom.xml", parent), ("child.pom.xml", child)]);
        let expected = vec![
            "http://example.com/nexus/xyz".to_owned(),
            "http://example.com/".to_owned(),
            "https://repo.maven.apache.org/maven2".to_owned(),
        ];
        assert_eq!(packages.len(), 2);
        for dep in packages.iter().flatten() {
            assert_eq!(dep.registry_urls, expected);
        }
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

    // Ported: "should return empty array if extensions file is invalid or empty" — maven/extract.spec.ts line 998
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

    // Ported: "should skip root pom.xml" — maven/extract.spec.ts line 1011
    #[test]
    fn extract_all_package_files_marks_child_parent_as_parent_root() {
        let root = r#"<project>
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.example</groupId>
  <artifactId>root</artifactId>
  <version>1.0.0</version>
</project>"#;
        let child = r#"<project>
  <parent>
    <groupId>org.example</groupId>
    <artifactId>root</artifactId>
    <version>1.0.0</version>
  </parent>
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.example</groupId>
  <artifactId>child</artifactId>
</project>"#;

        let packages = extract_all_package_files(&[("pom.xml", root), ("foo.bar/pom.xml", child)]);
        // Upstream returns both packages; root has empty deps, child has parent-root dep.
        assert_eq!(packages.len(), 2);
        assert!(packages[0].is_empty());
        assert_eq!(packages[1][0].dep_name, "org.example:root");
        assert_eq!(packages[1][0].dep_type, MavenDepType::ParentRoot);
        assert_eq!(packages[1][0].renovate_dep_type(), "parent-root");
    }

    // Ported: "should skip root pom.xml when it has an external parent" — maven/extract.spec.ts line 1045
    #[test]
    fn extract_all_package_files_keeps_external_root_parent() {
        let root = r#"<project>
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.example</groupId>
  <artifactId>root</artifactId>
  <version>1.0.0</version>
  <parent>
    <groupId>org.acme</groupId>
    <artifactId>external-parent</artifactId>
    <version>1.0.0</version>
  </parent>
</project>"#;
        let child = r#"<project>
  <parent>
    <groupId>org.example</groupId>
    <artifactId>root</artifactId>
    <version>1.0.0</version>
  </parent>
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.example</groupId>
  <artifactId>child</artifactId>
</project>"#;

        let packages = extract_all_package_files(&[("pom.xml", root), ("foo.bar/pom.xml", child)]);
        assert_eq!(packages.len(), 2);
        assert_eq!(packages[0][0].dep_name, "org.acme:external-parent");
        assert_eq!(packages[0][0].dep_type, MavenDepType::Parent);
        assert_eq!(packages[1][0].dep_name, "org.example:root");
        assert_eq!(packages[1][0].dep_type, MavenDepType::ParentRoot);
    }

    // Ported: "handles cross-referencing" — maven/extract.spec.ts line 1087
    #[test]
    fn extract_all_package_files_handles_cross_referencing_modules() {
        let foo = r#"<project>
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.example</groupId>
  <artifactId>foo</artifactId>
  <version>1.0.0</version>
  <dependencies>
    <dependency>
      <groupId>org.example</groupId>
      <artifactId>bar</artifactId>
      <version>1.0.0</version>
    </dependency>
  </dependencies>
</project>"#;
        let bar = r#"<project>
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.example</groupId>
  <artifactId>bar</artifactId>
  <version>1.0.0</version>
  <dependencies>
    <dependency>
      <groupId>org.example</groupId>
      <artifactId>foo</artifactId>
      <version>1.0.0</version>
    </dependency>
  </dependencies>
</project>"#;

        let packages = extract_all_package_files(&[("foo.xml", foo), ("bar.xml", bar)]);
        assert_eq!(packages.len(), 2);
        assert_eq!(packages[0][0].dep_name, "org.example:bar");
        assert_eq!(packages[0][0].skip_reason, None);
        assert_eq!(packages[1][0].dep_name, "org.example:foo");
        assert_eq!(packages[1][0].skip_reason, None);
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

    // Ported: "should detect props infinitely recursing props" — manager/maven/extract.spec.ts line 448
    #[test]
    fn substitute_props_handles_unknown_key() {
        let mut props = HashMap::new();
        props.insert("known".to_owned(), "1.0".to_owned());
        let result = substitute_props("${known}-${unknown}", &props);
        assert_eq!(result, "1.0-${unknown}");
    }

    // Ported: "should detect props infinitely recursing props" — manager/maven/extract.spec.ts line 448
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

    // Rust-specific: maven behavior test
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
        assert_eq!(MavenDepType::ParentRoot.as_renovate_str(), "parent-root");
        assert_eq!(MavenDepType::Profile.as_renovate_str(), "compile");
    }

    // Rust-specific: maven behavior test
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
            shared_variable_name: None,
            file_replace_position: None,
        };
        assert_eq!(dep.renovate_dep_type(), "test");
    }

    // Rust-specific: maven behavior test
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
            shared_variable_name: None,
            file_replace_position: None,
        };
        assert_eq!(dep.renovate_dep_type(), "compile");
    }

    // Ported: "returns null for invalid XML" — maven/extract.spec.ts line 22
    #[test]
    fn invalid_xml_returns_empty() {
        // Empty string, invalid XML, and documents with wrong root element all
        // return Err (mirrors extractPackage returning null).
        assert!(extract("").is_err());
        assert!(extract("invalid xml content").is_err());
        assert!(extract("<foobar></foobar>").is_err());
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

    // Ported: "should apply props multiple times" — maven/extract.spec.ts line 433
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

    // Ported: "should extract from pom.template.xml file" — maven/extract.spec.ts line 917
    #[test]
    fn extracts_from_pom_template_xml_file() {
        let content = r#"<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0">
  <modelVersion>4.0.0</modelVersion>
  <groupId>org.example</groupId>
  <artifactId>template-project</artifactId>
  <version>1.0.0</version>
  <properties>
    <manifold.version>2021.1.12</manifold.version>
    <scala.version>{{scala_version}}</scala.version>
  </properties>
  <dependencies>
    <dependency>
      <groupId>org.springframework.boot</groupId>
      <artifactId>spring-boot-starter-web</artifactId>
      <version>3.2.0</version>
    </dependency>
    <dependency>
      <groupId>org.junit.jupiter</groupId>
      <artifactId>junit-jupiter</artifactId>
      <version>5.10.1</version>
      <scope>test</scope>
    </dependency>
    <dependency>
      <groupId>org.scala-lang</groupId>
      <artifactId>scala-library</artifactId>
      <version>${scala.version}</version>
    </dependency>
  </dependencies>
</project>"#;
        // pom.template.xml ends with .xml — extractor must process it
        let packages = extract_all_package_files(&[("pom.template.xml", content)]);
        assert!(!packages.is_empty(), "pom.template.xml should be processed");
        let deps = &packages[0];
        let spring = deps
            .iter()
            .find(|d| d.dep_name.contains("spring-boot-starter-web"));
        assert!(spring.is_some());
        assert_eq!(spring.unwrap().current_value, "3.2.0");
        assert!(spring.unwrap().skip_reason.is_none());
        let junit = deps.iter().find(|d| d.dep_name.contains("junit-jupiter"));
        assert!(junit.is_some());
        assert_eq!(junit.unwrap().current_value, "5.10.1");
        // scala-library version resolves to {{scala_version}} template placeholder — unresolvable
        let scala = deps.iter().find(|d| d.dep_name.contains("scala-library"));
        assert!(scala.is_some());
        assert!(scala.unwrap().skip_reason.is_some());
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

    // ── maven updateDependency tests ───────────────────────────────────────

    const SIMPLE_POM_FULL: &str = include_str!("../../tests/fixtures/maven/simple.pom.xml");
    const MINIMUM_POM: &str = include_str!("../../tests/fixtures/maven/minimum.pom.xml");
    const MINIMUM_SNAPSHOT_POM: &str =
        include_str!("../../tests/fixtures/maven/minimum_snapshot.pom.xml");
    const PRERELEASE_POM: &str = include_str!("../../tests/fixtures/maven/prerelease.pom.xml");
    const FULL_CNB_POM: &str = include_str!("../../tests/fixtures/maven/full_cnb.pom.xml");

    fn parse_xml_value(xml: &str, path: &[&str]) -> Option<String> {
        // Simple XPath-like traversal using quick-xml to extract a text value.
        use quick_xml::{Reader, events::Event};
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);
        reader.config_mut().check_end_names = false;
        let mut stack: Vec<String> = Vec::new();
        let mut result = None;
        loop {
            match reader.read_event() {
                Ok(Event::Start(e)) => {
                    let name = std::str::from_utf8(e.name().as_ref())
                        .unwrap_or("")
                        .to_owned();
                    stack.push(name);
                }
                Ok(Event::Text(e)) if result.is_none() => {
                    let text = std::str::from_utf8(e.as_ref())
                        .unwrap_or("")
                        .trim()
                        .to_owned();
                    if !text.is_empty() {
                        let cur_path: Vec<&str> = stack.iter().map(|s| s.as_str()).collect();
                        // Return first non-empty match where path ends with target path.
                        if cur_path.len() >= path.len()
                            && &cur_path[cur_path.len() - path.len()..] == path
                        {
                            result = Some(text);
                        }
                    }
                }
                Ok(Event::End(_)) => {
                    stack.pop();
                }
                Ok(Event::Eof) | Err(_) => break,
                _ => {}
            }
        }
        result
    }

    // Ported: "should update version" — maven/update.spec.ts line 15
    #[test]
    fn maven_update_dep_version() {
        let upgrade = MavenUpdateUpgrade {
            dep_name: Some("org.example:foo".into()),
            current_value: Some("0.0.1".into()),
            new_value: Some("0.0.2".into()),
            file_replace_position: 905,
            ..Default::default()
        };
        let result = maven_update_dependency(SIMPLE_POM_FULL, &upgrade).unwrap();
        let value = parse_xml_value(
            &result,
            &[
                "project",
                "dependencyManagement",
                "dependencies",
                "dependency",
                "version",
            ],
        );
        assert_eq!(value.as_deref(), Some("0.0.2"));
    }

    // Ported: "should do simple replacement" — maven/update.spec.ts line 36
    #[test]
    fn maven_update_dep_simple_replacement() {
        let upgrade = MavenUpdateUpgrade {
            dep_name: Some("org.example:foo".into()),
            new_name: Some("org.example.new:foo".into()),
            current_value: Some("0.0.1".into()),
            new_value: Some("0.0.1".into()),
            file_replace_position: 905,
            ..Default::default()
        };
        let result = maven_update_dependency(SIMPLE_POM_FULL, &upgrade).unwrap();
        let group_id = parse_xml_value(
            &result,
            &[
                "project",
                "dependencyManagement",
                "dependencies",
                "dependency",
                "groupId",
            ],
        );
        assert_eq!(group_id.as_deref(), Some("org.example.new"));
    }

    // Ported: "should do full replacement" — maven/update.spec.ts line 58
    #[test]
    fn maven_update_dep_full_replacement() {
        let upgrade = MavenUpdateUpgrade {
            dep_name: Some("org.example:foo".into()),
            new_name: Some("org.example.new:bar".into()),
            current_value: Some("0.0.1".into()),
            new_value: Some("0.0.2".into()),
            file_replace_position: 905,
            ..Default::default()
        };
        let result = maven_update_dependency(SIMPLE_POM_FULL, &upgrade).unwrap();
        let group_id = parse_xml_value(
            &result,
            &[
                "project",
                "dependencyManagement",
                "dependencies",
                "dependency",
                "groupId",
            ],
        );
        let artifact_id = parse_xml_value(
            &result,
            &[
                "project",
                "dependencyManagement",
                "dependencies",
                "dependency",
                "artifactId",
            ],
        );
        let version = parse_xml_value(
            &result,
            &[
                "project",
                "dependencyManagement",
                "dependencies",
                "dependency",
                "version",
            ],
        );
        assert_eq!(group_id.as_deref(), Some("org.example.new"));
        assert_eq!(artifact_id.as_deref(), Some("bar"));
        assert_eq!(version.as_deref(), Some("0.0.2"));
    }

    // Ported: "should do replacement if version is first" — maven/update.spec.ts line 90
    #[test]
    fn maven_update_dep_replacement_version_first() {
        let content = "<project xmlns=\"http://maven.apache.org/POM/4.0.0\">\n  <dependencyManagement>\n    <dependencies>\n      <dependency>\n        <version>0.0.1</version>\n        <artifactId>foo</artifactId>\n        <groupId>org.example</groupId>\n      </dependency>\n    </dependencies>\n  </dependencyManagement>\n</project>\n";
        // fileReplacePosition 132: offset in restContent after first '<'
        // Let's find it dynamically
        let first_lt = content.find('<').unwrap_or(0);
        let rest = &content[first_lt..];
        let pos = rest.find("0.0.1</version>").unwrap();
        let upgrade = MavenUpdateUpgrade {
            dep_name: Some("org.example:foo".into()),
            new_name: Some("org.example.new:bar".into()),
            current_value: Some("0.0.1".into()),
            new_value: Some("0.0.1".into()),
            file_replace_position: pos,
            ..Default::default()
        };
        let result = maven_update_dependency(content, &upgrade).unwrap();
        let group_id = parse_xml_value(
            &result,
            &[
                "project",
                "dependencyManagement",
                "dependencies",
                "dependency",
                "groupId",
            ],
        );
        let artifact_id = parse_xml_value(
            &result,
            &[
                "project",
                "dependencyManagement",
                "dependencies",
                "dependency",
                "artifactId",
            ],
        );
        assert_eq!(group_id.as_deref(), Some("org.example.new"));
        assert_eq!(artifact_id.as_deref(), Some("bar"));
    }

    // Ported: "should ignore replacement if name does not match" — maven/update.spec.ts line 134
    #[test]
    fn maven_update_dep_ignore_mismatched_name() {
        let upgrade = MavenUpdateUpgrade {
            dep_name: Some("org.example.old:bar".into()),
            new_name: Some("org.example:foo".into()),
            current_value: Some("0.0.1".into()),
            new_value: Some("0.0.1".into()),
            file_replace_position: 905,
            ..Default::default()
        };
        let result = maven_update_dependency(SIMPLE_POM_FULL, &upgrade).unwrap();
        // Should return original content (group doesn't match, so updateValue returns content unchanged)
        // The result may differ due to version staying the same
        // Just verify the group didn't change
        let group_id = parse_xml_value(
            &result,
            &[
                "project",
                "dependencyManagement",
                "dependencies",
                "dependency",
                "groupId",
            ],
        );
        // groupId should remain "org.example" since name doesn't match
        assert_eq!(group_id.as_deref(), Some("org.example"));
    }

    // Ported: "should update a cloud native buildpack version" — maven/update.spec.ts line 151
    #[test]
    fn maven_update_dep_cnb_version() {
        let upgrade = MavenUpdateUpgrade {
            datasource: Some("docker".into()),
            dep_name: Some("paketo-buildpacks/nodejs".into()),
            current_value: Some("6.1.1".into()),
            new_value: Some("6.1.2".into()),
            file_replace_position: 1430,
            ..Default::default()
        };
        let result = maven_update_dependency(FULL_CNB_POM, &upgrade).unwrap();
        assert!(result.contains("paketo-buildpacks/nodejs@6.1.2"));
    }

    // Ported: "should update a cloud native buildpack digest" — maven/update.spec.ts line 173
    #[test]
    fn maven_update_dep_cnb_digest() {
        let upgrade = MavenUpdateUpgrade {
            datasource: Some("docker".into()),
            dep_name: Some("docker.io/paketobuildpacks/python".into()),
            current_value: Some("2.22.1".into()),
            new_value: Some("2.24.3".into()),
            current_digest: Some(
                "sha256:2c27cd0b4482a4aa5aeb38104f6d934511cd87c1af34a10d1d6cdf2d9d16f138".into(),
            ),
            new_digest: Some(
                "sha256:ab0cf962a92158f15d9e4fed6f905d5d292ed06a8e6291aa1ce3c33a5c78bde1".into(),
            ),
            file_replace_position: 1634,
            ..Default::default()
        };
        let result = maven_update_dependency(FULL_CNB_POM, &upgrade).unwrap();
        assert!(result.contains("paketobuildpacks/python:2.24.3@sha256:ab0cf962a92158f15d9e4fed6f905d5d292ed06a8e6291aa1ce3c33a5c78bde1"));
    }

    // ── maven bumpPackageVersion tests ─────────────────────────────────────

    // Ported: "bumps pom.xml version" — maven/update.spec.ts line 215
    #[test]
    fn maven_bump_version_patch() {
        let result = maven_bump_package_version(SIMPLE_POM_FULL, "0.0.1", "patch");
        let version = parse_xml_value(&result.bumped_content, &["project", "version"]);
        assert_eq!(version.as_deref(), Some("0.0.2"));
    }

    // Ported: "bumps pom.xml version keeping SNAPSHOT" — maven/update.spec.ts line 226
    #[test]
    fn maven_bump_version_snapshot_patch() {
        let result = maven_bump_package_version(MINIMUM_SNAPSHOT_POM, "0.0.1-SNAPSHOT", "patch");
        let version = parse_xml_value(&result.bumped_content, &["project", "version"]);
        assert_eq!(version.as_deref(), Some("0.0.2-SNAPSHOT"));
    }

    // Ported: "bumps pom.xml minor version keeping SNAPSHOT" — maven/update.spec.ts line 237
    #[test]
    fn maven_bump_version_snapshot_minor() {
        let result = maven_bump_package_version(MINIMUM_SNAPSHOT_POM, "0.0.1-SNAPSHOT", "minor");
        let version = parse_xml_value(&result.bumped_content, &["project", "version"]);
        assert_eq!(version.as_deref(), Some("0.1.0-SNAPSHOT"));
    }

    // Ported: "bumps pom.xml major version keeping SNAPSHOT" — maven/update.spec.ts line 248
    #[test]
    fn maven_bump_version_snapshot_major() {
        let result = maven_bump_package_version(MINIMUM_SNAPSHOT_POM, "0.0.1-SNAPSHOT", "major");
        let version = parse_xml_value(&result.bumped_content, &["project", "version"]);
        assert_eq!(version.as_deref(), Some("1.0.0-SNAPSHOT"));
    }

    // Ported: "bumps pom.xml version keeping qualifier with -SNAPSHOT" — maven/update.spec.ts line 259
    #[test]
    fn maven_bump_version_qualified_snapshot() {
        let content = MINIMUM_SNAPSHOT_POM.replace("0.0.1-SNAPSHOT", "0.0.1-qualified-SNAPSHOT");
        let result = maven_bump_package_version(&content, "0.0.1-qualified-SNAPSHOT", "patch");
        let version = parse_xml_value(&result.bumped_content, &["project", "version"]);
        assert_eq!(version.as_deref(), Some("0.0.2-qualified-SNAPSHOT"));
    }

    // Ported: "does not bump version twice" — maven/update.spec.ts line 273
    #[test]
    fn maven_bump_version_not_twice() {
        let result1 = maven_bump_package_version(SIMPLE_POM_FULL, "0.0.1", "patch");
        let bumped = &result1.bumped_content;
        let result2 = maven_bump_package_version(bumped, "0.0.1", "patch");
        // Second bump should not change (version is 0.0.2, not 0.0.1)
        assert_eq!(result2.bumped_content, *bumped);
    }

    // Ported: "does not bump version if version is not a semantic version" — maven/update.spec.ts line 288
    #[test]
    fn maven_bump_version_non_semver() {
        let result = maven_bump_package_version(MINIMUM_POM, "1", "patch");
        let version = parse_xml_value(&result.bumped_content, &["project", "version"]);
        // Non-semver stays as-is
        assert_eq!(version.as_deref(), Some("1"));
    }

    // Ported: "does not bump version if pom.xml has no version" — maven/update.spec.ts line 299
    #[test]
    fn maven_bump_version_no_version() {
        let result = maven_bump_package_version(MINIMUM_POM, "", "patch");
        assert_eq!(result.bumped_content, MINIMUM_POM);
    }

    // Ported: "returns content if bumping errors" — maven/update.spec.ts line 305
    #[test]
    fn maven_bump_version_error_returns_content() {
        // Invalid bump_version → returns original content
        let result = maven_bump_package_version(SIMPLE_POM_FULL, "0.0.1", "invalid_bump_type");
        assert_eq!(result.bumped_content, SIMPLE_POM_FULL);
    }

    // Ported: "bumps pom.xml version to SNAPSHOT with prerelease" — maven/update.spec.ts line 314
    #[test]
    fn maven_bump_version_prerelease_adds_snapshot() {
        let result = maven_bump_package_version(SIMPLE_POM_FULL, "0.0.1", "prerelease");
        let version = parse_xml_value(&result.bumped_content, &["project", "version"]);
        assert_eq!(version.as_deref(), Some("0.0.2-SNAPSHOT"));
    }

    // Ported: "bumps pom.xml version with prerelease semver level" — maven/update.spec.ts line 325
    #[test]
    fn maven_bump_version_prerelease_increment() {
        let result = maven_bump_package_version(PRERELEASE_POM, "1.0.0-1", "prerelease");
        let version = parse_xml_value(&result.bumped_content, &["project", "version"]);
        assert_eq!(version.as_deref(), Some("1.0.0-2"));
    }

    // Verify file_replace_position tracking matches known fixture positions.

    // Rust-specific: maven behavior test
    #[test]
    fn extract_tracks_file_replace_position() {
        let deps = extract(SIMPLE_POM_FULL).unwrap();
        let foo_dep = deps
            .iter()
            .find(|d| d.dep_name == "org.example:foo")
            .unwrap();
        assert_eq!(foo_dep.file_replace_position, Some(905));

        let quuz_dep = deps
            .iter()
            .find(|d| d.dep_name == "org.example:quuz")
            .unwrap();
        assert_eq!(quuz_dep.file_replace_position, Some(3086));

        let hard_range = deps
            .iter()
            .find(|d| d.dep_name == "org.example:hard-range")
            .unwrap();
        assert_eq!(hard_range.file_replace_position, Some(3410));
    }

    // ── Ported from maven/index.spec.ts ───────────────────────────────────────

    // Ported: "should update an existing dependency" — maven/index.spec.ts line 26
    #[test]
    fn maven_index_update_existing_dependency() {
        let new_value = "9.9.9.9-final";
        let deps = extract(SIMPLE_POM_FULL).unwrap();
        let dep = deps
            .iter()
            .find(|d| d.dep_name == "org.example:quuz")
            .unwrap();
        let upgrade = MavenUpdateUpgrade {
            dep_name: Some(dep.dep_name.clone()),
            current_value: Some(dep.current_value.clone()),
            new_value: Some(new_value.to_owned()),
            file_replace_position: dep.file_replace_position.unwrap(),
            ..Default::default()
        };
        let updated = maven_update_dependency(SIMPLE_POM_FULL, &upgrade).unwrap();
        let updated_deps = extract(&updated).unwrap();
        let updated_dep = updated_deps
            .iter()
            .find(|d| d.dep_name == "org.example:quuz")
            .unwrap();
        assert_eq!(updated_dep.current_value, new_value);
    }

    // Ported: "should not touch content if new and old versions are equal" — maven/index.spec.ts line 67
    #[test]
    fn maven_index_no_touch_when_equal() {
        let deps = extract(SIMPLE_POM_FULL).unwrap();
        let dep = deps
            .iter()
            .find(|d| d.dep_name == "org.example:quuz")
            .unwrap();
        let upgrade = MavenUpdateUpgrade {
            dep_name: Some(dep.dep_name.clone()),
            current_value: Some(dep.current_value.clone()),
            new_value: Some(dep.current_value.clone()),
            file_replace_position: dep.file_replace_position.unwrap(),
            ..Default::default()
        };
        let updated = maven_update_dependency(SIMPLE_POM_FULL, &upgrade);
        assert_eq!(updated.as_deref(), Some(SIMPLE_POM_FULL));
    }

    // Ported: "should return null if current versions in content and upgrade are not same" — maven/index.spec.ts line 150
    #[test]
    fn maven_index_returns_none_when_current_mismatch() {
        let deps = extract(SIMPLE_POM_FULL).unwrap();
        let dep = deps
            .iter()
            .find(|d| d.dep_name == "org.example:quuz")
            .unwrap();
        let upgrade = MavenUpdateUpgrade {
            dep_name: Some(dep.dep_name.clone()),
            current_value: Some("1.2.2".to_owned()),
            new_value: Some("1.2.4".to_owned()),
            file_replace_position: dep.file_replace_position.unwrap(),
            ..Default::default()
        };
        let updated = maven_update_dependency(SIMPLE_POM_FULL, &upgrade);
        assert!(updated.is_none());
    }

    // Ported: "should update ranges" — maven/index.spec.ts line 162
    #[test]
    fn maven_index_update_ranges() {
        let new_value = "[1.2.3]";
        let deps = extract(SIMPLE_POM_FULL).unwrap();
        let dep = deps
            .iter()
            .find(|d| d.dep_name == "org.example:hard-range")
            .unwrap();
        let upgrade = MavenUpdateUpgrade {
            dep_name: Some(dep.dep_name.clone()),
            current_value: Some(dep.current_value.clone()),
            new_value: Some(new_value.to_owned()),
            file_replace_position: dep.file_replace_position.unwrap(),
            ..Default::default()
        };
        let updated = maven_update_dependency(SIMPLE_POM_FULL, &upgrade).unwrap();
        let updated_deps = extract(&updated).unwrap();
        let updated_dep = updated_deps
            .iter()
            .find(|d| d.dep_name == "org.example:hard-range")
            .unwrap();
        assert_eq!(updated_dep.current_value, new_value);
    }

    // Ported: "should preserve ranges" — maven/index.spec.ts line 181
    #[test]
    fn maven_index_preserve_ranges() {
        let deps = extract(SIMPLE_POM_FULL).unwrap();
        let dep = deps
            .iter()
            .find(|d| d.dep_name == "org.example:hard-range")
            .unwrap();
        let upgrade = MavenUpdateUpgrade {
            dep_name: Some(dep.dep_name.clone()),
            current_value: Some(dep.current_value.clone()),
            new_value: Some("[1.0.0]".to_owned()),
            file_replace_position: dep.file_replace_position.unwrap(),
            ..Default::default()
        };
        let updated = maven_update_dependency(SIMPLE_POM_FULL, &upgrade).unwrap();
        assert_eq!(updated, SIMPLE_POM_FULL);
    }

    #[test]
    fn maven_update_at_position_basic() {
        let content = r#"<dependency>
  <groupId>com.example</groupId>
  <artifactId>lib</artifactId>
  <version>1.0.0</version>
</dependency>"#;
        let pos = content.find("1.0.0").unwrap();
        let upgrade = MavenUpdateUpgrade {
            dep_name: Some("com.example:lib".to_owned()),
            current_value: Some("1.0.0".to_owned()),
            new_value: Some("2.0.0".to_owned()),
            file_replace_position: pos,
            ..Default::default()
        };
        let updated = maven_update_at_position(content, &upgrade, "</version>").unwrap();
        assert!(updated.contains("2.0.0"));
    }

    // Ported: "should update existing dependency defined via properties" — maven/index.spec.ts line 43
    #[test]
    fn maven_index_update_property_based_dependency() {
        let parent = include_str!("../../tests/fixtures/maven/parent.pom.xml");
        let prop_pos = parent.find("1.2.3.4").unwrap();
        let upgrade = MavenUpdateUpgrade {
            dep_name: Some("org.example:quux".into()),
            current_value: Some("1.2.3.4".into()),
            new_value: Some("9.9.9.9-final".into()),
            shared_variable_name: Some("quuxVersion".into()),
            file_replace_position: prop_pos,
            ..Default::default()
        };
        let updated = maven_update_dependency(parent, &upgrade).unwrap();
        assert!(updated.contains("9.9.9.9-final"));
    }

    // Ported: "should update to version of the latest dep in implicit group" — maven/index.spec.ts line 79
    #[test]
    fn maven_index_update_implicit_group_latest_version() {
        let grouping = include_str!("../../tests/fixtures/maven/grouping.pom.xml");
        let prop_pos = grouping.find("1.0.0</foo.version>").unwrap();
        let upgrade1 = MavenUpdateUpgrade {
            dep_name: Some("org.example:foo-1".into()),
            current_value: Some("1.0.0".into()),
            new_value: Some("1.0.2".into()),
            shared_variable_name: Some("foo.version".into()),
            file_replace_position: prop_pos,
            ..Default::default()
        };
        let upgrade2 = MavenUpdateUpgrade {
            dep_name: Some("org.example:foo-2".into()),
            current_value: Some("1.0.0".into()),
            new_value: Some("1.0.3".into()),
            shared_variable_name: Some("foo.version".into()),
            file_replace_position: prop_pos,
            ..Default::default()
        };

        // Update on original content.
        let updated1 = maven_update_dependency(grouping, &upgrade1).unwrap();
        assert!(updated1.contains("<foo.version>1.0.2</foo.version>"));

        // Update on content modified outside still works because shared_variable_name bypasses
        // the version-match check.
        let updated_outside = grouping.replacen("1.0.0", "1.0.1", 1);
        let updated1_outside = maven_update_dependency(&updated_outside, &upgrade1).unwrap();
        assert!(updated1_outside.contains("<foo.version>1.0.2</foo.version>"));

        // Second update on already-updated content.
        let updated2 = maven_update_dependency(&updated1, &upgrade2).unwrap();
        assert!(updated2.contains("<foo.version>1.0.3</foo.version>"));

        // Second update on outside-modified content.
        let updated2_outside = maven_update_dependency(&updated_outside, &upgrade2).unwrap();
        assert!(updated2_outside.contains("<foo.version>1.0.3</foo.version>"));

        // Second update on original content.
        let updated2_orig = maven_update_dependency(grouping, &upgrade2).unwrap();
        assert!(updated2_orig.contains("<foo.version>1.0.3</foo.version>"));
    }

    // Ported: "should return null for ungrouped deps if content was updated outside" — maven/index.spec.ts line 135
    #[test]
    fn maven_index_returns_none_when_ungrouped_dep_updated_outside() {
        let grouping = include_str!("../../tests/fixtures/maven/grouping.pom.xml");
        let bar_pos = grouping.find("2.0.0</version>").unwrap();
        let upgrade = MavenUpdateUpgrade {
            dep_name: Some("org.example:bar".into()),
            current_value: Some("2.0.0".into()),
            new_value: Some("2.0.2".into()),
            file_replace_position: bar_pos,
            ..Default::default()
        };
        // Simulate external edit changing bar's version from 2.0.0 to 2.0.1.
        let updated_outside = grouping.replacen("2.0.0", "2.0.1", 1);
        let result = maven_update_dependency(&updated_outside, &upgrade);
        assert!(result.is_none());
    }
}
