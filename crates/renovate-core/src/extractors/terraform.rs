//! Terraform `.tf` / `.tofu` dependency extractor.
//!
//! Parses HCL-formatted Terraform files with a brace-depth state machine and
//! returns provider and module dependencies ready for Terraform Registry
//! version lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/terraform/index.ts` — `defaultConfig`
//! - `lib/modules/manager/terraform/extractors/terraform-block/required-provider.ts`
//! - `lib/modules/manager/terraform/extractors/others/modules.ts`
//!
//! ## Supported declarations
//!
//! | Form | Treatment |
//! |---|---|
//! | `required_providers { aws = { source = "hashicorp/aws", version = "~> 5.0" } }` | Provider dep |
//! | `module "vpc" { source = "terraform-aws-modules/vpc/aws", version = "~> 5.0" }` | Module dep |
//! | Module source without `version` | Skipped — `NoVersionConstraint` |
//! | Module with git/http source | Skipped — `ExternalSource` |
//!
//! ## Limitations
//!
//! Uses a line-oriented scanner — does not handle HCL string interpolation,
//! multi-line values, or heredocs. Covers the common single-file patterns
//! that most Terraform projects use.

use std::sync::LazyLock;

use regex::Regex;

/// Dep type: whether this came from `required_providers` or a `module` block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerraformDepType {
    /// Declared in `terraform { required_providers { … } }`.
    Provider,
    /// Declared in `module "name" { … }`.
    Module,
}

impl TerraformDepType {
    pub fn as_renovate_str(&self) -> &'static str {
        match self {
            TerraformDepType::Provider => "provider",
            TerraformDepType::Module => "module",
        }
    }
}


/// Why a Terraform dep is skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TerraformSkipReason {
    /// Module has no `version` field.
    NoVersionConstraint,
    /// Module source is a git URL, HTTPS URL, or local path.
    ExternalSource,
}

/// A single extracted Terraform dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerraformExtractedDep {
    /// Dep name:
    /// - Providers: `{namespace}/{type}` (e.g. `hashicorp/aws`) or bare type.
    /// - Modules: `{namespace}/{name}/{provider}` (e.g. `terraform-aws-modules/vpc/aws`).
    pub name: String,
    /// Version constraint (e.g. `~> 5.0`). Empty = unconstrained.
    pub current_value: String,
    /// Which block this dep came from.
    pub dep_type: TerraformDepType,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<TerraformSkipReason>,
}

// ── Compiled regexes ──────────────────────────────────────────────────────────

/// `key = "value"` or `key = value` — captures key and value.
static KV_LINE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r##"^\s*(\w+)\s*=\s*"?([^"#\n]+?)"?\s*(?:#.*)?$"##).unwrap());

/// `module "name" {` — captures module name.
static MODULE_BLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\s*module\s+"([^"]+)"\s*\{"#).unwrap());

