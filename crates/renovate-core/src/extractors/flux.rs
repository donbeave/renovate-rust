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
pub const BITBUCKET_TAGS_DATASOURCE: &str = "bitbucket-tags";
pub const GIT_REFS_DATASOURCE: &str = "git-refs";
pub const GIT_TAGS_DATASOURCE: &str = "git-tags";
pub const GITHUB_TAGS_DATASOURCE: &str = "github-tags";
pub const GITLAB_TAGS_DATASOURCE: &str = "gitlab-tags";

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
    UnsupportedDatasource,
    UnversionedReference,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FluxGitRepositoryDep {
    pub dep_name: String,
    pub datasource: Option<&'static str>,
    pub current_value: Option<String>,
    pub current_digest: Option<String>,
    pub package_name: Option<String>,
    pub replace_string: Option<String>,
    pub source_url: Option<String>,
    pub skip_reason: Option<FluxSkipReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HelmRepository {
    name: String,
    namespace: String,
    url: String,
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
    let docs = yaml_documents(content);
    let repositories: Vec<_> = docs
        .iter()
        .filter_map(|doc| extract_helm_repo(doc))
        .collect();

    let mut deps = Vec::new();
    for doc in &docs {
        if let Some(dep) = extract_helm_release_doc(doc, registry_aliases, &repositories) {
            deps.push(dep);
        }
        if let Some(dep) = extract_helm_chart_doc(doc, registry_aliases, &repositories) {
            deps.push(dep);
        }
    }
    deps
}

pub fn extract_git_repositories(content: &str) -> Vec<FluxGitRepositoryDep> {
    yaml_documents(content)
        .iter()
        .filter_map(|doc| extract_git_repository_doc(doc))
        .collect()
}

fn yaml_documents(content: &str) -> Vec<Vec<(Vec<String>, String)>> {
    content.split("\n---").map(yaml_scalars).collect()
}

fn extract_helm_repo(scalars: &[(Vec<String>, String)]) -> Option<HelmRepository> {
    if value_at(scalars, &["apiVersion"]).is_none()
        || value_at(scalars, &["kind"]) != Some("HelmRepository")
    {
        return None;
    }

    Some(HelmRepository {
        name: value_at(scalars, &["metadata", "name"])?.to_owned(),
        namespace: value_at(scalars, &["metadata", "namespace"])?.to_owned(),
        url: value_at(scalars, &["spec", "url"])?.to_owned(),
    })
}

fn extract_git_repository_doc(scalars: &[(Vec<String>, String)]) -> Option<FluxGitRepositoryDep> {
    if value_at(scalars, &["apiVersion"]).is_none()
        || value_at(scalars, &["kind"]) != Some("GitRepository")
    {
        return None;
    }

    let dep_name = value_at(scalars, &["metadata", "name"])?;
    let url = value_at(scalars, &["spec", "url"])?;
    if let Some(commit) = value_at(scalars, &["spec", "ref", "commit"]) {
        return Some(FluxGitRepositoryDep {
            dep_name: dep_name.to_owned(),
            datasource: Some(GIT_REFS_DATASOURCE),
            current_value: None,
            current_digest: Some(commit.to_owned()),
            package_name: Some(url.to_owned()),
            replace_string: Some(commit.to_owned()),
            source_url: Some(normalize_git_source_url(url)),
            skip_reason: None,
        });
    }

    if let Some(tag) = value_at(scalars, &["spec", "ref", "tag"]) {
        let normalized_url = normalize_git_source_url(url);
        let (datasource, package_name) = git_tag_source(&normalized_url);
        return Some(FluxGitRepositoryDep {
            dep_name: dep_name.to_owned(),
            datasource: Some(datasource),
            current_value: Some(tag.to_owned()),
            current_digest: None,
            package_name: Some(package_name),
            replace_string: None,
            source_url: Some(normalized_url),
            skip_reason: None,
        });
    }

    Some(FluxGitRepositoryDep {
        dep_name: dep_name.to_owned(),
        datasource: None,
        current_value: None,
        current_digest: None,
        package_name: None,
        replace_string: None,
        source_url: None,
        skip_reason: Some(FluxSkipReason::UnversionedReference),
    })
}

fn normalize_git_source_url(url: &str) -> String {
    if let Some(path) = url.strip_prefix("git@github.com:") {
        return format!(
            "https://github.com/{}",
            path.strip_suffix(".git").unwrap_or(path)
        );
    }
    url.strip_suffix(".git").unwrap_or(url).to_owned()
}

fn git_tag_source(url: &str) -> (&'static str, String) {
    if let Some(path) = url.strip_prefix("https://github.com/") {
        return (GITHUB_TAGS_DATASOURCE, path.to_owned());
    }
    if let Some(path) = url.strip_prefix("https://gitlab.com/") {
        return (GITLAB_TAGS_DATASOURCE, path.to_owned());
    }
    if let Some(path) = url.strip_prefix("https://bitbucket.org/") {
        return (BITBUCKET_TAGS_DATASOURCE, path.to_owned());
    }
    (GIT_TAGS_DATASOURCE, url.to_owned())
}

fn extract_helm_release_doc(
    scalars: &[(Vec<String>, String)],
    registry_aliases: &[(&str, &str)],
    repositories: &[HelmRepository],
) -> Option<FluxHelmReleaseDep> {
    if value_at(scalars, &["apiVersion"]).is_none()
        || value_at(scalars, &["kind"]) != Some("HelmRelease")
    {
        return None;
    }

    let chart = value_at(scalars, &["spec", "chart", "spec", "chart"])?;
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

    let current_value = value_at(scalars, &["spec", "chart", "spec", "version"])?;
    let source_name = value_at(scalars, &["spec", "chart", "spec", "sourceRef", "name"]);
    let release_namespace = value_at(scalars, &["metadata", "namespace"]);
    let source_namespace = value_at(
        scalars,
        &["spec", "chart", "spec", "sourceRef", "namespace"],
    )
    .or(release_namespace);

    if source_name.is_some() && release_namespace.is_none() {
        return None;
    }

    let mut dep = FluxHelmReleaseDep {
        dep_name: chart.to_owned(),
        current_value: Some(current_value.to_owned()),
        datasource: Some(HELM_DATASOURCE),
        registry_urls: Vec::new(),
        package_name: None,
        skip_reason: Some(FluxSkipReason::UnknownRegistry),
    };

    if let Some(source_name) = source_name {
        if let Some(source_namespace) = source_namespace
            && let Some(repository) = repositories
                .iter()
                .find(|repo| repo.name == source_name && repo.namespace == source_namespace)
        {
            dep.skip_reason = None;
            dep.registry_urls.push(repository.url.clone());
            return Some(dep);
        }

        if let Some((_, alias)) = registry_aliases
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
    }

    Some(dep)
}

fn extract_helm_chart_doc(
    scalars: &[(Vec<String>, String)],
    registry_aliases: &[(&str, &str)],
    repositories: &[HelmRepository],
) -> Option<FluxHelmReleaseDep> {
    if value_at(scalars, &["apiVersion"]).is_none()
        || value_at(scalars, &["kind"]) != Some("HelmChart")
    {
        return None;
    }

    let chart = value_at(scalars, &["spec", "chart"])?;
    let source_kind = value_at(scalars, &["spec", "sourceRef", "kind"]);
    if chart.starts_with("./") || chart.starts_with("../") {
        if source_kind == Some("Bucket") {
            return Some(FluxHelmReleaseDep {
                dep_name: chart.to_owned(),
                current_value: None,
                datasource: None,
                registry_urls: Vec::new(),
                package_name: None,
                skip_reason: Some(FluxSkipReason::UnsupportedDatasource),
            });
        }
        return None;
    }

    if source_kind.is_some_and(|kind| kind != "HelmRepository") {
        return None;
    }

    let current_value = value_at(scalars, &["spec", "version"])?;
    let source_name = value_at(scalars, &["spec", "sourceRef", "name"]);
    let chart_namespace = value_at(scalars, &["metadata", "namespace"]);
    let source_namespace =
        value_at(scalars, &["spec", "sourceRef", "namespace"]).or(chart_namespace);

    let mut dep = FluxHelmReleaseDep {
        dep_name: chart.to_owned(),
        current_value: Some(current_value.to_owned()),
        datasource: Some(HELM_DATASOURCE),
        registry_urls: Vec::new(),
        package_name: None,
        skip_reason: Some(FluxSkipReason::UnknownRegistry),
    };

    if let Some(source_name) = source_name {
        if let Some(source_namespace) = source_namespace
            && let Some(repository) = repositories
                .iter()
                .find(|repo| repo.name == source_name && repo.namespace == source_namespace)
        {
            dep.skip_reason = None;
            dep.registry_urls.push(repository.url.clone());
            return Some(dep);
        }

        if let Some((_, alias)) = registry_aliases
            .iter()
            .find(|(source, _)| *source == source_name)
        {
            dep.skip_reason = None;
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

    // Ported: "does not match HelmRelease resources without a namespace to HelmRepository resources without a namespace" — flux/extract.spec.ts line 299
    #[test]
    fn does_not_match_release_without_namespace_to_repository_without_namespace() {
        let content = r#"
apiVersion: source.toolkit.fluxcd.io/v1beta1
kind: HelmRepository
metadata:
  name: sealed-secrets
spec:
  url: https://bitnami-labs.github.io/sealed-secrets
---
apiVersion: helm.toolkit.fluxcd.io/v2beta1
kind: HelmRelease
spec:
  chart:
    spec:
      chart: sealed-secrets
      sourceRef:
        kind: HelmRepository
        name: sealed-secrets
      version: "2.0.2"
"#;
        assert!(extract_helm_releases(content).is_empty());
    }

    // Ported: "does not match HelmRelease resources without a sourceRef" — flux/extract.spec.ts line 325
    #[test]
    fn release_without_source_ref_is_unknown_registry() {
        let content = format!(
            "{HELM_REPOSITORY}\n---\napiVersion: helm.toolkit.fluxcd.io/v2beta1\nkind: HelmRelease\nmetadata:\n  name: sealed-secrets\n  namespace: test\nspec:\n  chart:\n    spec:\n      chart: sealed-secrets\n      version: \"2.0.2\"\n"
        );
        let deps = extract_helm_releases(&content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(FluxSkipReason::UnknownRegistry));
    }

    // Ported: "does not match HelmRelease resources without a namespace" — flux/extract.spec.ts line 355
    #[test]
    fn does_not_match_release_without_namespace() {
        let content = format!(
            "{HELM_REPOSITORY}\n---\napiVersion: helm.toolkit.fluxcd.io/v2beta1\nkind: HelmRelease\nspec:\n  chart:\n    spec:\n      chart: sealed-secrets\n      sourceRef:\n        kind: HelmRepository\n        name: sealed-secrets\n      version: \"2.0.2\"\n"
        );
        assert!(extract_helm_releases(&content).is_empty());
    }

    // Ported: "ignores HelmRepository resources without a namespace" — flux/extract.spec.ts line 376
    #[test]
    fn ignores_helm_repository_without_namespace() {
        let content = format!(
            "{HELM_RELEASE}\n---\napiVersion: source.toolkit.fluxcd.io/v1beta1\nkind: HelmRepository\nmetadata:\n  name: test\n"
        );
        let deps = extract_helm_releases(&content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(FluxSkipReason::UnknownRegistry));
    }

    // Ported: "ignores HelmRepository resources without a URL" — flux/extract.spec.ts line 400
    #[test]
    fn ignores_helm_repository_without_url() {
        let content = format!(
            "{HELM_RELEASE}\n---\napiVersion: source.toolkit.fluxcd.io/v1beta1\nkind: HelmRepository\nmetadata:\n  name: sealed-secrets\n  namespace: kube-system\n"
        );
        let deps = extract_helm_releases(&content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(FluxSkipReason::UnknownRegistry));
    }

    // Ported: "ignores HelmRelease resources using an invalid chartRef" — flux/extract.spec.ts line 425
    #[test]
    fn ignores_helm_release_with_invalid_chart_ref() {
        let content = r#"
apiVersion: helm.toolkit.fluxcd.io/v2
kind: HelmRelease
metadata:
  name: sealed-secrets
  namespace: kube-system
spec:
  interval: 10m
  chartRef:
    kind: HelmChart
    name: sealed-secrets
    namespace: kube-system
"#;
        assert!(extract_helm_releases(content).is_empty());
    }

    // Ported: "ignores HelmRelease resources using a chartRef targetting a HelmChart" — flux/extract.spec.ts line 433
    #[test]
    fn ignores_release_chart_ref_and_extracts_helm_chart() {
        let content =
            format!("{HELM_CHART_REF_RELEASE}\n---\n{HELM_CHART}\n---\n{HELM_REPOSITORY}");
        let deps = extract_helm_releases(&content);
        assert_eq!(
            deps,
            vec![FluxHelmReleaseDep {
                dep_name: "sealed-secrets".to_owned(),
                current_value: Some("2.0.2".to_owned()),
                datasource: Some(HELM_DATASOURCE),
                registry_urls: vec!["https://bitnami-labs.github.io/sealed-secrets".to_owned()],
                package_name: None,
                skip_reason: None,
            }]
        );
    }

    // Ported: "extracts HelmChart version" — flux/extract.spec.ts line 492
    #[test]
    fn extracts_helm_chart_version() {
        let content = format!("{HELM_REPOSITORY}\n---\n{HELM_CHART}");
        let deps = extract_helm_releases(&content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "sealed-secrets");
        assert_eq!(deps[0].current_value.as_deref(), Some("2.0.2"));
        assert_eq!(
            deps[0].registry_urls,
            vec!["https://bitnami-labs.github.io/sealed-secrets"]
        );
    }

    // Ported: "does not match HelmChart resources without a namespace" — flux/extract.spec.ts line 513
    #[test]
    fn helm_chart_without_namespace_is_unknown_registry() {
        let content = format!(
            "{HELM_REPOSITORY}\n---\napiVersion: source.toolkit.fluxcd.io/v1\nkind: HelmChart\nmetadata:\n  name: sealed-secrets\nspec:\n  interval: 10m\n  chart: sealed-secrets\n  sourceRef:\n    kind: HelmRepository\n    name: sealed-secrets\n  version: \"2.0.2\"\n"
        );
        let deps = extract_helm_releases(&content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(FluxSkipReason::UnknownRegistry));
    }

    // Ported: "falls back to unknown-registry when registryAliases has no matching HelmChart sourceRef name" — flux/extract.spec.ts line 544
    #[test]
    fn helm_chart_registry_alias_without_source_match_is_unknown() {
        let deps = extract_helm_releases_with_registry_aliases(
            HELM_CHART,
            &[("other-repo", "https://example.com/charts")],
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(FluxSkipReason::UnknownRegistry));
    }

    // Ported: "uses registryAliases to resolve HelmChart sourceRef name when repository is missing" — flux/extract.spec.ts line 566
    #[test]
    fn helm_chart_registry_alias_resolves_source_name() {
        let deps = extract_helm_releases_with_registry_aliases(
            HELM_CHART,
            &[("sealed-secrets", "https://example.com/charts")],
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, None);
        assert_eq!(deps[0].registry_urls, vec!["https://example.com/charts"]);
    }

    // Ported: "ignores HelmChart resources using git sources" — flux/extract.spec.ts line 588
    #[test]
    fn ignores_helm_chart_using_git_source() {
        let content = r#"
apiVersion: source.toolkit.fluxcd.io/v1
kind: HelmChart
metadata:
  name: sealed-secrets
  namespace: kube-system
spec:
  interval: 10m
  chart: ./helm/sealed-secrets
  sourceRef:
    kind: GitRepository
    name: sealed-secrets
"#;
        assert!(extract_helm_releases(content).is_empty());
    }

    // Ported: "ignores HelmChart resources using bucket sources" — flux/extract.spec.ts line 608
    #[test]
    fn helm_chart_using_bucket_source_is_unsupported() {
        let content = r#"
apiVersion: source.toolkit.fluxcd.io/v1
kind: Bucket
metadata:
  name: sealed-secrets
  namespace: kube-system
spec:
  interval: 5m0s
  endpoint: sealed-secrets.example.com
  bucketName: example
---
apiVersion: source.toolkit.fluxcd.io/v1
kind: HelmChart
metadata:
  name: sealed-secrets
  namespace: kube-system
spec:
  interval: 10m
  chart: ./helm/sealed-secrets
  sourceRef:
    kind: Bucket
    name: sealed-secrets
"#;
        let deps = extract_helm_releases(content);
        assert_eq!(
            deps,
            vec![FluxHelmReleaseDep {
                dep_name: "./helm/sealed-secrets".to_owned(),
                current_value: None,
                datasource: None,
                registry_urls: Vec::new(),
                package_name: None,
                skip_reason: Some(FluxSkipReason::UnsupportedDatasource),
            }]
        );
    }

    // Ported: "ignores GitRepository without a tag nor a commit" — flux/extract.spec.ts line 645
    #[test]
    fn ignores_git_repository_without_tag_or_commit() {
        let deps = extract_git_repositories(
            "apiVersion: source.toolkit.fluxcd.io/v1beta1\nkind: GitRepository\nmetadata:\n  name: renovate-repo\n  namespace: renovate-system\nspec:\n  url: https://github.com/renovatebot/renovate\n",
        );
        assert_eq!(
            deps,
            vec![FluxGitRepositoryDep {
                dep_name: "renovate-repo".to_owned(),
                datasource: None,
                current_value: None,
                current_digest: None,
                package_name: None,
                replace_string: None,
                source_url: None,
                skip_reason: Some(FluxSkipReason::UnversionedReference),
            }]
        );
    }

    // Ported: "extracts GitRepository with a commit" — flux/extract.spec.ts line 665
    #[test]
    fn extracts_git_repository_with_commit() {
        let deps = extract_git_repositories(
            "apiVersion: source.toolkit.fluxcd.io/v1beta1\nkind: GitRepository\nmetadata:\n  name: renovate-repo\n  namespace: renovate-system\nspec:\n  ref:\n    commit: c93154b\n  url: https://github.com/renovatebot/renovate\n",
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].datasource, Some(GIT_REFS_DATASOURCE));
        assert_eq!(deps[0].current_digest.as_deref(), Some("c93154b"));
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("https://github.com/renovatebot/renovate")
        );
        assert_eq!(deps[0].replace_string.as_deref(), Some("c93154b"));
    }

    // Ported: "extracts GitRepository with a tag from github with ssh" — flux/extract.spec.ts line 694
    #[test]
    fn extracts_git_repository_tag_from_github_ssh() {
        let deps = extract_git_repositories(
            "apiVersion: source.toolkit.fluxcd.io/v1beta1\nkind: GitRepository\nmetadata:\n  name: renovate-repo\n  namespace: renovate-system\nspec:\n  ref:\n    tag: v11.35.9\n  url: git@github.com:renovatebot/renovate.git\n",
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].datasource, Some(GITHUB_TAGS_DATASOURCE));
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("renovatebot/renovate")
        );
        assert_eq!(
            deps[0].source_url.as_deref(),
            Some("https://github.com/renovatebot/renovate")
        );
    }

    // Ported: "extracts GitRepository with a tag from github" — flux/extract.spec.ts line 722
    #[test]
    fn extracts_git_repository_tag_from_github() {
        let deps = extract_git_repositories(GIT_REPOSITORY);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].datasource, Some(GITHUB_TAGS_DATASOURCE));
        assert_eq!(deps[0].current_value.as_deref(), Some("v11.35.4"));
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("renovatebot/renovate")
        );
    }

    // Ported: "extracts GitRepository with a tag from gitlab" — flux/extract.spec.ts line 750
    #[test]
    fn extracts_git_repository_tag_from_gitlab() {
        let deps = extract_git_repositories(
            "apiVersion: source.toolkit.fluxcd.io/v1beta1\nkind: GitRepository\nmetadata:\n  name: renovate-repo\n  namespace: renovate-system\nspec:\n  ref:\n    tag: 1.2.3\n  url: https://gitlab.com/renovatebot/renovate\n",
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].datasource, Some(GITLAB_TAGS_DATASOURCE));
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("renovatebot/renovate")
        );
    }

    // Ported: "extracts GitRepository with a tag from bitbucket" — flux/extract.spec.ts line 778
    #[test]
    fn extracts_git_repository_tag_from_bitbucket() {
        let deps = extract_git_repositories(
            "apiVersion: source.toolkit.fluxcd.io/v1beta1\nkind: GitRepository\nmetadata:\n  name: renovate-repo\n  namespace: renovate-system\nspec:\n  ref:\n    tag: 2020.5.6+staging.ze\n  url: https://bitbucket.org/renovatebot/renovate\n",
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].datasource, Some(BITBUCKET_TAGS_DATASOURCE));
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("renovatebot/renovate")
        );
    }

    // Ported: "extracts GitRepository with a tag from an unkown domain" — flux/extract.spec.ts line 806
    #[test]
    fn extracts_git_repository_tag_from_unknown_domain() {
        let deps = extract_git_repositories(
            "apiVersion: source.toolkit.fluxcd.io/v1beta1\nkind: GitRepository\nmetadata:\n  name: renovate-repo\n  namespace: renovate-system\nspec:\n  ref:\n    tag: \"7.56.4_p1\"\n  url: https://example.com/renovatebot/renovate\n",
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].datasource, Some(GIT_TAGS_DATASOURCE));
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("https://example.com/renovatebot/renovate")
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

    const HELM_REPOSITORY: &str = r#"
