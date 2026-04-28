//! Docker Hub datasource.
//!
//! Fetches available tags for a Docker image from the Docker Hub REST API.
//!
//! Renovate reference:
//! - `lib/modules/datasource/docker/index.ts` — `DockerDatasource._getDockerHubTags`
//! - `lib/modules/datasource/docker/schema.ts`  — `DockerHubTagsPage`
//!
//! ## Protocol
//!
//! `GET https://hub.docker.com/v2/repositories/{namespace}/{image}/tags?page_size=100&ordering=last_updated`
//!
//! Official images (no slash in the name, e.g. `ubuntu`, `nginx`) use the
//! `library` namespace.  User/org images (`owner/repo`) use the owner as the
//! namespace.
//!
//! Non-Docker-Hub registries (those containing a hostname, e.g.
//! `ghcr.io/owner/image`) are detected and returned as-is with a skip reason.
//!
//! ## Tag versioning
//!
//! Docker tags are not semver.  This module uses component-wise numeric
//! comparison: `"22.04"` < `"22.04.1"`, `"18"` < `"20"`.  The non-numeric
//! suffix (e.g. `-alpine`, `-bullseye-slim`) is treated as a variant selector:
//! only tags sharing the exact same suffix are considered compatible.

use std::sync::Arc;

use serde::Deserialize;
use thiserror::Error;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

use crate::http::{HttpClient, HttpError};

/// Docker Hub REST API base URL.
pub const DOCKER_HUB_API: &str = "https://hub.docker.com";

/// Errors from Docker Hub lookups.
#[derive(Debug, Error)]
pub enum DockerHubError {
    #[error("HTTP error: {0}")]
    Http(#[from] HttpError),
    #[error("failed to parse Docker Hub response: {0}")]
    Parse(String),
    #[error("non-Docker-Hub registry: {0}")]
    NonDockerHub(String),
}

/// A single tag entry from the Docker Hub API.
#[derive(Debug, Clone, Deserialize)]
pub struct DockerHubTag {
    pub name: String,
}

/// One page of Docker Hub tags.
#[derive(Debug, Deserialize)]
struct DockerHubTagsPage {
    #[serde(default)]
    next: Option<String>,
    results: Vec<DockerHubTag>,
}

/// Update summary for a single Docker image dep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DockerUpdateSummary {
    pub current_tag: String,
    /// Latest tag with the same variant suffix as `current_tag`, or `None`
    /// when the current tag is non-versioned (e.g. `latest`).
    pub latest: Option<String>,
    /// `true` when `latest` differs from `current_tag` and both are versioned.
    pub update_available: bool,
}

// ── Image name parsing ────────────────────────────────────────────────────────

/// Resolved Docker Hub namespace and repository name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DockerHubRepo {
    /// Docker Hub namespace (e.g. `"library"`, `"tiangolo"`).
    pub namespace: String,
    /// Repository name within the namespace (e.g. `"ubuntu"`, `"fastapi"`).
    pub repo: String,
}

/// Parse a Docker image name into a `DockerHubRepo`.
///
/// Returns `Err(NonDockerHub)` when the image name contains a hostname (any
/// `/`-separated component that contains a `.` or `:` — registry host
/// convention from the OCI spec).
///
/// Official images (`ubuntu`, `nginx`) map to namespace `library`.
pub fn parse_image_name(image: &str) -> Result<DockerHubRepo, DockerHubError> {
    let parts: Vec<&str> = image.splitn(3, '/').collect();
    match parts.as_slice() {
        [name] => {
            // No slash — official Docker Hub image.
            Ok(DockerHubRepo {
                namespace: "library".into(),
                repo: (*name).to_owned(),
            })
        }
        [ns, repo] => {
            // One slash — could be user/repo OR registry/image.
            // A registry host always contains a '.' or ':'.
            if ns.contains('.') || ns.contains(':') {
                Err(DockerHubError::NonDockerHub(image.to_owned()))
            } else {
                Ok(DockerHubRepo {
                    namespace: (*ns).to_owned(),
                    repo: (*repo).to_owned(),
                })
            }
        }
        _ => {
            // Multiple slashes — registry/namespace/repo or deeper paths.
            // Treat the first component as a registry if it looks like a host.
            if parts[0].contains('.') || parts[0].contains(':') {
                Err(DockerHubError::NonDockerHub(image.to_owned()))
            } else {
                // Concatenate remaining parts as the image name.
                Ok(DockerHubRepo {
                    namespace: parts[0].to_owned(),
                    repo: parts[1..].join("/"),
                })
            }
        }
    }
}

