//! Azure Pipelines Tasks datasource.
//!
//! Renovate reference: `lib/modules/datasource/azure-pipelines-tasks/index.ts`
//!
//! ## GitHub fallback path
//! `GET https://raw.githubusercontent.com/renovatebot/azure-devops-marketplace/main/azure-pipelines-builtin-tasks.json`
//! `GET .../azure-pipelines-marketplace-tasks.json`
//! Response: `Record<string, string[]>` — task name → version list (case-insensitive lookup).
//!
//! ## Azure org path (requires auth token + platform=azure config)
//! `GET {endpoint}/_apis/distributedtask/tasks/`
//! Response: `{ value: [{ id, name, version: { major, minor, patch }, deprecated, releaseNotes, serverOwned, contributionIdentifier }] }`

use std::collections::HashMap;

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

pub const BUILTIN_URL: &str = "https://raw.githubusercontent.com/renovatebot/azure-devops-marketplace/main/azure-pipelines-builtin-tasks.json";
pub const MARKETPLACE_URL: &str = "https://raw.githubusercontent.com/renovatebot/azure-devops-marketplace/main/azure-pipelines-marketplace-tasks.json";
pub const DATASOURCE_ID: &str = "azure-pipelines-tasks";

#[derive(Debug, Error)]
pub enum AzureTasksError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
}

