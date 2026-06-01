//! PR update types.
//!
//! Mirrors `lib/workers/repository/update/pr/changelog/types.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PrConfig {
    pub title: String,
    pub body: Option<String>,
    pub labels: Vec<String>,
    pub reviewers: Vec<String>,
    pub additional_reviewers: Vec<String>,
    pub assignees: Vec<String>,
    pub additional_assignees: Vec<String>,
    pub milestone: Option<u64>,
    pub draft: Option<bool>,
    pub header: Option<String>,
    pub footer: Option<String>,
    pub pr_priority: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeLogPlatform {
    Bitbucket,
    BitbucketServer,
    Forgejo,
    Gitea,
    Github,
    Gitlab,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeLogError {
    MissingBitbucketToken,
    MissingGithubToken,
    MissingGitlabToken,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeLogProject {
    pub package_name: Option<String>,
    pub dep_name: Option<String>,
    pub platform: Option<ChangeLogPlatform>,
    pub api_base_url: Option<String>,
    pub base_url: Option<String>,
    pub repository: Option<String>,
    pub source_url: Option<String>,
    pub source_directory: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeLogRelease {
    pub version: Option<String>,
    pub git_ref: Option<String>,
    pub date: Option<String>,
    pub body: Option<String>,
    pub url: Option<String>,
    pub name: Option<String>,
    pub compare_url: Option<String>,
    pub is_rollback: Option<bool>,
    pub changes: Vec<ChangeLogChange>,
    pub release_notes: Option<ChangeLogNotes>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeLogChange {
    pub date: Option<String>,
    pub message: Option<String>,
    pub sha: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeLogNotes {
    pub body: Option<String>,
    pub id: Option<u64>,
    pub name: Option<String>,
    pub tag: Option<String>,
    pub notes_source_url: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeLogResult {
    pub has_release_notes: Option<bool>,
    pub project: Option<ChangeLogProject>,
    pub versions: Option<Vec<ChangeLogRelease>>,
    pub error: Option<ChangeLogError>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeLogFile {
    pub changelog_file: String,
    pub changelog_md: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pr_config_default() {
        let c = PrConfig::default();
        assert!(c.title.is_empty());
        assert!(c.body.is_none());
        assert!(c.labels.is_empty());
        assert!(c.reviewers.is_empty());
    }

    #[test]
    fn pr_config_construct() {
        let c = PrConfig {
            title: "Update lodash to v4.18.2".into(),
            body: Some("This PR updates lodash.".into()),
            labels: vec!["dependencies".into()],
            reviewers: vec!["alice".into(), "bob".into()],
            draft: Some(false),
            ..Default::default()
        };
        assert_eq!(c.title, "Update lodash to v4.18.2");
        assert_eq!(c.labels, vec!["dependencies"]);
        assert_eq!(c.reviewers.len(), 2);
        assert_eq!(c.draft, Some(false));
    }

    #[test]
    fn pr_config_serialization_roundtrip() {
        let c = PrConfig {
            title: "Update react".into(),
            labels: vec!["deps".into()],
            pr_priority: Some(5),
            ..Default::default()
        };
        let json = serde_json::to_string(&c).unwrap();
        let back: PrConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(back.title, "Update react");
        assert_eq!(back.labels, vec!["deps"]);
        assert_eq!(back.pr_priority, Some(5));
    }

    #[test]
    fn changelog_platform_variants() {
        let variants = [
            ChangeLogPlatform::Bitbucket,
            ChangeLogPlatform::BitbucketServer,
            ChangeLogPlatform::Forgejo,
            ChangeLogPlatform::Gitea,
            ChangeLogPlatform::Github,
            ChangeLogPlatform::Gitlab,
        ];
        for (i, a) in variants.iter().enumerate() {
            for (j, b) in variants.iter().enumerate() {
                if i != j {
                    assert_ne!(a, b);
                }
            }
        }
    }

    #[test]
    fn changelog_error_variants() {
        assert_ne!(
            ChangeLogError::MissingBitbucketToken,
            ChangeLogError::MissingGithubToken
        );
        assert_ne!(
            ChangeLogError::MissingGitlabToken,
            ChangeLogError::MissingGithubToken
        );
    }

    #[test]
    fn changelog_result_default() {
        let r = ChangeLogResult::default();
        assert!(r.has_release_notes.is_none());
        assert!(r.project.is_none());
        assert!(r.versions.is_none());
        assert!(r.error.is_none());
    }

    #[test]
    fn changelog_result_construct() {
        let r = ChangeLogResult {
            has_release_notes: Some(true),
            project: Some(ChangeLogProject {
                dep_name: Some("lodash".into()),
                platform: Some(ChangeLogPlatform::Github),
                repository: Some("lodash/lodash".into()),
                ..Default::default()
            }),
            versions: Some(vec![ChangeLogRelease {
                version: Some("4.18.2".into()),
                ..Default::default()
            }]),
            ..Default::default()
        };
        assert_eq!(r.has_release_notes, Some(true));
        let proj = r.project.as_ref().unwrap();
        assert_eq!(proj.dep_name, Some("lodash".into()));
        assert_eq!(proj.platform, Some(ChangeLogPlatform::Github));
        let versions = r.versions.as_ref().unwrap();
        assert_eq!(versions.len(), 1);
        assert_eq!(versions[0].version, Some("4.18.2".into()));
    }

    #[test]
    fn changelog_release_serialization() {
        let r = ChangeLogRelease {
            version: Some("1.0.0".into()),
            git_ref: Some("v1.0.0".into()),
            date: Some("2024-01-01".into()),
            changes: vec![ChangeLogChange {
                message: Some("fix: bug".into()),
                sha: Some("abc123".into()),
                ..Default::default()
            }],
            ..Default::default()
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: ChangeLogRelease = serde_json::from_str(&json).unwrap();
        assert_eq!(back.version, Some("1.0.0".into()));
        assert_eq!(back.changes.len(), 1);
    }

    #[test]
    fn changelog_notes_construct() {
        let n = ChangeLogNotes {
            body: Some("Release notes body".into()),
            url: Some("https://github.com/repo/releases/tag/v1".into()),
            notes_source_url: Some("https://api.github.com/repos/repo/releases/1".into()),
            ..Default::default()
        };
        assert_eq!(n.body, Some("Release notes body".into()));
    }

    #[test]
    fn changelog_file_construct() {
        let f = ChangeLogFile {
            changelog_file: "CHANGELOG.md".into(),
            changelog_md: Some("# Changelog\n\n## 1.0.0".into()),
        };
        assert_eq!(f.changelog_file, "CHANGELOG.md");
        assert!(f.changelog_md.is_some());
    }

    #[test]
    fn changelog_platform_serialization_roundtrip() {
        let platforms = [
            ChangeLogPlatform::Github,
            ChangeLogPlatform::Gitlab,
            ChangeLogPlatform::Gitea,
        ];
        for p in &platforms {
            let json = serde_json::to_string(p).unwrap();
            let back: ChangeLogPlatform = serde_json::from_str(&json).unwrap();
            assert_eq!(*p, back);
        }
    }

    #[test]
    fn changelog_error_serialization_roundtrip() {
        let errors = [
            ChangeLogError::MissingBitbucketToken,
            ChangeLogError::MissingGithubToken,
            ChangeLogError::MissingGitlabToken,
        ];
        for e in &errors {
            let json = serde_json::to_string(e).unwrap();
            let back: ChangeLogError = serde_json::from_str(&json).unwrap();
            assert_eq!(*e, back);
        }
    }

    #[test]
    fn changelog_result_with_error() {
        let r = ChangeLogResult {
            error: Some(ChangeLogError::MissingGithubToken),
            ..Default::default()
        };
        assert_eq!(r.error, Some(ChangeLogError::MissingGithubToken));
        assert!(r.has_release_notes.is_none());
    }

    #[test]
    fn changelog_project_default() {
        let p = ChangeLogProject::default();
        assert!(p.package_name.is_none());
        assert!(p.platform.is_none());
        assert!(p.repository.is_none());
    }
}
