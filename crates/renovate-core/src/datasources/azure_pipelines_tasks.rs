//! Azure Pipelines Tasks datasource.
//!
//! Fetches the latest task version from the Renovate-maintained mirror
//! JSON files hosted on GitHub. Two registries are tried in order:
//! 1. Built-in tasks (`azure-pipelines-builtin-tasks.json`)
//! 2. Marketplace tasks (`azure-pipelines-marketplace-tasks.json`)
//!
//! Renovate reference:
//! - `lib/modules/datasource/azure-pipelines-tasks/index.ts`
//! - Fallback URLs: `https://raw.githubusercontent.com/renovatebot/azure-devops-marketplace/main/*.json`
//!
//! The JSON format is `Record<string, string[]>` — task name → version list.
//! Lookups are case-insensitive.

use std::collections::HashMap;
use std::sync::OnceLock;

use serde::Deserialize;
use thiserror::Error;

use crate::http::HttpClient;

const BUILTIN_URL: &str = "https://raw.githubusercontent.com/renovatebot/azure-devops-marketplace/main/azure-pipelines-builtin-tasks.json";
const MARKETPLACE_URL: &str = "https://raw.githubusercontent.com/renovatebot/azure-devops-marketplace/main/azure-pipelines-marketplace-tasks.json";

/// Errors from fetching Azure Pipelines task metadata.
#[derive(Debug, Error)]
pub enum AzureTasksError {
    #[error("HTTP error: {0}")]
    Http(#[from] crate::http::HttpError),
    #[error("task not found: {0}")]
    NotFound(String),
}

/// Update summary for an Azure Pipelines task.
#[derive(Debug, Clone)]
pub struct AzureTaskUpdateSummary {
    pub current_value: String,
    pub latest: Option<String>,
    pub update_available: bool,
}

type TaskMap = HashMap<String, Vec<String>>;

// Global process-wide cache for the two JSON files.
static BUILTIN_CACHE: OnceLock<TaskMap> = OnceLock::new();
static MARKETPLACE_CACHE: OnceLock<TaskMap> = OnceLock::new();

#[derive(Deserialize)]
struct TasksResponse(HashMap<String, Vec<String>>);

async fn load_map(
    http: &HttpClient,
    url: &str,
    cache: &'static OnceLock<TaskMap>,
) -> Result<&'static TaskMap, AzureTasksError> {
    if let Some(map) = cache.get() {
        return Ok(map);
    }
    let resp: TasksResponse = http.get_json(url).await?;
    // Normalize keys to lowercase for case-insensitive lookup.
    let map: TaskMap = resp
        .0
        .into_iter()
        .map(|(k, v)| (k.to_lowercase(), v))
        .collect();
    let _ = cache.set(map);
    Ok(cache.get().unwrap())
}

/// Look up the latest version for a task name (case-insensitive).
///
/// Tries built-in tasks first, then marketplace tasks.
pub async fn fetch_latest(
    http: &HttpClient,
    task_name: &str,
    current_value: &str,
) -> Result<AzureTaskUpdateSummary, AzureTasksError> {
    let key = task_name.to_lowercase();

    let builtin = load_map(http, BUILTIN_URL, &BUILTIN_CACHE).await?;
    let versions = if let Some(v) = builtin.get(&key) {
        v
    } else {
        let marketplace = load_map(http, MARKETPLACE_URL, &MARKETPLACE_CACHE).await?;
        marketplace
            .get(&key)
            .ok_or_else(|| AzureTasksError::NotFound(task_name.to_owned()))?
    };

    let latest = versions.iter().max_by(|a, b| cmp_version(a, b)).cloned();
    let update_available = match &latest {
        Some(l) => l != current_value,
        None => false,
    };

    Ok(AzureTaskUpdateSummary {
        current_value: current_value.to_owned(),
        latest,
        update_available,
    })
}

/// Compare two version strings numerically by components.
fn cmp_version(a: &str, b: &str) -> std::cmp::Ordering {
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
    use super::*;

    #[test]
    fn cmp_version_basic() {
        use std::cmp::Ordering::*;
        assert_eq!(cmp_version("2.0.0", "1.9.9"), Greater);
        assert_eq!(cmp_version("1.0.0", "1.0.0"), Equal);
        assert_eq!(cmp_version("1.0.0", "2.0.0"), Less);
        assert_eq!(cmp_version("2.198.0", "2.20.0"), Greater);
    }
}
