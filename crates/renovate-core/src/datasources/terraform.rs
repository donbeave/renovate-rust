//! Terraform Registry datasource.
//!
//! Fetches available provider and module versions from the Terraform Registry.
//!
//! Renovate reference:
//! - `lib/modules/datasource/terraform-provider/index.ts`
//! - `lib/modules/datasource/terraform-module/index.ts`
//!
//! ## Provider API
//!
//! `GET https://registry.terraform.io/v2/providers/{namespace}/{type}?include=provider-versions`
//!
//! Response: `{ "data": { "attributes": { ... } }, "included": [{ "type": "provider-versions",
//! "attributes": { "version": "5.0.0", "published-at": "..." } }] }`
//!
//! ## Module API
//!
//! `GET https://registry.terraform.io/v1/modules/{namespace}/{name}/{provider}/versions`
//!
//! Response: `{ "modules": [{ "versions": [{ "version": "5.0.0" }, ...] }] }`

use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::HttpClient;

pub const TERRAFORM_REGISTRY: &str = "https://registry.terraform.io";

/// Errors from fetching Terraform Registry metadata.
#[derive(Debug, Error)]
pub enum TerraformError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
    #[error("Invalid package name (expected namespace/type or namespace/name/provider): {0}")]
    InvalidName(String),
}

/// Whether to look up a provider or a module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerraformLookupKind {
    Provider,
    Module,
}

/// Input for a single Terraform dep lookup.
#[derive(Debug, Clone)]
pub struct TerraformDepInput {
    pub name: String,
    pub current_value: String,
    pub kind: TerraformLookupKind,
}

/// Update summary for a Terraform dep.
#[derive(Debug, Clone)]
pub struct TerraformUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Per-dep result from `fetch_updates_concurrent`.
#[derive(Debug)]
pub struct TerraformUpdateResult {
    pub name: String,
    pub summary: Result<TerraformUpdateSummary, TerraformError>,
}

// ── Provider API response ─────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct ProviderV2Response {
    included: Vec<ProviderVersionEntry>,
}

#[derive(Debug, Deserialize)]
struct ProviderVersionEntry {
    #[serde(rename = "type")]
    entry_type: String,
    attributes: ProviderVersionAttributes,
}

#[derive(Debug, Deserialize)]
struct ProviderVersionAttributes {
    version: String,
}

// ── Module API response ───────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct ModuleVersionsResponse {
    modules: Vec<ModuleEntry>,
}

#[derive(Debug, Deserialize)]
struct ModuleEntry {
    versions: Vec<ModuleVersion>,
}

#[derive(Debug, Deserialize)]
struct ModuleVersion {
    version: String,
}

// ── Fetch functions ───────────────────────────────────────────────────────────

/// Fetch the latest version of a Terraform provider.
///
/// `name` must be `{namespace}/{type}` (e.g. `hashicorp/aws`).
pub async fn fetch_provider_latest(
    name: &str,
    http: &HttpClient,
    registry_base: &str,
) -> Result<Option<String>, TerraformError> {
    let (namespace, provider_type) = split_provider_name(name)?;
    let url = format!(
        "{registry_base}/v2/providers/{namespace}/{provider_type}?include=provider-versions"
    );

    let resp = http.get_retrying(&url).await?;
    if resp.status().as_u16() == 404 {
        return Ok(None);
    }
    if !resp.status().is_success() {
        return Ok(None);
    }

    let body: ProviderV2Response = resp.json().await.map_err(TerraformError::Json)?;

    // Included entries are newest-first; find latest provider-versions entry.
    let latest = body
        .included
        .into_iter()
        .filter(|e| e.entry_type == "provider-versions")
        .map(|e| e.attributes.version)
        .next();

    Ok(latest)
}

/// Fetch all releases (versions) of a Terraform provider.
///
/// Returns a Vec of version strings, or `None` if the provider is not found.
pub async fn fetch_provider_releases(
    name: &str,
    http: &HttpClient,
    registry_base: &str,
) -> Result<Option<Vec<String>>, TerraformError> {
    let (namespace, provider_type) = split_provider_name(name)?;
    let url = format!(
        "{registry_base}/v2/providers/{namespace}/{provider_type}?include=provider-versions"
    );

    let resp = http.get_retrying(&url).await?;
    if resp.status().as_u16() == 404 {
        return Ok(None);
    }
    if !resp.status().is_success() {
        return Ok(None);
    }

    let body: ProviderV2Response = resp.json().await.map_err(TerraformError::Json)?;

    let versions: Vec<String> = body
        .included
        .into_iter()
        .filter(|e| e.entry_type == "provider-versions")
        .map(|e| e.attributes.version)
        .collect();

    Ok(if versions.is_empty() { None } else { Some(versions) })
}

