//! Proto `.prototools` dependency extractor.
//!
//! Parses tool version pins from `.prototools` TOML files.
//!
//! Renovate reference:
//! - `lib/modules/manager/proto/extract.ts`
//! - `lib/modules/manager/proto/upgradeable-tooling.ts`

use toml::Value;

/// Non-version TOML sections in `.prototools` files.
const NON_VERSION_KEYS: &[&str] = &["settings", "plugins", "tools", "env", "shell", "backends"];

/// Version aliases that cannot be updated via semver.
const VERSION_ALIASES: &[&str] = &["latest", "stable", "canary", "nightly"];

/// A single extracted proto dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtoDep {
    pub dep_name: String,
    pub current_value: String,
    pub datasource: Option<&'static str>,
    pub package_name: Option<&'static str>,
    pub extract_version: Option<&'static str>,
    pub skip_reason: Option<&'static str>,
}

struct ToolConfig {
    datasource: &'static str,
    package_name: &'static str,
    extract_version: Option<&'static str>,
}

fn tool_config(tool_name: &str) -> Option<ToolConfig> {
    match tool_name {
        "bun" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "oven-sh/bun",
            extract_version: Some("^bun-v(?<version>\\S+)"),
        }),
        "deno" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "denoland/deno",
            extract_version: Some("^v(?<version>\\S+)"),
        }),
        "gh" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "cli/cli",
            extract_version: Some("^v(?<version>\\S+)"),
        }),
        "go" => Some(ToolConfig {
            datasource: "github-tags",
            package_name: "golang/go",
            extract_version: Some("^go(?<version>\\S+)"),
        }),
        "moon" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "moonrepo/moon",
            extract_version: Some("^v(?<version>\\S+)"),
        }),
        "node" => Some(ToolConfig {
            datasource: "node-version",
            package_name: "node",
            extract_version: None,
        }),
        "npm" => Some(ToolConfig {
            datasource: "npm",
            package_name: "npm",
            extract_version: None,
        }),
        "pnpm" => Some(ToolConfig {
            datasource: "npm",
            package_name: "pnpm",
            extract_version: None,
        }),
        "poetry" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "python-poetry/poetry",
            extract_version: None,
        }),
        "proto" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "moonrepo/proto",
            extract_version: Some("^v(?<version>\\S+)"),
        }),
        "python" => Some(ToolConfig {
            datasource: "github-tags",
            package_name: "python/cpython",
            extract_version: Some("^v(?<version>\\S+)"),
        }),
        "ruby" => Some(ToolConfig {
            datasource: "ruby-version",
            package_name: "ruby-version",
            extract_version: None,
        }),
        "rust" => Some(ToolConfig {
            datasource: "github-tags",
            package_name: "rust-lang/rust",
            extract_version: None,
        }),
        "uv" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "astral-sh/uv",
            extract_version: None,
        }),
        "yarn" => Some(ToolConfig {
            datasource: "npm",
            package_name: "@yarnpkg/cli",
            extract_version: None,
        }),
        _ => None,
    }
}

