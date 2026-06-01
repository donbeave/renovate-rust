//! Terraform artifact runner — `.terraform.lock.hcl` regeneration.
//!
//! Wraps `extractors::terraform::update_terraform_artifacts` in the
//! `ArtifactRunner` trait so the CLI can invoke it.

use std::future::Future;
use std::pin::Pin;

use crate::artifacts::{ArtifactError, ArtifactResult, ArtifactRunner, UpdateArtifact};
use crate::extractors::terraform::{TerraformArtifactConfig, TerraformArtifactDep};

/// Artifact runner for Terraform projects.
#[derive(Debug, Clone, Default)]
pub struct TerraformArtifactRunner;

impl TerraformArtifactRunner {
    /// Create a new terraform artifact runner.
    pub fn new() -> Self {
        Self
    }
}

impl ArtifactRunner for TerraformArtifactRunner {
    fn update_artifacts(
        &self,
        input: &UpdateArtifact,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Vec<ArtifactResult>>, ArtifactError>> + Send + '_>>
    {
        let package_file_name = input.package_file_name.clone();
        let updated_deps = input.updated_deps.clone();
        let config = input.config.clone();

        Box::pin(async move {
            let tf_deps: Vec<TerraformArtifactDep> = updated_deps
                .iter()
                .map(|d| TerraformArtifactDep {
                    dep_name: d.dep_name.clone(),
                    dep_type: d.datasource.as_ref().map(|ds| {
                        if ds == "terraform-provider" {
                            "provider".to_owned()
                        } else if ds == "terraform-module" {
                            "module".to_owned()
                        } else {
                            "provider".to_owned()
                        }
                    }),
                    package_name: d.package_name.clone(),
                    current_value: d.current_value.clone(),
                    current_version: d.locked_version.clone(),
                    new_value: d.new_value.clone(),
                    new_version: d.new_version.clone(),
                    registry_urls: Vec::new(),
                    versioning: None,
                    is_lockfile_update: false,
                })
                .collect();

            let tf_config = TerraformArtifactConfig {
                is_lock_file_maintenance: config.is_lockfile_maintenance,
            };

            match crate::extractors::terraform::update_terraform_artifacts(
                &config.lock_file_dir,
                &package_file_name,
                &tf_deps,
                &tf_config,
            )
            .await
            {
                Ok(Some(results)) => Ok(Some(results)),
                Ok(None) => Ok(None),
                Err(e) => Err(e),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::{ArtifactConfig, UpdateArtifact, UpdatedDep};

    fn make_runner() -> TerraformArtifactRunner {
        TerraformArtifactRunner::new()
    }

    #[tokio::test]
    async fn returns_none_when_no_lock_file() {
        let dir = tempfile::tempdir().unwrap();
        let runner = make_runner();
        let input = UpdateArtifact {
            package_file_name: "main.tf".to_owned(),
            updated_deps: vec![UpdatedDep {
                dep_name: "hashicorp/aws".to_owned(),
                package_name: Some("hashicorp/aws".to_owned()),
                current_value: Some("~> 3.0".to_owned()),
                new_value: Some("~> 4.0".to_owned()),
                locked_version: Some("3.76.1".to_owned()),
                new_version: Some("4.67.0".to_owned()),
                package_file: "main.tf".to_owned(),
                manager: "terraform".to_owned(),
                datasource: Some("terraform-provider".to_owned()),
                update_type: None,
            }],
            new_package_file_content: String::new(),
            config: ArtifactConfig {
                lock_file_dir: dir.path().to_path_buf(),
                ..Default::default()
            },
        };
        let result = runner.update_artifacts(&input).await;
        assert!(result.unwrap().is_none());
    }
}
