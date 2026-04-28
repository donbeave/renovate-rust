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

use crate::extractors::dockerfile::{DockerfileExtractedDep, classify_image_ref};

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

/// A single dependency extracted from a `kustomization.yaml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KustomizeDep {
    /// Docker image from `images:` section.
    Image(DockerfileExtractedDep),
    /// Helm chart from `helmCharts:` section.
    Helm(KustomizeHelmDep),
}

/// Extract dependencies from a `kustomization.yaml` file.
pub fn extract(content: &str) -> Vec<KustomizeDep> {
    let mut out = Vec::new();

    #[derive(Clone, Copy)]
    enum State {
        Default,
        InImages,
        InHelmCharts,
    }

    let mut state = State::Default;
    // Current image entry being assembled.
    let mut img_name: Option<String> = None;
    let mut img_new_name: Option<String> = None;
    let mut img_new_tag: Option<String> = None;
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
                }
            }
            State::InImages => {
                if indent == 0 && !trimmed.starts_with('-') {
                    // Exited images section — flush last entry
                    flush_image(&mut img_name, &mut img_new_name, &mut img_new_tag, &mut out);
                    state = State::Default;
                    if trimmed == "helmCharts:" {
                        state = State::InHelmCharts;
                    }
                    continue;
                }
                // New list item
                if let Some(rest) = trimmed.strip_prefix("- ") {
                    flush_image(&mut img_name, &mut img_new_name, &mut img_new_tag, &mut out);
                    if let Some(val) = strip_key(rest, "name") {
                        img_name = Some(val.trim().trim_matches('"').trim_matches('\'').to_owned());
                    }
                } else if let Some(val) = strip_key(trimmed, "name") {
                    img_name = Some(val.trim().trim_matches('"').trim_matches('\'').to_owned());
                } else if let Some(val) = strip_key(trimmed, "newName") {
                    img_new_name = Some(val.trim().trim_matches('"').trim_matches('\'').to_owned());
                } else if let Some(val) = strip_key(trimmed, "newTag") {
                    img_new_tag = Some(val.trim().trim_matches('"').trim_matches('\'').to_owned());
                }
            }
            State::InHelmCharts => {
                if indent == 0 && !trimmed.starts_with('-') {
                    flush_helm(&mut helm_name, &mut helm_repo, &mut helm_version, &mut out);
                    state = State::Default;
                    if trimmed == "images:" {
                        state = State::InImages;
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
        }
    }

    // Flush trailing entries.
    flush_image(&mut img_name, &mut img_new_name, &mut img_new_tag, &mut out);
    flush_helm(&mut helm_name, &mut helm_repo, &mut helm_version, &mut out);

    out
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn leading_spaces(s: &str) -> usize {
    s.len() - s.trim_start_matches([' ', '\t']).len()
}

fn strip_key<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}:");
    line.strip_prefix(prefix.as_str())
}

fn flush_image(
    name: &mut Option<String>,
    new_name: &mut Option<String>,
    new_tag: &mut Option<String>,
    out: &mut Vec<KustomizeDep>,
) {
    let Some(n) = name.take() else {
        new_name.take();
        new_tag.take();
        return;
    };
    let lookup_name = new_name.take().unwrap_or(n);
    let tag = new_tag.take().unwrap_or_default();
    if tag.is_empty() || lookup_name.is_empty() {
        return;
    }
    let image_ref = format!("{lookup_name}:{tag}");
    let dep = classify_image_ref(&image_ref);
    out.push(KustomizeDep::Image(dep));
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
    out.push(KustomizeDep::Helm(KustomizeHelmDep {
        chart_name,
        repository_url,
        current_value,
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn empty_content_returns_empty() {
        assert!(extract("").is_empty());
    }
}
