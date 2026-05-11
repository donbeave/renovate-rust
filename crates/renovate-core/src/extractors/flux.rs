//! FluxCD `gotk-components.yaml` system manifest extractor.
//!
//! Reads the Flux version from the standard comment header that Flux
//! injects into all generated `gotk-components.yaml` manifests, and
//! returns a single dep for the `fluxcd/flux2` GitHub release.
//!
//! Renovate reference:
//! - `lib/modules/manager/flux/common.ts` — `systemManifestHeaderRegex`
//! - `lib/modules/manager/flux/extract.ts` — `extractSystemManifest`
//! - Pattern: `/(^|/)gotk-components\.ya?ml$/`
//! - Datasource: GitHub Releases (`fluxcd/flux2`)
//!
//! ## Header format
//!
//! ```yaml
//! # Flux Version: v2.2.3
//! # Components: source-controller,kustomize-controller,...
//! ```

use std::sync::LazyLock;

use regex::Regex;

pub const FLUX2_REPO: &str = "fluxcd/flux2";

pub const HELM_DATASOURCE: &str = "helm";
pub const DOCKER_DATASOURCE: &str = "docker";

/// A single extracted FluxCD system-manifest dep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FluxSystemDep {
    /// Flux version (e.g. `"v2.2.3"`).
    pub version: String,
    /// Raw components string (e.g. `"source-controller,kustomize-controller"`).
    pub components: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FluxSkipReason {
    UnknownRegistry,
    LocalChart,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FluxHelmReleaseDep {
    pub dep_name: String,
    pub current_value: Option<String>,
    pub datasource: Option<&'static str>,
    pub registry_urls: Vec<String>,
    pub package_name: Option<String>,
    pub skip_reason: Option<FluxSkipReason>,
}

static HEADER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"#\s*Flux\s+Version:\s*(\S+)(?:\s*#\s*Components:\s*([A-Za-z,\-]+))?").unwrap()
});

/// Extract the Flux version from a `gotk-components.yaml` file.
///
/// Returns `None` when no valid Flux Version header is found.
pub fn extract(content: &str) -> Option<FluxSystemDep> {
    // Apply against whole content so the optional Components clause can span
    // the line boundary between `# Flux Version:` and `# Components:`.
    let cap = HEADER_RE.captures(content)?;
    Some(FluxSystemDep {
        version: cap[1].to_owned(),
        components: cap.get(2).map(|m| m.as_str().to_owned()),
    })
}

pub fn extract_helm_releases(content: &str) -> Vec<FluxHelmReleaseDep> {
    extract_helm_releases_with_registry_aliases(content, &[])
}

pub fn extract_helm_releases_with_registry_aliases(
    content: &str,
    registry_aliases: &[(&str, &str)],
) -> Vec<FluxHelmReleaseDep> {
    content
        .split("\n---")
        .filter_map(|doc| extract_helm_release_doc(doc, registry_aliases))
        .collect()
}

fn extract_helm_release_doc(
    doc: &str,
    registry_aliases: &[(&str, &str)],
) -> Option<FluxHelmReleaseDep> {
    let scalars = yaml_scalars(doc);
    if value_at(&scalars, &["apiVersion"]).is_none()
        || value_at(&scalars, &["kind"]) != Some("HelmRelease")
    {
        return None;
    }

    let chart = value_at(&scalars, &["spec", "chart", "spec", "chart"])?;
    if chart.starts_with("./") || chart.starts_with("../") {
        return Some(FluxHelmReleaseDep {
            dep_name: chart.to_owned(),
            current_value: None,
            datasource: None,
            registry_urls: Vec::new(),
            package_name: None,
            skip_reason: Some(FluxSkipReason::LocalChart),
        });
    }

    let current_value = value_at(&scalars, &["spec", "chart", "spec", "version"])?;
    let source_name = value_at(&scalars, &["spec", "chart", "spec", "sourceRef", "name"]);

    let mut dep = FluxHelmReleaseDep {
        dep_name: chart.to_owned(),
        current_value: Some(current_value.to_owned()),
        datasource: Some(HELM_DATASOURCE),
        registry_urls: Vec::new(),
        package_name: None,
        skip_reason: Some(FluxSkipReason::UnknownRegistry),
    };

    if let Some(source_name) = source_name
        && let Some((_, alias)) = registry_aliases
            .iter()
            .find(|(source, _)| *source == source_name)
    {
        dep.skip_reason = None;
        if let Some(oci_url) = alias.strip_prefix("oci://") {
            dep.datasource = Some(DOCKER_DATASOURCE);
            dep.package_name = Some(format!("{}/{}", oci_url.trim_end_matches('/'), chart));
        } else {
            dep.registry_urls.push((*alias).to_owned());
        }
    }

    Some(dep)
}

