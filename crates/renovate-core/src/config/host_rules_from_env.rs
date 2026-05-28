//! Parse environment variables into Renovate host rules.
//!
//! Ports `lib/workers/global/config/parse/host-rules-from-env.ts`.
//!
//! # Environment variable format
//!
//! ```text
//! [RENOVATE_]<HOSTTYPE>[_<HOST_PART>...][_<AUTH_FIELD>]
//! ```
//!
//! - `RENOVATE_` prefix is optional and stripped before parsing.
//! - `<HOSTTYPE>` must be a known datasource ID or the `github` platform.
//! - Double underscores `__` become hyphens `-` in the host name.
//! - `<AUTH_FIELD>` is one of `token`, `username`, `password`,
//!   `httpscertificate`, `httpsprivatekey`, `httpscertificateauthority`.
//! - If host parts are present they are joined by `.` to form `matchHost`.

use std::collections::HashMap;

use serde_json::{Map, Value};

// ── Datasource and platform sets ──────────────────────────────────────────────

/// Known datasource IDs — mirrors `getDatasourceList()` from TypeScript.
pub static DATASOURCE_IDS: &[&str] = &[
    "aws-eks-addon",
    "aws-machine-image",
    "aws-rds",
    "azure-bicep-resource",
    "azure-pipelines-tasks",
    "azure-tags",
    "bazel",
    "bitbucket-server-tags",
    "bitbucket-tags",
    "bitrise",
    "buildpacks-registry",
    "cdnjs",
    "clojure",
    "cpan",
    "crate",
    "custom",
    "dart",
    "dart-version",
    "deb",
    "deno",
    "docker",
    "dotnet-version",
    "elm-package",
    "flutter-version",
    "forgejo-releases",
    "forgejo-tags",
    "galaxy",
    "galaxy-collection",
    "git-refs",
    "git-tags",
    "gitea-releases",
    "gitea-tags",
    "github-digest",
    "github-release-attachments",
    "github-releases",
    "github-runners",
    "github-tags",
    "gitlab-packages",
    "gitlab-releases",
    "gitlab-tags",
    "glasskube-packages",
    "go",
    "golang-version",
    "gradle-version",
    "hackage",
    "helm",
    "hermit",
    "hex",
    "jenkins-plugins",
    "jsr",
    "kubernetes-api",
    "maven",
    "nextcloud",
    "npm",
    "nuget",
    "orb",
    "packagist",
    "pod",
    "puppet-forge",
    "pypi",
    "repology",
    "ruby-version",
    "rubygems",
    "rust-version",
    "sbt-package",
    "sbt-plugin",
    "terraform-module",
    "terraform-provider",
    "typst",
    "unity3d",
    "unity3d-packages",
];

/// Platform names that support host rules.
static PLATFORMS: &[&str] = &["github"];

fn is_datasource(name: &str) -> bool {
    DATASOURCE_IDS.contains(&name)
}

fn is_platform(name: &str) -> bool {
    PLATFORMS.contains(&name)
}

// ── Auth field handling ───────────────────────────────────────────────────────

fn is_auth_field(s: &str) -> bool {
    matches!(s, "token" | "username" | "password")
}

fn is_https_auth_field(s: &str) -> bool {
    matches!(
        s,
        "httpscertificate" | "httpsprivatekey" | "httpscertificateauthority"
    )
}

fn restore_https_auth_field(s: &str) -> &'static str {
    match s {
        "httpsprivatekey" => "httpsPrivateKey",
        "httpscertificate" => "httpsCertificate",
        "httpscertificateauthority" => "httpsCertificateAuthority",
        _ => unreachable!(),
    }
}

fn canonical_field(s: &str) -> Option<&'static str> {
    if is_auth_field(s) {
        // token / username / password are kept as-is
        Some(match s {
            "token" => "token",
            "username" => "username",
            "password" => "password",
            _ => unreachable!(),
        })
    } else if is_https_auth_field(s) {
        Some(restore_https_auth_field(s))
    } else {
        None
    }
}

// ── npm env prefix guard ──────────────────────────────────────────────────────

static NPM_PREFIXES: &[&str] = &["npm_config_", "npm_lifecycle_", "npm_package_"];

// ── Core parsing ──────────────────────────────────────────────────────────────

