//! Bitbucket Pipelines (`bitbucket-pipelines.yml`) extractor.
//!
//! Extracts Docker image and Bitbucket pipe dependencies from Bitbucket
//! Pipelines files. Three image forms and two pipe forms are handled:
//!
//! * **Simple `image:` line** — `image: ubuntu:22.04`
//! * **Image object** — `image:` followed by `name: image:tag` (possibly with username/password)
//! * **Docker pipe** — `- pipe: docker://image:tag`
//! * **Bitbucket pipe** — `- pipe: owner/repo:version`
//!
//! Renovate reference:
//! - `lib/modules/manager/bitbucket-pipelines/extract.ts`
//! - `lib/modules/manager/bitbucket-pipelines/util.ts`
//! - Pattern: `**/*-pipelines.yml`

use crate::extractors::dockerfile::{DockerfileExtractedDep, classify_image_ref};

/// A Bitbucket Pipelines dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitbucketPipelinesDep {
    Docker(DockerfileExtractedDep),
    BitbucketPipe {
        dep_name: String,
        current_value: String,
    },
}

/// Extract all deps from a Bitbucket Pipelines YAML file.
pub fn extract(content: &str) -> Vec<BitbucketPipelinesDep> {
    let mut out: Vec<BitbucketPipelinesDep> = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let len = lines.len();
    let mut i = 0;

    while i < len {
        let raw = lines[i];
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        let trimmed = line.trim_start();

        if trimmed.is_empty() {
            i += 1;
            continue;
        }

        let image_indent = line.len() - line.trim_start().len();

        // Strip optional `- ` list-item prefix.
        let key_line = trimmed.strip_prefix("- ").unwrap_or(trimmed);

        // ── `image: <value>` or `- image: <value>` ───────────────────────────
        if let Some(val) = strip_key(key_line, "image") {
            let image_str = val.trim().trim_matches('"').trim_matches('\'');
            if image_str.is_empty() {
                // Image object: `image:\n  name: <ref>` — look ahead for `name:`.
                if let Some(name_val) = next_name_value(&lines, i + 1, image_indent) {
                    let image = name_val.trim().trim_matches('"').trim_matches('\'');
                    if !image.is_empty() {
                        out.push(BitbucketPipelinesDep::Docker(classify_image_ref(image)));
                    }
                }
            } else {
                out.push(BitbucketPipelinesDep::Docker(classify_image_ref(image_str)));
            }
            i += 1;
            continue;
        }

        // ── `- pipe: <value>` ─────────────────────────────────────────────────
        if let Some(pipe_val) = strip_key(key_line, "pipe") {
            let pipe = pipe_val.trim().trim_matches('"').trim_matches('\'');
            if let Some(docker_ref) = pipe.strip_prefix("docker://") {
                if !docker_ref.is_empty() {
                    out.push(BitbucketPipelinesDep::Docker(classify_image_ref(
                        docker_ref,
                    )));
                }
            } else if let Some((owner_repo, version)) = parse_bitbucket_pipe(pipe) {
                out.push(BitbucketPipelinesDep::BitbucketPipe {
                    dep_name: owner_repo,
                    current_value: version,
                });
            }
        }

        i += 1;
    }

    out
}

/// Extract Docker deps only (backwards-compatible helper for callers that don't need pipes).
pub fn extract_docker(content: &str) -> Vec<DockerfileExtractedDep> {
    extract(content)
        .into_iter()
        .filter_map(|d| match d {
            BitbucketPipelinesDep::Docker(dep) => Some(dep),
            _ => None,
        })
        .collect()
}

/// Parse a non-docker Bitbucket pipe reference `owner/repo:version`.
fn parse_bitbucket_pipe(pipe: &str) -> Option<(String, String)> {
    // Must have exactly one `/` before the `:` (owner/repo:version)
    let (owner_repo, version) = pipe.split_once(':')?;
    if owner_repo.is_empty() || version.is_empty() {
        return None;
    }
    // owner/repo must contain exactly one `/`
    if owner_repo.matches('/').count() != 1 {
        return None;
    }
    Some((owner_repo.to_owned(), version.to_owned()))
}

/// Look ahead from `start` for a `name:` key inside an image object block.
/// `parent_indent` is the indent of the `image:` key — we stop when we see
/// a line at that indent or less (which means we've exited the block).
fn next_name_value<'a>(lines: &[&'a str], start: usize, parent_indent: usize) -> Option<&'a str> {
    let len = lines.len();
    let mut j = start;
    while j < len {
        let raw = lines[j];
        let l = raw.split(" #").next().unwrap_or(raw).trim_end();
        let trimmed = l.trim_start();
        if trimmed.is_empty() {
            j += 1;
            continue;
        }

        let indent = l.len() - trimmed.len();

        // Exited the image block — stop.
        if indent <= parent_indent {
            break;
        }

        if let Some(val) = strip_key(trimmed, "name") {
            return Some(val);
        }

        j += 1;
    }
    None
}

