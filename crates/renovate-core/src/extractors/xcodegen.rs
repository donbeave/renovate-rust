//! XcodeGen `project.yml` Swift Package dependency extractor.
//!
//! Parses the `packages:` section of a `project.yml` file and extracts
//! Swift package dependencies for GitHub/GitLab/Git version tracking.
//!
//! Renovate reference:
//! - `lib/modules/manager/xcodegen/extract.ts`
//! - Pattern: `**/project.yml`
//! - Datasources: GitHub Tags, GitLab Tags, Git Tags
//!
//! ## File format
//!
//! ```yaml
//! packages:
//!   Alamofire:
//!     url: https://github.com/Alamofire/Alamofire.git
//!     from: "5.4.3"
//!   Charts:
//!     github: danielgindi/Charts
//!     exactVersion: "4.1.0"
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// Package source type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XcodeGenSource {
    /// GitHub repository (owner/repo).
    GitHub(String),
    /// GitLab repository (owner/repo).
    GitLab(String),
    /// Generic git URL.
    Git(String),
}

/// Why a dep is skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XcodeGenSkipReason {
    /// Package has a `path:` — local dependency.
    LocalPath,
    /// Package has only a `branch:` / `revision:` — not semver.
    NoSemverVersion,
    /// No `url:` or `github:` source found.
    MissingSource,
}

/// A single extracted XcodeGen Swift package dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XcodeGenDep {
    /// Package name (the key in the `packages:` map).
    pub name: String,
    /// Source location.
    pub source: Option<XcodeGenSource>,
    /// Version constraint (from `from`, `exactVersion`, `version`, etc.).
    pub current_value: String,
    /// Version field type used.
    pub dep_type: &'static str,
    /// Set when no lookup should be performed.
    pub skip_reason: Option<XcodeGenSkipReason>,
}

