//! Sveltos `ClusterProfile`/`Profile` Helm chart extractor.
//!
//! Extracts Helm chart references from Sveltos Kubernetes GitOps manifests.
//!
//! Renovate reference:
//! - `lib/modules/manager/sveltos/extract.ts`
//! - Default patterns: `[]` (user-configured). We add `sveltos/` convention.
//! - Datasources: `helm`
//!
//! ## File format
//!
//! ```yaml
//! apiVersion: config.projectsveltos.io/v1beta1
//! kind: ClusterProfile
//! spec:
//!   helmCharts:
//!   - repositoryURL: https://charts.helm.sh/stable
//!     repositoryName: stable
//!     chartName: redis
//!     chartVersion: 10.5.7
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// A single Sveltos Helm chart dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SveltosDep {
    pub chart_name: String,
    pub current_value: String,
    pub registry_url: String,
    pub dep_type: String,
}

/// Sveltos dependency metadata after resolving datasource-specific fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SveltosResolvedDep {
    pub dep_name: String,
    pub package_name: String,
    pub current_value: String,
    pub datasource: &'static str,
    pub dep_type: String,
    pub registry_urls: Vec<String>,
}

// ── Regexes ───────────────────────────────────────────────────────────────────

static SVELTOS_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"apiVersion:\s*["']?(config|lib)\.projectsveltos\.io/"#).unwrap()
});

static HELM_CHARTS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s+helmCharts:\s*$").unwrap());

static KIND_VALUE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\s*kind:\s*["']?([^"'\s]+)"#).unwrap());

static KV_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\s+(\w+):\s+(?:"([^"]+)"|'([^']+)'|(\S+))"#).unwrap());

fn kv_val<'a>(cap: &'a regex::Captures) -> &'a str {
    cap.get(2)
        .or_else(|| cap.get(3))
        .or_else(|| cap.get(4))
        .map(|m| m.as_str())
        .unwrap_or("")
}