#[derive(Debug, Clone)]
pub struct AzureTaskRelease {
    pub version: String,
    pub is_deprecated: bool,
    pub changelog_url: Option<String>,
    pub changelog_content: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AzureTasksResult {
    pub releases: Vec<AzureTaskRelease>,
}

// ── API types ──────────────────────────────────────────────────────────────

type FallbackMap = HashMap<String, Vec<String>>;

#[derive(Debug, Deserialize)]
struct OrgTaskVersion {
    major: u32,
    minor: u32,
    patch: u32,
}

#[derive(Debug, Deserialize)]
struct OrgTask {
    id: String,
    name: String,
    #[serde(default)]
    deprecated: bool,
    #[serde(rename = "releaseNotes")]
    release_notes: Option<String>,
    #[serde(rename = "serverOwned")]
    #[expect(dead_code, reason = "Deserialized for API parity; not currently read")]
    server_owned: Option<bool>,
    version: Option<OrgTaskVersion>,
    #[serde(rename = "contributionIdentifier")]
    contribution_identifier: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OrgResponse {
    value: Vec<OrgTask>,
}

const CHANGELOG_URL: &str = "https://github.com/microsoft/azure-pipelines-tasks/releases";

/// Fetch releases from GitHub fallback JSON files (builtin + marketplace).
///
/// Returns `Ok(None)` if the task is not found in either map.
/// All HTTP errors → `Err(...)`.
pub async fn fetch_releases(
    builtin_url: &str,
    marketplace_url: &str,
    package_name: &str,
    http: &HttpClient,
) -> Result<Option<AzureTasksResult>, AzureTasksError> {
    let key = package_name.to_lowercase();

    // Try builtin map first
    let builtin: FallbackMap = http.get_json(builtin_url).await?;
    if let Some(versions) = builtin.get(&key) {
        let releases = versions
            .iter()
            .map(|v| AzureTaskRelease {
                version: v.clone(),
                is_deprecated: false,
                changelog_url: None,
                changelog_content: None,
            })
            .collect();
        return Ok(Some(AzureTasksResult { releases }));
    }

    // Try marketplace map
    let marketplace: FallbackMap = http.get_json(marketplace_url).await?;
    if let Some(versions) = marketplace.get(&key) {
        let releases = versions
            .iter()
            .map(|v| AzureTaskRelease {
                version: v.clone(),
                is_deprecated: false,
                changelog_url: None,
                changelog_content: None,
            })
            .collect();
        return Ok(Some(AzureTasksResult { releases }));
    }

    Ok(None)
}

/// Fetch releases from the Azure organization API endpoint.
///
/// `package_name` can be:
/// - A task name (e.g. `AzurePowerShell`)
/// - A task UUID (e.g. `5d437bf5-f193-4449-b531-c4c69eebaa48`)
/// - `{contributionIdentifier}.{id}` (e.g. `gittools.gittools.open-gitreleasemanager-task.5d437bf5-...`)
/// - `{contributionIdentifier}.{name}` (e.g. `gittools.gittools.open-gitreleasemanager-task.gitreleasemanager/open`)
pub async fn fetch_releases_org(
    org_url: &str,
    package_name: &str,
    auth_token: &str,
    http: &HttpClient,
) -> Result<Option<AzureTasksResult>, AzureTasksError> {
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    let auth = STANDARD.encode(format!("renovate:{auth_token}"));

    let text = match http
        .get_raw_with_accept_and_header(
            org_url,
            "application/json",
            "authorization",
            &format!("Basic {auth}"),
        )
        .await
    {
        Ok(v) => v,
        Err(e) => return Err(AzureTasksError::Http(e)),
    };

    let org: OrgResponse = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    let matching: Vec<&OrgTask> = org
        .value
        .iter()
        .filter(|task| {
            let ci = task.contribution_identifier.as_deref().unwrap_or("");
            task.id == package_name
                || task.name == package_name
                || (!ci.is_empty() && format!("{ci}.{}", task.id) == package_name)
                || (!ci.is_empty() && format!("{ci}.{}", task.name) == package_name)
        })
        .collect();

    if matching.is_empty() {
        return Ok(None);
    }

    let mut sorted = matching;
    sorted.sort_by(|a, b| {
        let av = a.version.as_ref();
        let bv = b.version.as_ref();
        match (av, bv) {
            (Some(a), Some(b)) => (a.major, a.minor, a.patch).cmp(&(b.major, b.minor, b.patch)),
            (Some(_), None) => std::cmp::Ordering::Greater,
            (None, Some(_)) => std::cmp::Ordering::Less,
            (None, None) => std::cmp::Ordering::Equal,
        }
    });

    let releases = sorted
        .into_iter()
        .filter_map(|task| {
            let ver = task.version.as_ref()?;
            Some(AzureTaskRelease {
                version: format!("{}.{}.{}", ver.major, ver.minor, ver.patch),
                is_deprecated: task.deprecated,
                changelog_url: Some(CHANGELOG_URL.to_owned()),
                changelog_content: task.release_notes.clone(),
            })
        })
        .collect();

    Ok(Some(AzureTasksResult { releases }))
}

/// Update summary used by pipeline.
#[derive(Debug, Clone)]
pub struct AzureTaskUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

/// Fetch latest version (pipeline helper, GitHub fallback path).
pub async fn fetch_latest(
    http: &HttpClient,
    task_name: &str,
    current_value: &str,
) -> Result<AzureTaskUpdateSummary, AzureTasksError> {
    let result = fetch_releases(BUILTIN_URL, MARKETPLACE_URL, task_name, http).await?;
    let latest = result.and_then(|r| {
        let mut versions: Vec<&str> = r.releases.iter().map(|rel| rel.version.as_str()).collect();
        versions.sort_by(|a, b| cmp_version(a, b));
        versions.last().map(|s| s.to_string())
    });
    let update_available = latest
        .as_deref()
        .map(|l| l != current_value)
        .unwrap_or(false);
    Ok(AzureTaskUpdateSummary {
        current_value: current_value.to_owned(),
        latest,
        update_available,
    })
}

/// Compare two version strings numerically by components.
pub fn cmp_version(a: &str, b: &str) -> std::cmp::Ordering {
    let parse_parts = |s: &str| {
        s.split('.')
            .map(|p| p.parse::<u64>().unwrap_or(0))
            .collect::<Vec<_>>()
    };
    let av = parse_parts(a);
    let bv = parse_parts(b);
    for i in 0..av.len().max(bv.len()) {
        let ai = av.get(i).copied().unwrap_or(0);
        let bi = bv.get(i).copied().unwrap_or(0);
        match ai.cmp(&bi) {
            std::cmp::Ordering::Equal => continue,
            other => return other,
        }
    }
    std::cmp::Ordering::Equal
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    const TASKS_JSON: &str = include_str!(
        "../../../../../renovate/lib/modules/datasource/azure-pipelines-tasks/__fixtures__/tasks.json"
    );

    // Ported: "returns null for unknown task" — datasource/azure-pipelines-tasks/index.spec.ts line 21
    #[tokio::test]
    async fn returns_null_for_unknown_task() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/builtin-tasks.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/marketplace-tasks.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let builtin_url = format!("{}/builtin-tasks.json", server.uri());
        let marketplace_url = format!("{}/marketplace-tasks.json", server.uri());
        let result = fetch_releases(&builtin_url, &marketplace_url, "unknown", &http)
            .await
            .unwrap();
        assert!(result.is_none());
    }

