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

// ── Public API ─────────────────────────────────────────────────────────────

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

    #[test]
    fn empty_pubspec_returns_empty() {
        let content = "name: myapp\nversion: 1.0.0\n";
        assert!(extract(content).is_empty());
    }
}
