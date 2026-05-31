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
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GithubActionsExtractedDep {
    /// `owner/repo` (without sub-path or version).
    pub action: String,
    /// The tag/ref used (e.g. `"v4"`, `"v4.0.1"`).
    pub current_value: String,
    /// Set when the dep should not be looked up in the registry.
    pub skip_reason: Option<GithubActionsSkipReason>,
    /// Registry URLs to use for this dep (empty = use default github.com).
    pub registry_urls: Vec<String>,
}

/// Context for enriched extraction — endpoint and platform for registry URL detection.
#[derive(Debug, Default)]
pub struct GithubActionsContext {
    /// Platform name (e.g., `"github"`, `"gitlab"`, `"bitbucket"`).
    pub platform: Option<String>,
    /// API endpoint URL (e.g., `"https://github.enterprise.com"`).
    pub endpoint: Option<String>,
}

/// Parsed GitHub Actions `uses:` reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GithubActionReference {
    Docker {
        image: String,
        tag: Option<String>,
        digest: Option<String>,
        original_ref: String,
    },
    Local {
        path: String,
    },
    Repository {
        hostname: String,
        is_explicit_hostname: bool,
        owner: String,
        repo: String,
        path: Option<String>,
        reference: String,
    },
}

/// Parsed trailing GitHub Actions comment metadata.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GithubActionsCommentData {
    pub pinned_version: Option<String>,
    pub reference: Option<String>,
    pub ratchet_exclude: bool,
    pub matched_string: Option<String>,
    pub index: Option<usize>,
}

/// A possibly quoted GitHub Actions scalar value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GithubActionsQuotedValue {
    pub value: String,
    pub quote: Option<char>,
}

/// Parsed GitHub Actions `uses:` line components.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GithubActionsParsedUsesLine {
    pub indentation: String,
    pub uses_prefix: String,
    pub replace_string: String,
    pub comment_preceding_whitespace: String,
    pub comment_string: String,
    pub action_ref: Option<GithubActionReference>,
    pub comment_data: GithubActionsCommentData,
    pub quote: Option<char>,
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

static COMMENT_PIN_TOKEN_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^\s*(?:(?:renovate\s*:\s*)?(?:pin\s+|tag\s*=\s*)?|(?:ratchet:[\w-]+/[.\w-]+))?@?(?<version>([\w-]*[-/])?v?\d+(?:\.\d+(?:\.\d+)?)?(?:-[a-zA-Z0-9.]+)?)",
    )
    .unwrap()
});

static COMMENT_BARE_TOKEN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*(?<token>\S+)\s*$").unwrap());

static PARSE_USES_LINE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?<prefix>\s+(?:-\s+)?uses\s*:\s*)(?<remainder>.+)$").unwrap());

// ── Public API ─────────────────────────────────────────────────────────────

/// Detect registry URLs to apply to all extracted actions, mirroring TypeScript's
/// `detectCustomGitHubRegistryUrlsForActions()`.
///
/// Returns an empty vec when the default github.com registry should be used,
/// or `[enterprise_url, "https://github.com"]` for GitHub Enterprise endpoints.
fn detect_registry_urls(ctx: &GithubActionsContext) -> Vec<String> {
    let Some(endpoint) = ctx.endpoint.as_deref() else {
        return vec![];
    };
    if ctx.platform.as_deref() != Some("github") {
        return vec![];
    }
    // Parse host from endpoint URL: "https://foo.example.com/api/v3" → "foo.example.com"
    let (scheme, host) = match endpoint.split_once("://") {
        Some((s, rest)) if s == "https" || s == "http" => {
            let host = rest
                .split('/')
                .next()
                .unwrap_or("")
                .split('?')
                .next()
                .unwrap_or("");
            if host.is_empty() {
                return vec![]; // invalid URL
            }
            (s, host)
        }
        _ => return vec![], // not a valid URL
    };
    if host == "github.com" || host == "api.github.com" {
        return vec![]; // standard github.com, no custom registry
    }
    vec![
        format!("{scheme}://{host}"),
        "https://github.com".to_owned(),
    ]
}

/// Parse a GitHub Actions workflow YAML file and extract `uses:` references.
pub fn extract(content: &str) -> Vec<GithubActionsExtractedDep> {
    extract_with_context(content, &GithubActionsContext::default())
}

/// Parse a GitHub Actions workflow YAML file with endpoint context for registry URL detection.
pub fn extract_with_context(
    content: &str,
    ctx: &GithubActionsContext,
) -> Vec<GithubActionsExtractedDep> {
    let registry_urls = detect_registry_urls(ctx);
    let mut deps = Vec::new();

    for cap in USES_LINE.captures_iter(content) {
        let remainder = cap[1].trim();
        // Capture the version comment (e.g. "v4" from "# v4") before stripping.
        let version_comment = comment_version(remainder);
        // Strip inline comment (# …) and trailing quotes.
        let raw = strip_comment(remainder);
        let raw = raw.trim_matches(|c| c == '\'' || c == '"');

        if let Some(mut dep) = parse_uses(raw, version_comment) {
            dep.registry_urls = registry_urls.clone();
            deps.push(dep);
        }
    }

    deps
}

/// Parse a single GitHub Actions action reference.
///
/// Renovate reference: `lib/modules/manager/github-actions/parse.ts`
/// `parseActionReference`.
pub fn parse_action_reference(raw: &str) -> Option<GithubActionReference> {
    if raw.is_empty() {
        return None;
    }

    if let Some(docker_ref) = raw.strip_prefix("docker://") {
        return parse_docker_action_reference(docker_ref);
    }

    if raw.starts_with("./") || raw.starts_with("../") {
        return Some(GithubActionReference::Local {
            path: raw.to_owned(),
        });
    }

    parse_repository_action_reference(raw)
}

/// Parse Renovate metadata from a GitHub Actions trailing comment.
///
/// Renovate reference: `lib/modules/manager/github-actions/parse.ts`
/// `parseComment`.
pub fn parse_comment(comment_body: &str) -> GithubActionsCommentData {
    if comment_body.trim() == "ratchet:exclude" {
        return GithubActionsCommentData {
            ratchet_exclude: true,
            ..GithubActionsCommentData::default()
        };
    }

    if let Some(caps) = COMMENT_PIN_TOKEN_RE.captures(comment_body)
        && let (Some(matched), Some(version)) = (caps.get(0), caps.name("version"))
    {
        return GithubActionsCommentData {
            pinned_version: Some(version.as_str().to_owned()),
            matched_string: Some(matched.as_str().to_owned()),
            index: Some(matched.start()),
            ..GithubActionsCommentData::default()
        };
    }

    if let Some(caps) = COMMENT_BARE_TOKEN_RE.captures(comment_body)
        && let (Some(matched), Some(token)) = (caps.get(0), caps.name("token"))
    {
        return GithubActionsCommentData {
            reference: Some(token.as_str().to_owned()),
            matched_string: Some(matched.as_str().to_owned()),
            index: Some(matched.start()),
            ..GithubActionsCommentData::default()
        };
    }

    GithubActionsCommentData::default()
}

/// Parse surrounding quotes from a GitHub Actions scalar value.
///
/// Renovate reference: `lib/modules/manager/github-actions/parse.ts`
/// `parseQuote`.
pub fn parse_quote(input: &str) -> GithubActionsQuotedValue {
    let trimmed = input.trim();
    let mut chars = trimmed.chars();
    let first = chars.next();
    let last = trimmed.chars().next_back();

    if trimmed.len() >= 2 && first == last && matches!(first, Some('"') | Some('\'')) {
        return GithubActionsQuotedValue {
            value: trimmed[1..trimmed.len() - 1].to_owned(),
            quote: first,
        };
    }

    GithubActionsQuotedValue {
        value: trimmed.to_owned(),
        quote: None,
    }
}

/// Parse a single GitHub Actions `uses:` line into replaceable components.
///
/// Renovate reference: `lib/modules/manager/github-actions/parse.ts`
/// `parseUsesLine`.
pub fn parse_uses_line(line: &str) -> Option<GithubActionsParsedUsesLine> {
    let caps = PARSE_USES_LINE_RE.captures(line)?;
    let prefix = caps.name("prefix")?.as_str();
    let remainder = caps.name("remainder")?.as_str();

    if remainder.starts_with('#') {
        return None;
    }

    let indentation = prefix
        .find("uses")
        .map(|idx| &prefix[..idx])
        .unwrap_or_default();

    if let Some(comment_index) = remainder.find(" #") {
        let raw_value_part = &remainder[..comment_index];
        let comment_part = &remainder[comment_index + 1..];
        let part_before_hash = &remainder[..comment_index + 1];
        let trimmed_len = part_before_hash.trim_end().len();
        let comment_preceding_whitespace = &part_before_hash[trimmed_len..];
        let quoted = parse_quote(raw_value_part);
        let clean_comment_body = comment_part.strip_prefix('#').unwrap_or(comment_part);

        return Some(GithubActionsParsedUsesLine {
            indentation: indentation.to_owned(),
            uses_prefix: prefix.to_owned(),
            replace_string: raw_value_part.trim().to_owned(),
            comment_preceding_whitespace: comment_preceding_whitespace.to_owned(),
            comment_string: comment_part.to_owned(),
            action_ref: parse_action_reference(&quoted.value),
            comment_data: parse_comment(clean_comment_body),
            quote: quoted.quote,
        });
    }

    let quoted = parse_quote(remainder);
    Some(GithubActionsParsedUsesLine {
        indentation: indentation.to_owned(),
        uses_prefix: prefix.to_owned(),
        replace_string: remainder.trim().to_owned(),
        comment_preceding_whitespace: String::new(),
        comment_string: String::new(),
        action_ref: parse_action_reference(&quoted.value),
        comment_data: GithubActionsCommentData::default(),
        quote: quoted.quote,
    })
}

