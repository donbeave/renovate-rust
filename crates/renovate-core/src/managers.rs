//! Package manager detection.
//!
//! Each supported package manager declares a set of file patterns. Given the
//! full file list from a repository, the detection step matches those patterns
//! and returns which managers apply — and which specific files they should
//! process.
//!
//! Renovate reference: `lib/modules/manager/*/index.ts` `defaultConfig.managerFilePatterns`.
//!
//! ## Pattern format
//!
//! Renovate's patterns are JavaScript regex strings (e.g. `"/(^|/)Cargo\\.toml$/"`).
//! This module stores the inner regex (without surrounding `/`) and compiles
//! them with the `regex` crate, which is RE2-compatible.

use std::sync::LazyLock;

use regex::Regex;

/// A detected package manager with the list of matching files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectedManager {
    /// Manager identifier matching Renovate's manager names.
    pub name: &'static str,
    /// Files that matched one of the manager's patterns.
    pub matched_files: Vec<String>,
}

/// A single manager's detection record: (name, regex patterns).
///
/// Patterns are the inner regex strings from Renovate's `managerFilePatterns`.
struct ManagerDef {
    name: &'static str,
    patterns: &'static [&'static str],
}

/// Pre-compiled manager patterns.  Compiled once at first use via
/// `LazyLock` — avoids re-compilation on every `detect()` call.
static COMPILED: LazyLock<Vec<(&'static str, Vec<Regex>)>> = LazyLock::new(|| {
    MANAGER_DEFS
        .iter()
        .filter_map(|def| {
            let compiled: Vec<Regex> = def
                .patterns
                .iter()
                .filter_map(|pat| {
                    Regex::new(pat)
                        .map_err(|e| {
                            // Programmer error: a pattern in the static table
                            // is invalid.  Log and skip the manager.
                            tracing::error!(
                                manager = def.name,
                                pattern = pat,
                                %e,
                                "invalid manager pattern (bug in pattern definition)"
                            );
                        })
                        .ok()
                })
                .collect();
            if compiled.len() == def.patterns.len() {
                Some((def.name, compiled))
            } else {
                None
            }
        })
        .collect()
});

/// The initial set of supported manager definitions, ported from upstream
/// `managerFilePatterns` entries. Coverage grows with each parity slice.
///
/// Sources (all from `lib/modules/manager/*/index.ts`):
/// - cargo:          `/(^|/)Cargo\\.toml$/`
/// - npm:            `/(^|/)package\\.json$/`, `/(^|/)pnpm-workspace\\.yaml$/`, `/(^|/)\\.yarnrc\\.yml$/`
/// - pip_requirements: `/(^|/)[\\w-]*requirements([-._]\\w+)?\\.(txt|pip)$/`
/// - pep621:         `/(^|/)pyproject\\.toml$/`
/// - maven:          `/(^|/|\\.)pom\\.xml$/`, `/^(((\\.mvn)|(\\.m2))/)?settings\\.xml$/`
/// - github-actions: `/(^|/)(workflow-templates|\\.(?:github|gitea|forgejo)/(?:workflows|actions))/.+\\.ya?ml$/`, `/(^|/)action\\.ya?ml$/`
/// - dockerfile:     `/(^|/)(Dockerfile|Containerfile)(\\.[^/]*)?$/`
/// - docker-compose: `/(^|/)(?:docker-)?compose\\.ya?ml$/`
const MANAGER_DEFS: &[ManagerDef] = &[
    ManagerDef {
        name: "bundler",
        patterns: &[r"(^|/)Gemfile$"],
    },
    ManagerDef {
        name: "cocoapods",
        patterns: &[r"(^|/)Podfile$"],
    },
    ManagerDef {
        name: "mix",
        patterns: &[r"(^|/)mix\.exs$"],
    },
    ManagerDef {
        name: "swift",
        patterns: &[r"(^|/)Package\.swift$"],
    },
    ManagerDef {
        name: "gradle",
        patterns: &[
            r"\.gradle(\.kts)?$",
            r"(^|/)gradle\.properties$",
            r"\.versions\.toml$",
        ],
    },
    ManagerDef {
        name: "helmv3",
        patterns: &[r"(^|/)Chart\.ya?ml$", r"(^|/)requirements\.ya?ml$"],
    },
    ManagerDef {
        name: "terraform",
        patterns: &[r"\.tf$", r"\.tofu$"],
    },
    ManagerDef {
        name: "composer",
        patterns: &[r"(^|/)([\w-]*)composer\.json$"],
    },
    ManagerDef {
        name: "pub",
        patterns: &[r"(^|/)pubspec\.ya?ml$"],
    },
    ManagerDef {
        name: "nuget",
        patterns: &[r"\.(cs|fs|vb)proj$", r"\.(props|targets)$"],
    },
    ManagerDef {
        name: "cargo",
        patterns: &[r"(^|/)Cargo\.toml$"],
    },
    ManagerDef {
        name: "npm",
        patterns: &[
            r"(^|/)package\.json$",
            r"(^|/)pnpm-workspace\.yaml$",
            r"(^|/)\.yarnrc\.yml$",
        ],
    },
    ManagerDef {
        name: "pip_requirements",
        patterns: &[r"(^|/)[\w-]*requirements([-._]\w+)?\.(txt|pip)$"],
    },
    ManagerDef {
        name: "setup-cfg",
        patterns: &[r"(^|/)setup\.cfg$"],
    },
    ManagerDef {
        name: "pre-commit",
        patterns: &[r"(^|/)\.pre-commit-config\.ya?ml$"],
    },
    ManagerDef {
        name: "asdf",
        patterns: &[r"(^|/)\.tool-versions$"],
    },
    ManagerDef {
        name: "pep621",
        patterns: &[r"(^|/)pyproject\.toml$"],
    },
    ManagerDef {
        name: "poetry",
        patterns: &[r"(^|/)pyproject\.toml$"],
    },
    ManagerDef {
        name: "gomod",
        patterns: &[r"(^|/)go\.mod$"],
    },
    ManagerDef {
        name: "maven",
        patterns: &[r"(^|/|\.)(pom\.xml)$", r"^((\.mvn|\.m2)/)?settings\.xml$"],
    },
    ManagerDef {
        name: "github-actions",
        patterns: &[
            r"(^|/)(workflow-templates|\.(?:github|gitea|forgejo)/(?:workflows|actions))/.+\.ya?ml$",
            r"(^|/)action\.ya?ml$",
        ],
    },
    ManagerDef {
        name: "dockerfile",
        patterns: &[r"(^|/)(Dockerfile|Containerfile)(\.[^/]*)?$"],
    },
    ManagerDef {
        name: "docker-compose",
        patterns: &[r"(^|/)(?:docker-)?compose\.ya?ml$"],
    },
];

