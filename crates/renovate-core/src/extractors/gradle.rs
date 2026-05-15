//! Gradle dependency extractor.
//!
//! Handles two file formats:
//!
//! 1. **`.gradle` / `.gradle.kts`** — Groovy/Kotlin DSL build files, parsed
//!    with a regex scanner that finds string-notation dependency declarations.
//! 2. **`libs.versions.toml` / `.versions.toml`** — Gradle version catalogs
//!    (TOML format), parsed with the `toml` crate.
//!
//! All extracted deps use the Maven coordinate format `group:artifact` as the
//! dep name and are looked up via the Maven Central datasource.
//!
//! Renovate reference:
//! - `lib/modules/manager/gradle/index.ts` — `defaultConfig`, file patterns
//! - `lib/modules/manager/gradle/utils.ts`  — `parseDependencyString`
//! - `lib/modules/manager/gradle/extract/catalog.ts` — TOML catalog parsing

use std::collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

/// Which file type this dep came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradleDepSource {
    /// Extracted from a `.gradle` or `.gradle.kts` build file.
    BuildScript,
    /// Extracted from a `libs.versions.toml` / `.versions.toml` catalog.
    VersionCatalog,
}

/// Why a Gradle dep is skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GradleSkipReason {
    /// Version string contains a variable reference (`$var`, `${var}`).
    VariableReference,
    /// Version string is dynamic (`1.+`, `latest.release`, `SNAPSHOT`).
    DynamicVersion,
}

/// A single extracted Gradle dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GradleExtractedDep {
    /// Maven coordinate `group:artifact` (e.g. `com.google.guava:guava`).
    ///
    /// For `plugins {}` entries, this is `{id}:{id}.gradle.plugin` — the
    /// conventional Maven marker artifact for Gradle plugins.
    pub dep_name: String,
    /// Version string (e.g. `31.0-jre`).
    pub current_value: String,
    /// Which file format produced this dep.
    pub source: GradleDepSource,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<GradleSkipReason>,
}

/// Whether this dep came from the `plugins {}` block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradleDepKind {
    /// Normal dependency declaration (`implementation`, `api`, `classpath`, etc.).
    Dependency,
    /// Plugin declared in the `plugins {}` block (`id "..." version "..."`).
    Plugin,
}

// ── Compiled regexes ──────────────────────────────────────────────────────────

/// Matches any Gradle configuration keyword followed by a quoted dep string.
///
/// Group 1: the raw `group:artifact:version[@ext]` string.
static STRING_DEP: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?:implementation|api|compileOnly|runtimeOnly|testImplementation|testCompileOnly|testRuntimeOnly|annotationProcessor|kapt|ksp|classpath|provided|compile|runtime|testCompile|testRuntime|debugImplementation|releaseImplementation|androidTestImplementation|coreLibraryDesugaring)\s*[\(]?\s*['"]([^'"]+)['"]\s*[\)]?"#
    ).unwrap()
});

/// Matches a Gradle `plugins {}` block entry:
/// `id "plugin.id" version "X.Y.Z"` or `id("plugin.id") version "X.Y.Z"`.
///
/// Group 1: plugin ID string.
/// Group 2: version string.
static PLUGIN_DEP: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\bid\s*[\(]?\s*['"]([^'"]+)['"]\s*[\)]?\s+version\s+['"]([^'"]+)['"]"#).unwrap()
});

// ── Public API ────────────────────────────────────────────────────────────────

/// Parse a `.gradle` or `.gradle.kts` file and extract all dep declarations.
///
/// Handles both regular dependency declarations (`implementation`, `api`, etc.)
/// and `plugins {}` block entries (`id "..." version "..."`).
pub fn extract_build_file(content: &str) -> Vec<GradleExtractedDep> {
    let mut deps = Vec::new();

    for cap in STRING_DEP.captures_iter(content) {
        let raw = cap[1].trim();
        if let Some(dep) = parse_dep_string(raw, GradleDepSource::BuildScript) {
            deps.push(dep);
        }
    }

    for cap in PLUGIN_DEP.captures_iter(content) {
        let plugin_id = cap[1].trim();
        let version = cap[2].trim();
        if let Some(dep) = parse_plugin_dep(plugin_id, version) {
            deps.push(dep);
        }
    }

    // Deduplicate by dep_name, keeping the first occurrence.
    let mut seen = std::collections::HashSet::new();
    deps.retain(|d| seen.insert(d.dep_name.clone()));

    deps
}

/// Parse a `libs.versions.toml` / `.versions.toml` Gradle version catalog.
pub fn extract_version_catalog(content: &str) -> Vec<GradleExtractedDep> {
    let Ok(table) = toml::from_str::<toml::Value>(content) else {
        return Vec::new();
    };

    // Collect the `[versions]` section into a map for `version.ref` resolution.
    let versions: HashMap<String, String> = table
        .get("versions")
        .and_then(|v| v.as_table())
        .map(|t| {
            t.iter()
                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_owned())))
                .collect()
        })
        .unwrap_or_default();

    let mut deps = Vec::new();

    // Parse `[libraries]` section.
    if let Some(libs) = table.get("libraries").and_then(|v| v.as_table()) {
        for (_, entry) in libs {
            if let Some(dep) = parse_catalog_library(entry, &versions) {
                deps.push(dep);
            }
        }
    }

    // Parse `[plugins]` section.
    if let Some(plugins) = table.get("plugins").and_then(|v| v.as_table()) {
        for (_, entry) in plugins {
            if let Some(dep) = parse_catalog_plugin(entry, &versions) {
                deps.push(dep);
            }
        }
    }

    deps
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Parse a Gradle `plugins {}` block entry into a `GradleExtractedDep`.
///
/// Converts the plugin ID `org.springframework.boot` to the Maven marker
/// artifact coordinate `org.springframework.boot:org.springframework.boot.gradle.plugin`.
fn parse_plugin_dep(plugin_id: &str, version: &str) -> Option<GradleExtractedDep> {
    if plugin_id.is_empty() || version.is_empty() {
        return None;
    }
    // Plugin marker artifact: `{id}:{id}.gradle.plugin`
    let dep_name = format!("{plugin_id}:{plugin_id}.gradle.plugin");
    let skip_reason = classify_version(version);
    Some(GradleExtractedDep {
        dep_name,
        current_value: version.to_owned(),
        source: GradleDepSource::BuildScript,
        skip_reason,
    })
}