/// Fetch the latest version of a Terraform module.
///
/// `name` must be `{namespace}/{name}/{provider}` (e.g. `terraform-aws-modules/vpc/aws`).
pub async fn fetch_module_latest(
    name: &str,
    http: &HttpClient,
    registry_base: &str,
) -> Result<Option<String>, TerraformError> {
    let parts: Vec<&str> = name.splitn(3, '/').collect();
    if parts.len() != 3 {
        return Err(TerraformError::InvalidName(name.to_owned()));
    }
    let url = format!(
        "{registry_base}/v1/modules/{}/{}/{}/versions",
        parts[0], parts[1], parts[2]
    );

    let resp = http.get_retrying(&url).await?;
    if resp.status().as_u16() == 404 {
        return Ok(None);
    }
    if !resp.status().is_success() {
        return Ok(None);
    }

    let body: ModuleVersionsResponse = resp.json().await.map_err(TerraformError::Json)?;

    // Modules → first module → versions → first version (newest).
    let latest = body
        .modules
        .into_iter()
        .next()
        .and_then(|m| m.versions.into_iter().next())
        .map(|v| v.version);

    Ok(latest)
}

/// Fetch update summaries for multiple Terraform deps concurrently.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[TerraformDepInput],
    registry_base: &str,
    concurrency: usize,
) -> Vec<TerraformUpdateResult> {
    if deps.is_empty() {
        return Vec::new();
    }

    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<TerraformUpdateResult> = JoinSet::new();

    for dep in deps {
        let http = http.clone();
        let dep = dep.clone();
        let sem = Arc::clone(&sem);
        let registry_base = registry_base.to_owned();

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_update_summary(&dep, &http, &registry_base).await;
            TerraformUpdateResult {
                name: dep.name.clone(),
                summary: result,
            }
        });
    }

    let mut results = Vec::with_capacity(deps.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok(r) => results.push(r),
            Err(join_err) => tracing::error!(%join_err, "terraform lookup task panicked"),
        }
    }
    results
}

async fn fetch_update_summary(
    dep: &TerraformDepInput,
    http: &HttpClient,
    registry_base: &str,
) -> Result<TerraformUpdateSummary, TerraformError> {
    let latest = match dep.kind {
        TerraformLookupKind::Provider => {
            fetch_provider_latest(&dep.name, http, registry_base).await?
        }
        TerraformLookupKind::Module => fetch_module_latest(&dep.name, http, registry_base).await?,
    };

    let summary = crate::versioning::hashicorp::hashicorp_update_summary(
        &dep.current_value,
        latest.as_deref(),
    );
    Ok(TerraformUpdateSummary {
        current_value: summary.current_value,
        latest: summary.latest,
        update_available: summary.update_available,
    })
}

/// Split `{namespace}/{type}` provider name; use `hashicorp` as default namespace.
fn split_provider_name(name: &str) -> Result<(&str, &str), TerraformError> {
    match name.split_once('/') {
        Some((ns, typ)) => Ok((ns, typ)),
        None => {
            // Bare type name — use hashicorp as the default registry namespace.
            Ok(("hashicorp", name))
        }
    }
}

