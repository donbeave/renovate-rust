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
}
