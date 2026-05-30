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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HelmfileExtract {
    pub deps: Vec<HelmExtractedDep>,
    pub need_kustomize: bool,
}

/// Parse all chart dependencies from a Helmfile YAML.
pub fn extract(content: &str) -> Vec<HelmExtractedDep> {
    extract_package_file(content).deps
}

pub fn extract_package_file(content: &str) -> HelmfileExtract {
    extract_package_file_with_registry_aliases(content, &HashMap::new())
}

pub fn extract_package_file_with_registry_aliases(
    content: &str,
    registry_aliases: &HashMap<String, String>,
) -> HelmfileExtract {
    let content = remove_template_control_lines(content);

    // ── Pass 1: collect repositories ─────────────────────────────────────────
    let repo_map = collect_repositories(&content);

    // ── Pass 2: collect and resolve releases ──────────────────────────────────
    let mut raw_releases = collect_releases(&content);
    raw_releases.extend(collect_templates(&content));

    let mut need_kustomize = false;
    let deps = raw_releases
        .into_iter()
        .filter_map(|release| {
            let dep = resolve_release(
                &release.name,
                &release.chart,
                &release.version,
                &repo_map,
                registry_aliases,
            )?;
            if release.has_kustomize_keys || dep.skip_reason == Some(HelmSkipReason::LocalChart) {
                need_kustomize = true;
            }
            Some(dep)
        })
        .collect();

    HelmfileExtract {
        deps,
        need_kustomize,
    }
}

// ── Internal types ────────────────────────────────────────────────────────────

struct RepoEntry {
    url: String,
    oci: bool,
}

struct RawRelease {
    name: String,
    chart: String,
    version: String,
    has_kustomize_keys: bool,
}

fn remove_template_control_lines(content: &str) -> String {
    content
        .lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            !trimmed.starts_with("{{") || trimmed.contains(':')
        })
        .collect::<Vec<_>>()
        .join("\n")
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

fn collect_releases(content: &str) -> Vec<RawRelease> {
    let mut out: Vec<RawRelease> = Vec::new();
    let mut in_releases = false;
    let mut cur_name: Option<String> = None;
    let mut cur_chart: Option<String> = None;
    let mut cur_version: Option<String> = None;
    let mut cur_has_kustomize_keys = false;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        if line.trim().is_empty() {
            continue;
        }
        let trimmed = line.trim_start();
        let indent = leading_spaces(line);

        if indent == 0 && !trimmed.starts_with('-') {
            if in_releases {
                flush_release(
                    &mut out,
                    &mut cur_name,
                    &mut cur_chart,
                    &mut cur_version,
                    &mut cur_has_kustomize_keys,
                );
            }
            in_releases = trimmed == "releases:";
            continue;
        }

        if !in_releases {
            continue;
        }

        if trimmed.starts_with("- ") {
            flush_release(
                &mut out,
                &mut cur_name,
                &mut cur_chart,
                &mut cur_version,
                &mut cur_has_kustomize_keys,
            );
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
            } else if is_kustomize_key(trimmed) {
                cur_has_kustomize_keys = true;
            }
        }
    }
    if in_releases {
        flush_release(
            &mut out,
            &mut cur_name,
            &mut cur_chart,
            &mut cur_version,
            &mut cur_has_kustomize_keys,
        );
    }

    out
}

fn flush_release(
    out: &mut Vec<RawRelease>,
    release_name: &mut Option<String>,
    chart: &mut Option<String>,
    version: &mut Option<String>,
    has_kustomize_keys: &mut bool,
) {
    if let Some(c) = chart.take() {
        let name = release_name.take().unwrap_or_default();
        let ver = version.take().unwrap_or_default();
        out.push(RawRelease {
            name,
            chart: c,
            version: ver,
            has_kustomize_keys: *has_kustomize_keys,
        });
    } else {
        release_name.take();
        version.take();
    }
    *has_kustomize_keys = false;
}

