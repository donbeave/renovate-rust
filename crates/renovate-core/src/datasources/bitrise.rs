//! Bitrise step library datasource.
//!
//! Fetches available versions for Bitrise CI steps from a GitHub-hosted
//! steplib repository using the GitHub Contents API.
//!
//! Renovate reference: `lib/modules/datasource/bitrise/index.ts`
//!
//! ## API
//!
//! 1. `GET {api_base}/repos/{owner}/{repo}/contents/steps/{step_name}`
//!    → array of `{type, name, path}` directory entries
//! 2. For each semver-valid version directory:
//!    `GET {api_base}/repos/{owner}/{repo}/contents/steps/{step_name}/{version}/step.yml`
//!    → `{type, encoding, content}` base64-encoded YAML file
//!
//! The default steplib is `https://github.com/bitrise-io/bitrise-steplib.git`.

use base64::Engine;

use crate::http::HttpClient;

pub const DEFAULT_STEPLIB_URL: &str = "https://github.com/bitrise-io/bitrise-steplib.git";

/// A single release entry returned by `fetch_releases`.
#[derive(Debug, Clone, PartialEq)]
pub struct BitriseRelease {
    pub version: String,
    pub release_timestamp: Option<String>,
    pub source_url: Option<String>,
}

/// Result from `fetch_releases`.
#[derive(Debug, Clone, PartialEq)]
pub struct BitriseReleasesResult {
    pub releases: Vec<BitriseRelease>,
    pub homepage: String,
    pub registry_url: String,
}


/// Update summary for the legacy `fetch_latest` API used by ci.rs.
#[derive(Debug)]
pub struct BitriseUpdateSummary {
    pub latest: Option<String>,
    pub update_available: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum BitriseError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON error: {0}")]
    Json(reqwest::Error),
}

/// Parse a GitHub steplib URL into (api_base, full_name, registry_url).
///
/// Returns None for non-GitHub platforms (e.g. gitlab.com).
/// Mirrors `detectPlatform(registryUrl) !== 'github'` check in TypeScript.
fn parse_steplib_url(registry_url: &str) -> Option<(String, String, String)> {
    let url = registry_url.trim_end_matches('/');
    let url_no_git = url.strip_suffix(".git").unwrap_or(url);

    // Determine scheme + host + path
    let (scheme, rest) = if let Some(r) = url_no_git.strip_prefix("https://") {
        ("https", r)
    } else if let Some(r) = url_no_git.strip_prefix("http://") {
        ("http", r)
    } else {
        return None;
    };

    let slash = rest.find('/')?;
    let host = &rest[..slash];
    let path = &rest[slash + 1..];

    // Reject non-GitHub platforms (gitlab.com, bitbucket.org, etc.)
    if host.contains("gitlab") || host.contains("bitbucket") {
        return None;
    }

    if path.split('/').filter(|p| !p.is_empty()).count() < 2 {
        return None;
    }

    let full_name = path.to_string();

    // github.com uses the central API; everything else uses /api/v3 (GHE or test servers)
    if host == "github.com" {
        let registry_url = format!("https://github.com/{full_name}.git");
        return Some(("https://api.github.com".to_string(), full_name, registry_url));
    }

    let api_base = format!("{scheme}://{host}/api/v3");
    Some((api_base, full_name, url.to_string()))
}

/// Returns true if `name` looks like a semver version (major.minor.patch pattern).
fn is_semver_version(name: &str) -> bool {
    let parts: Vec<&str> = name.split('.').collect();
    parts.len() >= 2 && parts.iter().all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
}

/// Decode base64-encoded YAML content and extract `published_at` + `source_code_url`.
fn parse_step_yaml(content_b64: &str) -> (Option<String>, Option<String>) {
    let raw = content_b64.replace('\n', "").replace(' ', "");
    let bytes = match base64::engine::general_purpose::STANDARD.decode(&raw) {
        Ok(b) => b,
        Err(_) => return (None, None),
    };
    let yaml = match std::str::from_utf8(&bytes) {
        Ok(s) => s,
        Err(_) => return (None, None),
    };

    let mut published_at: Option<String> = None;
    let mut source_code_url: Option<String> = None;

    for line in yaml.lines() {
        let line = line.trim();
        if let Some(val) = line.strip_prefix("published_at:") {
            published_at = Some(normalize_timestamp(val.trim().trim_matches('"')));
        } else if let Some(val) = line.strip_prefix("source_code_url:") {
            source_code_url = Some(val.trim().to_string());
        }
    }

    (published_at, source_code_url)
}

/// Normalize a timestamp to millisecond precision UTC ISO 8601.
///
/// Handles:
/// - Nanosecond precision: `2024-07-03T08:53:25.668504731Z` → `2024-07-03T08:53:25.668Z`
/// - Timezone offset: `2024-03-19T13:54:48.081077+01:00` → `2024-03-19T12:54:48.081Z`
fn normalize_timestamp(ts: &str) -> String {
    use chrono::{DateTime, FixedOffset, Utc};
    if let Ok(dt) = ts.parse::<DateTime<FixedOffset>>() {
        let utc: DateTime<Utc> = dt.into();
        return utc.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
    }
    ts.to_string()
}

