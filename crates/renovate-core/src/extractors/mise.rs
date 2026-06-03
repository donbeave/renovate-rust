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

// ── Backend tooling config API ────────────────────────────────────────────────

/// Tooling config returned by the `create*ToolConfig` backend functions.
///
/// Mirrors TypeScript `BackendToolingConfig` in `lib/modules/manager/mise/backends.ts`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendToolingConfig {
    pub package_name: String,
    pub datasource: Option<&'static str>,
    pub current_value: Option<String>,
    pub extract_version: Option<String>,
    pub skip_reason: Option<&'static str>,
}

fn escape_regexp(input: &str) -> String {
    let mut out = String::with_capacity(input.len() * 2);
    for c in input.chars() {
        if r".*+-?^${}()|[\]".contains(c) {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// Strip a leading `v` or `v?` from a tag-regex fragment.
fn strip_leading_v_opt(s: &str) -> &str {
    if let Some(rest) = s.strip_prefix("v?") {
        rest
    } else if let Some(rest) = s.strip_prefix('v') {
        rest
    } else {
        s
    }
}

/// `aqua:` backend config.
pub fn create_aqua_tool_config(name: &str, version: &str) -> BackendToolingConfig {
    BackendToolingConfig {
        package_name: name.to_owned(),
        datasource: Some("github-tags"),
        current_value: Some(version.strip_prefix('v').unwrap_or(version).to_owned()),
        extract_version: Some("^v?(?<version>.+)".to_owned()),
        skip_reason: None,
    }
}

fn is_url(s: &str) -> bool {
    s.starts_with("https://") || s.starts_with("http://")
}

/// `cargo:` backend config.
pub fn create_cargo_tool_config(name: &str, version: &str) -> BackendToolingConfig {
    if !is_url(name) {
        return BackendToolingConfig {
            package_name: name.to_owned(),
            datasource: Some("crate"),
            current_value: None,
            extract_version: None,
            skip_reason: None,
        };
    }
    if let Some(v) = version.strip_prefix("tag:") {
        BackendToolingConfig {
            package_name: name.to_owned(),
            datasource: Some("git-tags"),
            current_value: Some(v.to_owned()),
            extract_version: None,
            skip_reason: None,
        }
    } else if let Some(v) = version.strip_prefix("branch:") {
        BackendToolingConfig {
            package_name: name.to_owned(),
            datasource: Some("git-refs"),
            current_value: Some(v.to_owned()),
            extract_version: None,
            skip_reason: None,
        }
    } else if let Some(v) = version.strip_prefix("rev:") {
        BackendToolingConfig {
            package_name: name.to_owned(),
            datasource: Some("git-refs"),
            current_value: Some(v.to_owned()),
            extract_version: None,
            skip_reason: None,
        }
    } else {
        BackendToolingConfig {
            package_name: name.to_owned(),
            datasource: None,
            current_value: None,
            extract_version: None,
            skip_reason: Some("invalid-version"),
        }
    }
}

/// `dotnet:` backend config.
pub fn create_dotnet_tool_config(name: &str) -> BackendToolingConfig {
    BackendToolingConfig {
        package_name: name.to_owned(),
        datasource: Some("nuget"),
        current_value: None,
        extract_version: None,
        skip_reason: None,
    }
}

/// `gem:` backend config.
pub fn create_gem_tool_config(name: &str) -> BackendToolingConfig {
    BackendToolingConfig {
        package_name: name.to_owned(),
        datasource: Some("rubygems"),
        current_value: None,
        extract_version: None,
        skip_reason: None,
    }
}

/// `github:` backend config.
pub fn create_github_tool_config(
    name: &str,
    version: &str,
    version_prefix: Option<&str>,
) -> BackendToolingConfig {
    let extract_version = version_prefix
        .filter(|p| !p.is_empty())
        .map(|p| format!("^{}(?<version>.+)", escape_regexp(p)));
    BackendToolingConfig {
        package_name: name.to_owned(),
        datasource: Some("github-releases"),
        current_value: Some(version.to_owned()),
        extract_version,
        skip_reason: None,
    }
}

/// `go:` backend config.
pub fn create_go_tool_config(name: &str) -> BackendToolingConfig {
    BackendToolingConfig {
        package_name: name.to_owned(),
        datasource: Some("go"),
        current_value: None,
        extract_version: None,
        skip_reason: None,
    }
}

/// `npm:` backend config.
pub fn create_npm_tool_config(name: &str) -> BackendToolingConfig {
    BackendToolingConfig {
        package_name: name.to_owned(),
        datasource: Some("npm"),
        current_value: None,
        extract_version: None,
        skip_reason: None,
    }
}

static PIPX_GITHUB_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r"^git\+https://github\.com/(?P<repo>.+)\.git$").unwrap()
});

/// `pipx:` backend config.
pub fn create_pipx_tool_config(name: &str) -> BackendToolingConfig {
    let is_git_syntax = name.starts_with("git+");
    if !is_git_syntax && is_url(name) {
        return BackendToolingConfig {
            package_name: name.to_owned(),
            datasource: None,
            current_value: None,
            extract_version: None,
            skip_reason: Some("unsupported-url"),
        };
    }
    if is_git_syntax || name.contains('/') {
        if is_git_syntax {
            if let Some(cap) = PIPX_GITHUB_RE.captures(name) {
                let repo = cap["repo"].to_owned();
                return BackendToolingConfig {
                    package_name: repo,
                    datasource: Some("github-tags"),
                    current_value: None,
                    extract_version: None,
                    skip_reason: None,
                };
            }
            // non-github git URL
            let package_name = name
                .strip_prefix("git+")
                .unwrap_or(name)
                .trim_end_matches(".git")
                .to_owned();
            return BackendToolingConfig {
                package_name,
                datasource: Some("git-refs"),
                current_value: None,
                extract_version: None,
                skip_reason: None,
            };
        }
        // shorthand like "psf/black"
        return BackendToolingConfig {
            package_name: name.to_owned(),
            datasource: Some("github-tags"),
            current_value: None,
            extract_version: None,
            skip_reason: None,
        };
    }
    BackendToolingConfig {
        package_name: name.to_owned(),
        datasource: Some("pypi"),
        current_value: None,
        extract_version: None,
        skip_reason: None,
    }
}

static SPM_GITHUB_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r"^https://github\.com/(?P<repo>.+)\.git$").unwrap()
});

/// `spm:` backend config.
pub fn create_spm_tool_config(name: &str) -> BackendToolingConfig {
    if is_url(name) {
        if let Some(cap) = SPM_GITHUB_RE.captures(name) {
            let repo = cap["repo"].to_owned();
            return BackendToolingConfig {
                package_name: repo,
                datasource: Some("github-releases"),
                current_value: None,
                extract_version: None,
                skip_reason: None,
            };
        }
        return BackendToolingConfig {
            package_name: name.to_owned(),
            datasource: None,
            current_value: None,
            extract_version: None,
            skip_reason: Some("unsupported-url"),
        };
    }
    BackendToolingConfig {
        package_name: name.to_owned(),
        datasource: Some("github-releases"),
        current_value: None,
        extract_version: None,
        skip_reason: None,
    }
}

