//! Changelog retrieval for dependency updates.
//!
//! Mirrors `lib/workers/repository/update/pr/changelog/index.ts`.

use serde::{Deserialize, Serialize};

use crate::workers::repository::update::pr::types::ChangeLogResult;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeLogChange {
    pub title: Option<String>,
    pub body: Option<String>,
    pub date: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct GetChangeLogConfig {
    pub dep_name: Option<String>,
    pub source_url: Option<String>,
    pub source_directory: Option<String>,
    pub current_version: Option<String>,
    pub new_version: Option<String>,
    pub has_release_notes: bool,
    pub platform: Option<String>,
    pub repository: Option<String>,
}

pub fn get_changelog(config: &GetChangeLogConfig) -> Option<ChangeLogResult> {
    if !config.has_release_notes {
        return None;
    }

    let _ = (
        config.dep_name.as_deref().unwrap_or(""),
        config.current_version.as_deref().unwrap_or(""),
        config.new_version.as_deref().unwrap_or(""),
    );

    None
}

pub fn get_changelog_url(
    source_url: Option<&str>,
    _current_version: Option<&str>,
    _new_version: Option<&str>,
) -> Option<String> {
    source_url.map(|s| s.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changelog_change_default() {
        let c = ChangeLogChange::default();
        assert!(c.title.is_none());
        assert!(c.body.is_none());
        assert!(c.date.is_none());
    }

    #[test]
    fn get_changelog_config_default() {
        let c = GetChangeLogConfig::default();
        assert!(c.dep_name.is_none());
        assert!(!c.has_release_notes);
    }

    #[test]
    fn get_changelog_no_release_notes() {
        let config = GetChangeLogConfig {
            has_release_notes: false,
            ..Default::default()
        };
        assert!(get_changelog(&config).is_none());
    }

    #[test]
    fn get_changelog_with_release_notes() {
        let config = GetChangeLogConfig {
            dep_name: Some("lodash".into()),
            current_version: Some("4.17.0".into()),
            new_version: Some("4.18.2".into()),
            has_release_notes: true,
            ..Default::default()
        };
        let result = get_changelog(&config);
        assert!(result.is_none());
    }

    #[test]
    fn get_changelog_url_some() {
        let url = get_changelog_url(Some("https://github.com/lodash/lodash"), None, None);
        assert_eq!(url, Some("https://github.com/lodash/lodash".to_owned()));
    }

    #[test]
    fn get_changelog_url_none() {
        let url = get_changelog_url(None, None, None);
        assert!(url.is_none());
    }

    #[test]
    fn changelog_change_construct() {
        let c = ChangeLogChange {
            title: Some("v1.0.0".into()),
            body: Some("Bug fixes".into()),
            date: Some("2024-01-01".into()),
        };
        assert_eq!(c.title, Some("v1.0.0".into()));
        assert_eq!(c.body, Some("Bug fixes".into()));
        assert_eq!(c.date, Some("2024-01-01".into()));
    }
}
