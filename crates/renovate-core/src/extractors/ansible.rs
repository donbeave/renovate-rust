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

/// Docker dep metadata after applying Ansible registry alias config.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnsibleDockerDep {
    pub dep_name: String,
    pub package_name: String,
    pub current_value: Option<String>,
    pub current_digest: Option<String>,
    pub replace_string: String,
    pub auto_replace_string_template: String,
}

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

/// Extract Docker deps and apply Renovate-style registry aliases.
pub fn extract_with_registry_aliases(
    content: &str,
    registry_aliases: &[(&str, &str)],
) -> Vec<AnsibleDockerDep> {
    extract(content)
        .into_iter()
        .map(|dep| ansible_docker_dep(dep, registry_aliases))
        .collect()
}

fn ansible_docker_dep(
    dep: DockerfileExtractedDep,
    registry_aliases: &[(&str, &str)],
) -> AnsibleDockerDep {
    let dep_name = dep.image;
    let package_name = apply_registry_alias(&dep_name, registry_aliases);
    let alias_applied = package_name != dep_name;
    let replace_string = image_ref(&dep_name, dep.tag.as_deref(), dep.digest.as_deref());
    let auto_replace_string_template = if alias_applied {
        format!(
            "{dep_name}:{{{{#if newValue}}}}{{{{newValue}}}}{{{{/if}}}}{{{{#if newDigest}}}}@{{{{newDigest}}}}{{{{/if}}}}"
        )
    } else {
        "{{depName}}{{#if newValue}}:{{newValue}}{{/if}}{{#if newDigest}}@{{newDigest}}{{/if}}"
            .to_owned()
    };

    AnsibleDockerDep {
        dep_name,
        package_name,
        current_value: dep.tag,
        current_digest: dep.digest,
        replace_string,
        auto_replace_string_template,
    }
}

fn apply_registry_alias(image: &str, registry_aliases: &[(&str, &str)]) -> String {
    let Some((registry, rest)) = image.split_once('/') else {
        return image.to_owned();
    };
    registry_aliases
        .iter()
        .find_map(|(from, to)| {
            if *from == registry {
                Some(format!("{to}/{rest}"))
            } else {
                None
            }
        })
        .unwrap_or_else(|| image.to_owned())
}

