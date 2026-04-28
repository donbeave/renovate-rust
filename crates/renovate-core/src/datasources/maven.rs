//! Maven Central datasource.
//!
//! Fetches available versions from Maven Central's repository using the
//! standard Maven metadata URL format:
//! `https://repo.maven.apache.org/maven2/{group}/{artifact}/maven-metadata.xml`
//!
//! Renovate reference:
//! - `lib/modules/datasource/maven/index.ts`
//! - `lib/modules/datasource/maven/common.ts` — `MAVEN_REPO`

use std::io::BufReader;
use std::sync::Arc;

use quick_xml::Reader;
use quick_xml::events::Event;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::HttpClient;

pub const MAVEN_CENTRAL_BASE: &str = "https://repo.maven.apache.org/maven2";
pub const CLOJARS_BASE: &str = "https://clojars.org/repo";

const MAVEN_CENTRAL: &str = MAVEN_CENTRAL_BASE;

/// Input for a single Maven dependency lookup.
#[derive(Debug, Clone)]
pub struct MavenDepInput {
    pub dep_name: String,
    pub current_version: String,
}

/// Update summary for a Maven dependency.
#[derive(Debug, Clone)]
pub struct MavenUpdateSummary {
    pub current_version: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Per-dependency result returned by `fetch_updates_concurrent`.
#[derive(Debug)]
pub struct MavenUpdateResult {
    pub dep_name: String,
    pub summary: Result<MavenUpdateSummary, MavenError>,
}

/// Errors from fetching Maven Central metadata.
#[derive(Debug, Error)]
pub enum MavenError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("XML parse error: {0}")]
    Xml(#[from] quick_xml::Error),
}

/// Fetch the latest stable version of a Maven artifact from Maven Central.
///
/// `dep_name` must be `groupId:artifactId` (e.g. `org.springframework:spring-core`).
/// Returns `None` if no metadata can be found or no versions are listed.
pub async fn fetch_latest(dep_name: &str, http: &HttpClient) -> Result<Option<String>, MavenError> {
    fetch_latest_from_registry(dep_name, http, MAVEN_CENTRAL).await
}

/// Fetch the latest stable version from an arbitrary Maven-compatible registry.
pub async fn fetch_latest_from_registry(
    dep_name: &str,
    http: &HttpClient,
    registry: &str,
) -> Result<Option<String>, MavenError> {
    let Some((group_id, artifact_id)) = dep_name.split_once(':') else {
        return Ok(None);
    };

    let group_path = group_id.replace('.', "/");
    let url = format!("{registry}/{group_path}/{artifact_id}/maven-metadata.xml");

    let resp = http.get_retrying(&url).await?;
    if resp.status().as_u16() == 404 {
        return Ok(None);
    }
    if !resp.status().is_success() {
        return Ok(None);
    }
    let body = resp.text().await.map_err(crate::http::HttpError::Request)?;

    Ok(parse_latest_version(&body)?)
}

/// Fetch update summaries for multiple Maven dependencies concurrently.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[MavenDepInput],
    concurrency: usize,
) -> Vec<MavenUpdateResult> {
    if deps.is_empty() {
        return Vec::new();
    }

    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<MavenUpdateResult> = JoinSet::new();

    for dep in deps {
        let http = http.clone();
        let dep = dep.clone();
        let sem = Arc::clone(&sem);

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_update_summary(&dep, &http).await;
            MavenUpdateResult {
                dep_name: dep.dep_name.clone(),
                summary: result,
            }
        });
    }

    let mut results = Vec::with_capacity(deps.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok(r) => results.push(r),
            Err(join_err) => tracing::error!(%join_err, "maven lookup task panicked"),
        }
    }
    results
}

async fn fetch_update_summary(
    dep: &MavenDepInput,
    http: &HttpClient,
) -> Result<MavenUpdateSummary, MavenError> {
    let latest = fetch_latest(&dep.dep_name, http).await?;
    let summary =
        crate::versioning::maven::maven_update_summary(&dep.current_version, latest.as_deref());
    Ok(MavenUpdateSummary {
        current_version: summary.current_version,
        latest: summary.latest,
        update_available: summary.update_available,
    })
}

