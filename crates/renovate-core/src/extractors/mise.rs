//! mise-en-place (`mise.toml` / `.mise.toml`) dependency extractor.
//!
//! Parses the `[tools]` section of mise configuration files and maps each
//! tool to the appropriate datasource.
//!
//! Renovate reference:
//! - `lib/modules/manager/mise/extract.ts`
//! - `lib/modules/manager/mise/upgradeable-tooling.ts` — `miseCoreTooling`
//! - `lib/modules/manager/asdf/upgradeable-tooling.ts` — `asdfTooling`
//! - Patterns: `**/{,.}mise{,.*}.toml`, `**/{,.}mise/config{,.*}.toml`,
//!   `**/.config/mise{,.*}.toml`
//!
//! ## Resolution order
//!
//! 1. `MISE_CORE_TABLE` — mise-specific tool names (e.g. `node`, `go`).
//! 2. `asdf::TOOL_TABLE` — asdf-compatible tool names reused for mise.
//! 3. Dynamic tools: `java`, `scala` (version-dependent datasource).

use crate::extractors::asdf::AsdfDatasource;
use crate::extractors::asdf::{
    self, AsdfDep, AsdfSkipReason, AsdfToolDef, datasource_id, tag_strip_from_extract_version,
};

/// Mise-specific core tool names that differ from the asdf tool key.
///
/// Source: `lib/modules/manager/mise/upgradeable-tooling.ts` → `miseCoreTooling`
static MISE_CORE_TABLE: &[(&str, AsdfToolDef)] = &[
    (
        "bun",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("oven-sh/bun"),
            dep_name: None,
            extract_version: Some("^bun-v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "deno",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("denoland/deno"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "elixir",
        AsdfToolDef {
            datasource: datasource_id::HEXPM_BOB,
            package_name: None,
            dep_name: None,
            extract_version: None,
            versioning: None,
        },
    ),
    (
        "erlang",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_TAGS,
            package_name: Some("erlang/otp"),
            dep_name: None,
            extract_version: Some("^OTP-(?<version>\\S+)"),
            versioning: Some("regex:^(?<major>\\d+?)\\.(?<minor>\\d+?)(\\.(?<patch>\\d+))?$"),
        },
    ),
    (
        "go",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_TAGS,
            package_name: Some("golang/go"),
            dep_name: None,
            extract_version: Some("^go(?<version>\\S+)"),
            versioning: None,
        },
    ),
    // java handled dynamically via try_dynamic_tool
    (
        "node",
        AsdfToolDef {
            datasource: datasource_id::NODE_VERSION,
            package_name: Some("node"),
            dep_name: None,
            extract_version: None,
            versioning: None,
        },
    ),
    (
        "python",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_TAGS,
            package_name: Some("python/cpython"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "ruby",
        AsdfToolDef {
            datasource: datasource_id::RUBY_VERSION,
            package_name: Some("ruby-version"),
            dep_name: None,
            extract_version: None,
            versioning: Some("semver"),
        },
    ),
    (
        "rust",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_TAGS,
            package_name: Some("rust-lang/rust"),
            dep_name: None,
            extract_version: None,
            versioning: None,
        },
    ),
    (
        "swift",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("swift-lang/swift"),
            dep_name: None,
            extract_version: Some("^swift-(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "zig",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_TAGS,
            package_name: Some("ziglang/zig"),
            dep_name: None,
            extract_version: None,
            versioning: None,
        },
    ),
];

fn make_dep_from_def(tool_name: &str, version: &str, def: &AsdfToolDef) -> AsdfDep {
    let dep_name = def.dep_name.unwrap_or(tool_name).to_owned();
    let pkg = def.package_name;
    let strip = tag_strip_from_extract_version(def.extract_version);
    let legacy = match def.datasource {
        datasource_id::GITHUB_TAGS => pkg.map(|r| AsdfDatasource::GithubTags {
            repo: r,
            tag_strip: strip,
        }),
        datasource_id::GITHUB_RELEASES => pkg.map(|r| AsdfDatasource::GithubReleases {
            repo: r,
            tag_strip: strip,
        }),
        _ => None,
    };
    AsdfDep {
        tool_name: tool_name.to_owned(),
        dep_name,
        current_value: version.to_owned(),
        datasource: legacy,
        datasource_id: Some(def.datasource),
        package_name: pkg,
        extract_version: def.extract_version,
        versioning: def.versioning,
        skip_reason: None,
    }
}

