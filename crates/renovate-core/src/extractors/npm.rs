//! package.json dependency extractor.
//!
//! Parses an npm `package.json` file and returns the set of package
//! dependencies with their version constraints, ready for registry lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/npm/extract/common/package-file.ts`
//! - `lib/modules/manager/npm/dep-types.ts` — `knownDepTypes`
//!
//! ## Supported dep sections
//!
//! Four standard dependency sections are extracted:
//! `dependencies`, `devDependencies`, `peerDependencies`,
//! `optionalDependencies`.
//!
//! ## Skip-reason classification
//!
//! Constraint strings that are not plain semver ranges are classified and
//! skipped:
//! - `workspace:*` / `workspace:^` etc. — pnpm/yarn workspace protocol
//! - `file:../path` / `link:../path` — local path reference
//! - `github:owner/repo` / `gitlab:...` / `bitbucket:...` — git platform shorthand
//! - `git+https://...` / `git://...` — git URL
//! - `http://...` / `https://...` — URL install
//! - `npm:other-pkg@...` — npm alias (deferred)

use std::collections::BTreeMap;

use serde::Deserialize;
use thiserror::Error;

/// Why an npm dependency is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NpmSkipReason {
    /// Dependency uses the workspace protocol (`workspace:*`).
    WorkspaceProtocol,
    /// Dependency is a local file/link reference (`file:../path`).
    LocalPath,
    /// Dependency is resolved from a git source.
    GitSource,
    /// Dependency is installed from a URL.
    UrlInstall,
    /// Dependency uses an npm alias (`npm:other-pkg`).
    NpmAlias,
}

/// Which `package.json` section the dep came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpmDepType {
    Regular,
    Dev,
    Peer,
    Optional,
}

/// A single extracted npm dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NpmExtractedDep {
    /// Package name (the key in the dep section).
    pub name: String,
    /// The version constraint string (e.g. `"^18.0.0"`).
    pub current_value: String,
    /// Which dep section this came from.
    pub dep_type: NpmDepType,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<NpmSkipReason>,
}

/// Errors from parsing a `package.json`.
#[derive(Debug, Error)]
pub enum NpmExtractError {
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
}

// ── Internal deserialization ──────────────────────────────────────────────────

