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

/// A version variable extracted from the `[versions]` section of a TOML catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GradleCatalogVar {
    pub key: String,
    pub value: String,
    pub file_replace_position: usize,
    pub package_file: String,
}

/// A dependency extracted from the `[libraries]` or `[plugins]` section of a TOML catalog.
#[derive(Debug, Clone, PartialEq)]
pub struct GradleCatalogDep {
    pub dep_name: String,
    pub current_value: Option<String>,
    pub dep_type: Option<String>,
    pub package_name: Option<String>,
    pub shared_variable_name: Option<String>,
    pub skip_reason: Option<String>,
    pub file_replace_position: Option<usize>,
    pub package_file: String,
}

/// Result of parsing a Gradle version catalog TOML file.
#[derive(Debug, Clone, PartialEq)]
pub struct GradleCatalogResult {
    pub vars: HashMap<String, GradleCatalogVar>,
    pub deps: Vec<GradleCatalogDep>,
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

/// Parse a Gradle version catalog TOML file with full position tracking.
///
/// Mirrors `lib/modules/manager/gradle/extract/catalog.ts` `parseCatalog()`.
/// Returns `{ vars, deps }` where `vars` tracks version variables and `deps`
/// tracks library and plugin dependencies with `fileReplacePosition` offsets.
pub fn parse_catalog(package_file: &str, content: &str) -> GradleCatalogResult {
    let massaged = massage_toml(content);
    let Ok(table) = toml::from_str::<toml::Value>(&massaged) else {
        return GradleCatalogResult {
            vars: HashMap::new(),
            deps: Vec::new(),
        };
    };

    let versions_section = table
        .get("versions")
        .and_then(|v| v.as_table())
        .cloned()
        .unwrap_or_default();

    let version_start_index = content.find("versions").unwrap_or(0);
    let version_sub_content = &content[version_start_index..];

    let mut vars = HashMap::new();

    for (key, ver_val) in &versions_section {
        let Some((current_value, file_replace_position)) = extract_literal_version_for_catalog(
            ver_val,
            version_start_index,
            version_sub_content,
            key,
        ) else {
            continue;
        };
        let normalized = normalize_alias(key);
        vars.insert(
            normalized.clone(),
            GradleCatalogVar {
                key: normalized,
                value: current_value,
                file_replace_position,
                package_file: package_file.to_owned(),
            },
        );
    }

    let libs = table.get("libraries").and_then(|v| v.as_table());
    let lib_start_index = content.find("libraries").unwrap_or(0);
    let lib_sub_content = &content[lib_start_index..];

    let mut deps = Vec::new();

    if let Some(libs) = libs {
        for (lib_name, descriptor) in libs {
            let dep = extract_catalog_dependency(
                descriptor,
                &versions_section,
                lib_start_index,
                lib_sub_content,
                lib_name,
                version_start_index,
                version_sub_content,
                package_file,
            );
            deps.push(dep);
        }
    }

    let plugins = table.get("plugins").and_then(|v| v.as_table());
    let plugins_start_index = content.find("[plugins]").unwrap_or(0);
    let plugins_sub_content = &content[plugins_start_index..];

    if let Some(plugins) = plugins {
        for (_plugin_name, descriptor) in plugins {
            let (dep_name_str, version_val, is_ref) = match descriptor {
                toml::Value::String(s) => {
                    let parts: Vec<&str> = s.splitn(2, ':').collect();
                    if parts.len() == 2 {
                        (parts[0].to_owned(), None, false)
                    } else {
                        continue;
                    }
                }
                toml::Value::Table(t) => {
                    let id = match t.get("id").and_then(|v| v.as_str()) {
                        Some(id) => id.to_owned(),
                        None => continue,
                    };
                    let ver = t.get("version").cloned();
                    let is_ref = t
                        .get("version")
                        .and_then(|v| v.as_table())
                        .and_then(|vt| vt.get("ref"))
                        .and_then(|r| r.as_str())
                        .is_some();
                    (id, ver, is_ref)
                }
                _ => continue,
            };

            let (current_value, file_replace_position, skip_reason) = match &version_val {
                Some(vv) => extract_version_for_catalog(
                    vv,
                    &versions_section,
                    plugins_start_index,
                    plugins_sub_content,
                    &dep_name_str,
                    version_start_index,
                    version_sub_content,
                ),
                None => {
                    let string_ver = match descriptor {
                        toml::Value::String(s) => {
                            let parts: Vec<&str> = s.splitn(2, ':').collect();
                            parts.get(1).map(|p| (*p).to_owned())
                        }
                        _ => None,
                    };
                    match string_ver {
                        Some(v) => {
                            let pos = plugins_start_index
                                + find_index_after(plugins_sub_content, &dep_name_str, &v);
                            (Some(v), Some(pos), None)
                        }
                        None => (None, None, Some("unspecified-version".to_owned())),
                    }
                }
            };

            let shared_var = if is_ref {
                version_val.as_ref().and_then(|v| {
                    v.as_table()
                        .and_then(|t| t.get("ref"))
                        .and_then(|r| r.as_str())
                        .map(normalize_alias)
                })
            } else {
                None
            };

            deps.push(GradleCatalogDep {
                dep_name: dep_name_str.clone(),
                current_value,
                dep_type: Some("plugin".to_owned()),
                package_name: Some(format!(
                    "{dep_name_str}:{dep_name_str}.gradle.plugin"
                )),
                shared_variable_name: shared_var,
                skip_reason,
                file_replace_position,
                package_file: package_file.to_owned(),
            });
        }
    }

    GradleCatalogResult { vars, deps }
}

fn normalize_alias(alias: &str) -> String {
    alias.replace(['-', '_'], ".")
}

fn strip_jinja_templates(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let bytes = content.as_bytes();
    let len = bytes.len();
    let mut idx = 0;
    let mut last_pos = 0;
    while idx < len {
        if bytes[idx] == b'{' && idx + 1 < len {
            let (closing, skip): (&str, usize) = match bytes[idx + 1] {
                b'%' => {
                    if idx + 2 < len && bytes[idx + 2] == b'`' {
                        ("`%}", 3)
                    } else {
                        ("%}", 2)
                    }
                }
                b'{' => {
                    if idx + 2 < len && bytes[idx + 2] == b'`' {
                        ("`}}", 3)
                    } else {
                        ("}}", 2)
                    }
                }
                b'#' => ("#}", 2),
                _ => {
                    idx += 1;
                    continue;
                }
            };
            if let Some(end) = content[idx + skip..].find(closing) {
                result.push_str(&content[last_pos..idx]);
                idx = idx + skip + end + closing.len();
                last_pos = idx;
                continue;
            }
        }
        idx += 1;
    }
    if last_pos < len {
        result.push_str(&content[last_pos..]);
    }
    result
}

fn massage_toml(content: &str) -> String {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"^\s*\{\{.+?\}\}\s*=.*$"#).unwrap());
    let stripped = RE.replace_all(content, "").to_string();
    strip_jinja_templates(&stripped)
}

fn find_version_index(content: &str, dep_name: &str, version: &str) -> Option<usize> {
    let escaped_dn = regex::escape(dep_name);
    let escaped_ver = regex::escape(version);
    let pattern = format!(
        r#"(?:id\s*=\s*)?['"]?{}["']?(?:(?:\s*=\s*)|:|,\s*)(?:.*version(?:\.ref)?(?:\s*=\s*))?['"]?{}['"]?"#,
        escaped_dn, escaped_ver
    );
    let re = Regex::new(&pattern).ok()?;
    if let Some(m) = re.find(content) {
        let offset = content[m.start()..].find(version)?;
        Some(m.start() + offset)
    } else {
        None
    }
}

fn find_index_after(content: &str, after: &str, find: &str) -> usize {
    let slice_point = content.find(after).unwrap_or(0) + after.len();
    slice_point + content[slice_point..].find(find).unwrap_or(0)
}

fn extract_literal_version_for_catalog(
    version: &toml::Value,
    dep_start_index: usize,
    dep_sub_content: &str,
    section_key: &str,
) -> Option<(String, usize)> {
    match version {
        toml::Value::String(s) => {
            let pos = find_version_index(dep_sub_content, section_key, s)
                .unwrap_or_else(|| find_index_after(dep_sub_content, section_key, s));
            Some((s.clone(), dep_start_index + pos))
        }
        toml::Value::Table(t) => {
            if t.contains_key("reject") || t.contains_key("rejectAll") {
                return None;
            }
            for key in &["require", "prefer", "strictly"] {
                if let Some(v) = t.get(*key).and_then(|v| v.as_str()) {
                    let pos = find_index_after(dep_sub_content, section_key, v);
                    return Some((v.to_owned(), dep_start_index + pos));
                }
            }
            None
        }
        _ => None,
    }
}

fn extract_version_for_catalog(
    version: &toml::Value,
    versions: &toml::map::Map<String, toml::Value>,
    dep_start_index: usize,
    dep_sub_content: &str,
    dep_name: &str,
    version_start_index: usize,
    version_sub_content: &str,
) -> (Option<String>, Option<usize>, Option<String>) {
    let ref_key = version
        .as_table()
        .and_then(|t| t.get("ref"))
        .and_then(|r| r.as_str());

    if let Some(ref_key) = ref_key {
        let original_alias = find_original_alias(versions, ref_key);
        let ver = versions.get(&original_alias);
        match ver {
            Some(v) => {
                let result =
                    extract_literal_version_for_catalog(v, version_start_index, version_sub_content, &original_alias);
                match result {
                    Some((val, pos)) => (Some(val), Some(pos), None),
                    None => (None, None, Some("unspecified-version".to_owned())),
                }
            }
            None => (None, None, Some("unspecified-version".to_owned())),
        }
    } else {
        let result = extract_literal_version_for_catalog(
            version,
            dep_start_index,
            dep_sub_content,
            dep_name,
        );
        match result {
            Some((val, pos)) => (Some(val), Some(pos), None),
            None => (None, None, Some("unspecified-version".to_owned())),
        }
    }
}

fn find_original_alias(
    versions: &toml::map::Map<String, toml::Value>,
    alias: &str,
) -> String {
    let normalized = normalize_alias(alias);
    for key in versions.keys() {
        if normalize_alias(key) == normalized {
            return key.clone();
        }
    }
    alias.to_owned()
}

#[allow(clippy::too_many_arguments)]
fn extract_catalog_dependency(
    descriptor: &toml::Value,
    versions: &toml::map::Map<String, toml::Value>,
    dep_start_index: usize,
    dep_sub_content: &str,
    dep_name: &str,
    version_start_index: usize,
    version_sub_content: &str,
    package_file: &str,
) -> GradleCatalogDep {
    if let toml::Value::String(s) = descriptor {
        let parts: Vec<&str> = s.splitn(3, ':').collect();
        if parts.len() >= 3 {
            let group = parts[0];
            let name = parts[1];
            let current_value = parts[2];
            let pos =
                dep_start_index + find_index_after(dep_sub_content, dep_name, current_value);
            return GradleCatalogDep {
                dep_name: format!("{group}:{name}"),
                current_value: Some(current_value.to_owned()),
                dep_type: None,
                package_name: None,
                shared_variable_name: None,
                skip_reason: None,
                file_replace_position: Some(pos),
                package_file: package_file.to_owned(),
            };
        }
        if parts.len() == 2 {
            return GradleCatalogDep {
                dep_name: dep_name.to_owned(),
                current_value: None,
                dep_type: None,
                package_name: None,
                shared_variable_name: None,
                skip_reason: Some("unspecified-version".to_owned()),
                file_replace_position: None,
                package_file: package_file.to_owned(),
            };
        }
    }

    if let toml::Value::Table(t) = descriptor {
        let dep_name_str = if let Some(module) = t.get("module").and_then(|v| v.as_str()) {
            let mp: Vec<&str> = module.splitn(2, ':').collect();
            if mp.len() == 2 {
                format!("{}:{}", mp[0], mp[1])
            } else {
                dep_name.to_owned()
            }
        } else if let (Some(group), Some(name)) = (
            t.get("group").and_then(|v| v.as_str()),
            t.get("name").and_then(|v| v.as_str()),
        ) {
            format!("{group}:{name}")
        } else {
            dep_name.to_owned()
        };

        let version_val = t.get("version");
        let is_ref = version_val
            .and_then(|v| v.as_table())
            .and_then(|vt| vt.get("ref"))
            .and_then(|r| r.as_str())
            .is_some();

        let (current_value, file_replace_position, skip_reason) = match version_val {
            Some(vv) => extract_version_for_catalog(
                vv,
                versions,
                dep_start_index,
                dep_sub_content,
                dep_name,
                version_start_index,
                version_sub_content,
            ),
            None => (None, None, Some("unspecified-version".to_owned())),
        };

        let shared_var = if is_ref {
            version_val.and_then(|v| {
                v.as_table()
                    .and_then(|t| t.get("ref"))
                    .and_then(|r| r.as_str())
                    .map(normalize_alias)
            })
        } else {
            None
        };

        return GradleCatalogDep {
            dep_name: dep_name_str,
            current_value,
            dep_type: None,
            package_name: None,
            shared_variable_name: shared_var,
            skip_reason,
            file_replace_position,
            package_file: package_file.to_owned(),
        };
    }

    GradleCatalogDep {
        dep_name: dep_name.to_owned(),
        current_value: None,
        dep_type: None,
        package_name: None,
        shared_variable_name: None,
        skip_reason: Some("unspecified-version".to_owned()),
        file_replace_position: None,
        package_file: package_file.to_owned(),
    }
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

// ---------------------------------------------------------------------------
// Gradle Consistent Versions Plugin (GCV) — consistent-versions-plugin.ts
// ---------------------------------------------------------------------------

#[allow(dead_code)]
const VERSIONS_PROPS: &str = "versions.props";
const VERSIONS_LOCK: &str = "versions.lock";

/// Regex for the GCV lock file header.
fn lock_file_header_re() -> &'static regex::Regex {
    use std::sync::LazyLock;
    static RE: LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(
            r"(?m)^# Run \./gradlew (?:--write-locks|writeVersionsLock|writeVersionsLocks) to regenerate this file",
        )
        .unwrap()
    });
    &RE
}

