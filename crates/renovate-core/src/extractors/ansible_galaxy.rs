//! Ansible Galaxy `requirements.yml` dependency extractor.
//!
//! Parses Ansible Galaxy requirements files and extracts role/collection
//! dependencies. GitHub-URL-sourced roles are routed to the GitHub Tags
//! datasource. Galaxy-hosted collections and roles are noted but deferred
//! (require a specialized Galaxy API datasource).
//!
//! Renovate reference:
//! - `lib/modules/manager/ansible-galaxy/extract.ts`
//! - `lib/modules/manager/ansible-galaxy/roles.ts`
//! - Pattern: `/(^|/)(galaxy|requirements)(\\.ansible)?\\.ya?ml$/`
//!
//! ## Supported forms
//!
//! ```yaml
//! roles:
//!   - name: my_role
//!     src: https://github.com/owner/ansible-role-something
//!     version: v2.1.0
//!
//!   - name: galaxy_role         # Galaxy-hosted → skipped (GalaxyDatasource)
//!     src: namespace.role_name
//!
//! collections:
//!   - name: community.general   # Galaxy-hosted → skipped (GalaxyDatasource)
//!     version: ">=7.0.0"
//! ```

/// Where this dep comes from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnsibleGalaxySource {
    /// GitHub repo URL — routes to GitHub Tags datasource.
    GitHub { owner_repo: String },
    /// Ansible Galaxy ID (`namespace.name`) — deferred (GalaxyDatasource).
    Galaxy,
}

/// Why a dep is being skipped (in the context of this extractor).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnsibleGalaxySkipReason {
    /// No version was specified.
    NoVersion,
    /// Hosted on Ansible Galaxy (requires `GalaxyDatasource`, not yet implemented).
    GalaxyHosted,
}

/// A single extracted Ansible Galaxy dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnsibleGalaxyDep {
    /// Role/collection name or display name.
    pub dep_name: String,
    /// Version string (git tag or semver range).
    pub current_value: String,
    pub source: AnsibleGalaxySource,
    pub skip_reason: Option<AnsibleGalaxySkipReason>,
}

