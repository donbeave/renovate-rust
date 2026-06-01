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

use crate::artifacts::{ArtifactNotice, ArtifactResult, ArtifactRunner, UpdateArtifact};
use crate::exec::raw::raw_exec;
use crate::exec::types::ExecOptions;
use crate::extractors::gomod::get_extra_deps_notice;

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
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<Option<Vec<ArtifactResult>>, crate::artifacts::ArtifactError>,
                > + Send
                + '_,
        >,
    > {
        let lock_dir = input.config.lock_file_dir.clone();
        let package_file_name = input.package_file_name.clone();
        let new_package_file_content = input.new_package_file_content.clone();
        let config = input.config.clone();
        let updated_deps = input.updated_deps.clone();

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
            let Ok(existing_go_sum) = tokio::fs::read_to_string(&go_sum_path).await else {
                return Ok(None);
            };

            // Write updated go.mod.
            let go_mod_path = package_dir.join("go.mod");
            if let Err(e) = tokio::fs::write(&go_mod_path, &new_package_file_content).await {
                return Err(crate::artifacts::ArtifactError {
                    lock_file: "go.mod".to_owned(),
                    stderr: format!("failed to write go.mod: {}", e),
                });
            }

            // Build env for go commands.
            let mut env = std::env::vars().collect::<HashMap<String, String>>();
            for (k, v) in &config.env {
                env.insert(k.clone(), v.clone());
            }

            let opts = ExecOptions {
                cwd: Some(package_dir.to_string_lossy().to_string()),
                timeout: Some(300_000), // 5 minutes
                ..Default::default()
            };

            // Determine -modfile flag (used by go get and go mod tidy).
            let modfile_flag = if package_file_name == "go.mod" {
                "".to_owned()
            } else {
                format!(" -modfile={}", package_file_name)
            };

            // Build the `go get` target directories.
            let get_target = if config.go_get_dirs.is_empty() {
                "./...".to_owned()
            } else {
                let valid: Vec<&str> = config
                    .go_get_dirs
                    .iter()
                    .filter(|d| is_valid_go_get_dir(d))
                    .map(|d| d.as_str())
                    .collect();
                if valid.is_empty() {
                    return Err(crate::artifacts::ArtifactError {
                        lock_file: sum_file_name.clone(),
                        stderr: "Invalid goGetDirs".to_owned(),
                    });
                }
                valid.join(" ")
            };

            // Run `go get` to update dependencies.
            let get_cmd = format!("go get{} -d -t {}", modfile_flag, get_target);
            match raw_exec(&get_cmd, &opts, &env).await {
                Ok(_) => {}
                Err(e) => {
                    return Err(crate::artifacts::ArtifactError {
                        lock_file: sum_file_name.clone(),
                        stderr: format!("go get failed: {}", e.message),
                    });
                }
            }

            // Run `go mod tidy` when explicitly requested or when import path
            // updates are required on a major update.
            // Skip tidy on major updates unless gomodUpdateImportPaths is set
            // (mirrors upstream mustSkipGoModTidy logic).
            let is_major = config.update_type.as_deref() == Some("major");
            let has_import_paths = config
                .post_update_options
                .contains(&"gomodUpdateImportPaths".to_owned());
            let skip_tidy = is_major && !has_import_paths;
            let is_import_path_update_required = has_import_paths && is_major;
            let run_tidy = !skip_tidy
                && (config.post_update_options.contains(&"gomodTidy".to_owned())
                    || config
                        .post_update_options
                        .contains(&"gomodTidy1.17".to_owned())
                    || config
                        .post_update_options
                        .contains(&"gomodTidyE".to_owned())
                    || is_import_path_update_required);
            let tidy_cmd = format!("go mod tidy{}", modfile_flag);
            if run_tidy {
                match raw_exec(&tidy_cmd, &opts, &env).await {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(crate::artifacts::ArtifactError {
                            lock_file: sum_file_name.clone(),
                            stderr: format!("go mod tidy failed: {}", e.message),
                        });
                    }
                }
            }

            // Run `go mod vendor` when explicitly requested or when a
            // vendor directory with modules.txt exists and gomodSkipVendor
            // is not requested.
            let vendor_path = package_dir.join("vendor");
            let vendor_modules_txt = vendor_path.join("modules.txt");
            let explicit_vendor = config
                .post_update_options
                .contains(&"gomodVendor".to_owned());
            let skip_vendor = config
                .post_update_options
                .contains(&"gomodSkipVendor".to_owned());
            let use_vendor = explicit_vendor
                || (!skip_vendor && vendor_path.exists() && vendor_modules_txt.exists());
            if use_vendor {
                let vendor_cmd = format!("go mod vendor{}", modfile_flag);
                match raw_exec(&vendor_cmd, &opts, &env).await {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(crate::artifacts::ArtifactError {
                            lock_file: sum_file_name.clone(),
                            stderr: format!("go mod vendor failed: {}", e.message),
                        });
                    }
                }
                // Tidy again after vendor (mirrors upstream).
                if run_tidy {
                    match raw_exec(&tidy_cmd, &opts, &env).await {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(crate::artifacts::ArtifactError {
                                lock_file: sum_file_name.clone(),
                                stderr: format!("go mod tidy failed: {}", e.message),
                            });
                        }
                    }
                }
            }

            // Run `go generate` when requested and permitted.
            let use_go_generate = config
                .post_update_options
                .contains(&"goGenerate".to_owned());
            let go_generate_allowed = config
                .allowed_unsafe_executions
                .contains(&"goGenerate".to_owned());
            if use_go_generate && go_generate_allowed {
                let gen_cmd = "go generate ./...";
                match raw_exec(gen_cmd, &opts, &env).await {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(crate::artifacts::ArtifactError {
                            lock_file: sum_file_name.clone(),
                            stderr: format!("go generate failed: {}", e.message),
                        });
                    }
                }
            }

            // Tidy one more time as a solution for upstream issue #6795.
            if run_tidy {
                match raw_exec(&tidy_cmd, &opts, &env).await {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(crate::artifacts::ArtifactError {
                            lock_file: sum_file_name.clone(),
                            stderr: format!("go mod tidy failed: {}", e.message),
                        });
                    }
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
                let dep_names: Vec<&str> =
                    updated_deps.iter().map(|d| d.dep_name.as_str()).collect();
                let notice = get_extra_deps_notice(
                    Some(&new_package_file_content),
                    Some(&updated_go_mod),
                    &dep_names,
                )
                .map(|msg| ArtifactNotice {
                    file: package_file_name.clone(),
                    message: msg,
                });
                results.push(ArtifactResult {
                    file: Some(crate::artifacts::FileChange::addition(
                        package_file_name.clone(),
                        updated_go_mod,
                    )),
                    artifact_error: None,
                    notice,
                });
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

/// Validate a `goGetDirs` entry.
///
/// Rejects paths that contain directory-traversal components.
fn is_valid_go_get_dir(dir: &str) -> bool {
    !dir.contains("..") && !dir.starts_with('/')
}

/// Derive the Go toolchain constraint from config and go.mod content.
///
/// Precedence:
/// 1. `config.constraints.go`
/// 2. `toolchain goX.Y.Z` directive in go.mod
/// 3. `go X.Y.Z` full version directive in go.mod
/// 4. `go X.Y` → returns `^X.Y`
///
/// Mirrors `deriveGoToolchainConstraints` from
/// `lib/modules/manager/gomod/artifacts.ts`.
pub fn derive_go_toolchain_constraints(
    config_constraint: Option<&str>,
    go_mod_content: &str,
) -> Option<String> {
    if let Some(c) = config_constraint {
        return Some(c.to_owned());
    }

    // Prefer toolchain directive.
    for line in go_mod_content.lines() {
        let trimmed = line.trim();
        if let Some(v) = trimmed.strip_prefix("toolchain go") {
            return Some(v.trim().to_owned());
        }
    }

    // Full go directive (e.g. go 1.23.5).
    for line in go_mod_content.lines() {
        let trimmed = line.trim();
        if let Some(v) = trimmed.strip_prefix("go ") {
            let v = v.trim();
            if v.matches('.').count() == 2 {
                return Some(v.to_owned());
            }
        }
    }

    // Major.minor go directive (e.g. go 1.17) → semver range.
    for line in go_mod_content.lines() {
        let trimmed = line.trim();
        if let Some(v) = trimmed.strip_prefix("go ") {
            let v = v.trim();
            if v.matches('.').count() == 1 {
                return Some(format!("^{v}"));
            }
        }
    }

    None
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
                update_type: None,
            }],
            new_package_file_content: go_mod.to_owned(),
            config: ArtifactConfig {
                lock_file_dir: lock_dir,
                ..Default::default()
            },
        }
    }

    fn make_fake_go(
        dir: &tempfile::TempDir,
        script: &[u8],
    ) -> std::collections::BTreeMap<String, String> {
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
        let mut input = make_input(
            &dir,
            "module example.com/test\n\ngo 1.22\n",
            Some("old sum\n"),
        );
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
        let mut input = make_input(
            &dir,
            "module example.com/test\n\ngo 1.22\n",
            Some("old sum\n"),
        );
        input
            .config
            .post_update_options
            .push("gomodTidy".to_owned());
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
        let go_mod_input =
            "module example.com/test\n\ngo 1.22\n\nrequire github.com/foo/bar v1.1.0\n";
        let mut input = make_input(&dir, go_mod_input, Some("old sum\n"));
        input
            .config
            .post_update_options
            .push("gomodTidy".to_owned());
        // Write a reference go.mod with extra trailing newline.
        let ref_go_mod = dir.path().join("ref_go.mod");
        std::fs::write(
            &ref_go_mod,
            "module example.com/test\n\ngo 1.22\n\nrequire github.com/foo/bar v1.1.0\n\n",
        )
        .unwrap();
        // fake go that copies the reference go.mod and updates go.sum
        let script = format!(
            "if [ \"$1\" = \"mod\" ] && [ \"$2\" = \"tidy\" ]; then cp '{}' go.mod; echo 'new sum' > go.sum; fi\n",
            ref_go_mod.to_string_lossy()
        );
        input.config.env = make_fake_go(&dir, script.as_bytes());
        let result = runner.update_artifacts(&input).await.unwrap().unwrap();
        assert_eq!(result.len(), 2);
        let paths: Vec<_> = result
            .iter()
            .map(|r| r.file.as_ref().unwrap().path.clone())
            .collect();
        assert!(paths.contains(&"go.mod".to_owned()));
        assert!(paths.contains(&"go.sum".to_owned()));
    }

    #[tokio::test]
    async fn gomod_artifact_runner_returns_error_when_go_missing() {
        let dir = tempfile::tempdir().unwrap();
        let input = make_input(
            &dir,
            "module example.com/test\n\ngo 1.22\n",
            Some("old sum\n"),
        );
        let runner = GomodArtifactRunner::new();
        let result = runner.update_artifacts(&input).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.stderr.contains("go get failed"));
    }

    #[test]
    fn gomod_artifact_runner_new_and_default() {
        let r1 = GomodArtifactRunner::new();
        let r2 = GomodArtifactRunner;
        assert_eq!(format!("{:?}", r1), format!("{:?}", r2));
    }

    fn make_input_with_name(
        dir: &tempfile::TempDir,
        package_file_name: &str,
        go_mod: &str,
        go_sum: Option<&str>,
    ) -> UpdateArtifact {
        let lock_dir = dir.path().to_path_buf();
        let sum_file = if package_file_name.ends_with(".mod") {
            format!("{}sum", &package_file_name[..package_file_name.len() - 3])
        } else {
            "go.sum".to_owned()
        };
        if let Some(sum) = go_sum {
            std::fs::write(lock_dir.join(&sum_file), sum).unwrap();
        }
        UpdateArtifact {
            package_file_name: package_file_name.to_owned(),
            updated_deps: vec![UpdatedDep {
                dep_name: "github.com/foo/bar".to_owned(),
                package_name: None,
                current_value: Some("v1.0.0".to_owned()),
                new_value: Some("v1.1.0".to_owned()),
                locked_version: None,
                new_version: None,
                package_file: package_file_name.to_owned(),
                manager: "gomod".to_owned(),
                datasource: None,
                update_type: None,
            }],
            new_package_file_content: go_mod.to_owned(),
            config: ArtifactConfig {
                lock_file_dir: lock_dir,
                ..Default::default()
            },
        }
    }

    // Ported: "uses -modfile flag for non-default go.mod filename" — gomod/artifacts.spec.ts line 2698
    #[tokio::test]
    async fn uses_modfile_for_non_default_go_mod() {
        let dir = tempfile::tempdir().unwrap();
        let runner = GomodArtifactRunner::new();
        let mut input = make_input_with_name(
            &dir,
            "tools.mod",
            "module example.com/test\n\ngo 1.22\n",
            Some("old sum\n"),
        );
        input
            .config
            .post_update_options
            .push("gomodTidy".to_owned());
        // fake go that verifies -modfile and updates tools.sum
        input.config.env = make_fake_go(
            &dir,
            b"if [ \"$1\" = \"get\" ]; then
                case \"$*\" in *-modfile=tools.mod*) ;; *) exit 1 ;; esac
              fi
              if [ \"$1\" = \"mod\" ] && [ \"$2\" = \"tidy\" ]; then
                case \"$*\" in *-modfile=tools.mod*) ;; *) exit 1 ;; esac
                echo 'updated sum' > tools.sum
              fi
            ",
        );
        let result = runner.update_artifacts(&input).await.unwrap().unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].file.as_ref().unwrap().path, "tools.sum");
    }

    // Ported: "uses -modfile flag with go mod tidy for non-default go.mod filename" — gomod/artifacts.spec.ts line 2733
    #[tokio::test]
    async fn uses_modfile_with_go_mod_tidy() {
        let dir = tempfile::tempdir().unwrap();
        let runner = GomodArtifactRunner::new();
        let mut input = make_input_with_name(
            &dir,
            "tools.mod",
            "module example.com/test\n\ngo 1.22\n",
            Some("old sum\n"),
        );
        input
            .config
            .post_update_options
            .push("gomodTidy".to_owned());
        input.config.env = make_fake_go(
            &dir,
            b"if [ \"$1\" = \"get\" ]; then
                case \"$*\" in *-modfile=tools.mod*) ;; *) exit 1 ;; esac
              fi
              if [ \"$1\" = \"mod\" ] && [ \"$2\" = \"tidy\" ]; then
                case \"$*\" in *-modfile=tools.mod*) ;; *) exit 1 ;; esac
                echo 'updated sum' > tools.sum
              fi
            ",
        );
        let result = runner.update_artifacts(&input).await.unwrap().unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].file.as_ref().unwrap().path, "tools.sum");
    }

    // Ported: "runs go mod vendor with gomodVendor" — gomod/artifacts.spec.ts line 192
    #[tokio::test]
    async fn runs_go_mod_vendor_with_gomod_vendor() {
        let dir = tempfile::tempdir().unwrap();
        let runner = GomodArtifactRunner::new();
        let mut input = make_input(
            &dir,
            "module example.com/test\n\ngo 1.22\n",
            Some("old sum\n"),
        );
        input
            .config
            .post_update_options
            .push("gomodVendor".to_owned());
        input.config.env = make_fake_go(
            &dir,
            b"if [ \"$1\" = \"mod\" ] && [ \"$2\" = \"vendor\" ]; then
                echo 'new sum' > go.sum
                echo 'new mod' > go.mod
              fi
            ",
        );
        let result = runner.update_artifacts(&input).await.unwrap().unwrap();
        assert_eq!(result.len(), 2);
        let paths: Vec<_> = result
            .iter()
            .map(|r| r.file.as_ref().unwrap().path.clone())
            .collect();
        assert!(paths.contains(&"go.sum".to_owned()));
        assert!(paths.contains(&"go.mod".to_owned()));
    }

    // Ported: "skips vendor directory update with gomodSkipVendor" — gomod/artifacts.spec.ts line 390
    #[tokio::test]
    async fn skips_vendor_with_gomod_skip_vendor() {
        let dir = tempfile::tempdir().unwrap();
        let runner = GomodArtifactRunner::new();
        let mut input = make_input(
            &dir,
            "module example.com/test\n\ngo 1.22\n",
            Some("old sum\n"),
        );
        input
            .config
            .post_update_options
            .push("gomodSkipVendor".to_owned());
        // Create vendor directory with modules.txt to trigger implicit vendor
        std::fs::create_dir(dir.path().join("vendor")).unwrap();
        std::fs::write(dir.path().join("vendor/modules.txt"), "txt").unwrap();
        input.config.env = make_fake_go(
            &dir,
            b"if [ \"$1\" = \"get\" ]; then
                echo 'new sum' > go.sum
                echo 'new mod' > go.mod
              fi
            ",
        );
        let result = runner.update_artifacts(&input).await.unwrap().unwrap();
        assert_eq!(result.len(), 2);
        let paths: Vec<_> = result
            .iter()
            .map(|r| r.file.as_ref().unwrap().path.clone())
            .collect();
        assert!(paths.contains(&"go.sum".to_owned()));
        assert!(paths.contains(&"go.mod".to_owned()));
    }

    // Ported: "supports go generate when configured" — gomod/artifacts.spec.ts line 647
    #[tokio::test]
    async fn supports_go_generate_when_configured_and_allowed() {
        let dir = tempfile::tempdir().unwrap();
        let runner = GomodArtifactRunner::new();
        let mut input = make_input(
            &dir,
            "module example.com/test\n\ngo 1.22\n",
            Some("old sum\n"),
        );
        input
            .config
            .post_update_options
            .push("goGenerate".to_owned());
        input
            .config
            .allowed_unsafe_executions
            .push("goGenerate".to_owned());
        input
            .config
            .post_update_options
            .push("gomodTidy".to_owned());
        input.config.env = make_fake_go(
            &dir,
            b"if [ \"$1\" = \"generate\" ]; then
                echo 'new sum' > go.sum
              fi
              if [ \"$1\" = \"mod\" ] && [ \"$2\" = \"tidy\" ]; then
                echo 'new mod' > go.mod
              fi
            ",
        );
        let result = runner.update_artifacts(&input).await.unwrap().unwrap();
        assert_eq!(result.len(), 2);
        let paths: Vec<_> = result
            .iter()
            .map(|r| r.file.as_ref().unwrap().path.clone())
            .collect();
        assert!(paths.contains(&"go.sum".to_owned()));
        assert!(paths.contains(&"go.mod".to_owned()));
    }

    // Ported: "only allows go generate usage when permitted globally" — gomod/artifacts.spec.ts line 735
    #[tokio::test]
    async fn skips_go_generate_when_not_permitted() {
        let dir = tempfile::tempdir().unwrap();
        let runner = GomodArtifactRunner::new();
        let mut input = make_input(
            &dir,
            "module example.com/test\n\ngo 1.22\n",
            Some("old sum\n"),
        );
        input
            .config
            .post_update_options
            .push("goGenerate".to_owned());
        // allowed_unsafe_executions is empty, so go generate should be skipped
        input.config.env = make_fake_go(
            &dir,
            b"if [ \"$1\" = \"generate\" ]; then exit 1; fi
              if [ \"$1\" = \"get\" ]; then
                echo 'new sum' > go.sum
                echo 'new mod' > go.mod
              fi
            ",
        );
        let result = runner.update_artifacts(&input).await.unwrap().unwrap();
        assert_eq!(result.len(), 2);
        let paths: Vec<_> = result
            .iter()
            .map(|r| r.file.as_ref().unwrap().path.clone())
            .collect();
        assert!(paths.contains(&"go.sum".to_owned()));
        assert!(paths.contains(&"go.mod".to_owned()));
    }

    // Ported: "handles goGetDirs configuration correctly" — gomod/artifacts.spec.ts line 2582
    #[tokio::test]
    async fn handles_go_get_dirs_with_invalid_paths() {
        let dir = tempfile::tempdir().unwrap();
        let runner = GomodArtifactRunner::new();
        let mut input = make_input(
            &dir,
            "module example.com/test\n\ngo 1.22\n",
            Some("old sum\n"),
        );
        input.config.go_get_dirs = vec![
            ".".to_owned(),
            "foo".to_owned(),
            ".bar/...".to_owned(),
            "&&".to_owned(),
            "cat".to_owned(),
            "/etc/passwd".to_owned(),
        ];
        // fake go that fails if it receives an invalid path
        input.config.env = make_fake_go(
            &dir,
            b"case \"$*\" in *'/etc/passwd'*) exit 1 ;; esac
              case \"$*\" in *'..'*) exit 1 ;; esac
            ",
        );
        let result = runner.update_artifacts(&input).await.unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns updated go.sum when goGetDirs is specified" — gomod/artifacts.spec.ts line 2613
    #[tokio::test]
    async fn returns_updated_go_sum_with_go_get_dirs() {
        let dir = tempfile::tempdir().unwrap();
        let runner = GomodArtifactRunner::new();
        let mut input = make_input(
            &dir,
            "module example.com/test\n\ngo 1.22\n",
            Some("old sum\n"),
        );
        input.config.go_get_dirs = vec![".".to_owned()];
        input
            .config
            .post_update_options
            .push("gomodTidy".to_owned());
        input.config.env = make_fake_go(
            &dir,
            b"if [ \"$1\" = \"mod\" ] && [ \"$2\" = \"tidy\" ]; then echo 'updated sum' > go.sum; fi\n",
        );
        let result = runner.update_artifacts(&input).await.unwrap().unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].file.as_ref().unwrap().path, "go.sum");
    }

    // Ported: "errors when goGetDirs is specified with all invalid paths" — gomod/artifacts.spec.ts line 2654
    #[tokio::test]
    async fn errors_when_all_go_get_dirs_invalid() {
        let dir = tempfile::tempdir().unwrap();
        let runner = GomodArtifactRunner::new();
        let mut input = make_input(
            &dir,
            "module example.com/test\n\ngo 1.22\n",
            Some("old sum\n"),
        );
        input.config.go_get_dirs = vec!["/etc".to_owned(), "../../../".to_owned()];
        let result = runner.update_artifacts(&input).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.stderr.contains("Invalid goGetDirs"));
    }

    // Ported: "skips gomodTidy without gomodUpdateImportPaths on major update" — gomod/artifacts.spec.ts line 1998
    #[tokio::test]
    async fn skips_go_mod_tidy_on_major_without_import_paths() {
        let dir = tempfile::tempdir().unwrap();
        let runner = GomodArtifactRunner::new();
        let mut input = make_input(
            &dir,
            "module example.com/test\n\ngo 1.22\n",
            Some("old sum\n"),
        );
        input
            .config
            .post_update_options
            .push("gomodTidy".to_owned());
        input.config.update_type = Some("major".to_owned());
        // Create vendor dir but no modules.txt, so vendor is not triggered
        std::fs::create_dir(dir.path().join("vendor")).unwrap();
        input.config.env = make_fake_go(
            &dir,
            b"if [ \"$1\" = \"get\" ]; then
                echo 'new sum' > go.sum
                echo 'new mod' > go.mod
              fi
              if [ \"$1\" = \"mod\" ]; then exit 1; fi
            ",
        );
        let result = runner.update_artifacts(&input).await.unwrap().unwrap();
        assert_eq!(result.len(), 2);
        let paths: Vec<_> = result
            .iter()
            .map(|r| r.file.as_ref().unwrap().path.clone())
            .collect();
        assert!(paths.contains(&"go.sum".to_owned()));
        assert!(paths.contains(&"go.mod".to_owned()));
    }

    // Ported: "does not execute go mod tidy when none of gomodTidy and gomodUpdateImportPaths are set" — gomod/artifacts.spec.ts line 2036
    #[tokio::test]
    async fn skips_go_mod_tidy_without_gomod_tidy_or_import_paths() {
        let dir = tempfile::tempdir().unwrap();
        let runner = GomodArtifactRunner::new();
        let mut input = make_input(
            &dir,
            "module example.com/test\n\ngo 1.22\n",
            Some("old sum\n"),
        );
        // Create vendor dir but no modules.txt, so vendor is not triggered
        std::fs::create_dir(dir.path().join("vendor")).unwrap();
        input.config.env = make_fake_go(
            &dir,
            b"if [ \"$1\" = \"get\" ]; then
                echo 'new sum' > go.sum
                echo 'new mod' > go.mod
              fi
              if [ \"$1\" = \"mod\" ]; then exit 1; fi
            ",
        );
        let result = runner.update_artifacts(&input).await.unwrap().unwrap();
        assert_eq!(result.len(), 2);
        let paths: Vec<_> = result
            .iter()
            .map(|r| r.file.as_ref().unwrap().path.clone())
            .collect();
        assert!(paths.contains(&"go.sum".to_owned()));
        assert!(paths.contains(&"go.mod".to_owned()));
    }

    // Ported: "returns config constraint when set" — gomod/artifacts.spec.ts line 2837
    #[test]
    fn derive_go_toolchain_constraints_config() {
        assert_eq!(
            derive_go_toolchain_constraints(Some("1.21"), ""),
            Some("1.21".to_owned())
        );
    }

    // Ported: "config constraint takes precedence over go.mod content" — gomod/artifacts.spec.ts line 2843
    #[test]
    fn derive_go_toolchain_constraints_config_precedence() {
        assert_eq!(
            derive_go_toolchain_constraints(Some("1.20"), "go 1.23.5"),
            Some("1.20".to_owned())
        );
    }

    // Ported: "returns toolchain version when toolchain directive is present" — gomod/artifacts.spec.ts line 2852
    #[test]
    fn derive_go_toolchain_constraints_toolchain() {
        assert_eq!(
            derive_go_toolchain_constraints(None, "go 1.13\ntoolchain go1.23.6"),
            Some("1.23.6".to_owned())
        );
    }

    // Ported: "returns full go version when only full go directive is present (no toolchain)" — gomod/artifacts.spec.ts line 2858
    #[test]
    fn derive_go_toolchain_constraints_full_go() {
        assert_eq!(
            derive_go_toolchain_constraints(None, "go 1.23.5"),
            Some("1.23.5".to_owned())
        );
    }

    // Ported: "returns range constraint for major.minor go directive" — gomod/artifacts.spec.ts line 2862
    #[test]
    fn derive_go_toolchain_constraints_minor_range() {
        assert_eq!(
            derive_go_toolchain_constraints(None, "go 1.17"),
            Some("^1.17".to_owned())
        );
    }

    // Ported: "returns undefined when no go version in content and no config constraint" — gomod/artifacts.spec.ts line 2866
    #[test]
    fn derive_go_toolchain_constraints_undefined() {
        assert_eq!(
            derive_go_toolchain_constraints(None, "module example.com/foo"),
            None
        );
    }

    // Ported: "ignores constraints.golang and falls back to go.mod content" — gomod/artifacts.spec.ts line 2873
    #[test]
    fn derive_go_toolchain_constraints_ignores_golang_constraint() {
        // Our Rust API only passes `constraints.go`, so `constraints.golang`
        // is never passed in.  This test documents that behaviour.
        assert_eq!(
            derive_go_toolchain_constraints(None, "go 1.23.5"),
            Some("1.23.5".to_owned())
        );
    }

    // Ported: "skips updating import paths when incompatible version" — gomod/artifacts.spec.ts line 1948
    #[tokio::test]
    async fn skips_updating_import_paths_when_incompatible_version() {
        let dir = tempfile::tempdir().unwrap();
        let runner = GomodArtifactRunner::new();
        let mut input = make_input(
            &dir,
            "module example.com/test\n\ngo 1.22\n",
            Some("old sum\n"),
        );
        input.config.update_type = Some("major".to_owned());
        input
            .config
            .post_update_options
            .push("gomodUpdateImportPaths".to_owned());
        input.updated_deps = vec![crate::artifacts::UpdatedDep {
            dep_name: "github.com/docker/docker".to_owned(),
            package_name: None,
            current_value: None,
            new_value: None,
            locked_version: None,
            new_version: Some("v23.0.0+incompatible".to_owned()),
            package_file: "go.mod".to_owned(),
            manager: "gomod".to_owned(),
            datasource: None,
            update_type: None,
        }];
        input.config.env = make_fake_go(
            &dir,
            b"if [ \"$1\" = \"get\" ]; then
                echo 'new sum' > go.sum
                echo 'new mod' > go.mod
              fi
            ",
        );
        let result = runner.update_artifacts(&input).await.unwrap().unwrap();
        assert_eq!(result.len(), 2);
        let paths: Vec<_> = result
            .iter()
            .map(|r| r.file.as_ref().unwrap().path.clone())
            .collect();
        assert!(paths.contains(&"go.sum".to_owned()));
        assert!(paths.contains(&"go.mod".to_owned()));
    }

    // Ported: "skips updating import paths when invalid major version" — gomod/artifacts.spec.ts line 1902
    #[tokio::test]
    async fn skips_updating_import_paths_when_invalid_major_version() {
        let dir = tempfile::tempdir().unwrap();
        let runner = GomodArtifactRunner::new();
        let mut input = make_input(
            &dir,
            "module example.com/test\n\ngo 1.22\n",
            Some("old sum\n"),
        );
        input.config.update_type = Some("major".to_owned());
        input
            .config
            .post_update_options
            .push("gomodUpdateImportPaths".to_owned());
        input.updated_deps = vec![crate::artifacts::UpdatedDep {
            dep_name: "github.com/pkg/errors".to_owned(),
            package_name: None,
            current_value: None,
            new_value: None,
            locked_version: None,
            new_version: Some("vx.0.0".to_owned()),
            package_file: "go.mod".to_owned(),
            manager: "gomod".to_owned(),
            datasource: None,
            update_type: None,
        }];
        input.config.env = make_fake_go(
            &dir,
            b"if [ \"$1\" = \"get\" ]; then
                echo 'new sum' > go.sum
                echo 'new mod' > go.mod
              fi
            ",
        );
        let result = runner.update_artifacts(&input).await.unwrap().unwrap();
        assert_eq!(result.len(), 2);
        let paths: Vec<_> = result
            .iter()
            .map(|r| r.file.as_ref().unwrap().path.clone())
            .collect();
        assert!(paths.contains(&"go.sum".to_owned()));
        assert!(paths.contains(&"go.mod".to_owned()));
    }

    // Ported: "skips updating import paths with gomodUpdateImportPaths on v0 to v1" — gomod/artifacts.spec.ts line 1856
    #[tokio::test]
    async fn skips_updating_import_paths_on_v0_to_v1() {
        let dir = tempfile::tempdir().unwrap();
        let runner = GomodArtifactRunner::new();
        let mut input = make_input(
            &dir,
            "module example.com/test\n\ngo 1.22\n",
            Some("old sum\n"),
        );
        input.config.update_type = Some("major".to_owned());
        input
            .config
            .post_update_options
            .push("gomodUpdateImportPaths".to_owned());
        input.updated_deps = vec![crate::artifacts::UpdatedDep {
            dep_name: "github.com/pkg/errors".to_owned(),
            package_name: None,
            current_value: None,
            new_value: None,
            locked_version: None,
            new_version: Some("v1.0.0".to_owned()),
            package_file: "go.mod".to_owned(),
            manager: "gomod".to_owned(),
            datasource: None,
            update_type: None,
        }];
        input.config.env = make_fake_go(
            &dir,
            b"if [ \"$1\" = \"get\" ]; then
                echo 'new sum' > go.sum
                echo 'new mod' > go.mod
              fi
            ",
        );
        let result = runner.update_artifacts(&input).await.unwrap().unwrap();
        assert_eq!(result.len(), 2);
        let paths: Vec<_> = result
            .iter()
            .map(|r| r.file.as_ref().unwrap().path.clone())
            .collect();
        assert!(paths.contains(&"go.sum".to_owned()));
        assert!(paths.contains(&"go.mod".to_owned()));
    }

    // Ported: "returns artifact notices" — gomod/artifacts.spec.ts line 2466
    #[tokio::test]
    async fn returns_artifact_notices() {
        let dir = tempfile::tempdir().unwrap();
        let runner = GomodArtifactRunner::new();
        let go_mod_before = "module example.com/test\n\ngo 1.22\n\nrequire github.com/foo/foo v1.0.0\nrequire github.com/bar/bar v1.0.0\n";
        let mut input = make_input(&dir, go_mod_before, Some("old sum\n"));
        input.config.update_type = Some("major".to_owned());
        input
            .config
            .post_update_options
            .push("gomodUpdateImportPaths".to_owned());
        // fake go bumps an extra dependency in go.mod
        input.config.env = make_fake_go(
            &dir,
            b"if [ \"$1\" = \"get\" ]; then
                echo 'new sum' > go.sum
                printf 'module example.com/test\n\ngo 1.22\n\nrequire github.com/foo/foo v1.0.0\nrequire github.com/bar/bar v2.0.0\n' > go.mod
              fi
            ",
        );
        let result = runner.update_artifacts(&input).await.unwrap().unwrap();
        let go_mod_result = result
            .iter()
            .find(|r| r.file.as_ref().unwrap().path == "go.mod")
            .unwrap();
        assert!(go_mod_result.notice.is_some());
        let notice = go_mod_result.notice.as_ref().unwrap();
        assert_eq!(notice.file, "go.mod");
        assert!(notice.message.contains("github.com/bar/bar"));
    }

    // Ported: "uses -modfile flag with go mod vendor for non-default go.mod filename" — gomod/artifacts.spec.ts line 2779
    #[tokio::test]
    async fn uses_modfile_with_go_mod_vendor() {
        let dir = tempfile::tempdir().unwrap();
        let runner = GomodArtifactRunner::new();
        let mut input = make_input_with_name(
            &dir,
            "tools.mod",
            "module example.com/test\n\ngo 1.22\n",
            Some("old sum\n"),
        );
        input
            .config
            .post_update_options
            .push("gomodTidy".to_owned());
        // Create vendor directory with modules.txt to trigger implicit vendor
        std::fs::create_dir(dir.path().join("vendor")).unwrap();
        std::fs::write(dir.path().join("vendor/modules.txt"), "txt").unwrap();
        input.config.env = make_fake_go(
            &dir,
            b"if [ \"$1\" = \"get\" ]; then
                case \"$*\" in *-modfile=tools.mod*) ;; *) exit 1 ;; esac
                echo 'new sum' > tools.sum
              fi
              if [ \"$1\" = \"mod\" ] && [ \"$2\" = \"tidy\" ]; then
                case \"$*\" in *-modfile=tools.mod*) ;; *) exit 1 ;; esac
              fi
              if [ \"$1\" = \"mod\" ] && [ \"$2\" = \"vendor\" ]; then
                case \"$*\" in *-modfile=tools.mod*) ;; *) exit 1 ;; esac
              fi
            ",
        );
        let result = runner.update_artifacts(&input).await.unwrap().unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].file.as_ref().unwrap().path, "tools.sum");
    }
}
