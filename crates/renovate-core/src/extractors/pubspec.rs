//! Dart/Flutter `pubspec.yaml` dependency extractor.
//!
//! Parses `pubspec.yaml` files using an indentation-aware line scanner and
//! returns package dependencies ready for pub.dev version lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/pub/extract.ts`   — `extractPackageFile`
//! - `lib/modules/manager/pub/schema.ts`    — `PubspecDependency`
//!
//! ## Supported sections
//!
//! | Section | Dep type |
//! |---|---|
//! | `dependencies` | `Regular` |
//! | `dev_dependencies` | `Dev` |
//!
//! ## Value forms
//!
//! | YAML form | Treatment |
//! |---|---|
//! | `http: ^0.13.4` | Actionable — version `^0.13.4` |
//! | `mypkg:\n  version: ^1.0.0` | Actionable — version `^1.0.0` |
//! | `flutter:\n  sdk: flutter` | Skipped — `SdkDep` |
//! | `mypkg:\n  git: …` | Skipped — `GitSource` |
//! | `mypkg:\n  path: …` | Skipped — `LocalPath` |
//! | `mypkg:\n  hosted: …` | Actionable — uses `version` if present |

use serde::Deserialize;

/// Parsed pubspec.yaml contents (schema-validated).
///
/// Mirrors `lib/modules/manager/pub/schema.ts` `Pubspec`.
#[derive(Debug, Deserialize)]
pub struct ParsedPubspec {
    pub environment: PubspecEnvironment,
    pub dependencies: Option<std::collections::HashMap<String, serde_yaml::Value>>,
    pub dev_dependencies: Option<std::collections::HashMap<String, serde_yaml::Value>>,
}

#[derive(Debug, Deserialize)]
pub struct PubspecEnvironment {
    pub sdk: String,
    pub flutter: Option<String>,
}

/// Parsed pubspec.lock contents (schema-validated).
///
/// Mirrors `lib/modules/manager/pub/schema.ts` `PubspecLock`.
#[derive(Debug, Deserialize)]
pub struct ParsedPubspecLock {
    pub sdks: PubspecLockSdks,
}

#[derive(Debug, Deserialize)]
pub struct PubspecLockSdks {
    pub dart: String,
    pub flutter: Option<String>,
}

/// Parse and schema-validate a pubspec.yaml string.
///
/// Mirrors `lib/modules/manager/pub/utils.ts` `parsePubspec()`.
pub fn parse_pubspec(_file_name: &str, content: &str) -> Option<ParsedPubspec> {
    serde_yaml::from_str(content).ok()
}

/// Parse and schema-validate a pubspec.lock string.
///
/// Mirrors `lib/modules/manager/pub/utils.ts` `parsePubspecLock()`.
pub fn parse_pubspec_lock(_file_name: &str, content: &str) -> Option<ParsedPubspecLock> {
    serde_yaml::from_str(content).ok()
}

/// Which `pubspec.yaml` section the dep came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PubspecDepType {
    /// `dependencies:` section.
    Regular,
    /// `dev_dependencies:` section.
    Dev,
}

impl PubspecDepType {
    /// Return the Renovate-canonical `depType` string.
    pub fn as_renovate_str(&self) -> &'static str {
        match self {
            PubspecDepType::Regular => "dependencies",
            PubspecDepType::Dev => "dev_dependencies",
        }
    }
}

/// Why a pub package is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PubspecSkipReason {
    /// Package references a Dart/Flutter SDK (`{sdk: flutter}`).
    SdkDep,
    /// Package is a VCS (git) source.
    GitSource,
    /// Package is a local path source.
    LocalPath,
}

/// A single extracted pub dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PubspecExtractedDep {
    /// Package name (e.g. `http`).
    pub name: String,
    /// Version constraint (e.g. `^0.13.4`). Empty = unconstrained (`any`).
    pub current_value: String,
    /// Which section this dep came from.
    pub dep_type: PubspecDepType,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<PubspecSkipReason>,
}

// ── Full extraction (mirrors pub/extract.ts) ─────────────────────────────────

