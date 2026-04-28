//! Dev Container (`devcontainer.json`) Docker image + features extractor.
//!
//! Extracts:
//! - The top-level `"image"` field as a Docker dep.
//! - Each entry in `"features"` as an OCI Docker dep.
//! - For the four known devcontainers/features (node, go, python, ruby),
//!   a separate version dep routed to the same GitHub datasource as
//!   the corresponding version-file manager.
//!
//! Renovate reference:
//! - `lib/modules/manager/devcontainer/extract.ts`
//! - Patterns: `^.devcontainer/devcontainer.json$`, `^.devcontainer.json$`

use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

use crate::extractors::asdf::AsdfDatasource;
use crate::extractors::dockerfile::{DockerfileExtractedDep, classify_image_ref};
use crate::extractors::version_file::VersionFileDep;

#[derive(Debug, Deserialize)]
struct DevContainerFile {
    image: Option<String>,
    features: Option<HashMap<String, Value>>,
}

/// All deps extracted from a `devcontainer.json`.
#[derive(Debug, Default, Clone)]
pub struct DevContainerDeps {
    /// Docker image refs: the top-level `image` plus each feature key.
    pub docker_deps: Vec<DockerfileExtractedDep>,
    /// Tool version deps from known devcontainer features (node, go, python, ruby).
    pub version_deps: Vec<VersionFileDep>,
}

/// Mapping from feature dep-name prefix → tool info.
struct KnownFeature {
    /// The dep-name prefix (image without tag).
    prefix: &'static str,
    tool: &'static str,
    datasource: AsdfDatasource,
}

static KNOWN_FEATURES: &[KnownFeature] = &[
    KnownFeature {
        prefix: "ghcr.io/devcontainers/features/node",
        tool: "nodejs",
        datasource: AsdfDatasource::GithubReleases {
            repo: "nodejs/node",
            tag_strip: "v",
        },
    },
    KnownFeature {
        prefix: "ghcr.io/devcontainers/features/go",
        tool: "golang",
        datasource: AsdfDatasource::GithubTags {
            repo: "golang/go",
            tag_strip: "go",
        },
    },
    KnownFeature {
        prefix: "ghcr.io/devcontainers/features/python",
        tool: "python",
        datasource: AsdfDatasource::GithubTags {
            repo: "python/cpython",
            tag_strip: "v",
        },
    },
    KnownFeature {
        prefix: "ghcr.io/devcontainers/features/ruby",
        tool: "ruby",
        datasource: AsdfDatasource::GithubTags {
            repo: "ruby/ruby",
            tag_strip: "v",
        },
    },
];

/// Extract all deps from a `devcontainer.json` file.
pub fn extract(content: &str) -> DevContainerDeps {
    let file: DevContainerFile = match serde_json::from_str(content.trim()) {
        Ok(f) => f,
        Err(_) => return DevContainerDeps::default(),
    };

    let mut result = DevContainerDeps::default();

    if let Some(image) = &file.image
        && !image.is_empty()
    {
        result.docker_deps.push(classify_image_ref(image));
    }

    if let Some(features) = &file.features {
        for (feature_ref, value) in features {
            let docker_dep = classify_image_ref(feature_ref);

            // Check if this is a known feature (match on dep-name prefix).
            if let Some(known) = KNOWN_FEATURES
                .iter()
                .find(|kf| docker_dep.image.starts_with(kf.prefix))
            {
                // The OCI ref for the feature container itself.
                result.docker_deps.push(docker_dep);

                // Extract the runtime version, if present.
                let version_str = value
                    .get("version")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .trim()
                    .trim_start_matches('v');

                if !version_str.is_empty() {
                    result.version_deps.push(VersionFileDep {
                        tool: known.tool,
                        current_value: version_str.to_owned(),
                        datasource: known.datasource.clone(),
                    });
                }
            } else {
                result.docker_deps.push(docker_dep);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_image() {
        let content = r#"{"image": "mcr.microsoft.com/devcontainers/base:ubuntu-22.04"}"#;
        let deps = extract(content);
        assert_eq!(deps.docker_deps.len(), 1);
        assert_eq!(
            deps.docker_deps[0].image,
            "mcr.microsoft.com/devcontainers/base"
        );
        assert_eq!(deps.docker_deps[0].tag.as_deref(), Some("ubuntu-22.04"));
    }

    #[test]
    fn no_image_returns_empty() {
        let content = r#"{"name": "Dev Container"}"#;
        let deps = extract(content);
        assert!(deps.docker_deps.is_empty());
    }

    #[test]
    fn invalid_json_returns_empty() {
        let deps = extract("not json");
        assert!(deps.docker_deps.is_empty());
        assert!(deps.version_deps.is_empty());
    }

    #[test]
    fn extracts_node_feature_and_version() {
        let content = r#"{
  "features": {
    "ghcr.io/devcontainers/features/node:1": {
      "version": "18"
    }
  }
}"#;
        let deps = extract(content);
        assert_eq!(deps.docker_deps.len(), 1);
        assert_eq!(
            deps.docker_deps[0].image,
            "ghcr.io/devcontainers/features/node"
        );
        assert_eq!(deps.version_deps.len(), 1);
        assert_eq!(deps.version_deps[0].tool, "nodejs");
        assert_eq!(deps.version_deps[0].current_value, "18");
        assert_eq!(
            deps.version_deps[0].datasource,
            AsdfDatasource::GithubReleases {
                repo: "nodejs/node",
                tag_strip: "v",
            }
        );
    }

    #[test]
    fn extracts_go_feature_and_version() {
        let content = r#"{
  "features": {
    "ghcr.io/devcontainers/features/go:1": {"version": "1.21"}
  }
}"#;
        let deps = extract(content);
        assert_eq!(deps.version_deps.len(), 1);
        assert_eq!(deps.version_deps[0].tool, "golang");
        assert_eq!(deps.version_deps[0].current_value, "1.21");
    }

    #[test]
    fn feature_without_version_skipped_from_version_deps() {
        let content = r#"{
  "features": {
    "ghcr.io/devcontainers/features/python:1": {}
  }
}"#;
        let deps = extract(content);
        assert_eq!(deps.docker_deps.len(), 1);
        assert!(deps.version_deps.is_empty());
    }

    #[test]
    fn image_and_feature_combined() {
        let content = r#"{
  "image": "node:20",
  "features": {
    "ghcr.io/devcontainers/features/go:1": {"version": "1.21"}
  }
}"#;
        let deps = extract(content);
        assert_eq!(deps.docker_deps.len(), 2);
        assert_eq!(deps.version_deps.len(), 1);
    }
}
