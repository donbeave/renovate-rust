//! RubyGems datasource.
//!
//! Fetches available gem versions from the RubyGems REST API.
//!
//! Renovate reference:
//! - `lib/modules/datasource/rubygems/index.ts` — `RubygemsDatasource`
//! - API: `GET https://rubygems.org/api/v1/versions/{gemname}.json`
//!
//! The `/api/v1/versions/{gem}.json` endpoint returns an array of version
//! objects, newest first, each with a `"number"` field and `"prerelease"` bool.
//! We pick the first entry where `prerelease == false` as the latest stable
//! version.

use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::HttpClient;

pub const RUBYGEMS_API: &str = "https://rubygems.org/api/v1";

/// Errors from fetching RubyGems metadata.
#[derive(Debug, Error)]
pub enum RubyGemsError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
}

/// Input for a single gem lookup.
#[derive(Debug, Clone)]
pub struct GemDepInput {
    pub name: String,
    pub current_value: String,
}

/// Update summary for a gem dependency.
#[derive(Debug, Clone)]
pub struct GemUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
    /// ISO 8601 timestamp when the latest stable version was published.
    /// Used for `minimumReleaseAge` evaluation.
    pub release_timestamp: Option<String>,
}

/// Per-dependency result from `fetch_updates_concurrent`.
#[derive(Debug)]
pub struct GemUpdateResult {
    pub name: String,
    pub summary: Result<GemUpdateSummary, RubyGemsError>,
}

#[derive(Debug, Deserialize)]
struct GemVersion {
    number: String,
    prerelease: bool,
    /// ISO 8601 creation timestamp, e.g. `"2024-01-15T10:30:00.000Z"`.
    /// Available from `/api/v1/versions/{gem}.json`.
    created_at: Option<String>,
}

/// Fetch the latest stable version of a gem and its release timestamp.
pub async fn fetch_latest(
    gem_name: &str,
    http: &HttpClient,
    api_base: &str,
) -> Result<Option<(String, Option<String>)>, RubyGemsError> {
    let url = format!("{api_base}/versions/{gem_name}.json");

    let resp = http.get_retrying(&url).await?;
    if resp.status().as_u16() == 404 {
        return Ok(None);
    }
    if !resp.status().is_success() {
        return Ok(None);
    }

    let versions: Vec<GemVersion> = resp.json().await.map_err(RubyGemsError::Json)?;

    // Versions are newest-first; return first stable entry.
    let latest = versions
        .into_iter()
        .find(|v| !v.prerelease)
        .map(|v| (v.number, v.created_at));

    Ok(latest)
}

/// Fetch update summaries for multiple gems concurrently.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[GemDepInput],
    api_base: &str,
    concurrency: usize,
) -> Vec<GemUpdateResult> {
    if deps.is_empty() {
        return Vec::new();
    }

    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<GemUpdateResult> = JoinSet::new();

    for dep in deps {
        let http = http.clone();
        let dep = dep.clone();
        let sem = Arc::clone(&sem);
        let api_base = api_base.to_owned();

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_update_summary(&dep, &http, &api_base).await;
            GemUpdateResult {
                name: dep.name.clone(),
                summary: result,
            }
        });
    }

    let mut results = Vec::with_capacity(deps.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok(r) => results.push(r),
            Err(join_err) => tracing::error!(%join_err, "rubygems lookup task panicked"),
        }
    }
    results
}

async fn fetch_update_summary(
    dep: &GemDepInput,
    http: &HttpClient,
    api_base: &str,
) -> Result<GemUpdateSummary, RubyGemsError> {
    let result = fetch_latest(&dep.name, http, api_base).await?;
    let (latest_version, release_timestamp) =
        result.map(|(v, ts)| (Some(v), ts)).unwrap_or((None, None));
    let s = crate::versioning::semver_generic::semver_update_summary(
        &dep.current_value,
        latest_version.as_deref(),
    );
    Ok(GemUpdateSummary {
        current_value: s.current_value,
        latest: s.latest,
        update_available: s.update_available,
        release_timestamp,
    })
}

