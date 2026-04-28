//! Helmsman Desired State File (DSF) extractor.
//!
//! Parses Helmsman YAML files and extracts Helm chart references for version
//! tracking. Helmsman DSF maps logical app names to Helm charts with versions.
//!
//! Renovate reference:
//! - `lib/modules/manager/helmsman/extract.ts`
//! - Default patterns: `[]` (user-configured — we add common ones)
//! - Datasource: `helm`
//!
//! ## File format
//!
//! ```yaml
//! helmRepos:
//!   stable: "https://charts.helm.sh/stable"
//!   bitnami: "https://charts.bitnami.com/bitnami"
//!
//! apps:
//!   redis:
//!     chart: "stable/redis"
//!     version: "10.5.7"
//!   postgres:
//!     chart: "bitnami/postgresql"
//!     version: "12.2.1"
//! ```

use std::collections::HashMap;
use std::sync::LazyLock;

use regex::Regex;

/// Skip reason for a Helmsman dep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HelmsmanSkipReason {
    /// No version field in app spec.
    UnspecifiedVersion,
    /// Chart field missing or unparseable.
    InvalidChart,
    /// Repository alias could not be resolved to a URL.
    NoRepository,
}

/// A single Helmsman app dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HelmsmanDep {
    /// App name / chart name used for display.
    pub dep_name: String,
    /// Resolved chart name within the registry (e.g. `redis`).
    pub chart_name: String,
    /// The Helm repository URL.
    pub registry_url: String,
    /// Version constraint.
    pub current_value: String,
    pub skip_reason: Option<HelmsmanSkipReason>,
}

// ── Regexes ───────────────────────────────────────────────────────────────────

/// Top-level section key: `helmRepos:` or `apps:` at indent 0.
static SECTION_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(helmRepos|apps):\s*$").unwrap());

/// Key-value line with optional quoted value: `  key: "value"` or `  key: value`.
static KV_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r##"^\s+(\S+):\s*(?:"([^"]*)"|([\S][^\s#]*?))\s*(?:#.*)?$"##).unwrap()
});

/// Count leading spaces.
fn leading_spaces(line: &str) -> usize {
    line.len() - line.trim_start_matches(' ').len()
}

/// Extract the value from a KV line (quoted or unquoted).
fn kv_value<'a>(cap: &regex::Captures<'a>) -> &'a str {
    cap.get(2)
        .or_else(|| cap.get(3))
        .map(|m| m.as_str())
        .unwrap_or("")
}

// ── State machine ─────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq)]
enum Section {
    None,
    HelmRepos,
    Apps,
}

