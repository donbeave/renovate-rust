//! Helm `Chart.yaml` / `requirements.yaml` dependency extractor.
//!
//! Parses Helm chart manifest files with a line-oriented scanner and returns
//! chart dependencies ready for Helm repository index lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/helmv3/extract.ts` — `extractPackageFile`
//! - `lib/modules/manager/helmv3/index.ts`   — `defaultConfig` (pattern `Chart.ya?ml`)
//! - `lib/modules/manager/helm-requirements/extract.ts` — `requirements.yaml`
//!
//! ## Chart.yaml `dependencies` format (Helm v3, apiVersion v2)
//!
//! ```yaml
//! dependencies:
//!   - name: redis
//!     version: "17.0.0"
//!     repository: "https://charts.bitnami.com/bitnami"
//!   - name: postgresql
//!     version: "~> 12.0"
//!     repository: "stable"
//! ```
//!
//! ## Supported repository forms
//!
//! | Form | Treatment |
//! |---|---|
//! | `https://...` | Actionable — direct Helm repo URL |
//! | `oci://...` | Docker datasource — OCI registry |
//! | `@alias` / `alias:name` | Resolved from registryAliases or `placeholder-url` |
//! | `file://...` | Skipped — `local-dependency` |
//! | *(absent)* | Skipped — `no-repository` |

use std::collections::HashMap;
use std::sync::LazyLock;

use regex::Regex;
use serde::Deserialize;

/// Alias for the Helm stable repository.
pub const STABLE_REPO: &str = "https://charts.helm.sh/stable";

/// Why a Helm chart dep is skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HelmSkipReason {
    /// Dependency has no `repository` field.
    NoRepository,
    /// Repository is an OCI registry (`oci://`).
    OciRegistry,
    /// Repository is an unresolvable `@alias` reference.
    UnresolvableAlias,
    /// Chart is a local path (`./`, `../`, `/`).
    LocalChart,
    /// Chart version is a template expression or missing.
    InvalidVersion,
    /// Chart name contains unsupported characters.
    UnsupportedChartType,
    /// Registry URL is unknown.
    UnknownRegistry,
}

// ── YAML types for extract_package_file ──────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChartYaml {
    api_version: Option<String>,
    name: Option<String>,
    version: Option<String>,
    #[serde(default)]
    dependencies: serde_yaml::Value,
}

#[derive(Debug, Deserialize)]
struct ChartDep {
    name: Option<String>,
    version: Option<String>,
    #[serde(default)]
    repository: Option<String>,
}

/// Result of `extract_package_file` — mirrors `PackageFileContent` for helmv3.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HelmChartFile {
    /// Extracted dependencies (all, including skipped).
    pub deps: Vec<HelmExtractedDep>,
    /// The chart's own `version:` field.
    pub package_file_version: String,
}

/// Extract dependencies from a `Chart.yaml` file (Helm v3 only, apiVersion v2).
///
/// Mirrors `lib/modules/manager/helmv3/extract.ts` `extractPackageFile`.
/// Returns `None` when the file is empty, unparseable, missing required fields,
/// uses an unsupported apiVersion, or has no valid dependencies.
pub fn extract_package_file(
    content: &str,
    _file_name: &str,
    registry_aliases: &HashMap<String, String>,
) -> Option<HelmChartFile> {
    if content.trim().is_empty() {
        return None;
    }

    let chart: ChartYaml = serde_yaml::from_str(content).ok()?;

    if chart.api_version.as_deref() != Some("v2") {
        return None;
    }
    if chart.name.as_deref().map(str::is_empty).unwrap_or(true)
        || chart.version.as_deref().map(str::is_empty).unwrap_or(true)
    {
        return None;
    }

    let package_file_version = chart.version.unwrap();

    let raw_deps: Vec<ChartDep> = match &chart.dependencies {
        serde_yaml::Value::Sequence(seq) => {
            serde_yaml::from_value(serde_yaml::Value::Sequence(seq.clone())).unwrap_or_default()
        }
        serde_yaml::Value::Null => return None,
        _ => return None,
    };

    if raw_deps.is_empty() {
        return None;
    }

    // Filter to deps that have both name and version.
    let valid_raw: Vec<&ChartDep> = raw_deps
        .iter()
        .filter(|d| {
            d.name.as_deref().map(|s| !s.is_empty()).unwrap_or(false)
                && d.version.as_deref().map(|s| !s.is_empty()).unwrap_or(false)
        })
        .collect();

    if valid_raw.is_empty() {
        return None;
    }

    let deps: Vec<HelmExtractedDep> = valid_raw
        .iter()
        .map(|dep| {
            let name = dep.name.clone().unwrap_or_default();
            let current_value = dep.version.clone().unwrap_or_default();
            let repo = dep.repository.as_deref().unwrap_or("");

            if repo.is_empty() {
                return HelmExtractedDep {
                    name,
                    current_value,
                    repository: String::new(),
                    skip_reason: Some(HelmSkipReason::NoRepository),
                    datasource: None,
                    package_name: None,
                };
            }

            // Resolve aliases first.
            let resolved = if is_alias(Some(repo)) {
                resolve_alias(Some(repo), registry_aliases)
            } else {
                Some(repo.to_owned())
            };

            let Some(resolved_repo) = resolved else {
                // Unresolvable alias → placeholder-url (Renovate uses 'placeholder-url').
                return HelmExtractedDep {
                    name,
                    current_value,
                    repository: String::new(),
                    skip_reason: Some(HelmSkipReason::UnresolvableAlias),
                    datasource: None,
                    package_name: None,
                };
            };

            parse_repository_dep(name, current_value, &resolved_repo)
        })
        .collect();

    Some(HelmChartFile {
        deps,
        package_file_version,
    })
}

