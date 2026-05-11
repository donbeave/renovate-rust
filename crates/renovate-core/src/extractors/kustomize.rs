//! Kustomize `kustomization.yaml` dependency extractor.
//!
//! Parses `kustomization.yaml` files and extracts Docker image references from
//! the `images:` section and Helm chart references from the `helmCharts:`
//! section.
//!
//! Renovate reference:
//! - `lib/modules/manager/kustomize/extract.ts` — `extractImage`, `extractHelmChart`
//! - `lib/modules/manager/kustomize/index.ts` — pattern `/(^|/)kustomization\.ya?ml$/`
//!
//! ## Supported forms
//!
//! ```yaml
//! images:
//!   - name: nginx              # image name for lookup
//!     newTag: 1.19.0           # version
//!   - name: myapp
//!     newName: registry.io/myapp  # overrides lookup name
//!     newTag: v2.1.0
//!
//! helmCharts:
//!   - name: podinfo
//!     repo: https://stefanprodan.github.io/podinfo
//!     version: 6.5.0
//! ```

/// A remote Kustomize base/resource/component reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KustomizeResourceDep {
    pub dep_name: String,
    pub package_name: Option<String>,
    pub current_value: String,
    pub datasource: &'static str,
}

/// Minimal parsed Kustomize file metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedKustomize {
    pub kind: String,
    pub chart_home: Option<String>,
}

/// A Helm chart reference from a kustomize `helmCharts:` entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KustomizeHelmDep {
    /// Chart name (e.g. `"podinfo"`).
    pub chart_name: String,
    /// Helm repository URL.
    pub repository_url: String,
    /// Chart version.
    pub current_value: String,
}

/// An OCI Helm chart reference from a kustomize `helmCharts:` entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KustomizeOciHelmDep {
    /// Chart name (e.g. `"redis"`).
    pub chart_name: String,
    /// OCI image package name (e.g. `"registry-1.docker.io/bitnamicharts/redis"`).
    pub package_name: String,
    /// Chart version.
    pub current_value: String,
}

/// OCI Helm chart metadata after applying registry aliases.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KustomizeResolvedOciHelmDep {
    pub dep_name: String,
    pub package_name: String,
    pub current_value: String,
    pub datasource: &'static str,
    pub pin_digests: bool,
}

/// Docker image metadata after applying registry aliases.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KustomizeResolvedImageDep {
    pub dep_name: String,
    pub package_name: String,
    pub current_value: Option<String>,
    pub current_digest: Option<String>,
    pub replace_string: String,
    pub auto_replace_string_template: String,
    pub datasource: &'static str,
}

/// Why a Kustomize image entry is skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KustomizeImageSkipReason {
    InvalidValue,
    InvalidDependencySpecification,
}

/// Docker image from a kustomize `images:` entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KustomizeImageDep {
    pub image: String,
    pub tag: Option<String>,
    pub digest: Option<String>,
    pub skip_reason: Option<KustomizeImageSkipReason>,
    pub replace_string: String,
}

/// A single dependency extracted from a `kustomization.yaml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KustomizeDep {
    /// Docker image from `images:` section.
    Image(KustomizeImageDep),
    /// Helm chart from `helmCharts:` section.
    Helm(KustomizeHelmDep),
    /// OCI Helm chart from `helmCharts:` section.
    OciHelm(KustomizeOciHelmDep),
    /// Remote base/resource/component Git reference.
    Resource(KustomizeResourceDep),
}

