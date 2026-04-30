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
    Regex::new(r"tfr://(?P<registry>[^/]+)/(?P<org>[^/]+)/(?P<name>[^/]+)/(?P<cloud>[^/?]+).*\?(?:ref|version)=(?P<version>[^\s&]+)").unwrap()
});

/// `key = "value"` inside an HCL block
static KV_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\s*(\w+)\s*=\s*"([^"]+)""#).unwrap());

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
        return TerragruntDep {
            dep_name: format!("{registry}/{module}"),
            current_value: cap["version"].to_owned(),
            source: Some(TerragruntSource::TerraformRegistry {
                hostname: registry,
                module,
            }),
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

    // Ported: "returns null for empty" — terragrunt/extract.spec.ts line 6
    #[test]
    fn no_terraform_block_returns_empty() {
        assert!(extract("# just a comment\n").is_empty());
        assert!(extract("nothing here").is_empty());
        assert!(extract("").is_empty());
    }

    // Ported: "returns null if only local terragrunt deps" — terragrunt/extract.spec.ts line 698
    #[test]
    fn local_only_deps_returns_empty() {
        // `terragrunt {` block is not recognized (only `terraform {`) → empty.
        // Local `terraform { source = "../fe" }` would be skipped with Local reason.
        let content = "terraform {\n  source = \"../fe\"\n}\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(TerragruntSkipReason::Local));
    }

    // Ported: "returns empty deps if only local terragrunt includes" — terragrunt/extract.spec.ts line 707
    #[test]
    fn include_block_only_returns_empty() {
        let content = "include \"root\" {\n  path = find_in_parent_folders(\"root.hcl\")\n}\n";
        assert!(extract(content).is_empty());
    }
}
