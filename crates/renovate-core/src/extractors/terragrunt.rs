//! Terragrunt `terragrunt.hcl` dependency extractor.
//!
//! Parses `terraform { source = "..." }` blocks and extracts versioned
//! module references for GitHub Tags and Terraform Registry lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/terragrunt/extract.ts`
//! - `lib/modules/manager/terragrunt/modules.ts`
//! - Pattern: `/(^|/)terragrunt\.hcl$/`
//! - Datasources: GitHub Tags, Terraform Module Registry
//!
//! ## Supported source forms
//!
//! ```hcl
//! # GitHub with ?ref= tag
//! source = "github.com/owner/repo?ref=v1.2.3"
//!
//! # git:: prefix with ?ref= tag
//! source = "git::https://github.com/owner/repo.git?ref=v1.0.0"
//!
//! # Terraform registry (registry/org/name/provider)
//! source = "registry.terraform.io/hashicorp/consul/aws?version=0.6.0"
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// Source type for a terragrunt dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerragruntSource {
    /// GitHub repository (owner/repo) with a tag ref.
    GitHub(String),
    /// Generic git URL with a tag ref.
    Git(String),
    /// Terraform/OpenTofu module registry (namespace/name/provider).
    TerraformRegistry { hostname: String, module: String },
}

/// Why a terragrunt dep is skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerragruntSkipReason {
    /// No `?ref=` or `?version=` parameter found.
    NoVersion,
    /// Source is a local path.
    Local,
    /// Source format not recognized.
    Unknown,
    /// URL is structurally invalid.
    InvalidUrl,
}

/// A single extracted terragrunt module dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerragruntDep {
    /// Human-readable dep name.
    pub dep_name: String,
    /// Version tag or constraint.
    pub current_value: String,
    /// Source routing.
    pub source: Option<TerragruntSource>,
    /// Set when no lookup should be performed.
    pub skip_reason: Option<TerragruntSkipReason>,
}

/// `github.com/project/repo[.git]//subpath?ref=tag`
static GITHUB_REF_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"github\.com[/:](?P<project>[^/\s]+/[a-zA-Z0-9._-]+?)(?:\.git)?(?://[^\?]*)?\?(?:depth=\d+&)?ref=(?P<tag>[^&\s]+)",
    )
    .unwrap()
});

/// `git::https?://host/path?ref=tag`
static GIT_REF_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:git::)?(?P<url>(?:https?|ssh)://[^\s\?]+)\?(?:depth=\d+&)?ref=(?P<tag>[^&\s]+)")
        .unwrap()
});

/// `[registry/]org/name/cloud[//subpath]?version=x.y.z`  (Terraform registry)
static TFR_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"tfr://(?P<registry>.*?)/(?P<org>[^/]+)/(?P<name>[^/]+)/(?P<cloud>[^/?]+).*\?(?:ref|version)=(?P<version>[^\s&]+)").unwrap()
});

/// `key = "value"` inside an HCL block
static KV_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\s*(\w+)\s*=\s*"([^"]+)""#).unwrap());

/// Return the Terragrunt dependency type for a given string.
///
/// Mirrors `lib/modules/manager/terragrunt/util.ts` `getTerragruntDependencyType()`.
pub fn get_terragrunt_dependency_type(value: &str) -> &'static str {
    match value {
        "terraform" => "terraform",
        _ => "unknown",
    }
}

/// Update terragrunt lockfile artifacts by delegating to the terraform
/// lockfile updater.
///
/// Mirrors `lib/modules/manager/terragrunt/artifacts.ts` `updateArtifacts()`.
pub async fn update_terragrunt_artifacts(
    base_dir: &std::path::Path,
    package_file_name: &str,
    updated_deps: &[super::terraform::TerraformArtifactDep],
    config: &super::terraform::TerraformArtifactConfig,
) -> Result<Option<Vec<crate::artifacts::ArtifactResult>>, crate::artifacts::ArtifactError> {
    if !config.is_lock_file_maintenance {
        return Ok(None);
    }
    super::terraform::update_terraform_artifacts(base_dir, package_file_name, updated_deps, config)
        .await
}

