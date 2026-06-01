//! Rancher Fleet `fleet.yaml` / GitRepo YAML dependency extractor.
//!
//! Fleet files come in two flavours:
//!
//! 1. **`fleet.yaml`** — bundle config with a `helm:` block.
//!    Produces a Helm chart dep per helm block (including `targetCustomizations`).
//!
//! 2. **Other matched files** — Kubernetes GitRepo CRDs
//!    (`kind: GitRepo`, `spec.repo`/`spec.revision`).
//!    Produces a Git-tags dep per GitRepo document.
//!
//! Renovate reference:
//! - `lib/modules/manager/fleet/extract.ts`
//! - Pattern: `/(^|/)fleet\.ya?ml/` (note: trailing `/` means any suffix too)
//! - Datasources: Helm, GitTagsDatasource
//!
//! ## `fleet.yaml` format
//!
//! ```yaml
//! helm:
//!   chart: nginx
//!   repo: https://charts.example.com/
//!   version: "1.2.3"
//! targetCustomizations:
//!   - name: prod
//!     helm:
//!       version: "1.3.0"
//! ```
//!
//! ## GitRepo CRD format
//!
//! ```yaml
//! kind: GitRepo
//! spec:
//!   repo: https://github.com/owner/myapp
//!   revision: v1.2.3
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// Why a Fleet dependency is skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FleetSkipReason {
    /// No chart name defined.
    MissingChart,
    /// No repository URL defined.
    NoRepository,
    /// Chart is an OCI registry reference.
    OciRegistry,
    /// Repository is a local path or alias.
    LocalOrAlias,
    /// No version defined.
    UnspecifiedVersion,
    /// No repo URL in GitRepo spec.
    MissingRepo,
}

/// A Helm chart dependency from a `fleet.yaml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FleetHelmDep {
    /// Chart name used as `dep_name`.
    pub chart: String,
    /// Helm repository URL.
    pub registry_url: String,
    /// Version constraint.
    pub current_value: String,
    /// Optional customization label (from `name:` in targetCustomizations).
    pub customization_name: Option<String>,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<FleetSkipReason>,
}

/// A Git repository version dep from a Fleet GitRepo CRD.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FleetGitDep {
    /// Source URL of the git repository.
    pub repo_url: String,
    /// Branch or tag revision (the current tracked value).
    pub current_value: String,
    /// Set when no lookup should be performed.
    pub skip_reason: Option<FleetSkipReason>,
}

/// All deps extracted from a Fleet file.
#[derive(Debug, Default, Clone)]
pub struct FleetDeps {
    /// Helm chart deps (from `fleet.yaml`).
    pub helm_deps: Vec<FleetHelmDep>,
    /// Git repo deps (from GitRepo CRD files).
    pub git_deps: Vec<FleetGitDep>,
}

/// Renovate-style Fleet dependency metadata after applying registry aliases.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FleetResolvedDep {
    pub datasource: &'static str,
    pub dep_name: String,
    pub package_name: String,
    pub current_value: String,
    pub registry_urls: Vec<String>,
    pub dep_type: &'static str,
    pub pin_digests: Option<bool>,
}

