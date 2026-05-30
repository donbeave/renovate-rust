use anyhow::{Context, Result, bail};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub const RENOVATE_FORK_UPSTREAM: &str = "renovate-fork-upstream";
pub const GIT_MINIMUM_VERSION: &str = "2.33.0";

pub type LongCommitSha = String;

#[derive(Debug, Clone)]
pub struct GitAuthor {
    pub name: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GitNoVerifyOption {
    Commit,
    Push,
}

#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub current_branch: Option<String>,
    pub default_branch: Option<String>,
    pub url: String,
    pub upstream_url: Option<String>,
    pub extra_clone_opts: HashMap<String, String>,
    pub clone_submodules: bool,
    pub clone_submodules_filter: Option<Vec<String>>,
    pub full_clone: bool,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            current_branch: None,
            default_branch: None,
            url: String::new(),
            upstream_url: None,
            extra_clone_opts: HashMap::new(),
            clone_submodules: false,
            clone_submodules_filter: None,
            full_clone: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum FileChange {
    Addition {
        path: String,
        contents: Vec<u8>,
        is_executable: bool,
    },
    Deletion {
        path: String,
    },
}

#[derive(Debug, Clone)]
pub struct CommitFilesConfig {
    pub base_branch: Option<String>,
    pub branch_name: String,
    pub files: Vec<FileChange>,
    pub message: String,
    pub force: bool,
}

#[derive(Debug, Clone)]
pub struct CommitResult {
    pub parent_commit_sha: LongCommitSha,
    pub commit_sha: LongCommitSha,
}

#[derive(Debug)]
pub struct GitStorage {
    workdir: PathBuf,
    git_dir: PathBuf,
    config: StorageConfig,
    branch_commits: HashMap<String, LongCommitSha>,
    current_branch: String,
    current_branch_sha: LongCommitSha,
    ignored_authors: Vec<String>,
    git_author_name: Option<String>,
    git_author_email: Option<String>,
    no_verify: Vec<GitNoVerifyOption>,
}

impl GitStorage {
    pub fn new(workdir: PathBuf) -> Self {
        let git_dir = workdir.join(".git");
        Self {
            workdir,
            git_dir,
            config: StorageConfig::default(),
            branch_commits: HashMap::new(),
            current_branch: String::new(),
            current_branch_sha: String::new(),
            ignored_authors: Vec::new(),
            git_author_name: None,
            git_author_email: None,
            no_verify: Vec::new(),
        }
    }

    pub fn is_cloned(&self) -> bool {
        self.git_dir.exists()
    }

    pub fn set_no_verify(&mut self, value: Vec<GitNoVerifyOption>) {
        self.no_verify = value;
    }

    pub fn get_no_verify(&self) -> &[GitNoVerifyOption] {
        &self.no_verify
    }

    pub fn set_ignored_authors(&mut self, authors: Vec<String>) {
        self.ignored_authors = authors;
    }

    pub fn set_git_author(&mut self, name: Option<String>, email: Option<String>) {
        self.git_author_name = name;
        self.git_author_email = email;
    }

    pub fn workdir(&self) -> &Path {
        &self.workdir
    }

    pub fn current_branch(&self) -> &str {
        &self.current_branch
    }

    pub fn config(&self) -> &StorageConfig {
        &self.config
    }

    pub fn branch_exists(&self, branch_name: &str) -> bool {
        self.branch_commits.contains_key(branch_name)
    }

    pub fn get_branch_commit(&self, branch_name: &str) -> Option<&LongCommitSha> {
        self.branch_commits.get(branch_name)
    }

    pub fn get_branch_list(&self) -> Vec<String> {
        self.branch_commits.keys().cloned().collect()
    }

    pub async fn init_repo(&mut self, config: StorageConfig) -> Result<()> {
        self.config = config.clone();
        if !self.is_cloned() {
            self.clone_repo().await?;
        } else {
            self.fetch_all().await?;
        }
        self.current_branch = config
            .current_branch
            .or(config.default_branch)
            .unwrap_or_else(|| "main".to_owned());
        self.read_current_sha().await?;
        Ok(())
    }

    async fn clone_repo(&mut self) -> Result<()> {
        let url = &self.config.url;
        std::fs::create_dir_all(&self.workdir)
            .with_context(|| format!("creating workdir {}", self.workdir.display()))?;
        let mut cmd = tokio::process::Command::new("git");
        cmd.arg("clone").arg("--no-tags").arg("--origin").arg("origin");
        if !self.config.full_clone {
            cmd.arg("--depth=2");
        }
        cmd.arg(url).arg(&self.workdir);
        let status = cmd.status().await.context("spawning git clone")?;
        if !status.success() {
            bail!("git clone failed for {}", url);
        }
        if let Some(ref upstream) = self.config.upstream_url {
            let status = tokio::process::Command::new("git")
                .arg("remote").arg("add").arg(RENOVATE_FORK_UPSTREAM).arg(upstream)
                .current_dir(&self.workdir)
                .status().await.context("adding upstream remote")?;
            if !status.success() {
                bail!("failed to add upstream remote {}", upstream);
            }
        }
        Ok(())
    }