fn strip_key<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}:");
    line.strip_prefix(prefix.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn docker_deps(content: &str) -> Vec<DockerfileExtractedDep> {
        extract(content)
            .into_iter()
            .filter_map(|d| {
                if let BitbucketPipelinesDep::Docker(dep) = d {
                    Some(dep)
                } else {
                    None
                }
            })
            .collect()
    }

    // Ported: "extracts dependencies" — bitbucket-pipelines/extract.spec.ts line 22
    #[test]
    fn top_level_image() {
        let content = "image: atlassian/default-image:4\n";
        let deps = docker_deps(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "atlassian/default-image");
        assert_eq!(deps[0].tag.as_deref(), Some("4"));
    }

    // Ported: "extracts dependencies" — bitbucket-pipelines/extract.spec.ts line 22
    #[test]
    fn step_image() {
        let content = r#"
pipelines:
  default:
  - step:
      image: node:18-alpine
      script:
        - npm test
"#;
        let deps = docker_deps(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "node");
        assert_eq!(deps[0].tag.as_deref(), Some("18-alpine"));
    }

    // Ported: "extracts dependencies" — bitbucket-pipelines/extract.spec.ts line 22
    #[test]
    fn image_object_with_name() {
        let content = r#"
pipelines:
  default:
  - step:
      image:
        name: gcr.io/cloud-builders/docker:latest
"#;
        let deps = docker_deps(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "gcr.io/cloud-builders/docker");
        assert_eq!(deps[0].tag.as_deref(), Some("latest"));
    }

    // Ported: "extracts dependencies" — bitbucket-pipelines/extract.spec.ts line 22
    #[test]
    fn image_object_with_username_and_name() {
        // image: { username: xxxx, name: node:18.15.1 } — scan past username to find name
        let content = "image:\n  username: xxxx\n  name: node:18.15.1\n";
        let deps = docker_deps(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "node");
        assert_eq!(deps[0].tag.as_deref(), Some("18.15.1"));
    }

    // Ported: "extracts dependencies" — bitbucket-pipelines/extract.spec.ts line 22
    #[test]
    fn docker_pipe() {
        let content = "pipelines:\n  default:\n  - step:\n      script:\n      - pipe: docker://alpine/helm:3.12.0\n";
        let deps = docker_deps(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "alpine/helm");
        assert_eq!(deps[0].tag.as_deref(), Some("3.12.0"));
    }

    // Ported: "extracts dependencies" — bitbucket-pipelines/extract.spec.ts line 22
    #[test]
    fn non_docker_pipe_produces_bitbucket_dep() {
        let content = "- pipe: atlassian/aws-s3-deploy:0.2.1\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        if let BitbucketPipelinesDep::BitbucketPipe {
            dep_name,
            current_value,
        } = &deps[0]
        {
            assert_eq!(dep_name, "atlassian/aws-s3-deploy");
            assert_eq!(current_value, "0.2.1");
        } else {
            panic!("expected BitbucketPipe dep");
        }
    }

    // Ported: "extracts dependencies" — bitbucket-pipelines/extract.spec.ts line 22
    #[test]
    fn multiple_images() {
        let content = r#"
image: python:3.11

pipelines:
  default:
  - step:
      image: node:18
  - step:
      image: golang:1.21
"#;
        let deps = docker_deps(content);
        assert_eq!(deps.len(), 3);
        assert!(deps.iter().any(|d| d.image == "python"));
        assert!(deps.iter().any(|d| d.image == "node"));
        assert!(deps.iter().any(|d| d.image == "golang"));
    }

    // Ported: "extracts dependencies" — bitbucket-pipelines/extract.spec.ts line 22
    #[test]
    fn variable_ref_skipped() {
        let content = "image: $BB_IMAGE\n";
        let deps = docker_deps(content);
        assert_eq!(deps.len(), 1);
        assert!(deps[0].skip_reason.is_some());
    }

    // Ported: "returns null for empty" — bitbucket-pipelines/extract.spec.ts line 6
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "returns null for malformed" — bitbucket-pipelines/extract.spec.ts line 12
    #[test]
    fn malformed_image_object_without_name_returns_empty() {
        let content = "image:\n  username: ccc\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "extracts dependencies" — bitbucket-pipelines/extract.spec.ts line 22
    #[test]
    fn extracts_full_fixture_seven_deps() {
        let content = r#"image: node:10.15.1

definitions:
  steps:
    - step: &build-test
        name: Build and test
        image:
          # comment
          name: node:18.15.0
        script:
          - mvn package

    - step: &build-test1
        image:
          username: xxxx
          name: node:18.15.1

    - step: &build-test2
        image:
          username: xxx
          password: xxx

          name: node:18.15.2

    - step:
        image:
        test:
          name: malformed

    - step:
        image:
          username: xxx
        test:
          name: malformed

    - step:
        image:
          username: xxx
          password: xxx
        test:
          name: malformed


pipelines:
  default:
    - step:
        name: Build and Test
        image: node:10.15.2
        script:
          - step: *build-test
          - pipe: docker://jfrogecosystem/jfrog-setup-cli:2.0.2
          - npm install
        artifacts:
          - dist/**
    - step:
        name: Deploy
        deployment: production
        script:
          - pipe: atlassian/aws-s3-deploy:0.2.1
"#;
        let deps = extract(content);
        // Expected: 6 docker deps + 1 bitbucket-tags dep = 7 total
        let docker_count = deps
            .iter()
            .filter(|d| matches!(d, BitbucketPipelinesDep::Docker(_)))
            .count();
        let pipe_count = deps
            .iter()
            .filter(|d| matches!(d, BitbucketPipelinesDep::BitbucketPipe { .. }))
            .count();
        assert_eq!(docker_count, 6);
        assert_eq!(pipe_count, 1);
        assert_eq!(deps.len(), 7);
    }
}
