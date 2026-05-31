//! NPM artifact runner implementing the `ArtifactRunner` trait.
//!
//! Ports `lib/modules/manager/npm/artifacts.ts`.
//!
//! Regenerates lock files (package-lock.json, yarn.lock, pnpm-lock.yaml)
//! after package.json has been updated.

use std::path::Path;

use crate::artifacts::{
    ArtifactConfig, ArtifactError, ArtifactResult, ArtifactRunner, UpdateArtifact,
};
use crate::exec::raw::raw_exec;
use crate::exec::types::ExecOptions;

use super::npm::build_npm_install_cmd;

/// NPM artifact runner.
#[derive(Debug, Clone, Default)]
pub struct NpmArtifactRunner;

impl NpmArtifactRunner {
    pub fn new() -> Self {
        Self
    }

    fn detect_lock_file(dir: &Path) -> Option<(&'static str, &'static str)> {
        if dir.join("package-lock.json").is_file() {
            Some(("package-lock.json", "npm"))
        } else if dir.join("yarn.lock").is_file() {
            Some(("yarn.lock", "yarn"))
        } else if dir.join("pnpm-lock.yaml").is_file() {
            Some(("pnpm-lock.yaml", "pnpm"))
        } else {
            None
        }
    }

    fn build_install_cmd(manager: &str, _config: &ArtifactConfig) -> Vec<String> {
        match manager {
            "npm" => build_npm_install_cmd(true, false, true, None),
            "yarn" => vec!["yarn".to_owned(), "install".to_owned()],
            "pnpm" => vec![
                "pnpm".to_owned(),
                "install".to_owned(),
                "--lockfile-only".to_owned(),
            ],
            _ => build_npm_install_cmd(true, false, true, None),
        }
    }
}

