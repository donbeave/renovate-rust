//! Bundler artifact runner — lock file regeneration after Gemfile edits.
//!
//! Ports `lib/modules/manager/bundler/artifacts.ts`.
//!
//! After the Gemfile is updated, this runner:
//! 1. Reads the existing Gemfile.lock.
//! 2. Writes the updated Gemfile.
//! 3. Runs `bundler lock --update` (or variant) commands.
//! 4. Checks if the lock file changed.
//! 5. Returns any changed files.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use crate::artifacts::{ArtifactError, ArtifactResult, ArtifactRunner, UpdateArtifact};
use crate::exec::raw::raw_exec;
use crate::exec::types::ExecOptions;

/// Artifact runner for Bundler (Ruby) projects.
#[derive(Debug, Clone, Default)]
pub struct BundlerArtifactRunner;

impl BundlerArtifactRunner {
    /// Create a new bundler artifact runner.
    pub fn new() -> Self {
        Self
    }

    /// Build the bundler lock commands to run.
    fn build_commands(
        updated_deps: &[crate::artifacts::UpdatedDep],
        is_maintenance: bool,
        post_update_options: &[String],
    ) -> Vec<String> {
        if is_maintenance {
            return vec!["bundler lock --update".to_owned()];
        }

        let bundler_upgraded = updated_deps.iter().any(|d| d.dep_name == "bundler");
        let ruby_upgraded = updated_deps.iter().any(|d| d.dep_name == "ruby");

        let mut cmds = Vec::new();

        if bundler_upgraded {
            cmds.push("bundler lock --update --bundler".to_owned());
        }

        let conservative = post_update_options.contains(&"bundlerConservative".to_owned());
        let update_types = [("patch", "--patch"), ("minor", "--minor"), ("major", "")];

        for (update_type, arg) in &update_types {
            let deps: Vec<String> = updated_deps
                .iter()
                .filter(|d| {
                    d.update_type.as_deref().unwrap_or("major") == *update_type
                        && d.dep_name != "ruby"
                        && d.dep_name != "bundler"
                })
                .map(|d| d.dep_name.clone())
                .collect();

            if !deps.is_empty() {
                let mut flags: Vec<&str> = Vec::new();
                if !arg.is_empty() {
                    flags.push(arg);
                }
                if conservative {
                    flags.push("--conservative");
                }
                let prefix = if flags.is_empty() {
                    "".to_owned()
                } else {
                    format!("{} ", flags.join(" "))
                };
                let quoted: Vec<String> =
                    deps.iter().map(|d| crate::util::shlex_quote(d)).collect();
                cmds.push(format!(
                    "bundler lock {}--update {}",
                    prefix,
                    quoted.join(" ")
                ));
            }
        }

        if ruby_upgraded && cmds.is_empty() {
            cmds.push("bundler lock".to_owned());
        }

        if cmds.is_empty() && !updated_deps.is_empty() {
            cmds.push("bundler lock --update".to_owned());
        }

        cmds
    }

    /// Run a single bundler command.
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
                lock_file: "Gemfile.lock".to_owned(),
                stderr: e.message,
            })
            .map(|_| ())
    }
}

impl ArtifactRunner for BundlerArtifactRunner {
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

            // 1. Determine lock file path.
            let Ok(lock_path) =
                crate::extractors::bundler::get_lock_file_path(&package_file_name, lock_file_dir)
                    .await
            else {
                return Ok(None);
            };

            // 2. Read existing lock file.
            let Ok(original_lock) = tokio::fs::read_to_string(&lock_path).await else {
                return Ok(None);
            };

            // 3. Write updated Gemfile.
            if let Err(e) = tokio::fs::write(&file_path, &new_package_file_content).await {
                return Err(ArtifactError {
                    lock_file: lock_path.to_string_lossy().to_string(),
                    stderr: format!("failed to write Gemfile: {}", e),
                });
            }

            // 4. Build and run commands.
            let cmds = Self::build_commands(
                &updated_deps,
                config.is_lockfile_maintenance,
                &config.post_update_options,
            );

            if cmds.is_empty() {
                return Ok(None);
            }

            let env = if config.env.is_empty() {
                std::env::vars().collect::<HashMap<String, String>>()
            } else {
                config
                    .env
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect()
            };

