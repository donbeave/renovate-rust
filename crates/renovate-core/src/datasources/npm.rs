//! npm registry datasource.
//!
//! Fetches available versions for an npm package from the npm registry
//! (`https://registry.npmjs.org/`).
//!
//! Renovate reference:
//! - `lib/modules/datasource/npm/index.ts`  — `NpmDatasource`
//! - `lib/modules/datasource/npm/get.ts`    — `getDependency`
//! - `lib/modules/datasource/npm/types.ts`  — `NpmResponse`
//!
//! ## Protocol
//!
//! Each package's full metadata (a "packument") lives at
//! `{registry}/{encoded_name}`.  Scoped packages require the `/` to be
//! percent-encoded: `@scope/pkg` → `@scope%2Fpkg`.
//!
//! The response is JSON with `versions` (map of version → metadata) and
//! `dist-tags` (`"latest"` being the stable recommendation).
//!
//! Deprecated versions (non-empty `deprecated` field) are excluded from the
//! version list before the update decision is made.

use std::collections::HashMap;
use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::{HttpClient, HttpError};
use crate::versioning::npm::{NpmUpdateSummary, npm_update_summary};

/// Default npm registry base URL.
pub const NPM_REGISTRY: &str = "https://registry.npmjs.org";

/// Errors from npm registry lookups.
#[derive(Debug, Error)]
pub enum NpmError {
    #[error("HTTP error: {0}")]
    Http(#[from] HttpError),
    #[error("failed to parse npm registry response: {0}")]
    Parse(String),
    #[error("package '{0}' not found in versions cache")]
    NotFound(String),
}

/// Per-version metadata from the registry (only fields we inspect).
#[derive(Debug, Deserialize)]
struct NpmVersionEntry {
    /// Non-empty string or `true` → deprecated; absent or empty → active.
    #[serde(default)]
    deprecated: Option<serde_json::Value>,
}

/// Top-level npm packument (registry response for a package).
#[derive(Debug, Deserialize)]
struct NpmPackument {
    #[serde(default)]
    versions: HashMap<String, NpmVersionEntry>,
    #[serde(rename = "dist-tags", default)]
    dist_tags: HashMap<String, String>,
    /// Publish timestamps keyed by version string (ISO 8601).
    /// Also contains `"created"` and `"modified"` meta-keys (ignored).
    #[serde(default)]
    time: HashMap<String, String>,
}

/// Encode a package name for use in the registry URL path.
///
/// Scoped packages (`@scope/name`) must use `@scope%2Fname` per the npm
/// registry protocol; plain names are returned as-is.
fn encode_package_name(name: &str) -> String {
    if let Some(rest) = name.strip_prefix('@')
        && let Some(slash) = rest.find('/')
    {
        let (scope, pkg) = rest.split_at(slash);
        let pkg = &pkg[1..]; // skip the '/'
        return format!("@{scope}%2F{pkg}");
    }
    name.to_owned()
}

/// Return `true` when a version entry's `deprecated` field represents a
/// non-empty deprecation message (any truthy non-empty value).
fn is_deprecated(entry: &NpmVersionEntry) -> bool {
    match &entry.deprecated {
        None => false,
        Some(serde_json::Value::Null | serde_json::Value::Bool(false)) => false,
        Some(serde_json::Value::String(s)) => !s.is_empty(),
        Some(_) => true,
    }
}

/// Fetch all active (non-deprecated) versions for an npm package, sorted
/// oldest-first by semver.
///
/// Also returns the `latest` dist-tag value, if present.
pub async fn fetch_versions(
    http: &HttpClient,
    package_name: &str,
    registry: &str,
) -> Result<NpmVersionsEntry, NpmError> {
    let encoded = encode_package_name(package_name);
    let url = format!("{}/{}", registry.trim_end_matches('/'), encoded);

    let resp = http.get_retrying(&url).await?;
    if !resp.status().is_success() {
        return Err(NpmError::Http(HttpError::Status {
            status: resp.status(),
            url,
        }));
    }

    let body = resp.text().await.map_err(HttpError::Request)?;
    let packument: NpmPackument =
        serde_json::from_str(&body).map_err(|e| NpmError::Parse(e.to_string()))?;

    let latest_tag = packument.dist_tags.get("latest").cloned();
    let latest_timestamp = latest_tag
        .as_deref()
        .and_then(|tag| packument.time.get(tag))
        .cloned();

    let mut versions: Vec<String> = packument
        .versions
        .into_iter()
        .filter(|(_, entry)| !is_deprecated(entry))
        .map(|(v, _)| v)
        .collect();

    // Sort by semver; non-semver strings fall back to lexicographic order.
    versions.sort_by(|a, b| {
        let av = semver::Version::parse(a);
        let bv = semver::Version::parse(b);
        match (av, bv) {
            (Ok(a), Ok(b)) => a.cmp(&b),
            (Ok(_), Err(_)) => std::cmp::Ordering::Less,
            (Err(_), Ok(_)) => std::cmp::Ordering::Greater,
            (Err(_), Err(_)) => a.cmp(b),
        }
    });

    Ok(NpmVersionsEntry {
        versions,
        latest_tag,
        latest_timestamp,
    })
}

/// Input descriptor for a single npm dependency in a batch fetch.
#[derive(Debug, Clone)]
pub struct NpmDepInput {
    /// The name used in package.json (also used for the registry lookup).
    pub dep_name: String,
    /// The version constraint string from package.json.
    pub constraint: String,
}

/// Result for a single npm dependency after a batch fetch.
#[derive(Debug)]
pub struct NpmDepUpdate {
    pub dep_name: String,
    pub summary: Result<NpmUpdateSummary, NpmError>,
}

/// Fetch version info for a batch of npm dependencies concurrently.
///
/// `concurrency` caps the number of simultaneous HTTP requests in flight.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[NpmDepInput],
    registry: &str,
    concurrency: usize,
) -> Vec<NpmDepUpdate> {
    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<NpmDepUpdate> = JoinSet::new();

    for dep in deps {
        let http = http.clone();
        let sem = Arc::clone(&sem);
        let dep_name = dep.dep_name.clone();
        let constraint = dep.constraint.clone();
        let registry = registry.to_owned();

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_versions(&http, &dep_name, &registry).await;
            let summary = result.map(|entry| {
                let mut s =
                    npm_update_summary(&constraint, &entry.versions, entry.latest_tag.as_deref());
                s.latest_timestamp = entry.latest_timestamp;
                s
            });
            NpmDepUpdate { dep_name, summary }
        });
    }

    let mut results = Vec::with_capacity(deps.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok(update) => results.push(update),
            Err(join_err) => tracing::error!(%join_err, "npm datasource task panicked"),
        }
    }
    results
}

