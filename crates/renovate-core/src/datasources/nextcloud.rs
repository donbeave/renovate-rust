//! Nextcloud app store datasource.
//!
//! Fetches application releases from a Nextcloud appstore API endpoint.
//!
//! Renovate reference: `lib/modules/datasource/nextcloud/index.ts`
//! API: `GET <registryUrl>` → JSON array of Application objects

use serde::Deserialize;
use std::collections::HashMap;
use thiserror::Error;

use crate::http::HttpClient;

pub const DATASOURCE_ID: &str = "nextcloud";

/// Errors from the Nextcloud datasource.
#[derive(Debug, Error)]
pub enum NextcloudError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

#[derive(Debug, Deserialize)]
struct Translation {
    changelog: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApplicationRelease {
    created: String,
    is_nightly: bool,
    #[serde(default)]
    translations: HashMap<String, Translation>,
    version: String,
}

#[derive(Debug, Deserialize)]
struct Application {
    id: String,
    #[serde(default)]
    releases: Vec<ApplicationRelease>,
    website: String,
}

/// One Nextcloud app release.
#[derive(Debug, Clone, PartialEq)]
pub struct NextcloudRelease {
    pub version: String,
    pub release_timestamp: Option<String>,
    pub changelog_content: Option<String>,
    pub is_stable: bool,
}

/// Result of a `fetch_releases` call.
#[derive(Debug, Clone)]
pub struct NextcloudResult {
    pub registry_url: String,
    pub source_url: Option<String>,
    pub changelog_url: Option<String>,
    pub releases: Vec<NextcloudRelease>,
}

fn derive_changelog_url(website: &str) -> String {
    // If website matches github.com/nextcloud/..., transform to github.com/nextcloud-releases/...
    if let Some(rest) = website.strip_prefix("https://github.com/nextcloud/") {
        return format!("https://github.com/nextcloud-releases/{}", rest);
    }
    website.to_owned()
}

fn is_github_nextcloud(website: &str) -> bool {
    website.starts_with("https://github.com/nextcloud/")
}

/// Parse an ISO 8601 timestamp to millisecond precision UTC (drop sub-ms).
///
/// "2025-07-25T09:41:26.318411Z" → "2025-07-25T09:41:26.318Z"
fn to_ms_timestamp(s: &str) -> Option<String> {
    // Find the '.' for sub-second part
    let dot_pos = s.find('.')?;
    let prefix = &s[..dot_pos];
    let rest = &s[dot_pos + 1..];
    // Strip trailing 'Z' or timezone
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    let ms: String = if digits.len() >= 3 {
        digits[..3].to_string()
    } else {
        format!("{:0<3}", digits)
    };
    Some(format!("{}.{}Z", prefix, ms))
}

/// Fetch Nextcloud app releases.
///
/// Returns `None` when registryUrl is empty or the package is not found.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<NextcloudResult>, NextcloudError> {
    if registry_url.is_empty() {
        return Ok(None);
    }

    let apps: Vec<Application> = match http.get_json(registry_url).await {
        Ok(v) => v,
        Err(e) => return Err(NextcloudError::Http(e)),
    };

    let Some(app) = apps.into_iter().find(|a| a.id == package_name) else {
        return Ok(None);
    };

    let source_url = if is_github_nextcloud(&app.website) {
        Some(app.website.clone())
    } else {
        None
    };
    let changelog_url = Some(derive_changelog_url(&app.website));

