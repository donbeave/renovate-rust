//! Azure Bicep (`.bicep`) resource type version extractor.
//!
//! Scans Bicep files for `resource` declarations with typed API versions
//! and extracts them for Azure Bicep resource datasource lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/bicep/extract.ts`
//! - Pattern: `/\.bicep$/`
//! - Datasource: azure-bicep-resource
//!
//! ## File format
//!
//! ```bicep
//! resource cluster 'Microsoft.ContainerService/managedClusters@2024-01-01' = {
//!   // ...
//! }
//! ```
//!
//! The resource type string is `'NamespaceName.Provider/Type@version'`.

use std::sync::LazyLock;

use regex::Regex;

/// A single extracted Bicep resource type dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BicepDep {
    /// Resource type path, e.g. `Microsoft.ContainerService/managedClusters`.
    pub dep_name: String,
    /// API version string, e.g. `2024-01-01`.
    pub current_value: String,
}

/// Matches `resource Name 'NamespaceName.Provider/Type[@Subtype...]@version'`
/// depName must contain at least one dot (namespace separator) and one slash (provider/type).
static RESOURCE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"resource\s+\w+\s+'(.+\..+/.+)@([^']+)'").unwrap());

/// Extract Azure Bicep resource type deps from a `.bicep` file.
pub fn extract(content: &str) -> Vec<BicepDep> {
    content
        .lines()
        .filter(|line| {
            let t = line.trim();
            !t.is_empty() && !t.starts_with("//")
        })
        .filter_map(|line| {
            let cap = RESOURCE_RE.captures(line)?;
            Some(BicepDep {
                dep_name: cap[1].to_owned(),
                current_value: cap[2].to_owned(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "should extract a normal resource" — bicep/extract.spec.ts line 5
    #[test]
    fn extracts_resource_declaration() {
        let content = r#"
resource cluster 'Microsoft.ContainerService/managedClusters@2024-01-01' = {
  name: 'myCluster'
}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].dep_name,
            "Microsoft.ContainerService/managedClusters"
        );
        assert_eq!(deps[0].current_value, "2024-01-01");
    }

    // Ported: "should extract a normal resource" — bicep/extract.spec.ts line 5
    #[test]
    fn extracts_multiple_resources() {
        let content = r#"
resource vnet 'Microsoft.Network/virtualNetworks@2023-04-01' = {}
resource nsg 'Microsoft.Network/networkSecurityGroups@2023-04-01' = {}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0].dep_name, "Microsoft.Network/virtualNetworks");
        assert_eq!(deps[1].dep_name, "Microsoft.Network/networkSecurityGroups");
    }

    // Ported: "should not extract a commented out resource" — bicep/extract.spec.ts line 35
    #[test]
    fn comment_lines_skipped() {
        let content = r#"
// resource cluster 'Microsoft.ContainerService/managedClusters@2024-01-01' = {}
resource vnet 'Microsoft.Network/virtualNetworks@2023-04-01' = {}
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "Microsoft.Network/virtualNetworks");
    }

    // Ported: "should not extract a commented out resource" — bicep/extract.spec.ts line 35
    #[test]
    fn no_resources_returns_empty() {
        assert!(extract("param location string = 'eastus'").is_empty());
    }

    // Ported: "should extract a normal resource" — bicep/extract.spec.ts line 5
    #[test]
    fn preview_version_captured() {
        let content =
            "resource storage 'Microsoft.Storage/storageAccounts@2023-05-01-preview' = {}";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "2023-05-01-preview");
    }

    // Ported: "should extract a conditional resource" — bicep/extract.spec.ts line 58
    #[test]
    fn extracts_conditional_resource() {
        let content = "resource s 'Microsoft.Storage/storageAccounts@2022-09-01' = if(0 == 1) {}";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "Microsoft.Storage/storageAccounts");
        assert_eq!(deps[0].current_value, "2022-09-01");
    }

    // Ported: "should extract a existing resource" — bicep/extract.spec.ts line 97
    #[test]
    fn extracts_existing_resource() {
        let content = "resource s 'Microsoft.Storage/storageAccounts@2022-09-01' existing = {}";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "2022-09-01");
    }

    // Ported: "should extract a loop resource" — bicep/extract.spec.ts line 153
    #[test]
    fn extracts_loop_resource() {
        let content =
            "resource s 'Microsoft.Storage/storageAccounts@2022-09-01' = [for x in []: {}]";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "2022-09-01");
    }

    // Ported: "should not extract a nested unversioned resource" — bicep/extract.spec.ts line 185
    #[test]
    fn nested_unversioned_resource_skipped() {
        // 'blobServices' has no dot → doesn't match depName pattern
        let content = "resource blobServices 'blobServices' = {}";
        assert!(extract(content).is_empty());
    }

    // Ported: "should not extract a nested versioned resource" — bicep/extract.spec.ts line 218
    #[test]
    fn nested_versioned_resource_skipped() {
        // 'blobServices@2022-09-01' has no dot → doesn't match depName pattern
        let content = "resource blobServices 'blobServices@2022-09-01' = {}";
        assert!(extract(content).is_empty());
    }

    // Ported: "should extract a sub resource" — bicep/extract.spec.ts line 252
    #[test]
    fn extracts_sub_resource_with_multiple_slashes() {
        let content = "resource s 'Microsoft.Storage/storageAccounts/blobServices/containers@2022-09-01' = {}";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].dep_name,
            "Microsoft.Storage/storageAccounts/blobServices/containers"
        );
        assert_eq!(deps[0].current_value, "2022-09-01");
    }
}