    async fn fetch_all(&mut self) -> Result<()> {
        let status = tokio::process::Command::new("git")
            .arg("fetch").arg("--prune").arg("--no-tags")
            .arg("origin").arg("+refs/heads/*:refs/remotes/origin/*")
            .arg("--depth=2")
            .current_dir(&self.workdir)
            .status().await.context("git fetch")?;
        if !status.success() {
            bail!("git fetch failed");
        }
        Ok(())
    }

    async fn read_current_sha(&mut self) -> Result<()> {
        let output = tokio::process::Command::new("git")
            .arg("rev-parse").arg(&self.current_branch)
            .current_dir(&self.workdir)
            .output().await.context("git rev-parse")?;
        if output.status.success() {
            let sha = String::from_utf8_lossy(&output.stdout).trim().to_owned();
            self.current_branch_sha = sha.clone();
            self.branch_commits.insert(self.current_branch.clone(), sha);
        }
        Ok(())
    }

    pub async fn sync_git(&mut self) -> Result<()> {
        self.fetch_all().await?;
        self.read_current_sha().await?;
        Ok(())
    }

    pub async fn checkout_branch(&mut self, branch_name: &str) -> Result<LongCommitSha> {
        let status = tokio::process::Command::new("git")
            .arg("checkout").arg("--force").arg(branch_name)
            .current_dir(&self.workdir)
            .status().await.context("git checkout")?;
        if !status.success() {
            bail!("git checkout {} failed", branch_name);
        }
        let sha = self.get_commit_sha(branch_name).await?;
        self.branch_commits.insert(branch_name.to_owned(), sha.clone());
        Ok(sha)
    }

    pub async fn checkout_branch_from_remote(
        &mut self, branch_name: &str, remote_name: &str,
    ) -> Result<LongCommitSha> {
        let remote_branch = format!("{}/{}", remote_name, branch_name);
        let status = tokio::process::Command::new("git")
            .arg("checkout").arg("-b").arg(branch_name).arg(&remote_branch)
            .current_dir(&self.workdir)
            .status().await.context("git checkout -b from remote")?;
        if !status.success() {
            bail!("git checkout -b {} {} failed", branch_name, remote_branch);
        }
        let sha = self.get_commit_sha(branch_name).await?;
        self.branch_commits.insert(branch_name.to_owned(), sha.clone());
        Ok(sha)
    }

    pub async fn get_commit_sha(&self, refname: &str) -> Result<LongCommitSha> {
        let output = tokio::process::Command::new("git")
            .arg("rev-parse").arg(refname)
            .current_dir(&self.workdir)
            .output().await.context("git rev-parse")?;
        if !output.status.success() {
            bail!("git rev-parse {} failed", refname);
        }
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    }

    pub async fn get_file(&self, file_path: &str, branch_name: Option<&str>) -> Result<Option<String>> {
        let mut cmd = tokio::process::Command::new("git");
        cmd.arg("show").current_dir(&self.workdir);
        match branch_name {
            Some(branch) => { cmd.arg(format!("{}:{}", branch, file_path)); }
            None => { cmd.arg(format!("HEAD:{}", file_path)); }
        }
        let output = cmd.output().await.context("git show")?;
        if output.status.success() {
            Ok(Some(String::from_utf8_lossy(&output.stdout).to_string()))
        } else {
            Ok(None)
        }
    }

    pub async fn get_file_list(&self) -> Result<Vec<String>> {
        let output = tokio::process::Command::new("git")
            .arg("ls-files").current_dir(&self.workdir)
            .output().await.context("git ls-files")?;
        if !output.status.success() {
            bail!("git ls-files failed");
        }
        Ok(String::from_utf8_lossy(&output.stdout).lines().map(|l| l.to_owned()).collect())
    }

    pub async fn is_branch_behind_base(&self, branch_name: &str, base_branch: &str) -> Result<bool> {
        let branch_sha = self.get_commit_sha(branch_name).await?;
        let base_sha = self.get_commit_sha(base_branch).await?;
        let output = tokio::process::Command::new("git")
            .arg("merge-base").arg("--is-ancestor").arg(&branch_sha).arg(&base_sha)
            .current_dir(&self.workdir)
            .output().await.context("git merge-base --is-ancestor")?;
        Ok(output.status.success())
    }

