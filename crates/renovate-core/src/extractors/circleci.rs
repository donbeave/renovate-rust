//! CircleCI `.circleci/config.yml` Docker image extractor.
//!
//! Scans CircleCI config files for Docker image references under `docker:`
//! executor sections. Orb dependencies are noted but deferred (require a
//! specialized CircleCI Orb datasource).
//!
//! Renovate reference:
//! - `lib/modules/manager/circleci/extract.ts`
//! - Pattern: `/(^|/)\.circleci/.+\.ya?ml$/`
//!
//! ## Supported forms
//!
//! ```yaml
//! executors:
//!   my-executor:
//!     docker:
//!       - image: cimg/python:3.11
//!       - image: postgres:15
//!
//! jobs:
//!   build:
//!     docker:
//!       - image: cimg/node:18.0
//! ```
//!
//! ## What is NOT supported (deferred)
//!
//! - `orbs:` section — requires CircleCI Orb API datasource
//! - `machine:` executor (uses CircleCI VM images, not Docker Hub)

use std::sync::LazyLock;

use regex::Regex;

use crate::extractors::dockerfile::{DockerfileExtractedDep, classify_image_ref};

/// A single Docker image reference extracted from a CircleCI config.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircleCiDep {
    pub dep: DockerfileExtractedDep,
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

    #[test]
    fn extracts_executor_images() {
        let deps = extract(SAMPLE);
        let py = deps.iter().find(|d| d.dep.image == "cimg/python").unwrap();
        assert_eq!(py.dep.tag.as_deref(), Some("3.11"));
        assert!(py.dep.skip_reason.is_none());

        let pg = deps.iter().find(|d| d.dep.image == "postgres").unwrap();
        assert_eq!(pg.dep.tag.as_deref(), Some("15"));
    }

    #[test]
    fn extracts_job_images() {
        let deps = extract(SAMPLE);
        let node = deps.iter().find(|d| d.dep.image == "cimg/node").unwrap();
        assert_eq!(node.dep.tag.as_deref(), Some("18.0"));

        let go = deps.iter().find(|d| d.dep.image == "golang").unwrap();
        assert_eq!(go.dep.tag.as_deref(), Some("1.21-alpine"));
    }

    #[test]
    fn skips_variable_image() {
        let deps = extract(SAMPLE);
        // $CI_IMAGE should be filtered out (starts with $)
        assert!(!deps.iter().any(|d| d.dep.image.starts_with('$')));
    }

    #[test]
    fn empty_content_returns_no_deps() {
        assert!(extract("").is_empty());
    }

    #[test]
    fn no_docker_section_returns_no_deps() {
        let content =
            "version: 2.1\njobs:\n  build:\n    machine:\n      image: ubuntu-2204:2022.04.1\n";
        assert!(extract(content).is_empty());
    }
}
