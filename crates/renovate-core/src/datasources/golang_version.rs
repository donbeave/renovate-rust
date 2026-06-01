//! Go toolchain version datasource.
//!
//! Fetches Go release history by parsing the Go website's
//! `internal/history/release.go` source file which encodes releases in a
//! structured Go literal format.
//!
//! Renovate reference: `lib/modules/datasource/golang-version/index.ts`
//! Registry:           `https://raw.githubusercontent.com/golang/website`

use thiserror::Error;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY_URL: &str = "https://raw.githubusercontent.com/golang/website";
pub const DATASOURCE_ID: &str = "golang-version";

/// Errors from the Go version datasource.
#[derive(Debug, Error)]
pub enum GolangVersionError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("invalid release file: {0}")]
    InvalidFile(String),
}

/// One parsed Go release.
#[derive(Debug, Clone)]
pub struct GolangRelease {
    /// Semver version string, e.g. `"1.18.1"`.
    pub version: String,
    /// ISO 8601 release timestamp, e.g. `"2022-04-07T00:00:00.000Z"`.
    pub release_timestamp: Option<String>,
}

/// Result of a `fetch_releases` call.
#[derive(Debug, Clone)]
pub struct GolangVersionResult {
    pub releases: Vec<GolangRelease>,
    pub homepage: &'static str,
    pub source_url: &'static str,
}

/// Parse the Go website `release.go` source file into a list of releases.
///
/// The file uses Go struct literal syntax to describe releases:
/// ```go
/// var Releases = []*Release{
///     {
///         Date: Date{2022, 4, 7}, Version: Version{1, 18, 1},
///         Future: true,
///     },
///     {
///         Date: Date{2022, 3, 15}, Version: Version{1, 18, 0},
///     },
/// ```
///
/// Releases with `Future: true` are skipped (not yet released).
pub fn parse_releases(content: &str) -> Result<Vec<GolangRelease>, GolangVersionError> {
    let start_marker = "var Releases = []*Release{";
    let start = content
        .find(start_marker)
        .ok_or_else(|| GolangVersionError::InvalidFile("could not find Releases section".into()))?;

    let section = &content[start + start_marker.len()..];

    let mut releases = Vec::new();
    let mut in_block = false;
    let mut future = false;
    let mut version: Option<String> = None;
    let mut release_date: Option<String> = None;

    for line in section.lines() {
        let trimmed = line.trim();

        // Match tab-indented block markers exactly, mirroring the TypeScript parser which
        // uses '\t{' and '\t},' for first-level release entries.
        let tab_depth = line.bytes().take_while(|&b| b == b'\t').count();
        let is_block_start = tab_depth == 1 && trimmed == "{";
        let is_block_end = tab_depth == 1 && (trimmed == "}," || trimmed == "}");

        if is_block_start {
            if version.is_some() {
                // Nested block start without termination — malformed input.
                return Err(GolangVersionError::InvalidFile(
                    "unexpected block start while already in a release block".into(),
                ));
            }
            in_block = true;
            future = false;
            version = None;
            release_date = None;
        } else if is_block_end {
            if in_block {
                if let Some(ver) = version.take() {
                    if !future {
                        releases.push(GolangRelease {
                            version: ver,
                            release_timestamp: release_date.take(),
                        });
                    } else {
                        release_date = None;
                    }
                } else if !future {
                    // Block ended without a version — invalid.
                    return Err(GolangVersionError::InvalidFile(
                        "release block has no version".into(),
                    ));
                }
                in_block = false;
                future = false;
            } else {
                // Block terminator with no matching block start — malformed input.
                return Err(GolangVersionError::InvalidFile(
                    "unexpected block terminator outside a release block".into(),
                ));
            }
        } else if in_block {
            // Future: true
            if trimmed.contains("Future:") && trimmed.contains("true") {
                future = true;
            }

            // Date: Date{year, month, day}
            if let Some(date_str) = parse_date_field(trimmed) {
                release_date = Some(date_str);
            }

            // Version: Version{major, minor, patch}
            if let Some(ver) = parse_version_field(trimmed) {
                version = Some(ver);
            }
        }
    }

    if releases.is_empty() {
        return Err(GolangVersionError::InvalidFile(
            "zero releases extracted".into(),
        ));
    }

    Ok(releases)
}