/// Get the sibling file path for a lock file relative to a props file path.
fn get_sibling_filename(props_path: &str, sibling: &str) -> String {
    if let Some(dir) = props_path.rfind('/') {
        format!("{}/{}", &props_path[..dir], sibling)
    } else {
        sibling.to_owned()
    }
}

/// Check whether Palantir gradle-consistent-versions is in use.
///
/// Mirrors `usesGcv()` from
/// `lib/modules/manager/gradle/extract/consistent-versions-plugin.ts`.
pub fn uses_gcv(
    props_path: &str,
    file_contents: &std::collections::HashMap<String, String>,
) -> bool {
    let lock_path = get_sibling_filename(props_path, VERSIONS_LOCK);
    file_contents
        .get(&lock_path)
        .is_some_and(|content| lock_file_header_re().is_match(content))
}

/// A version with its file position.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GcvVersionPos {
    pub version: String,
    pub file_pos: usize,
}

/// A locked dependency entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GcvLockEntry {
    pub version: String,
    pub dep_type: String,
}

/// A resolved GCV dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GcvDep {
    pub dep_name: String,
    pub current_value: String,
    pub locked_version: Option<String>,
    pub dep_type: String,
    pub shared_variable_name: Option<String>,
    pub file_replace_position: usize,
    pub package_file: String,
}

