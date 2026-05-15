//! SBT `build.sbt` and `project/*.scala` dependency extractor.
//!
//! Parses Scala-syntax dependency declarations and returns Maven-coordinate
//! deps for version lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/sbt/extract.ts`
//! - Patterns: `/\.sbt$/`, `/project/[^/]*\.scala$/`, `/project/build\.properties$/`
//! - Datasource: Maven Central (SbtPackageDatasource / SbtPluginDatasource)
//!
//! ## Supported dependency forms
//!
//! | Form | Notes |
//! |---|---|
//! | `"group" % "artifact" % "version"` | Java-style dep |
//! | `"group" %% "artifact" % "version"` | Scala-style dep (appends Scala version) |
//! | `addSbtPlugin("group" % "artifact" % "version")` | SBT plugin |
//!
//! ## `build.properties` support
//!
//! Extracts the `sbt.version=x.y.z` property.

use std::sync::LazyLock;

use regex::Regex;

/// Whether a dep uses `%%` (Scala cross-build) or `%` (Java) operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SbtDepStyle {
    /// `"group" % "artifact" % "version"` — exact artifact name.
    Java,
    /// `"group" %% "artifact" % "version"` — artifact name + `_<scalaVersion>`.
    Scala,
}

/// Dep type (regular vs SBT plugin).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SbtDepType {
    Library,
    Plugin,
    SbtVersion,
}

impl SbtDepType {
    pub fn as_renovate_str(&self) -> &'static str {
        match self {
            SbtDepType::Library => "library",
            SbtDepType::Plugin => "plugin",
            SbtDepType::SbtVersion => "sbt",
        }
    }
}

/// A single extracted SBT dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SbtDep {
    pub group_id: String,
    pub artifact_id: String,
    pub current_value: String,
    pub style: SbtDepStyle,
    pub dep_type: SbtDepType,
}

/// Resolved package-file dependency with SBT/Scala cross-version context applied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SbtResolvedDep {
    pub dep_name: String,
    pub package_name: String,
    pub current_value: Option<String>,
    pub dep_type: SbtDepType,
    pub shared_variable_name: Option<String>,
    pub registry_urls: Vec<String>,
}

/// Extracted `build.sbt`/Scala package-file data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SbtPackageFile {
    pub deps: Vec<SbtResolvedDep>,
    pub package_file_version: Option<String>,
    pub scala_version: Option<String>,
}

impl SbtDep {
    /// Maven `group:artifact` coordinates.
    pub fn dep_name(&self) -> String {
        format!("{}:{}", self.group_id, self.artifact_id)
    }
}

// ── Compiled regexes ─────────────────────────────────────────────────────────

/// Matches `"group" %%? "artifact" % "version"` in a dependency expression.
static DEP_EXPR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#""(?P<group>[^"]+)"\s+(?P<op>%%?)\s+"(?P<artifact>[^"]+)"\s+%\s+"(?P<version>[^"]+)""#,
    )
    .unwrap()
});

/// Matches `"group" %%? "artifact" % versionToken` in a dependency expression.
static DEP_EXPR_TOKEN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?P<group>"[^"]+"|[A-Za-z_][A-Za-z0-9_\.]*)\s+(?P<op>%%?)\s+(?P<artifact>"[^"]+"|[A-Za-z_][A-Za-z0-9_\.]*)\s+%\s+(?P<version>"[^"]+"|[A-Za-z_][A-Za-z0-9_\.]*)"#,
    )
    .unwrap()
});

/// Matches string variables such as `val Version: String = "1.2.3"`.
static STRING_VAR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?:(?:private|lazy)\s+)?val\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*(?::\s*String)?\s*=\s*"(?P<value>[^"]+)""#,
    )
    .unwrap()
});

/// Matches object fields such as `scala = "2.12.10"` inside `val versions = new { ... }`.
static OBJECT_FIELD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*=\s*"(?P<value>[^"]+)""#).unwrap()
});

/// Matches object declarations such as `val versions = new {`.
static OBJECT_DECL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?:(?:private|lazy)\s+)?val\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*=\s*new\s*\{"#)
        .unwrap()
});

/// Matches setting assignments such as `ThisBuild / scalaVersion := ScalaVersion,`.
static SETTING_ASSIGN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?:(?:ThisBuild|Global|LocalRootProject)\s*/\s*)?(?P<key>scalaVersion|version)\s*:=\s*(?P<value>"[^"]+"|[A-Za-z_][A-Za-z0-9_\.]*)"#,
    )
    .unwrap()
});

/// Matches `sbt.version=x.y.z` in `build.properties`.
static SBT_VERSION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"sbt\.version\s*=\s*(?P<ver>\d+\.\d+\.\d+)").unwrap());

/// Matches SBT resolver declarations such as `"Repo" at "https://repo/"`.
static RESOLVER_URL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#""[^"]+"\s+at\s+"(?P<url>[^"]+)""#).unwrap());

const DEFAULT_MAVEN_REGISTRY: &str = "https://repo.maven.apache.org/maven2";
const DEFAULT_SBT_PLUGIN_REGISTRY: &str = "https://repo.scala-sbt.org/scalasbt/sbt-plugin-releases";

/// Extract SBT dependencies from a `.sbt` or `project/*.scala` file.
pub fn extract(content: &str) -> Vec<SbtDep> {
    let mut out = Vec::new();
    for line in content.lines() {
        // Strip single-line comments.
        let line = line.split("//").next().unwrap_or(line).trim_end();

        // Detect plugin calls.
        let is_plugin = line.contains("addSbtPlugin") || line.contains("addCompilerPlugin");

        for cap in DEP_EXPR.captures_iter(line) {
            let style = if &cap["op"] == "%%" {
                SbtDepStyle::Scala
            } else {
                SbtDepStyle::Java
            };
            out.push(SbtDep {
                group_id: cap["group"].to_owned(),
                artifact_id: cap["artifact"].to_owned(),
                current_value: cap["version"].to_owned(),
                style,
                dep_type: if is_plugin {
                    SbtDepType::Plugin
                } else {
                    SbtDepType::Library
                },
            });
        }
    }
    out
}

