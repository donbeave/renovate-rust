//! GitHub Actions `uses:` dependency extractor and container/services image extractor.
//!
//! Scans workflow YAML files line-by-line for `uses:` entries (actions) and
//! `container:`/`services:` entries (Docker images).
//!
//! Renovate reference:
//! - `lib/modules/manager/github-actions/extract.ts` — `extractPackageFile`
//! - `lib/modules/manager/github-actions/parse.ts`   — `parseUsesLine`,
//!   `isSha`, `isShortSha`, `versionLikeRe`
//! - `lib/modules/manager/github-actions/schema.ts`  — `WorkFlowJobs.container`,
//!   `WorkFlowJobs.services`
//!
//! ## Supported `uses:` forms
//!
//! | Form | Treatment |
//! |---|---|
//! | `owner/repo@vX.Y` | Actionable — look up via `github-tags` |
//! | `owner/repo/path@vX.Y` | Actionable — `owner/repo` is the lookup target |
//! | `./.github/actions/local` | Skipped — `LocalAction` |
//! | `docker://image:tag` | Skipped — `DockerRef` (separate datasource) |
//! | `owner/repo@<40-hex>` | Skipped — `ShaPin` |
//! | `owner/repo@<6-7-hex>` | Skipped — `ShortShaPin` |
//!
//! ## Supported container/services forms
//!
//! ```yaml
//! jobs:
//!   build:
//!     container: node:18              # inline
//!     container:                      # block form
//!       image: node:18
//!     services:
//!       redis:                        # service block
//!         image: redis:5
//!       postgres: postgres:10        # inline service string
//! ```

use std::sync::LazyLock;

use regex::Regex;

use crate::extractors::dockerfile::{DockerfileExtractedDep, classify_image_ref};

/// Why a GitHub Actions dep is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GithubActionsSkipReason {
    /// `uses: ./.github/actions/…` — local action definition.
    LocalAction,
    /// `uses: docker://…` — references a Docker image (different datasource).
    DockerRef,
    /// Full 40- or 64-character hex SHA pin.
    ShaPin,
    /// Short 6–7 character hex SHA reference.
    ShortShaPin,
}

/// A single extracted GitHub Actions dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GithubActionsExtractedDep {
    /// `owner/repo` (without sub-path or version).
    pub action: String,
    /// The tag/ref used (e.g. `"v4"`, `"v4.0.1"`).
    pub current_value: String,
    /// Set when the dep should not be looked up in the registry.
    pub skip_reason: Option<GithubActionsSkipReason>,
}

// ── Compiled regexes ───────────────────────────────────────────────────────

/// Matches a `uses:` line inside a workflow YAML file.
/// Captures the remainder after `uses:`.
static USES_LINE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^\s+(?:-\s+)?uses:\s+(.+)$").unwrap());

/// 40- or 64-character lowercase hex SHA.
static SHA_FULL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-f0-9]{40}$|^[a-f0-9]{64}$").unwrap());

/// 6–7 character lowercase hex short SHA.
static SHA_SHORT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-f0-9]{6,7}$").unwrap());

// ── Public API ─────────────────────────────────────────────────────────────

/// Parse a GitHub Actions workflow YAML file and extract `uses:` references.
pub fn extract(content: &str) -> Vec<GithubActionsExtractedDep> {
    let mut deps = Vec::new();

    for cap in USES_LINE.captures_iter(content) {
        let remainder = cap[1].trim();
        // Strip inline comment (# …) and trailing quotes.
        let raw = strip_comment(remainder);
        let raw = raw.trim_matches(|c| c == '\'' || c == '"');

        if let Some(dep) = parse_uses(raw) {
            deps.push(dep);
        }
    }

    deps
}

// ── Helpers ───────────────────────────────────────────────────────────────

