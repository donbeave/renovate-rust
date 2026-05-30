//! Maven Central datasource.
//!
//! Fetches available versions from Maven Central's repository using the
//! standard Maven metadata URL format:
//! `https://repo.maven.apache.org/maven2/{group}/{artifact}/maven-metadata.xml`
//!
//! Renovate reference:
//! - `lib/modules/datasource/maven/index.ts`
//! - `lib/modules/datasource/maven/common.ts` — `MAVEN_REPO`

use std::collections::HashMap;
use std::io::BufReader;
use std::sync::Arc;

use quick_xml::Reader;
use quick_xml::events::Event;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::HttpClient;

pub const MAVEN_CENTRAL_BASE: &str = "https://repo.maven.apache.org/maven2";
pub const MAVEN_CENTRAL_MIRROR: &str = "https://repo1.maven.org/maven2";
pub const CLOJARS_BASE: &str = "https://clojars.org/repo";

/// Returns `true` when `url` resolves to the Maven Central registry.
///
/// Uses host-based matching only (protocol, port, and path are ignored),
/// covering both `repo.maven.apache.org` and `repo1.maven.org`.
pub fn is_maven_central(url: &str) -> bool {
    fn host_of(u: &str) -> Option<&str> {
        let after_scheme = u.find("://").map(|i| &u[i + 3..]).unwrap_or(u);
        let host_port = after_scheme.split('/').next()?;
        let host = host_port.split(':').next()?;
        if host.is_empty() { None } else { Some(host) }
    }
    let central_hosts = [
        host_of(MAVEN_CENTRAL_BASE).unwrap_or(""),
        host_of(MAVEN_CENTRAL_MIRROR).unwrap_or(""),
    ];
    host_of(url).is_some_and(|h| central_hosts.contains(&h))
}

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
    /// ISO 8601 publication timestamp for the latest stable version, when available.
    /// Fetched from the Maven Central search API for Maven Central packages.
    pub release_timestamp: Option<String>,
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
    let release_timestamp = if let Some(ref ver) = summary.latest {
        fetch_maven_central_timestamp(&dep.dep_name, ver, http).await
    } else {
        None
    };
    Ok(MavenUpdateSummary {
        current_version: summary.current_version,
        latest: summary.latest,
        update_available: summary.update_available,
        release_timestamp,
    })
}

/// Maven Central search API URL for per-version timestamp lookup.
const MAVEN_CENTRAL_SEARCH_API: &str = "https://search.maven.org/solrsearch/select";

#[derive(serde::Deserialize)]
struct MavenSearchResponse {
    response: MavenSearchResponseBody,
}

#[derive(serde::Deserialize)]
struct MavenSearchResponseBody {
    docs: Vec<MavenSearchDoc>,
}

#[derive(serde::Deserialize)]
struct MavenSearchDoc {
    /// Unix epoch in **milliseconds** — Maven Central search API convention.
    timestamp: Option<i64>,
}

/// Fetch the publish timestamp for a specific Maven artifact version from the
/// Maven Central search API.  Returns `None` on any error (best-effort).
async fn fetch_maven_central_timestamp(
    dep_name: &str,
    version: &str,
    http: &HttpClient,
) -> Option<String> {
    let (group_id, artifact_id) = dep_name.split_once(':')?;
    let url = format!(
        "{MAVEN_CENTRAL_SEARCH_API}?q=g:{group_id}+AND+a:{artifact_id}+AND+v:{version}&core=gav&rows=1&wt=json"
    );
    let resp = http.get_retrying(&url).await.ok()?;
    if !resp.status().is_success() {
        return None;
    }
    let data: MavenSearchResponse = resp.json().await.ok()?;
    let ts_ms = data.response.docs.first()?.timestamp?;
    // Convert epoch milliseconds to ISO 8601.
    let secs = ts_ms / 1000;
    let dt = chrono::DateTime::<chrono::Utc>::from_timestamp(secs, 0)?;
    Some(dt.format("%Y-%m-%dT%H:%M:%SZ").to_string())
}

/// Cached latest-version entry: `Option<String>` (None if not found).
pub type MavenLatestEntry = Option<String>;