// ── Tag fetching ──────────────────────────────────────────────────────────────

/// Fetch all tags for a Docker Hub image (up to two pages / 200 tags).
///
/// Tags are returned in the order the API provides them
/// (`ordering=last_updated` — most recently pushed first).
pub async fn fetch_tags(
    http: &HttpClient,
    dhr: &DockerHubRepo,
    api_base: &str,
) -> Result<Vec<String>, DockerHubError> {
    let mut tags = Vec::new();
    let base = api_base.trim_end_matches('/');
    let mut url = Some(format!(
        "{base}/v2/repositories/{}/{}/tags?page_size=100&ordering=last_updated",
        dhr.namespace, dhr.repo
    ));

    // Fetch at most 2 pages (200 tags) to keep the request count bounded.
    for _ in 0..2 {
        let Some(page_url) = url.take() else {
            break;
        };
        let resp = http.get_retrying(&page_url).await?;
        if !resp.status().is_success() {
            return Err(DockerHubError::Http(HttpError::Status {
                status: resp.status(),
                url: page_url,
            }));
        }
        let body = resp.text().await.map_err(HttpError::Request)?;
        let page: DockerHubTagsPage =
            serde_json::from_str(&body).map_err(|e| DockerHubError::Parse(e.to_string()))?;

        tags.extend(page.results.into_iter().map(|t| t.name));
        url = page.next;
    }

    Ok(tags)
}

// ── Tag versioning ────────────────────────────────────────────────────────────

/// Split a Docker tag into its numeric version prefix and non-numeric suffix.
///
/// Returns `None` when the tag has no leading numeric component (e.g.
/// `"latest"`, `"stable"`, `"edge"`).
///
/// Examples:
/// - `"22.04"`         → `("22.04",       "")`
/// - `"18-alpine"`     → `("18",          "-alpine")`
/// - `"1.25.3-slim"`   → `("1.25.3",      "-slim")`
/// - `"latest"`        → `None`
pub fn split_version_tag(tag: &str) -> Option<(&str, &str)> {
    let end = tag
        .find(|c: char| !c.is_ascii_digit() && c != '.')
        .unwrap_or(tag.len());
    if end == 0 {
        return None;
    }
    let numeric = tag[..end].trim_end_matches('.');
    if numeric.is_empty() {
        return None;
    }
    Some((numeric, &tag[end..]))
}

/// Compare two version strings using component-wise numeric ordering.
///
/// Each version is split on `.` and components are compared as `u64`.
/// A shorter version wins ties by treating missing components as `0`:
/// `"18"` == `"18.0"`.
fn cmp_version(a: &str, b: &str) -> std::cmp::Ordering {
    let parse_parts =
        |s: &str| -> Vec<u64> { s.split('.').filter_map(|p| p.parse().ok()).collect() };
    let ap = parse_parts(a);
    let bp = parse_parts(b);
    let len = ap.len().max(bp.len());
    for i in 0..len {
        let av = ap.get(i).copied().unwrap_or(0);
        let bv = bp.get(i).copied().unwrap_or(0);
        match av.cmp(&bv) {
            std::cmp::Ordering::Equal => continue,
            other => return other,
        }
    }
    std::cmp::Ordering::Equal
}

/// Produce an update summary for one Docker image dependency.
///
/// `tags` should be the full tag list for the image from the registry
/// (order doesn't matter here).
pub fn docker_update_summary(current_tag: &str, tags: &[String]) -> DockerUpdateSummary {
    let Some((current_ver, suffix)) = split_version_tag(current_tag) else {
        // Non-versioned tag (e.g. "latest") — nothing to update.
        return DockerUpdateSummary {
            current_tag: current_tag.to_owned(),
            latest: None,
            update_available: false,
        };
    };

    // Find all tags with the same variant suffix.
    let compatible: Vec<&str> = tags
        .iter()
        .filter_map(|t| {
            let (ver, sfx) = split_version_tag(t)?;
            if sfx == suffix { Some(ver) } else { None }
        })
        .collect();

    // Pick the highest version by component-wise comparison.
    let latest_ver = compatible.iter().max_by(|a, b| cmp_version(a, b)).copied();

    match latest_ver {
        None => DockerUpdateSummary {
            current_tag: current_tag.to_owned(),
            latest: None,
            update_available: false,
        },
        Some(latest) => {
            let latest_tag = format!("{latest}{suffix}");
            let update_available = cmp_version(latest, current_ver).is_gt();
            DockerUpdateSummary {
                current_tag: current_tag.to_owned(),
                latest: Some(latest_tag),
                update_available,
            }
        }
    }
}

