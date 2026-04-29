//! Ansible playbook Docker image extractor.
//!
//! Scans Ansible task files for `image:` keys and extracts Docker image
//! references for Docker Hub version lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/ansible/extract.ts`
//! - Pattern: `/(^|/)tasks/[^/]+\\.ya?ml$/`
//!
//! ## Supported form
//!
//! ```yaml
//! - name: Run my container
//!   community.docker.docker_container:
//!     image: nginx:1.25
//! ```

use crate::extractors::dockerfile::{DockerfileExtractedDep, classify_image_ref};

/// Extract Docker image deps from an Ansible task YAML file.
pub fn extract(content: &str) -> Vec<DockerfileExtractedDep> {
    let mut out = Vec::new();

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        let trimmed = line.trim_start();

        // Match `image: value` with optional quotes
        let Some(rest) = trimmed.strip_prefix("image:") else {
            continue;
        };
        let value = rest.trim().trim_matches('"').trim_matches('\'').trim();
        if value.is_empty() || value.contains("${") || value.starts_with('$') {
            continue;
        }

        out.push(classify_image_ref(value));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
- name: Start web server
  community.docker.docker_container:
    name: web
    image: nginx:1.25.3
    state: started

- name: Run database
  community.docker.docker_container:
    name: db
    image: "postgres:15-alpine"
    state: started

- name: Dynamic image
  community.docker.docker_container:
    image: ${MY_IMAGE}

- name: Custom registry
  community.docker.docker_container:
    image: registry.example.com/myapp:2.0
"#;

    #[test]
    fn extracts_images() {
        let deps = extract(SAMPLE);
        let nginx = deps.iter().find(|d| d.image == "nginx").unwrap();
        assert_eq!(nginx.tag.as_deref(), Some("1.25.3"));
        assert!(nginx.skip_reason.is_none());

        let pg = deps.iter().find(|d| d.image == "postgres").unwrap();
        assert_eq!(pg.tag.as_deref(), Some("15-alpine"));
    }

    #[test]
    fn skips_variable_images() {
        let deps = extract(SAMPLE);
        assert!(!deps.iter().any(|d| d.image.contains("MY_IMAGE")));
    }

    #[test]
    fn extracts_custom_registry_image() {
        let deps = extract(SAMPLE);
        assert!(deps.iter().any(|d| d.image.contains("myapp")));
    }

    // Ported: "returns null for empty" — ansible/extract.spec.ts line 6
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    #[test]
    fn no_image_keys_returns_empty() {
        let content = "- name: task\n  shell: echo hello\n";
        assert!(extract(content).is_empty());
    }
}
