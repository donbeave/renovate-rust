//! CircleCI `.circleci/config.yml` Docker image + Orb extractor.
//!
//! Scans CircleCI config files for:
//! 1. Docker image references under `docker:` executor sections.
//! 2. Orb references under the `orbs:` top-level block (`owner/name@version`).
//!
//! Renovate reference:
//! - `lib/modules/manager/circleci/extract.ts`
//! - Pattern: `/(^|/)\.circleci/.+\.ya?ml$/`
//!
//! ## Supported forms
//!
//! ```yaml
//! orbs:
//!   node: circleci/node@5.1.0
//!   aws-cli: circleci/aws-cli@3.1.4
//!
//! executors:
//!   my-executor:
//!     docker:
//!       - image: cimg/python:3.11
//!
//! jobs:
//!   build:
//!     docker:
//!       - image: cimg/node:18.0
//! ```
//!
//! ## What is NOT supported (deferred)
//!
//! - `machine:` executor (uses CircleCI VM images, not Docker Hub)
//! - Inline orbs (`orb:` blocks with `commands:`, `jobs:` etc.)

use std::{collections::HashMap, sync::LazyLock};

use regex::Regex;

use crate::extractors::dockerfile::{DockerfileExtractedDep, classify_image_ref};

/// A single Docker image reference extracted from a CircleCI config.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircleCiDep {
    pub dep: DockerfileExtractedDep,
}

/// Docker dep metadata after applying CircleCI registry alias config.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircleCiDockerDep {
    pub dep_name: String,
    pub package_name: String,
    pub current_value: Option<String>,
    pub current_digest: Option<String>,
    pub replace_string: String,
    pub auto_replace_string_template: String,
    pub dep_type: &'static str,
}

/// A single orb reference extracted from a CircleCI config `orbs:` block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircleCiOrbDep {
    /// Orb alias key (e.g. `"node"`).
    pub alias: String,
    /// Full orb package name (e.g. `"circleci/node"`).
    pub package_name: String,
    /// Version string (e.g. `"5.1.0"`).
    pub version: String,
}

// ── Compiled regexes ─────────────────────────────────────────────────────────

/// Matches `docker:` key (the executor block).
static DOCKER_KEY: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s*docker:\s*$").unwrap());

/// Matches `- image: ref` list item inside a docker block.
static IMAGE_ITEM: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*-\s+image:\s+(\S+.*)").unwrap());

/// Matches `image: ref` mapping entries, including entries inside anchored objects.
static IMAGE_ENTRY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*image:\s+(\S+.*)").unwrap());

/// Matches YAML anchor list items like `- &nodejs`.
static ANCHOR_ITEM: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*-\s+&([A-Za-z0-9_-]+)\s*$").unwrap());

/// Matches YAML alias list items like `- *nodejs`.
static ALIAS_ITEM: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*-\s+\*([A-Za-z0-9_-]+)\s*$").unwrap());

/// Extract Docker image deps from a CircleCI config YAML.
pub fn extract(content: &str) -> Vec<CircleCiDep> {
    let mut out = Vec::new();
    let mut in_docker_block = false;
    let mut docker_indent: usize = 0;
    let mut pending_anchor: Option<String> = None;
    let mut image_anchors: HashMap<String, String> = HashMap::new();

    for raw in content.lines() {
        // Strip inline comments.
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();

        if line.trim().is_empty() {
            continue;
        }

        let indent = leading_spaces(line);

        if let Some(cap) = ANCHOR_ITEM.captures(line) {
            pending_anchor = Some(cap[1].to_owned());
            continue;
        }

        if let Some(anchor) = pending_anchor.take()
            && let Some(cap) = IMAGE_ENTRY.captures(line)
        {
            let value = cap[1].trim().trim_matches('"').trim_matches('\'');
            if !value.is_empty() && !value.starts_with('$') {
                image_anchors.insert(anchor, value.to_owned());
                let dep = classify_image_ref(value);
                out.push(CircleCiDep { dep });
            }
            continue;
        }

        // Detect `docker:` key.
        if DOCKER_KEY.is_match(line) {
            in_docker_block = true;
            docker_indent = indent;
            continue;
        }

        if in_docker_block {
            if indent <= docker_indent && !line.trim_start().starts_with('-') {
                // Exited the docker block (non-list line at same or lower indent).
                in_docker_block = false;
            } else if let Some(cap) = IMAGE_ITEM.captures(line) {
                let value = cap[1].trim().trim_matches('"').trim_matches('\'');
                if !value.is_empty() && !value.starts_with('$') {
                    let dep = classify_image_ref(value);
                    out.push(CircleCiDep { dep });
                }
            } else if let Some(cap) = ALIAS_ITEM.captures(line)
                && let Some(value) = image_anchors.get(&cap[1])
            {
                let dep = classify_image_ref(value);
                out.push(CircleCiDep { dep });
            }
        }
    }

    out
}

