//! Puppet Forge datasource.
//!
//! Renovate reference: `lib/modules/datasource/puppet-forge/index.ts`
//! API: `GET {registry}/v3/modules/{author}-{name}?exclude_fields=current_release`

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY: &str = "https://forgeapi.puppet.com";
pub const DATASOURCE_ID: &str = "puppet-forge";

#[derive(Debug, Error)]
pub enum PuppetForgeError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

#[derive(Debug, Clone)]
pub struct PuppetForgeRelease {
    pub version: String,
    pub download_url: Option<String>,
    pub release_timestamp: Option<String>,
    pub registry_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PuppetForgeResult {
    pub releases: Vec<PuppetForgeRelease>,
    pub source_url: Option<String>,
    pub deprecation_message: Option<String>,
}

// ── API types ──────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct ForgeModule {
    #[serde(default)]
    releases: Vec<ForgeRelease>,
    homepage_url: Option<String>,
    deprecated_for: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ForgeRelease {
    version: String,
    file_uri: Option<String>,
    created_at: Option<String>,
}

fn parse_puppet_timestamp(s: &str) -> Option<String> {
    chrono::DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S %z")
        .ok()
        .map(|dt| dt.with_timezone(&chrono::Utc).format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string())
}

/// Fetch Puppet Forge module releases.
///
/// 4xx / HTTP error → `Ok(None)`. Empty releases → `Ok(None)`.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<PuppetForgeResult>, PuppetForgeError> {
    let base = registry_url.trim_end_matches('/');
    let slug = package_name.replace('/', "-");
    let url = format!("{base}/v3/modules/{slug}?exclude_fields=current_release");