/// Parse a Maven `group:artifact:version` dependency string.
///
/// Handles:
/// - `com.google.guava:guava:31.0-jre`
/// - `junit:junit:4.13.2@jar` (strips `@jar` classifier)
fn parse_dep_string(raw: &str, source: GradleDepSource) -> Option<GradleExtractedDep> {
    // Strip `@classifier` suffix.
    let without_classifier = raw.split('@').next().unwrap_or(raw);

    let parts: Vec<&str> = without_classifier.splitn(3, ':').collect();
    if parts.len() != 3 {
        return None;
    }
    let group = parts[0].trim();
    let artifact = parts[1].trim();
    let version = parts[2].trim();

    if group.is_empty() || artifact.is_empty() {
        return None;
    }

    let dep_name = format!("{group}:{artifact}");
    let skip_reason = classify_version(version);

    Some(GradleExtractedDep {
        dep_name,
        current_value: version.to_owned(),
        source,
        skip_reason,
    })
}

/// Parse a single entry from the `[libraries]` section of a TOML catalog.
fn parse_catalog_library(
    entry: &toml::Value,
    versions: &HashMap<String, String>,
) -> Option<GradleExtractedDep> {
    match entry {
        // Inline form: `guava = "com.google.guava:guava:31.0-jre"`
        toml::Value::String(s) => parse_dep_string(s.trim(), GradleDepSource::VersionCatalog),
        // Table form: `guava = { module = "com.google.guava:guava", version = "31.0-jre" }`
        toml::Value::Table(t) => {
            let module = t.get("module").and_then(|v| v.as_str())?.trim();
            // Module is `group:artifact` (no version).
            let parts: Vec<&str> = module.splitn(2, ':').collect();
            if parts.len() != 2 {
                return None;
            }
            let dep_name = format!("{}:{}", parts[0].trim(), parts[1].trim());

            // Version can be inline or a `version.ref = "key"` reference.
            let version = if let Some(ver) = t.get("version").and_then(|v| v.as_str()) {
                ver.to_owned()
            } else if let Some(ver_ref) = t
                .get("version")
                .and_then(|v| v.as_table())
                .and_then(|vt| vt.get("ref"))
                .and_then(|r| r.as_str())
            {
                versions.get(ver_ref).cloned().unwrap_or_default()
            } else {
                return None;
            };

            if version.is_empty() {
                return None;
            }

            let skip_reason = classify_version(&version);
            Some(GradleExtractedDep {
                dep_name,
                current_value: version,
                source: GradleDepSource::VersionCatalog,
                skip_reason,
            })
        }
        _ => None,
    }
}

/// Parse a single entry from the `[plugins]` section of a TOML catalog.
///
/// Supported forms:
/// - String: `spring-boot = "org.springframework.boot:3.2.0"` (`id:version`)
/// - Table inline: `spring-boot = { id = "org.springframework.boot", version = "3.2.0" }`
/// - Table ref: `spring-boot = { id = "...", version.ref = "key" }`
fn parse_catalog_plugin(
    entry: &toml::Value,
    versions: &HashMap<String, String>,
) -> Option<GradleExtractedDep> {
    let (plugin_id, version) = match entry {
        toml::Value::String(s) => {
            // String form: "plugin.id:version"
            let parts: Vec<&str> = s.splitn(2, ':').collect();
            if parts.len() != 2 {
                return None;
            }
            (parts[0].trim().to_owned(), parts[1].trim().to_owned())
        }
        toml::Value::Table(t) => {
            let id = t.get("id").and_then(|v| v.as_str())?.trim().to_owned();
            let ver = if let Some(v) = t.get("version").and_then(|v| v.as_str()) {
                v.to_owned()
            } else if let Some(ref_key) = t
                .get("version")
                .and_then(|v| v.as_table())
                .and_then(|vt| vt.get("ref"))
                .and_then(|r| r.as_str())
            {
                versions.get(ref_key).cloned().unwrap_or_default()
            } else {
                return None;
            };
            (id, ver)
        }
        _ => return None,
    };

    if plugin_id.is_empty() || version.is_empty() {
        return None;
    }

    let dep_name = format!("{plugin_id}:{plugin_id}.gradle.plugin");
    let skip_reason = classify_version(&version);
    Some(GradleExtractedDep {
        dep_name,
        current_value: version,
        source: GradleDepSource::VersionCatalog,
        skip_reason,
    })
}

/// Determine whether a version string is dynamic or interpolated.
fn classify_version(version: &str) -> Option<GradleSkipReason> {
    if version.contains('$') || version.contains('{') {
        return Some(GradleSkipReason::VariableReference);
    }
    if version.contains('+')
        || version.eq_ignore_ascii_case("latest.release")
        || version.eq_ignore_ascii_case("latest.integration")
        || version.to_uppercase().contains("SNAPSHOT")
    {
        return Some(GradleSkipReason::DynamicVersion);
    }
    None
}

/// Extract a version-like substring from the beginning of `input`.
///
/// Mirrors `lib/modules/manager/gradle/utils.ts` `versionLikeSubstring()`.
pub fn version_like_substring(input: &str) -> Option<String> {
    use std::sync::LazyLock;
    static RE: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new(r"^(?P<version>[-_.\[\](),a-zA-Z0-9+! ]+)").unwrap());
    let cap = RE.captures(input)?;
    let version = cap.name("version")?.as_str().trim();
    if version.is_empty() || !version.chars().any(|c| c.is_ascii_digit()) {
        return None;
    }
    if !is_gradle_version_like_valid(version) {
        return None;
    }
    Some(version.to_owned())
}

/// Check whether a version-like string is valid by Gradle's versioning rules.
///
/// Simplified version of `gradleVersioning.isValid()`: rejects strings with
/// commas outside brackets that are followed by non-version content.
fn is_gradle_version_like_valid(v: &str) -> bool {
    let mut depth: i32 = 0;
    let mut last_close = v.len();
    for (i, c) in v.char_indices() {
        match c {
            '[' | '(' => depth += 1,
            ']' | ')' => {
                depth -= 1;
                if depth <= 0 {
                    last_close = i + c.len_utf8();
                }
            }
            _ => {}
        }
    }
    // If there's content after the last bracket that contains alphabetic letters
    // (not just digits/separators), reject — e.g. "[1.6.0, ]  ,  abc"
    if last_close < v.len() {
        let tail = v[last_close..].trim();
        if !tail.is_empty() && tail.chars().any(|c| c.is_ascii_alphabetic()) {
            return false;
        }
    }
    true
}

