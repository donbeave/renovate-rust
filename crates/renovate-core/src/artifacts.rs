//! Artifact update infrastructure.
//!
//! Provides the shared `ArtifactRunner` trait and common types for lock file
//! regeneration / artifact update across all managers.
//!
//! Ports the `updateArtifacts` pattern from Renovate's manager API:
//! `lib/modules/manager/types.ts` — `UpdateArtifact`, `UpdateArtifactsResult`.

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// A single dep that was updated and needs artifact regeneration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdatedDep {
    pub dep_name: String,
    pub current_value: Option<String>,
    pub new_value: Option<String>,
    pub package_file: String,
    pub manager: String,
    pub datasource: Option<String>,
}

/// Configuration passed to artifact update.
#[derive(Debug, Clone, Default)]
pub struct ArtifactConfig {
    pub lock_file_dir: PathBuf,
    pub constraints: BTreeMap<String, String>,
    pub env: BTreeMap<String, String>,
    pub npmrc: Option<String>,
    pub post_update_options: Vec<String>,
    pub skip_installs: bool,
}

/// A file change resulting from artifact update.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileChange {
    #[serde(rename = "type")]
    pub change_type: String,
    pub path: String,
    pub contents: Option<String>,
}

impl FileChange {
    pub fn addition(path: impl Into<String>, contents: impl Into<String>) -> Self {
        Self {
            change_type: "addition".to_string(),
            path: path.into(),
            contents: Some(contents.into()),
        }
    }

    pub fn deletion(path: impl Into<String>) -> Self {
        Self {
            change_type: "deletion".to_string(),
            path: path.into(),
            contents: None,
        }
    }
}

/// An error from artifact update.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArtifactError {
    pub lock_file: String,
    pub stderr: String,
}

/// The result of an artifact update operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArtifactResult {
    pub file: Option<FileChange>,
    pub artifact_error: Option<ArtifactError>,
}

impl ArtifactResult {
    pub fn file_change(path: impl Into<String>, contents: impl Into<String>) -> Self {
        Self {
            file: Some(FileChange::addition(path, contents)),
            artifact_error: None,
        }
    }

    pub fn error(lock_file: impl Into<String>, stderr: impl Into<String>) -> Self {
        Self {
            file: None,
            artifact_error: Some(ArtifactError {
                lock_file: lock_file.into(),
                stderr: stderr.into(),
            }),
        }
    }
}

/// Input to an artifact update.
#[derive(Debug, Clone)]
pub struct UpdateArtifact {
    pub package_file_name: String,
    pub updated_deps: Vec<UpdatedDep>,
    pub new_package_file_content: String,
    pub config: ArtifactConfig,
}

/// The shared trait for artifact update across managers.
///
/// Each manager that supports lock file / artifact updates implements this trait.
pub trait ArtifactRunner: Send + Sync {
    /// Run artifact update for the given package file and deps.
    ///
    /// Returns `None` if no changes were made, or a list of results.
    fn update_artifacts(
        &self,
        input: &UpdateArtifact,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Option<Vec<ArtifactResult>>, ArtifactError>> + Send + '_>,
    >;
}

/// A no-op artifact runner for managers that don't support artifacts.
pub struct NoOpArtifactRunner;

impl ArtifactRunner for NoOpArtifactRunner {
    fn update_artifacts(
        &self,
        _input: &UpdateArtifact,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Option<Vec<ArtifactResult>>, ArtifactError>> + Send + '_>,
    > {
        Box::pin(async { Ok(None) })
    }
}

/// Registry of artifact runners by manager name.
pub struct ArtifactRegistry {
    runners: BTreeMap<String, Box<dyn ArtifactRunner>>,
}

impl ArtifactRegistry {
    pub fn new() -> Self {
        Self {
            runners: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, manager: impl Into<String>, runner: Box<dyn ArtifactRunner>) {
        self.runners.insert(manager.into(), runner);
    }

    pub fn get(&self, manager: &str) -> Option<&dyn ArtifactRunner> {
        self.runners.get(manager).map(|r| r.as_ref())
    }
}

impl std::fmt::Debug for ArtifactRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArtifactRegistry")
            .field("managers", &self.runners.keys().collect::<Vec<_>>())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_change_addition() {
        let fc = FileChange::addition("Cargo.lock", "contents here");
        assert_eq!(fc.change_type, "addition");
        assert_eq!(fc.path, "Cargo.lock");
        assert_eq!(fc.contents.as_deref(), Some("contents here"));
    }

    #[test]
    fn file_change_deletion() {
        let fc = FileChange::deletion("Cargo.lock");
        assert_eq!(fc.change_type, "deletion");
        assert!(fc.contents.is_none());
    }

    #[test]
    fn artifact_result_file_change() {
        let r = ArtifactResult::file_change("Cargo.lock", "new contents");
        assert!(r.file.is_some());
        assert!(r.artifact_error.is_none());
    }

    #[test]
    fn artifact_result_error() {
        let r = ArtifactResult::error("Cargo.lock", "build failed");
        assert!(r.file.is_none());
        assert_eq!(r.artifact_error.unwrap().stderr, "build failed");
    }

    #[tokio::test]
    async fn no_op_runner_returns_none() {
        let runner = NoOpArtifactRunner;
        let input = UpdateArtifact {
            package_file_name: "Cargo.toml".to_string(),
            updated_deps: vec![],
            new_package_file_content: String::new(),
            config: ArtifactConfig::default(),
        };
        let result = runner.update_artifacts(&input).await;
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn registry_register_and_get() {
        let mut reg = ArtifactRegistry::new();
        reg.register("cargo", Box::new(NoOpArtifactRunner));
        assert!(reg.get("cargo").is_some());
        assert!(reg.get("npm").is_none());
    }

    #[test]
    fn updated_dep_serialization() {
        let dep = UpdatedDep {
            dep_name: "serde".to_string(),
            current_value: Some("1.0.0".to_string()),
            new_value: Some("1.0.100".to_string()),
            package_file: "Cargo.toml".to_string(),
            manager: "cargo".to_string(),
            datasource: Some("crate".to_string()),
        };
        let json = serde_json::to_string(&dep).unwrap();
        assert!(json.contains("serde"));
    }
}
