//! Podman Quadlet (`.container`, `.image`, `.volume`) Docker image extractor.
//!
//! Quadlet files use a systemd unit file format with `[Container]` sections
//! containing an `Image=` key that specifies the Docker image to use.
//!
//! Renovate reference:
//! - `lib/modules/manager/quadlet/extract.ts`
//! - Patterns: `/.+\.container$/`, `/.+\.image$/`, `/.+\.volume$/`
//!
//! ## Supported form
//!
//! ```ini
//! [Container]
//! Image=docker.io/library/nginx:latest
//! PublishPort=8080:80
//! ```

use crate::extractors::dockerfile::{DockerfileExtractedDep, classify_image_ref};

/// Podman image transports that reference local storage rather than a registry.
/// These should be skipped since they aren't resolvable via Docker Hub.
const LOCAL_TRANSPORTS: &[&str] = &[
    "dir:",
    "docker-archive:",
    "oci-archive:",
    "oci:",
    "containers-storage:",
    "sif:",
];

/// Extract Docker image deps from a Quadlet systemd unit file.
///
/// Scans for `Image=` keys in `[Container]` sections. Ignores local transport
/// prefixes (`dir:`, `oci:`, etc.) and strips `docker://` transport prefix.
pub fn extract(content: &str) -> Vec<DockerfileExtractedDep> {
    let mut out = Vec::new();
    let mut in_container = false;

    for raw in content.lines() {
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
            continue;
        }
        // Section header
        if trimmed.starts_with('[') {
            in_container = trimmed == "[Container]";
            continue;
        }
        if !in_container {
            continue;
        }
        if let Some(val) = trimmed.strip_prefix("Image=") {
            let image_str = val.trim();
            if image_str.is_empty() {
                continue;
            }
            // Skip local transports
            if LOCAL_TRANSPORTS.iter().any(|t| image_str.starts_with(t)) {
                continue;
            }
            // Strip docker:// transport prefix
            let image = image_str.strip_prefix("docker://").unwrap_or(image_str);
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
    fn extracts_container_image() {
        let content = "[Container]\nImage=docker.io/library/nginx:latest\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "docker.io/library/nginx");
        assert_eq!(deps[0].tag.as_deref(), Some("latest"));
    }

    #[test]
    fn strips_docker_transport_prefix() {
        let content = "[Container]\nImage=docker://nginx:alpine\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "nginx");
        assert_eq!(deps[0].tag.as_deref(), Some("alpine"));
    }

    #[test]
    fn skips_local_transport() {
        let content = "[Container]\nImage=oci:/tmp/myimage\n";
        assert!(extract(content).is_empty());
    }

    #[test]
    fn ignores_non_container_sections() {
        let content = "[Unit]\nDescription=My Service\n[Container]\nImage=nginx:1.25\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "nginx");
    }

    #[test]
    fn skips_comment_lines() {
        let content = "[Container]\n# This is a comment\nImage=redis:7\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
    }

    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    #[test]
    fn variable_ref_skipped() {
        let content = "[Container]\nImage=${MY_IMAGE}\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert!(deps[0].skip_reason.is_some());
    }
}
