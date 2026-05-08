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
}

/// Extract include project references from a GitLab CI YAML file.
pub fn extract(content: &str) -> Vec<GitlabIncludeDep> {
    let mut out = Vec::new();
    let mut in_include = false;
    let mut cur_project: Option<String> = None;
    let mut cur_ref: Option<String> = None;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        let trimmed = line.trim_start();
        let indent = leading_spaces(line);

        if trimmed.is_empty() {
            continue;
        }

        // `include:` at any top-level indent.
        if trimmed == "include:" {
            flush(&mut out, &mut cur_project, &mut cur_ref);
            in_include = true;
            continue;
        }

        // Leaving the include block when we see a top-level key.
        if indent == 0 && !trimmed.starts_with('-') && in_include {
            flush(&mut out, &mut cur_project, &mut cur_ref);
            in_include = false;
            continue;
        }

        if !in_include {
            continue;
        }

        // New list item: `- project: X` or `- ref: Y`
        if trimmed.starts_with("- ") {
            flush(&mut out, &mut cur_project, &mut cur_ref);
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

    flush(&mut out, &mut cur_project, &mut cur_ref);
    out
}

fn flush(
    out: &mut Vec<GitlabIncludeDep>,
    project: &mut Option<String>,
    ref_val: &mut Option<String>,
) {
    if let (Some(p), Some(r)) = (project.take(), ref_val.take())
        && !p.is_empty()
        && !r.is_empty()
    {
        out.push(GitlabIncludeDep {
            project: p,
            ref_value: r,
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

    #[test]
    fn include_without_ref_skipped() {
        let content = "include:\n  - project: org/templates\n    file: /f.yml\n";
        let deps = extract(content);
        assert!(deps.is_empty());
    }

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
}
