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
        r#""(?P<group>[^"]+)"\s+(?P<op>%%?)\s+"(?P<artifact>[^"]+)"\s+%\s+(?P<version>"[^"]+"|[A-Za-z_][A-Za-z0-9_\.]*)"#,
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
        });
    }

    for raw in content.lines() {
        let line = strip_line_comment(raw).trim_end();
        let is_plugin = line.contains("addSbtPlugin") || line.contains("addCompilerPlugin");
        for cap in DEP_EXPR_TOKEN.captures_iter(line) {
            let group = cap["group"].to_owned();
            let artifact = cap["artifact"].to_owned();
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
}
