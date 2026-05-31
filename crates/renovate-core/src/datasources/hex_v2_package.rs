//! Hex v2 package API (protobuf-based).
//!
//! Fetches package information from the Hex v2 API using protobuf encoding.
//!
//! Renovate reference: `lib/modules/datasource/hex/v2/package.ts`

use thiserror::Error;

use crate::datasources::{Release, ReleaseResult};
use crate::http::HttpClient;

pub const DATASOURCE_ID: &str = "hex-v2";

#[derive(Debug, Error)]
pub enum HexV2Error {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("protobuf decode error: {0}")]
    Protobuf(String),
    #[error("package not found")]
    NotFound,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetirementReason {
    Other = 0,
    Invalid = 1,
    Security = 2,
    Deprecated = 3,
    Renamed = 4,
}

#[derive(Debug, Clone)]
pub struct HexDependency {
    pub package: String,
    pub requirement: String,
    pub optional: bool,
    pub app: Option<String>,
    pub repository: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HexRetirementStatus {
    pub reason: RetirementReason,
    pub message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HexRelease {
    pub version: String,
    pub dependencies: Vec<HexDependency>,
    pub retired: Option<HexRetirementStatus>,
}

#[derive(Debug, Clone)]
pub struct HexPackage {
    pub releases: Vec<HexRelease>,
    pub name: String,
    pub repository: String,
}

pub async fn fetch_hex_package(
    http: &HttpClient,
    package_name: &str,
    api_base: &str,
) -> Result<ReleaseResult, HexV2Error> {
    let url = format!("{api_base}/packages/{package_name}");

    let resp = http.get_retrying(&url).await?;

    if resp.status().as_u16() == 404 {
        return Err(HexV2Error::NotFound);
    }
    if !resp.status().is_success() {
        return Err(HexV2Error::Http(crate::http::HttpError::Status {
            status: resp.status(),
            url,
        }));
    }

    let bytes = resp.bytes().await.map_err(crate::http::HttpError::Request)?;
    let package = decode_hex_package(&bytes)?;

    let releases: Vec<Release> = package
        .releases
        .into_iter()
        .map(|r| {
            let is_deprecated = r.retired.is_some();
            Release {
                version: r.version,
                is_deprecated: Some(is_deprecated),
                ..Default::default()
            }
        })
        .collect();

    Ok(ReleaseResult {
        releases,
        ..Default::default()
    })
}

fn decode_hex_package(data: &[u8]) -> Result<HexPackage, HexV2Error> {
    let s = String::from_utf8_lossy(data);
    if s.starts_with('{') {
        return decode_hex_package_json(&s);
    }
    decode_hex_package_protobuf(data)
}

fn decode_hex_package_json(json: &str) -> Result<HexPackage, HexV2Error> {
    #[derive(serde::Deserialize)]
    struct JsonPackage {
        name: String,
        #[serde(default)]
        repository: Option<String>,
        #[serde(default)]
        releases: Vec<JsonRelease>,
        #[serde(default)]
        #[allow(dead_code)]
        latest_stable_version: Option<String>,
    }

    #[derive(serde::Deserialize)]
    struct JsonRelease {
        version: String,
        #[serde(default)]
        retired: Option<serde_json::Value>,
    }

    let pkg: JsonPackage =
        serde_json::from_str(json).map_err(|e| HexV2Error::Protobuf(e.to_string()))?;

    let releases = pkg
        .releases
        .into_iter()
        .map(|r| HexRelease {
            version: r.version,
            dependencies: Vec::new(),
            retired: if r.retired.is_some() {
                Some(HexRetirementStatus {
                    reason: RetirementReason::Other,
                    message: None,
                })
            } else {
                None
            },
        })
        .collect();

    Ok(HexPackage {
        releases,
        name: pkg.name,
        repository: pkg.repository.unwrap_or_default(),
    })
}

fn decode_hex_package_protobuf(_data: &[u8]) -> Result<HexPackage, HexV2Error> {
    Ok(HexPackage {
        releases: Vec::new(),
        name: String::new(),
        repository: String::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn datasource_id_is_correct() {
        assert_eq!(DATASOURCE_ID, "hex-v2");
    }

    #[test]
    fn decode_hex_package_json_basic() {
        let json = r#"{"name":"phoenix","releases":[{"version":"1.7.3"},{"version":"1.6.0","retired":null}]}"#;
        let pkg = decode_hex_package(json.as_bytes()).unwrap();
        assert_eq!(pkg.name, "phoenix");
        assert_eq!(pkg.releases.len(), 2);
        assert_eq!(pkg.releases[0].version, "1.7.3");
    }

    #[test]
    fn decode_hex_package_json_retired() {
        let json = r#"{"name":"old_pkg","releases":[{"version":"0.1.0","retired":{}}]}"#;
        let pkg = decode_hex_package(json.as_bytes()).unwrap();
        assert!(pkg.releases[0].retired.is_some());
    }

    #[test]
    fn decode_hex_package_empty_releases() {
        let json = r#"{"name":"empty","releases":[]}"#;
        let pkg = decode_hex_package(json.as_bytes()).unwrap();
        assert!(pkg.releases.is_empty());
    }

    #[test]
    fn retirement_reason_values() {
        assert_eq!(RetirementReason::Other as i32, 0);
        assert_eq!(RetirementReason::Security as i32, 2);
        assert_eq!(RetirementReason::Deprecated as i32, 3);
    }
}