/// Datasource IDs used when producing `FullPubDep`.
pub mod datasource {
    pub const DART: &str = "dart";
    pub const DART_VERSION: &str = "dart-version";
    pub const FLUTTER_VERSION: &str = "flutter-version";
    pub const GIT_REFS: &str = "git-refs";
}

/// A fully extracted pub dependency with the fields expected by the Renovate engine.
///
/// Mirrors the `PackageDependency` shape returned by
/// `lib/modules/manager/pub/extract.ts` `extractPackageFile()`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FullPubDep {
    pub dep_name: String,
    pub dep_type: Option<String>,
    pub current_value: String,
    pub datasource: &'static str,
    pub skip_reason: Option<&'static str>,
    pub registry_urls: Option<Vec<String>>,
    pub package_name: Option<String>,
}

/// Return value of `extract_package_file`.
#[derive(Debug)]
pub struct PubExtractResult {
    pub deps: Vec<FullPubDep>,
}

/// Parse a `pubspec.yaml` file and return all actionable + SDK dependencies.
///
/// Returns `None` when the file is not valid YAML or does not contain a
/// required `environment.sdk` field.
///
/// Mirrors `lib/modules/manager/pub/extract.ts` `extractPackageFile()`.
pub fn extract_package_file(content: &str, _package_file: &str) -> Option<PubExtractResult> {
    let root_val: serde_yaml::Value = serde_yaml::from_str(content).ok()?;
    let root = root_val.as_mapping()?;

    let env = root.get("environment")?.as_mapping()?;
    let sdk = env.get("sdk")?.as_str()?.to_owned();
    let flutter = env
        .get("flutter")
        .and_then(|v| v.as_str())
        .map(str::to_owned);

    const SKIPPED: &[&str] = &[
        "flutter_driver",
        "flutter_localizations",
        "flutter_test",
        "flutter_web_plugins",
        "meta",
    ];

    let mut deps: Vec<FullPubDep> = Vec::new();

    for section_key in &["dependencies", "dev_dependencies"] {
        if let Some(section) = root.get(*section_key).and_then(|v| v.as_mapping()) {
            for (k, v) in section {
                let Some(dep_name) = k.as_str() else {
                    continue;
                };
                if SKIPPED.contains(&dep_name) {
                    continue;
                }
                if let Some(dep) = pub_process_dep(dep_name, section_key, v) {
                    deps.push(dep);
                }
            }
        }
    }

    deps.push(FullPubDep {
        dep_name: "dart".into(),
        dep_type: None,
        current_value: sdk,
        datasource: datasource::DART_VERSION,
        skip_reason: None,
        registry_urls: None,
        package_name: None,
    });

    if let Some(flutter_val) = flutter {
        deps.push(FullPubDep {
            dep_name: "flutter".into(),
            dep_type: None,
            current_value: flutter_val,
            datasource: datasource::FLUTTER_VERSION,
            skip_reason: None,
            registry_urls: None,
            package_name: None,
        });
    }

    Some(PubExtractResult { deps })
}

