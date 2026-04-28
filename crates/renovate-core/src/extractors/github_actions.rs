//! GitHub Actions `uses:` dependency extractor.
//!
//! Scans workflow YAML files line-by-line for `uses:` entries and classifies
//! each reference as a versioned action, a local action, a Docker image, or a
//! SHA-pinned action.
//!
//! Renovate reference:
//! - `lib/modules/manager/github-actions/extract.ts` — `extractPackageFile`
//! - `lib/modules/manager/github-actions/parse.ts`   — `parseUsesLine`,
//!   `isSha`, `isShortSha`, `versionLikeRe`
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

use std::sync::LazyLock;

use regex::Regex;

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
}