fn yaml_scalars(doc: &str) -> Vec<(Vec<String>, String)> {
    let mut stack: Vec<(usize, String)> = Vec::new();
    let mut out = Vec::new();

    for raw_line in doc.lines() {
        let line = raw_line
            .split_once('#')
            .map_or(raw_line, |(before, _)| before);
        if line.trim().is_empty() || line.trim() == "---" {
            continue;
        }

        let indent = line.len() - line.trim_start().len();
        let trimmed = line.trim();
        let Some((key, value)) = trimmed.split_once(':') else {
            continue;
        };
        let key = trim_yaml_scalar(key);
        if key.is_empty() {
            continue;
        }

        while stack.last().is_some_and(|(level, _)| *level >= indent) {
            stack.pop();
        }

        let value = trim_yaml_scalar(value);
        if value.is_empty() {
            stack.push((indent, key));
        } else {
            let mut path: Vec<String> = stack.iter().map(|(_, key)| key.clone()).collect();
            path.push(key);
            out.push((path, value));
        }
    }

    out
}

fn value_at<'a>(scalars: &'a [(Vec<String>, String)], path: &[&str]) -> Option<&'a str> {
    scalars
        .iter()
        .find(|(candidate, _)| {
            candidate.len() == path.len()
                && candidate
                    .iter()
                    .zip(path.iter())
                    .all(|(candidate, expected)| candidate == expected)
        })
        .map(|(_, value)| value.as_str())
}

