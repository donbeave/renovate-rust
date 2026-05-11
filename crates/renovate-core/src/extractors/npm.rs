//! package.json dependency extractor.
//!
//! Parses an npm `package.json` file and returns the set of package
//! dependencies with their version constraints, ready for registry lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/npm/extract/common/package-file.ts`
//! - `lib/modules/manager/npm/dep-types.ts` — `knownDepTypes`
//!
//! ## Supported dep sections
//!
//! Four standard dependency sections are extracted:
//! `dependencies`, `devDependencies`, `peerDependencies`,
//! `optionalDependencies`.
//!
//! ## Skip-reason classification
//!
//! Constraint strings that are not plain semver ranges are classified and
//! skipped:
//! - `workspace:*` / `workspace:^` etc. — pnpm/yarn workspace protocol
//! - `file:../path` / `link:../path` — local path reference
//! - `github:owner/repo` / `gitlab:...` / `bitbucket:...` — git platform shorthand
//! - `git+https://...` / `git://...` — git URL
//! - `http://...` / `https://...` — URL install
//! - `npm:other-pkg@...` — npm alias (deferred)

use std::collections::BTreeMap;

use serde::Deserialize;
use thiserror::Error;

/// Why an npm dependency is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NpmSkipReason {
    /// Dependency uses the workspace protocol (`workspace:*`).
    WorkspaceProtocol,
    /// Dependency is a local file/link reference (`file:../path`).
    LocalPath,
    /// Dependency is resolved from a git source.
    GitSource,
    /// Dependency is installed from a URL.
    UrlInstall,
    /// Dependency uses an npm alias (`npm:other-pkg`).
    NpmAlias,
}

/// Which `package.json` section the dep came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpmDepType {
    Regular,
    Dev,
    Peer,
    Optional,
    /// yarn `resolutions` override.
    Resolutions,
    /// npm 8+ `overrides` override.
    Overrides,
}

impl NpmDepType {
    /// Return the Renovate-compatible dep type string used in `matchDepTypes`.
    pub fn as_renovate_str(&self) -> &'static str {
        match self {
            NpmDepType::Regular => "dependencies",
            NpmDepType::Dev => "devDependencies",
            NpmDepType::Peer => "peerDependencies",
            NpmDepType::Optional => "optionalDependencies",
            NpmDepType::Resolutions => "resolutions",
            NpmDepType::Overrides => "overrides",
        }
    }
}

/// A single extracted npm dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NpmExtractedDep {
    /// Package name (the key in the dep section).
    pub name: String,
    /// The version constraint string (e.g. `"^18.0.0"`).
    pub current_value: String,
    /// Which dep section this came from.
    pub dep_type: NpmDepType,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<NpmSkipReason>,
}

/// Yarn registry configuration relevant to npm package extraction.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct YarnConfig {
    pub npm_registry_server: Option<String>,
    pub npm_scopes: BTreeMap<String, YarnScopeConfig>,
}

/// Per-scope Yarn npm registry configuration.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct YarnScopeConfig {
    pub npm_registry_server: Option<String>,
}

/// Errors from parsing a `package.json`.
#[derive(Debug, Error)]
pub enum NpmExtractError {
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
}

// ── Internal deserialization ──────────────────────────────────────────────────