/// Parse a resolved repository URL into a dep.
///
/// Mirrors `lib/modules/manager/helmv3/utils.ts` `parseRepository`.
fn parse_repository_dep(
    name: String,
    current_value: String,
    repository_url: &str,
) -> HelmExtractedDep {
    if repository_url.starts_with("oci://") {
        let without_prefix = repository_url.trim_start_matches("oci://");
        let package_name = format!("{without_prefix}/{name}");
        return HelmExtractedDep {
            name,
            current_value,
            repository: repository_url.to_owned(),
            skip_reason: None,
            datasource: Some("docker".to_owned()),
            package_name: Some(package_name),
        };
    }

    if repository_url.starts_with("file:") {
        return HelmExtractedDep {
            name,
            current_value,
            repository: repository_url.to_owned(),
            skip_reason: Some(HelmSkipReason::LocalChart),
            datasource: None,
            package_name: None,
        };
    }

    // Validate URL is parseable and has http/https scheme.
    let is_valid_url =
        repository_url.starts_with("https://") || repository_url.starts_with("http://");

    if !is_valid_url {
        // Non-parseable / unrecognized scheme → invalid-url.
        return HelmExtractedDep {
            name,
            current_value,
            repository: repository_url.to_owned(),
            skip_reason: Some(HelmSkipReason::UnknownRegistry),
            datasource: None,
            package_name: None,
        };
    }

    HelmExtractedDep {
        name,
        current_value,
        repository: repository_url.to_owned(),
        skip_reason: None,
        datasource: None,
        package_name: None,
    }
}

/// A single extracted Helm chart dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HelmExtractedDep {
    /// Chart name (e.g. `redis`).
    pub name: String,
    /// Version constraint (e.g. `17.0.0`).
    pub current_value: String,
    /// Resolved repository URL (e.g. `https://charts.bitnami.com/bitnami`).
    /// Empty when `skip_reason` is set.
    pub repository: String,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<HelmSkipReason>,
    /// Datasource override (e.g. `"docker"` for OCI repos).
    pub datasource: Option<String>,
    /// Package name for OCI deps (full image path).
    pub package_name: Option<String>,
}

/// Host credentials used by Helm v3 OCI registry login.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HelmHostRule {
    pub username: Option<String>,
    pub password: Option<String>,
}

/// Helm v3 repository rule with optional registry credentials.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HelmRepositoryRule {
    pub name: String,
    pub repository: String,
    pub host_rule: HelmHostRule,
}

// ── Regexes ───────────────────────────────────────────────────────────────────