/// Extract SBT package-file metadata with variables and Scala cross-version package names resolved.
pub fn extract_package_file(content: &str) -> Option<SbtPackageFile> {
    let variables = collect_string_variables(content);
    let mut scala_version = None;
    let mut package_file_version = None;
    let mut deps = Vec::new();

    for raw in content.lines() {
        let line = strip_line_comment(raw).trim_end();
        for cap in SETTING_ASSIGN.captures_iter(line) {
            let value = resolve_token(&cap["value"], &variables);
            match &cap["key"] {
                "scalaVersion" => scala_version = value,
                "version" => package_file_version = value,
                _ => {}
            }
        }
    }

    if let Some(version) = &scala_version {
        let artifact = scala_library_artifact(version);
        deps.push(SbtResolvedDep {
            dep_name: "scala".to_owned(),
            package_name: format!("org.scala-lang:{artifact}"),
            current_value: Some(version.clone()),
            dep_type: SbtDepType::Library,
            shared_variable_name: None,
            registry_urls: Vec::new(),
        });
    }

    for raw in content.lines() {
        let line = strip_line_comment(raw).trim_end();
        let is_plugin = line.contains("addSbtPlugin") || line.contains("addCompilerPlugin");
        for cap in DEP_EXPR_TOKEN.captures_iter(line) {
            let Some(group) = resolve_token(&cap["group"], &variables) else {
                continue;
            };
            let Some(artifact) = resolve_token(&cap["artifact"], &variables) else {
                continue;
            };
            let version_token = cap["version"].trim_end_matches(',');
            let current_value = resolve_token(version_token, &variables);
            let shared_variable_name = unquoted_identifier(version_token)
                .filter(|name| variables.contains_key(*name))
                .map(str::to_owned);
            let dep_name = format!("{group}:{artifact}");
            let package_artifact = if &cap["op"] == "%%" {
                scala_version
                    .as_deref()
                    .map(|version| format!("{artifact}_{}", scala_binary_version(version)))
                    .unwrap_or_else(|| artifact.clone())
            } else {
                artifact.clone()
            };
            deps.push(SbtResolvedDep {
                dep_name,
                package_name: format!("{group}:{package_artifact}"),
                current_value,
                dep_type: if is_plugin {
                    SbtDepType::Plugin
                } else {
                    SbtDepType::Library
                },
                shared_variable_name,
                registry_urls: Vec::new(),
            });
        }
    }

    if deps.is_empty() && package_file_version.is_none() {
        return None;
    }

    Some(SbtPackageFile {
        deps,
        package_file_version,
        scala_version: scala_version.as_deref().map(scala_binary_version),
    })
}

/// Extract all SBT package files from already-read path/content pairs.
pub fn extract_all_package_files(files: &[(&str, &str)]) -> Vec<SbtPackageFile> {
    let mut out = Vec::new();
    let repository_registries = files
        .iter()
        .find(|(path, _)| *path == "repositories")
        .map(|(_, content)| extract_repositories(content));
    let has_repositories_file = repository_registries.is_some();
    let repository_registries = repository_registries.unwrap_or_default();

    for (path, content) in files {
        if *path == "repositories" {
            continue;
        }

        if path.ends_with("build.properties") {
            if let Some(dep) = extract_build_properties(content) {
                out.push(SbtPackageFile {
                    deps: vec![SbtResolvedDep {
                        dep_name: "sbt/sbt".to_owned(),
                        package_name: "sbt/sbt".to_owned(),
                        current_value: Some(dep.current_value),
                        dep_type: SbtDepType::SbtVersion,
                        shared_variable_name: None,
                        registry_urls: Vec::new(),
                    }],
                    package_file_version: None,
                    scala_version: None,
                });
            }
        } else if let Some(package_file) = extract_package_file(content) {
            out.push(apply_registry_urls(
                package_file,
                content,
                has_repositories_file,
                &repository_registries,
            ));
        }
    }
    out
}

/// Extract `sbt.version` from a `project/build.properties` file.
pub fn extract_build_properties(content: &str) -> Option<SbtDep> {
    let cap = SBT_VERSION.captures(content)?;
    Some(SbtDep {
        group_id: "org.scala-sbt".to_owned(),
        artifact_id: "sbt".to_owned(),
        current_value: cap["ver"].to_owned(),
        style: SbtDepStyle::Java,
        dep_type: SbtDepType::SbtVersion,
    })
}

fn strip_line_comment(line: &str) -> &str {
    line.split("//").next().unwrap_or(line)
}

fn collect_string_variables(content: &str) -> std::collections::HashMap<String, String> {
    let mut variables = std::collections::HashMap::new();
    let mut current_object: Option<String> = None;

    for raw in content.lines() {
        let line = strip_line_comment(raw).trim();
        if let Some(object_name) = current_object.clone() {
            if line.contains('}') {
                current_object = None;
                continue;
            }
            if let Some(cap) = OBJECT_FIELD.captures(line) {
                variables.insert(
                    format!("{object_name}.{}", &cap["name"]),
                    cap["value"].to_owned(),
                );
            }
            continue;
        }

        if let Some(cap) = STRING_VAR.captures(line) {
            variables.insert(cap["name"].to_owned(), cap["value"].to_owned());
        }

        if let Some(cap) = OBJECT_DECL.captures(line) {
            current_object = Some(cap["name"].to_owned());
        }
    }

    variables
}

