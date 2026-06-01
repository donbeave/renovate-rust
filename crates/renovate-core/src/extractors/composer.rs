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

use std::collections::HashMap;

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
    /// Package source is a local `path` repository.
    PathDependency,
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

/// Composer dependency metadata after applying repository configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposerResolvedDep {
    pub dep_name: String,
    pub current_value: String,
    pub locked_version: Option<String>,
    pub dep_type: ComposerDepType,
    pub datasource: Option<&'static str>,
    pub package_name: Option<String>,
    pub registry_urls: Vec<String>,
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
        #[serde(default)]
        repositories: serde_json::Value,
    }

    let manifest: Manifest = serde_json::from_str(content)?;

    // Collect path-type repo names → skip with PathDependency.
    let path_repos = collect_path_repos(&manifest.repositories);

    let mut deps = Vec::new();

    for (name, version) in &manifest.require {
        let mut dep = make_dep(name, version, ComposerDepType::Regular);
        if dep.skip_reason.is_none() && path_repos.contains(name.as_str()) {
            dep.skip_reason = Some(ComposerSkipReason::PathDependency);
        }
        deps.push(dep);
    }
    for (name, version) in &manifest.require_dev {
        let mut dep = make_dep(name, version, ComposerDepType::Dev);
        if dep.skip_reason.is_none() && path_repos.contains(name.as_str()) {
            dep.skip_reason = Some(ComposerSkipReason::PathDependency);
        }
        deps.push(dep);
    }

    // Sort by name for deterministic output (HashMap is unordered).
    deps.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(deps)
}

/// Parse a `composer.json` string and extract Renovate-style dependency metadata.
pub fn extract_resolved(content: &str) -> Result<Vec<ComposerResolvedDep>, ComposerExtractError> {
    extract_resolved_with_lock(content, None)
}

/// Parse a `composer.json` string and overlay versions from optional `composer.lock`.
pub fn extract_resolved_with_lock(
    content: &str,
    lock_content: Option<&str>,
) -> Result<Vec<ComposerResolvedDep>, ComposerExtractError> {
    #[derive(Deserialize)]
    struct Manifest {
        #[serde(default)]
        require: HashMap<String, String>,
        #[serde(rename = "require-dev", default)]
        require_dev: HashMap<String, String>,
        #[serde(default)]
        repositories: serde_json::Value,
    }

    let manifest: Manifest = serde_json::from_str(content)?;
    let repo_config = collect_repository_config(&manifest.repositories);
    let locked_versions = lock_content
        .and_then(|content| collect_locked_versions(content).ok())
        .unwrap_or_default();
    let mut deps = Vec::new();

    for (name, version) in &manifest.require {
        deps.push(make_resolved_dep(
            name,
            version,
            ComposerDepType::Regular,
            &repo_config,
            &locked_versions,
        ));
    }
    for (name, version) in &manifest.require_dev {
        deps.push(make_resolved_dep(
            name,
            version,
            ComposerDepType::Dev,
            &repo_config,
            &locked_versions,
        ));
    }

    deps.sort_by(|a, b| a.dep_name.cmp(&b.dep_name));
    Ok(deps)
}

/// Collect names of path-type repositories (both array and object forms).
fn collect_path_repos(repos: &serde_json::Value) -> std::collections::HashSet<&str> {
    let mut names = std::collections::HashSet::new();

    fn check_entry<'a>(
        entry: &'a serde_json::Value,
        key: Option<&'a str>,
        names: &mut std::collections::HashSet<&'a str>,
    ) {
        if entry.get("type").and_then(|v| v.as_str()) == Some("path")
            && let Some(k) = key
        {
            names.insert(k);
        }
    }

    match repos {
        serde_json::Value::Array(arr) => {
            // Array form: [{type, url}] — path repos have a name in the require section
            for entry in arr {
                // In array form, path repos match by name key if present
                if let Some(name) = entry.get("name").and_then(|v| v.as_str()) {
                    check_entry(entry, Some(name), &mut names);
                }
            }
        }
        serde_json::Value::Object(obj) => {
            // Object form: {"pkg-name": {type: "path", url: "..."}}
            for (key, entry) in obj {
                check_entry(entry, Some(key.as_str()), &mut names);
            }
        }
        _ => {}
    }

    names
}