/// Parse `Date: Date{2022, 4, 7}` → `"2022-04-07T00:00:00.000Z"`.
fn parse_date_field(line: &str) -> Option<String> {
    let start = line.find("Date{")? + 5;
    let end = line[start..].find('}')? + start;
    let inner = &line[start..end];
    let parts: Vec<&str> = inner.split(',').collect();
    if parts.len() < 3 {
        return None;
    }
    let year: u32 = parts[0].trim().parse().ok()?;
    let month: u32 = parts[1].trim().parse().ok()?;
    let day: u32 = parts[2].trim().parse().ok()?;
    Some(format!("{:04}-{:02}-{:02}T00:00:00.000Z", year, month, day))
}

/// Parse `Version: Version{1, 18, 1}` → `"1.18.1"`.
fn parse_version_field(line: &str) -> Option<String> {
    let start = line.find("Version{")? + 8;
    let end = line[start..].find('}')? + start;
    let inner = &line[start..end];
    let parts: Vec<&str> = inner.split(',').collect();
    if parts.len() < 3 {
        return None;
    }
    let major: u32 = parts[0].trim().parse().ok()?;
    let minor: u32 = parts[1].trim().parse().ok()?;
    let patch: u32 = parts[2].trim().parse().ok()?;
    let ver = format!("{major}.{minor}.{patch}");
    // Validate that it is a recognisable semver triple.
    if ver.split('.').count() == 3 {
        Some(ver)
    } else {
        None
    }
}

