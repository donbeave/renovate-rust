//! Vendir `vendir.yml` Helm chart dependency extractor.
//!
//! Scans Vendir configuration files for `helmChart:` entries within
//! `directories[*].contents[*]` and returns chart dependencies for
//! Helm repository version lookups.
//!
//! Renovate reference:
//! - `lib/modules/manager/vendir/extract.ts`
//! - Pattern: `/(^|/)vendir\.yml$/`
//! - Datasource: Helm
//!
//! ## Supported form
//!
//! ```yaml
//! directories:
//!   - path: vendor
//!     contents:
//!     - path: some-chart
//!       helmChart:
//!         name: renovate
//!         version: 36.109.4
//!         repository:
//!           url: https://docs.renovatebot.com/helm-charts
//! ```

/// A single extracted Vendir Helm chart dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VendirHelmDep {
    pub chart_name: String,
    pub version: String,
    pub repo_url: String,
}

/// Extract Helm chart deps from a `vendir.yml` file.
pub fn extract(content: &str) -> Vec<VendirHelmDep> {
    let mut out = Vec::new();
    let mut in_helm = false;
    let mut cur_name: Option<String> = None;
    let mut cur_version: Option<String> = None;
    let mut cur_repo: Option<String> = None;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        let trimmed = line.trim_start();

        if trimmed.is_empty() {
            continue;
        }

        // `helmChart:` at any indentation signals start of a helm chart block.
        if trimmed == "helmChart:" {
            // flush previous if complete
            flush(&mut out, &mut cur_name, &mut cur_version, &mut cur_repo);
            in_helm = true;
            continue;
        }

        // Leaving helm block at lower or equal indentation with a different key.
        if in_helm && !trimmed.starts_with('-') && trimmed.contains(':') {
            let key = trimmed.split(':').next().unwrap_or("").trim();
            if !matches!(key, "name" | "version" | "repository" | "url") {
                flush(&mut out, &mut cur_name, &mut cur_version, &mut cur_repo);
                in_helm = false;
            }
        }

        if !in_helm {
            continue;
        }

        if let Some(v) = strip_key(trimmed, "name") {
            cur_name = Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
        } else if let Some(v) = strip_key(trimmed, "version") {
            cur_version = Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
        } else if let Some(v) = strip_key(trimmed, "url") {
            cur_repo = Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
        }
    }
    flush(&mut out, &mut cur_name, &mut cur_version, &mut cur_repo);
    out
}

fn flush(
    out: &mut Vec<VendirHelmDep>,
    name: &mut Option<String>,
    version: &mut Option<String>,
    repo: &mut Option<String>,
) {
    if let (Some(n), Some(v), Some(r)) = (name.take(), version.take(), repo.take()) {
        if !n.is_empty() && !v.is_empty() && !r.is_empty() {
            out.push(VendirHelmDep {
                chart_name: n,
                version: v,
                repo_url: r,
            });
        }
    } else {
        name.take();
        version.take();
        repo.take();
    }
}

fn strip_key<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    line.strip_prefix(&format!("{key}:"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
apiVersion: vendir.k14s.io/v1alpha1
kind: Config

directories:
  - path: vendor
    contents:
    - path: renovate
      helmChart:
        name: renovate
        version: 36.109.4
        repository:
          url: https://docs.renovatebot.com/helm-charts
    - path: prometheus
      helmChart:
        name: prometheus
        version: 25.0.0
        repository:
          url: https://prometheus-community.github.io/helm-charts
"#;

    #[test]
    fn extracts_helm_charts() {
        let deps = extract(SAMPLE);
        assert_eq!(deps.len(), 2);
        let renovate = deps.iter().find(|d| d.chart_name == "renovate").unwrap();
        assert_eq!(renovate.version, "36.109.4");
        assert_eq!(
            renovate.repo_url,
            "https://docs.renovatebot.com/helm-charts"
        );
    }

    #[test]
    fn extracts_second_chart() {
        let deps = extract(SAMPLE);
        let prom = deps.iter().find(|d| d.chart_name == "prometheus").unwrap();
        assert_eq!(prom.version, "25.0.0");
    }

    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    #[test]
    fn no_helm_charts_returns_empty() {
        // Ported: "returns null for empty directories key" — vendir/extract.spec.ts line 20
        let content = "apiVersion: vendir.k14s.io/v1alpha1\nkind: Config\ndirectories: []\n";
        assert!(extract(content).is_empty());
    }

    #[test]
    fn invalid_yaml_returns_empty() {
        // Ported: "returns null for invalid yaml file content" — vendir/extract.spec.ts line 10
        // Unclosed bracket is treated as text, parser finds no helmChart entries
        assert!(extract("nothing here: [").is_empty());
    }
}
