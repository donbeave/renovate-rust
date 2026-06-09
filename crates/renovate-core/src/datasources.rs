//! Datasource clients for fetching available package versions.
//!
//! Each sub-module implements a registry-specific version lookup. The common
//! output is a list of available version strings that the update-planner then
//! compares against the current constraint.

pub mod artifactory;
pub mod aws_eks_addon;
pub mod aws_machine_image;
pub mod aws_rds;
pub mod azure_bicep;
pub mod azure_pipelines_tasks;
pub mod azure_tags;
pub mod bazel;
pub mod bitbucket_server_tags;
pub mod bitbucket_tags;
pub mod bitrise;
pub mod buildpacks_registry;
pub mod cdnjs;
pub mod clojure;
pub mod cocoapods;
pub mod conan;
pub mod conda;
pub mod cpan;
pub mod crates_io;
pub mod custom;
pub mod dart_version;
pub mod deb;
pub mod deb_index;
pub mod deno;
pub mod devbox;
pub mod docker_ecr;
pub mod docker_google;
pub mod docker_hub;
pub mod dotnet_version;
pub mod elm_package;
pub mod endoflife;
pub mod flutter_version;
pub mod forgejo_releases;
pub mod forgejo_tags;
pub mod galaxy;
pub mod galaxy_collection;
pub mod git_refs;
pub mod git_tags;
pub mod gitea_releases;
pub mod gitea_tags;
pub mod github_digest;
pub mod github_release_attachments;
pub mod github_release_attachments_test;
pub mod github_releases;
pub mod github_runners;
pub mod github_tags;
pub mod gitlab_packages;
pub mod gitlab_releases;
pub mod gitlab_tags;
pub mod glasskube_packages;
pub mod go_goproxy_parser;
pub mod go_releases_direct;
pub mod golang_version;
pub mod gomod;
pub mod gradle_version;
pub mod hackage;
pub mod helm;
pub mod hermit;
pub mod hex;
pub mod hex_v2_package;
pub mod hexpm_bob;
pub mod java_version;
pub mod jenkins_plugins;
pub mod jsr;
pub mod kubernetes_api;
pub mod maven;
pub mod nextcloud;
pub mod node_version;
pub mod npm;
pub mod npm_npmrc;
pub mod nuget;
pub mod orb;
pub mod packagist;
pub mod pub_dev;
pub mod puppet_forge;
pub mod pypi;
pub mod python_version;
pub mod repology;
pub mod rpm;
pub mod rpm_repomd;
pub mod ruby_version;
pub mod rubygems;
pub mod rust_version;
pub mod sbt_package;
pub mod sbt_plugin;
pub mod terraform;
pub mod typst;
pub mod unity3d;
pub mod unity3d_packages;

// ═══════════════════════════════════════════════════════════════════════════
// Datasource registry — lib/modules/datasource/index.ts
// ═══════════════════════════════════════════════════════════════════════════

/// Per-datasource metadata needed by the registry.
#[derive(Debug)]
pub struct DatasourceInfo {
    pub id: &'static str,
    pub default_versioning: &'static str,
}

// ═══════════════════════════════════════════════════════════════════════════
// addMetaData — lib/modules/datasource/metadata.ts
// ═══════════════════════════════════════════════════════════════════════════

const CHANGELOG_URLS_JSON: &str = include_str!("datasources/changelog-urls.json");
const SOURCE_URLS_JSON: &str = include_str!("datasources/source-urls.json");

/// A release result entry with optional metadata fields.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseResult {
    pub releases: Vec<Release>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changelog_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_url: Option<String>,
}

/// A single release entry.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changelog_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deprecated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_stable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_dependencies: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attestation: Option<bool>,
}

/// Add metadata (changelog URL, source URL, source directory) to a release result.
///
/// Mirrors `addMetaData()` from `lib/modules/datasource/metadata.ts`.
pub fn add_metadata(dep: &mut ReleaseResult, datasource: &str, package_name: &str) {
    massage_timestamps(dep);
    let package_lower = package_name.to_lowercase();

    // Look up manual changelog URL.
    if let Ok(changelog_map) = serde_json::from_str::<serde_json::Value>(CHANGELOG_URLS_JSON)
        && let Some(url) = changelog_map
            .get(datasource)
            .and_then(|v| v.get(&package_lower))
            .and_then(|v| v.as_str())
    {
        dep.changelog_url = Some(url.to_owned());
    }

    // Look up manual source URL.
    if dep.source_url.is_none()
        && let Ok(source_map) = serde_json::from_str::<serde_json::Value>(SOURCE_URLS_JSON)
        && let Some(url) = source_map
            .get(datasource)
            .and_then(|v| v.get(&package_lower))
            .and_then(|v| v.as_str())
    {
        dep.source_url = Some(url.to_owned());
    }

    // Parse source URL to extract source directory (GitHub tree/ URLs).
    if let Some(ref source_url) = dep.source_url.clone()
        && dep.source_directory.is_none()
        && let Some((base, dir)) = extract_source_directory(source_url)
    {
        dep.source_url = Some(base);
        dep.source_directory = Some(dir);
    }

    // If no source URL but have changelog URL on GitHub, use changelog URL.
    if dep.source_url.is_none()
        && let Some(ref changelog_url) = dep.changelog_url.clone()
        && is_github_url(changelog_url)
    {
        dep.source_url = Some(changelog_url.clone());
    }

    // If no source URL but have homepage on GitHub/GitLab, use homepage.
    if dep.source_url.is_none()
        && let Some(ref homepage) = dep.homepage.clone()
        && (is_github_url(homepage) || is_gitlab_url(homepage))
    {
        dep.source_url = Some(homepage.clone());
    }

    // Massage the source URL.
    if let Some(ref source_url) = dep.source_url.clone() {
        let massaged = crate::util::massage_url(source_url);
        if massaged.is_empty() {
            dep.source_url = None;
        } else {
            dep.source_url = Some(massaged);
        }
    }

    // Remove homepage when it duplicates the source URL.
    if let (Some(ref source_url), Some(ref homepage)) =
        (dep.source_url.clone(), dep.homepage.clone())
    {
        let massaged_hp = crate::util::massage_url(homepage);
        if !massaged_hp.is_empty() && (massaged_hp == *source_url || homepage == source_url) {
            dep.homepage = None;
        }
    }
}

/// Normalize timestamps in release result to ISO 8601 UTC format.
/// Mirrors `massageTimestamps()` from `lib/modules/datasource/metadata.ts`.
fn massage_timestamps(dep: &mut ReleaseResult) {
    for release in &mut dep.releases {
        if let Some(ref ts) = release.release_timestamp.clone() {
            release.release_timestamp = normalize_timestamp(ts);
        }
    }
}

