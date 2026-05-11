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
    InvalidValue,
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
pub struct FluxOciRepositoryDep {
    pub dep_name: String,
    pub datasource: &'static str,
    pub current_value: Option<String>,
    pub current_digest: Option<String>,
    pub package_name: String,
    pub replace_string: Option<String>,
    pub skip_reason: Option<FluxSkipReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FluxKustomizationImageDep {
    pub dep_name: String,
    pub datasource: &'static str,
    pub current_value: Option<String>,
    pub current_digest: Option<String>,
    pub package_name: String,
    pub replace_string: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FluxDep {
    Helm(FluxHelmReleaseDep),
    Git(FluxGitRepositoryDep),
    Oci(FluxOciRepositoryDep),
    KustomizationImage(FluxKustomizationImageDep),
    System(FluxSystemDep),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FluxPackageFile {
    pub package_file: String,
    pub deps: Vec<FluxDep>,
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

pub fn extract_oci_repositories(content: &str) -> Vec<FluxOciRepositoryDep> {
    extract_oci_repositories_with_registry_aliases(content, &[])
}

pub fn extract_oci_repositories_with_registry_aliases(
    content: &str,
    registry_aliases: &[(&str, &str)],
) -> Vec<FluxOciRepositoryDep> {
    yaml_documents(content)
        .iter()
        .filter_map(|doc| extract_oci_repository_doc(doc, registry_aliases))
        .collect()
}

pub fn extract_kustomizations(content: &str) -> Vec<FluxKustomizationImageDep> {
    content
        .split("\n---")
        .flat_map(extract_kustomization_doc)
        .collect()
}

pub fn extract_all_package_files(files: &[(&str, Option<&str>)]) -> Vec<FluxPackageFile> {
    extract_all_package_files_with_registry_aliases(files, &[])
}

pub fn extract_all_package_files_with_registry_aliases(
    files: &[(&str, Option<&str>)],
    registry_aliases: &[(&str, &str)],
) -> Vec<FluxPackageFile> {
    let parsed_files: Vec<_> = files
        .iter()
        .filter_map(|(path, content)| {
            content.map(|content| (*path, content, yaml_documents(content)))
        })
        .collect();
    let repositories: Vec<_> = parsed_files
        .iter()
        .flat_map(|(_, _, docs)| docs.iter().filter_map(|doc| extract_helm_repo(doc)))
        .collect();

    parsed_files
        .iter()
        .filter_map(|(path, content, docs)| {
            let mut deps = Vec::new();

            for doc in docs {
                if let Some(dep) = extract_helm_release_doc(doc, registry_aliases, &repositories) {
                    deps.push(FluxDep::Helm(dep));
                }
                if let Some(dep) = extract_helm_chart_doc(doc, registry_aliases, &repositories) {
                    deps.push(FluxDep::Helm(dep));
                }
                if let Some(dep) = extract_git_repository_doc(doc) {
                    deps.push(FluxDep::Git(dep));
                }
                if let Some(dep) = extract_oci_repository_doc(doc, registry_aliases) {
                    deps.push(FluxDep::Oci(dep));
                }
            }

            deps.extend(
                extract_kustomization_doc(content)
                    .into_iter()
                    .map(FluxDep::KustomizationImage),
            );

            if (path.ends_with("gotk-components.yaml") || path.ends_with("gotk-components.yml"))
                && let Some(dep) = extract(content)
            {
                deps.push(FluxDep::System(dep));
            }

            if deps.is_empty() {
                None
            } else {
                Some(FluxPackageFile {
                    package_file: (*path).to_owned(),
                    deps,
                })
            }
        })
        .collect()
}

pub fn extract_package_file(content: &str) -> Vec<FluxDep> {
    extract_package_file_with_registry_aliases(content, &[])
}

pub fn extract_package_file_with_registry_aliases(
    content: &str,
    registry_aliases: &[(&str, &str)],
) -> Vec<FluxDep> {
    let docs = yaml_documents(content);
    let repositories: Vec<_> = docs
        .iter()
        .filter_map(|doc| extract_helm_repo(doc))
        .collect();
    let mut deps = Vec::new();

    for doc in &docs {
        if let Some(dep) = extract_helm_release_doc(doc, registry_aliases, &repositories) {
            deps.push(FluxDep::Helm(dep));
        }
        if let Some(dep) = extract_helm_release_image_doc(doc, registry_aliases) {
            deps.push(FluxDep::Oci(dep));
        }
        if let Some(dep) = extract_helm_chart_doc(doc, registry_aliases, &repositories) {
            deps.push(FluxDep::Helm(dep));
        }
        if let Some(dep) = extract_git_repository_doc(doc) {
            deps.push(FluxDep::Git(dep));
        }
        if let Some(dep) = extract_oci_repository_doc(doc, registry_aliases) {
            deps.push(FluxDep::Oci(dep));
        }
    }

    deps.extend(
        extract_kustomization_doc(content)
            .into_iter()
            .map(FluxDep::KustomizationImage),
    );
    deps
}

fn yaml_documents(content: &str) -> Vec<Vec<(Vec<String>, String)>> {
    content.split("\n---").map(yaml_scalars).collect()
}

fn extract_kustomization_doc(doc: &str) -> Vec<FluxKustomizationImageDep> {
    let scalars = yaml_scalars(doc);
    if value_at(&scalars, &["apiVersion"]).is_none()
        || value_at(&scalars, &["kind"]) != Some("Kustomization")
    {
        return Vec::new();
    }

    let mut images = Vec::new();
    let mut current = KustomizationImageFields::default();
    let mut in_images = false;
    let mut images_indent = 0;

    for raw_line in doc.lines() {
        let line = raw_line
            .split_once('#')
            .map_or(raw_line, |(before, _)| before);
        if line.trim().is_empty() {
            continue;
        }

        let indent = line.len() - line.trim_start().len();
        let trimmed = line.trim();
        if trimmed == "images:" {
            in_images = true;
            images_indent = indent;
            continue;
        }
        if !in_images {
            continue;
        }
        if indent <= images_indent && !trimmed.starts_with('-') {
            break;
        }

        if let Some(rest) = trimmed.strip_prefix("- ") {
            if current.name.is_some() {
                if let Some(dep) = current.to_dep() {
                    images.push(dep);
                }
                current = KustomizationImageFields::default();
            }
            set_kustomization_image_field(&mut current, rest);
        } else {
            set_kustomization_image_field(&mut current, trimmed);
        }
    }

    if let Some(dep) = current.to_dep() {
        images.push(dep);
    }

    images
}

#[derive(Default)]
struct KustomizationImageFields {
    name: Option<String>,
    new_name: Option<String>,
    new_tag: Option<String>,
    digest: Option<String>,
}

impl KustomizationImageFields {
    fn to_dep(&self) -> Option<FluxKustomizationImageDep> {
        let name = self.name.as_ref()?;
        let dep_name = self.new_name.as_ref().unwrap_or(name).to_owned();
        let replace_string = self
            .new_tag
            .clone()
            .or_else(|| self.digest.clone())
            .or_else(|| self.new_name.clone());
        Some(FluxKustomizationImageDep {
            dep_name: dep_name.clone(),
            datasource: DOCKER_DATASOURCE,
            current_value: self.new_tag.clone(),
            current_digest: self.digest.clone(),
            package_name: dep_name,
            replace_string,
        })
    }
}

fn set_kustomization_image_field(fields: &mut KustomizationImageFields, entry: &str) {
    let Some((key, value)) = entry.split_once(':') else {
        return;
    };
    let value = trim_yaml_scalar(value);
    match trim_yaml_scalar(key).as_str() {
        "name" => fields.name = Some(value),
        "newName" => fields.new_name = Some(value),
        "newTag" => fields.new_tag = Some(value),
        "digest" => fields.digest = Some(value),
        _ => {}
    }
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

fn extract_oci_repository_doc(
    scalars: &[(Vec<String>, String)],
    registry_aliases: &[(&str, &str)],
) -> Option<FluxOciRepositoryDep> {
    if value_at(scalars, &["apiVersion"]).is_none()
        || value_at(scalars, &["kind"]) != Some("OCIRepository")
    {
        return None;
    }

    let image = value_at(scalars, &["spec", "url"])?.strip_prefix("oci://")?;
    let package_name = apply_registry_alias(image, registry_aliases);
    let tag = value_at(scalars, &["spec", "ref", "tag"]);
    let digest = value_at(scalars, &["spec", "ref", "digest"]);
    if tag.is_none() && digest.is_none() && value_at(scalars, &["spec", "ref"]).is_some() {
        return None;
    }
    let tag_is_yaml_alias = tag.is_some_and(|tag| tag.starts_with('*'));
    let tag = if tag_is_yaml_alias {
        tag.and_then(|tag| resolve_yaml_alias(scalars, tag))
    } else {
        tag
    };

    let (current_value, current_digest, replace_string) = match (tag, digest) {
        (Some(tag), Some(digest)) => (
            Some(tag.to_owned()),
            Some(digest.to_owned()),
            Some(format!("digest: {digest}\n            tag: {tag}")),
        ),
        (Some(tag), None) => {
            if let Some((tag, digest)) = tag.split_once('@') {
                (
                    Some(tag.to_owned()),
                    Some(digest.to_owned()),
                    Some(format!("{tag}@{digest}")),
                )
            } else {
                (Some(tag.to_owned()), None, Some(tag.to_owned()))
            }
        }
        (None, Some(digest)) => (None, Some(digest.to_owned()), None),
        (None, None) => (None, None, None),
    };

    let skip_reason = if tag_is_yaml_alias {
        Some(FluxSkipReason::InvalidValue)
    } else if current_value.is_none() && current_digest.is_none() {
        Some(FluxSkipReason::UnversionedReference)
    } else {
        None
    };

    Some(FluxOciRepositoryDep {
        dep_name: image.to_owned(),
        datasource: DOCKER_DATASOURCE,
        current_value,
        current_digest,
        package_name,
        replace_string,
        skip_reason,
    })
}

fn extract_helm_release_image_doc(
    scalars: &[(Vec<String>, String)],
    registry_aliases: &[(&str, &str)],
) -> Option<FluxOciRepositoryDep> {
    if value_at(scalars, &["apiVersion"]).is_none()
        || value_at(scalars, &["kind"]) != Some("HelmRelease")
    {
        return None;
    }

    let image = value_at(scalars, &["spec", "values", "image", "repository"])?;
    let tag = value_at(scalars, &["spec", "values", "image", "tag"])?;
    Some(FluxOciRepositoryDep {
        dep_name: image.to_owned(),
        datasource: DOCKER_DATASOURCE,
        current_value: Some(tag.to_owned()),
        current_digest: None,
        package_name: apply_registry_alias(image, registry_aliases),
        replace_string: Some(tag.to_owned()),
        skip_reason: None,
    })
}

fn resolve_yaml_alias<'a>(
    scalars: &'a [(Vec<String>, String)],
    alias_reference: &str,
) -> Option<&'a str> {
    let alias = alias_reference.strip_prefix('*')?;
    let anchor = format!("&{alias} ");
    scalars
        .iter()
        .find_map(|(_, value)| value.strip_prefix(&anchor))
}

fn apply_registry_alias(image: &str, registry_aliases: &[(&str, &str)]) -> String {
    registry_aliases
        .iter()
        .filter(|(source, _)| image == *source || image.starts_with(&format!("{source}/")))
        .max_by_key(|(source, _)| source.len())
        .map_or_else(
            || image.to_owned(),
            |(source, replacement)| {
                let suffix = image.strip_prefix(source).unwrap_or_default();
                format!("{}{}", replacement.trim_end_matches('/'), suffix)
            },
        )
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
            apply_helm_repository(&mut dep, repository, chart, registry_aliases);
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
            apply_helm_repository(&mut dep, repository, chart, registry_aliases);
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

fn apply_helm_repository(
    dep: &mut FluxHelmReleaseDep,
    repository: &HelmRepository,
    chart: &str,
    registry_aliases: &[(&str, &str)],
) {
    dep.skip_reason = None;
    if let Some(oci_url) = repository.url.strip_prefix("oci://") {
        dep.datasource = Some(DOCKER_DATASOURCE);
        dep.package_name = Some(apply_registry_alias(
            &format!("{}/{}", oci_url.trim_end_matches('/'), chart),
            registry_aliases,
        ));
    } else {
        dep.registry_urls.push(repository.url.clone());
    }
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

    // Ported: "extracts multiple resources" — flux/extract.spec.ts line 27
    #[test]
    fn extracts_multiple_resources() {
        let deps = extract_package_file(MULTIDOC);
        assert_eq!(deps.len(), 4);
        assert!(matches!(&deps[0], FluxDep::Helm(dep)
                if dep.dep_name == "external-dns"
                    && dep.current_value.as_deref() == Some("1.7.0")
                    && dep.registry_urls == vec!["https://kubernetes-sigs.github.io/external-dns/"]));
        assert!(matches!(&deps[1], FluxDep::Oci(dep)
                if dep.dep_name == "k8s.gcr.io/external-dns/external-dns"
                    && dep.current_value.as_deref() == Some("v0.13.4")
                    && dep.package_name == "k8s.gcr.io/external-dns/external-dns"
                    && dep.replace_string.as_deref() == Some("v0.13.4")));
        assert!(matches!(&deps[2], FluxDep::Git(dep)
                if dep.dep_name == "renovate-repo"
                    && dep.datasource == Some(GITHUB_TAGS_DATASOURCE)
                    && dep.current_value.as_deref() == Some("v11.35.4")
                    && dep.package_name.as_deref() == Some("renovatebot/renovate")));
        assert!(matches!(&deps[3], FluxDep::Oci(dep)
                if dep.dep_name == "ghcr.io/kyverno/manifests/kyverno"
                    && dep.current_value.as_deref() == Some("v1.8.2")
                    && dep.package_name == "ghcr.io/kyverno/manifests/kyverno"));
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

    // Ported: "ignores HelmRelease resources using a chartRef targetting an OCIRepository" — flux/extract.spec.ts line 457
    #[test]
    fn ignores_release_chart_ref_and_extracts_oci_repository() {
        let content = format!(
            "{OCI_REPOSITORY}\n---\napiVersion: helm.toolkit.fluxcd.io/v2\nkind: HelmRelease\nmetadata:\n  name: kyverno-controller\n  namespace: kube-system\nspec:\n  chartRef:\n    kind: OCIRepository\n    name: kyverno-controller\n    namespace: kube-system\n"
        );
        let deps = extract_oci_repositories(&content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "ghcr.io/kyverno/manifests/kyverno");
        assert_eq!(deps[0].current_value.as_deref(), Some("v1.8.2"));
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

    // Ported: "ignores OCIRepository with no tag and no digest" — flux/extract.spec.ts line 834
    #[test]
    fn oci_repository_without_tag_or_digest_is_unversioned() {
        let deps = extract_oci_repositories(
            "apiVersion: source.toolkit.fluxcd.io/v1beta2\nkind: OCIRepository\nmetadata:\n  name: kyverno-controller\n  namespace: flux-system\nspec:\n  url: oci://ghcr.io/kyverno/manifests/kyverno\n",
        );
        assert_eq!(
            deps,
            vec![FluxOciRepositoryDep {
                dep_name: "ghcr.io/kyverno/manifests/kyverno".to_owned(),
                datasource: DOCKER_DATASOURCE,
                current_value: None,
                current_digest: None,
                package_name: "ghcr.io/kyverno/manifests/kyverno".to_owned(),
                replace_string: None,
                skip_reason: Some(FluxSkipReason::UnversionedReference),
            }]
        );
    }

    // Ported: "extracts OCIRepository with a tag" — flux/extract.spec.ts line 861
    #[test]
    fn extracts_oci_repository_with_tag() {
        let deps = extract_oci_repositories_with_registry_aliases(
            OCI_REPOSITORY,
            &[("ghcr.io", "ghcr.proxy.test/some/path")],
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value.as_deref(), Some("v1.8.2"));
        assert_eq!(deps[0].current_digest, None);
        assert_eq!(deps[0].dep_name, "ghcr.io/kyverno/manifests/kyverno");
        assert_eq!(
            deps[0].package_name,
            "ghcr.proxy.test/some/path/kyverno/manifests/kyverno"
        );
        assert_eq!(deps[0].replace_string.as_deref(), Some("v1.8.2"));
    }

    // Ported: "extracts OCIRepository with a digest" — flux/extract.spec.ts line 897
    #[test]
    fn extracts_oci_repository_with_digest() {
        let deps = extract_oci_repositories(
            "apiVersion: source.toolkit.fluxcd.io/v1beta2\nkind: OCIRepository\nmetadata:\n  name: kyverno-controller\n  namespace: flux-system\nspec:\n  ref:\n    digest: sha256:761c3189c482d0f1f0ad3735ca05c4c398cae201d2169f6645280c7b7b2ce6fc\n  url: oci://ghcr.io/kyverno/manifests/kyverno\n",
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("sha256:761c3189c482d0f1f0ad3735ca05c4c398cae201d2169f6645280c7b7b2ce6fc")
        );
        assert_eq!(deps[0].current_value, None);
    }

    // Ported: "extracts OCIRepository with a tag that contains a digest" — flux/extract.spec.ts line 925
    #[test]
    fn extracts_oci_repository_with_tag_containing_digest() {
        let deps = extract_oci_repositories(
            "apiVersion: source.toolkit.fluxcd.io/v1beta2\nkind: OCIRepository\nmetadata:\n  name: kyverno-controller\n  namespace: flux-system\nspec:\n  ref:\n    tag: v1.8.2@sha256:761c3189c482d0f1f0ad3735ca05c4c398cae201d2169f6645280c7b7b2ce6fc\n  url: oci://ghcr.io/kyverno/manifests/kyverno\n",
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value.as_deref(), Some("v1.8.2"));
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("sha256:761c3189c482d0f1f0ad3735ca05c4c398cae201d2169f6645280c7b7b2ce6fc")
        );
        assert_eq!(
            deps[0].replace_string.as_deref(),
            Some("v1.8.2@sha256:761c3189c482d0f1f0ad3735ca05c4c398cae201d2169f6645280c7b7b2ce6fc")
        );
    }

    // Ported: "extracts OCIRepository with a digest and tag" — flux/extract.spec.ts line 958
    #[test]
    fn extracts_oci_repository_with_digest_and_tag() {
        let deps = extract_oci_repositories(
            "apiVersion: source.toolkit.fluxcd.io/v1beta2\nkind: OCIRepository\nmetadata:\n  name: kyverno-controller\n  namespace: flux-system\nspec:\n  ref:\n    digest: sha256:761c3189c482d0f1f0ad3735ca05c4c398cae201d2169f6645280c7b7b2ce6fc\n    tag: v1.8.2\n  url: oci://ghcr.io/kyverno/manifests/kyverno\n",
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value.as_deref(), Some("v1.8.2"));
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("sha256:761c3189c482d0f1f0ad3735ca05c4c398cae201d2169f6645280c7b7b2ce6fc")
        );
        assert!(deps[0].replace_string.as_deref().is_some_and(|value| {
            value.contains("digest: sha256:") && value.contains("tag: v1.8.2")
        }));
    }

    // Ported: "extracts OCIRepository with quoted digest and tag" — flux/extract.spec.ts line 994
    #[test]
    fn extracts_oci_repository_with_quoted_digest_and_tag() {
        let deps = extract_oci_repositories(
            "apiVersion: source.toolkit.fluxcd.io/v1beta2\nkind: OCIRepository\nmetadata:\n  name: kyverno-controller\n  namespace: flux-system\nspec:\n  ref:\n    tag: \"v1.8.2\"\n    digest: \"sha256:761c3189c482d0f1f0ad3735ca05c4c398cae201d2169f6645280c7b7b2ce6fc\"\n  url: oci://ghcr.io/kyverno/manifests/kyverno\n",
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value.as_deref(), Some("v1.8.2"));
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("sha256:761c3189c482d0f1f0ad3735ca05c4c398cae201d2169f6645280c7b7b2ce6fc")
        );
    }

    // Ported: "extracts OCIRepository with quoted keys" — flux/extract.spec.ts line 1030
    #[test]
    fn extracts_oci_repository_with_quoted_keys() {
        let deps = extract_oci_repositories(
            "apiVersion: source.toolkit.fluxcd.io/v1beta2\nkind: OCIRepository\nmetadata:\n  name: kyverno-controller\nspec:\n  ref:\n    \"tag\": v1.8.2\n    \"digest\": sha256:761c3189c482d0f1f0ad3735ca05c4c398cae201d2169f6645280c7b7b2ce6fc\n  url: oci://ghcr.io/kyverno/manifests/kyverno\n",
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value.as_deref(), Some("v1.8.2"));
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("sha256:761c3189c482d0f1f0ad3735ca05c4c398cae201d2169f6645280c7b7b2ce6fc")
        );
    }

    // Ported: "extracts OCIRepository when ref key is quoted" — flux/extract.spec.ts line 1063
    #[test]
    fn extracts_oci_repository_with_quoted_ref_key() {
        let deps = extract_oci_repositories(
            "apiVersion: source.toolkit.fluxcd.io/v1beta2\nkind: OCIRepository\nmetadata:\n  name: kyverno-controller\nspec:\n  url: oci://ghcr.io/kyverno/manifests/kyverno\n  \"ref\":\n    tag: v1.8.2\n    digest: sha256:761c3189c482d0f1f0ad3735ca05c4c398cae201d2169f6645280c7b7b2ce6fc\n",
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value.as_deref(), Some("v1.8.2"));
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("sha256:761c3189c482d0f1f0ad3735ca05c4c398cae201d2169f6645280c7b7b2ce6fc")
        );
    }

    // Ported: "skips OCIRepository when tag value is a YAML alias" — flux/extract.spec.ts line 1098
    #[test]
    fn skips_oci_repository_when_tag_value_is_yaml_alias() {
        let deps = extract_oci_repositories(
            "x-tag: &mytag v1.8.2\napiVersion: source.toolkit.fluxcd.io/v1beta2\nkind: OCIRepository\nmetadata:\n  name: kyverno-controller\nspec:\n  url: oci://ghcr.io/kyverno/manifests/kyverno\n  ref:\n    tag: *mytag\n    digest: sha256:761c3189c482d0f1f0ad3735ca05c4c398cae201d2169f6645280c7b7b2ce6fc\n",
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value.as_deref(), Some("v1.8.2"));
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("sha256:761c3189c482d0f1f0ad3735ca05c4c398cae201d2169f6645280c7b7b2ce6fc")
        );
        assert_eq!(deps[0].skip_reason, Some(FluxSkipReason::InvalidValue));
    }

