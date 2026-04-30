//! Copier `.copier-answers.yml` template dependency extractor.
//!
//! Extracts the template source URL and commit/tag from a Copier answers file.
//!
//! Renovate reference:
//! - `lib/modules/manager/copier/extract.ts`
//! - Pattern: `/(^|/)\.copier-answers(\..+)?\.ya?ml/`
//! - Datasource: git-tags (GitHub Tags for GitHub URLs)
//!
//! ## File format
//!
//! ```yaml
//! _commit: v1.2.3
//! _src_path: https://github.com/owner/template-repo
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// The extracted Copier template dep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopierDep {
    /// Full source URL (e.g. `"https://github.com/owner/template-repo"`).
    pub src_path: String,
    /// GitHub `owner/repo` form for GitHub URLs (empty for non-GitHub).
    pub github_repo: String,
    /// Commit or tag used as the current version (e.g. `"v1.2.3"`).
    pub current_value: String,
}

static SRC_PATH_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"_src_path:\s*(.+)").unwrap());

static COMMIT_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"_commit:\s*(.+)").unwrap());

static GITHUB_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:https?://github\.com/|git@github\.com:)([^/\s.]+/[^/\s.]+?)(?:\.git)?$")
        .unwrap()
});

/// Extract the Copier template dep from a `.copier-answers.yml` file.
///
/// Returns `None` when either `_src_path` or `_commit` is missing.
pub fn extract(content: &str) -> Option<CopierDep> {
    let mut src_path: Option<String> = None;
    let mut commit: Option<String> = None;

    for line in content.lines() {
        let trimmed = line.trim();
        if src_path.is_none()
            && let Some(cap) = SRC_PATH_RE.captures(trimmed)
        {
            src_path = Some(cap[1].trim().to_owned());
        }
        if commit.is_none()
            && let Some(cap) = COMMIT_RE.captures(trimmed)
        {
            commit = Some(cap[1].trim().to_owned());
        }
        if src_path.is_some() && commit.is_some() {
            break;
        }
    }

    let src = src_path?;
    let ver = commit?;
    if src.is_empty() || ver.is_empty() {
        return None;
    }

    // Strip optional `git+` prefix (Copier-specific extension not understood by git).
    let clean_src = src.strip_prefix("git+").unwrap_or(&src);

    let github_repo = GITHUB_RE
        .captures(clean_src)
        .map(|c| c[1].to_owned())
        .unwrap_or_default();

    Some(CopierDep {
        src_path: clean_src.to_owned(),
        github_repo,
        current_value: ver,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "extracts repository and version from .copier-answers.yml" — copier/extract.spec.ts line 6
    #[test]
    fn extracts_github_url() {
        let content = "_commit: v1.2.3\n_src_path: https://github.com/owner/template-repo\n";
        let dep = extract(content).unwrap();
        assert_eq!(dep.current_value, "v1.2.3");
        assert_eq!(dep.github_repo, "owner/template-repo");
        assert_eq!(dep.src_path, "https://github.com/owner/template-repo");
    }

    // Ported: "extracts and strips git+ prefix from $srcPath" — copier/extract.spec.ts line 84
    #[test]
    fn strips_git_plus_prefix() {
        let content = "_commit: v2.0.0\n_src_path: git+https://github.com/myorg/mytemplate\n";
        let dep = extract(content).unwrap();
        assert_eq!(dep.src_path, "https://github.com/myorg/mytemplate");
        assert_eq!(dep.github_repo, "myorg/mytemplate");
    }

    #[test]
    fn non_github_url_has_empty_repo() {
        let content = "_commit: v1.0.0\n_src_path: https://gitlab.com/user/repo\n";
        let dep = extract(content).unwrap();
        assert_eq!(dep.github_repo, "");
        assert_eq!(dep.current_value, "v1.0.0");
    }

    // Ported: "returns null for missing _commit field" — copier/extract.spec.ts line 137
    #[test]
    fn missing_commit_returns_none() {
        let content = "_src_path: https://github.com/owner/repo\n";
        assert!(extract(content).is_none());
    }

    #[test]
    fn empty_returns_none() {
        assert!(extract("").is_none());
    }

    // Ported: "extracts repository and version from .copier-answers.yml with ssh URL" — copier/extract.spec.ts line 25
    #[test]
    fn ssh_url_src_path_extracted() {
        let content =
            "_commit: v1.0.0\n_src_path: git@github.com:renovatebot/somedir/renovate.git\n";
        let dep = extract(content).unwrap();
        assert_eq!(dep.current_value, "v1.0.0");
        assert_eq!(
            dep.src_path,
            "git@github.com:renovatebot/somedir/renovate.git"
        );
    }

    // Ported: "returns null for missing _src_path field" — copier/extract.spec.ts line 145
    #[test]
    fn missing_src_path_returns_none() {
        let content = "_commit: v1.0.0\n";
        assert!(extract(content).is_none());
    }

    // Ported: "returns null for invalid .copier-answers.yml" — copier/extract.spec.ts line 119
    #[test]
    fn invalid_yaml_returns_none() {
        assert!(extract("foo: bar: 123").is_none());
    }
}
