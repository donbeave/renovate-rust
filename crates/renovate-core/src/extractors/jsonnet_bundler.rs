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
    /// Full dep name: `host/path[/subdir]` (e.g. `github.com/owner/repo/lib`).
    pub dep_name: String,
    /// Full git remote URL (package name for lookups).
    pub remote: String,
    /// GitHub `owner/repo` form (empty for non-GitHub remotes).
    pub github_repo: String,
    /// Subdirectory within the repo (may be empty).
    pub subdir: String,
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
    #[serde(default)]
    subdir: String,
}

/// Build the dep name as `host/path[/subdir]` matching TS `upath.join(host, pathname, subdir)`.
fn build_dep_name(remote: &str, subdir: &str) -> String {
    // Normalize: strip scheme and user@ prefix, then strip .git suffix.
    let normalized = remote
        .strip_prefix("https://")
        .or_else(|| remote.strip_prefix("http://"))
        .map(|s| {
            // For ssh://user@host/path
            if let Some(at) = s.find('@') {
                &s[at + 1..]
            } else {
                s
            }
        })
        .or_else(|| {
            // ssh://git@host/path
            remote.strip_prefix("ssh://").map(|s| {
                if let Some(at) = s.find('@') {
                    &s[at + 1..]
                } else {
                    s
                }
            })
        })
        .or_else(|| {
            // git@host:path
            remote.strip_prefix("git@").map(|s| {
                // Can't return &str from closure that mutates — handle below
                let _ = s;
                ""
            })
        })
        .unwrap_or(remote);

    // Special case for git@ SCP form
    let base = if let Some(s) = remote.strip_prefix("git@") {
        // git@github.com:owner/repo.git → github.com/owner/repo
        s.replacen(':', "/", 1)
    } else {
        normalized.to_owned()
    };

    let base = base.trim_end_matches(".git");

    if !subdir.is_empty() {
        format!("{base}/{subdir}")
    } else {
        base.to_owned()
    }
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
            let dep_name = build_dep_name(&git.remote, &git.subdir);
            let gh_repo = github_repo(&git.remote);
            Some(JsonnetDep {
                dep_name,
                remote: git.remote,
                github_repo: gh_repo,
                subdir: git.subdir,
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
        assert_eq!(first.dep_name, "github.com/grafana/grafonnet-lib/grafonnet");
        assert_eq!(first.version, "v0.0.1");
        let second = &deps[1];
        assert_eq!(second.dep_name, "github.com/grafana/jsonnet-libs");
    }

    #[test]
    fn dep_name_includes_subdir() {
        assert_eq!(
            build_dep_name(
                "https://github.com/owner/repo.git",
                "jsonnet/prometheus-operator"
            ),
            "github.com/owner/repo/jsonnet/prometheus-operator"
        );
    }

    #[test]
    fn dep_name_without_subdir() {
        assert_eq!(
            build_dep_name("https://github.com/owner/repo.git", ""),
            "github.com/owner/repo"
        );
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

    // Ported: "returns null for local dependencies" — jsonnet-bundler/extract.spec.ts line 36
    #[test]
    fn local_deps_returns_empty() {
        let content = r#"{"version":1,"dependencies":[{"source":{"local":{"directory":"jsonnet"}},"version":""}]}"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "returns null for dependencies with empty Git source" — jsonnet-bundler/extract.spec.ts line 48
    #[test]
    fn empty_git_source_returns_empty() {
        let content = r#"{"version":1,"dependencies":[{"source":{"git":{}},"version":"v0.50.0"}]}"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "extracts dependency" — jsonnet-bundler/extract.spec.ts line 57
    #[test]
    fn extracts_main_fixture_two_deps() {
        // Mirrors jsonnetfile.json: prometheus-operator + kube-prometheus
        let content = r#"{
  "version": 1,
  "dependencies": [
    {
      "source": {
        "git": {
          "remote": "https://github.com/prometheus-operator/prometheus-operator.git",
          "subdir": "jsonnet/prometheus-operator"
        }
      },
      "version": "v0.50.0"
    },
    {
      "source": {
        "git": {
          "remote": "ssh://git@github.com/prometheus-operator/kube-prometheus.git",
          "subdir": "jsonnet/kube-prometheus"
        }
      },
      "version": "v0.9.0"
    }
  ]
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(
            deps[0].dep_name,
            "github.com/prometheus-operator/prometheus-operator/jsonnet/prometheus-operator"
        );
        assert_eq!(deps[0].version, "v0.50.0");
        assert_eq!(
            deps[1].dep_name,
            "github.com/prometheus-operator/kube-prometheus/jsonnet/kube-prometheus"
        );
        assert_eq!(deps[1].version, "v0.9.0");
    }
}