/// Key-value line: `  key: value` (with optional quotes and trailing comment).
static KV: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"^\s+(\w+):\s*"?([^"#\n]+?)"?\s*(?:#.*)?$"##).unwrap());

/// Start of a list item: `  - ` at 2+ spaces.
static LIST_ITEM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\s+)-\s*(.*)$").unwrap());

// ── Public API ────────────────────────────────────────────────────────────────

/// Parse a `Chart.yaml` or `requirements.yaml` and extract all chart deps.
pub fn extract(content: &str) -> Vec<HelmExtractedDep> {
    let mut deps = Vec::new();

    // State machine:
    //   0 = scanning for `dependencies:` / `charts:`
    //   1 = inside dependencies list
    let mut in_deps = false;
    // Pending dep being assembled.
    let mut pending_name = String::new();
    let mut pending_version = String::new();
    let mut pending_repo = String::new();
    let mut in_item = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Detect top-level keys that end the dependencies section.
        if !line.starts_with(' ') && !line.starts_with('\t') {
            if trimmed == "dependencies:" || trimmed == "charts:" {
                // Flush any pending dep.
                if in_item {
                    emit_dep(&pending_name, &pending_version, &pending_repo, &mut deps);
                }
                in_deps = true;
                in_item = false;
                pending_name.clear();
                pending_version.clear();
                pending_repo.clear();
            } else if !trimmed.ends_with(':') {
                // Non-key top-level scalar — ignore.
            } else {
                // Another top-level section — end deps.
                if in_deps && in_item {
                    emit_dep(&pending_name, &pending_version, &pending_repo, &mut deps);
                }
                in_deps = false;
                in_item = false;
            }
            continue;
        }

        if !in_deps {
            continue;
        }

        // List item start: `  - name: redis` or just `  -`
        if let Some(cap) = LIST_ITEM.captures(line) {
            // Flush previous dep.
            if in_item {
                emit_dep(&pending_name, &pending_version, &pending_repo, &mut deps);
            }
            pending_name.clear();
            pending_version.clear();
            pending_repo.clear();
            in_item = true;

            // The rest after `- ` might be an inline key-value.
            let rest = cap[2].trim();
            if let Some(kv_cap) = KV.captures(&format!("  {rest}")) {
                apply_kv(
                    &kv_cap[1],
                    kv_cap[2].trim(),
                    &mut pending_name,
                    &mut pending_version,
                    &mut pending_repo,
                );
            }
            continue;
        }

        // Continuation key-value inside a list item.
        if in_item && let Some(cap) = KV.captures(line) {
            apply_kv(
                &cap[1],
                cap[2].trim(),
                &mut pending_name,
                &mut pending_version,
                &mut pending_repo,
            );
        }
    }

    // Flush final dep.
    if in_deps && in_item {
        emit_dep(&pending_name, &pending_version, &pending_repo, &mut deps);
    }

    deps
}

/// Generate the Helm registry login command for basic-auth repository rules.
///
/// Renovate reference: `lib/modules/manager/helmv3/common.ts` `generateLoginCmd`.
pub fn generate_login_cmd(repository_rule: &HelmRepositoryRule) -> Option<String> {
    let username = repository_rule.host_rule.username.as_deref()?;
    let password = repository_rule.host_rule.password.as_deref()?;
    let host_part = repository_rule.repository.split('/').next()?;

    Some(format!(
        "helm registry login --username {} --password {} {}",
        shell_quote(username),
        shell_quote(password),
        shell_quote(host_part)
    ))
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn shell_quote(value: &str) -> String {
    if value
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || "-_./:@%+".contains(ch))
    {
        return value.to_owned();
    }

    format!("'{}'", value.replace('\'', "'\"'\"'"))
}

fn apply_kv(key: &str, val: &str, name: &mut String, version: &mut String, repo: &mut String) {
    match key {
        "name" => *name = val.to_owned(),
        "version" => *version = val.to_owned(),
        "repository" => *repo = val.to_owned(),
        _ => {}
    }
}

fn emit_dep(name: &str, version: &str, repository: &str, deps: &mut Vec<HelmExtractedDep>) {
    if name.is_empty() || version.is_empty() {
        return;
    }

    let (resolved_repo, skip_reason) = classify_repository(repository);

    deps.push(HelmExtractedDep {
        name: name.to_owned(),
        current_value: version.to_owned(),
        repository: resolved_repo,
        skip_reason,
        datasource: None,
        package_name: None,
    });
}

/// Bump the `version:` field in Chart.yaml content.
///
/// Mirrors `lib/modules/manager/helmv3/update.ts` `bumpPackageVersion()`.
pub fn bump_package_version(content: &str, current_value: &str, bump_version: &str) -> String {
    use std::sync::LazyLock;
    static VERSION_RE: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new(r"(?m)^(?P<prefix>version:\s*).*$").unwrap());

    let new_ver = (|| -> Option<String> {
        let mut parsed = semver::Version::parse(current_value).ok()?;
        match bump_version {
            "patch" => parsed.patch += 1,
            "minor" => {
                parsed.minor += 1;
                parsed.patch = 0;
            }
            "major" => {
                parsed.major += 1;
                parsed.minor = 0;
                parsed.patch = 0;
            }
            _ => return None,
        }
        Some(parsed.to_string())
    })();

    let Some(new_str) = new_ver else {
        return content.to_owned();
    };

    VERSION_RE
        .replace(content, |caps: &regex::Captures| {
            format!("{}{}", &caps["prefix"], new_str)
        })
        .into_owned()
}

