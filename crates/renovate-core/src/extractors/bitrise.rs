//! Bitrise CI step extractor.
//!
//! Parses `bitrise.yml` / `bitrise.yaml` and extracts step references from
//! `workflows.*.steps[]`. Each step array entry is a single-key YAML map
//! whose key is the step reference string.
//!
//! Renovate reference:
//! - `lib/modules/manager/bitrise/extract.ts`
//! - `lib/modules/manager/bitrise/utils.ts`
//! - Pattern: `/(^|/)bitrise\.ya?ml$/`
//! - Datasources: `bitrise`, `git-tags`
//!
//! ## Step reference formats
//!
//! ```text
//! step@1.2.3                             # steplib step, default registry
//! git::https://github.com/foo/bar.git@1  # git-tags datasource
//! path::./local/step                     # local — skipped (unspecified-version)
//! https://github.com/foo/bar.git::step@1 # steplib step, custom registry
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// Source for a Bitrise step dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitriseSource {
    /// Standard steplib step (bitrise datasource).
    Steplib {
        /// Optional custom registry URL; `None` = use default steplib.
        registry_url: Option<String>,
    },
    /// Git repository reference (git-tags datasource).
    Git {
        /// The git remote URL.
        repo_url: String,
    },
    /// Local path reference — cannot be updated.
    Local,
}

/// Skip reason for a Bitrise step that cannot be updated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitriseSkipReason {
    /// No version specified in the step reference.
    UnspecifiedVersion,
}

/// A single Bitrise step dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitriseDep {
    /// The step name (for steplib steps) or full repo URL (for git steps).
    pub dep_name: String,
    /// Version extracted from the step ref (None when skipped).
    pub current_value: Option<String>,
    /// The original step ref string (used for string replacement).
    pub replace_string: String,
    /// Source routing.
    pub source: BitriseSource,
    /// Set when no lookup should be performed.
    pub skip_reason: Option<BitriseSkipReason>,
}

// ── Regexes ───────────────────────────────────────────────────────────────────

/// `default_step_lib_source: <url>`
static DEFAULT_REGISTRY_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"(?m)^\s*default_step_lib_source:\s*['"]?([^\s'"#]+)"##).unwrap()
});

/// A `steps:` key at any indent (followed by end-of-line or comment).
static STEPS_KEY_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\s*)steps:\s*(?:#.*)?$").unwrap());

/// A YAML list item: `<indent>- <content>`.
static LIST_ITEM_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\s+)-\s+(.+)$").unwrap());

// ── Step ref parsing ──────────────────────────────────────────────────────────

/// Extract the YAML key from a step list-item content string.
///
/// Step refs never contain `: ` (colon-space) within the key portion, but may
/// contain `::` (Bitrise separator) or `://` (URL scheme). We scan for the
/// first `:` that is followed by whitespace or end-of-string and that is not
/// part of `::` or `://`.
fn extract_yaml_key(s: &str) -> &str {
    let s = s.trim();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b':' {
            i += 1;
            continue;
        }
        let next = bytes.get(i + 1).copied();
        match next {
            // `::` — Bitrise double-colon separator, skip both
            Some(b':') => {
                i += 2;
            }
            // `://` — URL scheme, skip all three
            Some(b'/') if bytes.get(i + 2) == Some(&b'/') => {
                i += 3;
            }
            // `:` at end-of-string or before whitespace → YAML key/value separator
            None | Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r') => {
                return s[..i].trim();
            }
            // `:` followed by anything else is part of the key (e.g. `:latest`)
            _ => {
                i += 1;
            }
        }
    }
    s
}