// ── Helpers ───────────────────────────────────────────────────────────────

#[derive(Debug, Default)]
struct RepositoryConfig {
    packagist_disabled: bool,
    registry_urls: Vec<String>,
    package_sources: HashMap<String, PackageSource>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PackageSource {
    Vcs { url: String, bitbucket: bool },
    Path,
}

fn collect_repository_config(repos: &serde_json::Value) -> RepositoryConfig {
    let mut config = RepositoryConfig::default();

    match repos {
        serde_json::Value::Array(arr) => {
            for entry in arr {
                collect_repository_entry(entry, None, &mut config);
            }
        }
        serde_json::Value::Object(obj) => {
            for (key, entry) in obj {
                collect_repository_entry(entry, Some(key.as_str()), &mut config);
            }
        }
        _ => {}
    }

    if !config.packagist_disabled {
        push_unique(
            &mut config.registry_urls,
            "https://repo.packagist.org".to_owned(),
        );
    }

    config
}

fn collect_repository_entry(
    entry: &serde_json::Value,
    key: Option<&str>,
    config: &mut RepositoryConfig,
) {
    if key == Some("packagist.org") && entry.as_bool() == Some(false) {
        config.packagist_disabled = true;
        return;
    }

    if entry.get("packagist").and_then(|value| value.as_bool()) == Some(false) {
        config.packagist_disabled = true;
        return;
    }

    let repo_type = entry.get("type").and_then(|value| value.as_str());
    match repo_type {
        Some("composer") => {
            if let Some(url) = entry.get("url").and_then(|value| value.as_str()) {
                push_unique(&mut config.registry_urls, normalize_composer_repo_url(url));
            }
        }
        Some("vcs") | Some("git") => {
            if let Some(name) = entry.get("name").and_then(|value| value.as_str()).or(key)
                && let Some(url) = entry.get("url").and_then(|value| value.as_str())
            {
                config.package_sources.insert(
                    name.to_owned(),
                    PackageSource::Vcs {
                        url: url.to_owned(),
                        bitbucket: is_bitbucket_url(url),
                    },
                );
            }
        }
        Some("path") => {
            if let Some(name) = entry.get("name").and_then(|value| value.as_str()).or(key) {
                config
                    .package_sources
                    .insert(name.to_owned(), PackageSource::Path);
            }
        }
        _ => {}
    }
}

fn make_resolved_dep(
    name: &str,
    version: &str,
    dep_type: ComposerDepType,
    repo_config: &RepositoryConfig,
    locked_versions: &HashMap<String, String>,
) -> ComposerResolvedDep {
    if name == "php" {
        return ComposerResolvedDep {
            dep_name: name.to_owned(),
            current_value: version.to_owned(),
            locked_version: locked_versions.get(name).cloned(),
            dep_type,
            datasource: Some("github-tags"),
            package_name: Some("containerbase/php-prebuild".to_owned()),
            registry_urls: Vec::new(),
            skip_reason: None,
        };
    }

    let mut dep = ComposerResolvedDep {
        dep_name: name.to_owned(),
        current_value: version.to_owned(),
        locked_version: locked_versions.get(name).cloned(),
        dep_type,
        datasource: Some("packagist"),
        package_name: None,
        registry_urls: repo_config.registry_urls.clone(),
        skip_reason: None,
    };

    if is_platform_package(name) {
        dep.datasource = None;
        dep.registry_urls.clear();
        dep.skip_reason = Some(ComposerSkipReason::PlatformPackage);
        return dep;
    }

    match repo_config.package_sources.get(name) {
        Some(PackageSource::Vcs { url, bitbucket }) => {
            dep.datasource = Some(if *bitbucket {
                "bitbucket-tags"
            } else {
                "git-tags"
            });
            dep.package_name = Some(if *bitbucket {
                normalize_bitbucket_package_name(url).unwrap_or_else(|| name.to_owned())
            } else {
                url.to_owned()
            });
            dep.registry_urls.clear();
            dep.skip_reason = None;
        }
        Some(PackageSource::Path) => {
            dep.datasource = None;
            dep.registry_urls.clear();
            dep.skip_reason = Some(ComposerSkipReason::PathDependency);
        }
        None => {}
    }

    dep
}

fn collect_locked_versions(
    lock_content: &str,
) -> Result<HashMap<String, String>, serde_json::Error> {
    #[derive(Deserialize)]
    struct LockFile {
        #[serde(default)]
        packages: Vec<LockedPackage>,
        #[serde(rename = "packages-dev", default)]
        packages_dev: Vec<LockedPackage>,
    }

    #[derive(Deserialize)]
    struct LockedPackage {
        name: String,
        version: String,
    }

    let lock: LockFile = serde_json::from_str(lock_content)?;
    let mut versions = HashMap::new();
    for package in lock.packages.into_iter().chain(lock.packages_dev) {
        versions.insert(package.name, package.version);
    }
    Ok(versions)
}

fn normalize_composer_repo_url(url: &str) -> String {
    url.strip_suffix("/packages.json").unwrap_or(url).to_owned()
}

fn push_unique(values: &mut Vec<String>, value: String) {
    if !values.contains(&value) {
        values.push(value);
    }
}

fn is_bitbucket_url(url: &str) -> bool {
    url.contains("bitbucket.org")
}

fn normalize_bitbucket_package_name(url: &str) -> Option<String> {
    let trimmed = url.trim_end_matches(".git");
    if let Some(path) = trimmed.strip_prefix("https://bitbucket.org/") {
        return Some(path.to_owned());
    }
    if let Some(path) = trimmed.strip_prefix("git@bitbucket.org/") {
        return Some(path.to_owned());
    }
    if let Some(path) = trimmed.strip_prefix("git@bitbucket.org:") {
        return Some(path.to_owned());
    }
    None
}

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

/// Status result for `update_locked_composer_dependency`.
#[derive(Debug)]
pub enum ComposerUpdateLockedStatus {
    AlreadyUpdated,
    Unsupported,
}

impl ComposerUpdateLockedStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ComposerUpdateLockedStatus::AlreadyUpdated => "already-updated",
            ComposerUpdateLockedStatus::Unsupported => "unsupported",
        }
    }
}

