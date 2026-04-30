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

/// Matches `sbt.version=x.y.z` in `build.properties`.
static SBT_VERSION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"sbt\.version\s*=\s*(?P<ver>\d+\.\d+\.\d+)").unwrap());

/// Extract SBT dependencies from a `.sbt` or `project/*.scala` file.
pub fn extract(content: &str) -> Vec<SbtDep> {
    let mut out = Vec::new();
    for line in content.lines() {
        // Strip single-line comments.
        let line = line.split("//").next().unwrap_or(line).trim_end();

        // Detect addSbtPlugin calls.
        let is_plugin = line.contains("addSbtPlugin");

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

    // Ported: "extracts deps for generic use-cases" — sbt/extract.spec.ts line 47
    #[test]
    fn comment_line_skipped() {
        let deps = extract(SAMPLE_SBT);
        assert!(!deps.iter().any(|d| d.artifact_id == "skip"));
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
    }
}
