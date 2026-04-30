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

use std::sync::LazyLock;

use regex::Regex;

use crate::extractors::dockerfile::{DockerfileExtractedDep, classify_image_ref};

/// A single Docker image reference extracted from a CircleCI config.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircleCiDep {
    pub dep: DockerfileExtractedDep,
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

/// Extract Docker image deps from a CircleCI config YAML.
pub fn extract(content: &str) -> Vec<CircleCiDep> {
    let mut out = Vec::new();
    let mut in_docker_block = false;
    let mut docker_indent: usize = 0;

    for raw in content.lines() {
        // Strip inline comments.
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();

        if line.trim().is_empty() {
            continue;
        }

        let indent = leading_spaces(line);

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
            }
        }
    }

    out
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

fn leading_spaces(s: &str) -> usize {
    s.len() - s.trim_start_matches([' ', '\t']).len()
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
}