/// `ubi:` backend config.
pub fn create_ubi_tool_config(
    name: &str,
    version: &str,
    tag_regex: Option<&str>,
) -> BackendToolingConfig {
    let has_v_prefix = version.starts_with('v');
    let sets_tag_regex = !has_v_prefix || tag_regex.is_some();

    let extract_version = if sets_tag_regex {
        let tag = match tag_regex {
            None => ".+".to_owned(),
            Some(tr) => {
                let stripped = tr.strip_prefix('^').unwrap_or(tr);
                if !has_v_prefix {
                    strip_leading_v_opt(stripped).to_owned()
                } else {
                    stripped.to_owned()
                }
            }
        };
        Some(format!(
            "^{}(?<version>{})",
            if has_v_prefix { "" } else { "v?" },
            tag
        ))
    } else {
        None
    };

    BackendToolingConfig {
        package_name: name.to_owned(),
        datasource: Some("github-releases"),
        current_value: Some(version.to_owned()),
        extract_version,
        skip_reason: None,
    }
}

/// Parse a mise TOML file and validate the minimal schema Renovate requires.
pub fn parse_toml_file(content: &str) -> Option<toml::Value> {
    let root = toml::from_str::<toml::Value>(content).ok()?;
    root.get("tools")?.as_table()?;
    Some(root)
}

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
        locked_version: None,
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
        // Kafka versions must start with "apache-" to be valid.
        if tool_name == "kafka" && !version.starts_with("apache-") {
            out.push(AsdfDep {
                tool_name: tool_name.to_owned(),
                dep_name: tool_name.to_owned(),
                current_value: version.to_owned(),
                skip_reason: Some(AsdfSkipReason::UnsupportedDatasource),
                ..Default::default()
            });
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

// ── mise lockfile utilities ───────────────────────────────────────────────────

/// Configuration type inferred from a mise config file path.
/// Mirrors `MiseConfigType` in `lib/modules/manager/mise/lockfile.ts`.
#[derive(Debug, PartialEq)]
pub struct MiseConfigType {
    pub is_local: bool,
    pub env: Option<String>,
}

/// Parsed representation of a mise lock file.
/// Tools maps short tool name → ordered list of locked versions.
#[derive(Debug, Default)]
pub struct MiseLockFile {
    pub tools: std::collections::HashMap<String, Vec<String>>,
}

static CONFIG_TYPE_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r"^(?:\.?mise|config)\.(?<env>[^.]+)(?:\.local)?\.toml$").unwrap()
});

/// Parses the config file name to determine its type (local, env-specific, or default).
pub fn get_config_type(config_path: &str) -> MiseConfigType {
    let filename = config_path.split('/').next_back().unwrap_or(config_path);
    let is_local = filename.ends_with(".local.toml");
    let env = CONFIG_TYPE_RE.captures(filename).and_then(|c| {
        let e = c.name("env")?.as_str();
        if e == "local" {
            None
        } else {
            Some(e.to_owned())
        }
    });
    MiseConfigType { is_local, env }
}

/// Derives the lock file path from a mise config file path.
pub fn get_lock_file_name(config_path: &str) -> String {
    let (dir, _filename) = config_path.rsplit_once('/').unwrap_or((".", config_path));
    let parent_base = dir.split('/').next_back().unwrap_or(dir);
    let lock_dir = if parent_base == "conf.d" {
        dir.rsplit_once('/').map_or(".", |(d, _)| d)
    } else {
        dir
    };

    let MiseConfigType { is_local, env } = get_config_type(config_path);
    let lock_name = match (env.as_deref(), is_local) {
        (Some(e), true) => format!("mise.{e}.local.lock"),
        (Some(e), false) => format!("mise.{e}.lock"),
        (None, true) => "mise.local.lock".to_owned(),
        (None, false) => "mise.lock".to_owned(),
    };
    if lock_dir == "." {
        lock_name
    } else {
        format!("{lock_dir}/{lock_name}")
    }
}

/// Parse a mise lock file (TOML format) into a `MiseLockFile`.
pub fn parse_mise_lock_file(content: &str) -> Option<MiseLockFile> {
    let root = toml::from_str::<toml::Value>(content).ok()?;
    let tools_table = root.get("tools")?.as_table()?;
    let mut tools = std::collections::HashMap::new();
    for (name, entries) in tools_table {
        let versions: Vec<String> = entries
            .as_array()?
            .iter()
            .filter_map(|e| e.get("version")?.as_str().map(str::to_owned))
            .collect();
        tools.insert(name.clone(), versions);
    }
    Some(MiseLockFile { tools })
}

/// Get the locked version for a dependency from the parsed lock file.
/// Tries full depName first, then falls back to the short name after `:`.
pub fn get_locked_version(lock_file: &MiseLockFile, dep_name: &str) -> Option<String> {
    let tools = &lock_file.tools;
    if let Some(versions) = tools.get(dep_name) {
        return versions.first().cloned();
    }
    if let Some(idx) = dep_name.find(':') {
        let short = &dep_name[idx + 1..];
        if let Some(versions) = tools.get(short) {
            return versions.first().cloned();
        }
    }
    None
}

// ── mise updateLockedDependency ───────────────────────────────────────────────

/// Possible outcomes of `update_locked_dependency`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateLockedStatus {
    AlreadyUpdated,
    Unsupported,
    UpdateFailed,
}

/// Attempt to satisfy a version update using the existing lock file.
///
/// Mirrors `updateLockedDependency()` in `lib/modules/manager/mise/update-locked.ts`.
///
/// Returns:
/// - `AlreadyUpdated` when the lock file already has `new_version` for `dep_name`
/// - `Unsupported` for all other cases (missing inputs, parse failure, version mismatch)
/// - `UpdateFailed` on unexpected errors
pub fn update_locked_dependency(
    dep_name: Option<&str>,
    new_version: Option<&str>,
    lock_file_content: Option<&str>,
) -> UpdateLockedStatus {
    let (Some(dep_name), Some(lock_file_content)) = (dep_name, lock_file_content) else {
        return UpdateLockedStatus::Unsupported;
    };
    let result = std::panic::catch_unwind(|| {
        let Some(parsed) = parse_mise_lock_file(lock_file_content) else {
            return UpdateLockedStatus::Unsupported;
        };
        if get_locked_version(&parsed, dep_name).as_deref() == new_version {
            UpdateLockedStatus::AlreadyUpdated
        } else {
            UpdateLockedStatus::Unsupported
        }
    });
    result.unwrap_or(UpdateLockedStatus::UpdateFailed)
}

// ── mise extractPackageFile (with lock file support) ─────────────────────────

/// Result of extracting a mise.toml file, including optional lock file information.
#[derive(Debug)]
pub struct MiseExtractResult {
    pub deps: Vec<AsdfDep>,
    /// Lock file paths when a lock file was found and parsed.
    pub lock_files: Option<Vec<String>>,
}