/// Extract dependencies from an Ansible Galaxy `requirements.yml`.
pub fn extract(content: &str) -> Vec<AnsibleGalaxyDep> {
    let mut out = Vec::new();

    // Simple state machine: scan blocks starting with `- name:` or `- src:`.
    // Each block has `name:`, `src:`, and optionally `version:`.
    let mut in_list = false; // inside roles: or collections: list
    let mut current_name: Option<String> = None;
    let mut current_src: Option<String> = None;
    let mut current_ver: Option<String> = None;

    for raw in content.lines() {
        let line = raw.split(" #").next().unwrap_or(raw).trim_end();
        if line.trim().is_empty() {
            continue;
        }

        let trimmed = line.trim_start();
        let indent = leading_spaces(line);

        // Detect top-level section keys (`roles:`, `collections:`).
        if indent == 0 {
            if trimmed.starts_with("roles:") || trimmed.starts_with("collections:") {
                flush(
                    &mut current_name,
                    &mut current_src,
                    &mut current_ver,
                    &mut out,
                );
                in_list = true;
                continue;
            }
            // Any other top-level key ends the current section.
            if !trimmed.starts_with('-') {
                flush(
                    &mut current_name,
                    &mut current_src,
                    &mut current_ver,
                    &mut out,
                );
                in_list = false;
                continue;
            }
        }

        if !in_list {
            continue;
        }

        // New list item.
        if let Some(rest) = trimmed.strip_prefix("- ") {
            flush(
                &mut current_name,
                &mut current_src,
                &mut current_ver,
                &mut out,
            );
            // Inline `- name: val` or `- src: val` or `- source: val`.
            if let Some(val) = strip_key(rest, "name") {
                current_name = Some(val.trim().trim_matches('"').trim_matches('\'').to_owned());
            } else if let Some(val) = strip_key(rest, "src").or_else(|| strip_key(rest, "source")) {
                current_src = Some(val.trim().trim_matches('"').trim_matches('\'').to_owned());
            } else if let Some(val) = strip_key(rest, "version") {
                current_ver = Some(val.trim().trim_matches('"').trim_matches('\'').to_owned());
            }
            continue;
        }

        // Continuation key inside current list item.
        if let Some(val) = strip_key(trimmed, "name") {
            current_name = Some(val.trim().trim_matches('"').trim_matches('\'').to_owned());
        } else if let Some(val) = strip_key(trimmed, "src").or_else(|| strip_key(trimmed, "source"))
        {
            current_src = Some(val.trim().trim_matches('"').trim_matches('\'').to_owned());
        } else if let Some(val) = strip_key(trimmed, "version") {
            current_ver = Some(val.trim().trim_matches('"').trim_matches('\'').to_owned());
        }
    }

    flush(
        &mut current_name,
        &mut current_src,
        &mut current_ver,
        &mut out,
    );
    out
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn flush(
    name: &mut Option<String>,
    src: &mut Option<String>,
    ver: &mut Option<String>,
    out: &mut Vec<AnsibleGalaxyDep>,
) {
    let dep_name = name.take().or_else(|| src.clone()).unwrap_or_default();
    let raw_src = src.take().unwrap_or_default();
    let version = ver.take().unwrap_or_default();

    if dep_name.is_empty() && raw_src.is_empty() {
        return;
    }

    let display_name = if dep_name.is_empty() {
        raw_src.clone()
    } else {
        dep_name
    };

    let (source, skip_reason) = classify_source(&raw_src, &version);

    out.push(AnsibleGalaxyDep {
        dep_name: display_name,
        current_value: version,
        source,
        skip_reason,
    });
}

fn classify_source(
    src: &str,
    version: &str,
) -> (AnsibleGalaxySource, Option<AnsibleGalaxySkipReason>) {
    // GitHub URL: https://github.com/owner/repo or https://github.com/owner/repo.git
    if src.starts_with("https://github.com/") || src.starts_with("git@github.com:") {
        let path = src
            .trim_start_matches("https://github.com/")
            .trim_start_matches("git@github.com:")
            .trim_end_matches(".git");
        let owner_repo = path.to_owned();
        if version.is_empty() {
            return (
                AnsibleGalaxySource::GitHub { owner_repo },
                Some(AnsibleGalaxySkipReason::NoVersion),
            );
        }
        return (AnsibleGalaxySource::GitHub { owner_repo }, None);
    }

    // Galaxy-hosted (namespace.name format or empty src with just a name).
    (
        AnsibleGalaxySource::Galaxy,
        Some(AnsibleGalaxySkipReason::GalaxyHosted),
    )
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

    const SAMPLE: &str = r"
roles:
  - name: webserver
    src: https://github.com/geerlingguy/ansible-role-apache
    version: 3.2.0

  - name: database
    src: https://github.com/geerlingguy/ansible-role-mysql.git
    version: v4.0.0

  - name: galaxy_role
    src: geerlingguy.java

  - src: https://github.com/owner/role-without-version

collections:
  - name: community.general
    version: '>=7.0.0'
  - name: ansible.posix
    version: '1.5.4'
";

    // Ported: "extracts multiple dependencies from requirements.yml" — ansible-galaxy/extract.spec.ts line 19
    #[test]
    fn extracts_github_roles() {
        let deps = extract(SAMPLE);
        let apache = deps.iter().find(|d| d.dep_name == "webserver").unwrap();
        assert_eq!(apache.current_value, "3.2.0");
        assert_eq!(
            apache.source,
            AnsibleGalaxySource::GitHub {
                owner_repo: "geerlingguy/ansible-role-apache".to_owned()
            }
        );
        assert!(apache.skip_reason.is_none());
    }

    // Ported: "extracts multiple dependencies from requirements.yml" — ansible-galaxy/extract.spec.ts line 19
    #[test]
    fn strips_git_suffix() {
        let deps = extract(SAMPLE);
        let mysql = deps.iter().find(|d| d.dep_name == "database").unwrap();
        assert_eq!(
            mysql.source,
            AnsibleGalaxySource::GitHub {
                owner_repo: "geerlingguy/ansible-role-mysql".to_owned()
            }
        );
        assert_eq!(mysql.current_value, "v4.0.0");
    }

    // Ported: "extracts multiple dependencies from requirements.yml" — ansible-galaxy/extract.spec.ts line 19
    #[test]
    fn galaxy_roles_skipped() {
        let deps = extract(SAMPLE);
        let galaxy = deps.iter().find(|d| d.dep_name == "galaxy_role").unwrap();
        assert_eq!(
            galaxy.skip_reason,
            Some(AnsibleGalaxySkipReason::GalaxyHosted)
        );
    }

    // Ported: "extracts multiple dependencies from requirements.yml" — ansible-galaxy/extract.spec.ts line 19
    #[test]
    fn no_version_skipped() {
        let deps = extract(SAMPLE);
        let no_ver = deps
            .iter()
            .find(|d| d.dep_name.contains("role-without-version"))
            .unwrap();
        assert_eq!(no_ver.skip_reason, Some(AnsibleGalaxySkipReason::NoVersion));
    }

    // Ported: "check collection style requirements file" — ansible-galaxy/extract.spec.ts line 66
    #[test]
    fn collections_skipped_as_galaxy() {
        let deps = extract(SAMPLE);
        let cg = deps.iter().find(|d| d.dep_name == "community.general");
        // Collections without GitHub src are GalaxyHosted
        assert!(cg.is_some());
        assert_eq!(
            cg.unwrap().skip_reason,
            Some(AnsibleGalaxySkipReason::GalaxyHosted)
        );
    }

    // Ported: "returns null for empty" — ansible-galaxy/extract.spec.ts line 15
    #[test]
    fn empty_content_returns_no_deps() {
        assert!(extract("").is_empty());
    }

    // Ported: "extracts dependencies from requirements.yml with a space at the end of line" — ansible-galaxy/extract.spec.ts line 31
    #[test]
    fn collections_with_git_url_name_and_version() {
        let content = "collections:\n- name: https://github.com/lowlydba/lowlydba.sqlserver.git\n  type: git\n  version: 1.1.3\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "1.1.3");
    }

    // Ported: "extracts git@ dependencies" — ansible-galaxy/extract.spec.ts line 41
    #[test]
    fn collections_with_source_field_and_git_at_url() {
        let content = "collections:\n- name: community.docker\n  source: git@github.com:ansible-collections/community.docker\n  type: git\n  version: 2.7.5\n";
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].current_value, "2.7.5");
        // The source field is captured; dep is classified as GitHub
        assert!(
            matches!(deps[0].source, AnsibleGalaxySource::GitHub { .. }),
            "expected GitHub source"
        );
    }

    // Ported: "check if a requirements file of other systems returns null" — ansible-galaxy/extract.spec.ts line 61
    #[test]
    fn non_ansible_content_returns_empty() {
        let content = "dependencies:\n- name: nginx\n  version: 1.2.3\n  repository: https://charts.example.com\n";
        let deps = extract(content);
        assert!(deps.is_empty());
    }

    // Ported: "check if an empty file returns null" — ansible-galaxy/extract.spec.ts line 56
    #[test]
    fn blank_file_returns_no_deps() {
        assert!(extract("\n").is_empty());
    }

    const COLLECTIONS2: &str = r"---
collections:
  - name: geerlingguy.php_roles
    version: 0.9.3
    source: https://galaxy.ansible.com
  - name: davidban77.gns3
    version: 1.2.2
roles:
  - name: geerlingguy.java
    version: 1.9.6
  - name: geerlingguy.docker
    version: 2.9.0
";

    // Ported: "check collection style requirements file in reverse order and missing empty line" — ansible-galaxy/extract.spec.ts line 73
    #[test]
    fn collections_before_roles_extracts_all_four() {
        let deps = extract(COLLECTIONS2);
        assert_eq!(deps.len(), 4);
        assert!(
            deps.iter()
                .all(|d| d.skip_reason == Some(AnsibleGalaxySkipReason::GalaxyHosted))
        );
    }
}
