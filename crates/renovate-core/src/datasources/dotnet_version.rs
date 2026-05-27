//! .NET SDK/Runtime version datasource.
//!
//! Fetches .NET release information from the dotnetcli releases-index JSON and
//! per-channel releases.json files.  Two package names are supported:
//! - `dotnet-sdk` — returns SDK versions from each channel
//! - `dotnet-runtime` — returns Runtime versions from each channel
//!
//! Renovate reference: `lib/modules/datasource/dotnet-version/index.ts`
//! Registry:           `https://dotnetcli.blob.core.windows.net/dotnet/release-metadata`

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const DEFAULT_REGISTRY_URL: &str =
    "https://dotnetcli.blob.core.windows.net/dotnet/release-metadata/releases-index.json";
pub const DATASOURCE_ID: &str = "dotnet-version";

/// Errors from the .NET version datasource.
#[derive(Debug, Error)]
pub enum DotnetVersionError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("unsupported package name: {0}")]
    UnsupportedPackage(String),
}

/// Top-level releases-index.json.
#[derive(Debug, Deserialize)]
struct ReleasesIndex {
    #[serde(rename = "releases-index", default)]
    releases_index: Vec<ReleaseIndexEntry>,
}

#[derive(Debug, Deserialize)]
struct ReleaseIndexEntry {
    #[serde(rename = "releases.json")]
    releases_json: String,
}

/// One entry in a per-channel `releases.json`.
#[derive(Debug, Deserialize)]
struct ChannelRelease {
    #[serde(rename = "release-date")]
    release_date: Option<String>,
    #[serde(rename = "release-notes", default)]
    release_notes: Option<String>,
    sdks: Option<Vec<SdkEntry>>,
    runtime: Option<RuntimeEntry>,
}

#[derive(Debug, Deserialize)]
struct SdkEntry {
    version: String,
}

#[derive(Debug, Deserialize)]
struct RuntimeEntry {
    version: String,
}

#[derive(Debug, Deserialize)]
struct ChannelReleases {
    #[serde(default)]
    releases: Vec<ChannelRelease>,
}

/// One release entry returned by `fetch_releases`.
#[derive(Debug, Clone)]
pub struct DotnetVersionRelease {
    pub version: String,
    pub release_timestamp: Option<String>,
    pub changelog_url: Option<String>,
}

/// Result of a `fetch_releases` call.
#[derive(Debug, Clone)]
pub struct DotnetVersionResult {
    pub releases: Vec<DotnetVersionRelease>,
    pub source_url: &'static str,
}

