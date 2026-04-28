//! asdf `.tool-versions` dependency extractor.
//!
//! Parses `.tool-versions` files and maps each tool to the appropriate
//! datasource for version lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/asdf/extract.ts`
//! - `lib/modules/manager/asdf/upgradeable-tooling.ts`
//! - `lib/modules/manager/asdf/index.ts` — pattern `/(^|/)\\.tool-versions$/`
//!
//! ## Format
//!
//! ```text
//! nodejs 20.9.0
//! python 3.11.5
//! terraform 1.6.3
//! ```
//!
//! Each non-comment line: `<tool> <version> [<version2>...]`
//! Only the first version is captured (asdf installs the first when `asdf install`
//! is run without arguments).
//!
//! ## Datasource routing
//!
//! Each known tool maps to a GitHub Tags or GitHub Releases repository plus a
//! `tag_strip` prefix. When a tag like `v1.6.3` is returned from GitHub, the
//! prefix is stripped before semver comparison with the stored version `1.6.3`.

use std::sync::LazyLock;

use regex::Regex;

/// Which datasource to use for a tool.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsdfDatasource {
    /// GitHub repository tags API.
    GithubTags {
        /// `owner/repo` on GitHub.
        repo: &'static str,
        /// Prefix to strip from the tag name to get the bare version.
        /// E.g. `"v"` for tags like `v1.2.3`, `"go"` for `go1.21.0`, `""` for bare.
        tag_strip: &'static str,
    },
    /// GitHub repository releases API.
    GithubReleases {
        repo: &'static str,
        tag_strip: &'static str,
    },
}

/// Why a dep is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsdfSkipReason {
    /// Tool is not in the known-tools table.
    UnsupportedTool,
}

/// A single extracted asdf dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsdfDep {
    pub tool_name: String,
    pub current_value: String,
    pub datasource: Option<AsdfDatasource>,
    pub skip_reason: Option<AsdfSkipReason>,
}

/// Extract dependencies from a `.tool-versions` file.
pub fn extract(content: &str) -> Vec<AsdfDep> {
    let mut out = Vec::new();

    for raw in content.lines() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }

        if let Some(dep) = parse_line(line) {
            out.push(dep);
        }
    }

    out
}

// ── Helpers ───────────────────────────────────────────────────────────────────

static LINE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^([\w_-]+)\s+(\S+)").unwrap());

fn parse_line(line: &str) -> Option<AsdfDep> {
    let cap = LINE_RE.captures(line)?;
    let tool_name = cap[1].to_owned();
    let current_value = cap[2].to_owned();

    let (datasource, skip_reason) = match TOOL_TABLE.iter().find(|(name, _)| *name == tool_name) {
        Some((_, ds)) => (Some(ds.clone()), None),
        None => (None, Some(AsdfSkipReason::UnsupportedTool)),
    };

    Some(AsdfDep {
        tool_name,
        current_value,
        datasource,
        skip_reason,
    })
}

// ── Tool table ────────────────────────────────────────────────────────────────

