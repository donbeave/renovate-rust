//! Crow CI `.crow/*.yml` Docker image extractor.
//!
//! Scans Crow pipeline files for `image:` values in `pipeline`,
//! `steps`, `clone`, and `services` blocks.
//!
//! Renovate reference:
//! - `lib/modules/manager/crow/extract.ts`
//! - Pattern: `/^\.crow(?:/[^/]+)?\.ya?ml$/`
//! - Datasource: Docker
//!
//! ## Supported form
//!
//! ```yaml
//! pipeline:
//!   build:
//!     image: golang:1.21
//!   test:
//!     image: golangci/golangci-lint:v1.55
//! services:
//!   db:
//!     image: postgres:14
//! ```

use crate::extractors::dockerfile::{DockerfileExtractedDep, classify_image_ref};

/// Extract Docker image deps from a Crow CI YAML file.
///
/// Returns one dep for every `image:` key found in pipeline/steps/clone/services.
pub fn extract(content: &str) -> Vec<DockerfileExtractedDep> {
    let mut out = Vec::new();
    let mut in_section = false;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            continue;
        }

        let indent = line.len() - line.trim_start().len();

        // Top-level key — check if we're entering a relevant section.
        // Array items (`- ...`) at indent=0 belong to the current section.
        if indent == 0 && !trimmed.starts_with('-') {
            in_section = matches!(trimmed, "pipeline:" | "steps:" | "clone:" | "services:");
            continue;
        }

        if !in_section {
            continue;
        }

        // Strip optional list prefix `- `.
        let key_line = trimmed.strip_prefix("- ").unwrap_or(trimmed);
        if let Some(val) = key_line.strip_prefix("image:") {
            let image = val.trim().trim_matches('"').trim_matches('\'');
            if !image.is_empty() {
                out.push(classify_image_ref(image));
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_pipeline_images() {
        let content = r#"
pipeline:
  build:
    image: golang:1.21
  lint:
    image: golangci/golangci-lint:v1.55
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(deps.iter().any(|d| d.image == "golang"));
        assert!(deps.iter().any(|d| d.image == "golangci/golangci-lint"));
    }

    #[test]
    fn extracts_services_image() {
        let content = r#"
services:
  db:
    image: postgres:14
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "postgres");
        assert_eq!(deps[0].tag.as_deref(), Some("14"));
    }

    #[test]
    fn steps_as_array() {
        let content = r#"
steps:
- name: build
  image: node:18-alpine
- name: test
  image: node:18-alpine
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
    }

    #[test]
    fn clone_section() {
        let content = r#"
clone:
  default:
    image: plugins/git:latest
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "plugins/git");
    }

    #[test]
    fn top_level_image_ignored() {
        let content = r#"
image: not-a-pipeline-image
pipeline:
  build:
    image: golang:1.21
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "golang");
    }

    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    #[test]
    fn variable_ref_classified() {
        let content = r#"
pipeline:
  build:
    image: $CROW_IMAGE
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert!(deps[0].skip_reason.is_some());
    }

    #[test]
    fn no_image_keys_returns_empty() {
        // Ported: "returns null for non-object YAML" — crow/extract.spec.ts line 10
        assert!(extract("nothing here").is_empty());
        assert!(extract("clone: null").is_empty());
    }

    #[test]
    fn no_dependencies_returns_empty() {
        // Ported: "return null when no dependencies are provided" — crow/extract.spec.ts line 348
        let content = "info:\n  version:\n    3.5\n";
        assert!(extract(content).is_empty());
    }

    #[test]
    fn pipeline_without_valid_images_returns_empty() {
        // Ported: "returns null when pipeline keys exist but contain no valid images" — crow/extract.spec.ts line 390
        let content = "pipeline:\n  test:\n    script: echo 'hello'\n";
        assert!(extract(content).is_empty());
    }
}
