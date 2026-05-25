//! Azure Bicep resource type datasource.
//!
//! Fetches available API versions for Azure Bicep resource types from the
//! bicep-types-az GitHub repository index.
//!
//! Renovate reference: `lib/modules/datasource/azure-bicep-resource/index.ts`
//!
//! ## API
//!
//! `GET https://raw.githubusercontent.com/Azure/bicep-types-az/main/generated/index.json`
//!
//! Response: `{ "resources": { "microsoft.foo/bar@2024-01-01": {}, ... } }`

use std::collections::HashMap;
use std::sync::OnceLock;

use serde::Deserialize;

use crate::http::HttpClient;

pub const BICEP_INDEX_URL: &str =
    "https://raw.githubusercontent.com/Azure/bicep-types-az/main/generated/index.json";

#[derive(Debug, Deserialize)]
struct BicepIndexRaw {
    resources: HashMap<String, serde_json::Value>,
}

/// Process-level cache of the Bicep resource version index.
static VERSION_INDEX: OnceLock<HashMap<String, Vec<String>>> = OnceLock::new();

/// Update summary from the Bicep datasource.
#[derive(Debug)]
pub struct BicepUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Error from the Bicep datasource.
#[derive(Debug, thiserror::Error)]
pub enum BicepError {
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("resource type not found: {0}")]
    NotFound(String),
    #[error("JSON parse error: {0}")]
    Parse(String),
}

/// One release entry from `fetch_releases`.
#[derive(Debug, Clone, PartialEq)]
pub struct BicepRelease {
    pub version: String,
    pub changelog_url: String,
}

/// Result of `fetch_releases`.
#[derive(Debug, Clone)]
pub struct BicepReleasesResult {
    pub releases: Vec<BicepRelease>,
}

fn changelog_base(package_name_lower: &str) -> String {
    let slash = package_name_lower.find('/').unwrap_or(package_name_lower.len());
    let namespace = &package_name_lower[..slash];
    let type_ = &package_name_lower[slash + 1..];
    format!(
        "https://learn.microsoft.com/en-us/azure/templates/{}/change-log/{}",
        namespace, type_
    )
}