/// Extract the lower-bound version string from a HashiCorp constraint.
#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    fn provider_resp(versions: &[&str]) -> String {
        let included: Vec<serde_json::Value> = versions
            .iter()
            .map(|v| {
                serde_json::json!({
                    "type": "provider-versions",
                    "id": format!("hashicorp/aws/{v}"),
                    "attributes": { "version": v }
                })
            })
            .collect();
        serde_json::json!({
            "data": { "id": "hashicorp/aws", "type": "providers", "attributes": {} },
            "included": included
        })
        .to_string()
    }

    fn module_resp(versions: &[&str]) -> String {
        let versions_arr: Vec<serde_json::Value> = versions
            .iter()
            .map(|v| serde_json::json!({"version": v}))
            .collect();
        serde_json::json!({
            "modules": [{ "versions": versions_arr }]
        })
        .to_string()
    }

    #[tokio::test]
    async fn fetch_provider_latest_returns_newest() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v2/providers/hashicorp/aws"))
            .and(query_param("include", "provider-versions"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string(provider_resp(&["5.1.0", "5.0.0"])),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_provider_latest("hashicorp/aws", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, Some("5.1.0".to_owned()));
    }

    #[tokio::test]
    async fn fetch_provider_bare_name_uses_hashicorp() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v2/providers/hashicorp/random"))
            .and(query_param("include", "provider-versions"))
            .respond_with(ResponseTemplate::new(200).set_body_string(provider_resp(&["3.5.0"])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_provider_latest("random", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, Some("3.5.0".to_owned()));
    }

    #[tokio::test]
    async fn fetch_module_latest_returns_first_version() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v1/modules/terraform-aws-modules/vpc/aws/versions"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(module_resp(&["5.2.0", "5.1.0", "5.0.0"])),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_module_latest("terraform-aws-modules/vpc/aws", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, Some("5.2.0".to_owned()));
    }

    #[tokio::test]
    async fn fetch_provider_404_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v2/providers/hashicorp/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_provider_latest("hashicorp/nonexistent", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, None);
    }

    // Rust-specific: terraform behavior test
    #[test]
    fn lower_bound_tilde_gt() {
        use crate::versioning::hashicorp::lower_bound;
        assert_eq!(lower_bound("~> 5.0").as_deref(), Some("5.0"));
        assert_eq!(lower_bound(">= 2.0.0").as_deref(), Some("2.0.0"));
        assert_eq!(lower_bound("= 3.1.4").as_deref(), Some("3.1.4"));
    }

    // Rust-specific: terraform behavior test
    #[test]
    fn split_name_slash() {
        let (ns, t) = split_provider_name("hashicorp/aws").unwrap();
        assert_eq!((ns, t), ("hashicorp", "aws"));
    }

    // Rust-specific: terraform behavior test
    #[test]
    fn split_name_bare_defaults_hashicorp() {
        let (ns, t) = split_provider_name("random").unwrap();
        assert_eq!((ns, t), ("hashicorp", "random"));
    }
}

// ---------------------------------------------------------------------------
// Terraform Module utilities — lib/modules/datasource/terraform-module/utils.ts
// ---------------------------------------------------------------------------

/// Join URL parts, ensuring exactly one slash between each part.
fn join_url_parts(a: &str, b: &str) -> String {
    let a = a.trim_end_matches('/');
    let b = b.trim_start_matches('/');
    if b.is_empty() {
        return a.to_owned();
    }
    if a.is_empty() {
        return b.to_owned();
    }
    format!("{a}/{b}")
}

fn is_http_url(s: &str) -> bool {
    s.starts_with("https://") || s.starts_with("http://")
}

/// Build the Service Discovery backend URL for a Terraform registry.
///
/// Mirrors `createSDBackendURL` from
/// `lib/modules/datasource/terraform-module/utils.ts`.
pub fn create_sd_backend_url(
    registry_url: &str,
    sd_type: &str,
    sd_result: &std::collections::HashMap<String, String>,
    sub_path: &str,
) -> String {
    let sd_endpoint = sd_result.get(sd_type).map_or("", |s| s.as_str());
    let full_path = join_url_parts(sd_endpoint, sub_path);
    if is_http_url(&full_path) {
        return full_path;
    }
    join_url_parts(registry_url, &full_path)
}

/// Parse a Terraform package name into (registry, repository).
///
/// Mirrors `getRegistryRepository` from
/// `lib/modules/datasource/terraform-module/utils.ts`.
pub fn get_registry_repository(package_name: &str, registry_url: Option<&str>) -> (String, String) {
    let mut parts: Vec<&str> = package_name.split('/').collect();
    let registry = if parts.len() > 3 && parts[0].contains('.') {
        parts.remove(0).to_owned()
    } else {
        registry_url.unwrap_or("").to_owned()
    };
    let registry = if !is_http_url(&registry) && !registry.is_empty() {
        format!("https://{registry}")
    } else {
        registry
    };
    (registry, parts.join("/"))
}

