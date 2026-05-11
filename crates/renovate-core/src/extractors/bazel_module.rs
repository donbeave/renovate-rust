//! Bazel `MODULE.bazel` (Bzlmod) dependency extractor.
//!
//! Parses `bazel_dep()` and `single_version_override()` / `archive_override()`
//! calls from Bazel module files to extract Bazel Central Registry deps.
//!
//! Renovate reference:
//! - `lib/modules/manager/bazel-module/extract.ts`
//! - Pattern: `/(^|/|\.)MODULE\.bazel$/`
//! - Datasource: Bazel Central Registry
//!
//! ## File format
//!
//! ```starlark
//! module(name = "my_module", version = "1.0.0")
//!
//! bazel_dep(name = "rules_go", version = "0.41.0")
//! bazel_dep(name = "gazelle", version = "0.32.0", dev_dependency = True)
//!
//! single_version_override(
//!     module_name = "rules_go",
//!     version = "0.42.0",
//! )
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// A single extracted Bazel module dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BazelModuleDep {
    /// Module name (e.g. `rules_go`).
    pub name: String,
    /// Version string (e.g. `0.41.0`).
    pub current_value: String,
    /// Which MODULE.bazel declaration produced this dep.
    pub dep_type: BazelModuleDepType,
    /// Optional Bazel registry URLs declared by overrides.
    pub registry_urls: Vec<String>,
    /// Whether this is a dev dependency.
    pub dev_dependency: bool,
    /// Set when the dep should be skipped.
    pub skip_reason: Option<BazelSkipReason>,
}

/// A dependency extracted from `crate.spec(...)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BazelCrateSpecDep {
    pub name: String,
    pub current_value: String,
    pub datasource: &'static str,
    pub package_name: Option<String>,
    pub registry_urls: Vec<String>,
    pub nested_version: bool,
    pub skip_reason: Option<BazelSkipReason>,
}

/// A dependency extracted from `maven.install(...)` or `maven.artifact(...)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BazelMavenDep {
    pub dep_name: String,
    pub current_value: String,
    pub dep_type: &'static str,
    pub registry_urls: Vec<String>,
}

/// A dependency extracted from `oci.pull(...)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BazelOciPullDep {
    pub dep_name: String,
    pub package_name: String,
    pub current_value: Option<String>,
    pub current_digest: Option<String>,
    pub datasource: &'static str,
    pub dep_type: &'static str,
}

/// A dependency extracted from `git_repository(...)` or `new_git_repository(...)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BazelGitRepositoryDep {
    pub dep_name: String,
    pub package_name: Option<String>,
    pub current_value: Option<String>,
    pub current_digest: Option<String>,
    pub datasource: &'static str,
    pub dep_type: &'static str,
}

/// Which Bazel module declaration produced the dep.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BazelModuleDepType {
    /// `bazel_dep(...)`
    BazelDep,
    /// `single_version_override(...)`
    SingleVersionOverride,
    /// `archive_override(...)`
    ArchiveOverride,
    /// `local_path_override(...)`
    LocalPathOverride,
}

/// Why a Bazel dep is skipped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BazelSkipReason {
    /// No version attribute in the `bazel_dep()` call.
    UnspecifiedVersion,
    /// Version is pinned by an override declaration.
    IsPinned,
    /// Override declarations are metadata for pinning and are not updated.
    Ignored,
    /// Module is pinned to an archive URL.
    FileDependency,
    /// Module is pinned to a local path.
    LocalDependency,
    /// Override declaration does not use a supported datasource.
    UnsupportedDatasource,
    /// Crate is local-path based.
    PathDependency,
    /// Crate spec has neither a version nor a supported alternate source.
    InvalidDependencySpecification,
}

/// Matches a `bazel_dep(name = "...", version = "...", ...)` call.
/// Handles multi-line calls by matching `name` and `version` attributes anywhere.
static BAZEL_DEP_BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)bazel_dep\s*\(([^)]+)\)").unwrap());

/// Matches a `single_version_override(...)` call.
static SINGLE_VERSION_OVERRIDE_BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)single_version_override\s*\(([^)]+)\)").unwrap());

/// Matches an `archive_override(...)` call.
static ARCHIVE_OVERRIDE_BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)archive_override\s*\(([^)]+)\)").unwrap());

/// Matches a `local_path_override(...)` call.
static LOCAL_PATH_OVERRIDE_BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)local_path_override\s*\(([^)]+)\)").unwrap());

/// Matches a `crate.spec(...)` call.
static CRATE_SPEC_BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)crate\.spec\s*\(([^)]+)\)").unwrap());

/// Matches a `maven.install(...)` call.
static MAVEN_INSTALL_BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)maven\.install\s*\(([^)]+)\)").unwrap());

/// Matches a `maven.artifact(...)` call.
static MAVEN_ARTIFACT_BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)maven\.artifact\s*\(([^)]+)\)").unwrap());

/// Matches an `oci.pull(...)` call.
static OCI_PULL_BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)oci\.pull\s*\(([^)]+)\)").unwrap());

/// Matches a `git_repository(...)` call without matching `new_git_repository(...)`.
static GIT_REPOSITORY_BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)(?:^|[^\w.])git_repository\s*\(([^)]+)\)").unwrap());

/// Matches a `new_git_repository(...)` call.
static NEW_GIT_REPOSITORY_BLOCK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)new_git_repository\s*\(([^)]+)\)").unwrap());

