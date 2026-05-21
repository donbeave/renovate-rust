//! Kubernetes manifest Docker image extractor.
//!
//! Detects Kubernetes YAML manifests and extracts `image:` references for
//! version tracking via the Docker Hub datasource.
//!
//! Renovate reference:
//! - `lib/modules/manager/kubernetes/extract.ts`
//! - Default patterns: `[]` (user-configured). We add common conventions.
//! - Datasource: `docker` (Docker Hub)
//!
//! ## Supported `image:` forms
//!
//! ```yaml
//! containers:
//!   - name: app
//!     image: nginx:1.21.0
//!   - name: sidecar
//!     image: gcr.io/google-samples/hello-app:1.0
//!   - name: pinned
//!     image: nginx@sha256:abcdef...   # digest-pinned — skipped
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// Skip reason for a Kubernetes image dep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KubernetesSkipReason {
    /// Image is pinned by digest (`@sha256:...`) — no version to update.
    DigestPinned,
    /// `latest` tag or no tag — skip to avoid noisy updates.
    NoVersion,
}

/// A single Kubernetes container image dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KubernetesDep {
    /// Docker image name (e.g. `nginx`, `library/redis`).
    pub image_name: String,
    /// Image tag (e.g. `1.21.0`).
    pub current_value: String,
    pub skip_reason: Option<KubernetesSkipReason>,
}

/// Docker dep metadata after applying Kubernetes registry alias config.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KubernetesDockerDep {
    pub dep_name: String,
    pub package_name: String,
    pub current_value: String,
    pub replace_string: String,
    pub auto_replace_string_template: String,
    pub skip_reason: Option<KubernetesSkipReason>,
}

// ── Regexes ───────────────────────────────────────────────────────────────────

