//! Local-filesystem platform client.
//!
//! Used when `--platform=local` is passed.  Reads files directly from the
//! working directory instead of making API calls.  File enumeration uses
//! `git ls-files` so that `.gitignore` is respected automatically; falls back
//! to a simple recursive walk (skipping `target/`, `node_modules/`, `.git/`)
//! when the directory is not a git repository.
//!
//! Renovate reference: `lib/modules/platform/local/index.ts`

use std::path::{Path, PathBuf};

use crate::platform::{CombinedBranchStatus, CurrentUser, PlatformClient, PlatformError, RawFile};

/// Platform client that reads from the local filesystem.
#[derive(Debug, Clone)]
pub struct LocalClient {
    base_dir: PathBuf,
}

impl LocalClient {
    pub fn new(base_dir: impl Into<PathBuf>) -> Self {
        Self {
            base_dir: base_dir.into(),
        }
    }

    pub async fn get_current_user(&self) -> Result<CurrentUser, PlatformError> {
        Ok(CurrentUser {
            login: "local".to_owned(),
        })
    }

    pub async fn get_raw_file(
        &self,
        _owner: &str,
        _repo: &str,
        path: &str,
    ) -> Result<Option<RawFile>, PlatformError> {
        let full = self.base_dir.join(path);
        match std::fs::read_to_string(&full) {
            Ok(content) => Ok(Some(RawFile {
                path: path.to_owned(),
                content,
            })),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(PlatformError::Unexpected(format!(
                "reading {}: {e}",
                full.display()
            ))),
        }
    }

    pub async fn get_file_list(
        &self,
        _owner: &str,
        _repo: &str,
    ) -> Result<Vec<String>, PlatformError> {
        // Try git ls-files first; it respects .gitignore automatically.
        if let Some(files) = git_ls_files(&self.base_dir) {
            return Ok(files);
        }
        // Fall back to a simple recursive walk when not in a git repo.
        Ok(walk_dir(&self.base_dir))
    }

    pub async fn create_pr(
        &self,
        _owner: &str,
        _repo: &str,
        _source_branch: &str,
        _target_branch: &str,
        _title: &str,
        _body: &str,
    ) -> Result<Option<i64>, PlatformError> {
        tracing::debug!("local platform: create_pr is a no-op");
        Ok(None)
    }

    pub async fn update_pr(
        &self,
        _owner: &str,
        _repo: &str,
        _pr_number: i64,
        _title: Option<&str>,
        _body: Option<&str>,
        _state: Option<&str>,
    ) -> Result<(), PlatformError> {
        tracing::debug!("local platform: update_pr is a no-op");
        Ok(())
    }

    pub async fn get_branch_status(
        &self,
        _owner: &str,
        _repo: &str,
        _branch: &str,
    ) -> Result<CombinedBranchStatus, PlatformError> {
        use crate::platform::{BranchState, CombinedBranchState, GhBranchStatus};
        Ok(CombinedBranchStatus {
            state: CombinedBranchState::Success,
            statuses: vec![GhBranchStatus {
                context: "local".to_owned(),
                state: BranchState::Success,
            }],
        })
    }

}

/// Run `git ls-files` in `dir` and return the file list, or `None` on failure.
fn git_ls_files(dir: &Path) -> Option<Vec<String>> {
    let output = std::process::Command::new("git")
        .args(["ls-files", "--cached", "--others", "--exclude-standard"])
        .current_dir(dir)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = std::str::from_utf8(&output.stdout).ok()?;
    Some(
        stdout
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.to_owned())
            .collect(),
    )
}

const SKIP_DIRS: &[&str] = &[
    ".git",
    "target",
    "node_modules",
    ".tox",
    "__pycache__",
    "vendor",
];

/// Walk `dir` recursively, returning paths relative to `dir`.
/// Skips directories in [`SKIP_DIRS`] and hidden directories (`.`-prefixed).
fn walk_dir(dir: &Path) -> Vec<String> {
    let mut files = Vec::new();
    walk_dir_inner(dir, dir, &mut files);
    files
}

fn walk_dir_inner(base: &Path, current: &Path, files: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(current) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if name_str.starts_with('.') {
            continue;
        }
        if SKIP_DIRS.contains(&name_str.as_ref()) {
            continue;
        }

        if path.is_dir() {
            walk_dir_inner(base, &path, files);
        } else if let Ok(rel) = path.strip_prefix(base)
            && let Some(s) = rel.to_str()
        {
            files.push(s.to_owned());
        }
    }
}

impl PlatformClient for LocalClient {
    async fn get_current_user(&self) -> Result<CurrentUser, PlatformError> {
        Ok(CurrentUser {
            login: "local".to_owned(),
        })
    }

    async fn get_raw_file(
        &self,
        _owner: &str,
        _repo: &str,
        path: &str,
    ) -> Result<Option<RawFile>, PlatformError> {
        let full = self.base_dir.join(path);
        match std::fs::read_to_string(&full) {
            Ok(content) => Ok(Some(RawFile {
                path: path.to_owned(),
                content,
            })),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(PlatformError::Unexpected(format!(
                "reading {}: {e}",
                full.display()
            ))),
        }
    }

    async fn get_file_list(
        &self,
        _owner: &str,
        _repo: &str,
    ) -> Result<Vec<String>, PlatformError> {
        // Try git ls-files first; it respects .gitignore automatically.
        if let Some(files) = git_ls_files(&self.base_dir) {
            return Ok(files);
        }
        // Fall back to a simple recursive walk when not in a git repo.
        Ok(walk_dir(&self.base_dir))
    }

    async fn create_pr(
        &self,
        _owner: &str,
        _repo: &str,
        _source_branch: &str,
        _target_branch: &str,
        _title: &str,
        _body: &str,
    ) -> Result<Option<i64>, PlatformError> {
        Err(PlatformError::NotSupported("Local platform PR creation".to_string()))
    }

    async fn update_pr(
        &self,
        _owner: &str,
        _repo: &str,
        _pr_number: i64,
        _title: Option<&str>,
        _body: Option<&str>,
        _state: Option<&str>,
    ) -> Result<(), PlatformError> {
        Err(PlatformError::NotSupported("Local platform PR updates".to_string()))
    }

    async fn get_branch_status(
        &self,
        _owner: &str,
        _repo: &str,
        _branch: &str,
    ) -> Result<CombinedBranchStatus, PlatformError> {
        Err(PlatformError::NotSupported("Local platform branch status".to_string()))
    }

    async fn write_file(
        &self,
        _owner: &str,
        _repo: &str,
        path: &str,
        content: &str,
    ) -> Result<(), PlatformError> {
        let full = self.base_dir.join(path);
        if let Some(parent) = full.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                PlatformError::Unexpected(format!("creating dir {}: {e}", parent.display()))
            })?;
        }
        std::fs::write(&full, content).map_err(|e| {
            PlatformError::Unexpected(format!("writing {}: {e}", full.display()))
        })
    }
}
