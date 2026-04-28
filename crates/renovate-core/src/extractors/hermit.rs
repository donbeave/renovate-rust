//! Hermit package manager extractor.
//!
//! Hermit stores installed packages as hidden `.{name}-{version}.pkg` files
//! inside a `bin/` directory. The package manager file is `bin/hermit.hcl`
//! (or `bin/install`); the actual package list comes from the hidden `.*.pkg`
//! filenames in the same directory.
//!
//! Renovate reference:
//! - `lib/modules/manager/hermit/extract.ts`
//! - Patterns: `/(^|/)bin\/hermit\.hcl$|/` (config file)
//! - Datasource: Hermit (searches `cashapp/hermit-packages` by default)
//!
//! ## File naming convention
//!
//! - Versioned: `bin/.git-2.47.0.pkg` → name=`git`, version=`2.47.0`
//! - Channel:   `bin/.kubectl@stable.pkg` → name=`kubectl`, channel=`stable`

use std::sync::LazyLock;

use regex::Regex;

/// A single Hermit installed package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HermitDep {
    /// Package name (e.g. `git`, `kubectl`).
    pub name: String,
    /// Version string for versioned packages (e.g. `2.47.0`).
    /// For channel-pinned packages this is `@{channel}` (e.g. `@stable`).
    pub current_value: String,
    pub skip_reason: Option<HermitSkipReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HermitSkipReason {
    /// Package is pinned to a channel, not a version.
    ChannelPin,
}

// ── Regexes ───────────────────────────────────────────────────────────────────

/// Matches a versioned package file: `.{name}-{version}.pkg`
/// where version starts with a digit.
static PKG_VERSION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\.([A-Za-z][A-Za-z0-9_-]*?)-([0-9][^/]*)\.pkg$").expect("valid regex")
});

/// Matches a channel-pinned package file: `.{name}@{channel}.pkg`
static PKG_CHANNEL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\.([A-Za-z][A-Za-z0-9_-]*)@([A-Za-z][A-Za-z0-9_-]*)\.pkg$").expect("valid regex")
});

// ── Public API ────────────────────────────────────────────────────────────────

/// Extract Hermit package deps from the repository file list.
///
/// `files` is the full flat list of repository paths. This function scans
/// for hidden `.*.pkg` files in any `bin/` directory.
pub fn extract_from_file_list(files: &[String]) -> Vec<HermitDep> {
    let mut deps = Vec::new();

    for path in files {
        // Only care about files inside a `bin/` directory.
        let filename = match path.rsplit_once('/') {
            Some((dir, name)) if dir.ends_with("/bin") || dir == "bin" => name,
            _ => continue,
        };

        if !filename.starts_with('.') || !filename.ends_with(".pkg") {
            continue;
        }

        if let Some(cap) = PKG_VERSION_RE.captures(filename) {
            deps.push(HermitDep {
                name: cap[1].to_owned(),
                current_value: cap[2].to_owned(),
                skip_reason: None,
            });
        } else if let Some(cap) = PKG_CHANNEL_RE.captures(filename) {
            deps.push(HermitDep {
                name: cap[1].to_owned(),
                current_value: format!("@{}", &cap[2]),
                skip_reason: Some(HermitSkipReason::ChannelPin),
            });
        }
    }

    deps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_versioned_packages() {
        let files = vec![
            "bin/.git-2.47.0.pkg".to_owned(),
            "bin/.kubectl-1.31.2.pkg".to_owned(),
            "bin/.java-21.0.5.pkg".to_owned(),
        ];
        let deps = extract_from_file_list(&files);
        assert_eq!(deps.len(), 3);
        assert_eq!(deps[0].name, "git");
        assert_eq!(deps[0].current_value, "2.47.0");
        assert!(deps[0].skip_reason.is_none());
        assert_eq!(deps[1].name, "kubectl");
        assert_eq!(deps[1].current_value, "1.31.2");
        assert_eq!(deps[2].name, "java");
        assert_eq!(deps[2].current_value, "21.0.5");
    }

    #[test]
    fn extracts_channel_pinned_packages() {
        let files = vec!["bin/.kubectl@stable.pkg".to_owned()];
        let deps = extract_from_file_list(&files);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "kubectl");
        assert_eq!(deps[0].current_value, "@stable");
        assert_eq!(deps[0].skip_reason, Some(HermitSkipReason::ChannelPin));
    }

    #[test]
    fn ignores_non_bin_directories() {
        let files = vec![
            "src/.git-2.47.0.pkg".to_owned(),
            ".git-2.47.0.pkg".to_owned(),
            "lib/bin/.git-2.47.0.pkg".to_owned(), // nested bin dir is not matched
        ];
        let deps = extract_from_file_list(&files);
        // lib/bin/ has "bin" at the end, so it matches
        assert_eq!(deps.len(), 1);
    }

    #[test]
    fn ignores_hermit_hcl_and_non_pkg_files() {
        let files = vec![
            "bin/hermit.hcl".to_owned(),
            "bin/install".to_owned(),
            "bin/git".to_owned(), // no leading dot, not a .pkg
        ];
        let deps = extract_from_file_list(&files);
        assert!(deps.is_empty());
    }

    #[test]
    fn handles_empty_file_list() {
        assert!(extract_from_file_list(&[]).is_empty());
    }
}