/// Return `true` if `input` is a valid Gradle dependency notation string
/// (`group:artifact:version[:classifier][@dataType]`).
///
/// Mirrors `lib/modules/manager/gradle/utils.ts` `isDependencyString()`.
pub fn is_gradle_dependency_string(input: &str) -> bool {
    use std::sync::LazyLock;
    static ARTIFACT_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(r"^[a-zA-Z][-_a-zA-Z0-9]*(?:\.[a-zA-Z0-9][-_a-zA-Z0-9]*?)*$").unwrap()
    });

    let parts: Vec<&str> = input.splitn(2, '@').collect();
    if parts.len() > 2 {
        return false;
    }
    if input.matches('@').count() > 1 {
        return false;
    }
    let dep_notation = parts[0];
    let colon_parts: Vec<&str> = dep_notation.split(':').collect();
    if colon_parts.len() != 3 && colon_parts.len() != 4 {
        return false;
    }
    let group_id = colon_parts[0];
    let artifact_id = colon_parts[1];
    let version = colon_parts[2];
    let classifier = colon_parts.get(3).copied().unwrap_or("");

    if !ARTIFACT_RE.is_match(group_id) || !ARTIFACT_RE.is_match(artifact_id) {
        return false;
    }
    if !classifier.is_empty() && !ARTIFACT_RE.is_match(classifier) {
        return false;
    }
    version_like_substring(version).as_deref() == Some(version)
}

/// Parsed Gradle dependency string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GradleParsedDep {
    pub dep_name: String,
    pub current_value: String,
    pub data_type: Option<String>,
}

/// Parse a Gradle dependency notation string.
///
/// Mirrors `lib/modules/manager/gradle/utils.ts` `parseDependencyString()`.
pub fn parse_gradle_dependency_string(input: &str) -> Option<GradleParsedDep> {
    if !is_gradle_dependency_string(input) {
        return None;
    }
    let (dep_notation, data_type) = input
        .split_once('@')
        .map(|(d, t)| (d, Some(t.to_owned())))
        .unwrap_or((input, None));
    let parts: Vec<&str> = dep_notation.split(':').collect();
    let group_id = parts[0];
    let artifact_id = parts[1];
    let current_value = parts[2].to_owned();
    Some(GradleParsedDep {
        dep_name: format!("{group_id}:{artifact_id}"),
        current_value,
        data_type,
    })
}

// ── Filetype classification ───────────────────────────────────────────────────

/// Mirrors `lib/modules/manager/gradle/utils.ts` `isGradleScriptFile()`.
pub fn is_gradle_script_file(path: &str) -> bool {
    let filename = Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();
    filename.ends_with(".gradle.kts") || filename.ends_with(".gradle")
}

/// Mirrors `lib/modules/manager/gradle/utils.ts` `isGradleVersionsFile()`.
pub fn is_gradle_versions_file(path: &str) -> bool {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?i)^versions\.gradle(?:\.kts)?$").unwrap());
    let filename = Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    RE.is_match(filename)
}

/// Mirrors `lib/modules/manager/gradle/utils.ts` `isGradleBuildFile()`.
pub fn is_gradle_build_file(path: &str) -> bool {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?i)^build\.gradle(?:\.kts)?$").unwrap());
    let filename = Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    RE.is_match(filename)
}

/// Mirrors `lib/modules/manager/gradle/utils.ts` `isGradleSettingsFile()`.
pub fn is_gradle_settings_file(path: &str) -> bool {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?i)^settings\.gradle(?:\.kts)?$").unwrap());
    let filename = Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    RE.is_match(filename)
}

/// Mirrors `lib/modules/manager/gradle/utils.ts` `isGradleDefaultCatalogFile()`.
pub fn is_gradle_default_catalog_file(path: &str) -> bool {
    path.ends_with("/gradle/libs.versions.toml")
}

/// Mirrors `lib/modules/manager/gradle/utils.ts` `isPropsFile()`.
pub fn is_props_file(path: &str) -> bool {
    let filename = Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();
    filename == "gradle.properties"
}

/// Mirrors `lib/modules/manager/gradle/utils.ts` `isKotlinSourceFile()`.
pub fn is_kotlin_source_file(path: &str) -> bool {
    let filename = Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();
    filename.ends_with(".kt")
}

/// Mirrors `lib/modules/manager/gradle/utils.ts` `isTOMLFile()`.
pub fn is_toml_file(path: &str) -> bool {
    let filename = Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();
    filename.ends_with(".toml")
}

/// Ensure `package_file` starts with exactly one `/`.
///
/// Mirrors `lib/modules/manager/gradle/utils.ts` `toAbsolutePath()`.
pub fn to_absolute_path(package_file: &str) -> String {
    let stripped = package_file.trim_start_matches(['/', '\\']);
    if stripped.is_empty() {
        return "/".to_owned();
    }
    format!("/{stripped}")
}

fn gradle_parent_dir(path: &str) -> String {
    if path == "/" {
        return "/".to_owned();
    }
    match path.rfind('/') {
        Some(0) => "/".to_owned(),
        Some(i) => path[..i].to_owned(),
        None => "/".to_owned(),
    }
}

fn get_file_rank(abs_path: &str) -> u8 {
    if is_props_file(abs_path) {
        0
    } else if is_gradle_settings_file(abs_path) {
        1
    } else if is_gradle_default_catalog_file(abs_path) {
        2
    } else if is_gradle_versions_file(abs_path) {
        3
    } else if is_gradle_build_file(abs_path) {
        5
    } else {
        4
    }
}

/// Sort Gradle package files in dependency order.
///
/// Mirrors `lib/modules/manager/gradle/utils.ts` `reorderFiles()`.
pub fn reorder_files(package_files: &[&str]) -> Vec<String> {
    struct Entry<'a> {
        path: &'a str,
        abs_path: String,
        dir: String,
        rank: u8,
    }

    let mut entries: Vec<Entry<'_>> = package_files
        .iter()
        .map(|&path| {
            let abs_path = to_absolute_path(path);
            let current_dir = gradle_parent_dir(&abs_path);
            let dir = if is_gradle_default_catalog_file(&abs_path) {
                gradle_parent_dir(&current_dir)
            } else {
                current_dir
            };
            let rank = get_file_rank(&abs_path);
            Entry {
                path,
                abs_path,
                dir,
                rank,
            }
        })
        .collect();

    entries.sort_by(|a, b| {
        if a.dir != b.dir {
            if a.dir.starts_with(&format!("{}/", b.dir)) {
                return std::cmp::Ordering::Greater;
            }
            if b.dir.starts_with(&format!("{}/", a.dir)) {
                return std::cmp::Ordering::Less;
            }
            return a.dir.cmp(&b.dir);
        }
        a.rank
            .cmp(&b.rank)
            .then_with(|| a.abs_path.cmp(&b.abs_path))
    });

    entries.into_iter().map(|e| e.path.to_owned()).collect()
}

// ── Variable registry ─────────────────────────────────────────────────────────

/// A single Gradle build variable (version reference in a properties/catalog file).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageVariable {
    pub key: String,
    pub value: String,
    pub file_replace_position: Option<u64>,
    pub package_file: Option<String>,
}