/// Extract dependencies from a `kustomization.yaml` file.
pub fn extract(content: &str) -> Vec<KustomizeDep> {
    let mut out = Vec::new();

    #[derive(Clone, Copy)]
    enum State {
        Default,
        InImages,
        InHelmCharts,
        InResources,
    }

    let mut state = State::Default;
    // Current image entry being assembled.
    let mut img_name: Option<String> = None;
    let mut img_new_name: Option<String> = None;
    let mut img_new_tag: Option<String> = None;
    let mut img_new_tag_quoted = false;
    let mut img_digest: Option<String> = None;
    // Current helm entry being assembled.
    let mut helm_name: Option<String> = None;
    let mut helm_repo: Option<String> = None;
    let mut helm_version: Option<String> = None;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        if line.trim().is_empty() {
            continue;
        }
        let trimmed = line.trim_start();
        let indent = leading_spaces(line);

        match state {
            State::Default => {
                if trimmed == "images:" {
                    state = State::InImages;
                } else if trimmed == "helmCharts:" {
                    state = State::InHelmCharts;
                } else if matches!(trimmed, "bases:" | "resources:" | "components:") {
                    state = State::InResources;
                }
            }
            State::InImages => {
                if indent == 0 && !trimmed.starts_with('-') {
                    // Exited images section — flush last entry
                    flush_image(
                        &mut img_name,
                        &mut img_new_name,
                        &mut img_new_tag,
                        &mut img_new_tag_quoted,
                        &mut img_digest,
                        &mut out,
                    );
                    state = State::Default;
                    if trimmed == "helmCharts:" {
                        state = State::InHelmCharts;
                    } else if matches!(trimmed, "bases:" | "resources:" | "components:") {
                        state = State::InResources;
                    }
                    continue;
                }
                // New list item
                if let Some(rest) = trimmed.strip_prefix("- ") {
                    flush_image(
                        &mut img_name,
                        &mut img_new_name,
                        &mut img_new_tag,
                        &mut img_new_tag_quoted,
                        &mut img_digest,
                        &mut out,
                    );
                    if let Some(val) = strip_key(rest, "name") {
                        img_name = Some(trim_yaml_scalar(val).0);
                    } else if let Some(val) = strip_key(rest, "newName") {
                        img_new_name = Some(trim_yaml_scalar(val).0);
                    } else if let Some(val) = strip_key(rest, "newTag") {
                        let (tag, quoted) = trim_yaml_scalar(val);
                        img_new_tag = Some(tag);
                        img_new_tag_quoted = quoted;
                    } else if let Some(val) = strip_key(rest, "digest") {
                        img_digest = Some(trim_yaml_scalar(val).0);
                    }
                } else if let Some(val) = strip_key(trimmed, "name") {
                    img_name = Some(trim_yaml_scalar(val).0);
                } else if let Some(val) = strip_key(trimmed, "newName") {
                    img_new_name = Some(trim_yaml_scalar(val).0);
                } else if let Some(val) = strip_key(trimmed, "newTag") {
                    let (tag, quoted) = trim_yaml_scalar(val);
                    img_new_tag = Some(tag);
                    img_new_tag_quoted = quoted;
                } else if let Some(val) = strip_key(trimmed, "digest") {
                    img_digest = Some(trim_yaml_scalar(val).0);
                }
            }
            State::InHelmCharts => {
                if indent == 0 && !trimmed.starts_with('-') {
                    flush_helm(&mut helm_name, &mut helm_repo, &mut helm_version, &mut out);
                    state = State::Default;
                    if trimmed == "images:" {
                        state = State::InImages;
                    } else if matches!(trimmed, "bases:" | "resources:" | "components:") {
                        state = State::InResources;
                    }
                    continue;
                }
                if let Some(rest) = trimmed.strip_prefix("- ") {
                    flush_helm(&mut helm_name, &mut helm_repo, &mut helm_version, &mut out);
                    if let Some(val) = strip_key(rest, "name") {
                        helm_name =
                            Some(val.trim().trim_matches('"').trim_matches('\'').to_owned());
                    }
                } else if let Some(val) = strip_key(trimmed, "name") {
                    helm_name = Some(val.trim().trim_matches('"').trim_matches('\'').to_owned());
                } else if let Some(val) = strip_key(trimmed, "repo") {
                    helm_repo = Some(val.trim().trim_matches('"').trim_matches('\'').to_owned());
                } else if let Some(val) = strip_key(trimmed, "version") {
                    helm_version = Some(val.trim().trim_matches('"').trim_matches('\'').to_owned());
                }
            }
            State::InResources => {
                if indent == 0 && !trimmed.starts_with('-') {
                    state = State::Default;
                    if trimmed == "images:" {
                        state = State::InImages;
                    } else if trimmed == "helmCharts:" {
                        state = State::InHelmCharts;
                    } else if matches!(trimmed, "bases:" | "resources:" | "components:") {
                        state = State::InResources;
                    }
                    continue;
                }
                if let Some(rest) = trimmed.strip_prefix("- ")
                    && let Some(dep) =
                        extract_resource(rest.trim().trim_matches('"').trim_matches('\''))
                {
                    out.push(KustomizeDep::Resource(dep));
                }
            }
        }
    }

    // Flush trailing entries.
    flush_image(
        &mut img_name,
        &mut img_new_name,
        &mut img_new_tag,
        &mut img_new_tag_quoted,
        &mut img_digest,
        &mut out,
    );
    flush_helm(&mut helm_name, &mut helm_repo, &mut helm_version, &mut out);

    out
}

pub fn resolve_oci_helm_dep(
    dep: KustomizeOciHelmDep,
    registry_aliases: &[(&str, &str)],
) -> KustomizeResolvedOciHelmDep {
    KustomizeResolvedOciHelmDep {
        dep_name: dep.chart_name,
        package_name: apply_registry_alias(&dep.package_name, registry_aliases),
        current_value: dep.current_value,
        datasource: "docker",
        pin_digests: false,
    }
}

pub fn resolve_image_dep(
    dep: KustomizeImageDep,
    registry_aliases: &[(&str, &str)],
) -> KustomizeResolvedImageDep {
    KustomizeResolvedImageDep {
        package_name: apply_registry_alias(&dep.image, registry_aliases),
        dep_name: dep.image,
        current_value: dep.tag,
        current_digest: dep.digest,
        replace_string: dep.replace_string,
        auto_replace_string_template: "{{newValue}}{{#if newDigest}}@{{newDigest}}{{/if}}"
            .to_owned(),
        datasource: "docker",
    }
}

fn apply_registry_alias(value: &str, registry_aliases: &[(&str, &str)]) -> String {
    registry_aliases
        .iter()
        .filter_map(|(from, to)| {
            value
                .strip_prefix(*from)
                .filter(|rest| rest.is_empty() || rest.starts_with('/'))
                .map(|rest| (from.len(), format!("{to}{rest}")))
        })
        .max_by_key(|(len, _)| *len)
        .map(|(_, aliased)| aliased)
        .unwrap_or_else(|| value.to_owned())
}

/// Parse Kustomize metadata needed by the extractor.
pub fn parse_kustomize(content: &str) -> Option<ParsedKustomize> {
    if content.trim().is_empty() {
        return None;
    }

    let mut kind = None;
    let mut chart_home = None;
    let mut in_helm_globals = false;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        if line.trim().is_empty() {
            continue;
        }
        let trimmed = line.trim_start();
        let indent = leading_spaces(line);

        if indent == 0 {
            in_helm_globals = trimmed == "helmGlobals:";
        }

        if let Some(val) = strip_key(trimmed, "kind") {
            kind = Some(val.trim().trim_matches('"').trim_matches('\'').to_owned());
        } else if in_helm_globals && let Some(val) = strip_key(trimmed, "chartHome") {
            chart_home = Some(val.trim().trim_matches('"').trim_matches('\'').to_owned());
        }
    }

    let kind = kind.unwrap_or_else(|| "Kustomization".to_owned());
    if !matches!(kind.as_str(), "Kustomization" | "Component") {
        return None;
    }

    Some(ParsedKustomize { kind, chart_home })
}