    let text = match http.get_raw_with_accept(&url, "application/json").await {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    let module: ForgeModule = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    if module.releases.is_empty() {
        return Ok(None);
    }

    let mut releases: Vec<PuppetForgeRelease> = module
        .releases
        .into_iter()
        .map(|r| PuppetForgeRelease {
            version: r.version,
            download_url: r.file_uri,
            release_timestamp: r.created_at.as_deref().and_then(parse_puppet_timestamp),
            registry_url: Some(registry_url.trim_end_matches('/').to_string()),
        })
        .collect();

    // Sort by semver ascending (matches Renovate's sortAndRemoveDuplicates).
    releases.sort_by(|a, b| {
        let av = semver::Version::parse(&a.version).ok();
        let bv = semver::Version::parse(&b.version).ok();
        match (av, bv) {
            (Some(av), Some(bv)) => av.cmp(&bv),
            (Some(_), None) => std::cmp::Ordering::Greater,
            (None, Some(_)) => std::cmp::Ordering::Less,
            (None, None) => a.version.cmp(&b.version),
        }
    });

    Ok(Some(PuppetForgeResult {
        releases,
        source_url: module.homepage_url,
        deprecation_message: module.deprecated_for,
    }))
}

/// Update summary used by pipeline.
#[derive(Debug)]
pub struct PuppetForgeUpdateSummary {
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Fetch latest version (pipeline helper).
pub async fn fetch_latest(
    http: &HttpClient,
    module_name: &str,
    current_value: &str,
    registry_url: &str,
) -> Result<PuppetForgeUpdateSummary, PuppetForgeError> {
    let base = if registry_url.is_empty() { DEFAULT_REGISTRY } else { registry_url };
    let result = fetch_releases(base, module_name, http).await?;
    let latest = result.and_then(|r| r.releases.last().map(|rel| rel.version.clone()));
    let update_available = latest.as_deref().map(|l| l != current_value).unwrap_or(false);
    Ok(PuppetForgeUpdateSummary { latest, update_available })
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    const RESPONSE_JSON: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/puppet-forge/__fixtures__/puppetforge-response.json"
    );
    const DEPRECATED_FOR_JSON: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/puppet-forge/__fixtures__/puppetforge-deprecated-for.json"
    );
    const WITH_NULLS_JSON: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/puppet-forge/__fixtures__/puppetforge-response-with-nulls.json"
    );
    const NO_RELEASES_JSON: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/puppet-forge/__fixtures__/puppetforge-no-releases.json"
    );

    // Ported: "should use default forge if no other provided" — datasource/puppet-forge/index.spec.ts line 12
    #[tokio::test]
    async fn uses_default_forge() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v3/modules/puppetlabs-apache"))
            .and(query_param("exclude_fields", "current_release"))
            .respond_with(ResponseTemplate::new(200).set_body_string(RESPONSE_JSON))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "puppetlabs/apache", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 4);
        // Sorted ascending by semver
        assert_eq!(result.releases[0].version, "6.4.0");
        assert_eq!(result.releases[3].version, "7.0.0");
    }

    // Ported: "parses real data" — datasource/puppet-forge/index.spec.ts line 34
    #[tokio::test]
    async fn parses_real_data() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v3/modules/puppetlabs-apache"))
            .and(query_param("exclude_fields", "current_release"))
            .respond_with(ResponseTemplate::new(200).set_body_string(RESPONSE_JSON))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "puppetlabs/apache", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 4);

        let r0 = &result.releases[0];
        assert_eq!(r0.version, "6.4.0");
        assert_eq!(r0.download_url.as_deref(), Some("/v3/files/puppetlabs-apache-6.4.0.tar.gz"));
        assert_eq!(r0.release_timestamp.as_deref(), Some("2021-08-02T13:49:41.000Z"));

        let r3 = &result.releases[3];
        assert_eq!(r3.version, "7.0.0");
        assert_eq!(r3.release_timestamp.as_deref(), Some("2021-10-11T14:47:24.000Z"));

        assert_eq!(result.source_url.as_deref(), Some("https://github.com/puppetlabs/puppetlabs-apache"));
        assert!(result.deprecation_message.is_none());
    }

    // Ported: "has a deprecated for reason" — datasource/puppet-forge/index.spec.ts line 79
    #[tokio::test]
    async fn has_deprecated_for_reason() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v3/modules/puppetlabs-apache"))
            .and(query_param("exclude_fields", "current_release"))
            .respond_with(ResponseTemplate::new(200).set_body_string(DEPRECATED_FOR_JSON))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "puppetlabs/apache", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.deprecation_message.as_deref(), Some("use another module ..."));
        assert_eq!(result.releases.len(), 1);
        assert_eq!(result.releases[0].version, "7.0.0");
    }

    // Ported: "should return null if lookup fails 400" — datasource/puppet-forge/index.spec.ts line 107
    #[tokio::test]
    async fn returns_null_for_400() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v3/modules/foobar"))
            .and(query_param("exclude_fields", "current_release"))
            .respond_with(ResponseTemplate::new(400))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "foobar", &http).await.unwrap();
        assert!(result.is_none());
    }

    // Ported: "should return null if lookup fails" — datasource/puppet-forge/index.spec.ts line 123
    #[tokio::test]
    async fn returns_null_for_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v3/modules/foobar"))
            .and(query_param("exclude_fields", "current_release"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "foobar", &http).await.unwrap();
        assert!(result.is_none());
    }

    // Ported: "should fetch package info from custom registry" — datasource/puppet-forge/index.spec.ts line 137
    #[tokio::test]
    async fn fetches_from_custom_registry() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v3/modules/foobar"))
            .and(query_param("exclude_fields", "current_release"))
            .respond_with(ResponseTemplate::new(200).set_body_string(RESPONSE_JSON))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "foobar", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 4);
        assert_eq!(result.releases[0].version, "6.4.0");
        assert_eq!(result.releases[0].release_timestamp.as_deref(), Some("2021-08-02T13:49:41.000Z"));
        assert_eq!(result.releases[3].version, "7.0.0");
        assert_eq!(result.source_url.as_deref(), Some("https://github.com/puppetlabs/puppetlabs-apache"));
    }

    // Ported: "load all possible null values" — datasource/puppet-forge/index.spec.ts line 182
    #[tokio::test]
    async fn loads_null_values() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v3/modules/foobar"))
            .and(query_param("exclude_fields", "current_release"))
            .respond_with(ResponseTemplate::new(200).set_body_string(WITH_NULLS_JSON))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "foobar", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 1);
        assert_eq!(result.releases[0].version, "7.0.0");
        assert_eq!(result.releases[0].release_timestamp.as_deref(), Some("2021-10-11T14:47:24.000Z"));
        assert!(result.deprecation_message.is_none());
    }

    // Ported: "no releases available -> return null" — datasource/puppet-forge/index.spec.ts line 208
    #[tokio::test]
    async fn returns_null_for_no_releases() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/v3/modules/foobar"))
            .and(query_param("exclude_fields", "current_release"))
            .respond_with(ResponseTemplate::new(200).set_body_string(NO_RELEASES_JSON))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "foobar", &http).await.unwrap();
        assert!(result.is_none());
    }
}
