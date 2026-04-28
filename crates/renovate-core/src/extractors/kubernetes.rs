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
    /// Non-Docker-Hub registry — not supported by this extractor.
    NonDockerHub,
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

// ── Regexes ───────────────────────────────────────────────────────────────────

/// K8s manifest signature: both `apiVersion:` and `kind:` present.
static API_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^\s*apiVersion\s*:").unwrap());
static KIND_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?m)^\s*kind\s*:").unwrap());

/// `image: <value>` YAML line (with optional list prefix `-`).
static IMAGE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"^\s*-?\s*image:\s*['"]?([^'"#\s]+)['"]?\s*$"##).unwrap());

// ── Public API ────────────────────────────────────────────────────────────────

/// Extract Kubernetes container image deps from a manifest file.
///
/// Returns an empty Vec if the file is not a Kubernetes manifest.
pub fn extract(content: &str) -> Vec<KubernetesDep> {
    if !API_RE.is_match(content) || !KIND_RE.is_match(content) {
        return Vec::new();
    }

    let mut deps = Vec::new();

    for line in content.lines() {
        let stripped = match line.find(" #") {
            Some(pos) => &line[..pos],
            None => line,
        };
        let Some(cap) = IMAGE_RE.captures(stripped) else {
            continue;
        };
        let image_ref = &cap[1];

        if let Some(dep) = parse_image_ref(image_ref) {
            deps.push(dep);
        }
    }

    deps
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

    // Skip non-Docker-Hub registries (contain a dot or port before first slash).
    if is_non_docker_hub(name_part) {
        return Some(KubernetesDep {
            image_name: name_part.to_owned(),
            current_value: tag.to_owned(),
            skip_reason: Some(KubernetesSkipReason::NonDockerHub),
        });
    }

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

/// Returns true if the image appears to be from a non-Docker-Hub registry.
fn is_non_docker_hub(name: &str) -> bool {
    // If the first path segment contains a `.` or `:`, it's a hostname/registry.
    let first_segment = name.split('/').next().unwrap_or(name);
    first_segment.contains('.') || first_segment.contains(':')
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

    #[test]
    fn extracts_docker_hub_images() {
        let deps = extract(DEPLOYMENT);
        let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
        assert_eq!(actionable.len(), 2);
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

    #[test]
    fn skips_non_docker_hub() {
        let deps = extract(DEPLOYMENT);
        let non_hub: Vec<_> = deps
            .iter()
            .filter(|d| d.skip_reason == Some(KubernetesSkipReason::NonDockerHub))
            .collect();
        assert!(!non_hub.is_empty());
        assert!(non_hub.iter().any(|d| d.image_name.contains("gcr.io")));
    }

    #[test]
    fn skips_digest_pinned() {
        let deps = extract(DEPLOYMENT);
        let pinned: Vec<_> = deps
            .iter()
            .filter(|d| d.skip_reason == Some(KubernetesSkipReason::DigestPinned))
            .collect();
        assert!(!pinned.is_empty());
    }

    #[test]
    fn skips_latest_tag() {
        let deps = extract(DEPLOYMENT);
        let no_ver: Vec<_> = deps
            .iter()
            .filter(|d| d.skip_reason == Some(KubernetesSkipReason::NoVersion))
            .collect();
        assert!(!no_ver.is_empty());
    }

    #[test]
    fn returns_empty_for_non_k8s() {
        assert!(extract("key: value\nanother: field\n").is_empty());
    }
}
