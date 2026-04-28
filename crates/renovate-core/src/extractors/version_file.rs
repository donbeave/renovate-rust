//! Single-version-file dependency extractor.
//!
//! Handles files that contain a single tool version string on one line
//! (possibly with a `v` prefix or other decoration). Each file maps to one
//! dependency routed to a GitHub Tags or GitHub Releases datasource.
//!
//! ## Supported files
//!
//! | File | Tool | Datasource | Tag format |
//! |---|---|---|---|
//! | `.terraform-version` | terraform | GitHub Releases | `v{version}` |
//! | `.terragrunt-version` | terragrunt | GitHub Releases | `v{version}` |
//! | `.go-version` | golang | GitHub Tags | `go{version}` |
//! | `.python-version` | python | GitHub Tags | `v{version}` |
//! | `.node-version` | nodejs | GitHub Releases | `v{version}` |
//! | `.nvmrc` | nodejs | GitHub Releases | `v{version}` |
//! | `.bun-version` | bun | GitHub Releases | `bun-v{version}` |
//!
//! Renovate references:
//! - `lib/modules/manager/terraform-version/`
//! - `lib/modules/manager/runtime-version/`

use crate::extractors::asdf::AsdfDatasource;

/// A single dependency extracted from a version file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionFileDep {
    /// Logical tool name (e.g. `"terraform"`).
    pub tool: &'static str,
    /// Version string read from the file (trimmed, comments stripped).
    pub current_value: String,
    /// Datasource routing.
    pub datasource: AsdfDatasource,
}

/// Static table: file manager name → tool definition.
///
/// The manager name is the key used in `managers.rs` (not the filename).
static VERSION_FILE_DEFS: &[(&str, &str, AsdfDatasource)] = &[
    (
        "terraform-version",
        "terraform",
        AsdfDatasource::GithubReleases {
            repo: "hashicorp/terraform",
            tag_strip: "v",
        },
    ),
    (
        "terragrunt-version",
        "terragrunt",
        AsdfDatasource::GithubReleases {
            repo: "gruntwork-io/terragrunt",
            tag_strip: "v",
        },
    ),
    (
        "go-version",
        "golang",
        AsdfDatasource::GithubTags {
            repo: "golang/go",
            tag_strip: "go",
        },
    ),
    (
        "python-version",
        "python",
        AsdfDatasource::GithubTags {
            repo: "python/cpython",
            tag_strip: "v",
        },
    ),
    (
        "node-version",
        "nodejs",
        AsdfDatasource::GithubReleases {
            repo: "nodejs/node",
            tag_strip: "v",
        },
    ),
    (
        "nvmrc",
        "nodejs",
        AsdfDatasource::GithubReleases {
            repo: "nodejs/node",
            tag_strip: "v",
        },
    ),
    (
        "bun-version",
        "bun",
        AsdfDatasource::GithubReleases {
            repo: "oven-sh/bun",
            tag_strip: "bun-v",
        },
    ),
];

/// Extract a single version dep from a version file.
///
/// `manager_name` matches the key in `VERSION_FILE_DEFS` (e.g. `"terraform-version"`).
/// Returns `None` when the file is empty, comment-only, or the manager is unknown.
pub fn extract(content: &str, manager_name: &str) -> Option<VersionFileDep> {
    let def = VERSION_FILE_DEFS
        .iter()
        .find(|(name, _, _)| *name == manager_name)?;

    // Take the first non-empty, non-comment line.
    let version = content
        .lines()
        .map(|l| {
            // Strip inline comments.
            l.split('#').next().unwrap_or("").trim()
        })
        .find(|l| !l.is_empty())?;

    // Skip NVM aliases like `lts/*`, `latest`, `stable`, `node`.
    if version.contains('/') || matches!(version, "lts" | "latest" | "stable" | "node") {
        return None;
    }

    // Strip leading `v` if present (file may store `v1.6.0` or `1.6.0`).
    let bare_version = version.trim_start_matches('v').to_owned();
    if bare_version.is_empty() {
        return None;
    }

    Some(VersionFileDep {
        tool: def.1,
        current_value: bare_version,
        datasource: def.2.clone(),
    })
}

