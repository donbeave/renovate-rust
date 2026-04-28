//! CocoaPods `Podfile` dependency extractor.
//!
//! Parses `Podfile` files with a regex line-scanner and returns pod
//! dependencies ready for CocoaPods trunk version lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/cocoapods/extract.ts` — `parseLine`, `extractPackageFile`
//! - `lib/modules/manager/cocoapods/index.ts`   — pattern `/(^|/)Podfile$/`
//!
//! ## Supported forms
//!
//! | Form | Treatment |
//! |---|---|
//! | `pod 'Alamofire', '~> 5.6'` | Actionable — Trunk lookup |
//! | `pod 'Firebase/Analytics'` | Actionable (no constraint) |
//! | `pod 'MyPod', :path => '...'` | Skipped — `LocalPath` |
//! | `pod 'MyPod', :git => '...'` | Skipped — `GitSource` |
//! | `pod 'MyPod', :podspec => '...'` | Skipped — `PodspecSource` |

use std::sync::LazyLock;

use regex::Regex;

/// Why a pod dep is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CocoapodsSkipReason {
    /// Declared with `:path =>` option.
    LocalPath,
    /// Declared with `:git =>` or `:github =>` option.
    GitSource,
    /// Declared with `:podspec =>` option.
    PodspecSource,
}

/// A single extracted CocoaPods dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CocoapodsExtractedDep {
    /// Pod name, possibly with subspec (e.g. `Firebase/Analytics`).
    pub name: String,
    /// Version constraint string (e.g. `~> 5.6.0`). Empty = unconstrained.
    pub current_value: String,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<CocoapodsSkipReason>,
}

// ── Compiled regexes ──────────────────────────────────────────────────────────

/// Matches `pod 'Name'` or `pod 'Name', 'version'`.
/// Group 1: pod name (may include `/subspec`)
/// Group 2: optional version string
static POD_LINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^\s*pod\s+['"]([^'"]+)['"]\s*(?:,\s*['"]([^'"]+)['"])?(.*)"#).unwrap()
});

// ── Public API ────────────────────────────────────────────────────────────────

/// Parse a `Podfile` and extract all pod dependencies.
pub fn extract(content: &str) -> Vec<CocoapodsExtractedDep> {
    let mut deps = Vec::new();

    for line in content.lines() {
        // Strip trailing inline comments.
        let line = strip_comment(line);
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let Some(cap) = POD_LINE.captures(trimmed) else {
            continue;
        };

        let name = cap[1].to_owned();
        let version = cap.get(2).map(|m| m.as_str().to_owned());
        let tail = cap.get(3).map(|m| m.as_str()).unwrap_or("");

        // Detect special source options.
        if tail.contains(":path") || tail.contains("path:") {
            deps.push(CocoapodsExtractedDep {
                name,
                current_value: String::new(),
                skip_reason: Some(CocoapodsSkipReason::LocalPath),
            });
            continue;
        }
        if tail.contains(":git") || tail.contains("git:") || tail.contains(":github") {
            deps.push(CocoapodsExtractedDep {
                name,
                current_value: String::new(),
                skip_reason: Some(CocoapodsSkipReason::GitSource),
            });
            continue;
        }
        if tail.contains(":podspec") || tail.contains("podspec:") {
            deps.push(CocoapodsExtractedDep {
                name,
                current_value: String::new(),
                skip_reason: Some(CocoapodsSkipReason::PodspecSource),
            });
            continue;
        }

        deps.push(CocoapodsExtractedDep {
            name,
            current_value: version.unwrap_or_default(),
            skip_reason: None,
        });
    }

    deps
}

fn strip_comment(line: &str) -> &str {
    if let Some(idx) = line.find('#') {
        &line[..idx]
    } else {
        line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pod_with_version() {
        let content = "pod 'Alamofire', '~> 5.6'\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "Alamofire");
        assert_eq!(deps[0].current_value, "~> 5.6");
        assert!(deps[0].skip_reason.is_none());
    }

    #[test]
    fn pod_without_version() {
        let content = "pod 'Firebase/Analytics'\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "Firebase/Analytics");
        assert!(deps[0].current_value.is_empty());
        assert!(deps[0].skip_reason.is_none());
    }

    #[test]
    fn pod_exact_version() {
        let content = "pod 'SDWebImage', '5.13.2'\n";
        let deps = extract(content);
        assert_eq!(deps[0].current_value, "5.13.2");
    }

    #[test]
    fn path_pod_skipped() {
        let content = "pod 'MyLocalPod', :path => '../MyLocalPod'\n";
        let deps = extract(content);
        assert_eq!(deps[0].skip_reason, Some(CocoapodsSkipReason::LocalPath));
    }

    #[test]
    fn git_pod_skipped() {
        let content = "pod 'MyGitPod', :git => 'https://github.com/org/repo.git'\n";
        let deps = extract(content);
        assert_eq!(deps[0].skip_reason, Some(CocoapodsSkipReason::GitSource));
    }

    #[test]
    fn podspec_pod_skipped() {
        let content = "pod 'MyPod', :podspec => 'https://example.com/MyPod.podspec'\n";
        let deps = extract(content);
        assert_eq!(
            deps[0].skip_reason,
            Some(CocoapodsSkipReason::PodspecSource)
        );
    }

    #[test]
    fn double_quoted_pod() {
        let content = r#"pod "RxSwift", "~> 6.0""#;
        let deps = extract(content);
        assert_eq!(deps[0].name, "RxSwift");
        assert_eq!(deps[0].current_value, "~> 6.0");
    }

    #[test]
    fn comments_ignored() {
        let content = "# pod 'Commented', '1.0'\npod 'Real', '2.0'\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "Real");
    }

    #[test]
    fn real_world_podfile() {
        let content = r#"
source 'https://cdn.cocoapods.org/'
platform :ios, '14.0'

target 'MyApp' do
  use_frameworks!

  # Core
  pod 'Alamofire', '~> 5.6'
  pod 'SDWebImage', '5.13.2'
  pod 'Firebase/Analytics'
  pod 'RxSwift', '~> 6.0'

  # Local
  pod 'MyLocalPod', :path => '../MyLocalPod'

  # Git
  pod 'MyGitPod', :git => 'https://github.com/org/repo.git', :tag => '1.0.0'

  target 'MyAppTests' do
    inherit! :search_paths
    pod 'Quick', '~> 5.0'
    pod 'Nimble', '~> 11.0'
  end
end
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 8);

        let alamofire = deps.iter().find(|d| d.name == "Alamofire").unwrap();
        assert_eq!(alamofire.current_value, "~> 5.6");
        assert!(alamofire.skip_reason.is_none());

        let firebase = deps
            .iter()
            .find(|d| d.name == "Firebase/Analytics")
            .unwrap();
        assert!(firebase.current_value.is_empty());
        assert!(firebase.skip_reason.is_none());

        let local = deps.iter().find(|d| d.name == "MyLocalPod").unwrap();
        assert_eq!(local.skip_reason, Some(CocoapodsSkipReason::LocalPath));

        let git = deps.iter().find(|d| d.name == "MyGitPod").unwrap();
        assert_eq!(git.skip_reason, Some(CocoapodsSkipReason::GitSource));

        let quick = deps.iter().find(|d| d.name == "Quick").unwrap();
        assert_eq!(quick.current_value, "~> 5.0");
    }

    #[test]
    fn empty_podfile_returns_empty() {
        assert!(extract("source 'https://cdn.cocoapods.org/'\n").is_empty());
    }
}
