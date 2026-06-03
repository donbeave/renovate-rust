//! Pixi artifact runner — lock file regeneration after `pixi.toml` edits.
//!
//! Ports `lib/modules/manager/pixi/artifacts.ts`.
//!
//! After the pixi.toml is updated, this runner:
//! 1. Reads the existing `pixi.lock`.
//! 2. Writes the updated `pixi.toml`.
//! 3. Runs `pixi lock --no-progress --color=never --quiet`.
//! 4. Checks if the lock file changed.
//! 5. Returns any changed files.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use crate::artifacts::{ArtifactError, ArtifactResult, ArtifactRunner, UpdateArtifact};
use crate::exec::raw::raw_exec;
use crate::exec::types::ExecOptions;

/// Artifact runner for Pixi (conda) projects.
#[derive(Debug, Clone, Default)]
pub struct PixiArtifactRunner;

impl PixiArtifactRunner {
    /// Create a new pixi artifact runner.
    pub fn new() -> Self {
        Self
    }

    /// Run a single pixi command.
    async fn run_command(
        &self,
        cmd: &str,
        cwd: &std::path::Path,
        env: &HashMap<String, String>,
    ) -> Result<(), ArtifactError> {
        let opts = ExecOptions {
            cwd: Some(cwd.to_string_lossy().to_string()),
            timeout: Some(300_000), // 5 minutes
            ..Default::default()
        };
        raw_exec(cmd, &opts, env)
            .await
            .map_err(|e| ArtifactError {
                lock_file: "pixi.lock".to_owned(),
                stderr: e.message,
            })
            .map(|_| ())
    }
}

