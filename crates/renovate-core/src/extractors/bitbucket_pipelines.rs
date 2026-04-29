//! Bitbucket Pipelines (`bitbucket-pipelines.yml`) extractor.
//!
//! Extracts Docker image dependencies from Bitbucket Pipelines files.
//! Three forms are handled:
//!
//! * **Simple `image:` line** — `image: ubuntu:22.04`
//! * **Image object** — `image:` followed by `name: image:tag` on the next line
//! * **Docker pipe** — `- pipe: docker://image:tag` (pipe using a Docker image)
//!
//! Renovate reference:
//! - `lib/modules/manager/bitbucket-pipelines/extract.ts`
//! - `lib/modules/manager/bitbucket-pipelines/util.ts`
//! - Pattern: `**/*-pipelines.yml`
//!
//! ## Supported form
//!
//! ```yaml
//! image: atlassian/default-image:4
//!
//! pipelines:
//!   default:
//!   - step:
//!       image: node:18
//!       script: [npm test]
//!   - step:
//!       image:
//!         name: gcr.io/cloud-builders/docker:latest
//!   - step:
//!       script:
//!       - pipe: docker://alpine/helm:3.12.0
//! ```

use crate::extractors::dockerfile::{DockerfileExtractedDep, classify_image_ref};

/// Extract Docker image deps from a Bitbucket Pipelines YAML file.
pub fn extract(content: &str) -> Vec<DockerfileExtractedDep> {
    let mut out: Vec<DockerfileExtractedDep> = Vec::new();
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

        // Strip optional `- ` list-item prefix.
        let key_line = trimmed.strip_prefix("- ").unwrap_or(trimmed);

        // ── `image: <value>` or `- image: <value>` ───────────────────────────
        if let Some(val) = strip_key(key_line, "image") {
            let image_str = val.trim().trim_matches('"').trim_matches('\'');
            if image_str.is_empty() {
                // Image object: `image:\n  name: <ref>` — look ahead for `name:`.
                if let Some(name_val) = next_name_value(&lines, i + 1) {
                    let image = name_val.trim().trim_matches('"').trim_matches('\'');
                    if !image.is_empty() {
                        out.push(classify_image_ref(image));
                    }
                }
            } else {
                out.push(classify_image_ref(image_str));
            }
            i += 1;
            continue;
        }

        // ── `- pipe: docker://<image>` ────────────────────────────────────────
        if let Some(pipe_val) = strip_key(key_line, "pipe") {
            let pipe = pipe_val.trim().trim_matches('"').trim_matches('\'');
            if let Some(docker_ref) = pipe.strip_prefix("docker://")
                && !docker_ref.is_empty()
            {
                out.push(classify_image_ref(docker_ref));
            }
            // Non-docker pipes (BitbucketTags datasource) skipped — datasource pending.
        }

        i += 1;
    }

    out
}

/// Look ahead from `start` for a `name:` key at a deeper indent level.
/// Returns the value if found before a same-or-lower indent key.
fn next_name_value<'a>(lines: &[&'a str], start: usize) -> Option<&'a str> {
    let len = lines.len();
    let mut j = start;
    while j < len {
        let l = lines[j].split(" #").next().unwrap_or(lines[j]).trim_end();
        let trimmed = l.trim_start();
        if trimmed.is_empty() {
            j += 1;
            continue;
        }
        if let Some(val) = strip_key(trimmed, "name") {
            return Some(val);
        }
        // If we hit a line at indent 0 or another key at the same level, stop.
        break;
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

    #[test]
    fn top_level_image() {
        let content = "image: atlassian/default-image:4\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "atlassian/default-image");
        assert_eq!(deps[0].tag.as_deref(), Some("4"));
    }

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
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "node");
        assert_eq!(deps[0].tag.as_deref(), Some("18-alpine"));
    }

    #[test]
    fn image_object_with_name() {
        let content = r#"
pipelines:
  default:
  - step:
      image:
        name: gcr.io/cloud-builders/docker:latest
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "gcr.io/cloud-builders/docker");
        assert_eq!(deps[0].tag.as_deref(), Some("latest"));
    }

    #[test]
    fn docker_pipe() {
        let content = "pipelines:\n  default:\n  - step:\n      script:\n      - pipe: docker://alpine/helm:3.12.0\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "alpine/helm");
        assert_eq!(deps[0].tag.as_deref(), Some("3.12.0"));
    }

    #[test]
    fn non_docker_pipe_skipped() {
        let content = "pipelines:\n  default:\n  - step:\n      script:\n      - pipe: atlassian/git-secrets-scan:0.5.1\n";
        let deps = extract(content);
        assert!(deps.is_empty());
    }

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
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
        assert!(deps.iter().any(|d| d.image == "python"));
        assert!(deps.iter().any(|d| d.image == "node"));
        assert!(deps.iter().any(|d| d.image == "golang"));
    }

    #[test]
    fn variable_ref_skipped() {
        let content = "image: $BB_IMAGE\n";
        let deps = extract(content);
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
}
