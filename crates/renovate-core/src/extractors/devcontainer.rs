//! Dev Container (`devcontainer.json`) Docker image extractor.
//!
//! Reads the `"image"` field from Dev Container JSON configuration files.
//! Dev Containers are used by VS Code and GitHub Codespaces to define the
//! development environment container.
//!
//! Renovate reference:
//! - `lib/modules/manager/devcontainer/extract.ts`
//! - Patterns: `/^.devcontainer/devcontainer.json$/`, `/^.devcontainer.json$/`
//!
//! ## Supported form
//!
//! ```json
//! {
//!   "name": "My Dev Container",
//!   "image": "mcr.microsoft.com/devcontainers/base:ubuntu-22.04"
//! }
//! ```

use crate::extractors::dockerfile::{DockerfileExtractedDep, classify_image_ref};

/// Extract the Docker image dep from a `devcontainer.json` file.
///
/// Returns `Some(dep)` when an `"image"` key with a non-empty string value is
/// found. Returns `None` for empty files or files with no `image` key.
pub fn extract(content: &str) -> Option<DockerfileExtractedDep> {
    // Simple JSON scan: look for `"image": "value"` without pulling in serde_json.
    // This avoids adding a dependency and keeps the extractor fast.
    for line in content.lines() {
        let trimmed = line.trim();
        // Find `"image":` anywhere in the line (handles single-line JSON).
        let Some(after_key) = trimmed.find(r#""image":"#).map(|pos| &trimmed[pos + 8..]) else {
            continue;
        };
        let rest = after_key.trim();
        // Strip optional leading/trailing quotes and trailing comma.
        let image = rest
            .trim_start_matches('"')
            .trim_end_matches([',', '}'])
            .trim_end_matches('"')
            .trim();
        if !image.is_empty() {
            return Some(classify_image_ref(image));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_image() {
        let content = r#"
{
  "name": "Dev Container",
  "image": "mcr.microsoft.com/devcontainers/base:ubuntu-22.04"
}
"#;
        let dep = extract(content).unwrap();
        assert_eq!(dep.image, "mcr.microsoft.com/devcontainers/base");
        assert_eq!(dep.tag.as_deref(), Some("ubuntu-22.04"));
    }

    #[test]
    fn extracts_docker_hub_image() {
        let content = r#"{"image": "node:18-bullseye"}"#;
        let dep = extract(content).unwrap();
        assert_eq!(dep.image, "node");
        assert_eq!(dep.tag.as_deref(), Some("18-bullseye"));
    }

    #[test]
    fn variable_ref_skipped() {
        let content = r#"{"image": "$MY_IMAGE"}"#;
        let dep = extract(content).unwrap();
        assert!(dep.skip_reason.is_some());
    }

    #[test]
    fn no_image_key_returns_none() {
        let content = r#"{"name": "My Container"}"#;
        assert!(extract(content).is_none());
    }

    #[test]
    fn empty_returns_none() {
        assert!(extract("").is_none());
    }

    #[test]
    fn build_spec_no_image_returns_none() {
        let content = r#"{"build": {"dockerfile": "Dockerfile"}}"#;
        assert!(extract(content).is_none());
    }
}
