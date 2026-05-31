//! Changelog retrieval.
//!
//! Mirrors `lib/workers/repository/changelog/index.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeLogSource {
    pub source_url: Option<String>,
    pub source_directory: Option<String>,
    pub base_version: Option<String>,
    pub new_version: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeLogResult {
    pub project: Option<ChangeLogSource>,
    pub has_release_notes: bool,
    pub releases: Vec<ChangeLogRelease>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeLogRelease {
    pub version: Option<String>,
    pub date: Option<String>,
    pub git_ref: Option<String>,
    pub body: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
    pub compare_url: Option<String>,
}

pub fn get_change_log(source: &ChangeLogSource) -> ChangeLogResult {
    ChangeLogResult {
        project: Some(source.clone()),
        has_release_notes: false,
        releases: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn change_log_source_default() {
        let s = ChangeLogSource::default();
        assert!(s.source_url.is_none());
        assert!(s.base_version.is_none());
    }

    #[test]
    fn change_log_result_default() {
        let r = ChangeLogResult::default();
        assert!(r.project.is_none());
        assert!(!r.has_release_notes);
        assert!(r.releases.is_empty());
    }

    #[test]
    fn change_log_release_default() {
        let r = ChangeLogRelease::default();
        assert!(r.version.is_none());
    }

    #[test]
    fn get_change_log_returns_result() {
        let source = ChangeLogSource {
            source_url: Some("https://github.com/lodash/lodash".into()),
            base_version: Some("4.17.0".into()),
            new_version: Some("4.17.21".into()),
            ..Default::default()
        };
        let result = get_change_log(&source);
        assert!(result.project.is_some());
        assert!(!result.has_release_notes);
    }

    #[test]
    fn change_log_source_serialization_roundtrip() {
        let s = ChangeLogSource {
            source_url: Some("https://github.com/test/test".into()),
            source_directory: None,
            base_version: Some("1.0.0".into()),
            new_version: Some("2.0.0".into()),
        };
        let json = serde_json::to_string(&s).unwrap();
        let back: ChangeLogSource = serde_json::from_str(&json).unwrap();
        assert_eq!(back.source_url, Some("https://github.com/test/test".into()));
    }

    #[test]
    fn change_log_release_serialization_roundtrip() {
        let r = ChangeLogRelease {
            version: Some("1.0.0".into()),
            date: Some("2024-01-01".into()),
            body: Some("Bug fixes".into()),
            ..Default::default()
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: ChangeLogRelease = serde_json::from_str(&json).unwrap();
        assert_eq!(back.version, Some("1.0.0".into()));
    }
}
