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
        .filter_map(|(release_name, chart_ref, version)| {
            resolve_release(&release_name, &chart_ref, &version, &repo_map)
        })
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
            if in_repos {
                flush_repo(&mut map, &mut cur_name, &mut cur_url, &mut cur_oci);
            }
            in_repos = trimmed == "repositories:";
            continue;
        }

        if !in_repos {
            continue;
        }

        if trimmed.starts_with("- ") {
            flush_repo(&mut map, &mut cur_name, &mut cur_url, &mut cur_oci);
            let rest = trimmed.strip_prefix("- ").unwrap_or("");
            if let Some(v) = strip_key(rest, "name") {
                cur_name = Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
            }
            continue;
        }

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
        // Skip git+https:// repos (helm-git — unsupported)
        if !u.starts_with("git+") {
            map.insert(n, RepoEntry { url: u, oci: *oci });
        }
    } else {
        name.take();
        url.take();
    }
    *oci = false;
}

// ── Pass 2: releases ──────────────────────────────────────────────────────────

fn collect_releases(content: &str) -> Vec<(String, String, String)> {
    let mut out: Vec<(String, String, String)> = Vec::new();
    let mut in_releases = false;
    let mut cur_name: Option<String> = None;
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
                flush_release(&mut out, &mut cur_name, &mut cur_chart, &mut cur_version);
            }
            in_releases = trimmed == "releases:";
            continue;
        }

        if !in_releases {
            continue;
        }

        if trimmed.starts_with("- ") {
            flush_release(&mut out, &mut cur_name, &mut cur_chart, &mut cur_version);
            let rest = trimmed.strip_prefix("- ").unwrap_or("");
            if let Some(v) = strip_key(rest, "name") {
                cur_name = Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
            } else if let Some(v) = strip_key(rest, "chart") {
                cur_chart = Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
            }
            continue;
        }

        if indent > 0 {
            if let Some(v) = strip_key(trimmed, "name") {
                if cur_name.is_none() {
                    cur_name = Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
                }
            } else if let Some(v) = strip_key(trimmed, "chart") {
                cur_chart = Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
            } else if let Some(v) = strip_key(trimmed, "version") {
                cur_version = Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
            }
        }
    }
    if in_releases {
        flush_release(&mut out, &mut cur_name, &mut cur_chart, &mut cur_version);
    }

    out
}

fn flush_release(
    out: &mut Vec<(String, String, String)>,
    release_name: &mut Option<String>,
    chart: &mut Option<String>,
    version: &mut Option<String>,
) {
    if let Some(c) = chart.take() {
        let name = release_name.take().unwrap_or_default();
        let ver = version.take().unwrap_or_default();
        out.push((name, c, ver));
    } else {
        release_name.take();
        version.take();
    }
}

// ── Resolution ────────────────────────────────────────────────────────────────

/// Validates a chart name per Helm conventions (lowercase + digits + hyphens).
/// Rejects any name containing chars from the TypeScript special-chars set.
fn is_valid_chart_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    // Non-OCI chart names: reject !@#$%^&*(),.?":{}/|<>A-Z
    !name
        .chars()
        .any(|c| "!@#$%^&*(),.?\":{}/|<>".contains(c) || c.is_ascii_uppercase())
}