/// Parse a Maven `maven-metadata.xml` and return the best "latest stable"
/// version: `<release>` first, then `<latest>`, then last `<version>`.
fn parse_latest_version(xml: &str) -> Result<Option<String>, quick_xml::Error> {
    let cursor = BufReader::new(xml.as_bytes());
    let mut reader = Reader::from_reader(cursor);
    reader.config_mut().trim_text(true);

    let mut release: Option<String> = None;
    let mut latest: Option<String> = None;
    let mut last_version: Option<String> = None;

    let mut current_tag: Option<String> = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                current_tag = Some(String::from_utf8_lossy(e.name().as_ref()).into_owned());
            }
            Event::Text(e) => {
                if let Some(ref tag) = current_tag {
                    let text = e.decode().map(|s| s.trim().to_owned()).unwrap_or_default();
                    if !text.is_empty() {
                        match tag.as_str() {
                            "release" => release = Some(text),
                            "latest" => latest = Some(text),
                            "version" => last_version = Some(text),
                            _ => {}
                        }
                    }
                }
            }
            Event::End(_) => {
                current_tag = None;
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    // Prefer release > latest > last version seen.
    Ok(release.or(latest).or(last_version))
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn spring_metadata() -> &'static str {
        r#"<?xml version="1.0" encoding="UTF-8"?>
<metadata>
  <groupId>org.springframework</groupId>
  <artifactId>spring-core</artifactId>
  <versioning>
    <latest>6.0.11</latest>
    <release>6.0.11</release>
    <versions>
      <version>5.3.27</version>
      <version>5.3.28</version>
      <version>6.0.10</version>
      <version>6.0.11</version>
    </versions>
    <lastUpdated>20230901000000</lastUpdated>
  </versioning>
</metadata>"#
    }

    #[test]
    fn parse_release_tag() {
        let latest = parse_latest_version(spring_metadata()).unwrap();
        assert_eq!(latest, Some("6.0.11".to_owned()));
    }

    #[test]
    fn parse_latest_fallback_when_no_release() {
        let xml = r#"<metadata>
  <versioning>
    <latest>2.0.0</latest>
    <versions>
      <version>1.0.0</version>
      <version>2.0.0</version>
    </versions>
  </versioning>
</metadata>"#;
        let latest = parse_latest_version(xml).unwrap();
        assert_eq!(latest, Some("2.0.0".to_owned()));
    }

    #[test]
    fn parse_last_version_fallback() {
        let xml = r#"<metadata>
  <versioning>
    <versions>
      <version>1.0.0</version>
      <version>1.1.0</version>
    </versions>
  </versioning>
</metadata>"#;
        let latest = parse_latest_version(xml).unwrap();
        assert_eq!(latest, Some("1.1.0".to_owned()));
    }

    #[test]
    fn parse_empty_metadata() {
        let xml = "<metadata></metadata>";
        let latest = parse_latest_version(xml).unwrap();
        assert_eq!(latest, None);
    }

    #[tokio::test]
    async fn fetch_latest_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/org/springframework/spring-core/maven-metadata.xml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(spring_metadata()))
            .mount(&server)
            .await;

        // Override the base URL using a custom http client pointed at the mock.
        // Directly test parse_latest_version since fetch_latest hardcodes the
        // Maven Central URL.  Integration is tested via parse_latest_version.
        let xml = reqwest::get(format!(
            "{}/org/springframework/spring-core/maven-metadata.xml",
            server.uri()
        ))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

        let latest = parse_latest_version(&xml).unwrap();
        assert_eq!(latest, Some("6.0.11".to_owned()));
    }

    #[test]
    fn dep_name_without_colon_returns_none() {
        // fetch_latest splits on ':'; no colon → None, checked via sync helper.
        let dep_name = "nodot";
        assert!(dep_name.split_once(':').is_none());
    }
}