/// Check if a composer lock file has a dependency at the target version.
///
/// Mirrors `lib/modules/manager/composer/update-locked.ts` `updateLockedDependency()`.
/// Determine the effective Composer range strategy.
///
/// Mirrors `lib/modules/manager/composer/range.ts` `getRangeStrategy()`.
pub fn get_composer_range_strategy<'a>(
    range_strategy: &'a str,
    composer_json_type: Option<&str>,
    current_value: Option<&str>,
) -> &'a str {
    let is_complex = current_value.is_some_and(|v| v.contains(" || "));
    if range_strategy == "bump" && is_complex {
        return "widen";
    }
    if range_strategy != "auto" {
        return range_strategy;
    }
    if is_complex || composer_json_type == Some("typo3-cms-extension") {
        return "widen";
    }
    "update-lockfile"
}

pub fn update_locked_composer_dependency(
    dep_name: Option<&str>,
    new_version: Option<&str>,
    lock_file_content: Option<&str>,
) -> ComposerUpdateLockedStatus {
    let (Some(dep_name), Some(new_version), Some(lock_file_content)) =
        (dep_name, new_version, lock_file_content)
    else {
        return ComposerUpdateLockedStatus::Unsupported;
    };
    let Ok(locked) = collect_locked_versions(lock_file_content) else {
        return ComposerUpdateLockedStatus::Unsupported;
    };
    if locked.get(dep_name).is_some_and(|v| v == new_version) {
        ComposerUpdateLockedStatus::AlreadyUpdated
    } else {
        ComposerUpdateLockedStatus::Unsupported
    }
}

// ── Schema parsers (mirrors lib/modules/manager/composer/schema.ts) ──────────

/// A parsed Composer repository entry.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(tag = "type")]
pub enum ComposerRepo {
    #[serde(rename = "composer")]
    Composer {
        url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
    #[serde(rename = "git")]
    Git { url: String, name: String },
    #[serde(rename = "path")]
    Path { url: String, name: String },
    #[serde(rename = "disable-packagist")]
    DisablePackagist,
}

/// Parsed Repos result.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedComposerRepos {
    pub registry_urls: Option<Vec<String>>,
    pub git_repos: std::collections::HashMap<String, serde_json::Value>,
    pub path_repos: std::collections::HashMap<String, serde_json::Value>,
}

