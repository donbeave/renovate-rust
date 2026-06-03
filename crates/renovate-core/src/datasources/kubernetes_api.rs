//! Kubernetes API version datasource.
//!
//! Returns known API versions for a given Kubernetes resource type from a
//! static embedded data file. No network calls required.
//!
//! Renovate reference: `lib/modules/datasource/kubernetes-api/index.ts`
//! Data:               `data/kubernetes-api.json5`

use std::collections::HashMap;

use std::sync::OnceLock;

pub const DATASOURCE_ID: &str = "kubernetes-api";

static API_DATA: OnceLock<HashMap<String, Vec<String>>> = OnceLock::new();

const RAW_JSON5: &str = include_str!("../../../../../renovate/data/kubernetes-api.json5");

fn load_api_data() -> HashMap<String, Vec<String>> {
    json5::from_str(RAW_JSON5).unwrap_or_default()
}

fn api_data() -> &'static HashMap<String, Vec<String>> {
    API_DATA.get_or_init(load_api_data)
}

/// Result of a `get_releases` call.
#[derive(Debug, Clone)]
pub struct KubernetesApiRelease {
    pub version: String,
}

/// Result of a `get_releases` call.
#[derive(Debug, Clone)]
pub struct KubernetesApiResult {
    pub releases: Vec<KubernetesApiRelease>,
}

/// Look up known API versions for a Kubernetes resource type.
///
/// Returns `None` for unknown or incorrectly-cased resource names.
pub fn get_releases(package_name: &str) -> Option<KubernetesApiResult> {
    let versions = api_data().get(package_name)?;
    Some(KubernetesApiResult {
        releases: versions
            .iter()
            .map(|v| KubernetesApiRelease { version: v.clone() })
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "returns null for an unknown Kubernetes API type" — lib/modules/datasource/kubernetes-api/index.spec.ts line 8
    #[test]
    fn unknown_type_returns_none() {
        assert!(get_releases("Unknown").is_none());
    }

    // Ported: "returns for a known Kubernetes API type" — lib/modules/datasource/kubernetes-api/index.spec.ts line 13
    #[test]
    fn known_type_returns_versions() {
        let result = get_releases("CSIStorageCapacity").unwrap();
        let versions: Vec<&str> = result.releases.iter().map(|r| r.version.as_str()).collect();
        assert!(versions.contains(&"storage.k8s.io/v1beta1"));
        assert!(versions.contains(&"storage.k8s.io/v1"));
    }

    // Ported: "is case sensitive" — lib/modules/datasource/kubernetes-api/index.spec.ts line 27
    #[test]
    fn lookup_is_case_sensitive() {
        assert!(get_releases("csistoragecapacity").is_none());
        assert!(get_releases("CSIStorageCapacity").is_some());
    }
}