/// Update a single GitHub Actions dependency in a workflow YAML file.
///
/// Mirrors `updateDependency()` from `lib/modules/manager/github-actions/update.ts`.
///
/// Returns `Some(updated_content)` when the replacement was made, or `None`
/// when the dep could not be found or matched.
pub fn github_actions_update_dependency(
    file_content: &str,
    dep_name: &str,
    current_value: &str,
    new_value: &str,
) -> Option<String> {
    let mut content = file_content.to_owned();
    let mut found = false;

    for line in file_content.lines() {
        let Some(parsed) = parse_uses_line(line) else {
            continue;
        };
        let action_ref = parsed.action_ref?;
        let repo_ref = match action_ref {
            GithubActionReference::Repository { owner, repo, .. } => {
                format!("{owner}/{repo}")
            }
            _ => continue,
        };
        if repo_ref != dep_name {
            continue;
        }
        if !parsed.replace_string.contains(current_value) {
            continue;
        }
        let new_replace = parsed.replace_string.replacen(current_value, new_value, 1);
        let new_line = line.replacen(&parsed.replace_string, &new_replace, 1);
        content = content.replacen(line, &new_line, 1);
        found = true;
    }

    if found {
        Some(content)
    } else {
        None
    }
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
            registry_urls: vec![],
        });
    }

    // Docker container action.
    if raw.starts_with("docker://") {
        return Some(GithubActionsExtractedDep {
            action: raw.to_owned(),
            current_value: String::new(),
            skip_reason: Some(GithubActionsSkipReason::DockerRef),
            registry_urls: vec![],
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
                registry_urls: vec![],
            });
        }
        return Some(GithubActionsExtractedDep {
            action,
            current_value: version.to_owned(),
            skip_reason: Some(GithubActionsSkipReason::ShaPin),
            registry_urls: vec![],
        });
    }
    if SHA_SHORT.is_match(version) {
        if let Some(vc) = version_comment {
            return Some(GithubActionsExtractedDep {
                action,
                current_value: vc.to_owned(),
                skip_reason: None,
                registry_urls: vec![],
            });
        }
        return Some(GithubActionsExtractedDep {
            action,
            current_value: version.to_owned(),
            skip_reason: Some(GithubActionsSkipReason::ShortShaPin),
            registry_urls: vec![],
        });
    }

    Some(GithubActionsExtractedDep {
        action,
        current_value: version.to_owned(),
        skip_reason: None,
        registry_urls: vec![],
    })
}

fn parse_docker_action_reference(raw: &str) -> Option<GithubActionReference> {
    if raw.is_empty() {
        return None;
    }

    if let Some((image, digest)) = raw.split_once('@') {
        if image.is_empty() || digest.is_empty() {
            return None;
        }
        return Some(GithubActionReference::Docker {
            image: image.to_owned(),
            tag: None,
            digest: Some(digest.to_owned()),
            original_ref: raw.to_owned(),
        });
    }

    let last_slash = raw.rfind('/');
    let tag_sep = raw
        .rfind(':')
        .filter(|colon| last_slash.is_none_or(|slash| *colon > slash));

    let (image, tag) = match tag_sep {
        Some(idx) => (&raw[..idx], Some(raw[idx + 1..].to_owned())),
        None => (raw, None),
    };
    if image.is_empty() || tag.as_deref() == Some("") {
        return None;
    }

    Some(GithubActionReference::Docker {
        image: image.to_owned(),
        tag,
        digest: None,
        original_ref: raw.to_owned(),
    })
}

