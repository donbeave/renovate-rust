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
//! | `pod 'MyPod', :git => 'URL', :tag => 'v1.0'` | Actionable — git-tags / github-tags / gitlab-tags |
//! | `pod 'MyPod', :git => '...'` | Skipped — `GitSource` (no tag) |
//! | `pod 'MyPod', :podspec => '...'` | Skipped — `PodspecSource` |

use std::sync::LazyLock;

use regex::Regex;

/// Datasource for a git-sourced pod.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CocoapodsDatasource {
    GithubTags,
    GitlabTags,
    GitTags,
}

/// Why a pod dep is being skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CocoapodsSkipReason {
    /// Declared with `:path =>` option.
    LocalPath,
    /// Declared with `:git =>` but no `:tag =>` option.
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
    /// Set for `:git => URL, :tag => VER` pods.
    pub datasource: Option<CocoapodsDatasource>,
    /// Package name for git-sourced pods (e.g. `owner/repo` for GitHub).
    pub package_name: Option<String>,
}

// ── Compiled regexes ──────────────────────────────────────────────────────────

/// Matches `pod 'Name'` or `pod 'Name', 'version'`.
/// Group 1: pod name (may include `/subspec`)
/// Group 2: optional version string
static POD_LINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^\s*pod\s+['"]([^'"]+)['"]\s*(?:,\s*['"]([^'"]+)['"])?(.*)"#).unwrap()
});

/// Matches `:git => 'URL'` in a pod line tail.
static GIT_URL_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#":git\s*=>\s*['"]([^'"]+)['"]"#).unwrap());

/// Matches `:tag => 'VALUE'` in a pod line tail.
static GIT_TAG_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#":tag\s*=>\s*['"]([^'"]+)['"]"#).unwrap());

/// GitHub HTTPS URL: `https://github.com/owner/repo[.git]`.
static GITHUB_HTTPS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"https://github\.com/([^/]+/[^/.]+?)(?:\.git)?$").unwrap());

/// GitHub SSH URL: `git@github.com:owner/repo[.git]`.
static GITHUB_SSH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"git@github\.com:([^/]+/[^/.]+?)(?:\.git)?$").unwrap());

/// GitLab HTTPS URL: `https://gitlab.com/owner/repo[.git]`.
static GITLAB_HTTPS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"https://gitlab\.com/([^/]+/[^/.]+?)(?:\.git)?$").unwrap());

/// GitLab SSH URL: `git@gitlab.com:owner/repo[.git]`.
static GITLAB_SSH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"git@gitlab\.com:([^/]+/[^/.]+?)(?:\.git)?$").unwrap());

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
                datasource: None,
                package_name: None,
            });
            continue;
        }
        if tail.contains(":git") || tail.contains("git:") || tail.contains(":github") {
            let git_url = GIT_URL_RE
                .captures(tail)
                .map(|c| c[1].to_owned())
                .unwrap_or_default();
            let git_tag = GIT_TAG_RE.captures(tail).map(|c| c[1].to_owned());

            if let Some(tag) = git_tag {
                // Has a :tag — route to the appropriate git datasource.
                let (datasource, package_name) = classify_git_url(&git_url);
                deps.push(CocoapodsExtractedDep {
                    name,
                    current_value: tag,
                    skip_reason: None,
                    datasource: Some(datasource),
                    package_name: Some(package_name),
                });
            } else {
                deps.push(CocoapodsExtractedDep {
                    name,
                    current_value: String::new(),
                    skip_reason: Some(CocoapodsSkipReason::GitSource),
                    datasource: None,
                    package_name: None,
                });
            }
            continue;
        }
        if tail.contains(":podspec") || tail.contains("podspec:") {
            deps.push(CocoapodsExtractedDep {
                name,
                current_value: String::new(),
                skip_reason: Some(CocoapodsSkipReason::PodspecSource),
                datasource: None,
                package_name: None,
            });
            continue;
        }

        deps.push(CocoapodsExtractedDep {
            name,
            current_value: version.unwrap_or_default(),
            skip_reason: None,
            datasource: None,
            package_name: None,
        });
    }

    deps
}

