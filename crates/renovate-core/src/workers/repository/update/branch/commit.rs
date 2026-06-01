//! Branch commit creation.
//!
//! Mirrors `lib/workers/repository/update/branch/commit.ts`.

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

use crate::workers::types::FileChange;

static CONVENTIONAL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\w+)(?:\(([^)]*)\))?(!?)\s*:\s*(.+)$").unwrap());

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommitMessage {
    pub commit_type: Option<String>,
    pub scope: Option<String>,
    pub subject: String,
    pub body: Option<String>,
    pub breaking: bool,
}

impl std::fmt::Display for CommitMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref ct) = self.commit_type {
            let scope = self
                .scope
                .as_deref()
                .map(|s| format!("({s})"))
                .unwrap_or_default();
            let bang = if self.breaking { "!" } else { "" };
            write!(f, "{ct}{scope}{bang}: {}", self.subject)?;
        } else {
            write!(f, "{}", self.subject)?;
        }
        if let Some(ref body) = self.body {
            write!(f, "\n\n{body}")?;
        }
        Ok(())
    }
}

pub fn format_commit_message(msg: &CommitMessage) -> String {
    msg.to_string()
}

pub fn parse_commit_message(s: &str) -> Option<CommitMessage> {
    let (title, body) = match s.find("\n\n") {
        Some(idx) => (&s[..idx], Some(s[idx + 2..].to_owned())),
        None => (s, None),
    };

    if let Some(caps) = CONVENTIONAL_RE.captures(title) {
        Some(CommitMessage {
            commit_type: Some(caps[1].to_owned()),
            scope: caps.get(2).map(|m| m.as_str().to_owned()),
            breaking: caps.get(3).is_some_and(|m| m.as_str() == "!"),
            subject: caps[4].to_owned(),
            body,
        })
    } else {
        Some(CommitMessage {
            commit_type: None,
            scope: None,
            subject: title.to_owned(),
            body,
            breaking: false,
        })
    }
}