/// Extract a remote Kustomize base/resource/component reference.
pub fn extract_resource(raw: &str) -> Option<KustomizeResourceDep> {
    let raw = raw.trim();
    if raw.is_empty() || raw.starts_with("./") || raw.starts_with("../") || !raw.contains('?') {
        return None;
    }

    let (base, query) = raw.split_once('?')?;
    let current_value =
        first_query_value(query, "ref").or_else(|| first_query_value(query, "version"))?;
    if current_value.is_empty() {
        return None;
    }

    if let Some(rest) = base.strip_prefix("https://github.com/") {
        let dep_name = github_dep_name(rest)?;
        return Some(KustomizeResourceDep {
            dep_name,
            package_name: Some(format!(
                "https://{}",
                repo_base(&format!("github.com/{rest}"))?
            )),
            current_value,
            datasource: "github-tags",
        });
    }

    if let Some(rest) = base.strip_prefix("github.com/") {
        let dep_name = github_dep_name(rest)?;
        return Some(KustomizeResourceDep {
            dep_name,
            package_name: None,
            current_value,
            datasource: "github-tags",
        });
    }

    if let Some(rest) = base.strip_prefix("git@github.com:") {
        let dep_name = github_dep_name(rest)?;
        return Some(KustomizeResourceDep {
            dep_name,
            package_name: None,
            current_value,
            datasource: "github-tags",
        });
    }

    if let Some(rest) = base.strip_prefix("ssh://git@") {
        let package_name = format!("ssh://git@{}", repo_base(rest)?);
        let dep_name = dep_name_from_remote_path(rest)?;
        return Some(KustomizeResourceDep {
            dep_name,
            package_name: Some(package_name),
            current_value,
            datasource: "git-tags",
        });
    }

    if let Some(rest) = base.strip_prefix("https://") {
        let package_name = format!("https://{}", repo_base(rest)?);
        let dep_name = dep_name_from_remote_path(rest)?;
        return Some(KustomizeResourceDep {
            dep_name,
            package_name: Some(package_name),
            current_value,
            datasource: "git-tags",
        });
    }

    None
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn first_query_value(query: &str, key: &str) -> Option<String> {
    query.split('&').find_map(|part| {
        let (candidate, value) = part.split_once('=')?;
        (candidate == key).then(|| value.to_owned())
    })
}

fn github_dep_name(path: &str) -> Option<String> {
    let clean = repo_base(&format!("github.com/{path}"))?;
    let clean = clean.strip_prefix("github.com/").unwrap_or(clean.as_str());
    let clean = clean.strip_suffix(".git").unwrap_or(clean);
    let mut parts = clean.split('/');
    let owner = parts.next()?;
    let repo = parts.next()?;
    if owner.is_empty() || repo.is_empty() {
        return None;
    }
    Some(format!("{owner}/{repo}"))
}

fn dep_name_from_remote_path(path: &str) -> Option<String> {
    let clean = repo_base(path)?;
    let clean = clean.strip_suffix(".git").unwrap_or(clean.as_str());
    let parts = clean.split('/').collect::<Vec<_>>();
    match parts.as_slice() {
        [owner, repo] if !owner.is_empty() && !repo.is_empty() => Some(format!("{owner}/{repo}")),
        [host, owner, repo, ..] if !host.is_empty() && !owner.is_empty() && !repo.is_empty() => {
            Some(format!("{host}/{owner}/{repo}"))
        }
        _ => None,
    }
}

fn repo_base(path: &str) -> Option<String> {
    let without_subdir = path.split("//").next().unwrap_or(path);
    if let Some((before_git, _)) = without_subdir.split_once(".git") {
        return Some(format!("{before_git}.git"));
    }
    let parts = without_subdir.split('/').take(3).collect::<Vec<_>>();
    if parts.len() < 2 {
        return None;
    }
    Some(parts.join("/"))
}

fn leading_spaces(s: &str) -> usize {
    s.len() - s.trim_start_matches([' ', '\t']).len()
}

fn strip_key<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}:");
    line.strip_prefix(prefix.as_str())
}

fn trim_yaml_scalar(value: &str) -> (String, bool) {
    let value = value.trim();
    let quoted = (value.starts_with('"') && value.ends_with('"'))
        || (value.starts_with('\'') && value.ends_with('\''));
    (
        value.trim_matches('"').trim_matches('\'').to_owned(),
        quoted,
    )
}

fn flush_image(
    name: &mut Option<String>,
    new_name: &mut Option<String>,
    new_tag: &mut Option<String>,
    new_tag_quoted: &mut bool,
    digest: &mut Option<String>,
    out: &mut Vec<KustomizeDep>,
) {
    let Some(n) = name.take() else {
        new_name.take();
        new_tag.take();
        *new_tag_quoted = false;
        digest.take();
        return;
    };
    let lookup_name = new_name.take().unwrap_or(n);
    let tag = new_tag.take();
    let digest_value = digest.take();
    let was_quoted = *new_tag_quoted;
    *new_tag_quoted = false;
    if lookup_name.is_empty() {
        return;
    }
    if let Some(dep) = kustomize_image_dep(lookup_name, tag, was_quoted, digest_value) {
        out.push(KustomizeDep::Image(dep));
    }
}

