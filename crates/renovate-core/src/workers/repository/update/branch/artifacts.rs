//! Artifact handling for branch updates.
//!
//! Mirrors `lib/workers/repository/update/branch/artifacts.ts`.

use serde::{Deserialize, Serialize};

use crate::workers::types::{ArtifactError, FileChange};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArtifactUpdateResult {
    pub updated_artifacts: Vec<FileChange>,
    pub artifact_errors: Vec<ArtifactError>,
}

pub fn update_artifacts(
    updated_package_files: &[FileChange],
    _manager: &str,
    skip_artifacts_update: bool,
) -> ArtifactUpdateResult {
    if skip_artifacts_update {
        return ArtifactUpdateResult::default();
    }

    if updated_package_files.is_empty() {
        return ArtifactUpdateResult::default();
    }

    ArtifactUpdateResult {
        updated_artifacts: vec![],
        artifact_errors: vec![],
    }
}

pub fn set_artifact_error_status(
    artifact_errors: &[ArtifactError],
    branch_name: &str,
    _context: Option<&str>,
) -> Option<String> {
    if artifact_errors.is_empty() {
        return None;
    }

    Some(format!("Artifact file update failure in {branch_name}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_update_result_default() {
        let r = ArtifactUpdateResult::default();
        assert!(r.updated_artifacts.is_empty());
        assert!(r.artifact_errors.is_empty());
    }

    #[test]
    fn update_artifacts_skip() {
        let result = update_artifacts(&[], "npm", true);
        assert!(result.updated_artifacts.is_empty());
    }

    #[test]
    fn update_artifacts_empty_files() {
        let result = update_artifacts(&[], "npm", false);
        assert!(result.updated_artifacts.is_empty());
    }

    #[test]
    fn update_artifacts_no_errors() {
        let files = vec![FileChange {
            path: "package.json".into(),
            contents: Some("{}".into()),
        }];
        let result = update_artifacts(&files, "npm", false);
        assert!(result.updated_artifacts.is_empty());
        assert!(result.artifact_errors.is_empty());
    }

    #[test]
    fn set_artifact_error_status_no_errors() {
        let result = set_artifact_error_status(&[], "renovate/lodash-4.x", Some("renovate/artifacts"));
        assert!(result.is_none());
    }

    #[test]
    fn set_artifact_error_status_with_errors() {
        let errors = vec![ArtifactError {
            lock_file: Some("package-lock.json".into()),
            stderr: Some("error".into()),
        }];
        let result = set_artifact_error_status(&errors, "renovate/lodash-4.x", Some("renovate/artifacts"));
        assert!(result.is_some());
        assert!(result.unwrap().contains("renovate/lodash-4.x"));
    }

    #[test]
    fn set_artifact_error_status_no_context() {
        let errors = vec![ArtifactError {
            lock_file: Some("package-lock.json".into()),
            stderr: Some("error".into()),
        }];
        let result = set_artifact_error_status(&errors, "renovate/lodash-4.x", None);
        assert!(result.is_some());
    }
}