    pub async fn is_branch_modified(&self, branch_name: &str, base_branch: &str) -> Result<bool> {
        let output = tokio::process::Command::new("git")
            .arg("log").arg(format!("{}..{}", base_branch, branch_name))
            .arg("--format=%H").arg("--author").arg("^Renovate")
            .current_dir(&self.workdir)
            .output().await.context("git log for modified check")?;
        Ok(!String::from_utf8_lossy(&output.stdout).trim().is_empty())
    }

    pub async fn is_branch_conflicted(&self, base_branch: &str, branch: &str) -> Result<bool> {
        let status = tokio::process::Command::new("git")
            .arg("merge-tree").arg(base_branch).arg(branch)
            .current_dir(&self.workdir)
            .status().await.context("git merge-tree")?;
        Ok(!status.success())
    }

    pub async fn delete_branch(&self, branch_name: &str) -> Result<()> {
        let _ = tokio::process::Command::new("git")
            .arg("branch").arg("-D").arg(branch_name)
            .current_dir(&self.workdir).status().await;
        tokio::process::Command::new("git")
            .arg("push").arg("origin").arg("--delete").arg(branch_name)
            .current_dir(&self.workdir)
            .status().await.context("git push --delete")?;
        Ok(())
    }

    pub async fn merge_branch(&self, branch_name: &str) -> Result<()> {
        let status = tokio::process::Command::new("git")
            .arg("merge").arg("--no-ff").arg(branch_name)
            .current_dir(&self.workdir)
            .status().await.context("git merge")?;
        if !status.success() {
            bail!("git merge {} failed", branch_name);
        }
        Ok(())
    }

    pub async fn commit_files(&self, config: &CommitFilesConfig) -> Result<Option<LongCommitSha>> {
        for file_change in &config.files {
            match file_change {
                FileChange::Addition { path, contents, .. } => {
                    let full_path = self.workdir.join(path);
                    if let Some(parent) = full_path.parent() {
                        std::fs::create_dir_all(parent)
                            .with_context(|| format!("creating directory {}", parent.display()))?;
                    }
                    std::fs::write(&full_path, contents)
                        .with_context(|| format!("writing file {}", full_path.display()))?;
                    tokio::process::Command::new("git")
                        .arg("add").arg(path).current_dir(&self.workdir)
                        .status().await.context("git add")?;
                }
                FileChange::Deletion { path } => {
                    tokio::process::Command::new("git")
                        .arg("rm").arg("-f").arg(path).current_dir(&self.workdir)
                        .status().await.context("git rm")?;
                }
            }
        }
        let mut cmd = tokio::process::Command::new("git");
        cmd.arg("commit").arg("-m").arg(&config.message).current_dir(&self.workdir);
        if self.no_verify.contains(&GitNoVerifyOption::Commit) {
            cmd.arg("--no-verify");
        }
        if config.force {
            cmd.arg("--allow-empty");
        }
        let status = cmd.status().await.context("git commit")?;
        if !status.success() {
            return Ok(None);
        }
        let sha = self.get_commit_sha("HEAD").await?;
        Ok(Some(sha))
    }

    pub async fn push_commit(&self, branch_name: &str) -> Result<bool> {
        let mut cmd = tokio::process::Command::new("git");
        cmd.arg("push").arg("origin").arg(branch_name).current_dir(&self.workdir);
        if self.no_verify.contains(&GitNoVerifyOption::Push) {
            cmd.arg("--no-verify");
        }
        let status = cmd.status().await.context("git push")?;
        Ok(status.success())
    }

    pub async fn fetch_branch(&self, branch_name: &str) -> Result<Option<LongCommitSha>> {
        let status = tokio::process::Command::new("git")
            .arg("fetch").arg("origin").arg(format!("{}:{}", branch_name, branch_name))
            .current_dir(&self.workdir)
            .status().await.context("git fetch branch")?;
        if !status.success() {
            return Ok(None);
        }
        let sha = self.get_commit_sha(branch_name).await?;
        Ok(Some(sha))
    }

    pub async fn get_commit_messages(&self) -> Result<Vec<String>> {
        let output = tokio::process::Command::new("git")
            .arg("log").arg("-n").arg("10").arg("--format=%s")
            .current_dir(&self.workdir)
            .output().await.context("git log")?;
        Ok(String::from_utf8_lossy(&output.stdout).lines().map(|l| l.to_owned()).collect())
    }

