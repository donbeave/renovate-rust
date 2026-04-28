//! CircleCI Orb datasource.
//!
//! Fetches orb versions from the CircleCI GraphQL API.
//!
//! Renovate reference:
//! - `lib/modules/datasource/orb/index.ts` — `OrbDatasource`
//! - API: `POST https://circleci.com/graphql-unstable`
//!
//! The query returns `versions[].version` strings (newest first).

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::HttpClient;

pub const CIRCLECI_GRAPHQL_URL: &str = "https://circleci.com/graphql-unstable";

const ORB_QUERY: &str = "query($packageName: String!, $maxVersions: Int!) { orb(name: $packageName) { versions(count: $maxVersions) { version } } }";
const MAX_VERSIONS: u32 = 100;

/// Errors from fetching Orb metadata.
#[derive(Debug, Error)]
pub enum OrbError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
    #[error("orb not found: {0}")]
    NotFound(String),
}

/// Input for a single orb lookup.
#[derive(Debug, Clone)]
pub struct OrbDepInput {
    /// Orb package name, e.g. `circleci/node`.
    pub package_name: String,
    pub current_value: String,
}

/// Update summary for an orb dependency.
#[derive(Debug, Clone)]
pub struct OrbUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Per-dependency result from `fetch_updates_concurrent`.
#[derive(Debug)]
pub struct OrbUpdateResult {
    pub package_name: String,
    pub summary: Result<OrbUpdateSummary, OrbError>,
}

#[derive(Debug, Serialize)]
struct GraphqlRequest<'a> {
    query: &'a str,
    variables: GraphqlVariables<'a>,
}

#[derive(Debug, Serialize)]
struct GraphqlVariables<'a> {
    #[serde(rename = "packageName")]
    package_name: &'a str,
    #[serde(rename = "maxVersions")]
    max_versions: u32,
}

#[derive(Debug, Deserialize)]
struct GraphqlResponse {
    data: Option<GraphqlData>,
}

#[derive(Debug, Deserialize)]
struct GraphqlData {
    orb: Option<OrbData>,
}

#[derive(Debug, Deserialize)]
struct OrbData {
    versions: Vec<OrbVersion>,
}

#[derive(Debug, Deserialize)]
struct OrbVersion {
    version: String,
}

async fn fetch_latest(
    http: &HttpClient,
    package_name: &str,
    current_value: &str,
) -> Result<OrbUpdateSummary, OrbError> {
    let body = GraphqlRequest {
        query: ORB_QUERY,
        variables: GraphqlVariables {
            package_name,
            max_versions: MAX_VERSIONS,
        },
    };
    let json_body = serde_json::to_string(&body).unwrap();
    let resp = http
        .post_json::<GraphqlResponse>(CIRCLECI_GRAPHQL_URL, &json_body)
        .await?;

    let versions = resp
        .data
        .and_then(|d| d.orb)
        .map(|o| o.versions)
        .unwrap_or_default();

    let latest = versions.into_iter().next().map(|v| v.version);

    let update_available = match &latest {
        Some(l) => l != current_value,
        None => false,
    };

    Ok(OrbUpdateSummary {
        current_value: current_value.to_owned(),
        latest,
        update_available,
    })
}

/// Fetch update summaries for multiple orb deps concurrently.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[OrbDepInput],
    concurrency: usize,
) -> Vec<OrbUpdateResult> {
    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set = JoinSet::new();

    for dep in deps {
        let sem = Arc::clone(&sem);
        let http = http.clone();
        let package_name = dep.package_name.clone();
        let current_value = dep.current_value.clone();

        set.spawn(async move {
            let _permit = sem.acquire().await.unwrap();
            let summary = fetch_latest(&http, &package_name, &current_value).await;
            OrbUpdateResult {
                package_name,
                summary,
            }
        });
    }

    let mut results = Vec::new();
    while let Some(res) = set.join_next().await {
        if let Ok(r) = res {
            results.push(r);
        }
    }
    results
}