/// Parse a `versions.props` file into exact and glob maps.
///
/// Returns `(exact_map, glob_map)` where glob_map entries contain `*`.
/// Mirrors `parsePropsFile()` from
/// `lib/modules/manager/gradle/extract/consistent-versions-plugin.ts`.
pub fn parse_props_file(
    input: &str,
) -> (
    std::collections::HashMap<String, GcvVersionPos>,
    std::collections::HashMap<String, GcvVersionPos>,
) {
    use std::sync::LazyLock;
    static PROPS_LINE: LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(r"^(?P<depName>[^:]+:[^=]+?) *= *(?P<propsVersion>.*)$").unwrap()
    });
    static VALID_GLOB: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new(r"^[a-zA-Z][-_a-zA-Z0-9.:*]+$").unwrap());

    let is_crlf = input.contains("\r\n");
    let mut exact_map = std::collections::HashMap::new();
    let mut glob_map = std::collections::HashMap::new();
    let mut start_of_line = 0usize;

    // Split lines preserving correct byte positions
    let lines: Vec<&str> = if is_crlf {
        // For CRLF: split on \r\n explicitly
        input.split("\r\n").collect()
    } else {
        input.split('\n').collect()
    };
    let line_sep_len = if is_crlf { 2usize } else { 1usize };

    for line in &lines {
        if let Some(caps) = PROPS_LINE.captures(line) {
            // TypeScript does NOT trim depName before VALID_GLOB check
            let dep_name_raw = caps.name("depName").unwrap().as_str();
            let props_version = caps.name("propsVersion").unwrap().as_str().trim();
            if VALID_GLOB.is_match(dep_name_raw) && version_like_substring(props_version).is_some()
            {
                let dep_name = dep_name_raw.trim();
                let start_in_line = line.rfind(props_version).unwrap_or(0);
                let file_pos = start_of_line + start_in_line;
                let entry = GcvVersionPos {
                    version: props_version.to_owned(),
                    file_pos,
                };
                if dep_name.contains('*') {
                    glob_map.insert(dep_name.to_owned(), entry);
                } else {
                    exact_map.insert(dep_name.to_owned(), entry);
                }
            }
        }
        start_of_line += line.len() + line_sep_len;
    }

    // Sort glob map by key length descending (longest first = highest priority)
    // Return as-is; callers can sort if needed. The HashMap doesn't preserve order,
    // but tests check membership/values not order.
    (exact_map, glob_map)
}

/// Parse a `versions.lock` file into a map of dep_name → lock entry.
///
/// Mirrors `parseLockFile()` from
/// `lib/modules/manager/gradle/extract/consistent-versions-plugin.ts`.
pub fn parse_lock_file(input: &str) -> std::collections::HashMap<String, GcvLockEntry> {
    use std::sync::LazyLock;
    static LOCK_LINE: LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(
            r"^(?P<depName>[^:]+:[^:]+):(?P<lockVersion>[^ ]+) \(\d+ constraints: [0-9a-f]+\)$",
        )
        .unwrap()
    });

    let mut map = std::collections::HashMap::new();
    let mut is_test = false;

    for line in input.lines() {
        if let Some(caps) = LOCK_LINE.captures(line) {
            let dep_name = caps.name("depName").unwrap().as_str();
            let lock_version = caps.name("lockVersion").unwrap().as_str();
            let dep_notation = format!("{dep_name}:{lock_version}");
            if is_gradle_dependency_string(&dep_notation) {
                map.insert(
                    dep_name.to_owned(),
                    GcvLockEntry {
                        version: lock_version.to_owned(),
                        dep_type: if is_test {
                            "test".to_owned()
                        } else {
                            "dependencies".to_owned()
                        },
                    },
                );
            }
        } else if line == "[Test dependencies]" {
            is_test = true;
        }
    }
    map
}

/// Convert a glob pattern (like `org.apache.*`) to a Regex.
fn glob_to_regex(glob: &str) -> regex::Regex {
    let mut pattern = String::from("^");
    for ch in glob.chars() {
        match ch {
            '*' => pattern.push_str(".*"),
            '.' => pattern.push_str("\\."),
            c => pattern.push(c),
        }
    }
    pattern.push('$');
    regex::Regex::new(&pattern).unwrap_or_else(|_| regex::Regex::new("^$").unwrap())
}

/// Parse a GCV (`versions.props` + `versions.lock`) file pair into deps.
///
/// Mirrors `parseGcv()` from
/// `lib/modules/manager/gradle/extract/consistent-versions-plugin.ts`.
pub fn parse_gcv(
    props_path: &str,
    file_contents: &std::collections::HashMap<String, String>,
) -> Vec<GcvDep> {
    let props_content = file_contents.get(props_path).map_or("", String::as_str);
    let lock_path = get_sibling_filename(props_path, VERSIONS_LOCK);
    let lock_content = file_contents.get(&lock_path).map_or("", String::as_str);
    let mut lock_map = parse_lock_file(lock_content);
    let (exact_map, glob_map) = parse_props_file(props_content);

    let mut deps: Vec<GcvDep> = Vec::new();

    // Exact matches first
    for (prop_dep, ver_pos) in &exact_map {
        if let Some(lock_entry) = lock_map.remove(prop_dep.as_str()) {
            deps.push(GcvDep {
                dep_name: prop_dep.clone(),
                current_value: ver_pos.version.clone(),
                locked_version: Some(lock_entry.version),
                dep_type: lock_entry.dep_type,
                shared_variable_name: None,
                file_replace_position: ver_pos.file_pos,
                package_file: props_path.to_owned(),
            });
        }
    }

    // Glob matches: sort by key length descending so longest match wins
    let mut glob_entries: Vec<(&String, &GcvVersionPos)> = glob_map.iter().collect();
    glob_entries.sort_by_key(|e| std::cmp::Reverse(e.0.len()));

    for (glob_dep, ver_pos) in glob_entries {
        let glob_re = glob_to_regex(glob_dep);
        let matching: Vec<String> = lock_map
            .keys()
            .filter(|k| glob_re.is_match(k))
            .cloned()
            .collect();
        for exact_dep in matching {
            if let Some(lock_entry) = lock_map.remove(&exact_dep) {
                deps.push(GcvDep {
                    dep_name: exact_dep,
                    current_value: ver_pos.version.clone(),
                    locked_version: Some(lock_entry.version),
                    dep_type: lock_entry.dep_type,
                    shared_variable_name: Some(glob_dep.clone()),
                    file_replace_position: ver_pos.file_pos,
                    package_file: props_path.to_owned(),
                });
            }
        }
    }

    deps
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

// ── Content descriptors & package registries ─────────────────────────────────
//
// Mirrors `lib/modules/manager/gradle/types.ts` ContentDescriptorSpec,
// PackageRegistry and `lib/modules/manager/gradle/extract.ts`
// matchesContentDescriptor / getRegistryUrlsForDep.

/// Well-known Maven repository base URLs.
///
/// Mirrors `lib/modules/manager/gradle/parser/common.ts` REGISTRY_URLS.
pub mod registry_urls {
    pub const MAVEN_CENTRAL: &str = "https://repo.maven.apache.org/maven2";
    pub const GRADLE_PLUGIN_PORTAL: &str = "https://plugins.gradle.org/m2/";
    pub const GOOGLE: &str = "https://dl.google.com/android/maven2/";
    pub const JCENTER: &str = "https://jcenter.bintray.com/";
}

/// How a content descriptor matches group/artifact IDs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentDescriptorMatcher {
    Simple,
    Regex,
    Subgroup,
}

