//! ArgoCD Application manifest extractor.
//!
//! Extracts Helm chart and Git source references from ArgoCD `Application`
//! and `ApplicationSet` custom resource manifests.
//!
//! Renovate reference:
//! - `lib/modules/manager/argocd/extract.ts`
//! - `lib/modules/manager/argocd/util.ts`
//! - Default patterns: `[]` (user-configured). We add `(^|/)argocd/.+\.ya?ml$`.
//! - Datasources: `helm`, `git-tags`
//!
//! ## Supported source forms
//!
//! ```yaml
//! # Helm chart from registry
//! spec:
//!   source:
//!     repoURL: https://charts.helm.sh/stable
//!     chart: redis
//!     targetRevision: 10.5.7
//!
//! # Git source with tag
//! spec:
//!   source:
//!     repoURL: https://github.com/owner/repo.git
//!     targetRevision: v1.2.3
//! ```

use std::sync::LazyLock;

use regex::Regex;

/// Source type for an ArgoCD application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgocdSource {
    /// Helm chart from a registry.
    Helm {
        registry_url: String,
        chart_name: String,
    },
    /// Git repository reference (GitHub tags datasource).
    Git { repo_url: String },
    /// Unsupported or local source.
    Unsupported,
}

/// Skip reason for an ArgoCD dep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgocdSkipReason {
    /// No `targetRevision` field.
    UnspecifiedVersion,
    /// Chart or repo URL missing / unrecognized.
    InvalidConfig,
}

/// A single ArgoCD source dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgocdDep {
    pub dep_name: String,
    pub current_value: String,
    pub source: ArgocdSource,
    pub skip_reason: Option<ArgocdSkipReason>,
}

// ── Regexes ───────────────────────────────────────────────────────────────────

/// Detects ArgoCD manifests: `apiVersion: argoproj.io/`.
static ARGOCD_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"apiVersion:\s*'?"?argoproj\.io/"#).unwrap());

/// `source:` or `sources:` block start.
static SOURCE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s+sources?:\s*$").unwrap());

/// Key-value line inside a source block.
static KV_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^\s+(\w+):\s+(?:"([^"]+)"|'([^']+)'|(\S+))"#).unwrap());

fn kv_val<'a>(cap: &'a regex::Captures) -> &'a str {
    cap.get(2)
        .or_else(|| cap.get(3))
        .or_else(|| cap.get(4))
        .map(|m| m.as_str())
        .unwrap_or("")
}

// ── Parsing ───────────────────────────────────────────────────────────────────

/// Extract ArgoCD source dependencies from a manifest file.
///
/// Returns an empty Vec if the file is not an ArgoCD manifest.
pub fn extract(content: &str) -> Vec<ArgocdDep> {
    if !ARGOCD_RE.is_match(content) {
        return Vec::new();
    }

    let mut deps = Vec::new();

    // Collect sources: find `source:` blocks and read repo_url, chart, targetRevision.
    // Simple single-pass scanner that collects one source block at a time.
    let mut in_source = false;
    let mut repo_url: Option<String> = None;
    let mut chart: Option<String> = None;
    let mut target_revision: Option<String> = None;

    for line in content.lines() {
        // Strip comments.
        let line = match line.find(" #") {
            Some(pos) => &line[..pos],
            None => line,
        };

        if line.trim().is_empty() {
            continue;
        }

        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();

        // Detect `source:` or `- repoURL:` style (sources array).
        if SOURCE_RE.is_match(line) {
            // Flush previous source.
            flush_source(&mut repo_url, &mut chart, &mut target_revision, &mut deps);
            in_source = true;
            continue;
        }

        // Array item `- ` inside `sources:` also starts a new source.
        if in_source && indent >= 4 && trimmed.starts_with("- ") {
            flush_source(&mut repo_url, &mut chart, &mut target_revision, &mut deps);
            // parse inline KV if present after `- `
            let rest = &trimmed[2..];
            if let Some(cap) = KV_RE.captures(&format!("    {rest}")) {
                match &cap[1] {
                    "repoURL" => repo_url = Some(kv_val(&cap).to_owned()),
                    "chart" => chart = Some(kv_val(&cap).to_owned()),
                    "targetRevision" => target_revision = Some(kv_val(&cap).to_owned()),
                    _ => {}
                }
            }
            continue;
        }

        // Exit source block when we return to a lower indent level that is not a sub-field.
        if in_source && indent <= 2 && !trimmed.starts_with('-') {
            flush_source(&mut repo_url, &mut chart, &mut target_revision, &mut deps);
            in_source = false;
            continue;
        }

        if in_source && let Some(cap) = KV_RE.captures(line) {
            match &cap[1] {
                "repoURL" => repo_url = Some(kv_val(&cap).to_owned()),
                "chart" => chart = Some(kv_val(&cap).to_owned()),
                "targetRevision" => target_revision = Some(kv_val(&cap).to_owned()),
                _ => {}
            }
        }
    }

    // Final flush.
    flush_source(&mut repo_url, &mut chart, &mut target_revision, &mut deps);

    deps
}