/// Fetch all API versions for `package_name` from the given index URL.
///
/// Returns `Ok(None)` when the package is not found in `resources`.
pub async fn fetch_releases(
    index_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<BicepReleasesResult>, BicepError> {
    let body = http
        .get_raw_with_accept(index_url, "application/json")
        .await
        .map_err(|e| BicepError::Http(e.to_string()))?;

    let raw: BicepIndexRaw =
        serde_json::from_str(&body).map_err(|e| BicepError::Parse(e.to_string()))?;

    let pkg_lower = package_name.to_lowercase();
    let mut versions: Vec<String> = raw
        .resources
        .keys()
        .filter_map(|k| {
            let lower = k.to_lowercase();
            let (type_name, version) = lower.split_once('@')?;
            if type_name == pkg_lower { Some(version.to_owned()) } else { None }
        })
        .collect();

    if versions.is_empty() {
        return Ok(None);
    }
    versions.sort();

    let base = changelog_base(&pkg_lower);
    let releases = versions
        .into_iter()
        .map(|v| BicepRelease { changelog_url: format!("{}#{}", base, v), version: v })
        .collect();

    Ok(Some(BicepReleasesResult { releases }))
}

/// Fetch the latest API version for `resource_type` from the Bicep index.
///
/// `resource_type` is lowercase `namespace.provider/type`, e.g.
/// `microsoft.containerservice/managedclusters`.
pub async fn fetch_latest(
    http: &HttpClient,
    resource_type: &str,
    current_value: &str,
) -> Result<BicepUpdateSummary, BicepError> {
    let index = get_or_fetch_index(http).await?;

    let type_lower = resource_type.to_lowercase();
    let versions = index
        .get(&type_lower)
        .ok_or_else(|| BicepError::NotFound(resource_type.to_owned()))?;

    if versions.is_empty() {
        return Err(BicepError::NotFound(resource_type.to_owned()));
    }

    // Latest is the lexicographically largest version (ISO 8601 date format).
    let latest = versions.iter().max().cloned();
    let update_available = latest.as_deref() != Some(current_value);

    Ok(BicepUpdateSummary {
        current_value: current_value.to_owned(),
        latest,
        update_available,
    })
}

async fn get_or_fetch_index(
    http: &HttpClient,
) -> Result<&'static HashMap<String, Vec<String>>, BicepError> {
    if let Some(index) = VERSION_INDEX.get() {
        return Ok(index);
    }

    let body = http
        .get_raw_with_accept(BICEP_INDEX_URL, "application/json")
        .await
        .map_err(|e| BicepError::Http(e.to_string()))?;

    let raw: BicepIndexRaw =
        serde_json::from_str(&body).map_err(|e| BicepError::Parse(e.to_string()))?;

    let mut index: HashMap<String, Vec<String>> = HashMap::new();
    for key in raw.resources.keys() {
        let lower = key.to_lowercase();
        if let Some((type_name, version)) = lower.split_once('@') {
            index
                .entry(type_name.to_owned())
                .or_default()
                .push(version.to_owned());
        }
    }

    // Sort versions for deterministic output.
    for versions in index.values_mut() {
        versions.sort();
    }

    let _ = VERSION_INDEX.set(index);
    Ok(VERSION_INDEX.get().unwrap())
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    const INDEX_PATH: &str = "/Azure/bicep-types-az/main/generated/index.json";

    async fn mock_index(server: &MockServer, body: &str) {
        Mock::given(method("GET"))
            .and(path(INDEX_PATH))
            .respond_with(ResponseTemplate::new(200).set_body_string(body.to_owned()))
            .mount(server)
            .await;
    }

    fn index_url(server: &MockServer) -> String {
        format!("{}{}", server.uri(), INDEX_PATH)
    }

    // Ported: "should return null when no version is found" — datasource/azure-bicep-resource/index.spec.ts line 10
    #[tokio::test]
    async fn should_return_null_when_no_version_is_found() {
        let server = MockServer::start().await;
        mock_index(&server, r#"{"resources":{},"resourceFunctions":{}}"#).await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&index_url(&server), "unknown", &http).await.unwrap();
        assert!(result.is_none());
    }

    // Ported: "should return null when package is a function" — datasource/azure-bicep-resource/index.spec.ts line 32
    #[tokio::test]
    async fn should_return_null_when_package_is_a_function() {
        let server = MockServer::start().await;
        let body = r#"{"resources":{},"resourceFunctions":{"microsoft.billing/billingaccounts":{"2019-10-01-preview":[{"$ref":"billing/microsoft.billing/2019-10-01-preview/types.json#/304"}],"2020-05-01":[{"$ref":"billing/microsoft.billing/2020-05-01/types.json#/287"}]}}}"#;
        mock_index(&server, body).await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&index_url(&server), "unknown", &http).await.unwrap();
        assert!(result.is_none());
    }

    // Ported: "should return versions when package is a resource" — datasource/azure-bicep-resource/index.spec.ts line 67
    #[tokio::test]
    async fn should_return_versions_when_package_is_a_resource() {
        let server = MockServer::start().await;
        let body = r#"{"resources":{"Microsoft.Storage/storageAccounts@2015-05-01-preview":{"$ref":"storage/microsoft.storage/2015-05-01-preview/types.json#/31"},"Microsoft.Storage/storageAccounts@2018-02-01":{"$ref":"storage/microsoft.storage/2018-02-01/types.json#/85"}},"resourceFunctions":{}}"#;
        mock_index(&server, body).await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(
            &index_url(&server),
            "Microsoft.Storage/storageAccounts",
            &http,
        )
        .await
        .unwrap()
        .unwrap();

        assert_eq!(result.releases.len(), 2);
        assert_eq!(result.releases[0], BicepRelease {
            version: "2015-05-01-preview".to_owned(),
            changelog_url: "https://learn.microsoft.com/en-us/azure/templates/microsoft.storage/change-log/storageaccounts#2015-05-01-preview".to_owned(),
        });
        assert_eq!(result.releases[1], BicepRelease {
            version: "2018-02-01".to_owned(),
            changelog_url: "https://learn.microsoft.com/en-us/azure/templates/microsoft.storage/change-log/storageaccounts#2018-02-01".to_owned(),
        });
    }

    // Ported: "should return versions when package is a resource and a function" — datasource/azure-bicep-resource/index.spec.ts line 109
    #[tokio::test]
    async fn should_return_versions_when_package_is_a_resource_and_a_function() {
        let server = MockServer::start().await;
        let body = r#"{"resources":{"Microsoft.OperationalInsights/workspaces@2023-09-01":{"$ref":"operationalinsights/microsoft.operationalinsights/2023-09-01/types.json#/31"}},"resourceFunctions":{"microsoft.operationalinsights/workspaces":{"2015-03-20":[{"$ref":"operationalinsights/workspaces/2015-03-20/types.json#/304"}]}}}"#;
        mock_index(&server, body).await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(
            &index_url(&server),
            "Microsoft.OperationalInsights/workspaces",
            &http,
        )
        .await
        .unwrap()
        .unwrap();

        assert_eq!(result.releases.len(), 1);
        assert_eq!(result.releases[0], BicepRelease {
            version: "2023-09-01".to_owned(),
            changelog_url: "https://learn.microsoft.com/en-us/azure/templates/microsoft.operationalinsights/change-log/workspaces#2023-09-01".to_owned(),
        });
    }
}