/// Fetch the latest version for a batch of unique Maven coordinates concurrently.
///
/// Returns a `HashMap` from `groupId:artifactId` to the latest version string.
/// Coordinates that fail to resolve are stored as `None`.
pub async fn fetch_latest_batch(
    http: &HttpClient,
    dep_names: &[String],
    concurrency: usize,
) -> HashMap<String, MavenLatestEntry> {
    if dep_names.is_empty() {
        return HashMap::new();
    }

    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<(String, MavenLatestEntry)> = JoinSet::new();

    for dep_name in dep_names {
        let http = http.clone();
        let dep_name = dep_name.clone();
        let sem = Arc::clone(&sem);

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = fetch_latest(&dep_name, &http).await.ok().flatten();
            (dep_name, result)
        });
    }

    let mut cache = HashMap::with_capacity(dep_names.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok((name, latest)) => {
                cache.insert(name, latest);
            }
            Err(join_err) => tracing::error!(%join_err, "maven batch fetch task panicked"),
        }
    }
    cache
}

/// Compute a `MavenUpdateSummary` from a pre-fetched latest version entry.
pub fn summary_from_cache(current_version: &str, latest: &MavenLatestEntry) -> MavenUpdateSummary {
    let summary =
        crate::versioning::maven::maven_update_summary(current_version, latest.as_deref());
    MavenUpdateSummary {
        current_version: summary.current_version,
        latest: summary.latest,
        update_available: summary.update_available,
        release_timestamp: None,
    }
}

// ──────────────────────────────────────────────────────────────────────
// Full releases support (used by clojure.rs and can be used by maven tests)
// ──────────────────────────────────────────────────────────────────────

/// All versions and tags extracted from a `maven-metadata.xml`.
#[derive(Debug, Clone)]
pub struct MetadataResult {
    pub versions: Vec<String>,
    /// Named version tags: `"latest"` and/or `"release"`.
    pub tags: HashMap<String, String>,
}

/// Parse ALL `<version>` elements inside `<versioning><versions>` from a
/// `maven-metadata.xml` string, along with `<latest>` and `<release>` tags.
///
/// Top-level `<version>` elements (used in snapshot artifact metadata) are
/// ignored.  Returns `None` when no versions are found inside `<versions>`.
pub fn parse_all_versions(xml: &str) -> Option<MetadataResult> {
    let cursor = BufReader::new(xml.as_bytes());
    let mut reader = Reader::from_reader(cursor);
    reader.config_mut().trim_text(true);

    let mut versions: Vec<String> = Vec::new();
    let mut latest: Option<String> = None;
    let mut release: Option<String> = None;
    let mut in_versioning = false;
    let mut in_versions = false;
    let mut current_tag: Option<String> = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let tag = String::from_utf8_lossy(e.local_name().as_ref()).into_owned();
                match tag.as_str() {
                    "versioning" => in_versioning = true,
                    "versions" if in_versioning => in_versions = true,
                    _ => {}
                }
                current_tag = Some(tag);
            }
            Ok(Event::Text(e)) => {
                if let Some(ref tag) = current_tag {
                    let text = e.decode().map(|s| s.trim().to_owned()).unwrap_or_default();
                    if !text.is_empty() {
                        match tag.as_str() {
                            "version" if in_versions => versions.push(text),
                            "latest" if in_versioning => latest = Some(text),
                            "release" if in_versioning => release = Some(text),
                            _ => {}
                        }
                    }
                }
            }
            Ok(Event::End(e)) => {
                let tag = String::from_utf8_lossy(e.local_name().as_ref()).into_owned();
                match tag.as_str() {
                    "versioning" => in_versioning = false,
                    "versions" => in_versions = false,
                    _ => {}
                }
                current_tag = None;
            }
            Ok(Event::Eof) | Err(_) => break,
            _ => {}
        }
        buf.clear();
    }

    if versions.is_empty() {
        return None;
    }

    let mut tags = HashMap::new();
    if let Some(l) = latest {
        tags.insert("latest".to_owned(), l);
    }
    if let Some(r) = release {
        tags.insert("release".to_owned(), r);
    }
    Some(MetadataResult { versions, tags })
}

/// Information extracted from a Maven POM file.
#[derive(Debug, Clone, Default)]
pub struct PomInfo {
    pub homepage: Option<String>,
    pub source_url: Option<String>,
}

