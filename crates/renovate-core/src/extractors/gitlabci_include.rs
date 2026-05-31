//! GitLab CI `include:` project reference extractor.
//!
//! Extracts versioned project template includes from GitLab CI files.
//! These are `include:` blocks containing `project:` + `ref:` pairs,
//! where `ref:` is a tag or branch name.
//!
//! Renovate reference:
//! - `lib/modules/manager/gitlabci-include/extract.ts`
//! - Pattern: `/\.gitlab-ci\.ya?ml$/`
//!
//! ## Supported form
//!
//! ```yaml
//! include:
//!   - project: org/shared-templates
//!     ref: v2.1.0
//!     file: /templates/build.yml
//! ```

/// A single extracted GitLab CI include project reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitlabIncludeDep {
    /// GitLab project path (e.g. `"org/shared-templates"`).
    pub project: String,
    /// Git ref (tag or branch) used as version (e.g. `"v2.1.0"`).
    pub ref_value: String,
    /// Optional GitLab registry URL for self-hosted endpoints.
    pub registry_urls: Vec<String>,
}

/// Extract include project references from a GitLab CI YAML file.
pub fn extract(content: &str) -> Vec<GitlabIncludeDep> {
    extract_with_endpoint(content, None)
}

/// Extract include project references using an optional configured GitLab API endpoint.
pub fn extract_with_endpoint(content: &str, endpoint: Option<&str>) -> Vec<GitlabIncludeDep> {
    let mut out = Vec::new();
    let mut in_include = false;
    let mut cur_project: Option<String> = None;
    let mut cur_ref: Option<String> = None;
    let registry_urls = endpoint
        .and_then(normalize_gitlab_endpoint)
        .map(|url| vec![url])
        .unwrap_or_default();

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        let trimmed = line.trim_start();
        let indent = leading_spaces(line);

        if trimmed.is_empty() {
            continue;
        }

        // `include:` at any top-level indent.
        if trimmed == "include:" {
            flush(&mut out, &mut cur_project, &mut cur_ref, &registry_urls);
            in_include = true;
            continue;
        }

        // Leaving the include block when we see a top-level key.
        if indent == 0 && !trimmed.starts_with('-') && in_include {
            flush(&mut out, &mut cur_project, &mut cur_ref, &registry_urls);
            in_include = false;
            continue;
        }

        if !in_include {
            continue;
        }

        // New list item: `- project: X` or `- ref: Y`
        if trimmed.starts_with("- ") {
            flush(&mut out, &mut cur_project, &mut cur_ref, &registry_urls);
            let rest = trimmed.strip_prefix("- ").unwrap_or("");
            if let Some(val) = strip_key(rest, "project") {
                cur_project = Some(val.trim().trim_matches('"').trim_matches('\'').to_owned());
            } else if let Some(val) = strip_key(rest, "ref") {
                cur_ref = Some(val.trim().trim_matches('"').trim_matches('\'').to_owned());
            }
            continue;
        }

        // Fields within the current include item.
        if let Some(val) = strip_key(trimmed, "project") {
            cur_project = Some(val.trim().trim_matches('"').trim_matches('\'').to_owned());
        } else if let Some(val) = strip_key(trimmed, "ref") {
            cur_ref = Some(val.trim().trim_matches('"').trim_matches('\'').to_owned());
        }
    }

    flush(&mut out, &mut cur_project, &mut cur_ref, &registry_urls);
    out
}

fn flush(
    out: &mut Vec<GitlabIncludeDep>,
    project: &mut Option<String>,
    ref_val: &mut Option<String>,
    registry_urls: &[String],
) {
    if let (Some(p), Some(r)) = (project.take(), ref_val.take())
        && !p.is_empty()
        && !r.is_empty()
    {
        out.push(GitlabIncludeDep {
            project: p,
            ref_value: r,
            registry_urls: registry_urls.to_vec(),
        });
    }
    project.take();
    ref_val.take();
}

fn leading_spaces(s: &str) -> usize {
    s.len() - s.trim_start_matches([' ', '\t']).len()
}

fn strip_key<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}:");
    line.strip_prefix(prefix.as_str())
}

fn normalize_gitlab_endpoint(endpoint: &str) -> Option<String> {
    let mut endpoint = endpoint.trim().trim_end_matches('/').to_owned();
    if endpoint.is_empty() {
        return None;
    }
    if let Some(stripped) = endpoint.strip_suffix("/api/v4") {
        endpoint = stripped.to_owned();
    }
    Some(endpoint)
}

