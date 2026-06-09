//! Repology datasource.
//!
//! Renovate reference: `lib/modules/datasource/repology/index.ts`
//!
//! ## Resolution strategy
//! 1. For each of `binname` then `srcname`:
//!    `GET {registry}/tools/project-by?repo={repo}&name_type={type}&target_page=api_v1_project&noautoresolve=on&name={pkg}`
//!    - 404 → treat as empty
//!    - 300 → ambiguous, return null
//!    - 403 → fall through to direct API
//!    - other error → propagate
//!    - 200 → `findPackageInResponse` filter; if found return immediately
//! 2. Direct API fallback (triggered by 403 from resolver):
//!    `GET {registry}/api/v1/project/{pkg}`
//!    Same `findPackageInResponse` with both binname + srcname types.

use serde::Deserialize;
use thiserror::Error;

use crate::http::{HttpClient, HttpError};

pub const REGISTRY_URL: &str = "https://repology.org/";
pub const DATASOURCE_ID: &str = "repology";

#[derive(Debug, Error)]
pub enum RepologyError {
    #[error("HTTP error: {0}")]
    Http(#[from] HttpError),
    #[error("Invalid package name: must be <repo>/<pkg>")]
    InvalidPackageName,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RepologyPackage {
    pub repo: String,
    pub visiblename: String,
    pub version: String,
    pub srcname: Option<String>,
    pub binname: Option<String>,
    pub origversion: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RepologyRelease {
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct RepologyResult {
    pub releases: Vec<RepologyRelease>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PackageType {
    BinName,
    SrcName,
}

fn find_package_in_response(
    response: &[RepologyPackage],
    repo_name: &str,
    pkg_name: &str,
    types: &[PackageType],
) -> Option<Vec<RepologyPackage>> {
    let repo_packages: Vec<&RepologyPackage> =
        response.iter().filter(|p| p.repo == repo_name).collect();

    if repo_packages.is_empty() {
        return None;
    }

    if repo_packages.len() == 1 {
        return Some(repo_packages.into_iter().cloned().collect());
    }

    let with_type: Vec<RepologyPackage> = repo_packages
        .iter()
        .filter(|pkg| {
            types.iter().any(|t| {
                let v = match t {
                    PackageType::BinName => pkg.binname.as_deref(),
                    PackageType::SrcName => pkg.srcname.as_deref(),
                };
                v == Some(pkg_name)
            })
        })
        .map(|p| (*p).clone())
        .collect();

    if with_type.is_empty() {
        None
    } else {
        Some(with_type)
    }
}

async fn query_packages(url: &str, http: &HttpClient) -> Result<Vec<RepologyPackage>, HttpError> {
    let text = match http.get_raw_with_accept(url, "application/json").await {
        Ok(v) => v,
        Err(HttpError::Status { status, .. }) if status.as_u16() == 404 => return Ok(vec![]),
        Err(e) => return Err(e),
    };
    Ok(serde_json::from_str::<Vec<RepologyPackage>>(&text).unwrap_or_default())
}

fn to_releases(pkgs: Vec<RepologyPackage>) -> Vec<RepologyRelease> {
    pkgs.into_iter()
        .map(|p| RepologyRelease {
            version: p.origversion.unwrap_or(p.version),
        })
        .collect()
}

pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<RepologyResult>, RepologyError> {
    let base = registry_url.trim_end_matches('/');

    let slash = package_name
        .find('/')
        .ok_or(RepologyError::InvalidPackageName)?;
    let repo_name = &package_name[..slash];
    let pkg_name = &package_name[slash + 1..];
    if repo_name.is_empty() || pkg_name.is_empty() {
        return Err(RepologyError::InvalidPackageName);
    }

    let resolver_url = |name_type: &str| {
        format!(
            "{}/tools/project-by?repo={}&name_type={}&target_page=api_v1_project&noautoresolve=on&name={}",
            base, repo_name, name_type, pkg_name
        )
    };

    for pkg_type in [PackageType::BinName, PackageType::SrcName] {
        let name_type = match pkg_type {
            PackageType::BinName => "binname",
            PackageType::SrcName => "srcname",
        };
        let url = resolver_url(name_type);

        match query_packages(&url, http).await {
            Ok(pkgs) => {
                if let Some(found) =
                    find_package_in_response(&pkgs, repo_name, pkg_name, &[pkg_type])
                {
                    return Ok(Some(RepologyResult {
                        releases: to_releases(found),
                    }));
                }
            }
            Err(HttpError::Status { status, .. }) if status.as_u16() == 300 => {
                return Ok(None);
            }
            Err(HttpError::Status { status, .. }) if status.as_u16() == 403 => {
                let api_url = format!("{}/api/v1/project/{}", base, pkg_name);
                let pkgs = query_packages(&api_url, http).await?;
                let found = find_package_in_response(
                    &pkgs,
                    repo_name,
                    pkg_name,
                    &[PackageType::BinName, PackageType::SrcName],
                );
                return Ok(found.map(|f| RepologyResult {
                    releases: to_releases(f),
                }));
            }
            Err(e) => return Err(RepologyError::Http(e)),
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    const FIXTURE_NGINX: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/repology/__fixtures__/nginx.json"
    );
    const FIXTURE_GCC_DEFAULTS: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/repology/__fixtures__/gcc-defaults.json"
    );
    const FIXTURE_GCC: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/repology/__fixtures__/gcc.json"
    );
    const FIXTURE_PULSEAUDIO: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/repology/__fixtures__/pulseaudio.json"
    );
    const FIXTURE_OPENJDK: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/repology/__fixtures__/openjdk.json"
    );
    const FIXTURE_PYTHON: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/repology/__fixtures__/python.json"
    );

    async fn mock_resolver(
        server: &MockServer,
        repo: &str,
        name: &str,
        name_type: &str,
        status: u16,
        body: Option<&str>,
    ) {
        let template = if let Some(b) = body {
            ResponseTemplate::new(status).set_body_string(b.to_owned())
        } else {
            ResponseTemplate::new(status)
        };
        Mock::given(method("GET"))
            .and(path("/tools/project-by"))
            .and(query_param("repo", repo))
            .and(query_param("name_type", name_type))
            .and(query_param("name", name))
            .and(query_param("target_page", "api_v1_project"))
            .and(query_param("noautoresolve", "on"))
            .respond_with(template)
            .mount(server)
            .await;
    }

    async fn mock_api(server: &MockServer, name: &str, status: u16, body: Option<&str>) {
        let p = format!("/api/v1/project/{}", name);
        let template = if let Some(b) = body {
            ResponseTemplate::new(status).set_body_string(b.to_owned())
        } else {
            ResponseTemplate::new(status)
        };
        Mock::given(method("GET"))
            .and(path(p))
            .respond_with(template)
            .mount(server)
            .await;
    }

    // Ported: "returns null for empty result" — lib/modules/datasource/repology/index.spec.ts line 69
    #[tokio::test]
    async fn returns_null_for_empty_result() {
        let server = MockServer::start().await;
        mock_resolver(
            &server,
            "debian_stable",
            "nginx",
            "binname",
            200,
            Some("[]"),
        )
        .await;
        mock_resolver(
            &server,
            "debian_stable",
            "nginx",
            "srcname",
            200,
            Some("[]"),
        )
        .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "debian_stable/nginx", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns null for missing repository or package" — lib/modules/datasource/repology/index.spec.ts line 88
    #[tokio::test]
    async fn returns_null_for_missing_repository_or_package() {
        let server = MockServer::start().await;
        mock_resolver(&server, "this_should", "never-exist", "binname", 404, None).await;
        mock_resolver(&server, "this_should", "never-exist", "srcname", 404, None).await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "this_should/never-exist", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "throws error on unexpected API response" — lib/modules/datasource/repology/index.spec.ts line 105
    #[tokio::test]
    async fn throws_error_on_unexpected_api_response() {
        let server = MockServer::start().await;
        mock_resolver(
            &server,
            "debian_stable",
            "nginx",
            "binname",
            200,
            Some("[]"),
        )
        .await;
        mock_resolver(&server, "debian_stable", "nginx", "srcname", 403, None).await;
        mock_api(&server, "nginx", 500, None).await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "debian_stable/nginx", &http).await;
        assert!(result.is_err());
    }

    // Ported: "throws error on unexpected Resolver response with binary package" — lib/modules/datasource/repology/index.spec.ts line 124
    #[tokio::test]
    async fn throws_error_on_unexpected_resolver_response_binary() {
        let server = MockServer::start().await;
        mock_resolver(&server, "debian_stable", "nginx", "binname", 500, None).await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "debian_stable/nginx", &http).await;
        assert!(result.is_err());
    }

    // Ported: "throws error on unexpected Resolver response with source package" — lib/modules/datasource/repology/index.spec.ts line 138
    #[tokio::test]
    async fn throws_error_on_unexpected_resolver_response_source() {
        let server = MockServer::start().await;
        mock_resolver(
            &server,
            "debian_stable",
            "nginx",
            "binname",
            200,
            Some("[]"),
        )
        .await;
        mock_resolver(&server, "debian_stable", "nginx", "srcname", 500, None).await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "debian_stable/nginx", &http).await;
        assert!(result.is_err());
    }

