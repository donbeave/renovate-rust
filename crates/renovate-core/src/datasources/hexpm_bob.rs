//! Hex.pm Bob (Elixir/Erlang) builds datasource.
//!
//! Fetches pre-built releases from the hex.pm build service.
//!
//! Renovate reference: `lib/modules/datasource/hexpm-bob/index.ts`
//! API: `GET <registryUrl>/builds/<packageName>/builds.txt`

use reqwest::StatusCode;
use thiserror::Error;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY_URL: &str = "https://builds.hex.pm";
pub const DATASOURCE_ID: &str = "hexpm-bob";

/// Errors from the hexpm-bob datasource.
#[derive(Debug, Error)]
pub enum HexpmBobError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

#[derive(Debug, Clone, PartialEq)]
enum PackageType {
    Elixir,
    Erlang,
}

fn get_package_type(package_name: &str) -> Option<PackageType> {
    if package_name == "elixir" {
        return Some(PackageType::Elixir);
    }
    // otp/<distro>-<version>, e.g. "otp/ubuntu-20.04"
    let re = regex::Regex::new(r"^otp/\w+-\d+\.\d+$").unwrap();
    if re.is_match(package_name) {
        return Some(PackageType::Erlang);
    }
    None
}

fn clean_version(version: &str, pkg_type: &PackageType) -> String {
    match pkg_type {
        PackageType::Elixir => version.trim_start_matches('v').to_string(),
        PackageType::Erlang => version.trim_start_matches("OTP-").to_string(),
    }
}

fn is_stable(version: &str, pkg_type: &PackageType) -> bool {
    match pkg_type {
        PackageType::Elixir => {
            // matches /^v\d+\.\d+\.\d+($|-otp)/
            let re = regex::Regex::new(r"^v\d+\.\d+\.\d+($|-otp)").unwrap();
            re.is_match(version)
        }
        PackageType::Erlang => version.starts_with("OTP-"),
    }
}

fn parse_timestamp(s: &str) -> Option<String> {
    // Convert "2022-09-01T18:24:21Z" → "2022-09-01T18:24:21.000Z"
    let s = s.trim_end_matches('Z');
    if s.is_empty() {
        return None;
    }
    if s.contains('.') {
        // Already has sub-seconds
        let dot_pos = s.find('.')?;
        let prefix = &s[..dot_pos];
        let frac: String = s[dot_pos + 1..]
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect();
        let ms = format!("{:0<3}", &frac[..frac.len().min(3)]);
        return Some(format!("{}.{}Z", prefix, ms));
    }
    Some(format!("{}.000Z", s))
}

/// Details about a package type.
struct PackageDetails {
    homepage: &'static str,
    source_url: &'static str,
}

fn get_package_details(pkg_type: &PackageType) -> PackageDetails {
    match pkg_type {
        PackageType::Elixir => PackageDetails {
            homepage: "https://elixir-lang.org/",
            source_url: "https://github.com/elixir-lang/elixir",
        },
        PackageType::Erlang => PackageDetails {
            homepage: "https://www.erlang.org/",
            source_url: "https://github.com/erlang/otp",
        },
    }
}

/// One hexpm-bob release.
#[derive(Debug, Clone, PartialEq)]
pub struct HexpmBobRelease {
    pub version: String,
    pub git_ref: String,
    pub release_timestamp: Option<String>,
    pub is_stable: bool,
}

/// Result of a `fetch_releases` call.
#[derive(Debug, Clone)]
pub struct HexpmBobResult {
    pub registry_url: String,
    pub homepage: &'static str,
    pub source_url: &'static str,
    pub releases: Vec<HexpmBobRelease>,
}