fn kustomize_image_dep(
    image: String,
    tag: Option<String>,
    tag_quoted: bool,
    digest: Option<String>,
) -> Option<KustomizeImageDep> {
    if tag.is_some() && digest.is_some() {
        return Some(KustomizeImageDep {
            image,
            tag: None,
            digest,
            skip_reason: Some(KustomizeImageSkipReason::InvalidDependencySpecification),
            replace_string: String::new(),
        });
    }

    if let Some(digest) = digest {
        if !is_valid_digest(&digest) {
            return Some(KustomizeImageDep {
                image,
                tag: None,
                digest: None,
                skip_reason: Some(KustomizeImageSkipReason::InvalidValue),
                replace_string: digest,
            });
        }
        let (image, current_value) = split_image_tag(&image);
        return Some(KustomizeImageDep {
            image: image.to_owned(),
            tag: (!current_value.is_empty()).then(|| current_value.to_owned()),
            digest: Some(digest.clone()),
            skip_reason: None,
            replace_string: digest,
        });
    }

    let tag = tag?;

    if !tag_quoted && looks_like_yaml_number(&tag) {
        return Some(KustomizeImageDep {
            image,
            tag: Some(tag.clone()),
            digest: None,
            skip_reason: Some(KustomizeImageSkipReason::InvalidValue),
            replace_string: tag,
        });
    }

    if let Some((tag, digest)) = tag.split_once('@')
        && is_valid_digest(digest)
    {
        return Some(KustomizeImageDep {
            image,
            tag: Some(tag.to_owned()),
            digest: Some(digest.to_owned()),
            skip_reason: None,
            replace_string: format!("{tag}@{digest}"),
        });
    }

    if tag.starts_with("sha256:") || tag.starts_with("sha512:") {
        return Some(KustomizeImageDep {
            image,
            tag: Some(tag.clone()),
            digest: None,
            skip_reason: Some(KustomizeImageSkipReason::InvalidValue),
            replace_string: tag,
        });
    }

    Some(KustomizeImageDep {
        image,
        tag: Some(tag.clone()),
        digest: None,
        skip_reason: None,
        replace_string: tag,
    })
}

fn is_valid_digest(value: &str) -> bool {
    value.starts_with("sha256:") || value.starts_with("sha512:")
}

fn looks_like_yaml_number(value: &str) -> bool {
    let value = value.trim_start_matches(['-', '+']);
    let mut dot_count = 0;
    !value.is_empty()
        && value.chars().all(|ch| {
            if ch == '.' {
                dot_count += 1;
                true
            } else {
                ch.is_ascii_digit()
            }
        })
        && dot_count == 1
        && value.chars().any(|ch| ch.is_ascii_digit())
}

fn split_image_tag(s: &str) -> (&str, &str) {
    if let Some(pos) = s.rfind(':') {
        let tag = &s[pos + 1..];
        let name = &s[..pos];
        if !tag.contains('/') {
            return (name, tag);
        }
    }
    (s, "")
}