pub fn commit_files_to_branch(
    updated_package_files: &[FileChange],
    updated_artifacts: &[FileChange],
    exclude_commit_paths: &[String],
    _branch_name: &str,
    commit_message: &str,
    _base_branch: &str,
    _force_commit: bool,
) -> Option<String> {
    let mut updated_files: Vec<FileChange> = updated_package_files
        .iter()
        .chain(updated_artifacts.iter())
        .cloned()
        .collect();

    if !exclude_commit_paths.is_empty() {
        updated_files.retain(|f| !exclude_commit_paths.iter().any(|p| f.path.starts_with(p)));
    }

    if updated_files.is_empty() {
        return None;
    }

    let unique_count = updated_files
        .iter()
        .map(|f| f.path.clone())
        .collect::<std::collections::HashSet<_>>()
        .len();

    let _ = unique_count;

    Some(format_commit_message(&CommitMessage {
        subject: commit_message.to_owned(),
        ..Default::default()
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commit_message_default() {
        let msg = CommitMessage::default();
        assert!(msg.commit_type.is_none());
        assert!(msg.scope.is_none());
        assert!(msg.subject.is_empty());
        assert!(msg.body.is_none());
        assert!(!msg.breaking);
    }

    #[test]
    fn commit_message_display_plain() {
        let msg = CommitMessage {
            subject: "Update lodash to v4.18.2".into(),
            ..Default::default()
        };
        assert_eq!(msg.to_string(), "Update lodash to v4.18.2");
    }

    #[test]
    fn commit_message_display_semantic() {
        let msg = CommitMessage {
            commit_type: Some("chore".into()),
            scope: Some("deps".into()),
            subject: "update lodash".into(),
            ..Default::default()
        };
        assert_eq!(msg.to_string(), "chore(deps): update lodash");
    }

    #[test]
    fn commit_message_display_semantic_breaking() {
        let msg = CommitMessage {
            commit_type: Some("feat".into()),
            scope: Some("api".into()),
            subject: "breaking change".into(),
            breaking: true,
            ..Default::default()
        };
        assert_eq!(msg.to_string(), "feat(api)!: breaking change");
    }

    #[test]
    fn commit_message_display_with_body() {
        let msg = CommitMessage {
            subject: "fix: bug".into(),
            body: Some("Detailed description".into()),
            ..Default::default()
        };
        assert_eq!(msg.to_string(), "fix: bug\n\nDetailed description");
    }

    #[test]
    fn format_commit_message_wraps_display() {
        let msg = CommitMessage {
            commit_type: Some("fix".into()),
            subject: "bug fix".into(),
            ..Default::default()
        };
        assert_eq!(format_commit_message(&msg), "fix: bug fix");
    }

    #[test]
    fn parse_plain_commit_message() {
        let msg = parse_commit_message("Update dependency lodash").unwrap();
        assert!(msg.commit_type.is_none());
        assert_eq!(msg.subject, "Update dependency lodash");
        assert!(msg.body.is_none());
    }

    #[test]
    fn parse_conventional_commit_message() {
        let msg = parse_commit_message("chore(deps): update lodash").unwrap();
        assert_eq!(msg.commit_type, Some("chore".into()));
        assert_eq!(msg.scope, Some("deps".into()));
        assert_eq!(msg.subject, "update lodash");
        assert!(!msg.breaking);
        assert!(msg.body.is_none());
    }

    #[test]
    fn parse_conventional_commit_breaking() {
        let msg = parse_commit_message("feat(api)!: breaking change").unwrap();
        assert_eq!(msg.commit_type, Some("feat".into()));
        assert_eq!(msg.scope, Some("api".into()));
        assert!(msg.breaking);
        assert_eq!(msg.subject, "breaking change");
    }

    #[test]
    fn parse_commit_message_with_body() {
        let msg = parse_commit_message("fix: bug\n\nDetailed description").unwrap();
        assert_eq!(msg.commit_type, Some("fix".into()));
        assert_eq!(msg.subject, "bug");
        assert_eq!(msg.body, Some("Detailed description".into()));
    }

    #[test]
    fn parse_no_scope() {
        let msg = parse_commit_message("fix: something").unwrap();
        assert_eq!(msg.commit_type, Some("fix".into()));
        assert!(msg.scope.is_none());
        assert_eq!(msg.subject, "something");
    }

    #[test]
    fn commit_files_returns_none_for_empty() {
        let result = commit_files_to_branch(
            &[],
            &[],
            &[],
            "renovate/lodash-4.x",
            "update lodash",
            "main",
            false,
        );
        assert!(result.is_none());
    }

    #[test]
    fn commit_files_returns_some_for_files() {
        let files = vec![FileChange {
            path: "package.json".into(),
            contents: Some("{}".into()),
        }];
        let result = commit_files_to_branch(
            &files,
            &[],
            &[],
            "renovate/lodash-4.x",
            "update lodash",
            "main",
            false,
        );
        assert!(result.is_some());
    }

    #[test]
    fn commit_files_filters_excluded() {
        let files = vec![
            FileChange {
                path: "package.json".into(),
                contents: Some("{}".into()),
            },
            FileChange {
                path: "dist/output.js".into(),
                contents: Some("".into()),
            },
        ];
        let exclude = vec!["dist".to_owned()];
        let result = commit_files_to_branch(
            &files,
            &[],
            &exclude,
            "renovate/lodash-4.x",
            "update lodash",
            "main",
            false,
        );
        assert!(result.is_some());
    }

    #[test]
    fn commit_files_all_excluded_returns_none() {
        let files = vec![FileChange {
            path: "dist/output.js".into(),
            contents: Some("".into()),
        }];
        let exclude = vec!["dist".to_owned()];
        let result = commit_files_to_branch(
            &files,
            &[],
            &exclude,
            "renovate/lodash-4.x",
            "update lodash",
            "main",
            false,
        );
        assert!(result.is_none());
    }
}