/// Extract Helmsman DSF dependencies from file content.
pub fn extract(content: &str) -> Vec<HelmsmanDep> {
    let mut helm_repos: HashMap<String, String> = HashMap::new();
    let mut apps: Vec<(String, Option<String>, Option<String>)> = Vec::new(); // (name, chart, version)

    let mut section = Section::None;
    let mut current_app: Option<String> = None;
    let mut current_chart: Option<String> = None;
    let mut current_version: Option<String> = None;

    for raw_line in content.lines() {
        // Strip trailing comments for structural analysis.
        let line = match raw_line.find(" #") {
            Some(pos) => &raw_line[..pos],
            None => raw_line,
        };

        if line.trim().is_empty() {
            continue;
        }

        let indent = leading_spaces(line);
        let trimmed = line.trim_start();

        // Top-level section detection.
        if indent == 0 {
            if let Some(cap) = SECTION_RE.captures(trimmed) {
                // Flush current app before switching sections.
                if let Some(name) = current_app.take() {
                    apps.push((name, current_chart.take(), current_version.take()));
                } else {
                    current_chart = None;
                    current_version = None;
                }
                section = if &cap[1] == "helmRepos" {
                    Section::HelmRepos
                } else {
                    Section::Apps
                };
            } else {
                // Any other top-level key ends current section.
                if let Some(name) = current_app.take() {
                    apps.push((name, current_chart.take(), current_version.take()));
                }
                section = Section::None;
            }
            continue;
        }

        match section {
            Section::HelmRepos => {
                // `  alias: "url"` at indent 2
                if indent == 2 {
                    if let Some(cap) = KV_RE.captures(line) {
                        let alias = cap[1].to_owned();
                        let url = kv_value(&cap).to_owned();
                        if !url.is_empty() {
                            helm_repos.insert(alias, url);
                        }
                    }
                }
            }
            Section::Apps => {
                if indent == 2 {
                    // App-level key: `  app-name:` or `  app-name: {...}`
                    if let Some(cap) = KV_RE.captures(line) {
                        // Flush previous app.
                        if let Some(name) = current_app.take() {
                            apps.push((name, current_chart.take(), current_version.take()));
                        } else {
                            current_chart = None;
                            current_version = None;
                        }
                        current_app = Some(cap[1].to_owned());
                    } else if !trimmed.starts_with('-') {
                        // `  appname:` without a value
                        if let Some(name) = current_app.take() {
                            apps.push((name, current_chart.take(), current_version.take()));
                        }
                        let app_name = trimmed.trim_end_matches(':').to_owned();
                        if !app_name.is_empty() {
                            current_app = Some(app_name);
                        }
                    }
                } else if indent == 4 && current_app.is_some() {
                    // App fields: `    chart: "stable/redis"`, `    version: "1.0.0"`.
                    if let Some(cap) = KV_RE.captures(line) {
                        let key = &cap[1];
                        let val = kv_value(&cap).to_owned();
                        match key {
                            "chart" => current_chart = Some(val),
                            "version" => current_version = Some(val),
                            _ => {}
                        }
                    }
                }
            }
            Section::None => {}
        }
    }

    // Flush last app.
    if let Some(name) = current_app {
        apps.push((name, current_chart, current_version));
    }

    // Build deps from app + repo resolution.
    let mut deps = Vec::new();
    for (app_name, chart_opt, version_opt) in apps {
        let Some(chart) = chart_opt else {
            deps.push(HelmsmanDep {
                dep_name: app_name.clone(),
                chart_name: String::new(),
                registry_url: String::new(),
                current_value: String::new(),
                skip_reason: Some(HelmsmanSkipReason::InvalidChart),
            });
            continue;
        };

        let Some(version) = version_opt else {
            deps.push(HelmsmanDep {
                dep_name: app_name.clone(),
                chart_name: chart.clone(),
                registry_url: String::new(),
                current_value: String::new(),
                skip_reason: Some(HelmsmanSkipReason::UnspecifiedVersion),
            });
            continue;
        };

        // chart format: "alias/chart-name"
        let (alias, chart_name) = match chart.split_once('/') {
            Some((a, c)) => (a, c),
            None => {
                deps.push(HelmsmanDep {
                    dep_name: app_name.clone(),
                    chart_name: chart.clone(),
                    registry_url: String::new(),
                    current_value: version.clone(),
                    skip_reason: Some(HelmsmanSkipReason::InvalidChart),
                });
                continue;
            }
        };

        let registry_url = match helm_repos.get(alias) {
            Some(url) => url.clone(),
            None => {
                deps.push(HelmsmanDep {
                    dep_name: app_name.clone(),
                    chart_name: chart_name.to_owned(),
                    registry_url: String::new(),
                    current_value: version.clone(),
                    skip_reason: Some(HelmsmanSkipReason::NoRepository),
                });
                continue;
            }
        };

        deps.push(HelmsmanDep {
            dep_name: app_name.clone(),
            chart_name: chart_name.to_owned(),
            registry_url,
            current_value: version,
            skip_reason: None,
        });
    }

    deps
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
helmRepos:
  stable: "https://charts.helm.sh/stable"
  bitnami: "https://charts.bitnami.com/bitnami"

apps:
  redis:
    chart: "stable/redis"
    version: "10.5.7"
  postgres:
    chart: "bitnami/postgresql"
    version: "12.2.1"
  no-version:
    chart: "stable/nginx"
  unknown-repo:
    chart: "myrepo/app"
    version: "1.0.0"
"#;

    #[test]
    fn extracts_helm_deps() {
        let deps = extract(SAMPLE);
        let actionable: Vec<_> = deps.iter().filter(|d| d.skip_reason.is_none()).collect();
        assert_eq!(actionable.len(), 2);
        assert!(
            actionable
                .iter()
                .any(|d| d.chart_name == "redis" && d.current_value == "10.5.7")
        );
        assert!(actionable.iter().any(|d| d.chart_name == "postgresql"
            && d.current_value == "12.2.1"
            && d.registry_url == "https://charts.bitnami.com/bitnami"));
    }

    #[test]
    fn skips_missing_version() {
        let deps = extract(SAMPLE);
        let no_ver: Vec<_> = deps
            .iter()
            .filter(|d| d.skip_reason == Some(HelmsmanSkipReason::UnspecifiedVersion))
            .collect();
        assert!(!no_ver.is_empty());
    }

    #[test]
    fn skips_unknown_repo() {
        let deps = extract(SAMPLE);
        let no_repo: Vec<_> = deps
            .iter()
            .filter(|d| d.skip_reason == Some(HelmsmanSkipReason::NoRepository))
            .collect();
        assert!(!no_repo.is_empty());
        assert!(no_repo.iter().any(|d| d.dep_name == "unknown-repo"));
    }

    #[test]
    fn empty_file_returns_empty() {
        assert!(extract("").is_empty());
    }
}