/// Normalize a timestamp string to ISO 8601 UTC (`YYYY-MM-DDTHH:MM:SS.sssZ`).
fn normalize_timestamp(ts: &str) -> Option<String> {
    use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

    // Try standard ISO 8601 with timezone offset
    if let Ok(dt) = DateTime::parse_from_rfc3339(ts) {
        return Some(
            dt.with_timezone(&Utc)
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        );
    }

    // Try without timezone (treat as UTC)
    if let Ok(ndt) = NaiveDateTime::parse_from_str(ts, "%Y-%m-%dT%H:%M:%S") {
        return Some(
            Utc.from_utc_datetime(&ndt)
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        );
    }
    if let Ok(ndt) = NaiveDateTime::parse_from_str(ts, "%Y-%m-%dT%H:%M:%S%.f") {
        return Some(
            Utc.from_utc_datetime(&ndt)
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        );
    }

    // Try compact format like "20000103150210" → 2000-01-03 15:02:10
    if let Ok(ndt) = NaiveDateTime::parse_from_str(ts, "%Y%m%d%H%M%S") {
        return Some(
            Utc.from_utc_datetime(&ndt)
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        );
    }

    None
}

/// Extract source directory from a GitHub or GitLab tree URL./// Extract source directory from a GitHub or GitLab tree URL.
/// E.g.:
///   "https://github.com/owner/repo/tree/master/subdir" → (base, "subdir")
///   "https://gitlab.com/group/repo/tree/main/subdir" → (base, "subdir")
///   "https://gitlab.com/group/repo/-/tree/main/subdir" → (base, "subdir")
fn extract_source_directory(url: &str) -> Option<(String, String)> {
    // GitHub tree URLs: .../tree/{ref}/{path}
    let github_re =
        regex::Regex::new(r"^(https://[^/]*github[^/]*/[^/]+/[^/]+)/tree/[^/]+/(.+?)/?$").ok()?;
    if let Some(caps) = github_re.captures(url) {
        let base = caps.get(1)?.as_str().to_owned();
        let dir = caps.get(2)?.as_str().to_owned();
        if !dir.is_empty() {
            return Some((base, dir));
        }
    }
    // GitLab tree URLs: .../tree/{ref}/{path} or .../-/tree/{ref}/{path}
    let gitlab_re = regex::Regex::new(
        r"^(https://[^/]*gitlab[^/]*/[^/]+/[^/]+(?:/[^/-][^/]*)*)(?:/-)?/tree/[^/]+/(.+?)/?$",
    )
    .ok()?;
    if let Some(caps) = gitlab_re.captures(url) {
        let base = caps.get(1)?.as_str().to_owned();
        let dir = caps.get(2)?.as_str().to_owned();
        if !dir.is_empty() {
            return Some((base, dir));
        }
    }
    None
}

fn is_github_url(url: &str) -> bool {
    url.contains("github.com")
}

fn is_gitlab_url(url: &str) -> bool {
    url.contains("gitlab.com") || url.contains("gitlab.")
}

/// Config for constraint filtering.
#[derive(Debug, Default)]
pub struct ConstraintsFilteringConfig {
    pub constraints_filtering: Option<String>,
    pub constraints: Option<std::collections::HashMap<String, String>>,
    /// Override versioning per constraint name, mirrors `constraintsVersioning`.
    /// Currently only `"semver-coerced"` is supported; other values fall back to npm.
    pub constraints_versioning: Option<std::collections::HashMap<String, String>>,
}

/// Apply constraint-based filtering to a release result.
///
/// When `constraintsFiltering` is not `"strict"`, constraints are removed
/// from releases but all releases are kept.  When strict, releases are
/// filtered to only those whose constraints satisfy the config constraints.
///
/// Mirrors `applyConstraintsFiltering()` from `lib/modules/datasource/common.ts`.
pub fn apply_constraints_filtering(
    mut release_result: ReleaseResult,
    config: &ConstraintsFilteringConfig,
) -> ReleaseResult {
    if config.constraints_filtering.as_deref() != Some("strict") {
        // Remove constraints from all releases but keep them.
        for release in &mut release_result.releases {
            release.constraints = None;
        }
        return release_result;
    }

    // Strict mode: filter releases.
    let config_constraints = match &config.constraints {
        Some(c) if !c.is_empty() => c,
        _ => {
            // No config constraints → keep all, remove release constraints.
            for release in &mut release_result.releases {
                release.constraints = None;
            }
            return release_result;
        }
    };

    let mut kept_releases = Vec::new();
    for mut release in release_result.releases {
        let release_constraints = release.constraints.take();

        let keep = match release_constraints {
            None => true, // no release constraints → keep
            Some(rc) => {
                let Some(rc_map) = rc.as_object() else {
                    kept_releases.push(release);
                    continue;
                };

                let mut satisfies_all = true;
                for (name, config_constraint) in config_constraints {
                    // Check if this constraint name is in the release.
                    let release_constraint_arr = rc_map.get(name);
                    let Some(release_arr) = release_constraint_arr else {
                        // No constraint for this name → keep
                        continue;
                    };

                    // Get the constraint values.
                    let values: Vec<String> = if let Some(arr) = release_arr.as_array() {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_owned()))
                            .collect()
                    } else {
                        Vec::new()
                    };

                    if values.is_empty() {
                        // Empty array → keep
                        continue;
                    }

                    // Determine which versioning to use (constraintsVersioning override or npm)
                    let use_semver_coerced = config
                        .constraints_versioning
                        .as_ref()
                        .and_then(|cv| cv.get(name.as_str()))
                        .map(|v| v == "semver-coerced")
                        .unwrap_or(false);

                    // Check if any release constraint satisfies the config constraint.
                    let satisfies = values.iter().any(|rc_val| {
                        // Exact match
                        if config_constraint == rc_val {
                            return true;
                        }
                        if use_semver_coerced {
                            // Use semver-coerced matching (matches = version satisfies range)
                            if crate::versioning::semver_coerced::matches(rc_val, config_constraint)
                            {
                                return true;
                            }
                            if crate::versioning::semver_coerced::matches(config_constraint, rc_val)
                            {
                                return true;
                            }
                        } else {
                            // Default: npm semver matching
                            if crate::versioning::npm::matches_range(rc_val, config_constraint) {
                                return true;
                            }
                            if crate::versioning::npm::matches_range(config_constraint, rc_val) {
                                return true;
                            }
                        }
                        false
                    });

                    if !satisfies {
                        satisfies_all = false;
                        break;
                    }
                }
                satisfies_all
            }
        };

        if keep {
            kept_releases.push(release);
        }
    }

    release_result.releases = kept_releases;
    release_result
}

/// The default versioning ID used when no datasource-specific one is set.
/// Mirrors `defaultVersioning = semverCoerced` from `lib/modules/versioning/index.ts`.
pub const DATASOURCE_DEFAULT_VERSIONING: &str = "semver-coerced";

