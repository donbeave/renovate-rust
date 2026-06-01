//! HCL-to-JSON parser for Terraform files.
//!
//! Ports `lib/modules/manager/terraform/hcl/index.ts` which wraps
//! `@cdktf/hcl2json`. Uses the `hcl-rs` crate to parse HCL into a structured
//! `TerraformDefinitionFile` representation, then exposes it as JSON-compatible
//! `serde_json::Value` for downstream extraction.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TerraformDefinitionFile {
    #[serde(rename = "terraform", skip_serializing_if = "Option::is_none")]
    pub terraform: Option<Vec<TerraformBlock>>,
    #[serde(rename = "module", skip_serializing_if = "Option::is_none")]
    pub module: Option<BTreeMap<String, Vec<TerraformModule>>>,
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<TerraformResources>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<BTreeMap<String, Vec<TerraformProvider>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TerraformBlock {
    #[serde(rename = "required_providers", skip_serializing_if = "Option::is_none")]
    pub required_providers: Option<BTreeMap<String, TerraformRequiredProviderOrString>>,
    #[serde(rename = "required_version", skip_serializing_if = "Option::is_none")]
    pub required_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TerraformRequiredProviderOrString {
    Obj(TerraformRequiredProvider),
    Str(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TerraformRequiredProvider {
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TerraformModule {
    #[serde(rename = "source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TerraformResources {
    #[serde(rename = "helm_release", skip_serializing_if = "Option::is_none")]
    pub helm_release: Option<BTreeMap<String, TerraformHelmRelease>>,
    #[serde(rename = "tfe_workspace", skip_serializing_if = "Option::is_none")]
    pub tfe_workspace: Option<BTreeMap<String, TerraformWorkspace>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TerraformProvider {
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TerraformHelmRelease {
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "repository", skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(rename = "chart", skip_serializing_if = "Option::is_none")]
    pub chart: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TerraformWorkspace {
    #[serde(rename = "terraform_version", skip_serializing_if = "Option::is_none")]
    pub terraform_version: Option<String>,
}

pub fn parse_hcl(content: &str) -> Option<TerraformDefinitionFile> {
    let raw: hcl::Value = match hcl::from_str(content) {
        Ok(v) => v,
        Err(_) => return None,
    };
    let mut json_value = hcl_value_to_json(&raw);
    normalize_hcl_json(&mut json_value);
    let def: TerraformDefinitionFile = match serde_json::from_value(json_value) {
        Ok(d) => d,
        Err(_) => return None,
    };
    Some(def)
}

fn normalize_hcl_json(val: &mut serde_json::Value) {
    let Some(obj) = val.as_object_mut() else {
        return;
    };

    let block_keys = ["terraform"];
    for key in block_keys {
        if let Some(v) = obj.get_mut(key)
            && v.is_object()
        {
            *v = serde_json::Value::Array(vec![v.clone()]);
        }
    }

    let labeled_block_keys = ["module", "provider"];
    for key in labeled_block_keys {
        if let Some(v) = obj.get_mut(key)
            && v.is_object()
        {
            let map = std::mem::take(v).as_object_mut().unwrap().clone();
            let new_map: serde_json::Map<String, serde_json::Value> = map
                .into_iter()
                .map(|(k, inner)| {
                    let wrapped = if inner.is_object() {
                        serde_json::Value::Array(vec![inner])
                    } else {
                        inner
                    };
                    (k, wrapped)
                })
                .collect();
            *v = serde_json::Value::Object(new_map);
        }
    }

    if let Some(v) = obj.get_mut("resource")
        && v.is_object()
    {
        let resource_types = std::mem::take(v).as_object_mut().unwrap().clone();
        let mut result = serde_json::Map::new();
        for (type_name, instances) in resource_types {
            if instances.is_object() {
                result.insert(type_name, instances);
            }
        }
        *v = serde_json::Value::Object(result);
    }
}

pub fn parse_json(content: &str) -> Option<TerraformDefinitionFile> {
    serde_json::from_str(content).ok()
}

fn hcl_value_to_json(v: &hcl::Value) -> serde_json::Value {
    match v {
        hcl::Value::Null => serde_json::Value::Null,
        hcl::Value::Bool(b) => serde_json::Value::Bool(*b),
        hcl::Value::Number(n) => {
            if let Some(u) = n.as_u64() {
                serde_json::Value::Number(u.into())
            } else if let Some(f) = n.as_f64() {
                serde_json::Number::from_f64(f)
                    .map(serde_json::Value::Number)
                    .unwrap_or(serde_json::Value::Null)
            } else {
                serde_json::Value::Null
            }
        }
        hcl::Value::String(s) => serde_json::Value::String(s.clone()),
        hcl::Value::Array(arr) => {
            serde_json::Value::Array(arr.iter().map(hcl_value_to_json).collect())
        }
        hcl::Value::Object(obj) => {
            let map: serde_json::Map<String, serde_json::Value> = obj
                .iter()
                .map(|(k, v)| (k.clone(), hcl_value_to_json(v)))
                .collect();
            serde_json::Value::Object(map)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Rust-specific: terraform_hcl behavior test
    #[test]
    fn parse_hcl_returns_none_for_invalid() {
        assert!(parse_hcl("{{{invalid").is_none());
    }

    // Rust-specific: terraform_hcl behavior test
    #[test]
    fn parse_hcl_extracts_required_provider() {
        let hcl = r#"
            terraform {
              required_providers {
                aws = {
                  source  = "hashicorp/aws"
                  version = "~> 5.0"
                }
              }
            }
        "#;
        let def = parse_hcl(hcl).unwrap();
        let blocks = def.terraform.unwrap();
        assert_eq!(blocks.len(), 1);
        let rp = blocks[0].required_providers.as_ref().unwrap();
        assert!(rp.contains_key("aws"));
    }

    // Rust-specific: terraform_hcl behavior test
    #[test]
    fn parse_hcl_extracts_module() {
        let hcl = r#"
            module "vpc" {
              source  = "terraform-aws-modules/vpc/aws"
              version = "~> 5.0"
            }
        "#;
        let def = parse_hcl(hcl).unwrap();
        let modules = def.module.unwrap();
        assert!(modules.contains_key("vpc"));
        let vpc = &modules["vpc"][0];
        assert_eq!(vpc.source.as_deref(), Some("terraform-aws-modules/vpc/aws"));
        assert_eq!(vpc.version.as_deref(), Some("~> 5.0"));
    }

    // Rust-specific: terraform_hcl behavior test
    #[test]
    fn parse_hcl_extracts_provider() {
        let hcl = r#"
            provider "aws" {
              version = "~> 5.0"
            }
        "#;
        let def = parse_hcl(hcl).unwrap();
        let providers = def.provider.unwrap();
        assert!(providers.contains_key("aws"));
        assert_eq!(providers["aws"][0].version.as_deref(), Some("~> 5.0"));
    }

    // Rust-specific: terraform_hcl behavior test
    #[test]
    fn parse_json_extracts_module() {
        let json = r#"{
            "module": {
                "vpc": [{
                    "source": "terraform-aws-modules/vpc/aws",
                    "version": "~> 5.0"
                }]
            }
        }"#;
        let def = parse_json(json).unwrap();
        let modules = def.module.unwrap();
        assert!(modules.contains_key("vpc"));
    }

    // Rust-specific: terraform_hcl behavior test
    #[test]
    fn parse_json_returns_none_for_invalid() {
        assert!(parse_json("not json").is_none());
    }

    // Rust-specific: terraform_hcl behavior test
    #[test]
    fn parse_hcl_empty_file() {
        let def = parse_hcl("").unwrap();
        assert!(def.terraform.is_none());
        assert!(def.module.is_none());
    }

    // Rust-specific: terraform_hcl behavior test
    #[test]
    fn parse_hcl_required_version() {
        let hcl = r#"
            terraform {
              required_version = "~> 1.5"
            }
        "#;
        let def = parse_hcl(hcl).unwrap();
        let blocks = def.terraform.unwrap();
        assert_eq!(blocks[0].required_version.as_deref(), Some("~> 1.5"));
    }

    // Rust-specific: terraform_hcl behavior test
    #[test]
    fn parse_hcl_helm_release_resource() {
        let hcl = r#"
            resource "helm_release" "nginx" {
              name       = "nginx-ingress"
              repository = "https://kubernetes.github.io/ingress-nginx"
              chart      = "ingress-nginx"
              version    = "4.8.0"
            }
        "#;
        let def = parse_hcl(hcl).unwrap();
        let resources = def.resource.unwrap();
        let helm = resources.helm_release.unwrap();
        assert!(helm.contains_key("nginx"));
        assert_eq!(helm["nginx"].version.as_deref(), Some("4.8.0"));
        assert_eq!(helm["nginx"].chart.as_deref(), Some("ingress-nginx"));
    }

    // Rust-specific: terraform_hcl behavior test
    #[test]
    fn parse_hcl_tfe_workspace_resource() {
        let hcl = r#"
            resource "tfe_workspace" "test" {
              name              = "my-workspace"
              terraform_version = "1.6.0"
            }
        "#;
        let def = parse_hcl(hcl).unwrap();
        let resources = def.resource.unwrap();
        let ws = resources.tfe_workspace.unwrap();
        assert!(ws.contains_key("test"));
        assert_eq!(ws["test"].terraform_version.as_deref(), Some("1.6.0"));
    }
}