/// Parse a `.prototools` TOML file and extract tool version dependencies.
///
/// Returns `None` for empty content, invalid TOML, or files with no version pins.
pub fn extract_package_file(content: &str) -> Option<Vec<ProtoDep>> {
    if content.is_empty() {
        return None;
    }
    let doc: Value = toml::from_str(content).ok()?;
    let table = doc.as_table()?;

    let mut deps = Vec::new();
    for (key, value) in table {
        if NON_VERSION_KEYS.contains(&key.as_str()) {
            continue;
        }
        let version = match value.as_str() {
            Some(s) => s.to_owned(),
            None => continue, // non-string values (tables) are not version pins
        };

        if VERSION_ALIASES.contains(&version.as_str()) {
            deps.push(ProtoDep {
                dep_name: key.clone(),
                current_value: version,
                datasource: None,
                package_name: None,
                extract_version: None,
                skip_reason: Some("unsupported-version"),
            });
            continue;
        }

        if let Some(cfg) = tool_config(key) {
            deps.push(ProtoDep {
                dep_name: key.clone(),
                current_value: version,
                datasource: Some(cfg.datasource),
                package_name: Some(cfg.package_name),
                extract_version: cfg.extract_version,
                skip_reason: None,
            });
        } else {
            deps.push(ProtoDep {
                dep_name: key.clone(),
                current_value: version,
                datasource: None,
                package_name: None,
                extract_version: None,
                skip_reason: Some("unsupported-datasource"),
            });
        }
    }

    if deps.is_empty() { None } else { Some(deps) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dep_by_name<'a>(deps: &'a [ProtoDep], name: &str) -> Option<&'a ProtoDep> {
        deps.iter().find(|d| d.dep_name == name)
    }

    // Ported: "returns null for empty content" — proto/extract.spec.ts line 10
    #[test]
    fn returns_null_for_empty_content() {
        assert!(extract_package_file("").is_none());
    }

    // Ported: "returns null for invalid TOML" — proto/extract.spec.ts line 14
    #[test]
    fn returns_null_for_invalid_toml() {
        assert!(extract_package_file("{{invalid").is_none());
    }

    // Ported: "returns null when only config sections exist" — proto/extract.spec.ts line 18
    #[test]
    fn returns_null_when_only_config_sections() {
        let content = "[settings]\nauto-install = true\n\n[env]\nDEBUG = \"*\"\n";
        assert!(extract_package_file(content).is_none());
    }

    // Ported: "extracts a single tool version" — proto/extract.spec.ts line 29
    #[test]
    fn extracts_single_tool_version() {
        let deps = extract_package_file("node = \"22.14.0\"\n").unwrap();
        let node = dep_by_name(&deps, "node").unwrap();
        assert_eq!(node.current_value, "22.14.0");
        assert_eq!(node.datasource, Some("node-version"));
        assert_eq!(node.package_name, Some("node"));
        assert!(node.skip_reason.is_none());
    }

    // Ported: "extracts multiple tool versions" — proto/extract.spec.ts line 46
    #[test]
    fn extracts_multiple_tool_versions() {
        let content = "node = \"22.14.0\"\nbun = \"1.2.2\"\nnpm = \"11.6.2\"\n";
        let deps = extract_package_file(content).unwrap();
        let node = dep_by_name(&deps, "node").unwrap();
        assert_eq!(node.datasource, Some("node-version"));
        let bun = dep_by_name(&deps, "bun").unwrap();
        assert_eq!(bun.datasource, Some("github-releases"));
        assert_eq!(bun.package_name, Some("oven-sh/bun"));
        let npm = dep_by_name(&deps, "npm").unwrap();
        assert_eq!(npm.datasource, Some("npm"));
    }

    // Ported: "skips non-version sections" — proto/extract.spec.ts line 76
    #[test]
    fn skips_non_version_sections() {
        let content = "node = \"22.14.0\"\n\n[settings]\nauto-install = true\n\n[env]\nDEBUG = \"*\"\n";
        let deps = extract_package_file(content).unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "node");
    }

    // Ported: "handles proto self-versioning" — proto/extract.spec.ts line 105
    #[test]
    fn handles_proto_self_versioning() {
        let deps = extract_package_file("proto = \"0.56.0\"\n").unwrap();
        let proto = dep_by_name(&deps, "proto").unwrap();
        assert_eq!(proto.datasource, Some("github-releases"));
        assert_eq!(proto.package_name, Some("moonrepo/proto"));
    }

    // Ported: "handles moon tool" — proto/extract.spec.ts line 122
    #[test]
    fn handles_moon_tool() {
        let deps = extract_package_file("moon = \"1.30.0\"\n").unwrap();
        let moon = dep_by_name(&deps, "moon").unwrap();
        assert_eq!(moon.datasource, Some("github-releases"));
        assert_eq!(moon.package_name, Some("moonrepo/moon"));
    }

    // Ported: "handles uv tool" — proto/extract.spec.ts line 139
    #[test]
    fn handles_uv_tool() {
        let deps = extract_package_file("uv = \"0.6.0\"\n").unwrap();
        let uv = dep_by_name(&deps, "uv").unwrap();
        assert_eq!(uv.datasource, Some("github-releases"));
        assert_eq!(uv.package_name, Some("astral-sh/uv"));
    }

    // Ported: "marks unknown tools as unsupported-datasource" — proto/extract.spec.ts line 156
    #[test]
    fn marks_unknown_tools_as_unsupported_datasource() {
        let deps = extract_package_file("unknown-tool = \"1.0.0\"\n").unwrap();
        let dep = &deps[0];
        assert_eq!(dep.dep_name, "unknown-tool");
        assert_eq!(dep.skip_reason, Some("unsupported-datasource"));
    }

    // Ported: "skips alias values like latest" — proto/extract.spec.ts line 172
    #[test]
    fn skips_alias_values_like_latest() {
        let deps = extract_package_file("node = \"latest\"\n").unwrap();
        let dep = &deps[0];
        assert_eq!(dep.dep_name, "node");
        assert_eq!(dep.current_value, "latest");
        assert_eq!(dep.skip_reason, Some("unsupported-version"));
    }

    // Ported: "skips alias value stable" — proto/extract.spec.ts line 188
    #[test]
    fn skips_alias_value_stable() {
        let deps = extract_package_file("rust = \"stable\"\n").unwrap();
        let dep = &deps[0];
        assert_eq!(dep.skip_reason, Some("unsupported-version"));
    }

    // Ported: "handles partial versions" — proto/extract.spec.ts line 204
    #[test]
    fn handles_partial_versions() {
        let deps = extract_package_file("go = \"~1.22\"\n").unwrap();
        let go = dep_by_name(&deps, "go").unwrap();
        assert_eq!(go.current_value, "~1.22");
        assert_eq!(go.datasource, Some("github-tags"));
        assert_eq!(go.package_name, Some("golang/go"));
    }

    // Ported: "extracts all supported tools from fixture" — proto/extract.spec.ts line 221
    #[test]
    fn extracts_all_supported_tools_from_fixture() {
        let content = "node = \"22.14.0\"\nbun = \"1.2.2\"\nnpm = \"11.6.2\"\ngo = \"~1.22\"\nproto = \"0.56.0\"\n\n[settings]\nauto-install = true\n\n[env]\nDEBUG = \"*\"\n";
        let deps = extract_package_file(content).unwrap();
        assert_eq!(deps.len(), 5);
        assert!(dep_by_name(&deps, "node").is_some());
        assert!(dep_by_name(&deps, "bun").is_some());
        assert!(dep_by_name(&deps, "npm").is_some());
        assert!(dep_by_name(&deps, "go").is_some());
        assert!(dep_by_name(&deps, "proto").is_some());
    }

    // Ported: "extracts all supported built-in tools" — proto/extract.spec.ts line 278
    #[test]
    fn extracts_all_supported_builtin_tools() {
        let content = "bun = \"1.2.2\"\ndeno = \"2.0.0\"\ngo = \"1.22.0\"\nmoon = \"1.30.0\"\nnode = \"22.14.0\"\nnpm = \"11.6.2\"\npnpm = \"9.0.0\"\nyarn = \"4.0.0\"\npython = \"3.12.0\"\nruby = \"3.3.0\"\nrust = \"1.80.0\"\nproto = \"0.56.0\"\ngh = \"2.60.0\"\npoetry = \"1.8.0\"\nuv = \"0.6.0\"\n";
        let deps = extract_package_file(content).unwrap();
        assert_eq!(deps.len(), 15);
        for dep in &deps {
            assert!(dep.skip_reason.is_none(), "dep {} has skip_reason", dep.dep_name);
        }
    }
}