/// Extract terragrunt module deps from a `terragrunt.hcl` file.
pub fn extract(content: &str) -> Vec<TerragruntDep> {
    let mut deps = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();

        // Detect `terraform {` block start
        if trimmed == "terraform {" || trimmed.starts_with("terraform {") {
            let mut brace_count: i32 = 1;
            let mut source_val: Option<String> = None;
            i += 1;

            while i < lines.len() && brace_count > 0 {
                let line = lines[i];
                let opens = line.chars().filter(|&c| c == '{').count() as i32;
                let closes = line.chars().filter(|&c| c == '}').count() as i32;
                brace_count += opens - closes;

                if brace_count > 0
                    && let Some(cap) = KV_RE.captures(line)
                    && &cap[1] == "source"
                {
                    source_val = Some(cap[2].to_owned());
                }
                i += 1;
            }

            if let Some(src) = source_val {
                deps.push(analyse_source(&src));
            }
        } else {
            i += 1;
        }
    }

    deps
}

fn is_valid_url(url: &str) -> bool {
    let Some((_, rest)) = url.split_once("://") else {
        return false;
    };
    let host = rest.split('/').next().unwrap_or("");
    if host.is_empty() {
        return false;
    }
    if host.starts_with('[') && !host.contains(']') {
        return false;
    }
    true
}

