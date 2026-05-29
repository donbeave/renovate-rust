//! Datasource clients for fetching available package versions.
//!
//! Each sub-module implements a registry-specific version lookup. The common
//! output is a list of available version strings that the update-planner then
//! compares against the current constraint.

pub mod artifactory;
pub mod azure_bicep;
pub mod azure_pipelines_tasks;
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
pub mod dart_version;
pub mod deb;
pub mod deno;
pub mod devbox;
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
pub mod github_releases;
pub mod github_runners;
pub mod github_tags;
pub mod gitlab_packages;
pub mod gitlab_releases;
pub mod gitlab_tags;
pub mod glasskube_packages;
pub mod golang_version;
pub mod gomod;
pub mod gradle_version;
pub mod hackage;
pub mod helm;
pub mod hermit;
pub mod hex;
pub mod hexpm_bob;
pub mod java_version;
pub mod jenkins_plugins;
pub mod jsr;
pub mod kubernetes_api;
pub mod maven;
pub mod nextcloud;
pub mod node_version;
pub mod npm;
pub mod nuget;
pub mod orb;
pub mod packagist;
pub mod pub_dev;
pub mod puppet_forge;
pub mod pypi;
pub mod python_version;
pub mod repology;
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
pub struct DatasourceInfo {
    pub id: &'static str,
    pub default_versioning: &'static str,
}

/// The default versioning ID used when no datasource-specific one is set.
/// Mirrors `defaultVersioning = semverCoerced` from `lib/modules/versioning/index.ts`.
pub const DATASOURCE_DEFAULT_VERSIONING: &str = "semver-coerced";

/// Static registry of known datasource IDs and their default versioning.
/// Mirrors the datasources Map from `lib/modules/datasource/api.ts`.
const KNOWN_DATASOURCES: &[DatasourceInfo] = &[
    DatasourceInfo { id: "artifactory", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "aws-eks-addon", default_versioning: "aws-eks-addon" },
    DatasourceInfo { id: "aws-machine-image", default_versioning: "aws-machine-image" },
    DatasourceInfo { id: "aws-rds", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "azure-bicep-resource", default_versioning: "azure-rest-api" },
    DatasourceInfo { id: "azure-pipelines-tasks", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "azure-tags", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "bazel", default_versioning: "bazel-module" },
    DatasourceInfo { id: "bitbucket-server-tags", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "bitbucket-tags", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "bitrise", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "buildpacks-registry", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "cdnjs", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "clojure", default_versioning: "maven" },
    DatasourceInfo { id: "conan", default_versioning: "conan" },
    DatasourceInfo { id: "conda", default_versioning: "pep440" },
    DatasourceInfo { id: "cpan", default_versioning: "perl" },
    DatasourceInfo { id: "crate", default_versioning: "cargo" },
    DatasourceInfo { id: "custom", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "dart", default_versioning: "npm" },
    DatasourceInfo { id: "deb", default_versioning: "deb" },
    DatasourceInfo { id: "deno", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "devbox", default_versioning: "devbox" },
    DatasourceInfo { id: "docker", default_versioning: "docker" },
    DatasourceInfo { id: "dotnet-version", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "elm-package", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "endoflife-date", default_versioning: "loose" },
    DatasourceInfo { id: "flutter-version", default_versioning: "semver" },
    DatasourceInfo { id: "forgejo-releases", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "forgejo-tags", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "galaxy", default_versioning: "pep440" },
    DatasourceInfo { id: "galaxy-collection", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "git-refs", default_versioning: "git" },
    DatasourceInfo { id: "git-tags", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "gitea-releases", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "gitea-tags", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "github-digest", default_versioning: "exact" },
    DatasourceInfo { id: "github-release-attachments", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "github-releases", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "github-runners", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "github-tags", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "gitlab-packages", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "gitlab-releases", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "gitlab-tags", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "glasskube-packages", default_versioning: "glasskube" },
    DatasourceInfo { id: "go", default_versioning: "semver" },
    DatasourceInfo { id: "golang-version", default_versioning: "semver" },
    DatasourceInfo { id: "gradle-version", default_versioning: "gradle" },
    DatasourceInfo { id: "hackage", default_versioning: "pvp" },
    DatasourceInfo { id: "helm", default_versioning: "helm" },
    DatasourceInfo { id: "hermit", default_versioning: "hermit" },
    DatasourceInfo { id: "hex", default_versioning: "hex" },
    DatasourceInfo { id: "hexpm-bob", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "java-version", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "jenkins-plugins", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "jsr", default_versioning: "semver" },
    DatasourceInfo { id: "kubernetes-api", default_versioning: "kubernetes-api" },
    DatasourceInfo { id: "maven", default_versioning: "maven" },
    DatasourceInfo { id: "nextcloud", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "node-version", default_versioning: "node" },
    DatasourceInfo { id: "npm", default_versioning: "npm" },
    DatasourceInfo { id: "nuget", default_versioning: "nuget" },
    DatasourceInfo { id: "orb", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "packagist", default_versioning: "composer" },
    DatasourceInfo { id: "pod", default_versioning: "ruby" },
    DatasourceInfo { id: "pub", default_versioning: "npm" },
    DatasourceInfo { id: "puppet-forge", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "pypi", default_versioning: "pep440" },
    DatasourceInfo { id: "python-version", default_versioning: "pep440" },
    DatasourceInfo { id: "repology", default_versioning: "loose" },
    DatasourceInfo { id: "rpm", default_versioning: "rpm" },
    DatasourceInfo { id: "ruby-version", default_versioning: "ruby" },
    DatasourceInfo { id: "rubygems", default_versioning: "ruby" },
    DatasourceInfo { id: "rust-version", default_versioning: "rust-release-channel" },
    DatasourceInfo { id: "sbt-package", default_versioning: "ivy" },
    DatasourceInfo { id: "sbt-plugin", default_versioning: "ivy" },
    DatasourceInfo { id: "terraform-module", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "terraform-provider", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "typst", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "unity3d", default_versioning: "semver-coerced" },
    DatasourceInfo { id: "unity3d-packages", default_versioning: "semver-coerced" },
];