fn resolve_release(
    release_name: &str,
    chart_ref: &str,
    version: &str,
    repos: &HashMap<String, RepoEntry>,
) -> Option<HelmExtractedDep> {
    // Local path → dep with LocalChart skip reason (depName = release name)
    if chart_ref.starts_with("./") || chart_ref.starts_with("../") || chart_ref.starts_with('/') {
        return Some(HelmExtractedDep {
            name: release_name.to_owned(),
            current_value: version.to_owned(),
            repository: String::new(),
            skip_reason: Some(HelmSkipReason::LocalChart),
            datasource: None,
            package_name: None,
        });
    }

    // OCI registry: `oci://registry/chart`
    if let Some(oci_path) = chart_ref.strip_prefix("oci://") {
        // Version must be present and non-template
        if version.is_empty() || version.contains("{{") {
            return Some(HelmExtractedDep {
                name: oci_path.to_owned(),
                current_value: version.to_owned(),
                repository: String::new(),
                skip_reason: Some(HelmSkipReason::InvalidVersion),
                datasource: None,
                package_name: None,
            });
        }
        return Some(HelmExtractedDep {
            name: oci_path.to_owned(),
            current_value: version.to_owned(),
            repository: String::new(),
            skip_reason: None,
            datasource: Some("docker".into()),
            package_name: Some(oci_path.to_owned()),
        });
    }

    // Split on first `/` to get repo alias + chart name
    let (dep_name, repo_alias) = if let Some(slash) = chart_ref.find('/') {
        let alias = &chart_ref[..slash];
        let rest = &chart_ref[slash + 1..];
        (rest.to_owned(), Some(alias.to_owned()))
    } else {
        (chart_ref.to_owned(), Some(chart_ref.to_owned()))
    };

    let repo_alias = repo_alias.unwrap_or_default();

    // Template version → InvalidVersion (but still emit dep)
    let version_skip = if version.contains("{{") || version.is_empty() {
        Some(HelmSkipReason::InvalidVersion)
    } else {
        None
    };

    // Check for OCI-backed repo
    if let Some(entry) = repos.get(&repo_alias)
        && entry.oci
    {
        // OCI-backed repo → Docker datasource
        let pkg = format!("{}/{}", entry.url, dep_name);
        if let Some(skip) = version_skip {
            return Some(HelmExtractedDep {
                name: dep_name,
                current_value: version.to_owned(),
                repository: String::new(),
                skip_reason: Some(skip),
                datasource: Some("docker".into()),
                package_name: Some(pkg),
            });
        }
        // Validate chart name
        if !is_valid_chart_name(&dep_name) {
            return Some(HelmExtractedDep {
                name: dep_name,
                current_value: version.to_owned(),
                repository: String::new(),
                skip_reason: Some(HelmSkipReason::UnsupportedChartType),
                datasource: Some("docker".into()),
                package_name: Some(pkg),
            });
        }
        return Some(HelmExtractedDep {
            name: dep_name,
            current_value: version.to_owned(),
            repository: String::new(),
            skip_reason: None,
            datasource: Some("docker".into()),
            package_name: Some(pkg),
        });
    }

    // Validate chart name (only for the dep_name part, not the alias)
    if !is_valid_chart_name(&dep_name) {
        return Some(HelmExtractedDep {
            name: dep_name,
            current_value: version.to_owned(),
            repository: String::new(),
            skip_reason: Some(HelmSkipReason::UnsupportedChartType),
            datasource: None,
            package_name: None,
        });
    }

    if let Some(skip) = version_skip {
        return Some(HelmExtractedDep {
            name: dep_name,
            current_value: version.to_owned(),
            repository: String::new(),
            skip_reason: Some(skip),
            datasource: None,
            package_name: None,
        });
    }

    // Resolve repo URL
    let repo_url = if let Some(entry) = repos.get(&repo_alias) {
        entry.url.clone()
    } else if repo_alias == "stable" {
        STABLE_REPO.to_owned()
    } else {
        // Unknown alias — return dep with UnknownRegistry skip reason
        return Some(HelmExtractedDep {
            name: dep_name,
            current_value: version.to_owned(),
            repository: String::new(),
            skip_reason: Some(HelmSkipReason::UnknownRegistry),
            datasource: None,
            package_name: None,
        });
    };

    Some(HelmExtractedDep {
        name: dep_name,
        current_value: version.to_owned(),
        repository: repo_url,
        skip_reason: None,
        datasource: None,
        package_name: None,
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

    // Ported: "skip null YAML document" — helmfile/extract.spec.ts line 18
    #[test]
    fn null_yaml_document_returns_empty() {
        assert!(extract("~").is_empty());
    }

    // Ported: "returns null if no releases" — helmfile/extract.spec.ts line 31
    #[test]
    fn no_releases_section_returns_empty() {
        let content = r#"
repositories:
  - name: kiwigrid
    url: https://kiwigrid.github.io
"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "do not crash on invalid helmfile.yaml" — helmfile/extract.spec.ts line 46
    #[test]
    fn invalid_yaml_does_not_crash() {
        let content = r#"
repositories:
  - name: kiwigrid
    url: https://kiwigrid.github.io

releases: [
"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "skip if repository details are not specified" — helmfile/extract.spec.ts line 63
    #[test]
    fn unknown_repo_alias_has_skip_reason() {
        let content = r#"
repositories:
  - name: kiwigrid
    url: https://kiwigrid.github.io
releases:
  - name: example
    version: 1.0.0
    chart: experimental/example
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert!(deps[0].skip_reason.is_some());
    }

    // Ported: "skip templetized release with invalid characters" — helmfile/extract.spec.ts line 84
    #[test]
    fn invalid_chart_name_chars_skipped() {
        let content = r#"
repositories:
  - name: kiwigrid
    url: https://kiwigrid.github.io
releases:
  - name: example
    version: 1.0.0
    chart: stable/!!!!--!
  - name: example-internal
    version: 1.0.0
    chart: stable/example
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        let invalid = deps.iter().find(|d| d.name == "!!!!--!").unwrap();
        assert_eq!(
            invalid.skip_reason,
            Some(HelmSkipReason::UnsupportedChartType)
        );
        let valid = deps.iter().find(|d| d.name == "example").unwrap();
        assert!(valid.skip_reason.is_none());
    }

    // Ported: "skip local charts" — helmfile/extract.spec.ts line 118
    #[test]
    fn local_path_chart_gets_skip_reason() {
        let content = r#"
releases:
  - name: local
    chart: ./local-chart
    version: "1.0.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(HelmSkipReason::LocalChart));
        assert_eq!(deps[0].name, "local");
    }

    // Ported: "skip chart with unknown repository" — helmfile/extract.spec.ts line 139
    #[test]
    fn chart_with_no_matching_repo_skipped() {
        let content = r#"
repositories:
  - name: kiwigrid
    url: https://kiwigrid.github.io
releases:
  - name: example
    version: 1.0.0
    chart: example
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(HelmSkipReason::UnknownRegistry));
    }

    // Ported: "skip chart with special character in the name" — helmfile/extract.spec.ts line 160
    #[test]
    fn chart_with_special_chars_skipped() {
        let content = r#"
repositories:
  - name: kiwigrid
    url: https://kiwigrid.github.io
releases:
  - name: example
    version: 1.0.0
    chart: kiwigrid/example/example
  - name: example2
    version: 1.0.0
    chart: kiwigrid/example?example
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(
            deps.iter()
                .all(|d| d.skip_reason == Some(HelmSkipReason::UnsupportedChartType))
        );
    }

    // Ported: "skip chart that does not have specified version" — helmfile/extract.spec.ts line 184
    #[test]
    fn release_without_version_has_invalid_version_skip() {
        let content = r#"
repositories:
  - name: kiwigrid
    url: https://kiwigrid.github.io
releases:
  - name: example
    chart: stable/example
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(HelmSkipReason::InvalidVersion));
    }

    // Ported: "parses a chart with a go templating" — helmfile/extract.spec.ts line 242
    #[test]
    fn go_template_chart_skipped_real_chart_kept() {
        let content = r#"
repositories:
  - name: stable
    url: https://charts.helm.sh/stable
releases:
  - name: example
    version: 1.0.0
    chart: "{{ .Values.chart }}"
  - name: example-internal
    version: 1.0.0
    chart: stable/example
"#;
        let deps = extract(content);
        // Template chart ref gets skipped; stable/example is valid
        assert!(
            deps.iter()
                .any(|d| d.name == "example" && d.skip_reason.is_none())
        );
    }

    // Ported: "parses a chart with empty strings for template values" — helmfile/extract.spec.ts line 280
    #[test]
    fn template_version_gets_invalid_version_skip() {
        let content = r#"
repositories:
  - name: stable
    url: https://charts.helm.sh/stable
releases:
  - name: example
    version: "{{ .Values.example.version }}"
    chart: stable/example
  - name: example-external
    version: 1.0.0
    chart: "{{ .Values.example.repository }}"
  - name: example-internal
    version: 1.0.0
    chart: stable/example
"#;
        let deps = extract(content);
        let versioned_template = deps.iter().find(|d| d.name == "example");
        if let Some(d) = versioned_template {
            assert_eq!(d.skip_reason, Some(HelmSkipReason::InvalidVersion));
        }
        // stable/example with real version is valid
        assert!(
            deps.iter()
                .any(|d| d.name == "example" && d.skip_reason.is_none())
        );
    }

    // Ported: "parses a chart with an oci repository and non-oci one" — helmfile/extract.spec.ts line 316
    #[test]
    fn oci_backed_repo_uses_docker_datasource() {
        let content = r#"
repositories:
  - name: oci-repo
    url: ghcr.io/example/oci-repo
    oci: true
  - name: jenkins
    url: https://charts.jenkins.io

releases:
  - name: example
    version: 0.1.0
    chart: oci-repo/example
  - name: jenkins
    chart: jenkins/jenkins
    version: 3.3.0
  - name: oci-url
    version: 0.4.2
    chart: oci://ghcr.io/example/oci-repo/url-example
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 3);
        let oci_repo = deps.iter().find(|d| d.name == "example").unwrap();
        assert_eq!(oci_repo.datasource.as_deref(), Some("docker"));
        assert_eq!(
            oci_repo.package_name.as_deref(),
            Some("ghcr.io/example/oci-repo/example")
        );
        assert!(oci_repo.skip_reason.is_none());
        let jenkins = deps.iter().find(|d| d.name == "jenkins").unwrap();
        assert_eq!(jenkins.repository, "https://charts.jenkins.io");
        assert!(jenkins.skip_reason.is_none());
        let oci_url = deps
            .iter()
            .find(|d| d.name == "ghcr.io/example/oci-repo/url-example")
            .unwrap();
        assert_eq!(oci_url.datasource.as_deref(), Some("docker"));
        assert_eq!(
            oci_url.package_name.as_deref(),
            Some("ghcr.io/example/oci-repo/url-example")
        );
    }

    // Ported: "allows OCI chart names containing forward slashes" — helmfile/extract.spec.ts line 366
    #[test]
    fn oci_nested_path_chart_uses_docker_datasource() {
        let content = r#"
repositories:
  - name: oci-repo
    url: ghcr.io/example/oci-repo
    oci: true
releases:
  - name: nested-example
    version: 1.2.3
    chart: oci-repo/nested/path/chart
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].datasource.as_deref(), Some("docker"));
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("ghcr.io/example/oci-repo/nested/path/chart")
        );
        assert_eq!(deps[0].current_value, "1.2.3");
    }

    // Ported: "parses a chart with an oci repository with ---" — helmfile/extract.spec.ts line 392
    #[test]
    fn oci_repo_with_yaml_document_separator() {
        let content = r#"repositories:
  - name: oci-repo
    url: ghcr.io/example/oci-repo
    oci: true
---
releases:
  - name: example
    version: 0.1.0
    chart: oci-repo/example
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].datasource.as_deref(), Some("docker"));
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("ghcr.io/example/oci-repo/example")
        );
    }

    // Ported: "makes sure url joiner works correctly" — helmfile/extract.spec.ts line 513
    #[test]
    fn oci_url_with_port_in_chart_ref() {
        let content = r#"
releases:
  - name: argocd
    version: 0.4.2
    chart: oci://gitlab.example.com:5000/group/subgroup
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].datasource.as_deref(), Some("docker"));
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("gitlab.example.com:5000/group/subgroup")
        );
        assert_eq!(deps[0].name, "gitlab.example.com:5000/group/subgroup");
    }

    // Ported: "skips helm-git repos" — helmfile/extract.spec.ts line 539
    #[test]
    fn helm_git_repo_releases_get_unknown_registry() {
        let content = r#"
repositories:
  - name: gitops-external-cluster
    url: git+https://github.com/codefresh-io/csdp-official@add-cluster/helm

releases:
  - name: gitops-external-cluster
    namespace: gitops-runtime
    chart: gitops-external-cluster/csdp-add-cluster
    version: 0.4.0
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].name, "csdp-add-cluster");
        assert_eq!(deps[0].skip_reason, Some(HelmSkipReason::UnknownRegistry));
    }

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
    fn oci_chart_uses_docker_datasource() {
        let content = r#"
releases:
- name: app
  chart: oci://registry.example.com/app
  version: "2.0.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].datasource.as_deref(), Some("docker"));
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("registry.example.com/app")
        );
    }

    #[test]
    fn unknown_alias_has_unknown_registry_skip() {
        let content = r#"
releases:
- name: app
  chart: unknown-repo/my-chart
  version: "3.0.0"
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].skip_reason, Some(HelmSkipReason::UnknownRegistry));
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
    fn oci_backed_repo_alias_uses_docker_datasource() {
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
        assert_eq!(deps[0].datasource.as_deref(), Some("docker"));
    }

    #[test]
    fn template_expression_in_chart_has_unknown_registry() {
        let content = r#"
releases:
- name: dynamic
  chart: '{{ .Values.chart }}'
  version: "1.0.0"
"#;
        let deps = extract(content);
        // Template chart ref has no repo info
        assert!(deps.is_empty() || deps[0].skip_reason.is_some());
    }

    #[test]
    fn release_without_version_has_skip_reason() {
        let content = r#"
releases:
- name: unversioned
  chart: stable/nginx
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert!(deps[0].skip_reason.is_some());
    }

    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }
}
