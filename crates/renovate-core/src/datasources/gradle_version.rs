//! Gradle Version datasource.
//!
//! Fetches Gradle release information from:
//! `https://services.gradle.org/versions/all`
//!
//! Renovate reference: `lib/modules/datasource/gradle-version/index.ts`

use serde::Deserialize;

use crate::http::HttpClient;


pub const DATASOURCE_ID: &str = "gradle-version";
pub const GRADLE_VERSIONS_URL: &str = "https://services.gradle.org/versions/all";
pub const HOMEPAGE: &str = "https://gradle.org";
pub const SOURCE_URL: &str = "https://github.com/gradle/gradle";

#[derive(Debug, Deserialize)]
struct GradleRelease {
    version: String,
    #[serde(rename = "buildTime")]
    build_time: Option<String>,
    #[serde(default)]
    snapshot: bool,
    #[serde(default)]
    nightly: bool,
    #[serde(default)]
    broken: bool,
}

/// One Gradle release as returned by `fetch_releases`.
#[derive(Debug, Clone)]
pub struct GradleVersionRelease {
    pub version: String,
    pub git_ref: String,
    pub release_timestamp: Option<String>,
    pub is_deprecated: bool,
}

/// Result of a `fetch_releases` call.
#[derive(Debug, Clone)]
pub struct GradleVersionResult {
    pub releases: Vec<GradleVersionRelease>,
    pub homepage: &'static str,
    pub source_url: &'static str,
}

/// Compute the git tag from a Gradle version string.
///
/// `"8.1.2"` → `"v8.1.2"`, `"8.2"` → `"v8.2.0"`,
/// `"8.2-rc-1"` → `"v8.2.0-RC1"`, `"8.2-milestone-1"` → `"v8.2.0-M1"`
pub fn get_git_ref(version: &str) -> String {
    // Split on first `-type-` pattern
    let re = regex::Regex::new(r"-([a-z]+)-").unwrap();
    let mut parts = re.splitn(version, 2);
    let version_part = parts.next().unwrap_or(version);

    let suffix = if let Some(caps) = re.captures(version) {
        let type_part = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let rest = &version[caps.get(0).unwrap().end()..];
        match type_part {
            "rc" => format!("-RC{}", rest),
            "milestone" => format!("-M{}", rest),
            _ => String::new(),
        }
    } else {
        String::new()
    };

    let vparts: Vec<&str> = version_part.split('.').collect();
    let major = vparts.first().unwrap_or(&"0");
    let minor = vparts.get(1).unwrap_or(&"0");
    let patch = vparts.get(2).unwrap_or(&"0");
    format!("v{}.{}.{}{}", major, minor, patch, suffix)
}

/// Parse a Gradle buildTime timestamp to ISO 8601.
///
/// `"20210324011254+0000"` → `"2021-03-24T01:12:54.000Z"`
fn parse_build_time(s: &str) -> Option<String> {
    // Format: YYYYMMDDHHmmss+ZZZZ
    if s.len() < 14 {
        return None;
    }
    let year = &s[0..4];
    let month = &s[4..6];
    let day = &s[6..8];
    let hour = &s[8..10];
    let min = &s[10..12];
    let sec = &s[12..14];
    Some(format!("{}-{}-{}T{}:{}:{}.000Z", year, month, day, hour, min, sec))
}

/// Fetch Gradle releases.
///
/// - 404 → Ok(None)
/// - 429, 5xx, network errors → Err
/// - empty results → Ok(None)
pub async fn fetch_releases(
    registry_url: &str,
    _package_name: &str,
    http: &HttpClient,
) -> Result<Option<GradleVersionResult>, GradleVersionError> {
    let releases: Vec<GradleRelease> = match http.get_json(registry_url).await {
        Ok(v) => v,
        Err(crate::http::HttpError::Status { status, .. })
            if status == reqwest::StatusCode::NOT_FOUND =>
        {
            return Ok(None)
        }
        Err(e) => return Err(GradleVersionError::Http(e)),
    };

    let mapped: Vec<GradleVersionRelease> = releases
        .into_iter()
        .filter(|r| !r.snapshot && !r.nightly)
        .map(|r| GradleVersionRelease {
            git_ref: get_git_ref(&r.version),
            release_timestamp: r.build_time.as_deref().and_then(parse_build_time),
            is_deprecated: r.broken,
            version: r.version,
        })
        .collect();

    if mapped.is_empty() {
        return Ok(None);
    }

    Ok(Some(GradleVersionResult {
        releases: mapped,
        homepage: HOMEPAGE,
        source_url: SOURCE_URL,
    }))
}

