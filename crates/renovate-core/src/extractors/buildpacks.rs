//! Cloud Native Buildpacks `project.toml` extractor.
//!
//! Parses `project.toml` files (CNB Project Descriptor format) and extracts
//! buildpack references for version lookup.
//!
//! Renovate reference:
//! - `lib/modules/manager/buildpacks/extract.ts`
//! - `lib/modules/manager/buildpacks/schema.ts`
//! - Pattern: `(^|/)project\.toml$`
//! - Datasources: `buildpacks-registry`, `docker` (Docker entries skipped for now)
//!
//! ## Supported reference forms
//!
//! ```toml
//! [io.buildpacks]
//! builder = "registry.corp/builder/noble:1.1.1"  # Docker image — skipped
//!
//! [[io.buildpacks.group]]
//! id = "heroku/nodejs"
//! version = "3.3.3"                               # BuildpacksRegistry — actionable
//!
//! [[io.buildpacks.group]]
//! uri = "urn:cnb:registry:heroku/php@2.2.2"       # BuildpacksRegistry — actionable
//!
//! [[io.buildpacks.group]]
//! uri = "docker://buildpacks/java:2.2.2"           # Docker — skipped
//! ```

use std::sync::LazyLock;

use regex::Regex;
use toml::Value;

/// Source for a buildpack dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildpacksSource {
    /// CNB registry-based buildpack (`id` + `version` or `urn:cnb:registry:`).
    Registry,
    /// Docker image reference — not yet looked up by this extractor.
    Docker,
    /// Unsupported or unrecognized URI scheme.
    Unsupported,
}

/// Skip reason for a buildpack dep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildpacksSkipReason {
    /// Docker image lookup not yet supported.
    DockerImage,
    /// No version specified.
    NoVersion,
    /// URI scheme not supported.
    UnsupportedUri,
}

/// A single buildpack dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildpacksDep {
    /// Buildpack name (e.g. `heroku/nodejs`).
    pub dep_name: String,
    /// Version string.
    pub current_value: String,
    /// Source routing.
    pub source: BuildpacksSource,
    /// Set when no lookup should be performed.
    pub skip_reason: Option<BuildpacksSkipReason>,
}

/// `urn:cnb:registry:namespace/name@version`
static CNB_URN_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:urn:cnb:registry:)?([a-z0-9\-.]+/[a-z0-9\-.]+)@(.+)$").unwrap()
});

/// `docker://image:tag` or bare `image:tag` (container image reference with colon-tag).
static DOCKER_REF_RE: LazyLock<Regex> = LazyLock::new(|| {
    // Matches docker:// prefix or a bare docker image ref with an explicit tag
    Regex::new(r"^docker://").unwrap()
});

/// Extract all buildpack dependencies from a `project.toml` file.
pub fn extract(content: &str) -> Vec<BuildpacksDep> {
    let Ok(root) = toml::from_str::<Value>(content) else {
        return Vec::new();
    };

    let Some(io) = root.get("io").and_then(|v| v.get("buildpacks")) else {
        return Vec::new();
    };

    let mut deps = Vec::new();

    // `builder = "image:tag"` — Docker image, skipped
    if let Some(Value::String(builder)) = io.get("builder")
        && !builder.is_empty()
    {
        deps.push(BuildpacksDep {
            dep_name: strip_docker_prefix(builder),
            current_value: String::new(),
            source: BuildpacksSource::Docker,
            skip_reason: Some(BuildpacksSkipReason::DockerImage),
        });
    }

    // `[[io.buildpacks.group]]` entries
    if let Some(Value::Array(groups)) = io.get("group") {
        for group in groups {
            if let Some(dep) = parse_group_entry(group) {
                deps.push(dep);
            }
        }
    }

    deps
}

