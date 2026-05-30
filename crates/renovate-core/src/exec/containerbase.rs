use std::collections::HashMap;
use std::sync::LazyLock;

use crate::exec::error::ExecError;
use crate::exec::types::{BinarySource, ToolConstraint};

#[derive(Debug, Clone)]
pub struct ToolConfig {
    pub datasource: String,
    pub extract_version: Option<String>,
    pub package_name: String,
    pub versioning: String,
}

fn build_tool_configs() -> HashMap<&'static str, ToolConfig> {
    let mut m = HashMap::new();
    m.insert("node", ToolConfig {
        datasource: "github-releases".to_owned(),
        package_name: "containerbase/node-prebuild".to_owned(),
        versioning: "node".to_owned(),
        extract_version: None,
    });
    m.insert("npm", ToolConfig {
        datasource: "npm".to_owned(),
        package_name: "npm".to_owned(),
        versioning: "npm".to_owned(),
        extract_version: None,
    });
    m.insert("pnpm", ToolConfig {
        datasource: "npm".to_owned(),
        package_name: "pnpm".to_owned(),
        versioning: "npm".to_owned(),
        extract_version: None,
    });
    m.insert("yarn", ToolConfig {
        datasource: "npm".to_owned(),
        package_name: "yarn".to_owned(),
        versioning: "npm".to_owned(),
        extract_version: None,
    });
    m.insert("bun", ToolConfig {
        datasource: "npm".to_owned(),
        package_name: "@oven/bun".to_owned(),
        versioning: "npm".to_owned(),
        extract_version: None,
    });
    m.insert("python", ToolConfig {
        datasource: "github-releases".to_owned(),
        package_name: "containerbase/python-prebuild".to_owned(),
        versioning: "python".to_owned(),
        extract_version: None,
    });
    m.insert("pip", ToolConfig {
        datasource: "pypi".to_owned(),
        package_name: "pip".to_owned(),
        versioning: "pep440".to_owned(),
        extract_version: None,
    });
    m.insert("pipenv", ToolConfig {
        datasource: "pypi".to_owned(),
        package_name: "pipenv".to_owned(),
        versioning: "pep440".to_owned(),
        extract_version: None,
    });
    m.insert("poetry", ToolConfig {
        datasource: "pypi".to_owned(),
        package_name: "poetry".to_owned(),
        versioning: "pep440".to_owned(),
        extract_version: None,
    });
    m.insert("golang", ToolConfig {
        datasource: "golang-version".to_owned(),
        package_name: "go".to_owned(),
        versioning: "go-mod-directive".to_owned(),
        extract_version: None,
    });
    m.insert("rust", ToolConfig {
        datasource: "github-releases".to_owned(),
        package_name: "rust-lang/rust".to_owned(),
        versioning: "semver".to_owned(),
        extract_version: None,
    });
    m.insert("java", ToolConfig {
        datasource: "adoptium-java".to_owned(),
        package_name: "java".to_owned(),
        versioning: "semver".to_owned(),
        extract_version: None,
    });
    m.insert("gradle", ToolConfig {
        datasource: "gradle-version".to_owned(),
        package_name: "gradle".to_owned(),
        versioning: "semver".to_owned(),
        extract_version: None,
    });
    m.insert("maven", ToolConfig {
        datasource: "maven".to_owned(),
        package_name: "org.apache.maven:apache-maven".to_owned(),
        versioning: "semver".to_owned(),
        extract_version: None,
    });
    m.insert("dotnet", ToolConfig {
        datasource: "github-releases".to_owned(),
        package_name: "dotnet/dotnet".to_owned(),
        versioning: "semver".to_owned(),
        extract_version: None,
    });
    m.insert("helm", ToolConfig {
        datasource: "github-releases".to_owned(),
        package_name: "helm/helm".to_owned(),
        versioning: "semver".to_owned(),
        extract_version: None,
    });
    m.insert("terraform", ToolConfig {
        datasource: "github-releases".to_owned(),
        package_name: "hashicorp/terraform".to_owned(),
        versioning: "semver".to_owned(),
        extract_version: None,
    });
    m.insert("bazel", ToolConfig {
        datasource: "github-releases".to_owned(),
        package_name: "bazelbuild/bazel".to_owned(),
        versioning: "semver".to_owned(),
        extract_version: None,
    });
    m.insert("flutter", ToolConfig {
        datasource: "github-releases".to_owned(),
        package_name: "flutter/flutter".to_owned(),
        versioning: "semver".to_owned(),
        extract_version: None,
    });
    m.insert("swift", ToolConfig {
        datasource: "github-releases".to_owned(),
        package_name: "swift-lang/swift".to_owned(),
        versioning: "semver".to_owned(),
        extract_version: None,
    });
    m.insert("cocoapods", ToolConfig {
        datasource: "ruby-version".to_owned(),
        package_name: "cocoapods".to_owned(),
        versioning: "ruby".to_owned(),
        extract_version: None,
    });
    m.insert("hashin", ToolConfig {
        datasource: "pypi".to_owned(),
        package_name: "hashin".to_owned(),
        versioning: "pep440".to_owned(),
        extract_version: None,
    });
    m.insert("pdm", ToolConfig {
        datasource: "pypi".to_owned(),
        package_name: "pdm".to_owned(),
        versioning: "pep440".to_owned(),
        extract_version: None,
    });
    m
}

