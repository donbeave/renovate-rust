//! Drone CI `.drone.yml` Docker image extractor.
//!
//! Scans Drone pipeline files for `image:` values. Every `image:` key in the
//! file is a Docker image reference — this applies to both `steps` and
//! `services` blocks at any nesting depth.
//!
//! Renovate reference:
//! - `lib/modules/manager/droneci/extract.ts`
//! - Pattern: `/(^|/)\.drone\.yml$/`
//!
//! ## Supported form
//!
//! ```yaml
//! kind: pipeline
//! steps:
//! - name: test
//!   image: golang:1.21
//! services:
//! - name: db
//!   image: postgres:14
//! ```

use crate::extractors::dockerfile::{DockerfileExtractedDep, classify_image_ref};

/// Extract Docker image deps from a Drone CI `.drone.yml` file.
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

        // Strip optional `- ` list prefix before checking for `image:`.
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

    #[test]
    fn extracts_step_image() {
        let content = r#"
kind: pipeline
steps:
- name: test
  image: golang:1.21
  commands:
    - go test ./...
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "golang");
        assert_eq!(deps[0].tag.as_deref(), Some("1.21"));
    }

    #[test]
    fn extracts_service_image() {
        let content = r#"
services:
- name: db
  image: postgres:14
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "postgres");
        assert_eq!(deps[0].tag.as_deref(), Some("14"));
    }

    #[test]
    fn multiple_images() {
        let content = r#"
steps:
- name: build
  image: golang:1.21
- name: lint
  image: golangci/golangci-lint:v1.55
services:
- name: redis
  image: redis:7-alpine
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
        assert!(deps.iter().any(|d| d.image == "golang"));
        assert!(deps.iter().any(|d| d.image == "golangci/golangci-lint"));
        assert!(deps.iter().any(|d| d.image == "redis"));
    }

    #[test]
    fn variable_ref_skipped() {
        let content = "steps:\n- name: ci\n  image: $DRONE_IMAGE\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert!(deps[0].skip_reason.is_some());
    }

    #[test]
    fn private_registry_not_docker_hub() {
        let content = "steps:\n- image: gcr.io/myproject/myapp:v1.0\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "gcr.io/myproject/myapp");
    }

    // Ported: "returns null for empty" — droneci/extract.spec.ts line 8
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }
}