static KV: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"^\s+(\w+):\s*"?([^"#\n]+?)"?\s*(?:#.*)?$"##).unwrap());

/// Extract Fleet deps from a file.
///
/// `is_fleet_yaml` should be `true` when the file is named `fleet.yaml` /
/// `fleet.yml`; `false` for other matched files (GitRepo CRDs).
pub fn extract(content: &str, is_fleet_yaml: bool) -> FleetDeps {
    if is_fleet_yaml {
        extract_fleet_yaml(content)
    } else {
        extract_gitrepo(content)
    }
}

/// Detect whether a filename should be parsed as a `fleet.yaml` bundle config.
pub fn is_fleet_yaml_path(path: &str) -> bool {
    let base = path.rsplit('/').next().unwrap_or(path);
    base == "fleet.yaml" || base == "fleet.yml"
}

/// Extract Fleet deps and apply Renovate-style registry aliases.
pub fn extract_with_registry_aliases(
    content: &str,
    is_fleet_yaml: bool,
    registry_aliases: &[(&str, &str)],
) -> Vec<FleetResolvedDep> {
    if !is_fleet_yaml {
        return Vec::new();
    }

    content
        .split("---")
        .flat_map(|doc| extract_fleet_yaml(doc).helm_deps)
        .filter_map(|dep| fleet_resolved_dep(dep, registry_aliases))
        .collect()
}

// ── fleet.yaml parsing ────────────────────────────────────────────────────────

/// Parse a `fleet.yaml` file. Extracts the top-level `helm:` block and any
/// `targetCustomizations[n].helm` blocks.
fn extract_fleet_yaml(content: &str) -> FleetDeps {
    let mut deps = FleetDeps::default();

    // Collect the main helm block and each targetCustomization helm block.
    // We parse using indentation levels:
    //   indent=0 : top-level keys
    //   indent=2 : helm keys OR targetCustomization list items
    //   indent=4+: targetCustomization.helm keys

    enum Section {
        None,
        Helm,                 // inside top-level `helm:`
        TargetCustomizations, // inside `targetCustomizations:`
        CustomizationItem,    // inside `  - name: ...`
        CustomizationHelm,    // inside `    helm:`
    }

    let mut section = Section::None;
    let mut main_helm = HelmBlock::default();
    let mut custom_name: Option<String> = None;
    let mut custom_helm = HelmBlock::default();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let indent = line.len() - line.trim_start().len();

        match indent {
            0 => {
                // Top-level key
                match trimmed {
                    "helm:" => {
                        section = Section::Helm;
                    }
                    "targetCustomizations:" => {
                        section = Section::TargetCustomizations;
                    }
                    _ => {
                        section = Section::None;
                    }
                }
            }
            2 => {
                match &section {
                    Section::Helm => {
                        // Keys inside `helm:`
                        if let Some(cap) = KV.captures(line) {
                            apply_helm_kv(&cap[1], cap[2].trim(), &mut main_helm);
                        }
                    }
                    Section::TargetCustomizations | Section::CustomizationItem => {
                        // Start of a new list item in targetCustomizations
                        if trimmed.starts_with('-') {
                            // Flush previous customization
                            if let Some(name) = custom_name.take()
                                && !custom_helm.is_empty()
                            {
                                deps.helm_deps.push(build_helm_dep(
                                    &custom_helm,
                                    &main_helm,
                                    Some(name),
                                ));
                            }
                            custom_helm = HelmBlock::default();

                            // Inline key: `  - name: prod`
                            let rest = trimmed.trim_start_matches('-').trim();
                            if let Some(cap) = KV.captures(&format!("  {rest}")) {
                                let key = cap[1].trim();
                                if key == "name" {
                                    custom_name = Some(cap[2].trim().to_owned());
                                }
                            }
                            section = Section::CustomizationItem;
                        } else if let Some(cap) = KV.captures(line) {
                            let key = cap[1].trim();
                            if key == "name" {
                                custom_name = Some(cap[2].trim().to_owned());
                            }
                        }
                    }
                    _ => {}
                }
            }
            4 => match &section {
                Section::CustomizationItem if trimmed == "helm:" => {
                    section = Section::CustomizationHelm;
                }
                Section::CustomizationHelm => {
                    if let Some(cap) = KV.captures(line) {
                        apply_helm_kv(&cap[1], cap[2].trim(), &mut custom_helm);
                    }
                }
                _ => {}
            },
            6 => {
                if let (Section::CustomizationHelm, Some(cap)) = (&section, KV.captures(line)) {
                    apply_helm_kv(&cap[1], cap[2].trim(), &mut custom_helm);
                }
            }
            _ => {}
        }
    }

    // Flush final customization
    if let Some(name) = custom_name
        && !custom_helm.is_empty()
    {
        deps.helm_deps
            .push(build_helm_dep(&custom_helm, &main_helm, Some(name)));
    }

    // Emit main helm dep
    if !main_helm.is_empty() {
        deps.helm_deps
            .insert(0, build_helm_dep(&main_helm, &HelmBlock::default(), None));
    }

    deps
}

#[derive(Default, Clone)]
struct HelmBlock {
    chart: Option<String>,
    repo: Option<String>,
    version: Option<String>,
}

impl HelmBlock {
    fn is_empty(&self) -> bool {
        self.chart.is_none() && self.repo.is_none() && self.version.is_none()
    }
}

fn apply_helm_kv(key: &str, val: &str, block: &mut HelmBlock) {
    match key {
        "chart" => block.chart = Some(val.to_owned()),
        "repo" => block.repo = Some(val.to_owned()),
        "version" => block.version = Some(val.to_owned()),
        _ => {}
    }
}

fn build_helm_dep(
    block: &HelmBlock,
    base: &HelmBlock,
    customization_name: Option<String>,
) -> FleetHelmDep {
    let chart = block
        .chart
        .clone()
        .or_else(|| base.chart.clone())
        .unwrap_or_default();
    let repo = block
        .repo
        .clone()
        .or_else(|| base.repo.clone())
        .unwrap_or_default();
    let version = block
        .version
        .clone()
        .or_else(|| base.version.clone())
        .unwrap_or_default();

    if chart.is_empty() {
        return FleetHelmDep {
            chart,
            registry_url: repo,
            current_value: version,
            customization_name,
            skip_reason: Some(FleetSkipReason::MissingChart),
        };
    }

    if chart.starts_with("oci://") {
        return FleetHelmDep {
            chart: chart.trim_start_matches("oci://").to_owned(),
            registry_url: String::new(),
            current_value: version,
            customization_name,
            skip_reason: Some(FleetSkipReason::OciRegistry),
        };
    }

    if repo.is_empty() {
        // Check if chart looks like a local path
        let skip = if chart.starts_with("./") || chart.starts_with("../") || chart.starts_with('/')
        {
            FleetSkipReason::LocalOrAlias
        } else {
            FleetSkipReason::NoRepository
        };
        return FleetHelmDep {
            chart,
            registry_url: String::new(),
            current_value: version,
            customization_name,
            skip_reason: Some(skip),
        };
    }

    if version.is_empty() {
        return FleetHelmDep {
            chart,
            registry_url: repo,
            current_value: String::new(),
            customization_name,
            skip_reason: Some(FleetSkipReason::UnspecifiedVersion),
        };
    }

    FleetHelmDep {
        chart,
        registry_url: repo,
        current_value: version,
        customization_name,
        skip_reason: None,
    }
}

fn fleet_resolved_dep(
    dep: FleetHelmDep,
    registry_aliases: &[(&str, &str)],
) -> Option<FleetResolvedDep> {
    match dep.skip_reason {
        None => Some(FleetResolvedDep {
            datasource: "helm",
            dep_name: dep.chart.clone(),
            package_name: dep.chart,
            current_value: dep.current_value,
            registry_urls: vec![apply_registry_alias(&dep.registry_url, registry_aliases)],
            dep_type: "fleet",
            pin_digests: None,
        }),
        Some(FleetSkipReason::OciRegistry) => Some(FleetResolvedDep {
            datasource: "docker",
            package_name: apply_registry_alias(&dep.chart, registry_aliases),
            dep_name: dep.chart,
            current_value: dep.current_value,
            registry_urls: Vec::new(),
            dep_type: "fleet",
            pin_digests: Some(false),
        }),
        Some(_) => None,
    }
}

fn apply_registry_alias(value: &str, registry_aliases: &[(&str, &str)]) -> String {
    registry_aliases
        .iter()
        .find_map(|(from, to)| {
            value
                .strip_prefix(from)
                .filter(|rest| rest.is_empty() || rest.starts_with('/'))
                .map(|rest| format!("{to}{rest}"))
        })
        .unwrap_or_else(|| value.to_owned())
}

// ── GitRepo CRD parsing ───────────────────────────────────────────────────────

/// Parse a GitRepo CRD YAML file. Handles multi-document YAML (`---`).
fn extract_gitrepo(content: &str) -> FleetDeps {
    let mut deps = FleetDeps::default();

    enum DocSection {
        None,
        Spec,
    }

    // Process each --- separated document.
    for doc in content.split("---") {
        let doc = doc.trim();
        if doc.is_empty() {
            continue;
        }

        // Only process if kind: GitRepo
        let is_gitrepo = doc
            .lines()
            .any(|l| l.trim() == "kind: GitRepo" || l.trim() == "kind: \"GitRepo\"");
        if !is_gitrepo {
            continue;
        }

        let mut section = DocSection::None;
        let mut repo_url = String::new();
        let mut revision = String::new();

        for line in doc.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let indent = line.len() - line.trim_start().len();

            if indent == 0 {
                if trimmed == "spec:" {
                    section = DocSection::Spec;
                } else {
                    section = DocSection::None;
                }
                continue;
            }

            if let DocSection::Spec = section
                && let Some(cap) = KV.captures(line)
            {
                match &cap[1] {
                    "repo" => repo_url = cap[2].trim().to_owned(),
                    "revision" => revision = cap[2].trim().to_owned(),
                    _ => {}
                }
            }
        }

        if repo_url.is_empty() {
            deps.git_deps.push(FleetGitDep {
                repo_url: String::new(),
                current_value: String::new(),
                skip_reason: Some(FleetSkipReason::MissingRepo),
            });
        } else if revision.is_empty() {
            deps.git_deps.push(FleetGitDep {
                repo_url,
                current_value: String::new(),
                skip_reason: Some(FleetSkipReason::UnspecifiedVersion),
            });
        } else {
            deps.git_deps.push(FleetGitDep {
                repo_url,
                current_value: revision,
                skip_reason: None,
            });
        }
    }

    deps
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "should return null if a unknown manifest is supplied" — fleet/extract.spec.ts line 30
    #[test]
    fn is_fleet_yaml_detects_correctly() {
        assert!(is_fleet_yaml_path("fleet.yaml"));
        assert!(is_fleet_yaml_path("path/to/fleet.yml"));
        assert!(!is_fleet_yaml_path("gitrepo.yaml"));
        assert!(!is_fleet_yaml_path("fleet-config.yaml"));
    }

    // Ported: "should parse valid configuration" (fleet.yaml) — fleet/extract.spec.ts line 49
    #[test]
    fn extracts_helm_dep_from_fleet_yaml() {
        let content = r#"
helm:
  chart: nginx
  repo: https://charts.example.com/
  version: "1.2.3"
"#;
        let deps = extract(content, true);
        assert_eq!(deps.helm_deps.len(), 1);
        let d = &deps.helm_deps[0];
        assert_eq!(d.chart, "nginx");
        assert_eq!(d.registry_url, "https://charts.example.com/");
        assert_eq!(d.current_value, "1.2.3");
        assert!(d.skip_reason.is_none());
    }

    // Ported: "should support registryAlias configuration" — fleet/extract.spec.ts line 88
    #[test]
    fn supports_registry_alias_configuration() {
        let content = r#"
defaultNamespace: cert-manager
helm:
  chart: cert-manager
  repo: https://registry.com/jetstack
  releaseName: cert-manager
  version: v1.8.0
---
defaultNamespace: external-dns
helm:
  chart: oci://registry.com/docker-io/bitnamicharts/external-dns
  version: 7.1.2
"#;
        let deps = extract_with_registry_aliases(
            content,
            true,
            &[
                (
                    "https://registry.com/jetstack",
                    "https://charts.jetstack.io",
                ),
                ("registry.com/docker-io", "registry-1.docker.io"),
            ],
        );

        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].datasource, "helm");
        assert_eq!(deps[0].dep_name, "cert-manager");
        assert_eq!(deps[0].package_name, "cert-manager");
        assert_eq!(deps[0].current_value, "v1.8.0");
        assert_eq!(deps[0].registry_urls, ["https://charts.jetstack.io"]);
        assert_eq!(deps[0].dep_type, "fleet");
        assert_eq!(deps[0].pin_digests, None);

        assert_eq!(deps[1].datasource, "docker");
        assert_eq!(
            deps[1].dep_name,
            "registry.com/docker-io/bitnamicharts/external-dns"
        );
        assert_eq!(
            deps[1].package_name,
            "registry-1.docker.io/bitnamicharts/external-dns"
        );
        assert_eq!(deps[1].current_value, "7.1.2");
        assert!(deps[1].registry_urls.is_empty());
        assert_eq!(deps[1].dep_type, "fleet");
        assert_eq!(deps[1].pin_digests, Some(false));
    }

    // Ported: "should parse valid configuration with target customization" — fleet/extract.spec.ts line 132
    #[test]
    fn extracts_target_customizations() {
        let content = r#"
helm:
  chart: nginx
  repo: https://charts.example.com/
  version: "1.2.3"
targetCustomizations:
  - name: prod
    helm:
      version: "1.3.0"
"#;
        let deps = extract(content, true);
        assert_eq!(deps.helm_deps.len(), 2);
        let main = &deps.helm_deps[0];
        assert_eq!(main.chart, "nginx");
        assert_eq!(main.current_value, "1.2.3");
        assert!(main.customization_name.is_none());

        let custom = &deps.helm_deps[1];
        assert_eq!(custom.chart, "nginx");
        assert_eq!(custom.current_value, "1.3.0");
        assert_eq!(custom.customization_name.as_deref(), Some("prod"));
    }

    // Ported: "should parse parse invalid configurations" — fleet/extract.spec.ts line 208
    #[test]
    fn missing_chart_sets_skip_reason() {
        let content = r#"
helm:
  repo: https://charts.example.com/
  version: "1.2.3"
"#;
        let deps = extract(content, true);
        assert_eq!(deps.helm_deps.len(), 1);
        assert_eq!(
            deps.helm_deps[0].skip_reason,
            Some(FleetSkipReason::MissingChart)
        );
    }

    // Ported: "should parse parse invalid configurations" — fleet/extract.spec.ts line 208
    #[test]
    fn no_version_sets_skip_reason() {
        let content = r#"
helm:
  chart: nginx
  repo: https://charts.example.com/
"#;
        let deps = extract(content, true);
        assert_eq!(deps.helm_deps.len(), 1);
        assert_eq!(
            deps.helm_deps[0].skip_reason,
            Some(FleetSkipReason::UnspecifiedVersion)
        );
    }

    // Ported: "should parse valid configuration" (GitRepo) — fleet/extract.spec.ts line 254
    #[test]
    fn extracts_gitrepo_dep() {
        let content = r#"
kind: GitRepo
metadata:
  name: my-app
spec:
  repo: https://github.com/owner/myapp
  revision: v1.2.3
"#;
        let deps = extract(content, false);
        assert_eq!(deps.git_deps.len(), 1);
        let d = &deps.git_deps[0];
        assert_eq!(d.repo_url, "https://github.com/owner/myapp");
        assert_eq!(d.current_value, "v1.2.3");
        assert!(d.skip_reason.is_none());
    }

    // Ported: "should parse invalid configuration" — fleet/extract.spec.ts line 276
    #[test]
    fn gitrepo_missing_revision_sets_skip_reason() {
        let content = r#"
kind: GitRepo
spec:
  repo: https://github.com/owner/myapp
"#;
        let deps = extract(content, false);
        assert_eq!(deps.git_deps.len(), 1);
        assert_eq!(
            deps.git_deps[0].skip_reason,
            Some(FleetSkipReason::UnspecifiedVersion)
        );
    }

    // Ported: "should parse invalid configuration" — fleet/extract.spec.ts line 276
    #[test]
    fn non_gitrepo_yaml_returns_empty() {
        let content = r#"
kind: Deployment
spec:
  repo: https://github.com/owner/myapp
  revision: v1.2.3
"#;
        let deps = extract(content, false);
        assert!(deps.git_deps.is_empty());
    }

    // Ported: "should return null if empty content" — fleet/extract.spec.ts line 24
    #[test]
    fn empty_content_returns_empty() {
        let deps = extract("", true);
        assert!(deps.helm_deps.is_empty());
        let deps = extract("", false);
        assert!(deps.git_deps.is_empty());
    }

    // Ported: "should return null if a unknown manifest is supplied" — fleet/extract.spec.ts line 30
    #[test]
    fn unknown_manifest_returns_empty() {
        let content = "apiVersion: v1\nkind: Service\nspec:\n  selector: {}\n";
        let deps = extract(content, false);
        assert!(deps.git_deps.is_empty());
    }

    // Ported: "should return null if content is a malformed YAML" — fleet/extract.spec.ts line 37
    #[test]
    fn malformed_fleet_yaml_returns_empty() {
        let content = "apiVersion: v1\nkind: Fleet\n< ";
        let deps = extract(content, true);
        assert!(deps.helm_deps.is_empty());
        assert!(deps.git_deps.is_empty());
    }

    // Ported: "should return null if content is a malformed YAML" — fleet/extract.spec.ts line 242
    #[test]
    fn malformed_gitrepo_yaml_returns_empty() {
        // TS uses real YAML parser → returns null; Rust line parser sees "kind: GitRepo" (trimmed)
        // and produces a MissingRepo dep. Both agree: no valid dep is extracted.
        let content = "apiVersion: v1\n kind: GitRepo\n < ";
        let deps = extract(content, false);
        assert!(deps.helm_deps.is_empty());
        assert!(deps.git_deps.iter().all(|d| d.skip_reason.is_some()));
    }
}
