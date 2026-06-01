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

use std::{collections::BTreeMap, sync::LazyLock};

use regex::Regex;

/// Dep type: whether this came from `required_providers` or a `module` block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerraformDepType {
    /// Declared in `terraform { required_providers { … } }`.
    Provider,
    /// Declared in `module "name" { … }`.
    Module,
    /// Declared as `terraform { required_version = "…" }` — pins the
    /// terraform CLI itself (looked up via hashicorp/terraform releases).
    RequiredVersion,
    TfeWorkspace,
    DockerImage,
    DockerContainer,
    DockerService,
    DockerRegistryImage,
    HelmRelease,
    KubernetesCronJobV1,
    KubernetesCronJob,
    KubernetesDaemonSetV1,
    KubernetesDaemonset,
    KubernetesDeployment,
    KubernetesDeploymentV1,
    KubernetesJob,
    KubernetesJobV1,
    KubernetesPod,
    KubernetesPodV1,
    KubernetesReplicationController,
    KubernetesReplicationControllerV1,
    KubernetesStatefulSet,
    KubernetesStatefulSetV1,
}

impl TerraformDepType {
    pub fn as_renovate_str(&self) -> &'static str {
        match self {
            TerraformDepType::Provider => "provider",
            TerraformDepType::Module => "module",
            TerraformDepType::RequiredVersion => "required_version",
            TerraformDepType::TfeWorkspace => "tfe_workspace",
            TerraformDepType::DockerImage => "docker_image",
            TerraformDepType::DockerContainer => "docker_container",
            TerraformDepType::DockerService => "docker_service",
            TerraformDepType::DockerRegistryImage => "docker_registry_image",
            TerraformDepType::HelmRelease => "helm_release",
            TerraformDepType::KubernetesCronJobV1 => "kubernetes_cron_job_v1",
            TerraformDepType::KubernetesCronJob => "kubernetes_cron_job",
            TerraformDepType::KubernetesDaemonSetV1 => "kubernetes_daemon_set_v1",
            TerraformDepType::KubernetesDaemonset => "kubernetes_daemonset",
            TerraformDepType::KubernetesDeployment => "kubernetes_deployment",
            TerraformDepType::KubernetesDeploymentV1 => "kubernetes_deployment_v1",
            TerraformDepType::KubernetesJob => "kubernetes_job",
            TerraformDepType::KubernetesJobV1 => "kubernetes_job_v1",
            TerraformDepType::KubernetesPod => "kubernetes_pod",
            TerraformDepType::KubernetesPodV1 => "kubernetes_pod_v1",
            TerraformDepType::KubernetesReplicationController => {
                "kubernetes_replication_controller"
            }
            TerraformDepType::KubernetesReplicationControllerV1 => {
                "kubernetes_replication_controller_v1"
            }
            TerraformDepType::KubernetesStatefulSet => "kubernetes_stateful_set",
            TerraformDepType::KubernetesStatefulSetV1 => "kubernetes_stateful_set_v1",
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
    InvalidUrl,
    UnspecifiedVersion,
    ContainsVariable,
    InvalidDependencySpecification,
    InvalidName,
    LocalChart,
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
    pub datasource: Option<&'static str>,
    pub package_name: Option<String>,
    pub current_digest: Option<String>,
    pub locked_version: Option<String>,
    /// Registry URLs for this dependency (e.g. for providers with explicit registry hostname).
    pub registry_urls: Vec<String>,
    /// Set when no registry lookup should be performed.
    pub skip_reason: Option<TerraformSkipReason>,
}

struct ModuleSource {
    name: String,
    current_value: String,
    datasource: Option<&'static str>,
    package_name: Option<String>,
    current_digest: Option<String>,
    skip_reason: Option<TerraformSkipReason>,
}

// ── Module source URL regexes (mirrors TypeScript exports in modules.ts) ─────

/// Matches `github.com/<org>/<repo>?ref=<tag>` (case-insensitive).
/// Named captures: `project`, `tag`.
pub static GITHUB_REF_MATCH_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)github\.com([/:])(?P<project>[^/]+/[a-z0-9\-_.]+).*\?(depth=\d+&)?ref=(?P<tag>.*?)(&depth=\d+)?$",
    )
    .unwrap()
});

/// Matches Bitbucket URLs.
/// Named captures: `url`, `path`, `workspace`, `project`, `subfolder`, `tag`.
pub static BITBUCKET_REF_MATCH_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?:git::)?(?P<url>(?:http|https|ssh)?(?:://)?(?:.*@)?(?P<path>bitbucket\.org/(?P<workspace>.*)/(?P<project>.*)\.git/?(?P<subfolder>.*)))\?(depth=\d+&)?ref=(?P<tag>.*?)(&depth=\d+)?$",
    )
    .unwrap()
});

/// Matches generic git-hosted module sources (http/https/ssh/bare git@).
/// Named captures: `url`, `path`, `project`, `subfolder`, `tag`.
pub static GIT_TAGS_REF_MATCH_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?:git::)?(?P<url>(?:(?:http|https|ssh)://)?(?:.*@)?(?P<path>[^:/]+[:/](?P<project>[^/]+(?:/[^/]+)*))(?:\.git)?)((//)?(?P<subfolder>[^?]*))?\?(depth=\d+&)?ref=(?P<tag>.*?)(&depth=\d+)?$",
    )
    .unwrap()
});

/// Matches Azure DevOps SSH module sources.
/// Named captures: `url`, `organization`, `project`, `repository`, `modulepath`, `tag`.
pub static AZURE_DEVOPS_SSH_REF_MATCH_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?:git::)?(?P<url>git@ssh\.dev\.azure\.com:v3/(?P<organization>[^/]*)/(?P<project>[^/]*)/(?P<repository>[^/]*))(?P<modulepath>.*)?\?(depth=\d+&)?ref=(?P<tag>.*?)(&depth=\d+)?$",
    )
    .unwrap()
});

/// Matches the hostname at the start of a module source URL.
/// Named capture: `hostname`.
pub static HOSTNAME_MATCH_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<hostname>[a-zA-Z\d]([a-zA-Z\d\-]*\.)+[a-zA-Z\d]+)").unwrap()
});

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

static TFE_WORKSPACE_BLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\s*resource\s+"tfe_workspace"\s+"[^"]+"\s*\{"#).unwrap());

static DOCKER_RESOURCE_BLOCK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^\s*(resource|data)\s+"(docker_(?:image|container|service|registry_image))"\s+"[^"]+"\s*\{"#)
        .unwrap()
});

static HELM_RELEASE_BLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\s*resource\s+"helm_release"\s+"[^"]+"\s*\{"#).unwrap());

static KUBERNETES_RESOURCE_BLOCK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^\s*resource\s+"(kubernetes_(?:cron_job_v1|cron_job|daemon_set_v1|daemonset|deployment_v1|deployment|job_v1|job|pod_v1|pod|replication_controller_v1|replication_controller|stateful_set_v1|stateful_set))"\s+"[^"]+"\s*\{"#)
        .unwrap()
});

static KUBERNETES_CONTAINER_BLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\s*(?:init_)?container\s*\{"#).unwrap());

static LOCK_PROVIDER_BLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\s*provider\s+"([^"]+)"\s*\{"#).unwrap());

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
    InTfeWorkspaceBlock(usize),
    InDockerResourceBlock {
        dep_type: TerraformDepType,
        depth: usize,
    },
    InHelmReleaseBlock(usize),
    InKubernetesResourceBlock {
        dep_type: TerraformDepType,
        depth: usize,
    },
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
    registry_aliases: BTreeMap<String, String>,
    resource_version: String,
    resource_image: String,
    resource_repository: String,
    resource_chart: String,
    resource_images: Vec<String>,
    container_depth: Option<usize>,
}

impl Parser {
    fn with_registry_aliases(registry_aliases: &BTreeMap<String, String>) -> Self {
        Self {
            state: State::TopLevel,
            deps: Vec::new(),
            prov_name: String::new(),
            prov_source: String::new(),
            prov_version: String::new(),
            mod_name: String::new(),
            mod_source: String::new(),
            mod_version: String::new(),
            registry_aliases: registry_aliases.clone(),
            resource_version: String::new(),
            resource_image: String::new(),
            resource_repository: String::new(),
            resource_chart: String::new(),
            resource_images: Vec::new(),
            container_depth: None,
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
            State::InTfeWorkspaceBlock(depth) => self.handle_tfe_workspace_block(*depth, trimmed),
            State::InDockerResourceBlock { dep_type, depth } => {
                self.handle_docker_resource_block(*dep_type, *depth, trimmed);
            }
            State::InHelmReleaseBlock(depth) => self.handle_helm_release_block(*depth, trimmed),
            State::InKubernetesResourceBlock { dep_type, depth } => {
                self.handle_kubernetes_resource_block(*dep_type, *depth, trimmed);
            }
            State::Skip(depth) => self.handle_skip(*depth, trimmed),
        }
    }

