//! Ruby version datasource.
//!
//! Parses the `.release-list` HTML table from the Ruby downloads page to
//! build a list of stable Ruby releases.
//!
//! Renovate reference: `lib/modules/datasource/ruby-version/index.ts`
//! Registry:           `https://www.ruby-lang.org/`

use regex::Regex;
use std::sync::OnceLock;
use thiserror::Error;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY_URL: &str = "https://www.ruby-lang.org/";
pub const DATASOURCE_ID: &str = "ruby-version";

/// Errors from the Ruby version datasource.
#[derive(Debug, Error)]
pub enum RubyVersionError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

/// One parsed Ruby release.
#[derive(Debug, Clone)]
pub struct RubyRelease {
    pub version: String,
    pub release_timestamp: Option<String>,
    pub changelog_url: Option<String>,
}

/// Result of a `fetch_releases` call.
#[derive(Debug, Clone)]
pub struct RubyVersionResult {
    pub releases: Vec<RubyRelease>,
    pub homepage: &'static str,
    pub source_url: &'static str,
}

static ROW_RE: OnceLock<Regex> = OnceLock::new();
static TD_RE: OnceLock<Regex> = OnceLock::new();
static HREF_RE: OnceLock<Regex> = OnceLock::new();
static VERSION_RE: OnceLock<Regex> = OnceLock::new();

fn row_re() -> &'static Regex {
    ROW_RE.get_or_init(|| Regex::new(r"(?s)<tr>(.*?)</tr>").unwrap())
}

fn td_re() -> &'static Regex {
    TD_RE.get_or_init(|| Regex::new(r"(?s)<td>(.*?)</td>").unwrap())
}

fn href_re() -> &'static Regex {
    HREF_RE.get_or_init(|| Regex::new(r#"href="([^"]+)""#).unwrap())
}

fn version_re() -> &'static Regex {
    VERSION_RE.get_or_init(|| Regex::new(r"^\d+\.\d+\.\d+$").unwrap())
}

/// Parse the Ruby releases HTML page into a list of releases.
///
/// Returns an empty vec when the `.release-list` table is not found or has no
/// valid rows. Never returns Err — HTTP errors are handled by `fetch_releases`.
pub fn parse_releases(html: &str) -> Vec<RubyRelease> {
    // Find the release-list table.
    let start = match html.find("release-list") {
        Some(pos) => pos,
        None => return Vec::new(),
    };

    let table_start = match html[start..].find("<table") {
        Some(p) => start + p,
        None => start,
    };
    let table_end = match html[table_start..].find("</table>") {
        Some(p) => table_start + p + 8,
        None => html.len(),
    };
    let table = &html[table_start..table_end];

    let mut releases = Vec::new();

    for row_cap in row_re().captures_iter(table) {
        let row_content = &row_cap[1];
        let tds: Vec<&str> = td_re()
            .captures_iter(row_content)
            .map(|c| c.get(1).unwrap().as_str())
            .collect();

        if tds.len() < 2 {
            continue;
        }

        // First td: "Ruby X.Y.Z" — strip prefix.
        let raw_version = tds[0].trim().replace("Ruby ", "");
        let version = raw_version.trim().to_string();

        // Only accept X.Y.Z stable versions (no rc/preview/beta suffixes).
        if !version_re().is_match(&version) {
            continue;
        }

        // Second td: release date in YYYY-MM-DD format.
        let date_str = tds[1].trim();
        let release_timestamp = if date_str.len() == 10
            && date_str.chars().nth(4) == Some('-')
        {
            Some(format!("{}T00:00:00.000Z", date_str))
        } else {
            None
        };

        // Third td (optional): changelog link.
        let changelog_url = tds.get(2).and_then(|td| {
            href_re().captures(td).map(|c| {
                let href = c[1].to_string();
                if href.starts_with("http") {
                    href
                } else {
                    format!("https://www.ruby-lang.org{}", href)
                }
            })
        });

        releases.push(RubyRelease {
            version,
            release_timestamp,
            changelog_url,
        });
    }

    releases
}

/// Fetch Ruby releases from the Ruby website.
///
/// Returns `Err` for HTTP errors (404 and other status errors all propagate as
/// errors, matching TypeScript's `handleHttpErrors` behaviour).
/// Returns `None` when the page has no valid releases.
pub async fn fetch_releases(
    registry_url: &str,
    http: &HttpClient,
) -> Result<Option<RubyVersionResult>, RubyVersionError> {
    let base = registry_url.trim_end_matches('/');
    let url = format!("{}/en/downloads/releases/", base);

    let html = http
        .get_raw_with_accept(&url, "text/html")
        .await
        .map_err(RubyVersionError::Http)?;

    let releases = parse_releases(&html);
    if releases.is_empty() {
        return Ok(None);
    }

    Ok(Some(RubyVersionResult {
        releases,
        homepage: "https://www.ruby-lang.org",
        source_url: "https://github.com/ruby/ruby",
    }))
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    // Ported: "parses real data" — ruby-version/index.spec.ts line 10
    #[tokio::test]
    async fn parses_real_data() {
        let html = include_str!(
            "../../../../../renovate/lib/modules/datasource/ruby-version/__fixtures__/releases.html"
        );
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/en/downloads/releases/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(html))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), &http).await.unwrap().unwrap();
        // Stable X.Y.Z releases only; rc/preview filtered out.
        assert!(!result.releases.is_empty());
        // All versions should match X.Y.Z.
        for r in &result.releases {
            assert!(
                version_re().is_match(&r.version),
                "unexpected version: {}",
                r.version
            );
        }
        // Spot-check: 2.5.3 should be present.
        let r = result.releases.iter().find(|r| r.version == "2.5.3").unwrap();
        assert_eq!(r.release_timestamp.as_deref(), Some("2018-10-18T00:00:00.000Z"));
        assert!(r.changelog_url.as_deref().unwrap().contains("ruby-lang.org"));
    }

    // Ported: "returns null for empty result" — ruby-version/index.spec.ts line 22
    #[tokio::test]
    async fn returns_null_for_empty_result() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/en/downloads/releases/"))
            .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), &http).await.unwrap();
        assert_eq!(result.is_none(), true);
    }

    // Ported: "throws for 404" — ruby-version/index.spec.ts line 34
    #[tokio::test]
    async fn throws_for_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/en/downloads/releases/"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), &http).await;
        assert!(result.is_err(), "expected Err for 404, got: {:?}", result);
    }
}
