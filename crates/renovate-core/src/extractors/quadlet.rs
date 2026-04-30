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
        // Section header — Container, Image, and Volume sections all support Image=
        if trimmed.starts_with('[') {
            in_container = matches!(trimmed, "[Container]" | "[Image]" | "[Volume]");
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
            // Skip local file references (foo.image, foo.build)
            if image_str.ends_with(".image") || image_str.ends_with(".build") {
                continue;
            }
            // Skip local transports
            if LOCAL_TRANSPORTS.iter().any(|t| image_str.starts_with(t)) {
                continue;
            }
            // Strip docker:// and docker-daemon: transport prefixes
            let image = image_str
                .strip_prefix("docker://")
                .or_else(|| image_str.strip_prefix("docker-daemon:"))
                .unwrap_or(image_str);
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

    // Ported: "extracts from quadlet container unit" — quadlet/extract.spec.ts line 29
    #[test]
    fn extracts_container_image() {
        let content = "[Container]\nImage=docker.io/library/nginx:latest\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "docker.io/library/nginx");
        assert_eq!(deps[0].tag.as_deref(), Some("latest"));
    }

    // Ported: "handles docker prefix" — quadlet/extract.spec.ts line 83
    #[test]
    fn strips_docker_transport_prefix() {
        let content = "[Container]\nImage=docker://nginx:alpine\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "nginx");
        assert_eq!(deps[0].tag.as_deref(), Some("alpine"));
    }

    // Ported: "extracts from quadlet container unit" — quadlet/extract.spec.ts line 29
    #[test]
    fn skips_local_transport() {
        let content = "[Container]\nImage=oci:/tmp/myimage\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "extracts from quadlet container unit" — quadlet/extract.spec.ts line 29
    #[test]
    fn ignores_non_container_sections() {
        let content = "[Unit]\nDescription=My Service\n[Container]\nImage=nginx:1.25\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "nginx");
    }

    // Ported: "extracts from quadlet container unit" — quadlet/extract.spec.ts line 29
    #[test]
    fn skips_comment_lines() {
        let content = "[Container]\n# This is a comment\nImage=redis:7\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
    }

    // Ported: "returns null for empty yaml file content" — quadlet/extract.spec.ts line 24
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "extracts from quadlet container unit" — quadlet/extract.spec.ts line 29
    #[test]
    fn variable_ref_skipped() {
        let content = "[Container]\nImage=${MY_IMAGE}\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert!(deps[0].skip_reason.is_some());
    }

    // Ported: "extracts from quadlet image unit" — quadlet/extract.spec.ts line 47
    #[test]
    fn image_section_extracted() {
        let content = "[Image]\nImage=docker.io/library/alpine:3.22\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "docker.io/library/alpine");
        assert_eq!(deps[0].tag.as_deref(), Some("3.22"));
    }

    // Ported: "extracts from quadlet volume unit" — quadlet/extract.spec.ts line 65
    #[test]
    fn volume_section_extracted() {
        let content = "[Volume]\nImage=docker.io/library/alpine:3.22\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "docker.io/library/alpine");
    }

    // Ported: "handles docker-daemon prefix" — quadlet/extract.spec.ts line 101
    #[test]
    fn docker_daemon_prefix_stripped() {
        let content = "[Volume]\nImage=docker-daemon:docker.io/library/alpine:3.22\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "docker.io/library/alpine");
        assert_eq!(deps[0].tag.as_deref(), Some("3.22"));
    }

    // Ported: "does not extract an image file reference" — quadlet/extract.spec.ts line 119
    #[test]
    fn image_file_reference_skipped() {
        let content = "[Container]\nImage=foo.image\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "does not extract a build file reference" — quadlet/extract.spec.ts line 129
    #[test]
    fn build_file_reference_skipped() {
        let content = "[Container]\nImage=foo.build\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "handles an unsuccessful parse" — quadlet/extract.spec.ts line 158
    #[test]
    fn container_section_without_image_returns_empty() {
        let content = "[Container]\n";
        assert!(extract(content).is_empty());
    }
}