/// `terraform {` block.
static TERRAFORM_BLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*terraform\s*\{").unwrap());

/// `required_providers {` inside a terraform block.
static REQUIRED_PROVIDERS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*required_providers\s*\{").unwrap());

/// Provider name assignment: `<name> = {` or `<name> = version_string`.
static PROVIDER_ENTRY_BLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\s*(\w+)\s*=\s*\{"#).unwrap());

static PROVIDER_ENTRY_STRING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\s*(\w+)\s*=\s*"([^"]+)""#).unwrap());

// ── State machine ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
enum State {
    TopLevel,
    InTerraformBlock,
    InRequiredProviders,
    InProviderEntry,
    InModuleBlock,
    Skip(usize), // skip other blocks, depth counter
}

struct Parser {
    state: State,
    deps: Vec<TerraformExtractedDep>,
    // Pending provider being assembled.
    prov_name: String,
    prov_source: String,
    prov_version: String,
    // Pending module being assembled.
    mod_name: String,
    mod_source: String,
    mod_version: String,
}

impl Parser {
    fn new() -> Self {
        Self {
            state: State::TopLevel,
            deps: Vec::new(),
            prov_name: String::new(),
            prov_source: String::new(),
            prov_version: String::new(),
            mod_name: String::new(),
            mod_source: String::new(),
            mod_version: String::new(),
        }
    }

    fn process_line(&mut self, line: &str) {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with("//") {
            return;
        }

        match &self.state.clone() {
            State::TopLevel => self.handle_top_level(trimmed),
            State::InTerraformBlock => self.handle_terraform_block(trimmed),
            State::InRequiredProviders => self.handle_required_providers(trimmed),
            State::InProviderEntry => self.handle_provider_entry(trimmed),
            State::InModuleBlock => self.handle_module_block(trimmed),
            State::Skip(depth) => self.handle_skip(*depth, trimmed),
        }
    }

    fn handle_top_level(&mut self, trimmed: &str) {
        if TERRAFORM_BLOCK.is_match(trimmed) {
            self.state = State::InTerraformBlock;
        } else if let Some(cap) = MODULE_BLOCK.captures(trimmed) {
            self.mod_name = cap[1].to_owned();
            self.mod_source.clear();
            self.mod_version.clear();
            self.state = State::InModuleBlock;
        } else if trimmed.ends_with('{') {
            // Unknown top-level block — skip.
            self.state = State::Skip(1);
        }
    }

    fn handle_terraform_block(&mut self, trimmed: &str) {
        if REQUIRED_PROVIDERS.is_match(trimmed) {
            self.state = State::InRequiredProviders;
        } else if trimmed == "}" {
            self.state = State::TopLevel;
        } else if trimmed.ends_with('{') {
            // Other nested block inside terraform {}.
            self.state = State::Skip(1);
        }
    }

    fn handle_required_providers(&mut self, trimmed: &str) {
        if trimmed == "}" {
            self.state = State::InTerraformBlock;
            return;
        }
        // Provider with block form: `aws = {`
        if let Some(cap) = PROVIDER_ENTRY_BLOCK.captures(trimmed) {
            self.prov_name = cap[1].to_owned();
            self.prov_source.clear();
            self.prov_version.clear();
            self.state = State::InProviderEntry;
            return;
        }
        // Provider with inline string form: `aws = "~> 5.0"`
        if let Some(cap) = PROVIDER_ENTRY_STRING.captures(trimmed) {
            let name = cap[1].to_owned();
            let version = cap[2].to_owned();
            self.deps.push(TerraformExtractedDep {
                name,
                current_value: version,
                dep_type: TerraformDepType::Provider,
                skip_reason: None,
            });
        }
    }

    fn handle_provider_entry(&mut self, trimmed: &str) {
        if trimmed == "}" {
            // Emit the provider dep.
            let name = if self.prov_source.is_empty() {
                self.prov_name.clone()
            } else {
                self.prov_source.clone()
            };
            self.deps.push(TerraformExtractedDep {
                name,
                current_value: self.prov_version.clone(),
                dep_type: TerraformDepType::Provider,
                skip_reason: None,
            });
            self.state = State::InRequiredProviders;
            return;
        }
        if let Some(cap) = KV_LINE.captures(trimmed) {
            let key = &cap[1];
            let val = cap[2].trim().to_owned();
            match key {
                "source" => self.prov_source = val,
                "version" => self.prov_version = val,
                _ => {}
            }
        }
    }

    fn handle_module_block(&mut self, trimmed: &str) {
        if trimmed == "}" {
            let source = self.mod_source.clone();
            let version = self.mod_version.clone();
            let skip_reason = classify_module_source(&source, &version);
            self.deps.push(TerraformExtractedDep {
                name: source,
                current_value: version,
                dep_type: TerraformDepType::Module,
                skip_reason,
            });
            self.state = State::TopLevel;
            return;
        }
        if trimmed.ends_with('{') {
            // Nested block inside module (e.g. providers map) — skip.
            self.state = State::Skip(1);
            return;
        }
        if let Some(cap) = KV_LINE.captures(trimmed) {
            let key = &cap[1];
            let val = cap[2].trim().to_owned();
            match key {
                "source" => self.mod_source = val,
                "version" => self.mod_version = val,
                _ => {}
            }
        }
    }

    fn handle_skip(&mut self, depth: usize, trimmed: &str) {
        let opens = trimmed.chars().filter(|&c| c == '{').count();
        let closes = trimmed.chars().filter(|&c| c == '}').count();
        let new_depth = depth.saturating_add(opens).saturating_sub(closes);
        if new_depth == 0 {
            // Restored to the previous state.
            // We only enter Skip from InModuleBlock or top-level; restore accordingly.
            // Since we don't track the return state, use depth=0 as a heuristic:
            // if we entered from InModuleBlock, we stay there (no nested Skip from top).
            // The simple heuristic: if depth was 1 from top-level, go back TopLevel;
            // otherwise stay InModuleBlock. We track via the prov_name being set.
            self.state = if !self.mod_name.is_empty() && self.mod_source.is_empty() {
                State::InModuleBlock
            } else {
                State::TopLevel
            };
        } else {
            self.state = State::Skip(new_depth);
        }
    }
}

/// Classify a module source string.
fn classify_module_source(source: &str, version: &str) -> Option<TerraformSkipReason> {
    if source.is_empty() {
        return None;
    }
    // Local paths
    if source.starts_with("./") || source.starts_with("../") || source.starts_with('/') {
        return Some(TerraformSkipReason::ExternalSource);
    }
    // Remote URLs (git, https, etc.)
    if source.contains("://") || source.starts_with("git@") {
        return Some(TerraformSkipReason::ExternalSource);
    }
    // Registry modules must have a version; skip those that don't.
    if version.is_empty() {
        return Some(TerraformSkipReason::NoVersionConstraint);
    }
    None
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Parse a Terraform `.tf` file and extract all provider and module deps.
pub fn extract(content: &str) -> Vec<TerraformExtractedDep> {
    let mut parser = Parser::new();
    for line in content.lines() {
        parser.process_line(line);
    }
    parser.deps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_providers_block_form() {
        let content = r#"
terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = ">= 2.0.0"
    }
  }
}
"#;
        let deps = extract(content);
        let providers: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == TerraformDepType::Provider)
            .collect();
        assert_eq!(providers.len(), 2);

        let aws = providers
            .iter()
            .find(|d| d.name == "hashicorp/aws")
            .unwrap();
        assert_eq!(aws.current_value, "~> 5.0");
        assert!(aws.skip_reason.is_none());

        let k8s = providers
            .iter()
            .find(|d| d.name == "hashicorp/kubernetes")
            .unwrap();
        assert_eq!(k8s.current_value, ">= 2.0.0");
    }

    #[test]
    fn required_providers_inline_string_form() {
        let content = r#"
terraform {
  required_providers {
    google = ">= 4.0.0"
  }
}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "google");
        assert_eq!(deps[0].current_value, ">= 4.0.0");
    }

    #[test]
    fn module_with_version() {
        let content = r#"
module "vpc" {
  source  = "terraform-aws-modules/vpc/aws"
  version = "~> 5.0"
}
"#;
        let deps = extract(content);
        let modules: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == TerraformDepType::Module)
            .collect();
        assert_eq!(modules.len(), 1);
        assert_eq!(modules[0].name, "terraform-aws-modules/vpc/aws");
        assert_eq!(modules[0].current_value, "~> 5.0");
        assert!(modules[0].skip_reason.is_none());
    }

    #[test]
    fn module_without_version_skipped() {
        let content = r#"
module "vpc" {
  source = "terraform-aws-modules/vpc/aws"
}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(TerraformSkipReason::NoVersionConstraint)
        );
    }

    #[test]
    fn module_with_local_path_skipped() {
        let content = r#"
module "local" {
  source  = "./modules/mymodule"
  version = "1.0.0"
}
"#;
        let deps = extract(content);
        assert_eq!(
            deps[0].skip_reason,
            Some(TerraformSkipReason::ExternalSource)
        );
    }

    #[test]
    fn module_with_git_source_skipped() {
        let content = r#"
module "git_module" {
  source  = "git::https://github.com/org/repo.git"
  version = "1.0.0"
}
"#;
        let deps = extract(content);
        assert_eq!(
            deps[0].skip_reason,
            Some(TerraformSkipReason::ExternalSource)
        );
    }

    #[test]
    fn mixed_providers_and_modules() {
        let content = r#"
terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

module "eks" {
  source  = "terraform-aws-modules/eks/aws"
  version = "~> 20.0"
}

module "local_mod" {
  source = "./modules/local"
}
"#;
        let deps = extract(content);
        assert_eq!(
            deps.iter()
                .filter(|d| d.dep_type == TerraformDepType::Provider)
                .count(),
            1
        );
        let modules: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == TerraformDepType::Module)
            .collect();
        assert_eq!(modules.len(), 2);

        let eks = modules
            .iter()
            .find(|d| d.name == "terraform-aws-modules/eks/aws")
            .unwrap();
        assert_eq!(eks.current_value, "~> 20.0");
        assert!(eks.skip_reason.is_none());

        let local = modules
            .iter()
            .find(|d| d.name == "./modules/local")
            .unwrap();
        assert_eq!(local.skip_reason, Some(TerraformSkipReason::ExternalSource));
    }

    #[test]
    fn comments_ignored() {
        let content = r#"
# This is a terraform file
terraform {
  # required_providers block
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0" # latest stable
    }
  }
}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "hashicorp/aws");
    }

    #[test]
    fn empty_file_returns_empty() {
        assert!(extract("").is_empty());
    }

    #[test]
    fn provider_without_source_uses_name() {
        let content = r#"
terraform {
  required_providers {
    random = {
      version = "~> 3.0"
    }
  }
}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "random");
        assert_eq!(deps[0].current_value, "~> 3.0");
    }
}
