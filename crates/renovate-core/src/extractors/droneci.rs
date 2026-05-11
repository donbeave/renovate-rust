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

/// Docker dep metadata after applying DroneCI registry alias config.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DroneDockerDep {
    pub dep_name: String,
    pub package_name: String,
    pub current_value: Option<String>,
    pub current_digest: Option<String>,
    pub replace_string: String,
    pub auto_replace_string_template: String,
    pub dep_type: &'static str,
}

/// Extract Docker image deps from a Drone CI `.drone.yml` file.
///
/// Returns one dep for every `image:` key found anywhere in the file.
/// Handles multi-line quoted images with backslash continuation, e.g.:
/// ```yaml
/// image: "registry/image:tag\
///         @sha256:abc123"
/// ```
pub fn extract(content: &str) -> Vec<DockerfileExtractedDep> {
    let mut out = Vec::new();
    // When Some, we're accumulating a multi-line quoted image.
    let mut multiline: Option<String> = None;

    for raw in content.lines() {
        let line = raw.trim_end();

        // Continuing a multi-line image value.
        if let Some(ref mut acc) = multiline {
            let trimmed = line.trim();
            if let Some(middle) = trimmed.strip_suffix('\\') {
                // Middle continuation line — strip trailing `\`, accumulate.
                acc.push_str(middle.trim());
            } else {
                // Final line — strip closing quote and emit.
                let part = trimmed.trim_end_matches('"').trim_end_matches('\'');
                acc.push_str(part.trim());
                let image = acc.clone();
                multiline = None;
                if !image.is_empty() {
                    out.push(classify_image_ref(&image));
                }
            }
            continue;
        }

        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            continue;
        }

        // Strip optional `- ` list prefix before checking for `image:`.
        let key_line = trimmed.strip_prefix("- ").unwrap_or(trimmed);
        if let Some(val) = strip_key(key_line, "image") {
            let val = val.trim();
            // Detect start of a multi-line quoted image: starts with `"` or `'`, ends with `\`.
            if val.ends_with('\\') && (val.starts_with('"') || val.starts_with('\'')) {
                let partial = &val[1..val.len() - 1]; // strip opening quote and trailing `\`
                multiline = Some(partial.to_owned());
            } else {
                let image = val.trim_matches('"').trim_matches('\'');
                if !image.is_empty() {
                    out.push(classify_image_ref(image));
                }
            }
        }
    }

    out
}

/// Extract Docker deps and apply Renovate-style registry aliases.
pub fn extract_with_registry_aliases(
    content: &str,
    registry_aliases: &[(&str, &str)],
) -> Vec<DroneDockerDep> {
    extract(content)
        .into_iter()
        .map(|dep| drone_docker_dep(dep, registry_aliases))
        .collect()
}

