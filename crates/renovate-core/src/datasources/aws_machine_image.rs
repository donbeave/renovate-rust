//! AWS Machine Image datasource.
//!
//! Fetches AMI versions from the AWS EC2 DescribeImages API.
//!
//! Renovate reference: `lib/modules/datasource/aws-machine-image/index.ts`
//! API: `POST https://ec2.{region}.amazonaws.com/` (DescribeImages action)

use serde::Deserialize;
use thiserror::Error;

use crate::datasources::{Release, ReleaseResult};
use crate::http::HttpClient;

pub const DATASOURCE_ID: &str = "aws-machine-image";

#[derive(Debug, Error)]
pub enum AwsMachineImageError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
    #[error("no images found for the given filters")]
    NotFound,
}

#[derive(Debug, Clone)]
pub struct AwsMachineImageConfig {
    pub region: String,
    pub profile: Option<String>,
    pub filters: Vec<AmiFilter>,
}

#[derive(Debug, Clone)]
pub struct AmiFilter {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct DescribeImagesResponse {
    #[serde(default)]
    images: Vec<AmiImage>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AmiImage {
    image_id: Option<String>,
    #[allow(dead_code)]
    name: Option<String>,
    creation_date: Option<String>,
    deprecation_time: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AwsMachineImageRelease {
    pub image_id: String,
    pub name: Option<String>,
    pub creation_date: Option<String>,
    pub is_deprecated: bool,
}

pub async fn fetch_versions(
    http: &HttpClient,
    config: &AwsMachineImageConfig,
    api_base: Option<&str>,
) -> Result<ReleaseResult, AwsMachineImageError> {
    let base = api_base
        .map(|s| s.to_owned())
        .unwrap_or_else(|| format!("https://ec2.{}.amazonaws.com", config.region));

    let filters_json: Vec<serde_json::Value> = config
        .filters
        .iter()
        .map(|f| {
            serde_json::json!({
                "Name": f.name,
                "Values": f.values,
            })
        })
        .collect();

    let _body = serde_json::json!({
        "Filters": filters_json,
    });

    let url = format!("{}/?Action=DescribeImages", base);
    let resp = http.get_retrying(&url).await?;

    if !resp.status().is_success() {
        return Err(AwsMachineImageError::Http(crate::http::HttpError::Status {
            status: resp.status(),
            url,
        }));
    }

    let result: DescribeImagesResponse = resp.json().await?;

    let mut images = result.images;
    images.sort_by(|a, b| {
        let ts_a = a.creation_date.as_deref().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
        let ts_b = b.creation_date.as_deref().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
        ts_a.partial_cmp(&ts_b).unwrap_or(std::cmp::Ordering::Equal)
    });

    if images.is_empty() {
        return Err(AwsMachineImageError::NotFound);
    }

    let releases: Vec<Release> = images
        .into_iter()
        .filter_map(|img| {
            let image_id = img.image_id?;
            Some(Release {
                version: image_id,
                release_timestamp: img.creation_date,
                is_deprecated: Some(img.deprecation_time.is_some()),
                ..Default::default()
            })
        })
        .collect();

    if releases.is_empty() {
        return Err(AwsMachineImageError::NotFound);
    }

    Ok(ReleaseResult {
        releases,
        ..Default::default()
    })
}

pub fn load_config(serialized: &str) -> Result<AwsMachineImageConfig, serde_json::Error> {
    let parsed: Vec<serde_json::Value> = serde_json::from_str(serialized)?;
    let mut filters = Vec::new();
    let mut region = "us-east-1".to_owned();
    let mut profile = None;

    for elem in parsed {
        if let Some(obj) = elem.as_object() {
            if obj.contains_key("Name") && obj.contains_key("Values") {
                let name = obj.get("Name").and_then(|v| v.as_str()).unwrap_or("").to_owned();
                let values = obj
                    .get("Values")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_owned()))
                            .collect()
                    })
                    .unwrap_or_default();
                filters.push(AmiFilter { name, values });
            } else {
                if let Some(r) = obj.get("region").and_then(|v| v.as_str()) {
                    region = r.to_owned();
                }
                if let Some(p) = obj.get("profile").and_then(|v| v.as_str()) {
                    profile = Some(p.to_owned());
                }
            }
        }
    }

    Ok(AwsMachineImageConfig {
        region,
        profile,
        filters,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn datasource_id_is_correct() {
        assert_eq!(DATASOURCE_ID, "aws-machine-image");
    }

    #[test]
    fn load_config_parses_filters_and_config() {
        let input = r#"[
            {"Name": "name", "Values": ["amzn2-ami-hvm-*"]},
            {"region": "eu-west-1", "profile": "prod"}
        ]"#;
        let config = load_config(input).unwrap();
        assert_eq!(config.region, "eu-west-1");
        assert_eq!(config.profile.as_deref(), Some("prod"));
        assert_eq!(config.filters.len(), 1);
        assert_eq!(config.filters[0].name, "name");
        assert_eq!(config.filters[0].values, vec!["amzn2-ami-hvm-*"]);
    }

    #[test]
    fn load_config_empty_array() {
        let config = load_config("[]").unwrap();
        assert_eq!(config.region, "us-east-1");
        assert!(config.filters.is_empty());
    }

    #[test]
    fn load_config_invalid_json() {
        assert!(load_config("not json").is_err());
    }
}
