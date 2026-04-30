//! Jsonnet Bundler `jsonnetfile.json` dependency extractor.
//!
//! Parses `jsonnetfile.json` files and extracts git-sourced dependencies
//! for version tracking via GitHub Tags.
//!
//! Renovate reference:
//! - `lib/modules/manager/jsonnet-bundler/extract.ts`
//! - Pattern: `/(^|/)jsonnetfile\.json$/`
//! - Datasource: git-tags (GitHub Tags for GitHub remotes)
//!
//! ## File format
//!
//! ```json
//! {
//!   "dependencies": [
//!     {
//!       "source": {
//!         "git": {
//!           "remote": "https://github.com/owner/repo.git",
//!           "subdir": "lib"
//!         }
//!       },
//!       "version": "v1.2.3"
//!     }
//!   ]
//! }
//! ```

use serde::Deserialize;

/// A git-sourced dep extracted from `jsonnetfile.json`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonnetDep {
    /// Full git remote URL.
    pub remote: String,
    /// GitHub `owner/repo` form (empty for non-GitHub remotes).
    pub github_repo: String,
    /// Current version tag (e.g. `"v1.2.3"`).
    pub version: String,
}

#[derive(Debug, Deserialize)]
struct JsonnetFile {
    dependencies: Option<Vec<Dependency>>,
}

#[derive(Debug, Deserialize)]
struct Dependency {
    source: Source,
    version: String,
}

#[derive(Debug, Deserialize)]
struct Source {
    git: Option<GitSource>,
}

#[derive(Debug, Deserialize)]
struct GitSource {
    remote: String,
}

/// Parse GitHub `owner/repo` from a remote URL.
fn github_repo(remote: &str) -> String {
    // Handles: https://github.com/owner/repo[.git], git@github.com:owner/repo[.git]
    let path = if let Some(s) = remote
        .strip_prefix("https://github.com/")
        .or_else(|| remote.strip_prefix("http://github.com/"))
    {
        s
    } else if let Some(s) = remote.strip_prefix("git@github.com:") {
        s
    } else {
        return String::new();
    };
    path.trim_end_matches(".git")
        .split('/')
        .take(2)
        .collect::<Vec<_>>()
        .join("/")
}

/// Extract git deps from a `jsonnetfile.json` file.
pub fn extract(content: &str) -> Vec<JsonnetDep> {
    let file: JsonnetFile = match serde_json::from_str(content) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };

    file.dependencies
        .unwrap_or_default()
        .into_iter()
        .filter_map(|dep| {
            let git = dep.source.git?;
            if git.remote.is_empty() || dep.version.is_empty() {
                return None;
            }
            let repo = github_repo(&git.remote);
            Some(JsonnetDep {
                remote: git.remote,
                github_repo: repo,
                version: dep.version,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{
  "dependencies": [
    {
      "source": {
        "git": {
          "remote": "https://github.com/grafana/grafonnet-lib.git",
          "subdir": "grafonnet"
        }
      },
      "version": "v0.0.1"
    },
    {
      "source": {
        "git": {
          "remote": "https://github.com/grafana/jsonnet-libs.git"
        }
      },
      "version": "v1.2.0"
    }
  ]
}"#;

    // Ported: "extracts dependency" — jsonnet-bundler/extract.spec.ts line 57
    #[test]
    fn extracts_github_deps() {
        let deps = extract(SAMPLE);
        assert_eq!(deps.len(), 2);
        let first = &deps[0];
        assert_eq!(first.github_repo, "grafana/grafonnet-lib");
        assert_eq!(first.version, "v0.0.1");
    }

    #[test]
    fn github_repo_strips_git_suffix() {
        assert_eq!(
            github_repo("https://github.com/owner/repo.git"),
            "owner/repo"
        );
        assert_eq!(github_repo("git@github.com:owner/repo.git"), "owner/repo");
    }

    #[test]
    fn non_github_remote_has_empty_repo() {
        assert_eq!(github_repo("https://gitlab.com/user/repo.git"), "");
    }

    // Ported: "returns null for invalid jsonnetfile" — jsonnet-bundler/extract.spec.ts line 24
    #[test]
    fn invalid_json_returns_empty() {
        assert!(extract("not json").is_empty());
    }

    // Ported: "returns null for jsonnetfile with no dependencies" — jsonnet-bundler/extract.spec.ts line 30
    #[test]
    fn empty_returns_empty() {
        assert!(extract("{}").is_empty());
    }

    // Ported: "returns null for dependencies with empty Git source" — jsonnet-bundler/extract.spec.ts line 48
    #[test]
    fn empty_git_source_returns_empty() {
        let content = r#"{"version":1,"dependencies":[{"source":{"git":{}},"version":"v0.50.0"}]}"#;
        assert!(extract(content).is_empty());
    }
}