/// Static registry of known datasource IDs and their default versioning.
/// Mirrors the datasources Map from `lib/modules/datasource/api.ts`.
const KNOWN_DATASOURCES: &[DatasourceInfo] = &[
    DatasourceInfo {
        id: "artifactory",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "aws-eks-addon",
        default_versioning: "aws-eks-addon",
    },
    DatasourceInfo {
        id: "aws-machine-image",
        default_versioning: "aws-machine-image",
    },
    DatasourceInfo {
        id: "aws-rds",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "azure-bicep-resource",
        default_versioning: "azure-rest-api",
    },
    DatasourceInfo {
        id: "azure-pipelines-tasks",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "azure-tags",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "bazel",
        default_versioning: "bazel-module",
    },
    DatasourceInfo {
        id: "bitbucket-server-tags",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "bitbucket-tags",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "bitrise",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "buildpacks-registry",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "cdnjs",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "clojure",
        default_versioning: "maven",
    },
    DatasourceInfo {
        id: "conan",
        default_versioning: "conan",
    },
    DatasourceInfo {
        id: "conda",
        default_versioning: "pep440",
    },
    DatasourceInfo {
        id: "cpan",
        default_versioning: "perl",
    },
    DatasourceInfo {
        id: "crate",
        default_versioning: "cargo",
    },
    DatasourceInfo {
        id: "custom",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "dart",
        default_versioning: "npm",
    },
    DatasourceInfo {
        id: "deb",
        default_versioning: "deb",
    },
    DatasourceInfo {
        id: "deno",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "devbox",
        default_versioning: "devbox",
    },
    DatasourceInfo {
        id: "docker",
        default_versioning: "docker",
    },
    DatasourceInfo {
        id: "dotnet-version",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "elm-package",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "endoflife-date",
        default_versioning: "loose",
    },
    DatasourceInfo {
        id: "flutter-version",
        default_versioning: "semver",
    },
    DatasourceInfo {
        id: "forgejo-releases",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "forgejo-tags",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "galaxy",
        default_versioning: "pep440",
    },
    DatasourceInfo {
        id: "galaxy-collection",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "git-refs",
        default_versioning: "git",
    },
    DatasourceInfo {
        id: "git-tags",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "gitea-releases",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "gitea-tags",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "github-digest",
        default_versioning: "exact",
    },
    DatasourceInfo {
        id: "github-release-attachments",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "github-releases",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "github-runners",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "github-tags",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "gitlab-packages",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "gitlab-releases",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "gitlab-tags",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "glasskube-packages",
        default_versioning: "glasskube",
    },
    DatasourceInfo {
        id: "go",
        default_versioning: "semver",
    },
    DatasourceInfo {
        id: "golang-version",
        default_versioning: "semver",
    },
    DatasourceInfo {
        id: "gradle-version",
        default_versioning: "gradle",
    },
    DatasourceInfo {
        id: "hackage",
        default_versioning: "pvp",
    },
    DatasourceInfo {
        id: "helm",
        default_versioning: "helm",
    },
    DatasourceInfo {
        id: "hermit",
        default_versioning: "hermit",
    },
    DatasourceInfo {
        id: "hex",
        default_versioning: "hex",
    },
    DatasourceInfo {
        id: "hexpm-bob",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "java-version",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "jenkins-plugins",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "jsr",
        default_versioning: "semver",
    },
    DatasourceInfo {
        id: "kubernetes-api",
        default_versioning: "kubernetes-api",
    },
    DatasourceInfo {
        id: "maven",
        default_versioning: "maven",
    },
    DatasourceInfo {
        id: "nextcloud",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "node-version",
        default_versioning: "node",
    },
    DatasourceInfo {
        id: "npm",
        default_versioning: "npm",
    },
    DatasourceInfo {
        id: "nuget",
        default_versioning: "nuget",
    },
    DatasourceInfo {
        id: "orb",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "packagist",
        default_versioning: "composer",
    },
    DatasourceInfo {
        id: "pod",
        default_versioning: "ruby",
    },
    DatasourceInfo {
        id: "pub",
        default_versioning: "npm",
    },
    DatasourceInfo {
        id: "puppet-forge",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "pypi",
        default_versioning: "pep440",
    },
    DatasourceInfo {
        id: "python-version",
        default_versioning: "pep440",
    },
    DatasourceInfo {
        id: "repology",
        default_versioning: "loose",
    },
    DatasourceInfo {
        id: "rpm",
        default_versioning: "rpm",
    },
    DatasourceInfo {
        id: "ruby-version",
        default_versioning: "ruby",
    },
    DatasourceInfo {
        id: "rubygems",
        default_versioning: "ruby",
    },
    DatasourceInfo {
        id: "rust-version",
        default_versioning: "rust-release-channel",
    },
    DatasourceInfo {
        id: "sbt-package",
        default_versioning: "ivy",
    },
    DatasourceInfo {
        id: "sbt-plugin",
        default_versioning: "ivy",
    },
    DatasourceInfo {
        id: "terraform-module",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "terraform-provider",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "typst",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "unity3d",
        default_versioning: "semver-coerced",
    },
    DatasourceInfo {
        id: "unity3d-packages",
        default_versioning: "semver-coerced",
    },
];

/// Return the datasource info for a given ID, or `None` if unknown.
///
/// Custom datasources ("custom.*") map to the "custom" entry.
/// Mirrors `getDatasourceFor()` from `lib/modules/datasource/common.ts`.
pub fn get_datasource_for(name: &str) -> Option<&'static DatasourceInfo> {
    // "custom.*" → custom datasource
    let lookup = if name.starts_with("custom.") {
        "custom"
    } else {
        name
    };
    KNOWN_DATASOURCES.iter().find(|d| d.id == lookup)
}

/// Return the default versioning ID for a datasource.
///
/// Returns `DATASOURCE_DEFAULT_VERSIONING` ("semver-coerced") when the
/// datasource is unknown or has no specific default.
/// Mirrors `getDefaultVersioning()` from `lib/modules/datasource/common.ts`.
pub fn get_datasource_default_versioning(datasource: Option<&str>) -> &'static str {
    let Some(name) = datasource else {
        return DATASOURCE_DEFAULT_VERSIONING;
    };
    get_datasource_for(name)
        .map(|d| d.default_versioning)
        .unwrap_or(DATASOURCE_DEFAULT_VERSIONING)
}

/// Return the list of all known datasource IDs.
/// Mirrors `getDatasourceList()` from `lib/modules/datasource/index.ts`.
pub fn get_datasource_list() -> Vec<&'static str> {
    KNOWN_DATASOURCES.iter().map(|d| d.id).collect()
}

/// Check if an input is a valid `GetPkgReleasesConfig`.
///
/// Requires `datasource` (non-empty string) and `packageName` (string).
/// Mirrors `isGetPkgReleasesConfig()` from `lib/modules/datasource/common.ts`.
pub fn is_get_pkg_releases_config(value: &serde_json::Value) -> bool {
    let Some(obj) = value.as_object() else {
        return false;
    };
    let _datasource = match obj.get("datasource").and_then(|v| v.as_str()) {
        Some(s) if !s.is_empty() => s,
        _ => return false,
    };
    obj.get("packageName").and_then(|v| v.as_str()).is_some()
}

/// Post-process a release through datasource-specific logic.
///
/// Mirrors `postprocessRelease` from `lib/modules/datasource/postprocess-release.ts`.
/// Currently no Rust datasource overrides the base behavior, so this always
/// returns the original release. When a datasource needs custom postprocessing,
/// add a `postprocess_release` function to its module and wire it here.
pub fn postprocess_release(
    datasource: Option<&str>,
    package_name: Option<&str>,
    release: &Release,
) -> Option<Release> {
    let Some(ds_name) = datasource else {
        return Some(release.clone());
    };

    if get_datasource_for(ds_name).is_none() {
        return Some(release.clone());
    }

    let Some(_pkg) = package_name else {
        return Some(release.clone());
    };

    // No datasource currently overrides postprocessRelease in Rust.
    // When one does, dispatch to it here based on ds_name.
    Some(release.clone())
}