/// Update the `ref:` value for a given GitLab CI include project.
///
/// Searches the YAML content for an include block containing
/// `project: <dep_name>` and then updates the corresponding `ref:` line
/// from `current_value` to `new_value`.
pub fn gitlabci_include_update_dependency(
    content: &str,
    dep_name: &str,
    current_value: &str,
    new_value: &str,
) -> Option<String> {
    let mut result = String::with_capacity(content.len());
    let mut in_include = false;
    let mut found_project = false;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        let trimmed = line.trim_start();
        let indent = leading_spaces(line);

        if trimmed.is_empty() {
            result.push_str(raw);
            result.push('\n');
            continue;
        }

        // `include:` at any indent level (handles nested includes).
        if trimmed == "include:" {
            in_include = true;
            found_project = false;
            result.push_str(raw);
            result.push('\n');
            continue;
        }

        // Leaving the include block when we see a top-level key.
        if indent == 0 && !trimmed.starts_with('-') && in_include {
            in_include = false;
            found_project = false;
            result.push_str(raw);
            result.push('\n');
            continue;
        }

        if !in_include {
            result.push_str(raw);
            result.push('\n');
            continue;
        }

        // New list item resets the project match.
        if trimmed.starts_with("- ") {
            found_project = false;
            let rest = trimmed.strip_prefix("- ").unwrap_or("");
            if let Some(val) = strip_key(rest, "project") {
                let project_val = val.trim().trim_matches('"').trim_matches('\'');
                if project_val == dep_name {
                    found_project = true;
                }
            }
            result.push_str(raw);
            result.push('\n');
            continue;
        }

        // Inline project key (non-list form).
        if let Some(val) = strip_key(trimmed, "project") {
            let project_val = val.trim().trim_matches('"').trim_matches('\'');
            found_project = project_val == dep_name;
            result.push_str(raw);
            result.push('\n');
            continue;
        }

        if let Some(val) = strip_key(trimmed, "ref")
            && found_project
        {
            let ref_val = val.trim().trim_matches('"').trim_matches('\'');
            if ref_val == current_value {
                // Replace first occurrence of current_value on this line.
                let new_raw = raw.replacen(current_value, new_value, 1);
                result.push_str(&new_raw);
                result.push('\n');
                continue;
            }
        }

        result.push_str(raw);
        result.push('\n');
    }

    if result != content {
        Some(result)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "extracts single include block" — gitlabci-include/extract.spec.ts line 22
    #[test]
    fn extracts_include_with_ref() {
        let content = r#"
include:
  - project: org/templates
    ref: v2.1.0
    file: /templates/build.yml
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].project, "org/templates");
        assert_eq!(deps[0].ref_value, "v2.1.0");
    }

    // Ported: "extracts multiple include blocks" — gitlabci-include/extract.spec.ts line 28
    #[test]
    fn multiple_includes() {
        let content = r#"
include:
  - project: org/templates
    ref: v1.0.0
  - project: org/other
    ref: v2.0.0
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
    }

    // Rust-specific: gitlabci_include behavior test
    #[test]
    fn include_without_ref_skipped() {
        let content = "include:\n  - project: org/templates\n    file: /f.yml\n";
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    // Rust-specific: gitlabci_include behavior test
    #[test]
    fn non_include_blocks_ignored() {
        let content = r#"
stages: [build]

build:
  image: node:18
  script: npm test
"#;
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    // Ported: "returns null for empty" — gitlabci-include/extract.spec.ts line 13
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "extracts multiple embedded include blocks" — gitlabci-include/extract.spec.ts line 34
    //
    // The fixture has a top-level `include:` plus a nested `trigger.include:`
    // block inside a job. Both should produce a dep.
    #[test]
    fn extracts_multiple_embedded_include_blocks() {
        let content = "\
---
include:
- project: mikebryant/include-source-example
  file: /template.yaml
  ref: 1.0.0

trigger-my-job:
  extends: .extend-trigger-job
  trigger:
    include:
      - project: mikebryant/include-source-example
        file: /template.yaml
        ref: master
";
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].project, "mikebryant/include-source-example");
        assert_eq!(deps[0].ref_value, "1.0.0");
        assert_eq!(deps[1].project, "mikebryant/include-source-example");
        assert_eq!(deps[1].ref_value, "master");
    }

    // Ported: "supports multi-document files" — gitlabci-include/extract.spec.ts line 73
    //
    // A single YAML file may contain multiple documents separated by `---`.
    // Each document's `include:` block should be parsed independently.
    #[test]
    fn supports_multi_document_files() {
        let content = "\
other:
  content: to be ignored
---
include:
  - project: mikebryant/include-source-example
    ref: 1.0.0
---
include:
  - project: mikebryant/include-source-example2
    ref: 2.0.0
---
more:
  content: to be ignored
";
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].project, "mikebryant/include-source-example");
        assert_eq!(deps[0].ref_value, "1.0.0");
        assert_eq!(deps[1].project, "mikebryant/include-source-example2");
        assert_eq!(deps[1].ref_value, "2.0.0");
    }

    // Ported: "returns null for include block without any actual includes" — gitlabci-include/extract.spec.ts line 17
    //
    // A bare `include:` key with no list under it produces no deps.
    #[test]
    fn empty_include_block_returns_no_deps() {
        let content = "\
include:

script:
- !reference [.setup, script]
";
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    // Ported: "ignores includes without project and file keys" — gitlabci-include/extract.spec.ts line 51
    //
    // String form, `remote:` form, and `local:` form includes have no
    // project+ref pair so the extractor produces nothing.
    #[test]
    fn ignores_includes_without_project_and_file_keys() {
        let content = "\
include:
  - 'https://gitlab.com/mikebryant/include-source-example.yml'
  - remote: 'https://gitlab.com/mikebryant/include-source-example.yml'
  - local: mikebryant/include-source-example
";
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    // Ported: "normalizes configured endpoints" — gitlabci-include/extract.spec.ts line 60
    #[test]
    fn normalizes_configured_endpoints() {
        let content = "\
include:
  - project: mikebryant/include-source-example
    file: /template.yaml
    ref: 1.0.0
";

        for endpoint in ["http://gitlab.test/api/v4", "http://gitlab.test/api/v4/"] {
            let deps = extract_with_endpoint(content, Some(endpoint));
            assert_eq!(deps.len(), 1);
            assert_eq!(deps[0].registry_urls, vec!["http://gitlab.test"]);
        }
    }

    #[test]
    fn update_ref_value() {
        let content = "\
include:
  - project: org/templates
    ref: v2.1.0
    file: /templates/build.yml
";
        let updated = gitlabci_include_update_dependency(content, "org/templates", "v2.1.0", "v3.0.0");
        assert!(updated.is_some());
        let updated = updated.unwrap();
        assert!(updated.contains("ref: v3.0.0"));
        assert!(!updated.contains("ref: v2.1.0"));
    }

    #[test]
    fn update_ref_preserves_quotes() {
        let content = "\
include:
  - project: org/templates
    ref: 'v2.1.0'
";
        let updated = gitlabci_include_update_dependency(content, "org/templates", "v2.1.0", "v3.0.0");
        assert!(updated.is_some());
        let updated = updated.unwrap();
        assert!(updated.contains("ref: 'v3.0.0'"));
    }

    #[test]
    fn update_ref_no_match_returns_none() {
        let content = "\
include:
  - project: org/templates
    ref: v2.1.0
";
        let updated = gitlabci_include_update_dependency(content, "org/other", "v2.1.0", "v3.0.0");
        assert!(updated.is_none());
    }

    #[test]
    fn update_ref_only_matching_current() {
        let content = "\
include:
  - project: org/templates
    ref: v2.1.0
  - project: org/other
    ref: v1.0.0
";
        let updated = gitlabci_include_update_dependency(content, "org/other", "v1.0.0", "v2.0.0");
        assert!(updated.is_some());
        let updated = updated.unwrap();
        assert!(updated.contains("org/other\n    ref: v2.0.0"));
        assert!(updated.contains("org/templates\n    ref: v2.1.0"));
    }

    #[test]
    fn update_ref_nested_include() {
        let content = "\
trigger-my-job:
  trigger:
    include:
      - project: mikebryant/include-source-example
        file: /template.yaml
        ref: master
";
        let updated = gitlabci_include_update_dependency(content, "mikebryant/include-source-example", "master", "main");
        assert!(updated.is_some());
        let updated = updated.unwrap();
        assert!(updated.contains("ref: main"));
        assert!(!updated.contains("ref: master"));
    }
}