    // Ported: "supports built-in tasks" — datasource/azure-pipelines-tasks/index.spec.ts line 36
    #[tokio::test]
    async fn supports_builtin_tasks() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/builtin-tasks.json"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(r#"{"automatedanalysis":["0.171.0","0.198.0"]}"#),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let builtin_url = format!("{}/builtin-tasks.json", server.uri());
        let result = fetch_releases(&builtin_url, "http://invalid", "AutomatedAnalysis", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 2);
        assert_eq!(result.releases[0].version, "0.171.0");
        assert_eq!(result.releases[1].version, "0.198.0");
    }

    // Ported: "supports marketplace tasks" — datasource/azure-pipelines-tasks/index.spec.ts line 49
    #[tokio::test]
    async fn supports_marketplace_tasks() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/builtin-tasks.json"))
            .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/marketplace-tasks.json"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(r#"{"automatedanalysis-marketplace":["0.171.0","0.198.0"]}"#),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let builtin_url = format!("{}/builtin-tasks.json", server.uri());
        let marketplace_url = format!("{}/marketplace-tasks.json", server.uri());
        let result = fetch_releases(
            &builtin_url,
            &marketplace_url,
            "AutomatedAnalysis-Marketplace",
            &http,
        )
        .await
        .unwrap()
        .unwrap();