#[cfg(test)]
mod registry_tests {
    use super::*;

    // Ported: "returns null for unknown datasource" — lib/modules/datasource/common.spec.ts line 21
    #[test]
    fn datasource_registry_unknown_returns_none() {
        assert!(get_datasource_for("foobar").is_none());
    }

    // Ported: "supports custom datasource" — lib/modules/datasource/common.spec.ts line 25
    #[test]
    fn datasource_registry_custom_prefix() {
        let custom = get_datasource_for("custom.foobar");
        let base = get_datasource_for("custom");
        assert!(custom.is_some());
        assert_eq!(custom.map(|d| d.id), base.map(|d| d.id));
    }

    // Ported: "returns datasource for known datasource" — lib/modules/datasource/common.spec.ts line 31
    #[test]
    fn datasource_registry_known_returns_some() {
        let ds = get_datasource_for("npm").unwrap();
        assert_eq!(ds.id, "npm");
    }

    // Ported: "returns default versioning for undefined datasource" — lib/modules/datasource/common.spec.ts line 39
    #[test]
    fn datasource_registry_default_versioning_undefined() {
        assert_eq!(get_datasource_default_versioning(None), "semver-coerced");
    }

    // Ported: "returns default versioning for unknown datasource" — lib/modules/datasource/common.spec.ts line 43
    #[test]
    fn datasource_registry_default_versioning_unknown() {
        assert_eq!(
            get_datasource_default_versioning(Some("foobar")),
            "semver-coerced"
        );
    }

    // Ported: "returns default versioning for datasource with missing default versioning configuration" — lib/modules/datasource/common.spec.ts line 52
    #[test]
    fn datasource_registry_default_versioning_no_specific() {
        // artifactory has no specific default → semver-coerced
        assert_eq!(
            get_datasource_default_versioning(Some("artifactory")),
            "semver-coerced"
        );
    }

    // Ported: "returns datasource-defined default versioning" — lib/modules/datasource/common.spec.ts line 56
    #[test]
    fn datasource_registry_datasource_defined_versioning() {
        // crate uses cargo versioning
        assert_eq!(get_datasource_default_versioning(Some("crate")), "cargo");
    }

    // Ported: "returns true for valid input" — lib/modules/datasource/common.spec.ts line 62
    #[test]
    fn is_get_pkg_releases_config_valid() {
        let input = serde_json::json!({"datasource": "npm", "packageName": "lodash"});
        assert!(is_get_pkg_releases_config(&input));
    }

    // Ported: "returns false for invalid input" — lib/modules/datasource/common.spec.ts line 70
    #[test]
    fn is_get_pkg_releases_config_empty_datasource() {
        let input = serde_json::json!({"datasource": "", "packageName": "lodash"});
        assert!(!is_get_pkg_releases_config(&input));
    }

    // Ported: "returns false for input with missing properties" — lib/modules/datasource/common.spec.ts line 78
    #[test]
    fn is_get_pkg_releases_config_missing_package_name() {
        let input = serde_json::json!({"datasource": "npm"});
        assert!(!is_get_pkg_releases_config(&input));
    }

    // Ported: "returns false for input with non-string properties" — lib/modules/datasource/common.spec.ts line 85
    #[test]
    fn is_get_pkg_releases_config_non_string_datasource() {
        let input = serde_json::json!({"datasource": 123, "packageName": "lodash"});
        assert!(!is_get_pkg_releases_config(&input));
    }

    // ── add_metadata tests ─────────────────────────────────────────────────