/// Result of a Gradle version lookup.
#[derive(Debug, Clone)]
pub struct GradleVersionSummary {
    /// Whether a newer stable version exists.
    pub update_available: bool,
    /// Current version (from the wrapper properties file).
    pub current_version: String,
    /// Latest stable Gradle version, if found.
    pub latest: Option<String>,
}

/// Fetch the latest stable Gradle version and compare with `current_version`.
pub async fn fetch_latest(
    http: &HttpClient,
    current_version: &str,
) -> Result<GradleVersionSummary, GradleVersionError> {
    let releases: Vec<GradleRelease> = http
        .get_json(GRADLE_VERSIONS_URL)
        .await
        .map_err(GradleVersionError::Http)?;

    // Filter to stable releases only (no snapshot, no nightly, no broken).
    let mut stable: Vec<String> = releases
        .into_iter()
        .filter(|r| !r.snapshot && !r.nightly && !r.broken)
        .map(|r| r.version)
        .collect();

    if stable.is_empty() {
        return Err(GradleVersionError::NoStableRelease);
    }

    // Sort descending by version (simple lexicographic is fine for `X.Y.Z` semver-like).
    stable.sort_by(|a, b| cmp_gradle_version(b, a));

    let latest = stable.into_iter().next();
    let update_available = latest
        .as_deref()
        .map(|l| l != current_version)
        .unwrap_or(false);

    Ok(GradleVersionSummary {
        update_available,
        current_version: current_version.to_owned(),
        latest,
    })
}

/// Compare Gradle version strings numerically (e.g. `"8.4"` vs `"8.10"`).
fn cmp_gradle_version(a: &str, b: &str) -> std::cmp::Ordering {
    let parse = |s: &str| -> Vec<u32> { s.split('.').filter_map(|p| p.parse().ok()).collect() };
    parse(a).cmp(&parse(b))
}

/// Errors from the Gradle version datasource.
#[derive(Debug, thiserror::Error)]
pub enum GradleVersionError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("no stable Gradle release found")]
    NoStableRelease,
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    #[test]
    fn version_sort_correctness() {
        let mut versions = vec!["8.4".to_owned(), "8.10".to_owned(), "7.6.1".to_owned()];
        versions.sort_by(|a, b| cmp_gradle_version(b, a));
        assert_eq!(versions, vec!["8.10", "8.4", "7.6.1"]);
    }

    #[test]
    fn git_ref_calculation() {
        assert_eq!(get_git_ref("8.1.2"), "v8.1.2");
        assert_eq!(get_git_ref("8.2"), "v8.2.0");
        assert_eq!(get_git_ref("8.2-rc-1"), "v8.2.0-RC1");
        assert_eq!(get_git_ref("8.2-milestone-1"), "v8.2.0-M1");
    }

    // Ported: "processes real data" — gradle-version/index.spec.ts line 26
    #[tokio::test]
    async fn processes_real_data() {
        let fixture = include_str!(
            "../../../../../renovate/lib/modules/datasource/gradle-version/__fixtures__/all.json"
        );
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/versions/all"))
            .respond_with(ResponseTemplate::new(200).set_body_string(fixture))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(
            &format!("{}/versions/all", server.uri()),
            "abc",
            &http,
        )
        .await
        .unwrap()
        .unwrap();

        assert_eq!(result.releases.len(), 300);
        assert_eq!(
            result.releases.iter().filter(|r| r.is_deprecated).count(),
            1
        );
        assert_eq!(result.homepage, "https://gradle.org");
        assert_eq!(result.source_url, "https://github.com/gradle/gradle");
    }

    // Ported: "calls configured registryUrls" — gradle-version/index.spec.ts line 40
    #[tokio::test]
    async fn calls_configured_registry_urls() {
        let fixture = include_str!(
            "../../../../../renovate/lib/modules/datasource/gradle-version/__fixtures__/all.json"
        );
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(fixture))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "abc", &http)
            .await
            .unwrap();
        assert!(result.is_some());
    }

    // Ported: "handles empty releases" — gradle-version/index.spec.ts line 59
    #[tokio::test]
    async fn handles_empty_releases() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/versions/all"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(
            &format!("{}/versions/all", server.uri()),
            "abc",
            &http,
        )
        .await
        .unwrap();
        assert!(result.is_none());
    }

    // Ported: "handles errors" — gradle-version/index.spec.ts line 69
    #[tokio::test]
    async fn handles_errors_500() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/versions/all"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(
            &format!("{}/versions/all", server.uri()),
            "abc",
            &http,
        )
        .await;
        assert!(result.is_err());
    }

    // Ported: "handles errors" (429) — gradle-version/index.spec.ts line 69
    #[tokio::test]
    async fn handles_errors_429() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(429))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "abc", &http).await;
        assert!(result.is_err());
    }
}