/// Cached versions entry for a single npm package.
#[derive(Debug, Clone)]
pub struct NpmVersionsEntry {
    /// Versions sorted oldest-first by semver.
    pub versions: Vec<String>,
    /// The `latest` dist-tag value, if present.
    pub latest_tag: Option<String>,
    /// ISO 8601 publish timestamp for the `latest` dist-tag version, if known.
    /// Used for `minimumReleaseAge` checking.
    pub latest_timestamp: Option<String>,
}

/// Fetch versions for a batch of unique package names concurrently.
///
/// Returns a `HashMap` from package name to `(versions, latest_tag)`.
/// Packages that fail to fetch are omitted from the result.
/// Use this for cross-file deduplication when the same package may appear
/// in multiple manifests.
pub async fn fetch_versions_batch(
    http: &HttpClient,
    package_names: &[String],
    registry: &str,
    concurrency: usize,
) -> HashMap<String, NpmVersionsEntry> {
    if package_names.is_empty() {
        return HashMap::new();
    }

    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<(String, Option<NpmVersionsEntry>)> = JoinSet::new();

    for name in package_names {
        let http = http.clone();
        let sem = Arc::clone(&sem);
        let name = name.clone();
        let registry = registry.to_owned();

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_versions(&http, &name, &registry).await;
            (name, result.ok())
        });
    }

    let mut cache = HashMap::with_capacity(package_names.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok((name, Some(entry))) => {
                cache.insert(name, entry);
            }
            Ok((name, None)) => {
                tracing::debug!(package = %name, "npm versions fetch failed (package skipped)")
            }
            Err(join_err) => tracing::error!(%join_err, "npm versions fetch task panicked"),
        }
    }
    cache
}

