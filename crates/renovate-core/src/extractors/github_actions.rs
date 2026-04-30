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
        // Capture the version comment (e.g. "v4" from "# v4") before stripping.
        let version_comment = comment_version(remainder);
        // Strip inline comment (# …) and trailing quotes.
        let raw = strip_comment(remainder);
        let raw = raw.trim_matches(|c| c == '\'' || c == '"');

        if let Some(dep) = parse_uses(raw, version_comment) {
            deps.push(dep);
        }
    }

    deps
}

/// Extract and normalise the version string from a trailing `# <version>` comment.
///
/// Handles these forms (TypeScript parity):
/// - `# v1.2`            → `v1.2`
/// - `# @v1.2`           → `v1.2`  (leading `@` stripped)
/// - `# pin @v1.2`       → `v1.2`
/// - `# tag=v1.2`        → `v1.2`
/// - `# ratchet:o/r@v1`  → `v1`   (rightmost `@…` component)
/// - `# ratchet:exclude` → None
/// - `#v2` (no space)    → `v2`
fn comment_version(s: &str) -> Option<&str> {
    // Accept both ` #` and `#` (no space before hash).
    let comment_start = if let Some(i) = s.find(" #") {
        i + 2
    } else if let Some(i) = s.find('#') {
        i + 1
    } else {
        return None;
    };
    let raw = s[comment_start..].trim();
    if raw.is_empty() {
        return None;
    }

    // ratchet:owner/repo@version  or  ratchet:exclude
    if let Some(rest) = raw.strip_prefix("ratchet:") {
        if rest == "exclude" {
            return None;
        }
        return rest.rfind('@').map(|i| &rest[i + 1..]);
    }

    // tag=version
    if let Some(v) = raw.strip_prefix("tag=") {
        return if v.is_empty() { None } else { Some(v) };
    }

    // pin @version  or  pin@version
    let without_pin = raw
        .strip_prefix("pin ")
        .or_else(|| raw.strip_prefix("pin@"))
        .unwrap_or(raw);

    // Strip leading `@`
    let v = without_pin.trim_start_matches('@');
    if v.is_empty() { None } else { Some(v) }
}

// ── Helpers ───────────────────────────────────────────────────────────────