fn extract_repositories(content: &str) -> Vec<String> {
    let mut urls = Vec::new();
    let mut in_repositories = false;

    for raw in content.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') {
            in_repositories = line == "[repositories]";
            continue;
        }
        if !in_repositories {
            continue;
        }
        if line == "maven-central" {
            push_unique(&mut urls, DEFAULT_MAVEN_REGISTRY);
        } else if let Some((_, value)) = line.split_once(':') {
            let url = value.split(',').next().unwrap_or(value).trim();
            if !url.is_empty() {
                push_unique(&mut urls, url);
            }
        }
    }

    urls
}

fn extract_resolver_urls(content: &str) -> Vec<String> {
    let mut urls = Vec::new();
    for cap in RESOLVER_URL.captures_iter(content) {
        push_unique(&mut urls, &cap["url"]);
    }
    urls
}

fn apply_registry_urls(
    mut package_file: SbtPackageFile,
    content: &str,
    has_repositories_file: bool,
    repository_registries: &[String],
) -> SbtPackageFile {
    let resolver_urls = extract_resolver_urls(content);
    for dep in &mut package_file.deps {
        if dep.dep_type == SbtDepType::SbtVersion {
            continue;
        }

        if has_repositories_file {
            dep.registry_urls = repository_registries.to_vec();
            continue;
        }

        let mut urls = Vec::new();
        if dep.dep_type == SbtDepType::Plugin {
            push_unique(&mut urls, DEFAULT_SBT_PLUGIN_REGISTRY);
        }
        push_unique(&mut urls, DEFAULT_MAVEN_REGISTRY);
        for url in &resolver_urls {
            push_unique(&mut urls, url);
        }
        dep.registry_urls = urls;
    }
    package_file
}

fn push_unique(urls: &mut Vec<String>, value: &str) {
    if !urls.iter().any(|url| url == value) {
        urls.push(value.to_owned());
    }
}

fn resolve_token(
    token: &str,
    variables: &std::collections::HashMap<String, String>,
) -> Option<String> {
    let token = token.trim().trim_end_matches(',');
    if token.starts_with('"') {
        return Some(token.trim_matches('"').to_owned());
    }
    variables.get(token).cloned()
}

fn unquoted_identifier(token: &str) -> Option<&str> {
    let token = token.trim().trim_end_matches(',');
    (!token.starts_with('"')).then_some(token)
}

fn scala_binary_version(version: &str) -> String {
    if version.starts_with('3') {
        return "3".to_owned();
    }
    let mut parts = version.split('.');
    match (parts.next(), parts.next()) {
        (Some(major), Some(minor)) => format!("{major}.{minor}"),
        _ => version.to_owned(),
    }
}

fn scala_library_artifact(version: &str) -> String {
    if version.starts_with('3') {
        "scala3-library_3".to_owned()
    } else {
        "scala-library".to_owned()
    }
}

/// Sort sbt package files so that `build.sbt` comes first.
///
/// Mirrors `lib/modules/manager/sbt/util.ts` `sortPackageFiles()`.
pub fn sort_package_files(package_files: &[&str]) -> Vec<String> {
    let mut sorted: Vec<String> = package_files.iter().map(|s| (*s).to_owned()).collect();
    if let Some(idx) = sorted.iter().position(|f| f.ends_with("build.sbt")) {
        let build_sbt = sorted.remove(idx);
        sorted.insert(0, build_sbt);
    }
    sorted
}

/// Normalize a Scala version string for sbt cross-building.
///
/// - Versions ≤ 2.10.0: returned as-is.
/// - Versions 2.10.x and above (but not Scala 3): returns `major.minor`.
/// - Scala 3 versions (> 3.0.0): returns just `major`.
///
/// Mirrors `lib/modules/manager/sbt/util.ts` `normalizeScalaVersion()`.
pub fn normalize_scala_version(version: &str) -> String {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        return version.to_owned();
    }
    let Ok(major) = parts[0].parse::<u64>() else { return version.to_owned() };
    let Ok(minor) = parts[1].parse::<u64>() else { return version.to_owned() };
    let Ok(patch) = parts[2].parse::<u64>() else { return version.to_owned() };

    let gt_2_10 = (major, minor, patch) > (2, 10, 0);
    if !gt_2_10 {
        return version.to_owned();
    }
    let is_scala3 = (major, minor, patch) > (3, 0, 0);
    if is_scala3 {
        parts[0].to_owned()
    } else {
        format!("{}.{}", parts[0], parts[1])
    }
}

