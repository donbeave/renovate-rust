//! Renovate config `extends` preset version extractor.
//!
//! Reads `renovate.json`, `.renovaterc`, etc. and tracks preset repository
//! versions referenced in the `extends` field.
//!
//! Renovate reference:
//! - `lib/modules/manager/renovate-config-presets/extract.ts`
//! - Patterns: standard Renovate config file names
//! - Datasources: GitHub Tags, GitLab Tags
//!
//! ## Preset string formats
//!
//! ```json
//! {
//!   "extends": [
//!     "github>owner/renovate-config#v1.0.0",
//!     "gitlab>company/config#2.0",
//!     "config:base"
//!   ]
//! }
//! ```
//!
//! Only presets with an explicit `#tag` are actionable.

use std::sync::LazyLock;

use regex::Regex;

/// Which platform hosts the preset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PresetSource {
    /// `github>owner/repo` → GitHub Tags.
    GitHub,
    /// `gitlab>owner/repo` → GitLab Tags.
    GitLab,
}

/// Skip reason for a preset dep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PresetSkipReason {
    /// No `#tag` in the preset string.
    UnspecifiedVersion,
    /// Platform not supported (e.g. `gitea`, `npm`, `local`).
    UnsupportedDatasource,
}

/// A single Renovate preset dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresetDep {
    /// The repository path, e.g. `owner/renovate-config`.
    pub repo: String,
    /// The tag, e.g. `v1.0.0`.
    pub current_value: String,
    /// The source platform.
    pub source: PresetSource,
    pub skip_reason: Option<PresetSkipReason>,
}

// ── Regex ─────────────────────────────────────────────────────────────────────

/// Matches `"github>owner/repo#tag"` or `"gitlab>owner/repo#tag"` in a JSON file.
/// Also handles entries without a `#tag`.
static PRESET_STR_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##""(github|gitlab)>([^"#]+)(?:#([^"]+))?""##).unwrap());

// ── Public API ────────────────────────────────────────────────────────────────

/// Extract Renovate preset deps from a `renovate.json` / `.renovaterc` file.
pub fn extract(content: &str) -> Vec<PresetDep> {
    let mut deps = Vec::new();

    // Quick guard: must have "extends" somewhere and a platform prefix.
    if !content.contains("\"extends\"") || !content.contains(">") {
        return deps;
    }

    for cap in PRESET_STR_RE.captures_iter(content) {
        let platform = &cap[1];
        let repo = cap[2].trim().to_owned();
        let tag = cap.get(3).map(|m| m.as_str().to_owned());

        let source = match platform {
            "github" => PresetSource::GitHub,
            "gitlab" => PresetSource::GitLab,
            _ => {
                deps.push(PresetDep {
                    repo,
                    current_value: String::new(),
                    source: PresetSource::GitHub, // placeholder
                    skip_reason: Some(PresetSkipReason::UnsupportedDatasource),
                });
                continue;
            }
        };

        match tag {
            None => deps.push(PresetDep {
                repo,
                current_value: String::new(),
                source,
                skip_reason: Some(PresetSkipReason::UnspecifiedVersion),
            }),
            Some(t) => deps.push(PresetDep {
                repo,
                current_value: t,
                source,
                skip_reason: None,
            }),
        }
    }

    deps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_github_preset_with_tag() {
        let content = r#"{"extends": ["github>owner/renovate-config#v1.2.3", "config:base"]}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let d = &deps[0];
        assert_eq!(d.repo, "owner/renovate-config");
        assert_eq!(d.current_value, "v1.2.3");
        assert_eq!(d.source, PresetSource::GitHub);
        assert!(d.skip_reason.is_none());
    }

    #[test]
    fn skips_preset_without_tag() {
        let content = r#"{"extends": ["github>owner/renovate-config"]}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(PresetSkipReason::UnspecifiedVersion)
        );
    }

    #[test]
    fn extracts_gitlab_preset() {
        let content = r#"{"extends": ["gitlab>company/configs#2.0"]}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].source, PresetSource::GitLab);
        assert_eq!(deps[0].current_value, "2.0");
    }

    #[test]
    fn ignores_internal_presets() {
        let content = r#"{"extends": ["config:base", ":automergeMinor"]}"#;
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    #[test]
    fn empty_file_returns_empty() {
        assert!(extract("{}").is_empty());
        assert!(extract("").is_empty());
    }
}