// ── Concurrent batch fetch ────────────────────────────────────────────────────

/// Input for a single Dockerfile image dep.
#[derive(Debug, Clone)]
pub struct DockerDepInput {
    /// The display name (image:tag or image).
    pub dep_name: String,
    /// The full image name without tag (e.g. `"ubuntu"`, `"owner/repo"`).
    pub image: String,
    /// The current tag string.
    pub tag: String,
}

/// Result for a single Dockerfile image dep.
#[derive(Debug)]
pub struct DockerDepUpdate {
    pub dep_name: String,
    pub summary: Result<DockerUpdateSummary, DockerHubError>,
}

/// Fetch tag info for a batch of Docker image deps concurrently.
pub async fn fetch_updates_concurrent(
    http: &HttpClient,
    deps: &[DockerDepInput],
    api_base: &str,
    concurrency: usize,
) -> Vec<DockerDepUpdate> {
    let sem = Arc::new(Semaphore::new(concurrency));
    let mut set: JoinSet<DockerDepUpdate> = JoinSet::new();

    for dep in deps {
        let http = http.clone();
        let sem = Arc::clone(&sem);
        let dep_name = dep.dep_name.clone();
        let image = dep.image.clone();
        let tag = dep.tag.clone();
        let api_base = api_base.to_owned();

        set.spawn(async move {
            let _permit = sem.acquire_owned().await.expect("semaphore closed");
            let result = async {
                let dhr = parse_image_name(&image)?;
                let tags = fetch_tags(&http, &dhr, &api_base).await?;
                Ok::<_, DockerHubError>(docker_update_summary(&tag, &tags))
            }
            .await;
            DockerDepUpdate {
                dep_name,
                summary: result,
            }
        });
    }

    let mut results = Vec::with_capacity(deps.len());
    while let Some(outcome) = set.join_next().await {
        match outcome {
            Ok(update) => results.push(update),
            Err(join_err) => tracing::error!(%join_err, "docker datasource task panicked"),
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;
    use crate::http::HttpClient;

    // ── parse_image_name ──────────────────────────────────────────────────────

    #[test]
    fn official_image_maps_to_library() {
        let r = parse_image_name("ubuntu").unwrap();
        assert_eq!(r.namespace, "library");
        assert_eq!(r.repo, "ubuntu");
    }

    #[test]
    fn user_image_uses_owner_namespace() {
        let r = parse_image_name("tiangolo/fastapi").unwrap();
        assert_eq!(r.namespace, "tiangolo");
        assert_eq!(r.repo, "fastapi");
    }

    #[test]
    fn ghcr_image_is_non_docker_hub() {
        let err = parse_image_name("ghcr.io/owner/image").unwrap_err();
        assert!(matches!(err, DockerHubError::NonDockerHub(_)));
    }

    #[test]
    fn registry_with_port_is_non_docker_hub() {
        let err = parse_image_name("registry.example.com:5000/myimage").unwrap_err();
        assert!(matches!(err, DockerHubError::NonDockerHub(_)));
    }

    // ── split_version_tag ─────────────────────────────────────────────────────

    #[test]
    fn plain_version_splits_correctly() {
        assert_eq!(split_version_tag("22.04"), Some(("22.04", "")));
        assert_eq!(split_version_tag("1.25.3"), Some(("1.25.3", "")));
    }

    #[test]
    fn variant_tag_splits_correctly() {
        assert_eq!(split_version_tag("18-alpine"), Some(("18", "-alpine")));
        assert_eq!(split_version_tag("1.25.3-slim"), Some(("1.25.3", "-slim")));
    }

    #[test]
    fn non_versioned_tag_returns_none() {
        assert_eq!(split_version_tag("latest"), None);
        assert_eq!(split_version_tag("stable"), None);
        assert_eq!(split_version_tag("edge"), None);
    }

    // ── cmp_version ───────────────────────────────────────────────────────────

    #[test]
    fn version_ordering() {
        assert!(cmp_version("22.04", "22.04.1").is_lt());
        assert!(cmp_version("22.04.2", "22.04.1").is_gt());
        assert!(cmp_version("18", "20").is_lt());
        assert!(cmp_version("1.25.3", "1.26.0").is_lt());
        assert_eq!(cmp_version("18", "18.0"), std::cmp::Ordering::Equal);
    }

    // ── docker_update_summary ─────────────────────────────────────────────────

    #[test]
    fn detects_update_for_plain_version() {
        let tags: Vec<String> = ["22.04", "22.04.1", "22.04.2", "23.10"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let s = docker_update_summary("22.04", &tags);
        assert_eq!(s.latest.as_deref(), Some("23.10"));
        assert!(s.update_available);
    }

    #[test]
    fn detects_update_for_variant_tag() {
        let tags: Vec<String> = [
            "18-alpine",
            "18.1-alpine",
            "20-alpine",
            "20.1-alpine",
            "21-alpine",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let s = docker_update_summary("18-alpine", &tags);
        assert_eq!(s.latest.as_deref(), Some("21-alpine"));
        assert!(s.update_available);
    }

    #[test]
    fn variant_tags_do_not_cross_contaminate() {
        // "-alpine" and "-slim" should be treated separately.
        let tags: Vec<String> = ["18-alpine", "20-alpine", "18-slim", "20-slim"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let s_alpine = docker_update_summary("18-alpine", &tags);
        assert_eq!(s_alpine.latest.as_deref(), Some("20-alpine"));

        let s_slim = docker_update_summary("18-slim", &tags);
        assert_eq!(s_slim.latest.as_deref(), Some("20-slim"));
    }

    #[test]
    fn latest_tag_produces_no_update() {
        let tags: Vec<String> = ["latest", "22.04"].iter().map(|s| s.to_string()).collect();
        let s = docker_update_summary("latest", &tags);
        assert!(s.latest.is_none());
        assert!(!s.update_available);
    }

    #[test]
    fn already_latest_produces_no_update() {
        let tags: Vec<String> = ["22.04", "22.04.1"].iter().map(|s| s.to_string()).collect();
        let s = docker_update_summary("22.04.1", &tags);
        assert!(!s.update_available);
    }

    // ── fetch_tags (wiremock) ─────────────────────────────────────────────────

    fn tags_page(tags: &[&str], next: Option<&str>) -> String {
        let results: String = tags
            .iter()
            .map(|t| format!(r#"{{"name":"{t}","last_updated":"2024-01-01T00:00:00Z"}}"#))
            .collect::<Vec<_>>()
            .join(",");
        let next_str = match next {
            Some(url) => format!(r#""{url}""#),
            None => "null".to_owned(),
        };
        format!(
            r#"{{"count":{c},"next":{next_str},"results":[{results}]}}"#,
            c = tags.len()
        )
    }

    #[tokio::test]
    async fn fetch_tags_returns_tag_names() {
        let server = MockServer::start().await;
        let body = tags_page(&["22.04", "22.04.1", "22.04.2"], None);
        Mock::given(method("GET"))
            .and(path("/v2/repositories/library/ubuntu/tags"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let dhr = DockerHubRepo {
            namespace: "library".into(),
            repo: "ubuntu".into(),
        };
        let tags = fetch_tags(&http, &dhr, &server.uri()).await.unwrap();
        assert_eq!(tags, vec!["22.04", "22.04.1", "22.04.2"]);
    }

    #[tokio::test]
    async fn fetch_updates_concurrent_detects_update() {
        let server = MockServer::start().await;
        let body = tags_page(&["22.04", "22.04.1", "22.04.2", "23.10"], None);
        Mock::given(method("GET"))
            .and(path("/v2/repositories/library/ubuntu/tags"))
            .respond_with(ResponseTemplate::new(200).set_body_string(body))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let deps = vec![DockerDepInput {
            dep_name: "ubuntu:22.04".into(),
            image: "ubuntu".into(),
            tag: "22.04".into(),
        }];
        let results = fetch_updates_concurrent(&http, &deps, &server.uri(), 10).await;
        assert_eq!(results.len(), 1);
        let s = results[0].summary.as_ref().unwrap();
        assert!(s.update_available);
        assert_eq!(s.latest.as_deref(), Some("23.10"));
    }

    #[tokio::test]
    async fn non_docker_hub_image_returns_error() {
        let http = HttpClient::new().unwrap();
        let deps = vec![DockerDepInput {
            dep_name: "ghcr.io/owner/image:1.0".into(),
            image: "ghcr.io/owner/image".into(),
            tag: "1.0".into(),
        }];
        // No mock server — the error should come from parse_image_name, not the network.
        let results = fetch_updates_concurrent(&http, &deps, "https://hub.docker.com", 10).await;
        assert_eq!(results.len(), 1);
        assert!(matches!(
            &results[0].summary,
            Err(DockerHubError::NonDockerHub(_))
        ));
    }
}