    pub async fn get_repo_status(&self) -> Result<String> {
        let output = tokio::process::Command::new("git")
            .arg("status").arg("--porcelain").current_dir(&self.workdir)
            .output().await.context("git status")?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub async fn has_diff(&self, source_ref: &str, target_ref: &str) -> Result<bool> {
        let output = tokio::process::Command::new("git")
            .arg("diff").arg("--quiet").arg(source_ref).arg(target_ref)
            .current_dir(&self.workdir)
            .output().await.context("git diff")?;
        Ok(!output.status.success())
    }

    pub async fn reset_to_commit(&self, commit: &str) -> Result<()> {
        let status = tokio::process::Command::new("git")
            .arg("reset").arg("--hard").arg(commit)
            .current_dir(&self.workdir)
            .status().await.context("git reset --hard")?;
        if !status.success() {
            bail!("git reset --hard {} failed", commit);
        }
        Ok(())
    }

    pub async fn set_config(&self, key: &str, value: &str) -> Result<()> {
        let status = tokio::process::Command::new("git")
            .arg("config").arg(key).arg(value).current_dir(&self.workdir)
            .status().await.context("git config")?;
        if !status.success() {
            bail!("git config {} {} failed", key, value);
        }
        Ok(())
    }

    pub async fn write_git_author(&self) -> Result<()> {
        if let Some(ref name) = self.git_author_name {
            self.set_config("user.name", name).await?;
        }
        if let Some(ref email) = self.git_author_email {
            self.set_config("user.email", email).await?;
        }
        Ok(())
    }
}

pub fn parse_git_author_storage(input: &str) -> Option<GitAuthor> {
    let input = input.trim();
    if input.is_empty() {
        return None;
    }
    if let Some((name, email)) = input.split_once('<') {
        let name = name.trim().trim_end_matches(' ').to_owned();
        let email = email.trim_end_matches('>').trim().to_owned();
        Some(GitAuthor {
            name: if name.is_empty() { None } else { Some(name) },
            address: if email.is_empty() { None } else { Some(email) },
        })
    } else {
        Some(GitAuthor { name: Some(input.to_owned()), address: None })
    }
}

pub fn get_url(
    protocol: Option<&str>, auth: Option<&str>,
    hostname: Option<&str>, host: Option<&str>, repository: &str,
) -> String {
    let proto = protocol.unwrap_or("https");
    let auth_part = match auth {
        Some(a) => format!("{}@", a),
        None => String::new(),
    };
    let host_part = host.unwrap_or_else(|| hostname.unwrap_or("github.com"));
    format!("{}://{}{}/{}.git", proto, auth_part, host_part, repository)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_git_author_name_and_email() {
        let author = parse_git_author_storage("John Doe <john@example.com>").unwrap();
        assert_eq!(author.name.as_deref(), Some("John Doe"));
        assert_eq!(author.address.as_deref(), Some("john@example.com"));
    }

    #[test]
    fn parse_git_author_email_only() {
        let author = parse_git_author_storage("<john@example.com>").unwrap();
        assert_eq!(author.name, None);
        assert_eq!(author.address.as_deref(), Some("john@example.com"));
    }

    #[test]
    fn parse_git_author_empty() {
        assert!(parse_git_author_storage("").is_none());
    }

    #[test]
    fn get_url_default() {
        let url = get_url(None, None, None, None, "owner/repo");
        assert_eq!(url, "https://github.com/owner/repo.git");
    }

    #[test]
    fn get_url_with_auth() {
        let url = get_url(Some("https"), Some("token"), Some("gitlab.com"), None, "owner/repo");
        assert_eq!(url, "https://token@gitlab.com/owner/repo.git");
    }

    #[test]
    fn storage_config_default() {
        let config = StorageConfig::default();
        assert!(config.current_branch.is_none());
        assert!(config.default_branch.is_none());
        assert!(config.url.is_empty());
        assert!(config.upstream_url.is_none());
        assert!(!config.clone_submodules);
        assert!(config.clone_submodules_filter.is_none());
        assert!(!config.full_clone);
    }

    #[test]
    fn git_storage_new() {
        let dir = tempfile::tempdir().unwrap();
        let storage = GitStorage::new(dir.path().to_owned());
        assert_eq!(storage.workdir(), dir.path());
        assert!(!storage.is_cloned());
        assert!(storage.get_no_verify().is_empty());
    }

    #[test]
    fn git_storage_set_no_verify() {
        let dir = tempfile::tempdir().unwrap();
        let mut storage = GitStorage::new(dir.path().to_owned());
        storage.set_no_verify(vec![GitNoVerifyOption::Commit, GitNoVerifyOption::Push]);
        assert_eq!(storage.get_no_verify().len(), 2);
        assert!(storage.get_no_verify().contains(&GitNoVerifyOption::Commit));
        assert!(storage.get_no_verify().contains(&GitNoVerifyOption::Push));
    }
}
