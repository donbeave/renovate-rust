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

/// Docker dep metadata after applying Woodpecker registry alias config.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WoodpeckerDockerDep {
    pub dep_name: String,
    pub package_name: String,
    pub current_value: Option<String>,
    pub current_digest: Option<String>,
    pub replace_string: String,
    pub auto_replace_string_template: String,
}

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

/// Extract Docker deps and apply Renovate-style registry aliases.
pub fn extract_with_registry_aliases(
    content: &str,
    registry_aliases: &[(&str, &str)],
) -> Vec<WoodpeckerDockerDep> {
    extract(content)
        .into_iter()
        .map(|dep| woodpecker_docker_dep(dep, registry_aliases))
        .collect()
}

fn woodpecker_docker_dep(
    dep: DockerfileExtractedDep,
    registry_aliases: &[(&str, &str)],
) -> WoodpeckerDockerDep {
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

    WoodpeckerDockerDep {
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

fn strip_key<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}:");
    line.strip_prefix(prefix.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "extracts multiple image lines" — lib/modules/manager/woodpecker/extract.spec.ts line 21
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

    // Ported: "extracts multiple image lines" — lib/modules/manager/woodpecker/extract.spec.ts line 21
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

    // Ported: "extracts multiple image lines" — lib/modules/manager/woodpecker/extract.spec.ts line 21
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

    // Ported: "extracts image and replaces registry" — lib/modules/manager/woodpecker/extract.spec.ts line 129
    #[test]
    fn extracts_image_and_replaces_registry() {
        let content = r#"
pipeline:
  nginx:
    image: quay.io/nginx:0.0.1
"#;
        let deps =
            extract_with_registry_aliases(content, &[("quay.io", "my-quay-mirror.registry.com")]);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "quay.io/nginx");
        assert_eq!(deps[0].package_name, "my-quay-mirror.registry.com/nginx");
        assert_eq!(deps[0].current_value.as_deref(), Some("0.0.1"));
        assert_eq!(deps[0].replace_string, "quay.io/nginx:0.0.1");
        assert_eq!(
            deps[0].auto_replace_string_template,
            "quay.io/nginx:{{#if newValue}}{{newValue}}{{/if}}{{#if newDigest}}@{{newDigest}}{{/if}}"
        );
    }

    // Ported: "extracts image but no replacement" — lib/modules/manager/woodpecker/extract.spec.ts line 159
    #[test]
    fn extracts_image_without_registry_replacement() {
        let content = r#"
pipeline:
  nginx:
    image: quay.io/nginx:0.0.1
"#;
        let deps = extract_with_registry_aliases(
            content,
            &[("index.docker.io", "my-docker-mirror.registry.com")],
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "quay.io/nginx");
        assert_eq!(deps[0].package_name, "quay.io/nginx");
        assert_eq!(
            deps[0].auto_replace_string_template,
            "{{depName}}{{#if newValue}}:{{newValue}}{{/if}}{{#if newDigest}}@{{newDigest}}{{/if}}"
        );
    }

    // Ported: "extracts image and no double replacement" — lib/modules/manager/woodpecker/extract.spec.ts line 189
    #[test]
    fn extracts_image_without_double_registry_replacement() {
        let content = r#"
pipeline:
  nginx:
    image: quay.io/nginx:0.0.1
"#;
        let deps = extract_with_registry_aliases(
            content,
            &[
                ("quay.io", "my-quay-mirror.registry.com"),
                ("my-quay-mirror.registry.com", "quay.io"),
            ],
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "quay.io/nginx");
        assert_eq!(deps[0].package_name, "my-quay-mirror.registry.com/nginx");
    }

    // Ported: "extracts multiple image lines" — lib/modules/manager/woodpecker/extract.spec.ts line 21
    #[test]
    fn variable_ref_skipped() {
        let content = "steps:\n  ci:\n    image: ${CI_IMAGE}\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert!(deps[0].skip_reason.is_some());
    }

    // Ported: "returns null for empty" — lib/modules/manager/woodpecker/extract.spec.ts line 8
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "returns null for non-object YAML" — lib/modules/manager/woodpecker/extract.spec.ts line 12
    #[test]
    fn non_object_yaml_returns_empty() {
        assert!(extract("nothing here").is_empty());
        assert!(extract("clone: null").is_empty());
    }

    // Ported: "return null when no dependencies are provided" — lib/modules/manager/woodpecker/extract.spec.ts line 313
    #[test]
    fn no_steps_or_services_returns_empty() {
        let content = "pipeline: {}\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "returns null for malformed YAML" — lib/modules/manager/woodpecker/extract.spec.ts line 17
    #[test]
    fn malformed_yaml_returns_empty() {
        let content = "nothing here\n:::::::\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "extracts the v.1.0.x version" — lib/modules/manager/woodpecker/extract.spec.ts line 220
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

    // Ported: "should parse multiple sources of dependencies together" — lib/modules/manager/woodpecker/extract.spec.ts line 246
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

    // Ported: "return dependency when an plugin-git is cloned" — lib/modules/manager/woodpecker/extract.spec.ts line 286
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
