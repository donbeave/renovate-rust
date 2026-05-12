//! Artifactory datasource.
//!
//! Renovate reference:
//! - `lib/modules/datasource/artifactory/index.ts`
//! - `lib/modules/datasource/custom/formats/html.ts` for generic HTML listing parsing

use std::collections::HashSet;
use std::sync::LazyLock;

use chrono::{DateTime, NaiveDateTime, Utc};
use regex::Regex;
use reqwest::StatusCode;
use thiserror::Error;

use crate::http::{HttpClient, HttpError};

static ANCHOR_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?is)<a(?P<attrs>(?:\s+[^>]*)?)>(?P<text>.*?)</a>(?P<tail>[^<]*)"#).unwrap()
});
static HREF_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?is)\bhref\s*=\s*["']([^"']+)["']"#).unwrap());
static TAG_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?is)<[^>]+>").unwrap());
static HTML_ENTITY_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"&(?:nbsp|amp|lt|gt);").unwrap());
static WHITESPACE_RUN_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s{2,}").unwrap());

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactoryRelease {
    pub version: String,
    pub release_timestamp: Option<String>,
    pub registry_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactoryReleaseResult {
    pub registry_url: Option<String>,
    pub releases: Vec<ArtifactoryRelease>,
}

#[derive(Debug, Error)]
pub enum ArtifactoryError {
    #[error("external host error: HTTP {status} at {url}")]
    ExternalHost { status: StatusCode, url: String },
}

pub async fn get_releases(
    http: &HttpClient,
    registry_urls: &[&str],
    package_name: &str,
) -> Result<Option<ArtifactoryReleaseResult>, ArtifactoryError> {
    if registry_urls.is_empty() {
        return Ok(None);
    }

    let mut merged = Vec::new();
    for registry_url in registry_urls {
        if let Some(mut result) =
            get_releases_for_registry(http, registry_url, package_name).await?
        {
            if registry_urls.len() > 1 {
                for release in &mut result.releases {
                    release.registry_url = Some((*registry_url).to_owned());
                }
            }
            merged.extend(result.releases);
        }
    }

    if merged.is_empty() {
        return Ok(None);
    }

    Ok(Some(ArtifactoryReleaseResult {
        registry_url: (registry_urls.len() == 1).then(|| registry_urls[0].to_owned()),
        releases: merged,
    }))
}

async fn get_releases_for_registry(
    http: &HttpClient,
    registry_url: &str,
    package_name: &str,
) -> Result<Option<ArtifactoryReleaseResult>, ArtifactoryError> {
    let url = join_url_parts(registry_url, package_name);
    let Ok(response) = http.get(&url).send().await else {
        return Ok(None);
    };

    let status = response.status();
    if status == StatusCode::NOT_FOUND {
        return Ok(None);
    }
    if !status.is_success() {
        return Err(ArtifactoryError::ExternalHost { status, url });
    }

    let Ok(text) = response.text().await.map_err(HttpError::Request) else {
        return Ok(None);
    };

    let releases = parse_releases(&text);
    if releases.is_empty() {
        return Ok(None);
    }

    Ok(Some(ArtifactoryReleaseResult {
        registry_url: Some(registry_url.to_owned()),
        releases,
    }))
}

fn join_url_parts(base: &str, path: &str) -> String {
    format!(
        "{}/{}",
        base.trim_end_matches('/'),
        path.trim_start_matches('/')
    )
}

pub fn parse_releases(html: &str) -> Vec<ArtifactoryRelease> {
    ANCHOR_RE
        .captures_iter(html)
        .filter_map(|captures| {
            let raw = captures.name("text")?.as_str();
            let link_text = decode_html_text(&strip_tags(raw));
            if matches!(link_text.as_str(), ".." | "../") {
                return None;
            }

            let version = link_text.strip_suffix('/').unwrap_or(&link_text).to_owned();
            if version.is_empty() {
                return None;
            }

            let release_timestamp = captures
                .name("tail")
                .and_then(|tail| parse_listing_timestamp(tail.as_str()));

            Some(ArtifactoryRelease {
                version,
                release_timestamp,
                registry_url: None,
            })
        })
        .collect()
}

pub fn parse_html_directory_listing_links(html: &str) -> Vec<String> {
    let mut fragments = vec![html.to_owned()];
    fragments.extend(extract_pre_blocks(html));

    let mut seen = HashSet::new();
    fragments
        .iter()
        .flat_map(|fragment| {
            ANCHOR_RE
                .captures_iter(fragment)
                .filter_map(|captures| {
                    captures
                        .name("attrs")
                        .and_then(|attrs| HREF_RE.captures(attrs.as_str()))
                        .and_then(|href| href.get(1))
                        .map(|m| decode_html_text(m.as_str()))
                })
                .collect::<Vec<_>>()
        })
        .filter(|version| seen.insert(version.clone()))
        .collect()
}

fn extract_pre_blocks(html: &str) -> Vec<String> {
    static PRE_RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?is)<pre(?:\s+[^>]*)?>(.*?)</pre>").unwrap());
    PRE_RE
        .captures_iter(html)
        .filter_map(|captures| captures.get(1).map(|m| m.as_str().to_owned()))
        .collect()
}