static KV: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"^\s+(\w+):\s*"?([^"#\n]+?)"?\s*(?:#.*)?$"##).unwrap());

/// Extract Swift package deps from a `project.yml` file.
pub fn extract(content: &str) -> Vec<XcodeGenDep> {
    let mut deps = Vec::new();

    enum State {
        Scanning,
        InPackages,
        InPackageEntry,
    }

    let mut state = State::Scanning;
    let mut pkg_name = String::new();
    let mut url: Option<String> = None;
    let mut github: Option<String> = None;
    let mut is_path = false;
    let mut current_value = String::new();
    let mut dep_type: &'static str = "";
    let mut has_branch_or_revision = false;

    let flush = |name: &str,
                 url: &Option<String>,
                 github: &Option<String>,
                 is_path: bool,
                 current_value: &str,
                 dep_type: &'static str,
                 has_branch_or_revision: bool,
                 deps: &mut Vec<XcodeGenDep>| {
        if name.is_empty() {
            return;
        }

        if is_path {
            deps.push(XcodeGenDep {
                name: name.to_owned(),
                source: None,
                current_value: String::new(),
                dep_type,
                skip_reason: Some(XcodeGenSkipReason::LocalPath),
            });
            return;
        }

        if current_value.is_empty() {
            if has_branch_or_revision || url.is_some() || github.is_some() {
                deps.push(XcodeGenDep {
                    name: name.to_owned(),
                    source: build_source(url, github),
                    current_value: String::new(),
                    dep_type,
                    skip_reason: Some(XcodeGenSkipReason::NoSemverVersion),
                });
            } else {
                deps.push(XcodeGenDep {
                    name: name.to_owned(),
                    source: None,
                    current_value: String::new(),
                    dep_type,
                    skip_reason: Some(XcodeGenSkipReason::MissingSource),
                });
            }
            return;
        }

        let source = build_source(url, github);
        if source.is_none() {
            deps.push(XcodeGenDep {
                name: name.to_owned(),
                source: None,
                current_value: current_value.to_owned(),
                dep_type,
                skip_reason: Some(XcodeGenSkipReason::MissingSource),
            });
            return;
        }

        deps.push(XcodeGenDep {
            name: name.to_owned(),
            source,
            current_value: current_value.to_owned(),
            dep_type,
            skip_reason: None,
        });
    };

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let indent = line.len() - line.trim_start().len();

        match state {
            State::Scanning => {
                if indent == 0 && trimmed == "packages:" {
                    state = State::InPackages;
                }
            }
            State::InPackages => {
                if indent == 0 {
                    // End of packages section
                    flush(
                        &pkg_name,
                        &url,
                        &github,
                        is_path,
                        &current_value,
                        dep_type,
                        has_branch_or_revision,
                        &mut deps,
                    );
                    state = State::Scanning;
                    if trimmed == "packages:" {
                        state = State::InPackages;
                    }
                    pkg_name.clear();
                    url = None;
                    github = None;
                    is_path = false;
                    current_value.clear();
                    dep_type = "";
                    has_branch_or_revision = false;
                } else if indent == 2 {
                    // New package entry — flush previous
                    flush(
                        &pkg_name,
                        &url,
                        &github,
                        is_path,
                        &current_value,
                        dep_type,
                        has_branch_or_revision,
                        &mut deps,
                    );
                    pkg_name.clear();
                    url = None;
                    github = None;
                    is_path = false;
                    current_value.clear();
                    dep_type = "";
                    has_branch_or_revision = false;

                    // `  PackageName:` (key only) or `  PackageName: {inline}`
                    if let Some(colon) = trimmed.find(':') {
                        pkg_name = trimmed[..colon].trim().to_owned();
                    }
                    state = State::InPackageEntry;
                }
            }
            State::InPackageEntry => {
                if indent <= 2 {
                    // End of this entry
                    if indent == 0 {
                        flush(
                            &pkg_name,
                            &url,
                            &github,
                            is_path,
                            &current_value,
                            dep_type,
                            has_branch_or_revision,
                            &mut deps,
                        );
                        state = State::Scanning;
                        if trimmed == "packages:" {
                            state = State::InPackages;
                        }
                        pkg_name.clear();
                        url = None;
                        github = None;
                        is_path = false;
                        current_value.clear();
                        dep_type = "";
                        has_branch_or_revision = false;
                    } else {
                        // indent==2: new sibling package
                        flush(
                            &pkg_name,
                            &url,
                            &github,
                            is_path,
                            &current_value,
                            dep_type,
                            has_branch_or_revision,
                            &mut deps,
                        );
                        pkg_name.clear();
                        url = None;
                        github = None;
                        is_path = false;
                        current_value.clear();
                        dep_type = "";
                        has_branch_or_revision = false;

                        if let Some(colon) = trimmed.find(':') {
                            pkg_name = trimmed[..colon].trim().to_owned();
                        }
                        // stay in InPackageEntry
                    }
                } else if let Some(cap) = KV.captures(line) {
                    let key = &cap[1];
                    let val = cap[2].trim().trim_matches('"').to_owned();

                    match key {
                        "url" => url = Some(val),
                        "github" => github = Some(val),
                        "path" => is_path = true,
                        "branch" | "revision" => has_branch_or_revision = true,
                        "from" if current_value.is_empty() => {
                            current_value = val;
                            dep_type = "from";
                        }
                        "exactVersion" => {
                            current_value = val;
                            dep_type = "exactVersion";
                        }
                        "version" if (current_value.is_empty() || dep_type == "from") => {
                            current_value = val;
                            dep_type = "version";
                        }
                        "majorVersion" | "minorVersion" | "minVersion" | "maxVersion"
                            if current_value.is_empty() =>
                        {
                            current_value = val;
                            dep_type = "from";
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Flush final entry
    flush(
        &pkg_name,
        &url,
        &github,
        is_path,
        &current_value,
        dep_type,
        has_branch_or_revision,
        &mut deps,
    );

    deps
}

fn build_source(url: &Option<String>, github: &Option<String>) -> Option<XcodeGenSource> {
    if let Some(gh) = github {
        return Some(XcodeGenSource::GitHub(gh.clone()));
    }
    if let Some(u) = url {
        let cleaned = u.trim_end_matches(".git");
        if cleaned.starts_with("https://github.com/") || cleaned.starts_with("http://github.com/") {
            let repo = cleaned
                .trim_start_matches("https://github.com/")
                .trim_start_matches("http://github.com/");
            return Some(XcodeGenSource::GitHub(repo.to_owned()));
        }
        if cleaned.starts_with("https://gitlab.com/") || cleaned.starts_with("http://gitlab.com/") {
            let repo = cleaned
                .trim_start_matches("https://gitlab.com/")
                .trim_start_matches("http://gitlab.com/");
            return Some(XcodeGenSource::GitLab(repo.to_owned()));
        }
        return Some(XcodeGenSource::Git(u.clone()));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_github_url_with_from() {
        let content = r#"
packages:
  Alamofire:
    url: https://github.com/Alamofire/Alamofire.git
    from: "5.4.3"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let d = &deps[0];
        assert_eq!(d.name, "Alamofire");
        assert_eq!(
            d.source,
            Some(XcodeGenSource::GitHub("Alamofire/Alamofire".to_owned()))
        );
        assert_eq!(d.current_value, "5.4.3");
        assert!(d.skip_reason.is_none());
    }

    #[test]
    fn extracts_github_shorthand() {
        let content = r#"
packages:
  Charts:
    github: danielgindi/Charts
    exactVersion: "4.1.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].source,
            Some(XcodeGenSource::GitHub("danielgindi/Charts".to_owned()))
        );
        assert_eq!(deps[0].current_value, "4.1.0");
    }

    #[test]
    fn local_path_skipped() {
        let content = r#"
packages:
  LocalLib:
    path: ../LocalLib
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(XcodeGenSkipReason::LocalPath));
    }

    #[test]
    fn branch_only_skipped() {
        let content = r#"
packages:
  MyLib:
    url: https://github.com/owner/repo.git
    branch: main
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(XcodeGenSkipReason::NoSemverVersion)
        );
    }

    #[test]
    fn multiple_packages() {
        let content = r#"
packages:
  Alamofire:
    url: https://github.com/Alamofire/Alamofire.git
    from: "5.4.3"
  SnapKit:
    github: SnapKit/SnapKit
    version: "5.6.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].name, "Alamofire");
        assert_eq!(deps[1].name, "SnapKit");
    }

    #[test]
    fn gitlab_url_detected() {
        let content = r#"
packages:
  MyPkg:
    url: https://gitlab.com/owner/repo.git
    from: "1.0.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].source,
            Some(XcodeGenSource::GitLab("owner/repo".to_owned()))
        );
    }

    #[test]
    fn no_packages_returns_empty() {
        // Ported: "returns null for YAML without packages" — xcodegen/extract.spec.ts line 22
        assert!(extract("name: MyApp\ntargets: {}").is_empty());
    }

    #[test]
    fn empty_content_returns_empty() {
        // Ported: "returns null for empty content" — xcodegen/extract.spec.ts line 7
        assert!(extract("").is_empty());
    }

    #[test]
    fn empty_packages_section_returns_empty() {
        // Ported: "returns null for empty packages" — xcodegen/extract.spec.ts line 36
        let content = "name: MyProject\npackages: {}\n";
        assert!(extract(content).is_empty());
    }

    #[test]
    fn revision_reference_skipped() {
        // Ported: "skips packages with revision reference" — xcodegen/extract.spec.ts line 233
        let content = "packages:\n  MyLib:\n    url: https://github.com/owner/repo.git\n    revision: abc123\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(XcodeGenSkipReason::NoSemverVersion)
        );
    }

    #[test]
    fn package_without_url_or_github_skipped() {
        // Ported: "skips packages without url or github" — xcodegen/extract.spec.ts line 356
        let content = "packages:\n  MyLib:\n    from: \"1.0.0\"\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(XcodeGenSkipReason::MissingSource));
    }
}