fn parse_one_repo(key: &str, val: &serde_json::Value) -> Option<ComposerRepo> {
    if val.as_bool() == Some(false) {
        if key == "packagist" || key == "packagist.org" {
            return Some(ComposerRepo::DisablePackagist);
        }
        return None;
    }
    let obj = val.as_object()?;
    let repo_type = obj.get("type")?.as_str()?;
    let url = obj
        .get("url")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_owned();
    match repo_type {
        "composer" => Some(ComposerRepo::Composer { url, name: None }),
        "vcs" | "git" => Some(ComposerRepo::Git {
            url,
            name: key.to_owned(),
        }),
        "path" => Some(ComposerRepo::Path {
            url,
            name: key.to_owned(),
        }),
        _ => None,
    }
}

/// Parse `ReposRecord` - object mapping names to repo configs or `false`.
pub fn parse_repos_record(input: &serde_json::Value) -> Vec<ComposerRepo> {
    let Some(obj) = input.as_object() else {
        return vec![];
    };
    let mut result = Vec::new();
    for (key, val) in obj {
        if let Some(repo) = parse_one_repo(key, val) {
            result.push(repo);
        }
    }
    result
}

/// Parse `ReposArray` - array of repo configs or `{packagist: false}` entries.
pub fn parse_repos_array(input: &serde_json::Value) -> Vec<ComposerRepo> {
    let Some(arr) = input.as_array() else {
        return vec![];
    };
    let mut result = Vec::new();
    for (idx, val) in arr.iter().enumerate() {
        let Some(obj) = val.as_object() else { continue };
        // Check disable-packagist pattern
        if obj.get("packagist").and_then(|v| v.as_bool()) == Some(false)
            || obj.get("packagist.org").and_then(|v| v.as_bool()) == Some(false)
        {
            result.push(ComposerRepo::DisablePackagist);
            continue;
        }
        let repo_type = obj.get("type").and_then(|v| v.as_str()).unwrap_or("");
        let url = obj
            .get("url")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_owned();
        let name = obj
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or(&format!("__{idx}"))
            .to_owned();
        match repo_type {
            "composer" => result.push(ComposerRepo::Composer { url, name: None }),
            "vcs" | "git" => result.push(ComposerRepo::Git { url, name }),
            "path" => result.push(ComposerRepo::Path { url, name }),
            _ => { /* silently drop */ }
        }
    }
    result
}