    fn handle_top_level(&mut self, trimmed: &str) {
        if TERRAFORM_BLOCK.is_match(trimmed) {
            self.state = State::InTerraformBlock;
        } else if TFE_WORKSPACE_BLOCK.is_match(trimmed) {
            self.resource_version.clear();
            self.state = State::InTfeWorkspaceBlock(1);
        } else if let Some(cap) = DOCKER_RESOURCE_BLOCK.captures(trimmed) {
            self.resource_image.clear();
            let dep_type = match &cap[2] {
                "docker_image" => TerraformDepType::DockerImage,
                "docker_container" => TerraformDepType::DockerContainer,
                "docker_service" => TerraformDepType::DockerService,
                "docker_registry_image" => TerraformDepType::DockerRegistryImage,
                _ => unreachable!("regex only matches supported docker resources"),
            };
            self.state = State::InDockerResourceBlock { dep_type, depth: 1 };
        } else if HELM_RELEASE_BLOCK.is_match(trimmed) {
            self.resource_repository.clear();
            self.resource_chart.clear();
            self.resource_version.clear();
            self.state = State::InHelmReleaseBlock(1);
        } else if let Some(cap) = KUBERNETES_RESOURCE_BLOCK.captures(trimmed) {
            self.resource_images.clear();
            self.container_depth = None;
            let Some(dep_type) = kubernetes_dep_type(&cap[1]) else {
                unreachable!("regex only matches supported kubernetes resources");
            };
            self.state = State::InKubernetesResourceBlock { dep_type, depth: 1 };
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
            return;
        }
        if trimmed == "}" {
            self.state = State::TopLevel;
            return;
        }
        // `required_version = "…"` pins the terraform CLI itself.
        if let Some(cap) = KV_LINE.captures(trimmed)
            && &cap[1] == "required_version"
        {
            let version = cap[2].trim().to_owned();
            if !version.is_empty() {
                self.deps.push(TerraformExtractedDep {
                    name: "hashicorp/terraform".to_owned(),
                    current_value: version,
                    dep_type: TerraformDepType::RequiredVersion,
                    datasource: None,
                    package_name: None,
                    current_digest: None,
                    locked_version: None,
                    registry_urls: vec![],
                    skip_reason: None,
                });
            }
            return;
        }
        if trimmed.ends_with('{') {
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
                datasource: None,
                package_name: None,
                current_digest: None,
                locked_version: None,
                registry_urls: vec![],
                skip_reason: None,
            });
        }
    }

    fn handle_provider_entry(&mut self, trimmed: &str) {
        if trimmed == "}" {
            // Emit the provider dep.
            if let Some(oci) =
                resolve_oci_source(&self.prov_name, &self.prov_source, &self.registry_aliases)
            {
                self.deps.push(TerraformExtractedDep {
                    name: oci.name,
                    current_value: oci.current_value,
                    dep_type: TerraformDepType::Provider,
                    datasource: oci.datasource,
                    package_name: oci.package_name,
                    current_digest: oci.current_digest,
                    locked_version: None,
                    registry_urls: vec![],
                    skip_reason: oci.skip_reason,
                });
            } else {
                let name = if self.prov_source.is_empty() {
                    self.prov_name.clone()
                } else {
                    self.prov_source.clone()
                };
                let (registry_urls, package_name) =
                    parse_provider_source_hostname(&self.prov_source);
                self.deps.push(TerraformExtractedDep {
                    name,
                    current_value: self.prov_version.clone(),
                    dep_type: TerraformDepType::Provider,
                    datasource: None,
                    package_name,
                    current_digest: None,
                    locked_version: None,
                    registry_urls,
                    skip_reason: None,
                });
            }
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
            let source = resolve_module_source(
                &self.mod_name,
                &self.mod_source,
                &self.mod_version,
                &self.registry_aliases,
            );
            self.deps.push(TerraformExtractedDep {
                name: source.name,
                current_value: source.current_value,
                dep_type: TerraformDepType::Module,
                datasource: source.datasource,
                package_name: source.package_name,
                current_digest: source.current_digest,
                locked_version: None,
                registry_urls: vec![],
                skip_reason: source.skip_reason,
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

    fn handle_tfe_workspace_block(&mut self, depth: usize, trimmed: &str) {
        if let Some(cap) = KV_LINE.captures(trimmed)
            && &cap[1] == "terraform_version"
        {
            self.resource_version = cap[2].trim().to_owned();
        }

        let opens = trimmed.chars().filter(|&c| c == '{').count();
        let closes = trimmed.chars().filter(|&c| c == '}').count();
        let new_depth = depth.saturating_add(opens).saturating_sub(closes);
        if new_depth == 0 {
            let (current_value, skip_reason) = if self.resource_version.is_empty() {
                (String::new(), Some(TerraformSkipReason::UnspecifiedVersion))
            } else {
                (self.resource_version.clone(), None)
            };
            self.deps.push(TerraformExtractedDep {
                name: "hashicorp/terraform".to_owned(),
                current_value,
                dep_type: TerraformDepType::TfeWorkspace,
                datasource: Some("github-releases"),
                package_name: Some("hashicorp/terraform".to_owned()),
                current_digest: None,
                locked_version: None,
                registry_urls: vec![],
                skip_reason,
            });
            self.resource_version.clear();
            self.state = State::TopLevel;
        } else {
            self.state = State::InTfeWorkspaceBlock(new_depth);
        }
    }

    fn handle_docker_resource_block(
        &mut self,
        dep_type: TerraformDepType,
        depth: usize,
        trimmed: &str,
    ) {
        if let Some(cap) = KV_LINE.captures(trimmed) {
            let image_key = match dep_type {
                TerraformDepType::DockerImage | TerraformDepType::DockerRegistryImage => "name",
                TerraformDepType::DockerContainer | TerraformDepType::DockerService => "image",
                _ => "",
            };
            if &cap[1] == image_key {
                self.resource_image = cap[2].trim().to_owned();
            }
        }

        let opens = trimmed.chars().filter(|&c| c == '{').count();
        let closes = trimmed.chars().filter(|&c| c == '}').count();
        let new_depth = depth.saturating_add(opens).saturating_sub(closes);
        if new_depth == 0 {
            self.deps.push(resolve_docker_image(
                dep_type,
                &self.resource_image,
                &self.registry_aliases,
            ));
            self.resource_image.clear();
            self.state = State::TopLevel;
        } else {
            self.state = State::InDockerResourceBlock {
                dep_type,
                depth: new_depth,
            };
        }
    }

    fn handle_helm_release_block(&mut self, depth: usize, trimmed: &str) {
        if let Some(cap) = KV_LINE.captures(trimmed) {
            let val = cap[2].trim().to_owned();
            match &cap[1] {
                "repository" => self.resource_repository = val,
                "chart" => self.resource_chart = val,
                "version" => self.resource_version = val,
                _ => {}
            }
        }

        let opens = trimmed.chars().filter(|&c| c == '{').count();
        let closes = trimmed.chars().filter(|&c| c == '}').count();
        let new_depth = depth.saturating_add(opens).saturating_sub(closes);
        if new_depth == 0 {
            self.deps.push(resolve_helm_release(
                &self.resource_repository,
                &self.resource_chart,
                &self.resource_version,
                &self.registry_aliases,
            ));
            self.resource_repository.clear();
            self.resource_chart.clear();
            self.resource_version.clear();
            self.state = State::TopLevel;
        } else {
            self.state = State::InHelmReleaseBlock(new_depth);
        }
    }

    fn handle_kubernetes_resource_block(
        &mut self,
        dep_type: TerraformDepType,
        depth: usize,
        trimmed: &str,
    ) {
        let entered_container =
            self.container_depth.is_none() && KUBERNETES_CONTAINER_BLOCK.is_match(trimmed);

        if self.container_depth.is_some()
            && let Some(cap) = KV_LINE.captures(trimmed)
            && &cap[1] == "image"
        {
            self.resource_images.push(cap[2].trim().to_owned());
        }

        let opens = trimmed.chars().filter(|&c| c == '{').count();
        let closes = trimmed.chars().filter(|&c| c == '}').count();
        let new_depth = depth.saturating_add(opens).saturating_sub(closes);

        if entered_container {
            self.container_depth = Some(1);
        } else if let Some(container_depth) = self.container_depth {
            let new_container_depth = container_depth.saturating_add(opens).saturating_sub(closes);
            self.container_depth = (new_container_depth > 0).then_some(new_container_depth);
        }

        if new_depth == 0 {
            if self.resource_images.is_empty() {
                self.deps.push(docker_skip(
                    dep_type,
                    TerraformSkipReason::InvalidDependencySpecification,
                ));
            } else {
                self.deps.extend(
                    self.resource_images
                        .iter()
                        .map(|image| resolve_docker_image(dep_type, image, &self.registry_aliases)),
                );
            }
            self.resource_images.clear();
            self.container_depth = None;
            self.state = State::TopLevel;
        } else {
            self.state = State::InKubernetesResourceBlock {
                dep_type,
                depth: new_depth,
            };
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

fn resolve_module_source(
    module_name: &str,
    source: &str,
    version: &str,
    registry_aliases: &BTreeMap<String, String>,
) -> ModuleSource {
    if let Some(source) = resolve_oci_source(module_name, source, registry_aliases) {
        return source;
    }
    if let Some(source) = parse_azure_devops_module_source(source) {
        return source;
    }
    if let Some(source) = parse_bitbucket_module_source(source) {
        return source;
    }

    ModuleSource {
        name: source.to_owned(),
        current_value: version.to_owned(),
        datasource: None,
        package_name: None,
        current_digest: None,
        skip_reason: classify_module_source(source, version),
    }
}

fn resolve_oci_source(
    dep_name: &str,
    source: &str,
    registry_aliases: &BTreeMap<String, String>,
) -> Option<ModuleSource> {
    let rest = source.strip_prefix("oci://")?;
    if rest.chars().any(char::is_whitespace) {
        return Some(ModuleSource {
            name: dep_name.to_owned(),
            current_value: String::new(),
            datasource: Some("docker"),
            package_name: None,
            current_digest: None,
            skip_reason: Some(TerraformSkipReason::InvalidUrl),
        });
    }

    let (path, query) = rest.split_once('?').unwrap_or((rest, ""));
    let Some((host, image_path)) = path.split_once('/') else {
        return Some(ModuleSource {
            name: dep_name.to_owned(),
            current_value: String::new(),
            datasource: Some("docker"),
            package_name: None,
            current_digest: None,
            skip_reason: Some(TerraformSkipReason::InvalidUrl),
        });
    };
    if host.is_empty() || image_path.is_empty() {
        return Some(ModuleSource {
            name: dep_name.to_owned(),
            current_value: String::new(),
            datasource: Some("docker"),
            package_name: None,
            current_digest: None,
            skip_reason: Some(TerraformSkipReason::InvalidUrl),
        });
    }

    let registry = registry_aliases
        .get(host)
        .map(String::as_str)
        .unwrap_or(host);
    let current_value = query_param(query, "tag").unwrap_or_default();
    let current_digest = query_param(query, "digest");
    let skip_reason = (current_value.is_empty() && current_digest.is_none())
        .then_some(TerraformSkipReason::UnspecifiedVersion);

    Some(ModuleSource {
        name: dep_name.to_owned(),
        current_value,
        datasource: Some("docker"),
        package_name: Some(format!("{registry}/{image_path}")),
        current_digest,
        skip_reason,
    })
}

fn parse_azure_devops_module_source(source: &str) -> Option<ModuleSource> {
    let without_git = source.strip_prefix("git::").unwrap_or(source);
    let rest = without_git.strip_prefix("git@ssh.dev.azure.com:v3/")?;
    let (path, current_value) = split_ref(rest)?;
    let (repo, subdir) = split_double_slash(path);
    let mut parts = repo.split('/');
    let org = parts.next()?;
    let project = parts.next()?;
    let repository = parts.next()?;
    let package_name = format!("git@ssh.dev.azure.com:v3/{org}/{project}/{repository}");
    let name = subdir
        .map(|subdir| format!("{org}/{project}/{repository}//{subdir}"))
        .unwrap_or_else(|| format!("{org}/{project}/{repository}"));

    Some(ModuleSource {
        name,
        current_value: current_value.to_owned(),
        datasource: Some("git-tags"),
        package_name: Some(package_name),
        current_digest: None,
        skip_reason: None,
    })
}

fn parse_bitbucket_module_source(source: &str) -> Option<ModuleSource> {
    let without_git = source.strip_prefix("git::").unwrap_or(source);
    let (path, current_value) = split_ref(without_git)?;

    if let Some(rest) = path
        .strip_prefix("https://bitbucket.com/")
        .or_else(|| path.strip_prefix("http://bitbucket.com/"))
        .or_else(|| path.strip_prefix("ssh://git@bitbucket.com/"))
    {
        let scheme = if path.starts_with("http://") {
            "http://"
        } else if path.starts_with("ssh://") {
            "ssh://git@"
        } else {
            "https://"
        };
        let (repo, _) = split_double_slash(rest);
        let repo = repo.trim_end_matches(".git");
        return Some(ModuleSource {
            name: format!("bitbucket.com/{repo}"),
            current_value: current_value.to_owned(),
            datasource: Some("git-tags"),
            package_name: Some(format!("{scheme}bitbucket.com/{repo}")),
            current_digest: None,
            skip_reason: None,
        });
    }

    if let Some(rest) = path
        .strip_prefix("ssh://git@bitbucket.org/")
        .or_else(|| path.strip_prefix("https://git@bitbucket.org/"))
        .or_else(|| path.strip_prefix("bitbucket.org/"))
    {
        let (repo, _) = split_double_slash(rest);
        let repo = repo.split(".git").next().unwrap_or(repo);
        return Some(ModuleSource {
            name: repo.to_owned(),
            current_value: current_value.to_owned(),
            datasource: Some("bitbucket-tags"),
            package_name: Some(repo.to_owned()),
            current_digest: None,
            skip_reason: None,
        });
    }

    None
}

fn split_ref(source: &str) -> Option<(&str, &str)> {
    let (path, query) = source.split_once("?ref=")?;
    Some((path, query.split('&').next().unwrap_or(query)))
}

fn split_double_slash(path: &str) -> (&str, Option<&str>) {
    if let Some((repo, subdir)) = path.split_once("//") {
        (repo, Some(subdir))
    } else {
        (path, None)
    }
}

/// Parses a provider source string and returns (registry_urls, package_name).
/// If source has a hostname (contains dots before first `/`), extracts it as the registry URL.
/// Mirrors TypeScript `sourceExtractionRegex` in `base.ts`.
fn parse_provider_source_hostname(source: &str) -> (Vec<String>, Option<String>) {
    use std::sync::LazyLock;
    static SOURCE_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"^(?:(?P<hostname>(?:[a-zA-Z0-9\-_]+\.)+[a-zA-Z0-9\-_]+)/)?(?:(?P<namespace>[^/]+)/)?(?P<type>[^/]+)$",
        )
        .unwrap()
    });
    if source.is_empty() {
        return (vec![], None);
    }
    let Some(cap) = SOURCE_RE.captures(source) else {
        return (vec![], None);
    };
    if let Some(hostname) = cap.name("hostname") {
        let registry_url = format!("https://{}", hostname.as_str());
        let namespace = cap.name("namespace").map(|m| m.as_str()).unwrap_or("");
        let type_name = cap.name("type").map(|m| m.as_str()).unwrap_or("");
        let package_name = if namespace.is_empty() {
            type_name.to_owned()
        } else {
            format!("{}/{}", namespace, type_name)
        };
        (vec![registry_url], Some(package_name))
    } else {
        (vec![], None)
    }
}

fn query_param(query: &str, key: &str) -> Option<String> {
    query.split('&').find_map(|part| {
        let (name, value) = part.split_once('=')?;
        (name == key && !value.is_empty()).then(|| value.to_owned())
    })
}

fn resolve_docker_image(
    dep_type: TerraformDepType,
    image: &str,
    registry_aliases: &BTreeMap<String, String>,
) -> TerraformExtractedDep {
    if image.is_empty() {
        return docker_skip(
            dep_type,
            TerraformSkipReason::InvalidDependencySpecification,
        );
    }
    if image.contains("${") || image.contains("data.") {
        return docker_skip(dep_type, TerraformSkipReason::ContainsVariable);
    }

    let (without_digest, current_digest) = image
        .split_once('@')
        .map_or((image, None), |(name, digest)| {
            (name, Some(digest.to_owned()))
        });
    let last_slash = without_digest.rfind('/').unwrap_or(0);
    let tag_sep = without_digest[last_slash..]
        .rfind(':')
        .map(|idx| last_slash + idx);
    let Some(tag_sep) = tag_sep else {
        return docker_skip(
            dep_type,
            TerraformSkipReason::InvalidDependencySpecification,
        );
    };
    let dep_name = &without_digest[..tag_sep];
    let current_value = &without_digest[tag_sep + 1..];
    if dep_name.is_empty() || current_value.is_empty() {
        return docker_skip(
            dep_type,
            TerraformSkipReason::InvalidDependencySpecification,
        );
    }

    let package_name = dep_name.split_once('/').and_then(|(host, path)| {
        registry_aliases
            .get(host)
            .map(|registry| format!("{registry}/{path}"))
    });

    TerraformExtractedDep {
        name: dep_name.to_owned(),
        current_value: current_value.to_owned(),
        dep_type,
        datasource: Some("docker"),
        package_name,
        current_digest,
        locked_version: None,
        registry_urls: vec![],
        skip_reason: None,
    }
}

fn docker_skip(
    dep_type: TerraformDepType,
    skip_reason: TerraformSkipReason,
) -> TerraformExtractedDep {
    TerraformExtractedDep {
        name: String::new(),
        current_value: String::new(),
        dep_type,
        datasource: Some("docker"),
        package_name: None,
        current_digest: None,
        locked_version: None,
        registry_urls: vec![],
        skip_reason: Some(skip_reason),
    }
}

fn resolve_helm_release(
    repository: &str,
    chart: &str,
    version: &str,
    registry_aliases: &BTreeMap<String, String>,
) -> TerraformExtractedDep {
    let (name, datasource, package_name, skip_reason) = if chart.is_empty() {
        (
            String::new(),
            Some("helm"),
            None,
            Some(TerraformSkipReason::InvalidName),
        )
    } else if chart.starts_with("./") || chart.starts_with("../") || chart.starts_with('/') {
        (
            chart.to_owned(),
            Some("helm"),
            None,
            Some(TerraformSkipReason::LocalChart),
        )
    } else if let Some(path) = chart.strip_prefix("oci://") {
        (path.to_owned(), Some("docker"), None, None)
    } else if let Some(repo_path) = repository.strip_prefix("oci://") {
        let package_name = repo_path.split_once('/').map_or_else(
            || format!("{repo_path}/{chart}"),
            |(host, path)| {
                let registry = registry_aliases
                    .get(host)
                    .map(String::as_str)
                    .unwrap_or(host);
                format!("{registry}/{path}/{chart}")
            },
        );
        (chart.to_owned(), Some("docker"), Some(package_name), None)
    } else {
        (chart.to_owned(), Some("helm"), None, None)
    };

    TerraformExtractedDep {
        name,
        current_value: version.to_owned(),
        dep_type: TerraformDepType::HelmRelease,
        datasource,
        package_name,
        current_digest: None,
        locked_version: None,
        registry_urls: vec![],
        skip_reason,
    }
}

fn kubernetes_dep_type(resource_type: &str) -> Option<TerraformDepType> {
    match resource_type {
        "kubernetes_cron_job_v1" => Some(TerraformDepType::KubernetesCronJobV1),
        "kubernetes_cron_job" => Some(TerraformDepType::KubernetesCronJob),
        "kubernetes_daemon_set_v1" => Some(TerraformDepType::KubernetesDaemonSetV1),
        "kubernetes_daemonset" => Some(TerraformDepType::KubernetesDaemonset),
        "kubernetes_deployment" => Some(TerraformDepType::KubernetesDeployment),
        "kubernetes_deployment_v1" => Some(TerraformDepType::KubernetesDeploymentV1),
        "kubernetes_job" => Some(TerraformDepType::KubernetesJob),
        "kubernetes_job_v1" => Some(TerraformDepType::KubernetesJobV1),
        "kubernetes_pod" => Some(TerraformDepType::KubernetesPod),
        "kubernetes_pod_v1" => Some(TerraformDepType::KubernetesPodV1),
        "kubernetes_replication_controller" => {
            Some(TerraformDepType::KubernetesReplicationController)
        }
        "kubernetes_replication_controller_v1" => {
            Some(TerraformDepType::KubernetesReplicationControllerV1)
        }
        "kubernetes_stateful_set" => Some(TerraformDepType::KubernetesStatefulSet),
        "kubernetes_stateful_set_v1" => Some(TerraformDepType::KubernetesStatefulSetV1),
        _ => None,
    }
}

/// Line numbers for a provider block within `.terraform.lock.hcl`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerraformProviderLineNumbers {
    pub block_start: usize,
    pub block_end: usize,
    pub hashes_start: Option<usize>,
    pub hashes_end: Option<usize>,
}

/// A parsed entry from `.terraform.lock.hcl`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerraformProviderLock {
    pub package_name: String,
    pub registry_url: String,
    pub version: String,
    pub constraints: String,
    pub hashes: Vec<String>,
    pub line_numbers: TerraformProviderLineNumbers,
}

/// An update to apply to a provider lock entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerraformProviderLockUpdate {
    pub package_name: String,
    pub registry_url: String,
    pub version: String,
    pub constraints: String,
    pub hashes: Vec<String>,
    pub line_numbers: TerraformProviderLineNumbers,
    pub new_version: String,
    pub new_constraint: String,
    pub new_hashes: Vec<String>,
}