/// Returns the manager name for a given filename, if it is a known version file.
pub fn manager_for_file(filename: &str) -> Option<&'static str> {
    // Strip path prefix — compare basename only.
    let base = filename.rsplit('/').next().unwrap_or(filename);
    match base {
        ".terraform-version" => Some("terraform-version"),
        ".terragrunt-version" => Some("terragrunt-version"),
        ".go-version" => Some("go-version"),
        ".python-version" => Some("python-version"),
        ".node-version" => Some("node-version"),
        ".nvmrc" => Some("nvmrc"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terraform_version_plain() {
        let dep = extract("1.6.3\n", "terraform-version").unwrap();
        assert_eq!(dep.tool, "terraform");
        assert_eq!(dep.current_value, "1.6.3");
        assert_eq!(
            dep.datasource,
            AsdfDatasource::GithubReleases {
                repo: "hashicorp/terraform",
                tag_strip: "v",
            }
        );
    }

    #[test]
    fn terraform_version_with_v_prefix() {
        // Some tools write `v1.6.3` in the file; strip the `v`.
        let dep = extract("v1.6.3\n", "terraform-version").unwrap();
        assert_eq!(dep.current_value, "1.6.3");
    }

    #[test]
    fn go_version_file() {
        let dep = extract("1.21.4\n", "go-version").unwrap();
        assert_eq!(dep.tool, "golang");
        assert_eq!(dep.current_value, "1.21.4");
        assert_eq!(
            dep.datasource,
            AsdfDatasource::GithubTags {
                repo: "golang/go",
                tag_strip: "go",
            }
        );
    }

    #[test]
    fn python_version_file() {
        let dep = extract("3.11.5\n", "python-version").unwrap();
        assert_eq!(dep.tool, "python");
        assert_eq!(dep.current_value, "3.11.5");
    }

    #[test]
    fn node_version_file() {
        let dep = extract("20.9.0\n", "node-version").unwrap();
        assert_eq!(dep.tool, "nodejs");
    }

    #[test]
    fn nvmrc_lts_alias_returns_none() {
        assert!(extract("lts/*\n", "nvmrc").is_none());
    }

    #[test]
    fn nvmrc_latest_alias_returns_none() {
        assert!(extract("latest\n", "nvmrc").is_none());
    }

    #[test]
    fn nvmrc_plain_version() {
        let dep = extract("20.9.0\n", "nvmrc").unwrap();
        assert_eq!(dep.current_value, "20.9.0");
    }

    #[test]
    fn strips_inline_comment() {
        let dep = extract("1.6.3 # pinned\n", "terraform-version").unwrap();
        assert_eq!(dep.current_value, "1.6.3");
    }

    #[test]
    fn empty_file_returns_none() {
        assert!(extract("", "terraform-version").is_none());
    }

    #[test]
    fn comment_only_returns_none() {
        assert!(extract("# just a comment\n", "terraform-version").is_none());
    }

    #[test]
    fn unknown_manager_returns_none() {
        assert!(extract("1.0.0\n", "unknown-manager").is_none());
    }

    #[test]
    fn manager_for_file_dot_terraform() {
        assert_eq!(
            manager_for_file(".terraform-version"),
            Some("terraform-version")
        );
        assert_eq!(
            manager_for_file("app/.terraform-version"),
            Some("terraform-version")
        );
    }

    #[test]
    fn manager_for_file_nvmrc() {
        assert_eq!(manager_for_file(".nvmrc"), Some("nvmrc"));
    }

    #[test]
    fn manager_for_file_unknown() {
        assert_eq!(manager_for_file("Gemfile"), None);
    }
}
