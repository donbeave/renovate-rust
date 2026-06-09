//! Version comparison and update-decision logic.
//!
//! Each sub-module handles a specific versioning scheme's constraint syntax
//! and update planning. The Cargo module is first; others will follow.
//! @parity lib/modules/versioning/api.ts full
//! @parity lib/modules/versioning/schema.ts full
//! @parity lib/modules/versioning/index.ts full

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

/// Representation of a parsed versioning scheme spec.
///
/// Mirrors `Versioning.parse(...)` from `lib/modules/versioning/schema.ts`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersioningSelection {
    /// A known scheme selected by name without any constructor config.
    Default { id: &'static str },
    /// A scheme initialized with constructor config.
    Constructed {
        id: &'static str,
        config: Option<String>,
    },
}

impl VersioningSelection {
    pub fn id(&self) -> &'static str {
        match self {
            Self::Default { id } => id,
            Self::Constructed { id, .. } => id,
        }
    }
}

/// Parse a versioning spec into its resolved selection.
///
/// Mirrors `Versioning.parse(...)` from `lib/modules/versioning/schema.ts`.
pub fn parse_versioning(spec: &str) -> Result<VersioningSelection, String> {
    if spec.is_empty() {
        return Ok(VersioningSelection::Default {
            id: DEFAULT_VERSIONING,
        });
    }

    let (name, raw_config) = match spec.split_once(':') {
        Some((name, config)) => (name, Some(config)),
        None => (spec, None),
    };

    let Some(versioning_id) = ALL_VERSIONING_IDS.iter().copied().find(|id| *id == name) else {
        return Ok(VersioningSelection::Default {
            id: DEFAULT_VERSIONING,
        });
    };

    if name == "regex" {
        let input = match raw_config {
            Some(config) => format!("regex:{config}"),
            None => "regex".to_string(),
        };
        crate::versioning::regex_versioning::RegexVersioning::from_config(&input)
            .map_err(|err| format!("invalid regex config: {err}"))?;
        return Ok(VersioningSelection::Constructed {
            id: versioning_id,
            config: raw_config.filter(|v| !v.is_empty()).map(str::to_owned),
        });
    }

    Ok(VersioningSelection::Constructed {
        id: versioning_id,
        config: raw_config
            .filter(|value| !value.is_empty())
            .map(str::to_owned),
    })
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

    // Ported: "returns existing version scheme" — lib/modules/versioning/schema.spec.ts line 5
    // Ported: "falls back to default version scheme" — lib/modules/versioning/schema.spec.ts line 13
    // Ported: "catches errors" — lib/modules/versioning/schema.spec.ts line 19
    #[test]
    fn versioning_schema_parse() {
        let versioning1 = parse_versioning("hermit").expect("hermit is supported");
        let versioning2 =
            parse_versioning("hermit:foobar").expect("hermit supports constructor config");

        assert_ne!(versioning1, versioning2);
        assert_eq!(versioning1.id(), "hermit");
        assert_eq!(versioning2.id(), "hermit");

        let fallback = parse_versioning("foobarbaz").expect("unknown scheme falls back");
        assert_eq!(
            fallback,
            VersioningSelection::Default {
                id: "semver-coerced"
            }
        );

        let fallback_empty = parse_versioning("").expect("empty input falls back");
        assert_eq!(
            fallback_empty,
            VersioningSelection::Default {
                id: "semver-coerced"
            }
        );

        assert!(parse_versioning("regex:foobar").is_err());
    }

    #[test]
    fn gradle_version_matches_basic() {
        assert!(gradle_version_matches("1.2.3", "1.+"));
        assert!(!gradle_version_matches("2.0.0", "1.+"));
    }
}
