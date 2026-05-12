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

static MISE_REGISTRY_TABLE: &[(&str, AsdfToolDef)] = &[
    (
        "actionlint",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("rhysd/actionlint"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "astro",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("astronomer/astro-cli"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "aws-cli",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_TAGS,
            package_name: Some("aws/aws-cli"),
            dep_name: None,
            extract_version: None,
            versioning: None,
        },
    ),
    (
        "aws-vault",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("99designs/aws-vault"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "buf",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("bufbuild/buf"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "caddy",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("caddyserver/caddy"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "ccache",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("ccache/ccache"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "clang-format",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("llvm/llvm-project"),
            dep_name: None,
            extract_version: Some("^llvmorg-(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "committed",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("crate-ci/committed"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "conan",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("conan-io/conan"),
            dep_name: None,
            extract_version: None,
            versioning: None,
        },
    ),
    (
        "consul",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("hashicorp/consul"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "gh",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("cli/cli"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "dotenv-linter",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("dotenv-linter/dotenv-linter"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "hivemind",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("DarthSim/hivemind"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "hk",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("jdx/hk"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "jq",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("jqlang/jq"),
            dep_name: None,
            extract_version: Some("^jq-(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "kafka",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_TAGS,
            package_name: Some("apache/kafka"),
            dep_name: None,
            extract_version: Some("^apache-(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "lefthook",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("evilmartians/lefthook"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "localstack",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("localstack/localstack"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "lychee",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("lycheeverse/lychee"),
            dep_name: None,
            extract_version: Some("^lychee-v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "npm",
        AsdfToolDef {
            datasource: datasource_id::NPM,
            package_name: Some("npm"),
            dep_name: None,
            extract_version: None,
            versioning: None,
        },
    ),
    (
        "opentofu",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("opentofu/opentofu"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "openfga",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("openfga/openfga"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "packer",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("hashicorp/packer"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "pipx",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("pypa/pipx"),
            dep_name: None,
            extract_version: None,
            versioning: None,
        },
    ),
    (
        "pkl",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("apple/pkl"),
            dep_name: None,
            extract_version: None,
            versioning: None,
        },
    ),
    (
        "protoc",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("protocolbuffers/protobuf"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "redis",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("redis/redis"),
            dep_name: None,
            extract_version: None,
            versioning: None,
        },
    ),
    (
        "ruff",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("astral-sh/ruff"),
            dep_name: None,
            extract_version: None,
            versioning: None,
        },
    ),
    (
        "rumdl",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("rvben/rumdl"),
            dep_name: None,
            extract_version: None,
            versioning: None,
        },
    ),
    (
        "shellcheck",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("koalaman/shellcheck"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "skeema",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("skeema/skeema"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "sops",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("getsops/sops"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "sqlite",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_TAGS,
            package_name: Some("sqlite/sqlite"),
            dep_name: None,
            extract_version: Some("^version-(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "stripe",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("stripe/stripe-cli"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "swiftformat",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("nicklockwood/SwiftFormat"),
            dep_name: None,
            extract_version: None,
            versioning: None,
        },
    ),
    (
        "swiftlint",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("realm/SwiftLint"),
            dep_name: None,
            extract_version: None,
            versioning: None,
        },
    ),
    (
        "taplo",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("tamasfe/taplo"),
            dep_name: None,
            extract_version: None,
            versioning: None,
        },
    ),
    (
        "tart",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("cirruslabs/tart"),
            dep_name: None,
            extract_version: None,
            versioning: None,
        },
    ),
    (
        "terragrunt",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("gruntwork-io/terragrunt"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "tilt",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("tilt-dev/tilt"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "tusd",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("tus/tusd"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "usage",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("jdx/usage"),
            dep_name: None,
            extract_version: Some("^v(?<version>\\S+)"),
            versioning: None,
        },
    ),
    (
        "zola",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_TAGS,
            package_name: Some("getzola/zola"),
            dep_name: None,
            extract_version: None,
            versioning: None,
        },
    ),
    (
        "magika",
        AsdfToolDef {
            datasource: "crate",
            package_name: Some("magika-cli"),
            dep_name: None,
            extract_version: None,
            versioning: None,
        },
    ),
    (
        "allurectl",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("allure-framework/allurectl"),
            dep_name: None,
            extract_version: None,
            versioning: None,
        },
    ),
    (
        "bitwarden-secrets-manager",
        AsdfToolDef {
            datasource: datasource_id::GITHUB_RELEASES,
            package_name: Some("bitwarden/sdk"),
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
    let current_value = version.strip_prefix(strip).unwrap_or(version);
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
        current_value: current_value.to_owned(),
        datasource: legacy,
        datasource_id: Some(def.datasource),
        package_name: pkg.map(str::to_owned),
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
        let Some((tool_raw, val_raw)) = split_tool_assignment(trimmed) else {
            continue;
        };

        let tool_name = tool_raw.trim().trim_matches('"').trim_matches('\'');
        let version_raw = val_raw.trim();

        let parsed_value = if version_raw.starts_with('"')
            || version_raw.starts_with('\'')
            || version_raw.starts_with('{')
            || version_raw.starts_with('[')
        {
            parse_tool_value(version_raw)
        } else {
            // Other formats — skip.
            continue;
        };

        // No version → UnspecifiedVersion.
        let Some(version) = parsed_value.version else {
            out.push(AsdfDep {
                tool_name: tool_name.to_owned(),
                dep_name: tool_name.to_owned(),
                current_value: String::new(),
                skip_reason: Some(AsdfSkipReason::UnspecifiedVersion),
                ..Default::default()
            });
            continue;
        };

        if let Some(dep) = resolve_backend_tool(tool_name, version, &parsed_value) {
            out.push(dep);
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

        if let Some((_, def)) = MISE_REGISTRY_TABLE.iter().find(|(k, _)| *k == tool_name) {
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

#[derive(Debug, Default)]
struct ParsedToolValue<'a> {
    version: Option<&'a str>,
    tag_regex: Option<String>,
    has_options: bool,
}

fn parse_tool_value(raw: &str) -> ParsedToolValue<'_> {
    if raw.starts_with('"') || raw.starts_with('\'') {
        let value = raw.trim_matches('"').trim_matches('\'').trim();
        return ParsedToolValue {
            version: (!value.is_empty()).then_some(value),
            ..Default::default()
        };
    }

    if raw.starts_with('[') {
        let inner = raw.trim_start_matches('[').trim_end_matches(']');
        let first = inner
            .split(',')
            .map(|value| value.trim().trim_matches('"').trim_matches('\'').trim())
            .find(|value| !value.is_empty());
        return ParsedToolValue {
            version: first,
            ..Default::default()
        };
    }

    let mut parsed = ParsedToolValue::default();
    let inner = raw.trim_start_matches('{').trim_end_matches('}');
    for kv in inner.split(',') {
        let Some((k, v)) = kv.trim().split_once('=') else {
            continue;
        };
        let key = k.trim();
        let value = v.trim().trim_matches('"').trim_matches('\'').trim();
        if key == "version" {
            if !value.is_empty() {
                parsed.version = Some(value);
            }
        } else {
            parsed.has_options = true;
            if key == "tag_regex" && !value.is_empty() {
                parsed.tag_regex = Some(unescape_toml_backslashes(value));
            }
        }
    }
    parsed
}

fn split_tool_assignment(line: &str) -> Option<(&str, &str)> {
    let mut quote = None;
    let mut escaped = false;

    for (idx, ch) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }

        if quote.is_some() && ch == '\\' {
            escaped = true;
            continue;
        }

        match quote {
            Some(q) if ch == q => quote = None,
            None if ch == '"' || ch == '\'' => quote = Some(ch),
            None if ch == '=' => return Some((&line[..idx], &line[idx + 1..])),
            _ => {}
        }
    }

    None
}

fn resolve_backend_tool(
    tool_name: &str,
    version: &str,
    parsed_value: &ParsedToolValue<'_>,
) -> Option<AsdfDep> {
    let (backend, name) = tool_name.split_once(':')?;
    match backend {
        "core" => MISE_CORE_TABLE
            .iter()
            .find(|(k, _)| *k == name)
            .map(|(_, def)| make_dep_from_def(tool_name, version, def)),
        "asdf" => asdf::TOOL_TABLE
            .iter()
            .find(|(k, _)| *k == name)
            .map(|(_, def)| make_dep_from_def(tool_name, version, def)),
        "vfox" if name == "scala" => Some(asdf::parse_scala_dep(tool_name, version)),
        "aqua" => asdf::TOOL_TABLE
            .iter()
            .find(|(k, _)| *k == name)
            .map(|(_, def)| make_dep_from_def(tool_name, version, def))
            .or_else(|| {
                Some(backend_dep(
                    tool_name,
                    version.trim_start_matches('v'),
                    "github-tags",
                    strip_tool_options(name),
                    Some("^v?(?<version>.+)"),
                ))
            }),
        "cargo" => {
            if name.starts_with("https://github.com/") {
                let (datasource, current_value) =
                    prefixed_git_ref(version).unwrap_or(("git-tags", version));
                Some(backend_dep(
                    tool_name,
                    current_value,
                    datasource,
                    name,
                    None,
                ))
            } else {
                Some(backend_dep(tool_name, version, "crate", name, None))
            }
        }
        "dotnet" => Some(backend_dep(tool_name, version, "nuget", name, None)),
        "gem" => Some(backend_dep(tool_name, version, "rubygems", name, None)),
        "go" => Some(backend_dep(tool_name, version, "go", name, None)),
        "npm" => Some(backend_dep(tool_name, version, "npm", name, None)),
        "pipx" => {
            if name.contains('/') || name.starts_with("git+https://github.com/") {
                Some(backend_dep(
                    tool_name,
                    version,
                    "github-tags",
                    github_package_name(name).as_deref().unwrap_or(name),
                    None,
                ))
            } else {
                Some(backend_dep(tool_name, version, "pypi", name, None))
            }
        }
        "spm" => Some(backend_dep(
            tool_name,
            version,
            "github-releases",
            github_package_name(name).as_deref().unwrap_or(name),
            None,
        )),
        "github" => Some(backend_dep(
            tool_name,
            version,
            "github-releases",
            strip_tool_options(name),
            None,
        )),
        "ubi" => Some(backend_dep(
            tool_name,
            version,
            "github-releases",
            strip_tool_options(name),
            ubi_extract_version(name, parsed_value),
        )),
        _ => None,
    }
}

fn backend_dep(
    dep_name: &str,
    current_value: &str,
    datasource: &'static str,
    package_name: &str,
    extract_version: Option<&'static str>,
) -> AsdfDep {
    AsdfDep {
        tool_name: dep_name.to_owned(),
        dep_name: dep_name.to_owned(),
        current_value: current_value.to_owned(),
        datasource_id: Some(datasource),
        package_name: Some(package_name.to_owned()),
        extract_version,
        skip_reason: None,
        ..Default::default()
    }
}

fn prefixed_git_ref(version: &str) -> Option<(&'static str, &str)> {
    version
        .strip_prefix("tag:")
        .map(|value| ("git-tags", value))
        .or_else(|| {
            version
                .strip_prefix("branch:")
                .map(|value| ("git-refs", value))
        })
        .or_else(|| {
            version
                .strip_prefix("rev:")
                .map(|value| ("git-refs", value))
        })
}

fn github_package_name(value: &str) -> Option<String> {
    let value = value.strip_prefix("git+").unwrap_or(value);
    let value = value
        .strip_prefix("https://github.com/")
        .unwrap_or(value)
        .trim_end_matches(".git");
    if value.contains('/') {
        Some(value.to_owned())
    } else {
        None
    }
}

fn strip_tool_options(value: &str) -> &str {
    value.split_once('[').map(|(name, _)| name).unwrap_or(value)
}

fn tool_option_value(value: &str, option: &str) -> Option<String> {
    let (_, options) = value.split_once('[')?;
    let options = options.strip_suffix(']').unwrap_or(options);
    options.split(',').find_map(|kv| {
        let (key, value) = kv.trim().split_once('=')?;
        (key.trim() == option).then(|| unescape_toml_backslashes(value.trim()))
    })
}

fn unescape_toml_backslashes(value: &str) -> String {
    value.replace("\\\\", "\\")
}

fn ubi_extract_version(name: &str, parsed_value: &ParsedToolValue<'_>) -> Option<&'static str> {
    if let Some(tag_regex) = parsed_value
        .tag_regex
        .clone()
        .or_else(|| tool_option_value(name, "tag_regex"))
    {
        return Some(Box::leak(
            format!("^v?(?<version>{tag_regex})").into_boxed_str(),
        ));
    }

    if parsed_value.has_options || name.contains('[') {
        return Some("^v?(?<version>.+)");
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "extracts tools - mise core plugins" — mise/extract.spec.ts line 28
    #[test]
    fn extracts_node_version() {
        let content = "[tools]\nnode = \"18\"\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].tool_name, "node");
        assert_eq!(deps[0].current_value, "18");
        assert_eq!(deps[0].datasource_id, Some("node-version"));
        assert!(deps[0].datasource.is_none()); // not a GitHub tool
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "extracts tools - mise core plugins" — mise/extract.spec.ts line 28
    #[test]
    fn extracts_erlang_core_plugin() {
        let content = "[tools]\nerlang = \"23.3\"\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let d = &deps[0];
        assert_eq!(d.tool_name, "erlang");
        assert_eq!(d.current_value, "23.3");
        assert_eq!(d.datasource_id, Some("github-tags"));
        assert_eq!(d.package_name.as_deref(), Some("erlang/otp"));
        assert_eq!(d.extract_version, Some("^OTP-(?<version>\\S+)"));
        assert!(d.skip_reason.is_none());
    }

    // Ported: "extracts tools - mise core plugins" — mise/extract.spec.ts line 28
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

    // Ported: "extracts tools - mise registry tools" — mise/extract.spec.ts line 51
    #[test]
    fn extracts_mise_registry_tools() {
        let content = r#"[tools]
actionlint = "1.7.7"
astro = "1.34.0"
aws-cli = "2.25.10"
aws-vault = "6.6.1"
buf = "1.27.0"
caddy = "2.10.2"
ccache = "4.11.3"
clang-format = "20.1.0"
committed = "1.1.7"
conan = "2.24.0"
consul = "1.14.3"
gh = "2.87.0"
dotenv-linter = "3.3.0"
hivemind = "1.1.0"
hk = "1.1.2"
jq = "1.7.1"
kafka = "apache-3.9.0"
lefthook = "1.11.13"
localstack = "4.3.0"
lychee = "0.19.1"
npm = "11.2.0"
opentofu = "1.6.1"
openfga = "1.14.0"
packer = "1.15.0"
pipx = "1.7.1"
pkl = "0.28.2"
protoc = "30.2"
redis = "8.0.1"
ruff = "0.11.12"
rumdl = "0.1.58"
shellcheck = "0.10.0"
skeema = "1.12.3"
sops = "3.10.2"
sqlite = "3.50.1"
stripe = "1.25.0"
swiftformat = "0.58.0"
swiftlint = "0.55.1"
taplo = "0.10.0"
tart = "2.31.0"
terragrunt = "0.72.6"
tilt = "0.34.0"
tusd = "2.8.0"
usage = "2.1.1"
"#;
        let deps = extract(content);
        let expected = [
            (
                "actionlint",
                "1.7.7",
                "github-releases",
                "rhysd/actionlint",
                Some("^v(?<version>\\S+)"),
            ),
            (
                "astro",
                "1.34.0",
                "github-releases",
                "astronomer/astro-cli",
                Some("^v(?<version>\\S+)"),
            ),
            ("aws-cli", "2.25.10", "github-tags", "aws/aws-cli", None),
            (
                "aws-vault",
                "6.6.1",
                "github-releases",
                "99designs/aws-vault",
                Some("^v(?<version>\\S+)"),
            ),
            (
                "buf",
                "1.27.0",
                "github-releases",
                "bufbuild/buf",
                Some("^v(?<version>\\S+)"),
            ),
            (
                "caddy",
                "2.10.2",
                "github-releases",
                "caddyserver/caddy",
                Some("^v(?<version>\\S+)"),
            ),
            (
                "ccache",
                "4.11.3",
                "github-releases",
                "ccache/ccache",
                Some("^v(?<version>\\S+)"),
            ),
            (
                "clang-format",
                "20.1.0",
                "github-releases",
                "llvm/llvm-project",
                Some("^llvmorg-(?<version>\\S+)"),
            ),
            (
                "committed",
                "1.1.7",
                "github-releases",
                "crate-ci/committed",
                Some("^v(?<version>\\S+)"),
            ),
            ("conan", "2.24.0", "github-releases", "conan-io/conan", None),
            (
                "consul",
                "1.14.3",
                "github-releases",
                "hashicorp/consul",
                Some("^v(?<version>\\S+)"),
            ),
            (
                "gh",
                "2.87.0",
                "github-releases",
                "cli/cli",
                Some("^v(?<version>\\S+)"),
            ),
            (
                "dotenv-linter",
                "3.3.0",
                "github-releases",
                "dotenv-linter/dotenv-linter",
                Some("^v(?<version>\\S+)"),
            ),
            (
                "hivemind",
                "1.1.0",
                "github-releases",
                "DarthSim/hivemind",
                Some("^v(?<version>\\S+)"),
            ),
            (
                "hk",
                "1.1.2",
                "github-releases",
                "jdx/hk",
                Some("^v(?<version>\\S+)"),
            ),
            (
                "jq",
                "1.7.1",
                "github-releases",
                "jqlang/jq",
                Some("^jq-(?<version>\\S+)"),
            ),
            (
                "kafka",
                "3.9.0",
                "github-tags",
                "apache/kafka",
                Some("^apache-(?<version>\\S+)"),
            ),
            (
                "lefthook",
                "1.11.13",
                "github-releases",
                "evilmartians/lefthook",
                Some("^v(?<version>\\S+)"),
            ),
            (
                "localstack",
                "4.3.0",
                "github-releases",
                "localstack/localstack",
                Some("^v(?<version>\\S+)"),
            ),
            (
                "lychee",
                "0.19.1",
                "github-releases",
                "lycheeverse/lychee",
                Some("^lychee-v(?<version>\\S+)"),
            ),
            ("npm", "11.2.0", "npm", "npm", None),
            (
                "opentofu",
                "1.6.1",
                "github-releases",
                "opentofu/opentofu",
                Some("^v(?<version>\\S+)"),
            ),
            (
                "openfga",
                "1.14.0",
                "github-releases",
                "openfga/openfga",
                Some("^v(?<version>\\S+)"),
            ),
            (
                "packer",
                "1.15.0",
                "github-releases",
                "hashicorp/packer",
                Some("^v(?<version>\\S+)"),
            ),
            ("pipx", "1.7.1", "github-releases", "pypa/pipx", None),
            ("pkl", "0.28.2", "github-releases", "apple/pkl", None),
            (
                "protoc",
                "30.2",
                "github-releases",
                "protocolbuffers/protobuf",
                Some("^v(?<version>\\S+)"),
            ),
            ("redis", "8.0.1", "github-releases", "redis/redis", None),
            ("ruff", "0.11.12", "github-releases", "astral-sh/ruff", None),
            ("rumdl", "0.1.58", "github-releases", "rvben/rumdl", None),
            (
                "shellcheck",
                "0.10.0",
                "github-releases",
                "koalaman/shellcheck",
                Some("^v(?<version>\\S+)"),
            ),
            (
                "skeema",
                "1.12.3",
                "github-releases",
                "skeema/skeema",
                Some("^v(?<version>\\S+)"),
            ),
            (
                "sops",
                "3.10.2",
                "github-releases",
                "getsops/sops",
                Some("^v(?<version>\\S+)"),
            ),
            (
                "sqlite",
                "3.50.1",
                "github-tags",
                "sqlite/sqlite",
                Some("^version-(?<version>\\S+)"),
            ),
            (
                "stripe",
                "1.25.0",
                "github-releases",
                "stripe/stripe-cli",
                Some("^v(?<version>\\S+)"),
            ),
            (
                "swiftformat",
                "0.58.0",
                "github-releases",
                "nicklockwood/SwiftFormat",
                None,
            ),
            (
                "swiftlint",
                "0.55.1",
                "github-releases",
                "realm/SwiftLint",
                None,
            ),
            ("taplo", "0.10.0", "github-releases", "tamasfe/taplo", None),
            ("tart", "2.31.0", "github-releases", "cirruslabs/tart", None),
            (
                "terragrunt",
                "0.72.6",
                "github-releases",
                "gruntwork-io/terragrunt",
                Some("^v(?<version>\\S+)"),
            ),
            (
                "tilt",
                "0.34.0",
                "github-releases",
                "tilt-dev/tilt",
                Some("^v(?<version>\\S+)"),
            ),
            (
                "tusd",
                "2.8.0",
                "github-releases",
                "tus/tusd",
                Some("^v(?<version>\\S+)"),
            ),
            (
                "usage",
                "2.1.1",
                "github-releases",
                "jdx/usage",
                Some("^v(?<version>\\S+)"),
            ),
        ];
        assert_eq!(deps.len(), expected.len());
        for (dep, (name, current_value, datasource, package_name, extract_version)) in
            deps.iter().zip(expected)
        {
            assert_eq!(dep.dep_name, name);
            assert_eq!(dep.current_value, current_value);
            assert_eq!(dep.datasource_id, Some(datasource));
            assert_eq!(dep.package_name.as_deref(), Some(package_name));
            assert_eq!(dep.extract_version, extract_version);
            assert!(dep.skip_reason.is_none());
        }
    }

    // Ported: "extracts tools - asdf plugins" — mise/extract.spec.ts line 393
    #[test]
    fn asdf_tools_fall_through_to_asdf_table() {
        // Tools not in mise core but in asdf table should still be resolved.
        let content = "[tools]\nterraform = \"1.6.3\"\nhelm = \"3.13.1\"\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        let tf = deps.iter().find(|d| d.tool_name == "terraform").unwrap();
        assert_eq!(tf.datasource_id, Some("github-releases"));
        assert_eq!(tf.package_name.as_deref(), Some("hashicorp/terraform"));
    }

    // Ported: "provides skipReason for lines with unsupported tooling" — mise/extract.spec.ts line 781
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

    // Ported: "extracts tools with multiple versions" — mise/extract.spec.ts line 409
    #[test]
    fn ignores_array_versions() {
        let content = "[tools]\nnode = [\"18\", \"20\"]\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].tool_name, "node");
        assert_eq!(deps[0].current_value, "18");
        assert_eq!(deps[0].datasource_id, Some("node-version"));
    }

    // Ported: "returns null for empty" — mise/extract.spec.ts line 13
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "core java plugin function" — mise/extract.spec.ts line 911
    #[test]
    fn java_core_plugin_jdk() {
        let content = "[tools]\njava = \"adoptopenjdk-16.0.0+36\"\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let d = &deps[0];
        assert_eq!(d.datasource_id, Some("java-version"));
        assert_eq!(d.package_name.as_deref(), Some("java-jdk"));
        assert_eq!(d.current_value, "16.0.0+36");
        assert!(d.skip_reason.is_none());
    }

    // Ported: "returns null for invalid TOML" — mise/extract.spec.ts line 17
    #[test]
    fn invalid_toml_returns_empty() {
        assert!(extract("foo").is_empty());
    }

    // Ported: "returns null for empty tools section" — mise/extract.spec.ts line 21
    #[test]
    fn empty_tools_section_returns_empty() {
        assert!(extract("[tools]\n").is_empty());
    }

    // Ported: "provides skipReason for missing version - empty string" — mise/extract.spec.ts line 802
    #[test]
    fn empty_version_string_skipped() {
        let content = "[tools]\npython = ''\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert!(deps[0].skip_reason.is_some());
    }

    // Ported: "provides skipReason for missing version - missing version in object" — mise/extract.spec.ts line 818
    #[test]
    fn object_without_version_skipped() {
        let content = "[tools]\npython = {virtualenv='.venv'}\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert!(deps[0].skip_reason.is_some());
    }

    // Ported: "provides skipReason for missing version - empty array" — mise/extract.spec.ts line 834
    #[test]
    fn empty_array_version_skipped() {
        let content = "[tools]\njava = '21.0.2'\nerlang = []\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[1].tool_name, "erlang");
        assert_eq!(
            deps[1].skip_reason,
            Some(AsdfSkipReason::UnspecifiedVersion)
        );
    }

    // Ported: "complete mise.toml example" — mise/extract.spec.ts line 855
    #[test]
    fn complete_mise_toml_example() {
        let content = r#"[env]
NODE_ENV = 'production'

[tools]
java = '21.0.2'
erlang = ['23.3', '24.0']
node = ['16', 'prefix:20', 'ref:master', 'path:~/.nodes/14']

[plugins]
python = 'https://github.com/asdf-community/asdf-python'

[alias.node]
my_custom_node = '20'
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
        assert_eq!(deps[0].dep_name, "java");
        assert_eq!(deps[0].current_value, "21.0.2");
        assert_eq!(deps[0].datasource_id, Some("java-version"));
        assert_eq!(deps[1].dep_name, "erlang");
        assert_eq!(deps[1].current_value, "23.3");
        assert_eq!(deps[1].datasource_id, Some("github-tags"));
        assert_eq!(deps[2].dep_name, "node");
        assert_eq!(deps[2].current_value, "16");
        assert_eq!(deps[2].datasource_id, Some("node-version"));
    }

    // Ported: "complete example with skip" — mise/extract.spec.ts line 878
    #[test]
    fn complete_mise_example_with_skip() {
        let content = r#"[tools]
java = '21.0.2'
erlang = ['23.3', '24.0']
terraform = {version='1.8.0'}
fake-tool = '1.6.2'
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 4);
        assert_eq!(deps[0].dep_name, "java");
        assert_eq!(deps[0].current_value, "21.0.2");
        assert_eq!(deps[0].datasource_id, Some("java-version"));
        assert_eq!(deps[1].dep_name, "erlang");
        assert_eq!(deps[1].current_value, "23.3");
        assert_eq!(deps[1].datasource_id, Some("github-tags"));
        assert_eq!(deps[2].dep_name, "terraform");
        assert_eq!(deps[2].current_value, "1.8.0");
        assert_eq!(deps[3].dep_name, "fake-tool");
        assert_eq!(deps[3].skip_reason, Some(AsdfSkipReason::UnsupportedTool));
    }

    // Ported: "extracts tools with plugin options" — mise/extract.spec.ts line 432
    #[test]
    fn tool_with_version_object() {
        let content = "[tools]\npython = {version = \"3.12.3\"}\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "3.12.3");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "extracts tools in the default registry with backends" — mise/extract.spec.ts line 448
    #[test]
    fn extracts_default_registry_backend_prefixed_tools() {
        let content = r#"[tools]
"core:node" = "16"
"asdf:rust" = "1.82.0"
"vfox:scala" = "3.5.2"
"aqua:act" = "0.2.70"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 4);

        let node = deps.iter().find(|dep| dep.dep_name == "core:node").unwrap();
        assert_eq!(node.current_value, "16");
        assert_eq!(node.package_name.as_deref(), Some("node"));
        assert_eq!(node.datasource_id, Some("node-version"));

        let rust = deps.iter().find(|dep| dep.dep_name == "asdf:rust").unwrap();
        assert_eq!(rust.current_value, "1.82.0");
        assert_eq!(rust.package_name.as_deref(), Some("rust-lang/rust"));
        assert_eq!(rust.datasource_id, Some("github-tags"));

        let scala = deps
            .iter()
            .find(|dep| dep.dep_name == "vfox:scala")
            .unwrap();
        assert_eq!(scala.current_value, "3.5.2");
        assert_eq!(scala.package_name.as_deref(), Some("lampepfl/dotty"));
        assert_eq!(scala.datasource_id, Some("github-tags"));

        let act = deps.iter().find(|dep| dep.dep_name == "aqua:act").unwrap();
        assert_eq!(act.current_value, "0.2.70");
        assert_eq!(act.package_name.as_deref(), Some("nektos/act"));
        assert_eq!(act.datasource_id, Some("github-releases"));
    }

    // Ported: "extracts aqua backend tool" — mise/extract.spec.ts line 487
    #[test]
    fn extracts_aqua_backend_tools() {
        let content = r#"[tools]
"aqua:BurntSushi/ripgrep" = "14.1.0"
"aqua:cli/cli" = "v2.64.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        let ripgrep = deps
            .iter()
            .find(|dep| dep.dep_name == "aqua:BurntSushi/ripgrep")
            .unwrap();
        assert_eq!(ripgrep.current_value, "14.1.0");
        assert_eq!(ripgrep.package_name.as_deref(), Some("BurntSushi/ripgrep"));
        assert_eq!(ripgrep.datasource_id, Some("github-tags"));
        assert_eq!(ripgrep.extract_version, Some("^v?(?<version>.+)"));

        let gh = deps
            .iter()
            .find(|dep| dep.dep_name == "aqua:cli/cli")
            .unwrap();
        assert_eq!(gh.current_value, "2.64.0");
        assert_eq!(gh.package_name.as_deref(), Some("cli/cli"));
        assert_eq!(gh.datasource_id, Some("github-tags"));
    }

    // Ported: "extracts cargo backend tools" — mise/extract.spec.ts line 514
    #[test]
    fn extracts_cargo_backend_tools() {
        let content = r#"[tools]
"cargo:eza" = "0.18.21"
"cargo:https://github.com/username/demo1" = "tag:v0.1.0"
"cargo:https://github.com/username/demo2" = "branch:main"
"cargo:https://github.com/username/demo3" = "rev:abcdef"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 4);
        let eza = deps.iter().find(|dep| dep.dep_name == "cargo:eza").unwrap();
        assert_eq!(eza.current_value, "0.18.21");
        assert_eq!(eza.package_name.as_deref(), Some("eza"));
        assert_eq!(eza.datasource_id, Some("crate"));

        let tag = deps
            .iter()
            .find(|dep| dep.dep_name == "cargo:https://github.com/username/demo1")
            .unwrap();
        assert_eq!(tag.current_value, "v0.1.0");
        assert_eq!(
            tag.package_name.as_deref(),
            Some("https://github.com/username/demo1")
        );
        assert_eq!(tag.datasource_id, Some("git-tags"));

        let branch = deps
            .iter()
            .find(|dep| dep.dep_name == "cargo:https://github.com/username/demo2")
            .unwrap();
        assert_eq!(branch.current_value, "main");
        assert_eq!(branch.datasource_id, Some("git-refs"));

        let rev = deps
            .iter()
            .find(|dep| dep.dep_name == "cargo:https://github.com/username/demo3")
            .unwrap();
        assert_eq!(rev.current_value, "abcdef");
        assert_eq!(rev.datasource_id, Some("git-refs"));
    }

    // Ported: "extracts dotnet backend tool" — mise/extract.spec.ts line 553
    #[test]
    fn extracts_dotnet_backend_tool() {
        let deps = extract("[tools]\n\"dotnet:GitVersion.Tool\" = \"5.12.0\"\n");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "5.12.0");
        assert_eq!(deps[0].package_name.as_deref(), Some("GitVersion.Tool"));
        assert_eq!(deps[0].datasource_id, Some("nuget"));
    }

    // Ported: "extracts gem backend tool" — mise/extract.spec.ts line 571
    #[test]
    fn extracts_gem_backend_tool() {
        let deps = extract("[tools]\n\"gem:rubocop\" = \"1.69.2\"\n");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "1.69.2");
        assert_eq!(deps[0].package_name.as_deref(), Some("rubocop"));
        assert_eq!(deps[0].datasource_id, Some("rubygems"));
    }

    // Ported: "extracts go backend tool" — mise/extract.spec.ts line 589
    #[test]
    fn extracts_go_backend_tool() {
        let deps = extract("[tools]\n\"go:github.com/DarthSim/hivemind\" = \"1.0.6\"\n");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "1.0.6");
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("github.com/DarthSim/hivemind")
        );
        assert_eq!(deps[0].datasource_id, Some("go"));
    }

    // Ported: "extracts npm backend tool" — mise/extract.spec.ts line 607
    #[test]
    fn extracts_npm_backend_tool() {
        let deps = extract("[tools]\n\"npm:prettier\" = \"3.3.2\"\n");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "3.3.2");
        assert_eq!(deps[0].package_name.as_deref(), Some("prettier"));
        assert_eq!(deps[0].datasource_id, Some("npm"));
    }

    // Ported: "extracts pipx backend tools" — mise/extract.spec.ts line 625
    #[test]
    fn extracts_pipx_backend_tools() {
        let content = r#"[tools]
"pipx:yamllint" = "1.35.0"
"pipx:psf/black" = "24.4.1"
"pipx:git+https://github.com/psf/black.git" = "24.4.1"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
        let yamllint = deps
            .iter()
            .find(|dep| dep.dep_name == "pipx:yamllint")
            .unwrap();
        assert_eq!(yamllint.package_name.as_deref(), Some("yamllint"));
        assert_eq!(yamllint.datasource_id, Some("pypi"));

        let black = deps
            .iter()
            .find(|dep| dep.dep_name == "pipx:psf/black")
            .unwrap();
        assert_eq!(black.package_name.as_deref(), Some("psf/black"));
        assert_eq!(black.datasource_id, Some("github-tags"));

        let git_black = deps
            .iter()
            .find(|dep| dep.dep_name == "pipx:git+https://github.com/psf/black.git")
            .unwrap();
        assert_eq!(git_black.package_name.as_deref(), Some("psf/black"));
        assert_eq!(git_black.datasource_id, Some("github-tags"));
    }

    // Ported: "extracts spm backend tools" — mise/extract.spec.ts line 657
    #[test]
    fn extracts_spm_backend_tools() {
        let content = r#"[tools]
"spm:tuist/tuist" = "4.15.0"
"spm:https://github.com/tuist/tuist.git" = "4.13.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(
            deps.iter()
                .all(|dep| dep.datasource_id == Some("github-releases"))
        );
        assert!(
            deps.iter()
                .any(|dep| dep.package_name.as_deref() == Some("tuist/tuist")
                    && dep.current_value == "4.15.0")
        );
        assert!(
            deps.iter()
                .any(|dep| dep.package_name.as_deref() == Some("tuist/tuist")
                    && dep.current_value == "4.13.0")
        );
    }

    // Ported: "extracts ubi backend tools" — mise/extract.spec.ts line 682
    #[test]
    fn extracts_ubi_backend_tools() {
        let content = r#"[tools]
"ubi:nekto/act" = "v0.2.70"
"ubi:cli/cli" = { exe = "gh", version = "1.14.0" }
"ubi:cli/cli[exe=gh]" = "1.14.0"
"ubi:cargo-bins/cargo-binstall" = { tag_regex = "^\\d+\\.\\d+\\.", version = "1.0.0" }
"ubi:cargo-bins/cargo-binstall[tag_regex=^\\d+\\.]" = "1.0.0"
"ubi:cargo-bins/cargo-binstall[tag_regex=^\\d+\\.\\d+\\.]" = { tag_regex = "^\\d+\\.", version = "1.0.0" }
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 6);
        assert!(
            deps.iter()
                .all(|dep| dep.datasource_id == Some("github-releases"))
        );

        let act = deps
            .iter()
            .find(|dep| dep.dep_name == "ubi:nekto/act")
            .unwrap();
        assert_eq!(act.current_value, "v0.2.70");
        assert_eq!(act.package_name.as_deref(), Some("nekto/act"));
        assert!(act.extract_version.is_none());

        let gh_object = deps
            .iter()
            .find(|dep| dep.dep_name == "ubi:cli/cli")
            .unwrap();
        assert_eq!(gh_object.current_value, "1.14.0");
        assert_eq!(gh_object.package_name.as_deref(), Some("cli/cli"));
        assert_eq!(gh_object.extract_version, Some("^v?(?<version>.+)"));

        let gh_bracket = deps
            .iter()
            .find(|dep| dep.dep_name == "ubi:cli/cli[exe=gh]")
            .unwrap();
        assert_eq!(gh_bracket.package_name.as_deref(), Some("cli/cli"));
        assert_eq!(gh_bracket.extract_version, Some("^v?(?<version>.+)"));

        let table_regex = deps
            .iter()
            .find(|dep| dep.dep_name == "ubi:cargo-bins/cargo-binstall")
            .unwrap();
        assert_eq!(
            table_regex.extract_version,
            Some("^v?(?<version>^\\d+\\.\\d+\\.)")
        );

        let key_regex = deps
            .iter()
            .find(|dep| dep.dep_name == "ubi:cargo-bins/cargo-binstall[tag_regex=^\\\\d+\\\\.]")
            .unwrap();
        assert_eq!(key_regex.extract_version, Some("^v?(?<version>^\\d+\\.)"));

        let overridden_regex = deps
            .iter()
            .find(|dep| {
                dep.dep_name == "ubi:cargo-bins/cargo-binstall[tag_regex=^\\\\d+\\\\.\\\\d+\\\\.]"
            })
            .unwrap();
        assert_eq!(
            overridden_regex.extract_version,
            Some("^v?(?<version>^\\d+\\.)")
        );
    }

    // Ported: "extracts github backend tools" — mise/extract.spec.ts line 740
    #[test]
    fn extracts_github_backend_tools() {
        let content = r#"[tools]
"github:BurntSushi/ripgrep" = "14.1.1"
"github:cli/cli" = "v2.64.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        let ripgrep = deps
            .iter()
            .find(|dep| dep.dep_name == "github:BurntSushi/ripgrep")
            .unwrap();
        assert_eq!(ripgrep.current_value, "14.1.1");
        assert_eq!(ripgrep.package_name.as_deref(), Some("BurntSushi/ripgrep"));
        assert_eq!(ripgrep.datasource_id, Some("github-releases"));

        let gh = deps
            .iter()
            .find(|dep| dep.dep_name == "github:cli/cli")
            .unwrap();
        assert_eq!(gh.current_value, "v2.64.0");
        assert_eq!(gh.package_name.as_deref(), Some("cli/cli"));
        assert_eq!(gh.datasource_id, Some("github-releases"));
    }

    // Ported: "resolves tools from the mise registry data file via aqua backend" — mise/extract.spec.ts line 1086
    #[test]
    fn resolves_mise_registry_aqua_backend_tool() {
        let deps = extract("[tools]\nzola = \"0.19.2\"\n");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "zola");
        assert_eq!(deps[0].current_value, "0.19.2");
        assert_eq!(deps[0].datasource_id, Some("github-tags"));
        assert_eq!(deps[0].package_name.as_deref(), Some("getzola/zola"));
    }

    // Ported: "resolves tools from the mise registry data file via cargo backend" — mise/extract.spec.ts line 1104
    #[test]
    fn resolves_mise_registry_cargo_backend_tool() {
        let deps = extract("[tools]\nmagika = \"0.3.1\"\n");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "magika");
        assert_eq!(deps[0].current_value, "0.3.1");
        assert_eq!(deps[0].datasource_id, Some("crate"));
        assert_eq!(deps[0].package_name.as_deref(), Some("magika-cli"));
    }

    // Ported: "resolves tools from the mise registry data file via github backend" — mise/extract.spec.ts line 1122
    #[test]
    fn resolves_mise_registry_github_backend_tool() {
        let deps = extract("[tools]\nallurectl = \"2.14.0\"\n");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "allurectl");
        assert_eq!(deps[0].current_value, "2.14.0");
        assert_eq!(deps[0].datasource_id, Some("github-releases"));
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("allure-framework/allurectl")
        );
    }

    // Ported: "resolves a tool from the mise registry, prioritising the github backend over others" — mise/extract.spec.ts line 1140
    #[test]
    fn resolves_mise_registry_prefers_github_backend_tool() {
        let deps = extract("[tools]\nbitwarden-secrets-manager = \"1.2.3\"\n");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "bitwarden-secrets-manager");
        assert_eq!(deps[0].current_value, "1.2.3");
        assert_eq!(deps[0].datasource_id, Some("github-releases"));
        assert_eq!(deps[0].package_name.as_deref(), Some("bitwarden/sdk"));
    }
}
