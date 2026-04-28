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

use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::{HttpClient, HttpError};
use crate::versioning::cargo::{UpdateSummary, update_summary};

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

/// Input descriptor for a single dependency in a batch fetch.
#[derive(Debug, Clone)]
pub struct DepInput {
    /// The name used in Cargo.toml (key in `[dependencies]`).
    pub dep_name: String,
    /// The actual crate name to look up (may differ when `package =` is set).
    pub package_name: String,
    /// The version constraint string from Cargo.toml.
    pub constraint: String,
}

/// Result for a single dependency after a batch fetch.
#[derive(Debug)]
pub struct DepUpdate {
    pub dep_name: String,
    pub summary: Result<UpdateSummary, CratesIoError>,
}

/// Fetch version info for a batch of crate dependencies concurrently.
///
/// `concurrency` caps how many simultaneous HTTP requests are in flight.
/// Renovate's default HTTP queue depth is 10; that is a reasonable default.
///
/// Tasks are spawned on the current `tokio` runtime. The `HttpClient` is
/// cloned per task — that is cheap because `reqwest::Client` is an `Arc`
/// internally and shares the underlying connection pool.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[DepInput],
    index_base: &str,
    concurrency: usize,
) -> Vec<DepUpdate> {
    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<DepUpdate> = JoinSet::new();

    for dep in deps {
        let http = http.clone();
        let sem = Arc::clone(&sem);
        let dep_name = dep.dep_name.clone();
        let package_name = dep.package_name.clone();
        let constraint = dep.constraint.clone();
        let index_base = index_base.to_owned();

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_versions(&http, &package_name, &index_base).await;
            let summary = result.map(|records| {
                let non_yanked: Vec<String> = records
                    .into_iter()
                    .filter(|r| !r.yanked)
                    .map(|r| r.vers)
                    .collect();
                update_summary(&constraint, &non_yanked)
            });
            DepUpdate { dep_name, summary }
        });
    }

    let mut results = Vec::with_capacity(deps.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok(update) => results.push(update),
            Err(join_err) => {
                // A task panicked — this is a programmer error, not a user error.
                tracing::error!(%join_err, "datasource task panicked");
            }
        }
    }
    results
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
    async fn fetch_updates_concurrent_fetches_all_deps() {
        let server = MockServer::start().await;

        let body_serde = r#"{"name":"serde","vers":"1.0.195","deps":[],"cksum":"","features":{},"yanked":false}
{"name":"serde","vers":"1.0.228","deps":[],"cksum":"","features":{},"yanked":false}
"#;
        let body_tokio = r#"{"name":"tokio","vers":"1.52.0","deps":[],"cksum":"","features":{},"yanked":false}
{"name":"tokio","vers":"1.52.1","deps":[],"cksum":"","features":{},"yanked":false}
"#;
        Mock::given(method("GET"))
            .and(path("/se/rd/serde"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body_serde))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/to/ki/tokio"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body_tokio))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let deps = vec![
            DepInput {
                dep_name: "serde".into(),
                package_name: "serde".into(),
                constraint: "1.0.195".into(),
            },
            DepInput {
                dep_name: "tokio".into(),
                package_name: "tokio".into(),
                constraint: "1.52.0".into(),
            },
        ];

        let results = fetch_updates_concurrent(&http, &deps, &server.uri(), 10).await;
        assert_eq!(results.len(), 2);

        let serde_r = results.iter().find(|r| r.dep_name == "serde").unwrap();
        let serde_s = serde_r.summary.as_ref().unwrap();
        assert_eq!(serde_s.latest_compatible.as_deref(), Some("1.0.228"));
        assert!(serde_s.update_available); // 1.0.195 → 1.0.228

        let tokio_r = results.iter().find(|r| r.dep_name == "tokio").unwrap();
        let tokio_s = tokio_r.summary.as_ref().unwrap();
        assert!(tokio_s.update_available); // 1.52.0 → 1.52.1
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