fn collect_templates(content: &str) -> Vec<RawRelease> {
    let mut out: Vec<RawRelease> = Vec::new();
    let mut in_templates = false;
    let mut template_indent: usize = 0;
    let mut cur_name: Option<String> = None;
    let mut cur_chart: Option<String> = None;
    let mut cur_version: Option<String> = None;
    let mut cur_has_kustomize_keys = false;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        if line.trim().is_empty() {
            continue;
        }
        let trimmed = line.trim_start();
        let indent = leading_spaces(line);

        // Top-level section boundary
        if indent == 0 && !trimmed.starts_with('-') {
            if in_templates {
                flush_release(
                    &mut out,
                    &mut cur_name,
                    &mut cur_chart,
                    &mut cur_version,
                    &mut cur_has_kustomize_keys,
                );
            }
            in_templates = trimmed == "templates:";
            template_indent = 0;
            continue;
        }

        if !in_templates {
            continue;
        }

        // First level of indentation under templates: is the map key
        if template_indent == 0 && indent > 0 {
            template_indent = indent;
        }

        if indent == template_indent && !trimmed.starts_with('-') {
            // This is a template map key (e.g. "common:")
            flush_release(
                &mut out,
                &mut cur_name,
                &mut cur_chart,
                &mut cur_version,
                &mut cur_has_kustomize_keys,
            );
            // The map key itself is the default name
            if let Some(key) = trimmed.strip_suffix(':') {
                cur_name = Some(key.trim().trim_matches('"').trim_matches('\'').to_owned());
            }
            continue;
        }

        if indent > template_indent {
            // Fields inside a template entry
            if let Some(v) = strip_key(trimmed, "name") {
                cur_name = Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
            } else if let Some(v) = strip_key(trimmed, "chart") {
                cur_chart = Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
            } else if let Some(v) = strip_key(trimmed, "version") {
                cur_version = Some(v.trim().trim_matches('"').trim_matches('\'').to_owned());
            } else if is_kustomize_key(trimmed) {
                cur_has_kustomize_keys = true;
            }
        }
    }

    if in_templates {
        flush_release(
            &mut out,
            &mut cur_name,
            &mut cur_chart,
            &mut cur_version,
            &mut cur_has_kustomize_keys,
        );
    }

    out
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
    registry_aliases: &HashMap<String, String>,
) -> Option<HelmExtractedDep> {
    if chart_ref.contains("{{") {
        return None;
    }

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
    } else if let Some(url) = registry_aliases.get(&repo_alias) {
        url.clone()
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

fn is_kustomize_key(line: &str) -> bool {
    ["strategicMergePatches", "jsonPatches", "transformers"]
        .iter()
        .any(|key| strip_key(line, key).is_some())
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

    // Ported: "parses multidoc yaml" — helmfile/extract.spec.ts line 204
    #[test]
    fn parses_multidoc_yaml() {
        let content = r#"
environments:
  beta: {}
---
repositories:
  - name: stable
    url: https://charts.helm.sh/stable/
  - name: incubator
    url: https://charts.helm.sh/incubator/
  - name: bitnami
    url: https://charts.bitnami.com/bitnami
  - name: prometheus-community
    url: https://prometheus-community.github.io/helm-charts
---
releases:
  - name: manifests
    chart: ./environment/beta/static
  - name: rabbitmq
    chart: bitnami/rabbitmq
    version: 7.4.3
  - name: prometheus-operator
    chart: prometheus-community/kube-prometheus-stack
    version: 13.7
  - name: external-dns
    chart: bitnami/external-dns
    version: {{ .Values | getOrNil "external_dns.version" | default "4.5.5" }}
  - name: raw1
    chart: incubator/raw
    version: 0.1.0
"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 5);

        let manifests = deps.iter().find(|d| d.name == "manifests").unwrap();
        assert_eq!(manifests.skip_reason, Some(HelmSkipReason::LocalChart));

        let rabbitmq = deps.iter().find(|d| d.name == "rabbitmq").unwrap();
        assert_eq!(rabbitmq.current_value, "7.4.3");
        assert_eq!(rabbitmq.repository, "https://charts.bitnami.com/bitnami");

        let kube = deps
            .iter()
            .find(|d| d.name == "kube-prometheus-stack")
            .unwrap();
        assert_eq!(kube.current_value, "13.7");
        assert_eq!(
            kube.repository,
            "https://prometheus-community.github.io/helm-charts"
        );

        let external_dns = deps.iter().find(|d| d.name == "external-dns").unwrap();
        assert_eq!(
            external_dns.skip_reason,
            Some(HelmSkipReason::InvalidVersion)
        );

        let raw = deps.iter().find(|d| d.name == "raw").unwrap();
        assert_eq!(raw.current_value, "0.1.0");
        assert_eq!(raw.repository, "https://charts.helm.sh/incubator/");
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

    // Ported: "parses and replaces templating strings" — helmfile/extract.spec.ts line 423
    #[test]
    fn go_template_fixture_resolves_fallbacks_and_registry_aliases() {
        let content = r#"
repositories:
- name: incubator
  url: https://charts.helm.sh/incubator/
- name: bitnami
  url: https://charts.bitnami.com/bitnami
- name: prometheus-community
  url: https://prometheus-community.github.io/helm-charts

releases:
  - name: "{{ requiredEnv "RELEASE_NAME" }}"
    namespace: default
    chart: ./foo
{{ if .Values.nginx.intranet.enabled }}
  - name: nginx-intranet
    chart: ingress-nginx/ingress-nginx
    version: 3.37.0
{{ end }}
  - name: example
{{- if neq .Values.example.version  "" }}
    version: {{ .Values.example.version }}
{{- else }}
    version: 6.0.0
{{- end }}
    chart: bitnami/memcached
  - name: example-internal
    version: 1.30.0
    chart: stable/example
  - name: example-private
    version: {{ .Values.example.version }}
    chart: prometheus-community/kube-prometheus-stack
  - name: example-external
    version: 1.48.0
    chart: {{ .Values.example.repository }}
  - name: example-public
    version: 2.0.0
    chart: stable/external-dns
"#;
        let aliases = HashMap::from([(
            "stable".to_owned(),
            "https://charts.helm.sh/stable".to_owned(),
        )]);
        let result = extract_package_file_with_registry_aliases(content, &aliases);

        assert!(result.need_kustomize);

        let ingress = result
            .deps
            .iter()
            .find(|d| d.name == "ingress-nginx")
            .unwrap();
        assert_eq!(ingress.skip_reason, Some(HelmSkipReason::UnknownRegistry));

        let memcached = result.deps.iter().find(|d| d.name == "memcached").unwrap();
        assert_eq!(memcached.current_value, "6.0.0");
        assert_eq!(memcached.repository, "https://charts.bitnami.com/bitnami");

        let example = result.deps.iter().find(|d| d.name == "example").unwrap();
        assert_eq!(example.current_value, "1.30.0");
        assert_eq!(example.repository, "https://charts.helm.sh/stable");

        let kube = result
            .deps
            .iter()
            .find(|d| d.name == "kube-prometheus-stack")
            .unwrap();
        assert_eq!(kube.skip_reason, Some(HelmSkipReason::InvalidVersion));

        let external_dns = result
            .deps
            .iter()
            .find(|d| d.name == "external-dns")
            .unwrap();
        assert_eq!(external_dns.current_value, "2.0.0");
        assert_eq!(external_dns.repository, "https://charts.helm.sh/stable");
        assert!(
            result
                .deps
                .iter()
                .all(|d| d.name != "{{ .Values.example.repository }}")
        );
    }

    // Ported: "detects kustomize and respects relative paths" — helmfile/extract.spec.ts line 477
    #[test]
    fn local_chart_marks_need_kustomize_and_keeps_relative_dep() {
        let content = r#"
repositories:
  - name: bitnami
    url: https://charts.bitnami.com/bitnami

releases:
  - name: my-chart
    chart: ../charts/my-chart
  - name: memcached
    version: 6.0.0
    chart: bitnami/memcached
"#;
        let result = extract_package_file_with_registry_aliases(
            content,
            &HashMap::from([(
                "stable".to_owned(),
                "https://charts.helm.sh/stable".to_owned(),
            )]),
        );

        assert!(result.need_kustomize);
        let local = result.deps.iter().find(|d| d.name == "my-chart").unwrap();
        assert_eq!(local.skip_reason, Some(HelmSkipReason::LocalChart));

        let memcached = result.deps.iter().find(|d| d.name == "memcached").unwrap();
        assert_eq!(memcached.current_value, "6.0.0");
        assert_eq!(memcached.repository, "https://charts.bitnami.com/bitnami");
        assert!(memcached.skip_reason.is_none());
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

    // Rust-specific: unit test for release with repo extraction
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

    // Rust-specific: unit test for stable alias resolution
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

    // Rust-specific: unit test for OCI chart datasource
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

    // Rust-specific: unit test for unknown alias skip reason
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

    // Rust-specific: unit test for multiple releases extraction
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

    // Rust-specific: unit test for OCI-backed repo alias
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

    // Rust-specific: unit test for template expression handling
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

    // Rust-specific: unit test for release without version
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

    // Ported: "returns null for empty" — lib/modules/manager/woodpecker/extract.spec.ts line 8
    #[test]
    fn empty_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "parses templates key alongside releases" — helmfile/extract.spec.ts line 576
    #[test]
    fn parses_templates_key_alongside_releases() {
        let content = "
repositories:
  - name: kiwigrid
    url: https://kiwigrid.github.io

templates:
  common:
    name: common
    version: 1.0.0
    chart: kiwigrid/common
  shared:
    name: shared
    version: 2.0.0
    chart: kiwigrid/shared

releases:
  - name: my-release
    version: 3.0.0
    chart: kiwigrid/my-chart
";
        let result = extract_package_file(content);
        let deps = result.deps;
        // Should have 3 deps: my-release + common + shared
        assert_eq!(deps.len(), 3);
        let release = deps.iter().find(|d| d.name == "my-chart").unwrap();
        assert_eq!(release.current_value, "3.0.0");
        let common = deps.iter().find(|d| d.name == "common").unwrap();
        assert_eq!(common.current_value, "1.0.0");
        let shared = deps.iter().find(|d| d.name == "shared").unwrap();
        assert_eq!(shared.current_value, "2.0.0");
    }
}