fn parse_uses(raw: &str) -> Option<GithubActionsExtractedDep> {
    // Local action reference.
    if raw.starts_with("./") {
        return Some(GithubActionsExtractedDep {
            action: raw.to_owned(),
            current_value: String::new(),
            skip_reason: Some(GithubActionsSkipReason::LocalAction),
        });
    }

    // Docker container action.
    if raw.starts_with("docker://") {
        return Some(GithubActionsExtractedDep {
            action: raw.to_owned(),
            current_value: String::new(),
            skip_reason: Some(GithubActionsSkipReason::DockerRef),
        });
    }

    // Repository reference: `owner/repo[/path]@ref`
    let (action_path, version) = raw.split_once('@')?;

    // Strip optional sub-path to get `owner/repo`.
    let action = owner_repo(action_path)?;

    // Classify the ref.
    let skip_reason = if SHA_FULL.is_match(version) {
        Some(GithubActionsSkipReason::ShaPin)
    } else if SHA_SHORT.is_match(version) {
        Some(GithubActionsSkipReason::ShortShaPin)
    } else {
        None
    };

    Some(GithubActionsExtractedDep {
        action,
        current_value: version.to_owned(),
        skip_reason,
    })
}

/// Extract `owner/repo` from `owner/repo[/optional/sub/path]`.
fn owner_repo(action_path: &str) -> Option<String> {
    let mut parts = action_path.splitn(3, '/');
    let owner = parts.next()?.trim();
    let repo = parts.next()?.trim();
    if owner.is_empty() || repo.is_empty() {
        return None;
    }
    Some(format!("{owner}/{repo}"))
}

/// Strip a trailing `# comment` from a YAML value.
fn strip_comment(s: &str) -> &str {
    if let Some(idx) = s.find(" #") {
        s[..idx].trim()
    } else {
        s
    }
}

fn leading_spaces(s: &str) -> usize {
    s.len() - s.trim_start_matches([' ', '\t']).len()
}

fn strip_key<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}:");
    line.strip_prefix(prefix.as_str())
}

// ── Container / Services image extraction ────────────────────────────────────

#[derive(Clone, Copy)]
enum GaDockerState {
    Default,
    /// Inside `container:` block form — looking for `image:`.
    InContainerBlock {
        indent: usize,
    },
    /// Inside `services:` block.
    InServices {
        svc_indent: usize,
        /// Indent level of the first service-name entry (set on first deep line).
        service_level: Option<usize>,
    },
}

/// Extract Docker image deps from the `container:` and `services:` fields of
/// a GitHub Actions workflow YAML.
///
/// Supports the two container forms (inline string and `image:` block) and
/// services that are either inline strings or objects with an `image:` key.
pub fn extract_docker_images(content: &str) -> Vec<DockerfileExtractedDep> {
    let mut out: Vec<DockerfileExtractedDep> = Vec::new();
    let mut st = GaDockerState::Default;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        if line.trim().is_empty() {
            continue;
        }
        let trimmed = line.trim_start();
        let indent = leading_spaces(line);

        match st {
            GaDockerState::Default => {
                st = transition_default(trimmed, indent, &mut out);
            }
            GaDockerState::InContainerBlock {
                indent: block_indent,
            } => {
                if indent <= block_indent {
                    // Exited the container block — reprocess line as Default.
                    st = transition_default(trimmed, indent, &mut out);
                } else if let Some(rest) = strip_key(trimmed, "image") {
                    let val = rest.trim().trim_matches('"').trim_matches('\'');
                    if !val.is_empty() && !val.starts_with('$') {
                        out.push(classify_image_ref(val));
                    }
                    st = GaDockerState::Default;
                }
            }
            GaDockerState::InServices {
                svc_indent,
                service_level,
            } => {
                if indent <= svc_indent {
                    // Exited the services section — reprocess line as Default.
                    st = transition_default(trimmed, indent, &mut out);
                    continue;
                }
                let sni = service_level.unwrap_or(indent);
                if indent == sni {
                    // Service-name entry: `redis:` (block) or `postgres: image-ref` (inline).
                    if let Some(colon_pos) = trimmed.find(':') {
                        let value = trimmed[colon_pos + 1..].trim();
                        if !value.is_empty() && !value.starts_with('#') && !value.starts_with('$') {
                            let val = value.trim_matches('"').trim_matches('\'');
                            if !val.is_empty() {
                                out.push(classify_image_ref(val));
                            }
                        }
                    }
                } else if let Some(rest) = strip_key(trimmed, "image") {
                    // Inside a service block: `image: redis:5`.
                    let val = rest.trim().trim_matches('"').trim_matches('\'');
                    if !val.is_empty() && !val.starts_with('$') {
                        out.push(classify_image_ref(val));
                    }
                }
                st = GaDockerState::InServices {
                    svc_indent,
                    service_level: Some(sni),
                };
            }
        }
    }

    out
}

