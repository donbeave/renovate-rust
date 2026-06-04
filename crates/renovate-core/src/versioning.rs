//! Version comparison and update-decision logic.
//!
//! Each sub-module handles a specific versioning scheme's constraint syntax
//! and update planning. The Cargo module is first; others will follow.
//! @parity lib/modules/versioning/api.ts full

// ═══════════════════════════════════════════════════════════════════════════
// Versioning registry — lib/modules/versioning/index.ts
// ═══════════════════════════════════════════════════════════════════════════

/// The default versioning ID used as fallback.
/// Mirrors `defaultVersioning = semverCoerced` from `lib/modules/versioning/index.ts`.
pub const DEFAULT_VERSIONING: &str = "semver-coerced";

/// All registered versioning module IDs.
/// Mirrors the keys of the `versionings` Map from `lib/modules/versioning/api.ts`.
pub const ALL_VERSIONING_IDS: &[&str] = &[
    "apk",
    "aws-eks-addon",
    "aws-machine-image",
    "azure-rest-api",
    "bazel-module",
    "cargo",
    "composer",
    "conan",
    "conda",
    "deb",
    "debian",
    "deno",
    "devbox",
    "docker",
    "elm",
    "exact",
    "git",
    "github-actions",
    "glasskube",
    "go-mod-directive",
    "gradle",
    "hashicorp",
    "helm",
    "hermit",
    "hex",
    "ivy",
    "kubernetes-api",
    "lambda-node",
    "loose",
    "maven",
    "node",
    "npm",
    "nuget",
    "pep440",
    "perl",
    "poetry",
    "pvp",
    "python",
    "redhat",
    "regex",
    "rez",
    "rpm",
    "ruby",
    "rust-release-channel",
    "same-major",
    "semver",
    "semver-coerced",
    "swift",
    "ubuntu",
    "unity3d",
    "unity3d-packages",
];

/// Return the list of all versioning module IDs.
/// Mirrors `getVersioningList()` from `lib/modules/versioning/index.ts`.
pub fn get_versioning_list() -> Vec<&'static str> {
    ALL_VERSIONING_IDS.to_vec()
}

/// Return the versioning ID to use for a given input.
///
/// - `None` or unknown → falls back to `DEFAULT_VERSIONING`
/// - Config-suffixed names (e.g. `"semver:pattern"`) strip the suffix
/// - Known IDs pass through unchanged
///
/// Mirrors `get()` from `lib/modules/versioning/index.ts`.
pub fn get_versioning_id(versioning: Option<&str>) -> &'static str {
    let Some(v) = versioning else {
        return DEFAULT_VERSIONING;
    };
    // Strip config suffix: "semver:test" → "semver"
    let base = v.split(':').next().unwrap_or(v);
    if ALL_VERSIONING_IDS.contains(&base) {
        ALL_VERSIONING_IDS
            .iter()
            .copied()
            .find(|&id| id == base)
            .unwrap_or(DEFAULT_VERSIONING)
    } else {
        DEFAULT_VERSIONING
    }
}

pub mod apk;
pub mod aws_eks_addon;
pub mod aws_machine_image;
pub mod azure_rest_api;
pub mod bazel_module;
pub mod cargo;
pub mod composer;
pub mod conan;
pub mod conda;
pub mod deb;
pub mod debian;
pub mod deno;
pub mod devbox;
pub mod distro;
pub mod docker;
pub mod elm;
pub mod exact;
pub mod git;
pub mod github_actions;
pub mod glasskube;
pub mod go_mod_directive;
pub mod gradle;
pub mod hashicorp;
pub mod helm;
pub mod hermit;
pub mod hex;
pub mod ivy;
pub mod kubernetes_api;
pub mod lambda_node;
pub mod loose;
pub mod maven;
pub mod nixpkgs;
pub mod node;
pub mod npm;
pub mod nuget;
pub mod pep440;
pub mod perl;
pub mod poetry;
pub mod pvp;
pub mod python;
pub mod redhat;
pub mod regex_versioning;
pub mod rez;
pub mod rpm;
pub mod ruby;
pub mod rust_release_channel;
pub mod same_major;
pub mod semver_coerced;
pub mod semver_generic;
pub mod semver_node;
pub mod semver_partial;
pub mod swift;
pub mod ubuntu;
pub mod unity3d;
pub mod unity3d_packages;

/// Convenience wrapper for gradle version matching used by content descriptor logic.
pub fn gradle_version_matches(version: &str, range: &str) -> bool {
    gradle::matches_range(version, range)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "should return versioning list" — lib/modules/versioning/index.spec.ts line 8
    #[test]
    fn versioning_registry_get_list() {
        let list = get_versioning_list();
        assert!(!list.is_empty());
        assert!(list.contains(&"npm"));
        assert!(list.contains(&"semver"));
        assert!(list.contains(&"semver-coerced"));
    }

    // Ported: "should fallback to semver-coerced" — lib/modules/versioning/index.spec.ts line 12
    #[test]
    fn versioning_registry_fallback() {
        assert_eq!(get_versioning_id(None), DEFAULT_VERSIONING);
        assert_eq!(get_versioning_id(Some("unknown")), DEFAULT_VERSIONING);
        assert_eq!(get_versioning_id(Some("distro")), DEFAULT_VERSIONING);
        assert_eq!(get_versioning_id(Some("semver-coerced")), "semver-coerced");
    }

    // Ported: "should accept config" — lib/modules/versioning/index.spec.ts line 18
    #[test]
    fn versioning_registry_accept_config() {
        // "semver:test" → strips ":test" → "semver" which is valid
        let result = get_versioning_id(Some("semver:test"));
        assert_eq!(result, "semver");
    }

    #[test]
    fn gradle_version_matches_basic() {
        assert!(gradle_version_matches("1.2.3", "1.+"));
        assert!(!gradle_version_matches("2.0.0", "1.+"));
    }
}
