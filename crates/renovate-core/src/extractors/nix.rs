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
    /// Explicit commit/digest from `original.rev`, when the declared input pins one.
    pub current_digest: Option<String>,
    /// GitHub/GitLab package name (e.g. `"NixOS/nixpkgs"`).
    pub package_name: Option<String>,
    /// Input type for routing to the correct datasource.
    pub input_type: FlakeInputType,
    pub skip_reason: Option<NixSkipReason>,
}

/// Optional extraction context used while confirming digest replacement updates.
#[derive(Debug, Clone, Copy, Default)]
pub struct NixExtractConfig<'a> {
    pub flake_nix_content: Option<&'a str>,
    pub current_digest: Option<&'a str>,
    pub new_digest: Option<&'a str>,
}

/// Extract a `flake.nix` package file when both sibling file contents are available.
pub fn extract_package_file(
    flake_nix_content: Option<&str>,
    flake_lock_content: Option<&str>,
) -> Option<Vec<NixFlakeDep>> {
    extract_package_file_with_config(
        flake_nix_content,
        flake_lock_content,
        NixExtractConfig::default(),
    )
}

/// Extract a `flake.nix` package file with optional replacement context.
pub fn extract_package_file_with_config(
    flake_nix_content: Option<&str>,
    flake_lock_content: Option<&str>,
    config: NixExtractConfig<'_>,
) -> Option<Vec<NixFlakeDep>> {
    let flake_nix_content = flake_nix_content?;
    let flake_lock_content = flake_lock_content?;
    let deps = extract_with_config(
        flake_lock_content,
        NixExtractConfig {
            flake_nix_content: Some(flake_nix_content),
            ..config
        },
    );
    (!deps.is_empty()).then_some(deps)
}

/// Parse `flake.lock` content and extract top-level flake input deps.
pub fn extract(flake_lock_content: &str) -> Vec<NixFlakeDep> {
    extract_with_config(flake_lock_content, NixExtractConfig::default())
}

/// Parse `flake.lock` content with optional `flake.nix` replacement context.
pub fn extract_with_config(
    flake_lock_content: &str,
    config: NixExtractConfig<'_>,
) -> Vec<NixFlakeDep> {
    let lock: FlakeLock = match serde_json::from_str(flake_lock_content) {
        Ok(l) => l,
        Err(_) => return Vec::new(),
    };

    // Only version 7 is supported (current flake.lock format).
    if lock.version != 7 {
        return Vec::new();
    }

    let Some(root) = lock.nodes.get("root") else {
        return Vec::new();
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

        let Some(locked) = &node.locked else {
            deps.push(NixFlakeDep {
                input_name: name.clone(),
                locked_rev: String::new(),
                current_ref: None,
                current_digest: None,
                package_name: None,
                input_type: FlakeInputType::Unknown,
                skip_reason: Some(NixSkipReason::NoRev),
            });
            continue;
        };

        let Some(original) = &node.original else {
            deps.push(NixFlakeDep {
                input_name: name.clone(),
                locked_rev: String::new(),
                current_ref: None,
                current_digest: None,
                package_name: None,
                input_type: locked.input_type.clone(),
                skip_reason: Some(NixSkipReason::NoRev),
            });
            continue;
        };

        if matches!(
            locked.input_type,
            FlakeInputType::Unknown | FlakeInputType::File
        ) || matches!(
            original.input_type,
            FlakeInputType::Unknown | FlakeInputType::File
        ) {
            continue;
        }

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
                current_digest: None,
                package_name: None,
                input_type: locked.input_type.clone(),
                skip_reason: Some(
                    if locked.input_type == FlakeInputType::Path
                        || original.input_type == FlakeInputType::Path
                    {
                        NixSkipReason::LocalPath
                    } else {
                        NixSkipReason::Indirect
                    },
                ),
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
                    current_digest: None,
                    package_name: None,
                    input_type: locked.input_type.clone(),
                    skip_reason: Some(NixSkipReason::NoRev),
                });
                continue;
            }
        };

        let package_name = build_package_name(locked, original);
        let current_ref = original
            .git_ref
            .clone()
            .or_else(|| tarball_channel_ref(original));
        let current_digest = replacement_current_digest(original, config);

        deps.push(NixFlakeDep {
            input_name: name.clone(),
            locked_rev: rev,
            current_ref,
            current_digest,
            package_name,
            input_type: locked.input_type.clone(),
            skip_reason: None,
        });
    }

    deps
}

fn replacement_current_digest(
    original: &FlakeOriginal,
    config: NixExtractConfig<'_>,
) -> Option<String> {
    let original_rev = original.rev.as_deref()?;
    match (
        config.current_digest,
        config.new_digest,
        config.flake_nix_content,
    ) {
        (Some(current_digest), Some(new_digest), Some(flake_nix_content))
            if original_rev == current_digest && flake_nix_content.contains(new_digest) =>
        {
            Some(new_digest.to_owned())
        }
        _ => Some(original_rev.to_owned()),
    }
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
            let owner = decode_percent_slashes(original.owner.as_deref()?);
            let repo = original.repo.as_deref()?;
            Some(format!("https://{host}/{owner}/{repo}"))
        }
        FlakeInputType::Git => original.url.clone(),
        FlakeInputType::Tarball => original.url.as_deref().and_then(tarball_package_name),
        FlakeInputType::Sourcehut => {
            let host = original.host.as_deref().unwrap_or("git.sr.ht");
            let owner = original.owner.as_deref()?;
            let repo = original.repo.as_deref()?;
            Some(format!("https://{host}/{owner}/{repo}"))
        }
        _ => None,
    }
}

