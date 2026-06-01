//! Get updated files for a branch.
//!
//! Mirrors `lib/workers/repository/update/branch/get-updated.ts`.

use serde::{Deserialize, Serialize};

use crate::workers::types::FileChange;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdatedFile {
    pub path: String,
    pub contents: Option<String>,
    pub is_deletion: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdatedFilesResult {
    pub updated_package_files: Vec<FileChange>,
    pub updated_artifacts: Vec<FileChange>,
    pub artifact_errors: Vec<crate::workers::types::ArtifactError>,
    pub reuse_existing_branch: bool,
}

pub fn get_updated_files(
    existing_content: &str,
    current_value: Option<&str>,
    new_value: Option<&str>,
    _dep_name: &str,
) -> Option<String> {
    let search = current_value?;
    if !existing_content.contains(search) {
        return None;
    }

    let new_value = new_value?;
    Some(existing_content.replace(search, new_value))
}

pub fn collect_updated_package_files(
    content_map: &std::collections::HashMap<String, String>,
) -> Vec<FileChange> {
    content_map
        .iter()
        .map(|(path, contents)| FileChange {
            path: path.clone(),
            contents: Some(contents.clone()),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn updated_file_default() {
        let f = UpdatedFile::default();
        assert!(f.path.is_empty());
        assert!(f.contents.is_none());
        assert!(!f.is_deletion);
    }

    #[test]
    fn updated_files_result_default() {
        let r = UpdatedFilesResult::default();
        assert!(r.updated_package_files.is_empty());
        assert!(r.updated_artifacts.is_empty());
        assert!(r.artifact_errors.is_empty());
        assert!(!r.reuse_existing_branch);
    }

    #[test]
    fn get_updated_files_replaces_value() {
        let result = get_updated_files(
            r#"{"lodash": "4.17.0"}"#,
            Some("4.17.0"),
            Some("4.18.2"),
            "lodash",
        );
        assert_eq!(result, Some(r#"{"lodash": "4.18.2"}"#.to_owned()));
    }

    #[test]
    fn get_updated_files_no_current_value() {
        let result = get_updated_files("content", None, Some("new"), "dep");
        assert!(result.is_none());
    }

    #[test]
    fn get_updated_files_no_new_value() {
        let result = get_updated_files("content with old", Some("old"), None, "dep");
        assert!(result.is_none());
    }

    #[test]
    fn get_updated_files_not_found() {
        let result = get_updated_files("no match here", Some("4.17.0"), Some("4.18.2"), "lodash");
        assert!(result.is_none());
    }

    #[test]
    fn collect_updated_package_files_from_map() {
        let mut map = std::collections::HashMap::new();
        map.insert("package.json".to_owned(), "{}".to_owned());
        map.insert("Cargo.toml".to_owned(), "[package]".to_owned());

        let files = super::collect_updated_package_files(&map);
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn collect_updated_package_files_empty_map() {
        let map = std::collections::HashMap::new();
        let files = super::collect_updated_package_files(&map);
        assert!(files.is_empty());
    }
}