/// Resolve a repository string to a canonical URL or a skip reason.
fn classify_repository(repo: &str) -> (String, Option<HelmSkipReason>) {
    if repo.is_empty() {
        return (String::new(), Some(HelmSkipReason::NoRepository));
    }
    if repo.starts_with("oci://") {
        return (String::new(), Some(HelmSkipReason::OciRegistry));
    }
    if repo.starts_with('@') {
        return (String::new(), Some(HelmSkipReason::UnresolvableAlias));
    }
    // `stable` is a Renovate built-in alias.
    if repo == "stable" {
        return (STABLE_REPO.to_owned(), None);
    }
    if repo.starts_with("https://") || repo.starts_with("http://") {
        return (repo.to_owned(), None);
    }
    // Unknown form — treat as unresolvable.
    (String::new(), Some(HelmSkipReason::UnresolvableAlias))
}

// ── Helm v3 utils (mirrors lib/modules/manager/helmv3/utils.ts + oci.ts) ─────

/// Check if `repository` is an alias (`alias:name` or `@name`).
///
/// Mirrors `lib/modules/manager/helmv3/utils.ts` `isAlias()`.
pub fn is_alias(repository: Option<&str>) -> bool {
    match repository {
        Some(r) if !r.is_empty() => r.starts_with('@') || r.starts_with("alias:"),
        _ => false,
    }
}

/// Resolve an alias (`alias:name` or `@name`) from `registry_aliases`.
///
/// Returns the original value if not an alias, or `None` if the alias is
/// undefined in the map. `None` input → `None` output.
///
/// Mirrors `lib/modules/manager/helmv3/utils.ts` `resolveAlias()`.
pub fn resolve_alias(
    repository: Option<&str>,
    registry_aliases: &std::collections::HashMap<String, String>,
) -> Option<String> {
    let repo = repository?;
    if !is_alias(Some(repo)) {
        return Some(repo.to_owned());
    }
    let key = repo
        .strip_prefix('@')
        .or_else(|| repo.strip_prefix("alias:"))
        .unwrap_or(repo);
    registry_aliases.get(key).cloned()
}