/// Extract dependencies from a `mise.toml` file.
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

        // Dynamic tools with version-dependent datasource.
        if tool_name == "java" {
            out.push(asdf::parse_java_dep(tool_name, version));
            continue;
        }
        if tool_name == "scala" {
            out.push(asdf::parse_scala_dep(tool_name, version));
            continue;
        }

        // Mise core tooling.
        if let Some((_, def)) = MISE_CORE_TABLE.iter().find(|(k, _)| *k == tool_name) {
            out.push(make_dep_from_def(tool_name, version, def));
            continue;
        }

        // Fall back to asdf TOOL_TABLE (same tool names work in both).
        if let Some((_, def)) = asdf::TOOL_TABLE.iter().find(|(k, _)| *k == tool_name) {
            out.push(make_dep_from_def(tool_name, version, def));
            continue;
        }

        // Unknown tool.
        out.push(AsdfDep {
            tool_name: tool_name.to_owned(),
            dep_name: tool_name.to_owned(),
            current_value: version.to_owned(),
            skip_reason: Some(AsdfSkipReason::UnsupportedTool),
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
        // Ported: "extracts tools - mise core plugins" (node part) — mise/extract.spec.ts line 28
        let content = "[tools]\nnode = \"18\"\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].tool_name, "node");
        assert_eq!(deps[0].current_value, "18");
        assert_eq!(deps[0].datasource_id, Some("node-version"));
        assert!(deps[0].datasource.is_none()); // not a GitHub tool
        assert!(deps[0].skip_reason.is_none());
    }

    #[test]
    fn extracts_erlang_core_plugin() {
        // Ported: "extracts tools - mise core plugins" (erlang part) — mise/extract.spec.ts line 28
        let content = "[tools]\nerlang = \"23.3\"\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let d = &deps[0];
        assert_eq!(d.tool_name, "erlang");
        assert_eq!(d.current_value, "23.3");
        assert_eq!(d.datasource_id, Some("github-tags"));
        assert_eq!(d.package_name, Some("erlang/otp"));
        assert_eq!(d.extract_version, Some("^OTP-(?<version>\\S+)"));
        assert!(d.skip_reason.is_none());
    }

    #[test]
    fn extracts_multiple_tools() {
        let content = "[tools]\nnode = \"20.9.0\"\npython = \"3.11.5\"\ngo = \"1.21.0\"\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
        assert!(
            deps.iter()
                .any(|d| d.tool_name == "node" && d.datasource_id == Some("node-version"))
        );
        assert!(
            deps.iter()
                .any(|d| d.tool_name == "python" && d.datasource_id == Some("github-tags"))
        );
        assert!(
            deps.iter()
                .any(|d| d.tool_name == "go" && d.datasource_id == Some("github-tags"))
        );
    }

    #[test]
    fn asdf_tools_fall_through_to_asdf_table() {
        // Tools not in mise core but in asdf table should still be resolved.
        let content = "[tools]\nterraform = \"1.6.3\"\nhelm = \"3.13.1\"\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        let tf = deps.iter().find(|d| d.tool_name == "terraform").unwrap();
        assert_eq!(tf.datasource_id, Some("github-releases"));
        assert_eq!(tf.package_name, Some("hashicorp/terraform"));
    }

    #[test]
    fn unknown_tool_skipped() {
        // Ported: "provides skipReason for lines with unsupported tooling" — mise/extract.spec.ts line 781
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
        // Ported: "extracts tools with multiple versions" — mise/extract.spec.ts line 409
        // Array versions are skipped in this implementation (Renovate picks the first).
        let content = "[tools]\nnode = [\"18\", \"20\"]\n";
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    #[test]
    fn empty_returns_empty() {
        // Ported: "returns null for empty" — mise/extract.spec.ts line 13
        assert!(extract("").is_empty());
    }

    #[test]
    fn java_core_plugin_jdk() {
        // Ported: "core java plugin function" — mise/extract.spec.ts line 911 (partial)
        let content = "[tools]\njava = \"adoptopenjdk-16.0.0+36\"\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let d = &deps[0];
        assert_eq!(d.datasource_id, Some("java-version"));
        assert_eq!(d.package_name, Some("java-jdk"));
        assert_eq!(d.current_value, "16.0.0+36");
        assert!(d.skip_reason.is_none());
    }
}