#[cfg(test)]
mod terraform_module_utils_tests {
    use super::*;

    fn sd(pairs: &[(&str, &str)]) -> std::collections::HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect()
    }

    // Ported: "returns URL with relative SD for modules" — terraform-module/utils.spec.ts line 7
    #[test]
    fn create_sd_backend_url_relative_modules() {
        let result = create_sd_backend_url(
            "https://registry.example.com",
            "modules.v1",
            &sd(&[("modules.v1", "/v1/modules/")]),
            "hashicorp/consul/aws",
        );
        assert_eq!(
            result,
            "https://registry.example.com/v1/modules/hashicorp/consul/aws"
        );
    }

    // Ported: "returns URL with relative SD for providers" — terraform-module/utils.spec.ts line 21
    #[test]
    fn create_sd_backend_url_relative_providers() {
        let result = create_sd_backend_url(
            "https://registry.example.com",
            "providers.v1",
            &sd(&[("providers.v1", "/v1/providers/")]),
            "hashicorp/azure",
        );
        assert_eq!(
            result,
            "https://registry.example.com/v1/providers/hashicorp/azure"
        );
    }

    // Ported: "returns URL with absolute SD for modules" — terraform-module/utils.spec.ts line 35
    #[test]
    fn create_sd_backend_url_absolute_modules() {
        let result = create_sd_backend_url(
            "https://registry.example.com",
            "modules.v1",
            &sd(&[("modules.v1", "https://other.example.com/v1/modules/")]),
            "hashicorp/consul/aws",
        );
        assert_eq!(
            result,
            "https://other.example.com/v1/modules/hashicorp/consul/aws"
        );
    }

    // Ported: "returns URL with absolute SD for providers and missing trailing slash" — terraform-module/utils.spec.ts line 49
    #[test]
    fn create_sd_backend_url_absolute_no_trailing_slash() {
        let result = create_sd_backend_url(
            "https://registry.example.com",
            "providers.v1",
            &sd(&[("providers.v1", "https://other.example.com/v1/providers")]),
            "hashicorp/azure",
        );
        assert_eq!(
            result,
            "https://other.example.com/v1/providers/hashicorp/azure"
        );
    }

    // Ported: "returns URL with with empty SD" — terraform-module/utils.spec.ts line 63
    #[test]
    fn create_sd_backend_url_empty_sd() {
        let result = create_sd_backend_url(
            "https://registry.example.com",
            "providers.v1",
            &sd(&[("providers.v1", "")]),
            "hashicorp/azure",
        );
        assert_eq!(result, "https://registry.example.com/hashicorp/azure");
    }

    // Ported: "returns URL with with missing SD" — terraform-module/utils.spec.ts line 75
    #[test]
    fn create_sd_backend_url_missing_sd() {
        let result = create_sd_backend_url(
            "https://registry.example.com",
            "providers.v1",
            &sd(&[]),
            "hashicorp/azure",
        );
        assert_eq!(result, "https://registry.example.com/hashicorp/azure");
    }

    // Ported: "uses the configured registry URL for standard package names" — terraform-module/utils.spec.ts line 87
    #[test]
    fn get_registry_repository_standard() {
        let (registry, repo) = get_registry_repository(
            "hashicorp/consul/aws",
            Some("https://registry.terraform.io"),
        );
        assert_eq!(registry, "https://registry.terraform.io");
        assert_eq!(repo, "hashicorp/consul/aws");
    }

    // Ported: "extracts the registry from packageName when it is embedded" — terraform-module/utils.spec.ts line 99
    #[test]
    fn get_registry_repository_embedded() {
        let (registry, repo) =
            get_registry_repository("registry.terraform.io/hashicorp/consul/aws", None);
        assert_eq!(registry, "https://registry.terraform.io");
        assert_eq!(repo, "hashicorp/consul/aws");
    }

    // Ported: "normalizes an embedded registry without a scheme" — terraform-module/utils.spec.ts line 111
    #[test]
    fn get_registry_repository_no_scheme() {
        let (registry, repo) =
            get_registry_repository("terraform.company.com/hashicorp/consul/aws", Some(""));
        assert_eq!(registry, "https://terraform.company.com");
        assert_eq!(repo, "hashicorp/consul/aws");
    }
}