fn trim_yaml_scalar(value: &str) -> String {
    value.trim().trim_matches('"').trim_matches('\'').to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "extracts version and components from system manifest at $filepath" — flux/extract.spec.ts line 72
    #[test]
    fn extracts_version_with_components() {
        let content = "# Flux Version: v2.2.3\n# Components: source-controller,kustomize-controller,helm-controller\napiVersion: v1\n";
        let dep = extract(content).unwrap();
        assert_eq!(dep.version, "v2.2.3");
        assert_eq!(
            dep.components.as_deref(),
            Some("source-controller,kustomize-controller,helm-controller")
        );
    }

    // Ported: "considers components optional in system manifests" — flux/extract.spec.ts line 102
    #[test]
    fn extracts_version_without_components() {
        let content = "# Flux Version: v2.1.0\napiVersion: v1\n";
        let dep = extract(content).unwrap();
        assert_eq!(dep.version, "v2.1.0");
        assert!(dep.components.is_none());
    }

    #[test]
    fn version_in_middle_of_file() {
        let content = "---\napiVersion: v1\n# Flux Version: v2.0.1\nkind: Namespace\n";
        let dep = extract(content).unwrap();
        assert_eq!(dep.version, "v2.0.1");
    }

    // Ported: "ignores system manifests without a version" — flux/extract.spec.ts line 111
    #[test]
    fn no_header_returns_none() {
        let content = "apiVersion: v1\nkind: Namespace\nmetadata:\n  name: flux-system\n";
        assert!(extract(content).is_none());
    }

    // Ported: "extracts releases without repositories" — flux/extract.spec.ts line 119
    #[test]
    fn extracts_helm_release_without_repository() {
        let deps = extract_helm_releases(HELM_RELEASE);
        assert_eq!(
            deps,
            vec![FluxHelmReleaseDep {
                dep_name: "sealed-secrets".to_owned(),
                current_value: Some("2.0.2".to_owned()),
                datasource: Some(HELM_DATASOURCE),
                registry_urls: Vec::new(),
                package_name: None,
                skip_reason: Some(FluxSkipReason::UnknownRegistry),
            }]
        );
    }

    // Ported: "falls back to unknown-registry when registryAliases has no matching HelmRelease sourceRef name" — flux/extract.spec.ts line 136
    #[test]
    fn helm_release_registry_alias_without_source_match_is_unknown() {
        let deps = extract_helm_releases_with_registry_aliases(
            HELM_RELEASE,
            &[("other-repo", "https://example.com/charts")],
        );
        assert_eq!(deps[0].skip_reason, Some(FluxSkipReason::UnknownRegistry));
        assert!(deps[0].registry_urls.is_empty());
    }

    // Ported: "uses registryAliases to resolve HelmRelease sourceRef name when repository is missing" — flux/extract.spec.ts line 158
    #[test]
    fn helm_release_registry_alias_resolves_source_name() {
        let deps = extract_helm_releases_with_registry_aliases(
            HELM_RELEASE,
            &[("sealed-secrets", "https://example.com/charts")],
        );
        assert_eq!(deps[0].skip_reason, None);
        assert_eq!(deps[0].datasource, Some(HELM_DATASOURCE));
        assert_eq!(deps[0].registry_urls, vec!["https://example.com/charts"]);
    }

    // Ported: "uses registryAliases with an OCI URL for HelmRelease sourceRef name" — flux/extract.spec.ts line 180
    #[test]
    fn helm_release_registry_alias_oci_url_uses_docker() {
        let deps = extract_helm_releases_with_registry_aliases(
            HELM_RELEASE,
            &[("sealed-secrets", "oci://ghcr.io/charts")],
        );
        assert_eq!(deps[0].skip_reason, None);
        assert_eq!(deps[0].datasource, Some(DOCKER_DATASOURCE));
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("ghcr.io/charts/sealed-secrets")
        );
    }

    // Ported: "ignores HelmRelease resources without an apiVersion" — flux/extract.spec.ts line 202
    #[test]
    fn ignores_helm_release_without_api_version() {
        assert!(extract_helm_releases("kind: HelmRelease").is_empty());
    }

    // Ported: "ignores HelmRepository resources without an apiVersion" — flux/extract.spec.ts line 207
    #[test]
    fn ignores_helm_repository_without_api_version() {
        assert!(extract_helm_releases("kind: HelmRepository").is_empty());
    }

    // Ported: "ignores HelmRepository resources without metadata" — flux/extract.spec.ts line 212
    #[test]
    fn ignores_helm_repository_without_metadata() {
        let content = format!(
            "{HELM_RELEASE}\n---\napiVersion: source.toolkit.fluxcd.io/v1beta1\nkind: HelmRepository\n"
        );
        let deps = extract_helm_releases(&content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "sealed-secrets");
    }

    // Ported: "ignores HelmRelease resources without any chart reference" — flux/extract.spec.ts line 234
    #[test]
    fn ignores_helm_release_without_chart_reference() {
        let content = r#"
apiVersion: helm.toolkit.fluxcd.io/v2beta1
kind: HelmRelease
metadata:
  name: sealed-secrets
  namespace: kube-system
spec:
  interval: 10m
"#;
        assert!(extract_helm_releases(content).is_empty());
    }

    // Ported: "ignores HelmRelease resources without a chart name" — flux/extract.spec.ts line 250
    #[test]
    fn ignores_helm_release_without_chart_name() {
        let content = r#"
apiVersion: helm.toolkit.fluxcd.io/v2beta1
kind: HelmRelease
metadata:
  name: sealed-secrets
  namespace: kube-system
spec:
  chart:
    spec:
      sourceRef:
        kind: HelmRepository
        name: sealed-secrets
      version: "2.0.2"
"#;
        assert!(extract_helm_releases(content).is_empty());
    }

    // Ported: "skip HelmRelease with local chart" — flux/extract.spec.ts line 271
    #[test]
    fn skips_helm_release_with_local_chart() {
        let content = r#"
apiVersion: helm.toolkit.fluxcd.io/v2beta1
kind: HelmRelease
metadata:
  name: cert-manager-config
  namespace: kube-system
spec:
  chart:
    spec:
      chart: ./charts/cert-manager-config
      sourceRef:
        kind: GitRepository
        name: chart-repo
"#;
        let deps = extract_helm_releases(content);
        assert_eq!(
            deps,
            vec![FluxHelmReleaseDep {
                dep_name: "./charts/cert-manager-config".to_owned(),
                current_value: None,
                datasource: None,
                registry_urls: Vec::new(),
                package_name: None,
                skip_reason: Some(FluxSkipReason::LocalChart),
            }]
        );
    }

    #[test]
    fn empty_returns_none() {
        assert!(extract("").is_none());
    }

    const HELM_RELEASE: &str = r#"
apiVersion: helm.toolkit.fluxcd.io/v2beta1
kind: HelmRelease
metadata:
  name: sealed-secrets
  namespace: kube-system
spec:
  releaseName: sealed-secrets-controller
  chart:
    spec:
      chart: sealed-secrets
      sourceRef:
        kind: HelmRepository
        name: sealed-secrets
        namespace: kube-system
      version: "2.0.2"
  interval: 1h0m0s
"#;
}
