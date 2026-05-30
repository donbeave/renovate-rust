//! Differential parity harness — Rust side.
//!
//! Runs `renovate-rust` against fixture repositories and compares the
//! normalized JSON output against recorded expected output.
//!
//! Upstream Renovate cannot be run with the same `--platform=local`
//! invocation because its local platform does not accept a repositories
//! list. The harness therefore records the Rust output and guards
//! regressions; a live two-sided diff would require a matching platform
//! adapter or mock server setup.

use assert_cmd::Command;
use serde_json::Value;
use std::path::PathBuf;

fn renovate() -> Command {
    Command::cargo_bin("renovate").expect("binary 'renovate' built")
}

/// Normalizes volatile fields out of the JSON report so snapshots stay
/// stable across runs.
fn normalize_report(value: &mut Value) {
    if let Some(arr) = value.as_array_mut() {
        for repo in arr {
            if let Some(files) = repo.get_mut("files").and_then(|f| f.as_array_mut()) {
                for file in files {
                    if let Some(deps) = file.get_mut("deps").and_then(|d| d.as_array_mut()) {
                        for dep in deps {
                            // Remove release timestamps — they change when registries update.
                            if let Some(obj) = dep.as_object_mut() {
                                obj.remove("releaseTimestamp");
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Run `renovate-rust` against a fixture directory and return normalized JSON.
fn run_fixture(fixture_name: &str) -> Value {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture = manifest_dir
        .join("../../tests/parity/fixtures")
        .join(fixture_name);

    let output = renovate()
        .current_dir(&fixture)
        .env("LOG_LEVEL", "fatal")
        .arg("--platform=local")
        .arg("--dry-run=full")
        .arg("--output-format=json")
        .arg("local/test-repo")
        .output()
        .expect("renovate binary runs");

    assert!(
        output.status.success(),
        "renovate exited with non-zero status: stderr = {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut value: Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("stdout is not valid JSON: {e}\nstdout: {stdout}"));
    normalize_report(&mut value);
    value
}

#[test]
fn parity_npm_empty() {
    let actual = run_fixture("npm-empty");
    let expected = serde_json::json!([
        {
            "repoSlug": "local/test-repo",
            "stats": {
                "total": 0,
                "updateAvailable": 0,
                "upToDate": 0,
                "skipped": 0,
                "errors": 0
            },
            "files": [
                {
                    "path": "package.json",
                    "manager": "npm",
                    "stats": {
                        "total": 0,
                        "updateAvailable": 0,
                        "upToDate": 0,
                        "skipped": 0,
                        "errors": 0
                    },
                    "deps": []
                }
            ]
        }
    ]);
    assert_eq!(actual, expected, "normalized JSON output mismatch for npm-empty fixture");
}

#[test]
fn parity_npm_skipped() {
    let actual = run_fixture("npm-skipped");
    let expected = serde_json::json!([
        {
            "repoSlug": "local/test-repo",
            "stats": {
                "total": 2,
                "updateAvailable": 0,
                "upToDate": 0,
                "skipped": 2,
                "errors": 0
            },
            "files": [
                {
                    "path": "package.json",
                    "manager": "npm",
                    "stats": {
                        "total": 2,
                        "updateAvailable": 0,
                        "upToDate": 0,
                        "skipped": 2,
                        "errors": 0
                    },
                    "deps": [
                        {
                            "name": "local-pkg",
                            "status": "skipped",
                            "reason": "localpath"
                        },
                        {
                            "name": "url-pkg",
                            "status": "skipped",
                            "reason": "urlinstall"
                        }
                    ]
                }
            ]
        }
    ]);
    assert_eq!(actual, expected, "normalized JSON output mismatch for npm-skipped fixture");
}

#[test]
fn parity_cargo_workspace() {
    let actual = run_fixture("cargo-workspace");
    let expected = serde_json::json!([
        {
            "repoSlug": "local/test-repo",
            "stats": {
                "total": 1,
                "updateAvailable": 0,
                "upToDate": 0,
                "skipped": 1,
                "errors": 0
            },
            "files": [
                {
                    "path": "Cargo.toml",
                    "manager": "cargo",
                    "stats": {
                        "total": 1,
                        "updateAvailable": 0,
                        "upToDate": 0,
                        "skipped": 1,
                        "errors": 0
                    },
                    "deps": [
                        {
                            "name": "serde",
                            "status": "skipped",
                            "reason": "workspaceinherited"
                        }
                    ]
                }
            ]
        }
    ]);
    assert_eq!(actual, expected, "normalized JSON output mismatch for cargo-workspace fixture");
}

#[test]
fn parity_gomod_empty() {
    let actual = run_fixture("gomod-empty");
    let expected = serde_json::json!([
        {
            "repoSlug": "local/test-repo",
            "stats": {
                "total": 1,
                "updateAvailable": 0,
                "upToDate": 1,
                "skipped": 0,
                "errors": 0
            },
            "files": [
                {
                    "path": "go.mod",
                    "manager": "gomod",
                    "stats": {
                        "total": 1,
                        "updateAvailable": 0,
                        "upToDate": 1,
                        "skipped": 0,
                        "errors": 0
                    },
                    "deps": [
                        {
                            "name": "go",
                            "status": "upToDate",
                            "latest": null
                        }
                    ]
                }
            ]
        }
    ]);
    assert_eq!(actual, expected, "normalized JSON output mismatch for gomod-empty fixture");
}

#[test]
fn parity_maven_empty() {
    let actual = run_fixture("maven-empty");
    let expected = serde_json::json!([
        {
            "repoSlug": "local/test-repo",
            "stats": {
                "total": 0,
                "updateAvailable": 0,
                "upToDate": 0,
                "skipped": 0,
                "errors": 0
            },
            "files": [
                {
                    "path": "pom.xml",
                    "manager": "maven",
                    "stats": {
                        "total": 0,
                        "updateAvailable": 0,
                        "upToDate": 0,
                        "skipped": 0,
                        "errors": 0
                    },
                    "deps": []
                }
            ]
        }
    ]);
    assert_eq!(actual, expected, "normalized JSON output mismatch for maven-empty fixture");
}

#[test]
fn parity_gomod_replace() {
    let actual = run_fixture("gomod-replace");
    let expected = serde_json::json!([
        {
            "repoSlug": "local/test-repo",
            "stats": {
                "total": 2,
                "updateAvailable": 0,
                "upToDate": 1,
                "skipped": 1,
                "errors": 0
            },
            "files": [
                {
                    "path": "go.mod",
                    "manager": "gomod",
                    "stats": {
                        "total": 2,
                        "updateAvailable": 0,
                        "upToDate": 1,
                        "skipped": 1,
                        "errors": 0
                    },
                    "deps": [
                        {
                            "name": "github.com/pkg/errors",
                            "status": "skipped",
                            "reason": "localreplace"
                        },
                        {
                            "name": "go",
                            "status": "upToDate",
                            "latest": null
                        }
                    ]
                }
            ]
        }
    ]);
    assert_eq!(actual, expected, "normalized JSON output mismatch for gomod-replace fixture");
}

#[test]
fn parity_dockerfile_scratch() {
    let actual = run_fixture("dockerfile-scratch");
    let expected = serde_json::json!([
        {
            "repoSlug": "local/test-repo",
            "stats": {
                "total": 1,
                "updateAvailable": 0,
                "upToDate": 0,
                "skipped": 1,
                "errors": 0
            },
            "files": [
                {
                    "path": "Dockerfile",
                    "manager": "dockerfile",
                    "stats": {
                        "total": 1,
                        "updateAvailable": 0,
                        "upToDate": 0,
                        "skipped": 1,
                        "errors": 0
                    },
                    "deps": [
                        {
                            "name": "scratch",
                            "status": "skipped",
                            "reason": "scratch"
                        }
                    ]
                }
            ]
        }
    ]);
    assert_eq!(actual, expected, "normalized JSON output mismatch for dockerfile-scratch fixture");
}

#[test]
fn parity_github_actions_skipped() {
    let actual = run_fixture("github-actions-skipped");
    let expected = serde_json::json!([
        {
            "repoSlug": "local/test-repo",
            "stats": {
                "total": 3,
                "updateAvailable": 0,
                "upToDate": 0,
                "skipped": 3,
                "errors": 0
            },
            "files": [
                {
                    "path": ".github/workflows/ci.yml",
                    "manager": "github-actions",
                    "stats": {
                        "total": 3,
                        "updateAvailable": 0,
                        "upToDate": 0,
                        "skipped": 3,
                        "errors": 0
                    },
                    "deps": [
                        {
                            "name": "actions/checkout",
                            "status": "skipped",
                            "reason": "shapin"
                        },
                        {
                            "name": "./.github/actions/local",
                            "status": "skipped",
                            "reason": "localaction"
                        },
                        {
                            "name": "docker://node:18",
                            "status": "skipped",
                            "reason": "dockerref"
                        }
                    ]
                }
            ]
        }
    ]);
    assert_eq!(actual, expected, "normalized JSON output mismatch for github-actions-skipped fixture");
}

#[test]
fn parity_docker_compose_scratch() {
    let actual = run_fixture("docker-compose-scratch");
    let expected = serde_json::json!([
        {
            "repoSlug": "local/test-repo",
            "stats": {
                "total": 1,
                "updateAvailable": 0,
                "upToDate": 0,
                "skipped": 1,
                "errors": 0
            },
            "files": [
                {
                    "path": "docker-compose.yml",
                    "manager": "docker-compose",
                    "stats": {
                        "total": 1,
                        "updateAvailable": 0,
                        "upToDate": 0,
                        "skipped": 1,
                        "errors": 0
                    },
                    "deps": [
                        {
                            "name": "scratch",
                            "status": "skipped",
                            "reason": "dockerfile(scratch)"
                        }
                    ]
                }
            ]
        }
    ]);
    assert_eq!(actual, expected, "normalized JSON output mismatch for docker-compose-scratch fixture");
}

#[test]
fn parity_composer_skipped() {
    let actual = run_fixture("composer-skipped");
    let expected = serde_json::json!([
        {
            "repoSlug": "local/test-repo",
            "stats": {
                "total": 3,
                "updateAvailable": 0,
                "upToDate": 0,
                "skipped": 3,
                "errors": 0
            },
            "files": [
                {
                    "path": "composer.json",
                    "manager": "composer",
                    "stats": {
                        "total": 3,
                        "updateAvailable": 0,
                        "upToDate": 0,
                        "skipped": 3,
                        "errors": 0
                    },
                    "deps": [
                        {
                            "name": "ext-json",
                            "status": "skipped",
                            "reason": "platformpackage"
                        },
                        {
                            "name": "php",
                            "status": "skipped",
                            "reason": "platformpackage"
                        },
                        {
                            "name": "phpunit/phpunit",
                            "status": "skipped",
                            "reason": "devbranch"
                        }
                    ]
                }
            ]
        }
    ]);
    assert_eq!(actual, expected, "normalized JSON output mismatch for composer-skipped fixture");
}

#[test]
fn parity_terraform_local() {
    let actual = run_fixture("terraform-local");
    let expected = serde_json::json!([
        {
            "repoSlug": "local/test-repo",
            "stats": {
                "total": 1,
                "updateAvailable": 0,
                "upToDate": 0,
                "skipped": 1,
                "errors": 0
            },
            "files": [
                {
                    "path": "main.tf",
                    "manager": "terraform",
                    "stats": {
                        "total": 1,
                        "updateAvailable": 0,
                        "upToDate": 0,
                        "skipped": 1,
                        "errors": 0
                    },
                    "deps": [
                        {
                            "name": "./modules/local",
                            "status": "skipped",
                            "reason": "externalsource"
                        }
                    ]
                }
            ]
        }
    ]);
    assert_eq!(actual, expected, "normalized JSON output mismatch for terraform-local fixture");
}

#[test]
fn parity_pre_commit_skipped() {
    let actual = run_fixture("pre-commit-skipped");
    let expected = serde_json::json!([
        {
            "repoSlug": "local/test-repo",
            "stats": {
                "total": 2,
                "updateAvailable": 0,
                "upToDate": 0,
                "skipped": 2,
                "errors": 0
            },
            "files": [
                {
                    "path": ".pre-commit-config.yaml",
                    "manager": "pre-commit",
                    "stats": {
                        "total": 2,
                        "updateAvailable": 0,
                        "upToDate": 0,
                        "skipped": 2,
                        "errors": 0
                    },
                    "deps": [
                        {
                            "name": "local",
                            "status": "skipped",
                            "reason": "localhook"
                        },
                        {
                            "name": "meta",
                            "status": "skipped",
                            "reason": "metahook"
                        }
                    ]
                }
            ]
        }
    ]);
    assert_eq!(actual, expected, "normalized JSON output mismatch for pre-commit-skipped fixture");
}

#[test]
fn parity_asdf_skipped() {
    let actual = run_fixture("asdf-skipped");
    let expected = serde_json::json!([
        {
            "repoSlug": "local/test-repo",
            "stats": {
                "total": 1,
                "updateAvailable": 0,
                "upToDate": 0,
                "skipped": 1,
                "errors": 0
            },
            "files": [
                {
                    "path": ".tool-versions",
                    "manager": "asdf",
                    "stats": {
                        "total": 1,
                        "updateAvailable": 0,
                        "upToDate": 0,
                        "skipped": 1,
                        "errors": 0
                    },
                    "deps": [
                        {
                            "name": "unknowntool",
                            "status": "skipped",
                            "reason": "unsupportedtool"
                        }
                    ]
                }
            ]
        }
    ]);
    assert_eq!(actual, expected, "normalized JSON output mismatch for asdf-skipped fixture");
}

#[test]
fn parity_pip_skipped() {
    let actual = run_fixture("pip-skipped");
    let expected = serde_json::json!([
        {
            "repoSlug": "local/test-repo",
            "stats": {
                "total": 3,
                "updateAvailable": 0,
                "upToDate": 0,
                "skipped": 3,
                "errors": 0
            },
            "files": [
                {
                    "path": "requirements.txt",
                    "manager": "pip_requirements",
                    "stats": {
                        "total": 3,
                        "updateAvailable": 0,
                        "upToDate": 0,
                        "skipped": 3,
                        "errors": 0
                    },
                    "deps": [
                        {
                            "name": "repo",
                            "status": "skipped",
                            "reason": "gitsource"
                        },
                        {
                            "name": "unknown",
                            "status": "skipped",
                            "reason": "urlinstall"
                        },
                        {
                            "name": "other.txt",
                            "status": "skipped",
                            "reason": "subrequirement"
                        }
                    ]
                }
            ]
        }
    ]);
    assert_eq!(actual, expected, "normalized JSON output mismatch for pip-skipped fixture");
}

#[test]
fn parity_circleci_scratch() {
    let actual = run_fixture("circleci-scratch");
    let expected = serde_json::json!([
        {
            "repoSlug": "local/test-repo",
            "stats": {
                "total": 1,
                "updateAvailable": 0,
                "upToDate": 0,
                "skipped": 1,
                "errors": 0
            },
            "files": [
                {
                    "path": ".circleci/config.yml",
                    "manager": "circleci",
                    "stats": {
                        "total": 1,
                        "updateAvailable": 0,
                        "upToDate": 0,
                        "skipped": 1,
                        "errors": 0
                    },
                    "deps": [
                        {
                            "name": "scratch",
                            "status": "skipped",
                            "reason": "scratch"
                        }
                    ]
                }
            ]
        }
    ]);
    assert_eq!(actual, expected, "normalized JSON output mismatch for circleci-scratch fixture");
}

#[test]
fn parity_droneci_scratch() {
    let actual = run_fixture("droneci-scratch");
    let expected = serde_json::json!([
        {
            "repoSlug": "local/test-repo",
            "stats": {
                "total": 1,
                "updateAvailable": 0,
                "upToDate": 0,
                "skipped": 1,
                "errors": 0
            },
            "files": [
                {
                    "path": ".drone.yml",
                    "manager": "droneci",
                    "stats": {
                        "total": 1,
                        "updateAvailable": 0,
                        "upToDate": 0,
                        "skipped": 1,
                        "errors": 0
                    },
                    "deps": [
                        {
                            "name": "scratch",
                            "status": "skipped",
                            "reason": "scratch"
                        }
                    ]
                }
            ]
        }
    ]);
    assert_eq!(actual, expected, "normalized JSON output mismatch for droneci-scratch fixture");
}

#[test]
fn parity_gitlabci_scratch() {
    let actual = run_fixture("gitlabci-scratch");
    let expected = serde_json::json!([
        {
            "repoSlug": "local/test-repo",
            "stats": {
                "total": 1,
                "updateAvailable": 0,
                "upToDate": 0,
                "skipped": 1,
                "errors": 0
            },
            "files": [
                {
                    "path": ".gitlab-ci.yml",
                    "manager": "gitlabci",
                    "stats": {
                        "total": 1,
                        "updateAvailable": 0,
                        "upToDate": 0,
                        "skipped": 1,
                        "errors": 0
                    },
                    "deps": [
                        {
                            "name": "scratch",
                            "status": "skipped",
                            "reason": "scratch"
                        }
                    ]
                }
            ]
        }
    ]);
    assert_eq!(actual, expected, "normalized JSON output mismatch for gitlabci-scratch fixture");
}

#[test]
fn parity_woodpecker_scratch() {
    let actual = run_fixture("woodpecker-scratch");
    let expected = serde_json::json!([
        {
            "repoSlug": "local/test-repo",
            "stats": {
                "total": 1,
                "updateAvailable": 0,
                "upToDate": 0,
                "skipped": 1,
                "errors": 0
            },
            "files": [
                {
                    "path": ".woodpecker.yml",
                    "manager": "woodpecker",
                    "stats": {
                        "total": 1,
                        "updateAvailable": 0,
                        "upToDate": 0,
                        "skipped": 1,
                        "errors": 0
                    },
                    "deps": [
                        {
                            "name": "scratch",
                            "status": "skipped",
                            "reason": "scratch"
                        }
                    ]
                }
            ]
        }
    ]);
    assert_eq!(actual, expected, "normalized JSON output mismatch for woodpecker-scratch fixture");
}