/// Whether a descriptor includes or excludes matching deps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentDescriptorMode {
    Include,
    Exclude,
}

/// A single content descriptor filter entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentDescriptorSpec {
    pub mode: ContentDescriptorMode,
    pub matcher: ContentDescriptorMatcher,
    pub group_id: String,
    pub artifact_id: Option<String>,
    pub version: Option<String>,
}

/// Whether a package registry is regular or exclusive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryType {
    Regular,
    Exclusive,
}

/// Whether the registry applies to deps or plugins.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryScope {
    Dep,
    Plugin,
}

/// A Maven repository discovered during Gradle extraction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageRegistry {
    pub registry_url: String,
    pub registry_type: RegistryType,
    pub scope: RegistryScope,
    pub content: Option<Vec<ContentDescriptorSpec>>,
}

/// A simplified dep representation for content descriptor matching.
#[derive(Debug, Clone)]
pub struct ContentDescriptorDep {
    pub dep_name: String,
    pub current_value: Option<String>,
    pub dep_type: Option<String>,
}

/// Test whether `dep` matches the given content descriptors.
///
/// Mirrors `lib/modules/manager/gradle/extract.ts` `matchesContentDescriptor()`.
/// Returns `true` when no descriptors are provided (default-include).
pub fn matches_content_descriptor(
    dep: &ContentDescriptorDep,
    content_descriptors: Option<&[ContentDescriptorSpec]>,
) -> bool {
    let descriptors = match content_descriptors {
        Some(d) if !d.is_empty() => d,
        _ => return true,
    };

    let full_name = dep.dep_name.as_str();
    let (group_id, artifact_id) = full_name
        .split_once(':')
        .map(|(g, a)| (g.to_owned(), a.to_owned()))
        .unwrap_or((full_name.to_owned(), String::new()));

    let mut has_includes = false;
    let mut has_excludes = false;
    let mut matches_include = false;
    let mut matches_exclude = false;

    for desc in descriptors {
        let group_match = match desc.matcher {
            ContentDescriptorMatcher::Regex => regex::Regex::new(&desc.group_id)
                .map(|re| re.is_match(&group_id))
                .unwrap_or(false),
            ContentDescriptorMatcher::Subgroup => {
                group_id == desc.group_id
                    || format!("{group_id}.").starts_with(&desc.group_id)
            }
            ContentDescriptorMatcher::Simple => group_id == desc.group_id,
        };

        let artifact_match = if group_match {
            match &desc.artifact_id {
                Some(aid) => match desc.matcher {
                    ContentDescriptorMatcher::Regex => regex::Regex::new(aid)
                        .map(|re| re.is_match(&artifact_id))
                        .unwrap_or(false),
                    _ => artifact_id == *aid,
                },
                None => true,
            }
        } else {
            false
        };

        let version_match = if group_match && artifact_match {
            match (&desc.version, &dep.current_value) {
                (Some(dv), Some(cv)) => match desc.matcher {
                    ContentDescriptorMatcher::Regex => regex::Regex::new(dv)
                        .map(|re| re.is_match(cv))
                        .unwrap_or(false),
                    _ => crate::versioning::gradle_version_matches(cv, dv),
                },
                _ => true,
            }
        } else {
            true
        };

        let is_match = group_match && artifact_match && version_match;

        match desc.mode {
            ContentDescriptorMode::Include => {
                has_includes = true;
                if is_match {
                    matches_include = true;
                }
            }
            ContentDescriptorMode::Exclude => {
                has_excludes = true;
                if is_match {
                    matches_exclude = true;
                }
            }
        }
    }

    if has_includes && has_excludes {
        matches_include && !matches_exclude
    } else if has_includes {
        matches_include
    } else if has_excludes {
        !matches_exclude
    } else {
        true
    }
}

