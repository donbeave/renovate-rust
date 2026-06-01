//! Pip requirements artifact runner — hash regeneration after requirements.txt edits.
//!
//! Ports `lib/modules/manager/pip_requirements/artifacts.ts`.
//!
//! After the requirements file is updated with new version pins, this runner:
//! 1. Scans the updated content for dependencies that have `--hash=` entries.
//! 2. Runs `hashin <constraint> -r <file>` for each such dependency.
//! 3. Reads the updated requirements file.
//! 4. Returns any changed files.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use crate::artifacts::{ArtifactError, ArtifactResult, ArtifactRunner, UpdateArtifact};
use crate::exec::raw::raw_exec;
use crate::exec::types::ExecOptions;

/// Artifact runner for pip requirements files.
#[derive(Debug, Clone, Default)]
pub struct PipArtifactRunner;

impl PipArtifactRunner {
    /// Create a new pip artifact runner.
    pub fn new() -> Self {
        Self
    }

    /// Build a regex that matches a dependency constraint followed by `--hash=`.
    ///
    /// The regex captures the constraint (including the dependency name, optional
    /// extras, and `==<version>`) in a named group `depConstraint`.
    fn dependency_and_hash_pattern(dep_name: &str) -> regex::Regex {
        let escaped = regex::escape(dep_name);
        // extrasPattern from upstream: (?:\s*\[[^\]]+\])?
        let pattern = format!(
            r"^\s*(?P<depConstraint>{escaped}(?:\s*\[[^\]]+\])?\s*==.*?\S)\s+--hash=",
        );
        regex::Regex::new(&pattern).expect("valid regex")
    }

    /// Build the `hashin` commands to run for the given updated deps.
    ///
    /// Returns `None` if no commands are needed (e.g. no deps have hashes).
    fn build_commands(
        package_file_name: &str,
        updated_deps: &[crate::artifacts::UpdatedDep],
        new_package_file_content: &str,
    ) -> Option<Vec<String>> {
        // Strip backslash-newline continuations (upstream does this).
        let rewritten = new_package_file_content.replace("\\\n", "");

        let mut cmds = Vec::new();
        for dep in updated_deps {
            if dep.dep_name.is_empty() {
                continue;
            }
            let re = Self::dependency_and_hash_pattern(&dep.dep_name);
            if let Some(cap) = re.captures(&rewritten) {
                let constraint = cap
                    .name("depConstraint")
                    .map(|m| m.as_str())
                    .unwrap_or("");
                if !constraint.is_empty() {
                    let quoted_constraint = crate::util::shlex_quote(constraint);
                    let quoted_file = crate::util::shlex_quote(package_file_name);
                    cmds.push(format!(
                        "hashin {quoted_constraint} -r {quoted_file}"
                    ));
                }
            }
        }

        if cmds.is_empty() {
            None
        } else {
            Some(cmds)
        }
    }

    /// Run a single hashin command.
    async fn run_command(
        &self,
        cmd: &str,
        cwd: &std::path::Path,
        env: &HashMap<String, String>,
    ) -> Result<(), ArtifactError> {
        let opts = ExecOptions {
            cwd: Some(cwd.to_string_lossy().to_string()),
            timeout: Some(120_000), // 2 minutes
            ..Default::default()
        };
        raw_exec(cmd, &opts, env)
            .await
            .map_err(|e| ArtifactError {
                lock_file: "requirements.txt".to_owned(),
                stderr: e.message,
            })
            .map(|_| ())
    }
}

impl ArtifactRunner for PipArtifactRunner {
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
            // 1. No deps to update → nothing to do.
            if updated_deps.is_empty() {
                return Ok(None);
            }

            let lock_file_dir = &config.lock_file_dir;
            let file_path = lock_file_dir.join(&package_file_name);
            let file_dir = file_path
                .parent()
                .map(std::path::Path::to_path_buf)
                .unwrap_or_else(|| lock_file_dir.clone());

            // 2. Build hashin commands.
            let Some(cmds) = Self::build_commands(
                &package_file_name,
                &updated_deps,
                &new_package_file_content,
            ) else {
                return Ok(None);
            };

            // 3. Run commands.
            let mut env = if config.env.is_empty() {
                std::env::vars().collect::<HashMap<String, String>>()
            } else {
                config.env.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
            };

            // Set PIP_CACHE_DIR if not already set.
            if !env.contains_key("PIP_CACHE_DIR") {
                env.insert(
                    "PIP_CACHE_DIR".to_owned(),
                    std::env::temp_dir().to_string_lossy().to_string(),
                );
            }