    let releases = app
        .releases
        .into_iter()
        .map(|r| {
            let changelog_content = r
                .translations
                .get("en")
                .and_then(|t| t.changelog.as_deref())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_owned());
            NextcloudRelease {
                version: r.version,
                release_timestamp: to_ms_timestamp(&r.created),
                changelog_content,
                is_stable: !r.is_nightly,
            }
        })
        .collect();

    Ok(Some(NextcloudResult {
        registry_url: registry_url.to_owned(),
        source_url,
        changelog_url,
        releases,
    }))
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    // Ported: "no registryUrl" — nextcloud/index.spec.ts line 6
    #[tokio::test]
    async fn no_registry_url() {
        let http = HttpClient::new().unwrap();
        let result = fetch_releases("", "user_oidc", &http).await.unwrap();
        assert!(result.is_none());
    }

    // Ported: "no package" — nextcloud/index.spec.ts line 16
    #[tokio::test]
    async fn no_package() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_string("[]"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "user_oidc", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "package with no versions" — nextcloud/index.spec.ts line 30
    #[tokio::test]
    async fn package_with_no_versions() {
        let data = serde_json::json!([{
            "id": "user_oidc",
            "website": "https://github.com/nextcloud/user_oidc",
            "created": "2020-05-25T10:51:12.430005Z",
            "releases": []
        }]);
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&data))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "user_oidc", &http)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            result.source_url.as_deref(),
            Some("https://github.com/nextcloud/user_oidc")
        );
        assert_eq!(result.registry_url, server.uri());
        assert!(result.releases.is_empty());
    }

    // Ported: "package with website %s returns %s" — nextcloud/index.spec.ts line 56
    #[tokio::test]
    async fn package_with_website_changelog_url() {
        // Case 1: github.com/nextcloud/... → github.com/nextcloud-releases/...
        let data1 = serde_json::json!([{
            "id": "user_oidc",
            "website": "https://github.com/nextcloud/user_oidc",
            "created": "2020-05-25T10:51:12.430005Z",
            "releases": [{
                "version": "7.3.0",
                "created": "2025-07-25T09:41:26.318411Z",
                "isNightly": false,
                "translations": { "en": { "changelog": "testChangelog" } }
            }]
        }]);
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&data1))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "user_oidc", &http)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            result.changelog_url.as_deref(),
            Some("https://github.com/nextcloud-releases/user_oidc")
        );

        // Case 2: custom website → same URL as changelog
        assert_eq!(
            derive_changelog_url("https://custom.app"),
            "https://custom.app"
        );
    }

    // Ported: "package with changelog content and url" — nextcloud/index.spec.ts line 102
    #[tokio::test]
    async fn package_with_changelog_content_and_url() {
        let data = serde_json::json!([{
            "id": "user_oidc",
            "website": "https://github.com/nextcloud/user_oidc",
            "created": "2020-05-25T10:51:12.430005Z",
            "releases": [
                {
                    "version": "7.3.0",
                    "created": "2025-07-25T09:41:26.318411Z",
                    "isNightly": false,
                    "translations": { "en": { "changelog": "testChangelog" } }
                },
                {
                    "version": "7.2.0",
                    "created": "2025-04-24T09:24:43.232337Z",
                    "isNightly": true,
                    "translations": { "en": { "changelog": "" } }
                },
                {
                    "version": "7.1.0",
                    "created": "2025-01-14T09:13:25.123456Z",
                    "isNightly": false,
                    "translations": {}
                }
            ]
        }]);
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&data))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "user_oidc", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            result.changelog_url.as_deref(),
            Some("https://github.com/nextcloud-releases/user_oidc")
        );
        assert_eq!(
            result.source_url.as_deref(),
            Some("https://github.com/nextcloud/user_oidc")
        );
        assert_eq!(result.releases.len(), 3);

        let r71 = &result.releases[2];
        assert_eq!(r71.version, "7.1.0");
        assert_eq!(
            r71.release_timestamp.as_deref(),
            Some("2025-01-14T09:13:25.123Z")
        );
        assert_eq!(r71.changelog_content, None);
        assert!(r71.is_stable);

        let r72 = &result.releases[1];
        assert_eq!(r72.version, "7.2.0");
        assert_eq!(
            r72.release_timestamp.as_deref(),
            Some("2025-04-24T09:24:43.232Z")
        );
        assert_eq!(r72.changelog_content, None);
        assert!(!r72.is_stable);

        let r73 = &result.releases[0];
        assert_eq!(r73.version, "7.3.0");
        assert_eq!(
            r73.release_timestamp.as_deref(),
            Some("2025-07-25T09:41:26.318Z")
        );
        assert_eq!(r73.changelog_content.as_deref(), Some("testChangelog"));
        assert!(r73.is_stable);
    }
}