static ALL_TOOL_CONFIG: LazyLock<HashMap<&'static str, ToolConfig>> =
    LazyLock::new(build_tool_configs);

pub fn get_tool_config(tool_name: &str) -> Option<&'static ToolConfig> {
    ALL_TOOL_CONFIG.get(tool_name)
}

pub fn supports_dynamic_install(tool_name: &str) -> bool {
    ALL_TOOL_CONFIG.contains_key(tool_name)
}

pub fn is_containerbase() -> bool {
    std::env::var("CONTAINERBASE").is_ok()
}

pub fn is_dynamic_install(
    binary_source: &BinarySource,
    tool_constraints: &[ToolConstraint],
) -> bool {
    if binary_source != &BinarySource::Install {
        return false;
    }
    if !is_containerbase() {
        return false;
    }
    if tool_constraints.is_empty() {
        return false;
    }
    tool_constraints
        .iter()
        .all(|tc| supports_dynamic_install(&tc.tool_name))
}

pub async fn resolve_constraint(
    tool_constraint: &ToolConstraint,
) -> Result<String, ExecError> {
    if let Some(ref constraint) = tool_constraint.constraint {
        if constraint.starts_with('=') {
            return Ok(constraint.trim_start_matches('=').to_owned());
        }
        Ok(constraint.clone())
    } else {
        Ok("latest".to_owned())
    }
}

pub async fn generate_install_commands(
    binary_source: &BinarySource,
    tool_constraints: &[ToolConstraint],
) -> Result<Vec<String>, ExecError> {
    if !is_dynamic_install(binary_source, tool_constraints) {
        return Ok(vec![]);
    }

    let mut commands = Vec::new();
    for tc in tool_constraints {
        let version = resolve_constraint(tc).await?;
        commands.push(format!("install-tool {} '{}'", tc.tool_name, version));
    }
    Ok(commands)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Rust-specific: containerbase behavior test
    #[test]
    fn get_tool_config_node() {
        let config = get_tool_config("node").unwrap();
        assert_eq!(config.datasource, "github-releases");
        assert_eq!(config.package_name, "containerbase/node-prebuild");
    }

    // Rust-specific: containerbase behavior test
    #[test]
    fn get_tool_config_unknown() {
        assert!(get_tool_config("unknown-tool").is_none());
    }

    // Rust-specific: containerbase behavior test
    #[test]
    fn supports_dynamic_install_known() {
        assert!(supports_dynamic_install("node"));
        assert!(supports_dynamic_install("python"));
        assert!(supports_dynamic_install("golang"));
        assert!(supports_dynamic_install("rust"));
    }

    // Rust-specific: containerbase behavior test
    #[test]
    fn supports_dynamic_install_unknown() {
        assert!(!supports_dynamic_install("foobar"));
    }

    // Ported: "returns false if binarySource is not install" — util/exec/containerbase.spec.ts line 22
    #[test]
    fn is_dynamic_install_requires_install_source() {
        let tc = vec![ToolConstraint {
            tool_name: "node".to_owned(),
            constraint: Some("18".to_owned()),
        }];
        assert!(!is_dynamic_install(&BinarySource::Global, &tc));
        assert!(!is_dynamic_install(&BinarySource::Docker, &tc));
    }

    // Ported: "returns false if not containerbase" — util/exec/containerbase.spec.ts line 26
    #[test]
    fn is_dynamic_install_requires_containerbase_env() {
        let tc = vec![ToolConstraint {
            tool_name: "node".to_owned(),
            constraint: Some("18".to_owned()),
        }];
        // When CONTAINERBASE env is not set, should return false even with Install source
        if std::env::var("CONTAINERBASE").is_err() {
            assert!(!is_dynamic_install(&BinarySource::Install, &tc));
        }
    }

    // Ported: "returns false if any unsupported tools" — util/exec/containerbase.spec.ts line 31
    #[test]
    fn is_dynamic_install_false_if_any_unsupported() {
        let tc = vec![
            ToolConstraint {
                tool_name: "node".to_owned(),
                constraint: None,
            },
            ToolConstraint {
                tool_name: "invalid-tool".to_owned(),
                constraint: None,
            },
        ];
        assert!(!is_dynamic_install(&BinarySource::Install, &tc));
    }

    #[tokio::test]
    async fn resolve_constraint_exact() {
        let tc = ToolConstraint {
            tool_name: "node".to_owned(),
            constraint: Some("=18.0.0".to_owned()),
        };
        let version = resolve_constraint(&tc).await.unwrap();
        assert_eq!(version, "18.0.0");
    }

    #[tokio::test]
    async fn resolve_constraint_range() {
        let tc = ToolConstraint {
            tool_name: "node".to_owned(),
            constraint: Some(">=16".to_owned()),
        };
        let version = resolve_constraint(&tc).await.unwrap();
        assert_eq!(version, ">=16");
    }

    #[tokio::test]
    async fn resolve_constraint_none() {
        let tc = ToolConstraint {
            tool_name: "node".to_owned(),
            constraint: None,
        };
        let version = resolve_constraint(&tc).await.unwrap();
        assert_eq!(version, "latest");
    }

    #[tokio::test]
    async fn generate_install_commands_not_dynamic() {
        let tc = vec![ToolConstraint {
            tool_name: "node".to_owned(),
            constraint: Some("18".to_owned()),
        }];
        let cmds = generate_install_commands(&BinarySource::Global, &tc)
            .await
            .unwrap();
        assert!(cmds.is_empty());
    }
}