fn parse_repository_action_reference(raw: &str) -> Option<GithubActionReference> {
    let (action_path, reference) = raw.rsplit_once('@')?;
    if action_path.is_empty() || reference.is_empty() {
        return None;
    }

    let (hostname, is_explicit_hostname, path_without_host) =
        if let Some(without_scheme) = action_path.strip_prefix("https://") {
            let (hostname, rest) = without_scheme.split_once('/')?;
            (hostname, true, rest)
        } else {
            ("github.com", false, action_path)
        };

    let mut parts = path_without_host.splitn(3, '/');
    let owner = parts.next()?.trim();
    let repo = parts.next()?.trim();
    let path = parts
        .next()
        .filter(|path| !path.is_empty())
        .map(str::to_owned);
    if hostname.is_empty() || owner.is_empty() || repo.is_empty() {
        return None;
    }

    Some(GithubActionReference::Repository {
        hostname: hostname.to_owned(),
        is_explicit_hostname,
        owner: owner.to_owned(),
        repo: repo.to_owned(),
        path,
        reference: reference.to_owned(),
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

/// A GitHub Actions FQDN dependency from `uses: https://host/owner/repo[/path]@ref`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GithubActionsFqdnDep {
    /// Full URL dep_name: `https://host/owner/repo` (without sub-path).
    pub dep_name: String,
    /// Owner/repo portion: `owner/repo`.
    pub package_name: String,
    /// The ref after `@` (tag, branch, or digest).
    pub current_value: Option<String>,
    /// SHA digest when pinned.
    pub current_digest: Option<String>,
    /// Registry URL: `https://host/`.
    pub registry_url: Option<String>,
    /// Datasource identifier (e.g. `"github-tags"`, `"forgejo-tags"`, `"gitea-tags"`).
    pub datasource: Option<&'static str>,
    /// Skip reason when host is not supported.
    pub skip_reason: Option<&'static str>,
    /// Original `uses:` value verbatim (after quote stripping), used by writers
    /// to do literal in-place substitution.
    pub replace_string: String,
}

/// Detect the git platform based on the hostname.
///
/// Returns `"github"`, `"forgejo"`, `"gitea"`, or `None` (unsupported).
fn detect_host_platform(hostname: &str) -> Option<&'static str> {
    if hostname == "github.com" || hostname.contains("github") {
        return Some("github");
    }
    if hostname.contains("forgejo") || hostname == "codeberg.org" || hostname == "codefloe.com" {
        return Some("forgejo");
    }
    if hostname == "gitea.com" || hostname.contains("gitea") {
        return Some("gitea");
    }
    None
}

/// Extract FQDN-style `uses: https://host/...` dependencies from a workflow.
///
/// These are full URL action references like:
/// - `https://github.com/actions/cache/save@sha`
/// - `https://code.forgejo.org/actions/setup-node@sha`
/// - `https://gitea.com/actions/setup-node@sha`
pub fn extract_fqdn(content: &str) -> Vec<GithubActionsFqdnDep> {
    let mut out = Vec::new();

    for raw in content.lines() {
        if raw.trim().starts_with('#') {
            continue;
        }
        // Look for `uses: https://...` patterns.
        let trimmed = raw.trim();
        let uses_val = if let Some(v) = trimmed.strip_prefix("uses:") {
            v.trim()
        } else if let Some(v) = trimmed.strip_prefix("- uses:") {
            v.trim()
        } else {
            continue;
        };

        // Strip quotes.
        let uses_val = uses_val.trim_matches('"').trim_matches('\'');

        // Must start with https://.
        if !uses_val.starts_with("https://") {
            continue;
        }

        // Preserve the full `uses:` value as the replaceString.
        let replace_string = uses_val.to_owned();

        // Extract inline comment version (e.g. `# tag=v4.2.0` or `# v3.1.1`).
        let (url_part, comment_version) = if let Some(idx) = uses_val.find(" #") {
            let v = uses_val[idx + 2..].trim();
            // Strip `tag=` prefix.
            let v = v.strip_prefix("tag=").unwrap_or(v);
            (&uses_val[..idx], Some(v.to_owned()))
        } else {
            (uses_val, None)
        };

        // Split into url@ref.
        let Some((url_path, ref_str)) = url_part.split_once('@') else {
            continue;
        };

        // Parse the URL path: `https://host/owner/repo[/subpath]`
        let Some(without_scheme) = url_path.strip_prefix("https://") else {
            continue;
        };
        let mut segs = without_scheme.splitn(4, '/');
        let hostname = segs.next().unwrap_or("").trim();
        let owner = segs.next().unwrap_or("").trim();
        let repo = segs.next().unwrap_or("").trim();

        if hostname.is_empty() || owner.is_empty() || repo.is_empty() {
            continue;
        }

        let package_name = format!("{owner}/{repo}");
        let dep_name = format!("https://{hostname}/{package_name}");
        let registry_url_str = format!("https://{hostname}/");

        let platform = detect_host_platform(hostname);

        // Determine if ref is a full SHA.
        let (current_digest, current_value) = if SHA_FULL.is_match(ref_str) {
            (Some(ref_str.to_owned()), comment_version)
        } else {
            (
                None,
                Some(comment_version.unwrap_or_else(|| ref_str.to_owned())),
            )
        };

        let dep = match platform {
            Some("github") => GithubActionsFqdnDep {
                dep_name,
                package_name,
                current_value,
                current_digest,
                registry_url: Some(registry_url_str),
                datasource: Some("github-tags"),
                skip_reason: None,
                replace_string,
            },
            Some("forgejo") => GithubActionsFqdnDep {
                dep_name,
                package_name,
                current_value,
                current_digest,
                registry_url: Some(registry_url_str),
                datasource: Some("forgejo-tags"),
                skip_reason: None,
                replace_string,
            },
            Some("gitea") => GithubActionsFqdnDep {
                dep_name,
                package_name,
                current_value,
                current_digest,
                registry_url: Some(registry_url_str),
                datasource: Some("gitea-tags"),
                skip_reason: None,
                replace_string,
            },
            _ => GithubActionsFqdnDep {
                dep_name,
                package_name,
                current_value: None,
                current_digest: None,
                registry_url: None,
                datasource: None,
                skip_reason: Some("unsupported-url"),
                replace_string,
            },
        };

        out.push(dep);
    }

    out
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

// ── Uses-with extraction (setup-x and community actions) ─────────────────────

/// A dependency extracted from the `with:` block of a GitHub Actions step.
///
/// Covers two classes of step:
/// - Official `actions/setup-{go,node,python}` with `{lang}-version:` fields.
/// - Community-contributed setup/install actions with a known version-input schema.
///
/// Renovate reference:
/// - `lib/modules/manager/github-actions/extract.ts` — `extractVersionedAction`,
///   `extractSteps`, `extractWithYAMLParser`
/// - `lib/modules/manager/github-actions/community.ts` — `communityActions`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsesWithDep {
    pub dep_name: String,
    pub package_name: String,
    pub current_value: Option<String>,
    pub datasource: &'static str,
    pub versioning: Option<&'static str>,
    pub extract_version: Option<&'static str>,
    /// `"unspecified-version"` or `"invalid-version"` or `None`.
    pub skip_reason: Option<&'static str>,
    /// `"extract"` when the dep should not be looked up.
    pub skip_stage: Option<&'static str>,
}

/// Internal config for a community action's `with:` field schema.
struct CommActCfg {
    /// `owner/repo` name (no FQDN prefix, no `@ref`).
    action: &'static str,
    /// Override `depName`; if `None`, falls back to `package_name`.
    dep_name: Option<&'static str>,
    package_name: &'static str,
    datasource: &'static str,
    versioning: Option<&'static str>,
    extract_version: Option<&'static str>,
    /// Key inside `with:` that holds the version string (typically `"version"`).
    version_key: &'static str,
    /// When `true` the version comes from `tag:` and the package name from `repo:`.
    /// Used by `jaxxstorm/action-install-gh-release` and `sigoden/install-binary`.
    use_repo_tag: bool,
    /// Optional predicate that returns `true` when a value string is invalid.
    is_invalid: Option<fn(&str) -> bool>,
}

/// Official `actions/setup-X` descriptors.
/// Fields: (action_prefix, version_key, dep_name, package_name, versioning)
const SETUP_X: &[(&str, &str, &str, &str, &str)] = &[
    (
        "actions/setup-go",
        "go-version",
        "go",
        "actions/go-versions",
        "npm",
    ),
    (
        "actions/setup-node",
        "node-version",
        "node",
        "actions/node-versions",
        "node",
    ),
    (
        "actions/setup-python",
        "python-version",
        "python",
        "actions/python-versions",
        "npm",
    ),
];

const SETUP_X_EXTRACT_VERSION: &str = r"^(?<version>\d+\.\d+\.\d+)(-\d+)?$";

const COMMUNITY_ACTIONS: &[CommActCfg] = &[
    // https://github.com/aquasecurity/setup-trivy
    CommActCfg {
        action: "aquasecurity/setup-trivy",
        dep_name: None,
        package_name: "aquasecurity/trivy",
        datasource: "github-releases",
        versioning: None,
        extract_version: None,
        version_key: "version",
        use_repo_tag: false,
        is_invalid: None,
    },
    // https://github.com/aquasecurity/trivy-action
    CommActCfg {
        action: "aquasecurity/trivy-action",
        dep_name: None,
        package_name: "aquasecurity/trivy",
        datasource: "github-releases",
        versioning: None,
        extract_version: None,
        version_key: "version",
        use_repo_tag: false,
        is_invalid: None,
    },
    // https://github.com/astral-sh/setup-uv
    CommActCfg {
        action: "astral-sh/setup-uv",
        dep_name: None,
        package_name: "astral-sh/uv",
        datasource: "github-releases",
        versioning: Some("npm"),
        extract_version: None,
        version_key: "version",
        use_repo_tag: false,
        is_invalid: None,
    },
    CommActCfg {
        action: "denoland/setup-deno",
        dep_name: None,
        package_name: "deno",
        datasource: "npm",
        versioning: None,
        extract_version: None,
        version_key: "deno-version",
        use_repo_tag: false,
        is_invalid: None,
    },
    // https://github.com/docker/setup-docker-action
    CommActCfg {
        action: "docker/setup-docker-action",
        dep_name: Some("docker/setup-docker-action"),
        package_name: "moby/moby",
        datasource: "github-releases",
        versioning: None,
        extract_version: Some("^docker-(?<version>.+)$"),
        version_key: "version",
        use_repo_tag: false,
        is_invalid: None,
    },
    CommActCfg {
        action: "golangci/golangci-lint-action",
        dep_name: None,
        package_name: "golangci/golangci-lint",
        datasource: "github-releases",
        versioning: None,
        extract_version: None,
        version_key: "version",
        use_repo_tag: false,
        is_invalid: None,
    },
    CommActCfg {
        action: "jakebailey/pyright-action",
        dep_name: None,
        package_name: "pyright",
        datasource: "npm",
        versioning: None,
        extract_version: None,
        version_key: "version",
        use_repo_tag: false,
        is_invalid: Some(|v| v == "PATH"),
    },
    // https://github.com/jaxxstorm/action-install-gh-release
    CommActCfg {
        action: "jaxxstorm/action-install-gh-release",
        dep_name: None,
        package_name: "",
        datasource: "github-releases",
        versioning: None,
        extract_version: None,
        version_key: "tag",
        use_repo_tag: true,
        is_invalid: None,
    },
    CommActCfg {
        action: "oven-sh/setup-bun",
        dep_name: None,
        package_name: "bun",
        datasource: "npm",
        versioning: None,
        extract_version: None,
        version_key: "bun-version",
        use_repo_tag: false,
        is_invalid: None,
    },
    CommActCfg {
        action: "pdm-project/setup-pdm",
        dep_name: None,
        package_name: "pdm",
        datasource: "pypi",
        versioning: None,
        extract_version: None,
        version_key: "version",
        use_repo_tag: false,
        is_invalid: None,
    },
    CommActCfg {
        action: "pnpm/action-setup",
        dep_name: None,
        package_name: "pnpm",
        datasource: "npm",
        versioning: None,
        extract_version: None,
        version_key: "version",
        use_repo_tag: false,
        is_invalid: None,
    },
    // https://github.com/prefix-dev/setup-pixi
    CommActCfg {
        action: "prefix-dev/setup-pixi",
        dep_name: None,
        package_name: "prefix-dev/pixi",
        datasource: "github-releases",
        versioning: Some("conda"),
        extract_version: None,
        version_key: "pixi-version",
        use_repo_tag: false,
        is_invalid: None,
    },
    // https://github.com/pypa/hatch
    CommActCfg {
        action: "pypa/hatch",
        dep_name: None,
        package_name: "pypa/hatch",
        datasource: "github-releases",
        versioning: None,
        extract_version: Some("^hatch-(?<version>.+)$"),
        version_key: "version",
        use_repo_tag: false,
        is_invalid: None,
    },
    CommActCfg {
        action: "ruby/setup-ruby",
        dep_name: None,
        package_name: "ruby",
        datasource: "ruby-version",
        versioning: None,
        extract_version: None,
        version_key: "ruby-version",
        use_repo_tag: false,
        is_invalid: None,
    },
    CommActCfg {
        action: "sigoden/install-binary",
        dep_name: None,
        package_name: "",
        datasource: "github-releases",
        versioning: None,
        extract_version: None,
        version_key: "tag",
        use_repo_tag: true,
        is_invalid: None,
    },
    CommActCfg {
        action: "zizmorcore/zizmor-action",
        dep_name: None,
        package_name: "ghcr.io/zizmorcore/zizmor",
        datasource: "docker",
        versioning: None,
        extract_version: None,
        version_key: "version",
        use_repo_tag: false,
        is_invalid: None,
    },
];

#[derive(Clone)]
enum ActionMatch {
    Official(usize),
    Community(usize),
}

/// Scan a workflow or action YAML for `uses:` lines that match known setup or
/// community actions, then look ahead for the accompanying `with:` block to
/// extract the version field.
///
/// Returns one `UsesWithDep` per matched step.  Steps without a recognisable
/// `uses:` are ignored; steps whose `with:` block lacks the expected version
/// key emit an `unspecified-version` dep.
pub fn extract_uses_with(content: &str) -> Vec<UsesWithDep> {
    enum St {
        Scanning,
        /// Found a matching `uses:` line; waiting for `with:` or end of step.
        MatchedUses {
            m: ActionMatch,
            /// Raw indent of the line that started the step (`- uses:` or `uses:`).
            step_raw: usize,
            /// Indent of the key character (step_raw + 2 for seq items).
            key_ind: usize,
        },
        /// Inside a `with:` block; collecting version / repo / tag values.
        InWith {
            m: ActionMatch,
            step_raw: usize,
            with_key_ind: usize,
            repo: Option<String>,
            tag: Option<String>,
            version: Option<String>,
        },
    }

    let mut out = Vec::new();
    let mut st = St::Scanning;
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let raw = lines[i];
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            i += 1;
            continue;
        }
        let raw_ind = raw.len() - raw.trim_start().len();

        // Helper: a line exits the current step when its raw indent is at or
        // below the step-start indent (for sequence items), or strictly below
        // the key indent (for plain mappings).
        let exits_step = |step_raw: usize, key_ind: usize| -> bool {
            if raw_ind < key_ind {
                // Line is shallower than the step's keys → left the step.
                return true;
            }
            if raw_ind == step_raw && (trimmed.starts_with("- ") || trimmed == "-") {
                // New sequence item at the same level → new step.
                return true;
            }
            false
        };

        match st {
            St::Scanning => {
                if let Some((m, key_ind)) = match_uses_line(trimmed, raw_ind) {
                    st = St::MatchedUses {
                        m,
                        step_raw: raw_ind,
                        key_ind,
                    };
                }
                i += 1;
            }
            St::MatchedUses {
                ref m,
                step_raw,
                key_ind,
            } => {
                if exits_step(step_raw, key_ind) {
                    // Step ended without a `with:` block.
                    out.push(build_unspecified(m));
                    st = St::Scanning;
                    // Re-process current line.
                    continue;
                }
                // Look for `with:` at the same key indent.
                if key_indent_of(raw) == key_ind
                    && let Some(with_val) = strip_key(trimmed, "with")
                {
                    let with_val = with_val.trim();
                    if with_val.is_empty() {
                        // Block form: value follows on subsequent lines.
                        st = St::InWith {
                            m: m.clone(),
                            step_raw,
                            with_key_ind: key_ind,
                            repo: None,
                            tag: None,
                            version: None,
                        };
                    } else {
                        // Inline value (`with: {}`, `with: null`, etc.) — no version key.
                        out.push(build_unspecified(m));
                        st = St::Scanning;
                    }
                }
                i += 1;
            }
            St::InWith {
                ref m,
                step_raw,
                with_key_ind,
                ref mut repo,
                ref mut tag,
                ref mut version,
            } => {
                // Exit condition: raw_ind has returned to step level.
                if raw_ind <= with_key_ind
                    || (raw_ind == step_raw && (trimmed.starts_with("- ") || trimmed == "-"))
                {
                    out.push(build_dep(m, version.take(), repo.take(), tag.take()));
                    st = St::Scanning;
                    continue;
                }

                let ver_key = version_key_of(m);
                if let Some(val) = strip_key(trimmed, ver_key) {
                    *version = Some(unquote(val.trim()));
                }
                // Collect `repo:` and `tag:` for use_repo_tag actions.
                if is_repo_tag(m) {
                    if let Some(val) = strip_key(trimmed, "repo") {
                        *repo = Some(unquote(val.trim()));
                    }
                    if let Some(val) = strip_key(trimmed, "tag") {
                        *tag = Some(unquote(val.trim()));
                    }
                }
                i += 1;
            }
        }
    }

    // Flush any pending state at end of input.
    match st {
        St::MatchedUses { ref m, .. } => out.push(build_unspecified(m)),
        St::InWith {
            ref m,
            version,
            repo,
            tag,
            ..
        } => out.push(build_dep(m, version, repo, tag)),
        St::Scanning => {}
    }

    out
}

