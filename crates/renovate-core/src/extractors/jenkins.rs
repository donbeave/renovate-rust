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
    /// Plugin has a `# renovate:ignore` comment.
    Ignored,
}

/// Parse a `plugins.txt` file (one `plugin-id:version` entry per line).
pub fn extract_txt(content: &str) -> Vec<JenkinsPluginDep> {
    let mut out = Vec::new();

    use crate::string_match::is_skip_comment;
    for raw in content.lines() {
        // Extract comment before discarding it (needed for renovate:ignore check).
        let mut parts = raw.splitn(2, '#');
        let line = parts.next().unwrap_or(raw).trim();
        let comment = parts.next().unwrap_or("").trim();

        if line.is_empty() {
            continue;
        }
        let renovate_ignored = is_skip_comment(comment);

        if let Some((id, version)) = line.split_once(':') {
            let id = id.trim();
            let version = version.trim();
            if id.is_empty() {
                continue;
            }
            let skip_reason = if renovate_ignored {
                Some(JenkinsSkipReason::Ignored)
            } else if version.is_empty() {
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

    // Ported: "returns empty list for an empty text file" — jenkins/extract.spec.ts line 15
    #[test]
    fn txt_empty_file_returns_empty() {
        assert!(extract_txt("").is_empty());
    }

    // Ported: "returns empty list for an invalid yaml file" — jenkins/extract.spec.ts line 27
    #[test]
    fn yml_invalid_yaml_returns_empty() {
        let content = "this: is: invalid: yaml: content\n";
        let deps = extract_yml(content);
        assert!(deps.is_empty());
    }

    // Ported: "extracts multiple image lines in text format" — jenkins/extract.spec.ts line 33
    #[test]
    fn txt_plugins_fixture_six_deps() {
        // Mirrors jenkins/__fixtures__/plugins.txt; 4 valid + 2 renovate:ignore = 6 total
        let content = "email-ext:1.2.3\n\
                       apache-httpcomponents-client-4-api:4.4.10-2.0 # comment\n\
                       authentication-tokens:1.2\n\
                       blueocean:1.21.0 # another comment\n\
                       #blueocean:1.22.0\n\
                       \n\
                       # these deps will be ignored:\n\
                       git:4.2.0         # renovate:ignore\n\
                       git-client:3.3.1  # renovate:ignore\n";
        let deps = extract_txt(content);
        assert_eq!(deps.len(), 6);
        assert!(
            deps.iter()
                .any(|d| d.artifact_id == "email-ext" && d.version.as_deref() == Some("1.2.3"))
        );
        assert!(
            deps.iter()
                .any(|d| d.artifact_id == "authentication-tokens")
        );
        assert!(deps.iter().any(|d| d.artifact_id == "blueocean"));
        let git = deps.iter().find(|d| d.artifact_id == "git").unwrap();
        assert_eq!(git.skip_reason, Some(JenkinsSkipReason::Ignored));
        let git_client = deps.iter().find(|d| d.artifact_id == "git-client").unwrap();
        assert_eq!(git_client.skip_reason, Some(JenkinsSkipReason::Ignored));
    }

    // Ported: "extracts multiple image lines in yaml format" — jenkins/extract.spec.ts line 40
    #[test]
    fn yml_plugins_fixture_eight_deps() {
        // Mirrors jenkins/__fixtures__/plugins.yaml; 8 total including skipped
        let content = r#"plugins:
  - artifactId: git
    source:
      version: latest
  - artifactId: job-import-plugin
    source:
      version: '2.10'
  - artifactId: invalid-version-plugin
    source:
      version: 2.10
  - artifactId: ignore-plugin
    source:
      version: '2.10'
    renovate:
      ignore: true
  - artifactId: docker
  - artifactId: cloudbees-bitbucket-branch-source
    source:
      version: experimental
  - artifactId: script-security
    source:
      url: http://ftp-chi.osuosl.org/pub/jenkins/plugins/script-security/1.56/script-security.hpi
  - artifactId: workflow-step-api
    groupId: org.jenkins-ci.plugins.workflow
    source:
      version: 2.19-rc289.d09828a05a74
"#;
        let deps = extract_yml(content);
        assert_eq!(deps.len(), 8);
        // git: version=latest → UnsupportedVersion
        let git = deps.iter().find(|d| d.artifact_id == "git").unwrap();
        assert_eq!(git.skip_reason, Some(JenkinsSkipReason::UnsupportedVersion));
        // job-import-plugin: valid
        let jip = deps
            .iter()
            .find(|d| d.artifact_id == "job-import-plugin")
            .unwrap();
        assert!(jip.skip_reason.is_none());
        // docker: no version → UnspecifiedVersion
        let docker = deps.iter().find(|d| d.artifact_id == "docker").unwrap();
        assert_eq!(
            docker.skip_reason,
            Some(JenkinsSkipReason::UnspecifiedVersion)
        );
        // workflow-step-api: prerelease → valid
        let wsa = deps
            .iter()
            .find(|d| d.artifact_id == "workflow-step-api")
            .unwrap();
        assert!(wsa.skip_reason.is_none());
    }
}