/// Parse `Repos` - object or array of repositories.
pub fn parse_repos(input: &serde_json::Value) -> ParsedComposerRepos {
    let repos: Vec<ComposerRepo> = if input.is_null() {
        vec![]
    } else if input.is_array() {
        parse_repos_array(input)
    } else if input.is_object() {
        parse_repos_record(input)
    } else {
        vec![]
    };

    let mut packagist = true;
    let mut registry_urls: Vec<String> = Vec::new();
    let mut git_repos: std::collections::HashMap<String, serde_json::Value> = Default::default();
    let mut path_repos: std::collections::HashMap<String, serde_json::Value> = Default::default();

    for repo in &repos {
        match repo {
            ComposerRepo::Composer { url, name } => {
                let clean_url = url.trim_end_matches("/packages.json").to_owned();
                registry_urls.push(clean_url.clone());
                let _ = name;
            }
            ComposerRepo::Git { url, name } => {
                git_repos.insert(
                    name.clone(),
                    serde_json::json!({
                        "name": name, "type": "git", "url": url
                    }),
                );
            }
            ComposerRepo::Path { url, name } => {
                path_repos.insert(
                    name.clone(),
                    serde_json::json!({
                        "name": name, "type": "path", "url": url
                    }),
                );
            }
            ComposerRepo::DisablePackagist => {
                packagist = false;
            }
        }
    }

    if packagist && !registry_urls.is_empty() {
        registry_urls.push("https://repo.packagist.org".to_owned());
    }

    ParsedComposerRepos {
        registry_urls: if registry_urls.is_empty() {
            None
        } else {
            Some(registry_urls)
        },
        git_repos,
        path_repos,
    }
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

    // Ported: "extracts dependencies with no lock file" — composer/extract.spec.ts line 32
    #[test]
    fn composer1_fixture_has_33_deps() {
        // Full composer1.json fixture: 27 require + 6 require-dev = 33 total.
        let content = include_str!("../../tests/fixtures/composer/composer1.json");
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 33);
    }

    // Ported: "skips path dependencies" — composer/extract.spec.ts line 284
    #[test]
    fn path_dependency_skipped() {
        let content = r#"{
            "name": "acme/path-sources",
            "repositories": {
                "acme/path1": {
                    "type": "path",
                    "url": "packages/acme/path1"
                }
            },
            "require": {
                "acme/path1": "*"
            }
        }"#;
        let deps = extract_ok(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "acme/path1");
        assert_eq!(
            deps[0].skip_reason,
            Some(ComposerSkipReason::PathDependency)
        );
    }

    // Ported: "extracts registryUrls" — composer/extract.spec.ts line 38
    #[test]
    fn extracts_registry_urls() {
        let content = r#"{
            "repositories": [
                {"type": "composer", "url": "https://wpackagist.org"},
                {"packagist": false}
            ],
            "require": {
                "aws/aws-sdk-php": "*",
                "composer/composer": "^1.10.0",
                "wpackagist-plugin/akismet": "dev-trunk",
                "wpackagist-plugin/wordpress-seo": ">=7.0.2",
                "wpackagist-theme/hueman": "*"
            }
        }"#;
        let deps = extract_resolved(content).unwrap();
        assert_eq!(deps.len(), 5);
        assert!(
            deps.iter()
                .all(|dep| dep.registry_urls == ["https://wpackagist.org"])
        );
        let akismet = deps
            .iter()
            .find(|dep| dep.dep_name == "wpackagist-plugin/akismet")
            .unwrap();
        assert_eq!(akismet.datasource, Some("packagist"));
        assert_eq!(akismet.current_value, "dev-trunk");
        assert!(akismet.skip_reason.is_none());
    }

    // Ported: "extracts object registryUrls" — composer/extract.spec.ts line 81
    #[test]
    fn extracts_object_registry_urls() {
        let content = r#"{
            "type": "project",
            "repositories": {
                "packagist.org": false,
                "wp-packagist": {"type": "composer", "url": "https://wpackagist.org"},
                "theme": {
                    "type": "package",
                    "package": {"name": "asha23/wp-seed-timber", "version": "1.2.6"}
                }
            },
            "require": {
                "php": ">=5.5",
                "composer/installers": "~1.0.12",
                "johnpbloch/wordpress": "*",
                "vlucas/phpdotenv": "^2.0.1",
                "asha23/wp-seed-timber": "*"
            }
        }"#;
        let deps = extract_resolved(content).unwrap();
        let php = deps.iter().find(|dep| dep.dep_name == "php").unwrap();
        assert_eq!(php.datasource, Some("github-tags"));
        assert_eq!(
            php.package_name.as_deref(),
            Some("containerbase/php-prebuild")
        );
        assert!(php.registry_urls.is_empty());

        for name in [
            "composer/installers",
            "johnpbloch/wordpress",
            "vlucas/phpdotenv",
            "asha23/wp-seed-timber",
        ] {
            let dep = deps.iter().find(|dep| dep.dep_name == name).unwrap();
            assert_eq!(dep.datasource, Some("packagist"));
            assert_eq!(dep.registry_urls, ["https://wpackagist.org"]);
        }
    }

    // Ported: "extracts repositories and registryUrls" — composer/extract.spec.ts line 186
    #[test]
    fn extracts_repositories_and_registry_urls() {
        let content = r#"{
            "repositories": [
                {"name": "awesome/vcs", "type": "vcs", "url": "https://my-vcs.example/my-vcs-repo"},
                {"name": "awesome/git", "type": "git", "url": "https://my-git.example/my-git-repo"},
                {"type": "composer", "url": "https://wpackagist.org"},
                {"type": "composer", "url": "https://gitlab.vendor.com/api/v4/group/2/-/packages/composer/packages.json"}
            ],
            "require": {
                "aws/aws-sdk-php": "*",
                "awesome/vcs": "dev-trunk",
                "awesome/git": ">=7.0.2"
            }
        }"#;
        let deps = extract_resolved(content).unwrap();
        let aws = deps
            .iter()
            .find(|dep| dep.dep_name == "aws/aws-sdk-php")
            .unwrap();
        assert_eq!(
            aws.registry_urls,
            [
                "https://wpackagist.org",
                "https://gitlab.vendor.com/api/v4/group/2/-/packages/composer",
                "https://repo.packagist.org",
            ]
        );

        let vcs = deps
            .iter()
            .find(|dep| dep.dep_name == "awesome/vcs")
            .unwrap();
        assert_eq!(vcs.datasource, Some("git-tags"));
        assert_eq!(
            vcs.package_name.as_deref(),
            Some("https://my-vcs.example/my-vcs-repo")
        );
        assert!(vcs.registry_urls.is_empty());

        let git = deps
            .iter()
            .find(|dep| dep.dep_name == "awesome/git")
            .unwrap();
        assert_eq!(git.datasource, Some("git-tags"));
        assert_eq!(
            git.package_name.as_deref(),
            Some("https://my-git.example/my-git-repo")
        );
    }

    // Ported: "extracts bitbucket repositories and registryUrls" — composer/extract.spec.ts line 219
    #[test]
    fn extracts_bitbucket_repositories() {
        let content = r#"{
            "repositories": [
                {"name": "awesome/bitbucket-repo1", "type": "vcs", "url": "https://bitbucket.org/awesome/bitbucket-repo1.git"},
                {"name": "awesome/bitbucket-repo2", "type": "vcs", "url": "git@bitbucket.org/awesome/bitbucket-repo2.git"},
                {"name": "awesome/bitbucket-repo3", "type": "vcs", "url": "git@bitbucket.org/awesome/bitbucket-repo3"}
            ],
            "require": {
                "awesome/bitbucket-repo1": "dev-trunk",
                "awesome/bitbucket-repo2": "dev-trunk",
                "awesome/bitbucket-repo3": "dev-trunk"
            }
        }"#;
        let deps = extract_resolved(content).unwrap();
        assert_eq!(deps.len(), 3);
        for dep in deps {
            assert_eq!(dep.datasource, Some("bitbucket-tags"));
            assert_eq!(dep.package_name.as_deref(), Some(dep.dep_name.as_str()));
            assert!(dep.registry_urls.is_empty());
            assert!(dep.skip_reason.is_none());
        }
    }

    // Ported: "extracts object repositories and registryUrls with lock file" — composer/extract.spec.ts line 248
    #[test]
    fn extracts_object_repositories_and_registry_urls_with_lock_file() {
        let content = r#"{
            "repositories": {
                "awesome/vcs": {"type": "vcs", "url": "https://my-vcs.example/my-vcs-repo"},
                "awesome/git": {"type": "git", "url": "git@my-git.example:my-git-repo"},
                "wpackagist": {"type": "composer", "url": "https://wpackagist.org"}
            },
            "require": {
                "aws/aws-sdk-php": "*",
                "awesome/vcs": "dev-trunk",
                "awesome/git": ">=7.0.2"
            }
        }"#;
        let lock = r#"{
            "packages": [
                {"name": "awesome/vcs", "version": "1.1.0"},
                {"name": "awesome/git", "version": "1.2.0"}
            ]
        }"#;
        let deps = extract_resolved_with_lock(content, Some(lock)).unwrap();
        assert_eq!(deps.len(), 3);

        let aws = deps
            .iter()
            .find(|dep| dep.dep_name == "aws/aws-sdk-php")
            .unwrap();
        assert_eq!(
            aws.registry_urls,
            ["https://wpackagist.org", "https://repo.packagist.org"]
        );
        assert_eq!(aws.locked_version, None);

        let vcs = deps
            .iter()
            .find(|dep| dep.dep_name == "awesome/vcs")
            .unwrap();
        assert_eq!(vcs.datasource, Some("git-tags"));
        assert_eq!(vcs.locked_version.as_deref(), Some("1.1.0"));
        assert_eq!(
            vcs.package_name.as_deref(),
            Some("https://my-vcs.example/my-vcs-repo")
        );

        let git = deps
            .iter()
            .find(|dep| dep.dep_name == "awesome/git")
            .unwrap();
        assert_eq!(git.datasource, Some("git-tags"));
        assert_eq!(git.locked_version.as_deref(), Some("1.2.0"));
        assert_eq!(
            git.package_name.as_deref(),
            Some("git@my-git.example:my-git-repo")
        );
    }

    // Ported: "extracts dependencies with lock file" — composer/extract.spec.ts line 313
    #[test]
    fn extracts_dependencies_with_empty_lock_file() {
        let content = include_str!("../../tests/fixtures/composer/composer1.json");
        let deps = extract_resolved_with_lock(content, Some("{}")).unwrap();
        assert_eq!(deps.len(), 33);
        assert!(deps.iter().all(|dep| dep.locked_version.is_none()));
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

    const LOCK_CONTENT: &str = include_str!("../../tests/fixtures/composer/composer5.lock");

    // Ported: "detects already updated" — modules/manager/composer/update-locked.spec.ts line 10
    #[test]
    fn composer_update_locked_detects_already_updated() {
        let result = update_locked_composer_dependency(
            Some("awesome/git"),
            Some("1.2.0"),
            Some(LOCK_CONTENT),
        );
        assert_eq!(result.as_str(), "already-updated");
    }

    // Ported: "returns unsupported" — modules/manager/composer/update-locked.spec.ts line 22
    #[test]
    fn composer_update_locked_returns_unsupported() {
        let result = update_locked_composer_dependency(
            Some("awesome/git"),
            Some("1.0.0"),
            Some(LOCK_CONTENT),
        );
        assert_eq!(result.as_str(), "unsupported");
    }

    // Ported: "returns same if not auto" — modules/manager/composer/range.spec.ts line 5
    #[test]
    fn composer_range_returns_same_if_not_auto() {
        assert_eq!(get_composer_range_strategy("widen", None, None), "widen");
    }

    // Ported: "replaces require-dev" — modules/manager/composer/range.spec.ts line 10
    #[test]
    fn composer_range_auto_require_dev_returns_update_lockfile() {
        assert_eq!(
            get_composer_range_strategy("auto", None, None),
            "update-lockfile"
        );
    }

    // Ported: "replaces project require" — modules/manager/composer/range.spec.ts line 18
    #[test]
    fn composer_range_auto_project_returns_update_lockfile() {
        assert_eq!(
            get_composer_range_strategy("auto", Some("project"), None),
            "update-lockfile"
        );
    }

    // Ported: "widens complex ranges" — modules/manager/composer/range.spec.ts line 27
    #[test]
    fn composer_range_auto_complex_returns_widen() {
        assert_eq!(
            get_composer_range_strategy("auto", None, Some("^1.6.0 || ^2.0.0")),
            "widen"
        );
    }

    // Ported: "widens complex bump" — modules/manager/composer/range.spec.ts line 36
    #[test]
    fn composer_range_bump_complex_returns_widen() {
        assert_eq!(
            get_composer_range_strategy("bump", None, Some("^1.6.0 || ^2.0.0")),
            "widen"
        );
    }

    // Ported: "defaults to update-lockfile" — modules/manager/composer/range.spec.ts line 45
    #[test]
    fn composer_range_auto_defaults_to_update_lockfile() {
        assert_eq!(
            get_composer_range_strategy("auto", None, Some("^1.6.0")),
            "update-lockfile"
        );
    }

    // Ported: "defaults to widen for TYPO3 extensions" — modules/manager/composer/range.spec.ts line 50
    #[test]
    fn composer_range_auto_typo3_returns_widen() {
        assert_eq!(
            get_composer_range_strategy("auto", Some("typo3-cms-extension"), None),
            "widen"
        );
    }
}