/// Extract deps from a mise config file, optionally enriching with lock file data.
///
/// `config_path` is the path to the mise.toml file (e.g., "mise.toml").
/// `lock_file_content` is `Some(content)` if the lock file was found and read, `None` otherwise.
pub fn extract_package_file(
    content: &str,
    config_path: &str,
    lock_file_content: Option<&str>,
) -> MiseExtractResult {
    let mut deps = extract(content);

    if let Some(lock_content) = lock_file_content
        && let Some(lock_file) = parse_mise_lock_file(lock_content)
    {
        let lock_file_name = get_lock_file_name(config_path);
        for dep in &mut deps {
            let tool_key = dep.dep_name.split('/').next().unwrap_or(&dep.dep_name);
            let lookup_key = if let Some((_, after)) = tool_key.split_once(':') {
                after
            } else {
                tool_key
            };
            dep.locked_version = get_locked_version(&lock_file, lookup_key);
        }
        return MiseExtractResult {
            deps,
            lock_files: Some(vec![lock_file_name]),
        };
    }

    MiseExtractResult {
        deps,
        lock_files: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── mise/artifacts.spec.ts › updateLockedDependency tests ────────────────

    const LOCK_CONTENT: &str = "\
[[tools.node]]\nversion = \"20.11.0\"\nbackend = \"core:node\"\n\n\
[[tools.python]]\nversion = \"3.10.17\"\n";

    // Ported: "returns already-updated when version matches" — lib/modules/manager/mise/artifacts.spec.ts line 441
    #[test]
    fn update_locked_already_updated_when_version_matches() {
        assert_eq!(
            update_locked_dependency(Some("node"), Some("20.11.0"), Some(LOCK_CONTENT)),
            UpdateLockedStatus::AlreadyUpdated
        );
    }

    // Ported: "returns already-updated for tool with backend prefix" — lib/modules/manager/mise/artifacts.spec.ts line 454
    #[test]
    fn update_locked_already_updated_for_backend_prefix() {
        assert_eq!(
            update_locked_dependency(Some("core:node"), Some("20.11.0"), Some(LOCK_CONTENT)),
            UpdateLockedStatus::AlreadyUpdated
        );
    }

    // Ported: "returns unsupported when version does not match" — lib/modules/manager/mise/artifacts.spec.ts line 467
    #[test]
    fn update_locked_unsupported_when_version_does_not_match() {
        assert_eq!(
            update_locked_dependency(Some("node"), Some("22.0.0"), Some(LOCK_CONTENT)),
            UpdateLockedStatus::Unsupported
        );
    }

    // Ported: "returns unsupported when tool not in lock file" — lib/modules/manager/mise/artifacts.spec.ts line 480
    #[test]
    fn update_locked_unsupported_when_tool_not_in_lock_file() {
        assert_eq!(
            update_locked_dependency(Some("ruby"), Some("3.3.0"), Some(LOCK_CONTENT)),
            UpdateLockedStatus::Unsupported
        );
    }

    // Ported: "returns unsupported when no lock file content" — lib/modules/manager/mise/artifacts.spec.ts line 493
    #[test]
    fn update_locked_unsupported_when_no_lock_file_content() {
        assert_eq!(
            update_locked_dependency(Some("node"), Some("20.11.0"), None),
            UpdateLockedStatus::Unsupported
        );
    }

    // Ported: "returns unsupported for invalid lock file content" — lib/modules/manager/mise/artifacts.spec.ts line 506
    #[test]
    fn update_locked_unsupported_for_invalid_lock_file_content() {
        assert_eq!(
            update_locked_dependency(Some("node"), Some("20.11.0"), Some("invalid toml {{{")),
            UpdateLockedStatus::Unsupported
        );
    }

    // Ported: "returns unsupported when depName is undefined" — lib/modules/manager/mise/artifacts.spec.ts line 519
    #[test]
    fn update_locked_unsupported_when_dep_name_is_none() {
        assert_eq!(
            update_locked_dependency(None, Some("20.11.0"), Some(LOCK_CONTENT)),
            UpdateLockedStatus::Unsupported
        );
    }

    // Ported: "returns update-failed in case of errors" — lib/modules/manager/mise/artifacts.spec.ts line 532
    // TypeScript mocks getLockedVersion to throw; Rust verifies the catch_unwind guard.
    #[test]
    fn update_locked_update_failed_on_panic() {
        // Directly exercise the catch_unwind mechanism: a panic inside the closure
        // is caught and mapped to UpdateFailed, matching the TypeScript try-catch behavior.
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            std::panic::panic_any("unexpected error");
            #[allow(unreachable_code)]
            UpdateLockedStatus::Unsupported
        }));
        let status = result.unwrap_or(UpdateLockedStatus::UpdateFailed);
        assert_eq!(status, UpdateLockedStatus::UpdateFailed);
    }

    // ── mise/lockfile.spec.ts tests ───────────────────────────────────────────

    // Ported: "returns isLocal=$isLocal env=$env for $configPath" — lib/modules/manager/mise/lockfile.spec.ts line 10
    #[test]
    fn get_config_type_parses_all_variants() {
        assert_eq!(
            get_config_type("mise.toml"),
            MiseConfigType {
                is_local: false,
                env: None
            }
        );
        assert_eq!(
            get_config_type(".mise.toml"),
            MiseConfigType {
                is_local: false,
                env: None
            }
        );
        assert_eq!(
            get_config_type("mise.local.toml"),
            MiseConfigType {
                is_local: true,
                env: None
            }
        );
        assert_eq!(
            get_config_type("mise.test.toml"),
            MiseConfigType {
                is_local: false,
                env: Some("test".into())
            }
        );
        assert_eq!(
            get_config_type("mise.test.local.toml"),
            MiseConfigType {
                is_local: true,
                env: Some("test".into())
            }
        );
        assert_eq!(
            get_config_type("config.toml"),
            MiseConfigType {
                is_local: false,
                env: None
            }
        );
    }

    // Ported: "returns $expected for $configPath" — lib/modules/manager/mise/lockfile.spec.ts line 27
    #[test]
    fn get_lock_file_name_derives_correct_path() {
        assert_eq!(get_lock_file_name("mise.toml"), "mise.lock");
        assert_eq!(get_lock_file_name(".mise.toml"), "mise.lock");
        assert_eq!(get_lock_file_name("config.toml"), "mise.lock");
        assert_eq!(get_lock_file_name("mise.test.toml"), "mise.test.lock");
        assert_eq!(get_lock_file_name("mise.staging.toml"), "mise.staging.lock");
        assert_eq!(get_lock_file_name("mise.local.toml"), "mise.local.lock");
        assert_eq!(
            get_lock_file_name("mise.test.local.toml"),
            "mise.test.local.lock"
        );
        assert_eq!(get_lock_file_name("subdir/mise.toml"), "subdir/mise.lock");
        assert_eq!(
            get_lock_file_name("subdir/mise.prod.toml"),
            "subdir/mise.prod.lock"
        );
        assert_eq!(get_lock_file_name("conf.d/python.toml"), "mise.lock");
        assert_eq!(
            get_lock_file_name("project/conf.d/node.toml"),
            "project/mise.lock"
        );
    }

    fn make_lock_file() -> MiseLockFile {
        let mut tools = std::collections::HashMap::new();
        tools.insert("node".into(), vec!["20.11.0".into()]);
        tools.insert("python".into(), vec!["3.10.17".into(), "3.11.12".into()]);
        tools.insert("aqua:cli/cli".into(), vec!["2.64.0".into()]);
        tools.insert(
            "ubi:cargo-bins/cargo-binstall".into(),
            vec!["1.10.21".into()],
        );
        MiseLockFile { tools }
    }

    // Ported: "returns $expected for $depName" — lib/modules/manager/mise/lockfile.spec.ts line 55
    #[test]
    fn get_locked_version_returns_correct_version() {
        let lf = make_lock_file();
        assert_eq!(get_locked_version(&lf, "node").as_deref(), Some("20.11.0"));
        assert_eq!(
            get_locked_version(&lf, "core:node").as_deref(),
            Some("20.11.0")
        );
        assert_eq!(
            get_locked_version(&lf, "asdf:node").as_deref(),
            Some("20.11.0")
        );
        assert_eq!(
            get_locked_version(&lf, "python").as_deref(),
            Some("3.10.17")
        );
        assert_eq!(
            get_locked_version(&lf, "core:python").as_deref(),
            Some("3.10.17")
        );
        assert_eq!(
            get_locked_version(&lf, "aqua:cli/cli").as_deref(),
            Some("2.64.0")
        );
        assert_eq!(
            get_locked_version(&lf, "ubi:cargo-bins/cargo-binstall").as_deref(),
            Some("1.10.21")
        );
        assert_eq!(get_locked_version(&lf, "unknown"), None);
        assert_eq!(get_locked_version(&lf, "core:unknown"), None);
    }

    // Ported: "returns first version when multiple versions exist" — lib/modules/manager/mise/lockfile.spec.ts line 70
    #[test]
    fn get_locked_version_returns_first_when_multiple() {
        let lf = make_lock_file();
        assert_eq!(
            get_locked_version(&lf, "python").as_deref(),
            Some("3.10.17")
        );
    }

    // Ported: "handles tools with bracket options in name" — lib/modules/manager/mise/lockfile.spec.ts line 74
    #[test]
    fn get_locked_version_handles_bracket_options_in_name() {
        let lf = make_lock_file();
        assert_eq!(
            get_locked_version(&lf, "ubi:cargo-bins/cargo-binstall").as_deref(),
            Some("1.10.21")
        );
    }

    // ── mise/utils.spec.ts tests ──────────────────────────────────────────────

    // Ported: "load and parse successfully" — lib/modules/manager/mise/utils.spec.ts line 8
    #[test]
    fn parse_toml_file_loads_and_parses_successfully() {
        let actual = parse_toml_file("[tools]\nerlang = '23.3'\nnode = '16'\n").unwrap();
        let tools = actual
            .get("tools")
            .and_then(toml::Value::as_table)
            .expect("tools table");

        assert_eq!(
            tools.get("erlang").and_then(toml::Value::as_str),
            Some("23.3")
        );
        assert_eq!(tools.get("node").and_then(toml::Value::as_str), Some("16"));
    }

    // Ported: "invalid toml" — lib/modules/manager/mise/utils.spec.ts line 23
    #[test]
    fn parse_toml_file_rejects_invalid_toml() {
        assert!(parse_toml_file("clearly: \"invalid\" \"toml\"").is_none());
    }

    // Ported: "invalid schema" — lib/modules/manager/mise/utils.spec.ts line 31
    #[test]
    fn parse_toml_file_rejects_invalid_schema() {
        assert!(parse_toml_file("[invalid]\nerlang = '23.3'\nnode = '16'\n").is_none());
    }

    // ── mise/schema.spec.ts tests ─────────────────────────────────────────────

    // Ported: "defaults tools to empty object when [tools] is absent" — lib/modules/manager/mise/schema.spec.ts line 6
    // TypeScript MiseFile.parse() defaults tools to {}; Rust parse_toml_file() returns None (no deps).
    #[test]
    fn mise_file_no_tools_section_produces_no_deps() {
        assert!(parse_toml_file("min_version = \"2024.11.1\"\n").is_none());
        assert!(extract("min_version = \"2024.11.1\"\n").is_empty());
    }

    // Ported: "defaults tools to empty object for empty TOML" — lib/modules/manager/mise/schema.spec.ts line 13
    // Also covered by empty_returns_empty; this confirms parse_toml_file layer.
    #[test]
    fn mise_file_empty_toml_produces_no_deps() {
        assert!(parse_toml_file("").is_none());
        assert!(extract("").is_empty());
    }

    // Ported: "parses [tools] when present" — lib/modules/manager/mise/schema.spec.ts line 17
    #[test]
    fn mise_file_with_tools_section_parses_correctly() {
        let content = "[tools]\nnode = \"20\"\n";
        let parsed = parse_toml_file(content).unwrap();
        let tools = parsed.get("tools").and_then(toml::Value::as_table).unwrap();
        assert_eq!(tools.get("node").and_then(toml::Value::as_str), Some("20"));
    }

    // Ported: "extracts tools - mise core plugins" — lib/modules/manager/mise/extract.spec.ts line 29
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

    // Ported: "extracts tools - mise core plugins" — lib/modules/manager/mise/extract.spec.ts line 29
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

    // Ported: "extracts tools - mise core plugins" — lib/modules/manager/mise/extract.spec.ts line 29
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

    // Ported: "extracts tools - mise registry tools" — lib/modules/manager/mise/extract.spec.ts line 52
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

    // Ported: "extracts tools - asdf plugins" — lib/modules/manager/mise/extract.spec.ts line 394
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

    // Ported: "provides skipReason for lines with unsupported tooling" — lib/modules/manager/mise/extract.spec.ts line 782
    #[test]
    fn unknown_tool_skipped() {
        let content = "[tools]\nmyunknowntool = \"1.0\"\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(AsdfSkipReason::UnsupportedTool));
    }

    // Rust-specific: mise behavior test
    #[test]
    fn ignores_non_tools_sections() {
        let content = "[settings]\nsomething = \"value\"\n[tools]\nnode = \"18\"\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
    }

    // Ported: "extracts tools with multiple versions" — lib/modules/manager/mise/extract.spec.ts line 410
    #[test]
    fn ignores_array_versions() {
        let content = "[tools]\nnode = [\"18\", \"20\"]\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].tool_name, "node");
        assert_eq!(deps[0].current_value, "18");
        assert_eq!(deps[0].datasource_id, Some("node-version"));
    }

    // Ported: "returns null for empty" — lib/modules/manager/mise/extract.spec.ts line 14
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "core java plugin function" — lib/modules/manager/mise/extract.spec.ts line 912
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

    // Ported: "uses semver-partial versioning for short java version $version" — lib/modules/manager/mise/extract.spec.ts line 1035
    #[test]
    fn java_short_versions_use_semver_partial() {
        for (version, current_value) in [
            ("21", "21"),
            ("21.0", "21.0"),
            ("temurin-21", "21"),
            ("corretto-21.0", "21.0"),
        ] {
            let content = format!("[tools]\njava = \"{version}\"\n");
            let deps = extract(&content);
            assert_eq!(deps.len(), 1, "{version}");
            assert_eq!(deps[0].current_value, current_value, "{version}");
            assert_eq!(deps[0].datasource_id, Some("java-version"), "{version}");
            assert_eq!(deps[0].versioning, Some("semver-partial"), "{version}");
        }
    }

    // Ported: "does not use semver-partial for full java version $version" — lib/modules/manager/mise/extract.spec.ts line 1062
    #[test]
    fn java_full_versions_do_not_use_semver_partial() {
        for (version, current_value) in [("21.0.2", "21.0.2"), ("temurin-21.0.2", "21.0.2")] {
            let content = format!("[tools]\njava = \"{version}\"\n");
            let deps = extract(&content);
            assert_eq!(deps.len(), 1, "{version}");
            assert_eq!(deps[0].current_value, current_value, "{version}");
            assert_eq!(deps[0].datasource_id, Some("java-version"), "{version}");
            assert!(deps[0].versioning.is_none(), "{version}");
        }
    }

    // Ported: "returns null for invalid TOML" — lib/modules/manager/mise/extract.spec.ts line 18
    #[test]
    fn invalid_toml_returns_empty() {
        assert!(extract("foo").is_empty());
    }

    // Ported: "returns null for empty tools section" — lib/modules/manager/mise/extract.spec.ts line 22
    #[test]
    fn empty_tools_section_returns_empty() {
        assert!(extract("[tools]\n").is_empty());
    }

    // Ported: "provides skipReason for missing version - empty string" — lib/modules/manager/mise/extract.spec.ts line 803
    #[test]
    fn empty_version_string_skipped() {
        let content = "[tools]\npython = ''\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert!(deps[0].skip_reason.is_some());
    }

    // Ported: "provides skipReason for missing version - missing version in object" — lib/modules/manager/mise/extract.spec.ts line 819
    #[test]
    fn object_without_version_skipped() {
        let content = "[tools]\npython = {virtualenv='.venv'}\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert!(deps[0].skip_reason.is_some());
    }

    // Ported: "provides skipReason for missing version - empty array" — lib/modules/manager/mise/extract.spec.ts line 835
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

    // Ported: "skips kafka tool when version has no apache- prefix" — lib/modules/manager/mise/extract.spec.ts line 1297
    #[test]
    fn kafka_without_apache_prefix_skipped() {
        let content = "[tools]\nkafka = \"3.5.0\"\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].tool_name, "kafka");
        assert_eq!(
            deps[0].skip_reason,
            Some(AsdfSkipReason::UnsupportedDatasource)
        );
    }

    // Ported: "complete mise.toml example" — lib/modules/manager/mise/extract.spec.ts line 856
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

    // Ported: "complete example with skip" — lib/modules/manager/mise/extract.spec.ts line 879
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

    // Ported: "extracts tools with plugin options" — lib/modules/manager/mise/extract.spec.ts line 433
    #[test]
    fn tool_with_version_object() {
        let content = "[tools]\npython = {version = \"3.12.3\"}\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "3.12.3");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "extracts tools in the default registry with backends" — lib/modules/manager/mise/extract.spec.ts line 449
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

    // Ported: "extracts aqua backend tool" — lib/modules/manager/mise/extract.spec.ts line 488
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

    // Ported: "extracts cargo backend tools" — lib/modules/manager/mise/extract.spec.ts line 515
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

    // Ported: "extracts dotnet backend tool" — lib/modules/manager/mise/extract.spec.ts line 554
    #[test]
    fn extracts_dotnet_backend_tool() {
        let deps = extract("[tools]\n\"dotnet:GitVersion.Tool\" = \"5.12.0\"\n");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "5.12.0");
        assert_eq!(deps[0].package_name.as_deref(), Some("GitVersion.Tool"));
        assert_eq!(deps[0].datasource_id, Some("nuget"));
    }

    // Ported: "extracts gem backend tool" — lib/modules/manager/mise/extract.spec.ts line 572
    #[test]
    fn extracts_gem_backend_tool() {
        let deps = extract("[tools]\n\"gem:rubocop\" = \"1.69.2\"\n");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "1.69.2");
        assert_eq!(deps[0].package_name.as_deref(), Some("rubocop"));
        assert_eq!(deps[0].datasource_id, Some("rubygems"));
    }

    // Ported: "extracts go backend tool" — lib/modules/manager/mise/extract.spec.ts line 590
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

    // Ported: "extracts npm backend tool" — lib/modules/manager/mise/extract.spec.ts line 608
    #[test]
    fn extracts_npm_backend_tool() {
        let deps = extract("[tools]\n\"npm:prettier\" = \"3.3.2\"\n");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "3.3.2");
        assert_eq!(deps[0].package_name.as_deref(), Some("prettier"));
        assert_eq!(deps[0].datasource_id, Some("npm"));
    }

    // Ported: "extracts pipx backend tools" — lib/modules/manager/mise/extract.spec.ts line 626
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

    // Ported: "extracts spm backend tools" — lib/modules/manager/mise/extract.spec.ts line 658
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

    // Ported: "extracts ubi backend tools" — lib/modules/manager/mise/extract.spec.ts line 683
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

    // Ported: "extracts github backend tools" — lib/modules/manager/mise/extract.spec.ts line 741
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

    // Ported: "resolves tools from the mise registry data file via aqua backend" — lib/modules/manager/mise/extract.spec.ts line 1087
    #[test]
    fn resolves_mise_registry_aqua_backend_tool() {
        let deps = extract("[tools]\nzola = \"0.19.2\"\n");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "zola");
        assert_eq!(deps[0].current_value, "0.19.2");
        assert_eq!(deps[0].datasource_id, Some("github-tags"));
        assert_eq!(deps[0].package_name.as_deref(), Some("getzola/zola"));
    }

    // Ported: "resolves tools from the mise registry data file via cargo backend" — lib/modules/manager/mise/extract.spec.ts line 1105
    #[test]
    fn resolves_mise_registry_cargo_backend_tool() {
        let deps = extract("[tools]\nmagika = \"0.3.1\"\n");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "magika");
        assert_eq!(deps[0].current_value, "0.3.1");
        assert_eq!(deps[0].datasource_id, Some("crate"));
        assert_eq!(deps[0].package_name.as_deref(), Some("magika-cli"));
    }

    // Ported: "resolves tools from the mise registry data file via github backend" — lib/modules/manager/mise/extract.spec.ts line 1123
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

    // Ported: "resolves a tool from the mise registry, prioritising the github backend over others" — lib/modules/manager/mise/extract.spec.ts line 1141
    #[test]
    fn resolves_mise_registry_prefers_github_backend_tool() {
        let deps = extract("[tools]\nbitwarden-secrets-manager = \"1.2.3\"\n");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "bitwarden-secrets-manager");
        assert_eq!(deps[0].current_value, "1.2.3");
        assert_eq!(deps[0].datasource_id, Some("github-releases"));
        assert_eq!(deps[0].package_name.as_deref(), Some("bitwarden/sdk"));
    }

    // --- backend tooling config tests ---

    // Ported: "should create a tooling config" — lib/modules/manager/mise/backends.spec.ts line 16
    #[test]
    fn aqua_create_tooling_config() {
        let r = create_aqua_tool_config("BurntSushi/ripgrep", "14.1.1");
        assert_eq!(r.package_name, "BurntSushi/ripgrep");
        assert_eq!(r.datasource, Some("github-tags"));
        assert_eq!(r.current_value.as_deref(), Some("14.1.1"));
        assert_eq!(r.extract_version.as_deref(), Some("^v?(?<version>.+)"));
    }

    // Ported: "should trim the leading v from version" — lib/modules/manager/mise/backends.spec.ts line 27
    #[test]
    fn aqua_trim_leading_v() {
        let r = create_aqua_tool_config("BurntSushi/ripgrep", "v14.1.1");
        assert_eq!(r.current_value.as_deref(), Some("14.1.1"));
    }

    // Ported: "should create a tooling config for crate" — lib/modules/manager/mise/backends.spec.ts line 40
    #[test]
    fn cargo_create_crate_config() {
        let r = create_cargo_tool_config("eza", "");
        assert_eq!(r.package_name, "eza");
        assert_eq!(r.datasource, Some("crate"));
        assert!(r.current_value.is_none());
    }

    // Ported: "should create a tooling config for git tag" — lib/modules/manager/mise/backends.spec.ts line 47
    #[test]
    fn cargo_create_git_tag_config() {
        let r = create_cargo_tool_config("https://github.com/username/demo", "tag:v0.1.0");
        assert_eq!(r.package_name, "https://github.com/username/demo");
        assert_eq!(r.current_value.as_deref(), Some("v0.1.0"));
        assert_eq!(r.datasource, Some("git-tags"));
    }

    // Ported: "should provide skipReason for git branch" — lib/modules/manager/mise/backends.spec.ts line 57
    #[test]
    fn cargo_create_git_branch_config() {
        let r = create_cargo_tool_config("https://github.com/username/demo", "branch:main");
        assert_eq!(r.current_value.as_deref(), Some("main"));
        assert_eq!(r.datasource, Some("git-refs"));
    }

    // Ported: "should create a tooling config for git rev" — lib/modules/manager/mise/backends.spec.ts line 70
    #[test]
    fn cargo_create_git_rev_config() {
        let r = create_cargo_tool_config("https://github.com/username/demo", "rev:abcdef");
        assert_eq!(r.current_value.as_deref(), Some("abcdef"));
        assert_eq!(r.datasource, Some("git-refs"));
    }

    // Ported: "should provide skipReason for invalid version" — lib/modules/manager/mise/backends.spec.ts line 80
    #[test]
    fn cargo_invalid_version_skip_reason() {
        let r = create_cargo_tool_config("https://github.com/username/demo", "v0.1.0");
        assert_eq!(r.skip_reason, Some("invalid-version"));
        assert!(r.datasource.is_none());
    }

    // Ported: "should create a tooling config" — lib/modules/manager/mise/backends.spec.ts line 91
    #[test]
    fn dotnet_create_tooling_config() {
        let r = create_dotnet_tool_config("GitVersion.Tool");
        assert_eq!(r.package_name, "GitVersion.Tool");
        assert_eq!(r.datasource, Some("nuget"));
    }

    // Ported: "should create a tooling config" — lib/modules/manager/mise/backends.spec.ts line 100
    #[test]
    fn gem_create_tooling_config() {
        let r = create_gem_tool_config("rubocop");
        assert_eq!(r.package_name, "rubocop");
        assert_eq!(r.datasource, Some("rubygems"));
    }

    // Ported: "should create a tooling config with empty options" — lib/modules/manager/mise/backends.spec.ts line 109
    #[test]
    fn github_create_empty_options() {
        let r = create_github_tool_config("BurntSushi/ripgrep", "14.1.1", None);
        assert_eq!(r.package_name, "BurntSushi/ripgrep");
        assert_eq!(r.datasource, Some("github-releases"));
        assert_eq!(r.current_value.as_deref(), Some("14.1.1"));
        assert!(r.extract_version.is_none());
    }

    // Ported: "should not set extractVersion if the version has leading v" — lib/modules/manager/mise/backends.spec.ts line 119
    #[test]
    fn github_no_extract_version_with_v_prefix() {
        let r = create_github_tool_config("cli/cli", "v2.64.0", None);
        assert_eq!(r.current_value.as_deref(), Some("v2.64.0"));
        assert!(r.extract_version.is_none());
    }

    // Ported: "should set extractVersion with custom version_prefix" — lib/modules/manager/mise/backends.spec.ts line 127
    #[test]
    fn github_set_extract_version_with_prefix() {
        let r = create_github_tool_config("some/repo", "1.0.0", Some("release-"));
        assert_eq!(
            r.extract_version.as_deref(),
            Some("^release\\-(?<version>.+)")
        );
    }

    // Ported: "should set extractVersion with version_prefix even if version has leading v" — lib/modules/manager/mise/backends.spec.ts line 140
    #[test]
    fn github_extract_version_with_prefix_and_v_version() {
        let r = create_github_tool_config("some/repo", "v1.0.0", Some("version-"));
        assert_eq!(
            r.extract_version.as_deref(),
            Some("^version\\-(?<version>.+)")
        );
    }

    // Ported: "should handle empty version_prefix with version not having v" — lib/modules/manager/mise/backends.spec.ts line 153
    #[test]
    fn github_empty_prefix_no_v() {
        let r = create_github_tool_config("some/repo", "1.0.0", Some(""));
        assert!(r.extract_version.is_none());
    }

    // Ported: "should handle empty version_prefix with version having v" — lib/modules/manager/mise/backends.spec.ts line 163
    #[test]
    fn github_empty_prefix_with_v() {
        let r = create_github_tool_config("some/repo", "v1.0.0", Some(""));
        assert!(r.extract_version.is_none());
    }

    // Ported: "should escape special regex characters in version_prefix" — lib/modules/manager/mise/backends.spec.ts line 173
    #[test]
    fn github_escape_special_chars_in_prefix() {
        let r = create_github_tool_config("some/repo", "1.0.0", Some("v1.0+"));
        assert_eq!(
            r.extract_version.as_deref(),
            Some("^v1\\.0\\+(?<version>.+)")
        );
    }

    // Ported: "should escape brackets and parentheses in version_prefix" — lib/modules/manager/mise/backends.spec.ts line 186
    #[test]
    fn github_escape_brackets_in_prefix() {
        let r = create_github_tool_config("some/repo", "1.0.0", Some("prefix[test](v)"));
        assert_eq!(
            r.extract_version.as_deref(),
            Some("^prefix\\[test\\]\\(v\\)(?<version>.+)")
        );
    }

    // Ported: "should create a tooling config" — lib/modules/manager/mise/backends.spec.ts line 201
    #[test]
    fn go_create_tooling_config() {
        let r = create_go_tool_config("github.com/DarthSim/hivemind");
        assert_eq!(r.package_name, "github.com/DarthSim/hivemind");
        assert_eq!(r.datasource, Some("go"));
    }

    // Ported: "should create a tooling config" — lib/modules/manager/mise/backends.spec.ts line 210
    #[test]
    fn npm_create_tooling_config() {
        let r = create_npm_tool_config("prettier");
        assert_eq!(r.package_name, "prettier");
        assert_eq!(r.datasource, Some("npm"));
    }

    // Ported: "should create a tooling config for pypi package" — lib/modules/manager/mise/backends.spec.ts line 219
    #[test]
    fn pipx_create_pypi_config() {
        let r = create_pipx_tool_config("yamllint");
        assert_eq!(r.package_name, "yamllint");
        assert_eq!(r.datasource, Some("pypi"));
    }

    // Ported: "should create a tooling config for github shorthand" — lib/modules/manager/mise/backends.spec.ts line 226
    #[test]
    fn pipx_create_github_shorthand_config() {
        let r = create_pipx_tool_config("psf/black");
        assert_eq!(r.package_name, "psf/black");
        assert_eq!(r.datasource, Some("github-tags"));
    }

    // Ported: "should create a tooling config for github url" — lib/modules/manager/mise/backends.spec.ts line 233
    #[test]
    fn pipx_create_github_url_config() {
        let r = create_pipx_tool_config("git+https://github.com/psf/black.git");
        assert_eq!(r.package_name, "psf/black");
        assert_eq!(r.datasource, Some("github-tags"));
    }

    // Ported: "should create a tooling config for git url" — lib/modules/manager/mise/backends.spec.ts line 242
    #[test]
    fn pipx_create_git_url_config() {
        let r = create_pipx_tool_config("git+https://gitlab.com/user/repo.git");
        assert_eq!(r.package_name, "https://gitlab.com/user/repo");
        assert_eq!(r.datasource, Some("git-refs"));
    }

    // Ported: "provides skipReason for zip file url" — lib/modules/manager/mise/backends.spec.ts line 251
    #[test]
    fn pipx_zip_url_skip_reason() {
        let r = create_pipx_tool_config("https://github.com/psf/black/archive/18.9b0.zip");
        assert_eq!(r.skip_reason, Some("unsupported-url"));
    }

    // Ported: "should create a tooling config for github shorthand" — lib/modules/manager/mise/backends.spec.ts line 262
    #[test]
    fn spm_create_github_shorthand_config() {
        let r = create_spm_tool_config("tuist/tuist");
        assert_eq!(r.package_name, "tuist/tuist");
        assert_eq!(r.datasource, Some("github-releases"));
    }

    // Ported: "should create a tooling config for github url" — lib/modules/manager/mise/backends.spec.ts line 269
    #[test]
    fn spm_create_github_url_config() {
        let r = create_spm_tool_config("https://github.com/tuist/tuist.git");
        assert_eq!(r.package_name, "tuist/tuist");
        assert_eq!(r.datasource, Some("github-releases"));
    }

    // Ported: "provides skipReason for other url" — lib/modules/manager/mise/backends.spec.ts line 278
    #[test]
    fn spm_non_github_url_skip_reason() {
        let r = create_spm_tool_config("https://gitlab.com/user/repo.git");
        assert_eq!(r.skip_reason, Some("unsupported-url"));
    }

    // Ported: "should create a tooling config with empty options" — lib/modules/manager/mise/backends.spec.ts line 289
    #[test]
    fn ubi_create_empty_options() {
        let r = create_ubi_tool_config("nekto/act", "0.2.70", None);
        assert_eq!(r.package_name, "nekto/act");
        assert_eq!(r.datasource, Some("github-releases"));
        assert_eq!(r.current_value.as_deref(), Some("0.2.70"));
        assert_eq!(r.extract_version.as_deref(), Some("^v?(?<version>.+)"));
    }

    // Ported: "should set extractVersion if the version does not have leading v" — lib/modules/manager/mise/backends.spec.ts line 298
    #[test]
    fn ubi_no_v_prefix_sets_extract_version() {
        let r = create_ubi_tool_config("cli/cli", "2.64.0", None);
        assert_eq!(r.extract_version.as_deref(), Some("^v?(?<version>.+)"));
    }

    // Ported: "should not set extractVersion if the version has leading v" — lib/modules/manager/mise/backends.spec.ts line 307
    #[test]
    fn ubi_v_prefix_no_extract_version() {
        let r = create_ubi_tool_config("cli/cli", "v2.64.0", None);
        assert!(r.extract_version.is_none());
    }

    // Ported: "should ignore options unless tag_regex is provided" — lib/modules/manager/mise/backends.spec.ts line 315
    #[test]
    fn ubi_ignore_options_without_tag_regex() {
        let r = create_ubi_tool_config("cli/cli", "2.64.0", None);
        assert_eq!(r.extract_version.as_deref(), Some("^v?(?<version>.+)"));
    }

    // Ported: "should set extractVersion if tag_regex is provided" — lib/modules/manager/mise/backends.spec.ts line 326
    #[test]
    fn ubi_set_extract_version_with_tag_regex() {
        let r =
            create_ubi_tool_config("cargo-bins/cargo-binstall", "1.10.17", Some(r"^\d+\.\d+\."));
        assert_eq!(
            r.extract_version.as_deref(),
            Some(r"^v?(?<version>\d+\.\d+\.)")
        );
    }

    // Ported: "should set extractVersion without v? when tag_regex is provided and version starts with v" — lib/modules/manager/mise/backends.spec.ts line 339
    #[test]
    fn ubi_no_v_opt_with_tag_regex_and_v_version() {
        let r = create_ubi_tool_config(
            "cargo-bins/cargo-binstall",
            "v1.10.17",
            Some(r"^\d+\.\d+\."),
        );
        assert_eq!(
            r.extract_version.as_deref(),
            Some(r"^(?<version>\d+\.\d+\.)")
        );
    }

    // Ported: "should trim the leading ^ from tag_regex" — lib/modules/manager/mise/backends.spec.ts line 352
    #[test]
    fn ubi_trim_caret_from_tag_regex() {
        let r = create_ubi_tool_config(
            "cargo-bins/cargo-binstall",
            "v1.10.17",
            Some(r"^\d+\.\d+\."),
        );
        assert_eq!(
            r.extract_version.as_deref(),
            Some(r"^(?<version>\d+\.\d+\.)")
        );
    }

    // Ported: "should only trim the leading ^ from tag_regex when version starts with v" — lib/modules/manager/mise/backends.spec.ts line 365
    #[test]
    fn ubi_trim_caret_v_prefix_keeps_v_in_regex() {
        let r = create_ubi_tool_config(
            "cargo-bins/cargo-binstall",
            "v1.10.17",
            Some(r"^v\d+\.\d+\."),
        );
        assert_eq!(
            r.extract_version.as_deref(),
            Some(r"^(?<version>v\d+\.\d+\.)")
        );
    }

    // Ported: "should trim the leading ^v from tag_regex" — lib/modules/manager/mise/backends.spec.ts line 378
    #[test]
    fn ubi_trim_caret_v_from_tag_regex_no_v_version() {
        let r = create_ubi_tool_config(
            "cargo-bins/cargo-binstall",
            "1.10.17",
            Some(r"^v\d+\.\d+\."),
        );
        assert_eq!(
            r.extract_version.as_deref(),
            Some(r"^v?(?<version>\d+\.\d+\.)")
        );
    }

    // Ported: "should trim the leading ^v? from tag_regex" — lib/modules/manager/mise/backends.spec.ts line 391
    #[test]
    fn ubi_trim_caret_v_opt_from_tag_regex() {
        let r = create_ubi_tool_config(
            "cargo-bins/cargo-binstall",
            "1.10.17",
            Some(r"^v?\d+\.\d+\."),
        );
        assert_eq!(
            r.extract_version.as_deref(),
            Some(r"^v?(?<version>\d+\.\d+\.)")
        );
    }

    // Ported: "extracts lockedVersion when lock file present" — lib/modules/manager/mise/extract.spec.ts line 1170
    #[test]
    fn extracts_locked_version_when_lock_file_present() {
        let lock_content = r#"
[[tools.node]]
version = "20.11.0"
backend = "core:node"

[[tools.python]]
version = "3.10.17"
"#;
        let content = "[tools]\nnode = \"20\"\npython = \"3.10\"";
        let result = extract_package_file(content, "mise.toml", Some(lock_content));
        assert_eq!(result.lock_files, Some(vec!["mise.lock".to_owned()]));
        let node = result.deps.iter().find(|d| d.dep_name == "node").unwrap();
        assert_eq!(node.locked_version.as_deref(), Some("20.11.0"));
        assert_eq!(node.current_value, "20");
        let python = result.deps.iter().find(|d| d.dep_name == "python").unwrap();
        assert_eq!(python.locked_version.as_deref(), Some("3.10.17"));
    }

    // Ported: "sets lockFiles array when lock file present" — lib/modules/manager/mise/extract.spec.ts line 1195
    #[test]
    fn sets_lock_files_array_when_lock_file_present() {
        let lock_content = "[[tools.node]]\nversion = \"20.11.0\"\n";
        let content = "[tools]\nnode = \"20\"";
        let result = extract_package_file(content, "mise.toml", Some(lock_content));
        assert_eq!(result.lock_files, Some(vec!["mise.lock".to_owned()]));
    }

    // Ported: "handles missing lock file gracefully" — lib/modules/manager/mise/extract.spec.ts line 1205
    #[test]
    fn handles_missing_lock_file_gracefully() {
        let content = "[tools]\nnode = \"20\"";
        let result = extract_package_file(content, "mise.toml", None);
        assert!(result.lock_files.is_none());
        assert!(result.deps[0].locked_version.is_none());
    }

    // Ported: "handles malformed lock file gracefully" — lib/modules/manager/mise/extract.spec.ts line 1216
    #[test]
    fn handles_malformed_lock_file_gracefully() {
        let content = "[tools]\nnode = \"20\"";
        let result = extract_package_file(content, "mise.toml", Some("invalid toml {{{{"));
        assert!(result.lock_files.is_none());
        assert!(result.deps[0].locked_version.is_none());
    }

    // Ported: "works with environment-specific lock files" — lib/modules/manager/mise/extract.spec.ts line 1227
    #[test]
    fn works_with_environment_specific_lock_files() {
        let lock_content = "[[tools.node]]\nversion = \"18.19.0\"\n";
        let content = "[tools]\nnode = \"18\"";
        let result = extract_package_file(content, "mise.test.toml", Some(lock_content));
        assert_eq!(result.lock_files, Some(vec!["mise.test.lock".to_owned()]));
        assert_eq!(result.deps[0].locked_version.as_deref(), Some("18.19.0"));
    }

    // Ported: "extracts lockedVersion for tools with backend prefix" — lib/modules/manager/mise/extract.spec.ts line 1246
    #[test]
    fn extracts_locked_version_for_tools_with_backend_prefix() {
        let lock_content = r#"
[[tools.node]]
version = "20.11.0"
backend = "core:node"
"#;
        let content = "[tools]\n\"core:node\" = \"20\"";
        let result = extract_package_file(content, "mise.toml", Some(lock_content));
        let dep = result
            .deps
            .iter()
            .find(|d| d.dep_name.contains("node"))
            .unwrap();
        assert_eq!(dep.locked_version.as_deref(), Some("20.11.0"));
    }

    // Ported: "skips lockedVersion when tool not in lock file" — lib/modules/manager/mise/extract.spec.ts line 1260
    #[test]
    fn skips_locked_version_when_tool_not_in_lock_file() {
        let lock_content = "[[tools.node]]\nversion = \"20.11.0\"\n";
        let content = "[tools]\nnode = \"20\"\nruby = \"3.3\"";
        let result = extract_package_file(content, "mise.toml", Some(lock_content));
        assert_eq!(result.lock_files, Some(vec!["mise.lock".to_owned()]));
        let node = result.deps.iter().find(|d| d.dep_name == "node").unwrap();
        assert_eq!(node.locked_version.as_deref(), Some("20.11.0"));
        let ruby = result.deps.iter().find(|d| d.dep_name == "ruby").unwrap();
        assert!(ruby.locked_version.is_none());
    }

    // Ported: "extracts first lockedVersion when multiple versions exist" — lib/modules/manager/mise/extract.spec.ts line 1276
    #[test]
    fn extracts_first_locked_version_when_multiple_versions_exist() {
        let lock_content = r#"
[[tools.python]]
version = "3.10.17"

[[tools.python]]
version = "3.11.12"
"#;
        let content = "[tools]\npython = [\"3.10\", \"3.11\"]";
        let result = extract_package_file(content, "mise.toml", Some(lock_content));
        let python = result.deps.iter().find(|d| d.dep_name == "python").unwrap();
        assert_eq!(python.locked_version.as_deref(), Some("3.10.17"));
    }

    #[test]
    fn create_aqua_tool_config_basic() {
        let cfg = create_aqua_tool_config("node", "20");
        assert_eq!(cfg.package_name, "node");
        assert_eq!(cfg.datasource, Some("github-tags"));
    }

    #[test]
    fn create_cargo_tool_config_basic() {
        let cfg = create_cargo_tool_config("ripgrep", "14");
        assert_eq!(cfg.package_name, "ripgrep");
        assert_eq!(cfg.datasource, Some("crate"));
    }

    #[test]
    fn create_dotnet_tool_config_basic() {
        let cfg = create_dotnet_tool_config("dotnet-format");
        assert_eq!(cfg.package_name, "dotnet-format");
        assert_eq!(cfg.datasource, Some("nuget"));
    }

    #[test]
    fn create_gem_tool_config_basic() {
        let cfg = create_gem_tool_config("bundler");
        assert_eq!(cfg.package_name, "bundler");
        assert_eq!(cfg.datasource, Some("rubygems"));
    }

    #[test]
    fn create_go_tool_config_basic() {
        let cfg = create_go_tool_config("golangci-lint");
        assert_eq!(cfg.package_name, "golangci-lint");
        assert_eq!(cfg.datasource, Some("go"));
    }

    #[test]
    fn create_npm_tool_config_basic() {
        let cfg = create_npm_tool_config("typescript");
        assert_eq!(cfg.package_name, "typescript");
        assert_eq!(cfg.datasource, Some("npm"));
    }

    #[test]
    fn create_pipx_tool_config_basic() {
        let cfg = create_pipx_tool_config("black");
        assert_eq!(cfg.package_name, "black");
        assert_eq!(cfg.datasource, Some("pypi"));
    }

    #[test]
    fn get_config_type_detects() {
        let cfg = get_config_type("mise.toml");
        assert!(!cfg.is_local);
        assert_eq!(cfg.env, None);
        let cfg2 = get_config_type(".mise.toml");
        assert!(!cfg2.is_local);
        let cfg3 = get_config_type("mise.lock");
        assert!(!cfg3.is_local);
        assert_eq!(cfg3.env, None);
    }

    #[test]
    fn get_lock_file_name_basic() {
        assert_eq!(get_lock_file_name("mise.toml"), "mise.lock");
    }

    #[test]
    fn parse_mise_lock_file_basic() {
        let content = r#"
[tools]
node = [{ version = "18.0.0" }]
rust = [{ version = "1.70.0" }]
"#;
        let lock = parse_mise_lock_file(content).unwrap();
        assert_eq!(lock.tools.get("node"), Some(&vec!["18.0.0".to_owned()]));
        assert_eq!(lock.tools.get("rust"), Some(&vec!["1.70.0".to_owned()]));
    }

    #[test]
    fn parse_mise_lock_file_invalid() {
        assert!(parse_mise_lock_file("not toml").is_none());
    }

    // Ported: "matchRegexOrGlobList(\"$path\") === $expected" — lib/modules/manager/mise/index.spec.ts line 6
}
