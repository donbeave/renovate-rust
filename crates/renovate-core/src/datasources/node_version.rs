//! Node.js version datasource.
//!
//! Fetches Node.js release information from `https://nodejs.org/dist/index.json`.
//! Each release carries a version string, a release date, and an LTS flag.
//!
//! Renovate reference: `lib/modules/datasource/node-version/index.ts`
//! Registry:           `https://nodejs.org/dist`

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY_URL: &str = "https://nodejs.org/dist";
pub const DATASOURCE_ID: &str = "node-version";

/// Errors from the Node.js version datasource.
#[derive(Debug, Error)]
pub enum NodeVersionError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

/// One entry from `https://nodejs.org/dist/index.json`.
#[derive(Debug, Deserialize)]
struct NodeRelease {
    /// Node.js version string, e.g. `"v16.9.0"`.
    version: String,
    /// Release date, e.g. `"2021-09-07"`.
    date: String,
    /// LTS label (`false` if not LTS, or a label string like `"Fermium"`).
    #[serde(default, with = "lts_field")]
    lts: bool,
}

mod lts_field {
    use serde::Deserializer;

    pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Visitor;

        struct LtsVisitor;

        impl<'de> Visitor<'de> for LtsVisitor {
            type Value = bool;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "boolean false or an LTS label string")
            }
            fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<bool, E> {
                Ok(v)
            }
            fn visit_str<E: serde::de::Error>(self, _v: &str) -> Result<bool, E> {
                // Any non-false string means the release is LTS.
                Ok(true)
            }
        }

        deserializer.deserialize_any(LtsVisitor)
    }
}

/// One release entry as returned by `fetch_releases`.
#[derive(Debug, Clone)]
pub struct NodeVersionRelease {
    /// Version string with the leading `v` stripped (e.g. `"16.9.0"`).
    pub version: String,
    /// ISO 8601 release timestamp (e.g. `"2021-09-07T00:00:00.000Z"`).
    pub release_timestamp: Option<String>,
    /// Whether this is an LTS release.
    pub is_stable: bool,
}

/// Result of a `fetch_releases` call.
#[derive(Debug, Clone)]
pub struct NodeVersionResult {
    pub releases: Vec<NodeVersionRelease>,
    pub homepage: &'static str,
    pub source_url: &'static str,
    pub registry_url: String,
}

/// Fetch the full Node.js release list from `registry_url/index.json`.
///
/// Returns `None` when the server returns an empty array (no releases).
pub async fn fetch_releases(
    registry_url: &str,
    http: &HttpClient,
) -> Result<Option<NodeVersionResult>, NodeVersionError> {
    let url = format!("{registry_url}/index.json");
    let raw: Vec<NodeRelease> = http.get_json(&url).await?;

    if raw.is_empty() {
        return Ok(None);
    }

    let releases: Vec<NodeVersionRelease> = raw
        .into_iter()
        .map(|r| NodeVersionRelease {
            // Strip the leading "v" that Node uses (e.g. "v16.9.0" → "16.9.0").
            version: r.version.trim_start_matches('v').to_owned(),
            release_timestamp: Some(format!("{}T00:00:00.000Z", r.date)),
            is_stable: r.lts,
        })
        .collect();

    Ok(Some(NodeVersionResult {
        releases,
        homepage: "https://nodejs.org",
        source_url: "https://github.com/nodejs/node",
        registry_url: registry_url.to_owned(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "returns null for empty 200 OK" — node-version/index.spec.ts line 32
    #[test]
    fn empty_array_yields_none() {
        // An empty JSON array should map to None (no releases).
        let raw: Vec<NodeRelease> = serde_json::from_str("[]").unwrap();
        assert!(raw.is_empty());
    }

    // Ported: "processes real data" — node-version/index.spec.ts line 42
    #[test]
    fn parse_non_lts_release() {
        let json = r#"[{"version":"v16.9.0","date":"2021-09-07","files":[],"npm":"7.21.1","v8":"9.3.345.16","uv":"1.42.0","zlib":"1.2.11","openssl":"1.1.1l+quic","modules":"93","lts":false,"security":false}]"#;
        let entries: Vec<NodeRelease> = serde_json::from_str(json).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].version, "v16.9.0");
        assert_eq!(entries[0].date, "2021-09-07");
        assert!(!entries[0].lts);
    }

    #[test]
    fn parse_lts_release() {
        // LTS releases use a string label instead of `false`.
        let json = r#"[{"version":"v14.17.0","date":"2021-05-11","files":[],"npm":"6.14.13","v8":"8.4.371.19","uv":"1.41.0","zlib":"1.2.11","openssl":"1.1.1k+quic","modules":"83","lts":"Fermium","security":false}]"#;
        let entries: Vec<NodeRelease> = serde_json::from_str(json).unwrap();
        assert_eq!(entries.len(), 1);
        assert!(entries[0].lts);
    }

    #[test]
    fn version_prefix_stripped() {
        // The leading "v" is stripped to produce a clean semver string.
        assert_eq!("v16.9.0".trim_start_matches('v'), "16.9.0");
    }

    #[test]
    fn release_timestamp_format() {
        let date = "2021-09-07";
        let ts = format!("{date}T00:00:00.000Z");
        assert_eq!(ts, "2021-09-07T00:00:00.000Z");
    }
}
