//! GitLab CI `.gitlab-ci.yml` Docker image extractor.
//!
//! Scans `.gitlab-ci.yml` (and included YAML files) for `image:` keys and
//! extracts Docker image references for Docker Hub version lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/gitlabci/extract.ts`
//! - Pattern: `/\.gitlab-ci\.ya?ml$/`
//!
//! ## Supported forms
//!
//! ```yaml
//! image: node:18-alpine
//!
//! build:
//!   image: python:3.11
//!
//! deploy:
//!   image:
//!     name: registry.example.com/myapp:latest
//!     entrypoint: [""]
//!
//! services:
//!   - postgres:15
//! ```
//!
//! ## What is NOT supported
//!
//! - CircleCI orbs / Ansible Galaxy (different datasources not yet implemented)
//! - GitLab CI Components (`include: component`)
//! - Variable substitution (`image: $MY_IMAGE`)

use std::sync::LazyLock;

use regex::Regex;

use crate::extractors::dockerfile::{DockerfileExtractedDep, classify_image_ref};

/// A single Docker image reference extracted from a GitLab CI file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitlabCiDep {
    pub dep: DockerfileExtractedDep,
}

// ── Compiled regexes ─────────────────────────────────────────────────────────

/// Matches `image: ref` with an inline value.
static IMAGE_INLINE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*image:\s+(\S+.*)").unwrap());

/// Matches `image:` with NO inline value (block form follows).
static IMAGE_KEY_ONLY: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s*image:\s*$").unwrap());

/// Matches `name: ref` inside an `image:` block.
static IMAGE_NAME: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s*name:\s+(\S+.*)").unwrap());

/// Matches a YAML list item `- image_ref` under `services:`.
static SERVICE_ITEM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s*-\s+(\S+.*)").unwrap());

/// Extract Docker image deps from a GitLab CI YAML file.
pub fn extract(content: &str) -> Vec<GitlabCiDep> {
    let mut out = Vec::new();
    let mut in_image_block = false;
    let mut in_services_block = false;
    // Track indentation level of the `image:` key to detect when the block ends.
    let mut image_indent: usize = 0;

    for raw in content.lines() {
        // Strip inline comments.
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();

        if line.trim().is_empty() {
            continue;
        }

        let indent = leading_spaces(line);
        let trimmed = line.trim_start();

        // Detect `services:` block (list of Docker images).
        if trimmed == "services:" {
            in_services_block = true;
            in_image_block = false;
            continue;
        }

        // Collect service list items (only when inside `services:` block).
        if in_services_block {
            if let Some(cap) = SERVICE_ITEM.captures(line) {
                let image_str = cap[1].trim().trim_matches('"').trim_matches('\'');
                let effective = if image_str.starts_with('$') {
                    strip_dependency_proxy_prefix(image_str)
                } else if image_str.is_empty() {
                    None
                } else {
                    Some(image_str)
                };
                if let Some(eff) = effective {
                    let dep = classify_image_ref(eff);
                    out.push(GitlabCiDep { dep });
                }
            } else if indent == 0 {
                // New top-level key exits services block.
                in_services_block = false;
            }
        }

        // Detect `image: ref` (inline form).
        if let Some(cap) = IMAGE_INLINE.captures(line) {
            let value = cap[1].trim().trim_matches('"').trim_matches('\'');
            if !value.is_empty() {
                in_image_block = false;
                // Dependency proxy variables are stripped; other variables are
                // passed as-is so classify_image_ref can assign a skip reason.
                let effective = if value.starts_with('$') {
                    strip_dependency_proxy_prefix(value).unwrap_or(value)
                } else {
                    value
                };
                let dep = classify_image_ref(effective);
                out.push(GitlabCiDep { dep });
            }
            continue;
        }

        // Detect `image:` with no inline value (block form follows).
        if IMAGE_KEY_ONLY.is_match(line) {
            in_image_block = true;
            image_indent = indent;
            continue;
        }

        // Inside an `image:` block — look for `name: ref`.
        if in_image_block {
            if indent <= image_indent {
                // Exited the block.
                in_image_block = false;
            } else if let Some(cap) = IMAGE_NAME.captures(line) {
                let value = cap[1].trim().trim_matches('"').trim_matches('\'');
                if !value.is_empty() {
                    let effective = if value.starts_with('$') {
                        strip_dependency_proxy_prefix(value).unwrap_or(value)
                    } else {
                        value
                    };
                    let dep = classify_image_ref(effective);
                    out.push(GitlabCiDep { dep });
                    in_image_block = false;
                }
            }
        }
    }

    out
}

fn leading_spaces(s: &str) -> usize {
    s.len() - s.trim_start_matches([' ', '\t']).len()
}