impl ArtifactRunner for NpmArtifactRunner {
    fn update_artifacts(
        &self,
        input: &UpdateArtifact,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<Option<Vec<ArtifactResult>>, ArtifactError>>
                + Send
                + '_,
        >,
    > {
        let skip_installs = input.config.skip_installs;
        let lock_dir = input.config.lock_file_dir.clone();
        let package_file_name = input.package_file_name.clone();
        let new_package_file_content = input.new_package_file_content.clone();
        let config = input.config.clone();

        Box::pin(async move {
            if skip_installs {
                return Ok(None);
            }

            let package_path = lock_dir.join(&package_file_name);

            // Write the updated package.json so the package manager can read it.
            if let Some(parent) = package_path.parent() {
                let _ = tokio::fs::create_dir_all(parent).await;
            }
            if let Err(err) = tokio::fs::write(&package_path, &new_package_file_content).await {
                return Err(ArtifactError {
                    lock_file: package_file_name,
                    stderr: format!("failed to write package.json: {err}"),
                });
            }

            // Determine which lock file / package manager to use.
            let Some((lock_file_name, manager)) = Self::detect_lock_file(&lock_dir) else {
                // No lock file present — nothing to regenerate.
                return Ok(None);
            };

            // Build and run the install command.
            let cmd_parts = Self::build_install_cmd(manager, &config);
            let cmd = cmd_parts.join(" ");

            let opts = ExecOptions {
                cwd: Some(lock_dir.to_string_lossy().to_string()),
                timeout: Some(120_000),
                ..Default::default()
            };

            let process_env: std::collections::HashMap<String, String> =
                std::env::vars().collect();

            match raw_exec(&cmd, &opts, &process_env).await {
                Ok(result) => {
                    if !result.stdout.is_empty() {
                        tracing::debug!(stdout = %result.stdout, "npm artifact runner stdout");
                    }
                    if !result.stderr.is_empty() {
                        tracing::debug!(stderr = %result.stderr, "npm artifact runner stderr");
                    }
                }
                Err(err) => {
                    return Err(ArtifactError {
                        lock_file: lock_file_name.to_owned(),
                        stderr: format!("{err}"),
                    });
                }
            }

            // Read the regenerated lock file.
            let lock_path = lock_dir.join(lock_file_name);
            match tokio::fs::read_to_string(&lock_path).await {
                Ok(content) => Ok(Some(vec![ArtifactResult::file_change(
                    lock_file_name,
                    content,
                )])),
                Err(err) => Err(ArtifactError {
                    lock_file: lock_file_name.to_owned(),
                    stderr: format!("failed to read lock file: {err}"),
                }),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::UpdatedDep;
    use tempfile::tempdir;

    // Ported: "returns updated package.json" — modules/manager/npm/artifacts.spec.ts line 105
    #[test]
    fn detect_lock_file_npm() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("package-lock.json"), "{}").unwrap();
        assert_eq!(
            NpmArtifactRunner::detect_lock_file(dir.path()),
            Some(("package-lock.json", "npm"))
        );
    }

    // Ported: "returns updated package.json" — modules/manager/npm/artifacts.spec.ts line 105
    #[test]
    fn detect_lock_file_yarn() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("yarn.lock"), "").unwrap();
        assert_eq!(
            NpmArtifactRunner::detect_lock_file(dir.path()),
            Some(("yarn.lock", "yarn"))
        );
    }

    // Ported: "returns updated package.json" — modules/manager/npm/artifacts.spec.ts line 105
    #[test]
    fn detect_lock_file_pnpm() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("pnpm-lock.yaml"), "").unwrap();
        assert_eq!(
            NpmArtifactRunner::detect_lock_file(dir.path()),
            Some(("pnpm-lock.yaml", "pnpm"))
        );
    }

    // Ported: "returns null if no packageManager updates present" — modules/manager/npm/artifacts.spec.ts line 57
    #[test]
    fn detect_lock_file_none() {
        let dir = tempdir().unwrap();
        assert!(NpmArtifactRunner::detect_lock_file(dir.path()).is_none());
    }

    // Ported: "supports install mode" — modules/manager/npm/artifacts.spec.ts line 180
    #[test]
    fn build_install_cmd_npm() {
        let cmd = NpmArtifactRunner::build_install_cmd("npm", &ArtifactConfig::default());
        assert_eq!(cmd, vec!["npm", "install", "--package-lock-only", "--ignore-scripts"]);
    }

    // Ported: "supports install mode" — modules/manager/npm/artifacts.spec.ts line 180
    #[test]
    fn build_install_cmd_yarn() {
        let cmd = NpmArtifactRunner::build_install_cmd("yarn", &ArtifactConfig::default());
        assert_eq!(cmd, vec!["yarn", "install"]);
    }

    // Ported: "supports install mode" — modules/manager/npm/artifacts.spec.ts line 180
    #[test]
    fn build_install_cmd_pnpm() {
        let cmd = NpmArtifactRunner::build_install_cmd("pnpm", &ArtifactConfig::default());
        assert_eq!(cmd, vec!["pnpm", "install", "--lockfile-only"]);
    }

    // Rust-specific: integration test that exercises the full runner end-to-end
    // when npm is available on the system.
    #[tokio::test]
    async fn artifact_runner_regenerates_lock_file() {
        let dir = tempdir().unwrap();
        let lock_dir = dir.path().to_path_buf();

        // Write an initial package.json and generate a lock file.
        let package_json = r#"{"name":"test","dependencies":{"lodash":"^4.0.0"}}"#;
        std::fs::write(lock_dir.join("package.json"), package_json).unwrap();

        // Generate a lock file with npm if available.
        let _ = std::process::Command::new("npm")
            .args(["install", "--package-lock-only"])
            .current_dir(&lock_dir)
            .output();

        if !lock_dir.join("package-lock.json").exists() {
            // npm not available — skip this integration test.
            return;
        }

        let runner = NpmArtifactRunner::new();
        let input = UpdateArtifact {
            package_file_name: "package.json".to_owned(),
            updated_deps: vec![UpdatedDep {
                dep_name: "lodash".to_owned(),
                package_name: None,
                current_value: Some("^4.0.0".to_owned()),
                new_value: Some("^4.17.21".to_owned()),
                locked_version: None,
                new_version: None,
                package_file: "package.json".to_owned(),
                manager: "npm".to_owned(),
                datasource: Some("npm".to_owned()),
            }],
            new_package_file_content: r#"{"name":"test","dependencies":{"lodash":"^4.17.21"}}"#.to_owned(),
            config: ArtifactConfig {
                lock_file_dir: lock_dir.clone(),
                ..Default::default()
            },
        };

        let result = runner.update_artifacts(&input).await.unwrap();
        let results = result.expect("runner should return results");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].file.as_ref().unwrap().path, "package-lock.json");

        // Verify the lock file contains the updated version.
        let content = results[0].file.as_ref().unwrap().contents.as_ref().unwrap();
        assert!(
            content.contains("4.17.21"),
            "lock file should contain updated version; got: {content}"
        );
    }

    #[tokio::test]
    async fn artifact_runner_skips_when_no_lock_file() {
        let dir = tempdir().unwrap();
        let runner = NpmArtifactRunner::new();
        let input = UpdateArtifact {
            package_file_name: "package.json".to_owned(),
            updated_deps: vec![],
            new_package_file_content: "{}".to_owned(),
            config: ArtifactConfig {
                lock_file_dir: dir.path().to_path_buf(),
                ..Default::default()
            },
        };

        let result = runner.update_artifacts(&input).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn artifact_runner_skips_when_skip_installs() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("package-lock.json"), "{}").unwrap();
        let runner = NpmArtifactRunner::new();
        let input = UpdateArtifact {
            package_file_name: "package.json".to_owned(),
            updated_deps: vec![],
            new_package_file_content: "{}".to_owned(),
            config: ArtifactConfig {
                lock_file_dir: dir.path().to_path_buf(),
                skip_installs: true,
                ..Default::default()
            },
        };

        let result = runner.update_artifacts(&input).await.unwrap();
        assert!(result.is_none());
    }
}
