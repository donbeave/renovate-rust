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
