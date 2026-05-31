//! PR comment formatting.
//!
//! Formats PR comments with markdown for ensureComment/ensureCommentRemoval.
//!
//! Renovate reference: `lib/modules/platform/comment.ts`

use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct CommentConfig {
    pub number: i64,
    pub topic: Option<String>,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct CommentRemovalConfig {
    pub number: i64,
    pub removal_type: CommentRemovalType,
}

#[derive(Debug, Clone)]
pub enum CommentRemovalType {
    ByTopic { topic: String },
    ByContent { content: String },
}

pub fn hash_content(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let result = hasher.finalize();
    result.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn format_pr_comment(topic: Option<&str>, content: &str) -> String {
    match topic {
        Some(t) if !t.is_empty() => {
            format!("## {}\n\n{}", t, content)
        }
        _ => content.to_owned(),
    }
}

pub fn build_comment_config(number: i64, topic: Option<String>, content: String) -> CommentConfig {
    CommentConfig {
        number,
        topic,
        content,
    }
}

pub fn build_removal_config_by_topic(number: i64, topic: String) -> CommentRemovalConfig {
    CommentRemovalConfig {
        number,
        removal_type: CommentRemovalType::ByTopic { topic },
    }
}

pub fn build_removal_config_by_content(number: i64, content: String) -> CommentRemovalConfig {
    CommentRemovalConfig {
        number,
        removal_type: CommentRemovalType::ByContent { content },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_content_is_deterministic() {
        let h1 = hash_content("hello world");
        let h2 = hash_content("hello world");
        assert_eq!(h1, h2);
    }

    #[test]
    fn hash_content_differs_for_different_input() {
        let h1 = hash_content("hello");
        let h2 = hash_content("world");
        assert_ne!(h1, h2);
    }

    #[test]
    fn format_pr_comment_with_topic() {
        let result = format_pr_comment(Some("Dependencies"), "Updated deps");
        assert!(result.starts_with("## Dependencies"));
        assert!(result.contains("Updated deps"));
    }

    #[test]
    fn format_pr_comment_without_topic() {
        let result = format_pr_comment(None, "Just content");
        assert_eq!(result, "Just content");
    }

    #[test]
    fn format_pr_comment_empty_topic() {
        let result = format_pr_comment(Some(""), "Content");
        assert_eq!(result, "Content");
    }

    #[test]
    fn build_comment_config_fields() {
        let config = build_comment_config(42, Some("topic".into()), "body".into());
        assert_eq!(config.number, 42);
        assert_eq!(config.topic.as_deref(), Some("topic"));
        assert_eq!(config.content, "body");
    }

    #[test]
    fn removal_config_by_topic() {
        let config = build_removal_config_by_topic(42, "topic".into());
        assert_eq!(config.number, 42);
        match config.removal_type {
            CommentRemovalType::ByTopic { ref topic } => assert_eq!(topic, "topic"),
            CommentRemovalType::ByContent { .. } => panic!("expected ByTopic"),
        }
    }

    #[test]
    fn removal_config_by_content() {
        let config = build_removal_config_by_content(42, "body".into());
        assert_eq!(config.number, 42);
        match config.removal_type {
            CommentRemovalType::ByContent { ref content } => assert_eq!(content, "body"),
            CommentRemovalType::ByTopic { .. } => panic!("expected ByContent"),
        }
    }
}
