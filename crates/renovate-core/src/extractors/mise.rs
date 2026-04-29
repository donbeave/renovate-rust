//! mise-en-place (`mise.toml` / `.mise.toml`) dependency extractor.
//!
//! Parses the `[tools]` section of mise configuration files and maps each
//! tool to the appropriate datasource, reusing the asdf tool table.
//!
//! Renovate reference:
//! - `lib/modules/manager/mise/extract.ts`
//! - Patterns: `**/{,.}mise{,.*}.toml`, `**/{,.}mise/config{,.*}.toml`,
//!   `**/.config/mise{,.*}.toml`
//!
//! ## Supported form
//!
//! ```toml
//! [tools]
//! node = "18"
//! python = "3.11.5"
//! go = "1.21.0"
//! terraform = "1.6.3"
//! ```
//!
//! Only simple string versions are extracted. Array and inline-table forms
//! are skipped in this first-cut implementation.

use crate::extractors::asdf::{AsdfDatasource, AsdfDep, AsdfSkipReason};

/// Mise uses different tool names than asdf (e.g. `node` vs `nodejs`).
/// This table maps mise tool names → datasource.
static MISE_TOOL_TABLE: &[(&str, AsdfDatasource)] = &[
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
        "elixir",
        AsdfDatasource::GithubReleases {
            repo: "elixir-lang/elixir",
            tag_strip: "v",
        },
    ),
    (
        "go",
        AsdfDatasource::GithubTags {
            repo: "golang/go",
            tag_strip: "go",
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
        "node",
        AsdfDatasource::GithubReleases {
            repo: "nodejs/node",
            tag_strip: "v",
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
        "ruby",
        AsdfDatasource::GithubTags {
            repo: "ruby/ruby",
            tag_strip: "v_",
        },
    ),
    (
        "rust",
        AsdfDatasource::GithubTags {
            repo: "rust-lang/rust",
            tag_strip: "",
        },
    ),
    (
        "scala",
        AsdfDatasource::GithubTags {
            repo: "scala/scala",
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
        "zig",
        AsdfDatasource::GithubTags {
            repo: "ziglang/zig",
            tag_strip: "",
        },
    ),
];

/// Extract dependencies from a `mise.toml` file.
///
/// Scans the `[tools]` section and returns one dep per recognized tool.
pub fn extract(content: &str) -> Vec<AsdfDep> {
    let mut out = Vec::new();
    let mut in_tools = false;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        // Section header detection.
        if trimmed.starts_with('[') {
            in_tools = trimmed == "[tools]";
            continue;
        }

        if !in_tools {
            continue;
        }

        // Parse `tool = "version"` or `tool = '...'`
        let Some((tool_raw, val_raw)) = trimmed.split_once('=') else {
            continue;
        };

        let tool_name = tool_raw.trim().trim_matches('"').trim_matches('\'');
        let version_raw = val_raw.trim();

        // Only handle simple quoted string versions (not arrays or tables).
        if !version_raw.starts_with('"') && !version_raw.starts_with('\'') {
            continue;
        }

        let version = version_raw.trim_matches('"').trim_matches('\'').trim();

        if version.is_empty() {
            continue;
        }

        // Look up the tool in the mise tool table.
        let (datasource, skip_reason) =
            match MISE_TOOL_TABLE.iter().find(|(name, _)| *name == tool_name) {
                Some((_, ds)) => (Some(ds.clone()), None),
                None => (None, Some(AsdfSkipReason::UnsupportedTool)),
            };

        out.push(AsdfDep {
            tool_name: tool_name.to_owned(),
            dep_name: tool_name.to_owned(),
            current_value: version.to_owned(),
            datasource,
            skip_reason,
            ..Default::default()
        });
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_node_version() {
        let content = "[tools]\nnode = \"18\"\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].tool_name, "node");
        assert_eq!(deps[0].current_value, "18");
        assert!(deps[0].skip_reason.is_none());
    }

    #[test]
    fn extracts_multiple_tools() {
        let content = "[tools]\nnode = \"20.9.0\"\npython = \"3.11.5\"\ngo = \"1.21.0\"\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
        assert!(deps.iter().any(|d| d.tool_name == "node"));
        assert!(deps.iter().any(|d| d.tool_name == "python"));
        assert!(deps.iter().any(|d| d.tool_name == "go"));
    }

    #[test]
    fn unknown_tool_skipped() {
        let content = "[tools]\nmyunknowntool = \"1.0\"\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(AsdfSkipReason::UnsupportedTool));
    }

    #[test]
    fn ignores_non_tools_sections() {
        let content = "[settings]\nsomething = \"value\"\n[tools]\nnode = \"18\"\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
    }

    #[test]
    fn ignores_array_versions() {
        // Array format - skipped in first cut
        let content = "[tools]\nnode = [\"18\", \"20\"]\n";
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }
}
