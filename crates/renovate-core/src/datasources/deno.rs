//! Deno registry datasource.
//!
//! Fetches module releases from the Deno API land registry.
//!
//! Renovate reference: `lib/modules/datasource/deno/index.ts`
//! API: `GET <registryUrl>/v2/modules/<name>` → versions list
//!      `GET <registryUrl>/v2/modules/<name>/<version>` → version details

use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY_URL: &str = "https://apiland.deno.dev";
pub const DATASOURCE_ID: &str = "deno";

/// Errors from the Deno datasource.
#[derive(Debug, Error)]
pub enum DenoError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

#[derive(Debug, Deserialize)]
struct ApiTag {
    kind: String,
    value: String,
}

#[derive(Debug, Deserialize)]
struct ModuleResponse {
    versions: Vec<String>,
    #[serde(default)]
    tags: Vec<ApiTag>,
}

#[derive(Debug, Deserialize)]
struct UploadOptions {
    #[serde(rename = "type")]
    upload_type: String,
    repository: String,
    #[serde(rename = "ref")]
    git_ref: String,
}

#[derive(Debug, Deserialize)]
struct VersionDetails {
    upload_options: Option<UploadOptions>,
    uploaded_at: Option<String>,
}

/// One Deno module release.
#[derive(Debug, Clone, PartialEq)]
pub struct DenoRelease {
    pub version: String,
    pub source_url: Option<String>,
    pub git_ref: Option<String>,
    pub release_timestamp: Option<String>,
}

/// Result of a `fetch_releases` call.
#[derive(Debug, Clone)]
pub struct DenoResult {
    pub releases: Vec<DenoRelease>,
    pub tags: HashMap<String, String>,
}

/// Extract the Deno module name from a package URL.
///
/// `https://deno.land/std` → `std`
/// `https://deno.land/x/postgres` → `postgres`
fn extract_module_name(package_name: &str) -> Option<String> {
    let stripped = package_name.strip_prefix("https://deno.land/")?;
    // Remove third-party prefix "x/"
    let name = stripped.strip_prefix("x/").unwrap_or(stripped);
    // Take up to first '/' or '@'
    let name = name.split('/').next()?.split('@').next()?;
    if name.is_empty() {
        return None;
    }
    Some(name.to_owned())
}

fn github_source_url(repository: &str) -> String {
    format!("https://github.com/{}", repository)
}

