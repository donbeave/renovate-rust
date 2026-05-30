//! Renovate config `extends` preset version extractor.
//!
//! Reads `renovate.json`, `.renovaterc`, etc. and tracks preset repository
//! versions referenced in the `extends` field, and tool constraints.
//!
//! Renovate reference:
//! - `lib/modules/manager/renovate-config/extract.ts`
//! - `lib/config/presets/parse.ts`
//! - Patterns: standard Renovate config file names
//! - Datasources: GitHub Tags, GitLab Tags, Gitea Tags, GitHub Releases

use std::sync::LazyLock;

use regex::Regex;

// ── Legacy structs (kept for backward compatibility) ─────────────────────────

/// Which platform hosts the preset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PresetSource {
    /// `github>owner/repo` → GitHub Tags.
    GitHub,
    /// `gitlab>owner/repo` → GitLab Tags.
    GitLab,
}

/// Skip reason for a preset dep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PresetSkipReason {
    /// No `#tag` in the preset string.
    UnspecifiedVersion,
    /// Platform not supported (e.g. `gitea`, `npm`, `local`).
    UnsupportedDatasource,
}

/// A single Renovate preset dependency (legacy struct).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresetDep {
    /// The repository path, e.g. `owner/renovate-config`.
    pub repo: String,
    /// The tag, e.g. `v1.0.0`.
    pub current_value: String,
    /// The source platform.
    pub source: PresetSource,
    pub skip_reason: Option<PresetSkipReason>,
}

// ── New RenovateConfigDep struct ──────────────────────────────────────────────

/// A dependency extracted from a Renovate config file (presets + constraints).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RenovateConfigDep {
    /// The dep name (repo path for presets, tool name for constraints).
    pub dep_name: String,
    /// Datasource identifier, e.g. `"github-tags"`, `"github-releases"`.
    pub datasource: Option<&'static str>,
    /// Current version or range.
    pub current_value: Option<String>,
    /// Package name override (used for constraints tool lookup).
    pub package_name: Option<&'static str>,
    /// Versioning scheme.
    pub versioning: Option<&'static str>,
    /// Dependency type, e.g. `"tool-constraint"` or `"constraint"`.
    pub dep_type: Option<&'static str>,
    /// Skip reason when the dep cannot be processed.
    pub skip_reason: Option<&'static str>,
    /// Commit message topic template.
    pub commit_message_topic: Option<&'static str>,
}

// ── Tool config table ─────────────────────────────────────────────────────────

struct ToolConfig {
    datasource: &'static str,
    package_name: &'static str,
    versioning: &'static str,
}

/// Look up tool configuration from the containerbase tool registry.
/// Returns `None` when the name is not a known tool (i.e. `!isToolName`).
fn get_tool_config(name: &str) -> Option<ToolConfig> {
    // Derived from lib/util/exec/containerbase.ts `allToolConfig`.
    match name {
        "bazelisk" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "bazelbuild/bazelisk",
            versioning: "semver",
        }),
        "bun" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "oven-sh/bun",
            versioning: "npm",
        }),
        "bundler" => Some(ToolConfig {
            datasource: "rubygems",
            package_name: "bundler",
            versioning: "ruby",
        }),
        "cocoapods" => Some(ToolConfig {
            datasource: "rubygems",
            package_name: "cocoapods",
            versioning: "ruby",
        }),
        "composer" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "containerbase/composer-prebuild",
            versioning: "composer",
        }),
        "conan" => Some(ToolConfig {
            datasource: "pypi",
            package_name: "conan",
            versioning: "pep440",
        }),
        "copier" => Some(ToolConfig {
            datasource: "pypi",
            package_name: "copier",
            versioning: "pep440",
        }),
        "corepack" => Some(ToolConfig {
            datasource: "npm",
            package_name: "corepack",
            versioning: "npm",
        }),
        "dart" => Some(ToolConfig {
            datasource: "dart-version",
            package_name: "dart",
            versioning: "npm",
        }),
        "deno" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "denoland/deno",
            versioning: "deno",
        }),
        "devbox" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "jetify-com/devbox",
            versioning: "semver",
        }),
        "dotnet" => Some(ToolConfig {
            datasource: "dotnet-version",
            package_name: "dotnet-sdk",
            versioning: "semver",
        }),
        "elixir" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "elixir-lang/elixir",
            versioning: "semver",
        }),
        "erlang" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "containerbase/erlang-prebuild",
            versioning: "semver-coerced",
        }),
        "flutter" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "containerbase/flutter-prebuild",
            versioning: "npm",
        }),
        "flux" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "fluxcd/flux2",
            versioning: "semver",
        }),
        "gleam" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "gleam-lang/gleam",
            versioning: "semver",
        }),
        "golang" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "containerbase/golang-prebuild",
            versioning: "npm",
        }),
        "gradle" => Some(ToolConfig {
            datasource: "gradle-version",
            package_name: "gradle",
            versioning: "gradle",
        }),
        "hashin" => Some(ToolConfig {
            datasource: "pypi",
            package_name: "hashin",
            versioning: "pep440",
        }),
        "helm" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "helm/helm",
            versioning: "semver",
        }),
        "helmfile" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "helmfile/helmfile",
            versioning: "semver",
        }),
        "java" => Some(ToolConfig {
            datasource: "java-version",
            package_name: "java?system=true",
            versioning: "npm",
        }),
        "java-maven" => Some(ToolConfig {
            datasource: "java-version",
            package_name: "java?system=true",
            versioning: "maven",
        }),
        "jb" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "jsonnet-bundler/jsonnet-bundler",
            versioning: "semver",
        }),
        "kustomize" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "kubernetes-sigs/kustomize",
            versioning: "semver",
        }),
        "maven" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "containerbase/maven-prebuild",
            versioning: "maven",
        }),
        "nix" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "containerbase/nix-prebuild",
            versioning: "semver",
        }),
        "node" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "containerbase/node-prebuild",
            versioning: "node",
        }),
        "npm" => Some(ToolConfig {
            datasource: "npm",
            package_name: "npm",
            versioning: "npm",
        }),
        "pdm" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "pdm-project/pdm",
            versioning: "semver",
        }),
        "php" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "containerbase/php-prebuild",
            versioning: "composer",
        }),
        "pip-tools" => Some(ToolConfig {
            datasource: "pypi",
            package_name: "pip-tools",
            versioning: "pep440",
        }),
        "pixi" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "prefix-dev/pixi",
            versioning: "conda",
        }),
        "pipenv" => Some(ToolConfig {
            datasource: "pypi",
            package_name: "pipenv",
            versioning: "pep440",
        }),
        "pnpm" => Some(ToolConfig {
            datasource: "npm",
            package_name: "pnpm",
            versioning: "npm",
        }),
        "poetry" => Some(ToolConfig {
            datasource: "pypi",
            package_name: "poetry",
            versioning: "pep440",
        }),
        "python" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "containerbase/python-prebuild",
            versioning: "python",
        }),
        "ruby" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "containerbase/ruby-prebuild",
            versioning: "ruby",
        }),
        "rust" => Some(ToolConfig {
            datasource: "docker",
            package_name: "rust",
            versioning: "semver",
        }),
        "uv" => Some(ToolConfig {
            datasource: "pypi",
            package_name: "uv",
            versioning: "pep440",
        }),
        "vendir" => Some(ToolConfig {
            datasource: "github-releases",
            package_name: "carvel-dev/vendir",
            versioning: "semver",
        }),
        "yarn" => Some(ToolConfig {
            datasource: "npm",
            package_name: "yarn",
            versioning: "npm",
        }),
        "yarn-slim" => Some(ToolConfig {
            datasource: "npm",
            package_name: "yarn",
            versioning: "npm",
        }),
        _ => None,
    }
}