/// Return `(ActionMatch, key_indent)` if `trimmed` is a matching `uses:` line.
fn match_uses_line(trimmed: &str, raw_ind: usize) -> Option<(ActionMatch, usize)> {
    let (uses_raw, key_ind) = if let Some(rest) = trimmed.strip_prefix("uses:") {
        (rest.trim(), raw_ind)
    } else if let Some(rest) = trimmed.strip_prefix("- uses:") {
        (rest.trim(), raw_ind + 2)
    } else {
        return None;
    };

    // Strip quotes then inline comment.
    let uses_val = uses_raw
        .trim_matches('\'')
        .trim_matches('"')
        .split(" #")
        .next()
        .unwrap_or(uses_raw)
        .trim()
        .trim_matches('\'')
        .trim_matches('"');

    // Normalise FQDN prefix: `https://github.com/owner/repo@ref` → `owner/repo@ref`
    let path = if let Some(rest) = uses_val.strip_prefix("https://") {
        // Drop the hostname segment.
        rest.split_once('/')?.1
    } else {
        uses_val
    };

    // Strip `@ref` suffix.
    let without_ref = path.split('@').next().unwrap_or(path);

    // Extract `owner/repo` (drop any sub-path).
    let owner_repo = {
        let mut parts = without_ref.splitn(3, '/');
        let owner = parts.next()?;
        let repo = parts.next()?;
        if owner.is_empty() || repo.is_empty() {
            return None;
        }
        format!("{owner}/{repo}")
    };

    for (idx, &(action, ..)) in SETUP_X.iter().enumerate() {
        if owner_repo == action {
            return Some((ActionMatch::Official(idx), key_ind));
        }
    }
    for (idx, cfg) in COMMUNITY_ACTIONS.iter().enumerate() {
        if owner_repo == cfg.action {
            return Some((ActionMatch::Community(idx), key_ind));
        }
    }

    None
}

/// Return the indent of the first key character on `raw` (after stripping `- `).
fn key_indent_of(raw: &str) -> usize {
    let trimmed = raw.trim_start();
    let leading = raw.len() - trimmed.len();
    if trimmed.starts_with("- ") || trimmed == "-" {
        leading + 2
    } else {
        leading
    }
}

/// Remove surrounding single or double quotes from a YAML scalar.
fn unquote(s: &str) -> String {
    s.trim_matches('\'').trim_matches('"').to_owned()
}

fn version_key_of(m: &ActionMatch) -> &'static str {
    match m {
        ActionMatch::Official(idx) => SETUP_X[*idx].1,
        ActionMatch::Community(idx) => COMMUNITY_ACTIONS[*idx].version_key,
    }
}

fn is_repo_tag(m: &ActionMatch) -> bool {
    match m {
        ActionMatch::Official(_) => false,
        ActionMatch::Community(idx) => COMMUNITY_ACTIONS[*idx].use_repo_tag,
    }
}

fn build_unspecified(m: &ActionMatch) -> UsesWithDep {
    build_dep(m, None, None, None)
}

fn build_dep(
    m: &ActionMatch,
    version: Option<String>,
    repo: Option<String>,
    tag: Option<String>,
) -> UsesWithDep {
    match m {
        ActionMatch::Official(idx) => {
            let (_, _, dep_name, pkg, versioning) = SETUP_X[*idx];
            let (skip_reason, skip_stage) = if version.is_none() {
                (Some("unspecified-version"), Some("extract"))
            } else {
                (None, None)
            };
            UsesWithDep {
                dep_name: dep_name.to_owned(),
                package_name: pkg.to_owned(),
                current_value: version,
                datasource: "github-releases",
                versioning: Some(versioning),
                extract_version: Some(SETUP_X_EXTRACT_VERSION),
                skip_reason,
                skip_stage,
            }
        }
        ActionMatch::Community(idx) => {
            let cfg = &COMMUNITY_ACTIONS[*idx];
            build_community_dep(cfg, version, repo, tag)
        }
    }
}