/// Process one line in the Default context and return the next state.
fn transition_default(
    trimmed: &str,
    indent: usize,
    out: &mut Vec<DockerfileExtractedDep>,
) -> GaDockerState {
    if let Some(rest) = strip_key(trimmed, "container") {
        let val = rest.trim().trim_matches('"').trim_matches('\'');
        if val.is_empty() || val.starts_with('#') {
            GaDockerState::InContainerBlock { indent }
        } else if !val.starts_with('$') {
            out.push(classify_image_ref(val));
            GaDockerState::Default
        } else {
            GaDockerState::Default
        }
    } else if trimmed == "services:" {
        GaDockerState::InServices {
            svc_indent: indent,
            service_level: None,
        }
    } else {
        GaDockerState::Default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dep(action: &str, current_value: &str) -> GithubActionsExtractedDep {
        GithubActionsExtractedDep {
            action: action.to_owned(),
            current_value: current_value.to_owned(),
            skip_reason: None,
        }
    }

    #[test]
    fn extracts_simple_action() {
        let content = r#"
jobs:
  build:
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v3
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(deps.contains(&dep("actions/checkout", "v4")));
        assert!(deps.contains(&dep("actions/setup-node", "v3")));
    }

    #[test]
    fn action_with_sub_path_uses_owner_repo() {
        let content = "      - uses: org/repo/subpath@v2\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].action, "org/repo");
        assert_eq!(deps[0].current_value, "v2");
    }

    #[test]
    fn local_action_skipped() {
        let content = "      - uses: ./.github/actions/my-action\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(GithubActionsSkipReason::LocalAction)
        );
    }

    #[test]
    fn docker_ref_skipped() {
        let content = "      - uses: docker://alpine:3.18\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(GithubActionsSkipReason::DockerRef)
        );
    }

    #[test]
    fn full_sha_pin_skipped() {
        let content = "      - uses: actions/checkout@a81bbbf8298c0fa03ea29cdc473d45769f953675\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(GithubActionsSkipReason::ShaPin));
    }

    #[test]
    fn short_sha_pin_skipped() {
        let content = "      - uses: actions/checkout@abc1234\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(GithubActionsSkipReason::ShortShaPin)
        );
    }

    #[test]
    fn quoted_action_is_parsed() {
        let content = r#"      - uses: "actions/checkout@v4""#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].action, "actions/checkout");
        assert_eq!(deps[0].current_value, "v4");
    }

    #[test]
    fn inline_comment_stripped() {
        let content = "      - uses: actions/checkout@v4 # pinned\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "v4");
    }

    #[test]
    fn real_workflow_fixture() {
        let content = r#"
name: CI
on: [push]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
      - name: Run tests
        run: pytest
      - uses: ./.github/actions/my-local
      - uses: actions/upload-artifact@a8a3f3ad30e3422c9c7b888a15615d19a852ae32
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 4); // checkout, setup-python, local, sha-pinned

        let checkout = deps
            .iter()
            .find(|d| d.action == "actions/checkout")
            .unwrap();
        assert!(checkout.skip_reason.is_none());
        assert_eq!(checkout.current_value, "v4");

        let local = deps
            .iter()
            .find(|d| d.action == "./.github/actions/my-local")
            .unwrap();
        assert_eq!(
            local.skip_reason,
            Some(GithubActionsSkipReason::LocalAction)
        );

        let pinned = deps
            .iter()
            .find(|d| d.action == "actions/upload-artifact")
            .unwrap();
        assert_eq!(pinned.skip_reason, Some(GithubActionsSkipReason::ShaPin));
    }

    #[test]
    fn empty_content_returns_empty() {
        assert!(extract("").is_empty());
    }

    #[test]
    fn owner_repo_strips_subpath() {
        assert_eq!(
            owner_repo("actions/setup-node"),
            Some("actions/setup-node".to_owned())
        );
        assert_eq!(owner_repo("org/repo/sub/path"), Some("org/repo".to_owned()));
        assert_eq!(owner_repo("nodot"), None);
    }

    // ── extract_docker_images tests ───────────────────────────────────────────

    #[test]
    fn docker_container_inline() {
        let content = r#"
jobs:
  build:
    runs-on: ubuntu-latest
    container: node:16-bullseye
    steps:
      - uses: actions/checkout@v4
"#;
        let deps = extract_docker_images(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "node");
        assert_eq!(deps[0].tag.as_deref(), Some("16-bullseye"));
        assert!(deps[0].skip_reason.is_none());
    }

    #[test]
    fn docker_container_block_form() {
        let content = r#"
jobs:
  build:
    runs-on: ubuntu-latest
    container:
      image: node:16-bullseye
      options: --cpus 1
"#;
        let deps = extract_docker_images(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "node");
        assert_eq!(deps[0].tag.as_deref(), Some("16-bullseye"));
    }

    #[test]
    fn docker_services_block_image() {
        let content = r#"
jobs:
  test:
    runs-on: ubuntu-latest
    services:
      redis:
        image: redis:5
      postgres:
        image: postgres:14
"#;
        let deps = extract_docker_images(content);
        assert_eq!(deps.len(), 2);
        assert!(
            deps.iter()
                .any(|d| d.image == "redis" && d.tag.as_deref() == Some("5"))
        );
        assert!(
            deps.iter()
                .any(|d| d.image == "postgres" && d.tag.as_deref() == Some("14"))
        );
    }

    #[test]
    fn docker_services_inline_string() {
        let content = r#"
jobs:
  test:
    runs-on: ubuntu-latest
    services:
      postgres: postgres:10
"#;
        let deps = extract_docker_images(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].image, "postgres");
        assert_eq!(deps[0].tag.as_deref(), Some("10"));
    }

    #[test]
    fn docker_mixed_container_and_services() {
        let content = r#"
jobs:
  container-job:
    runs-on: ubuntu-latest
    container: node:16-bullseye
    services:
      redis:
        image: redis:5
      postgres: postgres:10
  container-job-with-image-keyword:
    runs-on: ubuntu-latest
    container:
      image: node:18-alpine
"#;
        let deps = extract_docker_images(content);
        assert_eq!(deps.len(), 4);
        assert!(
            deps.iter()
                .any(|d| d.image == "node" && d.tag.as_deref() == Some("16-bullseye"))
        );
        assert!(
            deps.iter()
                .any(|d| d.image == "redis" && d.tag.as_deref() == Some("5"))
        );
        assert!(
            deps.iter()
                .any(|d| d.image == "postgres" && d.tag.as_deref() == Some("10"))
        );
        assert!(
            deps.iter()
                .any(|d| d.image == "node" && d.tag.as_deref() == Some("18-alpine"))
        );
    }

    #[test]
    fn docker_var_refs_skipped() {
        let content = r#"
jobs:
  build:
    container: ${{ env.MY_IMAGE }}
    services:
      db:
        image: $MY_DB_IMAGE
"#;
        let deps = extract_docker_images(content);
        assert!(deps.is_empty(), "variable references should be skipped");
    }

    #[test]
    fn docker_no_container_no_services_returns_empty() {
        let content = r#"
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
"#;
        assert!(extract_docker_images(content).is_empty());
    }

    #[test]
    fn docker_workflow_fixture() {
        // Reflects upstream workflow_1.yml fixture from renovatebot/renovate.
        let content = r#"
jobs:
  container-job:
    runs-on: ubuntu-latest
    container: node:16-bullseye
    services:
      redis:
        image: redis:5
      postgres: postgres:10
  container-job-with-image-keyword:
    runs-on: ubuntu-latest
    container:
      image: node:16-bullseye
"#;
        let deps = extract_docker_images(content);
        // container inline, redis (block), postgres (inline), container block
        assert_eq!(deps.len(), 4);
    }
}