/// Parse environment variables into a list of host-rule objects.
///
/// Each rule is a JSON object with at least `hostType` and one auth field.
/// If a host is encoded in the variable name, `matchHost` is also present.
pub fn host_rules_from_env(env: &HashMap<String, String>) -> Vec<Value> {
    let mut sorted_keys: Vec<&str> = env.keys().map(|s| s.as_str()).collect();
    sorted_keys.sort();

    // Keyed by (hostType, matchHost) for deduplication.
    let mut rules: Vec<Map<String, Value>> = Vec::new();

    for env_name in sorted_keys {
        let value = env.get(env_name).map(|s| s.as_str()).unwrap_or("");

        // Skip special GitHub tokens handled elsewhere.
        if env_name == "GITHUB_COM_TOKEN" || env_name == "RENOVATE_GITHUB_COM_TOKEN" {
            continue;
        }

        // Skip npm env variables.
        if NPM_PREFIXES
            .iter()
            .any(|prefix| env_name.starts_with(prefix))
        {
            continue;
        }

        // Strip optional RENOVATE_ prefix, lowercase, replace __ with -.
        let stripped = env_name
            .strip_prefix("RENOVATE_")
            .unwrap_or(env_name)
            .to_lowercase()
            .replace("__", "-");

        let mut parts: Vec<String> = stripped.split('_').map(|s| s.to_owned()).collect();

        let host_type = parts.remove(0);

        let has_enough_parts = is_datasource(&host_type)
            || (is_platform(&host_type) && parts.len() > 1);

        if !has_enough_parts {
            continue;
        }

        // The last part must be a recognized auth field.
        let suffix = match parts.last() {
            Some(s) => s.clone(),
            None => continue,
        };
        if canonical_field(&suffix).is_none() {
            continue;
        }
        parts.pop();

        let field_name = canonical_field(&suffix).unwrap();

        // The remaining parts form the host name (joined by '.').
        let match_host: Option<String> = match parts.len() {
            0 => None,
            1 => {
                // A single remaining part for a datasource → ambiguous; warn and skip.
                // (The TypeScript logs logger.warn here.)
                continue;
            }
            _ => Some(parts.join(".")),
        };

        // Find or create the rule for (hostType, matchHost).
        let existing = rules.iter_mut().find(|r| {
            r.get("hostType").and_then(|v| v.as_str()) == Some(&host_type)
                && r.get("matchHost").and_then(|v| v.as_str())
                    == match_host.as_deref()
        });

        if let Some(rule) = existing {
            rule.insert(field_name.to_owned(), Value::String(value.to_owned()));
        } else {
            let mut new_rule = Map::new();
            new_rule.insert("hostType".to_owned(), Value::String(host_type.clone()));
            if let Some(ref host) = match_host {
                new_rule.insert("matchHost".to_owned(), Value::String(host.clone()));
            }
            new_rule.insert(field_name.to_owned(), Value::String(value.to_owned()));
            rules.push(new_rule);
        }
    }

    rules.into_iter().map(Value::Object).collect()
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn env(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    fn rules(env: &HashMap<String, String>) -> Vec<Value> {
        host_rules_from_env(env)
    }

    // Ported: "supports docker username/password" — workers/global/config/parse/host-rules-from-env.spec.ts line 5
    #[test]
    fn host_rules_docker_user_pass() {
        let e = env(&[("DOCKER_USERNAME", "some-username"), ("DOCKER_PASSWORD", "some-password")]);
        let r = rules(&e);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0]["hostType"], "docker");
        assert_eq!(r[0]["username"], "some-username");
        assert_eq!(r[0]["password"], "some-password");
    }

    // Ported: "supports password-only" — workers/global/config/parse/host-rules-from-env.spec.ts line 19
    #[test]
    fn host_rules_npm_password_only() {
        let e = env(&[("NPM_PASSWORD", "some-password")]);
        let r = rules(&e);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0]["hostType"], "npm");
        assert_eq!(r[0]["password"], "some-password");
    }

    // Ported: "supports domain and host names with case insensitivity" — workers/global/config/parse/host-rules-from-env.spec.ts line 28
    #[test]
    fn host_rules_domain_and_host() {
        let e = env(&[
            ("GITHUB__TAGS_GITHUB_COM_TOKEN", "some-token"),
            ("pypi_my_CUSTOM_HOST_passWORD", "some-password"),
        ]);
        let r = rules(&e);
        // Find the github-tags rule
        let github_rule = r.iter().find(|v| v["hostType"] == "github-tags").unwrap();
        assert_eq!(github_rule["matchHost"], "github.com");
        assert_eq!(github_rule["token"], "some-token");
        // Find the pypi rule
        let pypi_rule = r.iter().find(|v| v["hostType"] == "pypi").unwrap();
        assert_eq!(pypi_rule["matchHost"], "my.custom.host");
        assert_eq!(pypi_rule["password"], "some-password");
    }

    // Ported: "regression test for #10937" — workers/global/config/parse/host-rules-from-env.spec.ts line 40
    #[test]
    fn host_rules_regression_10937() {
        let e = env(&[
            ("GIT__TAGS_GITLAB_EXAMPLE__DOMAIN_NET_USERNAME", "some-user"),
            ("GIT__TAGS_GITLAB_EXAMPLE__DOMAIN_NET_PASSWORD", "some-password"),
        ]);
        let r = rules(&e);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0]["hostType"], "git-tags");
        assert_eq!(r[0]["matchHost"], "gitlab.example-domain.net");
        assert_eq!(r[0]["username"], "some-user");
        assert_eq!(r[0]["password"], "some-password");
    }

    // Ported: "support RENOVATE_ prefixed host rules" — workers/global/config/parse/host-rules-from-env.spec.ts line 55
    #[test]
    fn host_rules_renovate_prefix() {
        let e = env(&[("RENOVATE_GITHUB__TAGS_GITHUB_COM_TOKEN", "some-token")]);
        let r = rules(&e);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0]["matchHost"], "github.com");
        assert_eq!(r[0]["token"], "some-token");
    }

    // Ported: "supports renovate in the env variable" — workers/global/config/parse/host-rules-from-env.spec.ts line 65
    #[test]
    fn host_rules_renovate_in_var() {
        let e = env(&[
            ("PYPI_MY_RENOVATE_HOST_PASSWORD", "some-password"),
            ("RENOVATE_DOCKER_MY_RENOVATE_HOST_PASSWORD", "docker-password"),
        ]);
        let r = rules(&e);
        let pypi_rule = r.iter().find(|v| v["hostType"] == "pypi").unwrap();
        assert_eq!(pypi_rule["matchHost"], "my.renovate.host");
        assert_eq!(pypi_rule["password"], "some-password");
        let docker_rule = r.iter().find(|v| v["hostType"] == "docker").unwrap();
        assert_eq!(docker_rule["matchHost"], "my.renovate.host");
        assert_eq!(docker_rule["password"], "docker-password");
    }

    // Ported: "support https authentication options" — workers/global/config/parse/host-rules-from-env.spec.ts line 77
    #[test]
    fn host_rules_https_auth() {
        let e = env(&[
            ("GITHUB_SOME_GITHUB__ENTERPRISE_HOST_HTTPSPRIVATEKEY", "private-key"),
            ("GITHUB_SOME_GITHUB__ENTERPRISE_HOST_HTTPSCERTIFICATE", "certificate"),
            ("GITHUB_SOME_GITHUB__ENTERPRISE_HOST_HTTPSCERTIFICATEAUTHORITY", "certificate-authority"),
        ]);
        let r = rules(&e);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0]["hostType"], "github");
        assert_eq!(r[0]["matchHost"], "some.github-enterprise.host");
        assert_eq!(r[0]["httpsPrivateKey"], "private-key");
        assert_eq!(r[0]["httpsCertificate"], "certificate");
        assert_eq!(r[0]["httpsCertificateAuthority"], "certificate-authority");
    }

    // Ported: "make sure {{PLATFORM}}_TOKEN will not be picked up" — workers/global/config/parse/host-rules-from-env.spec.ts line 95
    #[test]
    fn host_rules_platform_token_skipped() {
        let e = env(&[("GITHUB_TOKEN", "private-key")]);
        let r = rules(&e);
        assert_eq!(r.len(), 0);
    }

    // Ported: "supports datasource env token" — workers/global/config/parse/host-rules-from-env.spec.ts line 106
    #[test]
    fn host_rules_datasource_token() {
        let e = env(&[("PYPI_TOKEN", "some-token")]);
        let r = rules(&e);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0]["hostType"], "pypi");
        assert_eq!(r[0]["token"], "some-token");
    }

    // Ported: "supports platform env token" — workers/global/config/parse/host-rules-from-env.spec.ts line 115
    #[test]
    fn host_rules_platform_token() {
        let e = env(&[
            ("GITHUB_COM_TOKEN", "this-should-be-ignored-here"),
            ("GITHUB_SOME_GITHUB__ENTERPRISE_HOST_TOKEN", "some-token"),
        ]);
        let r = rules(&e);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0]["hostType"], "github");
        assert_eq!(r[0]["matchHost"], "some.github-enterprise.host");
        assert_eq!(r[0]["token"], "some-token");
    }

    // Ported: "rejects incomplete datasource env token" — workers/global/config/parse/host-rules-from-env.spec.ts line 130
    #[test]
    fn host_rules_incomplete_token() {
        let e = env(&[("PYPI_FOO_TOKEN", "some-token")]);
        let r = rules(&e);
        assert_eq!(r.len(), 0); // single middle part → warn + skip
    }

    // Ported: "rejects npm env" — workers/global/config/parse/host-rules-from-env.spec.ts line 137
    #[test]
    fn host_rules_npm_env_skipped() {
        let e = env(&[("npm_package_devDependencies__types_registry_auth_token", "4.2.0")]);
        let r = rules(&e);
        assert_eq!(r.len(), 0);
    }
}