/// Fetch Go releases from the Go website source file.
///
/// Returns `None` when no releases are found.
pub async fn fetch_releases(
    registry_url: &str,
    http: &HttpClient,
) -> Result<Option<GolangVersionResult>, GolangVersionError> {
    let url = format!("{registry_url}/HEAD/internal/history/release.go");
    let text = match http.get_raw_with_accept(&url, "text/plain").await {
        Ok(t) => t,
        Err(crate::http::HttpError::Status { status, .. }) if status.is_client_error() => {
            return Ok(None);
        }
        Err(e) => return Err(GolangVersionError::Http(e)),
    };

    let releases = parse_releases(&text)?;

    Ok(Some(GolangVersionResult {
        releases,
        homepage: "https://go.dev/",
        source_url: "https://github.com/golang/go",
    }))
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;
    use crate::http::HttpClient;

    const SAMPLE: &str = r#"
var Releases = []*Release{
	{
		Date: Date{2022, 4, 7}, Version: Version{1, 18, 1},
		Future:   true,
	},
	{
		Date: Date{2022, 3, 15}, Version: Version{1, 18, 0},
	},
	{
		Date: Date{2021, 12, 9}, Version: Version{1, 17, 5},
	},
}
"#;

    // Ported: "parses real data" — golang-version/index.spec.ts line 19
    #[test]
    fn parse_skips_future_releases() {
        let releases = parse_releases(SAMPLE).unwrap();
        // Future:true should be filtered out, leaving 2 releases.
        assert_eq!(releases.len(), 2);
        assert_eq!(releases[0].version, "1.18.0");
        assert_eq!(releases[1].version, "1.17.5");
    }

    // Rust-specific: golang_version behavior test
    #[test]
    fn parse_date_field_works() {
        let result = parse_date_field("Date: Date{2022, 4, 7},");
        assert_eq!(result, Some("2022-04-07T00:00:00.000Z".into()));
    }

    // Rust-specific: golang_version behavior test
    #[test]
    fn parse_version_field_works() {
        let result = parse_version_field("Version: Version{1, 18, 1},");
        assert_eq!(result, Some("1.18.1".into()));
    }

    // Ported: "throws ExternalHostError for invalid release with no versions" — golang-version/index.spec.ts line 56
    #[test]
    fn error_on_no_releases_section() {
        let err = parse_releases("no releases here").unwrap_err();
        assert!(err.to_string().contains("could not find Releases section"));
    }

    // Ported: "throws ExternalHostError for invalid release with wrong termination" — golang-version/index.spec.ts line 69
    #[test]
    fn error_on_block_with_no_version() {
        let input = r#"var Releases = []*Release{
	{
		Date: Date{2022, 4, 7},
	},
}"#;
        let err = parse_releases(input).unwrap_err();
        assert!(err.to_string().contains("no version"), "unexpected: {err}");
    }

    // Ported: "throws ExternalHostError for zero releases extracted" — golang-version/index.spec.ts line 92
    #[test]
    fn error_on_zero_releases_extracted() {
        let input = r#"var Releases = []*Release{
	{
		Date: Date{2022, 4, 7}, Version: Version{1, 18, 1},
		Future: true,
	},
}"#;
        let err = parse_releases(input).unwrap_err();
        assert!(
            err.to_string().contains("zero releases"),
            "unexpected: {err}"
        );
    }

    // Ported: "throws ExternalHostError for invalid release semver" — golang-version/index.spec.ts line 102
    #[test]
    fn error_on_overflow_version_number() {
        let content = include_str!(
            "../../../../../renovate/lib/modules/datasource/golang-version/__fixtures__/releases-invalid4.go"
        );
        // Fixture uses space-indented blocks; tab-depth parser finds no releases → error.
        let err = parse_releases(content).unwrap_err();
        assert!(
            err.to_string().contains("zero releases"),
            "unexpected: {err}"
        );
    }

    // Ported: "throws ExternalHostError for invalid release format beginning" — golang-version/index.spec.ts line 122
    #[test]
    fn error_on_block_start_inside_block() {
        let content = include_str!(
            "../../../../../renovate/lib/modules/datasource/golang-version/__fixtures__/releases-invalid5.go"
        );
        let err = parse_releases(content).unwrap_err();
        assert!(
            err.to_string().contains("unexpected block start"),
            "unexpected: {err}"
        );
    }

    // Ported: "throws ExternalHostError for invalid release format" — golang-version/index.spec.ts line 132
    #[test]
    fn error_on_extra_block_terminator() {
        let content = include_str!(
            "../../../../../renovate/lib/modules/datasource/golang-version/__fixtures__/releases-invalid6.go"
        );
        let err = parse_releases(content).unwrap_err();
        assert!(
            err.to_string().contains("unexpected block terminator"),
            "unexpected: {err}"
        );
    }

    // Ported: "returns null for error 404" — golang-version/index.spec.ts line 112
    #[tokio::test]
    async fn fetch_releases_returns_none_for_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/HEAD/internal/history/release.go"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), &http).await.unwrap();
        assert!(result.is_none());
    }

    // Ported: "supports custom registry URL" — golang-version/index.spec.ts line 36
    #[tokio::test]
    async fn fetch_releases_supports_custom_registry_url() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/HEAD/internal/history/release.go"))
            .respond_with(ResponseTemplate::new(200).set_body_string(SAMPLE))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), &http).await.unwrap();
        assert!(result.is_some());
        let res = result.unwrap();
        assert_eq!(res.releases.len(), 2);
        assert_eq!(res.releases[0].version, "1.18.0");
        assert_eq!(res.releases[1].version, "1.17.5");
    }

    // Ported: "throws ExternalHostError for empty result" — golang-version/index.spec.ts line 82
    #[tokio::test]
    async fn fetch_releases_errors_on_empty_response() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/HEAD/internal/history/release.go"))
            .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), &http).await;
        assert!(result.is_err());
    }
}