/// Extracts `name = "value"` or `name = 'value'` from a call argument list.
static ATTR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(\w+)\s*=\s*['"]([^'"]+)['"]"#).unwrap());

/// Extracts quoted strings from a Starlark list body.
static STRING_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"['"]([^'"]+)['"]"#).unwrap());

/// Extracts `dev_dependency = True` flag.
static DEV_DEP_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"dev_dependency\s*=\s*True").unwrap());

struct SingleVersionOverride {
    name: String,
    version: String,
    registry_urls: Vec<String>,
}

struct UnsupportedOverride {
    name: String,
    dep_type: BazelModuleDepType,
    bazel_dep_skip_reason: BazelSkipReason,
}

/// Extract Bazel module deps from a `MODULE.bazel` file.
pub fn extract(content: &str) -> Vec<BazelModuleDep> {
    // Strip single-line comments
    let stripped = strip_comments(content);

    let overrides = parse_single_version_overrides(&stripped);
    let unsupported_overrides = parse_unsupported_overrides(&stripped);
    let mut deps = Vec::new();

    for cap in BAZEL_DEP_BLOCK_RE.captures_iter(&stripped) {
        let args = &cap[1];

        let mut name = String::new();
        let mut version = String::new();

        for kv in ATTR_RE.captures_iter(args) {
            let key = &kv[1];
            let val = kv[2].to_owned();
            match key {
                "name" => name = val,
                "version" => version = val,
                _ => {}
            }
        }

        if name.is_empty() {
            continue;
        }

        let dev_dependency = DEV_DEP_RE.is_match(args);
        let override_metadata = overrides
            .iter()
            .find(|override_dep| override_dep.name == name);
        let pinned = override_metadata.filter(|override_dep| !override_dep.version.is_empty());
        let unsupported_override = unsupported_overrides
            .iter()
            .find(|override_dep| override_dep.name == name);
        let registry_urls = pinned
            .or(override_metadata)
            .map(|override_dep| override_dep.registry_urls.clone())
            .unwrap_or_default();
        let skip_reason = unsupported_override
            .map(|override_dep| override_dep.bazel_dep_skip_reason)
            .or_else(|| pinned.map(|_| BazelSkipReason::IsPinned));

        if version.is_empty() {
            deps.push(BazelModuleDep {
                name,
                current_value: String::new(),
                dep_type: BazelModuleDepType::BazelDep,
                registry_urls,
                dev_dependency,
                skip_reason: Some(skip_reason.unwrap_or(BazelSkipReason::UnspecifiedVersion)),
            });
        } else {
            deps.push(BazelModuleDep {
                name,
                current_value: version,
                dep_type: BazelModuleDepType::BazelDep,
                registry_urls,
                dev_dependency,
                skip_reason,
            });
        }
    }

    deps.extend(
        unsupported_overrides
            .into_iter()
            .map(|override_dep| BazelModuleDep {
                name: override_dep.name,
                current_value: String::new(),
                dep_type: override_dep.dep_type,
                registry_urls: Vec::new(),
                dev_dependency: false,
                skip_reason: Some(BazelSkipReason::UnsupportedDatasource),
            }),
    );
    deps.extend(
        overrides
            .into_iter()
            .filter(|override_dep| !override_dep.version.is_empty())
            .map(|override_dep| BazelModuleDep {
                name: override_dep.name,
                current_value: override_dep.version,
                dep_type: BazelModuleDepType::SingleVersionOverride,
                registry_urls: override_dep.registry_urls,
                dev_dependency: false,
                skip_reason: Some(BazelSkipReason::Ignored),
            }),
    );
    deps
}

/// Extract `crate.spec(...)` dependencies from a `MODULE.bazel` file.
pub fn extract_crate_specs(content: &str) -> Vec<BazelCrateSpecDep> {
    let stripped = strip_comments(content);
    CRATE_SPEC_BLOCK_RE
        .captures_iter(&stripped)
        .filter_map(|cap| {
            let args = &cap[1];
            let attrs = attrs_from_args(args);
            let name = attrs.get("package")?.clone();

            if let Some(tag) = attrs.get("tag")
                && let Some(git) = attrs.get("git")
            {
                return Some(BazelCrateSpecDep {
                    name,
                    current_value: tag.clone(),
                    datasource: "github-tags",
                    package_name: github_package_name(git),
                    registry_urls: vec!["https://github.com".to_owned()],
                    nested_version: false,
                    skip_reason: None,
                });
            }

            if attrs.contains_key("path") {
                return Some(BazelCrateSpecDep {
                    name,
                    current_value: String::new(),
                    datasource: "crate",
                    package_name: None,
                    registry_urls: Vec::new(),
                    nested_version: false,
                    skip_reason: Some(BazelSkipReason::PathDependency),
                });
            }

            let Some(version) = attrs.get("version") else {
                return Some(BazelCrateSpecDep {
                    name,
                    current_value: String::new(),
                    datasource: "crate",
                    package_name: None,
                    registry_urls: Vec::new(),
                    nested_version: false,
                    skip_reason: Some(BazelSkipReason::InvalidDependencySpecification),
                });
            };

            Some(BazelCrateSpecDep {
                name,
                current_value: version.clone(),
                datasource: "crate",
                package_name: None,
                registry_urls: Vec::new(),
                nested_version: true,
                skip_reason: None,
            })
        })
        .collect()
}