/// Determine the datasource and packageName for a git URL.
fn classify_git_url(url: &str) -> (CocoapodsDatasource, String) {
    if let Some(cap) = GITHUB_HTTPS
        .captures(url)
        .or_else(|| GITHUB_SSH.captures(url))
    {
        return (CocoapodsDatasource::GithubTags, cap[1].to_owned());
    }
    if let Some(cap) = GITLAB_HTTPS
        .captures(url)
        .or_else(|| GITLAB_SSH.captures(url))
    {
        return (CocoapodsDatasource::GitlabTags, cap[1].to_owned());
    }
    // Generic git URL — use as-is for the package name.
    (CocoapodsDatasource::GitTags, url.to_owned())
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
        // MyGitPod has :git + :tag → extracted as GitHub dep, not skipped
        assert_eq!(git.datasource, Some(CocoapodsDatasource::GithubTags));
        assert_eq!(git.current_value, "1.0.0");

        let quick = deps.iter().find(|d| d.name == "Quick").unwrap();
        assert_eq!(quick.current_value, "~> 5.0");
    }

    #[test]
    fn empty_podfile_returns_empty() {
        assert!(extract("source 'https://cdn.cocoapods.org/'\n").is_empty());
    }

    // Ported: "extracts from simple file" — cocoapods/extract.spec.ts line 13
    #[test]
    fn simple_podfile_fixture() {
        let content = r#"source 'https://github.com/Artsy/Specs.git'
pod 'a'
pod 'a/sub'
pod 'b', '1.2.3'
pod 'c', "1.2.3"
pod 'd', :path => '~/Documents/Alamofire'
pod 'e', :git => 'e.git'
pod 'f', :git => 'f.git', :branch => 'dev'
pod 'g', :git => 'g.git', :tag => '3.2.1'
pod 'h', :git => 'https://github.com/foo/foo.git', :tag => '0.0.1'
pod 'i', :git => 'git@github.com:foo/foo.git', :tag => '0.0.1'
pod 'j', :git => 'https://gitlab.com/bar/bar.git', :tag => '0.1.0'
pod 'k', :git => 'https://gitlab.com/bar/bar.git', :tag => '0.1.0'
pod 'l', :git => 'https://example.com/baz/baz.git', :tag => '1.0.0'
pod 'm', :git => 'git@example.com:baz/baz.git', :tag => '1.0.0'
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 14);

        let b = deps.iter().find(|d| d.name == "b").unwrap();
        assert_eq!(b.current_value, "1.2.3");

        let d = deps.iter().find(|d| d.name == "d").unwrap();
        assert_eq!(d.skip_reason, Some(CocoapodsSkipReason::LocalPath));

        let e = deps.iter().find(|d| d.name == "e").unwrap();
        assert_eq!(e.skip_reason, Some(CocoapodsSkipReason::GitSource));

        let g = deps.iter().find(|d| d.name == "g").unwrap();
        assert_eq!(g.current_value, "3.2.1");
        assert_eq!(g.datasource, Some(CocoapodsDatasource::GitTags));

        let h = deps.iter().find(|d| d.name == "h").unwrap();
        assert_eq!(h.current_value, "0.0.1");
        assert_eq!(h.datasource, Some(CocoapodsDatasource::GithubTags));
        assert_eq!(h.package_name.as_deref(), Some("foo/foo"));

        let j = deps.iter().find(|d| d.name == "j").unwrap();
        assert_eq!(j.datasource, Some(CocoapodsDatasource::GitlabTags));
        assert_eq!(j.package_name.as_deref(), Some("bar/bar"));

        let l = deps.iter().find(|d| d.name == "l").unwrap();
        assert_eq!(l.datasource, Some(CocoapodsDatasource::GitTags));
        assert_eq!(
            l.package_name.as_deref(),
            Some("https://example.com/baz/baz.git")
        );
    }
}