#[derive(Debug, Deserialize, Default)]
struct PackageJson {
    #[serde(default)]
    dependencies: BTreeMap<String, String>,
    #[serde(rename = "devDependencies", default)]
    dev_dependencies: BTreeMap<String, String>,
    #[serde(rename = "peerDependencies", default)]
    peer_dependencies: BTreeMap<String, String>,
    #[serde(rename = "optionalDependencies", default)]
    optional_dependencies: BTreeMap<String, String>,
    /// yarn `resolutions` block — flat `{ "pkg": "version" }`.
    #[serde(default)]
    resolutions: BTreeMap<String, String>,
    /// npm 8+ `overrides` block — flat `{ "pkg": "version" }`.
    #[serde(default)]
    overrides: BTreeMap<String, String>,
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Parse a `package.json` string and extract all npm dependencies.
///
/// Returns a flat list of deps across all four sections, each annotated with
/// its section type and any applicable skip reason.
pub fn extract(content: &str) -> Result<Vec<NpmExtractedDep>, NpmExtractError> {
    let pkg: PackageJson = serde_json::from_str(content)?;
    let mut out = Vec::new();

    for (section, dep_type) in [
        (&pkg.dependencies, NpmDepType::Regular),
        (&pkg.dev_dependencies, NpmDepType::Dev),
        (&pkg.peer_dependencies, NpmDepType::Peer),
        (&pkg.optional_dependencies, NpmDepType::Optional),
        (&pkg.resolutions, NpmDepType::Resolutions),
        (&pkg.overrides, NpmDepType::Overrides),
    ] {
        for (name, value) in section {
            out.push(classify(name.clone(), value, dep_type));
        }
    }

    Ok(out)
}

/// Resolve the registry URL for a package name from Yarn config.
///
/// Renovate reference: `lib/modules/manager/npm/extract/yarnrc.ts` `resolveRegistryUrl`.
pub fn resolve_yarn_registry_url(package_name: &str, config: &YarnConfig) -> Option<String> {
    if let Some(scope) = package_name
        .strip_prefix('@')
        .and_then(|rest| rest.split_once('/').map(|(scope, _)| scope))
        && let Some(scope_config) = config.npm_scopes.get(scope)
    {
        return scope_config.npm_registry_server.clone();
    }

    config.npm_registry_server.clone()
}

/// Parse the subset of `.yarnrc.yml` used for npm registry resolution.
///
/// Renovate reference: `lib/modules/manager/npm/extract/yarnrc.ts` `loadConfigFromYarnrcYml`.
pub fn load_config_from_yarnrc_yml(content: &str) -> Option<YarnConfig> {
    if content.trim().is_empty() {
        return None;
    }

    let mut config = YarnConfig::default();
    let mut current_scope: Option<String> = None;
    let mut saw_relevant_key = false;

    for raw_line in content.lines() {
        let line = raw_line.trim_end();
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        if !line.starts_with(' ') {
            current_scope = None;
            if let Some(value) = trimmed.strip_prefix("npmRegistryServer:") {
                let value = parse_yarn_string_value(value)?;
                config.npm_registry_server = Some(value);
                saw_relevant_key = true;
            } else if let Some(value) = trimmed.strip_prefix("npmScopes:") {
                if !value.trim().is_empty() {
                    return None;
                }
                saw_relevant_key = true;
            }
            continue;
        }

        let indent = raw_line.chars().take_while(|ch| *ch == ' ').count();
        if indent == 2 && trimmed.ends_with(':') {
            let scope = trimmed.trim_end_matches(':').to_owned();
            config.npm_scopes.entry(scope.clone()).or_default();
            current_scope = Some(scope);
        } else if indent == 2 && trimmed.contains(':') {
            return None;
        } else if indent == 4
            && let Some(scope) = &current_scope
            && let Some(value) = trimmed.strip_prefix("npmRegistryServer:")
        {
            let value = parse_yarn_string_value(value)?;
            config
                .npm_scopes
                .entry(scope.clone())
                .or_default()
                .npm_registry_server = Some(value);
        }
    }

    saw_relevant_key.then_some(config)
}

/// Parse legacy `.yarnrc` registry settings.
///
/// Renovate reference: `lib/modules/manager/npm/extract/yarnrc.ts`
/// `loadConfigFromLegacyYarnrc`.
pub fn load_config_from_legacy_yarnrc(content: &str) -> YarnConfig {
    let mut config = YarnConfig::default();

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("--") {
            continue;
        }

        let Some((raw_key, raw_value)) = split_legacy_yarnrc_line(line) else {
            continue;
        };
        let key = unquote_yarnrc_token(raw_key);
        let value = unquote_yarnrc_token(raw_value);

        if key == "registry" {
            config.npm_registry_server = Some(value);
        } else if let Some(scope) = key
            .strip_prefix('@')
            .and_then(|key| key.strip_suffix(":registry"))
        {
            config
                .npm_scopes
                .entry(scope.to_owned())
                .or_default()
                .npm_registry_server = Some(value);
        }
    }

    config
}

