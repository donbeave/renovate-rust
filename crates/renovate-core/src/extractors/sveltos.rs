//! Sveltos `ClusterProfile`/`Profile` Helm chart extractor.
//!
//! Extracts Helm chart references from Sveltos Kubernetes GitOps manifests.
//!
//! Renovate reference:
//! - `lib/modules/manager/sveltos/extract.ts`
//! - Default patterns: `[]` (user-configured). We add `sveltos/` convention.
//! - Datasources: `helm`
//!
//! ## File format
//!
//! ```yaml
//! apiVersion: config.projectsveltos.io/v1beta1
//! kind: ClusterProfile
//! spec:
//!   helmCharts:
//!   - repositoryURL: https://charts.helm.sh/stable
//!     repositoryName: stable
//!     chartName: redis
//!     chartVersion: 10.5.7
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// A single Sveltos Helm chart dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SveltosDep {
    pub chart_name: String,
    pub current_value: String,
    pub registry_url: String,
}

// ── Regexes ───────────────────────────────────────────────────────────────────

static SVELTOS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"apiVersion:\s*(config|lib)\.projectsveltos\.io/").unwrap());

static HELM_CHARTS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s+helmCharts:\s*$").unwrap());

static KV_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\s+(\w+):\s+(?:"([^"]+)"|'([^']+)'|(\S+))"#).unwrap());

fn kv_val<'a>(cap: &'a regex::Captures) -> &'a str {
    cap.get(2)
        .or_else(|| cap.get(3))
        .or_else(|| cap.get(4))
        .map(|m| m.as_str())
        .unwrap_or("")
}

fn leading_spaces(line: &str) -> usize {
    line.len() - line.trim_start_matches(' ').len()
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Extract Sveltos Helm chart deps from a manifest file.
pub fn extract(content: &str) -> Vec<SveltosDep> {
    if !SVELTOS_RE.is_match(content) {
        return Vec::new();
    }

    let mut deps = Vec::new();

    for doc in content.split("\n---") {
        if !SVELTOS_RE.is_match(doc) {
            continue;
        }
        deps.extend(extract_from_doc(doc));
    }

    deps
}

fn extract_from_doc(doc: &str) -> Vec<SveltosDep> {
    let mut deps = Vec::new();
    let mut in_helm_charts = false;
    let mut repo_url: Option<String> = None;
    let mut chart_name: Option<String> = None;
    let mut chart_version: Option<String> = None;

    for line in doc.lines() {
        let stripped = match line.find(" #") {
            Some(p) => &line[..p],
            None => line,
        };
        if stripped.trim().is_empty() {
            continue;
        }

        let indent = leading_spaces(stripped);

        if HELM_CHARTS_RE.is_match(stripped) {
            flush_chart(
                &mut repo_url,
                &mut chart_name,
                &mut chart_version,
                &mut deps,
            );
            in_helm_charts = true;
            continue;
        }

        if in_helm_charts {
            // Exit if we return to top-level indent without being a list item.
            if indent <= 2 && !stripped.trim_start().starts_with('-') {
                flush_chart(
                    &mut repo_url,
                    &mut chart_name,
                    &mut chart_version,
                    &mut deps,
                );
                in_helm_charts = false;
                continue;
            }

            let trimmed = stripped.trim_start();

            // New list item — flush previous chart.
            if let Some(rest) = trimmed.strip_prefix("- ") {
                flush_chart(
                    &mut repo_url,
                    &mut chart_name,
                    &mut chart_version,
                    &mut deps,
                );
                // Parse inline KV if present.
                if let Some(cap) = KV_RE.captures(&format!("    {rest}")) {
                    set_field(
                        &cap[1],
                        kv_val(&cap),
                        &mut repo_url,
                        &mut chart_name,
                        &mut chart_version,
                    );
                }
                continue;
            }

            if let Some(cap) = KV_RE.captures(stripped) {
                set_field(
                    &cap[1],
                    kv_val(&cap),
                    &mut repo_url,
                    &mut chart_name,
                    &mut chart_version,
                );
            }
        }
    }

    flush_chart(
        &mut repo_url,
        &mut chart_name,
        &mut chart_version,
        &mut deps,
    );
    deps
}

fn set_field(
    key: &str,
    val: &str,
    repo_url: &mut Option<String>,
    chart_name: &mut Option<String>,
    chart_version: &mut Option<String>,
) {
    match key {
        "repositoryURL" => *repo_url = Some(val.to_owned()),
        "chartName" => *chart_name = Some(val.to_owned()),
        "chartVersion" => *chart_version = Some(val.to_owned()),
        _ => {}
    }
}

fn flush_chart(
    repo_url: &mut Option<String>,
    chart_name: &mut Option<String>,
    chart_version: &mut Option<String>,
    deps: &mut Vec<SveltosDep>,
) {
    if let (Some(url), Some(name), Some(ver)) =
        (repo_url.take(), chart_name.take(), chart_version.take())
    {
        if !url.is_empty() && !name.is_empty() && !ver.is_empty() {
            deps.push(SveltosDep {
                chart_name: name,
                current_value: ver,
                registry_url: url,
            });
        }
    } else {
        repo_url.take();
        chart_name.take();
        chart_version.take();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_helm_chart() {
        let content = r#"
apiVersion: config.projectsveltos.io/v1beta1
kind: ClusterProfile
spec:
  helmCharts:
  - repositoryURL: https://charts.helm.sh/stable
    repositoryName: stable
    chartName: redis
    chartVersion: 10.5.7
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].chart_name, "redis");
        assert_eq!(deps[0].current_value, "10.5.7");
        assert_eq!(deps[0].registry_url, "https://charts.helm.sh/stable");
    }

    #[test]
    fn extracts_multiple_charts() {
        let content = r#"
apiVersion: config.projectsveltos.io/v1beta1
kind: ClusterProfile
spec:
  helmCharts:
  - repositoryURL: https://charts.helm.sh/stable
    repositoryName: stable
    chartName: redis
    chartVersion: 10.5.7
  - repositoryURL: https://charts.bitnami.com/bitnami
    repositoryName: bitnami
    chartName: postgresql
    chartVersion: 12.2.1
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
    }

    #[test]
    fn skips_non_sveltos_files() {
        assert!(extract("apiVersion: v1\nkind: ConfigMap\n").is_empty());
    }
}