apiVersion: source.toolkit.fluxcd.io/v1beta1
kind: HelmRepository
metadata:
  name: sealed-secrets
  namespace: kube-system
spec:
  interval: 1h0m0s
  url: https://bitnami-labs.github.io/sealed-secrets
"#;

    const HELM_CHART: &str = r#"
apiVersion: source.toolkit.fluxcd.io/v1
kind: HelmChart
metadata:
  name: sealed-secrets
  namespace: kube-system
spec:
  interval: 10m
  chart: sealed-secrets
  sourceRef:
    kind: HelmRepository
    name: sealed-secrets
  version: "2.0.2"
  valuesFiles:
    - values-prod.yaml
"#;

    const HELM_CHART_REF_RELEASE: &str = r#"
apiVersion: helm.toolkit.fluxcd.io/v2
kind: HelmRelease
metadata:
  name: sealed-secrets
  namespace: kube-system
spec:
  interval: 10m
  chartRef:
    kind: HelmChart
    name: sealed-secrets
    namespace: kube-system
  values:
    replicaCount: 2
"#;

    const GIT_REPOSITORY: &str = r#"
apiVersion: source.toolkit.fluxcd.io/v1beta1
kind: GitRepository
metadata:
  name: renovate-repo
  namespace: renovate-system
spec:
  interval: 1h0m0s
  url: https://github.com/renovatebot/renovate
  ref:
    tag: v11.35.4
"#;
}