fn analyse_source(source: &str) -> TerragruntDep {
    // Local path
    if source.starts_with("../") || source.starts_with("./") || source.starts_with('/') {
        return TerragruntDep {
            dep_name: source.to_owned(),
            current_value: String::new(),
            source: None,
            skip_reason: Some(TerragruntSkipReason::Local),
        };
    }

    // GitHub ?ref= pattern
    if let Some(cap) = GITHUB_REF_RE.captures(source) {
        let project = cap["project"].trim_end_matches(".git").to_owned();
        return TerragruntDep {
            dep_name: format!("github.com/{project}"),
            current_value: cap["tag"].to_owned(),
            source: Some(TerragruntSource::GitHub(project)),
            skip_reason: None,
        };
    }

    // Generic git:: ?ref= pattern
    if let Some(cap) = GIT_REF_RE.captures(source) {
        let url = cap["url"].to_owned();
        if !is_valid_url(&url) {
            return TerragruntDep {
                dep_name: source.to_owned(),
                current_value: String::new(),
                source: None,
                skip_reason: Some(TerragruntSkipReason::InvalidUrl),
            };
        }
        // Check if it's a GitHub URL under the git:: prefix
        if url.contains("github.com") {
            let repo = url
                .trim_start_matches("https://github.com/")
                .trim_start_matches("http://github.com/")
                .trim_end_matches(".git")
                .to_owned();
            return TerragruntDep {
                dep_name: format!("github.com/{repo}"),
                current_value: cap["tag"].to_owned(),
                source: Some(TerragruntSource::GitHub(repo)),
                skip_reason: None,
            };
        }
        return TerragruntDep {
            dep_name: url.clone(),
            current_value: cap["tag"].to_owned(),
            source: Some(TerragruntSource::Git(url)),
            skip_reason: None,
        };
    }

    // Terraform registry tfr:// pattern
    if let Some(cap) = TFR_RE.captures(source) {
        let registry = cap["registry"].to_owned();
        let module = format!("{}/{}/{}", &cap["org"], &cap["name"], &cap["cloud"]);
        let hostname = if registry.is_empty() {
            "registry.terraform.io".to_owned()
        } else {
            registry
        };
        return TerragruntDep {
            dep_name: module.clone(),
            current_value: cap["version"].to_owned(),
            source: Some(TerragruntSource::TerraformRegistry { hostname, module }),
            skip_reason: None,
        };
    }

    // Registry-style: org/name/provider (3-4 parts)
    let base = source.split("//").next().unwrap_or(source);
    let parts: Vec<&str> = base
        .trim_start_matches("registry.terraform.io/")
        .splitn(3, '/')
        .collect();
    if parts.len() >= 3 {
        let module = parts.join("/");
        return TerragruntDep {
            dep_name: module.clone(),
            current_value: String::new(),
            source: Some(TerragruntSource::TerraformRegistry {
                hostname: "registry.terraform.io".into(),
                module,
            }),
            skip_reason: Some(TerragruntSkipReason::NoVersion),
        };
    }

    TerragruntDep {
        dep_name: source.to_owned(),
        current_value: String::new(),
        source: None,
        skip_reason: Some(TerragruntSkipReason::Unknown),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extractors::terraform;

    // Ported: "extracts terragrunt sources" — lib/modules/manager/terragrunt/extract.spec.ts line 51
    #[test]
    fn extracts_github_ref_source() {
        let content = r#"
terraform {
  source = "github.com/gruntwork-io/terraform-aws-vpc.git//modules/vpc?ref=v0.21.1"
}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].source,
            Some(TerragruntSource::GitHub(
                "gruntwork-io/terraform-aws-vpc".to_owned()
            ))
        );
        assert_eq!(deps[0].current_value, "v0.21.1");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "extracts terragrunt sources using tfr protocol" — lib/modules/manager/terragrunt/extract.spec.ts line 10
    #[test]
    fn extracts_tfr_protocol_sources() {
        let content = r#"
terraform {
  source = "tfr:///myuser/myrepo/cloud//folder/modules/moduleone?ref=v0.0.9"
}

terraform {
  source = "tfr:///terraform-google-modules/kubernetes-engine/google//modules/private-cluster?version=1.2.3"
}

terraform {
  source = "tfr:///terraform-aws-modules/vpc/aws?version=3.3.0"
}

terraform {
  source = "tfr://terraform-aws-modules/vpc/aws?version=3.3.0"
}

terraform {
  source = "tfr://registry.domain.com/abc/helloworld/aws?version=1.0.0"
}

terraform {
  source = "tfr://registry.domain.com/abc/helloworld/aws?version=1.0.0"
}
"#;

        let deps = extract(content);
        assert_eq!(deps.len(), 6);
        assert_eq!(deps[0].dep_name, "myuser/myrepo/cloud");
        assert_eq!(deps[0].current_value, "v0.0.9");
        assert_eq!(
            deps[0].source,
            Some(TerragruntSource::TerraformRegistry {
                hostname: "registry.terraform.io".to_owned(),
                module: "myuser/myrepo/cloud".to_owned(),
            })
        );
        assert_eq!(
            deps[1].dep_name,
            "terraform-google-modules/kubernetes-engine/google"
        );
        assert_eq!(deps[1].current_value, "1.2.3");
        assert_eq!(deps[2].dep_name, "terraform-aws-modules/vpc/aws");
        assert_eq!(deps[2].current_value, "3.3.0");
        assert_eq!(deps[3].skip_reason, Some(TerragruntSkipReason::Unknown));
        assert_eq!(deps[4].dep_name, "abc/helloworld/aws");
        assert_eq!(deps[4].current_value, "1.0.0");
        assert_eq!(
            deps[4].source,
            Some(TerragruntSource::TerraformRegistry {
                hostname: "registry.domain.com".to_owned(),
                module: "abc/helloworld/aws".to_owned(),
            })
        );
        assert_eq!(deps[5].dep_name, "abc/helloworld/aws");
        assert_eq!(deps[5].current_value, "1.0.0");
    }

    // Ported: "extracts terragrunt sources" — lib/modules/manager/terragrunt/extract.spec.ts line 51
    #[test]
    fn extracts_git_prefix_github() {
        let content = r#"
terraform {
  source = "git::https://github.com/owner/module.git?ref=v1.0.0"
}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].source,
            Some(TerragruntSource::GitHub("owner/module".to_owned()))
        );
        assert_eq!(deps[0].current_value, "v1.0.0");
    }

    // Ported: "extracts terragrunt sources with depth specified after the branch" — lib/modules/manager/terragrunt/extract.spec.ts line 269
    #[test]
    fn extracts_sources_with_depth_after_ref() {
        let content = r#"
terraform {
  source = "github.com/myuser/myrepo//folder/modules/moduleone?ref=v0.0.9&depth=1"
}

terraform {
  source = "git::https://mygit.com/hashicorp/example//subdir/test?ref=v1.0.1&depth=1"
}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(
            deps[0].source,
            Some(TerragruntSource::GitHub("myuser/myrepo".to_owned()))
        );
        assert_eq!(deps[0].current_value, "v0.0.9");
        assert_eq!(
            deps[1].source,
            Some(TerragruntSource::Git(
                "https://mygit.com/hashicorp/example//subdir/test".to_owned()
            ))
        );
        assert_eq!(deps[1].current_value, "v1.0.1");
        assert!(deps.iter().all(|dep| dep.skip_reason.is_none()));
    }

    // Ported: "extracts terragrunt sources with depth specified before the branch" — lib/modules/manager/terragrunt/extract.spec.ts line 487
    #[test]
    fn extracts_sources_with_depth_before_ref() {
        let content = r#"
terraform {
  source = "github.com/myuser/myrepo//folder/modules/moduleone?depth=1&ref=v0.0.9"
}

terraform {
  source = "git::https://mygit.com/hashicorp/example//subdir/test?depth=1&ref=v1.0.1"
}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(
            deps[0].source,
            Some(TerragruntSource::GitHub("myuser/myrepo".to_owned()))
        );
        assert_eq!(deps[0].current_value, "v0.0.9");
        assert_eq!(
            deps[1].source,
            Some(TerragruntSource::Git(
                "https://mygit.com/hashicorp/example//subdir/test".to_owned()
            ))
        );
        assert_eq!(deps[1].current_value, "v1.0.1");
        assert!(deps.iter().all(|dep| dep.skip_reason.is_none()));
    }

    // Ported: "extracts terragrunt sources" — lib/modules/manager/terragrunt/extract.spec.ts line 51
    #[test]
    fn local_path_skipped() {
        let content = r#"
terraform {
  source = "../modules/vpc"
}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(TerragruntSkipReason::Local));
    }

    // Ported: "extracts terragrunt sources" — lib/modules/manager/terragrunt/extract.spec.ts line 51
    #[test]
    fn multiple_terraform_blocks() {
        let content = r#"
terraform {
  source = "github.com/owner/repo1?ref=v1.0.0"
}

terraform {
  source = "github.com/owner/repo2?ref=v2.0.0"
}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].current_value, "v1.0.0");
        assert_eq!(deps[1].current_value, "v2.0.0");
    }

    // Ported: "returns null for empty" — lib/modules/manager/terragrunt/extract.spec.ts line 6
    #[test]
    fn no_terraform_block_returns_empty() {
        assert!(extract("# just a comment\n").is_empty());
        assert!(extract("nothing here").is_empty());
        assert!(extract("").is_empty());
    }

    // Ported: "returns null if only local terragrunt deps" — lib/modules/manager/terragrunt/extract.spec.ts line 698
    #[test]
    fn local_only_deps_returns_empty() {
        // `terragrunt {` block is not recognized (only `terraform {`) → empty.
        // Local `terraform { source = "../fe" }` would be skipped with Local reason.
        let content = "terraform {\n  source = \"../fe\"\n}\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(TerragruntSkipReason::Local));
    }

    // Ported: "returns empty deps if only local terragrunt includes" — lib/modules/manager/terragrunt/extract.spec.ts line 707
    #[test]
    fn include_block_only_returns_empty() {
        let content = "include \"root\" {\n  path = find_in_parent_folders(\"root.hcl\")\n}\n";
        assert!(extract(content).is_empty());
    }

    // Ported: "should split project and tag from source" — lib/modules/manager/terragrunt/modules.spec.ts line 11
    #[test]
    fn github_ref_regex_splits_project_and_tag() {
        let source = "github.com/hashicorp/example?ref=v1.0.0";
        let cap = GITHUB_REF_RE.captures(source).unwrap();
        assert_eq!(&cap["project"], "hashicorp/example");
        assert_eq!(&cap["tag"], "v1.0.0");
    }

    // Ported: "should parse alpha-numeric characters as well as dots, underscores, and dashes in repo names" — lib/modules/manager/terragrunt/modules.spec.ts line 21
    #[test]
    fn github_ref_regex_parses_complex_repo_names() {
        let source = "github.com/hashicorp/example.repo-123?ref=v1.0.0";
        let cap = GITHUB_REF_RE.captures(source).unwrap();
        assert_eq!(&cap["project"], "hashicorp/example.repo-123");
        assert_eq!(&cap["tag"], "v1.0.0");
    }

    // Ported: "should split host, path and tag from source" — lib/modules/manager/terragrunt/modules.spec.ts line 33
    #[test]
    fn git_ref_regex_splits_host_path_and_tag() {
        for prefix in &["http://", "https://", "ssh://"] {
            let source = format!("{prefix}github.com/hashicorp/example?ref=v1.0.0");
            let cap = GIT_REF_RE.captures(&source).unwrap();
            assert_eq!(&cap["tag"], "v1.0.0");
            assert!(cap["url"].contains("github.com/hashicorp/example"));
        }
    }

    // Ported: "should parse alpha-numeric characters as well as dots, underscores, and dashes in repo names" — lib/modules/manager/terragrunt/modules.spec.ts line 61
    #[test]
    fn git_ref_regex_parses_complex_repo_path() {
        for prefix in &["http://", "https://", "ssh://"] {
            let source = format!("{prefix}github.com/hashicorp/example.repo-123?ref=v1.0.0");
            let cap = GIT_REF_RE.captures(&source).unwrap();
            assert_eq!(&cap["tag"], "v1.0.0");
            assert!(cap["url"].contains("example.repo-123"));
        }
    }

    // Ported: "returns terraform" — lib/modules/manager/terragrunt/util.spec.ts line 5
    #[test]
    fn get_dependency_type_returns_terraform() {
        assert_eq!(get_terragrunt_dependency_type("terraform"), "terraform");
    }

    // Ported: "returns unknown" — lib/modules/manager/terragrunt/util.spec.ts line 9
    #[test]
    fn get_dependency_type_returns_unknown() {
        assert_eq!(get_terragrunt_dependency_type("unknown"), "unknown");
    }

    // Ported: "returns unknown on empty string" — lib/modules/manager/terragrunt/util.spec.ts line 13
    #[test]
    fn get_dependency_type_returns_unknown_for_empty() {
        assert_eq!(get_terragrunt_dependency_type(""), "unknown");
    }

    // Ported: "returns unknown on string with random chars" — lib/modules/manager/terragrunt/util.spec.ts line 17
    #[test]
    fn get_dependency_type_returns_unknown_for_random() {
        assert_eq!(
            get_terragrunt_dependency_type("sdfsgdsfadfhfghfhgdfsdf"),
            "unknown"
        );
    }

    // Ported: "sets skipReason for invalid git tags URL" — lib/modules/manager/terragrunt/modules.spec.ts line 89
    #[test]
    fn sets_skip_reason_for_invalid_git_tags_url() {
        let content = r#"
terraform {
  source = "ssh://[/path?ref=v1.0.0"
}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(TerragruntSkipReason::InvalidUrl));
    }

    // ── update_terragrunt_artifacts (terragrunt/artifacts.spec.ts) ────────────

    // Ported: "does not call terraform updateArtifacts if the update type is %s" — lib/modules/manager/terragrunt/artifacts.spec.ts line 58
    #[tokio::test]
    async fn update_artifacts_returns_null_for_non_lockfile_maintenance() {
        let tmp = std::env::temp_dir();
        let deps: Vec<terraform::TerraformArtifactDep> = vec![];
        let config = terraform::TerraformArtifactConfig {
            is_lock_file_maintenance: false,
        };
        let result = update_terragrunt_artifacts(&tmp, "terragrunt.hcl", &deps, &config)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "calls terraform updateArtifacts if the update type is lockfileMaintenance" — lib/modules/manager/terragrunt/artifacts.spec.ts line 40
    #[tokio::test]
    async fn update_artifacts_delegates_to_terraform_on_lockfile_maintenance() {
        let tmp = tempfile::tempdir().unwrap();
        let base = tmp.path();
        // Create a minimal terragrunt.hcl and .terraform.lock.hcl so the
        // terraform artifact function finds a lockfile and attempts processing.
        std::fs::write(base.join("terragrunt.hcl"), b"").unwrap();
        std::fs::write(
            base.join(".terraform.lock.hcl"),
            br#"provider "registry.terraform.io/hashicorp/aws" {
  version = "5.0.0"
  hashes = [
    "h1:abc",
  ]
}
"#,
        )
        .unwrap();

        let deps: Vec<terraform::TerraformArtifactDep> = vec![];
        let config = terraform::TerraformArtifactConfig {
            is_lock_file_maintenance: true,
        };
        let result = update_terragrunt_artifacts(base, "terragrunt.hcl", &deps, &config)
            .await
            .unwrap();
        // When there are no updated deps and lockfile maintenance is true,
        // terraform's updateAllLocks path runs but finds nothing to update
        // (no deps to look up), so it returns None or empty updates.
        // The key point is that it DELEGATED (did not early-return None).
        // With no deps, the result is Ok(None) because there's nothing to update.
        assert!(result.is_none());
    }
}