fn leading_spaces(line: &str) -> usize {
    line.len() - line.trim_start_matches(' ').len()
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Extract Sveltos Helm chart deps from a manifest file.
pub fn extract(content: &str) -> Vec<SveltosDep> {
    if !SVELTOS_RE.is_match(content) {
        return Vec::new();
    }

    let mut deps = Vec::new();

    for doc in content.split("\n---") {
        if !SVELTOS_RE.is_match(doc) {
            continue;
        }
        deps.extend(extract_from_doc(doc));
    }

    deps
}

/// Extract Sveltos deps and apply Renovate-style registry aliases.
pub fn extract_with_registry_aliases(
    content: &str,
    registry_aliases: &[(&str, &str)],
) -> Vec<SveltosResolvedDep> {
    extract(content)
        .into_iter()
        .map(|dep| sveltos_resolved_dep(dep, registry_aliases))
        .collect()
}

fn sveltos_resolved_dep(dep: SveltosDep, registry_aliases: &[(&str, &str)]) -> SveltosResolvedDep {
    if let Some(package_name) = dep.chart_name.strip_prefix("oci://") {
        let package_name = apply_registry_alias(package_name, registry_aliases);
        return SveltosResolvedDep {
            dep_name: dep.chart_name,
            package_name,
            current_value: dep.current_value,
            datasource: "docker",
            dep_type: dep.dep_type,
            registry_urls: Vec::new(),
        };
    }

    SveltosResolvedDep {
        package_name: dep
            .chart_name
            .rsplit('/')
            .next()
            .unwrap_or(&dep.chart_name)
            .to_owned(),
        dep_name: dep.chart_name,
        current_value: dep.current_value,
        datasource: "helm",
        dep_type: dep.dep_type,
        registry_urls: vec![dep.registry_url],
    }
}

fn apply_registry_alias(package_name: &str, registry_aliases: &[(&str, &str)]) -> String {
    let Some((registry, rest)) = package_name.split_once('/') else {
        return package_name.to_owned();
    };
    registry_aliases
        .iter()
        .find_map(|(from, to)| {
            if *from == registry {
                Some(format!("{to}/{rest}"))
            } else {
                None
            }
        })
        .unwrap_or_else(|| package_name.to_owned())
}

fn extract_from_doc(doc: &str) -> Vec<SveltosDep> {
    let mut deps = Vec::new();
    let mut in_helm_charts = false;
    let mut repo_url: Option<String> = None;
    let mut chart_name: Option<String> = None;
    let mut chart_version: Option<String> = None;
    let dep_type = doc
        .lines()
        .find_map(|line| KIND_VALUE_RE.captures(line).map(|cap| cap[1].to_owned()))
        .unwrap_or_default();

    for line in doc.lines() {
        let stripped = match line.find(" #") {
            Some(p) => &line[..p],
            None => line,
        };
        if stripped.trim().is_empty() {
            continue;
        }

        let indent = leading_spaces(stripped);

        if HELM_CHARTS_RE.is_match(stripped) {
            flush_chart(
                &mut repo_url,
                &mut chart_name,
                &mut chart_version,
                &dep_type,
                &mut deps,
            );
            in_helm_charts = true;
            continue;
        }

        if in_helm_charts {
            // Exit if we return to top-level indent without being a list item.
            if indent <= 2 && !stripped.trim_start().starts_with('-') {
                flush_chart(
                    &mut repo_url,
                    &mut chart_name,
                    &mut chart_version,
                    &dep_type,
                    &mut deps,
                );
                in_helm_charts = false;
                continue;
            }

            let trimmed = stripped.trim_start();

            // New list item — flush previous chart.
            if let Some(rest) = trimmed.strip_prefix("- ") {
                flush_chart(
                    &mut repo_url,
                    &mut chart_name,
                    &mut chart_version,
                    &dep_type,
                    &mut deps,
                );
                // Parse inline KV if present.
                if let Some(cap) = KV_RE.captures(&format!("    {rest}")) {
                    set_field(
                        &cap[1],
                        kv_val(&cap),
                        &mut repo_url,
                        &mut chart_name,
                        &mut chart_version,
                    );
                }
                continue;
            }

            if let Some(cap) = KV_RE.captures(stripped) {
                set_field(
                    &cap[1],
                    kv_val(&cap),
                    &mut repo_url,
                    &mut chart_name,
                    &mut chart_version,
                );
            }
        }
    }

    flush_chart(
        &mut repo_url,
        &mut chart_name,
        &mut chart_version,
        &dep_type,
        &mut deps,
    );
    deps
}

fn set_field(
    key: &str,
    val: &str,
    repo_url: &mut Option<String>,
    chart_name: &mut Option<String>,
    chart_version: &mut Option<String>,
) {
    match key {
        "repositoryURL" => *repo_url = Some(val.to_owned()),
        "chartName" => *chart_name = Some(val.to_owned()),
        "chartVersion" => *chart_version = Some(val.to_owned()),
        _ => {}
    }
}

fn flush_chart(
    repo_url: &mut Option<String>,
    chart_name: &mut Option<String>,
    chart_version: &mut Option<String>,
    dep_type: &str,
    deps: &mut Vec<SveltosDep>,
) {
    if let (Some(url), Some(name), Some(ver)) =
        (repo_url.take(), chart_name.take(), chart_version.take())
    {
        if !url.is_empty() && !name.is_empty() && !ver.is_empty() {
            deps.push(SveltosDep {
                chart_name: name,
                current_value: ver,
                registry_url: url,
                dep_type: dep_type.to_owned(),
            });
        }
    } else {
        repo_url.take();
        chart_name.take();
        chart_version.take();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "supports clusterprofiles" — sveltos/extract.spec.ts line 444
    #[test]
    fn extracts_helm_chart() {
        let content = r#"
apiVersion: config.projectsveltos.io/v1beta1
kind: ClusterProfile
spec:
  helmCharts:
  - repositoryURL: https://charts.helm.sh/stable
    repositoryName: stable
    chartName: redis
    chartVersion: 10.5.7
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].chart_name, "redis");
        assert_eq!(deps[0].current_value, "10.5.7");
        assert_eq!(deps[0].registry_url, "https://charts.helm.sh/stable");
    }

    // Ported: "supports clusterprofiles" — sveltos/extract.spec.ts line 444
    #[test]
    fn extracts_multiple_charts() {
        let content = r#"
apiVersion: config.projectsveltos.io/v1beta1
kind: ClusterProfile
spec:
  helmCharts:
  - repositoryURL: https://charts.helm.sh/stable
    repositoryName: stable
    chartName: redis
    chartVersion: 10.5.7
  - repositoryURL: https://charts.bitnami.com/bitnami
    repositoryName: bitnami
    chartName: postgresql
    chartVersion: 12.2.1
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
    }

    // Ported: "considers registryAliases" — sveltos/extract.spec.ts line 495
    #[test]
    fn considers_registry_aliases_for_oci_charts() {
        let content = r#"---
apiVersion: config.projectsveltos.io/v1beta1
kind: ClusterProfile
metadata:
  name: vault
spec:
  syncMode: Continuous
  helmCharts:
  - repositoryURL:    oci://registry-1.docker.io/bitnamicharts/vault
    repositoryName:   oci-vault
    chartName:        oci://registry-1.docker.io/bitnamicharts/vault
    chartVersion:     0.7.2
"#;
        let deps = extract_with_registry_aliases(
            content,
            &[("registry-1.docker.io", "docker.proxy.test/some/path")],
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "0.7.2");
        assert_eq!(
            deps[0].dep_name,
            "oci://registry-1.docker.io/bitnamicharts/vault"
        );
        assert_eq!(
            deps[0].package_name,
            "docker.proxy.test/some/path/bitnamicharts/vault"
        );
        assert_eq!(deps[0].datasource, "docker");
        assert_eq!(deps[0].dep_type, "ClusterProfile");
        assert!(deps[0].registry_urls.is_empty());
    }

    // Ported: "return null for Kubernetes manifest" — sveltos/extract.spec.ts line 308
    #[test]
    fn skips_non_sveltos_files() {
        assert!(extract("apiVersion: v1\nkind: ConfigMap\n").is_empty());
    }

    // Ported: "returns an empty array when parsing fails" — sveltos/extract.spec.ts line 278
    #[test]
    fn extract_definition_invalid_input_returns_empty() {
        // TypeScript: extractDefinition({}) returns [] — empty/invalid doc has no deps
        assert!(extract("{}").is_empty());
    }

    // Ported: "returns null for empty" — sveltos/extract.spec.ts line 298
    #[test]
    fn empty_content_returns_empty() {
        assert!(extract("").is_empty());
        assert!(extract("nothing here").is_empty());
    }

    // Ported: "return null if deps array would be empty" — sveltos/extract.spec.ts line 313
    #[test]
    fn malformed_no_charts_returns_empty() {
        let content = r#"apiVersion: config.projectsveltos.io/v1beta1
kind: ClusterProfile
metadata:
  name: test
spec:
  clusterSelector:
    matchLabels:
      env: production
"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "return result for double quoted projectsveltos.io apiVersion reference" — sveltos/extract.spec.ts line 332
    #[test]
    fn double_quoted_api_version_extracted() {
        let content = r#"apiVersion: "config.projectsveltos.io/v1beta1"
kind: ClusterProfile
metadata:
  name: prometheus
spec:
  helmCharts:
  - repositoryURL: https://prometheus-community.github.io/helm-charts
    repositoryName: prometheus-community
    chartName: prometheus-community/prometheus
    chartVersion: "23.4.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].chart_name, "prometheus-community/prometheus");
        assert_eq!(deps[0].current_value, "23.4.0");
    }

    // Ported: "return result for single quoted projectsveltos.io apiVersion reference" — sveltos/extract.spec.ts line 364
    #[test]
    fn single_quoted_api_version_extracted() {
        let content = r#"apiVersion: 'config.projectsveltos.io/v1beta1'
kind: ClusterProfile
metadata:
  name: prometheus
spec:
  helmCharts:
  - repositoryURL: https://prometheus-community.github.io/helm-charts
    repositoryName: prometheus-community
    chartName: prometheus-community/prometheus
    chartVersion: "23.4.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].chart_name, "prometheus-community/prometheus");
    }

    // Ported: "returns null if extractDefinition returns an empty array" — sveltos/extract.spec.ts line 284
    #[test]
    fn clusterprofile_with_no_helm_charts_returns_empty() {
        let content = r#"apiVersion: "config.projectsveltos.io/v1beta1"
kind: ClusterProfile
metadata:
  name: empty-profile
"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "returns null for invalid" — sveltos/extract.spec.ts line 302
    #[test]
    fn malformed_profiles_all_empty_charts_returns_empty() {
        let content = r#"---
apiVersion: lib.projectsveltos.io/v1beta1
kind: EventTrigger
spec:
  helmCharts: []
---
apiVersion: config.projectsveltos.io/v1beta1
kind: ClusterProfile
spec:
  helmCharts: []
"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "return null if YAML is invalid" — sveltos/extract.spec.ts line 318
    #[test]
    fn invalid_yaml_with_no_valid_helm_charts_returns_empty() {
        let content = r#"----
apiVersion: "config.projectsveltos.io/v1beta1"
   kind ClusterProfile
metadata:
name: prometheus
"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "supports profiles" — sveltos/extract.spec.ts line 396
    #[test]
    fn profile_kind_extracted() {
        let content = r#"---
apiVersion: config.projectsveltos.io/v1beta1
kind: Profile
metadata:
  name: baseline
spec:
  helmCharts:
  - repositoryURL: https://prometheus-community.github.io/helm-charts
    repositoryName: prometheus-community
    chartName: prometheus-community/prometheus
    chartVersion: "23.4.0"
  - repositoryURL: https://kyverno.github.io/kyverno/
    repositoryName: kyverno
    chartName: kyverno/kyverno
    chartVersion: "v3.2.5"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(
            deps.iter()
                .any(|d| d.chart_name == "prometheus-community/prometheus"
                    && d.current_value == "23.4.0")
        );
        assert!(
            deps.iter()
                .any(|d| d.chart_name == "kyverno/kyverno" && d.current_value == "v3.2.5")
        );
    }

    // Ported: "supports clusterpromotions" — sveltos/extract.spec.ts line 518
    #[test]
    fn clusterpromotion_kind_extracted() {
        let content = r#"---
apiVersion: config.projectsveltos.io/v1beta1
kind: ClusterPromotion
metadata:
  name: baseline
spec:
  profileSpec:
    helmCharts:
    - repositoryURL:    https://prometheus-community.github.io/helm-charts
      repositoryName:   prometheus-community
      chartName:        prometheus-community/prometheus
      chartVersion:     "23.4.0"
    - repositoryURL:    https://kyverno.github.io/kyverno/
      repositoryName:   kyverno
      chartName:        kyverno/kyverno
      chartVersion:     "v3.2.5"
---
apiVersion: config.projectsveltos.io/v1beta1
kind: ClusterPromotion
metadata:
  name: vault
spec:
  profileSpec:
    helmCharts:
    - repositoryURL:    oci://registry-1.docker.io/bitnamicharts/vault
      repositoryName:   oci-vault
      chartName:        oci://registry-1.docker.io/bitnamicharts/vault
      chartVersion:     0.7.2
"#;
        let deps = extract_with_registry_aliases(content, &[]);
        assert_eq!(deps.len(), 3);
        let prometheus = deps
            .iter()
            .find(|d| d.dep_name == "prometheus-community/prometheus")
            .unwrap();
        assert_eq!(prometheus.current_value, "23.4.0");
        assert_eq!(prometheus.datasource, "helm");
        assert_eq!(prometheus.dep_type, "ClusterPromotion");
        assert_eq!(prometheus.package_name, "prometheus");
        assert_eq!(
            prometheus.registry_urls,
            vec!["https://prometheus-community.github.io/helm-charts"]
        );
        let kyverno = deps
            .iter()
            .find(|d| d.dep_name == "kyverno/kyverno")
            .unwrap();
        assert_eq!(kyverno.current_value, "v3.2.5");
        assert_eq!(kyverno.datasource, "helm");
        assert_eq!(kyverno.dep_type, "ClusterPromotion");
        assert_eq!(kyverno.package_name, "kyverno");
        assert_eq!(
            kyverno.registry_urls,
            vec!["https://kyverno.github.io/kyverno/"]
        );
        let vault = deps
            .iter()
            .find(|d| d.dep_name == "oci://registry-1.docker.io/bitnamicharts/vault")
            .unwrap();
        assert_eq!(vault.current_value, "0.7.2");
        assert_eq!(vault.datasource, "docker");
        assert_eq!(vault.dep_type, "ClusterPromotion");
        assert_eq!(
            vault.package_name,
            "registry-1.docker.io/bitnamicharts/vault"
        );
        assert!(vault.registry_urls.is_empty());
    }

    // Ported: "supports eventtriggers" — sveltos/extract.spec.ts line 554
    #[test]
    fn eventtrigger_kind_extracted() {
        let content = r#"---
apiVersion: lib.projectsveltos.io/v1beta1
kind: EventTrigger
metadata:
  name: baseline
spec:
  helmCharts:
  - repositoryURL: https://prometheus-community.github.io/helm-charts
    repositoryName: prometheus-community
    chartName: prometheus-community/prometheus
    chartVersion: "23.4.0"
  - repositoryURL: https://kyverno.github.io/kyverno/
    repositoryName: kyverno
    chartName: kyverno/kyverno
    chartVersion: "v3.2.5"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(
            deps.iter()
                .any(|d| d.chart_name == "prometheus-community/prometheus")
        );
        assert!(
            deps.iter()
                .any(|d| d.chart_name == "kyverno/kyverno" && d.current_value == "v3.2.5")
        );
    }
}