pub type PackageVariables = HashMap<String, PackageVariable>;
pub type VariableRegistry = HashMap<String, PackageVariables>;

/// Collect variables visible from `dir` by walking up to the root.
///
/// Mirrors `lib/modules/manager/gradle/utils.ts` `getVars()`.
pub fn get_vars(registry: &VariableRegistry, dir: &str) -> PackageVariables {
    let abs_dir = to_absolute_path(dir);
    let mut paths: Vec<String> = Vec::new();
    let mut current = abs_dir;
    loop {
        paths.push(current.clone());
        let parent = gradle_parent_dir(&current);
        if parent == current {
            break;
        }
        current = parent;
    }
    // Merge from root → dir so child overrides parent.
    let mut merged: PackageVariables = HashMap::new();
    for path in paths.iter().rev() {
        if let Some(vars) = registry.get(path.as_str()) {
            merged.extend(vars.iter().map(|(k, v)| (k.clone(), v.clone())));
        }
    }
    merged
}

/// Merge `new_vars` into the registry at `dir`.
///
/// Mirrors `lib/modules/manager/gradle/utils.ts` `updateVars()`.
pub fn update_vars(registry: &mut VariableRegistry, dir: &str, new_vars: PackageVariables) {
    let entry = registry.entry(dir.to_owned()).or_default();
    entry.extend(new_vars);
}

