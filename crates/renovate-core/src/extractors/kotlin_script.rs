//! Kotlin Script (`*.main.kts`) Maven dependency extractor.
//!
//! Parses `@file:DependsOn("group:artifact:version")` annotations and
//! optional `@file:Repository("url")` declarations.
//!
//! Renovate reference:
//! - `lib/modules/manager/kotlin-script/extract.ts`
//! - Pattern: `/^.+\.main\.kts$/`
//! - Datasource: Maven
//!
//! ## File format
//!
//! ```kotlin
//! @file:DependsOn("org.apache.commons:commons-lang3:3.12.0")
//! @file:Repository("https://repo.custom.example/")
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// A single extracted Kotlin Script Maven dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KotlinScriptDep {
    /// `groupId:artifactId` (e.g. `"org.apache.commons:commons-lang3"`).
    pub dep_name: String,
    /// Version string (e.g. `"3.12.0"`).
    pub current_value: String,
    /// Registry URLs declared via `@file:Repository("url")`, if any.
    pub registry_urls: Vec<String>,
}

static DEPENDS_ON_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"@file\s*:\s*DependsOn\s*\(\s*"(?P<groupId>[^:]+):(?P<artifactId>[^:]+):(?P<version>[^"]+)"\s*\)"#,
    )
    .unwrap()
});

static REPOSITORY_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"@file\s*:\s*Repository\s*\(\s*"(?P<url>[^"]+)"\s*\)"#).unwrap());

/// Extract Maven deps from a Kotlin Script file.
pub fn extract(content: &str) -> Vec<KotlinScriptDep> {
    let registry_urls: Vec<String> = REPOSITORY_RE
        .captures_iter(content)
        .filter_map(|cap| cap.name("url").map(|m| m.as_str().to_owned()))
        .collect();

    DEPENDS_ON_RE
        .captures_iter(content)
        .filter_map(|cap| {
            let group_id = cap.name("groupId")?.as_str();
            let artifact_id = cap.name("artifactId")?.as_str();
            let version = cap.name("version")?.as_str();
            Some(KotlinScriptDep {
                dep_name: format!("{}:{}", group_id, artifact_id),
                current_value: version.to_owned(),
                registry_urls: registry_urls.clone(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_single_dep() {
        let content = r#"@file:DependsOn("it.krzeminski:github-actions-kotlin-dsl:0.22.0")"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "it.krzeminski:github-actions-kotlin-dsl");
        assert_eq!(deps[0].current_value, "0.22.0");
        assert!(deps[0].registry_urls.is_empty());
    }

    #[test]
    fn extracts_multiple_deps() {
        let content = r#"
@file:DependsOn("it.krzeminski:github-actions-kotlin-dsl:0.22.0")
@file:DependsOn("org.eclipse.jgit:org.eclipse.jgit:4.6.0.201612231935-r")
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_name, "it.krzeminski:github-actions-kotlin-dsl");
        assert_eq!(deps[1].dep_name, "org.eclipse.jgit:org.eclipse.jgit");
        assert_eq!(deps[1].current_value, "4.6.0.201612231935-r");
    }

    #[test]
    fn extracts_custom_repositories() {
        let content = r#"
@file:Repository("https://jitpack.io")
@file:DependsOn("it.krzeminski:github-actions-kotlin-dsl:0.22.0")
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].registry_urls, vec!["https://jitpack.io"]);
    }

    #[test]
    fn no_annotations_returns_empty() {
        assert!(extract("// just a comment\nfun main() {}").is_empty());
    }

    // Ported: "dep with classifier version" — kotlin-script/extract.spec.ts
    #[test]
    fn dep_with_classifier_version() {
        let content = r#"@file:DependsOn("org.jetbrains.lets-plot:lets-plot-kotlin-jvm:3.0.2")"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].dep_name,
            "org.jetbrains.lets-plot:lets-plot-kotlin-jvm"
        );
        assert_eq!(deps[0].current_value, "3.0.2");
    }

    // Ported: "extracts dependencies in a generic case" — kotlin-script/extract.spec.ts line 12
    #[test]
    fn extracts_generic_case_fixture_three_deps() {
        // Mirrors kotlin-script/__fixtures__/generic-case.main.kts
        let content = r#"#!/usr/bin/env kotlin
@file:DependsOn("it.krzeminski:github-actions-kotlin-dsl:0.22.0")
@file:DependsOn(
  "org.eclipse.jgit:org.eclipse.jgit:4.6.0.201612231935-r"
)

@file : DependsOn
  (
  "org.jetbrains.lets-plot:lets-plot-kotlin-jvm:3.0.2"

)

println("Hello world")
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
        assert!(
            deps.iter()
                .any(|d| d.dep_name == "it.krzeminski:github-actions-kotlin-dsl"
                    && d.current_value == "0.22.0")
        );
        assert!(
            deps.iter()
                .any(|d| d.dep_name == "org.eclipse.jgit:org.eclipse.jgit"
                    && d.current_value == "4.6.0.201612231935-r")
        );
        assert!(deps.iter().any(
            |d| d.dep_name == "org.jetbrains.lets-plot:lets-plot-kotlin-jvm"
                && d.current_value == "3.0.2"
        ));
    }

    // Ported: "skips dependencies with missing parts" — kotlin-script/extract.spec.ts line 81
    #[test]
    fn skips_missing_parts() {
        // Mirrors missing-parts.main.kts: :group:version, group::version, group:artifact: are invalid
        let content = r#"#!/usr/bin/env kotlin
@file:DependsOn("it.krzeminski:github-actions-kotlin-dsl:0.22.0")
@file:DependsOn(":org.eclipse.jgit:4.6.0.201612231935-r")
@file:DependsOn("org.jetbrains.lets-plot::3.0.2")
@file:DependsOn("org.jetbrains.lets-plot:lets-plot-kotlin-jvm:")

println("Hello world")
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "it.krzeminski:github-actions-kotlin-dsl");
    }
}