            for cmd in &cmds {
                if let Err(err) = self.run_command(cmd, &file_dir, &env).await {
                    // Catch the error and return it as an artifact error result
                    // (matching upstream behavior, except TEMPORARY_ERROR which we
                    // don't distinguish at this layer).
                    return Ok(Some(vec![ArtifactResult::error(
                        &package_file_name,
                        err.stderr,
                    )]));
                }
            }

            // 4. Read updated file.
            let new_content = match tokio::fs::read_to_string(&file_path).await {
                Ok(c) => c,
                Err(e) => {
                    return Ok(Some(vec![ArtifactResult::error(
                        &package_file_name,
                        format!("failed to read updated file: {e}"),
                    )]));
                }
            };

            // 5. If unchanged, return null.
            if new_content == new_package_file_content {
                return Ok(None);
            }

            Ok(Some(vec![ArtifactResult::file_change(
                package_file_name,
                new_content,
            )]))
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

    fn make_runner() -> PipArtifactRunner {
        PipArtifactRunner::new()
    }

    fn updated_dep(dep_name: &str) -> UpdatedDep {
        UpdatedDep {
            dep_name: dep_name.to_owned(),
            package_name: None,
            current_value: None,
            new_value: None,
            locked_version: None,
            new_version: None,
            package_file: "requirements.txt".to_owned(),
            manager: "pip_requirements".to_owned(),
            datasource: None,
            update_type: None,
        }
    }

    const NEW_PACKAGE_FILE_CONTENT: &str = "atomicwrites==1.4.0 \\\n\
  --hash=sha256:03472c30eb2c5d1ba9227e4c2ca66ab8287fbfbbda3888aa93dc2e28fc6811b4 \\\n\
  --hash=sha256:75a9445bac02d8d058d5e1fe689654ba5a6556a1dfd8ce6ec55a0ed79866cfa6\n\
boto3-stubs[iam] == 1.24.36.post1 \\n\
--hash=sha256:39acbbc8c87a101bdf46e058fbb012d044b773b43f7ed02cc4c24192a564411e \\n\
--hash=sha256:ca3b3066773fc727fea0dbec252d098098e45fe0def011b22036ef674344def2\n\
botocore==1.27.46 \\n\
--hash=sha256:747b7e94aef41498f063fc0be79c5af102d940beea713965179e1ead89c7e9ec \\n\
--hash=sha256:f66d8305d1f59d83334df9b11b6512bb1e14698ec4d5d6d42f833f39f3304ca7\n";

    // Ported: "returns null if no updatedDeps were provided" — pip_requirements/artifacts.spec.ts line 51
    #[tokio::test]
    async fn returns_null_if_no_updated_deps() {
        let dir = tempdir().unwrap();
        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "requirements.txt".to_owned(),
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

    // Ported: "returns null if no hashes" — pip_requirements/artifacts.spec.ts line 62
    #[tokio::test]
    async fn returns_null_if_no_hashes() {
        let dir = tempdir().unwrap();
        std::fs::write(
            dir.path().join("requirements.txt"),
            "eventlet==0.30.2\npbr>=1.9\n",
        )
        .unwrap();

        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "requirements.txt".to_owned(),
            updated_deps: vec![updated_dep("eventlet")],
            new_package_file_content: "eventlet==0.30.2\npbr>=1.9\n".to_owned(),
            config: ArtifactConfig {
                lock_file_dir: dir.path().to_path_buf(),
                ..Default::default()
            },
        };
        let result = runner.update_artifacts(&input).await;
        assert!(result.unwrap().is_none());
    }