/// Extract Maven dependencies from `maven.install(...)` and `maven.artifact(...)`.
pub fn extract_maven_deps(content: &str) -> Vec<BazelMavenDep> {
    let stripped = strip_comments(content);
    let mut deps = Vec::new();

    for cap in MAVEN_INSTALL_BLOCK_RE.captures_iter(&stripped) {
        let args = &cap[1];
        let registries = list_attr(args, "repositories");
        for artifact in list_attr(args, "artifacts") {
            if let Some((dep_name, current_value)) = parse_maven_coordinate(&artifact) {
                deps.push(BazelMavenDep {
                    dep_name,
                    current_value,
                    dep_type: "maven_install",
                    registry_urls: registries.clone(),
                });
            }
        }
    }

    for cap in MAVEN_ARTIFACT_BLOCK_RE.captures_iter(&stripped) {
        let args = &cap[1];
        let attrs = attrs_from_args(args);
        let Some(group) = attrs.get("group") else {
            continue;
        };
        let Some(artifact) = attrs.get("artifact") else {
            continue;
        };
        let Some(version) = attrs.get("version") else {
            continue;
        };
        deps.push(BazelMavenDep {
            dep_name: format!("{group}:{artifact}"),
            current_value: version.clone(),
            dep_type: "maven_install",
            registry_urls: Vec::new(),
        });
    }

    let install_registries = deps
        .iter()
        .find(|dep| !dep.registry_urls.is_empty())
        .map(|dep| dep.registry_urls.clone());
    if let Some(registries) = install_registries {
        for dep in &mut deps {
            if dep.registry_urls.is_empty() {
                dep.registry_urls = registries.clone();
            }
        }
    }

    deps
}

/// Extract OCI image dependencies from `oci.pull(...)`.
pub fn extract_oci_pull_deps(
    content: &str,
    registry_aliases: &[(&str, &str)],
) -> Vec<BazelOciPullDep> {
    let stripped = strip_comments(content);
    OCI_PULL_BLOCK_RE
        .captures_iter(&stripped)
        .filter_map(|cap| {
            let attrs = attrs_from_args(&cap[1]);
            let dep_name = attrs.get("name")?.clone();
            let image = attrs.get("image")?;

            Some(BazelOciPullDep {
                dep_name,
                package_name: apply_registry_alias(image, registry_aliases),
                current_value: attrs.get("tag").cloned(),
                current_digest: attrs.get("digest").cloned(),
                datasource: "docker",
                dep_type: "oci_pull",
            })
        })
        .collect()
}

/// Extract Git repository dependencies from Bazel repository rules.
pub fn extract_git_repository_deps(content: &str) -> Vec<BazelGitRepositoryDep> {
    let stripped = strip_comments(content);
    let mut deps = Vec::new();
    deps.extend(parse_git_repository_deps(
        &stripped,
        &GIT_REPOSITORY_BLOCK_RE,
        "git_repository",
    ));
    deps.extend(parse_git_repository_deps(
        &stripped,
        &NEW_GIT_REPOSITORY_BLOCK_RE,
        "new_git_repository",
    ));
    deps
}

fn parse_unsupported_overrides(content: &str) -> Vec<UnsupportedOverride> {
    let mut deps = Vec::new();
    deps.extend(parse_named_overrides(
        content,
        &ARCHIVE_OVERRIDE_BLOCK_RE,
        BazelModuleDepType::ArchiveOverride,
        BazelSkipReason::FileDependency,
    ));
    deps.extend(parse_named_overrides(
        content,
        &LOCAL_PATH_OVERRIDE_BLOCK_RE,
        BazelModuleDepType::LocalPathOverride,
        BazelSkipReason::LocalDependency,
    ));
    deps
}

fn parse_named_overrides(
    content: &str,
    regex: &Regex,
    dep_type: BazelModuleDepType,
    bazel_dep_skip_reason: BazelSkipReason,
) -> Vec<UnsupportedOverride> {
    regex
        .captures_iter(content)
        .filter_map(|cap| {
            let args = &cap[1];
            let name = ATTR_RE.captures_iter(args).find_map(|kv| {
                if &kv[1] == "module_name" {
                    Some(kv[2].to_owned())
                } else {
                    None
                }
            })?;
            Some(UnsupportedOverride {
                name,
                dep_type,
                bazel_dep_skip_reason,
            })
        })
        .collect()
}

fn parse_single_version_overrides(content: &str) -> Vec<SingleVersionOverride> {
    let mut deps = Vec::new();

    for cap in SINGLE_VERSION_OVERRIDE_BLOCK_RE.captures_iter(content) {
        let args = &cap[1];
        let mut name = String::new();
        let mut version = String::new();
        let mut registry_url = String::new();

        for kv in ATTR_RE.captures_iter(args) {
            let key = &kv[1];
            let val = kv[2].to_owned();
            match key {
                "module_name" => name = val,
                "version" => version = val,
                "registry" => registry_url = val,
                _ => {}
            }
        }

        if name.is_empty() {
            continue;
        }

        let registry_urls = if registry_url.is_empty() {
            Vec::new()
        } else {
            vec![registry_url]
        };

        deps.push(SingleVersionOverride {
            name,
            version,
            registry_urls,
        });
    }

    deps
}