/// Parse a step reference string into a [`BitriseDep`].
fn parse_step_ref(step_ref: &str, default_registry: Option<&str>) -> BitriseDep {
    let replace_string = step_ref.to_owned();

    // `git::URL@version` — git-tags datasource
    if let Some(rest) = step_ref.strip_prefix("git::") {
        let (repo_url, current_value) = split_at_last_at(rest);
        let skip_reason = current_value
            .is_none()
            .then_some(BitriseSkipReason::UnspecifiedVersion);
        return BitriseDep {
            dep_name: repo_url.to_owned(),
            current_value: current_value.map(str::to_owned),
            replace_string,
            source: BitriseSource::Git {
                repo_url: repo_url.to_owned(),
            },
            skip_reason,
        };
    }

    // `path::./local` — local dependency, always skipped
    if step_ref.starts_with("path::") {
        return BitriseDep {
            dep_name: step_ref.to_owned(),
            current_value: None,
            replace_string,
            source: BitriseSource::Local,
            skip_reason: Some(BitriseSkipReason::UnspecifiedVersion),
        };
    }

    // `https://github.com/registry::step@version` — custom steplib registry
    if let Some(dcolon_pos) = find_double_colon(step_ref) {
        let registry_url = &step_ref[..dcolon_pos];
        let rest = &step_ref[dcolon_pos + 2..];
        let (step_name, current_value) = split_at_last_at(rest);
        let skip_reason = current_value
            .is_none()
            .then_some(BitriseSkipReason::UnspecifiedVersion);
        return BitriseDep {
            dep_name: step_name.to_owned(),
            current_value: current_value.map(str::to_owned),
            replace_string,
            source: BitriseSource::Steplib {
                registry_url: Some(registry_url.to_owned()),
            },
            skip_reason,
        };
    }

    // Plain `step@version` — standard steplib reference
    let (step_name, current_value) = split_at_last_at(step_ref);
    let skip_reason = current_value
        .is_none()
        .then_some(BitriseSkipReason::UnspecifiedVersion);
    BitriseDep {
        dep_name: step_name.to_owned(),
        current_value: current_value.map(str::to_owned),
        replace_string,
        source: BitriseSource::Steplib {
            registry_url: default_registry.map(str::to_owned),
        },
        skip_reason,
    }
}

/// Find the first `::` that is not `://` (i.e., not a URL scheme).
fn find_double_colon(s: &str) -> Option<usize> {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b':' && bytes[i + 1] == b':' {
            // Skip `://` (URL scheme)
            if bytes.get(i + 2) == Some(&b'/') {
                i += 3;
                continue;
            }
            return Some(i);
        }
        i += 1;
    }
    None
}

/// Split `name@version` at the last `@`.
fn split_at_last_at(s: &str) -> (&str, Option<&str>) {
    if let Some(pos) = s.rfind('@') {
        let (name, version) = s.split_at(pos);
        (name, Some(&version[1..]))
    } else {
        (s, None)
    }
}