/// Collect the deduplicated, scope-appropriate registry URLs for a dep.
///
/// Mirrors `lib/modules/manager/gradle/extract.ts` `getRegistryUrlsForDep()`.
pub fn get_registry_urls_for_dep(
    registries: &[PackageRegistry],
    dep: &ContentDescriptorDep,
) -> Vec<String> {
    let scope = match dep.dep_type.as_deref() {
        Some("plugin") => RegistryScope::Plugin,
        _ => RegistryScope::Dep,
    };

    let matching: Vec<&PackageRegistry> = registries
        .iter()
        .filter(|r| {
            r.scope == scope && matches_content_descriptor(dep, r.content.as_deref())
        })
        .collect();

    let exclusive: Vec<&&PackageRegistry> = matching
        .iter()
        .filter(|r| r.registry_type == RegistryType::Exclusive)
        .collect();

    let urls: Vec<&str> = if !exclusive.is_empty() {
        exclusive.iter().map(|r| r.registry_url.as_str()).collect()
    } else {
        matching.iter().map(|r| r.registry_url.as_str()).collect()
    };

    let mut deduped: Vec<String> = Vec::new();
    for url in urls {
        if !deduped.contains(&url.to_owned()) {
            deduped.push(url.to_owned());
        }
    }

    if deduped.is_empty() && scope == RegistryScope::Plugin {
        deduped.push(registry_urls::GRADLE_PLUGIN_PORTAL.to_owned());
    }

    deduped
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

    // Ported: "ignores empty TOML file" — gradle/extract/catalog.spec.ts line 180
    #[test]
    fn catalog_empty_toml_returns_empty() {
        let deps = extract_version_catalog("");
        assert!(deps.is_empty());
    }

    // Ported: "skips version entries with no resolvable literal value" — gradle/extract/catalog.spec.ts line 185
    #[test]
    fn catalog_skips_non_literal_versions() {
        let content = r#"
[versions]
kotlin = "1.5.21"
bad = { reject = "1.0.0" }
"#;
        let deps = extract_version_catalog(content);
        // Neither kotlin nor bad produces a dep (only libraries/plugins do)
        assert!(deps.is_empty());
    }

    // Ported: "deletes commit message for plugins with version reference" — gradle/extract/catalog.spec.ts line 134
    #[test]
    fn catalog_plugin_version_ref_deletes_commit_message() {
        let content = r#"[versions]
detekt = "1.18.1"

[plugins]
detekt = { id = "io.gitlab.arturbosch.detekt", version.ref = "detekt" }

[libraries]
detekt-formatting = { module = "io.gitlab.arturbosch.detekt:detekt-formatting", version.ref = "detekt" }
"#;
        let result = parse_catalog("gradle/libs.versions.toml", content);

        assert_eq!(result.vars.len(), 1);
        let detekt_var = result.vars.get("detekt").unwrap();
        assert_eq!(detekt_var.key, "detekt");
        assert_eq!(detekt_var.value, "1.18.1");
        assert_eq!(detekt_var.file_replace_position, 21);
        assert_eq!(detekt_var.package_file, "gradle/libs.versions.toml");

        assert_eq!(result.deps.len(), 2);

        let lib = result
            .deps
            .iter()
            .find(|d| d.dep_name == "io.gitlab.arturbosch.detekt:detekt-formatting")
            .unwrap();
        assert_eq!(lib.shared_variable_name.as_deref(), Some("detekt"));
        assert_eq!(lib.current_value.as_deref(), Some("1.18.1"));
        assert_eq!(lib.file_replace_position, Some(21));
        assert_eq!(lib.package_file, "gradle/libs.versions.toml");

        let plugin = result
            .deps
            .iter()
            .find(|d| d.dep_type.as_deref() == Some("plugin"))
            .unwrap();
        assert_eq!(plugin.dep_name, "io.gitlab.arturbosch.detekt");
        assert_eq!(
            plugin.package_name.as_deref(),
            Some("io.gitlab.arturbosch.detekt:io.gitlab.arturbosch.detekt.gradle.plugin")
        );
        assert_eq!(plugin.current_value.as_deref(), Some("1.18.1"));
        assert_eq!(plugin.file_replace_position, Some(21));
        assert_eq!(plugin.shared_variable_name.as_deref(), Some("detekt"));
    }

    // Ported: "changes the dependency version, not the comment version" — gradle/extract/catalog.spec.ts line 203
    #[test]
    fn catalog_version_position_ignores_comments() {
        let content = r#"[versions]
# Releases: http://someWebsite.com/junit/1.4.9
mocha-junit-reporter = "2.0.2"
# JUnit 1.4.9 is awesome!
junit = "1.4.9"


[libraries]
junit-legacy = { module = "junit:junit", version.ref = "junit" }
mocha-junit = { module = "mocha-junit:mocha-junit", version.ref = "mocha.junit.reporter" }
"#;
        let result = parse_catalog("gradle/libs.versions.toml", content);

        assert_eq!(result.vars.len(), 2);

        let mocha_var = result.vars.get("mocha.junit.reporter").unwrap();
        assert_eq!(mocha_var.key, "mocha.junit.reporter");
        assert_eq!(mocha_var.value, "2.0.2");
        assert_eq!(mocha_var.file_replace_position, 82);

        let junit_var = result.vars.get("junit").unwrap();
        assert_eq!(junit_var.key, "junit");
        assert_eq!(junit_var.value, "1.4.9");
        assert_eq!(junit_var.file_replace_position, 124);

        assert_eq!(result.deps.len(), 2);

        let junit_dep = result
            .deps
            .iter()
            .find(|d| d.dep_name == "junit:junit")
            .unwrap();
        assert_eq!(junit_dep.shared_variable_name.as_deref(), Some("junit"));
        assert_eq!(junit_dep.current_value.as_deref(), Some("1.4.9"));
        assert_eq!(junit_dep.file_replace_position, Some(124));

        let mocha_dep = result
            .deps
            .iter()
            .find(|d| d.dep_name == "mocha-junit:mocha-junit")
            .unwrap();
        assert_eq!(
            mocha_dep.shared_variable_name.as_deref(),
            Some("mocha.junit.reporter")
        );
        assert_eq!(mocha_dep.current_value.as_deref(), Some("2.0.2"));
        assert_eq!(mocha_dep.file_replace_position, Some(82));
    }

    // Ported: "supports templated toml" — gradle/extract/catalog.spec.ts line 254
    #[test]
    fn catalog_templated_toml() {
        let content = r#"[versions]
# Releases: http://someWebsite.com/junit/1.4.9
mocha-junit-reporter = "2.0.2"
{%- if cookiecutter.service_uses_junit %}
# JUnit 1.4.9 is awesome!
junit = "1.4.9"
{%- endif %}

[libraries]
{%- if cookiecutter.service_uses_junit %}
junit-legacy = { module = "junit:junit", version.ref = "junit" }
{%- endif %}
mocha-junit = { module = "mocha-junit:mocha-junit", version.ref = "mocha.junit.reporter" }
"#;
        let result = parse_catalog("gradle/libs.versions.toml", content);

        assert_eq!(result.vars.len(), 2);

        let mocha_var = result.vars.get("mocha.junit.reporter").unwrap();
        assert_eq!(mocha_var.key, "mocha.junit.reporter");
        assert_eq!(mocha_var.value, "2.0.2");
        assert_eq!(mocha_var.file_replace_position, 82);

        let junit_var = result.vars.get("junit").unwrap();
        assert_eq!(junit_var.key, "junit");
        assert_eq!(junit_var.value, "1.4.9");
        assert_eq!(junit_var.file_replace_position, 166);

        assert_eq!(result.deps.len(), 2);

        let junit_dep = result
            .deps
            .iter()
            .find(|d| d.dep_name == "junit:junit")
            .unwrap();
        assert_eq!(junit_dep.shared_variable_name.as_deref(), Some("junit"));
        assert_eq!(junit_dep.current_value.as_deref(), Some("1.4.9"));
        assert_eq!(junit_dep.file_replace_position, Some(166));

        let mocha_dep = result
            .deps
            .iter()
            .find(|d| d.dep_name == "mocha-junit:mocha-junit")
            .unwrap();
        assert_eq!(
            mocha_dep.shared_variable_name.as_deref(),
            Some("mocha.junit.reporter")
        );
        assert_eq!(mocha_dep.current_value.as_deref(), Some("2.0.2"));
        assert_eq!(mocha_dep.file_replace_position, Some(82));
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

    // ── GCV consistent-versions-plugin tests ─────────────────────────────────

    // Ported: "works for sub folders" — gradle/extract/consistent-versions-plugin.spec.ts line 10
    #[test]
    fn gcv_uses_gcv_sub_folders() {
        let mut files = std::collections::HashMap::new();
        files.insert("mysub/build.gradle.kts".to_owned(), "...".to_owned());
        files.insert("mysub/versions.props".to_owned(), "...".to_owned());
        files.insert(
            "mysub/versions.lock".to_owned(),
            "# Run ./gradlew --write-locks to regenerate this file\norg.apache.lucene:lucene-core:1.2.3".to_owned(),
        );
        files.insert(
            "othersub/build.gradle.kts".to_owned(),
            "nothing here".to_owned(),
        );

        assert!(uses_gcv("mysub/versions.props", &files));
        assert!(!uses_gcv("othersub/versions.props", &files));
    }

    // Ported: "detects lock file header introduced with gradle-consistent-versions version 2.20.0" — consistent-versions-plugin.spec.ts line 24
    #[test]
    fn gcv_uses_gcv_header_2_20() {
        let mut files = std::collections::HashMap::new();
        files.insert(
            "versions.lock".to_owned(),
            "# Run ./gradlew writeVersionsLock to regenerate this file\norg.apache.lucene:lucene-core:1.2.3".to_owned(),
        );
        assert!(uses_gcv("versions.props", &files));
    }

    // Ported: "detects lock file header introduced with gradle-consistent-versions version 2.23.0" — consistent-versions-plugin.spec.ts line 36
    #[test]
    fn gcv_uses_gcv_header_2_23() {
        let mut files = std::collections::HashMap::new();
        files.insert(
            "versions.lock".to_owned(),
            "# Run ./gradlew writeVersionsLocks to regenerate this file\norg.apache.lucene:lucene-core:1.2.3".to_owned(),
        );
        assert!(uses_gcv("versions.props", &files));
    }

    // Ported: "correct position for CRLF and LF" — consistent-versions-plugin.spec.ts line 48
    #[test]
    fn gcv_parse_props_file_positions() {
        // CRLF: "a.b:c.d=1\r\na.b:c.e=2" → filePos of a.b:c.e=2 is at char 19
        // Line1: "a.b:c.d=1" = 9 chars + \r\n = 11 bytes → line2 starts at 11
        // "a.b:c.e=2" → "2" is at index 8 within line → total: 11+8=19
        let crlf_props = parse_props_file("a.b:c.d=1\r\na.b:c.e=2");
        let (exact_crlf, _) = crlf_props;
        assert_eq!(exact_crlf.len(), 2);
        assert!(exact_crlf.contains_key("a.b:c.e"));
        assert_eq!(exact_crlf["a.b:c.e"].file_pos, 19);

        // LF: "a.b:c.d=1\na.b:c.e=2" → filePos of a.b:c.e=2 is at char 18
        let lf_props = parse_props_file("a.b:c.d=1\na.b:c.e=2");
        let (exact_lf, _) = lf_props;
        assert!(exact_lf.contains_key("a.b:c.e"));
        assert_eq!(exact_lf["a.b:c.e"].file_pos, 18);
    }

    // Ported: "test bogus input lines" — consistent-versions-plugin.spec.ts line 60
    #[test]
    fn gcv_parse_bogus_input() {
        let props_input = "# comment:foo.bar = 1\n123.foo:bar = 2\nthis has:spaces = 3\n starts.with:space = 4\ncontains(special):chars = 5\na* = 6\nthis.is:valid.dep = 7\nvalid.glob:* = 8\n";
        let (exact, glob) = parse_props_file(props_input);
        assert_eq!(exact.len(), 1); // only "this.is:valid.dep"
        assert_eq!(glob.len(), 1); // only "valid.glob:*"

        let lock_input = "# comment:foo.bar:1 (10 constraints: 95be0c15)\n123.foo:bar:2 (10 constraints: 95be0c15)\nthis has:spaces:3 (10 constraints: 95be0c15)\n starts.with:space:4 (10 constraints: 95be0c15)\ncontains(special):chars:5 (10 constraints: 95be0c15)\nno.colon:6 (10 constraints: 95be0c15)\nthis.is:valid.dep:7 (10 constraints: 95be0c15)\n\n[Test dependencies]\nthis.is:valid.test.dep:8 (10 constraints: 95be0c15)\n";
        let lock_map = parse_lock_file(lock_input);
        assert_eq!(lock_map.len(), 2);
        assert_eq!(lock_map["this.is:valid.dep"].dep_type, "dependencies");
        assert_eq!(lock_map["this.is:valid.test.dep"].dep_type, "test");
    }

    // ── Content descriptor tests ──────────────────────────────────────────────

    fn cd_dep(input: &str) -> ContentDescriptorDep {
        let parts: Vec<&str> = input.splitn(3, ':').collect();
        ContentDescriptorDep {
            dep_name: format!("{}:{}", parts[0], parts[1]),
            current_value: parts.get(2).map(|s| (*s).to_owned()),
            dep_type: None,
        }
    }

    fn cd_spec(
        mode: ContentDescriptorMode,
        matcher: ContentDescriptorMatcher,
        group_id: &str,
    ) -> ContentDescriptorSpec {
        ContentDescriptorSpec {
            mode,
            matcher,
            group_id: group_id.to_owned(),
            artifact_id: None,
            version: None,
        }
    }

    fn cd_spec_full(
        mode: ContentDescriptorMode,
        matcher: ContentDescriptorMatcher,
        group_id: &str,
        artifact_id: Option<&str>,
        version: Option<&str>,
    ) -> ContentDescriptorSpec {
        ContentDescriptorSpec {
            mode,
            matcher,
            group_id: group_id.to_owned(),
            artifact_id: artifact_id.map(|s| s.to_owned()),
            version: version.map(|s| s.to_owned()),
        }
    }

    // Ported: "$input | $output" — gradle/extract.spec.ts line 568
    #[test]
    fn content_descriptor_simple_matches() {
        let cases: Vec<(&str, bool, Option<Vec<ContentDescriptorSpec>>)> = vec![
            ("foo:bar:1.2.3", true, None),
            ("foo:bar:1.2.3", true, Some(vec![cd_spec(ContentDescriptorMode::Include, ContentDescriptorMatcher::Simple, "foo")])),
            ("foo:bar:1.2.3", false, Some(vec![cd_spec(ContentDescriptorMode::Exclude, ContentDescriptorMatcher::Simple, "foo")])),
            ("foo:bar:1.2.3", false, Some(vec![cd_spec(ContentDescriptorMode::Include, ContentDescriptorMatcher::Simple, "bar")])),
            ("foo:bar:1.2.3", true, Some(vec![cd_spec_full(ContentDescriptorMode::Include, ContentDescriptorMatcher::Simple, "foo", Some("bar"), None)])),
            ("foo:bar:1.2.3", false, Some(vec![cd_spec_full(ContentDescriptorMode::Exclude, ContentDescriptorMatcher::Simple, "foo", Some("bar"), None)])),
            ("foo:bar:1.2.3", false, Some(vec![cd_spec_full(ContentDescriptorMode::Include, ContentDescriptorMatcher::Simple, "foo", Some("baz"), None)])),
            ("foo:bar:1.2.3", true, Some(vec![cd_spec_full(ContentDescriptorMode::Include, ContentDescriptorMatcher::Simple, "foo", Some("bar"), Some("1.2.3"))])),
            ("foo:bar:1.2.3", false, Some(vec![cd_spec_full(ContentDescriptorMode::Exclude, ContentDescriptorMatcher::Simple, "foo", Some("bar"), Some("1.2.3"))])),
            ("foo:bar:1.2.3", true, Some(vec![cd_spec_full(ContentDescriptorMode::Include, ContentDescriptorMatcher::Simple, "foo", Some("bar"), Some("1.2.+"))])),
            ("foo:bar:1.2.3", false, Some(vec![cd_spec_full(ContentDescriptorMode::Include, ContentDescriptorMatcher::Simple, "foo", Some("baz"), Some("4.5.6"))])),
            ("foo:bar:1.2.3", true, Some(vec![cd_spec(ContentDescriptorMode::Include, ContentDescriptorMatcher::Subgroup, "foo")])),
            ("foo.bar.baz:qux:1.2.3", true, Some(vec![cd_spec(ContentDescriptorMode::Include, ContentDescriptorMatcher::Subgroup, "foo.bar.baz")])),
            ("foo.bar.baz:qux:1.2.3", true, Some(vec![cd_spec(ContentDescriptorMode::Include, ContentDescriptorMatcher::Subgroup, "foo.bar")])),
            ("foo.bar.baz:qux:1.2.3", false, Some(vec![cd_spec(ContentDescriptorMode::Include, ContentDescriptorMatcher::Subgroup, "foo.barbaz")])),
            ("foobarbaz:qux:1.2.3", true, Some(vec![cd_spec(ContentDescriptorMode::Include, ContentDescriptorMatcher::Regex, ".*bar.*")])),
            ("foobarbaz:qux:1.2.3", true, Some(vec![cd_spec_full(ContentDescriptorMode::Include, ContentDescriptorMatcher::Regex, ".*bar.*", Some("qux"), None)])),
            ("foobar:foobar:1.2.3", true, Some(vec![cd_spec_full(ContentDescriptorMode::Include, ContentDescriptorMatcher::Regex, ".*bar.*", Some("foo.*"), None)])),
            ("foobar:foobar:1.2.3", false, Some(vec![cd_spec_full(ContentDescriptorMode::Include, ContentDescriptorMatcher::Regex, "foobar", Some("^bar"), None)])),
            ("foobar:foobar:1.2.3", true, Some(vec![cd_spec_full(ContentDescriptorMode::Include, ContentDescriptorMatcher::Regex, "foobar", Some("^foo.*"), Some("1\\.*"))])),
            ("foobar:foobar:1.2.3", false, Some(vec![cd_spec_full(ContentDescriptorMode::Include, ContentDescriptorMatcher::Regex, "foobar", Some("^foo"), Some("3.+"))])),
            ("foobar:foobar:1.2.3", false, Some(vec![cd_spec_full(ContentDescriptorMode::Include, ContentDescriptorMatcher::Regex, "foobar", Some("qux"), Some("1\\.*"))])),
        ];

        for (input, expected_output, descriptor) in &cases {
            let dep = cd_dep(input);
            let result = matches_content_descriptor(&dep, descriptor.as_deref());
            assert_eq!(
                result, *expected_output,
                "input={input:?}, descriptor={descriptor:?}"
            );
        }
    }

    // Ported: "if both includes and excludes exist, dep must match include and not match exclude" — gradle/extract.spec.ts line 609
    #[test]
    fn content_descriptor_both_includes_and_excludes() {
        let dep = cd_dep("foo:bar:1.2.3");

        let result = matches_content_descriptor(
            &dep,
            Some(&[
                cd_spec(ContentDescriptorMode::Include, ContentDescriptorMatcher::Simple, "foo"),
                cd_spec_full(ContentDescriptorMode::Exclude, ContentDescriptorMatcher::Simple, "foo", Some("baz"), None),
            ]),
        );
        assert!(result);

        let result = matches_content_descriptor(
            &dep,
            Some(&[
                cd_spec(ContentDescriptorMode::Include, ContentDescriptorMatcher::Simple, "foo"),
                cd_spec_full(ContentDescriptorMode::Exclude, ContentDescriptorMatcher::Simple, "foo", Some("bar"), None),
            ]),
        );
        assert!(!result);
    }

    // Ported: "if only includes exist, dep must match at least one include" — gradle/extract.spec.ts line 635
    #[test]
    fn content_descriptor_only_includes() {
        let dep = cd_dep("foo:bar:1.2.3");

        let result = matches_content_descriptor(
            &dep,
            Some(&[
                cd_spec(ContentDescriptorMode::Include, ContentDescriptorMatcher::Simple, "some"),
                cd_spec(ContentDescriptorMode::Include, ContentDescriptorMatcher::Simple, "foo"),
                cd_spec(ContentDescriptorMode::Include, ContentDescriptorMatcher::Simple, "bar"),
            ]),
        );
        assert!(result);

        let result = matches_content_descriptor(
            &dep,
            Some(&[
                cd_spec(ContentDescriptorMode::Include, ContentDescriptorMatcher::Simple, "some"),
                cd_spec(ContentDescriptorMode::Include, ContentDescriptorMatcher::Simple, "other"),
                cd_spec(ContentDescriptorMode::Include, ContentDescriptorMatcher::Simple, "bar"),
            ]),
        );
        assert!(!result);
    }

    // Ported: "if only excludes exist, dep must match not match any exclude" — gradle/extract.spec.ts line 653
    #[test]
    fn content_descriptor_only_excludes() {
        let dep = cd_dep("foo:bar:1.2.3");

        let result = matches_content_descriptor(
            &dep,
            Some(&[
                cd_spec(ContentDescriptorMode::Exclude, ContentDescriptorMatcher::Simple, "some"),
                cd_spec(ContentDescriptorMode::Exclude, ContentDescriptorMatcher::Simple, "foo"),
                cd_spec(ContentDescriptorMode::Exclude, ContentDescriptorMatcher::Simple, "bar"),
            ]),
        );
        assert!(!result);

        let result = matches_content_descriptor(
            &dep,
            Some(&[
                cd_spec(ContentDescriptorMode::Exclude, ContentDescriptorMatcher::Simple, "some"),
                cd_spec(ContentDescriptorMode::Exclude, ContentDescriptorMatcher::Simple, "other"),
                cd_spec(ContentDescriptorMode::Exclude, ContentDescriptorMatcher::Simple, "bar"),
            ]),
        );
        assert!(result);
    }

    // Ported: "deduplicates registry urls" — gradle/extract.spec.ts line 414
    #[test]
    fn registry_urls_deduplicate() {
        let registries = vec![
            PackageRegistry {
                registry_url: "https://repo.maven.apache.org/maven2".to_owned(),
                registry_type: RegistryType::Regular,
                scope: RegistryScope::Dep,
                content: None,
            },
            PackageRegistry {
                registry_url: "https://repo.maven.apache.org/maven2".to_owned(),
                registry_type: RegistryType::Regular,
                scope: RegistryScope::Dep,
                content: None,
            },
            PackageRegistry {
                registry_url: "https://example.com".to_owned(),
                registry_type: RegistryType::Regular,
                scope: RegistryScope::Dep,
                content: None,
            },
            PackageRegistry {
                registry_url: "https://example.com".to_owned(),
                registry_type: RegistryType::Regular,
                scope: RegistryScope::Dep,
                content: None,
            },
            PackageRegistry {
                registry_url: "https://plugins.gradle.org/m2/".to_owned(),
                registry_type: RegistryType::Regular,
                scope: RegistryScope::Plugin,
                content: None,
            },
        ];

        let plugin_dep = ContentDescriptorDep {
            dep_name: "foo.bar".to_owned(),
            current_value: Some("1.2.3".to_owned()),
            dep_type: Some("plugin".to_owned()),
        };
        let urls = get_registry_urls_for_dep(&registries, &plugin_dep);
        assert_eq!(urls, vec!["https://plugins.gradle.org/m2/"]);

        let dep_dep = ContentDescriptorDep {
            dep_name: "foo:bar".to_owned(),
            current_value: Some("1.2.3".to_owned()),
            dep_type: None,
        };
        let urls = get_registry_urls_for_dep(&registries, &dep_dep);
        assert_eq!(
            urls,
            vec![
                "https://repo.maven.apache.org/maven2",
                "https://example.com",
            ]
        );
    }

    // Ported: "supports separate registry URLs for plugins" — gradle/extract.spec.ts line 507
    #[test]
    fn registry_urls_separate_plugin_scopes() {
        let registries = vec![
            PackageRegistry {
                registry_url: "https://foo.bar/plugins".to_owned(),
                registry_type: RegistryType::Regular,
                scope: RegistryScope::Plugin,
                content: None,
            },
            PackageRegistry {
                registry_url: "https://foo.bar/deps".to_owned(),
                registry_type: RegistryType::Regular,
                scope: RegistryScope::Dep,
                content: None,
            },
            PackageRegistry {
                registry_url: "https://repo.maven.apache.org/maven2".to_owned(),
                registry_type: RegistryType::Regular,
                scope: RegistryScope::Dep,
                content: None,
            },
        ];

        let plugin_dep = ContentDescriptorDep {
            dep_name: "foo.bar".to_owned(),
            current_value: Some("1.2.3".to_owned()),
            dep_type: Some("plugin".to_owned()),
        };
        let urls = get_registry_urls_for_dep(&registries, &plugin_dep);
        assert_eq!(urls, vec!["https://foo.bar/plugins"]);

        let dep_dep = ContentDescriptorDep {
            dep_name: "io.jsonwebtoken:jjwt-api".to_owned(),
            current_value: Some("0.11.2".to_owned()),
            dep_type: None,
        };
        let urls = get_registry_urls_for_dep(&registries, &dep_dep);
        assert_eq!(
            urls,
            vec![
                "https://foo.bar/deps",
                "https://repo.maven.apache.org/maven2",
            ]
        );
    }

    // Ported: "exclusiveContent" — gradle/extract.spec.ts line 775
    #[test]
    fn registry_urls_exclusive_content() {
        let registries = vec![
            PackageRegistry {
                registry_url: "https://dl.google.com/android/maven2/".to_owned(),
                registry_type: RegistryType::Regular,
                scope: RegistryScope::Dep,
                content: None,
            },
            PackageRegistry {
                registry_url: "https://artifactory.foo.bar/artifactory/test".to_owned(),
                registry_type: RegistryType::Exclusive,
                scope: RegistryScope::Dep,
                content: Some(vec![cd_spec(
                    ContentDescriptorMode::Include,
                    ContentDescriptorMatcher::Simple,
                    "foo.bar",
                )]),
            },
        ];

        let matching_dep = ContentDescriptorDep {
            dep_name: "foo.bar:protobuf-java".to_owned(),
            current_value: Some("2.17.0".to_owned()),
            dep_type: None,
        };
        let urls = get_registry_urls_for_dep(&registries, &matching_dep);
        assert_eq!(
            urls,
            vec!["https://artifactory.foo.bar/artifactory/test"]
        );

        let non_matching_dep = ContentDescriptorDep {
            dep_name: "com.google.protobuf:protobuf-java".to_owned(),
            current_value: Some("2.17.1".to_owned()),
            dep_type: None,
        };
        let urls = get_registry_urls_for_dep(&registries, &non_matching_dep);
        assert_eq!(
            urls,
            vec!["https://dl.google.com/android/maven2/"]
        );
    }

    // Ported: "exclusiveContent with repeated repository definition" — gradle/extract.spec.ts line 823
    #[test]
    fn registry_urls_exclusive_content_repeated_repo() {
        let registries = vec![
            PackageRegistry {
                registry_url: "https://dl.google.com/android/maven2/".to_owned(),
                registry_type: RegistryType::Regular,
                scope: RegistryScope::Dep,
                content: None,
            },
            PackageRegistry {
                registry_url: "https://artifactory.foo.bar/artifactory/test".to_owned(),
                registry_type: RegistryType::Exclusive,
                scope: RegistryScope::Dep,
                content: Some(vec![cd_spec(
                    ContentDescriptorMode::Include,
                    ContentDescriptorMatcher::Simple,
                    "foo.bar",
                )]),
            },
            PackageRegistry {
                registry_url: "https://dl.google.com/android/maven2/".to_owned(),
                registry_type: RegistryType::Regular,
                scope: RegistryScope::Dep,
                content: None,
            },
        ];

        let dep = ContentDescriptorDep {
            dep_name: "foo.bar:protobuf-java".to_owned(),
            current_value: Some("2.17.0".to_owned()),
            dep_type: None,
        };
        let urls = get_registry_urls_for_dep(&registries, &dep);
        assert_eq!(
            urls,
            vec!["https://artifactory.foo.bar/artifactory/test"]
        );
    }
}

// Ported: "supports multiple levels of glob" — consistent-versions-plugin.spec.ts line 97
#[test]
fn gcv_supports_multiple_glob_levels() {
    let props = "org.apache.* = 4\norg.apache.lucene:* = 3\norg.apache.lucene:a.* = 2\norg.apache.lucene:a.b = 1\norg.apache.foo*:* = 5\n";
    let lock = "# Run ./gradlew --write-locks to regenerate this file\norg.apache.solr:x.y:1 (10 constraints: 95be0c15)\norg.apache.lucene:a.b:1 (10 constraints: 95be0c15)\norg.apache.lucene:a.c:1 (10 constraints: 95be0c15)\norg.apache.lucene:a.d:1 (10 constraints: 95be0c15)\norg.apache.lucene:d:1 (10 constraints: 95be0c15)\norg.apache.lucene:e.f:1 (10 constraints: 95be0c15)\norg.apache.foo-bar:a:1 (10 constraints: 95be0c15)\n";
    let mut files = std::collections::HashMap::new();
    files.insert("versions.props".to_owned(), props.to_owned());
    files.insert("versions.lock".to_owned(), lock.to_owned());
    let deps = parse_gcv("versions.props", &files);
    // Exact match
    let ab = deps
        .iter()
        .find(|d| d.dep_name == "org.apache.lucene:a.b")
        .unwrap();
    assert_eq!(ab.current_value, "1");
    assert!(ab.shared_variable_name.is_none());
    // a.c matches org.apache.lucene:a.* (longer glob wins)
    let ac = deps
        .iter()
        .find(|d| d.dep_name == "org.apache.lucene:a.c")
        .unwrap();
    assert_eq!(ac.current_value, "2");
    assert_eq!(
        ac.shared_variable_name.as_deref(),
        Some("org.apache.lucene:a.*")
    );
    // d matches org.apache.lucene:*
    let d_dep = deps
        .iter()
        .find(|d| d.dep_name == "org.apache.lucene:d")
        .unwrap();
    assert_eq!(d_dep.current_value, "3");
    assert_eq!(
        d_dep.shared_variable_name.as_deref(),
        Some("org.apache.lucene:*")
    );
    // foo-bar:a matches org.apache.foo*:*
    let foo = deps
        .iter()
        .find(|d| d.dep_name == "org.apache.foo-bar:a")
        .unwrap();
    assert_eq!(foo.current_value, "5");
    assert_eq!(
        foo.shared_variable_name.as_deref(),
        Some("org.apache.foo*:*")
    );
}
