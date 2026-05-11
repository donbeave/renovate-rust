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

/// Extracts `name = "value"` or `name = 'value'` from a call argument list.
static ATTR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(\w+)\s*=\s*['"]([^'"]+)['"]"#).unwrap());

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