/// Compute an `NpmUpdateSummary` from a pre-fetched versions cache entry.
pub fn summary_from_cache(constraint: &str, entry: &NpmVersionsEntry) -> NpmUpdateSummary {
    let mut s = npm_update_summary(constraint, &entry.versions, entry.latest_tag.as_deref());
    s.latest_timestamp = entry.latest_timestamp.clone();
    s
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;
    use crate::http::HttpClient;

    // ── encode_package_name ───────────────────────────────────────────────────

    #[test]
    fn encodes_scoped_package() {
        assert_eq!(encode_package_name("@types/node"), "@types%2Fnode");
        assert_eq!(encode_package_name("@babel/core"), "@babel%2Fcore");
    }

    #[test]
    fn plain_package_name_unchanged() {
        assert_eq!(encode_package_name("express"), "express");
        assert_eq!(encode_package_name("lodash"), "lodash");
    }

    // ── is_deprecated helper ──────────────────────────────────────────────────

    #[test]
    fn deprecated_string_is_deprecated() {
        assert!(is_deprecated(&NpmVersionEntry {
            deprecated: Some(serde_json::Value::String("use newpkg instead".into())),
        }));
    }

    #[test]
    fn empty_string_not_deprecated() {
        assert!(!is_deprecated(&NpmVersionEntry {
            deprecated: Some(serde_json::Value::String(String::new())),
        }));
    }

    #[test]
    fn null_not_deprecated() {
        assert!(!is_deprecated(&NpmVersionEntry {
            deprecated: Some(serde_json::Value::Null),
        }));
    }

    #[test]
    fn none_not_deprecated() {
        assert!(!is_deprecated(&NpmVersionEntry { deprecated: None }));
    }

    // ── fetch_versions (wiremock) ─────────────────────────────────────────────

    fn packument_json(versions: &[(&str, bool)], latest: &str) -> String {
        let versions_obj: String = versions
            .iter()
            .map(|(v, dep)| {
                if *dep {
                    format!(r#""{v}": {{"deprecated": "old version"}}"#)
                } else {
                    format!(r#""{v}": {{}}"#)
                }
            })
            .collect::<Vec<_>>()
            .join(",");
        format!(
            r#"{{"name":"test","versions":{{{versions_obj}}},"dist-tags":{{"latest":"{latest}"}}}}"#
        )
    }

    #[tokio::test]
    async fn fetch_versions_returns_non_deprecated_sorted() {
        let server = MockServer::start().await;
        let body = packument_json(
            &[("1.0.0", false), ("1.1.0", true), ("1.2.0", false)],
            "1.2.0",
        );
        Mock::given(method("GET"))
            .and(path("/express"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let entry = fetch_versions(&http, "express", &server.uri())
            .await
            .unwrap();

        // 1.1.0 is deprecated — should be excluded
        assert_eq!(entry.versions, vec!["1.0.0", "1.2.0"]);
        assert_eq!(entry.latest_tag.as_deref(), Some("1.2.0"));
    }

    #[tokio::test]
    async fn fetch_versions_scoped_package_encodes_slash() {
        let server = MockServer::start().await;
        let body = packument_json(&[("20.0.0", false)], "20.0.0");
        Mock::given(method("GET"))
            .and(path("/@types%2Fnode"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let entry = fetch_versions(&http, "@types/node", &server.uri())
            .await
            .unwrap();
        assert_eq!(entry.versions, vec!["20.0.0"]);
    }

    #[tokio::test]
    async fn fetch_versions_404_returns_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_versions(&http, "nonexistent", &server.uri()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn fetch_updates_concurrent_fetches_all() {
        let server = MockServer::start().await;

        let express_body = packument_json(
            &[("4.17.21", false), ("4.18.0", false), ("4.18.2", false)],
            "4.18.2",
        );
        let react_body = packument_json(
            &[("17.0.2", false), ("18.0.0", false), ("18.2.0", false)],
            "18.2.0",
        );
        Mock::given(method("GET"))
            .and(path("/express"))
            .respond_with(ResponseTemplate::new(200).set_body_string(express_body))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/react"))
            .respond_with(ResponseTemplate::new(200).set_body_string(react_body))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let deps = vec![
            NpmDepInput {
                dep_name: "express".into(),
                constraint: "4.17.21".into(),
            },
            NpmDepInput {
                dep_name: "react".into(),
                constraint: "^18.0.0".into(),
            },
        ];
        let results = fetch_updates_concurrent(&http, &deps, &server.uri(), 10).await;
        assert_eq!(results.len(), 2);

        let express_r = results.iter().find(|r| r.dep_name == "express").unwrap();
        let express_s = express_r.summary.as_ref().unwrap();
        // "4.17.21" is exact pin; latest_compatible is also 4.17.21 → no update
        assert_eq!(express_s.latest.as_deref(), Some("4.18.2"));

        let react_r = results.iter().find(|r| r.dep_name == "react").unwrap();
        let react_s = react_r.summary.as_ref().unwrap();
        // "^18.0.0" is a range → no update flagged
        assert!(!react_s.update_available);
        assert_eq!(react_s.latest_compatible.as_deref(), Some("18.2.0"));
    }
}