// Ported: "parses default values" — modules/manager/composer/schema.spec.ts line 5
#[test]
fn repos_record_parses_default() {
    let result = parse_repos_record(&serde_json::json!({}));
    assert_eq!(result, vec![]);
}

// Ported: "parses repositories" — modules/manager/composer/schema.spec.ts line 9
#[test]
fn repos_record_parses_repositories() {
    let input = serde_json::json!({
        "wpackagist": {"type": "composer", "url": "https://wpackagist.org"},
        "someGit": {"type": "vcs", "url": "https://some-vcs.com"},
        "somePath": {"type": "path", "url": "/some/path"},
        "packagist": false,
        "packagist.org": false,
        "foo": "bar",
    });
    let result = parse_repos_record(&input);
    // Order is not guaranteed for HashMap iteration, so check membership
    assert!(result.contains(&ComposerRepo::Composer {
        url: "https://wpackagist.org".to_owned(),
        name: None
    }));
    assert!(result.contains(&ComposerRepo::Git {
        url: "https://some-vcs.com".to_owned(),
        name: "someGit".to_owned()
    }));
    assert!(result.contains(&ComposerRepo::Path {
        url: "/some/path".to_owned(),
        name: "somePath".to_owned()
    }));
    assert_eq!(
        result
            .iter()
            .filter(|r| **r == ComposerRepo::DisablePackagist)
            .count(),
        2
    );
    assert_eq!(result.len(), 5); // foo: 'bar' is filtered
}

