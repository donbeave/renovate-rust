//! crates.io sparse registry datasource.
//!
//! Fetches available versions for a crate from the crates.io sparse index
//! (`https://index.crates.io/`).
//!
//! Renovate reference:
//! - `lib/modules/datasource/crate/index.ts` — `CrateDatasource`
//! - `lib/modules/datasource/crate/types.ts` — `CrateRecord`
//!
//! ## Sparse index protocol
//!
//! Each crate's version list lives at a URL derived from the crate name:
//!
//! | Name length | URL path pattern |
//! |---|---|
//! | 1 | `1/{name}` |
//! | 2 | `2/{name}` |
//! | 3 | `3/{name[0]}/{name}` |
//! | ≥4 | `{name[0..2]}/{name[2..4]}/{name}` |
//!
//! The response body is newline-delimited JSON: one `CrateRecord` per line.

use serde::Deserialize;
use thiserror::Error;

use crate::http::{HttpClient, HttpError};

/// Default crates.io sparse index base URL.
pub const CRATES_IO_SPARSE_INDEX: &str = "https://index.crates.io";

/// Errors from crates.io lookups.
#[derive(Debug, Error)]
pub enum CratesIoError {
    #[error("HTTP error: {0}")]
    Http(#[from] HttpError),

    #[error("Failed to parse crate index record: {0}")]
    Parse(String),
}

/// A single version record from the sparse index.
///
/// Source: `lib/modules/datasource/crate/types.ts` `CrateRecord`.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct CrateRecord {
    /// The version string (e.g. `"1.52.0"`).
    pub vers: String,
    /// Whether this version has been yanked from the registry.
    pub yanked: bool,
}

/// Compute the sparse-index URL path for a crate name.
///
/// Ports `CrateDatasource.getIndexSuffix` from
/// `lib/modules/datasource/crate/index.ts`.
pub fn index_path(name: &str) -> String {
    let lower = name.to_lowercase();
    let len = lower.len();
    match len {
        0 => lower,
        1 => format!("1/{lower}"),
        2 => format!("2/{lower}"),
        3 => {
            let first = &lower[..1];
            format!("3/{first}/{lower}")
        }
        _ => {
            let a = &lower[..2];
            let b = &lower[2..4];
            format!("{a}/{b}/{lower}")
        }
    }
}

/// Fetch all version records for a crate from the crates.io sparse index.
///
/// Returns records in the order they appear in the index (oldest first).
/// Callers should filter `yanked == true` entries before presenting versions
/// to users.
pub async fn fetch_versions(
    http: &HttpClient,
    crate_name: &str,
    index_base: &str,
) -> Result<Vec<CrateRecord>, CratesIoError> {
    let path = index_path(crate_name);
    let url = format!("{}/{}", index_base.trim_end_matches('/'), path);

    let resp = http.get(&url).send().await.map_err(HttpError::Request)?;

    if !resp.status().is_success() {
        return Err(CratesIoError::Http(HttpError::Status {
            status: resp.status(),
            url,
        }));
    }

    let body = resp.text().await.map_err(HttpError::Request)?;
    parse_index_body(&body)
}

fn parse_index_body(body: &str) -> Result<Vec<CrateRecord>, CratesIoError> {
    body.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            serde_json::from_str::<CrateRecord>(line)
                .map_err(|e| CratesIoError::Parse(e.to_string()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;
    use crate::http::HttpClient;

    // ── index_path ────────────────────────────────────────────────────────────

    #[test]
    fn index_path_len_1() {
        assert_eq!(index_path("a"), "1/a");
    }

    #[test]
    fn index_path_len_2() {
        assert_eq!(index_path("ab"), "2/ab");
    }

    #[test]
    fn index_path_len_3() {
        assert_eq!(index_path("foo"), "3/f/foo");
    }

    #[test]
    fn index_path_len_4() {
        assert_eq!(index_path("serde"), "se/rd/serde");
    }

    #[test]
    fn index_path_long_name() {
        assert_eq!(index_path("tokio"), "to/ki/tokio");
    }

    #[test]
    fn index_path_is_lowercase() {
        assert_eq!(index_path("Serde"), "se/rd/serde");
    }

    // ── parse_index_body ─────────────────────────────────────────────────────

    #[test]
    fn parses_newline_delimited_records() {
        let body = r#"{"name":"serde","vers":"1.0.0","deps":[],"cksum":"abc","features":{},"yanked":false}
{"name":"serde","vers":"1.0.1","deps":[],"cksum":"def","features":{},"yanked":false}
{"name":"serde","vers":"1.0.2","deps":[],"cksum":"ghi","features":{},"yanked":true}
"#;
        let records = parse_index_body(body).unwrap();
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].vers, "1.0.0");
        assert!(!records[0].yanked);
        assert!(records[2].yanked);
    }

    #[test]
    fn ignores_blank_lines() {
        let body = "\n{\"name\":\"x\",\"vers\":\"0.1.0\",\"deps\":[],\"cksum\":\"\",\"features\":{},\"yanked\":false}\n\n";
        let records = parse_index_body(body).unwrap();
        assert_eq!(records.len(), 1);
    }

    // ── fetch_versions (wiremock) ─────────────────────────────────────────────

    #[tokio::test]
    async fn fetch_versions_returns_records() {
        let server = MockServer::start().await;
        let body = r#"{"name":"serde","vers":"1.0.195","deps":[],"cksum":"","features":{},"yanked":false}
{"name":"serde","vers":"1.0.196","deps":[],"cksum":"","features":{},"yanked":false}
"#;
        Mock::given(method("GET"))
            .and(path("/se/rd/serde"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let records = fetch_versions(&http, "serde", &server.uri()).await.unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!(records[1].vers, "1.0.196");
    }

    #[tokio::test]
    async fn fetch_versions_404_returns_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/no/nc/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_versions(&http, "nonexistent", &server.uri()).await;
        assert!(result.is_err());
    }
}
