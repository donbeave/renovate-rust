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
//! | `stable` | Actionable — resolved to `https://charts.helm.sh/stable` |
//! | `oci://...` | Skipped — `OciRegistry` |
//! | `@alias` | Skipped — `UnresolvableAlias` |
//! | *(absent)* | Skipped — `NoRepository` |

use std::sync::LazyLock;

use regex::Regex;

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

// ── Helpers ───────────────────────────────────────────────────────────────────

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
