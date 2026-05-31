//! AWS RDS datasource.
//!
//! Fetches RDS engine versions from the AWS RDS DescribeDBEngineVersions API.
//!
//! Renovate reference: `lib/modules/datasource/aws-rds/index.ts`
//! API: `GET https://rds.{region}.amazonaws.com/` (DescribeDBEngineVersions action)

use serde::Deserialize;
use thiserror::Error;

use crate::datasources::{Release, ReleaseResult};
use crate::http::HttpClient;

pub const DATASOURCE_ID: &str = "aws-rds";

#[derive(Debug, Error)]
pub enum AwsRdsError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
    #[error("no engine versions found")]
    NotFound,
}

#[derive(Debug, Clone)]
pub struct AwsRdsConfig {
    pub region: String,
    pub filters: Vec<RdsFilter>,
}

#[derive(Debug, Clone)]
pub struct RdsFilter {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct DescribeDbEngineVersionsResponse {
    #[serde(default)]
    db_engine_versions: Vec<DbEngineVersion>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DbEngineVersion {
    engine_version: Option<String>,
    status: Option<String>,
}

pub async fn fetch_versions(
    http: &HttpClient,
    serialized_filter: &str,
    region: &str,
    api_base: Option<&str>,
) -> Result<ReleaseResult, AwsRdsError> {
    let base = api_base
        .map(|s| s.to_owned())
        .unwrap_or_else(|| format!("https://rds.{}.amazonaws.com", region));

    let filters: Vec<serde_json::Value> = serde_json::from_str(serialized_filter)
        .unwrap_or_default();

    let filter_params: String = filters
        .iter()
        .enumerate()
        .flat_map(|(i, f)| {
            let name = f.get("Name").and_then(|v| v.as_str()).unwrap_or("");
            let values = f
                .get("Values")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_owned()))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let mut parts = Vec::new();
            for (j, val) in values.iter().enumerate() {
                parts.push(format!(
                    "Filters.{}.Name={}&Filters.{}.Values.{}={}",
                    i, name, i, j, val
                ));
            }
            parts
        })
        .collect::<Vec<_>>()
        .join("&");

    let url = if filter_params.is_empty() {
        format!("{}/?Action=DescribeDBEngineVersions", base)
    } else {
        format!(
            "{}/?Action=DescribeDBEngineVersions&{}",
            base, filter_params
        )
    };

    let resp = http.get_retrying(&url).await?;

    if !resp.status().is_success() {
        return Err(AwsRdsError::Http(crate::http::HttpError::Status {
            status: resp.status(),
            url,
        }));
    }

    let result: DescribeDbEngineVersionsResponse = resp.json().await?;

    let releases: Vec<Release> = result
        .db_engine_versions
        .into_iter()
        .filter_map(|v| {
            let version = v.engine_version?;
            Some(Release {
                version,
                is_deprecated: Some(v.status.as_deref() == Some("deprecated")),
                ..Default::default()
            })
        })
        .collect();

    if releases.is_empty() {
        return Err(AwsRdsError::NotFound);
    }

    Ok(ReleaseResult {
        releases,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn datasource_id_is_correct() {
        assert_eq!(DATASOURCE_ID, "aws-rds");
    }

    #[test]
    fn rds_filter_default_region() {
        let config = AwsRdsConfig {
            region: "us-east-1".to_owned(),
            filters: vec![],
        };
        assert_eq!(config.region, "us-east-1");
    }
}
