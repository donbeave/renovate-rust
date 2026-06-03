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

static ANNOTATION_URL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"https://(?:(?P<release_host>github\.com)/(?P<release_org>[A-Za-z0-9_.-]+)/(?P<release_repo>[A-Za-z0-9_.-]+)/releases/download/(?P<release_version>v[^\s,\]/]+)|raw\.githubusercontent\.com/(?P<raw_org>[A-Za-z0-9_.-]+)/(?P<raw_repo>[A-Za-z0-9_.-]+)/(?P<raw_version>v[^\s,\]/]+)|github\.com/(?P<github_raw_org>[A-Za-z0-9_.-]+)/(?P<github_raw_repo>[A-Za-z0-9_.-]+)/raw/(?P<github_raw_version>v[^\s,\]/]+))",
    )
    .unwrap()
});

/// Dependency extracted from Pipelines-as-Code Tekton annotations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TektonAnnotationDep {
    pub dep_name: String,
    pub current_value: String,
    pub datasource: &'static str,
    pub package_name: String,
}

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

/// Extract Pipelines-as-Code task/pipeline URLs from Tekton annotations.
pub fn extract_annotation_deps(content: &str) -> Vec<TektonAnnotationDep> {
    ANNOTATION_URL_RE
        .captures_iter(content)
        .filter_map(|caps| {
            if let (Some(org), Some(repo), Some(version)) = (
                caps.name("release_org"),
                caps.name("release_repo"),
                caps.name("release_version"),
            ) {
                return Some(TektonAnnotationDep {
                    dep_name: format!("github.com/{}/{}", org.as_str(), repo.as_str()),
                    current_value: version.as_str().to_owned(),
                    datasource: "github-releases",
                    package_name: format!("{}/{}", org.as_str(), repo.as_str()),
                });
            }

            let (org, repo, version) = if let (Some(org), Some(repo), Some(version)) = (
                caps.name("raw_org"),
                caps.name("raw_repo"),
                caps.name("raw_version"),
            ) {
                (org, repo, version)
            } else if let (Some(org), Some(repo), Some(version)) = (
                caps.name("github_raw_org"),
                caps.name("github_raw_repo"),
                caps.name("github_raw_version"),
            ) {
                (org, repo, version)
            } else {
                return None;
            };

            Some(TektonAnnotationDep {
                dep_name: format!("github.com/{}/{}", org.as_str(), repo.as_str()),
                current_value: version.as_str().to_owned(),
                datasource: "git-tags",
                package_name: format!("https://github.com/{}/{}", org.as_str(), repo.as_str()),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "extracts deps from a file" — lib/modules/manager/tekton/extract.spec.ts line 6
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

    // Rust-specific: tekton behavior test
    #[test]
    fn returns_empty_for_non_tekton() {
        let content = "apiVersion: apps/v1\nkind: Deployment\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "extracts deps from a file" — lib/modules/manager/tekton/extract.spec.ts line 6
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

    // Ported: "extracts deps from a file in annotations" — lib/modules/manager/tekton/extract.spec.ts line 15
    #[test]
    fn extracts_annotation_task_and_pipeline_refs() {
        let content = r#"
---
kind: PipelineRun
metadata:
  annotations:
    pipelinesascode.tekton.dev/task: "[git-clone,https://github.com/foo/bar/releases/download/v0.0.4/stakater-create-git-tag.yaml]"
    pipelinesascode.tekton.dev/pipeline: "https://raw.githubusercontent.com/foo/baz/v0.0.12/pipeline/deploy/deploy.yaml"
---
kind: PipelineRun
metadata:
  annotations:
    pipelinesascode.tekton.dev/task: "[git-clone,
      https://raw.githubusercontent.com/foo/bar/v0.0.6/tasks/create-git-tag/create-git-tag.yaml]"
    pipelinesascode.tekton.dev/pipeline: "
      https://raw.githubusercontent.com/foo/baz/v0.0.12/pipeline/deploy/deploy.yaml"
---
kind: PipelineRun
metadata:
  annotations:
    pipelinesascode.tekton.dev/task: "git-clone"
    pipelinesascode.tekton.dev/task-1: "https://github.com/foo/bar/raw/v0.0.8/tasks/create-git-tag/create-git-tag.yaml"
    pipelinesascode.tekton.dev/pipeline: "https://github.com/foo/baz/raw/v0.0.14/pipeline/deploy/deploy.yaml"
---
kind: PipelineRun
metadata:
  annotations:
    pipelinesascode.tekton.dev/task: "[git-clone,
      https://github.com/foo/bar/releases/download/v0.0.9/stakater-create-git-tag.yaml,
      https://github.com/foo/bar/raw/v0.0.7/tasks/create-git-tag/create-git-tag.yaml,
      https://raw.githubusercontent.com/foo/bar/v0.0.5/tasks/create-git-tag/create-git-tag.yaml]"
    pipelinesascode.tekton.dev/pipeline: "https://raw.githubusercontent.com/foo/baz/v0.0.25/pipeline/deploy/deploy.yaml"
"#;
        let deps = extract_annotation_deps(content);
        assert_eq!(deps.len(), 10);
        assert_eq!(deps[0].dep_name, "github.com/foo/bar");
        assert_eq!(deps[0].current_value, "v0.0.4");
        assert_eq!(deps[0].datasource, "github-releases");
        assert_eq!(deps[0].package_name, "foo/bar");
        assert_eq!(deps[1].dep_name, "github.com/foo/baz");
        assert_eq!(deps[1].current_value, "v0.0.12");
        assert_eq!(deps[1].datasource, "git-tags");
        assert_eq!(deps[1].package_name, "https://github.com/foo/baz");
        assert_eq!(deps[8].dep_name, "github.com/foo/bar");
        assert_eq!(deps[8].current_value, "v0.0.5");
        assert_eq!(deps[9].dep_name, "github.com/foo/baz");
        assert_eq!(deps[9].current_value, "v0.0.25");
    }

    // Ported: "ignores file without any deps" — lib/modules/manager/tekton/extract.spec.ts line 96
    #[test]
    fn ignores_file_without_deps() {
        assert!(extract("foo: bar").is_empty());
    }

    // Ported: "ignores empty file" — lib/modules/manager/tekton/extract.spec.ts line 112
    #[test]
    fn ignores_empty_file() {
        assert!(extract("").is_empty());
    }

    // Ported: "ignores invalid YAML" — lib/modules/manager/tekton/extract.spec.ts line 100
    //
    // Content `bundle: registry.com/repo` looks like a stray bundle key
    // outside any tekton resource. Rust extractor walks the YAML
    // document — without a valid `kind:` it produces no deps.
    #[test]
    fn ignores_invalid_yaml_with_stray_bundle_key() {
        let content = "\n---\nbundle: registry.com/repo\n";
        assert!(extract(content).is_empty());
    }
}
