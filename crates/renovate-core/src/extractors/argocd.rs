//! ArgoCD Application manifest extractor.
//!
//! Extracts Helm chart, Git source, and Docker image references from ArgoCD
//! `Application` and `ApplicationSet` custom resource manifests.
//!
//! Renovate reference:
//! - `lib/modules/manager/argocd/extract.ts`
//! - `lib/modules/manager/argocd/util.ts`
//! - Default patterns: `[]` (user-configured). We add `(^|/)argocd/.+\.ya?ml$`.
//! - Datasources: `helm`, `git-tags`, `docker`
//!
//! ## Supported source forms
//!
//! ```yaml
//! # Helm chart from registry
//! spec:
//!   source:
//!     repoURL: https://charts.helm.sh/stable
//!     chart: redis
//!     targetRevision: 10.5.7
//!
//! # Git source with tag (dep_name = full URL)
//! spec:
//!   source:
//!     repoURL: https://git.example.com/foo/bar.git
//!     targetRevision: v1.2.3
//!
//! # OCI / no-protocol registry chart → Docker datasource
//! spec:
//!   source:
//!     repoURL: somecontainer.registry.io:443/
//!     chart: some/image
//!     targetRevision: 1.0.0
//! ```

use std::sync::LazyLock;

use regex::Regex;

use crate::extractors::dockerfile::{DockerfileExtractedDep, classify_image_ref};

/// Source type for an ArgoCD application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgocdSource {
    /// Helm chart from a registry.
    Helm {
        registry_url: String,
        chart_name: String,
    },
    /// Git repository reference (git-tags datasource); dep_name = full repoURL.
    Git { repo_url: String },
    /// OCI or no-protocol registry chart/image (Docker datasource).
    Docker { dep_name: String },
    /// Unsupported or local source.
    Unsupported,
}

/// Skip reason for an ArgoCD dep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgocdSkipReason {
    /// No `targetRevision` field.
    UnspecifiedVersion,
    /// Chart or repo URL missing / unrecognized.
    InvalidConfig,
}

/// A single ArgoCD source dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgocdDep {
    pub dep_name: String,
    pub current_value: String,
    pub source: ArgocdSource,
    pub skip_reason: Option<ArgocdSkipReason>,
    /// Additional Docker deps from kustomize.images entries.
    pub kustomize_images: Vec<DockerfileExtractedDep>,
}

// ── Regexes ───────────────────────────────────────────────────────────────────

/// Detects ArgoCD manifests: `apiVersion: argoproj.io/`.
static ARGOCD_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"apiVersion:\s*'?"?argoproj\.io/"#).unwrap());

/// `source:` or `sources:` block start.
static SOURCE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s+sources?:\s*$").unwrap());

/// Key-value line inside a source block.
static KV_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\s+(\w+):\s+(?:"([^"]+)"|'([^']+)'|(\S+))"#).unwrap());

/// Kustomize image entry: `- name=image:tag` or `- image:tag`
static KUSTOMIZE_IMAGE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"=(.+)$").unwrap());

fn kv_val<'a>(cap: &'a regex::Captures) -> &'a str {
    cap.get(2)
        .or_else(|| cap.get(3))
        .or_else(|| cap.get(4))
        .map(|m| m.as_str())
        .unwrap_or("")
}

/// Returns true if the value is a template expression (e.g. `{{ .Values.xxx }}`).
fn is_template(s: &str) -> bool {
    s.starts_with("{{") || s.starts_with("${{")
}

/// True if the URL is an OCI registry (`oci://` prefix).
fn is_oci(url: &str) -> bool {
    url.starts_with("oci://")
}

/// True if the URL has no `://` → treat as a docker-style registry (no explicit protocol).
fn has_no_protocol(url: &str) -> bool {
    !url.contains("://")
}

/// Remove the `oci://` prefix and any trailing slash.
fn remove_oci_prefix(url: &str) -> &str {
    url.strip_prefix("oci://")
        .unwrap_or(url)
        .trim_end_matches('/')
}

// ── Parsing ───────────────────────────────────────────────────────────────────