// ── Internal preset package names ─────────────────────────────────────────────

const INTERNAL_PRESET_PACKAGES: &[&str] = &[
    "abandonments",
    "compatibility",
    "config",
    "customManagers",
    "default",
    "docker",
    "global",
    "group",
    "helpers",
    "mergeConfidence",
    "monorepo",
    "npm",
    "packages",
    "preview",
    "replacements",
    "schedule",
    "security",
    "workarounds",
];

// ── Preset parsing ────────────────────────────────────────────────────────────

/// Result of parsing a preset string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedPreset {
    pub preset_source: String,
    pub repo: String,
    /// The preset name inside the repo (e.g. `"default"`, `"base"`, `"webapp"`).
    /// Empty string for http sources (matching TypeScript behaviour).
    pub preset_name: String,
    /// Subdirectory path used with the `//` syntax, e.g. `"somepath/somesubpath"`.
    pub preset_path: Option<String>,
    pub tag: Option<String>,
    /// Positional parameters extracted from `(...)` in the input.
    pub params: Option<Vec<String>>,
    /// The raw string inside `(...)`, before splitting on commas.
    pub raw_params: Option<String>,
}

/// Regex for non-scoped preset with a subdirectory (`//`).
/// Matches: `repo//[presetPath/]presetName[#tag]`
static NON_SCOPED_WITH_SUBDIR_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?P<repo>[~\w\-. /%]+?)//(?:(?P<preset_path>[\w\-./]+)/)?(?P<preset_name>[\w\-.]+)(?:#(?P<tag>[\w\-./ ]+?))?$",
    )
    .unwrap()
});

/// Regex for git preset (standard form): `repo[:presetName][#tag]`
static GIT_PRESET_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?P<repo>[~\w\-. /%]+?)(?::(?P<preset_name>[\w\-.+/]+))?(?:#(?P<tag>[\w\-./ ]+?))?$",
    )
    .unwrap()
});

/// Parse a preset string into its components.
/// Ported from `lib/config/presets/parse.ts`.
pub fn parse_preset(input: &str) -> ParsedPreset {
    let mut s = input.to_owned();
    let mut preset_source: Option<String> = None;

    if s.starts_with("github>") {
        preset_source = Some("github".into());
        s = s["github>".len()..].to_owned();
    } else if s.starts_with("gitlab>") {
        preset_source = Some("gitlab".into());
        s = s["gitlab>".len()..].to_owned();
    } else if s.starts_with("gitea>") {
        preset_source = Some("gitea".into());
        s = s["gitea>".len()..].to_owned();
    } else if s.starts_with("forgejo>") {
        preset_source = Some("forgejo".into());
        s = s["forgejo>".len()..].to_owned();
    } else if s.starts_with("local>") {
        preset_source = Some("local".into());
        s = s["local>".len()..].to_owned();
    } else if s.starts_with("http://") || s.starts_with("https://") {
        preset_source = Some("http".into());
    } else if !s.starts_with('@') && !s.starts_with(':') && s.contains('/') {
        preset_source = Some("local".into());
    }

    // strip npm> prefix if present
    if s.starts_with("npm>") {
        s = s["npm>".len()..].to_owned();
    }

    let preset_source = preset_source.unwrap_or_else(|| "npm".into());

    // extract params `(...)` from s
    let (raw_params, params) = if let Some(paren_pos) = s.find('(') {
        // rawParams is the content between '(' and the last char (closing ')')
        let raw = s[paren_pos + 1..s.len() - 1].to_owned();
        let parsed: Vec<String> = raw.split(',').map(|e| e.trim().to_owned()).collect();
        s = s[..paren_pos].to_owned();
        (Some(raw), Some(parsed))
    } else {
        (None, None)
    };

    // http source: return early
    if preset_source == "http" {
        return ParsedPreset {
            preset_source,
            repo: s,
            preset_name: String::new(),
            preset_path: None,
            tag: None,
            params,
            raw_params,
        };
    }

    // internal: starts with `packageName:` or `:`
    for pkg in INTERNAL_PRESET_PACKAGES {
        if s.starts_with(&format!("{pkg}:")) {
            let preset_name = s[pkg.len() + 1..].to_owned();
            return ParsedPreset {
                preset_source: "internal".into(),
                repo: (*pkg).to_owned(),
                preset_name,
                preset_path: None,
                tag: None,
                params,
                raw_params,
            };
        }
    }
    if let Some(stripped) = s.strip_prefix(':') {
        let preset_name = stripped.to_owned();
        return ParsedPreset {
            preset_source: "internal".into(),
            repo: "default".into(),
            preset_name,
            preset_path: None,
            tag: None,
            params,
            raw_params,
        };
    }

    // scoped npm `@scope/...`
    if s.starts_with('@') {
        // repo = @scope or @scope/name up to first `:` or end
        let at_re = Regex::new(r"(@.*?)(:|$)").unwrap();
        let mut repo = if let Some(caps) = at_re.captures(&s) {
            caps.get(1)
                .map(|m| m.as_str().to_owned())
                .unwrap_or_default()
        } else {
            s.clone()
        };
        let rest = s[repo.len()..].to_owned();
        if !repo.contains('/') {
            repo.push_str("/renovate-config");
        }
        let preset_name = if rest.is_empty() {
            "default".to_owned()
        } else {
            // rest starts with ':', skip it
            rest[1..].to_owned()
        };
        return ParsedPreset {
            preset_source: "npm".into(),
            repo,
            preset_name,
            preset_path: None,
            tag: None,
            params,
            raw_params,
        };
    }

    // non-scoped with subdirectory `//`
    if s.contains("//")
        && let Some(caps) = NON_SCOPED_WITH_SUBDIR_RE.captures(&s)
    {
        let repo = caps
            .name("repo")
            .map(|m| m.as_str().to_owned())
            .unwrap_or_default();
        let preset_path = caps.name("preset_path").map(|m| m.as_str().to_owned());
        let preset_name = caps
            .name("preset_name")
            .map(|m| m.as_str().to_owned())
            .unwrap_or_else(|| "default".to_owned());
        let tag = caps.name("tag").map(|m| m.as_str().to_owned());
        return ParsedPreset {
            preset_source,
            repo,
            preset_name,
            preset_path,
            tag,
            params,
            raw_params,
        };
    }

    // standard git preset form
    if let Some(caps) = GIT_PRESET_RE.captures(&s) {
        let mut repo = caps
            .name("repo")
            .map(|m| m.as_str().to_owned())
            .unwrap_or_default();
        let preset_name = caps
            .name("preset_name")
            .map(|m| m.as_str().to_owned())
            .filter(|n| !n.is_empty())
            .unwrap_or_else(|| "default".to_owned());
        let tag = caps.name("tag").map(|m| m.as_str().to_owned());

        if preset_source == "npm" && !repo.starts_with("renovate-config-") {
            repo = format!("renovate-config-{repo}");
        }

        return ParsedPreset {
            preset_source,
            repo,
            preset_name,
            preset_path: None,
            tag,
            params,
            raw_params,
        };
    }

    // fallback
    ParsedPreset {
        preset_source,
        repo: s,
        preset_name: "default".to_owned(),
        preset_path: None,
        tag: None,
        params,
        raw_params,
    }
}

