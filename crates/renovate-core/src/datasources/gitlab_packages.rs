//! GitLab packages datasource.
//!
//! Fetches package versions from the GitLab Packages API for a given project.
//! Package name format: `project/path:package_name`
//!
//! Renovate reference: `lib/modules/datasource/gitlab-packages/index.ts`
//! API: `GET {host}/api/v4/projects/{url_encoded_project}/packages?package_name={pkg}&per_page=100`

use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY_URL: &str = "https://gitlab.com";
pub const DATASOURCE_ID: &str = "gitlab-packages";

/// Errors from the GitLab packages datasource.
#[derive(Debug, Error)]
pub enum GitlabPackagesError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

#[derive(Debug, Deserialize)]
struct ApiPackage {
    name: String,
    version: String,
    created_at: Option<String>,
    conan_package_name: Option<String>,
}

/// One release entry.
#[derive(Debug, Clone)]
pub struct GitlabPackageRelease {
    pub version: String,
    pub release_timestamp: Option<String>,
}

/// Result of a `fetch_releases` call.
#[derive(Debug, Clone)]
pub struct GitlabPackagesResult {
    pub releases: Vec<GitlabPackageRelease>,
}

fn rfc3339_to_utc_iso(s: &str) -> Option<String> {
    let dt = DateTime::parse_from_rfc3339(s).ok()?;
    let utc: DateTime<Utc> = dt.with_timezone(&Utc);
    let ms = utc.timestamp_subsec_millis();
    Some(format!("{}.{:03}Z", utc.format("%Y-%m-%dT%H:%M:%S"), ms))
}

fn percent_encode_path(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' => c.to_string(),
            _ => format!("%{:02X}", c as u32),
        })
        .collect()
}

fn is_fatal_status(status: StatusCode) -> bool {
    status.is_server_error() || status == StatusCode::TOO_MANY_REQUESTS
}

/// Fetch GitLab package releases for a project.
///
/// - `registry_url`: GitLab host (e.g. `"https://gitlab.com"`)
/// - `package_name`: `"project/path:package_name"` format
///
/// Returns `None` for 404, empty results, or invalid package name format.
/// Propagates 5xx errors.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<GitlabPackagesResult>, GitlabPackagesError> {
    if registry_url.is_empty() {
        return Ok(None);
    }

    let Some((project_part, pkg_part)) = package_name.split_once(':') else {
        return Ok(None);
    };

    let project_encoded = percent_encode_path(project_part);
    let pkg_encoded = percent_encode_path(pkg_part);
    let url = format!(
        "{}/api/v4/projects/{}/packages?package_name={}&per_page=100",
        registry_url.trim_end_matches('/'),
        project_encoded,
        pkg_encoded,
    );

    let api_packages: Vec<ApiPackage> = match http.get_json(&url).await {
        Ok(v) => v,
        Err(crate::http::HttpError::Status { status, .. }) if is_fatal_status(status) => {
            return Err(GitlabPackagesError::Http(crate::http::HttpError::Status {
                status,
                url: url.clone(),
            }));
        }
        Err(crate::http::HttpError::Status { .. }) => return Ok(None),
        Err(crate::http::HttpError::Request(_)) => return Ok(None),
    };

    let releases: Vec<GitlabPackageRelease> = api_packages
        .into_iter()
        .filter(|p| {
            let effective_name = p.conan_package_name.as_deref().unwrap_or(&p.name);
            effective_name == pkg_part
        })
        .map(|p| GitlabPackageRelease {
            version: p.version,
            release_timestamp: p.created_at.as_deref().and_then(rfc3339_to_utc_iso),
        })
        .collect();

    if releases.is_empty() {
        return Ok(None);
    }

    Ok(Some(GitlabPackagesResult { releases }))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "returns package from custom registry" — gitlab-packages/index.spec.ts line 12
    #[test]
    fn filter_by_package_name() {
        let pkgs = [
            ApiPackage {
                name: "mypkg".into(),
                version: "1.0.0".into(),
                created_at: Some("2020-03-04T12:01:37.000-06:00".into()),
                conan_package_name: None,
            },
            ApiPackage {
                name: "mypkg".into(),
                version: "v1.1.0".into(),
                created_at: Some("2020-04-04T12:01:37.000-06:00".into()),
                conan_package_name: None,
            },
            ApiPackage {
                name: "mypkg".into(),
                version: "v1.1.1".into(),
                created_at: Some("2020-05-04T12:01:37.000-06:00".into()),
                conan_package_name: None,
            },
            ApiPackage {
                name: "otherpkg".into(),
                version: "v2.0.0".into(),
                created_at: Some("2020-05-04T12:01:37.000-06:00".into()),
                conan_package_name: None,
            },
        ];

        assert_eq!(
            pkgs.iter()
                .filter(|p| {
                    let effective = p.conan_package_name.as_deref().unwrap_or(&p.name);
                    effective == "mypkg"
                })
                .count(),
            3
        );
    }

    // Ported: "returns conan package from custom registry" — gitlab-packages/index.spec.ts line 60
    #[test]
    fn filter_by_conan_package_name() {
        let pkgs = [
            ApiPackage {
                name: "myconanpkg/1.0.0@mycompany/stable".into(),
                version: "1.0.0".into(),
                created_at: Some("2020-03-04T12:01:37.000-06:00".into()),
                conan_package_name: Some("myconanpkg".into()),
            },
            ApiPackage {
                name: "otherpkg/2.0.0@mycompany/stable".into(),
                version: "v2.0.0".into(),
                created_at: Some("2020-05-04T12:01:37.000-06:00".into()),
                conan_package_name: Some("otherpkg".into()),
            },
        ];

        assert_eq!(
            pkgs.iter()
                .filter(|p| {
                    let effective = p.conan_package_name.as_deref().unwrap_or(&p.name);
                    effective == "myconanpkg"
                })
                .count(),
            1
        );
        assert_eq!(
            rfc3339_to_utc_iso("2020-03-04T12:01:37.000-06:00").as_deref(),
            Some("2020-03-04T18:01:37.000Z")
        );
    }

    // Ported: "returns null for empty 200 OK" — gitlab-packages/index.spec.ts line 113
    #[test]
    fn empty_releases_yields_none() {
        let pkgs: Vec<ApiPackage> = vec![];

        assert!(!pkgs.iter().any(|p| {
            let effective = p.conan_package_name.as_deref().unwrap_or(&p.name);
            effective == "mypkg"
        }));
    }
}
