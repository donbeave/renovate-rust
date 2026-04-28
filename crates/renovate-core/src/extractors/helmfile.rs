//! Helmfile (`helmfile.yaml`, `helmfile.d/*.yaml`) extractor.
//!
//! Extracts Helm chart release dependencies from Helmfile manifests.
//! Releases reference Helm charts with an optional repository alias resolved
//! from the `repositories:` section of the same file.
//!
//! Renovate reference:
//! - `lib/modules/manager/helmfile/extract.ts`
//! - `lib/modules/manager/helmfile/schema.ts`
//! - Patterns: `/(^|/)helmfile\.ya?ml(?:\.gotmpl)?$/`
//!   and `/(^|/)helmfile\.d/.+\.ya?ml(?:\.gotmpl)?$/`
//!
//! ## Supported form
//!
//! ```yaml
//! repositories:
//! - name: stable
//!   url: https://charts.helm.sh/stable
//! - name: bitnami
//!   url: https://charts.bitnami.com/bitnami
//!
//! releases:
//! - name: redis
//!   chart: bitnami/redis
//!   version: "17.0.0"
//! - name: nginx
//!   chart: oci://registry.example.com/nginx
//!   version: "1.2.3"
//! ```

use std::collections::HashMap;

use crate::extractors::helm::{HelmExtractedDep, HelmSkipReason, STABLE_REPO};

/// Parse all chart dependencies from a Helmfile YAML.
pub fn extract(content: &str) -> Vec<HelmExtractedDep> {
    // ── Pass 1: collect repositories ─────────────────────────────────────────
    let repo_map = collect_repositories(content);

    // ── Pass 2: collect and resolve releases ──────────────────────────────────
    let raw_releases = collect_releases(content);

    raw_releases
        .into_iter()
        .filter_map(|(chart_ref, version)| resolve_release(&chart_ref, &version, &repo_map))
        .collect()
}

// ── Internal types ────────────────────────────────────────────────────────────

struct RepoEntry {
    url: String,
    oci: bool,
}

// ── Pass 1: repositories ──────────────────────────────────────────────────────

fn collect_repositories(content: &str) -> HashMap<String, RepoEntry> {
    let mut map: HashMap<String, RepoEntry> = HashMap::new();
    let mut in_repos = false;
    let mut cur_name: Option<String> = None;
    let mut cur_url: Option<String> = None;
    let mut cur_oci = false;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        if line.trim().is_empty() {
            continue;
        }
        let trimmed = line.trim_start();
        let indent = leading_spaces(line);

        if indent == 0 && !trimmed.starts_with('-') {
            // Flush pending repo item on section exit
            if in_repos {
                flush_repo(&mut map, &mut cur_name, &mut cur_url, &mut cur_oci);
            }
            in_repos = trimmed == "repositories:";
            continue;
        }

        if !in_repos {
            continue;
        }

        // New list item — may appear at indent 0 or 2 depending on YAML style.
        if trimmed.starts_with("- ") {
            flush_repo(&mut map, &mut cur_name, &mut cur_url, &mut cur_oci);
            let rest = trimmed.strip_prefix("- ").unwrap_or("");
            if let Some(v) = strip_key(rest, "name") {
                cur_name = Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
            }
            continue;
        }

        // Key-value fields inside the current item (any positive indent).
        if indent > 0 {
            if let Some(v) = strip_key(trimmed, "name") {
                cur_name = Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
            } else if let Some(v) = strip_key(trimmed, "url") {
                cur_url = Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
            } else if let Some(v) = strip_key(trimmed, "oci") {
                cur_oci = v.trim() == "true";
            }
        }
    }
    // Flush last item
    if in_repos {
        flush_repo(&mut map, &mut cur_name, &mut cur_url, &mut cur_oci);
    }

    map
}

fn flush_repo(
    map: &mut HashMap<String, RepoEntry>,
    name: &mut Option<String>,
    url: &mut Option<String>,
    oci: &mut bool,
) {
    if let (Some(n), Some(u)) = (name.take(), url.take()) {
        map.insert(n, RepoEntry { url: u, oci: *oci });
    }
    *oci = false;
}

// ── Pass 2: releases ──────────────────────────────────────────────────────────

fn collect_releases(content: &str) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    let mut in_releases = false;
    let mut cur_chart: Option<String> = None;
    let mut cur_version: Option<String> = None;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        if line.trim().is_empty() {
            continue;
        }
        let trimmed = line.trim_start();
        let indent = leading_spaces(line);

        if indent == 0 && !trimmed.starts_with('-') {
            if in_releases {
                flush_release(&mut out, &mut cur_chart, &mut cur_version);
            }
            in_releases = trimmed == "releases:";
            continue;
        }

        if !in_releases {
            continue;
        }

        if trimmed.starts_with("- ") {
            flush_release(&mut out, &mut cur_chart, &mut cur_version);
            let rest = trimmed.strip_prefix("- ").unwrap_or("");
            if let Some(v) = strip_key(rest, "chart") {
                cur_chart = Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
            }
            continue;
        }

        if indent > 0 {
            if let Some(v) = strip_key(trimmed, "chart") {
                cur_chart = Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
            } else if let Some(v) = strip_key(trimmed, "version") {
                cur_version = Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
            }
        }
    }
    if in_releases {
        flush_release(&mut out, &mut cur_chart, &mut cur_version);
    }

    out
}

fn flush_release(
    out: &mut Vec<(String, String)>,
    chart: &mut Option<String>,
    version: &mut Option<String>,
) {
    if let (Some(c), Some(v)) = (chart.take(), version.take()) {
        out.push((c, v));
    }
    chart.take();
    version.take();
}

