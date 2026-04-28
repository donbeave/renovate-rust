//! Batect `batect.yml` Docker image extractor.
//!
//! Scans Batect configuration YAML for container `image:` references.
//! Git-based bundle includes (`type: git`) are noted but deferred.
//!
//! Renovate reference:
//! - `lib/modules/manager/batect/extract.ts`
//! - Patterns: `/(^|/)batect(-bundle)?\.ya?ml$/`
//!
//! ## Supported forms
//!
//! ```yaml
//! containers:
//!   my-container:
//!     image: alpine:3.18
//!   another:
//!     image: postgres:15-alpine
//! ```

use crate::extractors::dockerfile::{DockerfileExtractedDep, classify_image_ref};

/// Extract Docker image deps from a Batect YAML file.
pub fn extract(content: &str) -> Vec<DockerfileExtractedDep> {
    let mut out = Vec::new();
    let mut in_containers = false;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        let trimmed = line.trim_start();

        if trimmed.is_empty() {
            continue;
        }

        let indent = line.len() - line.trim_start().len();

        // Top-level `containers:` key (indent 0).
        if trimmed == "containers:" && indent == 0 {
            in_containers = true;
            continue;
        }

        // Exit containers block at next top-level key (indent 0, not a list).
        if indent == 0 && !trimmed.starts_with('-') && in_containers {
            in_containers = false;
        }

        if !in_containers {
            continue;
        }

        // `image:` at indent ≥ 4 (inside a container definition).
        if trimmed.starts_with("image:") {
            let rest = trimmed.strip_prefix("image:").unwrap_or("").trim();
            let value = rest.trim_matches('"').trim_matches('\'');
            if !value.is_empty() && !value.starts_with('$') {
                out.push(classify_image_ref(value));
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
containers:
  app:
    image: alpine:3.18
  db:
    image: "postgres:15-alpine"
  custom:
    build_directory: ./custom

tasks:
  build:
    run:
      container: app
"#;

    #[test]
    fn extracts_images() {
        let deps = extract(SAMPLE);
        let alpine = deps.iter().find(|d| d.image == "alpine").unwrap();
        assert_eq!(alpine.tag.as_deref(), Some("3.18"));
        assert!(alpine.skip_reason.is_none());

        let pg = deps.iter().find(|d| d.image == "postgres").unwrap();
        assert_eq!(pg.tag.as_deref(), Some("15-alpine"));
    }

    #[test]
    fn skips_build_directory_containers() {
        let deps = extract(SAMPLE);
        assert_eq!(deps.len(), 2); // only alpine and postgres
    }

    #[test]
    fn stops_at_tasks_block() {
        // Tasks section should not contribute images.
        let deps = extract(SAMPLE);
        assert!(!deps.iter().any(|d| d.image == "app"));
    }

    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    #[test]
    fn no_containers_block_returns_empty() {
        let content = "tasks:\n  build:\n    run:\n      container: app\n";
        assert!(extract(content).is_empty());
    }
}