/// Fetch hexpm-bob releases.
///
/// Returns `None` for invalid package names, 404, or empty responses.
/// Returns `Err` for 5xx and network errors.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<HexpmBobResult>, HexpmBobError> {
    let pkg_type = match get_package_type(package_name) {
        Some(t) => t,
        None => return Ok(None),
    };

    let url = format!(
        "{}/builds/{}/builds.txt",
        registry_url.trim_end_matches('/'),
        package_name
    );

    let body = match http.get_raw_with_accept(&url, "text/plain").await {
        Ok(t) => t,
        Err(crate::http::HttpError::Status { status, .. })
            if status == StatusCode::NOT_FOUND =>
        {
            return Ok(None)
        }
        Err(e) => return Err(HexpmBobError::Http(e)),
    };

    let releases: Vec<HexpmBobRelease> = body
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(4, ' ').collect();
            if parts.len() < 3 {
                return None;
            }
            let raw_version = parts[0];
            let git_ref = parts[1].to_string();
            let build_date = parts[2];
            Some(HexpmBobRelease {
                version: clean_version(raw_version, &pkg_type),
                git_ref,
                release_timestamp: parse_timestamp(build_date),
                is_stable: is_stable(raw_version, &pkg_type),
            })
        })
        .collect();

    if releases.is_empty() {
        return Ok(None);
    }

    let details = get_package_details(&pkg_type);
    Ok(Some(HexpmBobResult {
        registry_url: registry_url.trim_end_matches('/').to_string(),
        homepage: details.homepage,
        source_url: details.source_url,
        releases,
    }))
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    // Ported: "throws for error" — hexpm-bob/index.spec.ts line 9
    #[tokio::test]
    async fn throws_for_network_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/builds/elixir/builds.txt"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "elixir", &http).await;
        assert!(result.is_err(), "5xx should propagate as Err");
    }

    // Ported: "returns null for 404" — hexpm-bob/index.spec.ts line 22
    #[tokio::test]
    async fn returns_null_for_404() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/builds/elixir/builds.txt"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "elixir", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null for empty result" — hexpm-bob/index.spec.ts line 35
    #[tokio::test]
    async fn returns_null_for_empty_result() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/builds/elixir/builds.txt"))
            .respond_with(ResponseTemplate::new(200).set_body_string(""))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "elixir", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns empty list for empty 200 OK" — hexpm-bob/index.spec.ts line 48
    #[tokio::test]
    async fn returns_null_for_empty_200_ok() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/builds/elixir/builds.txt"))
            .respond_with(ResponseTemplate::new(200).set_body_string(""))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "elixir", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "throws for 5xx" — hexpm-bob/index.spec.ts line 61
    #[tokio::test]
    async fn throws_for_5xx() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/builds/elixir/builds.txt"))
            .respond_with(ResponseTemplate::new(502))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "elixir", &http).await;
        assert!(result.is_err());
    }

    // Ported: "processes real data" — hexpm-bob/index.spec.ts line 74
    #[tokio::test]
    async fn processes_real_data_elixir() {
        let fixture = include_str!(
            "../../../../../renovate/lib/modules/datasource/hexpm-bob/__fixtures__/elixir/builds.txt"
        );
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/builds/elixir/builds.txt"))
            .respond_with(ResponseTemplate::new(200).set_body_string(fixture))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "elixir", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.homepage, "https://elixir-lang.org/");
        assert_eq!(result.source_url, "https://github.com/elixir-lang/elixir");

        // Verify specific releases from the TypeScript test
        let rc1 = result.releases.iter().find(|r| r.version == "1.14.0-rc.1");
        assert!(rc1.is_some());
        let rc1 = rc1.unwrap();
        assert_eq!(rc1.git_ref, "185eeec5ecbc2a0c8d9b8b97cb2d23108615ffdb");
        assert!(!rc1.is_stable);
        assert_eq!(rc1.release_timestamp.as_deref(), Some("2022-08-15T10:28:05.000Z"));

        let v140 = result.releases.iter().find(|r| r.version == "1.14.0");
        assert!(v140.is_some());
        let v140 = v140.unwrap();
        assert!(v140.is_stable);
        assert_eq!(v140.release_timestamp.as_deref(), Some("2022-09-01T18:24:21.000Z"));

        let v141 = result.releases.iter().find(|r| r.version == "1.14.1");
        assert!(v141.is_some());
        assert!(v141.unwrap().is_stable);

        let v141_otp25 = result.releases.iter().find(|r| r.version == "1.14.1-otp-25");
        assert!(v141_otp25.is_some());
        assert!(v141_otp25.unwrap().is_stable);
    }

    // Ported: "processes real data (erlang / ubuntu 20.04)" — hexpm-bob/index.spec.ts line 122
    #[tokio::test]
    async fn processes_real_data_erlang_ubuntu() {
        let fixture = include_str!(
            "../../../../../renovate/lib/modules/datasource/hexpm-bob/__fixtures__/otp/ubuntu-20.04/builds.txt"
        );
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/builds/otp/ubuntu-20.04/builds.txt"))
            .respond_with(ResponseTemplate::new(200).set_body_string(fixture))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "otp/ubuntu-20.04", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.homepage, "https://www.erlang.org/");
        assert_eq!(result.source_url, "https://github.com/erlang/otp");

        // OTP-prefixed builds are stable
        let v25_1 = result.releases.iter().find(|r| r.version == "25.1");
        assert!(v25_1.is_some());
        let v25_1 = v25_1.unwrap();
        assert!(v25_1.is_stable);
        assert_eq!(v25_1.git_ref, "6efb5e31df6bc512ed6c466584ef15b846dcecab");
        assert_eq!(v25_1.release_timestamp.as_deref(), Some("2022-09-21T09:54:48.000Z"));

        let v25_1_2 = result.releases.iter().find(|r| r.version == "25.1.2");
        assert!(v25_1_2.is_some());
        assert!(v25_1_2.unwrap().is_stable);

        // non-OTP builds are unstable
        let maint = result.releases.iter().find(|r| r.version == "maint");
        assert!(maint.is_some());
        assert!(!maint.unwrap().is_stable);
    }

    // Ported: "can override registry url" — hexpm-bob/index.spec.ts line 155
    #[tokio::test]
    async fn can_override_registry_url() {
        let fixture = include_str!(
            "../../../../../renovate/lib/modules/datasource/hexpm-bob/__fixtures__/otp/ubuntu-20.04/builds.txt"
        );
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/builds/otp/ubuntu-20.04/builds.txt"))
            .respond_with(ResponseTemplate::new(200).set_body_string(fixture))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "otp/ubuntu-20.04", &http)
            .await
            .unwrap();
        // Custom registry URL is honored: we get a non-None result from the mock server.
        assert!(result.is_some());
        assert_eq!(result.unwrap().registry_url, server.uri().trim_end_matches('/'));
    }

    // Ported: "returns empty list for invalid package name" — hexpm-bob/index.spec.ts line 172
    #[tokio::test]
    async fn returns_null_for_invalid_package_name() {
        let http = HttpClient::new().unwrap();
        let result = fetch_releases(DEFAULT_REGISTRY_URL, "invalid", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }
}