fn flush_helm(
    name: &mut Option<String>,
    repo: &mut Option<String>,
    version: &mut Option<String>,
    out: &mut Vec<KustomizeDep>,
) {
    let Some(chart_name) = name.take() else {
        repo.take();
        version.take();
        return;
    };
    let repository_url = repo.take().unwrap_or_default();
    let current_value = version.take().unwrap_or_default();
    if chart_name.is_empty() || current_value.is_empty() {
        return;
    }
    if let Some(oci_repo) = repository_url.strip_prefix("oci://") {
        out.push(KustomizeDep::OciHelm(KustomizeOciHelmDep {
            package_name: format!("{}/{}", oci_repo.trim_end_matches('/'), chart_name),
            chart_name,
            current_value,
        }));
        return;
    }
    out.push(KustomizeDep::Helm(KustomizeHelmDep {
        chart_name,
        repository_url,
        current_value,
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "should correctly extract a default image" — kustomize/extract.spec.ts line 287
    #[test]
    fn extracts_images() {
        let content = r#"
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization

images:
  - name: nginx
    newTag: 1.19.0
  - name: myapp
    newName: registry.example.com/myapp
    newTag: v2.1.0
"#;
        let deps = extract(content);
        let images: Vec<_> = deps
            .iter()
            .filter_map(|d| {
                if let KustomizeDep::Image(i) = d {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(images.len(), 2);
        assert!(
            images
                .iter()
                .any(|i| i.image == "nginx" && i.tag.as_deref() == Some("1.19.0"))
        );
        assert!(
            images
                .iter()
                .any(|i| i.image == "registry.example.com/myapp"
                    && i.tag.as_deref() == Some("v2.1.0"))
        );
    }

    // Ported: "should return null when header has invalid resource kind" — kustomize/extract.spec.ts line 38
    #[test]
    fn invalid_resource_kind_returns_none() {
        let parsed = parse_kustomize(
            r#"
kind: NoKustomization
bases:
- github.com/fluxcd/flux/deploy?ref=1.19.0
"#,
        );
        assert!(parsed.is_none());
    }

    // Ported: "should fall back to default resource kind when header is missing" — kustomize/extract.spec.ts line 47
    #[test]
    fn missing_kind_defaults_to_kustomization() {
        let parsed = parse_kustomize(
            r#"
bases:
- github.com/fluxcd/flux/deploy?ref=1.19.0
"#,
        )
        .expect("kustomization should parse");
        assert_eq!(parsed.kind, "Kustomization");
    }

    // Ported: "should extract chartHome" — kustomize/extract.spec.ts line 56
    #[test]
    fn extracts_chart_home() {
        let parsed = parse_kustomize(
            r#"
helmGlobals:
  chartHome: customPathToCharts
"#,
        )
        .expect("kustomization should parse");
        assert_eq!(parsed.chart_home.as_deref(), Some("customPathToCharts"));
    }

    // Ported: "should correctly extract an image in a repo" — kustomize/extract.spec.ts line 305
    #[test]
    fn extracts_image_in_repo() {
        let deps = extract(
            r#"
images:
  - name: test/node
    newTag: v1.0.0
"#,
        );
        assert_eq!(deps.len(), 1);
        let KustomizeDep::Image(image) = &deps[0] else {
            panic!("expected image dependency");
        };
        assert_eq!(image.image, "test/node");
        assert_eq!(image.tag.as_deref(), Some("v1.0.0"));
        assert!(image.skip_reason.is_none());
    }

    // Ported: "should correctly extract from a different registry" — kustomize/extract.spec.ts line 323
    #[test]
    fn extracts_image_from_different_registry() {
        let deps = extract(
            r#"
images:
  - name: quay.io/repo/image
    newTag: v1.0.0
"#,
        );
        assert_eq!(deps.len(), 1);
        let KustomizeDep::Image(image) = &deps[0] else {
            panic!("expected image dependency");
        };
        assert_eq!(image.image, "quay.io/repo/image");
        assert_eq!(image.tag.as_deref(), Some("v1.0.0"));
        assert!(image.skip_reason.is_none());
    }

    // Ported: "should correctly extract from a different port" — kustomize/extract.spec.ts line 341
    #[test]
    fn extracts_image_from_registry_with_port() {
        let deps = extract(
            r#"
images:
  - name: localhost:5000/repo/image
    newTag: v1.0.0
"#,
        );
        assert_eq!(deps.len(), 1);
        let KustomizeDep::Image(image) = &deps[0] else {
            panic!("expected image dependency");
        };
        assert_eq!(image.image, "localhost:5000/repo/image");
        assert_eq!(image.tag.as_deref(), Some("v1.0.0"));
        assert!(image.skip_reason.is_none());
    }

    // Ported: "should correctly extract from a multi-depth registry" — kustomize/extract.spec.ts line 359
    #[test]
    fn extracts_image_from_multi_depth_registry() {
        let deps = extract(
            r#"
images:
  - name: localhost:5000/repo/image/service
    newTag: v1.0.0
"#,
        );
        assert_eq!(deps.len(), 1);
        let KustomizeDep::Image(image) = &deps[0] else {
            panic!("expected image dependency");
        };
        assert_eq!(image.image, "localhost:5000/repo/image/service");
        assert_eq!(image.tag.as_deref(), Some("v1.0.0"));
        assert!(image.skip_reason.is_none());
    }

    // Ported: "should correctly extract with registryAliases" — kustomize/extract.spec.ts line 377
    #[test]
    fn extracts_image_with_registry_aliases() {
        let deps = extract(
            r#"
images:
  - name: localhost:5000/repo/image/service
    newTag: v1.0.0
"#,
        );
        assert_eq!(deps.len(), 1);
        let KustomizeDep::Image(image) = &deps[0] else {
            panic!("expected image dependency");
        };
        let resolved = resolve_image_dep(image.clone(), &[("localhost:5000/repo", "docker.io")]);
        assert_eq!(resolved.dep_name, "localhost:5000/repo/image/service");
        assert_eq!(resolved.package_name, "docker.io/image/service");
        assert_eq!(resolved.current_value.as_deref(), Some("v1.0.0"));
        assert_eq!(resolved.current_digest, None);
        assert_eq!(resolved.replace_string, "v1.0.0");
        assert_eq!(
            resolved.auto_replace_string_template,
            "{{newValue}}{{#if newDigest}}@{{newDigest}}{{/if}}"
        );
        assert_eq!(resolved.datasource, "docker");
    }

    // Ported: "extracts newName" — kustomize/extract.spec.ts line 757
    #[test]
    fn extracts_new_name_override() {
        let deps = extract(
            r#"
images:
  - name: node
    newName: registry.example.com/runtime/node
    newTag: 20.10.0
"#,
        );
        assert_eq!(deps.len(), 1);
        let KustomizeDep::Image(image) = &deps[0] else {
            panic!("expected image dependency");
        };
        assert_eq!(image.image, "registry.example.com/runtime/node");
        assert_eq!(image.tag.as_deref(), Some("20.10.0"));
        assert!(image.skip_reason.is_none());
    }

    // Ported: "should extract out image versions" — kustomize/extract.spec.ts line 506
    #[test]
    fn package_file_extracts_image_versions() {
        let content = r#"
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization
images:
- name: node
  newTag: v0.1.0
- newTag: v0.0.1
  name: group/instance
- name: quay.io/test/repo
  newTag: v0.0.2
- name: gitlab.com/org/suborg/image
  newTag: v0.0.3
- name: this-lives/on-docker-hub
  newName: but.this.lives.on.local/private-registry
  newTag: v0.0.4
- name: nginx
  newTag: 2.5
"#;
        let images: Vec<_> = extract(content)
            .into_iter()
            .filter_map(|dep| match dep {
                KustomizeDep::Image(image) => Some(image),
                _ => None,
            })
            .collect();
        assert_eq!(images.len(), 6);
        assert_eq!(images[0].image, "node");
        assert_eq!(images[0].tag.as_deref(), Some("v0.1.0"));
        assert_eq!(images[1].image, "group/instance");
        assert_eq!(images[1].tag.as_deref(), Some("v0.0.1"));
        assert_eq!(images[2].image, "quay.io/test/repo");
        assert_eq!(images[2].tag.as_deref(), Some("v0.0.2"));
        assert_eq!(images[3].image, "gitlab.com/org/suborg/image");
        assert_eq!(images[3].tag.as_deref(), Some("v0.0.3"));
        assert_eq!(images[4].image, "but.this.lives.on.local/private-registry");
        assert_eq!(images[4].tag.as_deref(), Some("v0.0.4"));
        assert_eq!(images[5].image, "nginx");
        assert_eq!(
            images[5].skip_reason,
            Some(KustomizeImageSkipReason::InvalidValue)
        );
    }

    // Ported: "should correctly extract a chart" — kustomize/extract.spec.ts line 217
    #[test]
    fn extracts_helm_charts() {
        let content = r#"
helmCharts:
  - name: podinfo
    repo: https://stefanprodan.github.io/podinfo
    version: 6.5.0
  - name: nginx-ingress
    repo: https://kubernetes.github.io/ingress-nginx
    version: 4.9.0
"#;
        let deps = extract(content);
        let charts: Vec<_> = deps
            .iter()
            .filter_map(|d| {
                if let KustomizeDep::Helm(h) = d {
                    Some(h)
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(charts.len(), 2);
        assert!(
            charts
                .iter()
                .any(|c| c.chart_name == "podinfo" && c.current_value == "6.5.0")
        );
        assert!(
            charts
                .iter()
                .any(|c| c.chart_name == "nginx-ingress" && c.current_value == "4.9.0")
        );
    }

    // Ported: "should correctly extract an OCI chart" — kustomize/extract.spec.ts line 233
    #[test]
    fn extracts_oci_helm_chart() {
        let content = r#"
helmCharts:
  - name: redis
    repo: oci://registry-1.docker.io/bitnamicharts
    version: 18.12.1
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let KustomizeDep::OciHelm(chart) = &deps[0] else {
            panic!("expected OCI Helm dependency");
        };
        assert_eq!(chart.chart_name, "redis");
        assert_eq!(
            chart.package_name,
            "registry-1.docker.io/bitnamicharts/redis"
        );
        assert_eq!(chart.current_value, "18.12.1");
    }

    // Ported: "should correctly extract an OCI chart with registryAliases" — kustomize/extract.spec.ts line 249
    #[test]
    fn extracts_oci_helm_chart_with_registry_aliases() {
        let content = r#"
helmCharts:
  - name: redis
    repo: oci://localhost:5000/bitnamicharts
    version: 18.12.1
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let KustomizeDep::OciHelm(chart) = &deps[0] else {
            panic!("expected OCI Helm dependency");
        };
        let resolved =
            resolve_oci_helm_dep(chart.clone(), &[("localhost:5000", "registry-1.docker.io")]);
        assert_eq!(resolved.dep_name, "redis");
        assert_eq!(
            resolved.package_name,
            "registry-1.docker.io/bitnamicharts/redis"
        );
        assert_eq!(resolved.current_value, "18.12.1");
        assert_eq!(resolved.datasource, "docker");
        assert!(!resolved.pin_digests);
    }

    // Ported: "extracts from digest" — kustomize/extract.spec.ts line 710
    #[test]
    fn extracts_images_from_digest() {
        let digest = "sha256:b0cfe264cb1143c7c660ddfd5c482464997d62d6bc9f97f8fdf3deefce881a8c";
        let content = format!(
            r#"
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization
images:
  - name: postgres
    digest: {digest}
  - name: postgres:11
    digest: {digest}
  - name: postgres
    newTag: 11
    digest: {digest}
  - name: postgres
    digest: 02641143766
  - name: postgres
    digest: b0cfe264cb1143c7c660ddfd5c482464997d62d6bc9f97f8fdf3deefce881a8c
"#
        );
        let images: Vec<_> = extract(&content)
            .into_iter()
            .filter_map(|dep| match dep {
                KustomizeDep::Image(image) => Some(image),
                _ => None,
            })
            .collect();
        assert_eq!(images.len(), 5);
        assert_eq!(images[0].image, "postgres");
        assert_eq!(images[0].digest.as_deref(), Some(digest));
        assert_eq!(images[0].tag, None);
        assert_eq!(images[0].replace_string, digest);
        assert_eq!(images[1].image, "postgres");
        assert_eq!(images[1].tag.as_deref(), Some("11"));
        assert_eq!(images[1].digest.as_deref(), Some(digest));
        assert_eq!(images[1].replace_string, digest);
        assert_eq!(
            images[2].skip_reason,
            Some(KustomizeImageSkipReason::InvalidDependencySpecification)
        );
        assert_eq!(
            images[3].skip_reason,
            Some(KustomizeImageSkipReason::InvalidValue)
        );
        assert_eq!(
            images[4].skip_reason,
            Some(KustomizeImageSkipReason::InvalidValue)
        );
    }

    // Ported: "should return null for a local base" — kustomize/extract.spec.ts line 66
    #[test]
    fn local_base_returns_none() {
        assert!(extract_resource("./service-1").is_none());
    }

    // Ported: "should return null for an http base without ref/version" — kustomize/extract.spec.ts line 71
    #[test]
    fn http_base_without_ref_returns_none() {
        assert!(extract_resource("https://github.com/user/test-repo.git?timeout=10s").is_none());
    }

    // Ported: "should extract out the version of an http base" — kustomize/extract.spec.ts line 77
    #[test]
    fn extracts_http_base_ref() {
        let dep = extract_resource("https://github.com/user/test-repo.git?ref=v1.0.0").unwrap();
        assert_eq!(dep.current_value, "v1.0.0");
        assert_eq!(dep.datasource, "github-tags");
        assert_eq!(dep.dep_name, "user/test-repo");
    }

    // Ported: "should extract the version of a non http base" — kustomize/extract.spec.ts line 90
    #[test]
    fn extracts_non_http_ssh_base_ref() {
        let dep = extract_resource("ssh://git@bitbucket.com/user/test-repo?ref=v1.2.3").unwrap();
        assert_eq!(dep.current_value, "v1.2.3");
        assert_eq!(dep.datasource, "git-tags");
        assert_eq!(dep.dep_name, "bitbucket.com/user/test-repo");
        assert_eq!(
            dep.package_name.as_deref(),
            Some("ssh://git@bitbucket.com/user/test-repo")
        );
    }

    // Ported: "should extract the depName if the URL includes a port number" — kustomize/extract.spec.ts line 102
    #[test]
    fn extracts_ssh_base_with_port() {
        let dep =
            extract_resource("ssh://git@bitbucket.com:7999/user/test-repo?ref=v1.2.3").unwrap();
        assert_eq!(dep.dep_name, "bitbucket.com:7999/user/test-repo");
        assert_eq!(
            dep.package_name.as_deref(),
            Some("ssh://git@bitbucket.com:7999/user/test-repo")
        );
    }

    // Ported: "should extract the version of a non http base with subdir" — kustomize/extract.spec.ts line 114
    #[test]
    fn extracts_ssh_base_with_subdir() {
        let dep =
            extract_resource("ssh://git@bitbucket.com/user/test-repo/subdir?ref=v1.2.3").unwrap();
        assert_eq!(dep.current_value, "v1.2.3");
        assert_eq!(dep.dep_name, "bitbucket.com/user/test-repo");
        assert_eq!(
            dep.package_name.as_deref(),
            Some("ssh://git@bitbucket.com/user/test-repo")
        );
    }

    // Ported: "should extract out the version of an github base" — kustomize/extract.spec.ts line 126
    #[test]
    fn extracts_github_shorthand_base_ref() {
        let dep = extract_resource("github.com/fluxcd/flux/deploy?ref=v1.0.0").unwrap();
        assert_eq!(dep.current_value, "v1.0.0");
        assert_eq!(dep.datasource, "github-tags");
        assert_eq!(dep.dep_name, "fluxcd/flux");
    }

    // Ported: "should extract out the version of a git base" — kustomize/extract.spec.ts line 139
    #[test]
    fn extracts_git_at_github_base_ref() {
        let dep = extract_resource("git@github.com:user/repo.git?ref=v1.0.0").unwrap();
        assert_eq!(dep.current_value, "v1.0.0");
        assert_eq!(dep.datasource, "github-tags");
        assert_eq!(dep.dep_name, "user/repo");
    }

    // Ported: "should extract out the version of a git base with subdir" — kustomize/extract.spec.ts line 152
    #[test]
    fn extracts_git_at_github_base_with_subdir() {
        let dep = extract_resource("git@github.com:user/repo.git/subdir?ref=v1.0.0").unwrap();
        assert_eq!(dep.current_value, "v1.0.0");
        assert_eq!(dep.dep_name, "user/repo");
    }

    // Ported: "should extract out the version of an http base with additional params" — kustomize/extract.spec.ts line 165
    #[test]
    fn extracts_http_base_ref_with_additional_params() {
        let dep = extract_resource(
            "https://github.com/user/test-repo.git?timeout=120&ref=v1.0.0&submodules=false&version=v1",
        )
        .unwrap();
        assert_eq!(dep.current_value, "v1.0.0");
        assert_eq!(dep.dep_name, "user/test-repo");
    }

    // Ported: "should extract out the version of an http base from first version param" — kustomize/extract.spec.ts line 180
    #[test]
    fn extracts_http_base_first_version_param() {
        let dep =
            extract_resource("https://github.com/user/test-repo.git?version=v1.0.0&version=v0")
                .unwrap();
        assert_eq!(dep.current_value, "v1.0.0");
        assert_eq!(dep.dep_name, "user/test-repo");
    }

    // Ported: "should extract out the version of an http base from first ref param" — kustomize/extract.spec.ts line 193
    #[test]
    fn extracts_http_base_first_ref_param() {
        let dep =
            extract_resource("https://github.com/user/test-repo.git?ref=v1.0.0&ref=v0").unwrap();
        assert_eq!(dep.current_value, "v1.0.0");
        assert_eq!(dep.dep_name, "user/test-repo");
    }

    // Ported: "extracts multiple image lines" — kustomize/extract.spec.ts line 416
    #[test]
    fn extracts_multiple_base_lines() {
        let deps = extract(
            r#"
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization
bases:
- service-1
- https://moredhel/remote-kustomize.git?ref=v0.0.1
- https://moredhel/remote-kustomize.git//deploy?ref=v0.0.1
"#,
        );
        let resources: Vec<_> = deps
            .iter()
            .filter_map(|dep| {
                if let KustomizeDep::Resource(resource) = dep {
                    Some(resource)
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(resources.len(), 2);
        assert!(
            resources
                .iter()
                .all(|dep| dep.dep_name == "moredhel/remote-kustomize")
        );
        assert!(resources.iter().all(|dep| dep.current_value == "v0.0.1"));
    }

    // Ported: "extracts ssh dependency" — kustomize/extract.spec.ts line 444
    #[test]
    fn package_file_extracts_ssh_dependency() {
        let deps = extract(
            r#"
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization
bases:
  - git@github.com:moredhel/remote-kustomize.git?ref=v0.0.1
"#,
        );
        assert_eq!(deps.len(), 1);
        let KustomizeDep::Resource(resource) = &deps[0] else {
            panic!("expected resource dependency");
        };
        assert_eq!(resource.dep_name, "moredhel/remote-kustomize");
        assert_eq!(resource.current_value, "v0.0.1");
        assert_eq!(resource.datasource, "github-tags");
    }

    // Ported: "extracts ssh dependency with a subdir" — kustomize/extract.spec.ts line 462
    #[test]
    fn package_file_extracts_ssh_dependency_with_subdir() {
        let deps = extract(
            r#"
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization
bases:
- git@github.com:kubernetes-sigs/kustomize.git//examples/helloWorld?ref=v2.0.0
"#,
        );
        assert_eq!(deps.len(), 1);
        let KustomizeDep::Resource(resource) = &deps[0] else {
            panic!("expected resource dependency");
        };
        assert_eq!(resource.dep_name, "kubernetes-sigs/kustomize");
        assert_eq!(resource.current_value, "v2.0.0");
    }

    // Ported: "extracts http dependency" — kustomize/extract.spec.ts line 481
    #[test]
    fn package_file_extracts_http_dependencies() {
        let deps = extract(
            r#"
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization
bases:
- github.com/user/repo//deploy?ref=v0.0.1
- github.com/fluxcd/flux/deploy?ref=1.19.0
"#,
        );
        let resources: Vec<_> = deps
            .iter()
            .filter_map(|dep| {
                if let KustomizeDep::Resource(resource) = dep {
                    Some(resource)
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(resources.len(), 2);
        assert_eq!(resources[0].dep_name, "user/repo");
        assert_eq!(resources[0].current_value, "v0.0.1");
        assert_eq!(resources[1].dep_name, "fluxcd/flux");
        assert_eq!(resources[1].current_value, "1.19.0");
    }

    // Ported: "should extract bases resources and components from their respective blocks" — kustomize/extract.spec.ts line 598
    #[test]
    fn extracts_bases_resources_and_components_blocks() {
        let deps = extract(
            r#"
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization
bases:
- git@github.com:moredhel/remote-kustomize.git?ref=v0.0.1
resources:
- github.com/fluxcd/flux/deploy?ref=1.19.0
components:
- github.com/fluxcd/flux/memcache-dep?ref=1.18.0
"#,
        );
        let resources: Vec<_> = deps
            .iter()
            .filter_map(|dep| {
                if let KustomizeDep::Resource(resource) = dep {
                    Some(resource)
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(resources.len(), 3);
        assert_eq!(resources[0].dep_name, "moredhel/remote-kustomize");
        assert_eq!(resources[1].dep_name, "fluxcd/flux");
        assert_eq!(resources[2].dep_name, "fluxcd/flux");
        assert_eq!(resources[2].current_value, "1.18.0");
    }

    // Ported: "should extract dependencies when kind is Component" — kustomize/extract.spec.ts line 632
    #[test]
    fn extracts_dependencies_when_kind_is_component() {
        let deps = extract(
            r#"
apiVersion: kustomize.config.k8s.io/v1alpha1
kind: Component
resources:
- deployment.yaml
- github.com/fluxcd/flux/deploy?ref=1.19.0
components:
- github.com/fluxcd/flux/memcache-dep?ref=1.18.0
"#,
        );
        let resources: Vec<_> = deps
            .iter()
            .filter_map(|dep| {
                if let KustomizeDep::Resource(resource) = dep {
                    Some(resource)
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(resources.len(), 2);
        assert_eq!(resources[0].dep_name, "fluxcd/flux");
        assert_eq!(resources[0].current_value, "1.19.0");
        assert_eq!(resources[1].dep_name, "fluxcd/flux");
        assert_eq!(resources[1].current_value, "1.18.0");
    }

    // Ported: "parses helmChart field" — kustomize/extract.spec.ts line 799
    #[test]
    fn mixed_images_and_helm() {
        let content = r#"
images:
  - name: nginx
    newTag: 1.19.0

helmCharts:
  - name: cert-manager
    repo: https://charts.jetstack.io
    version: v1.13.0
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(matches!(deps[0], KustomizeDep::Image(_)));
        assert!(matches!(deps[1], KustomizeDep::Helm(_)));
    }

    #[test]
    fn image_without_tag_skipped() {
        let content = "images:\n  - name: nginx\n";
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    #[test]
    fn helm_without_version_skipped() {
        let content = "helmCharts:\n  - name: podinfo\n    repo: https://example.com\n";
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    // Ported: "return null on an invalid file" — kustomize/extract.spec.ts line 33
    #[test]
    fn empty_content_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "ignores non-Kubernetes empty files" — kustomize/extract.spec.ts line 586
    #[test]
    fn ignores_non_kubernetes_empty_files() {
        assert!(extract("").is_empty());
    }

    // Ported: "does nothing with kustomize empty kustomize files" — kustomize/extract.spec.ts line 590
    #[test]
    fn empty_kustomization_returns_empty() {
        let content = r#"
apiVersion: kustomize.config.k8s.io/v1beta1
kind: Kustomization
"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "returns null for non kustomize kubernetes files" — kustomize/extract.spec.ts line 400
    #[test]
    fn non_kustomize_kubernetes_file_returns_empty() {
        let content = r#"
apiVersion: v1
kind: Service
metadata:
  name: sample-service
spec:
  ports:
  - port: 80
    protocol: TCP
    targetPort: http
    name: http
"#;
        assert!(extract(content).is_empty());
    }
}