/// Extract Docker image deps and apply Renovate-style registry aliases.
pub fn extract_with_registry_aliases(
    content: &str,
    registry_aliases: &[(&str, &str)],
) -> Vec<CircleCiDockerDep> {
    extract(content)
        .into_iter()
        .map(|dep| circleci_docker_dep(dep.dep, registry_aliases))
        .collect()
}

/// Extract orb references from the `orbs:` block of a CircleCI config.
///
/// Supports the simple `alias: owner/name@version` form only.
/// Inline orbs (map values) are skipped.
pub fn extract_orbs(content: &str) -> Vec<CircleCiOrbDep> {
    let mut out = Vec::new();
    let mut in_orbs = false;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        let trimmed = line.trim_start();

        if trimmed.is_empty() {
            continue;
        }

        let indent = leading_spaces(line);

        if trimmed == "orbs:" {
            in_orbs = true;
            continue;
        }

        // Exit orbs block on next top-level key (indent 0, not a list item).
        if indent == 0 && !trimmed.starts_with('-') && in_orbs {
            in_orbs = false;
            continue;
        }

        if !in_orbs {
            continue;
        }

        // `alias: owner/name@version` — value must contain `@`
        if let Some((alias, rest)) = trimmed.split_once(':') {
            let val = rest.trim().trim_matches('"').trim_matches('\'');
            if let Some((pkg, version)) = val.split_once('@') {
                let pkg = pkg.trim();
                let version = version.trim();
                if !pkg.is_empty() && !version.is_empty() && pkg.contains('/') {
                    out.push(CircleCiOrbDep {
                        alias: alias.trim().to_owned(),
                        package_name: pkg.to_owned(),
                        version: version.to_owned(),
                    });
                }
            }
        }
    }

    out
}

/// Update a single CircleCI orb dependency in a config YAML file.
///
/// Mirrors `updateDependency()` from `lib/modules/manager/circleci/update.ts`.
///
/// Returns `Some(updated_content)` when the replacement was made, or `None`
/// when the orb could not be found or matched.
pub fn circleci_update_orb(
    file_content: &str,
    package_name: &str,
    current_version: &str,
    new_version: &str,
) -> Option<String> {
    let old_ref = format!("{package_name}@{current_version}");
    let new_ref = format!("{package_name}@{new_version}");
    let mut content = file_content.to_owned();
    let mut found = false;

    for line in file_content.lines() {
        let raw = line.split(" #").next().unwrap_or(line).trim_end();
        if let Some((_, rest)) = raw.split_once(':') {
            let val = rest.trim().trim_matches('"').trim_matches('\'');
            if val == old_ref {
                let new_line = line.replace(&old_ref, &new_ref);
                content = content.replacen(line, &new_line, 1);
                found = true;
                break;
            }
        }
    }

    if found { Some(content) } else { None }
}

fn leading_spaces(s: &str) -> usize {
    s.len() - s.trim_start_matches([' ', '\t']).len()
}

