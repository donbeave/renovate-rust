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

    // Ported: "extracts multiple image lines" — crow/extract.spec.ts line 19
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

    // Ported: "extracts multiple image lines" — crow/extract.spec.ts line 19
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

    // Ported: "extracts images from array-based steps format" — crow/extract.spec.ts line 408
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

    // Ported: "return dependency when a plugin-git is cloned" — crow/extract.spec.ts line 321
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

    // Ported: "extracts multiple image lines" — crow/extract.spec.ts line 19
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

    // Ported: "returns null for empty" — crow/extract.spec.ts line 6
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "extracts multiple image lines" — crow/extract.spec.ts line 19
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

    // Ported: "returns null for non-object YAML" — crow/extract.spec.ts line 10
    #[test]
    fn no_image_keys_returns_empty() {
        assert!(extract("nothing here").is_empty());
        assert!(extract("clone: null").is_empty());
    }

    // Ported: "return null when no dependencies are provided" — crow/extract.spec.ts line 348
    #[test]
    fn no_dependencies_returns_empty() {
        let content = "info:\n  version:\n    3.5\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "returns null when pipeline keys exist but contain no valid images" — crow/extract.spec.ts line 390
    #[test]
    fn pipeline_without_valid_images_returns_empty() {
        let content = "pipeline:\n  test:\n    script: echo 'hello'\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "returns null for malformed YAML" — crow/extract.spec.ts line 15
    #[test]
    fn malformed_yaml_returns_empty() {
        assert!(extract("nothing here\n:::::::").is_empty());
    }

    // Ported: "extracts the 1.0.0 version" — crow/extract.spec.ts line 255
    #[test]
    fn extracts_semver_version_from_steps() {
        let content = "steps:\n  redis:\n    image: quay.io/something/redis:1.0.0\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "quay.io/something/redis");
        assert_eq!(deps[0].tag.as_deref(), Some("1.0.0"));
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "should parse multiple sources of dependencies together" — crow/extract.spec.ts line 281
    #[test]
    fn extracts_from_clone_and_steps_sections() {
        let content = r#"
clone:
  git:
    image: woodpeckerci/plugin-git:latest
steps:
  redis:
    image: quay.io/something/redis:alpine
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(deps.iter().any(|d| d.image == "woodpeckerci/plugin-git"));
        assert!(deps.iter().any(|d| d.image == "quay.io/something/redis"));
    }

    // Ported: "handles empty pipeline section gracefully" — crow/extract.spec.ts line 362
    #[test]
    fn empty_pipeline_object_is_skipped() {
        // `pipeline: {}` does not match "pipeline:" → not entered; `steps:` still extracted
        let content = "pipeline: {}\nsteps:\n  redis:\n    image: quay.io/something/redis:alpine\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "quay.io/something/redis");
    }
}