impl ArtifactRunner for PixiArtifactRunner {
    fn update_artifacts(
        &self,
        input: &UpdateArtifact,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Vec<ArtifactResult>>, ArtifactError>> + Send + '_>>
    {
        let package_file_name = input.package_file_name.clone();
        let updated_deps = input.updated_deps.clone();
        let new_package_file_content = input.new_package_file_content.clone();
        let config = input.config.clone();

        Box::pin(async move {
            let lock_file_dir = &config.lock_file_dir;
            let file_path = lock_file_dir.join(&package_file_name);
            let package_dir = file_path
                .parent()
                .map(std::path::Path::to_path_buf)
                .unwrap_or_else(|| lock_file_dir.clone());

            // 1. Check if there's anything to do.
            if updated_deps.is_empty() && !config.is_lockfile_maintenance {
                return Ok(None);
            }

            // 2. Determine lock file path.
            let lock_path = if let Some(name) = package_file_name.rsplit_once('/') {
                lock_file_dir.join(format!("{}/pixi.lock", name.0))
            } else {
                lock_file_dir.join("pixi.lock")
            };

            // 3. Read existing lock file.
            let Ok(original_lock) = tokio::fs::read_to_string(&lock_path).await else {
                return Ok(None);
            };

            // 4. Write updated package file.
            if let Err(e) = tokio::fs::write(&file_path, &new_package_file_content).await {
                return Err(ArtifactError {
                    lock_file: lock_path.to_string_lossy().to_string(),
                    stderr: format!("failed to write pixi.toml: {}", e),
                });
            }

            // 5. For lockfile maintenance, delete the lock file first.
            if config.is_lockfile_maintenance {
                let _ = tokio::fs::remove_file(&lock_path).await;
            }

            // 6. Build and run command.
            let cmd = "pixi lock --no-progress --color=never --quiet";

            let mut env = if config.env.is_empty() {
                std::env::vars().collect::<HashMap<String, String>>()
            } else {
                config
                    .env
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect()
            };

            // Set PIXI_CACHE_DIR if not already set.
            if !env.contains_key("PIXI_CACHE_DIR") {
                env.insert(
                    "PIXI_CACHE_DIR".to_owned(),
                    std::env::temp_dir().to_string_lossy().to_string(),
                );
            }

            if let Err(err) = self.run_command(cmd, &package_dir, &env).await {
                return Ok(Some(vec![ArtifactResult::error(
                    lock_path.to_string_lossy().as_ref(),
                    err.stderr,
                )]));
            }

            // 7. Read updated lock file.
            let new_lock = match tokio::fs::read_to_string(&lock_path).await {
                Ok(c) => c,
                Err(e) => {
                    return Ok(Some(vec![ArtifactResult::error(
                        lock_path.to_string_lossy().as_ref(),
                        format!("failed to read updated lock file: {}", e),
                    )]));
                }
            };

            if original_lock == new_lock {
                return Ok(None);
            }

            let rel = if package_file_name.contains('/') {
                let dir = package_file_name
                    .rsplit_once('/')
                    .map(|(d, _)| d)
                    .unwrap_or("");
                if dir.is_empty() {
                    "pixi.lock".to_owned()
                } else {
                    format!("{}/pixi.lock", dir)
                }
            } else {
                "pixi.lock".to_owned()
            };

            Ok(Some(vec![ArtifactResult::file_change(rel, new_lock)]))
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
    use tempfile::tempdir;

    fn make_runner() -> PixiArtifactRunner {
        PixiArtifactRunner::new()
    }

    fn updated_dep(dep_name: &str) -> UpdatedDep {
        UpdatedDep {
            dep_name: dep_name.to_owned(),
            package_name: None,
            current_value: None,
            new_value: None,
            locked_version: None,
            new_version: None,
            package_file: "pixi.toml".to_owned(),
            manager: "pixi".to_owned(),
            datasource: None,
            update_type: None,
        }
    }

    // Ported: "returns null if no pixi.lock found" — lib/modules/manager/pixi/artifacts.spec.ts line 70
    #[tokio::test]
    async fn returns_null_if_no_pixi_lock_found() {
        let dir = tempdir().unwrap();
        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "pixi.toml".to_owned(),
            updated_deps: vec![],
            new_package_file_content: String::new(),
            config: ArtifactConfig {
                lock_file_dir: dir.path().to_path_buf(),
                is_lockfile_maintenance: true,
                ..Default::default()
            },
        };
        let result = runner.update_artifacts(&input).await;
        assert!(result.unwrap().is_none());
    }

    // Ported: "returns null if updatedDeps is empty" — lib/modules/manager/pixi/artifacts.spec.ts line 83
    #[tokio::test]
    async fn returns_null_if_updated_deps_empty() {
        let dir = tempdir().unwrap();
        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "pixi.toml".to_owned(),
            updated_deps: vec![],
            new_package_file_content: String::new(),
            config: ArtifactConfig {
                lock_file_dir: dir.path().to_path_buf(),
                ..Default::default()
            },
        };
        let result = runner.update_artifacts(&input).await;
        assert!(result.unwrap().is_none());
    }

    // Ported: "returns null if unchanged" — lib/modules/manager/pixi/artifacts.spec.ts line 96
    #[tokio::test]
    async fn returns_null_if_unchanged() {
        let dir = tempdir().unwrap();
        let lock_dir = dir.path().to_path_buf();

        std::fs::write(lock_dir.join("pixi.toml"), "[project]\n").unwrap();
        std::fs::write(lock_dir.join("pixi.lock"), "Old pixi.lock\n").unwrap();

        let fake_pixi = dir.path().join("pixi");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_pixi).unwrap();
            f.write_all(b"#!/bin/sh\necho 'Old pixi.lock' > pixi.lock\n")
                .unwrap();
            let mut perms = std::fs::metadata(&fake_pixi).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_pixi, perms).unwrap();
        }