/// Fetch all releases for `package_name` from the Bitrise steplib at `registry_url`.
///
/// Returns `None` for unsupported registries, non-array responses, missing content,
/// or when no semver-valid version directories are found.
pub async fn fetch_releases(
    package_name: &str,
    registry_url: &str,
    http: &HttpClient,
) -> Result<Option<BitriseReleasesResult>, BitriseError> {
    let Some((api_base, full_name, canonical_registry)) = parse_steplib_url(registry_url) else {
        return Ok(None);
    };

    let list_url = format!("{api_base}/repos/{full_name}/contents/steps/{package_name}");

    let resp = http.get_retrying(&list_url).await?;
    if !resp.status().is_success() {
        return Ok(None);
    }
    let body: serde_json::Value = resp.json().await.map_err(BitriseError::Json)?;

    let items = match body.as_array() {
        Some(arr) => arr.clone(),
        None => return Ok(None),
    };

    let mut releases = Vec::new();

    for item in &items {
        let name = item["name"].as_str().unwrap_or("");
        if !is_semver_version(name) {
            continue;
        }

        let step_url = format!("{api_base}/repos/{full_name}/contents/steps/{package_name}/{name}/step.yml");
        let step_resp = http.get_retrying(&step_url).await?;
        if !step_resp.status().is_success() {
            continue;
        }
        let file_body: serde_json::Value = step_resp.json().await.map_err(BitriseError::Json)?;

        // Must be a file object (not array), with base64 encoding and content
        if file_body.is_array() {
            return Ok(None);
        }
        let encoding = file_body["encoding"].as_str();
        let content_b64 = file_body["content"].as_str();

        match (encoding, content_b64) {
            (None, None) if !file_body["content"].is_string() => {
                // No content field at all
                return Ok(None);
            }
            _ => {}
        }

        if content_b64.is_none() {
            return Ok(None);
        }
        match encoding {
            Some("base64") => {}
            Some(_) => return Ok(None),
            None => return Ok(None),
        }

        let (ts, src) = parse_step_yaml(content_b64.unwrap());
        releases.push(BitriseRelease {
            version: name.to_string(),
            release_timestamp: ts,
            source_url: src,
        });
    }

    if releases.is_empty() {
        return Ok(None);
    }

    Ok(Some(BitriseReleasesResult {
        releases,
        homepage: format!("https://bitrise.io/integrations/steps/{package_name}"),
        registry_url: canonical_registry,
    }))
}

/// Compatibility wrapper for ci.rs — finds the latest semver version.
pub async fn fetch_latest(
    http: &HttpClient,
    step_name: &str,
    current_value: &str,
    registry_url: &str,
) -> Result<BitriseUpdateSummary, BitriseError> {
    let result = fetch_releases(step_name, registry_url, http).await?;
    let latest = result.and_then(|r| {
        r.releases.into_iter().map(|rel| rel.version).last()
    });
    let update_available = latest.as_deref() != Some(current_value);
    Ok(BitriseUpdateSummary { latest, update_available })
}

#[cfg(test)]
mod tests {
    use base64::Engine;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    fn b64(s: &str) -> String {
        base64::engine::general_purpose::STANDARD.encode(s)
    }

    fn dir_item(name: &str) -> serde_json::Value {
        serde_json::json!({"type": "dir", "name": name, "path": format!("steps/script/{name}")})
    }

    fn step_file(content: &str) -> serde_json::Value {
        serde_json::json!({
            "type": "file",
            "name": "step.yml",
            "encoding": "base64",
            "content": b64(content),
        })
    }

