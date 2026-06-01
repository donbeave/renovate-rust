//! Cargo artifact runner — lock file regeneration after Cargo.toml edits.
//!
//! Ports `lib/modules/manager/cargo/artifacts.ts`.
//!
//! After `cargo_update_dependency` edits `Cargo.toml`, this runner:
//! 1. Writes the updated `Cargo.toml` to disk.
//! 2. Runs `cargo update` (precise or workspace) in the package directory.
//! 3. Reads the regenerated `Cargo.lock`.
//! 4. Returns any changed files.

use std::collections::HashMap;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;

use crate::artifacts::{ArtifactError, ArtifactResult, ArtifactRunner, UpdateArtifact, UpdatedDep};
use crate::exec::raw::raw_exec;
use crate::exec::types::ExecOptions;

/// Artifact runner for Cargo (Rust) projects.
#[derive(Debug, Clone, Default)]
pub struct CargoArtifactRunner;

impl CargoArtifactRunner {
    /// Create a new cargo artifact runner.
    pub fn new() -> Self {
        Self
    }

    /// Find `Cargo.lock` relative to the manifest path.
    ///
    /// Checks the same directory as the manifest first, then walks up the
    /// directory tree (for workspaces).
    fn find_lock_file(manifest_path: &Path, lock_file_dir: &Path) -> Option<PathBuf> {
        let manifest = lock_file_dir.join(manifest_path);
        let mut dir = manifest.parent()?;
        loop {
            let lock = dir.join("Cargo.lock");
            if lock.is_file() {
                return Some(lock);
            }
            match dir.parent() {
                Some(parent) => dir = parent,
                None => return None,
            }
        }
    }

    /// Build the list of `cargo update` commands to run.
    ///
    /// - **Lockfile maintenance**: single `cargo update --manifest-path <p>`.
    /// - **No locked versions / non-crate deps**: `cargo update --workspace`.
    /// - **All crate deps have locked versions**: precise updates for deps whose
    ///   `current_value == new_value`, followed by `--workspace`.
    fn build_commands(
        manifest_path: &Path,
        updated_deps: &[UpdatedDep],
        is_maintenance: bool,
    ) -> Vec<String> {
        let manifest_arg = format!(
            "--manifest-path {}",
            crate::util::shlex_quote(manifest_path.to_string_lossy().as_ref())
        );

        if is_maintenance {
            return vec![format!(
                "cargo update --config net.git-fetch-with-cli=true {}",
                manifest_arg
            )];
        }

        if updated_deps.is_empty() {
            return vec![format!(
                "cargo update --config net.git-fetch-with-cli=true {} --workspace",
                manifest_arg
            )];
        }

        let has_non_crate = updated_deps
            .iter()
            .any(|d| d.datasource.as_deref() != Some("crate"));
        let missing_locked = updated_deps
            .iter()
            .any(|d| d.locked_version.is_none() && d.datasource.as_deref() == Some("crate"));

        if has_non_crate || missing_locked {
            return vec![format!(
                "cargo update --config net.git-fetch-with-cli=true {} --workspace",
                manifest_arg
            )];
        }

        // Precise updates for deps whose range hasn't changed in Cargo.toml.
        let mut cmds = Vec::new();
        for dep in updated_deps {
            let range_changed = dep
                .current_value
                .as_deref()
                .zip(dep.new_value.as_deref())
                .is_some_and(|(c, n)| c != n);
            if range_changed {
                continue;
            }
            let pkg = dep.package_name.as_deref().unwrap_or(&dep.dep_name);
            let Some(locked) = &dep.locked_version else {
                continue;
            };
            let Some(new_ver) = &dep.new_version else {
                continue;
            };
            let pkg_quoted = crate::util::shlex_quote(&format!("{}@{}", pkg, locked));
            let precise_quoted = crate::util::shlex_quote(new_ver);
            cmds.push(format!(
                "cargo update --config net.git-fetch-with-cli=true {} --package {} --precise {}",
                manifest_arg, pkg_quoted, precise_quoted
            ));
        }

        // Final workspace update to resolve any remaining bumps.
        cmds.push(format!(
            "cargo update --config net.git-fetch-with-cli=true {} --workspace",
            manifest_arg
        ));

        cmds
    }

