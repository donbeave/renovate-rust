use std::collections::HashMap;

use crate::exec::error::ExecError;
use crate::exec::types::DockerOptions;

pub const SIDECAR_NAME: &str = "sidecar";

#[derive(Debug, Clone, Default)]
pub struct DockerConfig {
    pub docker_child_prefix: String,
    pub docker_sidecar_image: String,
    pub docker_user: Option<String>,
    pub docker_cli_options: Option<String>,
    pub local_dir: Option<String>,
    pub cache_dir: Option<String>,
    pub containerbase_dir: Option<String>,
}

impl DockerConfig {
    pub fn sidecar_container_name(&self) -> String {
        let prefix = if self.docker_child_prefix.is_empty() {
            "renovate_"
        } else {
            &self.docker_child_prefix
        };
        format!("{}{}", prefix, SIDECAR_NAME)
    }
}

pub async fn generate_docker_command(
    commands: &[String],
    pre_commands: &[String],
    docker_options: &DockerOptions,
    docker_config: &DockerConfig,
    env_vars: &[String],
    cwd: Option<&str>,
    install_commands: &[String],
) -> Result<String, ExecError> {
    let mut parts = vec![
        "docker run --rm".to_owned(),
        format!("--name={}", docker_config.sidecar_container_name()),
        "--label=renovate_child".to_owned(),
    ];

    if let Some(ref user) = docker_config.docker_user {
        parts.push(format!("--user={}", user));
    }

    if let Some(ref cli_opts) = docker_config.docker_cli_options {
        parts.push(cli_opts.clone());
    }

    if let Some(ref local_dir) = docker_config.local_dir {
        parts.push(format!("-v \"{}\":\"{}\"", local_dir, local_dir));
    }

    if let Some(ref cache_dir) = docker_config.cache_dir {
        parts.push(format!("-v \"{}\":\"{}\"", cache_dir, cache_dir));
    }

    if let Some(ref cb_dir) = docker_config.containerbase_dir {
        if docker_config.cache_dir.as_ref() != Some(cb_dir) {
            parts.push(format!("-v \"{}\":\"{}\"", cb_dir, cb_dir));
        }
    }

    for vol in &docker_options.volumes {
        if !vol.is_empty() {
            if vol.contains(':') {
                parts.push(format!("-v \"{}\"", vol));
            } else {
                parts.push(format!("-v \"{}\":\"{}\"", vol, vol));
            }
        }
    }

    for var in env_vars {
        parts.push(format!("-e {}", var));
    }

    if let Some(docker_cwd) = docker_options.cwd.as_deref().or(cwd) {
        parts.push(format!("-w \"{}\"", docker_cwd));
    }

    parts.push(docker_config.docker_sidecar_image.clone());

    let mut all_commands: Vec<String> = Vec::new();
    all_commands.extend_from_slice(install_commands);
    all_commands.extend_from_slice(pre_commands);
    all_commands.extend_from_slice(commands);

    let joined = all_commands.join(" && ");
    parts.push(format!("bash -l -c \"{}\"", joined));

    Ok(parts.join(" "))
}

pub async fn remove_docker_container(
    _image: &str,
    prefix: &str,
) -> Result<(), ExecError> {
    let container_name = format!("{}{}", prefix, SIDECAR_NAME);
    let cmd = format!("docker rm -f {}", container_name);
    let env: HashMap<String, String> = std::env::vars().collect();
    let opts = crate::exec::types::ExecOptions {
        ignore_stdout: true,
        ..Default::default()
    };
    let _ = crate::exec::raw::raw_exec(&cmd, &opts, &env).await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sidecar_container_name_default() {
        let config = DockerConfig {
            docker_child_prefix: "renovate_".to_owned(),
            ..Default::default()
        };
        assert_eq!(config.sidecar_container_name(), "renovate_sidecar");
    }

    #[test]
    fn sidecar_container_name_custom() {
        let config = DockerConfig {
            docker_child_prefix: "custom_".to_owned(),
            ..Default::default()
        };
        assert_eq!(config.sidecar_container_name(), "custom_sidecar");
    }

    #[tokio::test]
    async fn generate_docker_command_basic() {
        let docker_options = DockerOptions::default();
        let docker_config = DockerConfig {
            docker_child_prefix: "renovate_".to_owned(),
            docker_sidecar_image: "renovate/sidecar".to_owned(),
            local_dir: Some("/tmp/repo".to_owned()),
            cache_dir: Some("/tmp/cache".to_owned()),
            ..Default::default()
        };
        let commands = vec!["npm install".to_owned()];
        let result = generate_docker_command(
            &commands,
            &[],
            &docker_options,
            &docker_config,
            &[],
            Some("/tmp/repo"),
            &[],
        )
        .await
        .unwrap();

        assert!(result.contains("docker run --rm"));
        assert!(result.contains("--name=renovate_sidecar"));
        assert!(result.contains("renovate/sidecar"));
        assert!(result.contains("bash -l -c"));
        assert!(result.contains("npm install"));
        assert!(result.contains("/tmp/repo"));
        assert!(result.contains("/tmp/cache"));
    }

    #[tokio::test]
    async fn generate_docker_command_with_volumes() {
        let docker_options = DockerOptions {
            volumes: vec!["/host/path:/container/path".to_owned()],
            ..Default::default()
        };
        let docker_config = DockerConfig {
            docker_child_prefix: "renovate_".to_owned(),
            docker_sidecar_image: "renovate/sidecar".to_owned(),
            ..Default::default()
        };
        let result = generate_docker_command(
            &["echo hello".to_owned()],
            &[],
            &docker_options,
            &docker_config,
            &[],
            None,
            &[],
        )
        .await
        .unwrap();

        assert!(result.contains("-v \"/host/path:/container/path\""));
    }

    #[tokio::test]
    async fn generate_docker_command_with_pre_commands_and_installs() {
        let docker_options = DockerOptions::default();
        let docker_config = DockerConfig {
            docker_child_prefix: "renovate_".to_owned(),
            docker_sidecar_image: "renovate/sidecar".to_owned(),
            ..Default::default()
        };
        let install_cmds = vec!["install-tool node 18".to_owned()];
        let pre_cmds = vec!["echo prep".to_owned()];
        let cmds = vec!["npm test".to_owned()];

        let result = generate_docker_command(
            &cmds,
            &pre_cmds,
            &docker_options,
            &docker_config,
            &[],
            None,
            &install_cmds,
        )
        .await
        .unwrap();

        let inner = result.split("bash -l -c \"").nth(1).unwrap();
        assert!(inner.starts_with("install-tool node 18 && echo prep && npm test"));
    }
}