/// Return the datasource info for a given ID, or `None` if unknown.
///
/// Custom datasources ("custom.*") map to the "custom" entry.
/// Mirrors `getDatasourceFor()` from `lib/modules/datasource/common.ts`.
pub fn get_datasource_for(name: &str) -> Option<&'static DatasourceInfo> {
    // "custom.*" → custom datasource
    let lookup = if name.starts_with("custom.") { "custom" } else { name };
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

#[cfg(test)]
mod registry_tests {
    use super::*;

    // Ported: "returns null for unknown datasource" — modules/datasource/common.spec.ts line 21
    #[test]
    fn datasource_registry_unknown_returns_none() {
        assert!(get_datasource_for("foobar").is_none());
    }

    // Ported: "supports custom datasource" — modules/datasource/common.spec.ts line 25
    #[test]
    fn datasource_registry_custom_prefix() {
        let custom = get_datasource_for("custom.foobar");
        let base = get_datasource_for("custom");
        assert!(custom.is_some());
        assert_eq!(custom.map(|d| d.id), base.map(|d| d.id));
    }

    // Ported: "returns datasource for known datasource" — modules/datasource/common.spec.ts line 31
    #[test]
    fn datasource_registry_known_returns_some() {
        let ds = get_datasource_for("npm").unwrap();
        assert_eq!(ds.id, "npm");
    }

    // Ported: "returns default versioning for undefined datasource" — modules/datasource/common.spec.ts line 39
    #[test]
    fn datasource_registry_default_versioning_undefined() {
        assert_eq!(get_datasource_default_versioning(None), "semver-coerced");
    }

    // Ported: "returns default versioning for unknown datasource" — modules/datasource/common.spec.ts line 43
    #[test]
    fn datasource_registry_default_versioning_unknown() {
        assert_eq!(get_datasource_default_versioning(Some("foobar")), "semver-coerced");
    }

    // Ported: "returns default versioning for datasource with missing default versioning configuration" — modules/datasource/common.spec.ts line 52
    #[test]
    fn datasource_registry_default_versioning_no_specific() {
        // artifactory has no specific default → semver-coerced
        assert_eq!(get_datasource_default_versioning(Some("artifactory")), "semver-coerced");
    }

    // Ported: "returns datasource-defined default versioning" — modules/datasource/common.spec.ts line 56
    #[test]
    fn datasource_registry_datasource_defined_versioning() {
        // crate uses cargo versioning
        assert_eq!(get_datasource_default_versioning(Some("crate")), "cargo");
    }
}