fn build_community_dep(
    cfg: &CommActCfg,
    version: Option<String>,
    repo: Option<String>,
    tag: Option<String>,
) -> UsesWithDep {
    if cfg.use_repo_tag {
        // Package name comes from `repo:`, version from `tag:`.
        let pkg = repo.unwrap_or_default();
        let dep_name = cfg
            .dep_name
            .map(str::to_owned)
            .unwrap_or_else(|| pkg.clone());
        let (current_value, skip_reason, skip_stage) = if tag.is_some() {
            (tag, None, None)
        } else {
            (None, Some("unspecified-version"), Some("extract"))
        };
        return UsesWithDep {
            dep_name,
            package_name: pkg,
            current_value,
            datasource: cfg.datasource,
            versioning: cfg.versioning,
            extract_version: cfg.extract_version,
            skip_reason,
            skip_stage,
        };
    }

    let pkg = cfg.package_name.to_owned();
    let dep_name = cfg
        .dep_name
        .map(str::to_owned)
        .unwrap_or_else(|| pkg.clone());

    let (current_value, skip_reason, skip_stage) = match version {
        None => (None, Some("unspecified-version"), Some("extract")),
        Some(v) => {
            if cfg.is_invalid.is_some_and(|f| f(&v)) {
                (Some(v), Some("invalid-version"), Some("extract"))
            } else {
                (Some(v), None, None)
            }
        }
    };

    UsesWithDep {
        dep_name,
        package_name: pkg,
        current_value,
        datasource: cfg.datasource,
        versioning: cfg.versioning,
        extract_version: cfg.extract_version,
        skip_reason,
        skip_stage,
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
            registry_urls: vec![],
        }
    }

    fn comment_data(
        pinned_version: Option<&str>,
        reference: Option<&str>,
        ratchet_exclude: bool,
        matched_string: Option<&str>,
        index: Option<usize>,
    ) -> GithubActionsCommentData {
        GithubActionsCommentData {
            pinned_version: pinned_version.map(str::to_owned),
            reference: reference.map(str::to_owned),
            ratchet_exclude,
            matched_string: matched_string.map(str::to_owned),
            index,
        }
    }

    fn quoted_value(value: &str, quote: Option<char>) -> GithubActionsQuotedValue {
        GithubActionsQuotedValue {
            value: value.to_owned(),
            quote,
        }
    }

    fn repo_ref(owner: &str, repo: &str, reference: &str) -> GithubActionReference {
        GithubActionReference::Repository {
            hostname: "github.com".to_owned(),
            is_explicit_hostname: false,
            owner: owner.to_owned(),
            repo: repo.to_owned(),
            path: None,
            reference: reference.to_owned(),
        }
    }

    struct UsesLineExpected<'a> {
        indentation: &'a str,
        uses_prefix: &'a str,
        replace_string: &'a str,
        comment_preceding_whitespace: &'a str,
        comment_string: &'a str,
        action_ref: Option<GithubActionReference>,
        comment_data: GithubActionsCommentData,
        quote: Option<char>,
    }

    fn parsed_uses_line(expected: UsesLineExpected<'_>) -> GithubActionsParsedUsesLine {
        GithubActionsParsedUsesLine {
            indentation: expected.indentation.to_owned(),
            uses_prefix: expected.uses_prefix.to_owned(),
            replace_string: expected.replace_string.to_owned(),
            comment_preceding_whitespace: expected.comment_preceding_whitespace.to_owned(),
            comment_string: expected.comment_string.to_owned(),
            action_ref: expected.action_ref,
            comment_data: expected.comment_data,
            quote: expected.quote,
        }
    }

    // Ported: "returns null for empty string" — github-actions/parse.spec.ts line 11
    #[test]
    fn parse_action_reference_returns_none_for_empty_string() {
        assert!(parse_action_reference("").is_none());
    }

    // Ported: "returns null for empty docker reference" — github-actions/parse.spec.ts line 16
    #[test]
    fn parse_action_reference_returns_none_for_empty_docker_reference() {
        assert!(parse_action_reference("docker://").is_none());
    }

    // Ported: "parses docker image with digest" — github-actions/parse.spec.ts line 20
    #[test]
    fn parse_action_reference_parses_docker_image_with_digest() {
        assert_eq!(
            parse_action_reference("docker://alpine@sha256:abc123"),
            Some(GithubActionReference::Docker {
                image: "alpine".to_owned(),
                tag: None,
                digest: Some("sha256:abc123".to_owned()),
                original_ref: "alpine@sha256:abc123".to_owned(),
            })
        );
    }

    // Ported: "parses docker image with tag" — github-actions/parse.spec.ts line 29
    #[test]
    fn parse_action_reference_parses_docker_image_with_tag() {
        assert_eq!(
            parse_action_reference("docker://alpine:3.18"),
            Some(GithubActionReference::Docker {
                image: "alpine".to_owned(),
                tag: Some("3.18".to_owned()),
                digest: None,
                original_ref: "alpine:3.18".to_owned(),
            })
        );
    }

    // Ported: "parses docker image with registry port and tag" — github-actions/parse.spec.ts line 38
    #[test]
    fn parse_action_reference_parses_docker_image_with_registry_port_and_tag() {
        assert_eq!(
            parse_action_reference("docker://registry.example.com:5000/alpine:3.18"),
            Some(GithubActionReference::Docker {
                image: "registry.example.com:5000/alpine".to_owned(),
                tag: Some("3.18".to_owned()),
                digest: None,
                original_ref: "registry.example.com:5000/alpine:3.18".to_owned(),
            })
        );
    }

    // Ported: "parses docker image without tag or digest" — github-actions/parse.spec.ts line 51
    #[test]
    fn parse_action_reference_parses_docker_image_without_tag_or_digest() {
        assert_eq!(
            parse_action_reference("docker://alpine"),
            Some(GithubActionReference::Docker {
                image: "alpine".to_owned(),
                tag: None,
                digest: None,
                original_ref: "alpine".to_owned(),
            })
        );
    }

    // Ported: "parses docker image with registry but no tag" — github-actions/parse.spec.ts line 59
    #[test]
    fn parse_action_reference_parses_docker_image_with_registry_but_no_tag() {
        assert_eq!(
            parse_action_reference("docker://ghcr.io/owner/image"),
            Some(GithubActionReference::Docker {
                image: "ghcr.io/owner/image".to_owned(),
                tag: None,
                digest: None,
                original_ref: "ghcr.io/owner/image".to_owned(),
            })
        );
    }

    // Ported: "parses ./ local reference" — github-actions/parse.spec.ts line 69
    #[test]
    fn parse_action_reference_parses_dot_slash_local_reference() {
        assert_eq!(
            parse_action_reference("./path/to/action"),
            Some(GithubActionReference::Local {
                path: "./path/to/action".to_owned(),
            })
        );
    }

    // Ported: "parses ../ local reference" — github-actions/parse.spec.ts line 76
    #[test]
    fn parse_action_reference_parses_dot_dot_slash_local_reference() {
        assert_eq!(
            parse_action_reference("../other/action"),
            Some(GithubActionReference::Local {
                path: "../other/action".to_owned(),
            })
        );
    }

    // Ported: "returns null for invalid format" — github-actions/parse.spec.ts line 85
    #[test]
    fn parse_action_reference_returns_none_for_invalid_repository_format() {
        assert!(parse_action_reference("invalid").is_none());
        assert!(parse_action_reference("owner/repo").is_none());
    }

    // Ported: "parses owner/repo@ref with default hostname" — github-actions/parse.spec.ts line 90
    #[test]
    fn parse_action_reference_parses_owner_repo_ref_with_default_hostname() {
        assert_eq!(
            parse_action_reference("actions/checkout@v4"),
            Some(GithubActionReference::Repository {
                hostname: "github.com".to_owned(),
                is_explicit_hostname: false,
                owner: "actions".to_owned(),
                repo: "checkout".to_owned(),
                path: None,
                reference: "v4".to_owned(),
            })
        );
    }

    // Ported: "parses owner/repo/path@ref" — github-actions/parse.spec.ts line 102
    #[test]
    fn parse_action_reference_parses_owner_repo_path_ref() {
        assert_eq!(
            parse_action_reference("owner/repo/sub/path@main"),
            Some(GithubActionReference::Repository {
                hostname: "github.com".to_owned(),
                is_explicit_hostname: false,
                owner: "owner".to_owned(),
                repo: "repo".to_owned(),
                path: Some("sub/path".to_owned()),
                reference: "main".to_owned(),
            })
        );
    }

    // Ported: "parses https://host/owner/repo@ref with explicit hostname" — github-actions/parse.spec.ts line 114
    #[test]
    fn parse_action_reference_parses_https_owner_repo_ref_with_explicit_hostname() {
        assert_eq!(
            parse_action_reference("https://gitea.example.com/owner/repo@v1"),
            Some(GithubActionReference::Repository {
                hostname: "gitea.example.com".to_owned(),
                is_explicit_hostname: true,
                owner: "owner".to_owned(),
                repo: "repo".to_owned(),
                path: None,
                reference: "v1".to_owned(),
            })
        );
    }

    // Ported: "parses https://host/owner/repo/path@ref" — github-actions/parse.spec.ts line 128
    #[test]
    fn parse_action_reference_parses_https_owner_repo_path_ref() {
        assert_eq!(
            parse_action_reference("https://github.enterprise.com/org/repo/workflow.yml@main"),
            Some(GithubActionReference::Repository {
                hostname: "github.enterprise.com".to_owned(),
                is_explicit_hostname: true,
                owner: "org".to_owned(),
                repo: "repo".to_owned(),
                path: Some("workflow.yml".to_owned()),
                reference: "main".to_owned(),
            })
        );
    }

    // Ported: "returns ratchetExclude for ratchet:exclude" — github-actions/parse.spec.ts line 147
    #[test]
    fn parse_comment_returns_ratchet_exclude_for_ratchet_exclude() {
        assert_eq!(
            parse_comment("ratchet:exclude"),
            comment_data(None, None, true, None, None)
        );
        assert_eq!(
            parse_comment("  ratchet:exclude  "),
            comment_data(None, None, true, None, None)
        );
    }

    // Ported: "returns empty object for no match" — github-actions/parse.spec.ts line 154
    #[test]
    fn parse_comment_returns_empty_object_for_no_match() {
        assert_eq!(parse_comment(""), GithubActionsCommentData::default());
        assert_eq!(
            parse_comment("some random comment"),
            GithubActionsCommentData::default()
        );
    }

    // Ported: "parses pinned version with tag= prefix" — github-actions/parse.spec.ts line 159
    #[test]
    fn parse_comment_parses_pinned_version_with_tag_prefix() {
        assert_eq!(
            parse_comment(" tag=v1.2.3"),
            comment_data(Some("v1.2.3"), None, false, Some(" tag=v1.2.3"), Some(0))
        );
    }

    // Ported: "parses pinned version with pin prefix" — github-actions/parse.spec.ts line 168
    #[test]
    fn parse_comment_parses_pinned_version_with_pin_prefix() {
        assert_eq!(
            parse_comment("pin v2"),
            comment_data(Some("v2"), None, false, Some("pin v2"), Some(0))
        );
    }

    // Ported: "parses pinned version with renovate: prefix" — github-actions/parse.spec.ts line 177
    #[test]
    fn parse_comment_parses_pinned_version_with_renovate_prefix() {
        assert_eq!(
            parse_comment("renovate: pin v3.0.0"),
            comment_data(
                Some("v3.0.0"),
                None,
                false,
                Some("renovate: pin v3.0.0"),
                Some(0),
            )
        );
    }

    // Ported: "parses pinned version with renovate:pin prefix" — github-actions/parse.spec.ts line 186
    #[test]
    fn parse_comment_parses_pinned_version_with_renovate_pin_prefix() {
        assert_eq!(
            parse_comment("renovate:pin v3.0.0"),
            comment_data(
                Some("v3.0.0"),
                None,
                false,
                Some("renovate:pin v3.0.0"),
                Some(0),
            )
        );
    }

    // Ported: "parses bare version" — github-actions/parse.spec.ts line 195
    #[test]
    fn parse_comment_parses_bare_version() {
        assert_eq!(
            parse_comment("v1"),
            comment_data(Some("v1"), None, false, Some("v1"), Some(0))
        );
    }

    // Ported: "parses version with @ prefix" — github-actions/parse.spec.ts line 204
    #[test]
    fn parse_comment_parses_version_with_at_prefix() {
        assert_eq!(
            parse_comment("@v4.1.0"),
            comment_data(Some("v4.1.0"), None, false, Some("@v4.1.0"), Some(0))
        );
    }

    // Ported: "parses ratchet pinned version" — github-actions/parse.spec.ts line 213
    #[test]
    fn parse_comment_parses_ratchet_pinned_version() {
        assert_eq!(
            parse_comment("ratchet:actions/checkout@v4"),
            comment_data(
                Some("v4"),
                None,
                false,
                Some("ratchet:actions/checkout@v4"),
                Some(0),
            )
        );
    }

    // Ported: "parses version without v prefix" — github-actions/parse.spec.ts line 222
    #[test]
    fn parse_comment_parses_version_without_v_prefix() {
        assert_eq!(
            parse_comment("1.2.3"),
            comment_data(Some("1.2.3"), None, false, Some("1.2.3"), Some(0))
        );
    }

    // Ported: "parses version with leading whitespace" — github-actions/parse.spec.ts line 231
    #[test]
    fn parse_comment_parses_version_with_leading_whitespace() {
        assert_eq!(
            parse_comment("   v1.0"),
            comment_data(Some("v1.0"), None, false, Some("   v1.0"), Some(0))
        );
    }

    // Ported: "parses prefixed version like node/v20" — github-actions/parse.spec.ts line 240
    #[test]
    fn parse_comment_parses_prefixed_version_like_node_v20() {
        assert_eq!(
            parse_comment("node/v20"),
            comment_data(Some("node/v20"), None, false, Some("node/v20"), Some(0))
        );
    }

    // Ported: "parses prerelease version like v2.2-rc.1" — github-actions/parse.spec.ts line 249
    #[test]
    fn parse_comment_parses_prerelease_version_like_v2_2_rc_1() {
        assert_eq!(
            parse_comment("v2.2-rc.1"),
            comment_data(Some("v2.2-rc.1"), None, false, Some("v2.2-rc.1"), Some(0))
        );
    }

    // Ported: "parses full semver prerelease version like v2.2.0-rc.1" — github-actions/parse.spec.ts line 258
    #[test]
    fn parse_comment_parses_full_semver_prerelease_version_like_v2_2_0_rc_1() {
        assert_eq!(
            parse_comment("v2.2.0-rc.1"),
            comment_data(
                Some("v2.2.0-rc.1"),
                None,
                false,
                Some("v2.2.0-rc.1"),
                Some(0),
            )
        );
    }

    // Ported: "parses bare non-semver ref" — github-actions/parse.spec.ts line 267
    #[test]
    fn parse_comment_parses_bare_non_semver_ref() {
        assert_eq!(
            parse_comment(" cargo-llvm-cov"),
            comment_data(
                None,
                Some("cargo-llvm-cov"),
                false,
                Some(" cargo-llvm-cov"),
                Some(0)
            )
        );
    }

    // Ported: "parses bare branch name" — github-actions/parse.spec.ts line 276
    #[test]
    fn parse_comment_parses_bare_branch_name() {
        assert_eq!(
            parse_comment(" main"),
            comment_data(None, Some("main"), false, Some(" main"), Some(0))
        );
    }

    // Ported: "ignores multi-word comments" — github-actions/parse.spec.ts line 285
    #[test]
    fn parse_comment_ignores_multi_word_comments() {
        assert_eq!(
            parse_comment("do not update"),
            GithubActionsCommentData::default()
        );
    }

    // Ported: "returns empty quote for unquoted string" — github-actions/parse.spec.ts line 291
    #[test]
    fn parse_quote_returns_empty_quote_for_unquoted_string() {
        assert_eq!(parse_quote("value"), quoted_value("value", None));
    }

    // Ported: "returns empty quote for empty string" — github-actions/parse.spec.ts line 295
    #[test]
    fn parse_quote_returns_empty_quote_for_empty_string() {
        assert_eq!(parse_quote(""), quoted_value("", None));
    }

    // Ported: "returns empty quote for single char" — github-actions/parse.spec.ts line 299
    #[test]
    fn parse_quote_returns_empty_quote_for_single_char() {
        assert_eq!(parse_quote("a"), quoted_value("a", None));
    }

    // Ported: "parses double quoted string" — github-actions/parse.spec.ts line 303
    #[test]
    fn parse_quote_parses_double_quoted_string() {
        assert_eq!(parse_quote("\"value\""), quoted_value("value", Some('"')));
    }

    // Ported: "parses single quoted string" — github-actions/parse.spec.ts line 307
    #[test]
    fn parse_quote_parses_single_quoted_string() {
        assert_eq!(parse_quote("'value'"), quoted_value("value", Some('\'')));
    }

    // Ported: "handles whitespace around quotes" — github-actions/parse.spec.ts line 311
    #[test]
    fn parse_quote_handles_whitespace_around_quotes() {
        assert_eq!(
            parse_quote("  \"value\"  "),
            quoted_value("value", Some('"'))
        );
    }

    // Ported: "returns empty quote for mismatched quotes" — github-actions/parse.spec.ts line 315
    #[test]
    fn parse_quote_returns_empty_quote_for_mismatched_quotes() {
        assert_eq!(parse_quote("\"value'"), quoted_value("\"value'", None));
        assert_eq!(parse_quote("'value\""), quoted_value("'value\"", None));
    }

    // Ported: "returns empty quote for only opening quote" — github-actions/parse.spec.ts line 320
    #[test]
    fn parse_quote_returns_empty_quote_for_only_opening_quote() {
        assert_eq!(parse_quote("\"value"), quoted_value("\"value", None));
    }

    // Ported: "returns null for non-uses lines" — github-actions/parse.spec.ts line 326
    #[test]
    fn parse_uses_line_returns_none_for_non_uses_lines() {
        assert!(parse_uses_line("name: test").is_none());
        assert!(parse_uses_line("run: echo hello").is_none());
        assert!(parse_uses_line("").is_none());
        assert!(parse_uses_line("uses: value").is_none());
    }

    // Ported: "returns null when value is only a comment" — github-actions/parse.spec.ts line 333
    #[test]
    fn parse_uses_line_returns_none_when_value_is_only_a_comment() {
        assert!(parse_uses_line("      uses: # only comment").is_none());
    }

    // Ported: "parses simple uses line without comment" — github-actions/parse.spec.ts line 337
    #[test]
    fn parse_uses_line_parses_simple_uses_line_without_comment() {
        assert_eq!(
            parse_uses_line("      uses: actions/checkout@v4"),
            Some(parsed_uses_line(UsesLineExpected {
                indentation: "      ",
                uses_prefix: "      uses: ",
                replace_string: "actions/checkout@v4",
                comment_preceding_whitespace: "",
                comment_string: "",
                action_ref: Some(repo_ref("actions", "checkout", "v4")),
                comment_data: GithubActionsCommentData::default(),
                quote: None,
            }))
        );
    }

    // Ported: "parses uses line with - prefix" — github-actions/parse.spec.ts line 359
    #[test]
    fn parse_uses_line_parses_uses_line_with_dash_prefix() {
        assert_eq!(
            parse_uses_line("      - uses: actions/checkout@v4"),
            Some(parsed_uses_line(UsesLineExpected {
                indentation: "      - ",
                uses_prefix: "      - uses: ",
                replace_string: "actions/checkout@v4",
                comment_preceding_whitespace: "",
                comment_string: "",
                action_ref: Some(repo_ref("actions", "checkout", "v4")),
                comment_data: GithubActionsCommentData::default(),
                quote: None,
            }))
        );
    }

    // Ported: "parses uses line with comment" — github-actions/parse.spec.ts line 381
    #[test]
    fn parse_uses_line_parses_uses_line_with_comment() {
        assert_eq!(
            parse_uses_line("      uses: actions/checkout@abc123 # v4"),
            Some(parsed_uses_line(UsesLineExpected {
                indentation: "      ",
                uses_prefix: "      uses: ",
                replace_string: "actions/checkout@abc123",
                comment_preceding_whitespace: " ",
                comment_string: "# v4",
                action_ref: Some(repo_ref("actions", "checkout", "abc123")),
                comment_data: comment_data(Some("v4"), None, false, Some(" v4"), Some(0)),
                quote: None,
            }))
        );
    }

    // Ported: "parses uses line with multiple spaces before comment" — github-actions/parse.spec.ts line 407
    #[test]
    fn parse_uses_line_parses_uses_line_with_multiple_spaces_before_comment() {
        assert_eq!(
            parse_uses_line("      uses: actions/checkout@abc123   # v4"),
            Some(parsed_uses_line(UsesLineExpected {
                indentation: "      ",
                uses_prefix: "      uses: ",
                replace_string: "actions/checkout@abc123",
                comment_preceding_whitespace: "   ",
                comment_string: "# v4",
                action_ref: Some(repo_ref("actions", "checkout", "abc123")),
                comment_data: comment_data(Some("v4"), None, false, Some(" v4"), Some(0)),
                quote: None,
            }))
        );
    }

    // Ported: "parses double quoted value" — github-actions/parse.spec.ts line 435
    #[test]
    fn parse_uses_line_parses_double_quoted_value() {
        assert_eq!(
            parse_uses_line("      uses: \"actions/checkout@v4\""),
            Some(parsed_uses_line(UsesLineExpected {
                indentation: "      ",
                uses_prefix: "      uses: ",
                replace_string: "\"actions/checkout@v4\"",
                comment_preceding_whitespace: "",
                comment_string: "",
                action_ref: Some(repo_ref("actions", "checkout", "v4")),
                comment_data: GithubActionsCommentData::default(),
                quote: Some('"'),
            }))
        );
    }

    // Ported: "parses single quoted value" — github-actions/parse.spec.ts line 457
    #[test]
    fn parse_uses_line_parses_single_quoted_value() {
        assert_eq!(
            parse_uses_line("      uses: 'actions/checkout@v4'"),
            Some(parsed_uses_line(UsesLineExpected {
                indentation: "      ",
                uses_prefix: "      uses: ",
                replace_string: "'actions/checkout@v4'",
                comment_preceding_whitespace: "",
                comment_string: "",
                action_ref: Some(repo_ref("actions", "checkout", "v4")),
                comment_data: GithubActionsCommentData::default(),
                quote: Some('\''),
            }))
        );
    }

    // Ported: "parses quoted value with comment" — github-actions/parse.spec.ts line 479
    #[test]
    fn parse_uses_line_parses_quoted_value_with_comment() {
        assert_eq!(
            parse_uses_line("      uses: \"owner/repo@abc123\" # v1.0.0"),
            Some(parsed_uses_line(UsesLineExpected {
                indentation: "      ",
                uses_prefix: "      uses: ",
                replace_string: "\"owner/repo@abc123\"",
                comment_preceding_whitespace: " ",
                comment_string: "# v1.0.0",
                action_ref: Some(repo_ref("owner", "repo", "abc123")),
                comment_data: comment_data(Some("v1.0.0"), None, false, Some(" v1.0.0"), Some(0)),
                quote: Some('"'),
            }))
        );
    }

    // Ported: "parses docker action" — github-actions/parse.spec.ts line 505
    #[test]
    fn parse_uses_line_parses_docker_action() {
        assert_eq!(
            parse_uses_line("      uses: docker://alpine:3.18"),
            Some(parsed_uses_line(UsesLineExpected {
                indentation: "      ",
                uses_prefix: "      uses: ",
                replace_string: "docker://alpine:3.18",
                comment_preceding_whitespace: "",
                comment_string: "",
                action_ref: Some(GithubActionReference::Docker {
                    image: "alpine".to_owned(),
                    tag: Some("3.18".to_owned()),
                    digest: None,
                    original_ref: "alpine:3.18".to_owned(),
                }),
                comment_data: GithubActionsCommentData::default(),
                quote: None,
            }))
        );
    }

    // Ported: "parses local action" — github-actions/parse.spec.ts line 524
    #[test]
    fn parse_uses_line_parses_local_action() {
        assert_eq!(
            parse_uses_line("      uses: ./local/action"),
            Some(parsed_uses_line(UsesLineExpected {
                indentation: "      ",
                uses_prefix: "      uses: ",
                replace_string: "./local/action",
                comment_preceding_whitespace: "",
                comment_string: "",
                action_ref: Some(GithubActionReference::Local {
                    path: "./local/action".to_owned(),
                }),
                comment_data: GithubActionsCommentData::default(),
                quote: None,
            }))
        );
    }

    // Ported: "handles ratchet:exclude comment" — github-actions/parse.spec.ts line 541
    #[test]
    fn parse_uses_line_handles_ratchet_exclude_comment() {
        assert_eq!(
            parse_uses_line("      uses: actions/checkout@v4 # ratchet:exclude"),
            Some(parsed_uses_line(UsesLineExpected {
                indentation: "      ",
                uses_prefix: "      uses: ",
                replace_string: "actions/checkout@v4",
                comment_preceding_whitespace: " ",
                comment_string: "# ratchet:exclude",
                action_ref: Some(repo_ref("actions", "checkout", "v4")),
                comment_data: comment_data(None, None, true, None, None),
                quote: None,
            }))
        );
    }

    // Ported: "handles unrecognized comment" — github-actions/parse.spec.ts line 567
    #[test]
    fn parse_uses_line_handles_unrecognized_comment() {
        assert_eq!(
            parse_uses_line("      uses: actions/checkout@v4 # unrelated comment"),
            Some(parsed_uses_line(UsesLineExpected {
                indentation: "      ",
                uses_prefix: "      uses: ",
                replace_string: "actions/checkout@v4",
                comment_preceding_whitespace: " ",
                comment_string: "# unrelated comment",
                action_ref: Some(repo_ref("actions", "checkout", "v4")),
                comment_data: GithubActionsCommentData::default(),
                quote: None,
            }))
        );
    }

    // Ported: "returns null actionRef for invalid action" — github-actions/parse.spec.ts line 591
    #[test]
    fn parse_uses_line_returns_none_action_ref_for_invalid_action() {
        assert_eq!(
            parse_uses_line("      uses: invalid-no-at-symbol"),
            Some(parsed_uses_line(UsesLineExpected {
                indentation: "      ",
                uses_prefix: "      uses: ",
                replace_string: "invalid-no-at-symbol",
                comment_preceding_whitespace: "",
                comment_string: "",
                action_ref: None,
                comment_data: GithubActionsCommentData::default(),
                quote: None,
            }))
        );
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

    // Rust-specific: github_actions behavior test
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

    // Ported: "handles actions/setup-x without x-version field" — github-actions/extract.spec.ts line 873
    #[test]
    fn setup_x_without_version_returns_only_action_dep() {
        // When actions/setup-node is used without node-version in with:, only the action dep
        // is extracted (no spurious runtime version dep).
        let content = r#"
jobs:
  build:
    steps:
      - name: "Setup Node.js without version"
        uses: actions/setup-node@v3
        with:
          registry-url: 'https://npm.pkg.github.com'
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].action, "actions/setup-node");
        assert_eq!(deps[0].current_value, "v3");
        assert!(deps[0].skip_reason.is_none());
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

    // Ported: "extracts actions with fqdn" — github-actions/extract.spec.ts line 614
    #[test]
    fn extracts_actions_with_fqdn() {
        let content = r#"
jobs:
  build:
    steps:
      - name: "test1"
        uses: https://github.com/actions/cache/save@1bd1e32a3bdc45362d1e726936510720a7c30a57 # tag=v4.2.0
      - name: "test2"
        uses: https://code.forgejo.org/actions/setup-node@56337c425554a6be30cdef71bf441f15be286854 # v3.1.1
      - name: "test3"
        uses: https://gitea.com/actions/setup-node@56337c425554a6be30cdef71bf441f15be286854 # v3.1.1
      - name: "test4"
        uses: https://code.domain.test/actions/setup-node@56337c425554a6be30cdef71bf441f15be286854 # v3.1.1
"#;

        let deps = extract_fqdn(content);
        assert_eq!(deps.len(), 4);

        // [0] github.com
        assert_eq!(deps[0].dep_name, "https://github.com/actions/cache");
        assert_eq!(deps[0].package_name, "actions/cache");
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("1bd1e32a3bdc45362d1e726936510720a7c30a57")
        );
        assert_eq!(deps[0].current_value.as_deref(), Some("v4.2.0"));
        assert_eq!(
            deps[0].replace_string,
            "https://github.com/actions/cache/save@1bd1e32a3bdc45362d1e726936510720a7c30a57 # tag=v4.2.0"
        );
        assert_eq!(deps[0].datasource, Some("github-tags"));
        assert_eq!(deps[0].registry_url.as_deref(), Some("https://github.com/"));
        assert!(deps[0].skip_reason.is_none());

        // [1] code.forgejo.org
        assert_eq!(
            deps[1].dep_name,
            "https://code.forgejo.org/actions/setup-node"
        );
        assert_eq!(deps[1].package_name, "actions/setup-node");
        assert_eq!(
            deps[1].current_digest.as_deref(),
            Some("56337c425554a6be30cdef71bf441f15be286854")
        );
        assert_eq!(deps[1].current_value.as_deref(), Some("v3.1.1"));
        assert_eq!(
            deps[1].replace_string,
            "https://code.forgejo.org/actions/setup-node@56337c425554a6be30cdef71bf441f15be286854 # v3.1.1"
        );
        assert_eq!(deps[1].datasource, Some("forgejo-tags"));
        assert_eq!(
            deps[1].registry_url.as_deref(),
            Some("https://code.forgejo.org/")
        );
        assert!(deps[1].skip_reason.is_none());

        // [2] gitea.com
        assert_eq!(deps[2].dep_name, "https://gitea.com/actions/setup-node");
        assert_eq!(deps[2].package_name, "actions/setup-node");
        assert_eq!(deps[2].datasource, Some("gitea-tags"));
        assert_eq!(deps[2].registry_url.as_deref(), Some("https://gitea.com/"));
        assert!(deps[2].skip_reason.is_none());

        // [3] code.domain.test — unsupported host: skip with no registry
        assert_eq!(deps[3].skip_reason, Some("unsupported-url"));
        assert!(deps[3].registry_url.is_none());
        assert!(deps[3].datasource.is_none());
    }

    // ── extract_uses_with tests ───────────────────────────────────────────────

    // Ported: "extracts x-version from actions/setup-x" — github-actions/extract.spec.ts line 741
    #[test]
    fn setup_x_extracts_versioned_deps() {
        let content = r#"
jobs:
  build:
    steps:
      - name: "Setup Node.js"
        uses: actions/setup-node@v3
        with:
          node-version: '16.x'
      - name: "Setup Node.js exact"
        uses: actions/setup-node@v3
        with:
          node-version: '20.0.0'
      - name: "Setup Go"
        uses: actions/setup-go@v5
        with:
          go-version: '1.23'
      - name: "Setup Python with range"
        uses: actions/setup-python@v3
        with:
          python-version: '>=3.8.0 <3.10.0'
      - name: "Setup Node.js with latest"
        uses: actions/setup-node@v3
        with:
          node-version: 'latest'
"#;
        let deps = extract_uses_with(content);
        assert_eq!(deps.len(), 5);

        assert_eq!(deps[0].dep_name, "node");
        assert_eq!(deps[0].package_name, "actions/node-versions");
        assert_eq!(deps[0].current_value.as_deref(), Some("16.x"));
        assert_eq!(deps[0].datasource, "github-releases");
        assert_eq!(deps[0].versioning, Some("node"));
        assert_eq!(
            deps[0].extract_version,
            Some(r"^(?<version>\d+\.\d+\.\d+)(-\d+)?$")
        );
        assert_eq!(deps[0].skip_reason, None);

        assert_eq!(deps[1].dep_name, "node");
        assert_eq!(deps[1].current_value.as_deref(), Some("20.0.0"));

        assert_eq!(deps[2].dep_name, "go");
        assert_eq!(deps[2].package_name, "actions/go-versions");
        assert_eq!(deps[2].current_value.as_deref(), Some("1.23"));
        assert_eq!(deps[2].versioning, Some("npm"));

        assert_eq!(deps[3].dep_name, "python");
        assert_eq!(deps[3].package_name, "actions/python-versions");
        assert_eq!(deps[3].current_value.as_deref(), Some(">=3.8.0 <3.10.0"));
        assert_eq!(deps[3].versioning, Some("npm"));

        assert_eq!(deps[4].dep_name, "node");
        assert_eq!(deps[4].current_value.as_deref(), Some("latest"));
    }

    // Ported: "extracts x-version from actions/setup-x in composite action" — github-actions/extract.spec.ts line 891
    #[test]
    fn setup_x_composite_action() {
        let content = r#"
runs:
  using: 'composite'
  steps:
    - uses: actions/setup-node@v3
      with:
        node-version: '16.x'
    - uses: actions/setup-go@v5
      with:
        go-version: '1.23'
"#;
        let deps = extract_uses_with(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_name, "node");
        assert_eq!(deps[0].current_value.as_deref(), Some("16.x"));
        assert_eq!(deps[1].dep_name, "go");
        assert_eq!(deps[1].current_value.as_deref(), Some("1.23"));
    }

    // Ported: "extract from $step.uses" — github-actions/extract.spec.ts line 1033
    #[test]
    fn community_trivy_unspecified_version() {
        let content = "jobs:\n  build:\n    steps:\n      - uses: aquasecurity/setup-trivy@v0.2.6\n        with: {}\n";
        let deps = extract_uses_with(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "aquasecurity/trivy");
        assert_eq!(deps[0].package_name, "aquasecurity/trivy");
        assert_eq!(deps[0].datasource, "github-releases");
        assert_eq!(deps[0].current_value, None);
        assert_eq!(deps[0].skip_reason, Some("unspecified-version"));
        assert_eq!(deps[0].skip_stage, Some("extract"));
    }

    // Ported: "extract from $step.uses" — github-actions/extract.spec.ts line 1033
    #[test]
    fn community_trivy_with_version() {
        let content = "jobs:\n  build:\n    steps:\n      - uses: aquasecurity/setup-trivy@v0.2.6\n        with:\n          version: latest\n";
        let deps = extract_uses_with(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "aquasecurity/trivy");
        assert_eq!(deps[0].current_value.as_deref(), Some("latest"));
        assert_eq!(deps[0].datasource, "github-releases");
        assert_eq!(deps[0].skip_reason, None);
    }

    // Ported: "extract from $step.uses" — github-actions/extract.spec.ts line 1033
    #[test]
    fn community_pnpm_with_version() {
        let content = "jobs:\n  build:\n    steps:\n      - uses: pnpm/action-setup@v4\n        with:\n          version: latest\n";
        let deps = extract_uses_with(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "pnpm");
        assert_eq!(deps[0].package_name, "pnpm");
        assert_eq!(deps[0].datasource, "npm");
        assert_eq!(deps[0].current_value.as_deref(), Some("latest"));
    }

    // Ported: "extract from $step.uses" — github-actions/extract.spec.ts line 1033
    #[test]
    fn community_pnpm_numeric_version() {
        let content = "jobs:\n  build:\n    steps:\n      - uses: pnpm/action-setup@v4\n        with:\n          version: '10'\n";
        let deps = extract_uses_with(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value.as_deref(), Some("10"));
    }

    // Ported: "extract from $step.uses" — github-actions/extract.spec.ts line 1033
    #[test]
    fn community_bun_with_bun_version_key() {
        let content = "jobs:\n  build:\n    steps:\n      - uses: oven-sh/setup-bun@v2\n        with:\n          bun-version: '1.2.0'\n";
        let deps = extract_uses_with(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "bun");
        assert_eq!(deps[0].datasource, "npm");
        assert_eq!(deps[0].current_value.as_deref(), Some("1.2.0"));
    }

    // Ported: "extract from $step.uses" — github-actions/extract.spec.ts line 1033
    #[test]
    fn community_bun_unspecified_version() {
        let content =
            "jobs:\n  build:\n    steps:\n      - uses: oven-sh/setup-bun@v2\n        with: {}\n";
        let deps = extract_uses_with(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some("unspecified-version"));
    }

    // Ported: "extract from $step.uses" — github-actions/extract.spec.ts line 1033
    #[test]
    fn community_ruby_with_ruby_version_key() {
        let content = "jobs:\n  build:\n    steps:\n      - uses: ruby/setup-ruby@v1\n        with:\n          ruby-version: '3.4'\n";
        let deps = extract_uses_with(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "ruby");
        assert_eq!(deps[0].datasource, "ruby-version");
        assert_eq!(deps[0].current_value.as_deref(), Some("3.4"));
    }

    // Ported: "extract from $step.uses" — github-actions/extract.spec.ts line 1033
    #[test]
    fn community_pyright_invalid_version() {
        let content = "jobs:\n  build:\n    steps:\n      - uses: jakebailey/pyright-action@v2\n        with:\n          version: PATH\n";
        let deps = extract_uses_with(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "pyright");
        assert_eq!(deps[0].current_value.as_deref(), Some("PATH"));
        assert_eq!(deps[0].skip_reason, Some("invalid-version"));
        assert_eq!(deps[0].skip_stage, Some("extract"));
    }

    // Ported: "extract from $step.uses" — github-actions/extract.spec.ts line 1033
    #[test]
    fn community_jaxxstorm_repo_tag() {
        let content = "jobs:\n  build:\n    steps:\n      - uses: jaxxstorm/action-install-gh-release@v1.10.0\n        with:\n          repo: gotestyourself/gotestsum\n          tag: v1.12.1\n";
        let deps = extract_uses_with(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "gotestyourself/gotestsum");
        assert_eq!(deps[0].package_name, "gotestyourself/gotestsum");
        assert_eq!(deps[0].current_value.as_deref(), Some("v1.12.1"));
        assert_eq!(deps[0].datasource, "github-releases");
    }

    // Ported: "extract from $step.uses" — github-actions/extract.spec.ts line 1033
    #[test]
    fn community_pixi_with_pixi_version_key() {
        let content = "jobs:\n  build:\n    steps:\n      - uses: prefix-dev/setup-pixi@v0.8.3\n        with:\n          pixi-version: v0.41.4\n";
        let deps = extract_uses_with(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "prefix-dev/pixi");
        assert_eq!(deps[0].datasource, "github-releases");
        assert_eq!(deps[0].versioning, Some("conda"));
        assert_eq!(deps[0].current_value.as_deref(), Some("v0.41.4"));
    }

    // Ported: "extract from $step.uses" — github-actions/extract.spec.ts line 1033
    #[test]
    fn community_zizmor_with_version() {
        let content = "jobs:\n  build:\n    steps:\n      - uses: zizmorcore/zizmor-action@v0.5.2\n        with:\n          version: v1.23.1\n";
        let deps = extract_uses_with(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "ghcr.io/zizmorcore/zizmor");
        assert_eq!(deps[0].datasource, "docker");
        assert_eq!(deps[0].current_value.as_deref(), Some("v1.23.1"));
    }

    // Ported: "extract from $step.uses" — github-actions/extract.spec.ts line 1033
    #[test]
    fn community_docker_setup_docker_action() {
        let content = "jobs:\n  build:\n    steps:\n      - uses: docker/setup-docker-action@v4\n        with:\n          version: v27.1.0\n";
        let deps = extract_uses_with(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "docker/setup-docker-action");
        assert_eq!(deps[0].package_name, "moby/moby");
        assert_eq!(deps[0].extract_version, Some("^docker-(?<version>.+)$"));
        assert_eq!(deps[0].current_value.as_deref(), Some("v27.1.0"));
        assert_eq!(deps[0].datasource, "github-releases");
    }

    // Ported: "extract from $step.uses" — github-actions/extract.spec.ts line 1033
    #[test]
    fn community_setup_uv_fqdn() {
        let content = "jobs:\n  build:\n    steps:\n      - uses: https://github.com/astral-sh/setup-uv@v5\n        with:\n          version: 0.4.x\n";
        let deps = extract_uses_with(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "astral-sh/uv");
        assert_eq!(deps[0].datasource, "github-releases");
        assert_eq!(deps[0].versioning, Some("npm"));
        assert_eq!(deps[0].current_value.as_deref(), Some("0.4.x"));
    }

    // Ported: "extract from $step.uses" — github-actions/extract.spec.ts line 1033
    #[test]
    fn community_setup_uv_fqdn_empty_with() {
        let content = "jobs:\n  build:\n    steps:\n      - uses: https://github.com/astral-sh/setup-uv@v5\n        with: {}\n";
        let deps = extract_uses_with(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "astral-sh/uv");
        assert_eq!(deps[0].skip_reason, Some("unspecified-version"));
    }

    // Ported: "handles actions/setup-x without x-version field" — github-actions/extract.spec.ts line 873
    #[test]
    fn setup_x_missing_version_key_emits_unspecified() {
        // When the correct version key is absent, emit unspecified-version.
        let content = r#"
jobs:
  build:
    steps:
      - uses: actions/setup-node@v3
        with:
          registry-url: 'https://npm.pkg.github.com'
"#;
        let deps = extract_uses_with(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "node");
        assert_eq!(deps[0].current_value, None);
        assert_eq!(deps[0].skip_reason, Some("unspecified-version"));
    }

    // Fixture: first action in workflow_2.yml — actions/bin/shellcheck@master
    const WORKFLOW_2_FIRST_ACTION: &str = "steps:\n  - uses: actions/bin/shellcheck@master\n";

    // Ported: "use github.com as registry when no settings provided" — github-actions/extract.spec.ts line 79
    #[test]
    fn use_github_com_as_registry_when_no_settings_provided() {
        let deps = extract(WORKFLOW_2_FIRST_ACTION);
        assert!(!deps.is_empty());
        assert!(deps[0].registry_urls.is_empty());
    }

    // Ported: "use github.enterprise.com first and then github.com as registry running against github.enterprise.com" — github-actions/extract.spec.ts line 87
    #[test]
    fn use_enterprise_registry_when_endpoint_is_enterprise() {
        let ctx = GithubActionsContext {
            platform: Some("github".to_owned()),
            endpoint: Some("https://github.enterprise.com".to_owned()),
        };
        let deps = extract_with_context(WORKFLOW_2_FIRST_ACTION, &ctx);
        assert!(!deps.is_empty());
        assert_eq!(
            deps[0].registry_urls,
            vec!["https://github.enterprise.com", "https://github.com"]
        );
    }

    // Ported: "use github.enterprise.com first and then github.com as registry running against github.enterprise.com/api/v3" — github-actions/extract.spec.ts line 102
    #[test]
    fn use_enterprise_registry_when_endpoint_has_api_v3_path() {
        let ctx = GithubActionsContext {
            platform: Some("github".to_owned()),
            endpoint: Some("https://github.enterprise.com/api/v3".to_owned()),
        };
        let deps = extract_with_context(WORKFLOW_2_FIRST_ACTION, &ctx);
        assert!(!deps.is_empty());
        assert_eq!(
            deps[0].registry_urls,
            vec!["https://github.enterprise.com", "https://github.com"]
        );
    }

    // Ported: "use github.com only as registry when running against non-GitHub" — github-actions/extract.spec.ts line 117
    #[test]
    fn use_no_custom_registry_when_platform_is_not_github() {
        let ctx = GithubActionsContext {
            platform: Some("bitbucket".to_owned()),
            endpoint: Some("https://bitbucket.enterprise.com".to_owned()),
        };
        let deps = extract_with_context(WORKFLOW_2_FIRST_ACTION, &ctx);
        assert!(!deps.is_empty());
        assert!(deps[0].registry_urls.is_empty());
    }

    // Ported: "use github.com only as registry when running against github.com" — github-actions/extract.spec.ts line 129
    #[test]
    fn use_no_custom_registry_when_endpoint_is_github_com() {
        let ctx = GithubActionsContext {
            platform: Some("github".to_owned()),
            endpoint: Some("https://github.com".to_owned()),
        };
        let deps = extract_with_context(WORKFLOW_2_FIRST_ACTION, &ctx);
        assert!(!deps.is_empty());
        assert!(deps[0].registry_urls.is_empty());
    }

    // Ported: "use github.com only as registry when running against api.github.com" — github-actions/extract.spec.ts line 141
    #[test]
    fn use_no_custom_registry_when_endpoint_is_api_github_com() {
        let ctx = GithubActionsContext {
            platform: Some("github".to_owned()),
            endpoint: Some("https://api.github.com".to_owned()),
        };
        let deps = extract_with_context(WORKFLOW_2_FIRST_ACTION, &ctx);
        assert!(!deps.is_empty());
        assert!(deps[0].registry_urls.is_empty());
    }

    // Ported: "returns undefined registryUrls when endpoint is invalid URL" — github-actions/extract.spec.ts line 153
    #[test]
    fn returns_no_registry_urls_when_endpoint_is_invalid() {
        let ctx = GithubActionsContext {
            platform: Some("github".to_owned()),
            endpoint: Some("not-a-valid-url".to_owned()),
        };
        let deps = extract_with_context(WORKFLOW_2_FIRST_ACTION, &ctx);
        assert!(!deps.is_empty());
        assert!(deps[0].registry_urls.is_empty());
    }

    // Ported: "logs unknown schema" — github-actions/extract.spec.ts line 1055
    #[test]
    fn logs_unknown_schema_returns_empty() {
        // action.yml with node20 runner has no `uses:` lines → empty result (null in TS)
        let yaml = "runs:\n  using: 'node20'\n  main: 'index.js'\n";
        let deps = extract(yaml);
        assert!(deps.is_empty());
    }

    // Rust-specific: github-actions update behavior tests
    #[test]
    fn github_actions_update_dependency_replaces_tag() {
        let content = "jobs:\n  build:\n    steps:\n      - uses: actions/checkout@v3\n";
        let updated = github_actions_update_dependency(content, "actions/checkout", "v3", "v4");
        assert_eq!(
            updated,
            Some("jobs:\n  build:\n    steps:\n      - uses: actions/checkout@v4\n".to_owned())
        );
    }

    #[test]
    fn github_actions_update_dependency_no_match() {
        let content = "jobs:\n  build:\n    steps:\n      - uses: actions/checkout@v3\n";
        let updated = github_actions_update_dependency(content, "actions/setup-node", "v3", "v4");
        assert_eq!(updated, None);
    }

    #[test]
    fn github_actions_update_dependency_with_comment() {
        let content = "jobs:\n  build:\n    steps:\n      - uses: actions/checkout@v3 # v3\n";
        let updated = github_actions_update_dependency(content, "actions/checkout", "v3", "v4");
        assert_eq!(
            updated,
            Some("jobs:\n  build:\n    steps:\n      - uses: actions/checkout@v4 # v3\n".to_owned())
        );
    }

    #[test]
    fn github_actions_update_dependency_multiple_uses() {
        let content = "jobs:\n  build:\n    steps:\n      - uses: actions/checkout@v3\n      - uses: actions/setup-node@v3\n";
        let updated = github_actions_update_dependency(content, "actions/setup-node", "v3", "v4");
        assert_eq!(
            updated,
            Some("jobs:\n  build:\n    steps:\n      - uses: actions/checkout@v3\n      - uses: actions/setup-node@v4\n".to_owned())
        );
    }

    #[test]
    fn github_actions_update_dependency_quoted() {
        let content = "jobs:\n  build:\n    steps:\n      - uses: 'actions/checkout@v3'\n";
        let updated = github_actions_update_dependency(content, "actions/checkout", "v3", "v4");
        assert_eq!(
            updated,
            Some("jobs:\n  build:\n    steps:\n      - uses: 'actions/checkout@v4'\n".to_owned())
        );
    }
}