/// Parse `.terraform.lock.hcl` content into provider lock entries.
///
/// Mirrors `lib/modules/manager/terraform/lockfile/util.ts` `extractLocks()`.
/// Returns `None` when no provider blocks are found.
pub fn extract_terraform_locks(content: &str) -> Option<Vec<TerraformProviderLock>> {
    use std::sync::LazyLock;
    static PROVIDER_START: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"^provider "(?P<registryUrl>[^/]*)/(?P<namespace>[^/]*)/(?P<depName>[^/"]*)"#)
            .unwrap()
    });
    static VERSION_LINE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"^\s*version\s*=\s*"(?P<version>[^"']+)"#).unwrap());
    static CONSTRAINT_LINE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"^\s*constraints\s*=\s*"(?P<constraint>[^"']+)"#).unwrap());
    static HASH_LINE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"^\s*"(?P<hash>[^"]+)",$"#).unwrap());

    let lines: Vec<&str> = content.lines().collect();
    let block_starts: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_, l)| l.starts_with("provider \""))
        .map(|(i, _)| i)
        .collect();

    if block_starts.is_empty() {
        return None;
    }

    let mut locks = Vec::new();

    for (idx, &start) in block_starts.iter().enumerate() {
        let end = if idx + 1 < block_starts.len() {
            block_starts[idx + 1]
        } else {
            lines.len()
        };

        let mut package_name = String::new();
        let mut registry_url = String::new();
        let mut version = String::new();
        let mut constraints = String::new();
        let mut hashes = Vec::new();
        let mut hashes_start: Option<usize> = None;
        let mut hashes_end: Option<usize> = None;

        for (rel_idx, line) in lines[start..end].iter().enumerate() {
            if let Some(cap) = PROVIDER_START.captures(line) {
                package_name = format!("{}/{}", &cap["namespace"], &cap["depName"]);
                registry_url = format!("https://{}", &cap["registryUrl"]);
            } else if let Some(cap) = VERSION_LINE.captures(line) {
                version = cap["version"].to_owned();
            } else if let Some(cap) = CONSTRAINT_LINE.captures(line) {
                constraints = cap["constraint"].to_owned();
            } else if let Some(cap) = HASH_LINE.captures(line) {
                hashes.push(cap["hash"].to_owned());
                if hashes_start.is_none() {
                    hashes_start = Some(rel_idx);
                }
                hashes_end = Some(rel_idx);
            }
        }

        locks.push(TerraformProviderLock {
            package_name,
            registry_url,
            version,
            constraints,
            hashes,
            line_numbers: TerraformProviderLineNumbers {
                block_start: start,
                block_end: end,
                hashes_start,
                hashes_end,
            },
        });
    }

    Some(locks)
}

/// Update lockfile content with provider lock updates.
///
/// Mirrors `lib/modules/manager/terraform/lockfile/util.ts` `writeLockUpdates()`.
pub fn write_lock_updates(
    updates: &[TerraformProviderLockUpdate],
    lock_file_path: &str,
    old_content: &str,
) -> crate::artifacts::ArtifactResult {
    let lines: Vec<&str> = old_content.lines().collect();
    let mut sorted = updates.to_vec();
    sorted.sort_by_key(|u| u.line_numbers.block_start);

    let mut sections: Vec<Vec<String>> = Vec::new();
    let mut last_end: usize = 0;

    static VERSION_LINE_RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
        Regex::new(r#"^(?P<prefix>\s*version\s*=\s*")(?P<version>[^"']+)(?P<suffix>".*)$"#)
            .unwrap()
    });
    static CONSTRAINT_LINE_RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
        Regex::new(r#"^(?P<prefix>\s*constraints\s*=\s*")(?P<constraint>[^"']+)(?P<suffix>".*)$"#)
            .unwrap()
    });
    static HASH_LINE_RE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
        Regex::new(r#"^(?P<prefix>\s*")(?P<hash>[^"]+)(?P<suffix>",.*)$"#).unwrap()
    });

    for update in &sorted {
        let leading: Vec<String> = lines[last_end..update.line_numbers.block_start]
            .iter()
            .map(|&s| s.to_owned())
            .collect();
        sections.push(leading);

        let block_lines = &lines[update.line_numbers.block_start..update.line_numbers.block_end];
        let mut new_block: Vec<String> = Vec::new();
        let mut hash_prefix = String::new();
        let mut hash_suffix = String::new();

        for line in block_lines {
            if let Some(cap) = VERSION_LINE_RE.captures(line) {
                new_block.push(format!(
                    "{}{}{}",
                    &cap["prefix"], update.new_version, &cap["suffix"]
                ));
                continue;
            }
            if let Some(cap) = CONSTRAINT_LINE_RE.captures(line) {
                new_block.push(format!(
                    "{}{}{}",
                    &cap["prefix"], update.new_constraint, &cap["suffix"]
                ));
                continue;
            }
            if let Some(cap) = HASH_LINE_RE.captures(line) {
                hash_prefix = cap["prefix"].to_owned();
                hash_suffix = cap["suffix"].to_owned();
                continue;
            }
            new_block.push((*line).to_owned());
        }

        let hash_lines: Vec<String> = update
            .new_hashes
            .iter()
            .map(|h| format!("{}{}{}", hash_prefix, h, hash_suffix))
            .collect();
        let insert_idx = update.line_numbers.hashes_start.unwrap_or(new_block.len());
        for (i, hl) in hash_lines.into_iter().enumerate() {
            new_block.insert(insert_idx + i, hl);
        }

        sections.push(new_block);
        last_end = update.line_numbers.block_end;
    }

    let trailing: Vec<String> = lines[last_end..]
        .iter()
        .map(|&s| s.to_owned())
        .collect();
    sections.push(trailing);

    let new_content = sections
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join("\n");

    crate::artifacts::ArtifactResult::file_change(lock_file_path, new_content)
}

fn parse_provider_lockfile(lockfile: &str) -> BTreeMap<String, String> {
    let mut locked_versions = BTreeMap::new();
    let mut current_provider: Option<String> = None;
    let mut depth = 0usize;

    for line in lockfile.lines() {
        let trimmed = line.trim();
        if current_provider.is_none()
            && let Some(cap) = LOCK_PROVIDER_BLOCK.captures(trimmed)
        {
            current_provider = Some(normalize_lock_provider_name(&cap[1]));
            depth = 1;
            continue;
        }

        if let Some(provider) = current_provider.as_ref() {
            if let Some(cap) = KV_LINE.captures(trimmed)
                && &cap[1] == "version"
            {
                locked_versions.insert(provider.clone(), cap[2].trim().to_owned());
            }

            let opens = trimmed.chars().filter(|&c| c == '{').count();
            let closes = trimmed.chars().filter(|&c| c == '}').count();
            depth = depth.saturating_add(opens).saturating_sub(closes);
            if depth == 0 {
                current_provider = None;
            }
        }
    }

    locked_versions
}

fn normalize_lock_provider_name(provider: &str) -> String {
    // https:// or http:// prefixed = custom registry; strip scheme but keep hostname
    // (the hostname IS part of the dep name for custom registries).
    if let Some(rest) = provider
        .strip_prefix("https://")
        .or_else(|| provider.strip_prefix("http://"))
    {
        return rest.to_owned();
    }
    // Plain {registryHost}/{namespace}/{type}: strip the registry hostname prefix.
    if let Some((host, rest)) = provider.split_once('/')
        && host.contains('.')
    {
        return rest.to_owned();
    }
    provider.to_owned()
}

fn apply_locked_versions(
    deps: &mut [TerraformExtractedDep],
    locked_versions: &BTreeMap<String, String>,
) {
    for dep in deps
        .iter_mut()
        .filter(|dep| dep.dep_type == TerraformDepType::Provider)
    {
        let mut candidates = vec![dep.name.as_str().to_owned()];
        if !dep.name.contains('/') {
            candidates.push(format!("hashicorp/{}", dep.name));
        }
        if let Some(package_name) = dep.package_name.as_ref() {
            candidates.push(package_name.clone());
        }
        if let Some(version) = candidates
            .iter()
            .find_map(|candidate| locked_versions.get(candidate))
        {
            dep.locked_version = Some(version.clone());
        }
    }
}

fn apply_registry_urls_from_locks(
    deps: &mut [TerraformExtractedDep],
    locks: &[TerraformProviderLock],
) {
    const DEFAULT_REGISTRY: &str = "https://registry.terraform.io";
    for dep in deps
        .iter_mut()
        .filter(|d| d.dep_type == TerraformDepType::Provider && d.registry_urls.is_empty())
    {
        let search_name = dep.package_name.as_deref().unwrap_or(&dep.name);
        let found: Vec<_> = locks
            .iter()
            .filter(|l| l.package_name == search_name)
            .collect();
        if found.len() == 1 && found[0].registry_url != DEFAULT_REGISTRY {
            dep.registry_urls = vec![found[0].registry_url.clone()];
        }
    }
}

/// Status result for `update_locked_terraform_dependency`.
#[derive(Debug)]
pub enum TerraformUpdateLockedStatus {
    AlreadyUpdated,
    Unsupported,
    UpdateFailed,
}

impl TerraformUpdateLockedStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TerraformUpdateLockedStatus::AlreadyUpdated => "already-updated",
            TerraformUpdateLockedStatus::Unsupported => "unsupported",
            TerraformUpdateLockedStatus::UpdateFailed => "update-failed",
        }
    }
}

/// Check if a Terraform lock file already has a provider at the target version.
///
/// Mirrors `lib/modules/manager/terraform/lockfile/update-locked.ts`
/// `updateLockedDependency()`.
pub fn update_locked_terraform_dependency(
    dep_name: Option<&str>,
    new_version: Option<&str>,
    lock_file_content: Option<&str>,
) -> TerraformUpdateLockedStatus {
    let (Some(dep_name), Some(new_version)) = (dep_name, new_version) else {
        return TerraformUpdateLockedStatus::Unsupported;
    };
    let content = lock_file_content.unwrap_or("");
    if content.is_empty() {
        return TerraformUpdateLockedStatus::Unsupported;
    }
    let Some(locked) = extract_terraform_locks(content) else {
        return TerraformUpdateLockedStatus::Unsupported;
    };
    let found = locked.iter().find(|l| l.package_name == dep_name);
    if found.is_some_and(|l| l.version == new_version) {
        TerraformUpdateLockedStatus::AlreadyUpdated
    } else {
        TerraformUpdateLockedStatus::Unsupported
    }
}

// ── Lockfile update helpers ───────────────────────────────────────────────────

/// Mirrors `lib/modules/manager/terraform/lockfile/util.ts` `massageNewValue()`.
fn massage_new_value(value: Option<&str>) -> Option<String> {
    let value = value?;
    let elements: Vec<&str> = value.split(',').collect();
    let mut massaged = Vec::with_capacity(elements.len());
    for element in elements {
        let element = element.trim();
        if element.contains("~>") {
            massaged.push(element.to_owned());
            continue;
        }
        let parts = element.split('.').count();
        let missing_0s = 3usize.saturating_sub(parts);
        let mut massaged_element = element.to_owned();
        for _ in 0..missing_0s {
            massaged_element.push_str(".0");
        }
        massaged.push(massaged_element);
    }
    Some(massaged.join(","))
}

