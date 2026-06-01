//! Go module artifact runner — lock file regeneration after go.mod edits.
//!
//! Ports `lib/modules/manager/gomod/artifacts.ts`.
//!
//! After `gomod_update_dependency` edits `go.mod`, this runner:
//! 1. Writes the updated `go.mod` to disk.
//! 2. Runs `go mod tidy` in the package directory.
//! 3. Reads the regenerated `go.mod` and `go.sum`.
//! 4. Returns any changed files.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use crate::artifacts::{
    ArtifactResult, ArtifactRunner, UpdateArtifact,
};
use crate::exec::raw::raw_exec;
use crate::exec::types::ExecOptions;

/// Artifact runner for Go modules.
#[derive(Debug, Clone)]
pub struct GomodArtifactRunner;

impl GomodArtifactRunner {
    /// Create a new gomod artifact runner.
    pub fn new() -> Self {
        Self
    }
}

impl Default for GomodArtifactRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl ArtifactRunner for GomodArtifactRunner {
    fn update_artifacts(
        &self,
        input: &UpdateArtifact,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Vec<ArtifactResult>>, crate::artifacts::ArtifactError>> + Send + '_>>
    {
        let lock_dir = input.config.lock_file_dir.clone();
        let package_file_name = input.package_file_name.clone();
        let new_package_file_content = input.new_package_file_content.clone();
        let config = input.config.clone();

        Box::pin(async move {
            let package_dir = lock_dir.join(&package_file_name);
            let package_dir = package_dir.parent().unwrap_or(&lock_dir);

            // Determine the lock file name (go.sum or foo.sum for foo.mod).
            let sum_file_name = if package_file_name.ends_with(".mod") {
                format!("{}sum", &package_file_name[..package_file_name.len() - 3])
            } else {
                "go.sum".to_owned()
            };
            let go_sum_path = package_dir.join(&sum_file_name);

            // If there is no go.sum, there is nothing to update.
            let existing_go_sum = match tokio::fs::read_to_string(&go_sum_path).await {
                Ok(content) => content,
                Err(_) => {
                    return Ok(None);
                }
            };

            // Write updated go.mod.
            let go_mod_path = package_dir.join("go.mod");
            if let Err(e) = tokio::fs::write(&go_mod_path, &new_package_file_content).await {
                return Err(crate::artifacts::ArtifactError {
                    lock_file: "go.mod".to_owned(),
                    stderr: format!("failed to write go.mod: {}", e),
                });
            }

            // Build env for go mod tidy.
            let mut env = std::env::vars().collect::<HashMap<String, String>>();
            for (k, v) in &config.env {
                env.insert(k.clone(), v.clone());
            }

            // Run `go mod tidy`.
            let opts = ExecOptions {
                cwd: Some(package_dir.to_string_lossy().to_string()),
                timeout: Some(300_000), // 5 minutes
                ..Default::default()
            };

            match raw_exec("go mod tidy", &opts, &env).await {
                Ok(_) => {}
                Err(e) => {
                    return Err(crate::artifacts::ArtifactError {
                        lock_file: sum_file_name.clone(),
                        stderr: format!("go mod tidy failed: {}", e.message),
                    });
                }
            }

            // Collect changed files.
            let mut results = Vec::new();

            // Check go.mod changes (go mod tidy may have reformatted it).
            let updated_go_mod = match tokio::fs::read_to_string(&go_mod_path).await {
                Ok(c) => c,
                Err(e) => {
                    return Err(crate::artifacts::ArtifactError {
                        lock_file: "go.mod".to_owned(),
                        stderr: format!("failed to read updated go.mod: {}", e),
                    });
                }
            };
            if updated_go_mod != new_package_file_content {
                results.push(ArtifactResult::file_change(
                    package_file_name.clone(),
                    updated_go_mod,
                ));
            }

            // Check go.sum changes.
            let updated_go_sum = match tokio::fs::read_to_string(&go_sum_path).await {
                Ok(c) => c,
                Err(e) => {
                    return Err(crate::artifacts::ArtifactError {
                        lock_file: sum_file_name.clone(),
                        stderr: format!("failed to read updated {}: {}", sum_file_name, e),
                    });
                }
            };
            if updated_go_sum != existing_go_sum {
                let rel = if package_file_name.contains('/') {
                    let dir = package_file_name
                        .rsplit_once('/')
                        .map(|(d, _)| d)
                        .unwrap_or("");
                    if dir.is_empty() {
                        sum_file_name.clone()
                    } else {
                        format!("{}/{}", dir, sum_file_name)
                    }
                } else {
                    sum_file_name.clone()
                };
                results.push(ArtifactResult::file_change(rel, updated_go_sum));
            }

            if results.is_empty() {
                Ok(None)
            } else {
                Ok(Some(results))
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::{ArtifactConfig, UpdateArtifact, UpdatedDep};
    use std::io::Write;
    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;

    fn make_input(dir: &tempfile::TempDir, go_mod: &str, go_sum: Option<&str>) -> UpdateArtifact {
        let lock_dir = dir.path().to_path_buf();
        if let Some(sum) = go_sum {
            std::fs::write(lock_dir.join("go.sum"), sum).unwrap();
        }
        UpdateArtifact {
            package_file_name: "go.mod".to_owned(),
            updated_deps: vec![UpdatedDep {
                dep_name: "github.com/foo/bar".to_owned(),
                package_name: None,
                current_value: Some("v1.0.0".to_owned()),
                new_value: Some("v1.1.0".to_owned()),
                locked_version: None,
                new_version: None,
                package_file: "go.mod".to_owned(),
                manager: "gomod".to_owned(),
                datasource: None,
            }],
            new_package_file_content: go_mod.to_owned(),
            config: ArtifactConfig {
                lock_file_dir: lock_dir,
                ..Default::default()
            },
        }
    }

    fn make_fake_go(dir: &tempfile::TempDir, script: &[u8]) -> std::collections::BTreeMap<String, String> {
        let fake_go = dir.path().join("go");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_go).unwrap();
            f.write_all(b"#!/bin/sh\n").unwrap();
            f.write_all(script).unwrap();
            let mut perms = std::fs::metadata(&fake_go).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_go, perms).unwrap();
        }
        #[cfg(windows)]
        {
            let mut f = std::fs::File::create(&fake_go).unwrap();
            f.write_all(b"@echo off\n").unwrap();
            f.write_all(script).unwrap();
        }

        let mut env: std::collections::BTreeMap<String, String> = std::env::vars().collect();
        let mut path = dir.path().to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);
        env
    }

    // Ported: "returns if no go.sum found" — gomod/artifacts.spec.ts line 94
    #[tokio::test]
    async fn returns_none_if_no_go_sum_found() {
        let dir = tempfile::tempdir().unwrap();
        let runner = GomodArtifactRunner::new();
        let input = make_input(&dir, "module example.com/test\n\ngo 1.22\n", None);
        let result = runner.update_artifacts(&input).await.unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null if unchanged" — gomod/artifacts.spec.ts line 107
    #[tokio::test]
    async fn returns_none_if_unchanged() {
        let dir = tempfile::tempdir().unwrap();
        let runner = GomodArtifactRunner::new();
        let mut input = make_input(&dir, "module example.com/test\n\ngo 1.22\n", Some("old sum\n"));
        // fake go that exits successfully but does nothing
        input.config.env = make_fake_go(&dir, b"exit 0\n");
        let result = runner.update_artifacts(&input).await.unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns updated go.sum" — gomod/artifacts.spec.ts line 145
    #[tokio::test]
    async fn returns_updated_go_sum() {
        let dir = tempfile::tempdir().unwrap();
        let runner = GomodArtifactRunner::new();
        let mut input = make_input(&dir, "module example.com/test\n\ngo 1.22\n", Some("old sum\n"));
        input.config.env = make_fake_go(&dir, b"if [ \"$1\" = \"mod\" ] && [ \"$2\" = \"tidy\" ]; then echo 'updated sum' > go.sum; fi\n");
        let result = runner.update_artifacts(&input).await.unwrap().unwrap();
        assert_eq!(result.len(), 1);
        let file_change = result[0].file.as_ref().unwrap();
        assert_eq!(file_change.path, "go.sum");
        assert_eq!(file_change.contents.as_deref(), Some("updated sum\n"));
    }

    // Rust-specific: verifies go.mod is returned when go mod tidy reformats it.
    #[tokio::test]
    async fn returns_updated_go_mod_when_reformatted() {
        let dir = tempfile::tempdir().unwrap();
        let runner = GomodArtifactRunner::new();
        let go_mod_input = "module example.com/test\n\ngo 1.22\n\nrequire github.com/foo/bar v1.1.0\n";
        let mut input = make_input(&dir, go_mod_input, Some("old sum\n"));
        // Write a reference go.mod with extra trailing newline.
        let ref_go_mod = dir.path().join("ref_go.mod");
        std::fs::write(&ref_go_mod, "module example.com/test\n\ngo 1.22\n\nrequire github.com/foo/bar v1.1.0\n\n").unwrap();
        // fake go that copies the reference go.mod and updates go.sum
        let script = format!(
            "if [ \"$1\" = \"mod\" ] && [ \"$2\" = \"tidy\" ]; then cp '{}' go.mod; echo 'new sum' > go.sum; fi\n",
            ref_go_mod.to_string_lossy()
        );
        input.config.env = make_fake_go(&dir, script.as_bytes());
        let result = runner.update_artifacts(&input).await.unwrap().unwrap();
        assert_eq!(result.len(), 2);
        let paths: Vec<_> = result.iter().map(|r| r.file.as_ref().unwrap().path.clone()).collect();
        assert!(paths.contains(&"go.mod".to_owned()));
        assert!(paths.contains(&"go.sum".to_owned()));
    }

    #[tokio::test]
    async fn gomod_artifact_runner_returns_error_when_go_missing() {
        let dir = tempfile::tempdir().unwrap();
        let input = make_input(&dir, "module example.com/test\n\ngo 1.22\n", Some("old sum\n"));
        let runner = GomodArtifactRunner::new();
        let result = runner.update_artifacts(&input).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.stderr.contains("go mod tidy failed"));
    }

    #[test]
    fn gomod_artifact_runner_new_and_default() {
        let r1 = GomodArtifactRunner::new();
        let r2 = GomodArtifactRunner;
        assert_eq!(format!("{:?}", r1), format!("{:?}", r2));
    }
}