/// Detect which package managers are present in the repository.
///
/// Uses pre-compiled regex patterns (compiled once via [`COMPILED`]).
/// Managers with at least one matching file are included in the result.
pub fn detect(files: &[String]) -> Vec<DetectedManager> {
    let mut results = Vec::new();

    for (name, patterns) in COMPILED.iter() {
        let matched: Vec<String> = files
            .iter()
            .filter(|f| patterns.iter().any(|re| re.is_match(f)))
            .cloned()
            .collect();

        if !matched.is_empty() {
            results.push(DetectedManager {
                name,
                matched_files: matched,
            });
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    fn files(paths: &[&str]) -> Vec<String> {
        paths.iter().map(|s| (*s).to_owned()).collect()
    }

    #[test]
    fn detects_cargo() {
        let f = files(&["Cargo.toml", "src/main.rs", "crates/foo/Cargo.toml"]);
        let result = detect(&f);
        let cargo = result.iter().find(|m| m.name == "cargo").unwrap();
        assert_eq!(
            cargo.matched_files,
            vec!["Cargo.toml", "crates/foo/Cargo.toml"]
        );
    }

    #[test]
    fn detects_npm_package_json() {
        let f = files(&["package.json", "frontend/package.json", "README.md"]);
        let result = detect(&f);
        let npm = result.iter().find(|m| m.name == "npm").unwrap();
        assert!(npm.matched_files.contains(&"package.json".to_owned()));
        assert!(
            npm.matched_files
                .contains(&"frontend/package.json".to_owned())
        );
    }

    #[test]
    fn detects_pip_requirements() {
        let f = files(&["requirements.txt", "requirements-dev.txt", "src/setup.py"]);
        let result = detect(&f);
        let pip = result
            .iter()
            .find(|m| m.name == "pip_requirements")
            .unwrap();
        assert!(pip.matched_files.contains(&"requirements.txt".to_owned()));
        assert!(
            pip.matched_files
                .contains(&"requirements-dev.txt".to_owned())
        );
        // setup.py should NOT match
        assert!(!pip.matched_files.contains(&"src/setup.py".to_owned()));
    }

    #[test]
    fn detects_github_actions_workflow() {
        let f = files(&[
            ".github/workflows/ci.yml",
            ".github/workflows/deploy.yaml",
            "README.md",
        ]);
        let result = detect(&f);
        let ga = result.iter().find(|m| m.name == "github-actions").unwrap();
        assert_eq!(ga.matched_files.len(), 2);
    }

    #[test]
    fn detects_dockerfile() {
        let f = files(&["Dockerfile", "docker/Dockerfile.prod", "src/main.rs"]);
        let result = detect(&f);
        let df = result.iter().find(|m| m.name == "dockerfile").unwrap();
        assert!(df.matched_files.contains(&"Dockerfile".to_owned()));
    }

    #[test]
    fn detects_docker_compose() {
        let f = files(&["docker-compose.yml", "compose.yaml"]);
        let result = detect(&f);
        let dc = result.iter().find(|m| m.name == "docker-compose").unwrap();
        assert_eq!(dc.matched_files.len(), 2);
    }

    #[test]
    fn detects_maven_pom() {
        let f = files(&["pom.xml", "module/pom.xml", "parent.pom.xml"]);
        let result = detect(&f);
        let maven = result.iter().find(|m| m.name == "maven").unwrap();
        assert!(maven.matched_files.contains(&"pom.xml".to_owned()));
        assert!(maven.matched_files.contains(&"module/pom.xml".to_owned()));
        assert!(maven.matched_files.contains(&"parent.pom.xml".to_owned()));
    }

    #[test]
    fn empty_file_list_returns_no_managers() {
        assert!(detect(&[]).is_empty());
    }

    #[test]
    fn unrelated_files_return_no_managers() {
        let f = files(&["README.md", "LICENSE", "src/lib.rs"]);
        // .rs files don't match any manager pattern
        let result = detect(&f);
        assert!(!result.iter().any(|m| m.name == "cargo"));
    }

    #[test]
    fn detects_multiple_managers_in_same_repo() {
        let f = files(&["Cargo.toml", "package.json", ".github/workflows/ci.yml"]);
        let result = detect(&f);
        assert!(result.iter().any(|m| m.name == "cargo"));
        assert!(result.iter().any(|m| m.name == "npm"));
        assert!(result.iter().any(|m| m.name == "github-actions"));
    }
}