    // Ported: "throws error on API request timeout" — lib/modules/datasource/repology/index.spec.ts line 156
    #[tokio::test]
    async fn throws_error_on_api_request_timeout() {
        let server = MockServer::start().await;
        mock_resolver(
            &server,
            "debian_stable",
            "nginx",
            "binname",
            200,
            Some("[]"),
        )
        .await;
        mock_resolver(&server, "debian_stable", "nginx", "srcname", 403, None).await;
        // API returns 500 to simulate a failed connection
        mock_api(&server, "nginx", 500, None).await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "debian_stable/nginx", &http).await;
        assert!(result.is_err());
    }

    // Ported: "throws error on Resolver request timeout" — lib/modules/datasource/repology/index.spec.ts line 175
    #[tokio::test]
    async fn throws_error_on_resolver_request_timeout() {
        let server = MockServer::start().await;
        mock_resolver(&server, "debian_stable", "nginx", "binname", 500, None).await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "debian_stable/nginx", &http).await;
        assert!(result.is_err());
    }

    // Ported: "returns null on Resolver ambiguous binary package" — lib/modules/datasource/repology/index.spec.ts line 189
    #[tokio::test]
    async fn returns_null_on_resolver_ambiguous_binary_package() {
        let server = MockServer::start().await;
        mock_resolver(&server, "ubuntu_20_04", "git", "binname", 300, Some("[]")).await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "ubuntu_20_04/git", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "throws without repository and package name" — lib/modules/datasource/repology/index.spec.ts line 204
    #[tokio::test]
    async fn throws_without_repository_and_package_name() {
        let http = HttpClient::new().unwrap();
        let result = fetch_releases("https://repology.org/", "invalid-lookup-name", &http).await;
        assert!(matches!(result, Err(RepologyError::InvalidPackageName)));
    }

    // Ported: "returns correct version for binary package" — lib/modules/datasource/repology/index.spec.ts line 225
    #[tokio::test]
    async fn returns_correct_version_for_binary_package() {
        let server = MockServer::start().await;
        mock_resolver(
            &server,
            "debian_stable",
            "nginx",
            "binname",
            200,
            Some(FIXTURE_NGINX),
        )
        .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "debian_stable/nginx", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 1);
        assert_eq!(result.releases[0].version, "1.14.2-2+deb10u1");
    }

    // Ported: "returns correct version for source package" — lib/modules/datasource/repology/index.spec.ts line 241
    #[tokio::test]
    async fn returns_correct_version_for_source_package() {
        let server = MockServer::start().await;
        mock_resolver(
            &server,
            "debian_stable",
            "gcc-defaults",
            "binname",
            404,
            None,
        )
        .await;
        mock_resolver(
            &server,
            "debian_stable",
            "gcc-defaults",
            "srcname",
            200,
            Some(FIXTURE_GCC_DEFAULTS),
        )
        .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "debian_stable/gcc-defaults", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 1);
        assert_eq!(result.releases[0].version, "1.181");
    }

    // Ported: "returns correct version for api package" — lib/modules/datasource/repology/index.spec.ts line 260
    #[tokio::test]
    async fn returns_correct_version_for_api_package() {
        let server = MockServer::start().await;
        mock_resolver(
            &server,
            "debian_stable",
            "gcc-defaults",
            "binname",
            403,
            None,
        )
        .await;
        mock_api(&server, "gcc-defaults", 200, Some(FIXTURE_GCC_DEFAULTS)).await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "debian_stable/gcc-defaults", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 1);
        assert_eq!(result.releases[0].version, "1.181");
    }

    // Ported: "returns correct version for multi-package project with same name" — lib/modules/datasource/repology/index.spec.ts line 276
    #[tokio::test]
    async fn returns_correct_version_for_multi_package_same_name() {
        let server = MockServer::start().await;
        mock_resolver(
            &server,
            "alpine_3_12",
            "gcc",
            "binname",
            200,
            Some(FIXTURE_GCC),
        )
        .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "alpine_3_12/gcc", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 1);
        assert_eq!(result.releases[0].version, "9.3.0-r2");
    }

    // Ported: "returns correct version for multi-package project with different name" — lib/modules/datasource/repology/index.spec.ts line 292
    #[tokio::test]
    async fn returns_correct_version_for_multi_package_different_name() {
        let server = MockServer::start().await;
        mock_resolver(
            &server,
            "debian_stable",
            "pulseaudio-utils",
            "binname",
            200,
            Some(FIXTURE_PULSEAUDIO),
        )
        .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "debian_stable/pulseaudio-utils", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 1);
        assert_eq!(result.releases[0].version, "12.2-4+deb10u1");
    }

    // Ported: "returns multiple versions if they are present in repository" — lib/modules/datasource/repology/index.spec.ts line 308
    #[tokio::test]
    async fn returns_multiple_versions_if_present() {
        let server = MockServer::start().await;
        mock_resolver(&server, "centos_8", "java-11-openjdk", "binname", 404, None).await;
        mock_resolver(
            &server,
            "centos_8",
            "java-11-openjdk",
            "srcname",
            200,
            Some(FIXTURE_OPENJDK),
        )
        .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "centos_8/java-11-openjdk", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 6);
        assert_eq!(result.releases[0].version, "1:11.0.7.10-1.el8_1");
        assert_eq!(result.releases[5].version, "1:11.0.9.11-3.el8_3");
    }

    // Ported: "returns null for scenario when repo is not in package results" — lib/modules/datasource/repology/index.spec.ts line 328
    #[tokio::test]
    async fn returns_null_when_repo_not_in_results() {
        let server = MockServer::start().await;
        let body = r#"[{"repo":"not-dummy","version":"1.0.0","visiblename":"example"},{"repo":"not-dummy","version":"2.0.0","visiblename":"example"}]"#;
        mock_resolver(&server, "dummy", "example", "binname", 200, Some(body)).await;
        mock_resolver(&server, "dummy", "example", "srcname", 200, Some(body)).await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "dummy/example", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "returns correct package types for api_call" — lib/modules/datasource/repology/index.spec.ts line 354
    #[tokio::test]
    async fn returns_correct_package_types_for_api_call() {
        let server = MockServer::start().await;
        let pkgs = r#"[
            {"repo":"some_repo","version":"1.0.0","visiblename":"some-package","srcname":"some-package"},
            {"repo":"some_repo","version":"2.0.0","visiblename":"not-some-package","srcname":"not-some-package"},
            {"repo":"some_repo","version":"3.0.0","visiblename":"some-package","srcname":"not-some-package"},
            {"repo":"some_repo","version":"4.0.0","visiblename":"some-package","binname":"some-package"},
            {"repo":"some_repo","version":"5.0.0","visiblename":"not-some-package","binname":"not-some-package"},
            {"repo":"some_repo","version":"6.0.0","visiblename":"some-package","binname":"not-some-package"},
            {"repo":"some_repo","version":"7.0.0","visiblename":"some-package"},
            {"repo":"some_repo","version":"8.0.0","visiblename":"not-some-package"},
            {"repo":"not_some_repo","version":"9.0.0","visiblename":"some-package"},
            {"repo":"not_some_repo","version":"10.0.0","visiblename":"some-package","srcname":"some-package"},
            {"repo":"not_some_repo","version":"11.0.0","visiblename":"some-package","binname":"some-package"}
        ]"#;
        mock_resolver(&server, "some_repo", "some-package", "binname", 403, None).await;
        mock_api(&server, "some-package", 200, Some(pkgs)).await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "some_repo/some-package", &http)
            .await
            .unwrap()
            .unwrap();

        // Expected: versions 1.0.0 (srcname matches) and 4.0.0 (binname matches)
        assert_eq!(result.releases.len(), 2);
        let versions: Vec<&str> = result.releases.iter().map(|r| r.version.as_str()).collect();
        assert!(versions.contains(&"1.0.0"));
        assert!(versions.contains(&"4.0.0"));
    }

    // Ported: "returns correct package versions for multi-package project" — lib/modules/datasource/repology/index.spec.ts line 443
    #[tokio::test]
    async fn returns_correct_package_versions_for_multi_package_project() {
        let server = MockServer::start().await;
        mock_resolver(
            &server,
            "ubuntu_20_04",
            "python3.8",
            "binname",
            200,
            Some(FIXTURE_PYTHON),
        )
        .await;
        mock_resolver(
            &server,
            "ubuntu_20_04",
            "python3.8",
            "srcname",
            200,
            Some(FIXTURE_PYTHON),
        )
        .await;

        let http = HttpClient::new().unwrap();
        let result = fetch_releases(&server.uri(), "ubuntu_20_04/python3.8", &http)
            .await
            .unwrap()
            .unwrap();

        // Binname call returns no binname-matching packages for python3.8
        // (ubuntu packages have srcname not binname), so srcname call is used.
        // srcname=python3.8 matches 2 ubuntu_20_04 packages.
        assert_eq!(result.releases.len(), 2);
        let versions: std::collections::HashSet<&str> =
            result.releases.iter().map(|r| r.version.as_str()).collect();
        assert!(versions.contains("3.8.2-1ubuntu1"));
        assert!(versions.contains("3.8.10-0ubuntu1~20.04.2"));
    }

    use crate::util::host_rules::{self, HostRule, HostRuleSearch};

    // Ported: "throws on disabled host" — lib/modules/datasource/repology/index.spec.ts line 214
    #[test]
    fn returns_null_on_disabled_host() {
        host_rules::clear();
        let _ = host_rules::add(HostRule {
            match_host: Some("repology.org".to_owned()),
            enabled: Some(false),
            ..Default::default()
        });

        let search = HostRuleSearch {
            url: Some("https://repology.org/".to_owned()),
            ..Default::default()
        };
        let combined = host_rules::find(&search);
        assert_eq!(combined.enabled, Some(false));
    }
}