/// Count leading ASCII spaces/tabs on a line (returns number of spaces
/// equivalent; tabs count as 1 for simplicity since Bitrise YAML uses spaces).
fn leading_spaces(line: &str) -> usize {
    line.len() - line.trim_start_matches(' ').len()
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Extract all Bitrise step dependencies from a `bitrise.yml` file.
///
/// Returns an empty `Vec` if the file has no recognisable steps.
pub fn extract(content: &str) -> Vec<BitriseDep> {
    // Extract optional default registry.
    let default_registry = DEFAULT_REGISTRY_RE
        .captures(content)
        .map(|c| c[1].to_owned());
    let default_registry = default_registry.as_deref();

    let mut deps = Vec::new();

    // State: are we inside a `steps:` block?  If so, at what indent?
    let mut steps_indent: Option<usize> = None;

    for line in content.lines() {
        // Strip trailing comments for structural analysis.
        let stripped = match line.find(" #") {
            Some(pos) => &line[..pos],
            None => line,
        };

        if stripped.trim().is_empty() {
            continue;
        }

        let indent = leading_spaces(stripped);

        // If we're in a steps block, check if this line is still part of it.
        if let Some(step_indent) = steps_indent {
            // A line at or below the `steps:` indent level that isn't a
            // list item ends the current steps section.
            if indent <= step_indent && !stripped.trim_start().starts_with('-') {
                steps_indent = None;
            }

            // Step list items are at the same indent as `steps:`.
            if indent == step_indent {
                if let Some(cap) = LIST_ITEM_RE.captures(stripped) {
                    let content_str = &cap[2];
                    let step_ref = extract_yaml_key(content_str);
                    if !step_ref.is_empty() {
                        deps.push(parse_step_ref(step_ref, default_registry));
                    }
                }
                // Stay in steps mode — list item at correct indent.
                continue;
            }

            // Deeper indent (step parameters) — stay in steps mode, skip.
            if indent > step_indent {
                continue;
            }
        }

        // Check for `steps:` key — enter steps mode.
        if let Some(cap) = STEPS_KEY_RE.captures(stripped) {
            steps_indent = Some(cap[1].len());
        }
    }

    deps
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "returns a valid file" — bitrise/extract.spec.ts line 11
    #[test]
    fn extracts_plain_step() {
        let content = "workflows:\n  test:\n    steps:\n    - script@1.1.5:\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let d = &deps[0];
        assert_eq!(d.dep_name, "script");
        assert_eq!(d.current_value.as_deref(), Some("1.1.5"));
        assert_eq!(d.replace_string, "script@1.1.5");
        assert_eq!(d.source, BitriseSource::Steplib { registry_url: None });
        assert!(d.skip_reason.is_none());
    }

    // Ported: "returns a valid file with custom default_step_lib_source" — bitrise/extract.spec.ts line 34
    #[test]
    fn extracts_custom_default_registry() {
        let content = "default_step_lib_source: https://github.com/custom/steplib.git\nworkflows:\n  test:\n    steps:\n    - restore-cache@1.1.2:\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let d = &deps[0];
        assert_eq!(d.dep_name, "restore-cache");
        assert_eq!(d.current_value.as_deref(), Some("1.1.2"));
        assert_eq!(
            d.source,
            BitriseSource::Steplib {
                registry_url: Some("https://github.com/custom/steplib.git".to_owned())
            }
        );
    }

    // Ported: "extracts git and path prefixes" — bitrise/extract.spec.ts line 75
    #[test]
    fn extracts_git_step() {
        let content = "workflows:\n  test:\n    steps:\n    - git::https://github.com/bitrise-io/steps-script.git@1.1.3:\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let d = &deps[0];
        assert_eq!(d.dep_name, "https://github.com/bitrise-io/steps-script.git");
        assert_eq!(d.current_value.as_deref(), Some("1.1.3"));
        assert_eq!(
            d.source,
            BitriseSource::Git {
                repo_url: "https://github.com/bitrise-io/steps-script.git".to_owned()
            }
        );
    }

    #[test]
    fn path_step_is_skipped() {
        let content = "workflows:\n  test:\n    steps:\n    - path::./relative/path:\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let d = &deps[0];
        assert_eq!(d.skip_reason, Some(BitriseSkipReason::UnspecifiedVersion));
        assert_eq!(d.source, BitriseSource::Local);
    }

    // Ported: "extracts Bitrise library reference" — bitrise/extract.spec.ts line 142
    #[test]
    fn custom_steplib_reference() {
        let content =
            "workflows:\n  test:\n    steps:\n    - https://github.com/foo/bar.git::script@1:\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let d = &deps[0];
        assert_eq!(d.dep_name, "script");
        assert_eq!(d.current_value.as_deref(), Some("1"));
        assert_eq!(
            d.source,
            BitriseSource::Steplib {
                registry_url: Some("https://github.com/foo/bar.git".to_owned())
            }
        );
    }

    // Ported: "returns null on an empty file" — bitrise/extract.spec.ts line 7
    #[test]
    fn empty_file_returns_empty() {
        assert!(extract("").is_empty());
    }

    #[test]
    fn unversioned_step_is_skipped() {
        let content = "workflows:\n  test:\n    steps:\n    - script:\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(BitriseSkipReason::UnspecifiedVersion)
        );
    }

    #[test]
    fn step_with_empty_map_value() {
        let content = "workflows:\n  deploy:\n    steps:\n    - activate-ssh-key@1.0.0: {}\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "activate-ssh-key");
        assert_eq!(deps[0].current_value.as_deref(), Some("1.0.0"));
    }

    // Ported: "handles workflows without steps" — bitrise/extract.spec.ts line 114
    #[test]
    fn workflow_without_steps_ignored() {
        let content = "workflows:\n  deploy_staging:\n    envs:\n    - SCHEME: Staging\n    after_run:\n    - deploy\n  deploy:\n    steps:\n    - activate-ssh-key@1.0.0: {}\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "activate-ssh-key");
    }

    #[test]
    fn multiple_steps_extracted() {
        let content = "workflows:\n  test:\n    steps:\n    - script@1.1.5:\n    - restore-cache@1.1.2:\n        foo: bar\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_name, "script");
        assert_eq!(deps[1].dep_name, "restore-cache");
    }
}