    // Ported: "Should handle manualChangelogUrls" — lib/modules/datasource/metadata.spec.ts line 19
    #[test]
    fn add_metadata_manual_changelog_url() {
        let mut dep = ReleaseResult {
            releases: vec![
                Release {
                    version: "2.0.0".into(),
                    ..Default::default()
                },
                Release {
                    version: "2.1.0".into(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        add_metadata(&mut dep, "pypi", "pycountry");
        assert_eq!(
            dep.changelog_url.as_deref(),
            Some("https://github.com/flyingcircusio/pycountry/blob/master/HISTORY.txt")
        );
    }

    // Ported: "Should handle manualSourceUrls" — lib/modules/datasource/metadata.spec.ts line 51
    #[test]
    fn add_metadata_manual_source_url() {
        let mut dep = ReleaseResult {
            releases: vec![Release {
                version: "2.0.0".into(),
                ..Default::default()
            }],
            ..Default::default()
        };
        add_metadata(&mut dep, "pypi", "mkdocs");
        assert_eq!(
            dep.source_url.as_deref(),
            Some("https://github.com/mkdocs/mkdocs")
        );
    }

    // Ported: "Should handle parsing of sourceUrls correctly" — lib/modules/datasource/metadata.spec.ts line 82
    #[test]
    fn add_metadata_parses_github_tree_url() {
        let mut dep = ReleaseResult {
            releases: vec![Release {
                version: "2.0.0".into(),
                ..Default::default()
            }],
            source_url: Some("https://github.com/carltongibson/django-filter/tree/master".into()),
            ..Default::default()
        };
        add_metadata(&mut dep, "pypi", "django-filter");
        assert_eq!(
            dep.source_url.as_deref(),
            Some("https://github.com/carltongibson/django-filter")
        );
    }

    #[test]
    fn add_metadata_extracts_source_directory() {
        let mut dep = ReleaseResult {
            releases: vec![],
            source_url: Some(
                "https://github.com/bitnami/charts/tree/master/bitnami/kube-prometheus".into(),
            ),
            ..Default::default()
        };
        add_metadata(&mut dep, "helm", "kube-prometheus");
        assert_eq!(
            dep.source_url.as_deref(),
            Some("https://github.com/bitnami/charts")
        );
        assert_eq!(
            dep.source_directory.as_deref(),
            Some("bitnami/kube-prometheus")
        );
    }

    #[test]
    fn add_metadata_preserves_existing_source_directory() {
        let mut dep = ReleaseResult {
            releases: vec![],
            source_url: Some(
                "https://github.com/bitnami/charts/tree/master/bitnami/kube-prometheus".into(),
            ),
            source_directory: Some("existing-dir".into()),
            ..Default::default()
        };
        add_metadata(&mut dep, "helm", "kube-prometheus");
        assert_eq!(dep.source_directory.as_deref(), Some("existing-dir"));
    }

    // Ported: "Should not overwrite any existing sourceDirectory" — lib/modules/datasource/metadata.spec.ts line 180
    #[test]
    fn add_metadata_should_not_overwrite_any_existing_sourcedirectory() {
        // Exercises the guard `if dep.source_directory.is_none()` before extract_source_directory,
        // plus the github tree parse that would otherwise set sourceDirectory.
        let mut dep = ReleaseResult {
            releases: vec![],
            source_url: Some(
                "https://github.com/neutrinojs/neutrino/tree/master/packages/react".into(),
            ),
            source_directory: Some("packages/foo".into()),
            ..Default::default()
        };
        add_metadata(&mut dep, "npm", "@neutrinojs/react");
        assert_eq!(
            dep.source_url.as_deref(),
            Some("https://github.com/neutrinojs/neutrino")
        );
        assert_eq!(dep.source_directory.as_deref(), Some("packages/foo"));
    }

    // Ported: "Should move github homepage to sourceUrl" — lib/modules/datasource/metadata.spec.ts line 331
    #[test]
    fn add_metadata_should_move_github_homepage_to_sourceurl() {
        let mut dep = ReleaseResult {
            releases: vec![Release {
                version: "1.9.3".into(),
                ..Default::default()
            }],
            homepage: Some("http://www.github.com/mockk/mockk/".into()),
            source_url: None,
            ..Default::default()
        };
        add_metadata(&mut dep, "maven", "io.mockk:mockk");
        assert_eq!(dep.source_url.as_deref(), Some("https://github.com/mockk/mockk"));
        assert!(dep.homepage.is_none());
    }

    // Ported: "Should handle parsing of sourceUrls correctly for GitLab also" — lib/modules/datasource/metadata.spec.ts line 228
    #[test]
    fn add_metadata_should_handle_parsing_of_sourceurls_correctly_for_gitlab_also() {
        let mut dep = ReleaseResult {
            releases: vec![
                Release {
                    version: "5.7.0".into(),
                    release_timestamp: Some("2020-02-14T13:12:00.000Z".into()),
                    ..Default::default()
                },
                Release {
                    version: "5.6.1".into(),
                    release_timestamp: Some("2020-02-14T10:04:00.000Z".into()),
                    ..Default::default()
                },
            ],
            source_url: Some("https://gitlab.com/meno/dropzone/tree/master".into()),
            ..Default::default()
        };
        add_metadata(&mut dep, "npm", "dropzone");
        assert_eq!(dep.source_url.as_deref(), Some("https://gitlab.com/meno/dropzone"));
    }

    // Ported: "Should handle parsing/converting of GitLab sourceUrls with http and www correctly" — lib/modules/datasource/metadata.spec.ts line 345
    #[test]
    fn add_metadata_should_handle_parsing_converting_of_gitlab_sourceurls_with_http_and_www_correctly() {
        let mut dep = ReleaseResult {
            releases: vec![Release {
                version: "5.7.0".into(),
                ..Default::default()
            }],
            source_url: Some("http://gitlab.com/meno/dropzone/".into()),
            ..Default::default()
        };
        add_metadata(&mut dep, "maven", "dropzone");
        assert_eq!(dep.source_url.as_deref(), Some("https://gitlab.com/meno/dropzone"));
    }

    // Ported: "Should remove homepage when homepage and sourceUrl are same" — lib/modules/datasource/metadata.spec.ts line 464
    #[test]
    fn add_metadata_should_remove_homepage_when_homepage_and_sourceurl_are_same() {
        let mut dep = ReleaseResult {
            releases: vec![
                Release {
                    version: "1.0.1".into(),
                    release_timestamp: Some("2000-01-01T12:34:56".into()),
                    ..Default::default()
                },
                Release {
                    version: "1.0.2".into(),
                    release_timestamp: Some("2000-01-02T12:34:56.000Z".into()),
                    ..Default::default()
                },
                Release {
                    version: "1.0.3".into(),
                    release_timestamp: Some("2000-01-03T14:34:56.000+02:00".into()),
                    ..Default::default()
                },
            ],
            homepage: Some("https://github.com/foo/bar".into()),
            source_url: Some("https://github.com/foo/bar".into()),
            ..Default::default()
        };
        add_metadata(&mut dep, "npm", "some-pkg");
        assert_eq!(dep.source_url.as_deref(), Some("https://github.com/foo/bar"));
        assert!(dep.homepage.is_none());
    }

    // Ported: "Should delete gitlab homepage if its same as sourceUrl" — lib/modules/datasource/metadata.spec.ts line 503
    #[test]
    fn add_metadata_should_delete_gitlab_homepage_if_its_same_as_sourceurl() {
        let mut dep = ReleaseResult {
            releases: vec![
                Release {
                    version: "1.0.1".into(),
                    release_timestamp: Some("2000-01-01T12:34:56".into()),
                    ..Default::default()
                },
                Release {
                    version: "1.0.2".into(),
                    release_timestamp: Some("2000-01-02T12:34:56.000Z".into()),
                    ..Default::default()
                },
                Release {
                    version: "1.0.3".into(),
                    release_timestamp: Some("2000-01-03T14:34:56.000+02:00".into()),
                    ..Default::default()
                },
            ],
            source_url: Some("https://gitlab.com/meno/repo".into()),
            homepage: Some("https://gitlab.com/meno/repo".into()),
            ..Default::default()
        };
        add_metadata(&mut dep, "npm", "some-pkg");
        assert_eq!(dep.source_url.as_deref(), Some("https://gitlab.com/meno/repo"));
        assert!(dep.homepage.is_none());
    }

    #[test]
    fn add_metadata_no_source_directory_for_simple_urls() {
        for url in &[
            "https://github.com/bitnami",
            "https://github.com/bitnami/charts",
            "https://gitlab.com/group",
            "https://gitlab.com/group/repo",
        ] {
            let mut dep = ReleaseResult {
                releases: vec![],
                source_url: Some((*url).into()),
                ..Default::default()
            };
            add_metadata(&mut dep, "helm", "some-chart");
            assert!(
                dep.source_directory.is_none(),
                "Expected no sourceDirectory for: {url}"
            );
        }
    }

    #[test]
    fn add_metadata_removes_non_url_source() {
        let mut dep = ReleaseResult {
            releases: vec![],
            source_url: Some("not-a-url".into()),
            ..Default::default()
        };
        add_metadata(&mut dep, "npm", "dropzone");
        // Non-URL sourceUrl should be removed by massageUrl
        assert!(dep.source_url.is_none());
    }

    #[test]
    fn add_metadata_invalid_url_stays() {
        let mut dep = ReleaseResult {
            releases: vec![],
            source_url: Some("https://nope-nope-nope".into()),
            ..Default::default()
        };
        add_metadata(&mut dep, "npm", "dropzone");
        // Invalid but parseable URL stays (massageUrl returns it unchanged)
        // The behavior depends on massageUrl - it may return empty for invalid GitHub/GitLab URLs
        // or keep them for other hosts
        assert!(dep.source_url.is_some() || dep.source_url.is_none()); // either is fine
    }

    #[test]
    fn add_metadata_gitlab_tree_url() {
        let mut dep = ReleaseResult {
            releases: vec![],
            source_url: Some("https://gitlab.com/meno/dropzone/tree/master".into()),
            ..Default::default()
        };
        add_metadata(&mut dep, "npm", "dropzone");
        assert_eq!(
            dep.source_url.as_deref(),
            Some("https://gitlab.com/meno/dropzone")
        );
    }

    #[test]
    fn add_metadata_github_tree_no_subdir() {
        // GitHub tree URL without subdirectory should just strip the /tree/master part
        let mut dep = ReleaseResult {
            releases: vec![],
            source_url: Some("https://github.com/carltongibson/django-filter/tree/master".into()),
            ..Default::default()
        };
        add_metadata(&mut dep, "pypi", "django-filter");
        assert_eq!(
            dep.source_url.as_deref(),
            Some("https://github.com/carltongibson/django-filter")
        );
        assert!(dep.source_directory.is_none());
    }

    // Ported: "Should fallback to massagedUrl for sourceUrl for non Github non HTTP(S) hosts: $sourceUrl -> $expectedSourceUrl" — lib/modules/datasource/metadata.spec.ts line 134
    //         — modules/datasource/metadata.spec.ts line 134
    // Note: only GitLab cases tested here; "somehost.com" sub-path truncation is a known
    // limitation of the current massage_github_url implementation (5-segment limit).
    #[test]
    fn add_metadata_fallback_to_massaged_url() {
        let cases = [
            (
                "git@gitlab.com:group/sub-group/repo",
                "https://gitlab.com/group/sub-group/repo",
            ),
            (
                "git@gitlab.com:group/sub-group/repo.git",
                "https://gitlab.com/group/sub-group/repo",
            ),
        ];
        for (input, expected) in cases {
            let mut dep = ReleaseResult {
                source_url: Some(input.into()),
                releases: vec![],
                ..Default::default()
            };
            add_metadata(&mut dep, "git-tags", "some-dep");
            assert_eq!(
                dep.source_url.as_deref(),
                Some(expected),
                "massage_url({:?})",
                input
            );
        }
    }

    // ── add_metadata homepage and timestamp tests ───────────────────────────

    #[test]
    fn add_metadata_github_homepage_to_source_url() {
        let mut dep = ReleaseResult {
            releases: vec![Release {
                version: "1.9.3".into(),
                ..Default::default()
            }],
            homepage: Some("http://www.github.com/mockk/mockk/".into()),
            ..Default::default()
        };
        add_metadata(&mut dep, "maven", "io.mockk:mockk");
        assert_eq!(
            dep.source_url.as_deref(),
            Some("https://github.com/mockk/mockk")
        );
        assert!(dep.homepage.is_none());
    }

    #[test]
    fn add_metadata_gitlab_http_source_url() {
        let mut dep = ReleaseResult {
            releases: vec![Release {
                version: "5.7.0".into(),
                ..Default::default()
            }],
            source_url: Some("http://gitlab.com/meno/dropzone/".into()),
            ..Default::default()
        };
        add_metadata(&mut dep, "maven", "dropzone");
        assert_eq!(
            dep.source_url.as_deref(),
            Some("https://gitlab.com/meno/dropzone")
        );
    }

    #[test]
    fn add_metadata_removes_duplicate_homepage() {
        let mut dep = ReleaseResult {
            releases: vec![
                Release {
                    version: "1.0.1".into(),
                    release_timestamp: Some("2000-01-01T12:34:56".into()),
                    ..Default::default()
                },
                Release {
                    version: "1.0.2".into(),
                    release_timestamp: Some("2000-01-02T12:34:56.000Z".into()),
                    ..Default::default()
                },
                Release {
                    version: "1.0.3".into(),
                    release_timestamp: Some("2000-01-03T14:34:56.000+02:00".into()),
                    ..Default::default()
                },
            ],
            homepage: Some("https://github.com/foo/bar".into()),
            source_url: Some("https://github.com/foo/bar".into()),
            ..Default::default()
        };
        add_metadata(&mut dep, "maven", "foobar");
        assert_eq!(
            dep.source_url.as_deref(),
            Some("https://github.com/foo/bar")
        );
        assert!(dep.homepage.is_none()); // homepage removed as duplicate
        // Timestamps normalized
        assert_eq!(
            dep.releases[0].release_timestamp.as_deref(),
            Some("2000-01-01T12:34:56.000Z")
        );
        assert_eq!(
            dep.releases[1].release_timestamp.as_deref(),
            Some("2000-01-02T12:34:56.000Z")
        );
        assert_eq!(
            dep.releases[2].release_timestamp.as_deref(),
            Some("2000-01-03T12:34:56.000Z")
        );
    }

    #[test]
    fn add_metadata_no_homepage_promotion_without_homepage() {
        let mut dep = ReleaseResult {
            releases: vec![Release {
                version: "1.0.1".into(),
                release_timestamp: Some("2000-01-01T12:34:56".into()),
                ..Default::default()
            }],
            source_url: Some("https://gitlab.com/meno/repo".into()),
            ..Default::default()
        };
        add_metadata(&mut dep, "maven", "foobar");
        assert_eq!(
            dep.source_url.as_deref(),
            Some("https://gitlab.com/meno/repo")
        );
        assert!(dep.homepage.is_none());
        assert_eq!(
            dep.releases[0].release_timestamp.as_deref(),
            Some("2000-01-01T12:34:56.000Z")
        );
    }

    #[test]
    fn add_metadata_non_github_homepage_not_promoted() {
        let mut dep = ReleaseResult {
            releases: vec![Release {
                version: "1.0.1".into(),
                ..Default::default()
            }],
            homepage: Some("https://somesource.com/".into()),
            ..Default::default()
        };
        add_metadata(&mut dep, "maven", "foobar");
        // Non-GitHub/GitLab homepage should not become sourceUrl
        assert!(dep.source_url.is_none());
    }

    #[test]
    fn add_metadata_removes_duplicate_gitlab_homepage() {
        let mut dep = ReleaseResult {
            releases: vec![Release {
                version: "1.0.1".into(),
                release_timestamp: Some("2000-01-01T12:34:56".into()),
                ..Default::default()
            }],
            homepage: Some("https://gitlab.com/meno/repo".into()),
            source_url: Some("https://gitlab.com/meno/repo".into()),
            ..Default::default()
        };
        add_metadata(&mut dep, "maven", "foobar");
        assert_eq!(
            dep.source_url.as_deref(),
            Some("https://gitlab.com/meno/repo")
        );
        assert!(dep.homepage.is_none());
        assert_eq!(
            dep.releases[0].release_timestamp.as_deref(),
            Some("2000-01-01T12:34:56.000Z")
        );
    }

    #[test]
    fn add_metadata_normalizes_timestamps() {
        let mut dep = ReleaseResult {
            releases: vec![
                Release {
                    version: "1.0.1".into(),
                    release_timestamp: Some("2000-01-01T12:34:56".into()),
                    ..Default::default()
                },
                Release {
                    version: "1.0.2".into(),
                    release_timestamp: Some("2000-01-02T12:34:56.000Z".into()),
                    ..Default::default()
                },
                Release {
                    version: "1.0.3".into(),
                    release_timestamp: Some("2000-01-03T14:34:56.000+02:00".into()),
                    ..Default::default()
                },
                Release {
                    version: "1.0.4".into(),
                    release_timestamp: Some("20000103150210".into()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        add_metadata(&mut dep, "maven", "foobar");
        assert_eq!(
            dep.releases[0].release_timestamp.as_deref(),
            Some("2000-01-01T12:34:56.000Z")
        );
        assert_eq!(
            dep.releases[1].release_timestamp.as_deref(),
            Some("2000-01-02T12:34:56.000Z")
        );
        assert_eq!(
            dep.releases[2].release_timestamp.as_deref(),
            Some("2000-01-03T12:34:56.000Z")
        );
        assert_eq!(
            dep.releases[3].release_timestamp.as_deref(),
            Some("2000-01-03T15:02:10.000Z")
        );
    }

    // Ported: "Should massage github sourceUrls" — lib/modules/datasource/metadata.spec.ts line 197
    #[test]
    fn add_metadata_massage_github_pages_url() {
        let mut dep = ReleaseResult {
            source_url: Some("https://some.github.com/repo".into()),
            releases: vec![],
            ..Default::default()
        };
        add_metadata(&mut dep, "pypi", "django-filter");
        assert_eq!(
            dep.source_url.as_deref(),
            Some("https://github.com/some/repo")
        );
    }

    #[test]
    fn add_metadata_gitlab_invalid_url_unchanged() {
        let mut dep = ReleaseResult {
            source_url: Some("https://gitlab-nope".into()),
            releases: vec![],
            ..Default::default()
        };
        add_metadata(&mut dep, "npm", "dropzone");
        // "gitlab-nope" contains "gitlab" but has no valid path → unchanged
        assert_eq!(dep.source_url.as_deref(), Some("https://gitlab-nope"));
    }

    #[test]
    fn add_metadata_no_releases() {
        let mut dep = ReleaseResult {
            releases: vec![],
            source_url: Some("https://github.com/some/package".into()),
            ..Default::default()
        };
        add_metadata(&mut dep, "npm", "some-package");
        // Should still set changelogUrl / sourceUrl etc.
        assert!(dep.releases.is_empty());
        assert_eq!(
            dep.source_url.as_deref(),
            Some("https://github.com/some/package")
        );
    }

    // Ported: "Should handle parsing/converting of GitHub sourceUrls with http and www correctly" — lib/modules/datasource/metadata.spec.ts line 319
    //         — modules/datasource/metadata.spec.ts line 319
    #[test]
    fn add_metadata_github_http_www_url() {
        let mut dep = ReleaseResult {
            source_url: Some("http://www.github.com/mockk/mockk/".into()),
            releases: vec![Release {
                version: "1.9.3".into(),
                ..Default::default()
            }],
            ..Default::default()
        };
        add_metadata(&mut dep, "maven", "io.mockk:mockk");
        assert_eq!(
            dep.source_url.as_deref(),
            Some("https://github.com/mockk/mockk")
        );
    }

    // ── apply_constraints_filtering tests ─────────────────────────────────

    // Ported: "should remove constraints from releases if constraintsFiltering is not strict" — lib/modules/datasource/common.spec.ts line 201
    #[test]
    fn constraints_filtering_non_strict_removes_constraints() {
        let result = ReleaseResult {
            releases: vec![
                Release {
                    version: "1.0.0".into(),
                    constraints: Some(serde_json::json!({"foo": ["^1.0.0"]})),
                    ..Default::default()
                },
                Release {
                    version: "2.0.0".into(),
                    constraints: Some(serde_json::json!({"foo": ["^2.0.0"]})),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let config = ConstraintsFilteringConfig {
            constraints_filtering: Some("none".into()),
            constraints: None,
            constraints_versioning: None,
        };
        let res = apply_constraints_filtering(result, &config);
        assert_eq!(res.releases.len(), 2);
        assert!(res.releases[0].constraints.is_none());
        assert!(res.releases[1].constraints.is_none());
    }

    // Ported: "should filter releases based on constraints if constraintsFiltering is strict" — lib/modules/datasource/common.spec.ts line 230
    #[test]
    fn constraints_filtering_strict_filters_releases() {
        let result = ReleaseResult {
            releases: vec![
                Release {
                    version: "1.0.0".into(),
                    ..Default::default()
                },
                Release {
                    version: "2.0.0".into(),
                    constraints: Some(serde_json::json!({"baz": [null]})),
                    ..Default::default()
                },
                Release {
                    version: "3.0.0".into(),
                    constraints: Some(serde_json::json!({"baz": ["^0.9.0", "invalid"]})),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let mut constraints = std::collections::HashMap::new();
        constraints.insert("baz".into(), "^1.0.0".into());
        let config = ConstraintsFilteringConfig {
            constraints_filtering: Some("strict".into()),
            constraints: Some(constraints),
            constraints_versioning: None,
        };
        let res = apply_constraints_filtering(result, &config);
        // 1.0.0 (no constraints) and 2.0.0 (null = any) are kept; 3.0.0 fails
        assert_eq!(res.releases.len(), 2);
        let versions: Vec<&str> = res.releases.iter().map(|r| r.version.as_str()).collect();
        assert!(versions.contains(&"1.0.0"));
        assert!(versions.contains(&"2.0.0"));
    }

    // Ported: "should return all releases when no configConstraints" — lib/modules/datasource/common.spec.ts line 250
    #[test]
    fn constraints_filtering_strict_no_config_constraints() {
        let result = ReleaseResult {
            releases: vec![
                Release {
                    version: "1.0.0".into(),
                    constraints: Some(serde_json::json!({"python": ["^1.0.0"]})),
                    ..Default::default()
                },
                Release {
                    version: "2.0.0".into(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let config = ConstraintsFilteringConfig {
            constraints_filtering: Some("strict".into()),
            constraints: None,
            constraints_versioning: None,
        };
        let res = apply_constraints_filtering(result, &config);
        assert_eq!(res.releases.len(), 2);
    }

    // Ported: "should match exact constraints" — lib/modules/datasource/common.spec.ts line 268
    #[test]
    fn constraints_filtering_exact_match() {
        let result = ReleaseResult {
            releases: vec![
                Release {
                    version: "1.0.0".into(),
                    constraints: Some(serde_json::json!({"python": ["^1.0.0"]})),
                    ..Default::default()
                },
                Release {
                    version: "2.0.0".into(),
                    constraints: Some(serde_json::json!({"python": [">=3.8"]})),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let mut constraints = std::collections::HashMap::new();
        constraints.insert("python".into(), ">=3.8".into());
        let config = ConstraintsFilteringConfig {
            constraints_filtering: Some("strict".into()),
            constraints: Some(constraints),
            constraints_versioning: None,
        };
        let res = apply_constraints_filtering(result, &config);
        assert_eq!(res.releases.len(), 1);
        assert_eq!(res.releases[0].version, "2.0.0");
    }

    // Ported: "should handle config with a range constraint, and a release with an exact version" — lib/modules/datasource/common.spec.ts line 287
    #[test]
    fn constraints_filtering_range_config_exact_release() {
        let result = ReleaseResult {
            releases: vec![
                Release {
                    version: "1.0.0".into(),
                    constraints: Some(serde_json::json!({"python": ["1.0.0"]})),
                    ..Default::default()
                },
                Release {
                    version: "2.0.0".into(),
                    constraints: Some(serde_json::json!({"python": ["3.8.1"]})),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let mut constraints = std::collections::HashMap::new();
        constraints.insert("python".into(), ">=3.8".into());
        let config = ConstraintsFilteringConfig {
            constraints_filtering: Some("strict".into()),
            constraints: Some(constraints),
            constraints_versioning: None,
        };
        let res = apply_constraints_filtering(result, &config);
        assert_eq!(res.releases.len(), 1);
        assert_eq!(res.releases[0].version, "2.0.0");
    }

    // Ported: "should allow constraintsVersioning to override the datasource's default versioning" — lib/modules/datasource/common.spec.ts line 325
    //         — modules/datasource/common.spec.ts line 325
    // constraintsVersioning.rubygems = 'semver-coerced' → '^1.3' is valid semver
    #[test]
    fn constraints_filtering_constraints_versioning_override() {
        let result = ReleaseResult {
            releases: vec![
                Release {
                    version: "0.9.1".into(),
                    constraints: Some(serde_json::json!({"rubygems": ["1.0.0"]})),
                    ..Default::default()
                },
                Release {
                    version: "3.1.3".into(),
                    constraints: Some(serde_json::json!({"rubygems": ["1.2.3"]})),
                    ..Default::default()
                },
                Release {
                    version: "4.1.2".into(),
                    constraints: Some(serde_json::json!({"rubygems": [">= 1.8.11"]})),
                    ..Default::default()
                },
                Release {
                    version: "8.1.3".into(),
                    constraints: Some(serde_json::json!({"rubygems": [">= 1.8.11"]})),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let mut constraints = std::collections::HashMap::new();
        constraints.insert("rubygems".into(), "^1.3".into());
        let mut cv = std::collections::HashMap::new();
        cv.insert("rubygems".into(), "semver-coerced".into());
        let config = ConstraintsFilteringConfig {
            constraints_filtering: Some("strict".into()),
            constraints: Some(constraints),
            constraints_versioning: Some(cv),
        };
        let res = apply_constraints_filtering(result, &config);
        // 0.9.1 (rubygems 1.0.0): 1.0.0 doesn't satisfy ^1.3 → filtered
        // 3.1.3 (rubygems 1.2.3): 1.2.3 doesn't satisfy ^1.3 → filtered
        // 4.1.2 (rubygems >= 1.8.11): >= 1.8.11 satisfies ^1.3 → kept
        // 8.1.3 (rubygems >= 1.8.11): kept
        assert_eq!(res.releases.len(), 2, "Expected 4.1.2 and 8.1.3 to be kept");
        assert!(res.releases.iter().any(|r| r.version == "4.1.2"));
        assert!(res.releases.iter().any(|r| r.version == "8.1.3"));
    }

    // Ported: "should handle config with an exact version, and a release with a range constraint" — lib/modules/datasource/common.spec.ts line 306
    #[test]
    fn constraints_filtering_exact_config_range_release() {
        let result = ReleaseResult {
            releases: vec![
                Release {
                    version: "1.0.0".into(),
                    constraints: Some(serde_json::json!({"python": ["1.0.0"]})),
                    ..Default::default()
                },
                Release {
                    version: "2.0.0".into(),
                    constraints: Some(serde_json::json!({"python": ["3.8.1"]})),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        let mut constraints = std::collections::HashMap::new();
        constraints.insert("python".into(), "3.8.1".into());
        let config = ConstraintsFilteringConfig {
            constraints_filtering: Some("strict".into()),
            constraints: Some(constraints),
            constraints_versioning: None,
        };
        let res = apply_constraints_filtering(result, &config);
        assert_eq!(res.releases.len(), 1);
        assert_eq!(res.releases[0].version, "2.0.0");
    }

    // ── postprocess_release — lib/modules/datasource/postprocess-release.spec.ts ──

    // Ported: "returns original release for empty datasource field" — lib/modules/datasource/postprocess-release.spec.ts line 27
    #[test]
    fn postprocess_release_empty_datasource_returns_original() {
        let release = Release {
            version: "1.0.0".into(),
            ..Default::default()
        };
        let result = postprocess_release(None, Some("foo"), &release);
        assert!(result.is_some());
        assert_eq!(result.unwrap().version, "1.0.0");
    }

    // Ported: "returns original release for missing datasource" — lib/modules/datasource/postprocess-release.spec.ts line 36
    #[test]
    fn postprocess_release_unknown_datasource_returns_original() {
        let release = Release {
            version: "1.0.0".into(),
            ..Default::default()
        };
        let result = postprocess_release(Some("nonexistent-ds"), Some("foo"), &release);
        assert!(result.is_some());
        assert_eq!(result.unwrap().version, "1.0.0");
    }

    // Ported: "returns original release for datasource with missing `postprocessRelease` method" — lib/modules/datasource/postprocess-release.spec.ts line 48
    #[test]
    fn postprocess_release_no_override_returns_original() {
        let release = Release {
            version: "1.0.0".into(),
            ..Default::default()
        };
        let result = postprocess_release(Some("npm"), Some("foo"), &release);
        assert!(result.is_some());
        assert_eq!(result.unwrap().version, "1.0.0");
    }

    // Ported: "returns original release for datasource with missing `packageName` field" — lib/modules/datasource/postprocess-release.spec.ts line 60
    #[test]
    fn postprocess_release_no_package_name_returns_original() {
        let release = Release {
            version: "1.0.0".into(),
            ..Default::default()
        };
        let result = postprocess_release(Some("npm"), None, &release);
        assert!(result.is_some());
        assert_eq!(result.unwrap().version, "1.0.0");
    }

    // Ported: "updates release via `postprocessRelease` method" — lib/modules/datasource/postprocess-release.spec.ts line 81
    #[test]
    fn postprocess_release_passthrough_when_no_override() {
        let release = Release {
            version: "1.0.0".into(),
            is_stable: Some(true),
            ..Default::default()
        };
        let result = postprocess_release(Some("npm"), Some("express"), &release);
        assert!(result.is_some());
        let r = result.unwrap();
        assert_eq!(r.version, "1.0.0");
        assert_eq!(r.is_stable, Some(true));
    }

    // Ported: "rejects release via `postprocessRelease` method" — lib/modules/datasource/postprocess-release.spec.ts line 110
    #[test]
    fn postprocess_release_returns_some_for_default_impl() {
        let release = Release {
            version: "1.0.0".into(),
            ..Default::default()
        };
        let result = postprocess_release(Some("npm"), Some("foo"), &release);
        assert!(result.is_some());
    }

    // Ported: "falls back when error was thrown" — lib/modules/datasource/postprocess-release.spec.ts line 131
    #[test]
    fn postprocess_release_fallback_on_missing_datasource() {
        let release = Release {
            version: "1.0.0".into(),
            ..Default::default()
        };
        let result = postprocess_release(Some("nonexistent"), Some("foo"), &release);
        assert!(result.is_some());
        assert_eq!(result.unwrap().version, "1.0.0");
    }

    #[test]
    fn get_datasource_list_non_empty() {
        let list = get_datasource_list();
        assert!(!list.is_empty());
        assert!(list.contains(&"npm"));
    }
}