fn parse_yarn_string_value(value: &str) -> Option<String> {
    let value = value.trim().trim_matches('"').trim_matches('\'');
    if value.is_empty() || value.parse::<i64>().is_ok() {
        return None;
    }
    Some(value.to_owned())
}

fn split_legacy_yarnrc_line(line: &str) -> Option<(&str, &str)> {
    let trimmed = line.trim();
    if let Some(rest) = trimmed.strip_prefix('"') {
        let end = rest.find('"')? + 1;
        let key = &trimmed[..=end];
        let value = trimmed[end + 1..].trim();
        return (!value.is_empty()).then_some((key, value));
    }

    trimmed.split_once(char::is_whitespace)
}

fn unquote_yarnrc_token(token: &str) -> String {
    token.trim().trim_matches('"').trim_matches('\'').to_owned()
}

fn classify(name: String, value: &str, dep_type: NpmDepType) -> NpmExtractedDep {
    let skip_reason = skip_reason_for(value);
    NpmExtractedDep {
        name,
        current_value: value.to_owned(),
        dep_type,
        skip_reason,
    }
}

/// Classify an npm version string and return the skip reason, if any.
///
/// Returns `None` for plain semver-style constraints that should be looked up
/// in the npm registry.
fn skip_reason_for(value: &str) -> Option<NpmSkipReason> {
    let v = value.trim();

    // workspace protocol (pnpm / yarn)
    if v.starts_with("workspace:") {
        return Some(NpmSkipReason::WorkspaceProtocol);
    }

    // local path references
    if v.starts_with("file:")
        || v.starts_with("link:")
        || v.starts_with("portal:")
        || v.starts_with("patch:")
    {
        return Some(NpmSkipReason::LocalPath);
    }

    // git URL forms
    if v.starts_with("git+")
        || v.starts_with("git://")
        || v.starts_with("github:")
        || v.starts_with("gitlab:")
        || v.starts_with("bitbucket:")
        || v.starts_with("gist:")
        // GitHub shorthand: "owner/repo" (contains exactly one slash, no sigil)
        || (v.contains('/') && !v.starts_with('@') && !v.starts_with("http") && v.split('/').count() == 2)
    {
        return Some(NpmSkipReason::GitSource);
    }

    // URL installs
    if v.starts_with("http://") || v.starts_with("https://") {
        return Some(NpmSkipReason::UrlInstall);
    }

    // npm alias
    if v.starts_with("npm:") {
        return Some(NpmSkipReason::NpmAlias);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_ok(json: &str) -> Vec<NpmExtractedDep> {
        extract(json).expect("parse should succeed")
    }

    fn yarn_config(default_registry: Option<&str>, scopes: &[(&str, Option<&str>)]) -> YarnConfig {
        YarnConfig {
            npm_registry_server: default_registry.map(str::to_owned),
            npm_scopes: scopes
                .iter()
                .map(|(scope, registry)| {
                    (
                        (*scope).to_owned(),
                        YarnScopeConfig {
                            npm_registry_server: registry.map(str::to_owned),
                        },
                    )
                })
                .collect(),
        }
    }

    // Ported: "considers default registry" — npm/extract/yarnrc.spec.ts line 10
    #[test]
    fn yarnrc_resolve_registry_url_considers_default_registry() {
        let config = yarn_config(Some("https://private.example.com/npm"), &[]);
        assert_eq!(
            resolve_yarn_registry_url("a-package", &config).as_deref(),
            Some("https://private.example.com/npm")
        );
    }

    // Ported: "chooses matching scoped registry over default registry" — npm/extract/yarnrc.spec.ts line 17
    #[test]
    fn yarnrc_resolve_registry_url_prefers_matching_scope() {
        let config = yarn_config(
            Some("https://private.example.com/npm"),
            &[("scope", Some("https://scope.example.com/npm"))],
        );
        assert_eq!(
            resolve_yarn_registry_url("@scope/a-package", &config).as_deref(),
            Some("https://scope.example.com/npm")
        );
    }

    // Ported: "ignores non matching scoped registry" — npm/extract/yarnrc.spec.ts line 29
    #[test]
    fn yarnrc_resolve_registry_url_ignores_non_matching_scope() {
        let config = yarn_config(
            None,
            &[("other-scope", Some("https://other-scope.example.com/npm"))],
        );
        assert!(resolve_yarn_registry_url("@scope/a-package", &config).is_none());
    }

    // Ported: "ignores partial scope match" — npm/extract/yarnrc.spec.ts line 40
    #[test]
    fn yarnrc_resolve_registry_url_ignores_partial_scope_match() {
        let config = yarn_config(None, &[("scope", Some("https://scope.example.com/npm"))]);
        assert!(resolve_yarn_registry_url("@scope-2/a-package", &config).is_none());
    }

    // Ported: "ignores missing scope registryServer" — npm/extract/yarnrc.spec.ts line 51
    #[test]
    fn yarnrc_resolve_registry_url_ignores_missing_scope_registry_server() {
        let config = yarn_config(Some("https://private.example.com/npm"), &[("scope", None)]);
        assert!(resolve_yarn_registry_url("@scope/a-package", &config).is_none());
    }

    // Ported: "produces expected config (%s)" — npm/extract/yarnrc.spec.ts line 63
    #[test]
    fn load_config_from_yarnrc_yml_produces_expected_config() {
        let cases = [
            (
                "npmRegistryServer: https://npm.example.com",
                Some(yarn_config(Some("https://npm.example.com"), &[])),
            ),
            (
                "npmRegistryServer: https://npm.example.com\nnpmScopes:\n  foo:\n    npmRegistryServer: https://npm-foo.example.com\n",
                Some(yarn_config(
                    Some("https://npm.example.com"),
                    &[("foo", Some("https://npm-foo.example.com"))],
                )),
            ),
            (
                "npmRegistryServer: https://npm.example.com\nnodeLinker: pnp\n",
                Some(yarn_config(Some("https://npm.example.com"), &[])),
            ),
            ("npmRegistryServer: 42", None),
            ("npmScopes: 42", None),
            ("npmScopes:\n  foo: 42\n", None),
            ("npmScopes:\n  foo:\n    npmRegistryServer: 42\n", None),
            ("", None),
        ];

        for (content, expected) in cases {
            assert_eq!(load_config_from_yarnrc_yml(content), expected);
        }
    }

    // Ported: "produces expected config (%s)" — npm/extract/yarnrc.spec.ts line 117
    #[test]
    fn load_config_from_legacy_yarnrc_produces_expected_config() {
        let cases = [
            (
                "# yarn lockfile v1\nregistry \"https://npm.example.com\"\n",
                yarn_config(Some("https://npm.example.com"), &[]),
            ),
            (
                "disturl \"https://npm-dist.example.com\"\nregistry https://npm.example.com\nsass_binary_site \"https://node-sass.example.com\"\n",
                yarn_config(Some("https://npm.example.com"), &[]),
            ),
            (
                "--install.frozen-lockfile true\n\"registry\" \"https://npm.example.com\"\n\"@foo:registry\" \"https://npm-foo.example.com\"\n\"@bar:registry\" \"https://npm-bar.example.com\"\n",
                yarn_config(
                    Some("https://npm.example.com"),
                    &[
                        ("foo", Some("https://npm-foo.example.com")),
                        ("bar", Some("https://npm-bar.example.com")),
                    ],
                ),
            ),
        ];

        for (content, expected) in cases {
            assert_eq!(load_config_from_legacy_yarnrc(content), expected);
        }
    }

    #[test]
    fn extracts_all_four_sections() {
        let json = r#"{
          "dependencies": { "express": "^4.18.0" },
          "devDependencies": { "jest": "^29.0" },
          "peerDependencies": { "react": ">=17" },
          "optionalDependencies": { "fsevents": "^2.0" }
        }"#;
        let deps = extract_ok(json);
        assert_eq!(deps.len(), 4);
        assert!(
            deps.iter()
                .any(|d| d.name == "express" && d.dep_type == NpmDepType::Regular)
        );
        assert!(
            deps.iter()
                .any(|d| d.name == "jest" && d.dep_type == NpmDepType::Dev)
        );
        assert!(
            deps.iter()
                .any(|d| d.name == "react" && d.dep_type == NpmDepType::Peer)
        );
        assert!(
            deps.iter()
                .any(|d| d.name == "fsevents" && d.dep_type == NpmDepType::Optional)
        );
    }

    #[test]
    fn plain_semver_has_no_skip_reason() {
        let json =
            r#"{ "dependencies": { "lodash": "4.17.21", "axios": "^1.0", "chalk": "~5.0" } }"#;
        let deps = extract_ok(json);
        assert!(deps.iter().all(|d| d.skip_reason.is_none()));
    }

    #[test]
    fn workspace_protocol_is_skipped() {
        let json = r#"{ "dependencies": { "my-lib": "workspace:*", "other": "workspace:^1.0" } }"#;
        let deps = extract_ok(json);
        assert!(
            deps.iter()
                .all(|d| d.skip_reason == Some(NpmSkipReason::WorkspaceProtocol))
        );
    }

    #[test]
    fn file_reference_is_skipped() {
        let json =
            r#"{ "dependencies": { "local": "file:../local-lib", "linked": "link:../linked" } }"#;
        let deps = extract_ok(json);
        assert!(
            deps.iter()
                .all(|d| d.skip_reason == Some(NpmSkipReason::LocalPath))
        );
    }

    #[test]
    fn git_source_forms_are_skipped() {
        let json = r#"{ "dependencies": {
          "a": "git+https://github.com/owner/repo.git",
          "b": "github:owner/repo",
          "c": "gitlab:owner/repo",
          "d": "owner/repo"
        }}"#;
        let deps = extract_ok(json);
        assert!(
            deps.iter()
                .all(|d| d.skip_reason == Some(NpmSkipReason::GitSource))
        );
    }

    #[test]
    fn url_install_is_skipped() {
        let json = r#"{ "dependencies": { "pkg": "https://example.com/pkg.tgz" } }"#;
        let deps = extract_ok(json);
        assert_eq!(deps[0].skip_reason, Some(NpmSkipReason::UrlInstall));
    }

    #[test]
    fn npm_alias_is_skipped() {
        let json = r#"{ "dependencies": { "react": "npm:preact@^10" } }"#;
        let deps = extract_ok(json);
        assert_eq!(deps[0].skip_reason, Some(NpmSkipReason::NpmAlias));
    }

    #[test]
    fn scoped_package_name_is_not_confused_with_git_shorthand() {
        // "@scope/pkg" contains a slash but starts with "@" — must NOT be treated
        // as a git owner/repo shorthand.
        let json = r#"{ "dependencies": { "@types/node": "^20.0" } }"#;
        let deps = extract_ok(json);
        assert!(deps[0].skip_reason.is_none());
    }

    #[test]
    fn empty_package_json_returns_empty_list() {
        let json = r#"{}"#;
        let deps = extract_ok(json);
        assert!(deps.is_empty());
    }

    #[test]
    fn missing_sections_are_ignored() {
        let json = r#"{ "dependencies": { "lodash": "^4" } }"#;
        let deps = extract_ok(json);
        assert_eq!(deps.len(), 1);
    }

    #[test]
    fn extracts_yarn_resolutions() {
        let json = r#"{
          "dependencies": { "lodash": "^4.17.0" },
          "resolutions": { "minimist": "^1.2.6", "lodash": ">=4.17.21" }
        }"#;
        let deps = extract_ok(json);
        let resolutions: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == NpmDepType::Resolutions)
            .collect();
        assert_eq!(resolutions.len(), 2);
        assert!(resolutions.iter().any(|d| d.name == "minimist"));
        assert!(resolutions.iter().any(|d| d.name == "lodash"));
    }

    #[test]
    fn extracts_npm_overrides() {
        let json = r#"{
          "overrides": { "semver": "^7.5.2", "tough-cookie": ">=4.1.3" }
        }"#;
        let deps = extract_ok(json);
        let overrides: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == NpmDepType::Overrides)
            .collect();
        assert_eq!(overrides.len(), 2);
        let semver = overrides.iter().find(|d| d.name == "semver").unwrap();
        assert_eq!(semver.current_value, "^7.5.2");
        assert!(semver.skip_reason.is_none());
    }
}
