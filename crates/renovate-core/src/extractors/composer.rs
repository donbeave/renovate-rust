//! Composer `composer.json` dependency extractor.
//!
//! Parses PHP Composer manifest files and returns package dependencies with
//! their version constraints, ready for Packagist version lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/composer/extract.ts` — `extractPackageFile`
//! - `lib/modules/manager/composer/schema.ts`  — `ComposerExtract`
//!
//! ## Supported sections
//!
//! | Section | Dep type |
//! |---|---|
//! | `require`     | `Regular` |
//! | `require-dev` | `Dev` |
//!
//! ## Skip reasons
//!
//! | Reason | Example |
//! |---|---|
//! | `PlatformPackage` | `php`, `ext-intl`, `lib-curl`, `composer-plugin-api` |
//! | `DevBranch` | `dev-master`, `2.x-dev` — VCS branch references |

use serde::Deserialize;
use thiserror::Error;

/// Which `composer.json` section the dep came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerDepType {
    /// `require` section.
    Regular,
    /// `require-dev` section.
    Dev,
}

impl ComposerDepType {
    pub fn as_renovate_str(&self) -> &'static str {
        match self {
            ComposerDepType::Regular => "require",
            ComposerDepType::Dev => "require-dev",
        }
    }
}

/// Why a Composer dependency is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComposerSkipReason {
    /// Platform package (`php`, `ext-*`, `lib-*`, `composer-*`).
    PlatformPackage,
    /// Version is a VCS branch reference (`dev-master`, `2.x-dev`).
    DevBranch,
}

/// A single extracted Composer dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposerExtractedDep {
    /// Normalized package name (e.g. `symfony/framework-bundle`).
    pub name: String,
    /// Version constraint (e.g. `^6.0`, `*`).
    pub current_value: String,
    /// Which section this dep came from.
    pub dep_type: ComposerDepType,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<ComposerSkipReason>,
}

/// Errors from parsing a `composer.json`.
#[derive(Debug, Error)]
pub enum ComposerExtractError {
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
}

// ── Public API ─────────────────────────────────────────────────────────────

/// Parse a `composer.json` string and extract all dependencies.
pub fn extract(content: &str) -> Result<Vec<ComposerExtractedDep>, ComposerExtractError> {
    #[derive(Deserialize)]
    struct Manifest {
        #[serde(default)]
        require: std::collections::HashMap<String, String>,
        #[serde(rename = "require-dev", default)]
        require_dev: std::collections::HashMap<String, String>,
    }

    let manifest: Manifest = serde_json::from_str(content)?;
    let mut deps = Vec::new();

    for (name, version) in &manifest.require {
        deps.push(make_dep(name, version, ComposerDepType::Regular));
    }
    for (name, version) in &manifest.require_dev {
        deps.push(make_dep(name, version, ComposerDepType::Dev));
    }

    // Sort by name for deterministic output (HashMap is unordered).
    deps.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(deps)
}

// ── Helpers ───────────────────────────────────────────────────────────────

fn make_dep(name: &str, version: &str, dep_type: ComposerDepType) -> ComposerExtractedDep {
    let skip_reason = if is_platform_package(name) {
        Some(ComposerSkipReason::PlatformPackage)
    } else if is_dev_branch(version) {
        Some(ComposerSkipReason::DevBranch)
    } else {
        None
    };

    ComposerExtractedDep {
        name: name.to_owned(),
        current_value: version.to_owned(),
        dep_type,
        skip_reason,
    }
}

/// Returns `true` for PHP platform packages that aren't on Packagist.
///
/// Platform packages: `php`, `ext-*`, `lib-*`, `composer-*`, `hhvm`.
fn is_platform_package(name: &str) -> bool {
    name == "php"
        || name == "hhvm"
        || name.starts_with("ext-")
        || name.starts_with("lib-")
        || name.starts_with("composer-")
        || !name.contains('/')
}