// ── Schema parsers (mirrors lib/modules/datasource/rubygems/schema.ts) ───────

/// Parsed release entry.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct GemRelease {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changelog_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<GemConstraints>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct GemConstraints {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ruby: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rubygems: Option<Vec<String>>,
}

/// Parsed `GemMetadata` fields.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedGemMetadata {
    pub changelog_url: Option<String>,
    pub homepage: Option<String>,
    pub source_url: Option<String>,
}

/// Parse `MarshalledVersionInfo` - array of `{number: string}`.
/// Mirrors `MarshalledVersionInfo` schema from rubygems/schema.ts.
pub fn parse_marshalled_version_info(input: &serde_json::Value) -> Result<Vec<GemRelease>, String> {
    let arr = input.as_array().ok_or("not an array")?;
    if arr.is_empty() {
        return Err("Empty response from `/v1/dependencies` endpoint".to_owned());
    }
    let releases = arr
        .iter()
        .filter_map(|v| {
            v.get("number")?.as_str().map(|n| GemRelease {
                version: n.to_owned(),
                release_timestamp: None,
                changelog_url: None,
                source_url: None,
                constraints: None,
            })
        })
        .collect();
    Ok(releases)
}

/// Parse `GemMetadata` object.
pub fn parse_gem_metadata(input: &serde_json::Value) -> ParsedGemMetadata {
    let get_str = |key: &str| input.get(key).and_then(|v| v.as_str()).map(str::to_owned);
    ParsedGemMetadata {
        changelog_url: get_str("changelog_uri"),
        homepage: get_str("homepage_uri"),
        source_url: get_str("source_code_uri"),
    }
}

/// Parse `GemVersions` - array of version objects from `/api/v1/versions`.
pub fn parse_gem_versions(input: &serde_json::Value) -> Result<Vec<GemRelease>, String> {
    let arr = input.as_array().ok_or("not an array")?;
    let releases: Vec<GemRelease> = arr
        .iter()
        .filter_map(|v| {
            let version = v.get("number")?.as_str()?.to_owned();
            let release_timestamp = v.get("created_at")?.as_str().map(|s| {
                // Normalize to ISO 8601 with milliseconds
                if s.contains('T') {
                    s.to_owned()
                } else {
                    format!("{s}T00:00:00.000Z")
                }
            });
            let platform = v
                .get("platform")
                .and_then(|p| p.as_str())
                .map(str::to_owned);
            let ruby_version = v
                .get("ruby_version")
                .and_then(|p| p.as_str())
                .map(str::to_owned);
            let rubygems_version = v
                .get("rubygems_version")
                .and_then(|p| p.as_str())
                .map(str::to_owned);
            let meta = v.get("metadata").unwrap_or(&serde_json::Value::Null);
            let changelog_url = meta
                .get("changelog_uri")
                .and_then(|v| v.as_str())
                .map(str::to_owned);
            let source_url = meta
                .get("source_code_uri")
                .and_then(|v| v.as_str())
                .map(str::to_owned);

            let constraints =
                if platform.is_some() || ruby_version.is_some() || rubygems_version.is_some() {
                    Some(GemConstraints {
                        platform: platform.map(|p| vec![p]),
                        ruby: ruby_version.map(|r| vec![r]),
                        rubygems: rubygems_version.map(|r| vec![r]),
                    })
                } else {
                    None
                };

            Some(GemRelease {
                version,
                release_timestamp,
                changelog_url,
                source_url,
                constraints,
            })
        })
        .collect();

    if releases.is_empty() {
        return Err("Empty response from `/v1/gems` endpoint".to_owned());
    }
    Ok(releases)
}