        let mut env = std::env::vars().collect::<HashMap<String, String>>();
        let mut path = lock_dir.to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);

        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "pixi.toml".to_owned(),
            updated_deps: vec![],
            new_package_file_content: "[project]\n".to_owned(),
            config: ArtifactConfig {
                lock_file_dir: lock_dir.clone(),
                env: env.into_iter().collect(),
                is_lockfile_maintenance: true,
                ..Default::default()
            },
        };
        let result = runner.update_artifacts(&input).await;
        assert!(result.unwrap().is_none());
    }

    // Ported: "returns updated pixi.lock using docker" — lib/modules/manager/pixi/artifacts.spec.ts line 140
    #[tokio::test]
    async fn returns_updated_pixi_lock() {
        let dir = tempdir().unwrap();
        let lock_dir = dir.path().to_path_buf();

        std::fs::write(lock_dir.join("pixi.toml"), "[project]\n").unwrap();
        std::fs::write(lock_dir.join("pixi.lock"), "Old pixi.lock\n").unwrap();

        let fake_pixi = dir.path().join("pixi");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_pixi).unwrap();
            f.write_all(b"#!/bin/sh\necho 'New pixi.lock' > pixi.lock\n")
                .unwrap();
            let mut perms = std::fs::metadata(&fake_pixi).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_pixi, perms).unwrap();
        }

        let mut env = std::env::vars().collect::<HashMap<String, String>>();
        let mut path = lock_dir.to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);

        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "pixi.toml".to_owned(),
            updated_deps: vec![updated_dep("python")],
            new_package_file_content: "[project]\n".to_owned(),
            config: ArtifactConfig {
                lock_file_dir: lock_dir.clone(),
                env: env.into_iter().collect(),
                ..Default::default()
            },
        };
        let result = runner.update_artifacts(&input).await.unwrap();
        let results = result.expect("runner should return results");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].file.as_ref().unwrap().path, "pixi.lock");
    }

    // Ported: "returns updated pixi.lock when doing lockfile maintenance" — lib/modules/manager/pixi/artifacts.spec.ts line 348
    #[tokio::test]
    async fn performs_lockfile_maintenance() {
        let dir = tempdir().unwrap();
        let lock_dir = dir.path().to_path_buf();

        std::fs::write(lock_dir.join("pixi.toml"), "[project]\n").unwrap();
        std::fs::write(lock_dir.join("pixi.lock"), "Old pixi.lock\n").unwrap();

        let fake_pixi = dir.path().join("pixi");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_pixi).unwrap();
            f.write_all(b"#!/bin/sh\necho 'New pixi.lock' > pixi.lock\n")
                .unwrap();
            let mut perms = std::fs::metadata(&fake_pixi).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_pixi, perms).unwrap();
        }

        let mut env = std::env::vars().collect::<HashMap<String, String>>();
        let mut path = lock_dir.to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);

        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "pixi.toml".to_owned(),
            updated_deps: vec![],
            new_package_file_content: "[project]\n".to_owned(),
            config: ArtifactConfig {
                lock_file_dir: lock_dir.clone(),
                env: env.into_iter().collect(),
                is_lockfile_maintenance: true,
                ..Default::default()
            },
        };
        let result = runner.update_artifacts(&input).await.unwrap();
        let results = result.expect("runner should return results");
        assert_eq!(results.len(), 1);
    }

    // Ported: "catches errors" — lib/modules/manager/pixi/artifacts.spec.ts line 328
    #[tokio::test]
    async fn catches_errors() {
        let dir = tempdir().unwrap();
        let lock_dir = dir.path().to_path_buf();

        std::fs::write(lock_dir.join("pixi.toml"), "[project]\n").unwrap();
        std::fs::write(lock_dir.join("pixi.lock"), "Old pixi.lock\n").unwrap();

        let fake_pixi = dir.path().join("pixi");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_pixi).unwrap();
            f.write_all(b"#!/bin/sh\necho 'pixi error' >&2; exit 1\n")
                .unwrap();
            let mut perms = std::fs::metadata(&fake_pixi).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_pixi, perms).unwrap();
        }

        let mut env = std::env::vars().collect::<HashMap<String, String>>();
        let mut path = lock_dir.to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);

        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "pixi.toml".to_owned(),
            updated_deps: vec![updated_dep("python")],
            new_package_file_content: "[project]\n".to_owned(),
            config: ArtifactConfig {
                lock_file_dir: lock_dir.clone(),
                env: env.into_iter().collect(),
                ..Default::default()
            },
        };
        let result = runner.update_artifacts(&input).await.unwrap();
        let results = result.expect("runner should return error results");
        assert_eq!(results.len(), 1);
        assert!(results[0].artifact_error.is_some());
    }
}