fn parse_group_entry(group: &Value) -> Option<BuildpacksDep> {
    // `id` + optional `version` → BuildpacksRegistry
    if let Some(Value::String(id)) = group.get("id") {
        let version = group
            .get("version")
            .and_then(|v| v.as_str())
            .map(str::to_owned);
        return Some(match version {
            Some(v) => BuildpacksDep {
                dep_name: id.clone(),
                current_value: v,
                source: BuildpacksSource::Registry,
                skip_reason: None,
            },
            None => BuildpacksDep {
                dep_name: id.clone(),
                current_value: String::new(),
                source: BuildpacksSource::Registry,
                skip_reason: Some(BuildpacksSkipReason::NoVersion),
            },
        });
    }

    // `uri` — classify by scheme
    if let Some(Value::String(uri)) = group.get("uri") {
        // `urn:cnb:registry:namespace/name@version` or `namespace/name@version`
        if let Some(cap) = CNB_URN_RE.captures(uri) {
            return Some(BuildpacksDep {
                dep_name: cap[1].to_owned(),
                current_value: cap[2].to_owned(),
                source: BuildpacksSource::Registry,
                skip_reason: None,
            });
        }

        // `docker://image:tag` — Docker image
        if DOCKER_REF_RE.is_match(uri) {
            return Some(BuildpacksDep {
                dep_name: strip_docker_prefix(uri),
                current_value: String::new(),
                source: BuildpacksSource::Docker,
                skip_reason: Some(BuildpacksSkipReason::DockerImage),
            });
        }

        // Bare docker image ref: `image:tag` or `registry/image:tag`
        if uri.contains(':') && !uri.contains("://") {
            let tag = uri.rsplit(':').next().unwrap_or("");
            let image = &uri[..uri.rfind(':').unwrap_or(uri.len())];
            return Some(BuildpacksDep {
                dep_name: image.to_owned(),
                current_value: tag.to_owned(),
                source: BuildpacksSource::Docker,
                skip_reason: Some(BuildpacksSkipReason::DockerImage),
            });
        }

        // Unsupported URI (file://, foo://, etc.)
        if uri.contains("://") {
            return Some(BuildpacksDep {
                dep_name: uri.clone(),
                current_value: String::new(),
                source: BuildpacksSource::Unsupported,
                skip_reason: Some(BuildpacksSkipReason::UnsupportedUri),
            });
        }
    }

    None
}

fn strip_docker_prefix(s: &str) -> String {
    s.trim_start_matches("docker://")
        .trim_start_matches("docker:/")
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
[_]
schema-version = "0.2"

[io.buildpacks]
builder = "registry.corp/builder/noble:1.1.1"

[[io.buildpacks.group]]
id = "heroku/nodejs"
version = "3.3.3"

[[io.buildpacks.group]]
uri = "urn:cnb:registry:example/foo@1.0.0"

[[io.buildpacks.group]]
uri = "docker://buildpacks/java:2.2.2"

[[io.buildpacks.group]]
uri = "buildpacks/nodejs:3.3.3"

[[io.buildpacks.group]]
id = "no/version"
"#;

    #[test]
    fn extracts_registry_deps() {
        let deps = extract(SAMPLE);
        let registry: Vec<_> = deps
            .iter()
            .filter(|d| d.source == BuildpacksSource::Registry)
            .collect();
        assert!(registry.iter().any(|d| d.dep_name == "heroku/nodejs"
            && d.current_value == "3.3.3"
            && d.skip_reason.is_none()));
        assert!(
            registry
                .iter()
                .any(|d| d.dep_name == "example/foo" && d.current_value == "1.0.0")
        );
    }

    #[test]
    fn skips_docker_refs() {
        let deps = extract(SAMPLE);
        let docker: Vec<_> = deps
            .iter()
            .filter(|d| d.source == BuildpacksSource::Docker)
            .collect();
        // builder + docker:// uri + bare uri with tag
        assert!(docker.len() >= 2);
        for d in &docker {
            assert_eq!(d.skip_reason, Some(BuildpacksSkipReason::DockerImage));
        }
    }

    #[test]
    fn no_version_skipped() {
        let deps = extract(SAMPLE);
        let no_ver: Vec<_> = deps
            .iter()
            .filter(|d| d.skip_reason == Some(BuildpacksSkipReason::NoVersion))
            .collect();
        assert!(!no_ver.is_empty());
        assert!(no_ver.iter().any(|d| d.dep_name == "no/version"));
    }

    #[test]
    fn empty_file_returns_empty() {
        assert!(extract("").is_empty());
    }

    // Ported: "returns null for empty package.toml" — buildpacks/extract.spec.ts line 11
    #[test]
    fn no_io_buildpacks_returns_empty() {
        assert!(extract("[_]\nschema-version = \"0.2\"\n").is_empty());
    }

    // Ported: "returns null for invalid files" — buildpacks/extract.spec.ts line 7
    #[test]
    fn invalid_toml_returns_empty() {
        assert!(extract("not a project toml").is_empty());
    }
}