// Ported: "parses default values" — modules/manager/composer/schema.spec.ts line 30
#[test]
fn repos_array_parses_default() {
    let result = parse_repos_array(&serde_json::json!([]));
    assert_eq!(result, vec![]);
}

// Ported: "parses repositories" — modules/manager/composer/schema.spec.ts line 34
#[test]
fn repos_array_parses_repositories() {
    let input = serde_json::json!([
        {"type": "composer", "url": "https://wpackagist.org"},
        {"name": "someGit", "type": "vcs", "url": "https://some-vcs.com"},
        {"name": "somePath", "type": "path", "url": "/some/path"},
        {"packagist": false},
        {"packagist.org": false},
        {"foo": "bar"},
    ]);
    let result = parse_repos_array(&input);
    assert_eq!(
        result[0],
        ComposerRepo::Composer {
            url: "https://wpackagist.org".to_owned(),
            name: None
        }
    );
    assert_eq!(
        result[1],
        ComposerRepo::Git {
            url: "https://some-vcs.com".to_owned(),
            name: "someGit".to_owned()
        }
    );
    assert_eq!(
        result[2],
        ComposerRepo::Path {
            url: "/some/path".to_owned(),
            name: "somePath".to_owned()
        }
    );
    assert_eq!(result[3], ComposerRepo::DisablePackagist);
    assert_eq!(result[4], ComposerRepo::DisablePackagist);
    assert_eq!(result.len(), 5); // foo:bar filtered
}

