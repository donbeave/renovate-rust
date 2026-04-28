//! Google Cloud Build `cloudbuild.yaml` Docker image extractor.
//!
//! Scans Cloud Build pipeline files for `name:` values in the `steps:` list.
//! Each step's `name` field is a Docker image reference used as the build
//! environment.
//!
//! Renovate reference:
//! - `lib/modules/manager/cloudbuild/extract.ts`
//! - `lib/modules/manager/cloudbuild/schema.ts`
//! - Pattern: `/(^|/)cloudbuild\.ya?ml/`
//!
//! ## Supported form
//!
//! ```yaml
//! steps:
//!   - name: 'gcr.io/cloud-builders/docker'
//!     args: ['build', '-t', 'gcr.io/$PROJECT_ID/myapp', '.']
//!   - name: node:18-alpine
//!     entrypoint: npm
//!     args: ['install']
//! ```

use crate::extractors::dockerfile::{DockerfileExtractedDep, classify_image_ref};

/// Extract Docker image deps from a `cloudbuild.yaml` file.
///
/// Returns one dep per `steps[].name` value that looks like a container image
/// reference.  Variable references (`$PROJECT_ID`, `${VAR}`) cause the dep to
/// be classified as `ArgVariable` (and skipped).
pub fn extract(content: &str) -> Vec<DockerfileExtractedDep> {
    let mut out = Vec::new();
    let mut in_steps = false;
    let mut in_step = false;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        if line.trim().is_empty() {
            continue;
        }
        let trimmed = line.trim_start();
        let indent = leading_spaces(line);

        // Detect `steps:` top-level key.
        if indent == 0 {
            if trimmed == "steps:" {
                in_steps = true;
                in_step = false;
                continue;
            } else if !trimmed.starts_with('-') {
                in_steps = false;
                in_step = false;
                continue;
            }
        }

        if !in_steps {
            continue;
        }

        // New list item at indent 2 (step entry).
        if let Some(rest) = trimmed.strip_prefix("- ") {
            in_step = true;
            // Inline `- name: image`
            if let Some(val) = strip_key(rest, "name") {
                let image = val.trim().trim_matches('"').trim_matches('\'');
                if !image.is_empty() {
                    out.push(classify_image_ref(image));
                }
            }
            continue;
        }

        if !in_step {
            continue;
        }

        // `name:` key inside a step.
        if let Some(val) = strip_key(trimmed, "name") {
            let image = val.trim().trim_matches('"').trim_matches('\'');
            if !image.is_empty() {
                out.push(classify_image_ref(image));
            }
        }
    }

    out
}

fn leading_spaces(s: &str) -> usize {
    s.len() - s.trim_start_matches([' ', '\t']).len()
}

fn strip_key<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}:");
    line.strip_prefix(prefix.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_step_names() {
        let content = r#"
steps:
  - name: 'gcr.io/cloud-builders/docker'
    args: ['build', '.']
  - name: node:18-alpine
    entrypoint: npm
    args: ['install']
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(
            deps.iter()
                .any(|d| d.image == "gcr.io/cloud-builders/docker")
        );
        assert!(
            deps.iter()
                .any(|d| d.image == "node" && d.tag.as_deref() == Some("18-alpine"))
        );
    }

    #[test]
    fn variable_reference_classified() {
        let content = "steps:\n  - name: '$_BUILDER_IMAGE'\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        // $VAR → ArgVariable skip reason
        assert!(deps[0].skip_reason.is_some());
    }

    #[test]
    fn ignores_non_steps_sections() {
        let content = r#"
substitutions:
  _BUILDER_IMAGE: node:18

steps:
  - name: node:18
    args: ['npm', 'test']
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "node");
    }

    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    #[test]
    fn inline_name_in_list_item() {
        let content = "steps:\n  - name: ubuntu:22.04\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "ubuntu");
        assert_eq!(deps[0].tag.as_deref(), Some("22.04"));
    }
}