/// K8s manifest signature: both `apiVersion:` and `kind:` present.
static API_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^\s*apiVersion\s*:").unwrap());
static KIND_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^\s*kind\s*:").unwrap());
static KIND_VALUE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?m)^\s*kind\s*:\s*["']?([^"'\s]+)"#).unwrap());

/// `image: <value>` YAML line (with optional list prefix `-`).
static IMAGE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"^\s*-?\s*image:\s*['"]?([^'"#\s]+)['"]?\s*$"##).unwrap());
static IMAGE_VOLUME_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"^\s*reference:\s*['"]?([^'"#\s]+)['"]?\s*$"##).unwrap());

// ── Public API ────────────────────────────────────────────────────────────────

/// Extract Kubernetes container image deps from a manifest file.
///
/// Returns an empty Vec if the file is not a Kubernetes manifest.
pub fn extract(content: &str) -> Vec<KubernetesDep> {
    if !API_RE.is_match(content) || !KIND_RE.is_match(content) {
        return Vec::new();
    }

    content
        .split("\n---")
        .filter(|doc| API_RE.is_match(doc) && KIND_RE.is_match(doc))
        .flat_map(extract_doc)
        .collect()
}

fn extract_doc(doc: &str) -> Vec<KubernetesDep> {
    let mut deps = Vec::new();
    let mut in_image_volume = false;
    let supports_image_volumes = KIND_VALUE_RE
        .captures(doc)
        .is_some_and(|cap| supports_image_volume_kind(&cap[1]));

    for line in doc.lines() {
        let stripped = match line.find(" #") {
            Some(pos) => &line[..pos],
            None => line,
        };
        let trimmed = stripped.trim_start();

        let image_ref = if trimmed == "image:" {
            in_image_volume = supports_image_volumes;
            continue;
        } else if in_image_volume {
            if let Some(cap) = IMAGE_VOLUME_RE.captures(stripped) {
                in_image_volume = false;
                cap[1].to_owned()
            } else if trimmed.starts_with('-') || !trimmed.starts_with("reference:") {
                in_image_volume = false;
                continue;
            } else {
                continue;
            }
        } else if let Some(cap) = IMAGE_RE.captures(stripped) {
            cap[1].to_owned()
        } else {
            continue;
        };

        if let Some(dep) = parse_image_ref(&image_ref) {
            deps.push(dep);
        }
    }

    deps
}

fn supports_image_volume_kind(kind: &str) -> bool {
    matches!(
        kind,
        "CronJob"
            | "DaemonSet"
            | "Deployment"
            | "Job"
            | "Pod"
            | "ReplicaSet"
            | "ReplicationController"
            | "StatefulSet"
    )
}

/// Extract Docker deps and apply Renovate-style registry aliases.
pub fn extract_with_registry_aliases(
    content: &str,
    registry_aliases: &[(&str, &str)],
) -> Vec<KubernetesDockerDep> {
    extract(content)
        .into_iter()
        .map(|dep| kubernetes_docker_dep(dep, registry_aliases))
        .collect()
}

fn kubernetes_docker_dep(
    dep: KubernetesDep,
    registry_aliases: &[(&str, &str)],
) -> KubernetesDockerDep {
    let dep_name = dep.image_name;
    let package_name = apply_registry_alias(&dep_name, registry_aliases);
    let alias_applied = package_name != dep_name;
    let replace_string = image_ref(&dep_name, &dep.current_value);
    let auto_replace_string_template = if alias_applied {
        format!(
            "{dep_name}:{{{{#if newValue}}}}{{{{newValue}}}}{{{{/if}}}}{{{{#if newDigest}}}}@{{{{newDigest}}}}{{{{/if}}}}"
        )
    } else {
        "{{depName}}{{#if newValue}}:{{newValue}}{{/if}}{{#if newDigest}}@{{newDigest}}{{/if}}"
            .to_owned()
    };

    KubernetesDockerDep {
        dep_name,
        package_name,
        current_value: dep.current_value,
        replace_string,
        auto_replace_string_template,
        skip_reason: dep.skip_reason,
    }
}

fn apply_registry_alias(image: &str, registry_aliases: &[(&str, &str)]) -> String {
    let Some((registry, rest)) = image.split_once('/') else {
        return image.to_owned();
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
        .unwrap_or_else(|| image.to_owned())
}

fn image_ref(image: &str, tag: &str) -> String {
    if tag.is_empty() {
        image.to_owned()
    } else {
        format!("{image}:{tag}")
    }
}

/// Parse a Docker image reference into a [`KubernetesDep`].
fn parse_image_ref(image_ref: &str) -> Option<KubernetesDep> {
    // Skip digest-pinned images.
    if image_ref.contains("@sha256:") || image_ref.contains("@sha512:") {
        let name = image_ref.split('@').next().unwrap_or(image_ref);
        return Some(KubernetesDep {
            image_name: name.to_owned(),
            current_value: String::new(),
            skip_reason: Some(KubernetesSkipReason::DigestPinned),
        });
    }

    // Split at the last `:` that isn't a port number.
    let (name_part, tag) = split_image_tag(image_ref);

    // Skip `latest` or empty tags.
    if tag.is_empty() || tag == "latest" {
        return Some(KubernetesDep {
            image_name: name_part.to_owned(),
            current_value: tag.to_owned(),
            skip_reason: Some(KubernetesSkipReason::NoVersion),
        });
    }

    Some(KubernetesDep {
        image_name: name_part.to_owned(),
        current_value: tag.to_owned(),
        skip_reason: None,
    })
}

/// Split `image:tag` at the last `:` that is not part of a port number.
/// Returns `(image_name, tag)`.
fn split_image_tag(s: &str) -> (&str, &str) {
    if let Some(pos) = s.rfind(':') {
        let tag = &s[pos + 1..];
        let name = &s[..pos];
        // If the tag looks like a port number in a registry host, don't split there.
        // A port follows a hostname segment (no `/` in tag).
        if !tag.contains('/') {
            return (name, tag);
        }
    }
    (s, "")
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEPLOYMENT: &str = r#"
apiVersion: apps/v1
kind: Deployment
spec:
  template:
    spec:
      containers:
      - name: nginx
        image: nginx:1.21.0
      - name: redis
        image: redis:7.0.5
      - name: sidecar
        image: gcr.io/google-samples/hello-app:1.0
      - name: pinned
        image: nginx@sha256:abcdef1234567890
      - name: latest
        image: busybox:latest
"#;

    // Ported: "extracts multiple Kubernetes configurations" — manager/kubernetes/extract.spec.ts line 23
    #[test]
    fn extracts_docker_hub_images() {
        let deps = extract(DEPLOYMENT);
        let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
        // nginx, redis, and gcr.io image all extracted (all registries supported)
        assert_eq!(actionable.len(), 3);
        assert!(
            actionable
                .iter()
                .any(|d| d.image_name == "nginx" && d.current_value == "1.21.0")
        );
        assert!(
            actionable
                .iter()
                .any(|d| d.image_name == "redis" && d.current_value == "7.0.5")
        );
    }

    // Ported: "extracts multiple Kubernetes configurations" — manager/kubernetes/extract.spec.ts line 23
    #[test]
    fn extracts_non_docker_hub_registries() {
        // TypeScript extractor extracts all images regardless of registry — no NonDockerHub skip.
        let deps = extract(DEPLOYMENT);
        let gcr = deps
            .iter()
            .find(|d| d.image_name.contains("gcr.io"))
            .unwrap();
        assert!(gcr.skip_reason.is_none());
        assert_eq!(gcr.current_value, "1.0");
    }

    #[test]
    fn skips_digest_pinned() {
        let deps = extract(DEPLOYMENT);
        let has_pinned = deps
            .iter()
            .any(|d| d.skip_reason == Some(KubernetesSkipReason::DigestPinned));
        assert!(has_pinned);
    }

    #[test]
    fn skips_latest_tag() {
        let deps = extract(DEPLOYMENT);
        let has_no_ver = deps
            .iter()
            .any(|d| d.skip_reason == Some(KubernetesSkipReason::NoVersion));
        assert!(has_no_ver);
    }

    // Ported: "returns null for empty" — kubernetes/extract.spec.ts line 14
    #[test]
    fn returns_empty_for_non_k8s() {
        assert!(extract("key: value\nanother: field\n").is_empty());
    }

    // Ported: "returns null for empty" — kubernetes/extract.spec.ts line 14
    #[test]
    fn returns_empty_for_empty_input() {
        assert!(extract("").is_empty());
    }

    // Ported: "handles invalid YAML files" — kubernetes/extract.spec.ts line 125
    #[test]
    fn handles_invalid_yaml_with_no_images() {
        // apiVersion and kind present but malformed YAML — no image lines → empty
        let content = "apiVersion: v1\nkind: ConfigMap\n<\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "does not return unknown kind" — kubernetes/extract.spec.ts line 18
    #[test]
    fn configmap_with_no_images_returns_empty() {
        let content =
            "apiVersion: v1\nkind: ConfigMap\nmetadata:\n  name: test\ndata:\n  key: value\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "extracts image tag when it contains underscores" — kubernetes/extract.spec.ts line 98
    #[test]
    fn extracts_image_with_underscore_in_tag() {
        let content = r#"apiVersion: apps/v1
kind: Deployment
spec:
  template:
    spec:
      containers:
      - name: app
        image: ghcr.io/berriai/litellm:litellm_stable_release_branch-v1.67.0-stable
"#;
        let deps = extract(content);
        let dep = deps
            .iter()
            .find(|d| d.image_name == "ghcr.io/berriai/litellm")
            .unwrap();
        assert_eq!(
            dep.current_value,
            "litellm_stable_release_branch-v1.67.0-stable"
        );
        assert!(dep.skip_reason.is_none());
    }

    // Ported: "extracts images and replaces registries" — kubernetes/extract.spec.ts line 133
    #[test]
    fn extracts_images_and_replaces_registries() {
        let content = r#"
apiVersion: v1
kind: Pod
spec:
  containers:
    - name: node
      image: quay.io/node:0.0.1
"#;
        let deps =
            extract_with_registry_aliases(content, &[("quay.io", "my-quay-mirror.registry.com")]);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "quay.io/node");
        assert_eq!(deps[0].package_name, "my-quay-mirror.registry.com/node");
        assert_eq!(deps[0].current_value, "0.0.1");
        assert_eq!(deps[0].replace_string, "quay.io/node:0.0.1");
        assert_eq!(
            deps[0].auto_replace_string_template,
            "quay.io/node:{{#if newValue}}{{newValue}}{{/if}}{{#if newDigest}}@{{newDigest}}{{/if}}"
        );
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "extracts images but does no replacement" — kubernetes/extract.spec.ts line 155
    #[test]
    fn extracts_images_without_registry_replacement() {
        let content = r#"
apiVersion: v1
kind: Pod
spec:
  containers:
    - name: node
      image: quay.io/node:0.0.1
"#;
        let deps = extract_with_registry_aliases(
            content,
            &[("index.docker.io", "my-docker-mirror.registry.com")],
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "quay.io/node");
        assert_eq!(deps[0].package_name, "quay.io/node");
        assert_eq!(
            deps[0].auto_replace_string_template,
            "{{depName}}{{#if newValue}}:{{newValue}}{{/if}}{{#if newDigest}}@{{newDigest}}{{/if}}"
        );
    }

    // Ported: "extracts images and does no double replacements" — kubernetes/extract.spec.ts line 177
    #[test]
    fn extracts_images_without_double_registry_replacement() {
        let content = r#"
apiVersion: v1
kind: Pod
spec:
  containers:
    - name: node
      image: quay.io/node:0.0.1
"#;
        let deps = extract_with_registry_aliases(
            content,
            &[
                ("quay.io", "my-quay-mirror.registry.com"),
                ("my-quay-mirror.registry.com", "quay.io"),
            ],
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "quay.io/node");
        assert_eq!(deps[0].package_name, "my-quay-mirror.registry.com/node");
    }

    // Ported: "extracts from complex templates" — kubernetes/extract.spec.ts line 200
    #[test]
    fn extracts_from_complex_templates() {
        let content = r#"
apiVersion: v1
kind: Pod
metadata:
  name: "{{ include "jitsi-meet.web.fullname" . }}-test-connection"
spec:
  containers:
    - name: wget
      image: busybox
---
apiVersion: apps/v1
kind: Deployment
spec:
  template:
    spec:
      containers:
        - name: {{ .Chart.Name }}
          image: "{{ .Values.jvb.image.repository }}:{{ default .Chart.AppVersion .Values.jvb.image.tag }}"
        - name: metrics
          image: {{ .Values.jvb.metrics.image.repository }}:{{ .Values.jvb.metrics.image.tag }}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image_name, "busybox");
        assert_eq!(deps[0].current_value, "");
        assert_eq!(deps[0].skip_reason, Some(KubernetesSkipReason::NoVersion));
    }

    // Ported: "ignores non-Kubernetes YAML files" — kubernetes/extract.spec.ts line 121
    #[test]
    fn ignores_non_kubernetes_yaml() {
        // GitLab CI YAML has no apiVersion or kind → empty
        let content = "stages:\n  - build\nbuild:\n  image: node:18\n  script:\n    - npm ci\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "extracts image volumes from $kind" — kubernetes/extract.spec.ts line 223
    #[test]
    fn extracts_image_volumes_from_workload_kinds() {
        for (kind, api_version) in [
            ("DaemonSet", "apps/v1"),
            ("Deployment", "apps/v1"),
            ("Job", "batch/v1"),
            ("ReplicaSet", "apps/v1"),
            ("ReplicationController", "v1"),
            ("StatefulSet", "apps/v1"),
        ] {
            let content = format!(
                r#"
apiVersion: {api_version}
kind: {kind}
metadata:
  name: test
spec:
  template:
    spec:
      volumes:
        - name: vol
          image:
            reference: quay.io/test/image:v1.0.0
"#
            );
            let deps = extract(&content);
            assert!(
                deps.iter()
                    .any(|dep| dep.image_name == "quay.io/test/image"
                        && dep.current_value == "v1.0.0"),
                "expected image volume for {kind}"
            );
        }
    }

    // Ported: "extracts image volumes from Pod and CronJob" — kubernetes/extract.spec.ts line 265
    #[test]
    fn extracts_image_volumes_from_pod_and_cronjob() {
        let content = r#"
apiVersion: v1
kind: Pod
metadata:
  name: pod-test
spec:
  volumes:
    - name: vol
      image:
        reference: quay.io/test/pod-image:v1.0.0
---
apiVersion: batch/v1
kind: CronJob
metadata:
  name: cronjob-test
spec:
  jobTemplate:
    spec:
      template:
        spec:
          volumes:
            - name: vol
              image:
                reference: quay.io/test/cronjob-image:v2.0.0
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].image_name, "quay.io/test/pod-image");
        assert_eq!(deps[0].current_value, "v1.0.0");
        assert_eq!(deps[1].image_name, "quay.io/test/cronjob-image");
        assert_eq!(deps[1].current_value, "v2.0.0");
    }

    // Ported: "does not extract image volumes for unsupported kind" — kubernetes/extract.spec.ts line 326
    #[test]
    fn does_not_extract_image_volumes_for_unsupported_kind() {
        let content = r#"
apiVersion: extensions/v1beta1
kind: NetworkPolicy
metadata:
  name: test-network-policy
spec:
  volumes:
    - name: vol
      image:
        reference: quay.io/test/image:v1.0.0
"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "skips malformed volume entries and extracts valid ones" — kubernetes/extract.spec.ts line 349
    #[test]
    fn skips_malformed_image_volume_entries_and_extracts_valid_ones() {
        let content = r#"
apiVersion: v1
kind: Pod
metadata:
  name: pod-test
spec:
  volumes:
    - name: bad-vol
      image:
        notReference: invalid
    - name: good-vol
      image:
        reference: quay.io/test/image:v1.0.0
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image_name, "quay.io/test/image");
        assert_eq!(deps[0].current_value, "v1.0.0");
        assert!(deps[0].skip_reason.is_none());
    }
}
