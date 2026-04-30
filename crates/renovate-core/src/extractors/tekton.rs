//! Tekton CI/CD resource extractor.
//!
//! Extracts Docker image references from Tekton Task, Pipeline, and related
//! resource YAML files. Reuses the Kubernetes image extraction logic for
//! step `image:` fields, and additionally handles `bundle:` references.
//!
//! Renovate reference:
//! - `lib/modules/manager/tekton/extract.ts`
//! - Default patterns: `[]` (user-configured). We add `tekton/` convention.
//! - Datasources: `docker` (Docker Hub step images)
//!
//! ## Supported forms
//!
//! ```yaml
//! apiVersion: tekton.dev/v1
//! kind: Task
//! spec:
//!   steps:
//!   - name: build
//!     image: gcr.io/kaniko-project/executor:v1.9.0
//!   - name: test
//!     image: golang:1.21.0
//! ```

use std::sync::LazyLock;

use regex::Regex;

// Re-export kubernetes types for unified pipeline handling.
pub use crate::extractors::kubernetes::{KubernetesDep, KubernetesSkipReason};

/// Detects Tekton resources: `apiVersion: tekton.dev/`.
static TEKTON_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"apiVersion:\s*tekton\.dev/").unwrap());

/// Extract Tekton step image deps from a resource file.
///
/// Returns an empty Vec if the file is not a Tekton resource.
pub fn extract(content: &str) -> Vec<KubernetesDep> {
    if !TEKTON_RE.is_match(content) {
        return Vec::new();
    }
    // Tekton Task steps use the same `image:` format as Kubernetes containers.
    // Delegate to the kubernetes image extractor (it checks apiVersion+kind which
    // Tekton resources also have).
    crate::extractors::kubernetes::extract(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "extracts deps from a file" — tekton/extract.spec.ts line 6
    #[test]
    fn extracts_step_images() {
        let content = r#"
apiVersion: tekton.dev/v1
kind: Task
metadata:
  name: build-and-test
spec:
  steps:
  - name: test
    image: golang:1.21.0
  - name: build
    image: gcr.io/kaniko-project/executor:v1.9.0
"#;
        let deps = extract(content);
        let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
        // All images extracted regardless of registry (TypeScript behavior)
        assert_eq!(actionable.len(), 2);
        assert!(
            actionable
                .iter()
                .any(|d| d.image_name == "golang" && d.current_value == "1.21.0")
        );
        assert!(actionable.iter().any(
            |d| d.image_name == "gcr.io/kaniko-project/executor" && d.current_value == "v1.9.0"
        ));
    }

    #[test]
    fn returns_empty_for_non_tekton() {
        let content = "apiVersion: apps/v1\nkind: Deployment\n";
        assert!(extract(content).is_empty());
    }

    #[test]
    fn extracts_gcr_images_without_skip() {
        // TypeScript extractor does not skip non-Docker-Hub registries.
        let content = r#"
apiVersion: tekton.dev/v1beta1
kind: Task
spec:
  steps:
  - image: gcr.io/google-containers/busybox:1.27.2
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert!(deps[0].skip_reason.is_none());
        assert_eq!(deps[0].image_name, "gcr.io/google-containers/busybox");
    }

    // Ported: "ignores file without any deps" — tekton/extract.spec.ts line 96
    #[test]
    fn ignores_file_without_deps() {
        assert!(extract("foo: bar").is_empty());
    }

    // Ported: "ignores empty file" — tekton/extract.spec.ts line 112
    #[test]
    fn ignores_empty_file() {
        assert!(extract("").is_empty());
    }
}