/// Parse `GemInfo` - newline-separated `version |checksum:...` format.
pub fn parse_gem_info(input: &str) -> Result<Vec<GemRelease>, String> {
    let releases: Vec<GemRelease> = input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line == "---" {
                return None;
            }
            let space_idx = line.find(' ');
            let version = match space_idx {
                Some(i) if i > 0 => &line[..i],
                None if !line.is_empty() => line,
                _ => return None,
            };
            Some(GemRelease {
                version: version.to_owned(),
                release_timestamp: None,
                changelog_url: None,
                source_url: None,
                constraints: None,
            })
        })
        .collect();
    if releases.is_empty() {
        return Err("Empty response from `/info` endpoint".to_owned());
    }
    Ok(releases)
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    fn versions_json(versions: &[(&str, bool)]) -> String {
        let items: Vec<String> = versions
            .iter()
            .map(|(v, pre)| {
                serde_json::json!({"number": v, "prerelease": pre, "platform": "ruby"}).to_string()
            })
            .collect();
        format!("[{}]", items.join(","))
    }

    #[tokio::test]
    async fn fetch_latest_returns_newest_stable() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/versions/rails.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(versions_json(&[
                ("7.1.0.rc1", true),
                ("7.0.8", false),
                ("7.0.7", false),
            ])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("rails", &http, &server.uri()).await.unwrap();
        assert_eq!(result.map(|(v, _)| v), Some("7.0.8".to_owned()));
    }

    #[tokio::test]
    async fn fetch_latest_skips_prerelease() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/versions/mylib.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string(versions_json(&[
                ("2.0.0.alpha1", true),
                ("2.0.0.beta2", true),
            ])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("mylib", &http, &server.uri()).await.unwrap();
        assert_eq!(result, None); // no stable version available
    }

    // Ported: "returns null for 404" — lib/modules/datasource/pod/index.spec.ts line 60
    #[tokio::test]
    async fn fetch_latest_404_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/versions/nonexistent.json"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_latest("nonexistent", &http, &server.uri())
            .await
            .unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn concurrent_fetch_returns_all() {
        let server = MockServer::start().await;

        for (gem, ver) in [("rails", "7.0.8"), ("devise", "4.9.3")] {
            Mock::given(method("GET"))
                .and(path(format!("/versions/{gem}.json")))
                .respond_with(
                    ResponseTemplate::new(200).set_body_string(versions_json(&[(ver, false)])),
                )
                .mount(&server)
                .await;
        }

        let http = HttpClient::new().unwrap();
        let deps = vec![
            GemDepInput {
                name: "rails".to_owned(),
                current_value: "~> 7.0.4".to_owned(),
            },
            GemDepInput {
                name: "devise".to_owned(),
                current_value: "~> 4.9".to_owned(),
            },
        ];

        let results = fetch_updates_concurrent(&http, &deps, &server.uri(), 4).await;
        assert_eq!(results.len(), 2);

        let rails = results.iter().find(|r| r.name == "rails").unwrap();
        let s = rails.summary.as_ref().unwrap();
        assert_eq!(s.latest.as_deref(), Some("7.0.8"));
        assert!(s.update_available);
    }
}

// Ported: "parses valid input" — modules/datasource/rubygems/schema.spec.ts line 11
#[test]
fn marshalled_version_info_parses_valid() {
    let input = serde_json::json!([
        {"number": "1.0.0"},
        {"number": "2.0.0"},
        {"number": "3.0.0"},
    ]);
    let releases = parse_marshalled_version_info(&input).unwrap();
    assert_eq!(releases.len(), 3);
    assert_eq!(releases[0].version, "1.0.0");
    assert_eq!(releases[1].version, "2.0.0");
    assert_eq!(releases[2].version, "3.0.0");
}

// Ported: "errors on empty input" — modules/datasource/rubygems/schema.spec.ts line 27
#[test]
fn marshalled_version_info_errors_on_empty() {
    let result = parse_marshalled_version_info(&serde_json::json!([]));
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .contains("Empty response from `/v1/dependencies`")
    );
}