/// If the image reference uses a GitLab CI Dependency Proxy prefix variable,
/// strip the prefix and return the actual image path.
/// Handles: `${CI_DEPENDENCY_PROXY_*}/image:tag` and `$CI_DEPENDENCY_PROXY_*/image:tag`.
fn strip_dependency_proxy_prefix(s: &str) -> Option<&str> {
    // ${CI_DEPENDENCY_PROXY_*}/image
    if let Some(rest) = s.strip_prefix("${") {
        if let Some(slash_pos) = rest.find("}/") {
            let var_name = &rest[..slash_pos];
            if var_name.starts_with("CI_DEPENDENCY_PROXY") {
                return Some(&rest[slash_pos + 2..]);
            }
        }
        return None;
    }
    // $CI_DEPENDENCY_PROXY_*/image
    if let Some(rest) = s.strip_prefix('$') {
        if let Some(slash_pos) = rest.find('/') {
            let var_name = &rest[..slash_pos];
            if var_name.starts_with("CI_DEPENDENCY_PROXY") {
                return Some(&rest[slash_pos + 1..]);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::extractors::dockerfile::DockerfileSkipReason;

    use super::*;

    const SAMPLE: &str = r"
# GitLab CI configuration

image: node:18-alpine

stages:
  - build
  - test
  - deploy

services:
  - postgres:15
  - redis:7-alpine

build:
  image: golang:1.21
  script:
    - go build ./...

test:
  image:
    name: python:3.11-slim
    entrypoint: ['']
  script:
    - python -m pytest

deploy:
  image: $CI_REGISTRY_IMAGE:latest
  script:
    - deploy.sh

scratch_job:
  image: scratch
";

    // Ported: "extracts multiple image lines" — gitlabci/extract.spec.ts line 75
    #[test]
    fn extracts_top_level_image() {
        let deps = extract(SAMPLE);
        let node = deps.iter().find(|d| d.dep.image == "node").unwrap();
        assert_eq!(node.dep.tag.as_deref(), Some("18-alpine"));
        assert!(node.dep.skip_reason.is_none());
    }

    // Ported: "extracts multiple image lines" — gitlabci/extract.spec.ts line 75
    #[test]
    fn extracts_job_image() {
        let deps = extract(SAMPLE);
        let go = deps.iter().find(|d| d.dep.image == "golang").unwrap();
        assert_eq!(go.dep.tag.as_deref(), Some("1.21"));
    }

    // Ported: "extracts multiple image lines" — gitlabci/extract.spec.ts line 75
    #[test]
    fn extracts_block_form_image() {
        let deps = extract(SAMPLE);
        let py = deps.iter().find(|d| d.dep.image == "python").unwrap();
        assert_eq!(py.dep.tag.as_deref(), Some("3.11-slim"));
    }

    // Ported: "extracts named services" — gitlabci/extract.spec.ts line 57
    #[test]
    fn extracts_services() {
        let deps = extract(SAMPLE);
        let pg = deps.iter().find(|d| d.dep.image == "postgres").unwrap();
        assert_eq!(pg.dep.tag.as_deref(), Some("15"));

        let redis = deps.iter().find(|d| d.dep.image == "redis").unwrap();
        assert_eq!(redis.dep.tag.as_deref(), Some("7-alpine"));
    }

    // Ported: "extracts multiple image lines" — gitlabci/extract.spec.ts line 75
    #[test]
    fn scratch_image_skipped() {
        let deps = extract(SAMPLE);
        let scratch = deps
            .iter()
            .find(|d| d.dep.skip_reason == Some(DockerfileSkipReason::Scratch));
        assert!(scratch.is_some());
    }

    // Ported: "extracts from empty file" — gitlabci/extract.spec.ts line 22
    #[test]
    fn empty_content_returns_no_deps() {
        assert!(extract("").is_empty());
    }

    // Ported: "skips images with variables" — gitlabci/extract.spec.ts line 118
    #[test]
    fn variable_image_has_skip_reason() {
        let content = "image: $VARIABLE/renovate/renovate:31.65.1-slim\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert!(deps[0].dep.skip_reason.is_some());
    }

    // Ported: "extracts from multidoc yaml" — gitlabci/extract.spec.ts line 36
    #[test]
    fn multidoc_yaml_extracts_from_all_docs() {
        let content = "image: node:18\n---\nimage: python:3.11\n---\nimage: golang:1.21\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
    }

    // Ported: "extracts multiple image lines with comments" — gitlabci/extract.spec.ts line 94
    #[test]
    fn extracts_images_with_comment_lines() {
        let content = r#"image:
  # comment
  name: renovate/renovate:19.70.8-slim

services:
  # comment
  - mariadb:10.4.11
  # another comment
  - other/image:1.0.0
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
        assert!(
            deps.iter().any(|d| d.dep.image == "renovate/renovate"
                && d.dep.tag.as_deref() == Some("19.70.8-slim"))
        );
        assert!(deps.iter().any(|d| d.dep.image == "mariadb"));
        assert!(deps.iter().any(|d| d.dep.image == "other/image"));
    }

    // Ported: "extract images from dependency proxy" — gitlabci/extract.spec.ts line 172
    #[test]
    fn dependency_proxy_prefix_stripped() {
        // Inline image with ${CI_DEPENDENCY_PROXY_GROUP_IMAGE_PREFIX} prefix in block form
        let content = r#"image:
  name: ${CI_DEPENDENCY_PROXY_GROUP_IMAGE_PREFIX}/renovate/renovate:31.65.1-slim

services:
  - $CI_DEPENDENCY_PROXY_DIRECT_GROUP_IMAGE_PREFIX/mariadb:10.4.11
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(
            deps.iter()
                .any(|d| d.dep.image == "renovate/renovate" && d.dep.skip_reason.is_none())
        );
        assert!(
            deps.iter()
                .any(|d| d.dep.image == "mariadb" && d.dep.skip_reason.is_none())
        );
    }
}