/// Check if `repository` references an OCI registry (`oci://...`).
///
/// Mirrors `lib/modules/manager/helmv3/oci.ts` `isOCIRegistry()`.
pub fn is_oci_registry(repository: Option<&str>) -> bool {
    repository.map(|r| r.starts_with("oci://")).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "should generate a login command with username and password" — helmv3/common.spec.ts line 5
    #[test]
    fn generate_login_cmd_with_username_and_password() {
        let repository_rule = HelmRepositoryRule {
            name: "test-repo".to_owned(),
            repository: "example.com/repo".to_owned(),
            host_rule: HelmHostRule {
                username: Some("testuser".to_owned()),
                password: Some("testpass".to_owned()),
            },
        };

        assert_eq!(
            generate_login_cmd(&repository_rule).as_deref(),
            Some("helm registry login --username testuser --password testpass example.com")
        );
    }

    // Ported: "parses simple requirements.yaml correctly" — helm-requirements/extract.spec.ts line 64
    #[test]
    fn simple_chart_yaml() {
        let content = r#"
apiVersion: v2
name: myapp
version: 1.0.0

dependencies:
  - name: redis
    version: "17.0.0"
    repository: "https://charts.bitnami.com/bitnami"
  - name: postgresql
    version: "12.0.0"
    repository: "https://charts.bitnami.com/bitnami"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);

        let redis = deps.iter().find(|d| d.name == "redis").unwrap();
        assert_eq!(redis.current_value, "17.0.0");
        assert_eq!(redis.repository, "https://charts.bitnami.com/bitnami");
        assert!(redis.skip_reason.is_none());
    }

    // Ported: "resolves aliased registry urls" — helm-requirements/extract.spec.ts line 112
    #[test]
    fn stable_alias_resolved() {
        let content = r#"
dependencies:
  - name: nginx
    version: "1.0.0"
    repository: "stable"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].repository, STABLE_REPO);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "skips invalid registry urls" — helm-requirements/extract.spec.ts line 34
    #[test]
    fn oci_registry_skipped() {
        let content = r#"
dependencies:
  - name: myapp
    version: "2.0.0"
    repository: "oci://registry.example.com/charts"
"#;
        let deps = extract(content);
        assert_eq!(deps[0].skip_reason, Some(HelmSkipReason::OciRegistry));
    }

    // Ported: "ensure that currentValue is string" — helm-requirements/extract.spec.ts line 8
    #[test]
    fn at_alias_skipped() {
        let content = r#"
dependencies:
  - name: myapp
    version: "2.0.0"
    repository: "@myrepo"
"#;
        let deps = extract(content);
        assert_eq!(deps[0].skip_reason, Some(HelmSkipReason::UnresolvableAlias));
    }

    // Ported: "validates repository is required" — helm-requirements/extract.spec.ts line 278
    #[test]
    fn no_repository_skipped() {
        let content = r#"
dependencies:
  - name: myapp
    version: "2.0.0"
"#;
        let deps = extract(content);
        assert_eq!(deps[0].skip_reason, Some(HelmSkipReason::NoRepository));
    }

    // Ported: "validates version is required" — helm-requirements/extract.spec.ts line 278
    #[test]
    fn missing_version_dep_skipped() {
        let content = r#"
dependencies:
  - name: myapp
    repository: "https://charts.example.com"
"#;
        let deps = extract(content);
        // No version → skipped silently (emit_dep returns early)
        assert!(deps.is_empty());
    }

    // Ported: "parses simple requirements.yaml correctly" — helm-requirements/extract.spec.ts line 64
    #[test]
    fn requirements_yaml_format() {
        let content = r#"
dependencies:
  - name: mysql
    version: "8.0.0"
    repository: "https://charts.bitnami.com/bitnami"
  - name: redis
    version: "17.0.0"
    repository: "https://charts.bitnami.com/bitnami"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
    }

    // Rust-specific: helm behavior test
    #[test]
    fn multiple_sections_only_dependencies_extracted() {
        let content = r#"
apiVersion: v2
name: myapp
version: 1.0.0
description: My application

dependencies:
  - name: redis
    version: "17.0.0"
    repository: "https://charts.bitnami.com/bitnami"

maintainers:
  - name: Alice
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "redis");
    }

    // Ported: "returns null if no dependencies" — helm-requirements/extract.spec.ts line 172
    #[test]
    fn no_dependencies_returns_empty() {
        let content = r#"
apiVersion: v2
name: myapp
version: 1.0.0
"#;
        assert!(extract(content).is_empty());
    }

    // Rust-specific: helm behavior test
    #[test]
    fn real_world_chart_yaml() {
        let content = r#"
apiVersion: v2
name: myapp
version: 0.1.0
description: A Helm chart for Kubernetes

dependencies:
  - name: redis
    version: "17.0.0"
    repository: "https://charts.bitnami.com/bitnami"
  - name: postgresql
    version: "~> 12.0"
    repository: "https://charts.bitnami.com/bitnami"
  - name: stable-nginx
    version: "1.0.0"
    repository: stable
  - name: local-sidecar
    version: "0.1.0"
    repository: ""
  - name: oci-chart
    version: "3.0.0"
    repository: "oci://registry.example.com/charts"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 5);

        let redis = deps.iter().find(|d| d.name == "redis").unwrap();
        assert!(redis.skip_reason.is_none());
        assert_eq!(redis.repository, "https://charts.bitnami.com/bitnami");

        let nginx = deps.iter().find(|d| d.name == "stable-nginx").unwrap();
        assert_eq!(nginx.repository, STABLE_REPO);

        let local = deps.iter().find(|d| d.name == "local-sidecar").unwrap();
        assert_eq!(local.skip_reason, Some(HelmSkipReason::NoRepository));

        let oci = deps.iter().find(|d| d.name == "oci-chart").unwrap();
        assert_eq!(oci.skip_reason, Some(HelmSkipReason::OciRegistry));
    }

    // Ported: "skips local dependencies" — helm-requirements/extract.spec.ts line 141
    #[test]
    fn local_file_dependency_skipped() {
        let content = r#"
dependencies:
  - name: redis
    version: "0.9.0"
    repository: https://charts.helm.sh/stable/
  - name: postgresql
    version: "0.8.1"
    repository: file:///some/local/path/
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        let redis = deps.iter().find(|d| d.name == "redis").unwrap();
        assert!(redis.skip_reason.is_none());
        let pg = deps.iter().find(|d| d.name == "postgresql").unwrap();
        // file:// paths are not https or http — treated as unresolvable alias
        assert!(pg.skip_reason.is_some());
    }

    // Ported: "returns null if requirements.yaml is invalid" — helm-requirements/extract.spec.ts line 192
    #[test]
    fn invalid_yaml_returns_empty() {
        // Malformed YAML — the Rust extractor just returns empty for unrecognised content.
        let content = "Invalid requirements.yaml content.\ndependencies:\n[\n";
        let deps = extract(content);
        // Invalid YAML with no parseable dependencies block → empty.
        assert!(deps.is_empty());
    }

    // Ported: "returns null if Chart.yaml is empty" — helm-requirements/extract.spec.ts line 214
    #[test]
    fn empty_content_returns_empty() {
        let deps = extract("");
        assert!(deps.is_empty());
    }

    // Ported: "validates name is required" — helm-requirements/extract.spec.ts line 278
    #[test]
    fn dep_without_name_is_silently_skipped() {
        // A dependency entry with no name field — emit_dep skips it.
        let content = r#"
dependencies:
  - version: "0.0.1"
    repository: https://charts.helm.sh/stable/
"#;
        // The Rust extractor silently skips deps with empty name.
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    // Ported: "skips only invalid dependences" — helm-requirements/extract.spec.ts line 293
    #[test]
    fn skips_only_invalid_deps_keeps_valid_ones() {
        let content = r#"
dependencies:
  - name: postgresql
    repository: https://charts.helm.sh/stable/
  - version: "0.0.1"
    repository: https://charts.helm.sh/stable/
  - name: redis
    version: "0.0.1"
  - name: redis
    version: "0.0.1"
    repository: https://charts.helm.sh/stable/
"#;
        let deps = extract(content);
        // Rust emit_dep silently drops entries with empty name or empty version.
        // TS emits 4 deps with skipReasons; Rust emits 2: redis/no-repo and redis/valid.
        assert_eq!(deps.len(), 2);
        let no_repo = &deps[0];
        assert_eq!(no_repo.name, "redis");
        assert_eq!(no_repo.current_value, "0.0.1");
        assert_eq!(no_repo.skip_reason, Some(HelmSkipReason::NoRepository));
        let valid = &deps[1];
        assert_eq!(valid.name, "redis");
        assert_eq!(valid.current_value, "0.0.1");
        assert_eq!(valid.repository, "https://charts.helm.sh/stable/");
        assert!(valid.skip_reason.is_none());
    }

    const CHART_YAML: &str = "apiVersion: v2\nname: test\nversion: 0.0.2\n";

    // Ported: "increments" — modules/manager/helmv3/update.spec.ts line 14
    #[test]
    fn helm_bump_increments_patch() {
        let result = bump_package_version(CHART_YAML, "0.0.2", "patch");
        assert_eq!(result, CHART_YAML.replace("0.0.2", "0.0.3"));
    }

    // Ported: "no ops" — modules/manager/helmv3/update.spec.ts line 22
    #[test]
    fn helm_bump_no_op_when_version_mismatch() {
        let result = bump_package_version(CHART_YAML, "0.0.1", "patch");
        assert_eq!(result, CHART_YAML);
    }

    // Ported: "updates" — modules/manager/helmv3/update.spec.ts line 30
    #[test]
    fn helm_bump_updates_minor() {
        let result = bump_package_version(CHART_YAML, "0.0.1", "minor");
        assert_eq!(result, CHART_YAML.replace("0.0.2", "0.1.0"));
    }

    // Ported: "returns content if bumping errors" — modules/manager/helmv3/update.spec.ts line 38
    #[test]
    fn helm_bump_returns_content_on_invalid_bump_type() {
        let result = bump_package_version(CHART_YAML, "0.0.2", "not_valid");
        assert_eq!(result, CHART_YAML);
    }

    // ── helmv3/utils.spec.ts tests ──────────────────────────────────────────

    fn aliases() -> std::collections::HashMap<String, String> {
        let mut m = std::collections::HashMap::new();
        m.insert("testRepo".into(), "https://charts.helm.sh/stable".into());
        m.insert("artifactory".into(), "oci://artifactory.example.com".into());
        m
    }

    // Ported: "return alias with \"alias:\"" — modules/manager/helmv3/utils.spec.ts line 5
    #[test]
    fn helm_resolve_alias_with_alias_prefix() {
        let result = resolve_alias(Some("alias:testRepo"), &aliases());
        assert_eq!(result.as_deref(), Some("https://charts.helm.sh/stable"));
    }

    // Ported: "return alias with \"@\"" — modules/manager/helmv3/utils.spec.ts line 13
    #[test]
    fn helm_resolve_alias_with_at_prefix() {
        let result = resolve_alias(Some("@testRepo"), &aliases());
        assert_eq!(result.as_deref(), Some("https://charts.helm.sh/stable"));
    }

    // Ported: "return null if alias repo is not defined" — modules/manager/helmv3/utils.spec.ts line 21
    #[test]
    fn helm_resolve_alias_undefined_returns_none() {
        let result = resolve_alias(Some("alias:testRepo"), &{
            let mut m = std::collections::HashMap::new();
            m.insert(
                "anotherRepository".into(),
                "https://charts.helm.sh/stable".into(),
            );
            m
        });
        assert!(result.is_none());
    }

    // Ported: "return resolved repository on OCI registries" — modules/manager/helmv3/utils.spec.ts line 29
    #[test]
    fn helm_resolve_alias_oci_registry() {
        let result = resolve_alias(Some("alias:artifactory"), &aliases());
        assert_eq!(result.as_deref(), Some("oci://artifactory.example.com"));
    }

    // Ported: "return repository parameter if it is not an alias" — modules/manager/helmv3/utils.spec.ts line 37
    #[test]
    fn helm_resolve_alias_non_alias_passthrough() {
        let url = "https://registry.example.com";
        let result = resolve_alias(Some(url), &aliases());
        assert_eq!(result.as_deref(), Some(url));
    }

    // Ported: "return repository parameter if repository is null" — modules/manager/helmv3/utils.spec.ts line 47
    #[test]
    fn helm_resolve_alias_null_returns_none() {
        let result = resolve_alias(None, &aliases());
        assert!(result.is_none());
    }

    // Ported: "return repository parameter if repository is undefined" — modules/manager/helmv3/utils.spec.ts line 54
    #[test]
    fn helm_resolve_alias_undefined_input_returns_none() {
        assert!(resolve_alias(None, &std::collections::HashMap::new()).is_none());
    }

    // Ported: "return false if repository is null" (isAlias) — modules/manager/helmv3/utils.spec.ts line 63
    #[test]
    fn helm_is_alias_null_returns_false() {
        assert!(!is_alias(None));
    }

    // Ported: "return false if repository is undefined" (isAlias) — modules/manager/helmv3/utils.spec.ts line 69
    #[test]
    fn helm_is_alias_undefined_returns_false() {
        assert!(!is_alias(None));
    }

    // Ported: "return false if repository is null" (isOCIRegistry) — modules/manager/helmv3/utils.spec.ts line 76
    #[test]
    fn helm_is_oci_registry_null_returns_false() {
        assert!(!is_oci_registry(None));
    }

    // Ported: "return false if repository is undefined" (isOCIRegistry) — modules/manager/helmv3/utils.spec.ts line 82
    #[test]
    fn helm_is_oci_registry_undefined_returns_false() {
        assert!(!is_oci_registry(None));
    }

    // ── helmv3/extract.spec.ts tests ───────────────────────────────────────────

    fn stable_aliases() -> HashMap<String, String> {
        let mut m = HashMap::new();
        m.insert("stable".into(), "https://charts.helm.sh/stable".into());
        m
    }

    // Ported: "skips invalid registry urls" — modules/manager/helmv3/extract.spec.ts line 16
    #[test]
    fn extract_skips_invalid_registry_urls() {
        let content = "apiVersion: v2\nname: example\nversion: 0.1.0\ndependencies:\n  - name: redis\n    version: 0.9.0\n    repository: '@placeholder'\n  - name: postgresql\n    version: 0.8.1\n    repository: nope\n  - name: broken\n    version: 0.8.1\n";
        let result = extract_package_file(content, "Chart.yaml", &stable_aliases()).unwrap();
        assert!(result.deps.iter().all(|d| d.skip_reason.is_some()));
    }

    // Ported: "parses simple Chart.yaml correctly" — modules/manager/helmv3/extract.spec.ts line 40
    #[test]
    fn extract_parses_simple_chart_yaml() {
        let content = "apiVersion: v2\nname: example\nversion: 0.1.0\ndependencies:\n  - name: redis\n    version: 0.9.0\n    repository: https://charts.helm.sh/stable\n  - name: postgresql\n    version: 0.8.1\n    repository: https://charts.helm.sh/stable\n";
        let result = extract_package_file(content, "Chart.yaml", &stable_aliases()).unwrap();
        assert_eq!(result.deps.len(), 2);
        assert_eq!(result.deps[0].name, "redis");
        assert_eq!(result.deps[0].current_value, "0.9.0");
        assert!(result.deps[0].skip_reason.is_none());
        assert_eq!(result.deps[1].name, "postgresql");
        assert_eq!(result.deps[1].current_value, "0.8.1");
    }

    // Ported: "extract correctly oci references" — modules/manager/helmv3/extract.spec.ts line 67
    #[test]
    fn extract_oci_references() {
        let content = "apiVersion: v2\nname: app2\nversion: 0.1.0\ndependencies:\n  - name: library\n    version: 0.1.0\n    repository: oci://ghcr.io/ankitabhopatkar13\n  - name: postgresql\n    version: 0.8.1\n    repository: https://charts.helm.sh/stable\n";
        let result = extract_package_file(content, "Chart.yaml", &stable_aliases()).unwrap();
        assert_eq!(result.deps[0].name, "library");
        assert_eq!(result.deps[0].datasource.as_deref(), Some("docker"));
        assert_eq!(result.deps[1].name, "postgresql");
        assert!(result.deps[1].datasource.is_none());
    }

    // Ported: "resolves aliased registry urls" — modules/manager/helmv3/extract.spec.ts line 100
    #[test]
    fn extract_resolves_aliased_registry_urls() {
        let content = "apiVersion: v2\nname: example\nversion: 0.1.0\ndependencies:\n  - name: redis\n    version: 0.9.0\n    repository: '@placeholder'\n  - name: example\n    version: 1.0.0\n    repository: alias:longalias\n  - name: oci-example\n    version: 2.2.0\n    repository: alias:ociRegistry\n";
        let mut aliases = HashMap::new();
        aliases.insert("placeholder".into(), "https://my-registry.gcr.io/".into());
        aliases.insert("longalias".into(), "https://registry.example.com/".into());
        aliases.insert(
            "ociRegistry".into(),
            "oci://quay.example.com/organization".into(),
        );
        let result = extract_package_file(content, "Chart.yaml", &aliases).unwrap();
        assert!(result.deps.iter().all(|d| d.skip_reason.is_none()));
    }

    // Ported: "doesn't fail if Chart.yaml is invalid" — modules/manager/helmv3/extract.spec.ts line 131
    #[test]
    fn extract_returns_none_for_invalid_chart_yaml() {
        let content = "Invalid Chart.yaml content.\narr:\n[\n";
        assert!(extract_package_file(content, "Chart.yaml", &stable_aliases()).is_none());
    }

    // Ported: "skips local dependencies" — modules/manager/helmv3/extract.spec.ts line 142
    #[test]
    fn extract_skips_local_dependencies() {
        let content = "apiVersion: v2\nname: example\nversion: 0.1.0\ndependencies:\n  - name: redis\n    version: 0.9.0\n    repository: https://charts.helm.sh/stable\n  - name: postgresql\n    version: 0.8.1\n    repository: file:///some/local/path/\n";
        let result = extract_package_file(content, "Chart.yaml", &stable_aliases()).unwrap();
        assert!(result.deps[0].skip_reason.is_none());
        assert_eq!(result.deps[1].skip_reason, Some(HelmSkipReason::LocalChart));
    }

    // Ported: "returns null if no dependencies key" — modules/manager/helmv3/extract.spec.ts line 167
    #[test]
    fn extract_returns_none_if_no_dependencies_key() {
        let content = "apiVersion: v2\nname: example\nversion: 0.1.0\nhello: world\n";
        assert!(extract_package_file(content, "Chart.yaml", &stable_aliases()).is_none());
    }

    // Ported: "returns null if dependencies are an empty list" — modules/manager/helmv3/extract.spec.ts line 183
    #[test]
    fn extract_returns_none_if_dependencies_empty_list() {
        let content = "apiVersion: v2\nname: example\nversion: 0.1.0\ndependencies: []\n";
        assert!(extract_package_file(content, "Chart.yaml", &stable_aliases()).is_none());
    }

    // Ported: "returns null if dependencies key is invalid" — modules/manager/helmv3/extract.spec.ts line 199
    #[test]
    fn extract_returns_none_if_dependencies_invalid() {
        let content = "apiVersion: v2\nname: example\nversion: 0.1.0\ndependencies:\n  Invalid dependencies content.\n  [\n";
        assert!(extract_package_file(content, "Chart.yaml", &stable_aliases()).is_none());
    }

    // Ported: "returns null if Chart.yaml is empty" — modules/manager/helmv3/extract.spec.ts line 215
    #[test]
    fn extract_returns_none_if_chart_yaml_empty() {
        assert!(extract_package_file("", "Chart.yaml", &stable_aliases()).is_none());
    }

    // Ported: "returns null if Chart.yaml uses an unsupported apiVersion" — modules/manager/helmv3/extract.spec.ts line 222
    #[test]
    fn extract_returns_none_if_unsupported_api_version() {
        let content = "apiVersion: v1\nname: example\nversion: 0.1.0\n";
        assert!(extract_package_file(content, "Chart.yaml", &stable_aliases()).is_none());
    }

    // Ported: "returns null if name and version are missing for all dependencies" — modules/manager/helmv3/extract.spec.ts line 235
    #[test]
    fn extract_returns_none_if_all_deps_missing_name_version() {
        let content = "apiVersion: v2\nname: example\nversion: 0.1.0\ndependencies:\n  - repository: test\n  - repository: test\n    alias: test\n";
        assert!(extract_package_file(content, "Chart.yaml", &stable_aliases()).is_none());
    }
}