fn pub_process_dep(
    dep_name: &str,
    section_key: &str,
    value: &serde_yaml::Value,
) -> Option<FullPubDep> {
    use serde_yaml::Value;

    let dep_type = Some(section_key.to_owned());

    match value {
        Value::String(v) => Some(FullPubDep {
            dep_name: dep_name.to_owned(),
            dep_type,
            current_value: v.clone(),
            datasource: datasource::DART,
            skip_reason: None,
            registry_urls: None,
            package_name: None,
        }),

        Value::Mapping(m) => {
            let version = m.get("version").and_then(|v| v.as_str());
            let path = m.get("path").and_then(|v| v.as_str());
            let git = m.get("git");
            let hosted = m.get("hosted");

            let git_url: Option<String> = match git {
                Some(Value::String(s)) => Some(s.clone()),
                Some(Value::Mapping(gm)) => {
                    gm.get("url").and_then(|v| v.as_str()).map(str::to_owned)
                }
                _ => None,
            };

            let registry_urls: Option<Vec<String>> = match hosted {
                Some(Value::String(s)) => Some(vec![s.clone()]),
                Some(Value::Mapping(hm)) => hm
                    .get("url")
                    .and_then(|v| v.as_str())
                    .map(|s| vec![s.to_owned()]),
                _ => None,
            };

            let (current_value, skip_reason): (String, Option<&'static str>) =
                if let Some(v) = version {
                    (v.to_owned(), None)
                } else if path.is_some() {
                    (String::new(), Some("path-dependency"))
                } else {
                    match git {
                        Some(Value::Mapping(gm)) => {
                            let git_ref = gm.get("ref").and_then(|v| v.as_str());
                            if let Some(r) = git_ref {
                                (r.to_owned(), None)
                            } else {
                                (String::new(), Some("unspecified-version"))
                            }
                        }
                        Some(Value::String(_)) => (String::new(), Some("unspecified-version")),
                        _ => (String::new(), None),
                    }
                };

            if let Some(url) = git_url {
                Some(FullPubDep {
                    dep_name: dep_name.to_owned(),
                    dep_type,
                    current_value,
                    datasource: datasource::GIT_REFS,
                    skip_reason,
                    registry_urls: None,
                    package_name: Some(url),
                })
            } else {
                Some(FullPubDep {
                    dep_name: dep_name.to_owned(),
                    dep_type,
                    current_value,
                    datasource: datasource::DART,
                    skip_reason,
                    registry_urls,
                    package_name: None,
                })
            }
        }

        // Boolean, number, null, sequence → skip this dep entirely.
        _ => None,
    }
}

// ── Legacy line-scanner API ───────────────────────────────────────────────────

/// Parse a `pubspec.yaml` file and extract all package dependencies.
pub fn extract(content: &str) -> Vec<PubspecExtractedDep> {
    let mut deps = Vec::new();

    // State
    let mut section: Option<PubspecDepType> = None;
    // Pending multi-line dep being assembled.
    let mut pending: Option<PendingDep> = None;

    for line in content.lines() {
        // Skip blank lines and comments.
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let indent = leading_spaces(line);

        // Section headers are at indent 0.
        if indent == 0 {
            // Flush any pending dep before switching sections.
            if let Some(p) = pending.take() {
                emit_pending(p, &mut deps);
            }
            match trimmed {
                "dependencies:" => section = Some(PubspecDepType::Regular),
                "dev_dependencies:" => section = Some(PubspecDepType::Dev),
                _ => {
                    // Any other top-level key ends the dependency sections.
                    // But we only reset section if it looks like a YAML key.
                    if trimmed.ends_with(':') || trimmed.contains(": ") {
                        section = None;
                    }
                }
            }
            continue;
        }

        let Some(dep_type) = section else {
            continue;
        };

        if indent == 2 {
            // Flush previous pending dep.
            if let Some(p) = pending.take() {
                emit_pending(p, &mut deps);
            }

            if let Some((name, value)) = split_kv(trimmed) {
                let value = value.trim();
                if value.is_empty() {
                    // Multi-line dep: `  name:` with properties on next lines.
                    pending = Some(PendingDep {
                        name: name.to_owned(),
                        dep_type,
                        version: String::new(),
                        skip_reason: None,
                    });
                } else if value == "any" {
                    // No version constraint.
                    deps.push(PubspecExtractedDep {
                        name: name.to_owned(),
                        current_value: String::new(),
                        dep_type,
                        skip_reason: None,
                    });
                } else {
                    // Simple inline version: `  http: ^0.13.4`
                    deps.push(PubspecExtractedDep {
                        name: name.to_owned(),
                        current_value: value.to_owned(),
                        dep_type,
                        skip_reason: None,
                    });
                }
            }
        } else if indent >= 4 {
            // Property of a multi-line dep.
            if let Some(ref mut p) = pending
                && let Some((key, val)) = split_kv(trimmed)
            {
                let val = val.trim();
                match key {
                    "sdk" => p.skip_reason = Some(PubspecSkipReason::SdkDep),
                    "git" => p.skip_reason = Some(PubspecSkipReason::GitSource),
                    "path" => p.skip_reason = Some(PubspecSkipReason::LocalPath),
                    "version" if !val.is_empty() && p.version.is_empty() => {
                        p.version = val.to_owned();
                    }
                    _ => {}
                }
            }
        }
    }

    // Flush final pending dep.
    if let Some(p) = pending.take() {
        emit_pending(p, &mut deps);
    }

    deps
}

// ── Helpers ───────────────────────────────────────────────────────────────

struct PendingDep {
    name: String,
    dep_type: PubspecDepType,
    version: String,
    skip_reason: Option<PubspecSkipReason>,
}

fn emit_pending(p: PendingDep, deps: &mut Vec<PubspecExtractedDep>) {
    deps.push(PubspecExtractedDep {
        name: p.name,
        current_value: p.version,
        dep_type: p.dep_type,
        skip_reason: p.skip_reason,
    });
}

fn leading_spaces(line: &str) -> usize {
    line.len() - line.trim_start().len()
}

/// Split `"key: value"` or `"key:"` into `(key, value)`.
fn split_kv(s: &str) -> Option<(&str, &str)> {
    let colon = s.find(':')?;
    let key = s[..colon].trim();
    let value = s[colon + 1..].trim();
    if key.is_empty() {
        return None;
    }
    Some((key, value))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_regular(content: &str) -> Vec<PubspecExtractedDep> {
        extract(content)
            .into_iter()
            .filter(|d| d.dep_type == PubspecDepType::Regular)
            .collect()
    }

    // Rust-specific: unit test for simple inline version extraction
    #[test]
    fn simple_inline_versions() {
        let content = r#"
dependencies:
  http: ^0.13.4
  provider: ^6.0.5
  any_pkg: any
"#;
        let deps = extract_regular(content);
        assert_eq!(deps.len(), 3);

        let http = deps.iter().find(|d| d.name == "http").unwrap();
        assert_eq!(http.current_value, "^0.13.4");
        assert!(http.skip_reason.is_none());

        let any = deps.iter().find(|d| d.name == "any_pkg").unwrap();
        assert!(any.current_value.is_empty());
        assert!(any.skip_reason.is_none());
    }

    // Rust-specific: unit test for SDK dependency skipping
    #[test]
    fn sdk_dep_skipped() {
        let content = r#"
dependencies:
  flutter:
    sdk: flutter
  http: ^0.13.4
"#;
        let deps = extract_regular(content);
        assert_eq!(deps.len(), 2);
        let flutter = deps.iter().find(|d| d.name == "flutter").unwrap();
        assert_eq!(flutter.skip_reason, Some(PubspecSkipReason::SdkDep));
    }

    // Rust-specific: unit test for git dependency skipping
    #[test]
    fn git_dep_skipped() {
        let content = r#"
dependencies:
  mypkg:
    git:
      url: https://github.com/example/mypkg.git
"#;
        let deps = extract_regular(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(PubspecSkipReason::GitSource));
    }

    // Rust-specific: unit test for path dependency skipping
    #[test]
    fn path_dep_skipped() {
        let content = r#"
dependencies:
  local_pkg:
    path: ../localpackage
"#;
        let deps = extract_regular(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(PubspecSkipReason::LocalPath));
    }

    // Rust-specific: unit test for version object form
    #[test]
    fn version_object_form() {
        let content = r#"
dependencies:
  mypkg:
    version: ^1.0.0
"#;
        let deps = extract_regular(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "mypkg");
        assert_eq!(deps[0].current_value, "^1.0.0");
        assert!(deps[0].skip_reason.is_none());
    }

    // Rust-specific: unit test for dev_dependencies extraction
    #[test]
    fn dev_dependencies_extracted() {
        let content = r#"
dev_dependencies:
  flutter_test:
    sdk: flutter
  test: ^1.16.0
  mockito: ^5.4.0
"#;
        let deps: Vec<_> = extract(content)
            .into_iter()
            .filter(|d| d.dep_type == PubspecDepType::Dev)
            .collect();
        assert_eq!(deps.len(), 3);

        let flutter_test = deps.iter().find(|d| d.name == "flutter_test").unwrap();
        assert_eq!(flutter_test.skip_reason, Some(PubspecSkipReason::SdkDep));

        let test_dep = deps.iter().find(|d| d.name == "test").unwrap();
        assert_eq!(test_dep.current_value, "^1.16.0");
        assert!(test_dep.skip_reason.is_none());
    }

    // Rust-specific: unit test for real-world pubspec extraction
    #[test]
    fn real_world_pubspec() {
        let content = r#"
name: myapp
version: 1.0.0+1

environment:
  sdk: '>=2.12.0 <3.0.0'
  flutter: '>=2.0.0'

dependencies:
  flutter:
    sdk: flutter
  http: ^0.13.4
  provider: ^6.0.5
  shared_preferences: ^2.0.0
  git_dep:
    git:
      url: https://github.com/example/mypkg.git
  local_dep:
    path: ../mypkg
  versioned_obj:
    version: ^3.0.0

dev_dependencies:
  flutter_test:
    sdk: flutter
  mockito: ^5.0.0
"#;
        let all_deps = extract(content);
        let regular: Vec<_> = all_deps
            .iter()
            .filter(|d| d.dep_type == PubspecDepType::Regular)
            .collect();

        assert_eq!(regular.len(), 7);
        assert_eq!(
            all_deps
                .iter()
                .filter(|d| d.dep_type == PubspecDepType::Dev)
                .count(),
            2
        );

        // Flutter SDK dep skipped
        let flutter = regular.iter().find(|d| d.name == "flutter").unwrap();
        assert_eq!(flutter.skip_reason, Some(PubspecSkipReason::SdkDep));

        // http actionable
        let http = regular.iter().find(|d| d.name == "http").unwrap();
        assert_eq!(http.current_value, "^0.13.4");
        assert!(http.skip_reason.is_none());

        // git skipped
        let git = regular.iter().find(|d| d.name == "git_dep").unwrap();
        assert_eq!(git.skip_reason, Some(PubspecSkipReason::GitSource));

        // path skipped
        let local = regular.iter().find(|d| d.name == "local_dep").unwrap();
        assert_eq!(local.skip_reason, Some(PubspecSkipReason::LocalPath));

        // versioned object form
        let versioned = regular.iter().find(|d| d.name == "versioned_obj").unwrap();
        assert_eq!(versioned.current_value, "^3.0.0");
        assert!(versioned.skip_reason.is_none());
    }

    // Rust-specific: unit test for empty pubspec handling
    #[test]
    fn empty_pubspec_returns_empty() {
        let content = "name: myapp\nversion: 1.0.0\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "load and parse successfully" — modules/manager/pub/utils.spec.ts line 14
    #[test]
    fn parse_pubspec_loads_valid_yaml() {
        let content = "environment:\n  sdk: \">=3.0.0 <4.0.0\"\n  flutter: \">=3.10.0\"\ndependencies:\n  dep1: 1.0.0\ndev_dependencies:\n  dep2: 1.0.1\n";
        let result = parse_pubspec("pubspec.yaml", content).unwrap();
        assert_eq!(result.environment.sdk, ">=3.0.0 <4.0.0");
        assert_eq!(result.environment.flutter.as_deref(), Some(">=3.10.0"));
        assert!(result.dependencies.as_ref().unwrap().contains_key("dep1"));
        assert!(
            result
                .dev_dependencies
                .as_ref()
                .unwrap()
                .contains_key("dep2")
        );
    }

    // Ported: "invalid yaml" — modules/manager/pub/utils.spec.ts line 32
    #[test]
    fn parse_pubspec_invalid_yaml_returns_none() {
        let content = "clearly: \"invalid\" \"yaml\"\n";
        assert!(parse_pubspec("pubspec.yaml", content).is_none());
    }

    // Ported: "invalid schema" — modules/manager/pub/utils.spec.ts line 37
    #[test]
    fn parse_pubspec_invalid_schema_returns_none() {
        let content = "clearly: invalid\n";
        assert!(parse_pubspec("pubspec.yaml", content).is_none());
    }

    // Ported: "load and parse successfully" — modules/manager/pub/utils.spec.ts line 44
    #[test]
    fn parse_pubspec_lock_loads_valid_yaml() {
        let content = "sdks:\n  dart: \">=3.0.0 <4.0.0\"\n  flutter: \">=3.10.0\"\n";
        let result = parse_pubspec_lock("pubspec.lock", content).unwrap();
        assert_eq!(result.sdks.dart, ">=3.0.0 <4.0.0");
        assert_eq!(result.sdks.flutter.as_deref(), Some(">=3.10.0"));
    }

    // Ported: "invalid yaml" — modules/manager/pub/utils.spec.ts line 56
    #[test]
    fn parse_pubspec_lock_invalid_yaml_returns_none() {
        let content = "clearly: \"invalid\" \"yaml\"\n";
        assert!(parse_pubspec_lock("pubspec.lock", content).is_none());
    }

    // Ported: "invalid schema" — modules/manager/pub/utils.spec.ts line 61
    #[test]
    fn parse_pubspec_lock_invalid_schema_returns_none() {
        let content = "clearly: invalid\n";
        assert!(parse_pubspec_lock("pubspec.lock", content).is_none());
    }

    // Ported: "returns null for invalid pubspec file" — modules/manager/pub/extract.spec.ts line 8
    #[test]
    fn pub_extract_returns_none_for_invalid_yaml() {
        let content = "clarly: \"invalid\" \"yaml\"\n";
        assert!(extract_package_file(content, "pubspec.yaml").is_none());
    }

    // Ported: "returns dart sdk only" — modules/manager/pub/extract.spec.ts line 16
    #[test]
    fn pub_extract_returns_dart_sdk_only() {
        let content = "environment:\n  sdk: ^3.0.0\n";
        let result = extract_package_file(content, "pubspec.yaml").unwrap();
        assert_eq!(result.deps.len(), 1);
        assert_eq!(result.deps[0].dep_name, "dart");
        assert_eq!(result.deps[0].current_value, "^3.0.0");
        assert_eq!(result.deps[0].datasource, datasource::DART_VERSION);
        assert!(result.deps[0].dep_type.is_none());
    }

    // Ported: "returns valid dependencies" — modules/manager/pub/extract.spec.ts line 33
    #[test]
    fn pub_extract_returns_valid_dependencies() {
        let content = r#"
environment:
  sdk: ^3.0.0
  flutter: 2.0.0
dependencies:
  meta: 'something'
  foo: 1.0.0
  transmogrify:
    hosted:
      name: transmogrify
      url: https://some-package-server.com
    version: ^1.4.0
  bar:
    hosted: 'some-url'
    version: 1.1.0
  baz:
    non-sense: true
  qux: false
  path_dep:
    path: path1
  git_package:
    git:
      url: https://github.com/some-url/some-package
  git_package_ref:
    git:
      url: https://github.com/some-url/some-package-ref
      ref: v1.0.0
  git_package_version:
    git:
      url: https://github.com/some-url/some-package-version
    version: ^1.1.0
  git_package_version_git_url:
    git: https://github.com/some-url/some-package-version-url
    version: ^1.1.0
dev_dependencies:
  test: ^0.1.0
  build:
    version: 0.0.1
  flutter_test:
    sdk: flutter
  path_dev_dep:
    path: path2
"#;
        let result = extract_package_file(content, "pubspec.yaml").unwrap();
        let deps = &result.deps;

        // Check ordering and values match expected output
        assert_eq!(
            deps[0],
            FullPubDep {
                dep_name: "foo".into(),
                dep_type: Some("dependencies".into()),
                current_value: "1.0.0".into(),
                datasource: datasource::DART,
                skip_reason: None,
                registry_urls: None,
                package_name: None
            }
        );
        assert_eq!(
            deps[1],
            FullPubDep {
                dep_name: "transmogrify".into(),
                dep_type: Some("dependencies".into()),
                current_value: "^1.4.0".into(),
                datasource: datasource::DART,
                skip_reason: None,
                registry_urls: Some(vec!["https://some-package-server.com".into()]),
                package_name: None
            }
        );
        assert_eq!(
            deps[2],
            FullPubDep {
                dep_name: "bar".into(),
                dep_type: Some("dependencies".into()),
                current_value: "1.1.0".into(),
                datasource: datasource::DART,
                skip_reason: None,
                registry_urls: Some(vec!["some-url".into()]),
                package_name: None
            }
        );
        assert_eq!(
            deps[3],
            FullPubDep {
                dep_name: "baz".into(),
                dep_type: Some("dependencies".into()),
                current_value: "".into(),
                datasource: datasource::DART,
                skip_reason: None,
                registry_urls: None,
                package_name: None
            }
        );
        assert_eq!(
            deps[4],
            FullPubDep {
                dep_name: "path_dep".into(),
                dep_type: Some("dependencies".into()),
                current_value: "".into(),
                datasource: datasource::DART,
                skip_reason: Some("path-dependency"),
                registry_urls: None,
                package_name: None
            }
        );
        assert_eq!(
            deps[5],
            FullPubDep {
                dep_name: "git_package".into(),
                dep_type: Some("dependencies".into()),
                current_value: "".into(),
                datasource: datasource::GIT_REFS,
                skip_reason: Some("unspecified-version"),
                registry_urls: None,
                package_name: Some("https://github.com/some-url/some-package".into())
            }
        );
        assert_eq!(
            deps[6],
            FullPubDep {
                dep_name: "git_package_ref".into(),
                dep_type: Some("dependencies".into()),
                current_value: "v1.0.0".into(),
                datasource: datasource::GIT_REFS,
                skip_reason: None,
                registry_urls: None,
                package_name: Some("https://github.com/some-url/some-package-ref".into())
            }
        );
        assert_eq!(
            deps[7],
            FullPubDep {
                dep_name: "git_package_version".into(),
                dep_type: Some("dependencies".into()),
                current_value: "^1.1.0".into(),
                datasource: datasource::GIT_REFS,
                skip_reason: None,
                registry_urls: None,
                package_name: Some("https://github.com/some-url/some-package-version".into())
            }
        );
        assert_eq!(
            deps[8],
            FullPubDep {
                dep_name: "git_package_version_git_url".into(),
                dep_type: Some("dependencies".into()),
                current_value: "^1.1.0".into(),
                datasource: datasource::GIT_REFS,
                skip_reason: None,
                registry_urls: None,
                package_name: Some("https://github.com/some-url/some-package-version-url".into())
            }
        );
        assert_eq!(
            deps[9],
            FullPubDep {
                dep_name: "test".into(),
                dep_type: Some("dev_dependencies".into()),
                current_value: "^0.1.0".into(),
                datasource: datasource::DART,
                skip_reason: None,
                registry_urls: None,
                package_name: None
            }
        );
        assert_eq!(
            deps[10],
            FullPubDep {
                dep_name: "build".into(),
                dep_type: Some("dev_dependencies".into()),
                current_value: "0.0.1".into(),
                datasource: datasource::DART,
                skip_reason: None,
                registry_urls: None,
                package_name: None
            }
        );
        assert_eq!(
            deps[11],
            FullPubDep {
                dep_name: "path_dev_dep".into(),
                dep_type: Some("dev_dependencies".into()),
                current_value: "".into(),
                datasource: datasource::DART,
                skip_reason: Some("path-dependency"),
                registry_urls: None,
                package_name: None
            }
        );
        assert_eq!(
            deps[12],
            FullPubDep {
                dep_name: "dart".into(),
                dep_type: None,
                current_value: "^3.0.0".into(),
                datasource: datasource::DART_VERSION,
                skip_reason: None,
                registry_urls: None,
                package_name: None
            }
        );
        assert_eq!(
            deps[13],
            FullPubDep {
                dep_name: "flutter".into(),
                dep_type: None,
                current_value: "2.0.0".into(),
                datasource: datasource::FLUTTER_VERSION,
                skip_reason: None,
                registry_urls: None,
                package_name: None
            }
        );
        assert_eq!(deps.len(), 14);
    }

    #[test]
    fn pubspec_dep_type_as_renovate_str() {
        assert_eq!(PubspecDepType::Regular.as_renovate_str(), "dependencies");
        assert_eq!(PubspecDepType::Dev.as_renovate_str(), "dev_dependencies");
    }
}
