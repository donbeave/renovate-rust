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

    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }
}