// ── extract_package_file ──────────────────────────────────────────────────────

/// Parse a Renovate JSON/JSON5 config and extract preset + constraint deps.
///
/// Returns `None` when the file is empty, unparseable, or has no actionable
/// deps (matching TypeScript `extractPackageFile` returning `null`).
pub fn extract_package_file(content: &str) -> Option<Vec<RenovateConfigDep>> {
    if content.trim().is_empty() {
        return None;
    }

    let value: serde_json::Value = json5::from_str(content).ok()?;
    let obj = value.as_object()?;

    let mut deps: Vec<RenovateConfigDep> = Vec::new();

    // ── extends / presets ─────────────────────────────────────────────────────
    if let Some(arr) = obj.get("extends").and_then(|v| v.as_array()) {
        for item in arr {
            if let Some(preset_str) = item.as_str() {
                let parsed = parse_preset(preset_str);

                // Supported sources map to a datasource
                let datasource: Option<&'static str> = match parsed.preset_source.as_str() {
                    "github" => Some("github-tags"),
                    "gitlab" => Some("gitlab-tags"),
                    "gitea" => Some("gitea-tags"),
                    _ => None,
                };

                if let Some(ds) = datasource {
                    // Supported source
                    if let Some(tag) = parsed.tag {
                        deps.push(RenovateConfigDep {
                            dep_name: parsed.repo,
                            datasource: Some(ds),
                            current_value: Some(tag),
                            ..Default::default()
                        });
                    } else {
                        deps.push(RenovateConfigDep {
                            dep_name: parsed.repo,
                            skip_reason: Some("unspecified-version"),
                            ..Default::default()
                        });
                    }
                } else if parsed.preset_source != "internal" {
                    // Unsupported non-internal source
                    deps.push(RenovateConfigDep {
                        dep_name: parsed.repo,
                        skip_reason: Some("unsupported-datasource"),
                        ..Default::default()
                    });
                }
            }
        }
    }

    // ── top-level constraints ─────────────────────────────────────────────────
    if let Some(constraints_obj) = obj.get("constraints").and_then(|v| v.as_object()) {
        for (tool_name, version_val) in constraints_obj {
            if let Some(version_str) = version_val.as_str() {
                push_constraint_dep(&mut deps, tool_name, version_str);
            }
        }
    }

    // ── packageRules[*].constraints ───────────────────────────────────────────
    if let Some(rules_arr) = obj.get("packageRules").and_then(|v| v.as_array()) {
        for rule in rules_arr {
            if let Some(rule_obj) = rule.as_object()
                && let Some(constraints_obj) =
                    rule_obj.get("constraints").and_then(|v| v.as_object())
            {
                for (tool_name, version_val) in constraints_obj {
                    if let Some(version_str) = version_val.as_str() {
                        push_constraint_dep(&mut deps, tool_name, version_str);
                    }
                }
            }
        }
    }

    if deps.is_empty() { None } else { Some(deps) }
}

/// Push a constraint dep (tool or unknown) into the deps vec.
fn push_constraint_dep(deps: &mut Vec<RenovateConfigDep>, tool_name: &str, version: &str) {
    if let Some(tc) = get_tool_config(tool_name) {
        deps.push(RenovateConfigDep {
            dep_name: tool_name.to_owned(),
            datasource: Some(tc.datasource),
            current_value: Some(version.to_owned()),
            package_name: Some(tc.package_name),
            versioning: Some(tc.versioning),
            dep_type: Some("tool-constraint"),
            skip_reason: None,
            commit_message_topic: Some("{{{depName}}} tool constraint"),
        });
    } else {
        deps.push(RenovateConfigDep {
            dep_name: tool_name.to_owned(),
            current_value: Some(version.to_owned()),
            skip_reason: Some("unsupported"),
            dep_type: Some("constraint"),
            commit_message_topic: Some("{{{depName}}} constraint"),
            ..Default::default()
        });
    }
}

// ── Legacy regex-based extractor (kept for backward compat) ───────────────────

/// Matches `"github>owner/repo#tag"` or `"gitlab>owner/repo#tag"` in a JSON
/// file. Also handles entries without a `#tag`.
static PRESET_STR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##""(github|gitlab)>([^"#]+)(?:#([^"]+))?""##).unwrap());

