//! Auto-replace dependency versions in file content.
//!
//! Mirrors `lib/workers/repository/update/branch/auto-replace.ts`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutoReplaceResult {
    pub content: Option<String>,
    pub success: bool,
}

#[allow(clippy::too_many_arguments)]
pub fn auto_replace(
    content: &str,
    current_value: Option<&str>,
    new_value: Option<&str>,
    current_digest: Option<&str>,
    new_digest: Option<&str>,
    dep_name: Option<&str>,
    new_name: Option<&str>,
    replace_string: Option<&str>,
) -> AutoReplaceResult {
    let search = replace_string
        .or(current_value)
        .or(current_digest)
        .unwrap_or("");

    if search.is_empty() {
        return AutoReplaceResult {
            content: Some(content.to_owned()),
            success: true,
        };
    }

    let position = find_replace_position(content, search);
    if position.is_none() {
        return AutoReplaceResult {
            content: Some(content.to_owned()),
            success: false,
        };
    }

    let mut new_string = search.to_owned();

    if let (Some(cv), Some(nv)) = (current_value, new_value)
        && cv != nv
    {
        new_string = new_string.replace(cv, nv);
    }

    if let (Some(cd), Some(nd)) = (current_digest, new_digest)
        && cd != nd
    {
        new_string = new_string.replace(cd, nd);
    }

    if let (Some(dn), Some(nn)) = (dep_name, new_name)
        && dn != nn
    {
        new_string = new_string.replace(dn, nn);
    }

    if new_string == search {
        return AutoReplaceResult {
            content: Some(content.to_owned()),
            success: true,
        };
    }

    let pos = position.unwrap();
    let mut result = String::with_capacity(content.len() + new_string.len() - search.len());
    result.push_str(&content[..pos]);
    result.push_str(&new_string);
    result.push_str(&content[pos + search.len()..]);

    AutoReplaceResult {
        content: Some(result),
        success: true,
    }
}

pub fn find_replace_position(content: &str, search: &str) -> Option<usize> {
    content.find(search)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_replace_result_default() {
        let r = AutoReplaceResult::default();
        assert!(r.content.is_none());
        assert!(!r.success);
    }

    #[test]
    fn auto_replace_value() {
        let result = auto_replace(
            r#"{"version": "1.0.0"}"#,
            Some("1.0.0"),
            Some("2.0.0"),
            None,
            None,
            Some("lodash"),
            None,
            None,
        );
        assert!(result.success);
        assert_eq!(result.content, Some(r#"{"version": "2.0.0"}"#.to_owned()));
    }

    #[test]
    fn auto_replace_digest() {
        let result = auto_replace(
            "sha256:abc123def456",
            None,
            None,
            Some("abc123def456"),
            Some("newhash789xyz"),
            Some("image"),
            None,
            None,
        );
        assert!(result.success);
        assert_eq!(result.content, Some("sha256:newhash789xyz".to_owned()));
    }

    #[test]
    fn auto_replace_name() {
        let result = auto_replace(
            r#"{"@old/pkg": "^1.0.0"}"#,
            None,
            None,
            None,
            None,
            Some("@old/pkg"),
            Some("@new/pkg"),
            Some("@old/pkg"),
        );
        assert!(result.success);
        assert_eq!(
            result.content,
            Some(r#"{"@new/pkg": "^1.0.0"}"#.to_owned())
        );
    }

    #[test]
    fn auto_replace_not_found_returns_unsuccessful() {
        let result = auto_replace(
            "no match here",
            Some("1.0.0"),
            Some("2.0.0"),
            None,
            None,
            Some("lodash"),
            None,
            None,
        );
        assert!(!result.success);
    }

    #[test]
    fn auto_replace_empty_search_returns_original() {
        let result = auto_replace(
            "some content",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        assert!(result.success);
        assert_eq!(result.content, Some("some content".to_owned()));
    }

    #[test]
    fn auto_replace_with_explicit_replace_string() {
        let result = auto_replace(
            "version = \"1.2.3\"",
            None,
            None,
            None,
            None,
            None,
            None,
            Some("1.2.3"),
        );
        assert!(result.success);
        assert_eq!(result.content, Some("version = \"1.2.3\"".to_owned()));
    }

    #[test]
    fn auto_replace_same_value_no_change() {
        let result = auto_replace(
            "1.0.0",
            Some("1.0.0"),
            Some("1.0.0"),
            None,
            None,
            None,
            None,
            None,
        );
        assert!(result.success);
        assert_eq!(result.content, Some("1.0.0".to_owned()));
    }

    #[test]
    fn find_replace_position_found() {
        assert_eq!(find_replace_position("hello world", "world"), Some(6));
    }

    #[test]
    fn find_replace_position_not_found() {
        assert_eq!(find_replace_position("hello world", "xyz"), None);
    }

    #[test]
    fn auto_replace_combined_value_and_name() {
        let result = auto_replace(
            r#"lodash@4.17.0"#,
            Some("4.17.0"),
            Some("4.18.2"),
            None,
            None,
            Some("lodash"),
            Some("lodash-es"),
            None,
        );
        assert!(result.success);
        let content = result.content.unwrap();
        assert!(content.contains("4.18.2"));
    }
}
