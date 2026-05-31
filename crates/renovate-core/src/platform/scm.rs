//! SCM interface trait.
//!
//! Defines the Source Control Management abstraction used across platforms.
//!
//! Renovate reference: `lib/modules/platform/scm.ts`

use std::future::Future;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScmResult {
    Ok(String),
    Conflict,
    NotFound,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct CommitConfig {
    pub branch_name: String,
    pub base_branch: Option<String>,
    pub message: String,
    pub files: Vec<CommitFile>,
}

#[derive(Debug, Clone)]
pub struct CommitFile {
    pub path: String,
    pub content: String,
}

pub trait Scm: Send + Sync {
    fn branch_exists(
        &self,
        branch_name: &str,
    ) -> impl Future<Output = bool> + Send;

    fn commit_and_push(
        &self,
        config: &CommitConfig,
    ) -> impl Future<Output = ScmResult> + Send;

    fn delete_branch(
        &self,
        branch_name: &str,
    ) -> impl Future<Output = ScmResult> + Send;

    fn get_branch_commit(
        &self,
        branch_name: &str,
    ) -> impl Future<Output = Option<String>> + Send;

    fn is_branch_behind_base(
        &self,
        branch_name: &str,
        base_branch: &str,
    ) -> impl Future<Output = bool> + Send;

    fn is_branch_conflicted(
        &self,
        base_branch: &str,
        branch: &str,
    ) -> impl Future<Output = bool> + Send;

    fn is_branch_modified(
        &self,
        branch_name: &str,
        base_branch: &str,
    ) -> impl Future<Output = bool> + Send;

    fn get_file_list(&self) -> impl Future<Output = Vec<String>> + Send;

    fn checkout_branch(
        &self,
        branch_name: &str,
    ) -> impl Future<Output = ScmResult> + Send;

    fn merge_and_push(
        &self,
        branch_name: &str,
    ) -> impl Future<Output = ScmResult> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scm_result_ok_value() {
        let result = ScmResult::Ok("abc123".to_owned());
        assert_eq!(result, ScmResult::Ok("abc123".to_owned()));
    }

    #[test]
    fn scm_result_conflict() {
        let result = ScmResult::Conflict;
        assert_eq!(result, ScmResult::Conflict);
    }

    #[test]
    fn scm_result_not_found() {
        let result = ScmResult::NotFound;
        assert_eq!(result, ScmResult::NotFound);
    }

    #[test]
    fn scm_result_error_message() {
        let result = ScmResult::Error("something went wrong".to_owned());
        match result {
            ScmResult::Error(msg) => assert_eq!(msg, "something went wrong"),
            _ => panic!("expected Error variant"),
        }
    }

    #[test]
    fn commit_config_fields() {
        let config = CommitConfig {
            branch_name: "renovate/deps".into(),
            base_branch: Some("main".into()),
            message: "Update dependencies".into(),
            files: vec![CommitFile {
                path: "Cargo.toml".into(),
                content: "[dependencies]".into(),
            }],
        };
        assert_eq!(config.branch_name, "renovate/deps");
        assert_eq!(config.files.len(), 1);
    }

    #[test]
    fn commit_file_path_and_content() {
        let file = CommitFile {
            path: "src/main.rs".into(),
            content: "fn main() {}".into(),
        };
        assert_eq!(file.path, "src/main.rs");
        assert_eq!(file.content, "fn main() {}");
    }
}
