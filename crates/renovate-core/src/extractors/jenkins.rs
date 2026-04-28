//! Jenkins `plugins.txt` and `plugins.yml` extractor.
//!
//! Extracts Jenkins plugin dependencies from two file formats:
//!
//! * **`plugins.txt`** — one `plugin-id:version` per line (comments with `#`).
//! * **`plugins.yml`** — YAML file with a `plugins:` list where each item has
//!   an `artifactId:` field and optionally a `source.version:` or just
//!   `version:` field.
//!
//! Renovate reference:
//! - `lib/modules/manager/jenkins/extract.ts`
//! - Pattern: `/(^|/)plugins\.(txt|ya?ml)$/`
//!
//! ## Supported forms
//!
//! **plugins.txt:**
//! ```text
//! git:4.13.0
//! pipeline-model-definition:2.2189.v726e1b_df63f8
//! # comment line ignored
//! ```
//!
//! **plugins.yml:**
//! ```yaml
//! plugins:
//! - artifactId: git
//!   source:
//!     version: 4.13.0
//! - artifactId: workflow-aggregator
//!   source:
//!     version: 2.6
//! ```

/// A single Jenkins plugin dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JenkinsPluginDep {
    /// Plugin artifact ID (e.g. `git`, `pipeline-model-definition`).
    pub artifact_id: String,
    /// Version string, if specified.
    pub version: Option<String>,
    /// Set when this dep should be skipped.
    pub skip_reason: Option<JenkinsSkipReason>,
}

/// Why a Jenkins plugin dep is skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JenkinsSkipReason {
    UnspecifiedVersion,
    UnsupportedVersion,
}

/// Parse a `plugins.txt` file (one `plugin-id:version` entry per line).
pub fn extract_txt(content: &str) -> Vec<JenkinsPluginDep> {
    let mut out = Vec::new();

    for raw in content.lines() {
        // Strip inline comments.
        let line = raw.split('#').next().unwrap_or(raw).trim();
        if line.is_empty() {
            continue;
        }

        if let Some((id, version)) = line.split_once(':') {
            let id = id.trim();
            let version = version.trim();
            if id.is_empty() {
                continue;
            }
            let skip_reason = if version.is_empty() {
                Some(JenkinsSkipReason::UnspecifiedVersion)
            } else if matches!(version, "latest" | "experimental") {
                Some(JenkinsSkipReason::UnsupportedVersion)
            } else {
                None
            };
            out.push(JenkinsPluginDep {
                artifact_id: id.to_owned(),
                version: if version.is_empty() {
                    None
                } else {
                    Some(version.to_owned())
                },
                skip_reason,
            });
        }
    }

    out
}

/// Parse a `plugins.yml` file.
///
/// Expected structure:
/// ```yaml
/// plugins:
/// - artifactId: git
///   source:
///     version: 4.13.0
/// ```
pub fn extract_yml(content: &str) -> Vec<JenkinsPluginDep> {
    let mut out = Vec::new();
    let mut in_plugins = false;
    let mut cur_id: Option<String> = None;
    let mut cur_version: Option<String> = None;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            continue;
        }
        let indent = leading_spaces(line);

        if indent == 0 && !trimmed.starts_with('-') {
            if in_plugins {
                flush_plugin(&mut out, &mut cur_id, &mut cur_version);
            }
            in_plugins = trimmed == "plugins:";
            continue;
        }

        if !in_plugins {
            continue;
        }

        if trimmed.starts_with("- ") {
            flush_plugin(&mut out, &mut cur_id, &mut cur_version);
            let rest = trimmed.strip_prefix("- ").unwrap_or("");
            if let Some(v) = strip_key(rest, "artifactId") {
                cur_id = Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
            }
            continue;
        }

        if let Some(v) = strip_key(trimmed, "artifactId") {
            cur_id = Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
        } else if let Some(v) = strip_key(trimmed, "version") {
            cur_version = Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
        }
    }

    if in_plugins {
        flush_plugin(&mut out, &mut cur_id, &mut cur_version);
    }

    out
}

fn flush_plugin(
    out: &mut Vec<JenkinsPluginDep>,
    id: &mut Option<String>,
    version: &mut Option<String>,
) {
    if let Some(artifact_id) = id.take() {
        let ver = version.take();
        let skip_reason = match ver.as_deref() {
            None | Some("") => Some(JenkinsSkipReason::UnspecifiedVersion),
            Some("latest") | Some("experimental") => Some(JenkinsSkipReason::UnsupportedVersion),
            _ => None,
        };
        out.push(JenkinsPluginDep {
            artifact_id,
            version: ver.filter(|v| !v.is_empty()),
            skip_reason,
        });
    } else {
        version.take();
    }
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

    // ── plugins.txt ──────────────────────────────────────────────────────────

    #[test]
    fn txt_extracts_plugin() {
        let content = "git:4.13.0\npipeline-model-definition:2.2189.v726e\n";
        let deps = extract_txt(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].artifact_id, "git");
        assert_eq!(deps[0].version.as_deref(), Some("4.13.0"));
        assert!(deps[0].skip_reason.is_none());
    }

    #[test]
    fn txt_comment_lines_ignored() {
        let content = "# this is a header\ngit:4.13.0\n# another comment\n";
        let deps = extract_txt(content);
        assert_eq!(deps.len(), 1);
    }

    #[test]
    fn txt_inline_comment_stripped() {
        let content = "git:4.13.0 # keep this version\n";
        let deps = extract_txt(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].version.as_deref(), Some("4.13.0"));
    }

    #[test]
    fn txt_latest_skipped() {
        let content = "git:latest\n";
        let deps = extract_txt(content);
        assert_eq!(
            deps[0].skip_reason,
            Some(JenkinsSkipReason::UnsupportedVersion)
        );
    }

    #[test]
    fn txt_empty_returns_empty() {
        assert!(extract_txt("").is_empty());
    }

    // ── plugins.yml ──────────────────────────────────────────────────────────

    #[test]
    fn yml_extracts_plugin() {
        let content = r#"
plugins:
- artifactId: git
  source:
    version: 4.13.0
"#;
        let deps = extract_yml(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].artifact_id, "git");
        assert_eq!(deps[0].version.as_deref(), Some("4.13.0"));
        assert!(deps[0].skip_reason.is_none());
    }

    #[test]
    fn yml_multiple_plugins() {
        let content = r#"
plugins:
- artifactId: git
  source:
    version: 4.13.0
- artifactId: workflow-aggregator
  source:
    version: 2.6
"#;
        let deps = extract_yml(content);
        assert_eq!(deps.len(), 2);
        assert!(deps.iter().any(|d| d.artifact_id == "git"));
        assert!(deps.iter().any(|d| d.artifact_id == "workflow-aggregator"));
    }

    #[test]
    fn yml_no_version_skipped() {
        let content = "plugins:\n- artifactId: git\n";
        let deps = extract_yml(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(JenkinsSkipReason::UnspecifiedVersion)
        );
    }

    #[test]
    fn yml_empty_returns_empty() {
        assert!(extract_yml("").is_empty());
    }
}