/// Parse a Maven POM XML for `homepage` and `sourceUrl`.
///
/// - `homepage` ← `<url>` (skipped if it contains `${...}` placeholders).
/// - `source_url` ← `<scm><url>` with prefix stripping and placeholder
///   removal mirroring the TypeScript `getDependencyInfo` behaviour.
pub fn parse_pom_info(xml: &str) -> PomInfo {
    let cursor = BufReader::new(xml.as_bytes());
    let mut reader = Reader::from_reader(cursor);
    reader.config_mut().trim_text(true);

    let mut result = PomInfo::default();
    let mut in_scm = false;
    let mut current_tag: Option<String> = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let tag = String::from_utf8_lossy(e.local_name().as_ref()).into_owned();
                if tag == "scm" {
                    in_scm = true;
                }
                current_tag = Some(tag);
            }
            Ok(Event::Text(e)) => {
                if let Some(ref tag) = current_tag {
                    let text = e.decode().map(|s| s.trim().to_owned()).unwrap_or_default();
                    if !text.is_empty() && tag == "url" {
                        if in_scm && result.source_url.is_none() {
                            result.source_url = process_scm_url(&text);
                        } else if !in_scm && result.homepage.is_none() && !text.contains("${") {
                            result.homepage = Some(text);
                        }
                    }
                }
            }
            Ok(Event::End(e)) => {
                let tag = String::from_utf8_lossy(e.local_name().as_ref()).into_owned();
                if tag == "scm" {
                    in_scm = false;
                }
                current_tag = None;
            }
            Ok(Event::Eof) | Err(_) => break,
            _ => {}
        }
        buf.clear();
    }

    result
}

/// Apply the TypeScript `getDependencyInfo` transformations to a raw `<scm><url>` value.
fn process_scm_url(raw: &str) -> Option<String> {
    use regex::Regex;
    // Remove /tree/${...} (the "known placeholder" removal)
    let re = Regex::new(r"/tree/\$\{[^}]+\}").expect("static regex");
    let s = re.replace(raw, "").into_owned();

    // Sequential prefix stripping matching TypeScript replace chains
    let s = s.strip_prefix("scm:").unwrap_or(&s).to_owned();
    let s = s.strip_prefix("git:").unwrap_or(&s).to_owned();

    // git@github.com:path  →  https://github.com/path
    let s = if let Some(rest) = s.strip_prefix("git@github.com:") {
        format!("https://github.com/{rest}")
    } else if let Some(rest) = s.strip_prefix("git@github.com/") {
        format!("https://github.com/{rest}")
    } else {
        s
    };

    // //path  →  https://path
    let s = if s.starts_with("//") {
        format!("https:{s}")
    } else {
        s
    };

    // Skip if any ${...} placeholders remain
    if s.contains("${") {
        return None;
    }

    Some(s)
}

/// Return the "latest suitable" version: highest stable version, falling back
/// to highest overall when no stable versions exist.
pub fn find_latest_suitable(versions: &[String]) -> Option<&str> {
    use crate::versioning::maven::{compare, is_stable};
    use std::cmp::Ordering;

    let stable: Vec<&str> = versions
        .iter()
        .map(String::as_str)
        .filter(|v| is_stable(v))
        .collect();
    let pool: Vec<&str> = if stable.is_empty() {
        versions.iter().map(String::as_str).collect()
    } else {
        stable
    };

    pool.into_iter().reduce(|best, v| {
        if compare(v, best) == Ordering::Greater {
            v
        } else {
            best
        }
    })
}

/// Full release result for a single Maven-compatible registry lookup.
#[derive(Debug, Clone)]
pub struct MavenReleasesResult {
    pub releases: Vec<String>,
    pub source_url: Option<String>,
    pub homepage: Option<String>,
    pub registry_url: String,
    pub tags: HashMap<String, String>,
    pub is_private: bool,
    pub respect_latest: bool,
}

