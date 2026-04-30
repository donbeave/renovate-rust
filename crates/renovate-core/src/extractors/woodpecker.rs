//! Woodpecker CI `.woodpecker.yml` Docker image extractor.
//!
//! Scans Woodpecker pipeline files for `image:` keys in `steps`, `services`,
//! `pipeline`, and `clone` blocks. Woodpecker's structure mirrors Drone CI:
//! named steps at the top of each block, each with an optional `image:` key.
//!
//! Since `image:` can appear at any nesting depth, the extractor uses a simple
//! universal scan (same approach as Drone CI).
//!
//! Renovate reference:
//! - `lib/modules/manager/woodpecker/extract.ts`
//! - Pattern: `/^\.woodpecker(?:/[^/]+)?\.ya?ml$/`
//!
//! ## Supported form
//!
//! ```yaml
//! steps:
//!   build:
//!     image: golang:1.21
//!   test:
//!     image: golang:1.21
//! services:
//!   redis:
//!     image: redis:7-alpine
//! ```

use crate::extractors::dockerfile::{DockerfileExtractedDep, classify_image_ref};

/// Extract Docker image deps from a Woodpecker CI YAML file.
///
/// Returns one dep for every `image:` key found anywhere in the file.
pub fn extract(content: &str) -> Vec<DockerfileExtractedDep> {
    let mut out = Vec::new();

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            continue;
        }
        let key_line = trimmed.strip_prefix("- ").unwrap_or(trimmed);
        if let Some(val) = strip_key(key_line, "image") {
            let image = val.trim().trim_matches('"').trim_matches('\'');
            if !image.is_empty() {
                out.push(classify_image_ref(image));
            }
        }
    }

    out
}

fn strip_key<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}:");
    line.strip_prefix(prefix.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "extracts multiple image lines" — woodpecker/extract.spec.ts line 21
    #[test]
    fn extracts_step_image() {
        let content = r#"
steps:
  build:
    image: golang:1.21
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "golang");
        assert_eq!(deps[0].tag.as_deref(), Some("1.21"));
    }

    // Ported: "extracts multiple image lines" — woodpecker/extract.spec.ts line 21
    #[test]
    fn extracts_service_image() {
        let content = r#"
services:
  redis:
    image: redis:7-alpine
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "redis");
    }

    // Ported: "extracts multiple image lines" — woodpecker/extract.spec.ts line 21
    #[test]
    fn multiple_steps_and_services() {
        let content = r#"
steps:
  build:
    image: golang:1.21
  lint:
    image: golangci/golangci-lint:v1.55
services:
  db:
    image: postgres:15
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
    }

    // Ported: "extracts multiple image lines" — woodpecker/extract.spec.ts line 21
    #[test]
    fn variable_ref_skipped() {
        let content = "steps:\n  ci:\n    image: ${CI_IMAGE}\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert!(deps[0].skip_reason.is_some());
    }

    // Ported: "returns null for empty" — woodpecker/extract.spec.ts line 8
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "returns null for non-object YAML" — woodpecker/extract.spec.ts line 12
    #[test]
    fn non_object_yaml_returns_empty() {
        assert!(extract("nothing here").is_empty());
        assert!(extract("clone: null").is_empty());
    }

    // Ported: "return null when no dependencies are provided" — woodpecker/extract.spec.ts line 313
    #[test]
    fn no_steps_or_services_returns_empty() {
        let content = "pipeline: {}\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "returns null for malformed YAML" — woodpecker/extract.spec.ts line 17
    #[test]
    fn malformed_yaml_returns_empty() {
        let content = "nothing here\n:::::::\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "extracts the v.1.0.x version" — woodpecker/extract.spec.ts line 220
    #[test]
    fn steps_section_extracts_image() {
        let content = r#"
        steps:
          redis:
            image: quay.io/something/redis:alpine
        "#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "quay.io/something/redis");
        assert_eq!(deps[0].tag.as_deref(), Some("alpine"));
    }

    // Ported: "should parse multiple sources of dependencies together" — woodpecker/extract.spec.ts line 246
    #[test]
    fn clone_and_steps_both_extracted() {
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

    // Ported: "return dependency when an plugin-git is cloned" — woodpecker/extract.spec.ts line 286
    #[test]
    fn clone_section_extracted() {
        let content = r#"
        clone:
          git:
            image: woodpeckerci/plugin-git:latest
        "#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "woodpeckerci/plugin-git");
        assert_eq!(deps[0].tag.as_deref(), Some("latest"));
    }
}