fn drone_docker_dep(
    dep: DockerfileExtractedDep,
    registry_aliases: &[(&str, &str)],
) -> DroneDockerDep {
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

    DroneDockerDep {
        dep_name,
        package_name,
        current_value: dep.tag,
        current_digest: dep.digest,
        replace_string,
        auto_replace_string_template,
        dep_type: "docker",
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

fn strip_key<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}:");
    line.strip_prefix(prefix.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "extracts multiple image lines" — droneci/extract.spec.ts line 12
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

    // Ported: "extracts multiple image lines" — droneci/extract.spec.ts line 12
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

    // Ported: "extracts multiple image lines" — droneci/extract.spec.ts line 12
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

    // Ported: "extracts image but no replacement" — droneci/extract.spec.ts line 42
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

    // Ported: "extracts multiple image lines" — droneci/extract.spec.ts line 12
    #[test]
    fn extracts_drone_fixture_six_deps() {
        // Mirrors the .drone.yml fixture: steps (2) + services (4, including 2 multiline quoted).
        let content = r#"kind: pipeline
name: Test

steps:
  - name: mix
    image: elixir:1.8.1-alpine

  - name: node
    image: amd64/node:10.0.0@sha256:36adc17e9cceab32179d3314da9cb9c737ffb11f0de4e688f407ad6d9ca32201

services:
  - name: mysql
    image: mysql:5.7.24

  - name: redis
    image: redis:alpine

  - name: node
    image: "amd64/node:10.0.0\
            @sha256:36adc17e9cceab32179d3314da9cb9c737ffb11f0de4e688f407ad6d9ca32201"

  - name: node
    image: 'amd64/node\
            :10.0.0\
            @sha256:36adc17e9cceab32179d3314da9cb9c737ffb11f0de4e688f407ad6d9ca32201'
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 6);
        assert!(deps.iter().any(|d| d.image == "elixir"));
        assert!(deps.iter().any(|d| d.image == "mysql"));
        assert!(deps.iter().any(|d| d.image == "redis"));
        // The multiline forms resolve to the same image as the plain form.
        assert_eq!(deps.iter().filter(|d| d.image == "amd64/node").count(), 3);
    }

    // Ported: "extracts image and replaces registry" — droneci/extract.spec.ts line 19
    #[test]
    fn extracts_image_and_replaces_registry() {
        let content = "steps:\n- image: quay.io/elixir:1.8.1-alpine\n";
        let deps =
            extract_with_registry_aliases(content, &[("quay.io", "my-quay-mirror.registry.com")]);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_type, "docker");
        assert_eq!(deps[0].dep_name, "quay.io/elixir");
        assert_eq!(deps[0].package_name, "my-quay-mirror.registry.com/elixir");
        assert_eq!(deps[0].current_value.as_deref(), Some("1.8.1-alpine"));
        assert_eq!(deps[0].current_digest, None);
        assert_eq!(deps[0].replace_string, "quay.io/elixir:1.8.1-alpine");
        assert_eq!(
            deps[0].auto_replace_string_template,
            "quay.io/elixir:{{#if newValue}}{{newValue}}{{/if}}{{#if newDigest}}@{{newDigest}}{{/if}}"
        );
    }

    // Ported: "extracts image but no replacement" — droneci/extract.spec.ts line 42
    #[test]
    fn extracts_image_without_registry_replacement() {
        let content = "steps:\n- image: quay.io/elixir:1.8.1-alpine\n";
        let deps = extract_with_registry_aliases(
            content,
            &[("index.docker.io", "my-docker-mirror.registry.com")],
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_type, "docker");
        assert_eq!(deps[0].dep_name, "quay.io/elixir");
        assert_eq!(deps[0].package_name, "quay.io/elixir");
        assert_eq!(deps[0].current_value.as_deref(), Some("1.8.1-alpine"));
        assert_eq!(deps[0].replace_string, "quay.io/elixir:1.8.1-alpine");
        assert_eq!(
            deps[0].auto_replace_string_template,
            "{{depName}}{{#if newValue}}:{{newValue}}{{/if}}{{#if newDigest}}@{{newDigest}}{{/if}}"
        );
    }

    // Ported: "extracts image and no double replacement" — droneci/extract.spec.ts line 65
    #[test]
    fn extracts_image_without_double_registry_replacement() {
        let content = "steps:\n- image: quay.io/elixir:1.8.1-alpine\n";
        let deps = extract_with_registry_aliases(
            content,
            &[
                ("quay.io", "my-quay-mirror.registry.com"),
                ("my-quay-mirror.registry.com", "quay.io"),
            ],
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_type, "docker");
        assert_eq!(deps[0].dep_name, "quay.io/elixir");
        assert_eq!(deps[0].package_name, "my-quay-mirror.registry.com/elixir");
        assert_eq!(deps[0].current_value.as_deref(), Some("1.8.1-alpine"));
        assert_eq!(
            deps[0].auto_replace_string_template,
            "quay.io/elixir:{{#if newValue}}{{newValue}}{{/if}}{{#if newDigest}}@{{newDigest}}{{/if}}"
        );
    }
}