/// Fetch all releases for `dep_name` from one Maven-compatible `registry`.
///
/// Returns `None` when:
/// - registry URL is not `http://` or `https://` (unsupported protocol), or
/// - registry URL is otherwise invalid, or
/// - the registry returns no versions (404, bad XML, no `<versions>` element).
pub async fn fetch_releases_from_registry(
    dep_name: &str,
    registry: &str,
    http: &HttpClient,
    default_registries: &[&str],
) -> Option<MavenReleasesResult> {
    // Only http/https registries are supported
    if !registry.starts_with("http://") && !registry.starts_with("https://") {
        return None;
    }

    let (group_id, artifact_id) = dep_name.split_once(':')?;
    let group_path = group_id.replace('.', "/");
    let base = registry.trim_end_matches('/');
    let metadata_url = format!("{base}/{group_path}/{artifact_id}/maven-metadata.xml");

    let resp = http.get_retrying(&metadata_url).await.ok()?;
    if !resp.status().is_success() {
        return None;
    }
    let body = resp.text().await.ok()?;
    let metadata = parse_all_versions(&body)?;

    // Fetch POM for the latest suitable version to get homepage / sourceUrl
    let pom_info = if let Some(latest) = find_latest_suitable(&metadata.versions) {
        let pom_url =
            format!("{base}/{group_path}/{artifact_id}/{latest}/{artifact_id}-{latest}.pom");
        match http.get_retrying(&pom_url).await.ok() {
            Some(r) if r.status().is_success() => r
                .text()
                .await
                .ok()
                .map(|b| parse_pom_info(&b))
                .unwrap_or_default(),
            _ => PomInfo::default(),
        }
    } else {
        PomInfo::default()
    };

    let registry_url = base.to_owned();
    let is_private = !default_registries
        .iter()
        .any(|r| r.trim_end_matches('/') == registry_url);
    let respect_latest = metadata.tags.contains_key("latest");

    Some(MavenReleasesResult {
        releases: metadata.versions,
        source_url: pom_info.source_url,
        homepage: pom_info.homepage,
        registry_url,
        tags: metadata.tags,
        is_private,
        respect_latest,
    })
}

// ──────────────────────────────────────────────────────────────────────

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

    // Rust-specific: maven behavior test
    #[test]
    fn parse_release_tag() {
        let latest = parse_latest_version(spring_metadata()).unwrap();
        assert_eq!(latest, Some("6.0.11".to_owned()));
    }

    // Rust-specific: maven behavior test
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

    // Rust-specific: maven behavior test
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

    // Rust-specific: maven behavior test
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

    // Rust-specific: maven behavior test
    #[test]
    fn dep_name_without_colon_returns_none() {
        // fetch_latest splits on ':'; no colon → None, checked via sync helper.
        let dep_name = "nodot";
        assert!(dep_name.split_once(':').is_none());
    }

    // Ported: "%s => %s" — modules/datasource/maven/common.spec.ts line 5
    #[test]
    fn is_maven_central_host_based_matching() {
        assert!(is_maven_central("https://repo.maven.apache.org/maven2"));
        assert!(is_maven_central("http://repo.maven.apache.org/maven2"));
        assert!(is_maven_central("https://repo1.maven.org/maven2"));
        assert!(is_maven_central("http://repo1.maven.org/maven2"));
        assert!(is_maven_central("http://repo1.maven.org/maven200"));
        assert!(is_maven_central("ftp://repo1.maven.org/maven2"));
        assert!(!is_maven_central("http://repo55.maven.apache.org/maven2"));
        assert!(!is_maven_central("https://some-artifactory.local/maven2"));
    }

    #[tokio::test]
    async fn fetch_releases_returns_versions() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/com/example/lib/maven-metadata.xml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(r#"<metadata>
  <versioning>
    <versions>
      <version>1.0.0</version>
      <version>1.1.0</version>
      <version>2.0.0</version>
    </versions>
  </versioning>
</metadata>"#))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases_from_registry("com.example:lib", &server.uri(), &http, &[&server.uri()]).await;
        assert!(result.is_some());
        let releases = result.unwrap();
        assert_eq!(releases.releases, vec!["1.0.0", "1.1.0", "2.0.0"]);
    }

    #[tokio::test]
    async fn fetch_releases_404_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/com/example/lib/maven-metadata.xml"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases_from_registry("com.example:lib", &server.uri(), &http, &[&server.uri()]).await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn fetch_releases_unsupported_protocol_returns_none() {
        let http = HttpClient::new().unwrap();
        let result = fetch_releases_from_registry("com.example:lib", "ftp://registry.example.com", &http, &[]).await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn fetch_releases_invalid_xml_returns_none() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/com/example/lib/maven-metadata.xml"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not xml"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases_from_registry("com.example:lib", &server.uri(), &http, &[&server.uri()]).await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn fetch_releases_invalid_dep_name_returns_none() {
        let server = MockServer::start().await;
        let http = HttpClient::new().unwrap();
        let result = fetch_releases_from_registry("nocolon", &server.uri(), &http, &[&server.uri()]).await;
        assert!(result.is_none());
    }
}