fn flush_source(
    repo_url: &mut Option<String>,
    chart: &mut Option<String>,
    target_revision: &mut Option<String>,
    deps: &mut Vec<ArgocdDep>,
) {
    let url = match repo_url.take() {
        Some(u) if !u.is_empty() => u,
        _ => {
            chart.take();
            target_revision.take();
            return;
        }
    };

    let version = target_revision.take().unwrap_or_default();
    let chart_name = chart.take();

    if version.is_empty() {
        if let Some(cn) = chart_name {
            deps.push(ArgocdDep {
                dep_name: cn.clone(),
                current_value: String::new(),
                source: ArgocdSource::Helm {
                    registry_url: url,
                    chart_name: cn,
                },
                skip_reason: Some(ArgocdSkipReason::UnspecifiedVersion),
            });
        }
        return;
    }

    if let Some(cn) = chart_name {
        // Helm chart source.
        deps.push(ArgocdDep {
            dep_name: cn.clone(),
            current_value: version,
            source: ArgocdSource::Helm {
                registry_url: url,
                chart_name: cn,
            },
            skip_reason: None,
        });
    } else if url.contains("github.com") || url.ends_with(".git") {
        // Git source with a tag version.
        let dep_name = url
            .trim_end_matches(".git")
            .rsplit('/')
            .next()
            .unwrap_or(&url)
            .to_owned();
        deps.push(ArgocdDep {
            dep_name,
            current_value: version,
            source: ArgocdSource::Git { repo_url: url },
            skip_reason: None,
        });
    }
    // Other URL schemes (non-GitHub git, plain HTTP) are not actionable yet.
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "return null for kubernetes manifest" — argocd/extract.spec.ts line 21
    #[test]
    fn skips_non_argocd_file() {
        let content = "apiVersion: v1\nkind: ConfigMap\n";
        assert!(extract(content).is_empty());
    }

    #[test]
    fn extracts_helm_source() {
        let content = r#"
apiVersion: argoproj.io/v1alpha1
kind: Application
spec:
  source:
    repoURL: https://charts.helm.sh/stable
    chart: redis
    targetRevision: 10.5.7
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let d = &deps[0];
        assert_eq!(d.dep_name, "redis");
        assert_eq!(d.current_value, "10.5.7");
        assert_eq!(
            d.source,
            ArgocdSource::Helm {
                registry_url: "https://charts.helm.sh/stable".to_owned(),
                chart_name: "redis".to_owned(),
            }
        );
        assert!(d.skip_reason.is_none());
    }

    #[test]
    fn extracts_git_source() {
        let content = r#"
apiVersion: argoproj.io/v1alpha1
kind: Application
spec:
  source:
    repoURL: https://github.com/owner/myapp.git
    targetRevision: v1.2.3
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        let d = &deps[0];
        assert_eq!(d.current_value, "v1.2.3");
        assert_eq!(
            d.source,
            ArgocdSource::Git {
                repo_url: "https://github.com/owner/myapp.git".to_owned()
            }
        );
    }

    #[test]
    fn skips_missing_revision() {
        let content = r#"
apiVersion: argoproj.io/v1alpha1
kind: Application
spec:
  source:
    repoURL: https://charts.helm.sh/stable
    chart: redis
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(
            deps[0].skip_reason,
            Some(ArgocdSkipReason::UnspecifiedVersion)
        );
    }

    // Ported: "returns null for empty" — argocd/extract.spec.ts line 11
    #[test]
    fn empty_content_returns_empty() {
        assert!(extract("nothing here").is_empty());
        assert!(extract("").is_empty());
    }

    // Ported: "return result for double quoted argoproj.io apiVersion reference" — argocd/extract.spec.ts line 34
    #[test]
    fn double_quoted_apiversion_accepted() {
        let content = r#"
apiVersion: "argoproj.io/v1alpha1"
kind: Application
spec:
  source:
    chart: kube-state-metrics
    repoURL: https://prometheus-community.github.io/helm-charts
    targetRevision: 2.4.1
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "kube-state-metrics");
        assert_eq!(deps[0].current_value, "2.4.1");
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "return result for single quoted argoproj.io apiVersion reference" — argocd/extract.spec.ts line 61
    #[test]
    fn single_quoted_apiversion_accepted() {
        let content = "apiVersion: 'argoproj.io/v1alpha1'\nkind: Application\nspec:\n  source:\n    chart: kube-state-metrics\n    repoURL: https://prometheus-community.github.io/helm-charts\n    targetRevision: 2.4.1\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].dep_name, "kube-state-metrics");
        assert_eq!(deps[0].current_value, "2.4.1");
        assert!(deps[0].skip_reason.is_none());
    }
}