    // Ported: "extracts OCIRepository with tag and digest preceded by other document types" — flux/extract.spec.ts line 1129
    #[test]
    fn extracts_oci_repository_after_other_document_types() {
        let content = format!(
            "---\n---\napiVersion: source.toolkit.fluxcd.io/v1beta2\nkind: HelmRepository\nmetadata:\n  name: bitnami\n  namespace: flux-system\nspec:\n  url: https://charts.bitnami.com/bitnami\n---\napiVersion: source.toolkit.fluxcd.io/v1beta2\nkind: OCIRepository\nmetadata:\n  name: other-oci\n  namespace: flux-system\nspec:\n  url: oci://ghcr.io/other/repo\n  ref:\n    tag: v1.0.0\n---\n{OCI_REPOSITORY_WITH_DIGEST_AND_TAG}"
        );
        let deps = extract_oci_repositories(&content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_name, "ghcr.io/other/repo");
        assert_eq!(deps[0].current_value.as_deref(), Some("v1.0.0"));
        assert_eq!(deps[1].dep_name, "ghcr.io/kyverno/manifests/kyverno");
        assert_eq!(deps[1].current_value.as_deref(), Some("v1.8.2"));
        assert_eq!(
            deps[1].current_digest.as_deref(),
            Some("sha256:761c3189c482d0f1f0ad3735ca05c4c398cae201d2169f6645280c7b7b2ce6fc")
        );
    }