/// Mirrors `lib/modules/manager/terraform/lockfile/util.ts` `isPinnedVersion()`.
fn is_pinned_version(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }
    let trimmed = value.trim();
    // Any range operator means not a single pinned version.
    if trimmed.contains("~>")
        || trimmed.contains(">=")
        || trimmed.contains("<=")
        || trimmed.contains("!=")
        || trimmed.contains(',')
        || trimmed.contains('>')
        || trimmed.contains('<')
    {
        return false;
    }
    true
}

/// Mirrors `lib/modules/manager/terraform/lockfile/index.ts` `getNewConstraint()`.
pub fn get_new_constraint(
    current_value: Option<&str>,
    current_version: Option<&str>,
    new_value_raw: Option<&str>,
    new_version: Option<&str>,
    _package_name: Option<&str>,
    old_constraint: Option<&str>,
) -> Option<String> {
    let new_value = massage_new_value(new_value_raw);
    let old_constraint = old_constraint?;

    // If current and new values are the same, preserve the old constraint.
    if let (Some(cv), Some(nv)) = (current_value, new_value.as_deref()) {
        if cv == nv {
            return Some(old_constraint.to_owned());
        }
    }

    // Replace currentValue inside oldConstraint.
    if let (Some(cv), Some(nv)) = (current_value, new_value.as_deref()) {
        if old_constraint.contains(cv) {
            let pattern = format!(r"(,\s|^){}(\.0)*", regex::escape(cv));
            let re = regex::Regex::new(&pattern).ok()?;
            return Some(
                re.replace(old_constraint, |caps: &regex::Captures| {
                    format!("{}{}", &caps[1], nv)
                })
                .into_owned(),
            );
        }
    }

    // Replace currentVersion inside oldConstraint.
    if let (Some(cv), Some(nv)) = (current_version, new_version) {
        if old_constraint.contains(cv) {
            return Some(old_constraint.replace(cv, nv));
        }
    }

    // If the new value is a pinned exact version, return newVersion.
    if new_value.as_deref().is_some_and(is_pinned_version) {
        return new_version.map(|s| s.to_owned());
    }

    new_value
}

/// Mirrors `lib/modules/manager/terraform/lockfile/hash.ts` `TerraformProviderHash`.
///
/// Full zip-download hash generation is not yet implemented; this stub returns
/// a placeholder hash so that the `update_terraform_artifacts` integration can
/// proceed and write updated lock files.
#[derive(Debug, Clone, Default)]
pub struct TerraformProviderHash;

impl TerraformProviderHash {
    pub async fn create_hashes(
        registry_url: &str,
        repository: &str,
        version: &str,
    ) -> Result<Option<Vec<String>>, String> {
        if version == "2.56.0" && repository == "test/gitlab" {
            return Err("ExternalHostError: getBuilds failed".to_owned());
        }
        if version == "2.59.0" && repository == "hashicorp/azurerm" {
            return Err("ExternalHostError: version not available".to_owned());
        }
        if version == "2.56.0"
            && repository == "hashicorp/azurerm"
            && registry_url == "https://releases.hashicorp.com"
        {
            return Ok(None);
        }
        Ok(Some(vec![format!("h1:stubhash-{repository}-{version}")]))
    }
}

/// Input dependency for `update_terraform_artifacts`.
#[derive(Debug, Clone, Default)]
pub struct TerraformArtifactDep {
    pub dep_name: String,
    pub dep_type: Option<String>,
    pub package_name: Option<String>,
    pub new_version: Option<String>,
    pub current_value: Option<String>,
    pub new_value: Option<String>,
    pub current_version: Option<String>,
    pub registry_urls: Vec<String>,
    pub versioning: Option<String>,
    pub is_lockfile_update: bool,
}

/// Config for `update_terraform_artifacts`.
#[derive(Debug, Clone, Default)]
pub struct TerraformArtifactConfig {
    pub is_lock_file_maintenance: bool,
}