fn attrs_from_args(args: &str) -> std::collections::BTreeMap<String, String> {
    ATTR_RE
        .captures_iter(args)
        .map(|cap| (cap[1].to_owned(), cap[2].to_owned()))
        .collect()
}

fn list_attr(args: &str, attr: &str) -> Vec<String> {
    let pattern = format!(r#"(?s){attr}\s*=\s*\[([^\]]*)\]"#);
    let Ok(regex) = Regex::new(&pattern) else {
        return Vec::new();
    };
    let Some(cap) = regex.captures(args) else {
        return Vec::new();
    };
    STRING_RE
        .captures_iter(&cap[1])
        .map(|item| item[1].to_owned())
        .collect()
}

fn parse_maven_coordinate(raw: &str) -> Option<(String, String)> {
    let parts = raw.split(':').collect::<Vec<_>>();
    if parts.len() < 3 {
        return None;
    }
    let group = parts[0];
    let artifact = parts[1];
    let version = parts.last()?;
    if group.is_empty() || artifact.is_empty() || version.is_empty() {
        None
    } else {
        Some((format!("{group}:{artifact}"), (*version).to_owned()))
    }
}

fn parse_git_repository_deps(
    content: &str,
    regex: &Regex,
    dep_type: &'static str,
) -> Vec<BazelGitRepositoryDep> {
    regex
        .captures_iter(content)
        .filter_map(|cap| {
            let attrs = attrs_from_args(&cap[1]);
            let remote = attrs.get("remote")?;
            Some(BazelGitRepositoryDep {
                dep_name: attrs.get("name")?.clone(),
                package_name: github_package_name(remote),
                current_value: attrs.get("tag").cloned(),
                current_digest: attrs.get("commit").cloned(),
                datasource: "github-tags",
                dep_type,
            })
        })
        .collect()
}

fn apply_registry_alias(image: &str, registry_aliases: &[(&str, &str)]) -> String {
    let Some((registry, rest)) = image.split_once('/') else {
        return image.to_owned();
    };
    registry_aliases
        .iter()
        .find_map(|(from, to)| {
            if *from == registry {
                Some(format!("{to}/{rest}"))
            } else {
                None
            }
        })
        .unwrap_or_else(|| image.to_owned())
}

fn github_package_name(url: &str) -> Option<String> {
    let rest = url
        .strip_prefix("https://github.com/")
        .or_else(|| url.strip_prefix("git@github.com:"))?;
    let rest = rest.strip_suffix(".git").unwrap_or(rest);
    let mut parts = rest.split('/');
    let owner = parts.next()?;
    let repo = parts.next()?;
    if owner.is_empty() || repo.is_empty() {
        None
    } else {
        Some(format!("{owner}/{repo}"))
    }
}

/// Strip `# comment` lines from Starlark content.
fn strip_comments(content: &str) -> String {
    content
        .lines()
        .map(|line| {
            if let Some(pos) = line.find('#') {
                &line[..pos]
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "returns bazel_dep and git_override dependencies" — bazel-module/extract.spec.ts line 54
    #[test]
    fn extracts_bazel_dep() {
        let content = r#"
bazel_dep(name = "rules_go", version = "0.41.0")
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "rules_go");
        assert_eq!(deps[0].current_value, "0.41.0");
        assert!(!deps[0].dev_dependency);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "returns bazel_dep and git_override dependencies" — bazel-module/extract.spec.ts line 54
    #[test]
    fn extracts_dev_dependency() {
        let content = r#"bazel_dep(name = "gazelle", version = "0.32.0", dev_dependency = True)"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert!(deps[0].dev_dependency);
    }

    // Ported: "returns bazel_dep and git_override dependencies" — bazel-module/extract.spec.ts line 54
    #[test]
    fn extracts_multiline_dep() {
        let content = r#"
bazel_dep(
    name = "rules_python",
    version = "0.24.0",
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "rules_python");
        assert_eq!(deps[0].current_value, "0.24.0");
    }

    // Ported: "returns bazel_dep and git_override dependencies" — bazel-module/extract.spec.ts line 54
    #[test]
    fn multiple_deps() {
        let content = r#"
bazel_dep(name = "rules_go", version = "0.41.0")
bazel_dep(name = "gazelle", version = "0.32.0")
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].name, "rules_go");
        assert_eq!(deps[1].name, "gazelle");
    }

    // Ported: "returns bazel_dep with no version and git_override" — bazel-module/extract.spec.ts line 95
    #[test]
    fn dep_without_version_skipped() {
        let content = r#"bazel_dep(name = "rules_go")"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(BazelSkipReason::UnspecifiedVersion)
        );
    }

    // Ported: "returns crate.spec dependencies" — bazel-module/extract.spec.ts line 377
    #[test]
    fn extracts_crate_spec_dependencies() {
        let input = r#"
crate.spec(
    package = "axum",
    version = "0.8.4",
)
crate.spec(
    package = "tokio",
    version = "1.45.1",
    features = [
        "full",
    ],
)
crate.spec(
    package = "custom_crate",
    git = "https://github.com/example/custom_crate.git",
    tag = "v1.0.0",
)
crate.spec(
    package = "local_crate",
    path = "/var/crate",
)
crate.spec(
    package = "no_version_crate",
)
"#;
        let deps = extract_crate_specs(input);
        assert_eq!(deps.len(), 5);
        assert_eq!(deps[0].name, "axum");
        assert_eq!(deps[0].current_value, "0.8.4");
        assert_eq!(deps[0].datasource, "crate");
        assert!(deps[0].nested_version);
        assert_eq!(deps[1].name, "tokio");
        assert_eq!(deps[1].current_value, "1.45.1");
        assert!(deps[1].nested_version);
        assert_eq!(deps[2].name, "custom_crate");
        assert_eq!(deps[2].current_value, "v1.0.0");
        assert_eq!(deps[2].datasource, "github-tags");
        assert_eq!(
            deps[2].package_name.as_deref(),
            Some("example/custom_crate")
        );
        assert_eq!(deps[2].registry_urls, vec!["https://github.com"]);
        assert!(!deps[2].nested_version);
        assert_eq!(deps[3].name, "local_crate");
        assert_eq!(deps[3].skip_reason, Some(BazelSkipReason::PathDependency));
        assert_eq!(deps[4].name, "no_version_crate");
        assert_eq!(
            deps[4].skip_reason,
            Some(BazelSkipReason::InvalidDependencySpecification)
        );
    }

    // Ported: "returns maven.install and maven.artifact dependencies" — bazel-module/extract.spec.ts line 453
    #[test]
    fn extracts_maven_install_and_artifact_dependencies() {
        let input = r#"
maven.artifact(
    artifact = "core.specs.alpha",
    exclusions = ["org.clojure:clojure"],
    group = "org.clojure",
    version = "0.2.56",
)

maven.install(
    artifacts = [
        "junit:junit:4.13.2",
        "com.google.guava:guava:31.1-jre",
    ],
    lock_file = "//:maven_install.json",
    repositories = [
        "https://repo1.maven.org/maven2/",
    ],
    version_conflict_policy = "pinned",
)
"#;
        let deps = extract_maven_deps(input);
        assert_eq!(deps.len(), 3);
        assert_eq!(deps[0].dep_name, "junit:junit");
        assert_eq!(deps[0].current_value, "4.13.2");
        assert_eq!(deps[0].dep_type, "maven_install");
        assert_eq!(
            deps[0].registry_urls,
            vec!["https://repo1.maven.org/maven2/"]
        );
        assert_eq!(deps[1].dep_name, "com.google.guava:guava");
        assert_eq!(deps[1].current_value, "31.1-jre");
        assert_eq!(deps[1].dep_type, "maven_install");
        assert_eq!(
            deps[1].registry_urls,
            vec!["https://repo1.maven.org/maven2/"]
        );
        assert_eq!(deps[2].dep_name, "org.clojure:core.specs.alpha");
        assert_eq!(deps[2].current_value, "0.2.56");
        assert_eq!(deps[2].dep_type, "maven_install");
        assert_eq!(
            deps[2].registry_urls,
            vec!["https://repo1.maven.org/maven2/"]
        );
    }

    // Ported: "returns oci.pull dependencies" — bazel-module/extract.spec.ts line 507
    #[test]
    fn extracts_oci_pull_dependency() {
        let input = r#"
oci.pull(
    name = "nginx_image",
    digest = "sha256:287ff321f9e3cde74b600cc26197424404157a72043226cbbf07ee8304a2c720",
    image = "index.docker.io/library/nginx",
    platforms = ["linux/amd64"],
    tag = "1.27.1",
)
"#;
        let deps = extract_oci_pull_deps(input, &[]);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].datasource, "docker");
        assert_eq!(deps[0].dep_type, "oci_pull");
        assert_eq!(deps[0].dep_name, "nginx_image");
        assert_eq!(deps[0].package_name, "index.docker.io/library/nginx");
        assert_eq!(deps[0].current_value.as_deref(), Some("1.27.1"));
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("sha256:287ff321f9e3cde74b600cc26197424404157a72043226cbbf07ee8304a2c720")
        );
    }

    // Ported: "returns oci.pull dependencies without tags" — bazel-module/extract.spec.ts line 544
    #[test]
    fn extracts_oci_pull_dependency_without_tag() {
        let input = r#"
oci.pull(
    name = "nginx_image",
    digest = "sha256:287ff321f9e3cde74b600cc26197424404157a72043226cbbf07ee8304a2c720",
    image = "index.docker.io/library/nginx",
    platforms = ["linux/amd64"],
)
"#;
        let deps = extract_oci_pull_deps(input, &[]);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "nginx_image");
        assert_eq!(deps[0].package_name, "index.docker.io/library/nginx");
        assert_eq!(deps[0].current_value, None);
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("sha256:287ff321f9e3cde74b600cc26197424404157a72043226cbbf07ee8304a2c720")
        );
    }

    // Ported: "returns oci.pull dependencies with tag only (no digest)" — bazel-module/extract.spec.ts line 578
    #[test]
    fn extracts_oci_pull_dependency_with_tag_only() {
        let input = r#"
oci.pull(
    name = "nginx_image",
    image = "index.docker.io/library/nginx",
    platforms = ["linux/amd64"],
    tag = "1.27.1",
)
"#;
        let deps = extract_oci_pull_deps(input, &[]);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "nginx_image");
        assert_eq!(deps[0].package_name, "index.docker.io/library/nginx");
        assert_eq!(deps[0].current_value.as_deref(), Some("1.27.1"));
        assert_eq!(deps[0].current_digest, None);
    }

    // Ported: "returns oci.pull dependencies without tag or digest" — bazel-module/extract.spec.ts line 611
    #[test]
    fn extracts_oci_pull_dependency_without_tag_or_digest() {
        let input = r#"
oci.pull(
    name = "nginx_image",
    image = "index.docker.io/library/nginx",
    platforms = ["linux/amd64"],
)
"#;
        let deps = extract_oci_pull_deps(input, &[]);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "nginx_image");
        assert_eq!(deps[0].package_name, "index.docker.io/library/nginx");
        assert_eq!(deps[0].current_value, None);
        assert_eq!(deps[0].current_digest, None);
    }

    // Ported: "returns oci.pull dependencies with registryAliases" — bazel-module/extract.spec.ts line 641
    #[test]
    fn extracts_oci_pull_dependency_with_registry_alias() {
        let input = r#"
oci.pull(
    name = "nginx_image",
    digest = "sha256:287ff321f9e3cde74b600cc26197424404157a72043226cbbf07ee8304a2c720",
    image = "index.docker.io/library/nginx",
    platforms = ["linux/amd64"],
    tag = "1.27.1",
)
"#;
        let deps = extract_oci_pull_deps(
            input,
            &[("index.docker.io", "my-docker-mirror.registry.com")],
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "nginx_image");
        assert_eq!(
            deps[0].package_name,
            "my-docker-mirror.registry.com/library/nginx"
        );
        assert_eq!(deps[0].current_value.as_deref(), Some("1.27.1"));
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("sha256:287ff321f9e3cde74b600cc26197424404157a72043226cbbf07ee8304a2c720")
        );
    }

    // Ported: "returns oci.pull dependencies with registryAliases with multiple segments" — bazel-module/extract.spec.ts line 682
    #[test]
    fn extracts_oci_pull_dependency_with_multisegment_registry_alias() {
        let input = r#"
oci.pull(
    name = "custom_image",
    digest = "sha256:1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
    image = "quay.io/myorg/myapp",
    platforms = ["linux/amd64"],
    tag = "v2.0.0",
)
"#;
        let deps = extract_oci_pull_deps(input, &[("quay.io", "my-registry.com/mirror/quay.io")]);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "custom_image");
        assert_eq!(
            deps[0].package_name,
            "my-registry.com/mirror/quay.io/myorg/myapp"
        );
        assert_eq!(deps[0].current_value.as_deref(), Some("v2.0.0"));
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("sha256:1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef")
        );
    }

    // Ported: "returns maven.install and bazel_dep dependencies together" — bazel-module/extract.spec.ts line 723
    #[test]
    fn extracts_maven_install_and_bazel_dep_together() {
        let input = r#"
bazel_dep(name = "bazel_jar_jar", version = "0.1.0")

maven = use_extension("@rules_jvm_external//:extensions.bzl", "maven")

maven.install(
    artifacts = [
        "junit:junit:4.13.2",
        "com.google.guava:guava:31.1-jre",
    ],
    lock_file = "//:maven_install.json",
    repositories = [
        "https://repo1.maven.org/maven2/",
    ],
    version_conflict_policy = "pinned",
)
"#;
        let bazel_deps = extract(input);
        assert_eq!(bazel_deps.len(), 1);
        assert_eq!(bazel_deps[0].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(bazel_deps[0].name, "bazel_jar_jar");
        assert_eq!(bazel_deps[0].current_value, "0.1.0");
        assert!(bazel_deps[0].skip_reason.is_none());

        let maven_deps = extract_maven_deps(input);
        assert_eq!(maven_deps.len(), 2);
        assert_eq!(maven_deps[0].dep_name, "junit:junit");
        assert_eq!(maven_deps[0].current_value, "4.13.2");
        assert_eq!(maven_deps[0].dep_type, "maven_install");
        assert_eq!(
            maven_deps[0].registry_urls,
            vec!["https://repo1.maven.org/maven2/"]
        );
        assert_eq!(maven_deps[1].dep_name, "com.google.guava:guava");
        assert_eq!(maven_deps[1].current_value, "31.1-jre");
        assert_eq!(maven_deps[1].dep_type, "maven_install");
        assert_eq!(
            maven_deps[1].registry_urls,
            vec!["https://repo1.maven.org/maven2/"]
        );
    }

    // Ported: "returns git_repository dependencies with digest" — bazel-module/extract.spec.ts line 772
    #[test]
    fn extracts_git_repository_dependency_with_digest() {
        let input = r#"
git_repository(
    name = "rules_foo",
    commit = "850cb49c8649e463b80ef7984e7c744279746170",
    remote = "https://github.com/example/rules_foo.git"
)
"#;
        let deps = extract_git_repository_deps(input);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_type, "git_repository");
        assert_eq!(deps[0].dep_name, "rules_foo");
        assert_eq!(deps[0].datasource, "github-tags");
        assert_eq!(deps[0].package_name.as_deref(), Some("example/rules_foo"));
        assert_eq!(deps[0].current_value, None);
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("850cb49c8649e463b80ef7984e7c744279746170")
        );
    }

    // Ported: "returns git_repository dependencies with tag" — bazel-module/extract.spec.ts line 796
    #[test]
    fn extracts_git_repository_dependency_with_tag() {
        let input = r#"
git_repository(
    name = "rules_foo",
    tag = "1.2.3",
    remote = "https://github.com/example/rules_foo.git"
)
"#;
        let deps = extract_git_repository_deps(input);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_type, "git_repository");
        assert_eq!(deps[0].dep_name, "rules_foo");
        assert_eq!(deps[0].datasource, "github-tags");
        assert_eq!(deps[0].package_name.as_deref(), Some("example/rules_foo"));
        assert_eq!(deps[0].current_value.as_deref(), Some("1.2.3"));
        assert_eq!(deps[0].current_digest, None);
    }

    // Ported: "returns new_git_repository dependencies" — bazel-module/extract.spec.ts line 820
    #[test]
    fn extracts_new_git_repository_dependency() {
        let input = r#"
new_git_repository(
    name = "rules_foo",
    commit = "850cb49c8649e463b80ef7984e7c744279746170",
    remote = "https://github.com/example/rules_foo.git",
    tag = "1.2.3"
)
"#;
        let deps = extract_git_repository_deps(input);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_type, "new_git_repository");
        assert_eq!(deps[0].dep_name, "rules_foo");
        assert_eq!(deps[0].datasource, "github-tags");
        assert_eq!(deps[0].package_name.as_deref(), Some("example/rules_foo"));
        assert_eq!(deps[0].current_value.as_deref(), Some("1.2.3"));
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("850cb49c8649e463b80ef7984e7c744279746170")
        );
    }

    // Ported: "handles a real-world MODULE.bazel file (rules_sh)" — bazel-module/extract.spec.ts line 846
    #[test]
    fn extracts_rules_sh_real_world_module_bazel() {
        let input = r#"
module(
    name = "rules_sh",
    version = "0.5.0",
    compatibility_level = 0,
)
bazel_dep(name = "bazel_skylib", version = "1.2.1")
bazel_dep(name = "platforms", version = "0.0.8")
bazel_dep(name = "stardoc", version = "0.6.2", dev_dependency = True, repo_name = "io_bazel_stardoc")
sh_configure = use_extension("//bzlmod:extensions.bzl", "sh_configure")
use_repo(sh_configure, "local_posix_config", "rules_sh_shim_exe")
register_toolchains("@local_posix_config//...")
"#;
        let deps = extract(input);
        assert_eq!(deps.len(), 3);
        assert_eq!(deps[0].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(deps[0].name, "bazel_skylib");
        assert_eq!(deps[0].current_value, "1.2.1");
        assert!(!deps[0].dev_dependency);
        assert_eq!(deps[1].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(deps[1].name, "platforms");
        assert_eq!(deps[1].current_value, "0.0.8");
        assert!(!deps[1].dev_dependency);
        assert_eq!(deps[2].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(deps[2].name, "stardoc");
        assert_eq!(deps[2].current_value, "0.6.2");
        assert!(deps[2].dev_dependency);
        assert!(deps.iter().all(|dep| dep.skip_reason.is_none()));
    }

    // Ported: "returns bazel_dep and archive_override dependencies" — bazel-module/extract.spec.ts line 148
    #[test]
    fn extracts_archive_override_with_bazel_dep_version() {
        let content = r#"
bazel_dep(name = "rules_foo", version = "1.2.3")
archive_override(
  module_name = "rules_foo",
  urls = [
    "https://example.com/archive.tar.gz",
  ],
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(deps[0].name, "rules_foo");
        assert_eq!(deps[0].current_value, "1.2.3");
        assert_eq!(deps[0].skip_reason, Some(BazelSkipReason::FileDependency));
        assert_eq!(deps[1].dep_type, BazelModuleDepType::ArchiveOverride);
        assert_eq!(deps[1].name, "rules_foo");
        assert_eq!(
            deps[1].skip_reason,
            Some(BazelSkipReason::UnsupportedDatasource)
        );
    }

    // Ported: "returns bazel_dep with no version and archive_override dependencies" — bazel-module/extract.spec.ts line 179
    #[test]
    fn extracts_archive_override_with_unversioned_bazel_dep() {
        let content = r#"
bazel_dep(name = "rules_foo")
archive_override(
  module_name = "rules_foo",
  urls = [
    "https://example.com/archive.tar.gz",
  ],
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(deps[0].name, "rules_foo");
        assert!(deps[0].current_value.is_empty());
        assert_eq!(deps[0].skip_reason, Some(BazelSkipReason::FileDependency));
        assert_eq!(deps[1].dep_type, BazelModuleDepType::ArchiveOverride);
        assert_eq!(deps[1].name, "rules_foo");
        assert_eq!(
            deps[1].skip_reason,
            Some(BazelSkipReason::UnsupportedDatasource)
        );
    }

    // Ported: "returns bazel_dep and local_path_override dependencies" — bazel-module/extract.spec.ts line 209
    #[test]
    fn extracts_local_path_override_with_bazel_dep_version() {
        let content = r#"
bazel_dep(name = "rules_foo", version = "1.2.3")
local_path_override(
  module_name = "rules_foo",
  urls = "/path/to/repo",
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(deps[0].name, "rules_foo");
        assert_eq!(deps[0].current_value, "1.2.3");
        assert_eq!(deps[0].skip_reason, Some(BazelSkipReason::LocalDependency));
        assert_eq!(deps[1].dep_type, BazelModuleDepType::LocalPathOverride);
        assert_eq!(deps[1].name, "rules_foo");
        assert_eq!(
            deps[1].skip_reason,
            Some(BazelSkipReason::UnsupportedDatasource)
        );
    }

    // Ported: "returns bazel_dep with no version and local_path_override dependencies" — bazel-module/extract.spec.ts line 238
    #[test]
    fn extracts_local_path_override_with_unversioned_bazel_dep() {
        let content = r#"
bazel_dep(name = "rules_foo")
local_path_override(
  module_name = "rules_foo",
  urls = "/path/to/repo",
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(deps[0].name, "rules_foo");
        assert!(deps[0].current_value.is_empty());
        assert_eq!(deps[0].skip_reason, Some(BazelSkipReason::LocalDependency));
        assert_eq!(deps[1].dep_type, BazelModuleDepType::LocalPathOverride);
        assert_eq!(deps[1].name, "rules_foo");
        assert_eq!(
            deps[1].skip_reason,
            Some(BazelSkipReason::UnsupportedDatasource)
        );
    }

    // Ported: "returns bazel_dep and single_version_override dependencies if a version is specified" — bazel-module/extract.spec.ts line 266
    #[test]
    fn extracts_single_version_override_with_bazel_dep_version() {
        let content = r#"
bazel_dep(name = "rules_foo", version = "1.2.3")
single_version_override(
  module_name = "rules_foo",
  version = "1.2.5",
  registry = "https://example.com/custom_registry",
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(deps[0].name, "rules_foo");
        assert_eq!(deps[0].current_value, "1.2.3");
        assert_eq!(deps[0].skip_reason, Some(BazelSkipReason::IsPinned));
        assert_eq!(
            deps[0].registry_urls,
            vec!["https://example.com/custom_registry".to_owned()]
        );
        assert_eq!(deps[1].dep_type, BazelModuleDepType::SingleVersionOverride);
        assert_eq!(deps[1].name, "rules_foo");
        assert_eq!(deps[1].current_value, "1.2.5");
        assert_eq!(deps[1].skip_reason, Some(BazelSkipReason::Ignored));
        assert_eq!(
            deps[1].registry_urls,
            vec!["https://example.com/custom_registry".to_owned()]
        );
    }

    // Ported: "returns bazel_dep with no version and single_version_override dependencies if a version is specified" — bazel-module/extract.spec.ts line 299
    #[test]
    fn extracts_single_version_override_with_unversioned_bazel_dep() {
        let content = r#"
bazel_dep(name = "rules_foo")
single_version_override(
  module_name = "rules_foo",
  version = "1.2.3",
  registry = "https://example.com/custom_registry",
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(deps[0].name, "rules_foo");
        assert!(deps[0].current_value.is_empty());
        assert_eq!(deps[0].skip_reason, Some(BazelSkipReason::IsPinned));
        assert_eq!(
            deps[0].registry_urls,
            vec!["https://example.com/custom_registry".to_owned()]
        );
        assert_eq!(deps[1].dep_type, BazelModuleDepType::SingleVersionOverride);
        assert_eq!(deps[1].name, "rules_foo");
        assert_eq!(deps[1].current_value, "1.2.3");
        assert_eq!(deps[1].skip_reason, Some(BazelSkipReason::Ignored));
    }

    // Ported: "returns bazel_dep dependency if single_version_override does not have a version" — bazel-module/extract.spec.ts line 331
    #[test]
    fn single_version_override_without_version_only_adds_registry_to_versioned_bazel_dep() {
        let content = r#"
bazel_dep(name = "rules_foo", version = "1.2.3")
single_version_override(
  module_name = "rules_foo",
  registry = "https://example.com/custom_registry",
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(deps[0].name, "rules_foo");
        assert_eq!(deps[0].current_value, "1.2.3");
        assert!(deps[0].skip_reason.is_none());
        assert_eq!(
            deps[0].registry_urls,
            vec!["https://example.com/custom_registry".to_owned()]
        );
    }

    // Ported: "returns bazel_dep with no version dependency if single_version_override does not have a version" — bazel-module/extract.spec.ts line 355
    #[test]
    fn single_version_override_without_version_keeps_unversioned_bazel_dep_skipped() {
        let content = r#"
bazel_dep(name = "rules_foo")
single_version_override(
  module_name = "rules_foo",
  registry = "https://example.com/custom_registry",
)
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_type, BazelModuleDepType::BazelDep);
        assert_eq!(deps[0].name, "rules_foo");
        assert!(deps[0].current_value.is_empty());
        assert_eq!(
            deps[0].skip_reason,
            Some(BazelSkipReason::UnspecifiedVersion)
        );
        assert_eq!(
            deps[0].registry_urls,
            vec!["https://example.com/custom_registry".to_owned()]
        );
    }

    // Ported: "returns null if file is empty" — bazel-module/extract.spec.ts line 41
    #[test]
    fn empty_content_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "returns null if fails to parse" — bazel-module/extract.spec.ts line 25
    #[test]
    fn malformed_content_returns_empty() {
        assert!(extract("blahhhhh:foo:@what\n").is_empty());
    }

    // Ported: "returns null if file has unrecognized declarations" — bazel-module/extract.spec.ts line 46
    #[test]
    fn comment_lines_stripped() {
        let content = r#"
# This is a comment
bazel_dep(name = "rules_go", version = "0.41.0")  # inline comment
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "rules_go");
    }

    // Ported: "returns null if file has unrecognized declarations" — bazel-module/extract.spec.ts line 46
    #[test]
    fn empty_file_returns_empty() {
        assert!(extract("module(name = \"mymodule\", version = \"1.0.0\")\n").is_empty());
    }
}