/// Extract Renovate preset deps from a `renovate.json` / `.renovaterc` file.
/// Legacy function kept for backward compatibility.
pub fn extract(content: &str) -> Vec<PresetDep> {
    let mut deps = Vec::new();

    // Quick guard: must have "extends" somewhere and a platform prefix.
    if !content.contains("\"extends\"") || !content.contains('>') {
        return deps;
    }

    for cap in PRESET_STR_RE.captures_iter(content) {
        let platform = &cap[1];
        let repo = cap[2].trim().to_owned();
        let tag = cap.get(3).map(|m| m.as_str().to_owned());

        let source = match platform {
            "github" => PresetSource::GitHub,
            "gitlab" => PresetSource::GitLab,
            _ => {
                deps.push(PresetDep {
                    repo,
                    current_value: String::new(),
                    source: PresetSource::GitHub, // placeholder
                    skip_reason: Some(PresetSkipReason::UnsupportedDatasource),
                });
                continue;
            }
        };

        match tag {
            None => deps.push(PresetDep {
                repo,
                current_value: String::new(),
                source,
                skip_reason: Some(PresetSkipReason::UnspecifiedVersion),
            }),
            Some(t) => deps.push(PresetDep {
                repo,
                current_value: t,
                source,
                skip_reason: None,
            }),
        }
    }

    deps
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Legacy extract() tests ─────────────────────────────────────────────────

    // Rust-specific: renovate_config_presets behavior test
    #[test]
    fn extracts_github_preset_with_tag() {
        let content = r#"{"extends": ["github>owner/renovate-config#v1.2.3", "config:base"]}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let d = &deps[0];
        assert_eq!(d.repo, "owner/renovate-config");
        assert_eq!(d.current_value, "v1.2.3");
        assert_eq!(d.source, PresetSource::GitHub);
        assert!(d.skip_reason.is_none());
    }

    // Rust-specific: renovate_config_presets behavior test
    #[test]
    fn skips_preset_without_tag() {
        let content = r#"{"extends": ["github>owner/renovate-config"]}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(PresetSkipReason::UnspecifiedVersion)
        );
    }

    // Rust-specific: renovate_config_presets behavior test
    #[test]
    fn extracts_gitlab_preset() {
        let content = r#"{"extends": ["gitlab>company/configs#2.0"]}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].source, PresetSource::GitLab);
        assert_eq!(deps[0].current_value, "2.0");
    }

    // Rust-specific: renovate_config_presets behavior test
    #[test]
    fn ignores_internal_presets() {
        let content = r#"{"extends": ["config:base", ":automergeMinor"]}"#;
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    // Ported: "returns null for empty" — lib/modules/manager/terraform/extract.spec.ts line 39
    #[test]
    fn empty_file_returns_empty() {
        assert!(extract("{}").is_empty());
        assert!(extract("").is_empty());
    }

    // ── New extract_package_file() tests ───────────────────────────────────────

    // Ported: "returns null for empty file" — renovate-config/extract.spec.ts line 7
    #[test]
    fn epf_returns_null_for_empty_file() {
        assert_eq!(extract_package_file(""), None);
    }

    // Ported: "returns null for invalid file" — renovate-config/extract.spec.ts line 11
    #[test]
    fn epf_returns_null_for_invalid_file() {
        assert_eq!(extract_package_file("this-is-not-json-object"), None);
    }

    // Ported: "returns null for a config file without presets" — renovate-config/extract.spec.ts line 18
    #[test]
    fn epf_returns_null_without_presets() {
        assert_eq!(extract_package_file(r#"{ "draftPR": true }"#), None);
    }

    // Ported: "returns null for a config file only contains built-in presets" — renovate-config/extract.spec.ts line 34
    #[test]
    fn epf_returns_null_for_only_builtin_presets() {
        let content = r#"{ "extends": ["config:recommended", ":label(test)", "helpers:pinGitHubActionDigests"] }"#;
        assert_eq!(extract_package_file(content), None);
    }

    // Ported: "provides skipReason for unsupported preset sources" — renovate-config/extract.spec.ts line 50
    #[test]
    fn epf_skip_reason_for_unsupported_preset_sources() {
        let content = r#"{
            "extends": [
                "fastcore",
                "http://my.server/users/me/repos/renovate-presets/raw/default.json",
                "local>renovate/presets",
                "local>renovate/presets2#1.2.3"
            ]
        }"#;
        let deps = extract_package_file(content).expect("should return deps");
        assert_eq!(deps.len(), 4);
        assert_eq!(deps[0].dep_name, "renovate-config-fastcore");
        assert_eq!(deps[0].skip_reason, Some("unsupported-datasource"));
        assert_eq!(
            deps[1].dep_name,
            "http://my.server/users/me/repos/renovate-presets/raw/default.json"
        );
        assert_eq!(deps[1].skip_reason, Some("unsupported-datasource"));
        assert_eq!(deps[2].dep_name, "renovate/presets");
        assert_eq!(deps[2].skip_reason, Some("unsupported-datasource"));
        assert_eq!(deps[3].dep_name, "renovate/presets2");
        assert_eq!(deps[3].skip_reason, Some("unsupported-datasource"));
    }

    // Ported: "provides skipReason for presets without versions" — renovate-config/extract.spec.ts line 88
    #[test]
    fn epf_skip_reason_for_presets_without_versions() {
        let content = r#"{
            "extends": [
                "github>abc/foo",
                "gitlab>abc/bar:xyz",
                "gitea>cde/foo//path/xyz"
            ]
        }"#;
        let deps = extract_package_file(content).expect("should return deps");
        assert_eq!(deps.len(), 3);
        assert_eq!(deps[0].dep_name, "abc/foo");
        assert_eq!(deps[0].skip_reason, Some("unspecified-version"));
        assert_eq!(deps[1].dep_name, "abc/bar");
        assert_eq!(deps[1].skip_reason, Some("unspecified-version"));
        assert_eq!(deps[2].dep_name, "cde/foo");
        assert_eq!(deps[2].skip_reason, Some("unspecified-version"));
    }

    // Ported: "extracts from a config file with GitHub hosted presets" — renovate-config/extract.spec.ts line 120
    #[test]
    fn epf_extracts_github_presets() {
        let content = r#"{
            "extends": [
                "github>abc/foo#1.2.3",
                "github>abc/bar:xyz#1.2.3",
                "github>cde/foo//path/xyz#1.2.3",
                "github>cde/bar:xyz/sub#1.2.3"
            ]
        }"#;
        let deps = extract_package_file(content).expect("should return deps");
        assert_eq!(deps.len(), 4);
        for dep in &deps {
            assert_eq!(dep.datasource, Some("github-tags"));
            assert_eq!(dep.current_value.as_deref(), Some("1.2.3"));
            assert!(dep.skip_reason.is_none());
        }
        assert_eq!(deps[0].dep_name, "abc/foo");
        assert_eq!(deps[1].dep_name, "abc/bar");
        assert_eq!(deps[2].dep_name, "cde/foo");
        assert_eq!(deps[3].dep_name, "cde/bar");
    }

    // Ported: "extracts from a config file with GitLab hosted presets" — renovate-config/extract.spec.ts line 161
    #[test]
    fn epf_extracts_gitlab_presets() {
        let content = r#"{
            "extends": [
                "gitlab>abc/foo#1.2.3",
                "gitlab>abc/bar:xyz#1.2.3",
                "gitlab>cde/foo//path/xyz#1.2.3",
                "gitlab>cde/bar:xyz/sub#1.2.3"
            ]
        }"#;
        let deps = extract_package_file(content).expect("should return deps");
        assert_eq!(deps.len(), 4);
        for dep in &deps {
            assert_eq!(dep.datasource, Some("gitlab-tags"));
            assert_eq!(dep.current_value.as_deref(), Some("1.2.3"));
        }
        assert_eq!(deps[0].dep_name, "abc/foo");
        assert_eq!(deps[1].dep_name, "abc/bar");
        assert_eq!(deps[2].dep_name, "cde/foo");
        assert_eq!(deps[3].dep_name, "cde/bar");
    }

    // Ported: "extracts from a config file with Gitea hosted presets" — renovate-config/extract.spec.ts line 202
    #[test]
    fn epf_extracts_gitea_presets() {
        let content = r#"{
            "extends": [
                "gitea>abc/foo#1.2.3",
                "gitea>abc/bar:xyz#1.2.3",
                "gitea>cde/foo//path/xyz#1.2.3",
                "gitea>cde/bar:xyz/sub#1.2.3"
            ]
        }"#;
        let deps = extract_package_file(content).expect("should return deps");
        assert_eq!(deps.len(), 4);
        for dep in &deps {
            assert_eq!(dep.datasource, Some("gitea-tags"));
            assert_eq!(dep.current_value.as_deref(), Some("1.2.3"));
        }
        assert_eq!(deps[0].dep_name, "abc/foo");
        assert_eq!(deps[1].dep_name, "abc/bar");
        assert_eq!(deps[2].dep_name, "cde/foo");
        assert_eq!(deps[3].dep_name, "cde/bar");
    }

    // Ported: "supports JSON5" (presets) — renovate-config/extract.spec.ts line 243
    #[test]
    fn epf_supports_json5_presets() {
        let content = r#"{
            // comments are permitted
            "extends": [
                "github>abc/foo#1.2.3",
            ],
        }"#;
        let deps = extract_package_file(content).expect("should return deps");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "abc/foo");
        assert_eq!(deps[0].datasource, Some("github-tags"));
        assert_eq!(deps[0].current_value.as_deref(), Some("1.2.3"));
    }

    // Ported: "returns null for a config file without constraints" — renovate-config/extract.spec.ts line 269
    #[test]
    fn epf_returns_null_without_constraints() {
        assert_eq!(extract_package_file(r#"{ "draftPR": true }"#), None);
    }

    // Ported: "returns null for a config file has an empty constraints" — renovate-config/extract.spec.ts line 282
    #[test]
    fn epf_returns_null_for_empty_constraints() {
        assert_eq!(extract_package_file(r#"{ "constraints": {} }"#), None);
    }

    // Ported: "extracts known `ToolName`s with explicit versions" — renovate-config/extract.spec.ts line 295
    #[test]
    fn epf_extracts_known_toolnames_explicit_versions() {
        let content = r#"{ "constraints": { "bazelisk": "1.2.3", "maven": "4.0.0" } }"#;
        let deps = extract_package_file(content).expect("should return deps");
        assert_eq!(deps.len(), 2);

        assert_eq!(deps[0].dep_name, "bazelisk");
        assert_eq!(deps[0].datasource, Some("github-releases"));
        assert_eq!(deps[0].package_name, Some("bazelbuild/bazelisk"));
        assert_eq!(deps[0].versioning, Some("semver"));
        assert_eq!(deps[0].current_value.as_deref(), Some("1.2.3"));
        assert_eq!(deps[0].dep_type, Some("tool-constraint"));
        assert_eq!(
            deps[0].commit_message_topic,
            Some("{{{depName}}} tool constraint")
        );

        assert_eq!(deps[1].dep_name, "maven");
        assert_eq!(deps[1].datasource, Some("github-releases"));
        assert_eq!(deps[1].package_name, Some("containerbase/maven-prebuild"));
        assert_eq!(deps[1].versioning, Some("maven"));
        assert_eq!(deps[1].current_value.as_deref(), Some("4.0.0"));
        assert_eq!(deps[1].dep_type, Some("tool-constraint"));
    }

    // Ported: "extracts known `ToolName`s with ranges versions" — renovate-config/extract.spec.ts line 332
    #[test]
    fn epf_extracts_known_toolnames_range_versions() {
        let content = r#"{ "constraints": { "bazelisk": ">= 1.2.3", "maven": "< 4.0.0" } }"#;
        let deps = extract_package_file(content).expect("should return deps");
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].current_value.as_deref(), Some(">= 1.2.3"));
        assert_eq!(deps[1].current_value.as_deref(), Some("< 4.0.0"));
    }

    // Ported: "extracts `ToolName`s from packageRules" — renovate-config/extract.spec.ts line 369
    #[test]
    fn epf_extracts_toolnames_from_package_rules() {
        let content = r#"{
            "constraints": { "golang": "1.20.5" },
            "packageRules": [
                {
                    "matchFileNames": ["go.mod"],
                    "constraints": {
                        "golang": "1.26.0",
                        "gomodMod": "1.2.0"
                    }
                }
            ]
        }"#;
        let deps = extract_package_file(content).expect("should return deps");
        assert_eq!(deps.len(), 3);

        // top-level golang
        assert_eq!(deps[0].dep_name, "golang");
        assert_eq!(deps[0].datasource, Some("github-releases"));
        assert_eq!(deps[0].package_name, Some("containerbase/golang-prebuild"));
        assert_eq!(deps[0].versioning, Some("npm"));
        assert_eq!(deps[0].current_value.as_deref(), Some("1.20.5"));
        assert_eq!(deps[0].dep_type, Some("tool-constraint"));

        // packageRules golang
        assert_eq!(deps[1].dep_name, "golang");
        assert_eq!(deps[1].current_value.as_deref(), Some("1.26.0"));
        assert_eq!(deps[1].dep_type, Some("tool-constraint"));

        // packageRules gomodMod - not a tool
        assert_eq!(deps[2].dep_name, "gomodMod");
        assert_eq!(deps[2].skip_reason, Some("unsupported"));
        assert_eq!(deps[2].current_value.as_deref(), Some("1.2.0"));
        assert_eq!(deps[2].dep_type, Some("constraint"));
        assert_eq!(
            deps[2].commit_message_topic,
            Some("{{{depName}}} constraint")
        );
    }

    // Ported: "handles no `constraints` in packageRules" — renovate-config/extract.spec.ts line 421
    #[test]
    fn epf_handles_no_constraints_in_package_rules() {
        let content = r#"{
            "constraints": { "golang": "1.20.5" },
            "packageRules": [ {} ]
        }"#;
        let deps = extract_package_file(content).expect("should return deps");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "golang");
    }

    // Ported: "sets skipReason=unsupported for a constraint that is not a tool" — renovate-config/extract.spec.ts line 451
    #[test]
    fn epf_skip_reason_unsupported_for_unknown_constraint() {
        let content = r#"{ "constraints": { "gomodMod": "1.2.0" } }"#;
        let deps = extract_package_file(content).expect("should return deps");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "gomodMod");
        assert_eq!(deps[0].skip_reason, Some("unsupported"));
        assert_eq!(deps[0].current_value.as_deref(), Some("1.2.0"));
        assert_eq!(deps[0].dep_type, Some("constraint"));
        assert_eq!(
            deps[0].commit_message_topic,
            Some("{{{depName}}} constraint")
        );
    }

    // Ported: "extracts known `ToolName`s with ranges versions" (second) — renovate-config/extract.spec.ts line 476
    #[test]
    fn epf_extracts_toolnames_range_versions_476() {
        let content = r#"{ "constraints": { "bazelisk": ">= 1.2.3", "maven": "< 4.0.0" } }"#;
        let deps = extract_package_file(content).expect("should return deps");
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_name, "bazelisk");
        assert_eq!(deps[0].current_value.as_deref(), Some(">= 1.2.3"));
        assert_eq!(deps[1].dep_name, "maven");
        assert_eq!(deps[1].current_value.as_deref(), Some("< 4.0.0"));
    }

    // Ported: "supports JSON5" (constraints) — renovate-config/extract.spec.ts line 513
    #[test]
    fn epf_supports_json5_constraints() {
        let content = r#"{
            // comments are permitted
            "constraints": {
                // and no quotes around keys
                gleam: "3.4.5", // and trailing comma
            }
        }"#;
        let deps = extract_package_file(content).expect("should return deps");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "gleam");
        assert_eq!(deps[0].datasource, Some("github-releases"));
        assert_eq!(deps[0].package_name, Some("gleam-lang/gleam"));
        assert_eq!(deps[0].versioning, Some("semver"));
        assert_eq!(deps[0].current_value.as_deref(), Some("3.4.5"));
        assert_eq!(deps[0].dep_type, Some("tool-constraint"));
    }

    // Ported: "extracts all types of configuration" — renovate-config/extract.spec.ts line 543
    #[test]
    fn epf_extracts_all_types_of_configuration() {
        let content = r#"{
            "extends": [
                "github>abc/foo#1.2.3",
                "github>abc/bar:xyz#1.2.3",
                "github>cde/foo//path/xyz#1.2.3",
                "github>cde/bar:xyz/sub#1.2.3"
            ],
            "constraints": {
                "bazelisk": ">= 1.2.3",
                "maven": "< 4.0.0"
            }
        }"#;
        let deps = extract_package_file(content).expect("should return deps");
        assert_eq!(deps.len(), 6);

        // presets
        assert_eq!(deps[0].dep_name, "abc/foo");
        assert_eq!(deps[0].datasource, Some("github-tags"));
        assert_eq!(deps[1].dep_name, "abc/bar");
        assert_eq!(deps[2].dep_name, "cde/foo");
        assert_eq!(deps[3].dep_name, "cde/bar");

        // constraints
        assert_eq!(deps[4].dep_name, "bazelisk");
        assert_eq!(deps[4].datasource, Some("github-releases"));
        assert_eq!(deps[5].dep_name, "maven");
        assert_eq!(deps[5].datasource, Some("github-releases"));
    }

    // ── Ported parse_preset() tests from config/presets/parse.spec.ts ──────────

    // Ported: "returns default package name" — config/presets/parse.spec.ts line 6
    #[test]
    fn pp_returns_default_package_name() {
        let p = parse_preset(":base");
        assert_eq!(p.preset_source, "internal");
        assert_eq!(p.repo, "default");
        assert_eq!(p.preset_name, "base");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.tag, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses github" — config/presets/parse.spec.ts line 17
    #[test]
    fn pp_parses_github() {
        let p = parse_preset("github>some/repo");
        assert_eq!(p.preset_source, "github");
        assert_eq!(p.repo, "some/repo");
        assert_eq!(p.preset_name, "default");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.tag, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "handles special chars" — config/presets/parse.spec.ts line 28
    #[test]
    fn pp_handles_special_chars() {
        let p = parse_preset("github>some/repo:foo+bar");
        assert_eq!(p.preset_source, "github");
        assert_eq!(p.repo, "some/repo");
        assert_eq!(p.preset_name, "foo+bar");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses github subfiles" — config/presets/parse.spec.ts line 39
    #[test]
    fn pp_parses_github_subfiles() {
        let p = parse_preset("github>some/repo:somefile");
        assert_eq!(p.preset_source, "github");
        assert_eq!(p.repo, "some/repo");
        assert_eq!(p.preset_name, "somefile");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses github subfiles with preset name" — config/presets/parse.spec.ts line 50
    #[test]
    fn pp_parses_github_subfiles_with_preset_name() {
        let p = parse_preset("github>some/repo:somefile/somepreset");
        assert_eq!(p.preset_source, "github");
        assert_eq!(p.repo, "some/repo");
        assert_eq!(p.preset_name, "somefile/somepreset");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses github file with preset name with .json extension" — config/presets/parse.spec.ts line 61
    #[test]
    fn pp_parses_github_file_with_json_extension() {
        let p = parse_preset("github>some/repo:somefile.json");
        assert_eq!(p.preset_source, "github");
        assert_eq!(p.repo, "some/repo");
        assert_eq!(p.preset_name, "somefile.json");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.tag, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses github file with preset name with .json5 extension" — config/presets/parse.spec.ts line 73
    #[test]
    fn pp_parses_github_file_with_json5_extension() {
        let p = parse_preset("github>some/repo:somefile.json5");
        assert_eq!(p.preset_source, "github");
        assert_eq!(p.repo, "some/repo");
        assert_eq!(p.preset_name, "somefile.json5");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.tag, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses github subfiles with preset name with .json extension" — config/presets/parse.spec.ts line 85
    #[test]
    fn pp_parses_github_subfiles_with_json_extension() {
        let p = parse_preset("github>some/repo:somefile.json/somepreset");
        assert_eq!(p.preset_source, "github");
        assert_eq!(p.repo, "some/repo");
        assert_eq!(p.preset_name, "somefile.json/somepreset");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.tag, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses github subfiles with preset name with .json5 extension" — config/presets/parse.spec.ts line 97
    #[test]
    fn pp_parses_github_subfiles_with_json5_extension() {
        let p = parse_preset("github>some/repo:somefile.json5/somepreset");
        assert_eq!(p.preset_source, "github");
        assert_eq!(p.repo, "some/repo");
        assert_eq!(p.preset_name, "somefile.json5/somepreset");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.tag, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses github subfiles with preset and sub-preset name" — config/presets/parse.spec.ts line 111
    #[test]
    fn pp_parses_github_subfiles_with_sub_preset_name() {
        let p = parse_preset("github>some/repo:somefile/somepreset/somesubpreset");
        assert_eq!(p.preset_source, "github");
        assert_eq!(p.repo, "some/repo");
        assert_eq!(p.preset_name, "somefile/somepreset/somesubpreset");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses github subdirectories" — config/presets/parse.spec.ts line 124
    #[test]
    fn pp_parses_github_subdirectories() {
        let p = parse_preset("github>some/repo//somepath/somesubpath/somefile");
        assert_eq!(p.preset_source, "github");
        assert_eq!(p.repo, "some/repo");
        assert_eq!(p.preset_name, "somefile");
        assert_eq!(p.preset_path, Some("somepath/somesubpath".to_owned()));
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses github toplevel file using subdirectory syntax" — config/presets/parse.spec.ts line 137
    #[test]
    fn pp_parses_github_toplevel_file_subdirectory_syntax() {
        let p = parse_preset("github>some/repo//somefile");
        assert_eq!(p.preset_source, "github");
        assert_eq!(p.repo, "some/repo");
        assert_eq!(p.preset_name, "somefile");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses gitlab" — config/presets/parse.spec.ts line 148
    #[test]
    fn pp_parses_gitlab() {
        let p = parse_preset("gitlab>some/repo");
        assert_eq!(p.preset_source, "gitlab");
        assert_eq!(p.repo, "some/repo");
        assert_eq!(p.preset_name, "default");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses gitea" — config/presets/parse.spec.ts line 159
    #[test]
    fn pp_parses_gitea() {
        let p = parse_preset("gitea>some/repo");
        assert_eq!(p.preset_source, "gitea");
        assert_eq!(p.repo, "some/repo");
        assert_eq!(p.preset_name, "default");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses forgejo" — config/presets/parse.spec.ts line 170
    #[test]
    fn pp_parses_forgejo() {
        let p = parse_preset("forgejo>some/repo");
        assert_eq!(p.preset_source, "forgejo");
        assert_eq!(p.repo, "some/repo");
        assert_eq!(p.preset_name, "default");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses local" — config/presets/parse.spec.ts line 181
    #[test]
    fn pp_parses_local() {
        let p = parse_preset("local>some/repo");
        assert_eq!(p.preset_source, "local");
        assert_eq!(p.repo, "some/repo");
        assert_eq!(p.preset_name, "default");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses local with spaces" — config/presets/parse.spec.ts line 192
    #[test]
    fn pp_parses_local_with_spaces() {
        let p = parse_preset("local>A2B CD/A2B_Renovate");
        assert_eq!(p.preset_source, "local");
        assert_eq!(p.repo, "A2B CD/A2B_Renovate");
        assert_eq!(p.preset_name, "default");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses local with subdirectory" — config/presets/parse.spec.ts line 203
    #[test]
    fn pp_parses_local_with_subdirectory() {
        let p = parse_preset("local>some-group/some-repo//some-dir/some-file");
        assert_eq!(p.preset_source, "local");
        assert_eq!(p.repo, "some-group/some-repo");
        assert_eq!(p.preset_name, "some-file");
        assert_eq!(p.preset_path, Some("some-dir".to_owned()));
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses local with spaces and subdirectory" — config/presets/parse.spec.ts line 216
    #[test]
    fn pp_parses_local_with_spaces_and_subdirectory() {
        let p = parse_preset("local>A2B CD/A2B_Renovate//some-dir/some-file");
        assert_eq!(p.preset_source, "local");
        assert_eq!(p.repo, "A2B CD/A2B_Renovate");
        assert_eq!(p.preset_name, "some-file");
        assert_eq!(p.preset_path, Some("some-dir".to_owned()));
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses local with sub preset and tag" — config/presets/parse.spec.ts line 229
    #[test]
    fn pp_parses_local_with_sub_preset_and_tag() {
        let p = parse_preset("local>some-group/some-repo:some-file/subpreset#1.2.3");
        assert_eq!(p.preset_source, "local");
        assert_eq!(p.repo, "some-group/some-repo");
        assert_eq!(p.preset_name, "some-file/subpreset");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.tag, Some("1.2.3".to_owned()));
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses local with subdirectory and tag" — config/presets/parse.spec.ts line 243
    #[test]
    fn pp_parses_local_with_subdirectory_and_tag() {
        let p = parse_preset("local>some-group/some-repo//some-dir/some-file#1.2.3");
        assert_eq!(p.preset_source, "local");
        assert_eq!(p.repo, "some-group/some-repo");
        assert_eq!(p.preset_name, "some-file");
        assert_eq!(p.preset_path, Some("some-dir".to_owned()));
        assert_eq!(p.tag, Some("1.2.3".to_owned()));
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses local with subdirectory and branch/tag with a slash" — config/presets/parse.spec.ts line 257
    #[test]
    fn pp_parses_local_with_subdirectory_and_slash_tag() {
        let p = parse_preset("local>PROJECT/repository//path/to/preset#feature/branch");
        assert_eq!(p.preset_source, "local");
        assert_eq!(p.repo, "PROJECT/repository");
        assert_eq!(p.preset_name, "preset");
        assert_eq!(p.preset_path, Some("path/to".to_owned()));
        assert_eq!(p.tag, Some("feature/branch".to_owned()));
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses local with sub preset and branch/tag with a slash" — config/presets/parse.spec.ts line 271
    #[test]
    fn pp_parses_local_with_sub_preset_and_slash_tag() {
        let p = parse_preset("local>PROJECT/repository:preset/subpreset#feature/branch");
        assert_eq!(p.preset_source, "local");
        assert_eq!(p.repo, "PROJECT/repository");
        assert_eq!(p.preset_name, "preset/subpreset");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.tag, Some("feature/branch".to_owned()));
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses local repo with presetPath with URL-encoded characters" — config/presets/parse.spec.ts line 285
    #[test]
    fn pp_parses_local_url_encoded_with_preset_path() {
        let p = parse_preset("local>some%20group/some%20repo//some-dir/some-file");
        assert_eq!(p.preset_source, "local");
        assert_eq!(p.repo, "some%20group/some%20repo");
        assert_eq!(p.preset_name, "some-file");
        assert_eq!(p.preset_path, Some("some-dir".to_owned()));
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses local repo with URL-encoded characters" — config/presets/parse.spec.ts line 298
    #[test]
    fn pp_parses_local_url_encoded() {
        let p = parse_preset("local>some%20group/some%20repo//some-file");
        assert_eq!(p.preset_source, "local");
        assert_eq!(p.repo, "some%20group/some%20repo");
        assert_eq!(p.preset_name, "some-file");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses no prefix as local" — config/presets/parse.spec.ts line 309
    #[test]
    fn pp_parses_no_prefix_as_local() {
        let p = parse_preset("some/repo");
        assert_eq!(p.preset_source, "local");
        assert_eq!(p.repo, "some/repo");
        assert_eq!(p.preset_name, "default");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses local Bitbucket user repo with preset name" — config/presets/parse.spec.ts line 320
    #[test]
    fn pp_parses_local_bitbucket_user_repo_with_preset_name() {
        let p = parse_preset("local>~john_doe/repo//somefile");
        assert_eq!(p.preset_source, "local");
        assert_eq!(p.repo, "~john_doe/repo");
        assert_eq!(p.preset_name, "somefile");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses local Bitbucket user repo" — config/presets/parse.spec.ts line 331
    #[test]
    fn pp_parses_local_bitbucket_user_repo() {
        let p = parse_preset("local>~john_doe/renovate-config");
        assert_eq!(p.preset_source, "local");
        assert_eq!(p.repo, "~john_doe/renovate-config");
        assert_eq!(p.preset_name, "default");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "returns default package name with params" — config/presets/parse.spec.ts line 342
    #[test]
    fn pp_returns_default_package_name_with_params() {
        let p = parse_preset(":group(packages/eslint, eslint)");
        assert_eq!(p.preset_source, "internal");
        assert_eq!(p.repo, "default");
        assert_eq!(p.preset_name, "group");
        assert_eq!(p.preset_path, None);
        assert_eq!(
            p.params,
            Some(vec!["packages/eslint".to_owned(), "eslint".to_owned()])
        );
        assert_eq!(p.raw_params, Some("packages/eslint, eslint".to_owned()));
    }

    // Ported: "returns simple scope" — config/presets/parse.spec.ts line 354
    #[test]
    fn pp_returns_simple_scope() {
        let p = parse_preset("@somescope");
        assert_eq!(p.preset_source, "npm");
        assert_eq!(p.repo, "@somescope/renovate-config");
        assert_eq!(p.preset_name, "default");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "returns simple scope and params" — config/presets/parse.spec.ts line 365
    #[test]
    fn pp_returns_simple_scope_and_params() {
        let p = parse_preset("@somescope(param1)");
        assert_eq!(p.preset_source, "npm");
        assert_eq!(p.repo, "@somescope/renovate-config");
        assert_eq!(p.preset_name, "default");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, Some(vec!["param1".to_owned()]));
        assert_eq!(p.raw_params, Some("param1".to_owned()));
    }

    // Ported: "returns scope with repo and default" — config/presets/parse.spec.ts line 376
    #[test]
    fn pp_returns_scope_with_repo_and_default() {
        let p = parse_preset("@somescope/somepackagename");
        assert_eq!(p.preset_source, "npm");
        assert_eq!(p.repo, "@somescope/somepackagename");
        assert_eq!(p.preset_name, "default");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "returns scope with repo and params and default" — config/presets/parse.spec.ts line 387
    #[test]
    fn pp_returns_scope_with_repo_and_params_and_default() {
        let p = parse_preset("@somescope/somepackagename(param1, param2, param3)");
        assert_eq!(p.preset_source, "npm");
        assert_eq!(p.repo, "@somescope/somepackagename");
        assert_eq!(p.preset_name, "default");
        assert_eq!(p.preset_path, None);
        assert_eq!(
            p.params,
            Some(vec![
                "param1".to_owned(),
                "param2".to_owned(),
                "param3".to_owned()
            ])
        );
        assert_eq!(p.raw_params, Some("param1, param2, param3".to_owned()));
    }

    // Ported: "returns scope with presetName" — config/presets/parse.spec.ts line 400
    #[test]
    fn pp_returns_scope_with_preset_name() {
        let p = parse_preset("@somescope:somePresetName");
        assert_eq!(p.preset_source, "npm");
        assert_eq!(p.repo, "@somescope/renovate-config");
        assert_eq!(p.preset_name, "somePresetName");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "returns scope with presetName and params" — config/presets/parse.spec.ts line 411
    #[test]
    fn pp_returns_scope_with_preset_name_and_params() {
        let p = parse_preset("@somescope:somePresetName(param1)");
        assert_eq!(p.preset_source, "npm");
        assert_eq!(p.repo, "@somescope/renovate-config");
        assert_eq!(p.preset_name, "somePresetName");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, Some(vec!["param1".to_owned()]));
        assert_eq!(p.raw_params, Some("param1".to_owned()));
    }

    // Ported: "returns scope with repo and presetName" — config/presets/parse.spec.ts line 422
    #[test]
    fn pp_returns_scope_with_repo_and_preset_name() {
        let p = parse_preset("@somescope/somepackagename:somePresetName");
        assert_eq!(p.preset_source, "npm");
        assert_eq!(p.repo, "@somescope/somepackagename");
        assert_eq!(p.preset_name, "somePresetName");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "returns scope with repo and presetName and params" — config/presets/parse.spec.ts line 433
    #[test]
    fn pp_returns_scope_with_repo_and_preset_name_and_params() {
        let p = parse_preset("@somescope/somepackagename:somePresetName(param1, param2)");
        assert_eq!(p.preset_source, "npm");
        assert_eq!(p.repo, "@somescope/somepackagename");
        assert_eq!(p.preset_name, "somePresetName");
        assert_eq!(p.preset_path, None);
        assert_eq!(
            p.params,
            Some(vec!["param1".to_owned(), "param2".to_owned()])
        );
        assert_eq!(p.raw_params, Some("param1, param2".to_owned()));
    }

    // Ported: "returns non-scoped default" — config/presets/parse.spec.ts line 449
    #[test]
    fn pp_returns_non_scoped_default() {
        let p = parse_preset("somepackage");
        assert_eq!(p.preset_source, "npm");
        assert_eq!(p.repo, "renovate-config-somepackage");
        assert_eq!(p.preset_name, "default");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "returns non-scoped package name" — config/presets/parse.spec.ts line 460
    #[test]
    fn pp_returns_non_scoped_package_name() {
        let p = parse_preset("somepackage:webapp");
        assert_eq!(p.preset_source, "npm");
        assert_eq!(p.repo, "renovate-config-somepackage");
        assert_eq!(p.preset_name, "webapp");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "returns non-scoped package name full" — config/presets/parse.spec.ts line 471
    #[test]
    fn pp_returns_non_scoped_package_name_full() {
        let p = parse_preset("renovate-config-somepackage:webapp");
        assert_eq!(p.preset_source, "npm");
        assert_eq!(p.repo, "renovate-config-somepackage");
        assert_eq!(p.preset_name, "webapp");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "returns non-scoped package name with params" — config/presets/parse.spec.ts line 482
    #[test]
    fn pp_returns_non_scoped_package_name_with_params() {
        let p = parse_preset("somepackage:webapp(param1)");
        assert_eq!(p.preset_source, "npm");
        assert_eq!(p.repo, "renovate-config-somepackage");
        assert_eq!(p.preset_name, "webapp");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, Some(vec!["param1".to_owned()]));
        assert_eq!(p.raw_params, Some("param1".to_owned()));
    }

    // Ported: "parses HTTPS URLs for gitea" — config/presets/parse.spec.ts line 493
    #[test]
    fn pp_parses_https_urls_for_gitea() {
        let p =
            parse_preset("https://my.server/gitea/renovate-config/raw/branch/main/default.json");
        assert_eq!(p.preset_source, "http");
        assert_eq!(
            p.repo,
            "https://my.server/gitea/renovate-config/raw/branch/main/default.json"
        );
        assert_eq!(p.preset_name, "");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses HTTPS URLs for forgejo" — config/presets/parse.spec.ts line 508
    #[test]
    fn pp_parses_https_urls_for_forgejo() {
        let p =
            parse_preset("https://my.server/forgejo/renovate-config/raw/branch/main/default.json");
        assert_eq!(p.preset_source, "http");
        assert_eq!(
            p.repo,
            "https://my.server/forgejo/renovate-config/raw/branch/main/default.json"
        );
        assert_eq!(p.preset_name, "");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses HTTP URLs" — config/presets/parse.spec.ts line 523
    #[test]
    fn pp_parses_http_urls() {
        let p = parse_preset(
            "http://my.server/users/me/repos/renovate-presets/raw/default.json?at=refs%2Fheads%2Fmain",
        );
        assert_eq!(p.preset_source, "http");
        assert_eq!(
            p.repo,
            "http://my.server/users/me/repos/renovate-presets/raw/default.json?at=refs%2Fheads%2Fmain"
        );
        assert_eq!(p.preset_name, "");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, None);
        assert_eq!(p.raw_params, None);
    }

    // Ported: "parses HTTPS URLs with parameters for gitea" — config/presets/parse.spec.ts line 538
    #[test]
    fn pp_parses_https_urls_with_params_for_gitea() {
        let p = parse_preset(
            "https://my.server/gitea/renovate-config/raw/branch/main/default.json(param1)",
        );
        assert_eq!(p.preset_source, "http");
        assert_eq!(
            p.repo,
            "https://my.server/gitea/renovate-config/raw/branch/main/default.json"
        );
        assert_eq!(p.preset_name, "");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, Some(vec!["param1".to_owned()]));
        assert_eq!(p.raw_params, Some("param1".to_owned()));
    }

    // Ported: "parses HTTPS URLs with parameters for forgejo" — config/presets/parse.spec.ts line 553
    #[test]
    fn pp_parses_https_urls_with_params_for_forgejo() {
        let p = parse_preset(
            "https://my.server/forgejo/renovate-config/raw/branch/main/default.json(param1)",
        );
        assert_eq!(p.preset_source, "http");
        assert_eq!(
            p.repo,
            "https://my.server/forgejo/renovate-config/raw/branch/main/default.json"
        );
        assert_eq!(p.preset_name, "");
        assert_eq!(p.preset_path, None);
        assert_eq!(p.params, Some(vec!["param1".to_owned()]));
        assert_eq!(p.raw_params, Some("param1".to_owned()));
    }
}
