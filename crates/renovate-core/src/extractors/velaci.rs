//! Vela CI `.vela.yml` Docker image extractor.
//!
//! Scans Vela pipeline YAML files for `image:` keys in `steps:`, `services:`,
//! and `stages[].steps:` blocks. Vela's format mirrors Drone CI.
//!
//! Renovate reference:
//! - `lib/modules/manager/velaci/extract.ts`
//! - Pattern: `/(^|/)\.vela\.ya?ml$/`
//!
//! ## Supported form
//!
//! ```yaml
//! steps:
//!   - name: build
//!     image: golang:1.21
//! services:
//!   - name: redis
//!     image: redis:7-alpine
//! ```

use crate::extractors::dockerfile::{DockerfileExtractedDep, classify_image_ref};

/// Extract Docker image deps from a Vela CI `.vela.yml` file.
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

    // Ported: "extracts multiple step pipeline image lines" — velaci/extract.spec.ts line 16
    #[test]
    fn extracts_step_image() {
        let content = "steps:\n  - name: build\n    image: golang:1.21\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "golang");
        assert_eq!(deps[0].tag.as_deref(), Some("1.21"));
    }

    // Ported: "extracts multiple services pipeline image lines" — velaci/extract.spec.ts line 30
    #[test]
    fn extracts_service_image() {
        let content = "services:\n  - name: db\n    image: postgres:15\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "postgres");
    }

    // Ported: "extracts multiple step pipeline image lines" — velaci/extract.spec.ts line 16
    #[test]
    fn variable_skipped() {
        let content = "steps:\n  - name: ci\n    image: $VELA_IMAGE\n";
        let deps = extract(content);
        assert!(deps[0].skip_reason.is_some());
    }

    // Ported: "should handle YAML without pipeline/images" — velaci/extract.spec.ts line 11
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "should handle invalid YAML" — velaci/extract.spec.ts line 6
    #[test]
    fn invalid_yaml_returns_empty() {
        // invalid YAML ("foo: bar: invalid") has no pipeline steps
        assert!(extract("foo: bar: invalid").is_empty());
    }

    // Ported: "should handle YAML without pipeline/images" — velaci/extract.spec.ts line 11
    #[test]
    fn yaml_without_pipeline_returns_empty() {
        assert!(extract("no: pipeline").is_empty());
    }

    // Ported: "extracts multiple stages pipeline image lines" — velaci/extract.spec.ts line 48
    #[test]
    fn extracts_stages_pipeline_images() {
        // Mirrors __fixtures__/.vela-stages.yaml
        let content = "version: \"1\"\n\nstages:\n  go:\n    steps:\n      - name: go\n        image: golang:1.13\n  node:\n    steps:\n      - name: node\n        image: amd64/node:10.0.0@sha256:5082d4db78ee2d24f72b25eb75537f2e19c49f04a595378d1ae8814d6f2fbf28\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(
            deps.iter()
                .any(|d| d.image == "golang" && d.tag.as_deref() == Some("1.13"))
        );
        assert!(deps.iter().any(|d| d.image == "amd64/node"));
    }

    // Ported: "extracts multiple secrets pipeline image lines" — velaci/extract.spec.ts line 62
    #[test]
    fn extracts_secrets_pipeline_images() {
        // Mirrors __fixtures__/.vela-secrets.yml
        let content = "version: \"1\"\n\nsteps:\n  - name: node\n    image: amd64/node:10.0.0@sha256:5082d4db78ee2d24f72b25eb75537f2e19c49f04a595378d1ae8814d6f2fbf28\n\nsecrets:\n  - name: vault_token\n  - origin:\n      name: private vault\n      image: target/secret-vault:v0.1.0\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(deps.iter().any(|d| d.image == "amd64/node"));
        assert!(
            deps.iter()
                .any(|d| d.image == "target/secret-vault" && d.tag.as_deref() == Some("v0.1.0"))
        );
    }
}
