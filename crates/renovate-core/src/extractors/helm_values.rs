//! Helm `values.yaml` Docker image extractor.
//!
//! Scans Helm values files for Docker image references using two heuristics:
//!
//! 1. **Object form** — any key ending in `image` (case-insensitive) that
//!    contains `repository:` + `tag:` or `version:` child keys.
//!
//! 2. **Inline form** — any key ending in `image` whose value is a non-empty
//!    string (treated as a direct image reference).
//!
//! Renovate reference:
//! - `lib/modules/manager/helm-values/extract.ts`
//! - `lib/modules/manager/helm-values/util.ts` — `matchesHelmValuesDockerHeuristic`
//! - Pattern: `/(^|/)values\.ya?ml$/`
//!
//! ## Supported forms
//!
//! ```yaml
//! image:
//!   repository: nginx
//!   tag: "1.25"
//!
//! sidecarImage:
//!   repository: gcr.io/myproject/sidecar
//!   tag: v2.0.1
//!
//! busyboxImage: busybox:1.36
//! ```

use crate::extractors::dockerfile::{DockerfileExtractedDep, classify_image_ref};

/// Extract Docker image deps from a Helm `values.yaml` file.
pub fn extract(content: &str) -> Vec<DockerfileExtractedDep> {
    let mut out = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let len = lines.len();
    let mut i = 0;

    while i < len {
        let raw = lines[i];
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        let trimmed = line.trim_start();
        let indent = leading_spaces(line);

        if trimmed.is_empty() {
            i += 1;
            continue;
        }

        // Check for a key ending in "image" (case-insensitive).
        if let Some((key, val)) = parse_kv(trimmed)
            && key.to_lowercase().ends_with("image")
        {
            let val = val.trim().trim_matches('"').trim_matches('\'');
            if val.is_empty() {
                // Object form: look ahead for repository + tag/version.
                if let Some(dep) = try_object_image(&lines, i + 1, indent) {
                    out.push(dep);
                }
            } else if !val.starts_with('{') {
                // Inline string form: `image: nginx:1.25`
                out.push(classify_image_ref(val));
            }
        }

        i += 1;
    }

    out
}

/// Look ahead from `start` for `repository:` and `tag:`/`version:` keys
/// at an indent level greater than `parent_indent`.
fn try_object_image(
    lines: &[&str],
    start: usize,
    parent_indent: usize,
) -> Option<DockerfileExtractedDep> {
    let mut repository: Option<String> = None;
    let mut tag: Option<String> = None;

    let mut j = start;
    while j < lines.len() {
        let l = lines[j].split(" #").next().unwrap_or(lines[j]).trim_end();
        let trimmed = l.trim_start();
        let ind = leading_spaces(l);

        if trimmed.is_empty() {
            j += 1;
            continue;
        }
        // Dedented or same-level key → we've left the object.
        if ind <= parent_indent && !trimmed.is_empty() {
            break;
        }

        if let Some((k, v)) = parse_kv(trimmed) {
            let v = v.trim().trim_matches('"').trim_matches('\'');
            match k {
                "repository" => repository = Some(v.to_owned()),
                "tag" | "version" => tag = Some(v.to_owned()),
                _ => {}
            }
        }
        j += 1;
    }

    let repo = repository?;
    if repo.is_empty() {
        return None;
    }
    let image_ref = match tag {
        Some(t) if !t.is_empty() => format!("{repo}:{t}"),
        _ => repo,
    };
    Some(classify_image_ref(&image_ref))
}

/// Parse `key: value` from a line, returning `(key, value)`.
/// Returns `None` if no `:` is found.
fn parse_kv(line: &str) -> Option<(&str, &str)> {
    // Skip list items.
    let line = line.strip_prefix("- ").unwrap_or(line);
    let colon = line.find(':')?;
    let key = line[..colon].trim();
    let val = line[colon + 1..].trim();
    if key.is_empty() {
        return None;
    }
    Some((key, val))
}

fn leading_spaces(s: &str) -> usize {
    s.len() - s.trim_start_matches([' ', '\t']).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_form_repository_and_tag() {
        let content = r#"
image:
  repository: nginx
  tag: "1.25"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "nginx");
        assert_eq!(deps[0].tag.as_deref(), Some("1.25"));
    }

    #[test]
    fn object_form_version_key() {
        let content = r#"
image:
  repository: redis
  version: "7.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "redis");
        assert_eq!(deps[0].tag.as_deref(), Some("7.0"));
    }

    #[test]
    fn inline_string_form() {
        let content = "busyboxImage: busybox:1.36\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "busybox");
        assert_eq!(deps[0].tag.as_deref(), Some("1.36"));
    }

    #[test]
    fn multiple_images() {
        let content = r#"
image:
  repository: nginx
  tag: "1.25"
sidecar:
  image:
    repository: redis
    tag: "7"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(deps.iter().any(|d| d.image == "nginx"));
        assert!(deps.iter().any(|d| d.image == "redis"));
    }

    #[test]
    fn key_not_ending_in_image_ignored() {
        let content = "pullPolicy: IfNotPresent\nrepository: nginx\n";
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    #[test]
    fn registry_prefix_combined() {
        let content = r#"
image:
  repository: gcr.io/myproject/app
  tag: v1.0.0
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "gcr.io/myproject/app");
    }

    // Ported: "returns null for empty yaml file content" — helm-values/extract.spec.ts line 31
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "returns null for invalid yaml file content" — helm-values/extract.spec.ts line 26
    #[test]
    fn invalid_yaml_returns_empty() {
        assert!(extract("nothing here: [").is_empty());
    }

    // Ported: "extracts from values.yaml correctly with same structure as \"helm create\"" — helm-values/extract.spec.ts line 36
    #[test]
    fn helm_create_default_values() {
        let content = "image:\n  repository: nginx\n  tag: 1.16.1\n  pullPolicy: IfNotPresent\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "nginx");
        assert_eq!(deps[0].tag.as_deref(), Some("1.16.1"));
    }
}