    // Ported: "returns null for unsupported registryUrl" — datasource/bitrise/index.spec.ts line 7
    #[tokio::test]
    async fn returns_null_for_unsupported_registry_url() {
        let http = HttpClient::new().unwrap();
        let result = fetch_releases("script", "https://gitlab.com/bitrise-io/bitrise-steplib", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "support GitHub Enterprise API URL" — datasource/bitrise/index.spec.ts line 20
    #[tokio::test]
    async fn support_github_enterprise_api_url() {
        let server = MockServer::start().await;
        let base = server.uri();

        Mock::given(method("GET"))
            .and(path("/api/v3/repos/foo/bar/contents/steps/script"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([dir_item("1.0.0")])))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/api/v3/repos/foo/bar/contents/steps/script/1.0.0/step.yml"))
            .respond_with(ResponseTemplate::new(200).set_body_json(step_file(
                "published_at: 2024-03-19T13:54:48.081077+01:00\nsource_code_url: https://github.com/bitrise-steplib/bitrise-step-script\nwebsite: https://github.com/bitrise-steplib/bitrise-step-script\n"
            )))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let registry = format!("{base}/foo/bar");
        let result = fetch_releases("script", &registry, &http).await.unwrap().unwrap();

        assert_eq!(result.homepage, "https://bitrise.io/integrations/steps/script");
        assert_eq!(result.releases.len(), 1);
        assert_eq!(result.releases[0].version, "1.0.0");
        assert_eq!(result.releases[0].release_timestamp.as_deref(), Some("2024-03-19T12:54:48.081Z"));
        assert_eq!(result.releases[0].source_url.as_deref(), Some("https://github.com/bitrise-steplib/bitrise-step-script"));
    }

    // Ported: "returns version and filters out the asset folder" — datasource/bitrise/index.spec.ts line 55
    #[tokio::test]
    async fn returns_version_and_filters_asset_folder() {
        let server = MockServer::start().await;
        let base = server.uri();

        Mock::given(method("GET"))
            .and(path("/api/v3/repos/bitrise-io/bitrise-steplib/contents/steps/activate-build-cache-for-bazel"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                dir_item("1.0.0"),
                dir_item("1.0.1"),
                {"type": "dir", "name": "assets", "path": "steps/activate-build-cache-for-bazel/assets"},
            ])))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/api/v3/repos/bitrise-io/bitrise-steplib/contents/steps/activate-build-cache-for-bazel/1.0.0/step.yml"))
            .respond_with(ResponseTemplate::new(200).set_body_json(step_file(
                "published_at: 2024-03-19T13:54:48.081077+01:00\nsource_code_url: https://github.com/bitrise-steplib/bitrise-step-activate-build-cache-for-bazel\n"
            )))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/api/v3/repos/bitrise-io/bitrise-steplib/contents/steps/activate-build-cache-for-bazel/1.0.1/step.yml"))
            .respond_with(ResponseTemplate::new(200).set_body_json(step_file(
                "published_at: \"2024-07-03T08:53:25.668504731Z\"\nsource_code_url: https://github.com/bitrise-steplib/bitrise-step-activate-build-cache-for-bazel\n"
            )))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let registry = format!("{base}/bitrise-io/bitrise-steplib");
        let result = fetch_releases("activate-build-cache-for-bazel", &registry, &http).await.unwrap().unwrap();

        assert_eq!(result.releases.len(), 2);
        assert_eq!(result.releases[0].version, "1.0.0");
        assert_eq!(result.releases[0].release_timestamp.as_deref(), Some("2024-03-19T12:54:48.081Z"));
        assert_eq!(result.releases[1].version, "1.0.1");
        assert_eq!(result.releases[1].release_timestamp.as_deref(), Some("2024-07-03T08:53:25.668Z"));
    }

    // Ported: "returns null if there are no releases" — datasource/bitrise/index.spec.ts line 123
    #[tokio::test]
    async fn returns_null_if_no_releases() {
        let server = MockServer::start().await;
        let base = server.uri();

        Mock::given(method("GET"))
            .and(path("/api/v3/repos/bitrise-io/bitrise-steplib/contents/steps/activate-build-cache-for-bazel"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"type": "dir", "name": "assets", "path": "..."},
            ])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let registry = format!("{base}/bitrise-io/bitrise-steplib");
        let result = fetch_releases("activate-build-cache-for-bazel", &registry, &http).await.unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null if the package has an unexpected format" — datasource/bitrise/index.spec.ts line 137
    #[tokio::test]
    async fn returns_null_for_unexpected_format() {
        let server = MockServer::start().await;
        let base = server.uri();

        Mock::given(method("GET"))
            .and(path("/api/v3/repos/bitrise-io/bitrise-steplib/contents/steps/activate-build-cache-for-bazel"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "type": "file", "name": "assets", "path": "..."
            })))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let registry = format!("{base}/bitrise-io/bitrise-steplib");
        let result = fetch_releases("activate-build-cache-for-bazel", &registry, &http).await.unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null if the file object has no content" — datasource/bitrise/index.spec.ts line 153
    #[tokio::test]
    async fn returns_null_if_no_content() {
        let server = MockServer::start().await;
        let base = server.uri();

        Mock::given(method("GET"))
            .and(path("/api/v3/repos/bitrise-io/bitrise-steplib/contents/steps/script"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([dir_item("1.0.0")])))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/api/v3/repos/bitrise-io/bitrise-steplib/contents/steps/script/1.0.0/step.yml"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "type": "file",
                "name": "step.yml",
                "path": "steps/script/1.0.0/step.yml",
            })))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let registry = format!("{base}/bitrise-io/bitrise-steplib");
        let result = fetch_releases("script", &registry, &http).await.unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null if the file object has an unexpected encoding" — datasource/bitrise/index.spec.ts line 181
    #[tokio::test]
    async fn returns_null_for_unexpected_encoding() {
        let server = MockServer::start().await;
        let base = server.uri();

        Mock::given(method("GET"))
            .and(path("/api/v3/repos/bitrise-io/bitrise-steplib/contents/steps/script"))
            .respond_with(ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([dir_item("1.0.0")])))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/api/v3/repos/bitrise-io/bitrise-steplib/contents/steps/script/1.0.0/step.yml"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "type": "file",
                "name": "step.yml",
                "path": "steps/script/1.0.0/step.yml",
                "encoding": "none",
                "content": "",
            })))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let registry = format!("{base}/bitrise-io/bitrise-steplib");
        let result = fetch_releases("script", &registry, &http).await.unwrap();
        assert!(result.is_none());
    }
}