    // Ported: "extracts OCIRepository with tag and digest when preceded by same-named resource with scalar ref" — flux/extract.spec.ts line 1195
    #[test]
    fn extracts_oci_repository_after_same_name_scalar_ref() {
        let content = format!(
            "---\napiVersion: source.toolkit.fluxcd.io/v1beta2\nkind: OCIRepository\nmetadata:\n  name: kyverno-controller\n  namespace: flux-system\nspec:\n  url: oci://ghcr.io/kyverno/manifests/kyverno\n  ref: \"not-a-map\"\n---\n{OCI_REPOSITORY_WITH_DIGEST_AND_TAG}"
        );
        let deps = extract_oci_repositories(&content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value.as_deref(), Some("v1.8.2"));
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("sha256:761c3189c482d0f1f0ad3735ca05c4c398cae201d2169f6645280c7b7b2ce6fc")
        );
    }

    // Ported: "extracts OCIRepository with tag and digest when preceded by same-named resource with scalar spec" — flux/extract.spec.ts line 1241
    #[test]
    fn extracts_oci_repository_after_same_name_scalar_spec() {
        let content = format!(
            "---\napiVersion: source.toolkit.fluxcd.io/v1beta2\nkind: OCIRepository\nmetadata:\n  name: kyverno-controller\n  namespace: flux-system\nspec: \"not-a-map\"\n---\n{OCI_REPOSITORY_WITH_DIGEST_AND_TAG}"
        );
        let deps = extract_oci_repositories(&content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value.as_deref(), Some("v1.8.2"));
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("sha256:761c3189c482d0f1f0ad3735ca05c4c398cae201d2169f6645280c7b7b2ce6fc")
        );
    }

    // Ported: "extracts OCIRepository with tag and digest when ref contains a non-scalar key" — flux/extract.spec.ts line 1285
    #[test]
    fn extracts_oci_repository_when_ref_contains_non_scalar_key() {
        let deps = extract_oci_repositories(
            "apiVersion: source.toolkit.fluxcd.io/v1beta2\nkind: OCIRepository\nmetadata:\n  name: kyverno-controller\n  namespace: flux-system\nspec:\n  url: oci://ghcr.io/kyverno/manifests/kyverno\n  ref:\n    ? [seq-key]\n    : ignored\n    tag: v1.8.2\n    digest: sha256:761c3189c482d0f1f0ad3735ca05c4c398cae201d2169f6645280c7b7b2ce6fc\n",
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value.as_deref(), Some("v1.8.2"));
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("sha256:761c3189c482d0f1f0ad3735ca05c4c398cae201d2169f6645280c7b7b2ce6fc")
        );
    }

    // Ported: "extracts Kustomization" — flux/extract.spec.ts line 1323
    #[test]
    fn extracts_kustomization_images() {
        let deps = extract_kustomizations(
            "apiVersion: kustomize.toolkit.fluxcd.io/v1\nkind: Kustomization\nmetadata:\n  name: podinfo\n  namespace: flux-system\nspec:\n  images:\n  - name: podinfo\n    newName: my-registry/podinfo\n    newTag: v1\n  - name: podinfo\n    newTag: 1.8.0\n  - name: podinfo\n    newName: my-podinfo\n  - name: podinfo\n    digest: sha256:24a0c4b4a4c0eb97a1aabb8e29f18e917d05abfe1b7a7c07857230879ce7d3d3\n",
        );
        assert_eq!(
            deps,
            vec![
                FluxKustomizationImageDep {
                    dep_name: "my-registry/podinfo".to_owned(),
                    datasource: DOCKER_DATASOURCE,
                    current_value: Some("v1".to_owned()),
                    current_digest: None,
                    package_name: "my-registry/podinfo".to_owned(),
                    replace_string: Some("v1".to_owned()),
                },
                FluxKustomizationImageDep {
                    dep_name: "podinfo".to_owned(),
                    datasource: DOCKER_DATASOURCE,
                    current_value: Some("1.8.0".to_owned()),
                    current_digest: None,
                    package_name: "podinfo".to_owned(),
                    replace_string: Some("1.8.0".to_owned()),
                },
                FluxKustomizationImageDep {
                    dep_name: "my-podinfo".to_owned(),
                    datasource: DOCKER_DATASOURCE,
                    current_value: None,
                    current_digest: None,
                    package_name: "my-podinfo".to_owned(),
                    replace_string: Some("my-podinfo".to_owned()),
                },
                FluxKustomizationImageDep {
                    dep_name: "podinfo".to_owned(),
                    datasource: DOCKER_DATASOURCE,
                    current_value: None,
                    current_digest: Some(
                        "sha256:24a0c4b4a4c0eb97a1aabb8e29f18e917d05abfe1b7a7c07857230879ce7d3d3"
                            .to_owned()
                    ),
                    package_name: "podinfo".to_owned(),
                    replace_string: Some(
                        "sha256:24a0c4b4a4c0eb97a1aabb8e29f18e917d05abfe1b7a7c07857230879ce7d3d3"
                            .to_owned()
                    ),
                },
            ]
        );
    }

    // Ported: "ignores resources of an unknown kind" — flux/extract.spec.ts line 1389
    #[test]
    fn ignores_resources_of_unknown_kind() {
        let content = "kind: SomethingElse\napiVersion: helm.toolkit.fluxcd.io/v2beta1\n";
        assert!(extract_helm_releases(content).is_empty());
        assert!(extract_git_repositories(content).is_empty());
        assert!(extract_oci_repositories(content).is_empty());
        assert!(extract_kustomizations(content).is_empty());
    }

    // Ported: "ignores resources without a kind" — flux/extract.spec.ts line 1400
    #[test]
    fn ignores_resources_without_kind() {
        let content = "apiVersion: helm.toolkit.fluxcd.io/v2beta1";
        assert!(extract_helm_releases(content).is_empty());
        assert!(extract_git_repositories(content).is_empty());
        assert!(extract_oci_repositories(content).is_empty());
        assert!(extract_kustomizations(content).is_empty());
    }

    // Ported: "ignores bad manifests" — flux/extract.spec.ts line 1408
    #[test]
    fn ignores_bad_manifests() {
        let content = "\"bad YAML";
        assert!(extract_helm_releases(content).is_empty());
        assert!(extract_git_repositories(content).is_empty());
        assert!(extract_oci_repositories(content).is_empty());
        assert!(extract_kustomizations(content).is_empty());
    }

    // Ported: "ignores null resources" — flux/extract.spec.ts line 1413
    #[test]
    fn ignores_null_resources() {
        let content = "null";
        assert!(extract_helm_releases(content).is_empty());
        assert!(extract_git_repositories(content).is_empty());
        assert!(extract_oci_repositories(content).is_empty());
        assert!(extract_kustomizations(content).is_empty());
    }

    // Ported: "extracts multiple files" — flux/extract.spec.ts line 1420
    #[test]
    fn extract_all_package_files_extracts_multiple_files() {
        let files = [
            (
                "lib/modules/manager/flux/__fixtures__/helmRelease.yaml",
                Some(HELM_RELEASE),
            ),
            (
                "lib/modules/manager/flux/__fixtures__/helmSource.yaml",
                Some(HELM_REPOSITORY),
            ),
            (
                "lib/modules/manager/flux/__fixtures__/gitSource.yaml",
                Some(GIT_REPOSITORY),
            ),
            (
                "lib/modules/manager/flux/__fixtures__/ociSource.yaml",
                Some(OCI_REPOSITORY),
            ),
            (
                "lib/modules/manager/flux/__fixtures__/flux-system/gotk-components.yaml",
                Some(FLUX_SYSTEM_MANIFEST),
            ),
        ];
        let result = extract_all_package_files(&files);
        assert_eq!(result.len(), 4);
        assert_eq!(
            result[0].package_file,
            "lib/modules/manager/flux/__fixtures__/helmRelease.yaml"
        );
        assert!(
            matches!(&result[0].deps[0], FluxDep::Helm(dep) if dep.registry_urls == vec!["https://bitnami-labs.github.io/sealed-secrets"])
        );
        assert!(
            matches!(&result[1].deps[0], FluxDep::Git(dep) if dep.current_value.as_deref() == Some("v11.35.4"))
        );
        assert!(
            matches!(&result[2].deps[0], FluxDep::Oci(dep) if dep.current_value.as_deref() == Some("v1.8.2"))
        );
        assert!(matches!(&result[3].deps[0], FluxDep::System(dep) if dep.version == "v0.24.1"));
    }

    // Ported: "ignores files that do not exist" — flux/extract.spec.ts line 1535
    #[test]
    fn extract_all_package_files_ignores_missing_files() {
        let files = [("lib/modules/manager/flux/__fixtures__/bogus.yaml", None)];
        assert!(extract_all_package_files(&files).is_empty());
    }

    // Ported: "ignores system manifest files without valid Flux version header" — flux/extract.spec.ts line 1542
    #[test]
    fn extract_all_package_files_ignores_invalid_system_manifest() {
        let files = [(
            "lib/modules/manager/flux/__fixtures__/flux-system-invalid/gotk-components.yaml",
            Some("not actually a system manifest!"),
        )];
        assert!(extract_all_package_files(&files).is_empty());
    }

    // Ported: "should handle HelmRepository with type OCI" — flux/extract.spec.ts line 1486
    #[test]
    fn extract_all_package_files_handles_helm_repository_type_oci() {
        let files = [
            (
                "lib/modules/manager/flux/__fixtures__/helmOCISource.yaml",
                Some(HELM_OCI_REPOSITORY),
            ),
            (
                "lib/modules/manager/flux/__fixtures__/helmOCIRelease.yaml",
                Some(HELM_OCI_RELEASE),
            ),
        ];
        let result = extract_all_package_files_with_registry_aliases(
            &files,
            &[("ghcr.io", "ghcr.proxy.test/some/path")],
        );
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0].package_file,
            "lib/modules/manager/flux/__fixtures__/helmOCIRelease.yaml"
        );
        assert!(matches!(&result[0].deps[0], FluxDep::Helm(dep)
                if dep.datasource == Some(DOCKER_DATASOURCE)
                    && dep.dep_name == "actions-runner-controller-charts/gha-runner-scale-set"
                    && dep.current_value.as_deref() == Some("0.4.0")
                    && dep.package_name.as_deref() == Some("ghcr.proxy.test/some/path/actions/actions-runner-controller-charts/gha-runner-scale-set")));
    }

    // Ported: "should handle HelmRepository w/o type oci and url starts with oci" — flux/extract.spec.ts line 1514
    #[test]
    fn extract_all_package_files_handles_helm_repository_oci_url_without_type() {
        let files = [
            (
                "lib/modules/manager/flux/__fixtures__/helmOCISource2.yaml",
                Some(HELM_OCI_REPOSITORY_WITHOUT_TYPE),
            ),
            (
                "lib/modules/manager/flux/__fixtures__/helmOCIRelease2.yaml",
                Some(HELM_OCI_RELEASE_WITHOUT_TYPE),
            ),
        ];
        let result = extract_all_package_files(&files);
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0].package_file,
            "lib/modules/manager/flux/__fixtures__/helmOCIRelease2.yaml"
        );
        assert!(matches!(&result[0].deps[0], FluxDep::Helm(dep)
                if dep.datasource == Some(DOCKER_DATASOURCE)
                    && dep.dep_name == "kyverno"
                    && dep.current_value.as_deref() == Some("2.6.0")
                    && dep.package_name.as_deref() == Some("ghcr.io/kyverno/charts/kyverno")));
    }

    // Ported: "should pick correct package file when using HelmRepository with chartRef" — flux/extract.spec.ts line 1549
    #[test]
    fn extract_all_package_files_picks_helm_chart_package_file_for_chart_ref() {
        let files = [
            (
                "lib/modules/manager/flux/__fixtures__/helmChartRefRelease.yaml",
                Some(HELM_CHART_REF_RELEASE),
            ),
            (
                "lib/modules/manager/flux/__fixtures__/helmChart.yaml",
                Some(HELM_CHART),
            ),
            (
                "lib/modules/manager/flux/__fixtures__/helmSource.yaml",
                Some(HELM_REPOSITORY),
            ),
        ];
        let result = extract_all_package_files(&files);
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0].package_file,
            "lib/modules/manager/flux/__fixtures__/helmChart.yaml"
        );
        assert!(
            matches!(&result[0].deps[0], FluxDep::Helm(dep) if dep.dep_name == "sealed-secrets" && dep.skip_reason.is_none())
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

    const FLUX_SYSTEM_MANIFEST: &str = r#"
# Flux Version: v0.24.1
# Components: source-controller,kustomize-controller,helm-controller,notification-controller
apiVersion: v1
kind: Namespace
metadata:
  name: flux-system
"#;

    const MULTIDOC: &str = r#"
apiVersion: helm.toolkit.fluxcd.io/v2beta1
kind: HelmRelease
metadata:
  name: external-dns
  namespace: kube-system
spec:
  releaseName: external-dns
  chart:
    spec:
      chart: external-dns
      sourceRef:
        kind: HelmRepository
        name: external-dns
      version: "1.7.0"
  interval: 1h0m0s
  values:
    image:
      repository: k8s.gcr.io/external-dns/external-dns
      tag: v0.13.4
---
apiVersion: source.toolkit.fluxcd.io/v1beta1
kind: HelmRepository
metadata:
  name: external-dns
  namespace: kube-system
spec:
  interval: 1h0m0s
  url: https://kubernetes-sigs.github.io/external-dns/
---
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
---
apiVersion: source.toolkit.fluxcd.io/v1beta2
kind: OCIRepository
metadata:
  name: kyverno-controller
  namespace: flux-system
spec:
  interval: 1h0m0s
  provider: generic
  url: oci://ghcr.io/kyverno/manifests/kyverno
  ref:
    tag: v1.8.2
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

    const HELM_OCI_REPOSITORY: &str = r#"
apiVersion: source.toolkit.fluxcd.io/v1beta2
kind: HelmRepository
metadata:
  name: actions-runner-controller
  namespace: flux-system
spec:
  type: oci
  interval: 30m
  url: oci://ghcr.io/actions
  timeout: 3m
"#;

    const HELM_OCI_RELEASE: &str = r#"
apiVersion: helm.toolkit.fluxcd.io/v2beta1
kind: HelmRelease
metadata:
  name: arc-assets
  namespace: dev
spec:
  interval: 30m
  chart:
    spec:
      chart: actions-runner-controller-charts/gha-runner-scale-set
      version: 0.4.0
      sourceRef:
        kind: HelmRepository
        name: actions-runner-controller
        namespace: flux-system
      interval: 30m
"#;

    const HELM_OCI_REPOSITORY_WITHOUT_TYPE: &str = r#"
apiVersion: source.toolkit.fluxcd.io/v1beta2
kind: HelmRepository
metadata:
  name: kyverno
  namespace: flux-system
spec:
  interval: 6h
  url: oci://ghcr.io/kyverno/charts
"#;

    const HELM_OCI_RELEASE_WITHOUT_TYPE: &str = r#"
apiVersion: helm.toolkit.fluxcd.io/v2beta1
kind: HelmRelease
metadata:
  name: kyverno
  namespace: flux-system
spec:
  interval: 6h
  releaseName: kyverno
  targetNamespace: kyverno
  install:
    createNamespace: true
  chart:
    spec:
      chart: kyverno
      version: 2.6.0
      interval: 6h
      sourceRef:
        kind: HelmRepository
        name: kyverno
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

    const OCI_REPOSITORY: &str = r#"
apiVersion: source.toolkit.fluxcd.io/v1beta2
kind: OCIRepository
metadata:
  name: kyverno-controller
  namespace: flux-system
spec:
  interval: 1h0m0s
  provider: generic
  url: oci://ghcr.io/kyverno/manifests/kyverno
  ref:
    tag: v1.8.2
"#;

    const OCI_REPOSITORY_WITH_DIGEST_AND_TAG: &str = r#"
apiVersion: source.toolkit.fluxcd.io/v1beta2
kind: OCIRepository
metadata:
  name: kyverno-controller
  namespace: flux-system
spec:
  url: oci://ghcr.io/kyverno/manifests/kyverno
  ref:
    tag: v1.8.2
    digest: sha256:761c3189c482d0f1f0ad3735ca05c4c398cae201d2169f6645280c7b7b2ce6fc
"#;
}