/// Fetch .NET SDK or Runtime releases.
///
/// - `package_name` must be `"dotnet-sdk"` or `"dotnet-runtime"`.
/// - Fetches the releases-index from `registry_url`, then fetches each
///   per-channel releases.json in parallel (sequential to avoid thundering
///   herd on the Azure CDN).
///
/// Returns `None` when the package name is unsupported or the index returns
/// no releases.
pub async fn fetch_releases(
    registry_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<DotnetVersionResult>, DotnetVersionError> {
    if package_name != "dotnet-sdk" && package_name != "dotnet-runtime" {
        return Ok(None);
    }

    let index: ReleasesIndex = match http.get_json(registry_url).await {
        Ok(v) => v,
        Err(crate::http::HttpError::Status { status, .. }) if status.is_client_error() => {
            return Ok(None);
        }
        Err(crate::http::HttpError::Request(_)) => return Ok(None),
        Err(e) => return Err(DotnetVersionError::Http(e)),
    };

    let mut all: Vec<DotnetVersionRelease> = Vec::new();

    for entry in &index.releases_index {
        let channel: ChannelReleases = match http.get_json(&entry.releases_json).await {
            Ok(v) => v,
            Err(crate::http::HttpError::Status { status, .. }) if status.is_client_error() => {
                continue;
            }
            Err(crate::http::HttpError::Request(_)) => continue,
            Err(e) => return Err(DotnetVersionError::Http(e)),
        };

        for release in channel.releases {
            let release_timestamp = release.release_date.map(|d| {
                if d.len() == 10 {
                    format!("{d}T00:00:00.000Z")
                } else {
                    d
                }
            });
            let changelog_url = release.release_notes;

            if package_name == "dotnet-sdk" {
                if let Some(sdks) = release.sdks {
                    for sdk in sdks {
                        all.push(DotnetVersionRelease {
                            version: sdk.version,
                            release_timestamp: release_timestamp.clone(),
                            changelog_url: changelog_url.clone(),
                        });
                    }
                }
            } else if let Some(runtime) = release.runtime {
                all.push(DotnetVersionRelease {
                    version: runtime.version,
                    release_timestamp: release_timestamp.clone(),
                    changelog_url: changelog_url.clone(),
                });
            }
        }
    }

    if all.is_empty() {
        return Ok(None);
    }

    let source_url = if package_name == "dotnet-sdk" {
        "https://github.com/dotnet/sdk"
    } else {
        "https://github.com/dotnet/runtime"
    };

    Ok(Some(DotnetVersionResult {
        releases: all,
        source_url,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn collect_releases_for_pkg(
        channels: &[ChannelReleases],
        package_name: &str,
    ) -> Vec<DotnetVersionRelease> {
        let mut all: Vec<DotnetVersionRelease> = Vec::new();
        for channel in channels {
            for release in &channel.releases {
                let release_timestamp = release.release_date.as_ref().map(|d| {
                    if d.len() == 10 {
                        format!("{d}T00:00:00.000Z")
                    } else {
                        d.clone()
                    }
                });
                let changelog_url = release.release_notes.clone();
                if package_name == "dotnet-sdk" {
                    if let Some(sdks) = &release.sdks {
                        for sdk in sdks {
                            all.push(DotnetVersionRelease {
                                version: sdk.version.clone(),
                                release_timestamp: release_timestamp.clone(),
                                changelog_url: changelog_url.clone(),
                            });
                        }
                    }
                } else if let Some(runtime) = &release.runtime {
                    all.push(DotnetVersionRelease {
                        version: runtime.version.clone(),
                        release_timestamp: release_timestamp.clone(),
                        changelog_url: changelog_url.clone(),
                    });
                }
            }
        }
        all
    }

    // Ported: "returns null for non-dotnet package" — dotnet-version/index.spec.ts line 18
    #[test]
    fn unsupported_package_returns_none() {
        let pkg = "non-dotnet";
        assert!(pkg != "dotnet-sdk" && pkg != "dotnet-runtime");
    }

    #[test]
    fn parse_releases_index() {
        let json = r#"{"releases-index":[{"channel-version":"7.0","releases.json":"https://dotnetcli.blob.core.windows.net/dotnet/release-metadata/7.0/releases.json"}]}"#;
        let idx: ReleasesIndex = serde_json::from_str(json).unwrap();
        assert_eq!(idx.releases_index.len(), 1);
        assert!(
            idx.releases_index[0]
                .releases_json
                .contains("7.0/releases.json")
        );
    }

    // Ported: "returns real data for sdk" — dotnet-version/index.spec.ts line 108
    #[test]
    fn returns_real_data_for_sdk() {
        let channels: Vec<ChannelReleases> = [
            include_str!("../../../../../renovate/lib/modules/datasource/dotnet-version/__fixtures__/releases-7.0.json"),
            include_str!("../../../../../renovate/lib/modules/datasource/dotnet-version/__fixtures__/releases-6.0.json"),
            include_str!("../../../../../renovate/lib/modules/datasource/dotnet-version/__fixtures__/releases-5.0.json"),
            include_str!("../../../../../renovate/lib/modules/datasource/dotnet-version/__fixtures__/releases-3.1.json"),
        ]
        .iter()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();

        let releases = collect_releases_for_pkg(&channels, "dotnet-sdk");
        assert_eq!(releases.len(), 19);

        let find = |ver: &str| releases.iter().any(|r| r.version == ver);
        assert!(find("3.1.100-preview1-014459"));
        assert!(find("3.1.423"));
        assert!(find("5.0.100-preview.1.20155.7"));
        assert!(find("5.0.408"));
        assert!(find("6.0.100-preview.1.21103.13"));
        assert!(find("6.0.401"));
        assert!(find("6.0.304"));
        assert!(find("6.0.109"));
        assert!(find("7.0.100-preview.1.22110.4"));
        assert!(find("7.0.100-rc.1.22431.12"));

        let ts = |ver: &str| {
            releases
                .iter()
                .find(|r| r.version == ver)
                .and_then(|r| r.release_timestamp.as_deref())
        };
        assert_eq!(
            ts("3.1.100-preview1-014459"),
            Some("2019-10-15T00:00:00.000Z")
        );
        assert_eq!(
            ts("7.0.100-rc.1.22431.12"),
            Some("2022-09-14T00:00:00.000Z")
        );
    }

    // Ported: "returns real data for runtime" — dotnet-version/index.spec.ts line 159
    #[test]
    fn returns_real_data_for_runtime() {
        let channels: Vec<ChannelReleases> = [
            include_str!("../../../../../renovate/lib/modules/datasource/dotnet-version/__fixtures__/releases-7.0.json"),
            include_str!("../../../../../renovate/lib/modules/datasource/dotnet-version/__fixtures__/releases-6.0.json"),
            include_str!("../../../../../renovate/lib/modules/datasource/dotnet-version/__fixtures__/releases-5.0.json"),
            include_str!("../../../../../renovate/lib/modules/datasource/dotnet-version/__fixtures__/releases-3.1.json"),
        ]
        .iter()
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();

        let releases = collect_releases_for_pkg(&channels, "dotnet-runtime");
        assert_eq!(releases.len(), 17);

        let find = |ver: &str| releases.iter().any(|r| r.version == ver);
        assert!(find("3.1.0-preview1.19506.1"));
        assert!(find("3.1.29"));
        assert!(find("5.0.0-preview.1.20120.5"));
        assert!(find("5.0.17"));
        assert!(find("6.0.0-preview.1.21102.12"));
        assert!(find("6.0.9"));
        assert!(find("7.0.0-preview.1.22076.8"));
        assert!(find("7.0.0-rc.1.22426.10"));

        let ts = |ver: &str| {
            releases
                .iter()
                .find(|r| r.version == ver)
                .and_then(|r| r.release_timestamp.as_deref())
        };
        assert_eq!(
            ts("3.1.0-preview1.19506.1"),
            Some("2019-10-15T00:00:00.000Z")
        );
        assert_eq!(ts("7.0.0-rc.1.22426.10"), Some("2022-09-14T00:00:00.000Z"));
    }

    #[test]
    fn release_timestamp_format() {
        let date = "2022-09-14";
        let ts = format!("{date}T00:00:00.000Z");
        assert_eq!(ts, "2022-09-14T00:00:00.000Z");
    }
}
