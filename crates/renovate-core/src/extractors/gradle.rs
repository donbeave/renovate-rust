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

#[cfg(test)]
mod tests {
    use super::*;

    // ── build file tests ──────────────────────────────────────────────────────

    #[test]
    fn extracts_implementation_single_quote() {
        let content = "implementation 'com.google.guava:guava:31.0-jre'\n";
        let deps = extract_build_file(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "com.google.guava:guava");
        assert_eq!(deps[0].current_value, "31.0-jre");
        assert!(deps[0].skip_reason.is_none());
    }

    #[test]
    fn extracts_implementation_double_quote_parens() {
        let content = r#"implementation("com.google.guava:guava:31.0-jre")"#;
        let deps = extract_build_file(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "com.google.guava:guava");
    }

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

    #[test]
    fn skips_dynamic_versions() {
        let content = "implementation 'org.example:mylib:1.+'\n";
        let deps = extract_build_file(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(GradleSkipReason::DynamicVersion));
    }

    #[test]
    fn skips_snapshot_versions() {
        let content = "implementation 'com.example:mylib:1.0.0-SNAPSHOT'\n";
        let deps = extract_build_file(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(GradleSkipReason::DynamicVersion));
    }

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

    #[test]
    fn classpath_dependency() {
        let content = "classpath 'com.android.tools.build:gradle:7.4.0'\n";
        let deps = extract_build_file(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "com.android.tools.build:gradle");
    }

    // ── plugins {} block tests ────────────────────────────────────────────────

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
}
