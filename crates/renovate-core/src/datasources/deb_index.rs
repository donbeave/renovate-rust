//! Debian package index datasource.
//!
//! Main deb package datasource that fetches available versions from
//! a Debian-style package index (Packages file).
//!
//! Renovate reference: `lib/modules/datasource/deb/index.ts`
//! API: `GET {registry_url}/dists/{dist}/main/binary-{arch}/Packages`

use std::collections::HashMap;

use thiserror::Error;

use crate::datasources::{Release, ReleaseResult};
use crate::http::HttpClient;

pub const DATASOURCE_ID: &str = "deb";

#[derive(Debug, Error)]
pub enum DebIndexError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("invalid registry URL: {0}")]
    InvalidUrl(String),
    #[error("package '{0}' not found in index")]
    NotFound(String),
}

/// Parsed package entry from a Debian Packages file.
#[derive(Debug, Clone, Default)]
pub struct PackageEntry {
    pub package: String,
    pub version: String,
    pub architecture: Option<String>,
    pub description: Option<String>,
    pub filename: Option<String>,
    pub size: Option<String>,
}

/// Parse a Debian Packages file into a map of package name -> list of entries.
///
/// The Packages file format uses RFC-2822-style headers (like email headers):
/// each paragraph describes one package, separated by blank lines.
pub fn parse_packages_file(content: &str) -> HashMap<String, Vec<PackageEntry>> {
    let mut packages: HashMap<String, Vec<PackageEntry>> = HashMap::new();
    let mut current = PackageEntry::default();

    for line in content.lines() {
        if line.is_empty() {
            if !current.package.is_empty() && !current.version.is_empty() {
                let name = current.package.clone();
                packages.entry(name).or_default().push(current);
            }
            current = PackageEntry::default();
            continue;
        }

        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();
            match key {
                "Package" => current.package = value.to_owned(),
                "Version" => current.version = value.to_owned(),
                "Architecture" => current.architecture = Some(value.to_owned()),
                "Description" => current.description = Some(value.to_owned()),
                "Filename" => current.filename = Some(value.to_owned()),
                "Size" => current.size = Some(value.to_owned()),
                _ => {}
            }
        }
    }

    if !current.package.is_empty() && !current.version.is_empty() {
        let name = current.package.clone();
        packages.entry(name).or_default().push(current);
    }

    packages
}

/// Fetch versions for a Debian package from a registry URL.
///
/// The `registry_url` should include query parameters: `suite`, `components`, and `binaryArch`.
/// For example: `https://deb.debian.org/debian?suite=stable&components=main&binaryArch=amd64`
///
/// Returns a `ReleaseResult` with all versions found for the package.
pub async fn fetch_versions(
    http: &HttpClient,
    package: &str,
    registry_url: &str,
) -> Result<ReleaseResult, DebIndexError> {
    let component_urls =
        crate::datasources::deb::construct_component_urls(registry_url).map_err(DebIndexError::InvalidUrl)?;

    let mut all_releases = Vec::new();

    for base_url in &component_urls {
        let packages_url = format!("{base_url}/Packages");

        let resp = http.get_retrying(&packages_url).await?;
        if !resp.status().is_success() {
            continue;
        }

        let body = resp.text().await.map_err(crate::http::HttpError::Request)?;

        let packages = parse_packages_file(&body);
        if let Some(entries) = packages.get(package) {
            for entry in entries {
                all_releases.push(Release {
                    version: entry.version.clone(),
                    ..Default::default()
                });
            }
        }
    }

    if all_releases.is_empty() {
        return Err(DebIndexError::NotFound(package.to_owned()));
    }

    Ok(ReleaseResult {
        releases: all_releases,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    const PACKAGES_CONTENT: &str = "\
Package: nginx
Version: 1.18.0-6ubuntu1
Architecture: amd64
Filename: pool/main/n/nginx/nginx_1.18.0-6ubuntu1_amd64.deb
Size: 123456

Package: nginx
Version: 1.22.0-1ubuntu1
Architecture: amd64
Filename: pool/main/n/nginx/nginx_1.22.0-1ubuntu1_amd64.deb
Size: 234567

Package: curl
Version: 7.81.0-1ubuntu1
Architecture: amd64
Filename: pool/main/c/curl/curl_7.81.0-1ubuntu1_amd64.deb
Size: 345678
";

    #[test]
    fn parse_packages_file_extracts_packages() {
        let packages = parse_packages_file(PACKAGES_CONTENT);
        assert!(packages.contains_key("nginx"));
        assert!(packages.contains_key("curl"));

        let nginx = &packages["nginx"];
        assert_eq!(nginx.len(), 2);
        assert_eq!(nginx[0].version, "1.18.0-6ubuntu1");
        assert_eq!(nginx[1].version, "1.22.0-1ubuntu1");
        assert_eq!(
            nginx[0].filename.as_deref(),
            Some("pool/main/n/nginx/nginx_1.18.0-6ubuntu1_amd64.deb")
        );
    }

    #[test]
    fn parse_packages_file_empty_input() {
        let packages = parse_packages_file("");
        assert!(packages.is_empty());
    }

    #[test]
    fn parse_packages_file_single_entry() {
        let content = "Package: vim\nVersion: 9.0.1000\nArchitecture: amd64\n";
        let packages = parse_packages_file(content);
        assert_eq!(packages.len(), 1);
        let vim = &packages["vim"];
        assert_eq!(vim.len(), 1);
        assert_eq!(vim[0].version, "9.0.1000");
    }

    #[test]
    fn parse_packages_file_skips_incomplete_entries() {
        let content = "Package: incomplete\n\nPackage: complete\nVersion: 1.0\n";
        let packages = parse_packages_file(content);
        assert!(!packages.contains_key("incomplete"));
        assert!(packages.contains_key("complete"));
    }

    #[tokio::test]
    async fn fetch_versions_returns_releases() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/dists/stable/main/binary-amd64/Packages"))
            .respond_with(ResponseTemplate::new(200).set_body_string(PACKAGES_CONTENT))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let registry_url = format!(
            "{}/?suite=stable&components=main&binaryArch=amd64",
            server.uri()
        );
        let result = fetch_versions(&http, "nginx", &registry_url)
            .await
            .unwrap();

        assert_eq!(result.releases.len(), 2);
        assert_eq!(result.releases[0].version, "1.18.0-6ubuntu1");
        assert_eq!(result.releases[1].version, "1.22.0-1ubuntu1");
    }

    #[tokio::test]
    async fn fetch_versions_not_found_returns_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/dists/stable/main/binary-amd64/Packages"))
            .respond_with(ResponseTemplate::new(200).set_body_string(PACKAGES_CONTENT))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let registry_url = format!(
            "{}/?suite=stable&components=main&binaryArch=amd64",
            server.uri()
        );
        let result = fetch_versions(&http, "nonexistent", &registry_url).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn fetch_versions_invalid_url_returns_error() {
        let http = HttpClient::new().unwrap();
        let result = fetch_versions(&http, "nginx", "not-a-url").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn fetch_versions_404_continues_to_next_component() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/dists/stable/main/binary-amd64/Packages"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let registry_url = format!(
            "{}/?suite=stable&components=main&binaryArch=amd64",
            server.uri()
        );
        let result = fetch_versions(&http, "nginx", &registry_url).await;
        assert!(result.is_err());
    }

    #[test]
    fn datasource_id_is_correct() {
        assert_eq!(DATASOURCE_ID, "deb");
    }
}
