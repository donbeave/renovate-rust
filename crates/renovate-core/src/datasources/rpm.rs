//! RPM package datasource.
//!
//! Fetches package versions from RPM repository metadata (repomd XML).
//!
//! Renovate reference: `lib/modules/datasource/rpm/index.ts`
//! The RPM repo metadata structure is:
//! 1. `repodata/repomd.xml` — lists available metadata files
//! 2. `repodata/<sha>-primary.xml.gz` — contains package info
//!
//! This implementation parses the repomd.xml and primary.xml to extract
//! package version information.

use std::io::Read;

use thiserror::Error;

use crate::datasources::{Release, ReleaseResult};
use crate::http::HttpClient;

pub const DATASOURCE_ID: &str = "rpm";

#[derive(Debug, Error)]
pub enum RpmError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("XML parse error: {0}")]
    Xml(String),
    #[error("decompression error: {0}")]
    Decompression(String),
    #[error("package '{0}' not found in repository")]
    NotFound(String),
}

#[derive(Debug, Clone)]
struct RpmPackage {
    name: String,
    version: String,
    release: String,
    arch: String,
    summary: Option<String>,
}

/// Parse a repomd.xml to find the primary.xml location.
///
/// Returns the relative path to the primary.xml file (e.g., `repodata/abc-primary.xml.gz`).
pub fn parse_repomd_xml(content: &str) -> Result<Option<String>, RpmError> {
    let mut reader = quick_xml::Reader::from_str(content);
    reader.config_mut().trim_text(true);

    let mut in_data = false;
    let mut data_type = String::new();
    let mut location_href: Option<String> = None;

    let mut buf = Vec::new();
    loop {
        use quick_xml::events::Event;
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                match e.local_name().as_ref() {
                    b"data" => {
                        for attr in e.attributes().flatten() {
                            if attr.key.local_name().as_ref() == b"type" {
                                data_type = String::from_utf8_lossy(&attr.value).to_string();
                                if data_type == "primary" {
                                    in_data = true;
                                }
                            }
                        }
                    }
                    b"location" if in_data => {
                        for attr in e.attributes().flatten() {
                            if attr.key.local_name().as_ref() == b"href" {
                                location_href =
                                    Some(String::from_utf8_lossy(&attr.value).to_string());
                            }
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) if e.local_name().as_ref() == b"data" => {
                if in_data {
                    break;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(RpmError::Xml(e.to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(location_href)
}

/// Parse primary.xml content to extract package information.
///
/// Returns a list of RPM packages found in the metadata.
pub fn parse_primary_xml(content: &str) -> Result<Vec<RpmPackage>, RpmError> {
    let mut packages = Vec::new();
    let mut reader = quick_xml::Reader::from_str(content);
    reader.config_mut().trim_text(true);

    let mut in_package = false;
    let mut current_name = String::new();
    let mut current_version = String::new();
    let mut current_release = String::new();
    let mut current_arch = String::new();
    let mut current_summary = String::new();
    let mut current_tag = String::new();

    let mut buf = Vec::new();
    loop {
        use quick_xml::events::Event;
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let local = String::from_utf8_lossy(e.local_name().as_ref()).to_string();
                match local.as_str() {
                    "package" => {
                        in_package = true;
                        current_name.clear();
                        current_version.clear();
                        current_release.clear();
                        current_arch.clear();
                        current_summary.clear();
                    }
                    "version" if in_package => {
                        for attr in e.attributes().flatten() {
                            match attr.key.local_name().as_ref() {
                                b"ver" => {
                                    current_version =
                                        String::from_utf8_lossy(&attr.value).to_string()
                                }
                                b"rel" => {
                                    current_release =
                                        String::from_utf8_lossy(&attr.value).to_string()
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {
                        if in_package {
                            current_tag = local;
                        }
                    }
                }
            }
            Ok(Event::Empty(ref e)) => {
                if in_package && e.local_name().as_ref() == b"version" {
                    for attr in e.attributes().flatten() {
                        match attr.key.local_name().as_ref() {
                            b"ver" => {
                                current_version = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            b"rel" => {
                                current_release = String::from_utf8_lossy(&attr.value).to_string()
                            }
                            _ => {}
                        }
                    }
                }
            }
            Ok(Event::Text(ref e)) if in_package => {
                let text = e.decode().map(|s| s.trim().to_owned()).map_err(|e| RpmError::Xml(e.to_string()))?;
                match current_tag.as_str() {
                    "name" => current_name = text.to_string(),
                    "arch" => current_arch = text.to_string(),
                    "summary" => current_summary = text.to_string(),
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let local = String::from_utf8_lossy(e.local_name().as_ref()).to_string();
                if local == "package" && in_package {
                    in_package = false;
                    if !current_name.is_empty() && !current_version.is_empty() {
                        packages.push(RpmPackage {
                            name: current_name.clone(),
                            version: current_version.clone(),
                            release: current_release.clone(),
                            arch: current_arch.clone(),
                            summary: if current_summary.is_empty() {
                                None
                            } else {
                                Some(current_summary.clone())
                            },
                        });
                    }
                }
                current_tag.clear();
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(RpmError::Xml(e.to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(packages)
}

/// Decompress gzip data.
fn decompress_gz(data: &[u8]) -> Result<Vec<u8>, RpmError> {
    let mut decoder = flate2::read::GzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder
        .read_to_end(&mut decompressed)
        .map_err(|e| RpmError::Decompression(e.to_string()))?;
    Ok(decompressed)
}

/// Fetch versions for an RPM package from a repository.
///
/// The `registry_url` should be the base URL of the RPM repository
/// (e.g., `https://mirrors.centos.org/centos/8/BaseOS/x86_64/os/`).
///
/// This function:
/// 1. Fetches `repodata/repomd.xml` to find the primary metadata location
/// 2. Fetches and decompresses the primary metadata
/// 3. Parses package versions from the metadata
pub async fn fetch_versions(
    http: &HttpClient,
    package: &str,
    registry_url: &str,
) -> Result<ReleaseResult, RpmError> {
    let base = registry_url.trim_end_matches('/');
    let repomd_url = format!("{base}/repodata/repomd.xml");

    let resp = http.get_retrying(&repomd_url).await?;
    if !resp.status().is_success() {
        return Err(RpmError::Http(crate::http::HttpError::Status {
            status: resp.status(),
            url: repomd_url,
        }));
    }

    let repomd_body = resp.text().await.map_err(crate::http::HttpError::Request)?;
    let primary_href = parse_repomd_xml(&repomd_body)?;

    let primary_href = primary_href.ok_or_else(|| {
        RpmError::Xml("primary metadata not found in repomd.xml".to_owned())
    })?;

    let primary_url = format!("{base}/{primary_href}");
    let primary_resp = http.get_retrying(&primary_url).await?;
    if !primary_resp.status().is_success() {
        return Err(RpmError::Http(crate::http::HttpError::Status {
            status: primary_resp.status(),
            url: primary_url,
        }));
    }

    let primary_bytes = primary_resp
        .bytes()
        .await
        .map_err(crate::http::HttpError::Request)?;

    let primary_xml = if primary_href.ends_with(".gz") {
        let decompressed = decompress_gz(&primary_bytes)?;
        String::from_utf8(decompressed)
            .map_err(|e| RpmError::Xml(format!("invalid UTF-8 in primary.xml: {e}")))?
    } else {
        String::from_utf8(primary_bytes.to_vec())
            .map_err(|e| RpmError::Xml(format!("invalid UTF-8 in primary.xml: {e}")))?
    };

    let packages = parse_primary_xml(&primary_xml)?;

    let matching: Vec<RpmPackage> = packages
        .into_iter()
        .filter(|p| p.name == package)
        .collect();

    if matching.is_empty() {
        return Err(RpmError::NotFound(package.to_owned()));
    }

    let releases = matching
        .into_iter()
        .map(|p| {
            let full_version = if p.release.is_empty() {
                p.version
            } else {
                format!("{}-{}", p.version, p.release)
            };
            Release {
                version: full_version,
                ..Default::default()
            }
        })
        .collect();

    Ok(ReleaseResult {
        releases,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    const REPOMD_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<repomd xmlns="http://linux.duke.edu/metadata/repo">
  <data type="primary">
    <location href="repodata/abc123-primary.xml.gz"/>
    <checksum type="sha256">abc123</checksum>
  </data>
  <data type="filelists">
    <location href="repodata/def456-filelists.xml.gz"/>
  </data>
</repomd>"#;

    const PRIMARY_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<metadata xmlns="http://linux.duke.edu/metadata/common">
  <package type="rpm">
    <name>nginx</name>
    <arch>x86_64</arch>
    <version ver="1.18.0" rel="2.el8"/>
    <summary>High performance web server</summary>
  </package>
  <package type="rpm">
    <name>nginx</name>
    <arch>x86_64</arch>
    <version ver="1.22.0" rel="1.el8"/>
    <summary>High performance web server</summary>
  </package>
  <package type="rpm">
    <name>curl</name>
    <arch>x86_64</arch>
    <version ver="7.81.0" rel="1.el8"/>
    <summary>A utility for transferring files using URL syntax</summary>
  </package>
</metadata>"#;

    #[test]
    fn parse_repomd_finds_primary_location() {
        let result = parse_repomd_xml(REPOMD_XML).unwrap();
        assert_eq!(
            result.as_deref(),
            Some("repodata/abc123-primary.xml.gz")
        );
    }

    #[test]
    fn parse_repomd_empty_returns_none() {
        let result = parse_repomd_xml("").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn parse_repomd_no_primary_returns_none() {
        let xml = r#"<?xml version="1.0"?>
<repomd>
  <data type="filelists">
    <location href="repodata/filelists.xml.gz"/>
  </data>
</repomd>"#;
        let result = parse_repomd_xml(xml).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn parse_primary_xml_extracts_packages() {
        let packages = parse_primary_xml(PRIMARY_XML).unwrap();
        assert_eq!(packages.len(), 3);

        let nginx_packages: Vec<&RpmPackage> =
            packages.iter().filter(|p| p.name == "nginx").collect();
        assert_eq!(nginx_packages.len(), 2);
        assert_eq!(nginx_packages[0].version, "1.18.0");
        assert_eq!(nginx_packages[0].release, "2.el8");
        assert_eq!(nginx_packages[1].version, "1.22.0");
        assert_eq!(nginx_packages[1].release, "1.el8");

        let curl = packages.iter().find(|p| p.name == "curl").unwrap();
        assert_eq!(curl.version, "7.81.0");
        assert_eq!(curl.release, "1.el8");
        assert_eq!(
            curl.summary.as_deref(),
            Some("A utility for transferring files using URL syntax")
        );
    }

    #[test]
    fn parse_primary_xml_empty_returns_empty() {
        let packages = parse_primary_xml("").unwrap();
        assert!(packages.is_empty());
    }

    #[test]
    fn parse_primary_xml_no_packages_element() {
        let xml = r#"<?xml version="1.0"?>
<metadata></metadata>"#;
        let packages = parse_primary_xml(xml).unwrap();
        assert!(packages.is_empty());
    }

    #[tokio::test]
    async fn fetch_versions_returns_releases() {
        let server = MockServer::start().await;

        let primary_gz = {
            use std::io::Write;
            let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
            encoder.write_all(PRIMARY_XML.as_bytes()).unwrap();
            encoder.finish().unwrap()
        };

        Mock::given(method("GET"))
            .and(path("/repodata/repomd.xml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(REPOMD_XML))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/repodata/abc123-primary.xml.gz"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(primary_gz, "application/gzip"),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_versions(&http, "nginx", &server.uri())
            .await
            .unwrap();

        assert_eq!(result.releases.len(), 2);
        assert_eq!(result.releases[0].version, "1.18.0-2.el8");
        assert_eq!(result.releases[1].version, "1.22.0-1.el8");
    }

    #[tokio::test]
    async fn fetch_versions_not_found_returns_error() {
        let server = MockServer::start().await;

        let primary_gz = {
            use std::io::Write;
            let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
            encoder.write_all(PRIMARY_XML.as_bytes()).unwrap();
            encoder.finish().unwrap()
        };

        Mock::given(method("GET"))
            .and(path("/repodata/repomd.xml"))
            .respond_with(ResponseTemplate::new(200).set_body_string(REPOMD_XML))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/repodata/abc123-primary.xml.gz"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(primary_gz, "application/gzip"),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_versions(&http, "nonexistent", &server.uri()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn fetch_versions_repomd_404_returns_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/repodata/repomd.xml"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_versions(&http, "nginx", &server.uri()).await;
        assert!(result.is_err());
    }

    #[test]
    fn decompress_gz_roundtrip() {
        use std::io::Write;
        let original = b"hello world";
        let mut encoder =
            flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
        encoder.write_all(original).unwrap();
        let compressed = encoder.finish().unwrap();

        let decompressed = decompress_gz(&compressed).unwrap();
        assert_eq!(decompressed, original);
    }

    #[test]
    fn datasource_id_is_correct() {
        assert_eq!(DATASOURCE_ID, "rpm");
    }
}