// Ported: "parses default values" — modules/manager/composer/schema.spec.ts line 58
#[test]
fn repos_parses_null_default() {
    let result = parse_repos(&serde_json::Value::Null);
    assert!(result.registry_urls.is_none());
    assert!(result.git_repos.is_empty());
    assert!(result.path_repos.is_empty());
}

// Ported: "parses repositories" — modules/manager/composer/schema.spec.ts line 66
#[test]
fn repos_parses_array_repos() {
    let input = serde_json::json!([
        {"name": "wpackagist", "type": "composer", "url": "https://wpackagist.org"},
        {"name": "someGit", "type": "vcs", "url": "https://some-vcs.com"},
        {"name": "somePath", "type": "path", "url": "/some/path"},
    ]);
    let result = parse_repos(&input);
    assert_eq!(
        result.registry_urls.as_deref(),
        Some(
            &[
                "https://wpackagist.org".to_owned(),
                "https://repo.packagist.org".to_owned()
            ][..]
        )
    );
    assert!(result.git_repos.contains_key("someGit"));
    assert!(result.path_repos.contains_key("somePath"));
}

// Ported: "parses repositories with packagist disabled" — modules/manager/composer/schema.spec.ts line 92
#[test]
fn repos_parses_with_packagist_disabled() {
    let input = serde_json::json!({
        "wpackagist": {"type": "composer", "url": "https://wpackagist.org"},
        "someGit": {"type": "vcs", "url": "https://some-vcs.com"},
        "somePath": {"type": "path", "url": "/some/path"},
        "packagist": false,
    });
    let result = parse_repos(&input);
    // Only wpackagist.org, no packagist.org
    let urls = result.registry_urls.as_deref().unwrap();
    assert!(urls.contains(&"https://wpackagist.org".to_owned()));
    assert!(!urls.contains(&"https://repo.packagist.org".to_owned()));
}

#[test]
fn composer_dep_type_as_renovate_str() {
    assert_eq!(ComposerDepType::Regular.as_renovate_str(), "require");
    assert_eq!(ComposerDepType::Dev.as_renovate_str(), "require-dev");
}
