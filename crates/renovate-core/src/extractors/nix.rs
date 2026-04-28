//! Nix flakes `flake.lock` dependency extractor.
//!
//! Reads the sibling `flake.lock` JSON file when a `flake.nix` is encountered
//! and extracts each top-level (root-referenced) flake input as a versioned dep.
//!
//! Renovate reference:
//! - `lib/modules/manager/nix/extract.ts`
//! - `lib/modules/manager/nix/schema.ts`
//! - Pattern: `/(^|/)flake\.nix$/`
//! - Datasource: GitRefsDatasource (git tags / commits)
//!
//! ## flake.lock structure
//!
//! ```json
//! {
//!   "nodes": {
//!     "root": { "inputs": { "nixpkgs": "nixpkgs", "home-manager": "home-manager" } },
//!     "nixpkgs": {
//!       "locked":   { "type": "github", "rev": "abc123..." },
//!       "original": { "type": "github", "owner": "NixOS", "repo": "nixpkgs", "ref": "nixos-24.05" }
//!     }
//!   }
//! }
//! ```
//!
//! ## Skip reasons
//!
//! | Condition | Why |
//! |---|---|
//! | `type = "indirect"` | Depends on flake registry; not directly updatable |
//! | `type = "path"` | Local path; not a remote versioned dep |
//! | missing `locked.rev` | No pinned SHA to track |
//! | not in root's inputs | Transitive; only root inputs are managed |

use serde::Deserialize;

/// Type of a flake input.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FlakeInputType {
    File,
    Git,
    Github,
    Gitlab,
    Indirect,
    Path,
    Sourcehut,
    Tarball,
    #[serde(other)]
    Unknown,
}

/// The `locked` section of a flake input (pinned state).
#[derive(Debug, Clone, Deserialize)]
pub struct FlakeLocked {
    #[serde(rename = "type")]
    pub input_type: FlakeInputType,
    pub rev: Option<String>,
    #[serde(rename = "ref")]
    pub git_ref: Option<String>,
}

/// The `original` section of a flake input (user's declared intent).
#[derive(Debug, Clone, Deserialize)]
pub struct FlakeOriginal {
    #[serde(rename = "type")]
    pub input_type: FlakeInputType,
    pub owner: Option<String>,
    pub repo: Option<String>,
    #[serde(rename = "ref")]
    pub git_ref: Option<String>,
    pub rev: Option<String>,
    pub url: Option<String>,
    pub host: Option<String>,
}

/// A single flake input node.
#[derive(Debug, Clone, Deserialize)]
pub struct FlakeNode {
    pub inputs: Option<std::collections::HashMap<String, serde_json::Value>>,
    pub locked: Option<FlakeLocked>,
    pub original: Option<FlakeOriginal>,
}

/// The full `flake.lock` structure.
#[derive(Debug, Clone, Deserialize)]
pub struct FlakeLock {
    pub nodes: std::collections::HashMap<String, FlakeNode>,
    pub version: u32,
}

/// Skip reason for a nix flake input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NixSkipReason {
    Indirect,
    LocalPath,
    NoRev,
    Transitive,
    UnsupportedType,
}

/// A single extracted Nix flake input dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NixFlakeDep {
    /// Input name (key in `nodes`, e.g. `"nixpkgs"`).
    pub input_name: String,
    /// Pinned commit SHA from `locked.rev`.
    pub locked_rev: String,
    /// Current branch/tag ref from `original.ref` (e.g. `"nixos-24.05"`).
    pub current_ref: Option<String>,
    /// GitHub/GitLab package name (e.g. `"NixOS/nixpkgs"`).
    pub package_name: Option<String>,
    /// Input type for routing to the correct datasource.
    pub input_type: FlakeInputType,
    pub skip_reason: Option<NixSkipReason>,
}

