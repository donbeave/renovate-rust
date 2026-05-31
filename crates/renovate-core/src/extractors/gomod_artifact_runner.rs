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
        let _updated_deps = input.updated_deps.clone();
        let config = input.config.clone();

        Box::pin(async move {
            let package_dir = lock_dir.join(&package_file_name);
            let package_dir = package_dir.parent().unwrap_or(&lock_dir);

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
                        lock_file: "go.sum".to_owned(),
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
                let rel = package_file_name.clone();
                results.push(ArtifactResult::file_change(rel, updated_go_mod));
            }

            // Check go.sum changes.
            let go_sum_path = package_dir.join("go.sum");
            if let Ok(go_sum_content) = tokio::fs::read_to_string(&go_sum_path).await {
                let rel = if package_file_name.contains('/') {
                    let dir = package_file_name.rsplit_once('/').map(|(d, _)| d).unwrap_or("");
                    if dir.is_empty() {
                        "go.sum".to_owned()
                    } else {
                        format!("{}/go.sum", dir)
                    }
                } else {
                    "go.sum".to_owned()
                };
                results.push(ArtifactResult::file_change(rel, go_sum_content));
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
    use std::path::PathBuf;

    #[tokio::test]
    async fn gomod_artifact_runner_returns_error_when_go_missing() {
        let runner = GomodArtifactRunner::new();
        let dir = tempfile::tempdir().unwrap();
        let input = UpdateArtifact {
            package_file_name: "go.mod".to_owned(),
            updated_deps: vec![UpdatedDep {
                dep_name: "github.com/foo/bar".to_owned(),
                current_value: Some("v1.0.0".to_owned()),
                new_value: Some("v1.1.0".to_owned()),
                package_file: "go.mod".to_owned(),
                manager: "gomod".to_owned(),
                datasource: None,
            }],
            new_package_file_content: r#"module example.com/test

go 1.22

require github.com/foo/bar v1.1.0
"#
            .to_owned(),
            config: ArtifactConfig {
                lock_file_dir: dir.path().to_path_buf(),
                ..Default::default()
            },
        };

        let result = runner.update_artifacts(&input).await;
        // go is not available in the test environment, so we expect an error.
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.stderr.contains("go mod tidy failed"));
    }

    #[test]
    fn gomod_artifact_runner_new_and_default() {
        let r1 = GomodArtifactRunner::new();
        let r2 = GomodArtifactRunner::default();
        assert_eq!(format!("{:?}", r1), format!("{:?}", r2));
    }
}