/// Fetch Deno module releases.
///
/// Returns `None` when the package URL doesn't match deno.land.
/// Returns `Err` for HTTP errors (including on module endpoint or per-version).
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<DenoResult>, DenoError> {
    let Some(module_name) = extract_module_name(package_name) else {
        return Ok(None);
    };

    let base = registry_url.trim_end_matches('/');
    let module_url = format!("{}/v2/modules/{}", base, module_name);

    let module_resp: ModuleResponse = http.get_json(&module_url).await?;

    let tags: HashMap<String, String> = module_resp
        .tags
        .into_iter()
        .map(|t| (t.kind, t.value))
        .collect();

    let mut releases = Vec::new();
    for version in &module_resp.versions {
        let version_url = format!("{}/{}", module_url, version);
        let raw: serde_json::Value = http.get_json(&version_url).await?;

        let details: Option<VersionDetails> = serde_json::from_value(raw).ok();

        let (source_url, git_ref, release_timestamp) =
            if let Some(d) = details.and_then(|d| d.upload_options.map(|o| (o, d.uploaded_at))) {
                let (opts, ts) = d;
                let src = if opts.upload_type == "github" {
                    Some(github_source_url(&opts.repository))
                } else {
                    None
                };
                (src, Some(opts.git_ref), ts)
            } else {
                (None, None, None)
            };

        releases.push(DenoRelease {
            version: version.clone(),
            source_url,
            git_ref,
            release_timestamp,
        });
    }

    Ok(Some(DenoResult { releases, tags }))
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    // Ported: "returns releases of standard library" — deno/index.spec.ts line 10
    #[tokio::test]
    async fn returns_releases_of_standard_library() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v2/modules/std"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "versions": ["0.163.0", "0.162.0", "0.161.0"],
                "tags": [{ "value": "top_5_percent", "kind": "popularity" }]
            })))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/v2/modules/std/0.163.0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "version": "0.163.0",
                "upload_options": {
                    "repository": "denoland/deno_std",
                    "ref": "0.163.0",
                    "type": "github"
                },
                "uploaded_at": "2022-11-08T21:10:21.592Z"
            })))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/v2/modules/std/0.162.0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "version": "0.162.0",
                "upload_options": {
                    "repository": "denoland/deno_std",
                    "ref": "0.162.0",
                    "type": "github"
                },
                "uploaded_at": "2022-10-20T12:10:21.592Z"
            })))
            .mount(&server)
            .await;

        // 0.161.0 returns invalid schema
        Mock::given(method("GET"))
            .and(path("/v2/modules/std/0.161.0"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({ "foo": "bar" })),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "https://deno.land/std", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 3);

        let r163 = &result.releases[0];
        assert_eq!(r163.version, "0.163.0");
        assert_eq!(
            r163.source_url.as_deref(),
            Some("https://github.com/denoland/deno_std")
        );
        assert_eq!(
            r163.release_timestamp.as_deref(),
            Some("2022-11-08T21:10:21.592Z")
        );

        let r162 = &result.releases[1];
        assert_eq!(r162.version, "0.162.0");
        assert_eq!(
            r162.source_url.as_deref(),
            Some("https://github.com/denoland/deno_std")
        );
        assert_eq!(
            r162.release_timestamp.as_deref(),
            Some("2022-10-20T12:10:21.592Z")
        );

        // 0.161.0 has no details but version is still present
        let r161 = &result.releases[2];
        assert_eq!(r161.version, "0.161.0");
        assert_eq!(r161.source_url, None);
        assert_eq!(r161.release_timestamp, None);

        assert_eq!(
            result.tags.get("popularity").map(|s| s.as_str()),
            Some("top_5_percent")
        );
    }

    // Ported: "throws error if module endpoint fails" — deno/index.spec.ts line 75
    #[tokio::test]
    async fn throws_error_if_module_endpoint_fails() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v2/modules/std"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "https://deno.land/std", &http).await;
        assert!(result.is_err(), "404 on module endpoint should throw");
    }

    // Ported: "throws error if version endpoint fails" — deno/index.spec.ts line 91
    #[tokio::test]
    async fn throws_error_if_version_endpoint_fails() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v2/modules/std"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "versions": ["0.163.0", "0.162.0"],
                "tags": [{ "value": "top_5_percent", "kind": "popularity" }]
            })))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/v2/modules/std/0.163.0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "version": "0.163.0",
                "upload_options": {
                    "repository": "denoland/deno_std",
                    "ref": "0.163.0",
                    "type": "github"
                },
                "uploaded_at": "2022-11-08T21:10:21.592Z"
            })))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/v2/modules/std/0.162.0"))
            .respond_with(ResponseTemplate::new(503))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "https://deno.land/std", &http).await;
        assert!(result.is_err(), "503 on version endpoint should throw");
    }

    // Ported: "returns null if we could not match a deno land dependency" — deno/index.spec.ts line 131
    #[tokio::test]
    async fn returns_null_for_non_deno_land_package() {
        let http = HttpClient::new().unwrap();
        let result = fetch_releases(DEFAULT_REGISTRY_URL, "https://myexample.com/std", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns releases of third-party library" — deno/index.spec.ts line 137
    #[tokio::test]
    async fn returns_releases_of_third_party_library() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v2/modules/postgres"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "versions": ["v0.16.0", "v0.16.1"],
                "tags": []
            })))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/v2/modules/postgres/v0.16.0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "version": "v0.16.0",
                "upload_options": {
                    "repository": "denodrivers/postgres",
                    "ref": "v0.16.0",
                    "type": "gitlab"
                },
                "uploaded_at": "2022-06-01T20:29:52.413Z"
            })))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/v2/modules/postgres/v0.16.1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "version": "v0.16.1",
                "upload_options": {
                    "repository": "denoland/deno_std",
                    "ref": "v0.16.1",
                    "type": "gitlab"
                },
                "uploaded_at": "2022-06-07T22:43:44.098Z"
            })))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "https://deno.land/x/postgres", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 2);
        assert_eq!(result.releases[0].version, "v0.16.0");
        assert_eq!(
            result.releases[0].release_timestamp.as_deref(),
            Some("2022-06-01T20:29:52.413Z")
        );
        // gitlab type → no sourceUrl
        assert_eq!(result.releases[0].source_url, None);

        assert_eq!(result.releases[1].version, "v0.16.1");
        assert_eq!(
            result.releases[1].release_timestamp.as_deref(),
            Some("2022-06-07T22:43:44.098Z")
        );
    }

    // Ported: "returns releases of a alternative registry server" — deno/index.spec.ts line 184
    #[tokio::test]
    async fn returns_releases_of_alternative_registry() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v2/modules/postgres"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "versions": ["v0.16.0", "v0.16.1"],
                "tags": []
            })))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/v2/modules/postgres/v0.16.0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "version": "v0.16.0",
                "upload_options": {
                    "repository": "denodrivers/postgres",
                    "ref": "v0.16.0",
                    "type": "gitlab"
                },
                "uploaded_at": "2022-06-01T20:29:52.413Z"
            })))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/v2/modules/postgres/v0.16.1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "version": "v0.16.1",
                "upload_options": {
                    "repository": "denoland/deno_std",
                    "ref": "v0.16.1",
                    "type": "gitlab"
                },
                "uploaded_at": "2022-06-07T22:43:44.098Z"
            })))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "https://deno.land/x/postgres", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 2);
        assert_eq!(
            result.releases[0].release_timestamp.as_deref(),
            Some("2022-06-01T20:29:52.413Z")
        );
        assert_eq!(
            result.releases[1].release_timestamp.as_deref(),
            Some("2022-06-07T22:43:44.098Z")
        );
    }
}