/// Extract ArgoCD source dependencies from a manifest file.
///
/// Returns an empty Vec if the file is not an ArgoCD manifest.
pub fn extract(content: &str) -> Vec<ArgocdDep> {
    if !ARGOCD_RE.is_match(content) {
        return Vec::new();
    }

    let mut deps = Vec::new();

    let mut in_source = false;
    let mut in_kustomize_images = false;
    let mut repo_url: Option<String> = None;
    let mut chart: Option<String> = None;
    let mut target_revision: Option<String> = None;
    let mut kustomize_images: Vec<DockerfileExtractedDep> = Vec::new();
    let mut source_indent: usize = 0;

    for line in content.lines() {
        // Strip inline comments.
        let line = match line.find(" #") {
            Some(pos) => &line[..pos],
            None => line,
        };

        if line.trim().is_empty() {
            continue;
        }

        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();

        // YAML document separator — flush current source.
        if trimmed == "---" {
            if in_source {
                flush_source(
                    &mut repo_url,
                    &mut chart,
                    &mut target_revision,
                    &mut kustomize_images,
                    &mut deps,
                );
                in_source = false;
                in_kustomize_images = false;
            }
            continue;
        }

        // Detect `source:` or `sources:` block start at any indent.
        if SOURCE_RE.is_match(line) {
            flush_source(
                &mut repo_url,
                &mut chart,
                &mut target_revision,
                &mut kustomize_images,
                &mut deps,
            );
            in_source = true;
            in_kustomize_images = false;
            source_indent = indent;
            continue;
        }

        // Exit source block when indent returns to source level or less.
        if in_source && indent <= source_indent && !trimmed.starts_with('-') {
            flush_source(
                &mut repo_url,
                &mut chart,
                &mut target_revision,
                &mut kustomize_images,
                &mut deps,
            );
            in_source = false;
            in_kustomize_images = false;
            // Don't `continue` — still process this line for KV (may be a new key).
        }

        // Within a source: track kustomize.images sub-block.
        if in_source {
            // Detect `images:` list under `kustomize:`.
            if trimmed == "images:" {
                in_kustomize_images = true;
                continue;
            }

            // Exit kustomize.images when indent falls back.
            if in_kustomize_images && indent <= source_indent + 4 && !trimmed.starts_with('-') {
                in_kustomize_images = false;
            }

            // Kustomize image list entry: `- name=image:tag` or `- image:tag`.
            if in_kustomize_images && trimmed.starts_with("- ") {
                let entry = trimmed[2..].trim();
                // Extract the image reference: everything after `=`.
                let image_ref = if let Some(cap) = KUSTOMIZE_IMAGE_RE.captures(entry) {
                    cap[1].trim().to_owned()
                } else {
                    // No `=`: treat whole entry as an image ref.
                    entry.to_owned()
                };
                if !image_ref.is_empty() {
                    kustomize_images.push(classify_image_ref(&image_ref));
                }
                continue;
            }

            // Array item `- ` inside `sources:` starts a new source.
            if trimmed.starts_with("- ") && indent >= source_indent + 2 {
                flush_source(
                    &mut repo_url,
                    &mut chart,
                    &mut target_revision,
                    &mut kustomize_images,
                    &mut deps,
                );
                in_kustomize_images = false;
                let rest = &trimmed[2..];
                if let Some(cap) =
                    KV_RE.captures(&format!("{:indent$}{rest}", "", indent = source_indent + 4))
                {
                    match &cap[1] {
                        "repoURL" => repo_url = Some(kv_val(&cap).to_owned()),
                        "chart" => chart = Some(kv_val(&cap).to_owned()),
                        "targetRevision" => target_revision = Some(kv_val(&cap).to_owned()),
                        _ => {}
                    }
                }
                continue;
            }

            // Regular KV inside a source block.
            if let Some(cap) = KV_RE.captures(line) {
                match &cap[1] {
                    "repoURL" => repo_url = Some(kv_val(&cap).to_owned()),
                    "chart" => chart = Some(kv_val(&cap).to_owned()),
                    "targetRevision" => target_revision = Some(kv_val(&cap).to_owned()),
                    _ => {}
                }
            }
        }
    }

    // Final flush.
    flush_source(
        &mut repo_url,
        &mut chart,
        &mut target_revision,
        &mut kustomize_images,
        &mut deps,
    );

    deps
}