    // Ported: "returns null if unchanged" — pip_requirements/artifacts.spec.ts line 74
    #[tokio::test]
    async fn returns_null_if_unchanged() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("requirements.txt"), NEW_PACKAGE_FILE_CONTENT).unwrap();

        // Fake hashin binary that doesn't change the file.
        let fake_hashin = dir.path().join("hashin");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_hashin).unwrap();
            f.write_all(b"#!/bin/sh\nexit 0\n").unwrap();
            let mut perms = std::fs::metadata(&fake_hashin).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_hashin, perms).unwrap();
        }

        let mut env = std::env::vars().collect::<HashMap<String, String>>();
        let mut path = dir.path().to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);

        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "requirements.txt".to_owned(),
            updated_deps: vec![updated_dep("atomicwrites"), updated_dep("boto3-stubs")],
            new_package_file_content: NEW_PACKAGE_FILE_CONTENT.to_owned(),
            config: ArtifactConfig {
                lock_file_dir: dir.path().to_path_buf(),
                env: env.into_iter().collect(),
                ..Default::default()
            },
        };
        let result = runner.update_artifacts(&input).await;
        assert!(result.unwrap().is_none());
    }

    // Ported: "returns updated file" — pip_requirements/artifacts.spec.ts line 98
    #[tokio::test]
    async fn returns_updated_file() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("requirements.txt"), NEW_PACKAGE_FILE_CONTENT).unwrap();

        let fake_hashin = dir.path().join("hashin");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_hashin).unwrap();
            f.write_all(b"#!/bin/sh\necho 'new content' > requirements.txt\n").unwrap();
            let mut perms = std::fs::metadata(&fake_hashin).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_hashin, perms).unwrap();
        }
        #[cfg(windows)]
        {
            let mut f = std::fs::File::create(&fake_hashin).unwrap();
            f.write_all(b"@echo off\necho new content > requirements.txt\n")
                .unwrap();
        }

        let mut env = std::env::vars().collect::<HashMap<String, String>>();
        let mut path = dir.path().to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);

        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "requirements.txt".to_owned(),
            updated_deps: vec![updated_dep("atomicwrites"), updated_dep("boto3-stubs")],
            new_package_file_content: NEW_PACKAGE_FILE_CONTENT.to_owned(),
            config: ArtifactConfig {
                lock_file_dir: dir.path().to_path_buf(),
                env: env.into_iter().collect(),
                ..Default::default()
            },
        };
        let result = runner.update_artifacts(&input).await.unwrap();
        let results = result.expect("runner should return results");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].file.as_ref().unwrap().path, "requirements.txt");
        assert_eq!(
            results[0].file.as_ref().unwrap().contents.as_deref(),
            Some("new content\n")
        );
    }

    // Ported: "ignores falsy depNames" — pip_requirements/artifacts.spec.ts line 130
    #[tokio::test]
    async fn ignores_falsy_dep_names() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("requirements.txt"), NEW_PACKAGE_FILE_CONTENT).unwrap();

        let fake_hashin = dir.path().join("hashin");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_hashin).unwrap();
            f.write_all(b"#!/bin/sh\necho 'new content' > requirements.txt\n").unwrap();
            let mut perms = std::fs::metadata(&fake_hashin).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_hashin, perms).unwrap();
        }

        let mut env = std::env::vars().collect::<HashMap<String, String>>();
        let mut path = dir.path().to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);

        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "requirements.txt".to_owned(),
            updated_deps: vec![
                UpdatedDep {
                    dep_name: "".to_owned(),
                    ..updated_dep("")
                },
                updated_dep("atomicwrites"),
                UpdatedDep {
                    dep_name: "".to_owned(),
                    ..updated_dep("")
                },
            ],
            new_package_file_content: NEW_PACKAGE_FILE_CONTENT.to_owned(),
            config: ArtifactConfig {
                lock_file_dir: dir.path().to_path_buf(),
                env: env.into_iter().collect(),
                ..Default::default()
            },
        };
        let result = runner.update_artifacts(&input).await.unwrap();
        let results = result.expect("runner should return results");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].file.as_ref().unwrap().path, "requirements.txt");
    }

    // Ported: "catches and returns errors" — pip_requirements/artifacts.spec.ts line 162
    #[tokio::test]
    async fn catches_and_returns_errors() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("requirements.txt"), NEW_PACKAGE_FILE_CONTENT).unwrap();

        let fake_hashin = dir.path().join("hashin");
        #[cfg(unix)]
        {
            let mut f = std::fs::File::create(&fake_hashin).unwrap();
            f.write_all(b"#!/bin/sh\necho 'hashin error' >&2; exit 1\n").unwrap();
            let mut perms = std::fs::metadata(&fake_hashin).unwrap().permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&fake_hashin, perms).unwrap();
        }

        let mut env = std::env::vars().collect::<HashMap<String, String>>();
        let mut path = dir.path().to_string_lossy().to_string();
        if let Some(old_path) = env.get("PATH") {
            path = format!("{}:{}", path, old_path);
        }
        env.insert("PATH".to_owned(), path);

        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "requirements.txt".to_owned(),
            updated_deps: vec![updated_dep("atomicwrites")],
            new_package_file_content: NEW_PACKAGE_FILE_CONTENT.to_owned(),
            config: ArtifactConfig {
                lock_file_dir: dir.path().to_path_buf(),
                env: env.into_iter().collect(),
                ..Default::default()
            },
        };
        let result = runner.update_artifacts(&input).await.unwrap();
        let results = result.expect("runner should return error results");
        assert_eq!(results.len(), 1);
        assert!(results[0].artifact_error.is_some());
        assert_eq!(
            results[0].artifact_error.as_ref().unwrap().lock_file,
            "requirements.txt"
        );
    }
}
