//! RPM repomd parsing utilities.
//!
//! Parses repomd.xml and primary.xml for RPM repository metadata.
//!
//! Renovate reference: `lib/modules/datasource/rpm/repomd.ts`

use thiserror::Error;

use crate::http::HttpClient;

#[derive(Debug, Error)]
pub enum RepomdError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("XML parse error: {0}")]
    Xml(String),
    #[error("no primary data found in repomd.xml")]
    NoPrimaryData,
}

pub const REPOMD_XML_FILE_NAME: &str = "repodata/repomd.xml";

pub fn parse_repomd(content: &str) -> Result<Option<String>, RepomdError> {
    let mut reader = quick_xml::Reader::from_str(content);
    reader.config_mut().trim_text(true);

    let mut in_data = false;
    #[allow(unused_assignments)]
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
            Ok(Event::End(ref e)) if e.local_name().as_ref() == b"data" && in_data => {
                break;
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(RepomdError::Xml(e.to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(location_href)
}

pub fn parse_primary_xml(content: &str) -> Result<Vec<RpmPackageEntry>, RepomdError> {
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
            Ok(Event::Empty(ref e))
                if in_package && e.local_name().as_ref() == b"version" =>
            {
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
            Ok(Event::Text(ref e)) if in_package => {
                let text = e
                    .decode()
                    .map(|s| s.trim().to_owned())
                    .map_err(|e| RepomdError::Xml(e.to_string()))?;
                match current_tag.as_str() {
                    "name" => current_name = text.clone(),
                    "arch" => current_arch = text.clone(),
                    "summary" => current_summary = text.clone(),
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let local = String::from_utf8_lossy(e.local_name().as_ref()).to_string();
                if local == "package" && in_package {
                    in_package = false;
                    if !current_name.is_empty() && !current_version.is_empty() {
                        packages.push(RpmPackageEntry {
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
            Err(e) => return Err(RepomdError::Xml(e.to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(packages)
}

#[derive(Debug, Clone)]
pub struct RpmPackageEntry {
    pub name: String,
    pub version: String,
    pub release: String,
    pub arch: String,
    pub summary: Option<String>,
}

pub async fn fetch_primary_gzip_url(
    http: &HttpClient,
    registry_url: &str,
) -> Result<String, RepomdError> {
    let base = registry_url.trim_end_matches('/');
    let repomd_url = format!("{base}/{REPOMD_XML_FILE_NAME}");

    let resp = http.get_retrying(&repomd_url).await?;
    if !resp.status().is_success() {
        return Err(RepomdError::Http(crate::http::HttpError::Status {
            status: resp.status(),
            url: repomd_url.clone(),
        }));
    }

    let body = resp.text().await.map_err(crate::http::HttpError::Request)?;
    let trimmed = body.trim_start();

    if !(trimmed.starts_with("<?xml") || trimmed.starts_with("<repomd")) {
        return Err(RepomdError::Xml(format!(
            "{repomd_url} is not in XML format"
        )));
    }

    let primary_href =
        parse_repomd(trimmed)?.ok_or(RepomdError::NoPrimaryData)?;

    let registry_url_without_repodata =
        registry_url.trim_end_matches('/').trim_end_matches("repodata/");
    let full_url = format!(
        "{}/{}",
        registry_url_without_repodata.trim_end_matches('/'),
        primary_href.trim_start_matches('/')
    );

    Ok(full_url)
}

#[cfg(test)]
mod tests {
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
</metadata>"#;

    #[test]
    fn parse_repomd_finds_primary() {
        let result = parse_repomd(REPOMD_XML).unwrap();
        assert_eq!(
            result.as_deref(),
            Some("repodata/abc123-primary.xml.gz")
        );
    }

    #[test]
    fn parse_repomd_empty_returns_none() {
        let result = parse_repomd("").unwrap();
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
        let result = parse_repomd(xml).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn parse_primary_xml_extracts_packages() {
        let packages = parse_primary_xml(PRIMARY_XML).unwrap();
        assert_eq!(packages.len(), 2);
        assert_eq!(packages[0].name, "nginx");
        assert_eq!(packages[0].version, "1.18.0");
        assert_eq!(packages[0].release, "2.el8");
        assert_eq!(packages[1].version, "1.22.0");
    }

    #[test]
    fn parse_primary_xml_empty_returns_empty() {
        let packages = parse_primary_xml("").unwrap();
        assert!(packages.is_empty());
    }

    #[test]
    fn rpm_package_entry_fields() {
        let entry = RpmPackageEntry {
            name: "curl".into(),
            version: "7.81.0".into(),
            release: "1.el8".into(),
            arch: "x86_64".into(),
            summary: Some("URL transfer tool".into()),
        };
        assert_eq!(entry.name, "curl");
        assert_eq!(entry.version, "7.81.0");
    }
}