static TOOL_TABLE: &[(&str, AsdfDatasource)] = &[
    // ── GitHub Tags ─────────────────────────────────────────────────────────
    (
        "awscli",
        AsdfDatasource::GithubTags {
            repo: "aws/aws-cli",
            tag_strip: "",
        },
    ),
    (
        "erlang",
        AsdfDatasource::GithubTags {
            repo: "erlang/otp",
            tag_strip: "OTP-",
        },
    ),
    (
        "flux2",
        AsdfDatasource::GithubTags {
            repo: "fluxcd/flux2",
            tag_strip: "v",
        },
    ),
    (
        "golang",
        AsdfDatasource::GithubTags {
            repo: "golang/go",
            tag_strip: "go",
        },
    ),
    (
        "kubectl",
        AsdfDatasource::GithubTags {
            repo: "kubernetes/kubernetes",
            tag_strip: "v",
        },
    ),
    (
        "perl",
        AsdfDatasource::GithubTags {
            repo: "Perl/perl5",
            tag_strip: "v",
        },
    ),
    (
        "php",
        AsdfDatasource::GithubTags {
            repo: "php/php-src",
            tag_strip: "php-",
        },
    ),
    (
        "python",
        AsdfDatasource::GithubTags {
            repo: "python/cpython",
            tag_strip: "v",
        },
    ),
    (
        "rust",
        AsdfDatasource::GithubTags {
            repo: "rust-lang/rust",
            tag_strip: "",
        },
    ),
    // ── GitHub Releases ──────────────────────────────────────────────────────
    (
        "argocd",
        AsdfDatasource::GithubReleases {
            repo: "argoproj/argo-cd",
            tag_strip: "v",
        },
    ),
    (
        "consul",
        AsdfDatasource::GithubReleases {
            repo: "hashicorp/consul",
            tag_strip: "v",
        },
    ),
    (
        "helm",
        AsdfDatasource::GithubReleases {
            repo: "helm/helm",
            tag_strip: "v",
        },
    ),
    (
        "k9s",
        AsdfDatasource::GithubReleases {
            repo: "derailed/k9s",
            tag_strip: "v",
        },
    ),
    (
        "kind",
        AsdfDatasource::GithubReleases {
            repo: "kubernetes-sigs/kind",
            tag_strip: "v",
        },
    ),
    (
        "minikube",
        AsdfDatasource::GithubReleases {
            repo: "kubernetes/minikube",
            tag_strip: "v",
        },
    ),
    (
        "packer",
        AsdfDatasource::GithubReleases {
            repo: "hashicorp/packer",
            tag_strip: "v",
        },
    ),
    (
        "terraform",
        AsdfDatasource::GithubReleases {
            repo: "hashicorp/terraform",
            tag_strip: "v",
        },
    ),
    (
        "terragrunt",
        AsdfDatasource::GithubReleases {
            repo: "gruntwork-io/terragrunt",
            tag_strip: "v",
        },
    ),
    (
        "vault",
        AsdfDatasource::GithubReleases {
            repo: "hashicorp/vault",
            tag_strip: "v",
        },
    ),
    (
        "waypoint",
        AsdfDatasource::GithubReleases {
            repo: "hashicorp/waypoint",
            tag_strip: "v",
        },
    ),
    // ── Additional popular tools ──────────────────────────────────────────────
    (
        "bun",
        AsdfDatasource::GithubReleases {
            repo: "oven-sh/bun",
            tag_strip: "bun-v",
        },
    ),
    (
        "deno",
        AsdfDatasource::GithubReleases {
            repo: "denoland/deno",
            tag_strip: "v",
        },
    ),
    (
        "zig",
        AsdfDatasource::GithubTags {
            repo: "ziglang/zig",
            tag_strip: "",
        },
    ),
    (
        "elixir",
        AsdfDatasource::GithubTags {
            repo: "elixir-lang/elixir",
            tag_strip: "v",
        },
    ),
    (
        "java",
        AsdfDatasource::GithubReleases {
            repo: "adoptium/temurin17-binaries",
            tag_strip: "jdk-",
        },
    ),
    (
        "scala",
        AsdfDatasource::GithubTags {
            repo: "scala/scala",
            tag_strip: "v",
        },
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "
# tool versions
nodejs 20.9.0        # managed via asdf
python 3.11.5
terraform 1.6.3
kubectl 1.28.3
golang 1.21.4
helm 3.13.1
rust 1.73.0
unknowntool 9.9.9
";

    #[test]
    fn extracts_github_releases_tool() {
        let deps = extract(SAMPLE);
        let tf = deps.iter().find(|d| d.tool_name == "terraform").unwrap();
        assert_eq!(tf.current_value, "1.6.3");
        assert_eq!(
            tf.datasource,
            Some(AsdfDatasource::GithubReleases {
                repo: "hashicorp/terraform",
                tag_strip: "v",
            })
        );
        assert!(tf.skip_reason.is_none());
    }

    #[test]
    fn extracts_github_tags_tool() {
        let deps = extract(SAMPLE);
        let py = deps.iter().find(|d| d.tool_name == "python").unwrap();
        assert_eq!(py.current_value, "3.11.5");
        assert_eq!(
            py.datasource,
            Some(AsdfDatasource::GithubTags {
                repo: "python/cpython",
                tag_strip: "v",
            })
        );
    }

    #[test]
    fn extracts_golang_go_prefix() {
        let deps = extract(SAMPLE);
        let go = deps.iter().find(|d| d.tool_name == "golang").unwrap();
        assert_eq!(go.current_value, "1.21.4");
        assert_eq!(
            go.datasource,
            Some(AsdfDatasource::GithubTags {
                repo: "golang/go",
                tag_strip: "go",
            })
        );
    }

    #[test]
    fn unknown_tool_gets_skip_reason() {
        let deps = extract(SAMPLE);
        let unknown = deps.iter().find(|d| d.tool_name == "unknowntool").unwrap();
        assert_eq!(unknown.skip_reason, Some(AsdfSkipReason::UnsupportedTool));
        assert!(unknown.datasource.is_none());
    }

    #[test]
    fn strips_inline_comments() {
        let deps = extract(SAMPLE);
        let node = deps.iter().find(|d| d.tool_name == "nodejs");
        // nodejs is in the file but not in the tool table → UnsupportedTool
        assert!(node.is_some());
        assert_eq!(node.unwrap().current_value, "20.9.0");
    }

    #[test]
    fn empty_content_returns_no_deps() {
        assert!(extract("").is_empty());
    }

    #[test]
    fn comment_lines_skipped() {
        let deps = extract("# this is a comment\npython 3.11.5\n");
        assert_eq!(deps.len(), 1);
    }
}
