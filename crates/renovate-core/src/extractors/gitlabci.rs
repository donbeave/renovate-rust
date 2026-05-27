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

/// Docker dep metadata after applying GitLab CI registry aliases.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitlabCiDockerDep {
    pub dep_name: String,
    pub package_name: String,
    pub current_value: Option<String>,
    pub current_digest: Option<String>,
    pub dep_type: &'static str,
    pub replace_string: String,
    pub auto_replace_string_template: String,
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
    let mut seeking_service_name = false;
    // Indent level of the first service list item (filters out nested list items).
    let mut service_list_indent: Option<usize> = None;
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
            seeking_service_name = false;
            service_list_indent = None;
            continue;
        }

        // Collect service list items (only when inside `services:` block).
        if in_services_block {
            if let Some(cap) = SERVICE_ITEM.captures(line) {
                // Only process service items at the established list indent level.
                let list_indent = *service_list_indent.get_or_insert(indent);
                if indent != list_indent {
                    // Nested list item (e.g. command: sub-list) — ignore.
                } else {
                    seeking_service_name = false;
                    let captured = cap[1].trim();
                    if let Some(name_val) = captured.strip_prefix("name:").map(str::trim) {
                        // "- name: IMAGE"
                        let image = name_val.trim_matches('"').trim_matches('\'');
                        if !image.is_empty() {
                            let effective = if image.starts_with('$') {
                                strip_dependency_proxy_prefix(image)
                            } else {
                                Some(image)
                            };
                            if let Some(eff) = effective {
                                let dep = classify_image_ref(eff);
                                out.push(GitlabCiDep { dep });
                            }
                        }
                    } else if is_service_yaml_key(captured) {
                        // Other service-object key (alias, command, …) — name may follow
                        seeking_service_name = true;
                    } else {
                        // Plain image reference
                        let image_str = captured.trim_matches('"').trim_matches('\'');
                        if !image_str.is_empty() {
                            let effective = if image_str.starts_with('$') {
                                strip_dependency_proxy_prefix(image_str)
                            } else {
                                Some(image_str)
                            };
                            if let Some(eff) = effective {
                                let dep = classify_image_ref(eff);
                                out.push(GitlabCiDep { dep });
                            }
                        }
                    }
                }
            } else if seeking_service_name {
                if let Some(cap) = IMAGE_NAME.captures(line) {
                    let value = cap[1].trim().trim_matches('"').trim_matches('\'');
                    if !value.is_empty() {
                        let effective = if value.starts_with('$') {
                            strip_dependency_proxy_prefix(value)
                        } else {
                            Some(value)
                        };
                        if let Some(eff) = effective {
                            let dep = classify_image_ref(eff);
                            out.push(GitlabCiDep { dep });
                        }
                        seeking_service_name = false;
                    }
                }
            } else if indent == 0 {
                // New top-level key exits services block.
                in_services_block = false;
                service_list_indent = None;
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

/// Extract Docker image deps and apply Renovate-style registry aliases.
pub fn extract_docker_with_registry_aliases(
    content: &str,
    registry_aliases: &[(&str, &str)],
) -> Vec<GitlabCiDockerDep> {
    let mut out = Vec::new();
    let mut in_image_block = false;
    let mut in_services_block = false;
    let mut seeking_service_name = false;
    let mut service_list_indent: Option<usize> = None;
    let mut image_indent: usize = 0;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        if line.trim().is_empty() {
            continue;
        }

        let indent = leading_spaces(line);
        let trimmed = line.trim_start();

        if trimmed == "services:" {
            in_services_block = true;
            in_image_block = false;
            seeking_service_name = false;
            service_list_indent = None;
            continue;
        }

        if in_services_block {
            if let Some(cap) = SERVICE_ITEM.captures(line) {
                let list_indent = *service_list_indent.get_or_insert(indent);
                if indent != list_indent {
                    // Nested list item — ignore.
                } else {
                    seeking_service_name = false;
                    let captured = cap[1].trim();
                    if let Some(name_val) = captured.strip_prefix("name:").map(str::trim) {
                        let image = unquote(name_val);
                        if !image.is_empty() {
                            out.push(gitlab_docker_dep(image, "service-image", registry_aliases));
                        }
                    } else if is_service_yaml_key(captured) {
                        seeking_service_name = true;
                    } else {
                        let image = unquote(captured);
                        if !image.is_empty() {
                            out.push(gitlab_docker_dep(image, "service-image", registry_aliases));
                        }
                    }
                }
            } else if seeking_service_name {
                if let Some(cap) = IMAGE_NAME.captures(line) {
                    let image = unquote(cap[1].trim());
                    if !image.is_empty() {
                        out.push(gitlab_docker_dep(image, "service-image", registry_aliases));
                        seeking_service_name = false;
                    }
                }
            } else if indent == 0 {
                in_services_block = false;
                service_list_indent = None;
            }
        }

        if let Some(cap) = IMAGE_INLINE.captures(line) {
            let image = unquote(cap[1].trim());
            if !image.is_empty() {
                in_image_block = false;
                out.push(gitlab_docker_dep(image, "image-name", registry_aliases));
            }
            continue;
        }

        if IMAGE_KEY_ONLY.is_match(line) {
            in_image_block = true;
            image_indent = indent;
            continue;
        }

        if in_image_block {
            if indent <= image_indent {
                in_image_block = false;
            } else if let Some(cap) = IMAGE_NAME.captures(line) {
                let image = unquote(cap[1].trim());
                if !image.is_empty() {
                    out.push(gitlab_docker_dep(image, "image-name", registry_aliases));
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

/// True when a service list-item capture looks like a YAML key attribute
/// (not a Docker image reference). These attribute names indicate we are
/// inside a service-object block, not a plain string reference.
fn is_service_yaml_key(s: &str) -> bool {
    let key = s.split(':').next().unwrap_or("");
    !key.is_empty()
        && !key.contains('/')
        && !key.contains(' ')
        && !key.contains('@')
        && matches!(
            key,
            "name" | "alias" | "command" | "entrypoint"
                | "variables" | "pull_policy" | "docker"
        )
}

/// A GitLab CI component reference extracted from `include: - component:`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitlabCiComponentDep {
    /// The owner/repo path of the component (excludes host and component name).
    pub dep_name: String,
    /// The version/ref after `@`.
    pub current_value: String,
    /// Registry URL derived from the host part (`https://{host}`).
    pub registry_url: String,
    /// Skip reason (e.g. `unsupported-version` for `~latest`).
    pub skip_reason: Option<&'static str>,
}

/// Extract GitLab CI component references from `include: - component:` entries.
///
/// Format: `{host}/{owner}/{repo}/{component}@{version}`
/// - `dep_name` = `{owner}/{repo}` (all path segments except host and component)
/// - `registry_url` = `https://{host}`
/// - `current_value` = `{version}`
pub fn extract_components(content: &str) -> Vec<GitlabCiComponentDep> {
    extract_components_with_registry_aliases(content, &[])
}

/// Extract GitLab CI component references and apply registry aliases to hosts.
pub fn extract_components_with_registry_aliases(
    content: &str,
    registry_aliases: &[(&str, &str)],
) -> Vec<GitlabCiComponentDep> {
    let mut out = Vec::new();
    let mut in_include = false;
    let mut in_include_item = false;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        if line.trim().is_empty() {
            continue;
        }
        let trimmed = line.trim_start();
        let indent = leading_spaces(line);

        // Detect `include:` top-level block.
        if indent == 0 && trimmed == "include:" {
            in_include = true;
            in_include_item = false;
            continue;
        }

        // Exit include block when we return to indent 0.
        if indent == 0 && !trimmed.starts_with('-') {
            in_include = false;
            in_include_item = false;
        }

        if !in_include {
            continue;
        }

        // New include item.
        if let Some(after_dash) = trimmed.strip_prefix("- ") {
            in_include_item = true;
            let rest = after_dash.trim();
            if let Some(val) = rest.strip_prefix("component:") {
                let val = val.trim().trim_matches('"').trim_matches('\'');
                if let Some(dep) = parse_component_ref(val, registry_aliases) {
                    out.push(dep);
                }
            }
            continue;
        }

        // Continuation key inside current list item.
        if in_include_item && let Some(val) = trimmed.strip_prefix("component:") {
            let val = val.trim().trim_matches('"').trim_matches('\'');
            // Only parse if it's a scalar (not a nested object starting with `{`).
            if !val.is_empty()
                && !val.starts_with('{')
                && let Some(dep) = parse_component_ref(val, registry_aliases)
            {
                out.push(dep);
            }
        }
    }

    out
}

/// Parse a `host/owner/.../component@version` component reference.
///
/// Returns `None` for malformed references (missing `@`, too few path segments,
/// or dep_name with no `/`).
fn parse_component_ref(s: &str, registry_aliases: &[(&str, &str)]) -> Option<GitlabCiComponentDep> {
    let (path_part, version) = s.split_once('@')?;
    if version.is_empty() || path_part.is_empty() {
        return None;
    }

    let path_part = apply_registry_alias(path_part, registry_aliases);
    let segments: Vec<&str> = path_part.split('/').collect();
    // Need: host + at least 2 path segments + component = 4 total.
    if segments.len() < 4 {
        return None;
    }

    let host = segments[0];
    // component = last segment; dep_name = everything in between
    let dep_name = segments[1..segments.len() - 1].join("/");

    // dep_name must contain at least one `/` (owner/repo).
    if !dep_name.contains('/') {
        return None;
    }

    let registry_url = format!("https://{host}");

    let skip_reason = if version.starts_with('~') {
        Some("unsupported-version")
    } else {
        None
    };

    Some(GitlabCiComponentDep {
        dep_name,
        current_value: version.to_owned(),
        registry_url,
        skip_reason,
    })
}

fn gitlab_docker_dep(
    image_ref: &str,
    dep_type: &'static str,
    registry_aliases: &[(&str, &str)],
) -> GitlabCiDockerDep {
    let (dep_name, current_value, current_digest) = split_image_ref(image_ref);
    let package_name = apply_registry_alias(&dep_name, registry_aliases);
    let replace_string = image_ref_string(
        &dep_name,
        current_value.as_deref(),
        current_digest.as_deref(),
    );
    let auto_replace_string_template = format!(
        "{dep_name}:{{{{#if newValue}}}}{{{{newValue}}}}{{{{/if}}}}{{{{#if newDigest}}}}@{{{{newDigest}}}}{{{{/if}}}}"
    );

    GitlabCiDockerDep {
        dep_name,
        package_name,
        current_value,
        current_digest,
        dep_type,
        replace_string,
        auto_replace_string_template,
    }
}

fn split_image_ref(image_ref: &str) -> (String, Option<String>, Option<String>) {
    let (ref_no_digest, digest) = if let Some(at) = image_ref.find('@') {
        (&image_ref[..at], Some(image_ref[at + 1..].to_owned()))
    } else {
        (image_ref, None)
    };

    let (image, tag) = if let Some(colon) = ref_no_digest.rfind(':') {
        let slash_pos = ref_no_digest.rfind('/').unwrap_or(0);
        if colon > slash_pos {
            (
                ref_no_digest[..colon].to_owned(),
                Some(ref_no_digest[colon + 1..].to_owned()),
            )
        } else {
            (ref_no_digest.to_owned(), None)
        }
    } else {
        (ref_no_digest.to_owned(), None)
    };

    (image, tag, digest)
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

fn image_ref_string(image: &str, tag: Option<&str>, digest: Option<&str>) -> String {
    let mut out = image.to_owned();
    if let Some(tag) = tag {
        out.push(':');
        out.push_str(tag);
    }
    if let Some(digest) = digest {
        out.push('@');
        out.push_str(digest);
    }
    out
}

fn unquote(s: &str) -> &str {
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

/// Default auto-replace template used by `getGitlabDep` for unaliased images.
pub const DEFAULT_AUTO_REPLACE_TEMPLATE: &str =
    "{{depName}}{{#if newValue}}:{{newValue}}{{/if}}{{#if newDigest}}@{{newDigest}}{{/if}}";

/// Build the auto-replace template from a replace-string and version/digest.
///
/// Mirrors TypeScript `getAutoReplaceTemplate` in `dockerfile/extract.ts`.
fn get_auto_replace_template(
    replace_string: &str,
    current_value: Option<&str>,
    current_digest: Option<&str>,
) -> String {
    let mut template = replace_string.to_owned();
    if let Some(cv) = current_value {
        let placeholder = if current_digest.is_none() {
            "{{#if newValue}}{{newValue}}{{/if}}{{#if newDigest}}@{{newDigest}}{{/if}}"
        } else {
            "{{#if newValue}}{{newValue}}{{/if}}"
        };
        template = template.replace(cv, placeholder);
    }
    if let Some(cd) = current_digest {
        template = template.replace(cd, "{{#if newDigest}}{{newDigest}}{{/if}}");
    }
    template
}

/// Create a [`GitlabCiDockerDep`] using the same logic as TypeScript's `getGitlabDep`.
///
/// Handles:
/// - CI Dependency Proxy prefixes (`$CI_DEPENDENCY_PROXY_*_IMAGE_PREFIX/`)
/// - Registry aliases (alias_name → alias_value)
/// - Plain image references
pub fn get_gitlab_dep(image_name: &str, registry_aliases: &[(&str, &str)]) -> GitlabCiDockerDep {
    // Check for CI_DEPENDENCY_PROXY prefix
    if let Some(prefix_end) = dep_proxy_prefix_end(image_name) {
        let prefix = &image_name[..prefix_end];
        let image = &image_name[prefix_end..];
        let (dep_name, current_value, current_digest) = split_image_ref(image);
        return GitlabCiDockerDep {
            package_name: dep_name.clone(),
            dep_name,
            current_value,
            current_digest,
            dep_type: "image-name",
            replace_string: image_name.to_owned(),
            auto_replace_string_template: format!("{prefix}{DEFAULT_AUTO_REPLACE_TEMPLATE}"),
        };
    }

    // Check registry aliases
    for (alias_name, alias_value) in registry_aliases {
        let prefix = format!("{alias_name}/");
        if let Some(image_part) = image_name.strip_prefix(prefix.as_str()) {
            let (img_only, current_value, current_digest) = split_image_ref(image_part);
            let dep_name = format!("{alias_name}/{img_only}");
            let package_name = format!("{alias_value}/{img_only}");
            let auto_replace_string_template = get_auto_replace_template(
                image_name,
                current_value.as_deref(),
                current_digest.as_deref(),
            );
            return GitlabCiDockerDep {
                dep_name,
                package_name,
                current_value,
                current_digest,
                dep_type: "image-name",
                replace_string: image_name.to_owned(),
                auto_replace_string_template,
            };
        }
    }

    // Plain image — no proxy prefix, no alias
    let (dep_name, current_value, current_digest) = split_image_ref(image_name);
    GitlabCiDockerDep {
        package_name: dep_name.clone(),
        dep_name,
        current_value,
        current_digest,
        dep_type: "image-name",
        replace_string: image_name.to_owned(),
        auto_replace_string_template: DEFAULT_AUTO_REPLACE_TEMPLATE.to_owned(),
    }
}

/// Returns the byte offset just past the CI Dependency Proxy prefix slash, or `None`.
fn dep_proxy_prefix_end(s: &str) -> Option<usize> {
    // ${CI_DEPENDENCY_PROXY_*}/image
    if let Some(rest) = s.strip_prefix("${") {
        if let Some(slash_pos) = rest.find("}/")
            && rest[..slash_pos].starts_with("CI_DEPENDENCY_PROXY")
        {
            return Some(2 + slash_pos + 2); // "${" + var + "}/"
        }
        return None;
    }
    // $CI_DEPENDENCY_PROXY_*/image
    if let Some(rest) = s.strip_prefix('$')
        && let Some(slash_pos) = rest.find('/')
        && rest[..slash_pos].starts_with("CI_DEPENDENCY_PROXY")
    {
        return Some(1 + slash_pos + 1); // "$" + var + "/"
    }
    None
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
    if let Some(rest) = s.strip_prefix('$')
        && let Some(slash_pos) = rest.find('/')
    {
        let var_name = &rest[..slash_pos];
        if var_name.starts_with("CI_DEPENDENCY_PROXY") {
            return Some(&rest[slash_pos + 1..]);
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

    // Ported: "extract images via registry aliases" — gitlabci/extract.spec.ts line 229
    #[test]
    fn extract_images_via_registry_aliases() {
        let content = r#"
image:
  name: $CI_REGISTRY/renovate/renovate:31.65.1-slim

services:
  - foo/mariadb:10.4.11
  - name: $CI_REGISTRY/other/image1:1.0.0
    alias: imagealias1
  - $BUILD_IMAGES/image2:1.0.0
"#;
        let deps = extract_docker_with_registry_aliases(
            content,
            &[
                ("$CI_REGISTRY", "registry.com"),
                ("$BUILD_IMAGES", "registry.com/build-images"),
                ("foo", "foo.registry.com"),
            ],
        );

        assert_eq!(deps.len(), 4);
        assert_eq!(deps[0].dep_name, "$CI_REGISTRY/renovate/renovate");
        assert_eq!(deps[0].package_name, "registry.com/renovate/renovate");
        assert_eq!(deps[0].current_value.as_deref(), Some("31.65.1-slim"));
        assert_eq!(deps[0].dep_type, "image-name");
        assert_eq!(
            deps[0].replace_string,
            "$CI_REGISTRY/renovate/renovate:31.65.1-slim"
        );
        assert_eq!(
            deps[0].auto_replace_string_template,
            "$CI_REGISTRY/renovate/renovate:{{#if newValue}}{{newValue}}{{/if}}{{#if newDigest}}@{{newDigest}}{{/if}}"
        );

        assert_eq!(deps[1].dep_name, "foo/mariadb");
        assert_eq!(deps[1].package_name, "foo.registry.com/mariadb");
        assert_eq!(deps[1].current_value.as_deref(), Some("10.4.11"));
        assert_eq!(deps[1].dep_type, "service-image");

        assert_eq!(deps[2].dep_name, "$CI_REGISTRY/other/image1");
        assert_eq!(deps[2].package_name, "registry.com/other/image1");
        assert_eq!(deps[2].current_value.as_deref(), Some("1.0.0"));
        assert_eq!(deps[2].dep_type, "service-image");

        assert_eq!(deps[3].dep_name, "$BUILD_IMAGES/image2");
        assert_eq!(deps[3].package_name, "registry.com/build-images/image2");
        assert_eq!(deps[3].current_value.as_deref(), Some("1.0.0"));
        assert_eq!(deps[3].dep_type, "service-image");
    }

    // Ported: "extracts component references via registry aliases" — gitlabci/extract.spec.ts line 299
    #[test]
    fn extracts_component_references_via_registry_aliases() {
        let content = r#"include:
  - component: $CI_SERVER_HOST/an-org/a-project/a-component@1.0
    inputs:
      stage: build
  - component: $CI_SERVER_HOST/an-org/a-subgroup/a-project/a-component@e3262fdd0914fa823210cdb79a8c421e2cef79d8
  - component: $CI_SERVER_HOST/an-org/a-subgroup/another-project/a-component@main
  - component: $CI_SERVER_HOST/another-org/a-project/a-component@~latest
    inputs:
      stage: test
  - component: $CI_SERVER_HOST/malformed-component-reference
  - component:
      malformed: true
  - component: $CI_SERVER_HOST/an-org/a-component@1.0
  - component: other-gitlab.example.com/an-org/a-project/a-component@1.0
  - component: $COMPONENT_REGISTRY/a-project/a-component@1.0
"#;
        let deps = extract_components_with_registry_aliases(
            content,
            &[
                ("$CI_SERVER_HOST", "gitlab.example.com"),
                ("$COMPONENT_REGISTRY", "gitlab.example.com/a-group"),
            ],
        );

        assert_eq!(deps.len(), 6);
        assert_eq!(deps[0].dep_name, "an-org/a-project");
        assert_eq!(deps[0].current_value, "1.0");
        assert_eq!(deps[0].registry_url, "https://gitlab.example.com");

        assert_eq!(deps[1].dep_name, "an-org/a-subgroup/a-project");
        assert_eq!(
            deps[1].current_value,
            "e3262fdd0914fa823210cdb79a8c421e2cef79d8"
        );
        assert_eq!(deps[1].registry_url, "https://gitlab.example.com");

        assert_eq!(deps[2].dep_name, "an-org/a-subgroup/another-project");
        assert_eq!(deps[2].current_value, "main");
        assert_eq!(deps[2].registry_url, "https://gitlab.example.com");

        assert_eq!(deps[3].dep_name, "another-org/a-project");
        assert_eq!(deps[3].current_value, "~latest");
        assert_eq!(deps[3].registry_url, "https://gitlab.example.com");
        assert_eq!(deps[3].skip_reason, Some("unsupported-version"));

        assert_eq!(deps[4].dep_name, "an-org/a-project");
        assert_eq!(deps[4].registry_url, "https://other-gitlab.example.com");

        assert_eq!(deps[5].dep_name, "a-group/a-project");
        assert_eq!(deps[5].current_value, "1.0");
        assert_eq!(deps[5].registry_url, "https://gitlab.example.com");
    }

    // Ported: "extracts component references" — gitlabci/extract.spec.ts line 377
    #[test]
    fn extracts_component_references() {
        let content = r#"include:
  - component: gitlab.example.com/an-org/a-project/a-component@1.0
    inputs:
      stage: build
  - component: gitlab.example.com/an-org/a-subgroup/a-project/a-component@e3262fdd0914fa823210cdb79a8c421e2cef79d8
  - component: gitlab.example.com/an-org/a-subgroup/another-project/a-component@main
  - component: gitlab.example.com/another-org/a-project/a-component@~latest
    inputs:
      stage: test
  - component: gitlab.example.com/malformed-component-reference
  - component:
      malformed: true
  - component: gitlab.example.com/an-org/a-component@1.0
  - component: other-gitlab.example.com/an-org/a-project/a-component@1.0
"#;
        let deps = extract_components(content);

        assert_eq!(deps.len(), 5);

        // First dep: an-org/a-project@1.0
        assert_eq!(deps[0].dep_name, "an-org/a-project");
        assert_eq!(deps[0].current_value, "1.0");
        assert_eq!(deps[0].registry_url, "https://gitlab.example.com");
        assert!(deps[0].skip_reason.is_none());

        // Second dep: an-org/a-subgroup/a-project@sha
        assert_eq!(deps[1].dep_name, "an-org/a-subgroup/a-project");
        assert_eq!(
            deps[1].current_value,
            "e3262fdd0914fa823210cdb79a8c421e2cef79d8"
        );
        assert!(deps[1].skip_reason.is_none());

        // Third dep: @main (no skip)
        assert_eq!(deps[2].dep_name, "an-org/a-subgroup/another-project");
        assert_eq!(deps[2].current_value, "main");
        assert!(deps[2].skip_reason.is_none());

        // Fourth dep: ~latest → unsupported-version
        assert_eq!(deps[3].dep_name, "another-org/a-project");
        assert_eq!(deps[3].current_value, "~latest");
        assert_eq!(deps[3].skip_reason, Some("unsupported-version"));

        // Fifth dep: other-gitlab.example.com
        assert_eq!(deps[4].dep_name, "an-org/a-project");
        assert_eq!(deps[4].registry_url, "https://other-gitlab.example.com");

        // Malformed entries skipped:
        // - malformed-component-reference (no @)
        // - malformed: true (object, not string)
        // - an-org/a-component@1.0 (dep_name has no /)
        assert!(!deps.iter().any(|d| d.dep_name == "an-org"));
    }

    // --- get_gitlab_dep tests ---

    // Ported: "offical image - $name" — gitlabci/utils.spec.ts line 11
    #[test]
    fn get_gitlab_dep_official_image_no_prefix() {
        let dep = get_gitlab_dep("mariadb:10.4.11", &[]);
        assert_eq!(dep.dep_name, "mariadb");
        assert_eq!(dep.current_value.as_deref(), Some("10.4.11"));
        assert_eq!(dep.replace_string, "mariadb:10.4.11");
        assert_eq!(
            dep.auto_replace_string_template,
            DEFAULT_AUTO_REPLACE_TEMPLATE
        );
    }

    #[test]
    fn get_gitlab_dep_official_image_group_proxy() {
        let image = "$CI_DEPENDENCY_PROXY_GROUP_IMAGE_PREFIX/mariadb:10.4.11";
        let dep = get_gitlab_dep(image, &[]);
        assert_eq!(dep.dep_name, "mariadb");
        assert_eq!(dep.current_value.as_deref(), Some("10.4.11"));
        assert_eq!(dep.replace_string, image);
        assert_eq!(
            dep.auto_replace_string_template,
            format!("$CI_DEPENDENCY_PROXY_GROUP_IMAGE_PREFIX/{DEFAULT_AUTO_REPLACE_TEMPLATE}")
        );
    }

    #[test]
    fn get_gitlab_dep_official_image_group_proxy_brackets() {
        let image = "${CI_DEPENDENCY_PROXY_GROUP_IMAGE_PREFIX}/mariadb:10.4.11";
        let dep = get_gitlab_dep(image, &[]);
        assert_eq!(dep.dep_name, "mariadb");
        assert_eq!(dep.replace_string, image);
        assert_eq!(
            dep.auto_replace_string_template,
            format!("${{CI_DEPENDENCY_PROXY_GROUP_IMAGE_PREFIX}}/{DEFAULT_AUTO_REPLACE_TEMPLATE}")
        );
    }

    #[test]
    fn get_gitlab_dep_official_image_direct_group_proxy() {
        let image = "$CI_DEPENDENCY_PROXY_DIRECT_GROUP_IMAGE_PREFIX/mariadb:10.4.11";
        let dep = get_gitlab_dep(image, &[]);
        assert_eq!(dep.dep_name, "mariadb");
        assert_eq!(
            dep.auto_replace_string_template,
            format!(
                "$CI_DEPENDENCY_PROXY_DIRECT_GROUP_IMAGE_PREFIX/{DEFAULT_AUTO_REPLACE_TEMPLATE}"
            )
        );
    }

    // Ported: "image with organization - $name" — gitlabci/utils.spec.ts line 28
    #[test]
    fn get_gitlab_dep_image_with_org_no_prefix() {
        let dep = get_gitlab_dep("renovate/renovate:19.70.8-slim", &[]);
        assert_eq!(dep.dep_name, "renovate/renovate");
        assert_eq!(dep.current_value.as_deref(), Some("19.70.8-slim"));
        assert_eq!(dep.replace_string, "renovate/renovate:19.70.8-slim");
        assert_eq!(
            dep.auto_replace_string_template,
            DEFAULT_AUTO_REPLACE_TEMPLATE
        );
    }

    #[test]
    fn get_gitlab_dep_image_with_org_group_proxy() {
        let image = "$CI_DEPENDENCY_PROXY_GROUP_IMAGE_PREFIX/renovate/renovate:19.70.8-slim";
        let dep = get_gitlab_dep(image, &[]);
        assert_eq!(dep.dep_name, "renovate/renovate");
        assert_eq!(dep.current_value.as_deref(), Some("19.70.8-slim"));
        assert_eq!(dep.replace_string, image);
        assert_eq!(
            dep.auto_replace_string_template,
            format!("$CI_DEPENDENCY_PROXY_GROUP_IMAGE_PREFIX/{DEFAULT_AUTO_REPLACE_TEMPLATE}")
        );
    }

    // Ported: "supports registry aliases - $name" — gitlabci/utils.spec.ts line 48
    #[test]
    fn get_gitlab_dep_registry_alias_multiple() {
        let dep = get_gitlab_dep(
            "foo/image:1.0",
            &[("foo", "foo.registry.com"), ("bar", "bar.registry.com")],
        );
        assert_eq!(dep.dep_name, "foo/image");
        assert_eq!(dep.package_name, "foo.registry.com/image");
        assert_eq!(dep.current_value.as_deref(), Some("1.0"));
        assert_eq!(
            dep.auto_replace_string_template,
            "foo/image:{{#if newValue}}{{newValue}}{{/if}}{{#if newDigest}}@{{newDigest}}{{/if}}"
        );
    }

    #[test]
    fn get_gitlab_dep_registry_alias_variable() {
        let dep = get_gitlab_dep(
            "$CI_REGISTRY/image:1.0",
            &[("$CI_REGISTRY", "registry.com")],
        );
        assert_eq!(dep.dep_name, "$CI_REGISTRY/image");
        assert_eq!(dep.package_name, "registry.com/image");
        assert_eq!(dep.current_value.as_deref(), Some("1.0"));
        assert_eq!(
            dep.auto_replace_string_template,
            "$CI_REGISTRY/image:{{#if newValue}}{{newValue}}{{/if}}{{#if newDigest}}@{{newDigest}}{{/if}}"
        );
    }

    #[test]
    fn get_gitlab_dep_registry_alias_variable_brackets() {
        let dep = get_gitlab_dep(
            "${CI_REGISTRY}/image:1.0",
            &[("${CI_REGISTRY}", "registry.com")],
        );
        assert_eq!(dep.dep_name, "${CI_REGISTRY}/image");
        assert_eq!(dep.package_name, "registry.com/image");
        assert_eq!(
            dep.auto_replace_string_template,
            "${CI_REGISTRY}/image:{{#if newValue}}{{newValue}}{{/if}}{{#if newDigest}}@{{newDigest}}{{/if}}"
        );
    }

    #[test]
    fn get_gitlab_dep_registry_alias_not_aliased_variable() {
        // $CI_REGISTRY/image:1.0 with empty alias list → treated as plain image
        let dep = get_gitlab_dep("$CI_REGISTRY/image:1.0", &[]);
        assert_eq!(
            dep.auto_replace_string_template,
            DEFAULT_AUTO_REPLACE_TEMPLATE
        );
    }

    #[test]
    fn get_gitlab_dep_registry_alias_plain_image() {
        let dep = get_gitlab_dep("registry.com/image:1.0", &[]);
        assert_eq!(dep.dep_name, "registry.com/image");
        assert_eq!(dep.current_value.as_deref(), Some("1.0"));
        assert_eq!(
            dep.auto_replace_string_template,
            DEFAULT_AUTO_REPLACE_TEMPLATE
        );
    }

    // Ported: "no Docker hub" — gitlabci/utils.spec.ts line 73
    #[test]
    fn get_gitlab_dep_no_docker_hub() {
        let dep = get_gitlab_dep("quay.io/prometheus/node-exporter:v1.3.1", &[]);
        assert_eq!(dep.dep_name, "quay.io/prometheus/node-exporter");
        assert_eq!(dep.current_value.as_deref(), Some("v1.3.1"));
        assert_eq!(
            dep.replace_string,
            "quay.io/prometheus/node-exporter:v1.3.1"
        );
        assert_eq!(
            dep.auto_replace_string_template,
            DEFAULT_AUTO_REPLACE_TEMPLATE
        );
    }

    // Ported: "returns null for empty" — gitlabci/extract.spec.ts line 28
    #[test]
    fn extract_all_returns_empty_for_empty_content() {
        assert!(extract("").is_empty());
    }

    // Ported: "extracts multiple named services" — gitlabci/extract.spec.ts line 66
    #[test]
    fn extracts_multiple_named_services() {
        let content = r#"image:
  # comment
  name: renovate/renovate:19.70.8-slim

services:
  # comment
  - name: other/image1:1.0.0
    alias: imagealias1
  # another comment
  - alias: imagealias2
    name: other/image2:1.0.0
job1:
    services:
        - name: mooseagency/postgresql:12.3-1@sha256:a5a65569456f221ee1f8a0b3b4e2d440eb5830772d9440c9b30b1dbfd454c778
          command:
              - something:thatIsNotAnImage
              - -something2
job2:
    services:
      - "mariadb:10.4.11"
      - postgres:11.7
      - redis:latest
      - name: "registry.example.com/myimage:latest"
      - myimage@sha256:0ecb2ad60
      - tomcat:7-jre8
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 10);
        assert!(deps.iter().any(|d| d.dep.image == "renovate/renovate"));
        assert!(deps.iter().any(|d| d.dep.image == "other/image1"));
        assert!(deps.iter().any(|d| d.dep.image == "other/image2"));
        assert!(deps.iter().any(|d| d.dep.image == "mooseagency/postgresql"));
        assert!(deps.iter().any(|d| d.dep.image == "mariadb"));
        assert!(deps.iter().any(|d| d.dep.image == "postgres"));
        assert!(deps.iter().any(|d| d.dep.image == "redis"));
        assert!(deps.iter().any(|d| d.dep.image == "registry.example.com/myimage"));
        assert!(deps.iter().any(|d| d.dep.image == "myimage"));
        assert!(deps.iter().any(|d| d.dep.image == "tomcat"));
    }

    // Ported: "catches errors" — gitlabci/extract.spec.ts line 110
    #[test]
    fn catches_errors_returns_empty() {
        // Invalid YAML with duplicate key and unknown tags — extractor returns empty
        let content = "include:\n  - local: 'some/file.yml'\n\ntest: test:\n  script:\n    - !abc [.setup, script]\n";
        let deps = extract(content);
        assert!(deps.is_empty());
    }
}
