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

/// Update a Docker image dependency in a Helm `values.yaml` file.
///
/// Handles both inline form (`busyboxImage: busybox:1.36`) and object form
/// (`image:\n  repository: nginx\n  tag: "1.25"`).
///
/// For object form, `dep_name` is expected as `image:tag` (e.g. `nginx:1.25`).
/// The updater scans for object-form image blocks whose `repository:` matches
/// the image part and whose `tag:` or `version:` matches `current_value`, then
/// replaces the tag/version value.
pub fn helm_values_update_dependency(
    content: &str,
    dep_name: &str,
    current_value: &str,
    new_value: &str,
) -> Option<String> {
    let (image, _tag) = dep_name.rsplit_once(':').unwrap_or((dep_name, ""));
    let lines: Vec<&str> = content.lines().collect();
    let len = lines.len();
    let mut result = String::with_capacity(content.len());
    let mut changed = false;
    let mut i = 0;

    while i < len {
        let raw = lines[i];
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        let trimmed = line.trim_start();
        let indent = leading_spaces(line);

        let key_part = trimmed.strip_prefix("- ").unwrap_or(trimmed);
        let Some((key, val)) = parse_kv(key_part) else {
            result.push_str(raw);
            result.push('\n');
            i += 1;
            continue;
        };

        if !key.to_lowercase().ends_with("image") {
            result.push_str(raw);
            result.push('\n');
            i += 1;
            continue;
        }

        let val_trimmed = val.trim().trim_matches('"').trim_matches('\'');

        // Inline form: non-empty string value that is a direct image reference.
        if !val_trimmed.is_empty() && !val_trimmed.starts_with('{') {
            let dep = classify_image_ref(val_trimmed);
            let dep_report_name = match &dep.tag {
                Some(t) => format!("{}:{t}", dep.image),
                None => dep.image.clone(),
            };
            if dep_report_name == dep_name {
                let new_ref = match &dep.tag {
                    Some(_) => format!("{}:{new_value}", dep.image),
                    None => dep.image.clone(),
                };
                let new_raw = raw.replacen(val_trimmed, &new_ref, 1);
                result.push_str(&new_raw);
                result.push('\n');
                changed = true;
                i += 1;
                continue;
            }
            result.push_str(raw);
            result.push('\n');
            i += 1;
            continue;
        }

        // Object form: empty value — look ahead for repository + tag/version.
        let mut j = i + 1;
        let mut repository: Option<&str> = None;
        let mut tag_line_idx: Option<usize> = None;
        let mut tag_value: Option<&str> = None;

        while j < len {
            let l = lines[j].split(" #").next().unwrap_or(lines[j]).trim_end();
            let t = l.trim_start();
            let ind = leading_spaces(l);

            if t.is_empty() {
                j += 1;
                continue;
            }
            if ind <= indent && !t.is_empty() {
                break;
            }

            if let Some((k, v)) = parse_kv(t) {
                let v = v.trim().trim_matches('"').trim_matches('\'');
                match k {
                    "repository" => repository = Some(v),
                    "tag" | "version" => {
                        tag_line_idx = Some(j);
                        tag_value = Some(v);
                    }
                    _ => {}
                }
            }
            j += 1;
        }

        let matches =
            repository.is_some_and(|repo| repo == image || image.ends_with(&format!("/{repo}")));

        if matches && tag_value == Some(current_value) {
            let tag_line = lines[tag_line_idx.unwrap()];
            let new_tag_line = tag_line.replacen(current_value, new_value, 1);
            result.push_str(&new_tag_line);
            result.push('\n');
            changed = true;
            i = tag_line_idx.unwrap() + 1;
            continue;
        }

        result.push_str(raw);
        result.push('\n');
        i += 1;
    }

    if changed { Some(result) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "extracts from values.yaml correctly with same structure as \"helm create\"" — lib/modules/manager/helm-values/extract.spec.ts line 36
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

    // Ported: "extracts from values.yaml correctly with same structure as \"helm create\"" — lib/modules/manager/helm-values/extract.spec.ts line 36
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

    // Ported: "extracts from complex values file correctly"" — lib/modules/manager/helm-values/extract.spec.ts line 52
    #[test]
    fn inline_string_form() {
        let content = "busyboxImage: busybox:1.36\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "busybox");
        assert_eq!(deps[0].tag.as_deref(), Some("1.36"));
    }

    // Ported: "extracts from complex values file correctly"" — lib/modules/manager/helm-values/extract.spec.ts line 52
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

    // Ported: "extracts from values.yaml correctly with same structure as \"helm create\"" — lib/modules/manager/helm-values/extract.spec.ts line 36
    #[test]
    fn key_not_ending_in_image_ignored() {
        let content = "pullPolicy: IfNotPresent\nrepository: nginx\n";
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    // Ported: "extract data from file with registry aliases" — lib/modules/manager/helm-values/extract.spec.ts line 85
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

    // Ported: "returns null for empty yaml file content" — lib/modules/manager/helm-values/extract.spec.ts line 31
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "returns null for invalid yaml file content" — lib/modules/manager/helm-values/extract.spec.ts line 26
    #[test]
    fn invalid_yaml_returns_empty() {
        assert!(extract("nothing here: [").is_empty());
    }

    // Ported: "extracts from values.yaml correctly with same structure as \"helm create\"" — lib/modules/manager/helm-values/extract.spec.ts line 36
    #[test]
    fn helm_create_default_values() {
        let content = "image:\n  repository: nginx\n  tag: 1.16.1\n  pullPolicy: IfNotPresent\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "nginx");
        assert_eq!(deps[0].tag.as_deref(), Some("1.16.1"));
    }

    // Ported: "extract data from file with multiple documents" — lib/modules/manager/helm-values/extract.spec.ts line 62
    #[test]
    fn multidoc_yaml_extracts_nested_images() {
        // Simulates single_file_with_multiple_documents.yaml fixture:
        // two image blocks nested under values: controller/speaker.
        let content = r#"apiVersion: source.toolkit.fluxcd.io/v1beta2
kind: HelmRepository
metadata:
  name: metallb
---
apiVersion: helm.toolkit.fluxcd.io/v2beta1
kind: HelmRelease
spec:
  values:
    controller:
      image:
        repository: quay.io/metallb/controller
        tag: v0.13.10
    speaker:
      image:
        repository: quay.io/metallb/speaker
        tag: v0.13.10
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(deps.iter().any(
            |d| d.image == "quay.io/metallb/controller" && d.tag.as_deref() == Some("v0.13.10")
        ));
        assert!(
            deps.iter()
                .any(|d| d.image == "quay.io/metallb/speaker"
                    && d.tag.as_deref() == Some("v0.13.10"))
        );
    }

    #[test]
    fn update_object_form_tag() {
        let content = "image:\n  repository: nginx\n  tag: 1.16.1\n  pullPolicy: IfNotPresent\n";
        let updated = helm_values_update_dependency(content, "nginx:1.16.1", "1.16.1", "1.17.0");
        assert!(updated.is_some());
        let updated = updated.unwrap();
        assert!(updated.contains("tag: 1.17.0"));
        assert!(!updated.contains("tag: 1.16.1"));
    }

    #[test]
    fn update_object_form_version() {
        let content = "image:\n  repository: nginx\n  version: 1.16.1\n";
        let updated = helm_values_update_dependency(content, "nginx:1.16.1", "1.16.1", "1.17.0");
        assert!(updated.is_some());
        let updated = updated.unwrap();
        assert!(updated.contains("version: 1.17.0"));
    }

    #[test]
    fn update_object_form_preserves_quotes() {
        let content = "image:\n  repository: nginx\n  tag: \"1.16.1\"\n";
        let updated = helm_values_update_dependency(content, "nginx:1.16.1", "1.16.1", "1.17.0");
        assert!(updated.is_some());
        let updated = updated.unwrap();
        assert!(updated.contains("tag: \"1.17.0\""));
    }

    #[test]
    fn update_object_form_no_match_returns_none() {
        let content = "image:\n  repository: nginx\n  tag: 1.16.1\n";
        let updated = helm_values_update_dependency(content, "redis:1.16.1", "1.16.1", "1.17.0");
        assert!(updated.is_none());
    }

    #[test]
    fn update_inline_form() {
        let content = "busyboxImage: busybox:1.36\n";
        let updated = helm_values_update_dependency(content, "busybox:1.36", "1.36", "1.37");
        assert!(updated.is_some());
        let updated = updated.unwrap();
        assert!(updated.contains("busyboxImage: busybox:1.37"));
    }

    #[test]
    fn update_object_form_registry_prefix() {
        let content = "image:\n  repository: quay.io/metallb/controller\n  tag: v0.13.10\n";
        let updated = helm_values_update_dependency(
            content,
            "quay.io/metallb/controller:v0.13.10",
            "v0.13.10",
            "v0.14.0",
        );
        assert!(updated.is_some());
        let updated = updated.unwrap();
        assert!(updated.contains("tag: v0.14.0"));
    }
}