            for cmd in &cmds {
                if let Err(err) = self.run_command(cmd, &package_dir, &env).await {
                    let ae = ArtifactError {
                        lock_file: lock_path.to_string_lossy().to_string(),
                        stderr: err.stderr,
                    };
                    if ae.stderr.contains("temporary-error") {
                        return Err(ae);
                    }
                    return Ok(Some(vec![ArtifactResult::error(
                        lock_path.to_string_lossy().as_ref(),
                        ae.stderr,
                    )]));
                }
            }

            // 5. Read updated lock file.
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
                    "Gemfile.lock".to_owned()
                } else {
                    format!("{}/Gemfile.lock", dir)
                }
            } else {
                "Gemfile.lock".to_owned()
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

    fn make_runner() -> BundlerArtifactRunner {
        BundlerArtifactRunner::new()
    }

    fn updated_dep(dep_name: &str) -> UpdatedDep {
        UpdatedDep {
            dep_name: dep_name.to_owned(),
            package_name: None,
            current_value: None,
            new_value: None,
            locked_version: None,
            new_version: None,
            package_file: "Gemfile".to_owned(),
            manager: "bundler".to_owned(),
            datasource: None,
            update_type: None,
        }
    }

    // Ported: "returns null by default" — lib/modules/manager/bundler/artifacts.spec.ts line 66
    #[tokio::test]
    async fn returns_null_by_default() {
        let dir = tempdir().unwrap();
        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "Gemfile".to_owned(),
            updated_deps: vec![updated_dep("rails")],
            new_package_file_content: String::new(),
            config: ArtifactConfig {
                lock_file_dir: dir.path().to_path_buf(),
                ..Default::default()
            },
        };
        let result = runner.update_artifacts(&input).await;
        assert!(result.unwrap().is_none());
    }

    // Ported: "returns null if Gemfile.lock was not changed" — lib/modules/manager/bundler/artifacts.spec.ts line 77
    #[tokio::test]
    async fn returns_null_if_unchanged() {
        let dir = tempdir().unwrap();
        let lock_dir = dir.path().to_path_buf();

        std::fs::write(lock_dir.join("Gemfile"), "source 'https://rubygems.org'\n").unwrap();
        std::fs::write(lock_dir.join("Gemfile.lock"), "Old Gemfile.lock\n").unwrap();

        let fake_bundler = dir.path().join("bundler");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_bundler).unwrap();
            f.write_all(b"#!/bin/sh\nexit 0\n").unwrap();
            let mut perms = std::fs::metadata(&fake_bundler).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_bundler, perms).unwrap();
        }

        let mut env = std::env::vars().collect::<HashMap<String, String>>();
        let mut path = lock_dir.to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);

        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "Gemfile".to_owned(),
            updated_deps: vec![updated_dep("rails")],
            new_package_file_content: "source 'https://rubygems.org'\n".to_owned(),
            config: ArtifactConfig {
                lock_file_dir: lock_dir.clone(),
                env: env.into_iter().collect(),
                ..Default::default()
            },
        };
        let result = runner.update_artifacts(&input).await;
        assert!(result.unwrap().is_none());
    }

    // Ported: "works for default binarySource" — lib/modules/manager/bundler/artifacts.spec.ts line 123
    // Ported: "works explicit global binarySource" — lib/modules/manager/bundler/artifacts.spec.ts line 149
    #[tokio::test]
    async fn returns_updated_file() {
        let dir = tempdir().unwrap();
        let lock_dir = dir.path().to_path_buf();

        std::fs::write(lock_dir.join("Gemfile"), "source 'https://rubygems.org'\n").unwrap();
        std::fs::write(lock_dir.join("Gemfile.lock"), "Old Gemfile.lock\n").unwrap();

        let fake_bundler = dir.path().join("bundler");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_bundler).unwrap();
            f.write_all(b"#!/bin/sh\necho 'New Gemfile.lock' > Gemfile.lock\n")
                .unwrap();
            let mut perms = std::fs::metadata(&fake_bundler).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_bundler, perms).unwrap();
        }

        let mut env = std::env::vars().collect::<HashMap<String, String>>();
        let mut path = lock_dir.to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);

        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "Gemfile".to_owned(),
            updated_deps: vec![updated_dep("rails")],
            new_package_file_content: "source 'https://rubygems.org'\n".to_owned(),
            config: ArtifactConfig {
                lock_file_dir: lock_dir.clone(),
                env: env.into_iter().collect(),
                ..Default::default()
            },
        };
        let result = runner.update_artifacts(&input).await.unwrap();
        let results = result.expect("runner should return results");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].file.as_ref().unwrap().path, "Gemfile.lock");
    }

    // Ported: "performs lockFileMaintenance" — lib/modules/manager/bundler/artifacts.spec.ts line 517
    #[tokio::test]
    async fn performs_lockfile_maintenance() {
        let dir = tempdir().unwrap();
        let lock_dir = dir.path().to_path_buf();

        std::fs::write(lock_dir.join("Gemfile"), "source 'https://rubygems.org'\n").unwrap();
        std::fs::write(lock_dir.join("Gemfile.lock"), "Old Gemfile.lock\n").unwrap();

        let fake_bundler = dir.path().join("bundler");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_bundler).unwrap();
            f.write_all(b"#!/bin/sh\necho 'New Gemfile.lock' > Gemfile.lock\n")
                .unwrap();
            let mut perms = std::fs::metadata(&fake_bundler).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_bundler, perms).unwrap();
        }

        let mut env = std::env::vars().collect::<HashMap<String, String>>();
        let mut path = lock_dir.to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);

        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "Gemfile".to_owned(),
            updated_deps: vec![],
            new_package_file_content: "source 'https://rubygems.org'\n".to_owned(),
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

    // Ported: "updates the Gemfile.lock when upgrading ruby" — lib/modules/manager/bundler/artifacts.spec.ts line 678
    #[tokio::test]
    async fn updates_gemfile_lock_when_upgrading_ruby() {
        let dir = tempdir().unwrap();
        let lock_dir = dir.path().to_path_buf();

        std::fs::write(lock_dir.join("Gemfile"), "source 'https://rubygems.org'\n").unwrap();
        std::fs::write(lock_dir.join("Gemfile.lock"), "Old Gemfile.lock\n").unwrap();

        let fake_bundler = dir.path().join("bundler");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_bundler).unwrap();
            f.write_all(b"#!/bin/sh\necho 'New Gemfile.lock' > Gemfile.lock\n")
                .unwrap();
            let mut perms = std::fs::metadata(&fake_bundler).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_bundler, perms).unwrap();
        }

        let mut env = std::env::vars().collect::<HashMap<String, String>>();
        let mut path = lock_dir.to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);

        let runner = make_runner();
        let mut dep = updated_dep("ruby");
        dep.update_type = Some("patch".to_owned());
        let input = UpdateArtifact {
            package_file_name: "Gemfile".to_owned(),
            updated_deps: vec![dep],
            new_package_file_content: "source 'https://rubygems.org'\n".to_owned(),
            config: ArtifactConfig {
                lock_file_dir: lock_dir.clone(),
                env: env.into_iter().collect(),
                ..Default::default()
            },
        };
        let result = runner.update_artifacts(&input).await.unwrap();
        let results = result.expect("runner should return results");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].file.as_ref().unwrap().path, "Gemfile.lock");
    }

    // Ported: "updates the Gemfile.lock when upgrading bundler" — lib/modules/manager/bundler/artifacts.spec.ts line 699
    #[tokio::test]
    async fn updates_gemfile_lock_when_upgrading_bundler() {
        let dir = tempdir().unwrap();
        let lock_dir = dir.path().to_path_buf();

        std::fs::write(lock_dir.join("Gemfile"), "source 'https://rubygems.org'\n").unwrap();
        std::fs::write(lock_dir.join("Gemfile.lock"), "Old Gemfile.lock\n").unwrap();

        let fake_bundler = dir.path().join("bundler");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_bundler).unwrap();
            f.write_all(b"#!/bin/sh\necho 'New Gemfile.lock' > Gemfile.lock\n")
                .unwrap();
            let mut perms = std::fs::metadata(&fake_bundler).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_bundler, perms).unwrap();
        }

        let mut env = std::env::vars().collect::<HashMap<String, String>>();
        let mut path = lock_dir.to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);

        let runner = make_runner();
        let mut dep = updated_dep("bundler");
        dep.update_type = Some("patch".to_owned());
        let input = UpdateArtifact {
            package_file_name: "Gemfile".to_owned(),
            updated_deps: vec![dep],
            new_package_file_content: "source 'https://rubygems.org'\n".to_owned(),
            config: ArtifactConfig {
                lock_file_dir: lock_dir.clone(),
                env: env.into_iter().collect(),
                ..Default::default()
            },
        };
        let result = runner.update_artifacts(&input).await.unwrap();
        let results = result.expect("runner should return results");
        assert_eq!(results.len(), 1);
    }

    // Ported: "returns error when failing in lockFileMaintenance true" — lib/modules/manager/bundler/artifacts.spec.ts line 488
    #[tokio::test]
    async fn returns_error_when_failing_in_lockfile_maintenance() {
        let dir = tempdir().unwrap();
        let lock_dir = dir.path().to_path_buf();

        std::fs::write(lock_dir.join("Gemfile"), "source 'https://rubygems.org'\n").unwrap();
        std::fs::write(lock_dir.join("Gemfile.lock"), "Old Gemfile.lock\n").unwrap();

        let fake_bundler = dir.path().join("bundler");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_bundler).unwrap();
            f.write_all(b"#!/bin/sh\necho 'foo was resolved to' >&2; exit 1\n")
                .unwrap();
            let mut perms = std::fs::metadata(&fake_bundler).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_bundler, perms).unwrap();
        }

        let mut env = std::env::vars().collect::<HashMap<String, String>>();
        let mut path = lock_dir.to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);

        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "Gemfile".to_owned(),
            updated_deps: vec![],
            new_package_file_content: "{}".to_owned(),
            config: ArtifactConfig {
                lock_file_dir: lock_dir.clone(),
                env: env.into_iter().collect(),
                is_lockfile_maintenance: true,
                ..Default::default()
            },
        };
        let result = runner.update_artifacts(&input).await.unwrap();
        let results = result.expect("runner should return error results");
        assert_eq!(results.len(), 1);
        assert!(results[0].artifact_error.is_some());
    }

    // Unit tests for build_commands

    #[test]
    fn build_commands_default_update() {
        let deps = vec![updated_dep("rails")];
        let cmds = BundlerArtifactRunner::build_commands(&deps, false, &[]);
        assert_eq!(cmds, vec!["bundler lock --update 'rails'"]);
    }

    #[test]
    fn build_commands_maintenance() {
        let cmds = BundlerArtifactRunner::build_commands(&[], true, &[]);
        assert_eq!(cmds, vec!["bundler lock --update"]);
    }

    #[test]
    fn build_commands_bundler_upgrade() {
        let deps = vec![updated_dep("bundler")];
        let cmds = BundlerArtifactRunner::build_commands(&deps, false, &[]);
        assert_eq!(cmds, vec!["bundler lock --update --bundler"]);
    }

    #[test]
    fn build_commands_ruby_upgrade_only() {
        let deps = vec![updated_dep("ruby")];
        let cmds = BundlerArtifactRunner::build_commands(&deps, false, &[]);
        assert_eq!(cmds, vec!["bundler lock"]);
    }

    // Ported: "supports conservative mode and updateType option" — lib/modules/manager/bundler/artifacts.spec.ts line 176
    #[test]
    fn build_commands_conservative_and_update_type() {
        let mut foo = updated_dep("foo");
        foo.update_type = Some("minor".to_owned());
        let mut bar = updated_dep("bar");
        bar.update_type = Some("patch".to_owned());
        let deps = vec![foo, bar];
        let cmds = BundlerArtifactRunner::build_commands(
            &deps,
            false,
            &["bundlerConservative".to_owned()],
        );
        assert_eq!(cmds.len(), 2);
        assert!(
            cmds.iter()
                .any(|c| c == "bundler lock --patch --conservative --update 'bar'")
        );
        assert!(
            cmds.iter()
                .any(|c| c == "bundler lock --minor --conservative --update 'foo'")
        );
    }

    // Ported: "executes commands from lockFile path" — lib/modules/manager/bundler/artifacts.spec.ts line 100
    #[tokio::test]
    async fn executes_commands_from_lockfile_path() {
        let dir = tempdir().unwrap();
        let lock_dir = dir.path().to_path_buf();

        std::fs::create_dir(lock_dir.join("teamA")).unwrap();
        std::fs::write(
            lock_dir.join("teamA").join("Gemfile"),
            "source 'https://rubygems.org'\n",
        )
        .unwrap();
        std::fs::write(
            lock_dir.join("teamA").join("Gemfile.lock"),
            "Old Gemfile.lock\n",
        )
        .unwrap();

        let fake_bundler = dir.path().join("bundler");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_bundler).unwrap();
            f.write_all(b"#!/bin/sh\nexit 0\n").unwrap();
            let mut perms = std::fs::metadata(&fake_bundler).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_bundler, perms).unwrap();
        }

        let mut env = std::env::vars().collect::<HashMap<String, String>>();
        let mut path = lock_dir.to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);

        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "teamA/Gemfile".to_owned(),
            updated_deps: vec![updated_dep("foo"), updated_dep("bar")],
            new_package_file_content: "source 'https://rubygems.org'\n".to_owned(),
            config: ArtifactConfig {
                lock_file_dir: lock_dir.clone(),
                env: env.into_iter().collect(),
                ..Default::default()
            },
        };
        let result = runner.update_artifacts(&input).await;
        assert!(result.unwrap().is_none());
    }

    // Ported: "handles \"Could not parse object\" error" — lib/modules/manager/bundler/artifacts.spec.ts line 599
    #[tokio::test]
    async fn handles_could_not_parse_object_error() {
        let dir = tempdir().unwrap();
        let lock_dir = dir.path().to_path_buf();

        std::fs::write(lock_dir.join("Gemfile"), "source 'https://rubygems.org'\n").unwrap();
        std::fs::write(lock_dir.join("Gemfile.lock"), "Current Gemfile.lock\n").unwrap();

        let fake_bundler = dir.path().join("bundler");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_bundler).unwrap();
            f.write_all(b"#!/bin/sh\necho 'fatal: Could not parse object' >&2; exit 1\n")
                .unwrap();
            let mut perms = std::fs::metadata(&fake_bundler).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_bundler, perms).unwrap();
        }

        let mut env = std::env::vars().collect::<HashMap<String, String>>();
        let mut path = lock_dir.to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);

        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "Gemfile".to_owned(),
            updated_deps: vec![],
            new_package_file_content: "{}".to_owned(),
            config: ArtifactConfig {
                lock_file_dir: lock_dir.clone(),
                env: env.into_iter().collect(),
                is_lockfile_maintenance: true,
                ..Default::default()
            },
        };
        let result = runner.update_artifacts(&input).await.unwrap();
        let results = result.expect("runner should return error results");
        assert_eq!(results.len(), 1);
        assert!(results[0].artifact_error.is_some());
    }

    // Ported: "rethrows for temporary error" — lib/modules/manager/bundler/artifacts.spec.ts line 577
    #[tokio::test]
    async fn rethrows_for_temporary_error() {
        let dir = tempdir().unwrap();
        let lock_dir = dir.path().to_path_buf();

        std::fs::write(lock_dir.join("Gemfile"), "source 'https://rubygems.org'\n").unwrap();
        std::fs::write(lock_dir.join("Gemfile.lock"), "Old Gemfile.lock\n").unwrap();

        let fake_bundler = dir.path().join("bundler");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_bundler).unwrap();
            f.write_all(b"#!/bin/sh\necho 'temporary-error' >&2; exit 1\n")
                .unwrap();
            let mut perms = std::fs::metadata(&fake_bundler).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_bundler, perms).unwrap();
        }

        let mut env = std::env::vars().collect::<HashMap<String, String>>();
        let mut path = lock_dir.to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);

        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "Gemfile".to_owned(),
            updated_deps: vec![],
            new_package_file_content: "{}".to_owned(),
            config: ArtifactConfig {
                lock_file_dir: lock_dir.clone(),
                env: env.into_iter().collect(),
                is_lockfile_maintenance: true,
                ..Default::default()
            },
        };
        let result = runner.update_artifacts(&input).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.stderr.contains("temporary-error"));
    }
}