fn strip_tags(input: &str) -> String {
    TAG_RE.replace_all(input, "").into_owned()
}

fn decode_html_text(input: &str) -> String {
    HTML_ENTITY_RE
        .replace_all(input, |captures: &regex::Captures<'_>| match &captures[0] {
            "&nbsp;" => " ",
            "&amp;" => "&",
            "&lt;" => "<",
            "&gt;" => ">",
            _ => "",
        })
        .trim()
        .to_owned()
}

fn parse_listing_timestamp(tail: &str) -> Option<String> {
    let first_field = WHITESPACE_RUN_RE.split(tail.trim_start()).next()?.trim();
    if first_field.is_empty() {
        return None;
    }

    let parsed = NaiveDateTime::parse_from_str(first_field, "%d-%b-%Y %H:%M").ok()?;
    let utc: DateTime<Utc> = DateTime::from_naive_utc_and_offset(parsed, Utc);
    Some(utc.format("%Y-%m-%dT%H:%M:%S.000Z").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const RELEASES_AS_FOLDERS: &str = r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 3.2 Final//EN">
<html><body><h1>Index</h1><pre>Name      Last modified      Size</pre><hr/>
<pre>
  <a href="../">../</a>
  <a href="1.0.0/">1.0.0/</a>  21-Jul-2021 20:08    -
  <a href="1.0.1/">1.0.1/</a>  23-Aug-2021 20:03    -
  <a href="1.0.2/">1.0.2/</a>  21-Jul-2021 20:09    -
  <a href="1.0.3/">1.0.3/</a>  06-Feb-2021 09:54    -
</pre></body></html>"#;

    const RELEASES_AS_FILES: &str = r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 3.2 Final//EN">
<html><body><h1>Index</h1><pre>Name      Last modified      Size</pre><hr/>
<pre>
  <a href="..">..</a>
  <a href="1.0.0">1.0.0</a>  21-Jul-2021 20:08    -
  <a href="1.0.1">1.0.1</a>  23-Aug-2021 20:03    12 MB
  <a href="1.0.2">1.0.2</a>  21-Jul-2021 20:09    123.45 GB
  <a href="1.0.3">1.0.3</a>  06-Feb-2021 09:54    9.0 KB
</pre></body></html>"#;

    // Ported: "parses real data (folders): with slash at the end" — datasource/artifactory/index.spec.ts line 26
    #[test]
    fn parses_real_data_folders_with_slash_at_the_end() {
        let releases = parse_releases(RELEASES_AS_FOLDERS);

        assert_eq!(releases.len(), 4);
        assert_eq!(releases[0].version, "1.0.0");
        assert_eq!(
            releases[0].release_timestamp.as_deref(),
            Some("2021-07-21T20:08:00.000Z")
        );
        assert_eq!(releases[3].version, "1.0.3");
    }

    // Ported: "parses real data (files): without slash at the end" — datasource/artifactory/index.spec.ts line 42
    #[test]
    fn parses_real_data_files_without_slash_at_the_end() {
        let releases = parse_releases(RELEASES_AS_FILES);

        assert_eq!(releases.len(), 4);
        assert_eq!(releases[0].version, "1.0.0");
        assert_eq!(
            releases[1].release_timestamp.as_deref(),
            Some("2021-08-23T20:03:00.000Z")
        );
    }

    // Ported: "parses real data (merge strategy with 2 registries)" — datasource/artifactory/index.spec.ts line 58
    #[tokio::test]
    async fn parses_real_data_merge_strategy_with_two_registries() {
        let server = MockServer::start().await;
        let registry = server.uri();
        let second_registry = format!("{registry}/production");
        Mock::given(method("GET"))
            .and(path("/project"))
            .respond_with(ResponseTemplate::new(200).set_body_string(RELEASES_AS_FILES))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/production/project"))
            .respond_with(ResponseTemplate::new(200).set_body_string("<html><a>1.3.0</a></html>"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = get_releases(&http, &[&registry, &second_registry], "project")
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 5);
        assert_eq!(
            result.releases[0].registry_url.as_deref(),
            Some(registry.as_str())
        );
        assert_eq!(
            result.releases[4].registry_url.as_deref(),
            Some(second_registry.as_str())
        );
        assert_eq!(result.releases[4].version, "1.3.0");
    }

    // Ported: "returns null without registryUrl + warning" — datasource/artifactory/index.spec.ts line 80
    #[tokio::test]
    async fn returns_null_without_registry_url() {
        let http = HttpClient::new().unwrap();
        assert!(get_releases(&http, &[], "project").await.unwrap().is_none());
    }

    // Ported: "returns null for empty 200 OK" — datasource/artifactory/index.spec.ts line 94
    #[tokio::test]
    async fn returns_null_for_empty_200_ok() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/project"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string("<html><h1>Header</h1></html>"),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        assert!(
            get_releases(&http, &[server.uri().as_str()], "project")
                .await
                .unwrap()
                .is_none()
        );
    }

    // Ported: "404 returns null" — datasource/artifactory/index.spec.ts line 108
    #[tokio::test]
    async fn not_found_returns_null() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/project"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        assert!(
            get_releases(&http, &[server.uri().as_str()], "project")
                .await
                .unwrap()
                .is_none()
        );
    }

    // Ported: "throws for error diff than 404" — datasource/artifactory/index.spec.ts line 128
    #[tokio::test]
    async fn non_404_http_error_returns_external_host_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/project"))
            .respond_with(ResponseTemplate::new(502))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let err = get_releases(&http, &[server.uri().as_str()], "project")
            .await
            .unwrap_err();
        assert!(matches!(
            err,
            ArtifactoryError::ExternalHost {
                status: StatusCode::BAD_GATEWAY,
                ..
            }
        ));
    }

    // Ported: "throws no Http error" — datasource/artifactory/index.spec.ts line 139
    #[tokio::test]
    async fn request_error_returns_null() {
        let http = HttpClient::new().unwrap();
        assert!(
            get_releases(&http, &["http://127.0.0.1:9"], "project")
                .await
                .unwrap()
                .is_none()
        );
    }

    // Ported: "return releases from nginx directory listing" — datasource/custom/index.spec.ts line 738
    #[test]
    fn parses_nginx_pre_directory_listing_links() {
        let html = r#"<html><body><pre>
<a href="nginx-0.1.0.tar.gz">nginx-0.1.0.tar.gz</a>
<a href="nginx-0.1.1.tar.gz">nginx-0.1.1.tar.gz</a>
<a href="nginx-0.1.11.tar.gz">nginx-0.1.11.tar.gz</a>
</pre></body></html>"#;

        assert_eq!(
            parse_html_directory_listing_links(html),
            vec![
                "nginx-0.1.0.tar.gz".to_owned(),
                "nginx-0.1.1.tar.gz".to_owned(),
                "nginx-0.1.11.tar.gz".to_owned(),
            ]
        );
    }
}