/// Returns `true` for version strings that are VCS branch references.
///
/// Branch references: `dev-master`, `dev-main`, `2.x-dev`, `1.0.x-dev`.
fn is_dev_branch(version: &str) -> bool {
    version.starts_with("dev-") || version.ends_with("-dev")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn extract_ok(content: &str) -> Vec<ComposerExtractedDep> {
        extract(content).expect("parse should succeed")
    }

    // ── Platform packages ─────────────────────────────────────────────────────

    // Ported: "extracts dependencies with no lock file" — composer/extract.spec.ts line 32
    #[test]
    fn php_constraint_skipped() {
        let content = r#"{"require": {"php": ">=8.1"}}"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(ComposerSkipReason::PlatformPackage)
        );
    }

    // Ported: "extracts dependencies with no lock file" — composer/extract.spec.ts line 32
    #[test]
    fn ext_skipped() {
        let content = r#"{"require": {"ext-intl": "*"}}"#;
        let deps = extract_ok(content);
        assert_eq!(
            deps[0].skip_reason,
            Some(ComposerSkipReason::PlatformPackage)
        );
    }

    // Ported: "extracts dependencies with no lock file" — composer/extract.spec.ts line 32
    #[test]
    fn lib_skipped() {
        let content = r#"{"require": {"lib-curl": "*"}}"#;
        let deps = extract_ok(content);
        assert_eq!(
            deps[0].skip_reason,
            Some(ComposerSkipReason::PlatformPackage)
        );
    }

    // ── Dev branch versions ───────────────────────────────────────────────────

    // Ported: "extracts dependencies with no lock file" — composer/extract.spec.ts line 32
    #[test]
    fn dev_master_skipped() {
        let content = r#"{"require": {"vendor/pkg": "dev-master"}}"#;
        let deps = extract_ok(content);
        assert_eq!(deps[0].skip_reason, Some(ComposerSkipReason::DevBranch));
    }

    // Ported: "extracts dependencies with no lock file" — composer/extract.spec.ts line 32
    #[test]
    fn x_dev_skipped() {
        let content = r#"{"require": {"vendor/pkg": "2.x-dev"}}"#;
        let deps = extract_ok(content);
        assert_eq!(deps[0].skip_reason, Some(ComposerSkipReason::DevBranch));
    }

    // ── Normal deps ───────────────────────────────────────────────────────────

    // Ported: "extracts dependencies with no lock file" — composer/extract.spec.ts line 32
    #[test]
    fn extracts_regular_deps() {
        let content = r#"{
            "require": {
                "symfony/framework-bundle": "^6.4",
                "doctrine/orm": "^2.15"
            }
        }"#;
        let deps = extract_ok(content);
        let regular: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == ComposerDepType::Regular)
            .collect();
        assert_eq!(regular.len(), 2);
        assert!(
            regular
                .iter()
                .any(|d| d.name == "symfony/framework-bundle" && d.current_value == "^6.4")
        );
        assert!(
            regular
                .iter()
                .any(|d| d.name == "doctrine/orm" && d.current_value == "^2.15")
        );
    }

    // Ported: "extracts dependencies with no lock file" — composer/extract.spec.ts line 32
    #[test]
    fn extracts_dev_deps() {
        let content = r#"{
            "require-dev": {
                "phpunit/phpunit": "^10.0",
                "squizlabs/php_codesniffer": "^3.7"
            }
        }"#;
        let deps = extract_ok(content);
        assert_eq!(
            deps.iter()
                .filter(|d| d.dep_type == ComposerDepType::Dev)
                .count(),
            2
        );
    }

    // ── Fixture composer1.json (Renovate reference fixture) ──────────────────

    // Ported: "extracts dependencies with no lock file" — composer/extract.spec.ts line 32
    #[test]
    fn composer1_fixture() {
        let content = r#"{
            "require": {
                "php": ">=5.3.2",
                "ext-intl": "*",
                "symfony/assetic-bundle": "dev-master",
                "symfony/symfony": "2.1.*",
                "doctrine/common": "2.2.2",
                "doctrine/orm": "2.2.x-dev",
                "friendsofsymfony/user-bundle": "*",
                "composer/composer": "^1.10.0"
            },
            "require-dev": {
                "behat/behat": "2.3.*",
                "composer/composer": "^1.10.0"
            }
        }"#;
        let deps = extract_ok(content);

        // Platform packages skipped
        let php = deps.iter().find(|d| d.name == "php").unwrap();
        assert_eq!(php.skip_reason, Some(ComposerSkipReason::PlatformPackage));

        let ext = deps.iter().find(|d| d.name == "ext-intl").unwrap();
        assert_eq!(ext.skip_reason, Some(ComposerSkipReason::PlatformPackage));

        // Dev-branch versions skipped
        let assetic = deps
            .iter()
            .find(|d| d.name == "symfony/assetic-bundle")
            .unwrap();
        assert_eq!(assetic.skip_reason, Some(ComposerSkipReason::DevBranch));

        let orm = deps.iter().find(|d| d.name == "doctrine/orm").unwrap();
        assert_eq!(orm.skip_reason, Some(ComposerSkipReason::DevBranch));

        // Normal deps actionable
        let symfony = deps.iter().find(|d| d.name == "symfony/symfony").unwrap();
        assert!(symfony.skip_reason.is_none());
        assert_eq!(symfony.current_value, "2.1.*");
    }

    // Ported: "returns null for empty deps" — composer/extract.spec.ts line 28
    #[test]
    fn empty_content_ok() {
        let deps = extract_ok("{}");
        assert!(deps.is_empty());
    }

    // Ported: "returns null for invalid json" — composer/extract.spec.ts line 24
    #[test]
    fn invalid_json_returns_error() {
        assert!(extract("nothing here").is_err());
    }
}