fn tarball_package_name(url: &str) -> Option<String> {
    if let Some(stripped) = url.strip_prefix("https://channels.nixos.org/")
        && stripped.ends_with("/nixexprs.tar.xz")
    {
        return Some("https://github.com/NixOS/nixpkgs".to_owned());
    }

    let (base, _) = url.split_once("/archive/")?;
    Some(base.to_owned())
}

fn tarball_channel_ref(original: &FlakeOriginal) -> Option<String> {
    let url = original.url.as_deref()?;
    let stripped = url.strip_prefix("https://channels.nixos.org/")?;
    let (channel, rest) = stripped.split_once('/')?;
    if rest == "nixexprs.tar.xz" {
        Some(channel.to_owned())
    } else {
        None
    }
}

fn decode_percent_slashes(value: &str) -> String {
    value.replace("%2F", "/").replace("%2f", "/")
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

    const EMPTY_ROOT_LOCK: &str = r#"{
  "nodes": {
    "root": {}
  },
  "root": "root",
  "version": 7
}"#;

    // Ported: "returns null when no nixpkgs input exists" — nix/extract.spec.ts line 10
    #[test]
    fn package_file_returns_none_when_no_nixpkgs_input_exists() {
        let flake_nix = r#"{
  inputs = {};
}"#;
        assert!(extract_package_file(Some(flake_nix), Some(EMPTY_ROOT_LOCK)).is_none());
    }

    // Ported: "does not include nixpkgs input with no explicit ref" — nix/extract.spec.ts line 25
    #[test]
    fn package_file_returns_none_for_nixpkgs_without_explicit_ref_when_lock_has_no_input() {
        let flake_nix = r#"{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
  };
}"#;
        assert!(extract_package_file(Some(flake_nix), Some(EMPTY_ROOT_LOCK)).is_none());
    }

    // Ported: "includes nixpkgs input with only ref" — nix/extract.spec.ts line 42
    #[test]
    fn package_file_returns_none_for_ref_only_flake_when_lock_has_no_input() {
        let flake_nix = r#"{
  inputs = {
    nixpkgs-lib.url = "https://github.com/NixOS/nixpkgs/archive/072a6db25e947df2f31aab9eccd0ab75d5b2da11.tar.gz";
  };
}"#;
        assert!(extract_package_file(Some(flake_nix), Some(EMPTY_ROOT_LOCK)).is_none());
    }

    // Ported: "returns null when no inputs" — nix/extract.spec.ts line 59
    #[test]
    fn package_file_returns_none_when_flake_nix_has_no_inputs() {
        assert!(extract_package_file(Some(""), Some(EMPTY_ROOT_LOCK)).is_none());
    }

    #[test]
    fn skips_transitive() {
        let deps = extract(SAMPLE_LOCK);
        assert!(!deps.iter().any(|d| d.input_name == "nixpkgs_2"));
    }

    // Ported: "returns nixpkgs input" — nix/extract.spec.ts line 217
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

    // Ported: "returns null when original inputs are from local path" — nix/extract.spec.ts line 121
    #[test]
    fn original_path_input_is_skipped_as_local_path() {
        let content = r#"{
  "nodes": {
    "nixpkgs": {
      "locked": {
        "lastModified": 1720031269,
        "narHash": "sha256-rwz8NJZV+387rnWpTYcXaRNvzUSnnF9aHONoJIYmiUQ=",
        "owner": "NixOS",
        "repo": "nixpkgs",
        "rev": "9f4128e00b0ae8ec65918efeba59db998750ead6",
        "type": "github"
      },
      "original": {
        "owner": "NixOS",
        "ref": "nixos-unstable",
        "repo": "nixpkgs",
        "type": "path"
      }
    },
    "root": {
      "inputs": {
        "nixpkgs": "nixpkgs"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "nixpkgs");
        assert_eq!(deps[0].skip_reason, Some(NixSkipReason::LocalPath));
    }

    // Ported: "returns null when locked inputs are indirect" — nix/extract.spec.ts line 153
    #[test]
    fn locked_indirect_input_is_skipped() {
        let content = r#"{
  "nodes": {
    "nixpkgs": {
      "locked": {
        "lastModified": 1720031269,
        "narHash": "sha256-rwz8NJZV+387rnWpTYcXaRNvzUSnnF9aHONoJIYmiUQ=",
        "owner": "NixOS",
        "repo": "nixpkgs",
        "rev": "9f4128e00b0ae8ec65918efeba59db998750ead6",
        "type": "indirect"
      },
      "original": {
        "owner": "NixOS",
        "ref": "nixos-unstable",
        "repo": "nixpkgs",
        "type": "github"
      }
    },
    "root": {
      "inputs": {
        "nixpkgs": "nixpkgs"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "nixpkgs");
        assert_eq!(deps[0].skip_reason, Some(NixSkipReason::Indirect));
    }

    // Ported: "returns null when locked inputs are from local path" — nix/extract.spec.ts line 185
    #[test]
    fn locked_path_input_is_skipped_as_local_path() {
        let content = r#"{
  "nodes": {
    "nixpkgs": {
      "locked": {
        "lastModified": 1720031269,
        "narHash": "sha256-rwz8NJZV+387rnWpTYcXaRNvzUSnnF9aHONoJIYmiUQ=",
        "owner": "NixOS",
        "repo": "nixpkgs",
        "rev": "9f4128e00b0ae8ec65918efeba59db998750ead6",
        "type": "path"
      },
      "original": {
        "owner": "NixOS",
        "ref": "nixos-unstable",
        "repo": "nixpkgs",
        "type": "github"
      }
    },
    "root": {
      "inputs": {
        "nixpkgs": "nixpkgs"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "nixpkgs");
        assert_eq!(deps[0].skip_reason, Some(NixSkipReason::LocalPath));
    }

    // Ported: "returns null when inputs are missing locked" — nix/extract.spec.ts line 71
    #[test]
    fn missing_locked_section_is_skipped_as_no_rev() {
        let content = r#"{
  "nodes": {
    "nixpkgs": {
      "original": {
        "owner": "NixOS",
        "ref": "nixos-unstable",
        "repo": "nixpkgs",
        "type": "github"
      }
    },
    "root": {
      "inputs": {
        "nixpkgs": "nixpkgs"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "nixpkgs");
        assert_eq!(deps[0].skip_reason, Some(NixSkipReason::NoRev));
    }

    // Ported: "returns null when inputs are missing original" — nix/extract.spec.ts line 95
    #[test]
    fn missing_original_section_is_skipped_as_no_rev() {
        let content = r#"{
  "nodes": {
    "nixpkgs": {
      "locked": {
        "lastModified": 1720031269,
        "narHash": "sha256-rwz8NJZV+387rnWpTYcXaRNvzUSnnF9aHONoJIYmiUQ=",
        "owner": "NixOS",
        "repo": "nixpkgs",
        "rev": "9f4128e00b0ae8ec65918efeba59db998750ead6",
        "type": "github"
      }
    },
    "root": {
      "inputs": {
        "nixpkgs": "nixpkgs"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "nixpkgs");
        assert_eq!(deps[0].skip_reason, Some(NixSkipReason::NoRev));
    }

    // Ported: "includes nixpkgs with no explicit ref" — nix/extract.spec.ts line 260
    #[test]
    fn includes_nixpkgs_with_no_explicit_ref() {
        let content = r#"{
  "nodes": {
    "nixpkgs": {
      "locked": {
        "lastModified": 1728650607,
        "narHash": "sha256-0lOnVTzRXzpk5uxbHLm3Ti3tyPAvirAIQDfwEUd8arg=",
        "owner": "NixOS",
        "repo": "nixpkgs",
        "rev": "612ee628421ba2c1abca4c99684862f76cb3b089",
        "type": "github"
      },
      "original": {
        "owner": "NixOS",
        "repo": "nixpkgs",
        "type": "github"
      }
    },
    "root": {
      "inputs": {
        "nixpkgs": "nixpkgs"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "nixpkgs");
        assert_eq!(
            deps[0].locked_rev,
            "612ee628421ba2c1abca4c99684862f76cb3b089"
        );
        assert_eq!(deps[0].current_ref, None);
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("https://github.com/NixOS/nixpkgs")
        );
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "includes patchelf from HEAD" — nix/extract.spec.ts line 300
    #[test]
    fn includes_git_input_from_head() {
        let content = r#"{
  "nodes": {
    "patchelf": {
      "inputs": {
        "nixpkgs": "nixpkgs"
      },
      "locked": {
        "lastModified": 1718457448,
        "narHash": "sha256-FSoxTcRZMGHNJh8dNtKOkcUtjhmhU6yQXcZZfUPLhQM=",
        "ref": "refs/heads/master",
        "rev": "a0f54334df36770b335c051e540ba40afcbf8378",
        "revCount": 844,
        "type": "git",
        "url": "https://github.com/NixOS/patchelf.git"
      },
      "original": {
        "type": "git",
        "url": "https://github.com/NixOS/patchelf.git"
      }
    },
    "root": {
      "inputs": {
        "patchelf": "patchelf"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "patchelf");
        assert_eq!(
            deps[0].locked_rev,
            "a0f54334df36770b335c051e540ba40afcbf8378"
        );
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("https://github.com/NixOS/patchelf.git")
        );
        assert_eq!(deps[0].input_type, FlakeInputType::Git);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "includes ijq from sourcehut without a flake" — nix/extract.spec.ts line 358
    #[test]
    fn includes_sourcehut_input_without_flake() {
        let content = r#"{
  "nodes": {
    "ijq": {
      "flake": false,
      "locked": {
        "lastModified": 1723569650,
        "narHash": "sha256-Ho/sAhEUeSug52JALgjrKVUPCBe8+PovbJj/lniKxp8=",
        "owner": "~gpanders",
        "repo": "ijq",
        "rev": "88f0d9ae98942bf49cba302c42b2a0f6e05f9b58",
        "type": "sourcehut"
      },
      "original": {
        "owner": "~gpanders",
        "repo": "ijq",
        "type": "sourcehut"
      }
    },
    "root": {
      "inputs": {
        "ijq": "ijq"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "ijq");
        assert_eq!(
            deps[0].locked_rev,
            "88f0d9ae98942bf49cba302c42b2a0f6e05f9b58"
        );
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("https://git.sr.ht/~gpanders/ijq")
        );
        assert_eq!(deps[0].input_type, FlakeInputType::Sourcehut);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "includes home-manager from gitlab" — nix/extract.spec.ts line 399
    #[test]
    fn includes_gitlab_input() {
        let content = r#"{
  "nodes": {
    "home-manager": {
      "flake": false,
      "locked": {
        "lastModified": 1728650932,
        "narHash": "sha256-mGKzqdsRyLnGNl6WjEr7+sghGgBtYHhJQ4mjpgRTCsU=",
        "owner": "rycee",
        "repo": "home-manager",
        "rev": "65ae9c147349829d3df0222151f53f79821c5134",
        "type": "gitlab"
      },
      "original": {
        "owner": "rycee",
        "repo": "home-manager",
        "type": "gitlab"
      }
    },
    "root": {
      "inputs": {
        "home-manager": "home-manager"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "home-manager");
        assert_eq!(
            deps[0].locked_rev,
            "65ae9c147349829d3df0222151f53f79821c5134"
        );
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("https://gitlab.com/rycee/home-manager")
        );
        assert_eq!(deps[0].input_type, FlakeInputType::Gitlab);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "test other version" — nix/extract.spec.ts line 440
    #[test]
    fn other_lockfile_version_returns_empty() {
        let content = r#"{
  "nodes": {
    "root": {}
  },
  "root": "root",
  "version": 6
}"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "includes nixpkgs with ref and shallow arguments" — nix/extract.spec.ts line 452
    #[test]
    fn includes_git_input_with_ref_and_shallow_arguments() {
        let content = r#"{
  "nodes": {
    "nixpkgs": {
      "locked": {
        "lastModified": 1728492678,
        "narHash": "sha256-9UTxR8eukdg+XZeHgxW5hQA9fIKHsKCdOIUycTryeVw=",
        "ref": "nixos-unstable",
        "rev": "5633bcff0c6162b9e4b5f1264264611e950c8ec7",
        "shallow": true,
        "type": "git",
        "url": "https://github.com/NixOS/nixpkgs"
      },
      "original": {
        "ref": "nixos-unstable",
        "shallow": true,
        "type": "git",
        "url": "https://github.com/NixOS/nixpkgs"
      }
    },
    "root": {
      "inputs": {
        "nixpkgs": "nixpkgs"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "nixpkgs");
        assert_eq!(
            deps[0].locked_rev,
            "5633bcff0c6162b9e4b5f1264264611e950c8ec7"
        );
        assert_eq!(deps[0].current_ref.as_deref(), Some("nixos-unstable"));
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("https://github.com/NixOS/nixpkgs")
        );
        assert_eq!(deps[0].input_type, FlakeInputType::Git);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "includes nixpkgs but using indirect type that cannot be updated" — nix/extract.spec.ts line 494
    #[test]
    fn original_indirect_input_is_skipped() {
        let content = r#"{
  "nodes": {
    "nixpkgs": {
      "locked": {
        "lastModified": 1728538411,
        "narHash": "sha256-f0SBJz1eZ2yOuKUr5CA9BHULGXVSn6miBuUWdTyhUhU=",
        "owner": "NixOS",
        "repo": "nixpkgs",
        "rev": "b69de56fac8c2b6f8fd27f2eca01dcda8e0a4221",
        "type": "github"
      },
      "original": {
        "id": "nixpkgs",
        "type": "indirect"
      }
    },
    "root": {
      "inputs": {
        "nixpkgs": "nixpkgs"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "nixpkgs");
        assert_eq!(deps[0].skip_reason, Some(NixSkipReason::Indirect));
    }

    // Ported: "includes nixpkgs but using indirect type and path locked type that cannot be updated" — nix/extract.spec.ts line 524
    #[test]
    fn original_indirect_locked_path_input_is_skipped_as_local_path() {
        let content = r#"{
  "nodes": {
    "nixpkgs": {
      "locked": {
        "lastModified": 1687274257,
        "narHash": "sha256-TutzPriQcZ8FghDhEolnHcYU2oHIG5XWF+/SUBNnAOE=",
        "path": "/nix/store/22qgs3skscd9bmrxv9xv4q5d4wwm5ppx-source",
        "rev": "2c9ecd1f0400076a4d6b2193ad468ff0a7e7fdc5",
        "type": "path"
      },
      "original": {
        "id": "nixpkgs",
        "type": "indirect"
      }
    },
    "root": {
      "inputs": {
        "nixpkgs": "nixpkgs"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "nixpkgs");
        assert_eq!(deps[0].skip_reason, Some(NixSkipReason::LocalPath));
    }

    // Ported: "includes flake from GitHub Enterprise" — nix/extract.spec.ts line 553
    #[test]
    fn includes_github_enterprise_input() {
        let content = r#"{
  "nodes": {
    "flake-utils": {
      "locked": {
        "owner": "numtide",
        "repo": "flake-utils",
        "rev": "c1dfcf08411b08f6b8615f7d8971a2bfa81d5e8a",
        "type": "github"
      },
      "original": {
        "owner": "numtide",
        "repo": "flake-utils",
        "type": "github"
      }
    },
    "nixpkgs-extra-pkgs": {
      "inputs": {
        "flake-utils": "flake-utils"
      },
      "locked": {
        "host": "github.corp.example.com",
        "lastModified": 1728666512,
        "narHash": "sha256-p+l16Zzyl2DXG695yks6KQP7NkjsnEksu5GBvtL1QYg=",
        "owner": "my-org",
        "repo": "nixpkgs-extra-pkgs",
        "rev": "6bf2706348447df6f8b86b1c3e54f87b0afda84f",
        "type": "github"
      },
      "original": {
        "host": "github.corp.example.com",
        "owner": "my-org",
        "repo": "nixpkgs-extra-pkgs",
        "type": "github"
      }
    },
    "root": {
      "inputs": {
        "nixpkgs-extra-pkgs": "nixpkgs-extra-pkgs"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "nixpkgs-extra-pkgs");
        assert_eq!(
            deps[0].locked_rev,
            "6bf2706348447df6f8b86b1c3e54f87b0afda84f"
        );
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("https://github.corp.example.com/my-org/nixpkgs-extra-pkgs")
        );
        assert_eq!(deps[0].input_type, FlakeInputType::Github);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "includes flake with tarball type" — nix/extract.spec.ts line 649
    #[test]
    fn includes_tarball_input_with_archive_url() {
        let content = r#"{
  "nodes": {
    "data-mesher": {
      "inputs": {
        "nixpkgs": "nixpkgs"
      },
      "locked": {
        "lastModified": 1727355895,
        "narHash": "sha256-grZIaLgk5GgoDuTt49RTCLBh458H4YJdIAU4B3onXRw=",
        "rev": "c7e39452affcc0f89e023091524e38b3aaf109e9",
        "type": "tarball",
        "url": "https://git.clan.lol/api/v1/repos/clan/data-mesher/archive/c7e39452affcc0f89e023091524e38b3aaf109e9.tar.gz"
      },
      "original": {
        "type": "tarball",
        "url": "https://git.clan.lol/clan/data-mesher/archive/main.tar.gz"
      }
    },
    "root": {
      "inputs": {
        "data-mesher": "data-mesher"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "data-mesher");
        assert_eq!(
            deps[0].locked_rev,
            "c7e39452affcc0f89e023091524e38b3aaf109e9"
        );
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("https://git.clan.lol/clan/data-mesher")
        );
        assert_eq!(deps[0].input_type, FlakeInputType::Tarball);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "includes flake with nixpkgs channel as tarball type" — nix/extract.spec.ts line 897
    #[test]
    fn includes_nixpkgs_channel_tarball_input() {
        let content = r#"{
  "nodes": {
    "nixpkgs": {
      "locked": {
        "lastModified": 1756904031,
        "narHash": "sha256-V29Bu1nR6Ayt+uUhf/6L43DSxb66BQ+8E2wH1GHa5IA=",
        "rev": "0e6684e6c5755325f801bda1751a8a4038145d7d",
        "type": "tarball",
        "url": "https://releases.nixos.org/nixos/25.05/nixos-25.05.809350.0e6684e6c575/nixexprs.tar.xz"
      },
      "original": {
        "type": "tarball",
        "url": "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.xz"
      }
    },
    "root": {
      "inputs": {
        "nixpkgs": "nixpkgs"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "nixpkgs");
        assert_eq!(
            deps[0].locked_rev,
            "0e6684e6c5755325f801bda1751a8a4038145d7d"
        );
        assert_eq!(deps[0].current_ref.as_deref(), Some("nixpkgs-unstable"));
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("https://github.com/NixOS/nixpkgs")
        );
        assert_eq!(deps[0].input_type, FlakeInputType::Tarball);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "includes flake with only tarball type" — nix/extract.spec.ts line 790
    #[test]
    fn tarball_without_locked_rev_is_skipped_as_no_rev() {
        let content = r#"{
  "nodes": {
    "nixpkgs-lib": {
      "locked": {
        "lastModified": 1738452942,
        "narHash": "sha256-vJzFZGaCpnmo7I6i416HaBLpC+hvcURh/BQwROcGIp8=",
        "type": "tarball",
        "url": "https://github.com/NixOS/nixpkgs/archive/072a6db25e947df2f31aab9eccd0ab75d5b2da11.tar.gz"
      },
      "original": {
        "type": "tarball",
        "url": "https://github.com/NixOS/nixpkgs/archive/072a6db25e947df2f31aab9eccd0ab75d5b2da11.tar.gz"
      }
    },
    "root": {
      "inputs": {
        "nixpkgs-lib": "nixpkgs-lib"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "nixpkgs-lib");
        assert_eq!(deps[0].skip_reason, Some(NixSkipReason::NoRev));
    }

    // Ported: "includes flake with nixpkgs-lib as tarball type" — nix/extract.spec.ts line 818
    #[test]
    fn ignores_transitive_nixpkgs_lib_tarball_while_extracting_root_inputs() {
        let content = r#"{
  "nodes": {
    "flake-parts": {
      "inputs": {
        "nixpkgs-lib": "nixpkgs-lib"
      },
      "locked": {
        "lastModified": 1733312601,
        "narHash": "sha256-4pDvzqnegAfRkPwO3wmwBhVi/Sye1mzps0zHWYnP88c=",
        "owner": "hercules-ci",
        "repo": "flake-parts",
        "rev": "205b12d8b7cd4802fbcb8e8ef6a0f1408781a4f9",
        "type": "github"
      },
      "original": {
        "owner": "hercules-ci",
        "repo": "flake-parts",
        "type": "github"
      }
    },
    "nixpkgs": {
      "locked": {
        "lastModified": 1734649271,
        "narHash": "sha256-4EVBRhOjMDuGtMaofAIqzJbg4Ql7Ai0PSeuVZTHjyKQ=",
        "owner": "nixos",
        "repo": "nixpkgs",
        "rev": "d70bd19e0a38ad4790d3913bf08fcbfc9eeca507",
        "type": "github"
      },
      "original": {
        "owner": "nixos",
        "ref": "nixos-unstable",
        "repo": "nixpkgs",
        "type": "github"
      }
    },
    "nixpkgs-lib": {
      "locked": {
        "lastModified": 1733096140,
        "narHash": "sha256-1qRH7uAUsyQI7R1Uwl4T+XvdNv778H0Nb5njNrqvylY=",
        "type": "tarball",
        "url": "https://github.com/NixOS/nixpkgs/archive/5487e69da40cbd611ab2cadee0b4637225f7cfae.tar.gz"
      },
      "original": {
        "type": "tarball",
        "url": "https://github.com/NixOS/nixpkgs/archive/5487e69da40cbd611ab2cadee0b4637225f7cfae.tar.gz"
      }
    },
    "root": {
      "inputs": {
        "flake-parts": "flake-parts",
        "nixpkgs": "nixpkgs"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 2);
        assert!(!deps.iter().any(|dep| dep.input_name == "nixpkgs-lib"));

        let flake_parts = deps
            .iter()
            .find(|dep| dep.input_name == "flake-parts")
            .unwrap();
        assert_eq!(
            flake_parts.package_name.as_deref(),
            Some("https://github.com/hercules-ci/flake-parts")
        );
        assert_eq!(
            flake_parts.locked_rev,
            "205b12d8b7cd4802fbcb8e8ef6a0f1408781a4f9"
        );

        let nixpkgs = deps.iter().find(|dep| dep.input_name == "nixpkgs").unwrap();
        assert_eq!(nixpkgs.current_ref.as_deref(), Some("nixos-unstable"));
        assert_eq!(
            nixpkgs.package_name.as_deref(),
            Some("https://github.com/nixos/nixpkgs")
        );
    }

    // Ported: "includes tarball flake with ref when original has rev" — nix/extract.spec.ts line 1280
    #[test]
    fn includes_tarball_input_ref_and_current_digest() {
        let content = r#"{
  "nodes": {
    "data-mesher": {
      "locked": {
        "lastModified": 1727355895,
        "narHash": "sha256-grZIaLgk5GgoDuTt49RTCLBh458H4YJdIAU4B3onXRw=",
        "rev": "c7e39452affcc0f89e023091524e38b3aaf109e9",
        "type": "tarball",
        "url": "https://git.clan.lol/api/v1/repos/clan/data-mesher/archive/c7e39452affcc0f89e023091524e38b3aaf109e9.tar.gz"
      },
      "original": {
        "type": "tarball",
        "url": "https://git.clan.lol/clan/data-mesher/archive/main.tar.gz",
        "ref": "main",
        "rev": "specific-commit-hash"
      }
    },
    "root": {
      "inputs": {
        "data-mesher": "data-mesher"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "data-mesher");
        assert_eq!(deps[0].current_ref.as_deref(), Some("main"));
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("specific-commit-hash")
        );
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("https://git.clan.lol/clan/data-mesher")
        );
        assert_eq!(deps[0].input_type, FlakeInputType::Tarball);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "uri decode gitlab subgroup" — nix/extract.spec.ts line 750
    #[test]
    fn decodes_gitlab_subgroup_owner() {
        let content = r#"{
  "nodes": {
    "subgroup-project": {
      "locked": {
        "lastModified": 1739792862,
        "narHash": "sha256-n0MrSIZZknq2OqOYgNS0iMp2yVRekpBFGhrhsT7aXGg=",
        "owner": "group%2Fsub-group",
        "repo": "subgroup-project",
        "rev": "24b560624f154c9e962d146217b2a964faaf2055",
        "type": "gitlab"
      },
      "original": {
        "owner": "group%2Fsub-group",
        "repo": "subgroup-project",
        "type": "gitlab"
      }
    },
    "root": {
      "inputs": {
        "subgroup-project": "subgroup-project"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "subgroup-project");
        assert_eq!(
            deps[0].locked_rev,
            "24b560624f154c9e962d146217b2a964faaf2055"
        );
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("https://gitlab.com/group/sub-group/subgroup-project")
        );
        assert_eq!(deps[0].input_type, FlakeInputType::Gitlab);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "finds currentDigest correctly when input sha is pinned" — nix/extract.spec.ts line 937
    #[test]
    fn extracts_current_digest_from_original_rev() {
        let content = r#"{
  "nodes": {
    "disko": {
      "locked": {
        "lastModified": 1744145203,
        "narHash": "sha256-I2oILRiJ6G+BOSjY+0dGrTPe080L3pbKpc+gCV3Nmyk=",
        "owner": "nix-community",
        "repo": "disko",
        "rev": "76c0a6dba345490508f36c1aa3c7ba5b6b460989",
        "type": "github"
      },
      "original": {
        "owner": "nix-community",
        "repo": "disko",
        "rev": "76c0a6dba345490508f36c1aa3c7ba5b6b460989",
        "type": "github"
      }
    },
    "root": {
      "inputs": {
        "disko": "disko"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "disko");
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("76c0a6dba345490508f36c1aa3c7ba5b6b460989")
        );
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("https://github.com/nix-community/disko")
        );
        assert_eq!(deps[0].input_type, FlakeInputType::Github);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "does not duplicate nixpkgs dependency" — nix/extract.spec.ts line 983
    #[test]
    fn package_file_does_not_duplicate_nixpkgs_dependency() {
        let flake_nix = r#"{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-21.11";
  };
}"#;
        let content = r#"{
  "nodes": {
    "nixpkgs": {
      "locked": {
        "lastModified": 1756904031,
        "narHash": "sha256-V29Bu1nR6Ayt+uUhf/6L43DSxb66BQ+8E2wH1GHa5IA=",
        "rev": "0e6684e6c5755325f801bda1751a8a4038145d7d",
        "type": "tarball",
        "url": "https://releases.nixos.org/nixos/25.05/nixos-25.05.809350.0e6684e6c575/nixexprs.tar.xz"
      },
      "original": {
        "type": "tarball",
        "url": "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.xz"
      }
    },
    "root": {
      "inputs": {
        "nixpkgs": "nixpkgs"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract_package_file(Some(flake_nix), Some(content)).unwrap();
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "nixpkgs");
        assert_eq!(deps[0].current_ref.as_deref(), Some("nixpkgs-unstable"));
        assert_eq!(
            deps[0].locked_rev,
            "0e6684e6c5755325f801bda1751a8a4038145d7d"
        );
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("https://github.com/NixOS/nixpkgs")
        );
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "handles currentDigest replacement when config provided" — nix/extract.spec.ts line 1065
    #[test]
    fn replaces_current_digest_when_config_matches_flake_nix() {
        let flake_nix = r#"{
  inputs = {
    disko.url = "github:nix-community/disko/newdigest123";
  };
}"#;
        let content = r#"{
  "nodes": {
    "disko": {
      "locked": {
        "lastModified": 1744145203,
        "narHash": "sha256-I2oILRiJ6G+BOSjY+0dGrTPe080L3pbKpc+gCV3Nmyk=",
        "owner": "nix-community",
        "repo": "disko",
        "rev": "76c0a6dba345490508f36c1aa3c7ba5b6b460989",
        "type": "github"
      },
      "original": {
        "owner": "nix-community",
        "repo": "disko",
        "rev": "olddigest123",
        "type": "github"
      }
    },
    "root": {
      "inputs": {
        "disko": "disko"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract_with_config(
            content,
            NixExtractConfig {
                flake_nix_content: Some(flake_nix),
                current_digest: Some("olddigest123"),
                new_digest: Some("newdigest123"),
            },
        );
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "disko");
        assert_eq!(deps[0].current_digest.as_deref(), Some("newdigest123"));
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("https://github.com/nix-community/disko")
        );
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "returns null when flake.lock file cannot be read" — nix/extract.spec.ts line 1028
    #[test]
    fn package_file_returns_none_when_flake_lock_missing() {
        assert!(extract_package_file(Some(""), None).is_none());
    }

    // Ported: "returns null when flake.nix file cannot be read" — nix/extract.spec.ts line 1033
    #[test]
    fn package_file_returns_none_when_flake_nix_missing() {
        let content = r#"{
  "nodes": {
    "root": {}
  },
  "root": "root",
  "version": 7
}"#;
        assert!(extract_package_file(None, Some(content)).is_none());
    }

    // Ported: "includes nixpkgs with ref when original has rev" — nix/extract.spec.ts line 1112
    #[test]
    fn includes_nixpkgs_ref_and_original_rev() {
        let content = r#"{
  "nodes": {
    "nixpkgs": {
      "locked": {
        "lastModified": 1720031269,
        "narHash": "sha256-rwz8NJZV+387rnWpTYcXaRNvzUSnnF9aHONoJIYmiUQ=",
        "owner": "NixOS",
        "repo": "nixpkgs",
        "rev": "9f4128e00b0ae8ec65918efeba59db998750ead6",
        "type": "github"
      },
      "original": {
        "owner": "NixOS",
        "ref": "nixos-unstable",
        "repo": "nixpkgs",
        "rev": "specific-commit-hash",
        "type": "github"
      }
    },
    "root": {
      "inputs": {
        "nixpkgs": "nixpkgs"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "nixpkgs");
        assert_eq!(deps[0].current_ref.as_deref(), Some("nixos-unstable"));
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("specific-commit-hash")
        );
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("https://github.com/NixOS/nixpkgs")
        );
        assert_eq!(deps[0].input_type, FlakeInputType::Github);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "includes github flake with ref when original has rev" — nix/extract.spec.ts line 1154
    #[test]
    fn includes_github_ref_and_original_rev() {
        let content = r#"{
  "nodes": {
    "flake-utils": {
      "locked": {
        "lastModified": 1726560853,
        "narHash": "sha256-X6rJYSESBVr3hBoH0WbKE5KvhPU5bloyZ2L4K60/fPQ=",
        "owner": "numtide",
        "repo": "flake-utils",
        "rev": "c1dfcf08411b08f6b8615f7d8971a2bfa81d5e8a",
        "type": "github"
      },
      "original": {
        "owner": "numtide",
        "repo": "flake-utils",
        "ref": "main",
        "rev": "specific-commit-hash",
        "type": "github"
      }
    },
    "root": {
      "inputs": {
        "flake-utils": "flake-utils"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "flake-utils");
        assert_eq!(deps[0].current_ref.as_deref(), Some("main"));
        assert_eq!(
            deps[0].current_digest.as_deref(),
            Some("specific-commit-hash")
        );
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("https://github.com/numtide/flake-utils")
        );
        assert_eq!(deps[0].input_type, FlakeInputType::Github);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "includes gitlab flake with custom host" — nix/extract.spec.ts line 1196
    #[test]
    fn includes_gitlab_input_with_custom_host() {
        let content = r#"{
  "nodes": {
    "custom-project": {
      "locked": {
        "lastModified": 1728650932,
        "narHash": "sha256-mGKzqdsRyLnGNl6WjEr7+sghGgBtYHhJQ4mjpgRTCsU=",
        "owner": "group",
        "repo": "project",
        "rev": "65ae9c147349829d3df0222151f53f79821c5134",
        "type": "gitlab",
        "host": "gitlab.example.com"
      },
      "original": {
        "owner": "group",
        "repo": "project",
        "type": "gitlab",
        "host": "gitlab.example.com"
      }
    },
    "root": {
      "inputs": {
        "custom-project": "custom-project"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "custom-project");
        assert_eq!(
            deps[0].locked_rev,
            "65ae9c147349829d3df0222151f53f79821c5134"
        );
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("https://gitlab.example.com/group/project")
        );
        assert_eq!(deps[0].input_type, FlakeInputType::Gitlab);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "includes sourcehut flake with custom host" — nix/extract.spec.ts line 1238
    #[test]
    fn includes_sourcehut_input_with_custom_host() {
        let content = r#"{
  "nodes": {
    "custom-project": {
      "locked": {
        "lastModified": 1723569650,
        "narHash": "sha256-Ho/sAhEUeSug52JALgjrKVUPCBe8+PovbJj/lniKxp8=",
        "owner": "~user",
        "repo": "project",
        "rev": "88f0d9ae98942bf49cba302c42b2a0f6e05f9b58",
        "type": "sourcehut",
        "host": "git.custom.org"
      },
      "original": {
        "owner": "~user",
        "repo": "project",
        "type": "sourcehut",
        "host": "git.custom.org"
      }
    },
    "root": {
      "inputs": {
        "custom-project": "custom-project"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "custom-project");
        assert_eq!(
            deps[0].locked_rev,
            "88f0d9ae98942bf49cba302c42b2a0f6e05f9b58"
        );
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("https://git.custom.org/~user/project")
        );
        assert_eq!(deps[0].input_type, FlakeInputType::Sourcehut);
        assert!(deps[0].skip_reason.is_none());
    }

    // Ported: "returns null when flake.lock has invalid JSON" — nix/extract.spec.ts line 1046
    #[test]
    fn invalid_json_returns_empty() {
        assert!(extract("not json").is_empty());
    }

    // Ported: "returns deps when no root inputs but deps exist" — nix/extract.spec.ts line 1051
    #[test]
    fn root_without_inputs_returns_empty() {
        let content = r#"{
  "nodes": {
    "root": {}
  },
  "root": "root",
  "version": 7
}"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "handles unknown flake lock type" — nix/extract.spec.ts line 1321
    #[test]
    fn unknown_flake_lock_type_returns_empty() {
        let content = r#"{
  "nodes": {
    "unknown-flake": {
      "locked": {
        "rev": "c7e39452affcc0f89e023091524e38b3aaf109e9",
        "type": "unknown-type"
      },
      "original": {
        "type": "unknown-type"
      }
    },
    "root": {
      "inputs": {
        "unknown-flake": "unknown-flake"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        assert!(extract(content).is_empty());
    }

    // Ported: "ignores unsupported file type and still extracts other inputs" — nix/extract.spec.ts line 1348
    #[test]
    fn unsupported_file_type_is_ignored_while_other_inputs_extract() {
        let content = r#"{
  "nodes": {
    "file": {
      "locked": {
        "type": "file",
        "url": "https://raw.githubusercontent.com/NixOS/nixpkgs/abc/README.md"
      },
      "original": {
        "type": "file",
        "url": "https://raw.githubusercontent.com/NixOS/nixpkgs/abc/README.md"
      }
    },
    "nixpkgs": {
      "locked": {
        "owner": "NixOS",
        "repo": "nixpkgs",
        "rev": "8eb28adfa3dc4de28e792e3bf49fcf9007ca8ac9",
        "type": "github"
      },
      "original": {
        "owner": "NixOS",
        "ref": "nixos-unstable",
        "repo": "nixpkgs",
        "type": "github"
      }
    },
    "root": {
      "inputs": {
        "file": "file",
        "nixpkgs": "nixpkgs"
      }
    }
  },
  "root": "root",
  "version": 7
}"#;
        let deps = extract(content);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0].input_name, "nixpkgs");
        assert_eq!(
            deps[0].locked_rev,
            "8eb28adfa3dc4de28e792e3bf49fcf9007ca8ac9"
        );
        assert_eq!(
            deps[0].package_name.as_deref(),
            Some("https://github.com/NixOS/nixpkgs")
        );
    }
}
