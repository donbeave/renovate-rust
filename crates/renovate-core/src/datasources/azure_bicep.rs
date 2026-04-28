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