#[derive(Debug, Deserialize, Default)]
struct PackageJson {
    #[serde(default)]
    dependencies: BTreeMap<String, String>,
    #[serde(rename = "devDependencies", default)]
    dev_dependencies: BTreeMap<String, String>,
    #[serde(rename = "peerDependencies", default)]
    peer_dependencies: BTreeMap<String, String>,
    #[serde(rename = "optionalDependencies", default)]
    optional_dependencies: BTreeMap<String, String>,
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Parse a `package.json` string and extract all npm dependencies.
///
/// Returns a flat list of deps across all four sections, each annotated with
/// its section type and any applicable skip reason.
pub fn extract(content: &str) -> Result<Vec<NpmExtractedDep>, NpmExtractError> {
    let pkg: PackageJson = serde_json::from_str(content)?;
    let mut out = Vec::new();

    for (section, dep_type) in [
        (&pkg.dependencies, NpmDepType::Regular),
        (&pkg.dev_dependencies, NpmDepType::Dev),
        (&pkg.peer_dependencies, NpmDepType::Peer),
        (&pkg.optional_dependencies, NpmDepType::Optional),
    ] {
        for (name, value) in section {
            out.push(classify(name.clone(), value, dep_type));
        }
    }

    Ok(out)
}

fn classify(name: String, value: &str, dep_type: NpmDepType) -> NpmExtractedDep {
    let skip_reason = skip_reason_for(value);
    NpmExtractedDep {
        name,
        current_value: value.to_owned(),
        dep_type,
        skip_reason,
    }
}

/// Classify an npm version string and return the skip reason, if any.
///
/// Returns `None` for plain semver-style constraints that should be looked up
/// in the npm registry.
fn skip_reason_for(value: &str) -> Option<NpmSkipReason> {
    let v = value.trim();

    // workspace protocol (pnpm / yarn)
    if v.starts_with("workspace:") {
        return Some(NpmSkipReason::WorkspaceProtocol);
    }

    // local path references
    if v.starts_with("file:")
        || v.starts_with("link:")
        || v.starts_with("portal:")
        || v.starts_with("patch:")
    {
        return Some(NpmSkipReason::LocalPath);
    }

    // git URL forms
    if v.starts_with("git+")
        || v.starts_with("git://")
        || v.starts_with("github:")
        || v.starts_with("gitlab:")
        || v.starts_with("bitbucket:")
        || v.starts_with("gist:")
        // GitHub shorthand: "owner/repo" (contains exactly one slash, no sigil)
        || (v.contains('/') && !v.starts_with('@') && !v.starts_with("http") && v.split('/').count() == 2)
    {
        return Some(NpmSkipReason::GitSource);
    }

    // URL installs
    if v.starts_with("http://") || v.starts_with("https://") {
        return Some(NpmSkipReason::UrlInstall);
    }

    // npm alias
    if v.starts_with("npm:") {
        return Some(NpmSkipReason::NpmAlias);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_ok(json: &str) -> Vec<NpmExtractedDep> {
        extract(json).expect("parse should succeed")
    }

    #[test]
    fn extracts_all_four_sections() {
        let json = r#"{
          "dependencies": { "express": "^4.18.0" },
          "devDependencies": { "jest": "^29.0" },
          "peerDependencies": { "react": ">=17" },
          "optionalDependencies": { "fsevents": "^2.0" }
        }"#;
        let deps = extract_ok(json);
        assert_eq!(deps.len(), 4);
        assert!(
            deps.iter()
                .any(|d| d.name == "express" && d.dep_type == NpmDepType::Regular)
        );
        assert!(
            deps.iter()
                .any(|d| d.name == "jest" && d.dep_type == NpmDepType::Dev)
        );
        assert!(
            deps.iter()
                .any(|d| d.name == "react" && d.dep_type == NpmDepType::Peer)
        );
        assert!(
            deps.iter()
                .any(|d| d.name == "fsevents" && d.dep_type == NpmDepType::Optional)
        );
    }

    #[test]
    fn plain_semver_has_no_skip_reason() {
        let json =
            r#"{ "dependencies": { "lodash": "4.17.21", "axios": "^1.0", "chalk": "~5.0" } }"#;
        let deps = extract_ok(json);
        assert!(deps.iter().all(|d| d.skip_reason.is_none()));
    }

    #[test]
    fn workspace_protocol_is_skipped() {
        let json = r#"{ "dependencies": { "my-lib": "workspace:*", "other": "workspace:^1.0" } }"#;
        let deps = extract_ok(json);
        assert!(
            deps.iter()
                .all(|d| d.skip_reason == Some(NpmSkipReason::WorkspaceProtocol))
        );
    }

    #[test]
    fn file_reference_is_skipped() {
        let json =
            r#"{ "dependencies": { "local": "file:../local-lib", "linked": "link:../linked" } }"#;
        let deps = extract_ok(json);
        assert!(
            deps.iter()
                .all(|d| d.skip_reason == Some(NpmSkipReason::LocalPath))
        );
    }

    #[test]
    fn git_source_forms_are_skipped() {
        let json = r#"{ "dependencies": {
          "a": "git+https://github.com/owner/repo.git",
          "b": "github:owner/repo",
          "c": "gitlab:owner/repo",
          "d": "owner/repo"
        }}"#;
        let deps = extract_ok(json);
        assert!(
            deps.iter()
                .all(|d| d.skip_reason == Some(NpmSkipReason::GitSource))
        );
    }

    #[test]
    fn url_install_is_skipped() {
        let json = r#"{ "dependencies": { "pkg": "https://example.com/pkg.tgz" } }"#;
        let deps = extract_ok(json);
        assert_eq!(deps[0].skip_reason, Some(NpmSkipReason::UrlInstall));
    }

    #[test]
    fn npm_alias_is_skipped() {
        let json = r#"{ "dependencies": { "react": "npm:preact@^10" } }"#;
        let deps = extract_ok(json);
        assert_eq!(deps[0].skip_reason, Some(NpmSkipReason::NpmAlias));
    }

    #[test]
    fn scoped_package_name_is_not_confused_with_git_shorthand() {
        // "@scope/pkg" contains a slash but starts with "@" — must NOT be treated
        // as a git owner/repo shorthand.
        let json = r#"{ "dependencies": { "@types/node": "^20.0" } }"#;
        let deps = extract_ok(json);
        assert!(deps[0].skip_reason.is_none());
    }

    #[test]
    fn empty_package_json_returns_empty_list() {
        let json = r#"{}"#;
        let deps = extract_ok(json);
        assert!(deps.is_empty());
    }

    #[test]
    fn missing_sections_are_ignored() {
        let json = r#"{ "dependencies": { "lodash": "^4" } }"#;
        let deps = extract_ok(json);
        assert_eq!(deps.len(), 1);
    }
}