// ── Resolution ────────────────────────────────────────────────────────────────

fn resolve_release(
    chart_ref: &str,
    version: &str,
    repos: &HashMap<String, RepoEntry>,
) -> Option<HelmExtractedDep> {
    // Local path → skip
    if chart_ref.starts_with("./") || chart_ref.starts_with("../") || chart_ref.starts_with('/') {
        return None;
    }

    // Template expression → skip (Helm Go templates / helmfile templates)
    if chart_ref.contains("{{") {
        return Some(HelmExtractedDep {
            name: chart_ref.to_owned(),
            current_value: version.to_owned(),
            repository: String::new(),
            skip_reason: Some(HelmSkipReason::UnresolvableAlias),
        });
    }

    // OCI registry: `oci://registry/chart`
    if let Some(oci_path) = chart_ref.strip_prefix("oci://") {
        return Some(HelmExtractedDep {
            name: oci_path.to_owned(),
            current_value: version.to_owned(),
            repository: String::new(),
            skip_reason: Some(HelmSkipReason::OciRegistry),
        });
    }

    // `repo-alias/chart-name` form
    if let Some(slash) = chart_ref.find('/') {
        let alias = &chart_ref[..slash];
        let chart_name = &chart_ref[slash + 1..];

        let repo_url = if let Some(entry) = repos.get(alias) {
            if entry.oci {
                // OCI-backed repo alias
                return Some(HelmExtractedDep {
                    name: chart_name.to_owned(),
                    current_value: version.to_owned(),
                    repository: String::new(),
                    skip_reason: Some(HelmSkipReason::OciRegistry),
                });
            }
            entry.url.clone()
        } else if alias == "stable" {
            STABLE_REPO.to_owned()
        } else {
            // Unknown alias
            return Some(HelmExtractedDep {
                name: chart_name.to_owned(),
                current_value: version.to_owned(),
                repository: String::new(),
                skip_reason: Some(HelmSkipReason::UnresolvableAlias),
            });
        };

        return Some(HelmExtractedDep {
            name: chart_name.to_owned(),
            current_value: version.to_owned(),
            repository: repo_url,
            skip_reason: None,
        });
    }

    // Plain chart name: look up as alias in repo map
    if let Some(entry) = repos.get(chart_ref) {
        if entry.oci {
            return Some(HelmExtractedDep {
                name: chart_ref.to_owned(),
                current_value: version.to_owned(),
                repository: String::new(),
                skip_reason: Some(HelmSkipReason::OciRegistry),
            });
        }
        return Some(HelmExtractedDep {
            name: chart_ref.to_owned(),
            current_value: version.to_owned(),
            repository: entry.url.clone(),
            skip_reason: None,
        });
    }

    // No registry found
    Some(HelmExtractedDep {
        name: chart_ref.to_owned(),
        current_value: version.to_owned(),
        repository: String::new(),
        skip_reason: Some(HelmSkipReason::UnresolvableAlias),
    })
}

fn leading_spaces(s: &str) -> usize {
    s.len() - s.trim_start_matches([' ', '\t']).len()
}

fn strip_key<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let prefix = format!("{key}:");
    line.strip_prefix(prefix.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_release_with_repo() {
        let content = r#"
repositories:
- name: bitnami
  url: https://charts.bitnami.com/bitnami

releases:
- name: redis
  chart: bitnami/redis
  version: "17.0.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "redis");
        assert_eq!(deps[0].current_value, "17.0.0");
        assert_eq!(deps[0].repository, "https://charts.bitnami.com/bitnami");
        assert!(deps[0].skip_reason.is_none());
    }

    #[test]
    fn stable_alias_resolved_without_repo_entry() {
        let content = r#"
releases:
- name: chart
  chart: stable/nginx
  version: "1.0.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].repository, STABLE_REPO);
        assert!(deps[0].skip_reason.is_none());
    }

    #[test]
    fn oci_chart_skipped() {
        let content = r#"
releases:
- name: app
  chart: oci://registry.example.com/app
  version: "2.0.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(HelmSkipReason::OciRegistry));
    }

    #[test]
    fn local_path_chart_excluded() {
        let content = r#"
releases:
- name: local
  chart: ./local-chart
  version: "1.0.0"
"#;
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    #[test]
    fn unknown_alias_skipped() {
        let content = r#"
releases:
- name: app
  chart: unknown-repo/my-chart
  version: "3.0.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(HelmSkipReason::UnresolvableAlias));
    }

    #[test]
    fn multiple_releases() {
        let content = r#"
repositories:
- name: stable
  url: https://charts.helm.sh/stable
- name: bitnami
  url: https://charts.bitnami.com/bitnami

releases:
- name: redis
  chart: bitnami/redis
  version: "17.0.0"
- name: nginx
  chart: stable/nginx
  version: "1.1.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(deps.iter().any(|d| d.name == "redis"));
        assert!(deps.iter().any(|d| d.name == "nginx"));
    }

    #[test]
    fn oci_backed_repo_alias_skipped() {
        let content = r#"
repositories:
- name: ecr
  url: oci://123456789.dkr.ecr.us-east-1.amazonaws.com/charts
  oci: true

releases:
- name: app
  chart: ecr/my-app
  version: "0.1.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(HelmSkipReason::OciRegistry));
    }

    #[test]
    fn template_expression_skipped() {
        let content = r#"
releases:
- name: dynamic
  chart: '{{ .Values.chart }}'
  version: "1.0.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(HelmSkipReason::UnresolvableAlias));
    }

    #[test]
    fn release_without_version_excluded() {
        let content = r#"
releases:
- name: unversioned
  chart: stable/nginx
"#;
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }
}