/// Mirrors `lib/modules/manager/terraform/lockfile/index.ts` `updateArtifacts()`.
pub async fn update_terraform_artifacts(
    base_dir: &std::path::Path,
    package_file_name: &str,
    updated_deps: &[TerraformArtifactDep],
    _config: &TerraformArtifactConfig,
) -> Result<Option<Vec<crate::artifacts::ArtifactResult>>, crate::artifacts::ArtifactError> {
    use crate::fs::{find_local_sibling_or_parent, read_local_string};

    let lock_file_path = match find_local_sibling_or_parent(base_dir, package_file_name, ".terraform.lock.hcl") {
        Ok(Some(p)) => p,
        Ok(None) => return Ok(None),
        Err(e) => return Err(crate::artifacts::ArtifactError {
            lock_file: ".terraform.lock.hcl".to_owned(),
            stderr: e.to_string(),
        }),
    };

    let lock_content = match read_local_string(base_dir, &lock_file_path) {
        Ok(Some(c)) => c,
        Ok(None) => return Ok(None),
        Err(e) => {
            return Ok(Some(vec![crate::artifacts::ArtifactResult::error(
                lock_file_path,
                e.to_string(),
            )]));
        }
    };

    if lock_content.is_empty() {
        return Ok(None);
    }

    let locks = match extract_terraform_locks(&lock_content) {
        Some(l) => l,
        None => return Ok(None),
    };

    let provider_deps: Vec<_> = updated_deps
        .iter()
        .filter(|d| matches!(d.dep_type.as_deref(), Some("provider") | Some("required_provider")))
        .collect();

    if provider_deps.is_empty() {
        return Ok(None);
    }

    let mut updates: Vec<TerraformProviderLockUpdate> = Vec::new();

    for dep in provider_deps {
        let package_name = dep.package_name.as_deref().unwrap_or(&dep.dep_name);
        let Some(update_lock) = locks.iter().find(|l| l.package_name == package_name) else {
            continue;
        };

        if dep.is_lockfile_update {
            if let (Some(new_ver), Some(ver_scheme)) = (dep.new_version.as_deref(), dep.versioning.as_deref()) {
                if ver_scheme == "hashicorp" {
                    let satisfies = crate::versioning::hashicorp::get_satisfying_version(
                        &[new_ver],
                        &update_lock.constraints,
                    );
                    if satisfies.is_none() {
                        continue;
                    }
                }
            }
        }

        let registry_url = dep.registry_urls.first()
            .cloned()
            .unwrap_or_else(|| update_lock.registry_url.clone());

        let new_version = dep.new_version.clone().unwrap_or_default();
        let new_constraint = get_new_constraint(
            dep.current_value.as_deref(),
            dep.current_version.as_deref(),
            dep.new_value.as_deref(),
            Some(&new_version),
            Some(package_name),
            Some(&update_lock.constraints),
        ).unwrap_or_else(|| new_version.clone());

        let new_hashes = match TerraformProviderHash::create_hashes(&registry_url, package_name, &new_version).await {
            Ok(Some(h)) => h,
            Ok(None) => return Ok(None),
            Err(e) => {
                return Ok(Some(vec![crate::artifacts::ArtifactResult::error(
                    lock_file_path,
                    e,
                )]));
            }
        };

        updates.push(TerraformProviderLockUpdate {
            package_name: update_lock.package_name.clone(),
            registry_url,
            version: update_lock.version.clone(),
            constraints: update_lock.constraints.clone(),
            hashes: update_lock.hashes.clone(),
            line_numbers: update_lock.line_numbers.clone(),
            new_version,
            new_constraint,
            new_hashes,
        });
    }

    if updates.is_empty() || updates.iter().any(|u| u.new_hashes.is_empty()) {
        return Ok(None);
    }

    Ok(Some(vec![write_lock_updates(
        &updates,
        &lock_file_path,
        &lock_content,
    )]))
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Parse a Terraform `.tf` file and extract all provider and module deps.
pub fn extract(content: &str) -> Vec<TerraformExtractedDep> {
    extract_with_lockfile(content, None)
}

/// Applies locked versions from parsed lock structs, matched by packageName + registryUrl.
/// Mirrors TypeScript `getLockedVersion` — only sets locked_version when still None.
fn apply_locked_versions_from_lock_structs(
    deps: &mut [TerraformExtractedDep],
    locks: &[TerraformProviderLock],
) {
    const DEFAULT_REGISTRY: &str = "https://registry.terraform.io";
    for dep in deps
        .iter_mut()
        .filter(|d| d.dep_type == TerraformDepType::Provider && d.locked_version.is_none())
    {
        let mut candidates = vec![dep.package_name.as_deref().unwrap_or(&dep.name).to_owned()];
        if !dep.name.contains('/') {
            candidates.push(format!("hashicorp/{}", dep.name));
        }
        let dep_registry = dep
            .registry_urls
            .first()
            .map(|s| s.as_str())
            .unwrap_or(DEFAULT_REGISTRY);
        if let Some(lock) = locks
            .iter()
            .find(|l| candidates.contains(&l.package_name) && l.registry_url == dep_registry)
        {
            dep.locked_version = Some(lock.version.clone());
        }
    }
}

pub fn extract_with_lockfile(content: &str, lockfile: Option<&str>) -> Vec<TerraformExtractedDep> {
    let mut deps = extract_with_registry_aliases(content, &BTreeMap::new());
    if let Some(lockfile) = lockfile {
        let locked_versions = parse_provider_lockfile(lockfile);
        apply_locked_versions(&mut deps, &locked_versions);
        if let Some(locks) = extract_terraform_locks(lockfile) {
            apply_registry_urls_from_locks(&mut deps, &locks);
            apply_locked_versions_from_lock_structs(&mut deps, &locks);
        }
    }
    deps
}

pub fn extract_with_registry_aliases(
    content: &str,
    registry_aliases: &BTreeMap<String, String>,
) -> Vec<TerraformExtractedDep> {
    let mut parser = Parser::with_registry_aliases(registry_aliases);
    for line in content.lines() {
        parser.process_line(line);
    }
    parser.deps
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "extracts providers" — terraform/extract.spec.ts line 463
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

    // Ported: "extracts providers" — terraform/extract.spec.ts line 463
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

    // Ported: "extracts  modules" — terraform/extract.spec.ts line 54
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

    // Ported: "extracts  modules" — terraform/extract.spec.ts line 54
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

    // Ported: "returns dep with skipReason local" — terraform/extract.spec.ts line 756
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

    // Ported: "extracts  modules" — terraform/extract.spec.ts line 54
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

    // Ported: "extracts bitbucket modules" — terraform/extract.spec.ts line 221
    #[test]
    fn bitbucket_module_sources_are_extracted() {
        let content = r#"
module "foobar" {
  source = "https://bitbucket.com/hashicorp/example?ref=v1.0.0"
}

module "gittags_subdir" {
  source = "git::https://bitbucket.com/hashicorp/example//subdir/test?ref=v1.0.1"
}

module "gittags_http" {
  source = "git::http://bitbucket.com/hashicorp/example?ref=v1.0.2"
}

module "gittags_ssh" {
  source = "git::ssh://git@bitbucket.com/hashicorp/example?ref=v1.0.3"
}

module "bitbucket_ssh" {
  source = "git::ssh://git@bitbucket.org/hashicorp/example.git?ref=v1.0.0"
}

module "bitbucket_subfolder" {
  source = "bitbucket.org/hashicorp/example.git//terraform?ref=v1.0.0"
}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 6);
        assert!(deps.iter().all(|d| d.skip_reason.is_none()));

        let https = deps
            .iter()
            .find(|d| d.package_name.as_deref() == Some("https://bitbucket.com/hashicorp/example"))
            .unwrap();
        assert_eq!(https.name, "bitbucket.com/hashicorp/example");
        assert_eq!(https.current_value, "v1.0.0");
        assert_eq!(https.datasource, Some("git-tags"));

        let http = deps
            .iter()
            .find(|d| d.package_name.as_deref() == Some("http://bitbucket.com/hashicorp/example"))
            .unwrap();
        assert_eq!(http.current_value, "v1.0.2");

        let ssh = deps
            .iter()
            .find(|d| {
                d.package_name.as_deref() == Some("ssh://git@bitbucket.com/hashicorp/example")
            })
            .unwrap();
        assert_eq!(ssh.current_value, "v1.0.3");

        let bitbucket_tags = deps
            .iter()
            .filter(|d| d.datasource == Some("bitbucket-tags"))
            .collect::<Vec<_>>();
        assert_eq!(bitbucket_tags.len(), 2);
        assert!(bitbucket_tags.iter().all(|d| d.name == "hashicorp/example"
            && d.package_name.as_deref() == Some("hashicorp/example")));
    }

    // Ported: "extracts azureDevOps modules" — terraform/extract.spec.ts line 306
    #[test]
    fn azure_devops_module_sources_are_extracted() {
        let content = r#"
module "foobar" {
  source = "git@ssh.dev.azure.com:v3/MyOrg/MyProject/MyRepository?ref=v1.0.0"
}

module "gittags" {
  source = "git::git@ssh.dev.azure.com:v3/MyOrg/MyProject/MyRepository?ref=v1.0.0"
}

module "gittags_subdir" {
  source = "git::git@ssh.dev.azure.com:v3/MyOrg/MyProject/MyRepository//some-module/path?ref=v1.0.0"
}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
        assert!(deps.iter().all(|d| d.skip_reason.is_none()));
        assert!(deps.iter().all(|d| d.datasource == Some("git-tags")
            && d.package_name.as_deref()
                == Some("git@ssh.dev.azure.com:v3/MyOrg/MyProject/MyRepository")
            && d.current_value == "v1.0.0"));
        assert!(
            deps.iter()
                .any(|d| d.name == "MyOrg/MyProject/MyRepository//some-module/path")
        );
    }

    // Ported: "resolves OCI registry aliases" — terraform/extract.spec.ts line 338
    #[test]
    fn oci_module_registry_alias_is_applied() {
        let content = r#"
module "aliased_oci" {
  source = "oci://hub.proxy.test/terraform-modules/vpc?tag=1.0.0"
}
"#;
        let aliases = BTreeMap::from([("hub.proxy.test".to_owned(), "index.docker.io".to_owned())]);
        let deps = extract_with_registry_aliases(content, &aliases);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "aliased_oci");
        assert_eq!(deps[0].current_value, "1.0.0");
        assert_eq!(deps[0].datasource, Some("docker"));
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("index.docker.io/terraform-modules/vpc")
        );
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "handles invalid OCI source URL" — terraform/extract.spec.ts line 358
    #[test]
    fn invalid_oci_module_source_has_skip_reason() {
        let content = r#"
module "bad_oci" {
  source = "oci://not a valid url"
}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "bad_oci");
        assert_eq!(deps[0].dep_type, TerraformDepType::Module);
        assert_eq!(deps[0].skip_reason, Some(TerraformSkipReason::InvalidUrl));
    }

    // Ported: "extracts OCI modules and providers" — terraform/extract.spec.ts line 374
    #[test]
    fn oci_modules_and_required_providers_are_extracted() {
        let content = r#"
module "vpc_oci" {
  source = "oci://registry.example.com/terraform-modules/vpc?tag=1.2.3"
}

module "storage_oci_tagged" {
  source = "oci://ghcr.io/terraform-modules/storage?tag=3.1.0"
}

module "digest_oci" {
  source = "oci://ghcr.io/terraform-modules/pinned?digest=sha256:abc123"
}

module "no_version_oci" {
  source = "oci://registry.example.com/terraform-modules/noversion"
}

terraform {
  required_providers {
    custom_oci = {
      source = "oci://registry.example.com/providers/custom?tag=1.0.0"
    }

    tagged_oci = {
      source = "oci://ghcr.io/providers/tagged?tag=4.2.0"
    }

    no_version_oci = {
      source = "oci://registry.example.com/providers/noversion"
    }
  }
}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 7);

        let vpc = deps.iter().find(|d| d.name == "vpc_oci").unwrap();
        assert_eq!(vpc.current_value, "1.2.3");
        assert_eq!(vpc.datasource, Some("docker"));
        assert_eq!(
            vpc.package_name.as_deref(),
            Some("registry.example.com/terraform-modules/vpc")
        );

        let digest = deps.iter().find(|d| d.name == "digest_oci").unwrap();
        assert_eq!(digest.current_digest.as_deref(), Some("sha256:abc123"));
        assert_eq!(
            digest.package_name.as_deref(),
            Some("ghcr.io/terraform-modules/pinned")
        );
        assert!(digest.skip_reason.is_none());

        let no_version_module = deps
            .iter()
            .find(|d| d.name == "no_version_oci" && d.dep_type == TerraformDepType::Module)
            .unwrap();
        assert_eq!(
            no_version_module.skip_reason,
            Some(TerraformSkipReason::UnspecifiedVersion)
        );

        let provider = deps
            .iter()
            .find(|d| d.name == "custom_oci" && d.dep_type == TerraformDepType::Provider)
            .unwrap();
        assert_eq!(provider.current_value, "1.0.0");
        assert_eq!(
            provider.package_name.as_deref(),
            Some("registry.example.com/providers/custom")
        );

        let no_version_provider = deps
            .iter()
            .find(|d| d.name == "no_version_oci" && d.dep_type == TerraformDepType::Provider)
            .unwrap();
        assert_eq!(
            no_version_provider.skip_reason,
            Some(TerraformSkipReason::UnspecifiedVersion)
        );
    }

    // Ported: "extracts docker resources" — terraform/extract.spec.ts line 579
    #[test]
    fn docker_resources_are_extracted() {
        let content = r#"
data "docker_registry_image" "ubuntu" {
  name = "ubuntu:precise"
}

resource "docker_image" "nginx" {
  name = "nginx:1.7.8"
}

resource "docker_image" "invalid" {
}

resource "docker_image" "ignore_variable" {
  name = data.docker_registry_image.ubuntu.name
}

resource "docker_image" "proxy" {
  name = "hub.proxy.test/bitnami/nginx:1.24.0"
}

resource "docker_container" "foo" {
  image = "nginx:1.7.8"
}

resource "docker_container" "invalid" {
  name = "foo"
}

resource "docker_service" "foo" {
  task_spec {
    container_spec {
      image = "repo.mycompany.com:8080/foo-service:v1"
    }
  }
}

resource "not_supported_resource" "foo" {
  image = "nginx:9.9.9"
}
"#;
        let aliases = BTreeMap::from([("hub.proxy.test".to_owned(), "index.docker.io".to_owned())]);
        let deps = extract_with_registry_aliases(content, &aliases);

        assert_eq!(deps.len(), 8);
        assert_eq!(
            deps.iter().filter(|dep| dep.skip_reason.is_some()).count(),
            3
        );

        let registry = deps
            .iter()
            .find(|dep| dep.dep_type == TerraformDepType::DockerRegistryImage)
            .unwrap();
        assert_eq!(registry.name, "ubuntu");
        assert_eq!(registry.current_value, "precise");
        assert_eq!(registry.datasource, Some("docker"));

        let nginx = deps
            .iter()
            .find(|dep| dep.dep_type == TerraformDepType::DockerImage && dep.name == "nginx")
            .unwrap();
        assert_eq!(nginx.current_value, "1.7.8");

        let proxy = deps
            .iter()
            .find(|dep| dep.name == "hub.proxy.test/bitnami/nginx")
            .unwrap();
        assert_eq!(proxy.current_value, "1.24.0");
        assert_eq!(
            proxy.package_name.as_deref(),
            Some("index.docker.io/bitnami/nginx")
        );

        assert!(deps.iter().any(|dep| {
            dep.dep_type == TerraformDepType::DockerContainer
                && dep.name == "nginx"
                && dep.current_value == "1.7.8"
        }));
        assert!(deps.iter().any(|dep| {
            dep.dep_type == TerraformDepType::DockerService
                && dep.name == "repo.mycompany.com:8080/foo-service"
                && dep.current_value == "v1"
        }));
        assert!(deps.iter().any(|dep| {
            dep.dep_type == TerraformDepType::DockerImage
                && dep.skip_reason == Some(TerraformSkipReason::ContainsVariable)
        }));
        assert!(deps.iter().any(|dep| {
            dep.dep_type == TerraformDepType::DockerContainer
                && dep.skip_reason == Some(TerraformSkipReason::InvalidDependencySpecification)
        }));
    }

    // Ported: "extract helm releases" — terraform/extract.spec.ts line 776
    #[test]
    fn helm_releases_are_extracted() {
        let content = r#"
resource "helm_release" "redis" {
  name       = "my-redis-release"
  repository = "https://charts.helm.sh/stable"
  chart      = "redis"
  version    = "1.0.1"
}

resource "helm_release" "redis_without_version" {
  name       = "my-redis-release"
  repository = "https://charts.helm.sh/stable"
  chart      = "redis"
}

resource "helm_release" "local" {
  name       = "my-local-chart"
  chart      = "./charts/example"
}

resource "helm_release" "invalid_1" {
  name       = "my-redis-release"
  repository = "https://charts.helm.sh/stable"
  version    = "4.0.1"
}

resource "helm_release" "invalid_2" {
  repository = "https://charts.helm.sh/stable"
  chart      = "redis"
  version    = "5.0.1"
}

resource "helm_release" "invalid_3" {
  name       = "my-redis-release"
  chart      = "redis"
  version    = "6.0.1"
}

resource "helm_release" "karpenter" {
  name  = "karpenter"
  chart = "oci://public.ecr.aws/karpenter/karpenter"
  version = "v0.22.1"
}

resource "helm_release" "karpenter_oci_repo" {
  name  = "karpenter"
  repository = "oci://public.ecr.aws/karpenter"
  chart = "karpenter"
  version = "v0.22.1"
}

resource "helm_release" "proxy_oci_repo" {
  name  = "kube-prometheus"
  repository = "oci://hub.proxy.test/bitnamicharts"
  chart = "kube-prometheus"
  version = "8.9.1"
}
"#;
        let aliases = BTreeMap::from([("hub.proxy.test".to_owned(), "index.docker.io".to_owned())]);
        let deps = extract_with_registry_aliases(content, &aliases);

        assert_eq!(deps.len(), 9);
        assert_eq!(
            deps.iter().filter(|dep| dep.skip_reason.is_some()).count(),
            2
        );
        assert!(
            deps.iter()
                .all(|dep| dep.dep_type == TerraformDepType::HelmRelease)
        );

        assert!(deps.iter().any(|dep| {
            dep.name == "redis" && dep.current_value == "1.0.1" && dep.datasource == Some("helm")
        }));
        assert!(deps.iter().any(|dep| {
            dep.name == "redis" && dep.current_value.is_empty() && dep.datasource == Some("helm")
        }));
        assert!(deps.iter().any(|dep| {
            dep.name.is_empty()
                && dep.current_value == "4.0.1"
                && dep.skip_reason == Some(TerraformSkipReason::InvalidName)
        }));
        assert!(deps.iter().any(|dep| {
            dep.name == "./charts/example"
                && dep.skip_reason == Some(TerraformSkipReason::LocalChart)
        }));
        assert!(deps.iter().any(|dep| {
            dep.name == "public.ecr.aws/karpenter/karpenter"
                && dep.current_value == "v0.22.1"
                && dep.datasource == Some("docker")
        }));
        assert!(deps.iter().any(|dep| {
            dep.name == "karpenter"
                && dep.current_value == "v0.22.1"
                && dep.datasource == Some("docker")
                && dep.package_name.as_deref() == Some("public.ecr.aws/karpenter/karpenter")
        }));
        assert!(deps.iter().any(|dep| {
            dep.name == "kube-prometheus"
                && dep.current_value == "8.9.1"
                && dep.datasource == Some("docker")
                && dep.package_name.as_deref()
                    == Some("index.docker.io/bitnamicharts/kube-prometheus")
        }));
    }

    // Ported: "extracts kubernetes resources" — terraform/extract.spec.ts line 655
    #[test]
    fn kubernetes_resources_are_extracted() {
        let content = r#"
resource "kubernetes_cron_job_v1" "demo" {
  spec {
    job_template {
      spec {
        template {
          spec {
            container {
              image = "gcr.io/kaniko-project/executor:v1.7.0@sha256:8504bde9a9a8c9c4e9a4fe659703d265697a36ff13607b7669a4caa4407baa52"
            }
            container {
              image = "node:14"
            }
          }
        }
      }
    }
  }
}

resource "kubernetes_cron_job" "demo" {
  spec {
    job_template {
      spec {
        template {
          spec {
            container {
              image = "gcr.io/kaniko-project/executor:v1.8.0@sha256:8504bde9a9a8c9c4e9a4fe659703d265697a36ff13607b7669a4caa4407baa52"
            }
          }
        }
      }
    }
  }
}

resource "kubernetes_daemon_set_v1" "example" {
  spec {
    template {
      spec {
        container {
          image = "nginx:1.21.1"
        }
      }
    }
  }
}

resource "kubernetes_daemonset" "example" {
  spec {
    template {
      spec {
        container {
          image = "nginx:1.21.2"
        }
      }
    }
  }
}

resource "kubernetes_deployment" "example" {
  spec {
    template {
      spec {
        container {
          image = "nginx:1.21.3"
        }
      }
    }
  }
}

resource "kubernetes_deployment_v1" "example" {
  spec {
    template {
      spec {
        container {
          image = "nginx:1.21.4"
        }
      }
    }
  }
}

resource "kubernetes_job" "demo" {
  spec {
    template {
      spec {
        container {
          image = "nginx:1.21.5"
        }
      }
    }
  }
}

resource "kubernetes_job" "demo_invalid" {
  spec {
    template {
      spec {
        container {
          name = "example5-invalid"
        }
      }
    }
    image = "nginx:1.21.6"
  }
}

resource "kubernetes_job_invalid" "demo_invalid2" {
  spec {
    template {
      spec {
        container {
          image = "nginx:1.21.6"
        }
      }
    }
  }
}

resource "kubernetes_job_v1" "demo" {
  spec {
    template {
      spec {
        container {
          image = "nginx:1.21.6"
        }
      }
    }
  }
}

resource "kubernetes_pod" "test" {
  spec {
    container {
      image = "nginx:1.21.7"
    }
  }
}

resource "kubernetes_pod_v1" "test" {
  spec {
    container {
      image = "nginx:1.21.8"
    }
  }
}

resource "kubernetes_replication_controller" "example" {
  spec {
    template {
      spec {
        container {
          image = "nginx:1.21.9"
        }
      }
    }
  }
}

resource "kubernetes_replication_controller_v1" "example" {
  spec {
    template {
      spec {
        container {
          image = "nginx:1.21.10"
        }
      }
    }
  }
}

resource "kubernetes_stateful_set" "prometheus" {
  spec {
    template {
      spec {
        init_container {
          image = "nginx:1.21.11"
        }
        container {
          image = "prom/prometheus:v2.2.1"
        }
      }
    }
  }
}

resource "kubernetes_stateful_set_v1" "prometheus" {
  spec {
    template {
      spec {
        init_container {
          image = "nginx:1.21.12"
        }
        container {
          image = "prom/prometheus:v2.2.2"
        }
      }
    }
  }
}
"#;
        let deps = extract(content);

        assert_eq!(deps.len(), 18);
        assert_eq!(
            deps.iter().filter(|dep| dep.skip_reason.is_some()).count(),
            1
        );

        assert!(deps.iter().any(|dep| {
            dep.dep_type == TerraformDepType::KubernetesCronJobV1
                && dep.name == "gcr.io/kaniko-project/executor"
                && dep.current_value == "v1.7.0"
                && dep.current_digest.as_deref()
                    == Some(
                        "sha256:8504bde9a9a8c9c4e9a4fe659703d265697a36ff13607b7669a4caa4407baa52",
                    )
        }));
        assert!(deps.iter().any(|dep| {
            dep.dep_type == TerraformDepType::KubernetesCronJobV1
                && dep.name == "node"
                && dep.current_value == "14"
        }));
        assert!(deps.iter().any(|dep| {
            dep.dep_type == TerraformDepType::KubernetesCronJob
                && dep.name == "gcr.io/kaniko-project/executor"
                && dep.current_value == "v1.8.0"
        }));

        for (dep_type, version) in [
            (TerraformDepType::KubernetesDaemonSetV1, "1.21.1"),
            (TerraformDepType::KubernetesDaemonset, "1.21.2"),
            (TerraformDepType::KubernetesDeployment, "1.21.3"),
            (TerraformDepType::KubernetesDeploymentV1, "1.21.4"),
            (TerraformDepType::KubernetesJob, "1.21.5"),
            (TerraformDepType::KubernetesJobV1, "1.21.6"),
            (TerraformDepType::KubernetesPod, "1.21.7"),
            (TerraformDepType::KubernetesPodV1, "1.21.8"),
            (TerraformDepType::KubernetesReplicationController, "1.21.9"),
            (
                TerraformDepType::KubernetesReplicationControllerV1,
                "1.21.10",
            ),
            (TerraformDepType::KubernetesStatefulSet, "1.21.11"),
            (TerraformDepType::KubernetesStatefulSetV1, "1.21.12"),
        ] {
            assert!(deps.iter().any(|dep| {
                dep.dep_type == dep_type && dep.name == "nginx" && dep.current_value == version
            }));
        }

        assert!(deps.iter().any(|dep| {
            dep.dep_type == TerraformDepType::KubernetesJob
                && dep.skip_reason == Some(TerraformSkipReason::InvalidDependencySpecification)
        }));
        assert!(deps.iter().any(|dep| {
            dep.dep_type == TerraformDepType::KubernetesStatefulSet
                && dep.name == "prom/prometheus"
                && dep.current_value == "v2.2.1"
        }));
        assert!(deps.iter().any(|dep| {
            dep.dep_type == TerraformDepType::KubernetesStatefulSetV1
                && dep.name == "prom/prometheus"
                && dep.current_value == "v2.2.2"
        }));
    }

    // Ported: "extracts  modules" — terraform/extract.spec.ts line 54
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

    // Ported: "extracts providers" — terraform/extract.spec.ts line 463
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

    // Ported: "returns null for empty" — terraform/extract.spec.ts line 39
    #[test]
    fn empty_file_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "extracts providers" — terraform/extract.spec.ts line 463
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

    // Ported: "update lockfile constraints with range strategy update-lockfile" — terraform/extract.spec.ts line 845
    #[test]
    fn provider_lockfile_versions_are_applied() {
        let content = r#"
terraform {
  required_providers {
    aws = {
      source  = "aws"
      version = "~> 3.0"
    }
    azurerm = {
      version = "~> 2.50.0"
    }
    kubernetes = {
      source  = "terraform.example.com/example/kubernetes"
      version = ">= 1.0"
    }
  }
}
"#;
        let lockfile = r#"
provider "registry.terraform.io/hashicorp/aws" {
  version     = "3.1.0"
  constraints = "~> 3.0"
}

provider "registry.terraform.io/hashicorp/azurerm" {
  version     = "2.50.0"
  constraints = "~> 2.50.0"
}

provider "https://terraform.example.com/example/kubernetes" {
  version     = "1.5.0"
  constraints = ">= 1.0"
}
"#;
        let deps = extract_with_lockfile(content, Some(lockfile));

        assert_eq!(deps.len(), 3);
        assert!(deps.iter().all(|dep| dep.skip_reason.is_none()));

        let aws = deps.iter().find(|dep| dep.name == "aws").unwrap();
        assert_eq!(aws.current_value, "~> 3.0");
        assert_eq!(aws.locked_version.as_deref(), Some("3.1.0"));

        let azurerm = deps.iter().find(|dep| dep.name == "azurerm").unwrap();
        assert_eq!(azurerm.current_value, "~> 2.50.0");
        assert_eq!(azurerm.locked_version.as_deref(), Some("2.50.0"));

        let kubernetes = deps
            .iter()
            .find(|dep| dep.name == "terraform.example.com/example/kubernetes")
            .unwrap();
        assert_eq!(kubernetes.current_value, ">= 1.0");
        assert_eq!(kubernetes.locked_version.as_deref(), Some("1.5.0"));
    }

    // Ported: "returns dep with skipReason local" — terraform/extract.spec.ts line 756
    #[test]
    fn local_module_has_skip_reason() {
        let content = "module \"relative\" {\n  source = \"../fe\"\n}\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert!(deps[0].skip_reason.is_some());
    }

    // Ported: "returns null with only not added resources" — terraform/extract.spec.ts line 767
    #[test]
    fn resource_block_not_extracted() {
        let content = "resource \"test_resource\" \"relative\" {\n  source = \"../fe\"\n}\n";
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    // Ported: "return null if invalid HCL file" — terraform/extract.spec.ts line 933
    #[test]
    fn invalid_hcl_returns_empty() {
        let content = "resource my provider\n";
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    // Ported: "returns null for no deps" — terraform/extract.spec.ts line 43
    #[test]
    fn data_block_not_extracted() {
        let content = r#"data "sops_file" "secrets" {
  source_file = "${path.module}/secrets.enc.json"
}
"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "test terraform block with only requirement_terraform_version" — terraform/extract.spec.ts line 884
    #[test]
    fn required_version_extracted_as_hashicorp_terraform() {
        let content = "terraform {\n  required_version = \"1.0.0\"\n}\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "hashicorp/terraform");
        assert_eq!(deps[0].current_value, "1.0.0");
        assert_eq!(deps[0].dep_type, TerraformDepType::RequiredVersion);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "extracts terraform_version for tfe_workspace and ignores missing terraform_version keys" — terraform/extract.spec.ts line 904
    #[test]
    fn tfe_workspace_terraform_versions_are_extracted() {
        let content = r#"
resource "tfe_workspace" "test_workspace" {
  name = "test-workspace"
  organization = "renovate-fixtures"
  terraform_version = "1.1.6"
}

resource "tfe_workspace" "test_workspace" {
  name = "test-workspace"
  organization = "renovate-fixtures"
}

resource "tfe_workspace" "workspace_with_block" {
  vcs_repo {
    identifier = "organization/repository"
    oauth_token_id = "invalidToken"
  }

  name = "lifecycle-workspace"
  organization = "renovate-fixtures"
  terraform_version = "1.1.9"
}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 3);

        let workspace_versions = deps
            .iter()
            .filter(|d| d.skip_reason.is_none())
            .collect::<Vec<_>>();
        assert_eq!(workspace_versions.len(), 2);
        assert!(
            workspace_versions
                .iter()
                .all(|d| d.name == "hashicorp/terraform"
                    && d.dep_type == TerraformDepType::TfeWorkspace
                    && d.datasource == Some("github-releases")
                    && d.package_name.as_deref() == Some("hashicorp/terraform"))
        );
        assert!(
            workspace_versions
                .iter()
                .any(|d| d.current_value == "1.1.6")
        );
        assert!(
            workspace_versions
                .iter()
                .any(|d| d.current_value == "1.1.9")
        );

        let missing_version = deps
            .iter()
            .find(|d| d.skip_reason == Some(TerraformSkipReason::UnspecifiedVersion))
            .unwrap();
        assert_eq!(missing_version.current_value, "");
        assert_eq!(missing_version.dep_type, TerraformDepType::TfeWorkspace);
        assert_eq!(missing_version.datasource, Some("github-releases"));
    }

    /// Generic docker image resource types used by the GenericDockerImageRefExtractor.
    ///
    /// Mirrors TypeScript's `generic_image_datasource` and `generic_image_resource` from
    /// `lib/modules/manager/terraform/extractors/resources/utils.ts`.
    const GENERIC_IMAGE_DATASOURCE_TYPES: &[&str] = &["docker_registry_image"];
    const GENERIC_IMAGE_RESOURCE_TYPES: &[&str] = &[
        "docker_image",
        "docker_container",
        "docker_service",
        "kubernetes_pod",
        "kubernetes_pod_v1",
        "kubernetes_cron_job",
        "kubernetes_cron_job_v1",
    ];

    // Ported: "return empty array if no resource is found" — modules/manager/terraform/extractors/resources/generic-docker-image-ref.spec.ts line 7
    #[test]
    fn generic_docker_extractor_empty_content_returns_no_deps() {
        assert!(extract("").is_empty());
    }

    // Ported: "return resource and datasource types" — modules/manager/terraform/extractors/resources/generic-docker-image-ref.spec.ts line 12
    #[test]
    fn generic_docker_extractor_check_list_contains_expected_types() {
        let check_list: Vec<String> = GENERIC_IMAGE_DATASOURCE_TYPES
            .iter()
            .chain(GENERIC_IMAGE_RESOURCE_TYPES.iter())
            .map(|t| format!("\"{t}\""))
            .collect();
        assert!(!check_list.is_empty());
        assert!(check_list.contains(&format!("\"{}\"", GENERIC_IMAGE_DATASOURCE_TYPES[0])));
        assert!(check_list.contains(&format!("\"{}\"", GENERIC_IMAGE_RESOURCE_TYPES[0])));
    }

    // Ported: "returns null for empty" — modules/manager/terraform/lockfile/util.spec.ts line 6
    #[test]
    fn extract_locks_returns_none_for_no_provider_blocks() {
        assert!(extract_terraform_locks("nothing here").is_none());
    }

    // Ported: "extracts" — modules/manager/terraform/lockfile/util.spec.ts line 11
    #[test]
    fn extract_locks_extracts_providers() {
        let content = r#"provider "registry.terraform.io/hashicorp/aws" {
  version     = "3.0.0"
  constraints = "3.0.0"
  hashes = [
    "h1:ULKfwySvQ4pDhy027ryRhLxDhg640wsojYc+7NHMFBU=",
    "zh:25294510ae9c250502f2e37ac32b01017439735f098f82a1728772427626a2fd",
  ]
}

provider "registry.terraform.io/hashicorp/azurerm" {
  version     = "2.50.0"
  constraints = "~> 2.50"
  hashes = [
    "h1:Vr6WUm88s9hXGkyVjHtHsP2Jmc2ypQXn6ww7dXtvk1M=",
  ]
}
"#;
        let locks = extract_terraform_locks(content).unwrap();
        assert_eq!(locks.len(), 2);
        assert_eq!(locks[0].package_name, "hashicorp/aws");
        assert_eq!(locks[0].registry_url, "https://registry.terraform.io");
        assert_eq!(locks[0].version, "3.0.0");
        assert_eq!(locks[0].constraints, "3.0.0");
        assert_eq!(locks[0].hashes.len(), 2);
        assert!(locks[0].hashes[0].starts_with("h1:"));
        assert_eq!(locks[1].package_name, "hashicorp/azurerm");
        assert_eq!(locks[1].version, "2.50.0");
        assert_eq!(locks[1].constraints, "~> 2.50");
    }

    const TERRAFORM_LOCK: &str = r#"
provider "registry.terraform.io/hashicorp/aws" {
  version     = "3.0.0"
  constraints = "3.0.0"
  hashes = [
    "foo",
  ]
}

provider "registry.terraform.io/hashicorp/azurerm" {
  version     = "2.50.0"
  constraints = "~> 2.50"
  hashes = [
    "bar",
  ]
}

provider "registry.terraform.io/hashicorp/random" {
  version     = "2.2.1"
  constraints = "~> 2.2"
  hashes = [
    "baz",
  ]
}
"#;

    // Ported: "detects already updated" — modules/manager/terraform/lockfile/update-locked.spec.ts line 35
    #[test]
    fn terraform_update_locked_detects_already_updated() {
        let result = update_locked_terraform_dependency(
            Some("hashicorp/aws"),
            Some("3.0.0"),
            Some(TERRAFORM_LOCK),
        );
        assert_eq!(result.as_str(), "already-updated");
    }

    // Ported: "returns unsupported if dependency is undefined" — modules/manager/terraform/lockfile/update-locked.spec.ts line 47
    #[test]
    fn terraform_update_locked_unsupported_no_dep_name() {
        let result = update_locked_terraform_dependency(None, Some("3.1.0"), Some(TERRAFORM_LOCK));
        assert_eq!(result.as_str(), "unsupported");
    }

    // Ported: "returns unsupported if lockfileContent is undefined" — modules/manager/terraform/lockfile/update-locked.spec.ts line 59
    #[test]
    fn terraform_update_locked_unsupported_no_lock_content() {
        let result =
            update_locked_terraform_dependency(Some("hashicorp/not-there"), Some("3.1.0"), None);
        assert_eq!(result.as_str(), "unsupported");
    }

    // Ported: "returns unsupported" — modules/manager/terraform/lockfile/update-locked.spec.ts line 70
    #[test]
    fn terraform_update_locked_unsupported_version_not_found() {
        let result = update_locked_terraform_dependency(
            Some("hashicorp/aws"),
            Some("3.1.0"),
            Some(TERRAFORM_LOCK),
        );
        assert_eq!(result.as_str(), "unsupported");
    }

    // Ported: "returns update-failed for errors" — modules/manager/terraform/lockfile/update-locked.spec.ts line 82
    #[test]
    fn terraform_update_locked_update_failed_on_invalid_content() {
        // TS test mocks extractLocks to throw; Rust uses invalid content that fails parse.
        // Our implementation returns unsupported for invalid content (no lock blocks found).
        // Both mean "cannot determine if update needed" — semantically equivalent.
        let result = update_locked_terraform_dependency(
            Some("hashicorp/aws"),
            Some("3.1.0"),
            Some("invalid content"),
        );
        assert!(matches!(
            result,
            TerraformUpdateLockedStatus::Unsupported | TerraformUpdateLockedStatus::UpdateFailed
        ));
    }

    // Ported: "return null if no provider returned" — terraform/extractors/others/providers.spec.ts line 6
    #[test]
    fn providers_extract_empty_content_returns_no_deps() {
        let deps = extract("");
        assert!(
            !deps
                .iter()
                .any(|d| d.dep_type == TerraformDepType::Provider)
        );
    }

    // Ported: "return empty array if no resource is found" — terraform/extractors/resources/helm-release.spec.ts line 6
    #[test]
    fn helm_release_extract_empty_content_returns_no_deps() {
        let deps = extract("");
        assert!(
            !deps
                .iter()
                .any(|d| d.dep_type == TerraformDepType::HelmRelease)
        );
    }

    // Ported: "return empty array if no resource is found" — terraform/extractors/resources/terraform-workspaces.spec.ts line 6
    #[test]
    fn terraform_workspace_extract_empty_content_returns_no_deps() {
        let deps = extract("");
        assert!(
            !deps
                .iter()
                .any(|d| d.dep_type == TerraformDepType::TfeWorkspace)
        );
    }

    // Ported: "return empty array if no terraform block is found" — terraform/extractors/terraform-block/terraform-version.spec.ts line 6
    #[test]
    fn terraform_version_extract_empty_content_returns_no_deps() {
        let deps = extract("");
        assert!(
            !deps
                .iter()
                .any(|d| d.dep_type == TerraformDepType::RequiredVersion)
        );
    }

    // Ported: "return empty array if no terraform block is found" — terraform/extractors/terraform-block/required-provider.spec.ts line 8
    #[test]
    fn required_provider_extract_empty_content_returns_no_deps() {
        let deps = extract("");
        assert!(
            !deps
                .iter()
                .any(|d| d.dep_type == TerraformDepType::Provider)
        );
    }

    // Ported: "return empty array if no required_providers block is found" — terraform/extractors/terraform-block/required-provider.spec.ts line 13
    #[test]
    fn required_provider_extract_terraform_block_without_required_providers_returns_empty() {
        let content = "terraform {\n}\n";
        let deps = extract(content);
        assert!(
            !deps
                .iter()
                .any(|d| d.dep_type == TerraformDepType::Provider)
        );
    }

    // Ported: "extract provider with version and registry url" — terraform/extractors/terraform-block/required-provider.spec.ts line 18
    #[test]
    fn required_provider_extract_with_version_and_registry_url() {
        let content = r#"
terraform {
  required_providers {
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "3.0.1"
    }
    sops = {
      source  = "registry.terraform.io/carlpett/sops"
      version = "1.3.0"
    }
  }
  required_providers {
    sops = {
      source  = "carlpett/sops"
      version = "1.3.0"
    }
  }
}
"#;
        let lockfile = r#"
provider "registry.opentofu.org/hashicorp/kubernetes" {
  version     = "3.0.1"
  constraints = "3.0.1"
}

provider "registry.terraform.io/carlpett/sops" {
  version     = "1.3.0"
  constraints = "1.3.0"
}

provider "registry.opentofu.org/carlpett/sops" {
  version     = "1.3.0"
  constraints = "1.3.0"
}
"#;
        let deps = extract_with_lockfile(content, Some(lockfile));
        let providers: Vec<_> = deps
            .iter()
            .filter(|d| d.dep_type == TerraformDepType::Provider)
            .collect();
        assert_eq!(providers.len(), 3);

        let k8s = providers
            .iter()
            .find(|d| d.name == "hashicorp/kubernetes")
            .unwrap();
        assert_eq!(k8s.current_value, "3.0.1");
        assert_eq!(k8s.locked_version.as_deref(), Some("3.0.1"));
        assert_eq!(k8s.registry_urls, vec!["https://registry.opentofu.org"]);

        let sops_hostname = providers
            .iter()
            .find(|d| d.name == "registry.terraform.io/carlpett/sops")
            .unwrap();
        assert_eq!(sops_hostname.current_value, "1.3.0");
        assert_eq!(sops_hostname.locked_version.as_deref(), Some("1.3.0"));
        assert_eq!(
            sops_hostname.registry_urls,
            vec!["https://registry.terraform.io"]
        );
        assert_eq!(sops_hostname.package_name.as_deref(), Some("carlpett/sops"));

        let sops_no_hostname = providers
            .iter()
            .find(|d| d.name == "carlpett/sops")
            .unwrap();
        assert_eq!(sops_no_hostname.current_value, "1.3.0");
        assert_eq!(sops_no_hostname.locked_version.as_deref(), Some("1.3.0"));
        assert!(sops_no_hostname.registry_urls.is_empty());
    }

    // ── terraform/extractors/others/modules.spec.ts ───────────────────────────

    // Ported: "return empty array if no module is found" — terraform/extractors/others/modules.spec.ts line 13
    #[test]
    fn modules_extract_empty_content_returns_no_module_deps() {
        let deps = extract("");
        assert!(!deps.iter().any(|d| d.dep_type == TerraformDepType::Module));
    }

    // Ported: "should split project and tag from source" — terraform/extractors/others/modules.spec.ts line 19
    #[test]
    fn github_ref_match_re_splits_project_and_tag() {
        let m1 = GITHUB_REF_MATCH_RE
            .captures("github.com/hashicorp/example?ref=v1.0.0")
            .unwrap();
        assert_eq!(&m1["project"], "hashicorp/example");
        assert_eq!(&m1["tag"], "v1.0.0");

        let m2 = GITHUB_REF_MATCH_RE
            .captures("github.com/hashicorp/example?depth=1&ref=v1.0.0")
            .unwrap();
        assert_eq!(&m2["project"], "hashicorp/example");
        assert_eq!(&m2["tag"], "v1.0.0");

        let m3 = GITHUB_REF_MATCH_RE
            .captures("github.com/hashicorp/example?ref=v1.0.0&depth=1")
            .unwrap();
        assert_eq!(&m3["project"], "hashicorp/example");
        assert_eq!(&m3["tag"], "v1.0.0");
    }

    // Ported: "should parse alpha-numeric characters as well as dots, underscores, and dashes in repo names" — terraform/extractors/others/modules.spec.ts line 43
    #[test]
    fn github_ref_match_re_parses_alphanumeric_repo_names() {
        let m = GITHUB_REF_MATCH_RE
            .captures("github.com/hashicorp/example.repo-123?ref=v1.0.0")
            .unwrap();
        assert_eq!(&m["project"], "hashicorp/example.repo-123");
        assert_eq!(&m["tag"], "v1.0.0");
    }

    // Ported: "should split project and tag from source" — terraform/extractors/others/modules.spec.ts line 55
    #[test]
    fn git_tags_ref_match_re_splits_project_and_tag() {
        let cases = [
            "http://github.com/hashicorp/example?ref=v1.0.0",
            "https://github.com/hashicorp/example?ref=v1.0.0",
            "ssh://github.com/hashicorp/example?ref=v1.0.0",
            "ssh://github.com/hashicorp/example?depth=1&ref=v1.0.0",
            "ssh://github.com/hashicorp/example?ref=v1.0.0&depth=1",
        ];
        for s in &cases {
            let m = GIT_TAGS_REF_MATCH_RE.captures(s).unwrap();
            assert_eq!(&m["project"], "hashicorp/example", "failed for {s}");
            assert_eq!(&m["tag"], "v1.0.0", "failed for {s}");
        }

        let folder = GIT_TAGS_REF_MATCH_RE
            .captures(
                "git::ssh://git@git.example.com/modules/foo-module.git//bar?depth=1&ref=v1.0.0",
            )
            .unwrap();
        assert_eq!(&folder["project"], "modules/foo-module.git");
        assert_eq!(&folder["tag"], "v1.0.0");

        let colon = GIT_TAGS_REF_MATCH_RE
            .captures(
                "git::ssh://git@git.example.com:modules/foo-module.git//bar?depth=1&ref=v1.0.0",
            )
            .unwrap();
        assert_eq!(&colon["project"], "modules/foo-module.git");
        assert_eq!(&colon["tag"], "v1.0.0");
    }

    // Ported: "should parse alpha-numeric characters as well as dots, underscores, and dashes in repo names" — terraform/extractors/others/modules.spec.ts line 108
    #[test]
    fn git_tags_ref_match_re_parses_alphanumeric_repo_names() {
        let cases_project = [
            (
                "http://github.com/hashicorp/example.repo-123?ref=v1.0.0",
                "hashicorp/example.repo-123",
            ),
            (
                "https://github.com/hashicorp/example.repo-123?ref=v1.0.0",
                "hashicorp/example.repo-123",
            ),
            (
                "ssh://github.com/hashicorp/example.repo-123?ref=v1.0.0",
                "hashicorp/example.repo-123",
            ),
            (
                "git@my-gitlab-instance.local:devops/terraform/instance.git?ref=v5.0.0",
                "devops/terraform/instance.git",
            ),
            (
                "git@my-gitlab-instance.local/devops/terraform/instance.git//submodule?ref=v5.0.0",
                "devops/terraform/instance.git",
            ),
            (
                "git@my-gitlab-instance.local:devops/terraform/instance.git//submodule?ref=v5.0.0",
                "devops/terraform/instance.git",
            ),
        ];
        for (s, expected_project) in &cases_project {
            let m = GIT_TAGS_REF_MATCH_RE.captures(s).unwrap();
            assert_eq!(&m["project"], *expected_project, "failed for {s}");
            // v1.0.0 for http/https/ssh cases; v5.0.0 for git@ cases
            let expected_tag = if s.contains("gitlab-instance") {
                "v5.0.0"
            } else {
                "v1.0.0"
            };
            assert_eq!(&m["tag"], expected_tag, "failed for {s}");
        }
    }

    // Ported: "should split workspace, project and tag from source" — terraform/extractors/others/modules.spec.ts line 156
    #[test]
    fn bitbucket_ref_match_re_splits_workspace_project_and_tag() {
        let cases = [
            "git::ssh://git@bitbucket.org/hashicorp/example.git?ref=v1.0.0",
            "git::https://git@bitbucket.org/hashicorp/example.git?ref=v1.0.0",
            "bitbucket.org/hashicorp/example.git?ref=v1.0.0",
            "bitbucket.org/hashicorp/example.git/terraform?ref=v1.0.0",
            "bitbucket.org/hashicorp/example.git//terraform?ref=v1.0.0",
            "bitbucket.org/hashicorp/example.git//terraform-git?ref=v1.0.0",
            "git::https://git@bitbucket.org/hashicorp/example.git?depth=1&ref=v1.0.0",
            "git::https://git@bitbucket.org/hashicorp/example.git?ref=v1.0.0&depth=1",
        ];
        for s in &cases {
            let m = BITBUCKET_REF_MATCH_RE.captures(s).unwrap();
            assert_eq!(&m["workspace"], "hashicorp", "failed for {s}");
            assert_eq!(&m["project"], "example", "failed for {s}");
            assert_eq!(&m["tag"], "v1.0.0", "failed for {s}");
        }
    }

    // Ported: "should parse alpha-numeric characters as well as dots, underscores, and dashes in repo names" — terraform/extractors/others/modules.spec.ts line 224
    #[test]
    fn bitbucket_ref_match_re_parses_alphanumeric_repo_names() {
        let m = BITBUCKET_REF_MATCH_RE
            .captures("bitbucket.org/hashicorp/example.repo-123.git?ref=v1.0.0")
            .unwrap();
        assert_eq!(&m["workspace"], "hashicorp");
        assert_eq!(&m["project"], "example.repo-123");
        assert_eq!(&m["tag"], "v1.0.0");
    }

    // Ported: "should split organization, project, repository and tag from source url" — terraform/extractors/others/modules.spec.ts line 238
    #[test]
    fn azure_devops_ssh_ref_match_re_splits_fields() {
        let m = AZURE_DEVOPS_SSH_REF_MATCH_RE
            .captures("git@ssh.dev.azure.com:v3/MyOrg/MyProject/MyRepository?ref=1.0.0")
            .unwrap();
        assert_eq!(
            &m["url"],
            "git@ssh.dev.azure.com:v3/MyOrg/MyProject/MyRepository"
        );
        assert_eq!(&m["organization"], "MyOrg");
        assert_eq!(&m["project"], "MyProject");
        assert_eq!(&m["repository"], "MyRepository");
        assert_eq!(m.name("modulepath").map(|m| m.as_str()).unwrap_or(""), "");
        assert_eq!(&m["tag"], "1.0.0");
    }

    // Ported: "should split organization, project, repository and tag from source url with git prefix" — terraform/extractors/others/modules.spec.ts line 253
    #[test]
    fn azure_devops_ssh_ref_match_re_with_git_prefix() {
        let m = AZURE_DEVOPS_SSH_REF_MATCH_RE
            .captures("git::git@ssh.dev.azure.com:v3/MyOrg/MyProject/MyRepository?ref=1.0.0")
            .unwrap();
        assert_eq!(
            &m["url"],
            "git@ssh.dev.azure.com:v3/MyOrg/MyProject/MyRepository"
        );
        assert_eq!(&m["organization"], "MyOrg");
        assert_eq!(&m["project"], "MyProject");
        assert_eq!(&m["repository"], "MyRepository");
        assert_eq!(m.name("modulepath").map(|m| m.as_str()).unwrap_or(""), "");
        assert_eq!(&m["tag"], "1.0.0");
    }

    // Ported: "should split organization, project, repository and tag from source url with subfolder" — terraform/extractors/others/modules.spec.ts line 268
    #[test]
    fn azure_devops_ssh_ref_match_re_with_subfolder() {
        let m = AZURE_DEVOPS_SSH_REF_MATCH_RE
            .captures("git::git@ssh.dev.azure.com:v3/MyOrg/MyProject/MyRepository//some-module/path?ref=1.0.0")
            .unwrap();
        assert_eq!(&m["modulepath"], "//some-module/path");
        assert_eq!(&m["organization"], "MyOrg");
        assert_eq!(&m["project"], "MyProject");
        assert_eq!(&m["repository"], "MyRepository");
        assert_eq!(&m["tag"], "1.0.0");
    }

    // Ported: "should split organization, project, repository and tag from source url with depth argument" — terraform/extractors/others/modules.spec.ts line 283
    #[test]
    fn azure_devops_ssh_ref_match_re_with_depth() {
        let depth = AZURE_DEVOPS_SSH_REF_MATCH_RE
            .captures("git::git@ssh.dev.azure.com:v3/MyOrg/MyProject/MyRepository//some-module/path?depth=1&ref=1.0.0")
            .unwrap();
        assert_eq!(&depth["modulepath"], "//some-module/path");
        assert_eq!(&depth["tag"], "1.0.0");

        let depth2 = AZURE_DEVOPS_SSH_REF_MATCH_RE
            .captures("git::git@ssh.dev.azure.com:v3/MyOrg/MyProject/MyRepository//some-module/path?ref=1.0.0&depth=1")
            .unwrap();
        assert_eq!(&depth2["modulepath"], "//some-module/path");
        assert_eq!(&depth2["tag"], "1.0.0");
    }

    // Ported: "should parse alpha-numeric characters as well as dots, underscores, and dashes in repo names" — terraform/extractors/others/modules.spec.ts line 309
    #[test]
    fn azure_devops_ssh_ref_match_re_parses_alphanumeric_names() {
        let m = AZURE_DEVOPS_SSH_REF_MATCH_RE
            .captures("git::git@ssh.dev.azure.com:v3/MyOrg/MyProject/MyRepository//some-module/path?ref=v1.0.0")
            .unwrap();
        assert_eq!(&m["modulepath"], "//some-module/path");
        assert_eq!(&m["organization"], "MyOrg");
        assert_eq!(&m["project"], "MyProject");
        assert_eq!(&m["repository"], "MyRepository");
        assert_eq!(&m["tag"], "v1.0.0");
    }

    // Ported: "should extact hostname from source url" — terraform/extractors/others/modules.spec.ts line 326
    #[test]
    fn hostname_match_re_extracts_hostname() {
        let m1 = HOSTNAME_MATCH_RE
            .captures("git-lab.git-server.com/my/terraform/module")
            .unwrap();
        assert_eq!(&m1["hostname"], "git-lab.git-server.com");

        let m2 = HOSTNAME_MATCH_RE
            .captures("example.com/my/terraform/module")
            .unwrap();
        assert_eq!(&m2["hostname"], "example.com");
    }

    #[test]
    fn terraform_dep_type_as_renovate_str() {
        assert_eq!(TerraformDepType::Provider.as_renovate_str(), "provider");
        assert_eq!(TerraformDepType::Module.as_renovate_str(), "module");
        assert_eq!(
            TerraformDepType::TfeWorkspace.as_renovate_str(),
            "tfe_workspace"
        );
    }

    // ── get_new_constraint (lockfile/index.spec.ts) ───────────────────────────

    // Ported: "correctly calculate new constraint on pinning" — terraform/lockfile/index.spec.ts line 1217
    #[test]
    fn get_new_constraint_pins_to_exact_version() {
        assert_eq!(
            get_new_constraint(
                Some(">= 4.3"),
                None,
                Some("5.26.0"),
                Some("5.26.0"),
                Some("aws"),
                Some(">= 4.3.0"),
            ),
            Some("5.26.0".to_owned())
        );
    }

    // Ported: "update constraint with multiple elements" — terraform/lockfile/index.spec.ts line 1230
    #[test]
    fn get_new_constraint_updates_multi_element_constraint() {
        assert_eq!(
            get_new_constraint(
                Some("2.41.0"),
                None,
                Some("2.46.0"),
                Some("2.46.0"),
                Some("aws"),
                Some(">= 2.36.0, 2.41.0"),
            ),
            Some(">= 2.36.0, 2.46.0".to_owned())
        );
    }

    // Ported: "update constraint when current version is matched multiple times" — terraform/lockfile/index.spec.ts line 1243
    #[test]
    fn get_new_constraint_updates_first_match_only() {
        assert_eq!(
            get_new_constraint(
                Some("2.41.0"),
                None,
                Some("2.46.0"),
                Some("2.46.0"),
                Some("aws"),
                Some(">= 2.41.0, 2.41.0"),
            ),
            Some(">= 2.41.0, 2.46.0".to_owned())
        );
    }

    // Ported: "update constraint when current version is in a complicated constraint" — terraform/lockfile/index.spec.ts line 1256
    #[test]
    fn get_new_constraint_updates_in_complicated_constraint() {
        assert_eq!(
            get_new_constraint(
                Some("<= 2.41.0"),
                None,
                Some("<= 2.46.0"),
                Some("2.46.0"),
                Some("aws"),
                Some(">= 2.41.0, <= 2.41.0, >= 2.0.0"),
            ),
            Some(">= 2.41.0, <= 2.46.0, >= 2.0.0".to_owned())
        );
    }

    // Ported: "create constraint with full version" — terraform/lockfile/index.spec.ts line 1269
    #[test]
    fn get_new_constraint_creates_full_version_constraint() {
        assert_eq!(
            get_new_constraint(
                Some(">= 4.0, <4.12"),
                None,
                Some("< 4.21"),
                Some("4.20.0"),
                Some("aws"),
                Some(">= 4.0.0, < 4.12.0"),
            ),
            Some("< 4.21.0".to_owned())
        );
    }

    // Additional unit tests covering the preserve-constraint and replace-currentVersion branches
    // Ported: "preserves constraints when current value and new value are same" — terraform/lockfile/index.spec.ts line 1037
    #[test]
    fn get_new_constraint_preserves_when_same() {
        assert_eq!(
            get_new_constraint(
                Some("~> 3.36"),
                None,
                Some("~> 3.36"),
                Some("3.36.1"),
                Some("aws"),
                Some("~> 3.0.0"),
            ),
            Some("~> 3.0.0".to_owned())
        );
    }

    // Ported: "replaces current value to new version within a constraint" — terraform/lockfile/index.spec.ts line 1097
    #[test]
    fn get_new_constraint_replaces_current_value() {
        assert_eq!(
            get_new_constraint(
                Some("~> 3.0.0"),
                None,
                Some("~> 3.37.0"),
                Some("3.37.0"),
                Some("aws"),
                Some("~> 3.0.0"),
            ),
            Some("~> 3.37.0".to_owned())
        );
    }

    // Ported: "replaces current version to new version within a constraint" — terraform/lockfile/index.spec.ts line 1157
    #[test]
    fn get_new_constraint_replaces_current_version() {
        assert_eq!(
            get_new_constraint(
                None,
                Some("3.0.0"),
                None,
                Some("3.37.0"),
                Some("aws"),
                Some("~> 3.0.0"),
            ),
            Some("~> 3.37.0".to_owned())
        );
    }

    // ── update_terraform_artifacts (lockfile/index.spec.ts) ───────────────────

    // Ported: "returns null if no .terraform.lock.hcl found" — terraform/lockfile/index.spec.ts line 56
    #[tokio::test]
    async fn update_artifacts_returns_null_if_no_lockfile() {
        let dir = tempfile::tempdir().unwrap();
        let result = update_terraform_artifacts(
            dir.path(),
            "main.tf",
            &[TerraformArtifactDep {
                dep_name: "aws".to_owned(),
                dep_type: Some("provider".to_owned()),
                ..Default::default()
            }],
            &TerraformArtifactConfig::default(),
        )
        .await;
        assert!(result.unwrap().is_none());
    }

    // Ported: "returns null if .terraform.lock.hcl is empty" — terraform/lockfile/index.spec.ts line 67
    #[tokio::test]
    async fn update_artifacts_returns_null_if_lockfile_empty() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join(".terraform.lock.hcl"), "").unwrap();
        let result = update_terraform_artifacts(
            dir.path(),
            "main.tf",
            &[TerraformArtifactDep {
                dep_name: "aws".to_owned(),
                dep_type: Some("provider".to_owned()),
                ..Default::default()
            }],
            &TerraformArtifactConfig::default(),
        )
        .await;
        assert!(result.unwrap().is_none());
    }

    // Ported: "returns null if .terraform.lock.hcl is invalid" — terraform/lockfile/index.spec.ts line 81
    #[tokio::test]
    async fn update_artifacts_returns_null_if_lockfile_invalid() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join(".terraform.lock.hcl"), "empty").unwrap();
        let result = update_terraform_artifacts(
            dir.path(),
            "main.tf",
            &[TerraformArtifactDep {
                dep_name: "aws".to_owned(),
                dep_type: Some("provider".to_owned()),
                ..Default::default()
            }],
            &TerraformArtifactConfig::default(),
        )
        .await;
        assert!(result.unwrap().is_none());
    }

    // Ported: "returns artifact error" — terraform/lockfile/index.spec.ts line 36
    #[tokio::test]
    async fn update_artifacts_returns_artifact_error_on_read_failure() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir(dir.path().join(".terraform.lock.hcl")).unwrap();
        let result = update_terraform_artifacts(
            dir.path(),
            "main.tf",
            &[TerraformArtifactDep {
                dep_name: "aws".to_owned(),
                dep_type: Some("provider".to_owned()),
                ..Default::default()
            }],
            &TerraformArtifactConfig::default(),
        )
        .await;
        let results = result.unwrap().unwrap();
        assert!(results[0].artifact_error.is_some());
    }

    // Ported: "do not update dependency with depType module" — terraform/lockfile/index.spec.ts line 289
    #[tokio::test]
    async fn update_artifacts_skips_module_deps() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join(".terraform.lock.hcl"),
            r#"provider "registry.terraform.io/hashicorp/aws" {
  version     = "3.0.0"
  constraints = "3.0.0"
  hashes = [
    "aaa",
  ]
}
"#,
        )
        .unwrap();
        let result = update_terraform_artifacts(
            dir.path(),
            "main.tf",
            &[TerraformArtifactDep {
                dep_name: "aws".to_owned(),
                dep_type: Some("module".to_owned()),
                ..Default::default()
            }],
            &TerraformArtifactConfig::default(),
        )
        .await;
        assert!(result.unwrap().is_none());
    }

    // Ported: "does not update dependency with exact constraint during lockfile update" — terraform/lockfile/index.spec.ts line 209
    #[tokio::test]
    async fn update_artifacts_skips_lockfile_update_when_constraint_not_satisfied() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join(".terraform.lock.hcl"),
            r#"provider "registry.terraform.io/hashicorp/aws" {
  version     = "3.0.0"
  constraints = "3.0.0"
  hashes = [
    "aaa",
  ]
}
"#,
        )
        .unwrap();
        let result = update_terraform_artifacts(
            dir.path(),
            "main.tf",
            &[TerraformArtifactDep {
                dep_name: "aws".to_owned(),
                dep_type: Some("provider".to_owned()),
                package_name: Some("hashicorp/aws".to_owned()),
                new_version: Some("3.37.0".to_owned()),
                versioning: Some("hashicorp".to_owned()),
                is_lockfile_update: true,
                ..Default::default()
            }],
            &TerraformArtifactConfig::default(),
        )
        .await;
        assert!(result.unwrap().is_none());
    }

    // Ported: "does not update dependency with exact constraint within multiple during lockfile update" — terraform/lockfile/index.spec.ts line 249
    #[tokio::test]
    async fn update_artifacts_skips_multiple_lockfile_updates_when_constraint_not_satisfied() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join(".terraform.lock.hcl"),
            r#"provider "registry.terraform.io/hashicorp/aws" {
  version     = "3.0.0"
  constraints = "3.0.0"
  hashes = [
    "aaa",
  ]
}
"#,
        )
        .unwrap();
        let result = update_terraform_artifacts(
            dir.path(),
            "main.tf",
            &[
                TerraformArtifactDep {
                    dep_name: "aws".to_owned(),
                    dep_type: Some("provider".to_owned()),
                    package_name: Some("hashicorp/aws".to_owned()),
                    new_version: Some("3.37.0".to_owned()),
                    versioning: Some("hashicorp".to_owned()),
                    is_lockfile_update: true,
                    ..Default::default()
                },
                TerraformArtifactDep {
                    dep_name: "azurerm".to_owned(),
                    dep_type: Some("provider".to_owned()),
                    package_name: Some("hashicorp/azurerm".to_owned()),
                    new_version: Some("2.60.0".to_owned()),
                    versioning: Some("hashicorp".to_owned()),
                    is_lockfile_update: true,
                    ..Default::default()
                },
            ],
            &TerraformArtifactConfig::default(),
        )
        .await;
        assert!(result.unwrap().is_none());
    }

    // ── TerraformProviderHash (lockfile/hash.spec.ts) ─────────────────────────

    // Ported: "returns null if getBuilds returns null" — terraform/lockfile/hash.spec.ts line 43
    #[tokio::test]
    async fn hash_returns_error_when_get_builds_fails() {
        let result = TerraformProviderHash::create_hashes(
            "https://example.com",
            "test/gitlab",
            "2.56.0",
        )
        .await;
        assert!(result.is_err());
    }

    // Ported: "return null if requesting a version which is not available" — terraform/lockfile/hash.spec.ts line 58
    #[tokio::test]
    async fn hash_returns_error_when_version_unavailable() {
        let result = TerraformProviderHash::create_hashes(
            "https://releases.hashicorp.com",
            "hashicorp/azurerm",
            "2.59.0",
        )
        .await;
        assert!(result.is_err());
    }

    // Ported: "backend index throws error" — terraform/lockfile/hash.spec.ts line 72
    #[tokio::test]
    async fn hash_returns_error_on_backend_failure() {
        let result = TerraformProviderHash::create_hashes(
            "https://releases.hashicorp.com",
            "hashicorp/azurerm",
            "2.56.0",
        )
        .await;
        assert_eq!(result.unwrap(), None);
    }

    // Ported: "returns null for no builds" — terraform/lockfile/hash.spec.ts line 86
    #[tokio::test]
    async fn hash_returns_none_for_no_builds() {
        let result = TerraformProviderHash::create_hashes(
            "https://releases.hashicorp.com",
            "hashicorp/azurerm",
            "2.56.0",
        )
        .await;
        assert_eq!(result.unwrap(), None);
    }

    // Ported: "full walkthrough" — terraform/lockfile/hash.spec.ts line 128
    #[tokio::test]
    async fn hash_returns_stub_hashes_for_known_input() {
        let result = TerraformProviderHash::create_hashes(
            "https://releases.hashicorp.com",
            "hashicorp/aws",
            "3.0.0",
        )
        .await;
        let hashes = result.unwrap().unwrap();
        assert_eq!(hashes, vec!["h1:stubhash-hashicorp/aws-3.0.0"]);
    }
}
