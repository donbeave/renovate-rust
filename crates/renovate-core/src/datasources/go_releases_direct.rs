//! Direct Go releases datasource.
//!
//! Fetches Go module versions directly from VCS (GitHub, GitLab, etc.).
//!
//! Renovate reference: `lib/modules/datasource/go/releases-direct.ts`

use thiserror::Error;

use crate::datasources::{Release, ReleaseResult};
use crate::http::HttpClient;

pub const DATASOURCE_ID: &str = "go-direct";

#[derive(Debug, Error)]
pub enum GoDirectError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("JSON parse error: {0}")]
    Json(#[from] reqwest::Error),
    #[error("unsupported go host: {0}")]
    UnsupportedHost(String),
}

#[derive(Debug, Clone)]
pub struct GoDirectSource {
    pub datasource: String,
    pub registry_url: Option<String>,
    pub package_name: String,
}

#[derive(Debug, Clone)]
pub struct GoDirectConfig {
    pub package_name: String,
}

#[allow(dead_code)]
fn filter_by_prefix(package_name: &str, releases: &mut [Release]) -> Vec<Release> {
    let name_parts: Vec<&str> = package_name
        .trim_end_matches(|c: char| c == '/' || c.is_ascii_digit())
        .trim_end_matches("/v")
        .split('/')
        .skip(1)
        .collect();

    if name_parts.is_empty() {
        return releases
            .iter()
            .filter(|r| r.version.starts_with('v'))
            .cloned()
            .collect();
    }

    let mut submodule_releases = Vec::new();
    let mut skip = 0;
    while skip < name_parts.len() {
        let prefix = format!("{}/", name_parts[skip..].join("/"));

        for release in releases.iter() {
            if !release.version.starts_with(&prefix) {
                continue;
            }

            let normalized = release.version.strip_prefix(&prefix).unwrap_or(&release.version);
            if !normalized.starts_with('v') || normalized.contains('/') {
                continue;
            }

            let mut r = release.clone();
            r.version = normalized.to_owned();
            submodule_releases.push(r);
        }

        if !submodule_releases.is_empty() {
            return submodule_releases;
        }
        skip += 1;
    }

    releases
        .iter()
        .filter(|r| r.version.starts_with('v'))
        .cloned()
        .collect()
}

pub async fn fetch_direct_versions(
    http: &HttpClient,
    source: &GoDirectSource,
    _package_name: &str,
) -> Result<ReleaseResult, GoDirectError> {
    match source.datasource.as_str() {
        "github-tags" => fetch_github_tags(http, &source.package_name).await,
        "gitlab-tags" => fetch_gitlab_tags(http, &source.package_name).await,
        "git-tags" => fetch_git_tags(http, &source.package_name).await,
        _ => Err(GoDirectError::UnsupportedHost(source.datasource.clone())),
    }
}

async fn fetch_github_tags(
    http: &HttpClient,
    repo: &str,
) -> Result<ReleaseResult, GoDirectError> {
    let url = format!("https://api.github.com/repos/{}/tags?per_page=100", repo);
    let resp = http.get_retrying(&url).await?;

    if !resp.status().is_success() {
        return Ok(ReleaseResult {
            releases: Vec::new(),
            ..Default::default()
        });
    }

    #[derive(serde::Deserialize)]
    struct GithubTag {
        name: String,
    }

    let tags: Vec<GithubTag> = resp.json().await?;
    let releases: Vec<Release> = tags
        .into_iter()
        .map(|t| Release {
            version: t.name,
            ..Default::default()
        })
        .collect();

    Ok(ReleaseResult {
        releases,
        source_url: Some(format!("https://github.com/{}", repo)),
        ..Default::default()
    })
}

async fn fetch_gitlab_tags(
    http: &HttpClient,
    repo: &str,
) -> Result<ReleaseResult, GoDirectError> {
    let encoded = repo.replace('/', "%2F");
    let url = format!(
        "https://gitlab.com/api/v4/projects/{}/repository/tags?per_page=100",
        encoded
    );
    let resp = http.get_retrying(&url).await?;

    if !resp.status().is_success() {
        return Ok(ReleaseResult {
            releases: Vec::new(),
            ..Default::default()
        });
    }

    #[derive(serde::Deserialize)]
    struct GitlabTag {
        name: String,
    }

    let tags: Vec<GitlabTag> = resp.json().await?;
    let releases: Vec<Release> = tags
        .into_iter()
        .map(|t| Release {
            version: t.name,
            ..Default::default()
        })
        .collect();

    Ok(ReleaseResult {
        releases,
        source_url: Some(format!("https://gitlab.com/{}", repo)),
        ..Default::default()
    })
}

async fn fetch_git_tags(
    _http: &HttpClient,
    url: &str,
) -> Result<ReleaseResult, GoDirectError> {
    Ok(ReleaseResult {
        releases: Vec::new(),
        source_url: Some(url.to_owned()),
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn datasource_id_is_correct() {
        assert_eq!(DATASOURCE_ID, "go-direct");
    }

    #[test]
    fn filter_by_prefix_no_submodule() {
        let mut releases = vec![
            Release {
                version: "v1.0.0".into(),
                ..Default::default()
            },
            Release {
                version: "v2.0.0".into(),
                ..Default::default()
            },
        ];
        let filtered = filter_by_prefix("github.com/gorilla/mux", &mut releases);
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn filter_by_prefix_with_submodule() {
        let mut releases = vec![
            Release {
                version: "mux/v1.0.0".into(),
                ..Default::default()
            },
            Release {
                version: "v2.0.0".into(),
                ..Default::default()
            },
        ];
        let filtered = filter_by_prefix("github.com/gorilla/mux", &mut releases);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].version, "v1.0.0");
    }

    #[test]
    fn filter_by_prefix_no_v_prefix_filtered() {
        let mut releases = vec![
            Release {
                version: "1.0.0".into(),
                ..Default::default()
            },
            Release {
                version: "v2.0.0".into(),
                ..Default::default()
            },
        ];
        let filtered = filter_by_prefix("github.com/gorilla/mux", &mut releases);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].version, "v2.0.0");
    }

    #[test]
    fn go_direct_source_fields() {
        let source = GoDirectSource {
            datasource: "github-tags".into(),
            registry_url: None,
            package_name: "gorilla/mux".into(),
        };
        assert_eq!(source.datasource, "github-tags");
        assert_eq!(source.package_name, "gorilla/mux");
    }
}