        assert_eq!(result.releases.len(), 2);
        assert_eq!(result.releases[0].version, "0.171.0");
        assert_eq!(result.releases[1].version, "0.198.0");
    }

    // Ported: "is case insensitive" — datasource/azure-pipelines-tasks/index.spec.ts line 64
    #[tokio::test]
    async fn is_case_insensitive() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/builtin-tasks.json"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(r#"{"automatedanalysis":["0.171.0","0.198.0"]}"#),
            )
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let builtin_url = format!("{}/builtin-tasks.json", server.uri());
        let result = fetch_releases(&builtin_url, "http://invalid", "automatedanalysis", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 2);
    }

    // Ported: "returns organization task with single version" — datasource/azure-pipelines-tasks/index.spec.ts line 77
    #[tokio::test]
    async fn returns_org_task_single_version() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/_apis/distributedtask/tasks/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(TASKS_JSON))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let url = format!("{}/_apis/distributedtask/tasks/", server.uri());
        let result = fetch_releases_org(&url, "AzurePowerShell", "123test", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 1);
        assert_eq!(result.releases[0].version, "5.248.3");
        assert_eq!(
            result.releases[0].changelog_content.as_deref(),
            Some("Added support for Az Module and cross platform agents.")
        );
        assert_eq!(
            result.releases[0].changelog_url.as_deref(),
            Some(CHANGELOG_URL)
        );
    }

    // Ported: "identifies task based on task id" — datasource/azure-pipelines-tasks/index.spec.ts line 112
    #[tokio::test]
    async fn identifies_task_by_id() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/_apis/distributedtask/tasks/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(TASKS_JSON))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let url = format!("{}/_apis/distributedtask/tasks/", server.uri());
        let result = fetch_releases_org(
            &url,
            "5d437bf5-f193-4449-b531-c4c69eebaa48",
            "123test",
            &http,
        )
        .await
        .unwrap()
        .unwrap();

        assert_eq!(result.releases.len(), 1);
        assert_eq!(result.releases[0].version, "3.1.11");
    }

    // Ported: "identifies task based on contributionIdentifier and id" — datasource/azure-pipelines-tasks/index.spec.ts line 134
    #[tokio::test]
    async fn identifies_task_by_contribution_identifier_and_id() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/_apis/distributedtask/tasks/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(TASKS_JSON))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let url = format!("{}/_apis/distributedtask/tasks/", server.uri());
        let result = fetch_releases_org(
            &url,
            "gittools.gittools.open-gitreleasemanager-task.5d437bf5-f193-4449-b531-c4c69eebaa48",
            "123test",
            &http,
        )
        .await
        .unwrap()
        .unwrap();

        assert_eq!(result.releases.len(), 1);
        assert_eq!(result.releases[0].version, "3.1.11");
    }

    // Ported: "identifies task based on contributionIdentifier and name" — datasource/azure-pipelines-tasks/index.spec.ts line 157
    #[tokio::test]
    async fn identifies_task_by_contribution_identifier_and_name() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/_apis/distributedtask/tasks/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(TASKS_JSON))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let url = format!("{}/_apis/distributedtask/tasks/", server.uri());
        let result = fetch_releases_org(
            &url,
            "gittools.gittools.open-gitreleasemanager-task.gitreleasemanager/open",
            "123test",
            &http,
        )
        .await
        .unwrap()
        .unwrap();

        assert_eq!(result.releases.len(), 1);
        assert_eq!(result.releases[0].version, "3.1.11");
    }

    // Ported: "returns organization task with multiple versions" — datasource/azure-pipelines-tasks/index.spec.ts line 180
    #[tokio::test]
    async fn returns_org_task_multiple_versions() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/_apis/distributedtask/tasks/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(TASKS_JSON))
            .mount(&server)
            .await;

        let http = HttpClient::new().unwrap();
        let url = format!("{}/_apis/distributedtask/tasks/", server.uri());
        let result = fetch_releases_org(&url, "PowerShell", "123test", &http)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(result.releases.len(), 2);
        assert_eq!(result.releases[0].version, "1.2.3");
        assert!(result.releases[0].is_deprecated);
        assert_eq!(result.releases[1].version, "2.247.1");
        assert!(!result.releases[1].is_deprecated);
        assert_eq!(
            result.releases[1].changelog_content.as_deref(),
            Some("Script task consistency. Added support for macOS and Linux.")
        );
    }

    #[test]
    fn cmp_version_basic() {
        use std::cmp::Ordering::*;
        assert_eq!(cmp_version("2.0.0", "1.9.9"), Greater);
        assert_eq!(cmp_version("1.0.0", "1.0.0"), Equal);
        assert_eq!(cmp_version("1.0.0", "2.0.0"), Less);
        assert_eq!(cmp_version("2.198.0", "2.20.0"), Greater);
    }

    // Ported: "when versions is $a" — datasource/azure-pipelines-tasks/index.spec.ts line 222
    #[test]
    fn cmp_version_sorts_semver_cases() {
        for (input, expected) in [
            (vec![], vec![]),
            (vec![""], vec![""]),
            (vec!["", ""], vec!["", ""]),
            (vec!["1.0.0"], vec!["1.0.0"]),
            (
                vec!["1.0.1", "1.1.0", "1.0.0"],
                vec!["1.0.0", "1.0.1", "1.1.0"],
            ),
        ] {
            let mut versions = input;
            versions.sort_by(|a, b| cmp_version(a, b));
            assert_eq!(versions, expected);
        }
    }
}