fn image_ref(image: &str, tag: Option<&str>, digest: Option<&str>) -> String {
    let mut value = image.to_owned();
    if let Some(tag) = tag {
        value.push(':');
        value.push_str(tag);
    }
    if let Some(digest) = digest {
        value.push('@');
        value.push_str(digest);
    }
    value
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

    // Ported: "extracts multiple image lines from docker_container" — lib/modules/manager/ansible/extract.spec.ts line 10
    #[test]
    fn extracts_images() {
        let deps = extract(SAMPLE);
        let nginx = deps.iter().find(|d| d.image == "nginx").unwrap();
        assert_eq!(nginx.tag.as_deref(), Some("1.25.3"));
        assert!(nginx.skip_reason.is_none());

        let pg = deps.iter().find(|d| d.image == "postgres").unwrap();
        assert_eq!(pg.tag.as_deref(), Some("15-alpine"));
    }

    // Ported: "extracts multiple image lines from docker_container" — lib/modules/manager/ansible/extract.spec.ts line 10
    #[test]
    fn skips_variable_images() {
        let deps = extract(SAMPLE);
        assert!(!deps.iter().any(|d| d.image.contains("MY_IMAGE")));
    }

    // Ported: "extracts multiple image lines from docker_container" — lib/modules/manager/ansible/extract.spec.ts line 10
    #[test]
    fn extracts_custom_registry_image() {
        let deps = extract(SAMPLE);
        assert!(deps.iter().any(|d| d.image.contains("myapp")));
    }

    // Ported: "returns null for empty" — lib/modules/manager/ansible/extract.spec.ts line 6
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "returns null for empty" — lib/modules/manager/ansible/extract.spec.ts line 6
    #[test]
    fn no_image_keys_returns_empty() {
        let content = "- name: task\n  shell: echo hello\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "extracts multiple image lines from docker_service" — lib/modules/manager/ansible/extract.spec.ts line 16
    #[test]
    fn extracts_docker_service_images() {
        let content = r#"---
- name: run containers
  docker_service:
    project_name: gitlab
    definition:
      services:
        gitlab:
          image: sameersbn/gitlab:11.5.1
        db:
          image: sameersbn/postgresql:10
        redis:
          image: sameersbn/redis:4.0.9-1
        registry:
          image: registry:2.6.2
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 4);
        assert!(
            deps.iter()
                .any(|d| d.image == "sameersbn/gitlab" && d.tag.as_deref() == Some("11.5.1"))
        );
        assert!(deps.iter().any(|d| d.image == "sameersbn/postgresql"));
        assert!(deps.iter().any(|d| d.image == "registry"));
    }

    // Ported: "extracts image and replaces registry" — lib/modules/manager/ansible/extract.spec.ts line 22
    #[test]
    fn extracts_image_and_replaces_registry() {
        let content = r#"---
- name: Re-create a redis container
  docker_container:
  name: myredis
  image: quay.io/redis:0.0.1
"#;
        let deps =
            extract_with_registry_aliases(content, &[("quay.io", "my-quay-mirror.registry.com")]);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "quay.io/redis");
        assert_eq!(deps[0].package_name, "my-quay-mirror.registry.com/redis");
        assert_eq!(deps[0].current_value.as_deref(), Some("0.0.1"));
        assert_eq!(deps[0].current_digest, None);
        assert_eq!(deps[0].replace_string, "quay.io/redis:0.0.1");
        assert_eq!(
            deps[0].auto_replace_string_template,
            "quay.io/redis:{{#if newValue}}{{newValue}}{{/if}}{{#if newDigest}}@{{newDigest}}{{/if}}"
        );
    }

    // Ported: "extracts image but no replacement" — lib/modules/manager/ansible/extract.spec.ts line 52
    #[test]
    fn extracts_image_without_registry_replacement() {
        let content = r#"---
- name: Re-create a redis container
  docker_container:
  name: myredis
  image: quay.io/redis:0.0.1
"#;
        let deps = extract_with_registry_aliases(
            content,
            &[("index.docker.io", "my-docker-mirror.registry.com")],
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "quay.io/redis");
        assert_eq!(deps[0].package_name, "quay.io/redis");
        assert_eq!(deps[0].current_value.as_deref(), Some("0.0.1"));
        assert_eq!(deps[0].replace_string, "quay.io/redis:0.0.1");
        assert_eq!(
            deps[0].auto_replace_string_template,
            "{{depName}}{{#if newValue}}:{{newValue}}{{/if}}{{#if newDigest}}@{{newDigest}}{{/if}}"
        );
    }

    // Ported: "extracts image and no double replacement" — lib/modules/manager/ansible/extract.spec.ts line 82
    #[test]
    fn extracts_image_without_double_registry_replacement() {
        let content = r#"---
- name: Re-create a redis container
  docker_container:
  name: myredis
  image: quay.io/redis:0.0.1
"#;
        let deps = extract_with_registry_aliases(
            content,
            &[
                ("quay.io", "my-quay-mirror.registry.com"),
                ("my-quay-mirror.registry.com", "quay.io"),
            ],
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "quay.io/redis");
        assert_eq!(deps[0].package_name, "my-quay-mirror.registry.com/redis");
        assert_eq!(deps[0].current_value.as_deref(), Some("0.0.1"));
        assert_eq!(
            deps[0].auto_replace_string_template,
            "quay.io/redis:{{#if newValue}}{{newValue}}{{/if}}{{#if newDigest}}@{{newDigest}}{{/if}}"
        );
    }
}