/// Parse `flake.lock` content and extract top-level flake input deps.
pub fn extract(flake_lock_content: &str) -> Vec<NixFlakeDep> {
    let lock: FlakeLock = match serde_json::from_str(flake_lock_content) {
        Ok(l) => l,
        Err(_) => return Vec::new(),
    };

    // Only version 7 is supported (current flake.lock format).
    if lock.version != 7 {
        return Vec::new();
    }

    let root = match lock.nodes.get("root") {
        Some(r) => r,
        None => return Vec::new(),
    };

    let root_inputs: std::collections::HashSet<String> = root
        .inputs
        .as_ref()
        .map(|m| m.keys().cloned().collect())
        .unwrap_or_default();

    let mut deps = Vec::new();

    for (name, node) in &lock.nodes {
        if name == "root" {
            continue;
        }

        // Skip transitive inputs (not directly in root.inputs).
        if !root_inputs.contains(name.as_str()) {
            continue;
        }

        let locked = match &node.locked {
            Some(l) => l,
            None => {
                deps.push(NixFlakeDep {
                    input_name: name.clone(),
                    locked_rev: String::new(),
                    current_ref: None,
                    package_name: None,
                    input_type: FlakeInputType::Unknown,
                    skip_reason: Some(NixSkipReason::NoRev),
                });
                continue;
            }
        };

        let original = match &node.original {
            Some(o) => o,
            None => {
                deps.push(NixFlakeDep {
                    input_name: name.clone(),
                    locked_rev: String::new(),
                    current_ref: None,
                    package_name: None,
                    input_type: locked.input_type.clone(),
                    skip_reason: Some(NixSkipReason::NoRev),
                });
                continue;
            }
        };

        // Skip indirect and local-path inputs.
        if matches!(
            locked.input_type,
            FlakeInputType::Indirect | FlakeInputType::Path
        ) || matches!(
            original.input_type,
            FlakeInputType::Indirect | FlakeInputType::Path
        ) {
            deps.push(NixFlakeDep {
                input_name: name.clone(),
                locked_rev: String::new(),
                current_ref: None,
                package_name: None,
                input_type: locked.input_type.clone(),
                skip_reason: Some(if locked.input_type == FlakeInputType::Path {
                    NixSkipReason::LocalPath
                } else {
                    NixSkipReason::Indirect
                }),
            });
            continue;
        }

        let rev = match &locked.rev {
            Some(r) => r.clone(),
            None => {
                deps.push(NixFlakeDep {
                    input_name: name.clone(),
                    locked_rev: String::new(),
                    current_ref: None,
                    package_name: None,
                    input_type: locked.input_type.clone(),
                    skip_reason: Some(NixSkipReason::NoRev),
                });
                continue;
            }
        };

        let package_name = build_package_name(locked, original);

        deps.push(NixFlakeDep {
            input_name: name.clone(),
            locked_rev: rev,
            current_ref: original.git_ref.clone(),
            package_name,
            input_type: locked.input_type.clone(),
            skip_reason: None,
        });
    }

    deps
}

fn build_package_name(locked: &FlakeLocked, original: &FlakeOriginal) -> Option<String> {
    match &locked.input_type {
        FlakeInputType::Github => {
            let host = original.host.as_deref().unwrap_or("github.com");
            let owner = original.owner.as_deref()?;
            let repo = original.repo.as_deref()?;
            Some(format!("https://{host}/{owner}/{repo}"))
        }
        FlakeInputType::Gitlab => {
            let host = original.host.as_deref().unwrap_or("gitlab.com");
            let owner = original.owner.as_deref()?;
            let repo = original.repo.as_deref()?;
            Some(format!("https://{host}/{owner}/{repo}"))
        }
        FlakeInputType::Git | FlakeInputType::Tarball => original.url.clone(),
        FlakeInputType::Sourcehut => {
            let host = original.host.as_deref().unwrap_or("git.sr.ht");
            let owner = original.owner.as_deref()?;
            let repo = original.repo.as_deref()?;
            Some(format!("https://{host}/{owner}/{repo}"))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_LOCK: &str = r#"{
  "nodes": {
    "root": {
      "inputs": {
        "nixpkgs": "nixpkgs",
        "home-manager": "home-manager"
      }
    },
    "nixpkgs": {
      "locked": {
        "lastModified": 1714000000,
        "narHash": "sha256-abc",
        "owner": "NixOS",
        "repo": "nixpkgs",
        "rev": "5c211b47aeadcc178c5320a7e25c8a3f7b2e5a01",
        "type": "github"
      },
      "original": {
        "owner": "NixOS",
        "ref": "nixos-24.05",
        "repo": "nixpkgs",
        "type": "github"
      }
    },
    "home-manager": {
      "locked": {
        "lastModified": 1714100000,
        "narHash": "sha256-def",
        "owner": "nix-community",
        "repo": "home-manager",
        "rev": "aaabbbcccddd1234567890aaabbbcccddd123456",
        "type": "github"
      },
      "original": {
        "owner": "nix-community",
        "repo": "home-manager",
        "type": "github"
      }
    },
    "nixpkgs_2": {
      "locked": {
        "type": "github",
        "rev": "deadbeef",
        "owner": "NixOS",
        "repo": "nixpkgs"
      },
      "original": {
        "type": "github",
        "owner": "NixOS",
        "repo": "nixpkgs"
      }
    }
  },
  "version": 7
}"#;

    #[test]
    fn extracts_root_inputs() {
        let deps = extract(SAMPLE_LOCK);
        assert_eq!(deps.iter().filter(|d| d.skip_reason.is_none()).count(), 2);
    }

    #[test]
    fn skips_transitive() {
        let deps = extract(SAMPLE_LOCK);
        assert!(!deps.iter().any(|d| d.input_name == "nixpkgs_2"));
    }

    #[test]
    fn extracts_nixpkgs_correctly() {
        let deps = extract(SAMPLE_LOCK);
        let np = deps.iter().find(|d| d.input_name == "nixpkgs").unwrap();
        assert_eq!(np.locked_rev, "5c211b47aeadcc178c5320a7e25c8a3f7b2e5a01");
        assert_eq!(np.current_ref.as_deref(), Some("nixos-24.05"));
        assert_eq!(
            np.package_name.as_deref(),
            Some("https://github.com/NixOS/nixpkgs")
        );
        assert!(np.skip_reason.is_none());
    }

    #[test]
    fn invalid_json_returns_empty() {
        assert!(extract("not json").is_empty());
    }

    #[test]
    fn wrong_version_returns_empty() {
        let content = r#"{"nodes": {"root": {}}, "version": 6}"#;
        assert!(extract(content).is_empty());
    }
}