fn parse_uses(raw: &str, version_comment: Option<&str>) -> Option<GithubActionsExtractedDep> {
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

    // A SHA pin with a version comment (e.g. `@sha # v4`) is treated as a
    // versioned reference: the comment provides the version, the SHA is the
    // digest.  Without a comment, naked SHAs are skipped.
    if SHA_FULL.is_match(version) {
        if let Some(vc) = version_comment {
            return Some(GithubActionsExtractedDep {
                action,
                current_value: vc.to_owned(),
                skip_reason: None,
            });
        }
        return Some(GithubActionsExtractedDep {
            action,
            current_value: version.to_owned(),
            skip_reason: Some(GithubActionsSkipReason::ShaPin),
        });
    }
    if SHA_SHORT.is_match(version) {
        if let Some(vc) = version_comment {
            return Some(GithubActionsExtractedDep {
                action,
                current_value: vc.to_owned(),
                skip_reason: None,
            });
        }
        return Some(GithubActionsExtractedDep {
            action,
            current_value: version.to_owned(),
            skip_reason: Some(GithubActionsSkipReason::ShortShaPin),
        });
    }

    Some(GithubActionsExtractedDep {
        action,
        current_value: version.to_owned(),
        skip_reason: None,
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

// ── Runner label extraction ───────────────────────────────────────────────────

/// A `runs-on:` runner label extracted from a GitHub Actions workflow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GhRunnerDep {
    /// Runner family name (e.g., `"ubuntu"`, `"macos"`, `"windows"`).
    pub runner_name: String,
    /// Version string (e.g., `"22.04"`, `"14-xlarge"`, `"2022"`).
    pub current_value: String,
}

/// Extract `runs-on:` runner labels from a GitHub Actions workflow YAML.
///
/// Handles:
/// - Inline single value: `runs-on: ubuntu-22.04`
/// - Inline array: `runs-on: [ubuntu-22.04, self-hosted]`
///
/// Skips `ubuntu-latest`, matrix expressions (`${{...}}`), and any runner
/// names not in the known static runner table.
pub fn extract_runner_labels(content: &str) -> Vec<GhRunnerDep> {
    use crate::datasources::github_runners;

    let mut out = Vec::new();

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        let trimmed = line.trim_start();

        let Some(rest) = strip_key(trimmed, "runs-on") else {
            continue;
        };
        let rest = rest.trim();

        // Collect one or more runner strings from this line.
        let runners: Vec<&str> = if rest.starts_with('[') {
            // Inline array: `[ubuntu-22.04, self-hosted]`
            let inner = rest.trim_start_matches('[').trim_end_matches(']');
            inner.split(',').map(|s| s.trim()).collect()
        } else {
            vec![rest]
        };

        for runner_str in runners {
            let runner_str = runner_str.trim_matches('"').trim_matches('\'');
            // Skip variable references and empty values.
            if runner_str.is_empty() || runner_str.starts_with('$') {
                continue;
            }
            // Parse `{name}-{version}` — name is alpha-only, version is the rest.
            if let Some((name, version)) = parse_runner_label(runner_str) {
                // Skip `ubuntu-latest`, `macos-latest`, etc.
                if version == "latest" {
                    continue;
                }
                // Only emit if this is a known runner+version combination.
                if github_runners::is_valid_runner(name, version) {
                    out.push(GhRunnerDep {
                        runner_name: name.to_owned(),
                        current_value: version.to_owned(),
                    });
                }
            }
        }
    }

    out
}

/// Split `ubuntu-22.04` → `("ubuntu", "22.04")`, `macos-14-xlarge` → `("macos", "14-xlarge")`.
///
/// The runner name is the leading all-alpha prefix; the version is everything
/// after the first `-` that follows that prefix.
fn parse_runner_label(s: &str) -> Option<(&str, &str)> {
    let dash = s.find(|c: char| !c.is_ascii_alphabetic())?;
    if s.as_bytes().get(dash) != Some(&b'-') {
        return None;
    }
    let name = &s[..dash];
    let version = &s[dash + 1..];
    if name.is_empty() || version.is_empty() {
        return None;
    }
    Some((name, version))
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

    // Ported: "extracts multiple action tag lines from yaml configuration file" — github-actions/extract.spec.ts line 65
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

    // Ported: "extracts multiple action tag lines from yaml configuration file" — github-actions/extract.spec.ts line 65
    #[test]
    fn action_with_sub_path_uses_owner_repo() {
        let content = "      - uses: org/repo/subpath@v2\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].action, "org/repo");
        assert_eq!(deps[0].current_value, "v2");
    }

    // Ported: "extracts multiple action tag lines from yaml configuration file" — github-actions/extract.spec.ts line 65
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

    // Ported: "extracts multiple docker image lines from yaml configuration file" — github-actions/extract.spec.ts line 54
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

    // Ported: "disables naked SHA pins without version comment" — github-actions/extract.spec.ts line 527
    #[test]
    fn full_sha_pin_skipped() {
        let content = "      - uses: actions/checkout@a81bbbf8298c0fa03ea29cdc473d45769f953675\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(GithubActionsSkipReason::ShaPin));
    }

    // Ported: "disables naked short SHA pins without version comment" — github-actions/extract.spec.ts line 546
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

    // Ported: "does not disable SHA pins with version comment" — github-actions/extract.spec.ts line 565
    #[test]
    fn full_sha_with_version_comment_not_skipped() {
        let content =
            "      - uses: actions/checkout@c85c95e3d7251135ab7dc9ce3241c5835cc595a9 # v4\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].action, "actions/checkout");
        assert_eq!(deps[0].current_value, "v4");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "does not disable short SHA pins with version comment" — github-actions/extract.spec.ts line 590
    #[test]
    fn short_sha_with_version_comment_not_skipped() {
        let content = "      - uses: actions/checkout@c85c95e # v4\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].action, "actions/checkout");
        assert_eq!(deps[0].current_value, "v4");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "extracts tags in different formats" — github-actions/extract.spec.ts line 352
    #[test]
    fn comment_version_formats() {
        let sha = "1e204e9a9253d643386038d443f96446fa156a97";
        let cases: &[(&str, &str)] = &[
            // bare version comment
            (
                &format!("      - uses: actions/checkout@{sha} # 1.2.3\n"),
                "1.2.3",
            ),
            (
                &format!("      - uses: actions/checkout@{sha} # v1.2.3\n"),
                "v1.2.3",
            ),
            // leading @ stripped
            (
                &format!("      - uses: actions/checkout@{sha} # @v2.1.0\n"),
                "v2.1.0",
            ),
            // pin @version
            (
                &format!("      - uses: actions/checkout@{sha} # pin @v2.1.0\n"),
                "v2.1.0",
            ),
            // tag=version
            (
                &format!("      - uses: actions/checkout@{sha} # tag=v2.1.0\n"),
                "v2.1.0",
            ),
            // extra whitespace
            (
                &format!("      - uses: actions/checkout@{sha}  #   v2.1.0\n"),
                "v2.1.0",
            ),
            // no space before hash
            (
                &format!("      - uses: actions/checkout@{sha} #v2.1.0\n"),
                "v2.1.0",
            ),
            // ratchet:owner/repo@version
            (
                &format!(
                    "      - uses: actions/checkout@{sha} # ratchet:actions/checkout@v2.1.0\n"
                ),
                "v2.1.0",
            ),
        ];
        for (content, expected) in cases {
            let deps = extract(content);
            assert_eq!(deps[0].current_value, *expected, "failed for: {content}");
            assert!(
                deps[0].skip_reason.is_none(),
                "unexpected skip for: {content}"
            );
        }
    }

    // Ported: "extracts non-semver ref automatically" — github-actions/extract.spec.ts line 484
    #[test]
    fn non_semver_ref_extracted() {
        let content = "      - uses: taiki-e/install-action@cargo-llvm-cov\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].action, "taiki-e/install-action");
        assert_eq!(deps[0].current_value, "cargo-llvm-cov");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "extracts pinned non-semver ref with digest" — github-actions/extract.spec.ts line 504
    #[test]
    fn pinned_non_semver_ref_with_digest() {
        let content = "      - uses: taiki-e/install-action@4b1248585248751e3b12fd020cf7ac91540ca09c # cargo-llvm-cov\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].action, "taiki-e/install-action");
        assert_eq!(deps[0].current_value, "cargo-llvm-cov");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "maintains quotes" — github-actions/extract.spec.ts line 217
    #[test]
    fn single_and_double_quoted_uses_parsed() {
        let sha = "56337c425554a6be30cdef71bf441f15be286854";
        let content = [
            format!("      - uses: actions/setup-node@{sha} # tag=v3.1.1"),
            format!("      - uses: 'actions/setup-node@{sha}' # tag=v3.1.1"),
            format!("      - uses: \"actions/setup-node@{sha}\" # tag=v2.5.1"),
            "      - uses: \"actions/checkout@v2\" # comment after".to_owned(),
        ]
        .join("\n");
        let deps = extract(&content);
        assert_eq!(deps.len(), 4);
        assert_eq!(deps[0].current_value, "v3.1.1");
        assert_eq!(deps[1].current_value, "v3.1.1"); // single-quoted, tag= stripped
        assert_eq!(deps[2].current_value, "v2.5.1"); // double-quoted
        assert_eq!(deps[3].current_value, "v2"); // comment stripped, not used as version
    }

    // Ported: "extracts multiple action tag lines with double quotes and comments" — github-actions/extract.spec.ts line 153
    #[test]
    fn quoted_action_is_parsed() {
        let content = r#"      - uses: "actions/checkout@v4""#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].action, "actions/checkout");
        assert_eq!(deps[0].current_value, "v4");
    }

    // Ported: "maintains spaces between hash and comment" — github-actions/extract.spec.ts line 299
    #[test]
    fn inline_comment_stripped() {
        let content = "      - uses: actions/checkout@v4 # pinned\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "v4");
    }

    // Ported: "extracts multiple action tag lines from yaml configuration file" — github-actions/extract.spec.ts line 65
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

    // Ported: "returns null for empty" — github-actions/extract.spec.ts line 42
    #[test]
    fn empty_content_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "returns null for invalid yaml" — github-actions/extract.spec.ts line 48
    #[test]
    fn invalid_yaml_returns_empty() {
        // Our line-scanner doesn't parse YAML — malformed YAML with no `uses:` lines is empty.
        assert!(extract("nothing here: [").is_empty());
    }

    // Ported: "extracts multiple action tag lines from yaml configuration file" — github-actions/extract.spec.ts line 65
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

    // Ported: "extracts multiple docker image lines from yaml configuration file" — github-actions/extract.spec.ts line 54
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

    // Ported: "extracts multiple docker image lines from yaml configuration file" — github-actions/extract.spec.ts line 54
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

    // Ported: "extracts multiple docker image lines from yaml configuration file" — github-actions/extract.spec.ts line 54
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

    // Ported: "extracts multiple docker image lines from yaml configuration file" — github-actions/extract.spec.ts line 54
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

    // Ported: "extracts multiple docker image lines from yaml configuration file" — github-actions/extract.spec.ts line 54
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

    // Ported: "returns null for empty" — github-actions/extract.spec.ts line 42
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

    // Ported: "extracts multiple docker image lines from yaml configuration file" — github-actions/extract.spec.ts line 54
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

    // ── extract_runner_labels tests ───────────────────────────────────────────

    // Ported: "extracts multiple action runners from yaml configuration file" — github-actions/extract.spec.ts line 673
    #[test]
    fn runner_simple_ubuntu() {
        let content = "    runs-on: ubuntu-22.04\n";
        let runners = extract_runner_labels(content);
        assert_eq!(runners.len(), 1);
        assert_eq!(runners[0].runner_name, "ubuntu");
        assert_eq!(runners[0].current_value, "22.04");
    }

    // Ported: "extracts multiple action runners from yaml configuration file" — github-actions/extract.spec.ts line 673
    #[test]
    fn runner_macos_xlarge() {
        let content = "    runs-on: macos-14-xlarge\n";
        let runners = extract_runner_labels(content);
        assert_eq!(runners.len(), 1);
        assert_eq!(runners[0].runner_name, "macos");
        assert_eq!(runners[0].current_value, "14-xlarge");
    }

    // Ported: "extracts multiple action runners from yaml configuration file" — github-actions/extract.spec.ts line 673
    #[test]
    fn runner_windows() {
        let content = "    runs-on: windows-2022\n";
        let runners = extract_runner_labels(content);
        assert_eq!(runners.len(), 1);
        assert_eq!(runners[0].runner_name, "windows");
        assert_eq!(runners[0].current_value, "2022");
    }

    // Ported: "extracts multiple action runners from yaml configuration file" — github-actions/extract.spec.ts line 673
    #[test]
    fn runner_latest_skipped() {
        let content = "    runs-on: ubuntu-latest\n";
        assert!(extract_runner_labels(content).is_empty());
    }

    // Ported: "extracts multiple action runners from yaml configuration file" — github-actions/extract.spec.ts line 673
    #[test]
    fn runner_self_hosted_skipped() {
        let content = "    runs-on: self-hosted\n";
        assert!(extract_runner_labels(content).is_empty());
    }

    // Ported: "extracts multiple action runners from yaml configuration file" — github-actions/extract.spec.ts line 673
    #[test]
    fn runner_matrix_variable_skipped() {
        let content = "    runs-on: ${{ matrix.os }}\n";
        assert!(extract_runner_labels(content).is_empty());
    }

    // Ported: "extracts multiple action runners from yaml configuration file" — github-actions/extract.spec.ts line 673
    #[test]
    fn runner_inline_array() {
        let content = "    runs-on: [ubuntu-22.04, self-hosted]\n";
        let runners = extract_runner_labels(content);
        assert_eq!(runners.len(), 1);
        assert_eq!(runners[0].current_value, "22.04");
    }

    // Ported: "extracts multiple action runners from yaml configuration file" — github-actions/extract.spec.ts line 673
    #[test]
    fn runner_unknown_version_skipped() {
        let content = "    runs-on: ubuntu-99.99\n";
        assert!(extract_runner_labels(content).is_empty());
    }

    // Ported: "extracts multiple action runners from yaml configuration file" — github-actions/extract.spec.ts line 673
    #[test]
    fn parse_runner_label_splits_correctly() {
        assert_eq!(
            parse_runner_label("ubuntu-22.04"),
            Some(("ubuntu", "22.04"))
        );
        assert_eq!(
            parse_runner_label("macos-14-xlarge"),
            Some(("macos", "14-xlarge"))
        );
        assert_eq!(
            parse_runner_label("windows-2022"),
            Some(("windows", "2022"))
        );
        assert_eq!(parse_runner_label("self-hosted"), Some(("self", "hosted")));
        assert_eq!(parse_runner_label("nodash"), None);
    }
}