/// Bump the `version := "..."` field in build.sbt content.
///
/// Mirrors `lib/modules/manager/sbt/update.ts` `bumpPackageVersion()`.
pub fn bump_package_version(content: &str, current_value: &str, bump_version: &str) -> String {
    use std::sync::LazyLock;
    static VERSION_RE: LazyLock<regex::Regex> = LazyLock::new(|| {
        regex::Regex::new(r#"(?m)^(version\s*:=\s*).*$"#).unwrap()
    });

    let new_ver = (|| -> Option<semver::Version> {
        let parsed = semver::Version::parse(current_value).ok()?;
        let mut new = parsed;
        match bump_version {
            "patch" => new.patch += 1,
            "minor" => {
                new.minor += 1;
                new.patch = 0;
            }
            "major" => {
                new.major += 1;
                new.minor = 0;
                new.patch = 0;
            }
            _ => return None,
        }
        Some(new)
    })();

    let Some(new_ver) = new_ver else {
        return content.to_owned();
    };

    let new_str = new_ver.to_string();
    VERSION_RE
        .replace(content, |caps: &regex::Captures| {
            format!("{}\"{}\"", &caps[1], new_str)
        })
        .into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_SBT: &str = r#"
name := "myproject"
version := "1.0.0"
scalaVersion := "2.13.12"

libraryDependencies ++= Seq(
  "org.typelevel" %% "cats-core" % "2.10.0",
  "com.typesafe.akka" %% "akka-actor" % "2.8.5",
  "org.apache.commons" % "commons-lang3" % "3.14.0"
)

// A comment with deps in it: "com.example" % "skip" % "1.0"
addSbtPlugin("com.github.sbt" % "sbt-native-packager" % "1.9.16")
"#;

    // Ported: "extracts deps for generic use-cases" — sbt/extract.spec.ts line 47
    #[test]
    fn extracts_scala_style_deps() {
        let deps = extract(SAMPLE_SBT);
        let cats = deps.iter().find(|d| d.artifact_id == "cats-core").unwrap();
        assert_eq!(cats.group_id, "org.typelevel");
        assert_eq!(cats.current_value, "2.10.0");
        assert_eq!(cats.style, SbtDepStyle::Scala);
        assert_eq!(cats.dep_type, SbtDepType::Library);
    }

    // Ported: "extracts deps for generic use-cases" — sbt/extract.spec.ts line 47
    #[test]
    fn extracts_java_style_deps() {
        let deps = extract(SAMPLE_SBT);
        let commons = deps
            .iter()
            .find(|d| d.artifact_id == "commons-lang3")
            .unwrap();
        assert_eq!(commons.group_id, "org.apache.commons");
        assert_eq!(commons.current_value, "3.14.0");
        assert_eq!(commons.style, SbtDepStyle::Java);
    }

    // Ported: "extracts deps for generic use-cases" — sbt/extract.spec.ts line 47
    #[test]
    fn extracts_plugin() {
        let deps = extract(SAMPLE_SBT);
        let plugin = deps
            .iter()
            .find(|d| d.dep_type == SbtDepType::Plugin)
            .unwrap();
        assert_eq!(plugin.artifact_id, "sbt-native-packager");
        assert_eq!(plugin.current_value, "1.9.16");
    }

    // Ported: "extract addCompilerPlugin" — sbt/extract.spec.ts line 452
    #[test]
    fn extracts_add_compiler_plugin() {
        let deps = extract(r#"addCompilerPlugin("org.scala-tools.sxr" %% "sxr" % "0.3.0")"#);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].group_id, "org.scala-tools.sxr");
        assert_eq!(deps[0].artifact_id, "sxr");
        assert_eq!(deps[0].current_value, "0.3.0");
        assert_eq!(deps[0].style, SbtDepStyle::Scala);
        assert_eq!(deps[0].dep_type, SbtDepType::Plugin);
    }

    // Ported: "extracts deps for generic use-cases" — sbt/extract.spec.ts line 47
    #[test]
    fn comment_line_skipped() {
        let deps = extract(SAMPLE_SBT);
        assert!(!deps.iter().any(|d| d.artifact_id == "skip"));
    }

    // Ported: "extract deps with comment" — sbt/extract.spec.ts line 412
    #[test]
    fn extracts_dependencies_with_trailing_comments() {
        let content = r#"
libraryDependencies ++= Seq(
  "com.typesafe.scala-logging" %% "scala-logging" % "3.9.4", /** critical lib */
  "ch.qos.logback" % "logback-classic" % "1.2.10" // common lib
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);

        let scala_logging = deps
            .iter()
            .find(|dep| dep.artifact_id == "scala-logging")
            .unwrap();
        assert_eq!(scala_logging.group_id, "com.typesafe.scala-logging");
        assert_eq!(scala_logging.current_value, "3.9.4");
        assert_eq!(scala_logging.style, SbtDepStyle::Scala);

        let logback = deps
            .iter()
            .find(|dep| dep.artifact_id == "logback-classic")
            .unwrap();
        assert_eq!(logback.group_id, "ch.qos.logback");
        assert_eq!(logback.current_value, "1.2.10");
        assert_eq!(logback.style, SbtDepStyle::Java);
    }

    // Ported: "extracts deps when scala version is defined in an object" — sbt/extract.spec.ts line 99
    #[test]
    fn package_file_resolves_object_variables() {
        let content = r#"
val versions = new {
  scala = "2.12.10"
  example = "0.0.8"
}
scalaVersion := versions.scala
version := "3.2.1"
libraryDependencies += "org.example" % "foo" % versions.example
"#;
        let package_file = extract_package_file(content).unwrap();
        assert_eq!(package_file.package_file_version.as_deref(), Some("3.2.1"));
        assert_eq!(package_file.scala_version.as_deref(), Some("2.12"));
        assert!(
            package_file
                .deps
                .iter()
                .any(|dep| dep.package_name == "org.scala-lang:scala-library"
                    && dep.current_value.as_deref() == Some("2.12.10"))
        );
        let foo = package_file
            .deps
            .iter()
            .find(|dep| dep.dep_name == "org.example:foo")
            .unwrap();
        assert_eq!(foo.package_name, "org.example:foo");
        assert_eq!(foo.current_value.as_deref(), Some("0.0.8"));
        assert_eq!(
            foo.shared_variable_name.as_deref(),
            Some("versions.example")
        );
    }

    // Ported: "skips deps when dotted symbolds do not resolve to anything" — sbt/extract.spec.ts line 136
    #[test]
    fn package_file_keeps_unresolved_dotted_symbols_without_current_value() {
        let content = r#"
scalaVersion := versions.scala
version := "3.2.1"
libraryDependencies += "org.example" % "foo" % versions.example
"#;
        let package_file = extract_package_file(content).unwrap();
        assert_eq!(package_file.package_file_version.as_deref(), Some("3.2.1"));
        assert!(package_file.scala_version.is_none());
        let foo = package_file
            .deps
            .iter()
            .find(|dep| dep.dep_name == "org.example:foo")
            .unwrap();
        assert_eq!(foo.current_value, None);
        assert_eq!(foo.package_name, "org.example:foo");
    }

    // Ported: "extracts deps when scala version is defined in a variable" — sbt/extract.spec.ts line 74
    #[test]
    fn package_file_resolves_scala_version_variable_fixture() {
        let content = r#"
val ScalaVersion = "2.12.10"
val versionExample = "0.0.8"

version := "3.2.1"

scalaVersion := ScalaVersion

libraryDependencies += "org.example" % "foo" % "0.0.1"
libraryDependencies += "org.example" %% "bar" % "0.0.2"
libraryDependencies ++= Seq(
  "org.example" %% "baz" % "0.0.3",
  "org.example" % "qux" % "0.0.4"
)

dependencyOverrides += "org.example" % "quux" % "0.0.5"
dependencyOverrides ++= {
  val groupIdExample = "org.example"
  val artifactIdExample = "corge"

  Seq(
    groupIdExample %% "quuz" % "0.0.6" % "test",
    "org.example" % artifactIdExample % "0.0.7" % Provided,
    "org.example" % "grault" % versionExample % Test
  )
}

addSbtPlugin("org.example" % "waldo" % "0.0.9")
"#;
        let package_file = extract_package_file(content).unwrap();
        assert_eq!(package_file.package_file_version.as_deref(), Some("3.2.1"));
        assert_eq!(package_file.scala_version.as_deref(), Some("2.12"));
        for (package_name, current_value) in [
            ("org.scala-lang:scala-library", "2.12.10"),
            ("org.example:foo", "0.0.1"),
            ("org.example:bar_2.12", "0.0.2"),
            ("org.example:baz_2.12", "0.0.3"),
            ("org.example:qux", "0.0.4"),
            ("org.example:quux", "0.0.5"),
            ("org.example:quuz_2.12", "0.0.6"),
            ("org.example:corge", "0.0.7"),
            ("org.example:grault", "0.0.8"),
            ("org.example:waldo", "0.0.9"),
        ] {
            assert!(
                package_file
                    .deps
                    .iter()
                    .any(|dep| dep.package_name == package_name
                        && dep.current_value.as_deref() == Some(current_value)),
                "missing {package_name}@{current_value}"
            );
        }
        let plugin = package_file
            .deps
            .iter()
            .find(|dep| dep.package_name == "org.example:waldo")
            .unwrap();
        assert_eq!(plugin.dep_type, SbtDepType::Plugin);
    }

    // Ported: "skips deps when scala version is missing" — sbt/extract.spec.ts line 185
    #[test]
    fn package_file_extracts_deps_when_scala_version_is_missing() {
        let content = r#"
version := "1.0.1"

libraryDependencies ++= Seq(
  "org.scalatest" %% "scalatest" % "3.0.0"
)

val sbtReleaseVersion = "1.0.11"
addSbtPlugin("com.github.gseitz" % "sbt-release" % sbtReleaseVersion)
"#;
        let package_file = extract_package_file(content).unwrap();
        assert_eq!(package_file.package_file_version.as_deref(), Some("1.0.1"));
        assert!(package_file.scala_version.is_none());
        let scalatest = package_file
            .deps
            .iter()
            .find(|dep| dep.dep_name == "org.scalatest:scalatest")
            .unwrap();
        assert_eq!(scalatest.package_name, "org.scalatest:scalatest");
        assert_eq!(scalatest.current_value.as_deref(), Some("3.0.0"));

        let plugin = package_file
            .deps
            .iter()
            .find(|dep| dep.dep_name == "com.github.gseitz:sbt-release")
            .unwrap();
        assert_eq!(plugin.dep_type, SbtDepType::Plugin);
        assert_eq!(plugin.current_value.as_deref(), Some("1.0.11"));
        assert_eq!(
            plugin.shared_variable_name.as_deref(),
            Some("sbtReleaseVersion")
        );
    }

    // Ported: "extracts typed variables" — sbt/extract.spec.ts line 170
    #[test]
    fn package_file_resolves_typed_variables() {
        let content = r#"
val version: String = "1.2.3"
libraryDependencies += "foo" % "bar" % version
"#;
        let package_file = extract_package_file(content).unwrap();
        let dep = package_file
            .deps
            .iter()
            .find(|dep| dep.dep_name == "foo:bar")
            .unwrap();
        assert_eq!(dep.current_value.as_deref(), Some("1.2.3"));
        assert_eq!(dep.shared_variable_name.as_deref(), Some("version"));
    }

    // Ported: "extract deps from native scala file with variables" — sbt/extract.spec.ts line 213
    #[test]
    fn package_file_extracts_native_scala_file_variables() {
        let content = r#"
import sbt._

object Dependencies {
  val moreSettings = Seq(
    scalaVersion := "2.13.0-RC5"
  )

  val abcVersion = "1.2.3"

  val ujson = "com.example" %% "foo" % "0.7.1"

  lazy val abc = "com.abc" % "abc" % abcVersion

  val relatedDeps = Seq(
    "com.abc" % "abc-a" % abcVersion,
    "com.abc" % "abc-b" % abcVersion
  )

  val aloneDepInSeq = List("com.abc" % "abc-c" % abcVersion)
}
"#;
        let package_file = extract_package_file(content).unwrap();
        assert_eq!(package_file.scala_version.as_deref(), Some("2.13"));
        assert!(
            package_file
                .deps
                .iter()
                .any(|dep| dep.package_name == "com.example:foo_2.13"
                    && dep.current_value.as_deref() == Some("0.7.1"))
        );
        for artifact in ["abc", "abc-a", "abc-b", "abc-c"] {
            assert!(
                package_file
                    .deps
                    .iter()
                    .any(|dep| dep.package_name == format!("com.abc:{artifact}")
                        && dep.current_value.as_deref() == Some("1.2.3")),
                "missing {artifact}"
            );
        }
    }

    // Ported: "extract deps from native scala file with private variables" — sbt/extract.spec.ts line 349
    #[test]
    fn package_file_extracts_native_scala_private_variables() {
        let content = r#"
import sbt._

object Dependencies {
  val moreSettings = Seq(
    scalaVersion := "2.13.0-RC5"
  )

  private val abcVersion = "1.2.3"

  private lazy val ujson = "com.example" %% "foo" % "0.7.1"

  lazy val abc = "com.abc" % "abc" % abcVersion

  lazy val dependentLibraries = Seq(ujson, abc)
}
"#;
        let package_file = extract_package_file(content).unwrap();
        assert_eq!(package_file.scala_version.as_deref(), Some("2.13"));
        assert!(
            package_file
                .deps
                .iter()
                .any(|dep| dep.package_name == "com.example:foo_2.13"
                    && dep.current_value.as_deref() == Some("0.7.1"))
        );
        let abc = package_file
            .deps
            .iter()
            .find(|dep| dep.package_name == "com.abc:abc")
            .unwrap();
        assert_eq!(abc.current_value.as_deref(), Some("1.2.3"));
        assert_eq!(abc.shared_variable_name.as_deref(), Some("abcVersion"));
    }

    // Ported: "extract deps when they are defined in a new line" — sbt/extract.spec.ts line 371
    #[test]
    fn package_file_extracts_deps_defined_in_named_seq() {
        let content = r#"
name := "service"
scalaVersion := "2.13.8"

lazy val compileDependencies =
  Seq(
    "com.typesafe.scala-logging" %% "scala-logging" % "3.9.4",
    "ch.qos.logback" % "logback-classic" % "1.2.10"
  )

libraryDependencies ++= compileDependencies
"#;
        let package_file = extract_package_file(content).unwrap();
        assert!(
            package_file
                .deps
                .iter()
                .any(|dep| dep.package_name == "org.scala-lang:scala-library"
                    && dep.current_value.as_deref() == Some("2.13.8"))
        );
        assert!(package_file.deps.iter().any(|dep| dep.package_name
            == "com.typesafe.scala-logging:scala-logging_2.13"
            && dep.current_value.as_deref() == Some("3.9.4")));
        assert!(
            package_file
                .deps
                .iter()
                .any(|dep| dep.package_name == "ch.qos.logback:logback-classic"
                    && dep.current_value.as_deref() == Some("1.2.10"))
        );
    }

    // Ported: "extracts packageFileVersion when scala version is defined in a variable" — sbt/extract.spec.ts line 159
    #[test]
    fn package_file_resolves_package_file_version_variable() {
        let content = r#"
val fileVersion = "1.2.3"
version := fileVersion
libraryDependencies += "foo" % "bar" % "0.0.1"
"#;
        let package_file = extract_package_file(content).unwrap();
        assert_eq!(package_file.package_file_version.as_deref(), Some("1.2.3"));
    }

    // Ported: "extracts deps when scala version is defined with a trailing comma" — sbt/extract.spec.ts line 232
    #[test]
    fn package_file_resolves_scala_version_with_trailing_comma() {
        let content = r#"
lazy val commonSettings = Seq(
  scalaVersion := "2.12.10",
)
libraryDependencies += "org.example" %% "bar" % "0.0.2"
"#;
        let package_file = extract_package_file(content).unwrap();
        assert_eq!(package_file.scala_version.as_deref(), Some("2.12"));
        assert!(
            package_file
                .deps
                .iter()
                .any(|dep| dep.package_name == "org.example:bar_2.12"
                    && dep.current_value.as_deref() == Some("0.0.2"))
        );
    }

    // Ported: "extracts deps when scala version is defined in a variable with a trailing comma" — sbt/extract.spec.ts line 253
    #[test]
    fn package_file_resolves_variable_scala_version_with_trailing_comma() {
        let content = r#"
val ScalaVersion = "2.12.10"
lazy val commonSettings = Seq(
  scalaVersion := ScalaVersion,
)
libraryDependencies += "org.example" %% "bar" % "0.0.2"
"#;
        let package_file = extract_package_file(content).unwrap();
        assert_eq!(package_file.scala_version.as_deref(), Some("2.12"));
        assert!(
            package_file
                .deps
                .iter()
                .any(|dep| dep.package_name == "org.example:bar_2.12")
        );
    }

    // Ported: "extracts deps when scala version is defined with ThisBuild scope" — sbt/extract.spec.ts line 275
    #[test]
    fn package_file_resolves_thisbuild_scala_version() {
        let content = r#"
ThisBuild / scalaVersion := "2.12.10"
libraryDependencies += "org.example" %% "bar" % "0.0.2"
"#;
        let package_file = extract_package_file(content).unwrap();
        assert_eq!(package_file.scala_version.as_deref(), Some("2.12"));
        assert!(
            package_file
                .deps
                .iter()
                .any(|dep| dep.package_name == "org.example:bar_2.12")
        );
    }

    // Ported: "extracts correct scala library when dealing with scala 3" — sbt/extract.spec.ts line 294
    #[test]
    fn package_file_extracts_scala3_library() {
        let package_file = extract_package_file(r#"scalaVersion := "3.1.1""#).unwrap();
        assert!(
            package_file
                .deps
                .iter()
                .any(|dep| dep.package_name == "org.scala-lang:scala3-library_3"
                    && dep.current_value.as_deref() == Some("3.1.1"))
        );
    }

    // Ported: "extracts deps correctly when dealing with scala 3" — sbt/extract.spec.ts line 309
    #[test]
    fn package_file_resolves_scala3_cross_dependencies() {
        let content = r#"
scalaVersion := "3.3.4"
libraryDependencies += "org.example" %% "bar" % "0.0.5"
"#;
        let package_file = extract_package_file(content).unwrap();
        assert!(
            package_file
                .deps
                .iter()
                .any(|dep| dep.package_name == "org.example:bar_3"
                    && dep.current_value.as_deref() == Some("0.0.5"))
        );
    }

    // Ported: "extracts deps when scala version is defined in a variable with ThisBuild scope" — sbt/extract.spec.ts line 329
    #[test]
    fn package_file_resolves_thisbuild_variable_scala_version() {
        let content = r#"
val ScalaVersion = "2.12.10"
ThisBuild / scalaVersion := ScalaVersion
libraryDependencies += "org.example" %% "bar" % "0.0.2"
"#;
        let package_file = extract_package_file(content).unwrap();
        assert_eq!(package_file.scala_version.as_deref(), Some("2.12"));
        assert!(
            package_file
                .deps
                .iter()
                .any(|dep| dep.package_name == "org.example:bar_2.12")
        );
    }

    // Ported: "extracts deps for generic use-cases" — sbt/extract.spec.ts line 47
    #[test]
    fn dep_name_formats_correctly() {
        let dep = SbtDep {
            group_id: "org.typelevel".to_owned(),
            artifact_id: "cats-core".to_owned(),
            current_value: "2.10.0".to_owned(),
            style: SbtDepStyle::Scala,
            dep_type: SbtDepType::Library,
        };
        assert_eq!(dep.dep_name(), "org.typelevel:cats-core");
    }

    // Ported: "returns null for empty" — sbt/extract.spec.ts line 23
    #[test]
    fn build_properties_extraction() {
        let content = "sbt.version=1.9.8\n";
        let dep = extract_build_properties(content).unwrap();
        assert_eq!(dep.current_value, "1.9.8");
        assert_eq!(dep.group_id, "org.scala-sbt");
    }

    // Ported: "returns null for empty" — sbt/extract.spec.ts line 23
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
        assert!(extract_build_properties("").is_none());
        assert!(extract("non-sense").is_empty());
    }

    // Ported: "extract sbt version" — sbt/extract.spec.ts line 469
    #[test]
    fn build_properties_extracts_sbt_version() {
        let content = "sbt.version=1.6.0\n";
        let dep = extract_build_properties(content).unwrap();
        assert_eq!(dep.current_value, "1.6.0");
    }

    // Ported: "extract sbt version if the file contains other properties" — sbt/extract.spec.ts line 492
    #[test]
    fn build_properties_with_other_props_extracts_sbt_version() {
        let content = "sbt.version=1.6.0\nanother.conf=1.4.0\n";
        let dep = extract_build_properties(content).unwrap();
        assert_eq!(dep.current_value, "1.6.0");
    }

    // Ported: "ignores build.properties file if does not contain sbt version" — sbt/extract.spec.ts line 516
    #[test]
    fn build_properties_without_sbt_version_returns_none() {
        let content = "another.conf=1.4.0\n";
        assert!(extract_build_properties(content).is_none());
    }

    // Ported: "should return empty packagefiles is no content is provided" — sbt/extract.spec.ts line 637
    #[test]
    fn extract_all_package_files_empty_content_returns_empty() {
        assert!(extract_all_package_files(&[("build.sbt", "")]).is_empty());
    }

    // Ported: "extracts build properties correctly" — sbt/extract.spec.ts line 643
    #[test]
    fn extract_all_package_files_extracts_build_properties() {
        let packages =
            extract_all_package_files(&[("project/build.properties", "sbt.version=1.6.0\n")]);
        assert_eq!(packages.len(), 1);
        assert_eq!(packages[0].deps.len(), 1);
        let dep = &packages[0].deps[0];
        assert_eq!(dep.dep_type, SbtDepType::SbtVersion);
        assert_eq!(dep.dep_name, "sbt/sbt");
        assert_eq!(dep.package_name, "sbt/sbt");
        assert_eq!(dep.current_value.as_deref(), Some("1.6.0"));
    }

    // Ported: "extracts proxy repositories" — sbt/extract.spec.ts line 529
    #[test]
    fn extract_all_package_files_extracts_proxy_repositories() {
        let repositories = r#"
[repositories]
local
my-maven-repo: http://example.org/repo
my-ivy-repo: https://example.org/ivy-repo/, [organization]/[module]/[revision]/[type]s/[artifact](-[classifier]).[ext]
maven-central
"#;
        let build = r#"
scalaVersion := "2.13.0-RC5"
libraryDependencies += "com.example" %% "foo" % "0.7.1"
"#;
        let packages =
            extract_all_package_files(&[("repositories", repositories), ("build.sbt", build)]);
        assert_eq!(packages.len(), 1);
        let expected = vec![
            "http://example.org/repo".to_owned(),
            "https://example.org/ivy-repo/".to_owned(),
            DEFAULT_MAVEN_REGISTRY.to_owned(),
        ];
        for dep in &packages[0].deps {
            assert_eq!(dep.registry_urls, expected, "{}", dep.package_name);
        }
    }

    // Ported: "should include default registryUrls if no repositories file is provided" — sbt/extract.spec.ts line 607
    #[test]
    fn extract_all_package_files_uses_default_registry_urls_without_repositories_file() {
        let build = r#"
scalaVersion := "2.12.10"
libraryDependencies += "org.example" %% "bar" % "0.0.2"
resolvers += "Repo #1" at "https://example.com/repos/1/"
resolvers ++= Seq(
  "Repo #2" at "https://example.com/repos/2/",
  "Repo #3" at "https://example.com/repos/3/"
)
addSbtPlugin("org.example" % "waldo" % "0.0.9")
"#;
        let packages = extract_all_package_files(&[("build.sbt", build)]);
        assert_eq!(packages.len(), 1);

        let plugin = packages[0]
            .deps
            .iter()
            .find(|dep| dep.dep_type == SbtDepType::Plugin)
            .unwrap();
        assert_eq!(
            plugin.registry_urls,
            vec![
                DEFAULT_SBT_PLUGIN_REGISTRY.to_owned(),
                DEFAULT_MAVEN_REGISTRY.to_owned(),
                "https://example.com/repos/1/".to_owned(),
                "https://example.com/repos/2/".to_owned(),
                "https://example.com/repos/3/".to_owned(),
            ]
        );

        let library = packages[0]
            .deps
            .iter()
            .find(|dep| dep.package_name == "org.example:bar_2.12")
            .unwrap();
        assert_eq!(
            library.registry_urls,
            vec![
                DEFAULT_MAVEN_REGISTRY.to_owned(),
                "https://example.com/repos/1/".to_owned(),
                "https://example.com/repos/2/".to_owned(),
                "https://example.com/repos/3/".to_owned(),
            ]
        );
    }

    const SBT_CONTENT: &str =
        "name := \"test\"\norganization := \"test-org\"\nversion := \"0.0.2\"\n";

    // Ported: "increments" — modules/manager/sbt/update.spec.ts line 12
    #[test]
    fn sbt_bump_increments_patch() {
        let result = bump_package_version(SBT_CONTENT, "0.0.2", "patch");
        assert_eq!(result, SBT_CONTENT.replace("0.0.2", "0.0.3"));
        assert_ne!(result, SBT_CONTENT);
    }

    // Ported: "no ops" — modules/manager/sbt/update.spec.ts line 20
    #[test]
    fn sbt_bump_no_op_when_version_mismatch() {
        let result = bump_package_version(SBT_CONTENT, "0.0.1", "patch");
        assert_eq!(result, SBT_CONTENT);
    }

    // Ported: "updates" — modules/manager/sbt/update.spec.ts line 28
    #[test]
    fn sbt_bump_updates_minor() {
        let result = bump_package_version(SBT_CONTENT, "0.0.1", "minor");
        assert_eq!(result, SBT_CONTENT.replace("0.0.2", "0.1.0"));
        assert_ne!(result, SBT_CONTENT);
    }

    // Ported: "returns content if bumping errors" — modules/manager/sbt/update.spec.ts line 37
    #[test]
    fn sbt_bump_returns_content_on_invalid_bump_type() {
        let result = bump_package_version(SBT_CONTENT, "0.0.2", "not_a_bump");
        assert_eq!(result, SBT_CONTENT);
    }

    // Ported: "places build.sbt first" — modules/manager/sbt/util.spec.ts line 5
    #[test]
    fn sbt_sort_package_files_build_sbt_first() {
        let result = sort_package_files(&[
            "project/build.properties",
            "project/Dependencies.scala",
            "build.sbt",
        ]);
        assert_eq!(result, vec![
            "build.sbt",
            "project/build.properties",
            "project/Dependencies.scala",
        ]);
    }

    // Ported: "does not normalize prior to 2.10" — modules/manager/sbt/util.spec.ts line 17
    #[test]
    fn sbt_normalize_scala_version_prior_to_2_10() {
        assert_eq!(normalize_scala_version("2.9.3"), "2.9.3");
    }

    // Ported: "normalizes a Scala 2.10 version number" — modules/manager/sbt/util.spec.ts line 22
    #[test]
    fn sbt_normalize_scala_2_10() {
        assert_eq!(normalize_scala_version("2.10.7"), "2.10");
    }

    // Ported: "normalizes a Scala 2.11 version number" — modules/manager/sbt/util.spec.ts line 27
    #[test]
    fn sbt_normalize_scala_2_11() {
        assert_eq!(normalize_scala_version("2.11.12"), "2.11");
    }

    // Ported: "normalizes a Scala 2.12 version number" — modules/manager/sbt/util.spec.ts line 32
    #[test]
    fn sbt_normalize_scala_2_12() {
        assert_eq!(normalize_scala_version("2.12.19"), "2.12");
    }

    // Ported: "normalizes a Scala 2.13 version number" — modules/manager/sbt/util.spec.ts line 37
    #[test]
    fn sbt_normalize_scala_2_13() {
        assert_eq!(normalize_scala_version("2.13.14"), "2.13");
    }

    // Ported: "normalizes a Scala 3 LTS version number" — modules/manager/sbt/util.spec.ts line 42
    #[test]
    fn sbt_normalize_scala_3_lts() {
        assert_eq!(normalize_scala_version("3.3.3"), "3");
    }

    // Ported: "normalizes a Scala 3 current version number" — modules/manager/sbt/util.spec.ts line 47
    #[test]
    fn sbt_normalize_scala_3_current() {
        assert_eq!(normalize_scala_version("3.4.2"), "3");
    }
}
