//! Default SCM implementation using git CLI.
//!
//! Provides a git-CLI-based SCM that works for most platforms.
//!
//! Renovate reference: `lib/modules/platform/default-scm.ts`

use super::scm::{CommitConfig, Scm, ScmResult};

#[derive(Debug, Clone)]
pub struct DefaultScm {
    pub repo_dir: String,
}

impl DefaultScm {
    pub fn new(repo_dir: impl Into<String>) -> Self {
        Self {
            repo_dir: repo_dir.into(),
        }
    }
}

impl Scm for DefaultScm {
    async fn branch_exists(&self, branch_name: &str) -> bool {
        let output = tokio::process::Command::new("git")
            .args(["rev-parse", "--verify", branch_name])
            .current_dir(&self.repo_dir)
            .output()
            .await;
        match output {
            Ok(out) => out.status.success(),
            Err(_) => false,
        }
    }

    async fn commit_and_push(&self, config: &CommitConfig) -> ScmResult {
        for file in &config.files {
            let full_path = if self.repo_dir.is_empty() {
                file.path.clone()
            } else {
                format!("{}/{}", self.repo_dir.trim_end_matches('/'), file.path)
            };
            if let Some(parent) = std::path::Path::new(&full_path).parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Err(e) = std::fs::write(&full_path, &file.content) {
                return ScmResult::Error(format!("Failed to write file {}: {}", file.path, e));
            }

            let add_result = tokio::process::Command::new("git")
                .args(["add", &file.path])
                .current_dir(&self.repo_dir)
                .output()
                .await;
            if let Some(err) = add_result.err() {
                return ScmResult::Error(format!("git add failed: {}", err));
            }
        }

        let commit_result = tokio::process::Command::new("git")
            .args(["commit", "-m", &config.message])
            .current_dir(&self.repo_dir)
            .output()
            .await;
        match commit_result {
            Ok(out) if out.status.success() => {
                let hash_output = tokio::process::Command::new("git")
                    .args(["rev-parse", "HEAD"])
                    .current_dir(&self.repo_dir)
                    .output()
                    .await;
                match hash_output {
                    Ok(h) => {
                        let hash = String::from_utf8_lossy(&h.stdout).trim().to_owned();
                        ScmResult::Ok(hash)
                    }
                    Err(_) => ScmResult::Ok("unknown".to_owned()),
                }
            }
            Ok(out) => ScmResult::Error(String::from_utf8_lossy(&out.stderr).to_string()),
            Err(e) => ScmResult::Error(format!("git commit failed: {}", e)),
        }
    }

    async fn delete_branch(&self, branch_name: &str) -> ScmResult {
        let result = tokio::process::Command::new("git")
            .args(["branch", "-D", branch_name])
            .current_dir(&self.repo_dir)
            .output()
            .await;
        match result {
            Ok(out) if out.status.success() => ScmResult::Ok(String::new()),
            Ok(out) => ScmResult::Error(String::from_utf8_lossy(&out.stderr).to_string()),
            Err(e) => ScmResult::Error(e.to_string()),
        }
    }

    async fn get_branch_commit(&self, branch_name: &str) -> Option<String> {
        let output = tokio::process::Command::new("git")
            .args(["rev-parse", branch_name])
            .current_dir(&self.repo_dir)
            .output()
            .await
            .ok()?;
        if output.status.success() {
            Some(String::from_utf8_lossy(&output.stdout).trim().to_owned())
        } else {
            None
        }
    }

    async fn is_branch_behind_base(&self, branch_name: &str, base_branch: &str) -> bool {
        let output = tokio::process::Command::new("git")
            .args([
                "log",
                "--oneline",
                &format!("{}..{}", branch_name, base_branch),
            ])
            .current_dir(&self.repo_dir)
            .output()
            .await;
        match output {
            Ok(out) if out.status.success() => {
                !String::from_utf8_lossy(&out.stdout).trim().is_empty()
            }
            _ => false,
        }
    }

    async fn is_branch_conflicted(&self, _base_branch: &str, _branch: &str) -> bool {
        false
    }

    async fn is_branch_modified(&self, branch_name: &str, base_branch: &str) -> bool {
        let output = tokio::process::Command::new("git")
            .args(["diff", "--quiet", base_branch, branch_name])
            .current_dir(&self.repo_dir)
            .output()
            .await;
        match output {
            Ok(out) => !out.status.success(),
            Err(_) => false,
        }
    }

    async fn get_file_list(&self) -> Vec<String> {
        let output = tokio::process::Command::new("git")
            .args(["ls-files"])
            .current_dir(&self.repo_dir)
            .output()
            .await;
        match output {
            Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout)
                .lines()
                .map(|s| s.to_owned())
                .collect(),
            _ => Vec::new(),
        }
    }

    async fn checkout_branch(&self, branch_name: &str) -> ScmResult {
        let result = tokio::process::Command::new("git")
            .args(["checkout", branch_name])
            .current_dir(&self.repo_dir)
            .output()
            .await;
        match result {
            Ok(out) if out.status.success() => {
                let hash = self.get_branch_commit(branch_name).await;
                ScmResult::Ok(hash.unwrap_or_default())
            }
            Ok(out) => ScmResult::Error(String::from_utf8_lossy(&out.stderr).to_string()),
            Err(e) => ScmResult::Error(e.to_string()),
        }
    }

    async fn merge_and_push(&self, branch_name: &str) -> ScmResult {
        let result = tokio::process::Command::new("git")
            .args(["merge", branch_name])
            .current_dir(&self.repo_dir)
            .output()
            .await;
        match result {
            Ok(out) if out.status.success() => ScmResult::Ok(String::new()),
            Ok(out) => ScmResult::Error(String::from_utf8_lossy(&out.stderr).to_string()),
            Err(e) => ScmResult::Error(e.to_string()),
        }
    }

    async fn get_branch_update_date(&self, _branch_name: &str) -> Option<String> {
        None
    }

    async fn merge_branch(&self, _branch_name: &str) -> ScmResult {
        ScmResult::Ok(String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_scm_new() {
        let scm = DefaultScm::new("/tmp/test-repo");
        assert_eq!(scm.repo_dir, "/tmp/test-repo");
    }

    #[test]
    #[allow(clippy::redundant_clone)]
    fn default_scm_clone() {
        let scm = DefaultScm::new("/tmp/test-repo");
        let cloned = scm.clone();
        assert_eq!(cloned.repo_dir, "/tmp/test-repo");
    }

    #[test]
    fn commit_config_default_values() {
        let config = CommitConfig {
            branch_name: "test".into(),
            base_branch: None,
            message: "test commit".into(),
            files: vec![],
        };
        assert!(config.base_branch.is_none());
        assert!(config.files.is_empty());
    }
}