/// Register version-catalog variables under the project root with the correct
/// prefix (default `libs`, or the value of `defaultLibrariesExtensionName`).
///
/// Mirrors `lib/modules/manager/gradle/utils.ts` `updateVarsFromDefaultCatalog()`.
pub fn update_vars_from_default_catalog(
    registry: &mut VariableRegistry,
    dir: &str,
    package_file: &str,
    new_vars: PackageVariables,
) {
    let abs_pkg = to_absolute_path(package_file);
    if !is_gradle_default_catalog_file(&abs_pkg) {
        return;
    }
    let root_dir = gradle_parent_dir(&to_absolute_path(dir));
    let default_libs_ext_name = registry
        .get(&root_dir)
        .and_then(|vars| vars.get("defaultLibrariesExtensionName"))
        .and_then(|v| {
            if v.package_file
                .as_deref()
                .map(is_gradle_settings_file)
                .unwrap_or(false)
            {
                Some(v.value.clone())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "libs".to_owned());

    let remapped: PackageVariables = new_vars
        .into_iter()
        .map(|(old_key, var)| {
            let key = format!("{default_libs_ext_name}.versions.{old_key}");
            (key.clone(), PackageVariable { key, ..var })
        })
        .collect();

    let entry = registry.entry(root_dir).or_default();
    entry.extend(remapped);
}

// ── Dependency update ─────────────────────────────────────────────────────────

/// Update a Gradle dependency in file content.
///
/// Mirrors `lib/modules/manager/gradle/update.ts` `updateDependency()`.
/// Returns `None` when the file cannot be updated (wrong position, unknown version).
pub fn update_dependency(
    file_content: &str,
    offset: usize,
    current_value: &str,
    new_value: &str,
    shared_variable_name: Option<&str>,
    update_type: Option<&str>,
) -> Option<String> {
    if update_type == Some("replacement") {
        return None;
    }
    if offset > file_content.len() {
        return None;
    }
    let left_part = &file_content[..offset];
    let right_part = &file_content[offset..];
    let version = version_like_substring(right_part)?;
    let rest_part = &right_part[version.len()..];
    if version == new_value {
        return Some(file_content.to_owned());
    }
    if version == current_value || shared_variable_name.is_some() {
        return Some(format!("{left_part}{new_value}{rest_part}"));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── build file tests ──────────────────────────────────────────────────────

    // Ported: "extracts from cross-referenced files" — gradle/extract.spec.ts line 97
    #[test]
    fn extracts_implementation_single_quote() {
        let content = "implementation 'com.google.guava:guava:31.0-jre'\n";
        let deps = extract_build_file(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "com.google.guava:guava");
        assert_eq!(deps[0].current_value, "31.0-jre");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "resolves versions in build.gradle.kts" — gradle/extract.spec.ts line 125
    #[test]
    fn extracts_implementation_double_quote_parens() {
        let content = r#"implementation("com.google.guava:guava:31.0-jre")"#;
        let deps = extract_build_file(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "com.google.guava:guava");
    }

    // Ported: "ensures depType is assigned" — gradle/extract.spec.ts line 385
    #[test]
    fn extracts_multiple_configs() {
        let content = r#"
dependencies {
    implementation 'org.springframework:spring-core:5.3.28'
    testImplementation 'junit:junit:4.13.2'
    api("org.slf4j:slf4j-api:1.7.36")
    compileOnly 'org.projectlombok:lombok:1.18.24'
}
"#;
        let deps = extract_build_file(content);
        assert_eq!(deps.len(), 4);
        assert!(
            deps.iter()
                .any(|d| d.dep_name == "org.springframework:spring-core")
        );
        assert!(deps.iter().any(|d| d.dep_name == "junit:junit"));
        assert!(deps.iter().any(|d| d.dep_name == "org.slf4j:slf4j-api"));
        assert!(
            deps.iter()
                .any(|d| d.dep_name == "org.projectlombok:lombok")
        );
    }

    // Ported: "skips versions composed from multiple variables" — gradle/extract.spec.ts line 71
    #[test]
    fn skips_variable_references() {
        let content = r#"implementation "com.example:mylib:$version""#;
        let deps = extract_build_file(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(GradleSkipReason::VariableReference)
        );
    }

    // Ported: "skips versions composed from multiple variables" — gradle/extract.spec.ts line 71
    #[test]
    fn skips_dynamic_versions() {
        let content = "implementation 'org.example:mylib:1.+'\n";
        let deps = extract_build_file(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(GradleSkipReason::DynamicVersion));
    }

    // Ported: "skips versions composed from multiple variables" — gradle/extract.spec.ts line 71
    #[test]
    fn skips_snapshot_versions() {
        let content = "implementation 'com.example:mylib:1.0.0-SNAPSHOT'\n";
        let deps = extract_build_file(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(GradleSkipReason::DynamicVersion));
    }

    // Ported: "extracts from cross-referenced files" — gradle/extract.spec.ts line 97
    #[test]
    fn strips_classifier() {
        let content = "implementation 'junit:junit:4.13.2@jar'\n";
        let deps = extract_build_file(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "junit:junit");
        assert_eq!(deps[0].current_value, "4.13.2");
    }

    // Ported: "filters duplicate dependency findings" — gradle/extract.spec.ts line 341
    #[test]
    fn deduplicates_same_dep() {
        let content = r#"
implementation 'junit:junit:4.13.2'
testImplementation 'junit:junit:4.13.2'
"#;
        let deps = extract_build_file(content);
        assert_eq!(deps.len(), 1);
    }

    // Ported: "ensures depType is assigned" — gradle/extract.spec.ts line 385
    #[test]
    fn classpath_dependency() {
        let content = "classpath 'com.android.tools.build:gradle:7.4.0'\n";
        let deps = extract_build_file(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "com.android.tools.build:gradle");
    }

    // ── plugins {} block tests ────────────────────────────────────────────────

    // Ported: "ensures depType is assigned" — gradle/extract.spec.ts line 385
    #[test]
    fn plugins_block_single_quote() {
        let content = r#"
plugins {
    id 'org.springframework.boot' version '3.2.0'
}
"#;
        let deps = extract_build_file(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].dep_name,
            "org.springframework.boot:org.springframework.boot.gradle.plugin"
        );
        assert_eq!(deps[0].current_value, "3.2.0");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "resolves versions in build.gradle.kts" — gradle/extract.spec.ts line 125
    #[test]
    fn plugins_block_double_quote_parens() {
        let content = r#"
plugins {
    id("io.spring.dependency-management") version "1.1.4"
    id("org.jetbrains.kotlin.jvm") version "1.9.20"
}
"#;
        let deps = extract_build_file(content);
        assert_eq!(deps.len(), 2);
        assert!(deps.iter().any(|d| d.dep_name
            == "io.spring.dependency-management:io.spring.dependency-management.gradle.plugin"));
        assert!(deps.iter().any(|d| d.dep_name
            == "org.jetbrains.kotlin.jvm:org.jetbrains.kotlin.jvm.gradle.plugin"
            && d.current_value == "1.9.20"));
    }

    // Ported: "ensures depType is assigned" — gradle/extract.spec.ts line 385
    #[test]
    fn plugins_and_deps_in_same_file() {
        let content = r#"
plugins {
    id 'org.springframework.boot' version '3.2.0'
}

dependencies {
    implementation 'com.google.guava:guava:31.0-jre'
}
"#;
        let deps = extract_build_file(content);
        assert_eq!(deps.len(), 2);
        assert!(deps.iter().any(|d| d.dep_name.contains("springframework")));
        assert!(deps.iter().any(|d| d.dep_name == "com.google.guava:guava"));
    }

    // Ported: "skips versions composed from multiple variables" — gradle/extract.spec.ts line 71
    #[test]
    fn plugins_block_variable_version_skipped() {
        let content = r#"plugins { id 'org.example.plugin' version "$pluginVersion" }"#;
        let deps = extract_build_file(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(GradleSkipReason::VariableReference)
        );
    }

    // ── version catalog tests ─────────────────────────────────────────────────

    // Ported: "supports versions declared as single string" — gradle/extract/catalog.spec.ts line 5
    #[test]
    fn catalog_inline_string_form() {
        let content = r#"
[libraries]
guava = "com.google.guava:guava:31.0-jre"
junit = "junit:junit:4.13.2"
"#;
        let deps = extract_version_catalog(content);
        assert_eq!(deps.len(), 2);
        let guava = deps
            .iter()
            .find(|d| d.dep_name == "com.google.guava:guava")
            .unwrap();
        assert_eq!(guava.current_value, "31.0-jre");
    }

    // Ported: "supports versions declared as single string" — gradle/extract/catalog.spec.ts line 5
    #[test]
    fn catalog_table_form_inline_version() {
        let content = r#"
[libraries]
commons-io = { module = "org.apache.commons:commons-io", version = "2.11.0" }
"#;
        let deps = extract_version_catalog(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "org.apache.commons:commons-io");
        assert_eq!(deps[0].current_value, "2.11.0");
    }

    // Ported: "supports versions declared as single string" — gradle/extract/catalog.spec.ts line 5
    #[test]
    fn catalog_table_form_version_ref() {
        let content = r#"
[versions]
guava = "31.0-jre"

[libraries]
guava = { module = "com.google.guava:guava", version.ref = "guava" }
"#;
        let deps = extract_version_catalog(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "com.google.guava:guava");
        assert_eq!(deps[0].current_value, "31.0-jre");
    }

    // Ported: "supports versions declared as single string" — gradle/extract/catalog.spec.ts line 5
    #[test]
    fn catalog_plugins_section_table_version_ref() {
        let content = r#"
[versions]
kotlin = "1.9.0"

[libraries]
stdlib = { module = "org.jetbrains.kotlin:kotlin-stdlib", version.ref = "kotlin" }

[plugins]
kotlin-jvm = { id = "org.jetbrains.kotlin.jvm", version.ref = "kotlin" }
"#;
        let deps = extract_version_catalog(content);
        // Library AND plugin
        assert_eq!(deps.len(), 2);
        assert!(
            deps.iter()
                .any(|d| d.dep_name == "org.jetbrains.kotlin:kotlin-stdlib")
        );
        let plugin = deps
            .iter()
            .find(|d| d.dep_name.contains("kotlin.jvm.gradle.plugin"))
            .unwrap();
        assert_eq!(plugin.current_value, "1.9.0");
        assert_eq!(
            plugin.dep_name,
            "org.jetbrains.kotlin.jvm:org.jetbrains.kotlin.jvm.gradle.plugin"
        );
    }

    // Ported: "supports versions declared as single string" — gradle/extract/catalog.spec.ts line 5
    #[test]
    fn catalog_plugins_section_string_form() {
        let content = r#"
[plugins]
spring-boot = "org.springframework.boot:3.2.0"
"#;
        let deps = extract_version_catalog(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].dep_name,
            "org.springframework.boot:org.springframework.boot.gradle.plugin"
        );
        assert_eq!(deps[0].current_value, "3.2.0");
    }

    // Ported: "supports versions declared as single string" — gradle/extract/catalog.spec.ts line 5
    #[test]
    fn catalog_plugins_section_table_inline_version() {
        let content = r#"
[plugins]
dependency-management = { id = "io.spring.dependency-management", version = "1.1.4" }
"#;
        let deps = extract_version_catalog(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].dep_name,
            "io.spring.dependency-management:io.spring.dependency-management.gradle.plugin"
        );
        assert_eq!(deps[0].current_value, "1.1.4");
    }

    // Ported: "returns null" — gradle/extract.spec.ts line 37
    #[test]
    fn empty_returns_empty() {
        assert!(extract_build_file("").is_empty());
        assert!(extract_version_catalog("").is_empty());
    }

    // Ported: "replaces" — modules/manager/gradle/update.spec.ts line 4
    #[test]
    fn gradle_update_replaces_version() {
        let result = update_dependency("###1.2.3###", 3, "1.2.3", "1.2.4", None, None);
        assert_eq!(result.as_deref(), Some("###1.2.4###"));
    }

    // Ported: "groups" — modules/manager/gradle/update.spec.ts line 18
    #[test]
    fn gradle_update_groups_shared_variable() {
        let result = update_dependency("###1.2.4###", 3, "1.2.3", "1.2.5", Some("group"), None);
        assert_eq!(result.as_deref(), Some("###1.2.5###"));
    }

    // Ported: "returns same content" — modules/manager/gradle/update.spec.ts line 32
    #[test]
    fn gradle_update_returns_same_when_already_updated() {
        let result = update_dependency("###1.2.4###", 3, "1.2.3", "1.2.4", None, None);
        assert_eq!(result.as_deref(), Some("###1.2.4###"));
    }

    // Ported: "returns null" — modules/manager/gradle/update.spec.ts line 46
    #[test]
    fn gradle_update_returns_null_for_wrong_position() {
        // Version at offset doesn't match current_value or new_value
        let r1 = update_dependency("###1.3.0###", 3, "1.2.3", "1.2.4", None, None);
        assert!(r1.is_none());
        // Empty content
        let r2 = update_dependency("", 3, "1.2.3", "1.2.4", None, None);
        assert!(r2.is_none());
    }

    // Ported: "should return null for replacement" — modules/manager/gradle/update.spec.ts line 62
    #[test]
    fn gradle_update_returns_null_for_replacement() {
        let result = update_dependency("", 0, "", "", None, Some("replacement"));
        assert!(result.is_none());
    }

    // Ported: "extracts the actual version" — modules/manager/gradle/utils.spec.ts line 23
    #[test]
    fn gradle_version_like_substring_valid_versions() {
        let inputs = [
            "1.2.3",
            "[1.0,2.0]",
            "(,2.0[",
            "2.1.1.RELEASE",
            "1.0.+",
            "2022-05-10_55",
        ];
        let suffixes = ["", "'", "\"", "\n", "  ", "$"];
        for input in &inputs {
            for suffix in &suffixes {
                let combined = format!("{input}{suffix}");
                let result = version_like_substring(&combined);
                assert_eq!(result.as_deref(), Some(*input), "failed for {combined:?}");
            }
        }
    }

    // Ported: "returns null for invalid inputs" — modules/manager/gradle/utils.spec.ts line 41
    #[test]
    fn gradle_version_like_substring_invalid_inputs() {
        let invalid = ["", "foobar", "latest", "[1.6.0, ]  ,  abc"];
        for input in &invalid {
            let result = version_like_substring(input);
            assert!(
                result.is_none(),
                "expected None for {input:?}, got {result:?}"
            );
        }
    }

    // Ported: "$input" (isDependencyString it.each) — modules/manager/gradle/utils.spec.ts line 57
    #[test]
    fn gradle_is_dependency_string() {
        // Valid
        assert!(is_gradle_dependency_string("foo:bar:1.2.3"));
        assert!(is_gradle_dependency_string("foo.foo:bar.bar:1.2.3"));
        assert!(is_gradle_dependency_string("foo.bar:baz:1.2.3"));
        assert!(is_gradle_dependency_string(
            "foo.bar:baz:1.2.3:linux-cpu-x86_64"
        ));
        assert!(is_gradle_dependency_string("foo.bar:baz:1.2.3:sources@zip"));
        assert!(is_gradle_dependency_string("foo:bar:1.2.3@zip"));
        assert!(is_gradle_dependency_string("foo:bar:x86@x86"));
        assert!(is_gradle_dependency_string("foo.bar:baz:1.2.+"));
        assert!(is_gradle_dependency_string("foo.bar:baz:[1.6.0, ]"));
        assert!(is_gradle_dependency_string("foo.bar:baz:[, 1.6.0)"));
        assert!(is_gradle_dependency_string("foo.bar:baz:]1.6.0,]"));
        // Invalid
        assert!(!is_gradle_dependency_string("foo:bar:baz:qux"));
        assert!(!is_gradle_dependency_string("foo:bar:baz:qux:quux"));
        assert!(!is_gradle_dependency_string("foo:bar:1.2.3'"));
        assert!(!is_gradle_dependency_string("foo:bar:1.2.3\""));
        assert!(!is_gradle_dependency_string("-Xep:ParameterName:OFF"));
        assert!(!is_gradle_dependency_string("foo$bar:baz:1.2.+"));
        assert!(!is_gradle_dependency_string("scm:git:https://some.git"));
        assert!(!is_gradle_dependency_string(
            "foo.bar:baz:1.2.3:linux-cpu$-x86_64"
        ));
        assert!(!is_gradle_dependency_string("foo:bar:1.2.3@zip@foo"));
    }

    // Ported: "$input" (parseDependencyString it.each) — modules/manager/gradle/utils.spec.ts line 85
    #[test]
    fn gradle_parse_dependency_string() {
        let p = |i: &str| parse_gradle_dependency_string(i);
        assert_eq!(
            p("foo:bar:1.2.3"),
            Some(GradleParsedDep {
                dep_name: "foo:bar".into(),
                current_value: "1.2.3".into(),
                data_type: None
            })
        );
        assert_eq!(
            p("foo.foo:bar.bar:1.2.3"),
            Some(GradleParsedDep {
                dep_name: "foo.foo:bar.bar".into(),
                current_value: "1.2.3".into(),
                data_type: None
            })
        );
        assert_eq!(
            p("foo:bar:1.2.+"),
            Some(GradleParsedDep {
                dep_name: "foo:bar".into(),
                current_value: "1.2.+".into(),
                data_type: None
            })
        );
        assert_eq!(
            p("foo:bar:1.2.3@zip"),
            Some(GradleParsedDep {
                dep_name: "foo:bar".into(),
                current_value: "1.2.3".into(),
                data_type: Some("zip".into())
            })
        );
        assert_eq!(
            p("foo:bar:1.2.3:docs"),
            Some(GradleParsedDep {
                dep_name: "foo:bar".into(),
                current_value: "1.2.3".into(),
                data_type: None
            })
        );
        assert_eq!(
            p("foo:bar:1.2.3:docs@jar"),
            Some(GradleParsedDep {
                dep_name: "foo:bar".into(),
                current_value: "1.2.3".into(),
                data_type: Some("jar".into())
            })
        );
        assert_eq!(p("foo:bar:baz:qux"), None);
        assert_eq!(p("foo:bar:baz:qux:quux"), None);
        assert_eq!(p("foo:bar:1.2.3'"), None);
        assert_eq!(p("-Xep:ParameterName:OFF"), None);
    }

    // Ported: "filetype checks" — modules/manager/gradle/utils.spec.ts line 105
    #[test]
    fn gradle_filetype_checks() {
        assert!(is_gradle_script_file("/a/Somefile.gradle.kts"));
        assert!(is_gradle_script_file("/a/Somefile.gradle"));
        assert!(is_gradle_versions_file("/a/versions.gradle.kts"));
        assert!(is_gradle_settings_file("/a/settings.gradle"));
        assert!(is_gradle_settings_file("/a/settings.gradle.kts"));
        assert!(is_gradle_default_catalog_file(
            "/a/gradle/libs.versions.toml"
        ));
        assert!(is_gradle_build_file("/a/build.gradle"));
        assert!(is_props_file("/a/gradle.properties"));
        assert!(is_kotlin_source_file("/a/Somefile.kt"));
        assert!(is_toml_file("/a/Somefile.toml"));
    }

    // Ported: "reorderFiles" — modules/manager/gradle/utils.spec.ts line 120
    #[test]
    fn gradle_reorder_files_basic() {
        assert_eq!(
            reorder_files(&[
                "build.gradle",
                "a.gradle",
                "b.gradle",
                "a.gradle",
                "versions.gradle"
            ]),
            vec![
                "versions.gradle",
                "a.gradle",
                "a.gradle",
                "b.gradle",
                "build.gradle"
            ]
        );
    }

    // Ported: "reorderFiles" — modules/manager/gradle/utils.spec.ts line 127
    #[test]
    fn gradle_reorder_files_nested() {
        assert_eq!(
            reorder_files(&[
                "a/b/c/build.gradle",
                "a/b/versions.gradle",
                "a/build.gradle",
                "versions.gradle",
                "a/b/build.gradle",
                "a/versions.gradle",
                "build.gradle",
                "a/b/c/versions.gradle",
            ]),
            vec![
                "versions.gradle",
                "build.gradle",
                "a/versions.gradle",
                "a/build.gradle",
                "a/b/versions.gradle",
                "a/b/build.gradle",
                "a/b/c/versions.gradle",
                "a/b/c/build.gradle",
            ]
        );
    }

    // Ported: "reorderFiles" — modules/manager/gradle/utils.spec.ts line 148
    #[test]
    fn gradle_reorder_files_alphabetical() {
        assert_eq!(
            reorder_files(&["b.gradle", "c.gradle", "a.gradle"]),
            vec!["a.gradle", "b.gradle", "c.gradle"]
        );
        assert_eq!(
            reorder_files(&["b.gradle", "c.gradle", "a.gradle", "gradle.properties"]),
            vec!["gradle.properties", "a.gradle", "b.gradle", "c.gradle"]
        );
        assert_eq!(
            reorder_files(&[
                "b.gradle",
                "settings.gradle",
                "gradle/libs.versions.toml",
                "gradle.properties",
            ]),
            vec![
                "gradle.properties",
                "settings.gradle",
                "gradle/libs.versions.toml",
                "b.gradle",
            ]
        );
    }

    // Ported: "reorderFiles" — modules/manager/gradle/utils.spec.ts line 182
    #[test]
    fn gradle_reorder_files_independent_subfolders() {
        assert_eq!(
            reorder_files(&[
                "independent-project-in-subfolder/some.gradle",
                "build.gradle",
                "independent-project-in-subfolder/gradle/libs.versions.toml",
                "settings.gradle",
                "gradle/libs.versions.toml",
                "independent-project-in-subfolder/gradle.properties",
                "gradle.properties",
                "gradle/commonLibs.versions.toml",
                "b/another.gradle",
                "independent-project-in-subfolder/settings.gradle",
                "someothergradle.gradle",
                "z/some.gradle",
                "gradle/whatever.gradle",
                "o/build.gradle",
                "a/some.gradle",
                "o/settings.gradle",
            ]),
            vec![
                "gradle.properties",
                "settings.gradle",
                "gradle/libs.versions.toml",
                "someothergradle.gradle",
                "build.gradle",
                "a/some.gradle",
                "b/another.gradle",
                "gradle/commonLibs.versions.toml",
                "gradle/whatever.gradle",
                "independent-project-in-subfolder/gradle.properties",
                "independent-project-in-subfolder/settings.gradle",
                "independent-project-in-subfolder/gradle/libs.versions.toml",
                "independent-project-in-subfolder/some.gradle",
                "o/settings.gradle",
                "o/build.gradle",
                "z/some.gradle",
            ]
        );
    }

    // Ported: "reorderFiles" — modules/manager/gradle/utils.spec.ts line 221
    #[test]
    fn gradle_reorder_files_nested_props_and_build() {
        assert_eq!(
            reorder_files(&[
                "a/b/c/gradle.properties",
                "a/b/c/build.gradle",
                "a/build.gradle",
                "a/gradle.properties",
                "a/b/build.gradle",
                "a/b/gradle.properties",
                "build.gradle",
                "gradle.properties",
                "b.gradle",
                "c.gradle",
                "a.gradle",
            ]),
            vec![
                "gradle.properties",
                "a.gradle",
                "b.gradle",
                "c.gradle",
                "build.gradle",
                "a/gradle.properties",
                "a/build.gradle",
                "a/b/gradle.properties",
                "a/b/build.gradle",
                "a/b/c/gradle.properties",
                "a/b/c/build.gradle",
            ]
        );
    }

    // Ported: "getVars" — modules/manager/gradle/utils.spec.ts line 250
    #[test]
    fn gradle_get_vars() {
        let mut registry: VariableRegistry = HashMap::new();
        registry.insert(
            to_absolute_path("/foo"),
            [
                (
                    "foo".into(),
                    PackageVariable {
                        key: "foo".into(),
                        value: "FOO".into(),
                        file_replace_position: None,
                        package_file: None,
                    },
                ),
                (
                    "bar".into(),
                    PackageVariable {
                        key: "bar".into(),
                        value: "BAR".into(),
                        file_replace_position: None,
                        package_file: None,
                    },
                ),
                (
                    "baz".into(),
                    PackageVariable {
                        key: "baz".into(),
                        value: "BAZ".into(),
                        file_replace_position: None,
                        package_file: None,
                    },
                ),
                (
                    "qux".into(),
                    PackageVariable {
                        key: "qux".into(),
                        value: "QUX".into(),
                        file_replace_position: None,
                        package_file: None,
                    },
                ),
            ]
            .into_iter()
            .collect(),
        );
        registry.insert(
            to_absolute_path("/foo/bar"),
            [(
                "foo".into(),
                PackageVariable {
                    key: "foo".into(),
                    value: "foo".into(),
                    file_replace_position: None,
                    package_file: None,
                },
            )]
            .into_iter()
            .collect(),
        );
        registry.insert(
            to_absolute_path("/foo/bar/baz"),
            [
                (
                    "bar".into(),
                    PackageVariable {
                        key: "bar".into(),
                        value: "bar".into(),
                        file_replace_position: None,
                        package_file: None,
                    },
                ),
                (
                    "baz".into(),
                    PackageVariable {
                        key: "baz".into(),
                        value: "baz".into(),
                        file_replace_position: None,
                        package_file: None,
                    },
                ),
            ]
            .into_iter()
            .collect(),
        );
        let res = get_vars(&registry, "/foo/bar/baz/build.gradle");
        assert_eq!(res.get("foo").map(|v| v.value.as_str()), Some("foo"));
        assert_eq!(res.get("bar").map(|v| v.value.as_str()), Some("bar"));
        assert_eq!(res.get("baz").map(|v| v.value.as_str()), Some("baz"));
        assert_eq!(res.get("qux").map(|v| v.value.as_str()), Some("QUX"));
        assert_eq!(res.len(), 4);
    }

    // Ported: "empty registry" — modules/manager/gradle/utils.spec.ts line 276
    #[test]
    fn gradle_update_vars_empty_registry() {
        let mut registry: VariableRegistry = HashMap::new();
        let new_vars: PackageVariables = [(
            "qux".into(),
            PackageVariable {
                key: "qux".into(),
                value: "qux".into(),
                file_replace_position: None,
                package_file: None,
            },
        )]
        .into_iter()
        .collect();
        update_vars(&mut registry, "/foo/bar/baz", new_vars);
        assert!(registry.contains_key("/foo/bar/baz"));
        assert_eq!(registry["/foo/bar/baz"]["qux"].value, "qux");
    }

    // Ported: "updates the registry" — modules/manager/gradle/utils.spec.ts line 285
    #[test]
    fn gradle_update_vars_merges() {
        let mut registry: VariableRegistry = HashMap::new();
        registry.insert(
            to_absolute_path("/foo/bar/baz"),
            [
                (
                    "bar".into(),
                    PackageVariable {
                        key: "bar".into(),
                        value: "bar".into(),
                        file_replace_position: None,
                        package_file: None,
                    },
                ),
                (
                    "baz".into(),
                    PackageVariable {
                        key: "baz".into(),
                        value: "baz".into(),
                        file_replace_position: None,
                        package_file: None,
                    },
                ),
            ]
            .into_iter()
            .collect(),
        );
        update_vars(
            &mut registry,
            "/foo/bar/baz",
            [(
                "qux".into(),
                PackageVariable {
                    key: "qux".into(),
                    value: "qux".into(),
                    file_replace_position: None,
                    package_file: None,
                },
            )]
            .into_iter()
            .collect(),
        );
        let res = get_vars(&registry, "/foo/bar/baz/build.gradle");
        assert_eq!(res.get("bar").map(|v| v.value.as_str()), Some("bar"));
        assert_eq!(res.get("baz").map(|v| v.value.as_str()), Some("baz"));
        assert_eq!(res.get("qux").map(|v| v.value.as_str()), Some("qux"));
    }

    // Ported: "no default catalog file" — modules/manager/gradle/utils.spec.ts line 306
    #[test]
    fn gradle_update_vars_from_default_catalog_no_catalog() {
        let mut registry: VariableRegistry = HashMap::new();
        update_vars_from_default_catalog(
            &mut registry,
            "/a/gradle",
            "/a/gradle/other-catalog.toml",
            HashMap::new(),
        );
        assert!(registry.is_empty());
    }

    // Ported: "adds variables with default \"libs\" prefix" — modules/manager/gradle/utils.spec.ts line 317
    #[test]
    fn gradle_update_vars_from_default_catalog_default_prefix() {
        let mut registry: VariableRegistry = HashMap::new();
        let new_vars: PackageVariables = [
            (
                "kotlin".into(),
                PackageVariable {
                    key: "kotlin".into(),
                    value: "1.5.21".into(),
                    file_replace_position: Some(10),
                    package_file: Some("/project/gradle/libs.versions.toml".into()),
                },
            ),
            (
                "coroutines".into(),
                PackageVariable {
                    key: "coroutines".into(),
                    value: "1.5.0".into(),
                    file_replace_position: Some(40),
                    package_file: Some("/project/gradle/libs.versions.toml".into()),
                },
            ),
        ]
        .into_iter()
        .collect();
        update_vars_from_default_catalog(
            &mut registry,
            "/project/gradle",
            "/project/gradle/libs.versions.toml",
            new_vars,
        );
        let res = get_vars(&registry, "/project/build.gradle");
        assert_eq!(
            res.get("libs.versions.kotlin").map(|v| v.value.as_str()),
            Some("1.5.21")
        );
        assert_eq!(
            res.get("libs.versions.coroutines")
                .map(|v| v.value.as_str()),
            Some("1.5.0")
        );
        assert_eq!(res.len(), 2);
    }

    // Ported: "adds variables with custom libraries extension name" — modules/manager/gradle/utils.spec.ts line 357
    #[test]
    fn gradle_update_vars_from_default_catalog_custom_prefix() {
        let mut registry: VariableRegistry = HashMap::new();
        update_vars(
            &mut registry,
            "/project",
            [(
                "defaultLibrariesExtensionName".into(),
                PackageVariable {
                    key: "defaultLibrariesExtensionName".into(),
                    value: "myLibs".into(),
                    file_replace_position: Some(50),
                    package_file: Some("/project/settings.gradle".into()),
                },
            )]
            .into_iter()
            .collect(),
        );
        let new_vars: PackageVariables = [
            (
                "kotlin".into(),
                PackageVariable {
                    key: "kotlin".into(),
                    value: "1.5.21".into(),
                    file_replace_position: Some(10),
                    package_file: Some("/project/gradle/libs.versions.toml".into()),
                },
            ),
            (
                "coroutines".into(),
                PackageVariable {
                    key: "coroutines".into(),
                    value: "1.5.0".into(),
                    file_replace_position: Some(40),
                    package_file: Some("/project/gradle/libs.versions.toml".into()),
                },
            ),
        ]
        .into_iter()
        .collect();
        update_vars_from_default_catalog(
            &mut registry,
            "/project/gradle",
            "/project/gradle/libs.versions.toml",
            new_vars,
        );
        let res = get_vars(&registry, "/project/build.gradle");
        assert_eq!(
            res.get("defaultLibrariesExtensionName")
                .map(|v| v.value.as_str()),
            Some("myLibs")
        );
        assert_eq!(
            res.get("myLibs.versions.kotlin").map(|v| v.value.as_str()),
            Some("1.5.21")
        );
        assert_eq!(
            res.get("myLibs.versions.coroutines")
                .map(|v| v.value.as_str()),
            Some("1.5.0")
        );
        assert_eq!(res.len(), 3);
    }
}