fn circleci_docker_dep(
    dep: DockerfileExtractedDep,
    registry_aliases: &[(&str, &str)],
) -> CircleCiDockerDep {
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

    CircleCiDockerDep {
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

/// Determine the effective CircleCI range strategy.
///
/// Mirrors `lib/modules/manager/circleci/range.ts` `getRangeStrategy()`.
pub fn get_range_strategy(range_strategy: &str) -> &str {
    if range_strategy == "auto" {
        "pin"
    } else {
        range_strategy
    }
}

/// CircleCI manager file pattern.
///
/// Mirrors `managerFilePatterns` in `lib/modules/manager/circleci/index.ts`.
static MANAGER_FILE_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(^|/)\.circleci/.+\.ya?ml$").unwrap());

/// Returns true if the path matches CircleCI's file pattern.
pub fn matches_file_pattern(path: &str) -> bool {
    MANAGER_FILE_PATTERN.is_match(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r"
version: 2.1

orbs:
  node: circleci/node@5.1.0
  aws: circleci/aws-cli@3.1.4

executors:
  python-executor:
    docker:
      - image: cimg/python:3.11
      - image: postgres:15
        environment:
          POSTGRES_USER: test

jobs:
  build:
    docker:
      - image: cimg/node:18.0
    steps:
      - checkout
      - run: npm install

  test:
    docker:
      - image: golang:1.21-alpine
    steps:
      - run: go test ./...

  deploy:
    docker:
      - image: $CI_IMAGE
";

    // Ported: "extracts multiple image and resolves yaml anchors" — circleci/extract.spec.ts line 48
    #[test]
    fn extracts_executor_images() {
        let deps = extract(SAMPLE);
        let py = deps.iter().find(|d| d.dep.image == "cimg/python").unwrap();
        assert_eq!(py.dep.tag.as_deref(), Some("3.11"));
        assert!(py.dep.skip_reason.is_none());

        let pg = deps.iter().find(|d| d.dep.image == "postgres").unwrap();
        assert_eq!(pg.dep.tag.as_deref(), Some("15"));
    }

    // Ported: "extracts multiple image and resolves yaml anchors" — circleci/extract.spec.ts line 48
    #[test]
    fn extracts_job_images() {
        let deps = extract(SAMPLE);
        let node = deps.iter().find(|d| d.dep.image == "cimg/node").unwrap();
        assert_eq!(node.dep.tag.as_deref(), Some("18.0"));

        let go = deps.iter().find(|d| d.dep.image == "golang").unwrap();
        assert_eq!(go.dep.tag.as_deref(), Some("1.21-alpine"));
    }

    // Ported: "extracts multiple image and resolves yaml anchors" — circleci/extract.spec.ts line 48
    #[test]
    fn fixture_config_resolves_yaml_anchor_images() {
        let content = r#"workflows:
  version: 2
  node-multi-build:
    jobs:
      - node-v4
      - node-v6
      - node-v8

version: 2
jobs:
  node-base: &node-base
    docker:
      - image: node

  node-v4:
    <<: *node-base
    docker:
      - image: 'node:4'
  node-v6:
    <<: *node-base
    docker:
      - image: node:6
  node-v8:
    <<: *node-base
    docker:
      - image: "node:8.9.0"
"#;
        let deps = extract_with_registry_aliases(content, &[]);
        assert_eq!(deps.len(), 4);
        assert_eq!(deps[0].dep_name, "node");
        assert_eq!(deps[0].current_value, None);
        assert_eq!(deps[0].replace_string, "node");
        assert_eq!(deps[1].current_value.as_deref(), Some("4"));
        assert_eq!(deps[1].replace_string, "node:4");
        assert_eq!(deps[2].current_value.as_deref(), Some("6"));
        assert_eq!(deps[2].replace_string, "node:6");
        assert_eq!(deps[3].current_value.as_deref(), Some("8.9.0"));
        assert_eq!(deps[3].replace_string, "node:8.9.0");
    }

    // Ported: "extracts image without leading dash" — circleci/extract.spec.ts line 200
    #[test]
    fn anchor_image_without_leading_dash_is_resolved() {
        let content = r#"aliases:
  - &nodejs
    image: cimg/node:14.8.0

version: 2
jobs:
  checkout:
    docker:
      - *nodejs
"#;
        let deps = extract_with_registry_aliases(content, &[]);
        assert_eq!(deps.len(), 2);
        for dep in deps {
            assert_eq!(dep.dep_name, "cimg/node");
            assert_eq!(dep.package_name, "cimg/node");
            assert_eq!(dep.current_value.as_deref(), Some("14.8.0"));
            assert_eq!(dep.replace_string, "cimg/node:14.8.0");
        }
    }

    // Ported: "extracts and exclude android images" — circleci/extract.spec.ts line 226
    #[test]
    fn skips_variable_image() {
        let deps = extract(SAMPLE);
        // $CI_IMAGE should be filtered out (starts with $)
        assert!(!deps.iter().any(|d| d.dep.image.starts_with('$')));
    }

    // Ported: "returns null for empty" — circleci/extract.spec.ts line 12
    #[test]
    fn empty_content_returns_no_deps() {
        assert!(extract("").is_empty());
    }

    // Ported: "handles registry alias" — circleci/extract.spec.ts line 16
    #[test]
    fn handles_registry_alias() {
        let content = r#"
executors:
  my-executor:
    docker:
      - image: quay.io/myName/myPackage:0.6.2
"#;
        let deps = extract_with_registry_aliases(
            content,
            &[
                ("quay.io", "my-quay-mirror.registry.com"),
                ("index.docker.io", "my-docker-mirror.registry.com"),
            ],
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_type, "docker");
        assert_eq!(deps[0].dep_name, "quay.io/myName/myPackage");
        assert_eq!(
            deps[0].package_name,
            "my-quay-mirror.registry.com/myName/myPackage"
        );
        assert_eq!(deps[0].current_value.as_deref(), Some("0.6.2"));
        assert_eq!(deps[0].current_digest, None);
        assert_eq!(deps[0].replace_string, "quay.io/myName/myPackage:0.6.2");
        assert_eq!(
            deps[0].auto_replace_string_template,
            "quay.io/myName/myPackage:{{#if newValue}}{{newValue}}{{/if}}{{#if newDigest}}@{{newDigest}}{{/if}}"
        );
    }

    // Ported: "extracts orbs without jobs" — circleci/extract.spec.ts line 237
    #[test]
    fn no_docker_section_returns_no_deps() {
        let content =
            "version: 2.1\njobs:\n  build:\n    machine:\n      image: ubuntu-2204:2022.04.1\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "extracts orbs too" — circleci/extract.spec.ts line 93
    #[test]
    fn extracts_orbs() {
        let orbs = extract_orbs(SAMPLE);
        assert_eq!(orbs.len(), 2);
        let node = orbs.iter().find(|o| o.alias == "node").unwrap();
        assert_eq!(node.package_name, "circleci/node");
        assert_eq!(node.version, "5.1.0");
        let aws = orbs.iter().find(|o| o.alias == "aws").unwrap();
        assert_eq!(aws.package_name, "circleci/aws-cli");
        assert_eq!(aws.version, "3.1.4");
    }

    // Rust-specific: circleci behavior test
    #[test]
    fn orbs_without_at_sign_skipped() {
        let content = "orbs:\n  local: {commands: {run: {steps: []}}}\n";
        assert!(extract_orbs(content).is_empty());
    }

    // Ported: "extracts orb definitions" — circleci/extract.spec.ts line 273
    #[test]
    fn orbs_block_ends_at_next_top_level_key() {
        let content = "orbs:\n  node: circleci/node@5.0.0\njobs:\n  build:\n    docker:\n      - image: ubuntu:20.04\n";
        let orbs = extract_orbs(content);
        assert_eq!(orbs.len(), 1);
    }

    // Ported: "returns null for empty" — circleci/extract.spec.ts line 12
    #[test]
    fn empty_orbs_returns_empty() {
        assert!(extract_orbs("").is_empty());
    }

    // Ported: "returns null for empty" — circleci/extract.spec.ts line 12
    #[test]
    fn no_orbs_block_returns_empty() {
        let content = "version: 2.1\njobs:\n  build:\n    docker:\n      - image: ubuntu:20.04\n";
        assert!(extract_orbs(content).is_empty());
    }

    // Ported: "extracts and exclude android images" — circleci/extract.spec.ts line 226
    #[test]
    fn machine_image_not_extracted() {
        let content = "jobs:\n  build:\n    machine:\n      image: android:202102-01\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "extracts executors" — circleci/extract.spec.ts line 251
    #[test]
    fn executor_docker_image_extracted() {
        let content =
            "executors:\n  my-executor:\n    docker:\n      - image: cimg/ruby:3.0.3-browsers\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep.image, "cimg/ruby");
        assert_eq!(deps[0].dep.tag.as_deref(), Some("3.0.3-browsers"));
    }

    // Ported: "extracts orbs without jobs" — circleci/extract.spec.ts line 237
    #[test]
    fn extracts_orbs_without_jobs() {
        // config4.yml: only `orbs:` and `workflows:`, no jobs
        let content = "version: 2.1\n\norbs:\n  nodejs: circleci/node@5.2.0\n\nworkflows:\n  Test:\n    jobs:\n      - nodejs/test\n";
        let orbs = extract_orbs(content);
        assert_eq!(orbs.len(), 1);
        assert_eq!(orbs[0].alias, "nodejs");
        assert_eq!(orbs[0].package_name, "circleci/node");
        assert_eq!(orbs[0].version, "5.2.0");
        // No docker images
        assert!(extract(content).is_empty());
    }

    // Ported: "extracts orb definitions" — circleci/extract.spec.ts line 273
    #[test]
    fn extracts_orb_definitions() {
        let content = r#"version: 2.1

orbs:
  myorb:
    orbs:
      python: circleci/python@2.1.1

    executors:
      python:
        docker:
          - image: cimg/python:3.9

    jobs:
      test_image:
        docker:
          - image: cimg/python:3.7
        steps:
          - checkout

workflows:
  Test:
    jobs:
      - myorb/test_image
"#;
        let orbs = extract_orbs(content);
        assert_eq!(orbs.len(), 1);
        assert_eq!(orbs[0].alias, "python");
        assert_eq!(orbs[0].package_name, "circleci/python");
        assert_eq!(orbs[0].version, "2.1.1");

        let docker = extract(content);
        assert_eq!(docker.len(), 2);
        assert!(
            docker
                .iter()
                .any(|d| d.dep.image == "cimg/python" && d.dep.tag.as_deref() == Some("3.9"))
        );
        assert!(
            docker
                .iter()
                .any(|d| d.dep.image == "cimg/python" && d.dep.tag.as_deref() == Some("3.7"))
        );
    }

    // Ported: "returns same if not auto" — modules/manager/circleci/range.spec.ts line 5
    #[test]
    fn circleci_range_returns_same_if_not_auto() {
        assert_eq!(get_range_strategy("widen"), "widen");
    }

    // Ported: "defaults to bump" — modules/manager/circleci/range.spec.ts line 10
    #[test]
    fn circleci_range_defaults_to_pin() {
        assert_eq!(get_range_strategy("auto"), "pin");
    }

    // Ported: "matchRegexOrGlobList("$path") === $expected" — modules/manager/circleci/index.spec.ts line 6
    #[test]
    fn circleci_file_pattern_matches_expected_paths() {
        let should_match = [
            ".circleci/config.yml",
            ".circleci/config.yaml",
            ".circleci/foo.yaml",
            ".circleci/foo.yml",
            ".circleci/foo/config.yaml",
            ".circleci/foo/bar.yml",
            "foo/.circleci/bar.yaml",
        ];
        let should_not_match = [
            "foo.yml",
            "circleci/foo.yml",
            ".circleci_foo/bar.yml",
            ".circleci/foo.toml",
        ];
        for path in &should_match {
            assert!(matches_file_pattern(path), "expected match for {path}");
        }
        for path in &should_not_match {
            assert!(!matches_file_pattern(path), "expected no match for {path}");
        }
    }

    // Rust-specific: circleci orb update behavior tests
    #[test]
    fn circleci_update_orb_replaces_version() {
        let content = "orbs:\n  node: circleci/node@5.1.0\n";
        let updated = circleci_update_orb(content, "circleci/node", "5.1.0", "5.2.0");
        assert_eq!(
            updated,
            Some("orbs:\n  node: circleci/node@5.2.0\n".to_owned())
        );
    }

    #[test]
    fn circleci_update_orb_no_match() {
        let content = "orbs:\n  node: circleci/node@5.1.0\n";
        let updated = circleci_update_orb(content, "circleci/python", "2.1.1", "2.2.0");
        assert_eq!(updated, None);
    }

    #[test]
    fn circleci_update_orb_quoted() {
        let content = "orbs:\n  node: \"circleci/node@5.1.0\"\n";
        let updated = circleci_update_orb(content, "circleci/node", "5.1.0", "5.2.0");
        assert_eq!(
            updated,
            Some("orbs:\n  node: \"circleci/node@5.2.0\"\n".to_owned())
        );
    }

    #[test]
    fn circleci_update_orb_multiple_orbs() {
        let content = "orbs:\n  node: circleci/node@5.1.0\n  python: circleci/python@2.1.1\n";
        let updated = circleci_update_orb(content, "circleci/python", "2.1.1", "2.2.0");
        assert_eq!(
            updated,
            Some(
                "orbs:\n  node: circleci/node@5.1.0\n  python: circleci/python@2.2.0\n".to_owned()
            )
        );
    }
}
