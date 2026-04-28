//! Buildkite pipeline YAML plugin extractor.
//!
//! Parses Buildkite pipeline files and extracts plugin references of the form
//! `plugin-name#version` for GitHub Tags version lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/buildkite/extract.ts`
//! - Patterns: `/buildkite\.ya?ml/`, `/\.buildkite/.+\.ya?ml$/`
//!
//! ## Plugin name forms
//!
//! | Form | GitHub repo | Notes |
//! |---|---|---|
//! | `buildkite/matrix-joiner#v1.0` | `buildkite/matrix-joiner-buildkite-plugin` | 2-part shorthand |
//! | `my-plugin#v2.0` | `buildkite-plugins/my-plugin-buildkite-plugin` | 1-part shorthand |
//! | `https://github.com/owner/repo#v1.0` | `owner/repo` | Full URL |
//! | `ssh://git@github.com/owner/repo.git#v1.0` | `owner/repo` | SSH URL |
//!
//! Lines that don't match `depName#version` are ignored.  Non-semver versions
//! are skipped (`InvalidVersion` reason).

use std::sync::LazyLock;

use regex::Regex;

/// Why a Buildkite plugin dep is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildkiteSkipReason {
    /// `currentValue` is not a semver-like version string.
    InvalidVersion,
    /// Plugin name has more than 2 path components (unusual/unsupported form).
    InvalidName,
}

/// Datasource for a Buildkite plugin (only GitHub Tags supported for now).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildkiteDatasource {
    GithubTags { repo: String },
}

/// A single extracted Buildkite plugin dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildkiteDep {
    /// Display name (the raw `depName` part before `#`).
    pub dep_name: String,
    /// Version string (the raw `currentValue` part after `#`).
    pub current_value: String,
    pub datasource: Option<BuildkiteDatasource>,
    pub skip_reason: Option<BuildkiteSkipReason>,
}

// ── Compiled regexes ─────────────────────────────────────────────────────────

/// Matches a line containing `[prefix ]plugin-name#version[suffix]`.
static PLUGIN_LINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^\s*(?:-\s+(?:\?\s+)?)?['"]?(?P<dep>[^#\s'"]+)#(?P<ver>[^:'"]+)['"]?"#).unwrap()
});

/// Matches full-URL plugins: `(https://|ssh://git@)host/owner/repo[.git]#version`.
static GIT_URL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:ssh://git@|https://)(?P<host>[^/]+)/(?P<repo>[^#]+?)(?:\.git)?$").unwrap()
});

/// Loose semver check: `v1.2.3`, `1.2.3`, `v1.2.3-alpha`.
static SEMVER_LIKE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^v?\d+\.\d+").unwrap());

/// Extract Buildkite plugin dependencies from a pipeline YAML.
pub fn extract(content: &str) -> Vec<BuildkiteDep> {
    let mut out = Vec::new();

    for raw in content.lines() {
        // Strip inline comments.
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();

        let Some(cap) = PLUGIN_LINE.captures(line) else {
            continue;
        };

        let dep_name = cap["dep"].to_owned();
        let current_value = cap["ver"].trim().to_owned();

        // Handle full-URL plugins (git@ or https://).
        if let Some(url_cap) = GIT_URL.captures(&dep_name) {
            let host = &url_cap["host"];
            let repo_path = url_cap["repo"].trim_start_matches('/').to_owned();
            // Only GitHub is currently supported (Bitbucket Tags not yet implemented).
            if host == "github.com" {
                out.push(BuildkiteDep {
                    dep_name,
                    current_value,
                    datasource: Some(BuildkiteDatasource::GithubTags { repo: repo_path }),
                    skip_reason: None,
                });
            } else {
                out.push(BuildkiteDep {
                    dep_name,
                    current_value,
                    datasource: None,
                    skip_reason: Some(BuildkiteSkipReason::InvalidName),
                });
            }
            continue;
        }

        // Shorthand form: `[namespace/]name#version`.
        if !SEMVER_LIKE.is_match(&current_value) {
            out.push(BuildkiteDep {
                dep_name,
                current_value,
                datasource: None,
                skip_reason: Some(BuildkiteSkipReason::InvalidVersion),
            });
            continue;
        }

        let parts: Vec<&str> = dep_name.split('/').collect();
        let (repo, skip_reason) = match parts.len() {
            1 => (
                Some(format!("buildkite-plugins/{}-buildkite-plugin", parts[0])),
                None,
            ),
            2 => (Some(format!("{}-buildkite-plugin", dep_name)), None),
            _ => (None, Some(BuildkiteSkipReason::InvalidName)),
        };

        out.push(BuildkiteDep {
            dep_name,
            current_value,
            datasource: repo.map(|r| BuildkiteDatasource::GithubTags { repo: r }),
            skip_reason,
        });
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
steps:
  - label: "Build"
    plugins:
      - docker-compose#v5.1.0:
          run: app
      - buildkite/matrix-joiner#v1.0.0: ~
      - artifacts#v1.9.3:
          upload: "logs/**/*"

  - label: "Test"
    plugins:
      - https://github.com/my-org/my-plugin.git#v2.3.0:
          config: value

  - label: "Deploy"
    plugins:
      - cache#some-branch: ~
"#;

    #[test]
    fn one_part_plugin() {
        let deps = extract(SAMPLE);
        let dcp = deps
            .iter()
            .find(|d| d.dep_name == "docker-compose")
            .unwrap();
        assert_eq!(dcp.current_value, "v5.1.0");
        assert_eq!(
            dcp.datasource,
            Some(BuildkiteDatasource::GithubTags {
                repo: "buildkite-plugins/docker-compose-buildkite-plugin".to_owned()
            })
        );
        assert!(dcp.skip_reason.is_none());
    }

    #[test]
    fn two_part_plugin() {
        let deps = extract(SAMPLE);
        let mj = deps
            .iter()
            .find(|d| d.dep_name == "buildkite/matrix-joiner")
            .unwrap();
        assert_eq!(mj.current_value, "v1.0.0");
        assert_eq!(
            mj.datasource,
            Some(BuildkiteDatasource::GithubTags {
                repo: "buildkite/matrix-joiner-buildkite-plugin".to_owned()
            })
        );
    }

    #[test]
    fn github_url_plugin() {
        let deps = extract(SAMPLE);
        let url = deps
            .iter()
            .find(|d| d.dep_name.contains("github.com"))
            .unwrap();
        assert_eq!(url.current_value, "v2.3.0");
        assert_eq!(
            url.datasource,
            Some(BuildkiteDatasource::GithubTags {
                repo: "my-org/my-plugin".to_owned()
            })
        );
    }

    #[test]
    fn non_semver_version_skipped() {
        let deps = extract(SAMPLE);
        let cache = deps.iter().find(|d| d.dep_name == "cache").unwrap();
        assert_eq!(cache.skip_reason, Some(BuildkiteSkipReason::InvalidVersion));
    }

    #[test]
    fn empty_content_returns_no_deps() {
        assert!(extract("").is_empty());
    }
}