// Ported: "parses empty object into undefined fields" — modules/datasource/rubygems/schema.spec.ts line 35
#[test]
fn gem_metadata_parses_empty_object() {
    let result = parse_gem_metadata(&serde_json::json!({}));
    assert_eq!(result.changelog_url, None);
    assert_eq!(result.homepage, None);
    assert_eq!(result.source_url, None);
}

// Ported: "parses valid input" — modules/datasource/rubygems/schema.spec.ts line 43
#[test]
fn gem_metadata_parses_valid_input() {
    let input = serde_json::json!({
        "changelog_uri": "https://example.com",
        "homepage_uri": "https://example.com",
        "source_code_uri": "https://example.com",
    });
    let result = parse_gem_metadata(&input);
    assert_eq!(result.changelog_url.as_deref(), Some("https://example.com"));
    assert_eq!(result.homepage.as_deref(), Some("https://example.com"));
    assert_eq!(result.source_url.as_deref(), Some("https://example.com"));
}

// Ported: "parses valid input" — modules/datasource/rubygems/schema.spec.ts line 59
#[test]
fn gem_versions_parses_valid_input() {
    let input = serde_json::json!([
        {"number": "1.0.0", "created_at": "2021-01-01", "platform": "ruby", "ruby_version": "2.7.0", "rubygems_version": "3.2.0", "metadata": {"changelog_uri": "https://example.com", "source_code_uri": "https://example.com"}},
        {"number": "2.0.0", "created_at": "2022-01-01", "platform": "ruby", "ruby_version": "2.7.0", "rubygems_version": "3.2.0", "metadata": {"changelog_uri": "https://example.com", "source_code_uri": "https://example.com"}},
        {"number": "3.0.0", "created_at": "2023-01-01", "platform": "ruby", "ruby_version": "2.7.0", "rubygems_version": "3.2.0", "metadata": {"changelog_uri": "https://example.com", "source_code_uri": "https://example.com"}},
    ]);
    let releases = parse_gem_versions(&input).unwrap();
    assert_eq!(releases.len(), 3);
    assert_eq!(releases[0].version, "1.0.0");
    assert_eq!(
        releases[0].release_timestamp.as_deref(),
        Some("2021-01-01T00:00:00.000Z")
    );
    assert_eq!(
        releases[0].changelog_url.as_deref(),
        Some("https://example.com")
    );
    assert_eq!(
        releases[0].source_url.as_deref(),
        Some("https://example.com")
    );
    let c = releases[0].constraints.as_ref().unwrap();
    assert_eq!(c.platform.as_deref(), Some(&["ruby".to_owned()][..]));
    assert_eq!(c.ruby.as_deref(), Some(&["2.7.0".to_owned()][..]));
    assert_eq!(c.rubygems.as_deref(), Some(&["3.2.0".to_owned()][..]));
}

// Ported: "parses valid input" — modules/datasource/rubygems/schema.spec.ts line 137
#[test]
fn gem_info_parses_valid_input() {
    // codeBlock strips common indent; input matches:
    // ---\n1.1.1 |checksum:aaa\n2.2.2 |checksum:bbb\n3.3.3 |checksum:ccc
    let input = "---\n1.1.1 |checksum:aaa\n2.2.2 |checksum:bbb\n3.3.3 |checksum:ccc\n";
    let releases = parse_gem_info(input).unwrap();
    assert_eq!(releases.len(), 3);
    assert_eq!(releases[0].version, "1.1.1");
    assert_eq!(releases[1].version, "2.2.2");
    assert_eq!(releases[2].version, "3.3.3");
}

// Ported: "errors on empty input" — modules/datasource/rubygems/schema.spec.ts line 154
#[test]
fn gem_info_errors_on_empty() {
    let result = parse_gem_info("");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Empty response from `/info`"));
}