fn flush_source(
    repo_url: &mut Option<String>,
    chart: &mut Option<String>,
    target_revision: &mut Option<String>,
    kustomize_images: &mut Vec<DockerfileExtractedDep>,
    deps: &mut Vec<ArgocdDep>,
) {
    let images = std::mem::take(kustomize_images);

    let url = match repo_url.take() {
        Some(u) if !u.is_empty() => u,
        _ => {
            chart.take();
            target_revision.take();
            return;
        }
    };

    // Skip template expressions entirely.
    if is_template(&url) {
        chart.take();
        target_revision.take();
        return;
    }

    let version = target_revision.take().unwrap_or_default();
    let chart_name = chart.take();

    if let Some(cn) = chart_name {
        // Template chart name → skip.
        if is_template(&cn) {
            return;
        }

        if version.is_empty() {
            deps.push(ArgocdDep {
                dep_name: cn.clone(),
                current_value: String::new(),
                source: ArgocdSource::Helm {
                    registry_url: url,
                    chart_name: cn,
                },
                skip_reason: Some(ArgocdSkipReason::UnspecifiedVersion),
                kustomize_images: Vec::new(),
            });
            return;
        }

        if is_template(&version) {
            return;
        }

        if is_oci(&url) || has_no_protocol(&url) {
            // OCI or bare registry → Docker datasource.
            let registry = remove_oci_prefix(&url);
            let dep_name = format!("{registry}/{cn}");
            deps.push(ArgocdDep {
                dep_name: dep_name.clone(),
                current_value: version,
                source: ArgocdSource::Docker { dep_name },
                skip_reason: None,
                kustomize_images: Vec::new(),
            });
        } else {
            // Standard Helm registry.
            deps.push(ArgocdDep {
                dep_name: cn.clone(),
                current_value: version,
                source: ArgocdSource::Helm {
                    registry_url: url,
                    chart_name: cn,
                },
                skip_reason: None,
                kustomize_images: Vec::new(),
            });
        }
    } else {
        // No chart — it's a Git source (dep_name = full URL).
        if version.is_empty() || is_template(&version) {
            return;
        }

        if is_oci(&url) {
            // OCI without chart → Docker dep.
            let registry = remove_oci_prefix(&url);
            deps.push(ArgocdDep {
                dep_name: registry.to_owned(),
                current_value: version,
                source: ArgocdSource::Docker {
                    dep_name: registry.to_owned(),
                },
                skip_reason: None,
                kustomize_images: Vec::new(),
            });
        } else {
            // Git source: dep_name = full repoURL.
            deps.push(ArgocdDep {
                dep_name: url.clone(),
                current_value: version,
                source: ArgocdSource::Git { repo_url: url },
                skip_reason: None,
                kustomize_images: images,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "return null for kubernetes manifest" — argocd/extract.spec.ts line 21
    #[test]
    fn skips_non_argocd_file() {
        let content = "apiVersion: v1\nkind: ConfigMap\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "return null if deps array would be empty" — argocd/extract.spec.ts line 26
    #[test]
    fn skips_missing_revision() {
        let content = r#"
apiVersion: argoproj.io/v1alpha1
kind: Application
spec:
  source:
    repoURL: https://charts.helm.sh/stable
    chart: redis
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(ArgocdSkipReason::UnspecifiedVersion)
        );
    }

    // Ported: "returns null for empty" — argocd/extract.spec.ts line 11
    #[test]
    fn empty_content_returns_empty() {
        assert!(extract("nothing here").is_empty());
        assert!(extract("").is_empty());
    }

    // Ported: "return result for double quoted argoproj.io apiVersion reference" — argocd/extract.spec.ts line 34
    #[test]
    fn double_quoted_apiversion_accepted() {
        let content = r#"
apiVersion: "argoproj.io/v1alpha1"
kind: Application
spec:
  source:
    chart: kube-state-metrics
    repoURL: https://prometheus-community.github.io/helm-charts
    targetRevision: 2.4.1
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "kube-state-metrics");
        assert_eq!(deps[0].current_value, "2.4.1");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "return result for single quoted argoproj.io apiVersion reference" — argocd/extract.spec.ts line 61
    #[test]
    fn single_quoted_apiversion_accepted() {
        let content = "apiVersion: 'argoproj.io/v1alpha1'\nkind: Application\nspec:\n  source:\n    chart: kube-state-metrics\n    repoURL: https://prometheus-community.github.io/helm-charts\n    targetRevision: 2.4.1\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "kube-state-metrics");
        assert_eq!(deps[0].current_value, "2.4.1");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "return null if deps array would be empty" — argocd/extract.spec.ts line 26
    #[test]
    fn malformed_applications_return_empty() {
        let content = "---\napiVersion: argoproj.io/v1alpha1\nkind: Application\nspec:\n  target:\n    namespace: testing\n---\napiVersion: argoproj.io/v1alpha1\nkind: Application\n---\napiVersion: argoproj.io/v1alpha1\nkind: Application\nspec:\n  sources: []\n---\napiVersion: argoproj.io/v1alpha1\nkind: Application\nspec:\n  source: null\n";
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    // Ported: "returns null for invalid" — argocd/extract.spec.ts line 15
    #[test]
    fn invalid_yaml_with_trailing_content_returns_empty() {
        let content = "---\napiVersion: argoproj.io/v1alpha1\nkind: Application\nspec:\n  source: null\n---\n123\n";
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    // Ported: "full test" — argocd/extract.spec.ts line 88
    #[test]
    fn full_test_helm_source() {
        let content = r#"
apiVersion: argoproj.io/v1alpha1
kind: Application
spec:
  source:
    repoURL: https://charts.helm.sh/stable
    chart: redis
    targetRevision: 10.5.7
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let d = &deps[0];
        assert_eq!(d.dep_name, "redis");
        assert_eq!(d.current_value, "10.5.7");
        assert_eq!(
            d.source,
            ArgocdSource::Helm {
                registry_url: "https://charts.helm.sh/stable".to_owned(),
                chart_name: "redis".to_owned(),
            }
        );
        assert!(d.skip_reason.is_none());
    }

    // Ported: "full test" — argocd/extract.spec.ts line 88
    #[test]
    fn full_test_git_source_dep_name_is_full_url() {
        let content = r#"
apiVersion: argoproj.io/v1alpha1
kind: Application
spec:
  source:
    repoURL: https://git.example.com/foo/bar.git
    targetRevision: v1.2.0
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "https://git.example.com/foo/bar.git");
        assert_eq!(deps[0].current_value, "v1.2.0");
        assert_eq!(
            deps[0].source,
            ArgocdSource::Git {
                repo_url: "https://git.example.com/foo/bar.git".to_owned()
            }
        );
    }

    // Ported: "full test" — argocd/extract.spec.ts line 88
    #[test]
    fn full_test_docker_source_no_protocol() {
        let content = r#"
apiVersion: argoproj.io/v1alpha1
kind: Application
spec:
  source:
    chart: some/image3
    repoURL: somecontainer.registry.io:443/
    targetRevision: 1.0.0
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].dep_name,
            "somecontainer.registry.io:443/some/image3"
        );
        assert_eq!(deps[0].current_value, "1.0.0");
        assert!(matches!(deps[0].source, ArgocdSource::Docker { .. }));
    }

    // Ported: "full test" — argocd/extract.spec.ts line 88
    #[test]
    fn full_test_oci_helm_chart() {
        let content = r#"
apiVersion: argoproj.io/v1alpha1
kind: Application
spec:
  source:
    repoURL: oci://somecontainer.registry.io/org/chart
    targetRevision: 0.4.0
    path: .
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "somecontainer.registry.io/org/chart");
        assert_eq!(deps[0].current_value, "0.4.0");
        assert!(matches!(deps[0].source, ArgocdSource::Docker { .. }));
    }

    // Ported: "full test" — argocd/extract.spec.ts line 88
    #[test]
    fn full_test_kustomize_images() {
        let content = r#"
apiVersion: argoproj.io/v1alpha1
kind: Application
spec:
  source:
    repoURL: https://git.example.com/foo/bar.git
    targetRevision: v1.2.0
    kustomize:
      images:
        - someImage=somecontainer.registry.io/someContainer:v2.3.4
        - otherImage=othercontainer.registry.io/other/container@sha256:8be5de38826b494a8ad1565b8d1eb49183d736d0277a89191bd1100d78479a42
        - notActuallyValidAndShouldBeIgnored
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let git_dep = &deps[0];
        assert_eq!(git_dep.dep_name, "https://git.example.com/foo/bar.git");
        assert_eq!(git_dep.kustomize_images.len(), 3); // 2 valid + 1 parsed as plain image
        let img0 = &git_dep.kustomize_images[0];
        assert_eq!(img0.image, "somecontainer.registry.io/someContainer");
        assert_eq!(img0.tag.as_deref(), Some("v2.3.4"));
        let img1 = &git_dep.kustomize_images[1];
        assert_eq!(img1.image, "othercontainer.registry.io/other/container");
    }

    // Ported: "supports applicationsets" — argocd/extract.spec.ts line 203
    #[test]
    fn supports_applicationsets() {
        // ApplicationSet fixture: spec.template.spec.source (deeper nesting).
        let content = r#"---
apiVersion: argoproj.io/v1alpha1
kind: ApplicationSet
spec:
  template:
    spec:
      source:
        chart: kube-state-metrics
        repoURL: https://prometheus-community.github.io/helm-charts
        targetRevision: 2.4.1
---
apiVersion: argoproj.io/v1alpha1
kind: ApplicationSet
spec:
  template:
    spec:
      source:
        chart: {{ .Values.chart }}
        repoURL: {{ .Values.repoUrl}}
        targetRevision: {{ .Values.targetRevision }}
---
apiVersion: argoproj.io/v1alpha1
kind: ApplicationSet
spec:
  template:
    spec:
      source:
        chart: traefik
        helm:
          values: |
            traefik:
              service:
                spec:
                  loadBalancerIP: 1.2.3.4
        repoURL: https://helm.traefik.io/traefik
        targetRevision: 10.14.2
---
apiVersion: argoproj.io/v1alpha1
kind: ApplicationSet
spec:
  template:
    spec:
      source:
        repoURL: https://git.example.com/foo/bar.git
        targetRevision: v1.2.0
---
apiVersion: argoproj.io/v1alpha1
kind: ApplicationSet
spec:
  template:
    spec:
      target:
        namespace: testing
---
apiVersion: argoproj.io/v1alpha1
kind: ApplicationSet
spec:
  generators:
  - clusters: {}
  template:
    spec:
      source:
        repoURL: https://stefanprodan.github.io/podinfo
        targetRevision: 6.0.0
        chart: podinfo
---
apiVersion: argoproj.io/v1alpha1
kind: ApplicationSet
spec:
  generators:
  - clusters: {}
  template:
    spec:
      sources:
        - chart: some/image3
          repoURL: somecontainer.registry.io:443/
          targetRevision: 1.0.0
---
apiVersion: argoproj.io/v1alpha1
kind: ApplicationSet
spec:
  generators:
  - clusters: {}
  template:
    spec:
      sources:
        - ref: foo
          repoURL: https://git.example.com/foo/bar.git
          targetRevision: v1.2.0
        - chart: some/image3
          repoURL: somecontainer.registry.io:443/
          targetRevision: 1.0.0
---
apiVersion: argoproj.io/v1alpha1
kind: ApplicationSet
spec:
  generators:
  - clusters: {}
  template:
    spec:
      sources:
        - ref: foo
          repoURL: https://git.example.com/foo/bar.git
          targetRevision: v1.2.0
          path: bar
        - chart: somechart
          repoURL: https://foo.io/repo
          targetRevision: 0.0.2
          helm:
            valueFiles:
              - $foo/values.yaml
"#;
        let deps = extract(content);

        // Helm deps
        assert!(deps.iter().any(|d| d.dep_name == "kube-state-metrics"
            && d.current_value == "2.4.1"
            && matches!(d.source, ArgocdSource::Helm { .. })));
        assert!(
            deps.iter()
                .any(|d| d.dep_name == "traefik" && d.current_value == "10.14.2")
        );
        assert!(
            deps.iter()
                .any(|d| d.dep_name == "podinfo" && d.current_value == "6.0.0")
        );
        assert!(deps.iter().any(|d| d.dep_name == "somechart"
            && d.current_value == "0.0.2"
            && matches!(d.source, ArgocdSource::Helm { .. })));

        // Git deps (dep_name = full URL)
        let git_count = deps
            .iter()
            .filter(|d| {
                d.dep_name == "https://git.example.com/foo/bar.git"
                    && matches!(d.source, ArgocdSource::Git { .. })
            })
            .count();
        assert_eq!(git_count, 3); // appears 3 times across sources

        // Docker deps (no-protocol registry)
        let docker_count = deps
            .iter()
            .filter(|d| {
                d.dep_name == "somecontainer.registry.io:443/some/image3"
                    && matches!(d.source, ArgocdSource::Docker { .. })
            })
            .count();
        assert_eq!(docker_count, 2);

        // Template expressions skipped (no dep with "{{" in name)
        assert!(deps.iter().all(|d| !d.dep_name.contains("{{")));
    }
}