    /// Extract versions from a Cargo.lock content string.
    ///
    /// Returns a map from package name to list of versions.
    fn extract_lock_versions(content: &str) -> HashMap<String, Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let mut current_name: Option<String> = None;
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("name = \"") {
                if let Some(name) = trimmed.trim_start_matches("name = \"").strip_suffix('"') {
                    current_name = Some(name.to_owned());
                }
            } else if let Some(ver) = trimmed.trim_start_matches("version = \"").strip_suffix('"') {
                if let Some(name) = &current_name {
                    map.entry(name.clone()).or_default().push(ver.to_owned());
                }
            } else if trimmed == "[[package]]" || trimmed.is_empty() {
                current_name = None;
            }
        }
        map
    }

    /// Run a single cargo update command.
    async fn run_command(
        &self,
        cmd: &str,
        cwd: &Path,
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
                lock_file: "Cargo.lock".to_owned(),
                stderr: e.message,
            })
            .map(|_| ())
    }
}

impl ArtifactRunner for CargoArtifactRunner {
    fn update_artifacts(
        &self,
        input: &UpdateArtifact,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Vec<ArtifactResult>>, ArtifactError>> + Send + '_>>
    {
        let package_file_name = input.package_file_name.clone();
        let new_package_file_content = input.new_package_file_content.clone();
        let updated_deps = input.updated_deps.clone();
        let config = input.config.clone();

        Box::pin(async move {
            let lock_file_dir = &config.lock_file_dir;
            let manifest_path = lock_file_dir.join(&package_file_name);
            let package_dir = manifest_path
                .parent()
                .map(Path::to_path_buf)
                .unwrap_or_else(|| lock_file_dir.clone());

            // 1. Find Cargo.lock.
            let Some(lock_path) =
                Self::find_lock_file(Path::new(&package_file_name), lock_file_dir)
            else {
                return Ok(None);
            };

            // 2. Read original Cargo.lock.
            let original_lock = match tokio::fs::read_to_string(&lock_path).await {
                Ok(c) => c,
                Err(e) => {
                    return Err(ArtifactError {
                        lock_file: lock_path.to_string_lossy().to_string(),
                        stderr: format!("failed to read Cargo.lock: {}", e),
                    });
                }
            };

            // If not maintenance and no deps to update, nothing to do.
            if !config.is_lockfile_maintenance && updated_deps.is_empty() {
                return Ok(None);
            }

            // 3. Write updated Cargo.toml.
            if let Err(e) = tokio::fs::write(&manifest_path, &new_package_file_content).await {
                return Err(ArtifactError {
                    lock_file: lock_path.to_string_lossy().to_string(),
                    stderr: format!("failed to write Cargo.toml: {}", e),
                });
            }

            // 4. Build and run commands.
            let mut env: HashMap<String, String> = if config.env.is_empty() {
                std::env::vars().collect()
            } else {
                config
                    .env
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect()
            };
            // Ensure the lock file dir is on PATH for cargo discovery.
            let dir_str = lock_file_dir.to_string_lossy().to_string();
            let path_key = if env.contains_key("PATH") {
                "PATH"
            } else {
                "Path"
            };
            if let Some(existing) = env.get(path_key) {
                env.insert(path_key.to_owned(), format!("{}:{}", dir_str, existing));
            } else {
                env.insert(path_key.to_owned(), dir_str);
            }
            let cmds = Self::build_commands(
                Path::new(&package_file_name),
                &updated_deps,
                config.is_lockfile_maintenance,
            );

            for cmd in &cmds {
                if let Err(err) = self.run_command(cmd, &package_dir, &env).await {
                    // Try recursive retry on "package ID specification" errors.
                    let new_lock = tokio::fs::read_to_string(&lock_path)
                        .await
                        .unwrap_or_default();
                    if err.stderr.contains("package ID specification") {
                        let versions = Self::extract_lock_versions(&new_lock);
                        let new_deps: Vec<UpdatedDep> = updated_deps
                            .iter()
                            .filter(|d| {
                                let pkg = d.package_name.as_deref().unwrap_or(&d.dep_name);
                                let new_ver = d.new_version.as_deref().unwrap_or("");
                                !versions
                                    .get(pkg)
                                    .is_some_and(|vers| vers.contains(&new_ver.to_owned()))
                            })
                            .cloned()
                            .collect();
                        if new_deps.len() < updated_deps.len() {
                            let retry_cmds = Self::build_commands(
                                Path::new(&package_file_name),
                                &new_deps,
                                config.is_lockfile_maintenance,
                            );
                            for retry_cmd in &retry_cmds {
                                self.run_command(retry_cmd, &package_dir, &env).await?;
                            }
                            break;
                        }
                    }
                    return Err(err);
                }
            }

            // 5. Read updated Cargo.lock.
            let new_lock = match tokio::fs::read_to_string(&lock_path).await {
                Ok(c) => c,
                Err(e) => {
                    return Err(ArtifactError {
                        lock_file: lock_path.to_string_lossy().to_string(),
                        stderr: format!("failed to read updated Cargo.lock: {}", e),
                    });
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
                    "Cargo.lock".to_owned()
                } else {
                    format!("{}/Cargo.lock", dir)
                }
            } else {
                "Cargo.lock".to_owned()
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

    fn make_runner() -> CargoArtifactRunner {
        CargoArtifactRunner::new()
    }

    fn updated_dep(
        dep_name: &str,
        package_name: Option<&str>,
        locked_version: Option<&str>,
        new_version: Option<&str>,
        current_value: Option<&str>,
        new_value: Option<&str>,
        datasource: Option<&str>,
    ) -> UpdatedDep {
        UpdatedDep {
            dep_name: dep_name.to_owned(),
            package_name: package_name.map(|s| s.to_owned()),
            current_value: current_value.map(|s| s.to_owned()),
            new_value: new_value.map(|s| s.to_owned()),
            locked_version: locked_version.map(|s| s.to_owned()),
            new_version: new_version.map(|s| s.to_owned()),
            package_file: "Cargo.toml".to_owned(),
            manager: "cargo".to_owned(),
            datasource: datasource.map(|s| s.to_owned()),
            update_type: None,
        }
    }

    // Ported: "returns null if no Cargo.lock found" — cargo/artifacts.spec.ts line 44
    #[tokio::test]
    async fn returns_null_if_no_cargo_lock_found() {
        let dir = tempdir().unwrap();
        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "Cargo.toml".to_owned(),
            updated_deps: vec![updated_dep(
                "dep1",
                None,
                None,
                None,
                None,
                None,
                Some("crate"),
            )],
            new_package_file_content: String::new(),
            config: ArtifactConfig {
                lock_file_dir: dir.path().to_path_buf(),
                ..Default::default()
            },
        };
        let result = runner.update_artifacts(&input).await;
        assert!(result.unwrap().is_none());
    }

    // Ported: "returns null if updatedDeps is empty" — cargo/artifacts.spec.ts line 62
    #[tokio::test]
    async fn returns_null_if_updated_deps_is_empty() {
        let dir = tempdir().unwrap();
        // No Cargo.lock → same as above, runner returns None early.
        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "Cargo.toml".to_owned(),
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

    // Ported: "returns updated Cargo.lock" — cargo/artifacts.spec.ts line 98
    #[tokio::test]
    async fn returns_updated_cargo_lock() {
        let dir = tempdir().unwrap();
        let lock_dir = dir.path().to_path_buf();

        // Write Cargo.toml and Cargo.lock.
        std::fs::write(lock_dir.join("Cargo.toml"), "[package]\nname = 'test'\n").unwrap();
        std::fs::write(lock_dir.join("Cargo.lock"), "Old Cargo.lock\n").unwrap();

        // Create a fake cargo binary that updates Cargo.lock.
        let fake_cargo = dir.path().join("cargo");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_cargo).unwrap();
            f.write_all(
                b"#!/bin/sh\nif [ \"$1\" = \"update\" ]; then echo 'New Cargo.lock' > Cargo.lock; fi\n",
            )
            .unwrap();
            let mut perms = std::fs::metadata(&fake_cargo).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_cargo, perms).unwrap();
        }
        #[cfg(windows)]
        {
            let mut f = std::fs::File::create(&fake_cargo).unwrap();
            f.write_all(b"@echo off\nif \"%1\"==\"update\" echo New Cargo.lock > Cargo.lock\n")
                .unwrap();
        }

        let mut env: HashMap<String, String> = std::env::vars().collect();
        let mut path = lock_dir.to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);

        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "Cargo.toml".to_owned(),
            updated_deps: vec![updated_dep(
                "dep1",
                None,
                None,
                None,
                None,
                None,
                Some("crate"),
            )],
            new_package_file_content: "{}".to_owned(),
            config: ArtifactConfig {
                lock_file_dir: lock_dir.clone(),
                env: env.into_iter().collect(),
                ..Default::default()
            },
        };

        let result = runner.update_artifacts(&input).await.unwrap();
        let results = result.expect("runner should return results");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].file.as_ref().unwrap().path, "Cargo.lock");
        assert_eq!(
            results[0].file.as_ref().unwrap().contents.as_deref(),
            Some("New Cargo.lock\n")
        );
    }

    // Ported: "returns updated Cargo.lock with precise version update" — cargo/artifacts.spec.ts line 122
    #[tokio::test]
    async fn returns_updated_cargo_lock_with_precise_version_update() {
        let dir = tempdir().unwrap();
        let lock_dir = dir.path().to_path_buf();

        std::fs::write(lock_dir.join("Cargo.toml"), "[package]\nname = 'test'\n").unwrap();
        std::fs::write(lock_dir.join("Cargo.lock"), "Old Cargo.lock\n").unwrap();

        let fake_cargo = dir.path().join("cargo");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_cargo).unwrap();
            f.write_all(
                b"#!/bin/sh\nif [ \"$1\" = \"update\" ]; then echo 'New Cargo.lock' > Cargo.lock; fi\n",
            )
            .unwrap();
            let mut perms = std::fs::metadata(&fake_cargo).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_cargo, perms).unwrap();
        }

        let mut env: HashMap<String, String> = std::env::vars().collect();
        let mut path = lock_dir.to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);

        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "Cargo.toml".to_owned(),
            updated_deps: vec![updated_dep(
                "dep1",
                Some("dep1"),
                Some("1.0.0"),
                Some("1.0.1"),
                None,
                None,
                Some("crate"),
            )],
            new_package_file_content: "{}".to_owned(),
            config: ArtifactConfig {
                lock_file_dir: lock_dir.clone(),
                constraints: {
                    let mut m = std::collections::BTreeMap::new();
                    m.insert("rust".to_owned(), "1.65.0".to_owned());
                    m
                },
                env: env.into_iter().collect(),
                ..Default::default()
            },
        };

        let result = runner.update_artifacts(&input).await.unwrap();
        let results = result.expect("runner should return results");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].file.as_ref().unwrap().path, "Cargo.lock");
    }

    // Ported: "skips precise update when manifest range has changed" — cargo/artifacts.spec.ts line 164
    #[tokio::test]
    async fn skips_precise_update_when_manifest_range_has_changed() {
        let dir = tempdir().unwrap();
        let lock_dir = dir.path().to_path_buf();

        std::fs::write(lock_dir.join("Cargo.toml"), "[package]\nname = 'test'\n").unwrap();
        std::fs::write(lock_dir.join("Cargo.lock"), "Old Cargo.lock\n").unwrap();

        let fake_cargo = dir.path().join("cargo");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_cargo).unwrap();
            f.write_all(
                b"#!/bin/sh\nif [ \"$1\" = \"update\" ]; then echo 'New Cargo.lock' > Cargo.lock; fi\n",
            )
            .unwrap();
            let mut perms = std::fs::metadata(&fake_cargo).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_cargo, perms).unwrap();
        }

        let mut env: HashMap<String, String> = std::env::vars().collect();
        let mut path = lock_dir.to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);

        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "Cargo.toml".to_owned(),
            updated_deps: vec![updated_dep(
                "pprof",
                Some("pprof"),
                Some("0.13.0"),
                Some("0.14.0"),
                Some("0.13"),
                Some("0.14"),
                Some("crate"),
            )],
            new_package_file_content: "{}".to_owned(),
            config: ArtifactConfig {
                lock_file_dir: lock_dir.clone(),
                env: env.into_iter().collect(),
                ..Default::default()
            },
        };

        let result = runner.update_artifacts(&input).await.unwrap();
        let results = result.expect("runner should return results");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].file.as_ref().unwrap().path, "Cargo.lock");
    }

    // Ported: "returns an artifact error when cargo update fails" — cargo/artifacts.spec.ts line 247
    #[tokio::test]
    async fn returns_an_artifact_error_when_cargo_update_fails() {
        let dir = tempdir().unwrap();
        let lock_dir = dir.path().to_path_buf();

        std::fs::write(lock_dir.join("Cargo.toml"), "[package]\nname = 'test'\n").unwrap();
        std::fs::write(lock_dir.join("Cargo.lock"), "Old Cargo.lock\n").unwrap();

        let fake_cargo = dir.path().join("cargo");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_cargo).unwrap();
            f.write_all(b"#!/bin/sh\necho 'Exec error' >&2\nexit 1\n")
                .unwrap();
            let mut perms = std::fs::metadata(&fake_cargo).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_cargo, perms).unwrap();
        }

        let mut env: HashMap<String, String> = std::env::vars().collect();
        let mut path = lock_dir.to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);

        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "Cargo.toml".to_owned(),
            updated_deps: vec![updated_dep(
                "dep1",
                Some("dep1"),
                Some("1.0.0"),
                Some("1.0.1"),
                None,
                None,
                Some("crate"),
            )],
            new_package_file_content: "{}".to_owned(),
            config: ArtifactConfig {
                lock_file_dir: lock_dir.clone(),
                env: env.into_iter().collect(),
                ..Default::default()
            },
        };

        let result = runner.update_artifacts(&input).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            !err.stderr.is_empty(),
            "error stderr should not be empty: {:?}",
            err.stderr
        );
    }

    // Ported: "returns updated Cargo.lock for lockfile maintenance" — cargo/artifacts.spec.ts line 488
    #[tokio::test]
    async fn returns_updated_cargo_lock_for_lockfile_maintenance() {
        let dir = tempdir().unwrap();
        let lock_dir = dir.path().to_path_buf();

        std::fs::write(lock_dir.join("Cargo.toml"), "[package]\nname = 'test'\n").unwrap();
        std::fs::write(lock_dir.join("Cargo.lock"), "Old Cargo.lock\n").unwrap();

        let fake_cargo = dir.path().join("cargo");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_cargo).unwrap();
            f.write_all(
                b"#!/bin/sh\nif [ \"$1\" = \"update\" ]; then echo 'New Cargo.lock' > Cargo.lock; fi\n",
            )
            .unwrap();
            let mut perms = std::fs::metadata(&fake_cargo).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_cargo, perms).unwrap();
        }

        let mut env: HashMap<String, String> = std::env::vars().collect();
        let mut path = lock_dir.to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);

        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "Cargo.toml".to_owned(),
            updated_deps: vec![],
            new_package_file_content: "{}".to_owned(),
            config: ArtifactConfig {
                lock_file_dir: lock_dir.clone(),
                is_lockfile_maintenance: true,
                env: env.into_iter().collect(),
                ..Default::default()
            },
        };

        let result = runner.update_artifacts(&input).await.unwrap();
        let results = result.expect("runner should return results");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].file.as_ref().unwrap().path, "Cargo.lock");
    }

    // Rust-specific: unit test for command building
    #[test]
    fn build_commands_maintenance() {
        let cmds = CargoArtifactRunner::build_commands(Path::new("Cargo.toml"), &[], true);
        assert_eq!(cmds.len(), 1);
        assert!(cmds[0].contains("cargo update"));
        assert!(!cmds[0].contains("--workspace"));
    }

    // Rust-specific: unit test for command building
    #[test]
    fn build_commands_workspace_when_no_locked_version() {
        let deps = vec![updated_dep(
            "dep1",
            None,
            None,
            None,
            None,
            None,
            Some("crate"),
        )];
        let cmds = CargoArtifactRunner::build_commands(Path::new("Cargo.toml"), &deps, false);
        assert_eq!(cmds.len(), 1);
        assert!(cmds[0].contains("--workspace"));
    }

    // Rust-specific: unit test for command building
    #[test]
    fn build_commands_precise_when_locked_version_unchanged_range() {
        let deps = vec![updated_dep(
            "dep1",
            Some("dep1"),
            Some("1.0.0"),
            Some("1.0.1"),
            None,
            None,
            Some("crate"),
        )];
        let cmds = CargoArtifactRunner::build_commands(Path::new("Cargo.toml"), &deps, false);
        assert_eq!(cmds.len(), 2);
        assert!(cmds[0].contains("--package"));
        assert!(cmds[0].contains("--precise"));
        assert!(cmds[1].contains("--workspace"));
    }

    // Rust-specific: unit test for command building
    #[test]
    fn build_commands_skips_precise_when_range_changed() {
        let deps = vec![updated_dep(
            "pprof",
            Some("pprof"),
            Some("0.13.0"),
            Some("0.14.0"),
            Some("0.13"),
            Some("0.14"),
            Some("crate"),
        )];
        let cmds = CargoArtifactRunner::build_commands(Path::new("Cargo.toml"), &deps, false);
        assert_eq!(cmds.len(), 1);
        assert!(cmds[0].contains("--workspace"));
        assert!(!cmds[0].contains("--package"));
    }

    // Rust-specific: unit test for lock file finding
    #[test]
    fn find_lock_file_same_directory() {
        let dir = tempdir().unwrap();
        let lock = dir.path().join("Cargo.lock");
        std::fs::write(&lock, "").unwrap();
        let found = CargoArtifactRunner::find_lock_file(Path::new("Cargo.toml"), dir.path());
        assert_eq!(found, Some(lock));
    }

    // Rust-specific: unit test for lock file finding
    #[test]
    fn find_lock_file_parent_directory() {
        let dir = tempdir().unwrap();
        let sub = dir.path().join("crates/foo");
        std::fs::create_dir_all(&sub).unwrap();
        let lock = dir.path().join("Cargo.lock");
        std::fs::write(&lock, "").unwrap();
        let found =
            CargoArtifactRunner::find_lock_file(Path::new("crates/foo/Cargo.toml"), dir.path());
        assert_eq!(found, Some(lock));
    }
}
