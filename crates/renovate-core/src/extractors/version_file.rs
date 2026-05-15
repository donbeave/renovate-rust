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
    (
        "bazelisk",
        "bazel",
        AsdfDatasource::GithubReleases {
            repo: "bazelbuild/bazel",
            tag_strip: "",
        },
    ),
    (
        "bazel-version",
        "bazel",
        AsdfDatasource::GithubReleases {
            repo: "bazelbuild/bazel",
            tag_strip: "",
        },
    ),
    (
        "ruby-version",
        "ruby",
        AsdfDatasource::GithubTags {
            repo: "ruby/ruby",
            tag_strip: "v",
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

    // Skip NVM aliases with wildcard selectors such as `lts/*`.
    if manager_name == "nvmrc" && version.contains('/') {
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

/// A bazelisk (`.bazelversion`) dependency extracted by the bazelisk manager.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BazeliskDep {
    pub dep_name: &'static str,
    pub current_value: String,
    pub datasource: &'static str,
    pub package_name: &'static str,
}

/// Extract the bazel version from `.bazelversion` file content.
///
/// Mirrors `lib/modules/manager/bazelisk/extract.ts` `extractPackageFile()`.
pub fn extract_bazelisk(content: &str) -> BazeliskDep {
    BazeliskDep {
        dep_name: "bazel",
        current_value: content.lines().next().unwrap_or("").trim().to_owned(),
        datasource: "github-releases",
        package_name: "bazelbuild/bazel",
    }
}

/// A nodenv (`.node-version`) dependency extracted by the nodenv manager.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeenvDep {
    pub dep_name: &'static str,
    pub current_value: String,
    pub datasource: &'static str,
}

/// Extract the node version from `.node-version` file content.
///
/// Mirrors `lib/modules/manager/nodenv/extract.ts` `extractPackageFile()`.
pub fn extract_nodenv(content: &str) -> NodeenvDep {
    NodeenvDep {
        dep_name: "node",
        current_value: content.trim().to_owned(),
        datasource: "node-version",
    }
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
        ".bazelversion" => Some("bazelisk"),
        ".ruby-version" => Some("ruby-version"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "returns a result" — terraform-version/extract.spec.ts line 5
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

    // Ported: "skips non ranges" — terraform-version/extract.spec.ts line 18
    #[test]
    fn terraform_version_passes_through_non_alias_literal() {
        let dep = extract("latest", "terraform-version").unwrap();
        assert_eq!(dep.tool, "terraform");
        assert_eq!(dep.current_value, "latest");
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

    // Ported: "returns a result" — nvm/extract.spec.ts line 5
    #[test]
    fn nvmrc_plain_version() {
        let dep = extract("20.9.0\n", "nvmrc").unwrap();
        assert_eq!(dep.current_value, "20.9.0");
    }

    // Ported: "skips non ranges" — nvm/extract.spec.ts line 27
    #[test]
    fn nvmrc_passes_through_latest_literal() {
        let dep = extract("latest\n", "nvmrc").unwrap();
        assert_eq!(dep.tool, "nodejs");
        assert_eq!(dep.current_value, "latest");
        assert_eq!(
            dep.datasource,
            AsdfDatasource::GithubReleases {
                repo: "nodejs/node",
                tag_strip: "v",
            }
        );
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

    // Ported: "returns a result" — ruby-version/extract.spec.ts line 5
    #[test]
    fn ruby_version_file() {
        let dep = extract("3.3.0\n", "ruby-version").unwrap();
        assert_eq!(dep.tool, "ruby");
        assert_eq!(dep.current_value, "3.3.0");
        assert_eq!(
            dep.datasource,
            AsdfDatasource::GithubTags {
                repo: "ruby/ruby",
                tag_strip: "v",
            }
        );
    }

    #[test]
    fn manager_for_file_ruby_version() {
        assert_eq!(manager_for_file(".ruby-version"), Some("ruby-version"));
    }

    // Ported: "returns a result" — terragrunt-version/extract.spec.ts line 5
    #[test]
    fn terragrunt_version_file() {
        let dep = extract("12.0.0\n", "terragrunt-version").unwrap();
        assert_eq!(dep.tool, "terragrunt");
        assert_eq!(dep.current_value, "12.0.0");
        assert_eq!(
            dep.datasource,
            AsdfDatasource::GithubReleases {
                repo: "gruntwork-io/terragrunt",
                tag_strip: "v",
            }
        );
    }

    // Ported: "supports ranges" — nvm/extract.spec.ts line 16
    #[test]
    fn nvmrc_partial_version_range() {
        let dep = extract("8.4\n", "nvmrc").unwrap();
        assert_eq!(dep.tool, "nodejs");
        assert_eq!(dep.current_value, "8.4");
    }

    // Ported: "supports code comments" — nvm/extract.spec.ts line 38
    #[test]
    fn nvmrc_skips_full_line_comments_and_inline_comment() {
        let content =
            "# This is a comment\nv20.19.3 # This is an inline comment\n# This is another comment";
        let dep = extract(content, "nvmrc").unwrap();
        assert_eq!(dep.tool, "nodejs");
        // Leading `v` is stripped by the extractor.
        assert_eq!(dep.current_value, "20.19.3");
    }

    // Ported: "supports ranges" — ruby-version/extract.spec.ts line 16
    #[test]
    fn ruby_version_partial_range() {
        let dep = extract("8.4\n", "ruby-version").unwrap();
        assert_eq!(dep.tool, "ruby");
        assert_eq!(dep.current_value, "8.4");
    }

    // Ported: "skips non ranges" — ruby-version/extract.spec.ts line 27
    //
    // The TS spec passes the literal `latestn` (a typo for `latest`) and
    // expects the extractor to return it unchanged. Rust matches: `latestn`
    // is not in the alias list, so the value is returned verbatim.
    #[test]
    fn ruby_version_passes_through_non_alias_literal() {
        let dep = extract("latestn", "ruby-version").unwrap();
        assert_eq!(dep.tool, "ruby");
        assert_eq!(dep.current_value, "latestn");
    }

    // Ported: "returns a result" — modules/manager/nodenv/extract.spec.ts line 5
    #[test]
    fn nodenv_returns_dep_for_version() {
        let dep = extract_nodenv("8.4.0\n");
        assert_eq!(dep.dep_name, "node");
        assert_eq!(dep.current_value, "8.4.0");
        assert_eq!(dep.datasource, "node-version");
    }

    // Ported: "supports ranges" — modules/manager/nodenv/extract.spec.ts line 14
    #[test]
    fn nodenv_supports_partial_version() {
        let dep = extract_nodenv("8.4\n");
        assert_eq!(dep.dep_name, "node");
        assert_eq!(dep.current_value, "8.4");
        assert_eq!(dep.datasource, "node-version");
    }

    // Ported: "skips non ranges" — modules/manager/nodenv/extract.spec.ts line 23
    #[test]
    fn nodenv_passes_through_non_version_string() {
        let dep = extract_nodenv("latestn");
        assert_eq!(dep.dep_name, "node");
        assert_eq!(dep.current_value, "latestn");
        assert_eq!(dep.datasource, "node-version");
    }

    // Ported: "returns a result" — modules/manager/bazelisk/extract.spec.ts line 5
    #[test]
    fn bazelisk_returns_dep_for_version() {
        let dep = extract_bazelisk("5.2.0\n");
        assert_eq!(dep.dep_name, "bazel");
        assert_eq!(dep.current_value, "5.2.0");
        assert_eq!(dep.datasource, "github-releases");
        assert_eq!(dep.package_name, "bazelbuild/bazel");
    }

    // Ported: "supports ranges" — modules/manager/bazelisk/extract.spec.ts line 14
    #[test]
    fn bazelisk_supports_partial_version() {
        let dep = extract_bazelisk("5.2");
        assert_eq!(dep.current_value, "5.2");
    }

    // Ported: "skips non ranges" — modules/manager/bazelisk/extract.spec.ts line 23
    #[test]
    fn bazelisk_passes_through_non_version_string() {
        let dep = extract_bazelisk("latestn");
        assert_eq!(dep.current_value, "latestn");
    }

    // Ported: "ignores comments past the first line" — modules/manager/bazelisk/extract.spec.ts line 32
    #[test]
    fn bazelisk_ignores_comments_past_first_line() {
        let dep = extract_bazelisk("5.2.0\n# comment1\n\n# comment2");
        assert_eq!(dep.current_value, "5.2.0");
    }
}
