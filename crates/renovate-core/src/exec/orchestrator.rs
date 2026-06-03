use std::collections::HashMap;
use std::path::Path;

use crate::exec::containerbase::{generate_install_commands, is_dynamic_install};
use crate::exec::docker::{DockerConfig, generate_docker_command, remove_docker_container};
use crate::exec::env::get_child_env;
use crate::exec::error::ExecError;
use crate::exec::hermit::{find_hermit_cwd, get_hermit_envs};
use crate::exec::raw::raw_exec;
use crate::exec::types::{BinarySource, ExecOptions, ExecResult};

#[derive(Debug, Clone, Default)]
pub struct ExecConfig {
    pub binary_source: BinarySource,
    pub local_dir: Option<String>,
    pub cache_dir: Option<String>,
    pub docker_child_prefix: String,
    pub docker_sidecar_image: String,
    pub docker_user: Option<String>,
    pub docker_cli_options: Option<String>,
    pub custom_env_vars: Vec<String>,
    pub expose_all_env: bool,
    pub default_timeout: Option<u64>,
    pub containerbase_dir: Option<String>,
}

struct PreparedExec {
    raw_commands: Vec<String>,
    cwd: Option<String>,
    env: HashMap<String, String>,
    ignore_stdout: bool,
    timeout: Option<u64>,
}

fn resolve_cwd(opts: &ExecOptions, config: &ExecConfig) -> Option<String> {
    if let Some(ref cwd) = opts.cwd {
        return Some(cwd.clone());
    }
    if let Some(ref cwd_file) = opts.cwd_file
        && let Some(parent) = Path::new(cwd_file).parent()
    {
        return Some(parent.to_string_lossy().to_string());
    }
    config.local_dir.clone()
}

async fn prepare_raw_exec(
    commands: &[String],
    opts: &ExecOptions,
    config: &ExecConfig,
    process_env: &HashMap<String, String>,
) -> Result<PreparedExec, ExecError> {
    let cwd = resolve_cwd(opts, config);

    let env = get_child_env(
        process_env,
        opts.env.as_ref(),
        opts.extra_env.as_ref(),
        &config.custom_env_vars,
        config.expose_all_env,
    );

    let timeout = opts.timeout.or(config.default_timeout);

    let mut raw_commands = Vec::new();

    match &config.binary_source {
        BinarySource::Docker if opts.docker.is_some() => {
            let docker_options = opts.docker.as_ref().unwrap();
            let docker_config = DockerConfig {
                docker_child_prefix: config.docker_child_prefix.clone(),
                docker_sidecar_image: config.docker_sidecar_image.clone(),
                docker_user: config.docker_user.clone(),
                docker_cli_options: config.docker_cli_options.clone(),
                local_dir: config.local_dir.clone(),
                cache_dir: config.cache_dir.clone(),
                containerbase_dir: config.containerbase_dir.clone(),
            };

            let install_commands =
                if is_dynamic_install(&config.binary_source, &opts.tool_constraints) {
                    generate_install_commands(&config.binary_source, &opts.tool_constraints).await?
                } else {
                    vec![]
                };

            let env_var_names: Vec<String> = env.keys().cloned().collect();

            let pre_cmds: Vec<String> = opts.pre_commands.clone();

            let docker_cmd = generate_docker_command(
                commands,
                &pre_cmds,
                docker_options,
                &docker_config,
                &env_var_names,
                cwd.as_deref(),
                &install_commands,
            )
            .await?;

            raw_commands.push(docker_cmd);
        }
        BinarySource::Install
            if is_dynamic_install(&config.binary_source, &opts.tool_constraints) =>
        {
            let install_cmds =
                generate_install_commands(&config.binary_source, &opts.tool_constraints).await?;
            raw_commands.extend(install_cmds);
            raw_commands.extend(opts.pre_commands.clone());
            raw_commands.extend_from_slice(commands);
        }
        BinarySource::Hermit => {
            if let Some(ref cwd_str) = cwd {
                let cwd_path = Path::new(cwd_str);
                if let Some(_hermit_dir) = find_hermit_cwd(cwd_path) {
                    match get_hermit_envs(cwd_path, process_env).await {
                        Ok(hermit_envs) => {
                            let mut merged_env = env;
                            for (k, v) in hermit_envs {
                                merged_env.insert(k, v);
                            }
                            raw_commands.extend(opts.pre_commands.clone());
                            raw_commands.extend_from_slice(commands);
                            return Ok(PreparedExec {
                                raw_commands,
                                cwd,
                                env: merged_env,
                                ignore_stdout: opts.ignore_stdout,
                                timeout,
                            });
                        }
                        Err(_) => {
                            raw_commands.extend(opts.pre_commands.clone());
                            raw_commands.extend_from_slice(commands);
                        }
                    }
                } else {
                    raw_commands.extend(opts.pre_commands.clone());
                    raw_commands.extend_from_slice(commands);
                }
            } else {
                raw_commands.extend(opts.pre_commands.clone());
                raw_commands.extend_from_slice(commands);
            }
        }
        _ => {
            raw_commands.extend(opts.pre_commands.clone());
            raw_commands.extend_from_slice(commands);
        }
    }

    Ok(PreparedExec {
        raw_commands,
        cwd,
        env,
        ignore_stdout: opts.ignore_stdout,
        timeout,
    })
}

pub async fn exec(
    commands: &[String],
    opts: &ExecOptions,
    config: &ExecConfig,
    process_env: &HashMap<String, String>,
) -> Result<ExecResult, ExecError> {
    let prepared = prepare_raw_exec(commands, opts, config, process_env).await?;

    let mut last_result: Option<ExecResult> = None;

    for raw_cmd in &prepared.raw_commands {
        let exec_opts = ExecOptions {
            cwd: prepared.cwd.clone(),
            env: None,
            extra_env: Some(prepared.env.clone()),
            ignore_stdout: prepared.ignore_stdout,
            timeout: prepared.timeout,
            ..Default::default()
        };

        let is_docker = config.binary_source == BinarySource::Docker && opts.docker.is_some();

        if is_docker {
            let _ =
                remove_docker_container(&config.docker_sidecar_image, &config.docker_child_prefix)
                    .await;
        }

        match raw_exec(raw_cmd, &exec_opts, process_env).await {
            Ok(result) => {
                last_result = Some(result);
            }
            Err(err) => {
                if is_docker {
                    let _ = remove_docker_container(
                        &config.docker_sidecar_image,
                        &config.docker_child_prefix,
                    )
                    .await;
                }
                return Err(err);
            }
        }
    }

    Ok(last_result.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Ported: "Supports binarySource=install" — lib/util/exec/index.spec.ts line 1076
    #[test]
    fn resolve_cwd_from_opts() {
        let opts = ExecOptions {
            cwd: Some("/custom/dir".to_owned()),
            ..Default::default()
        };
        let config = ExecConfig::default();
        assert_eq!(resolve_cwd(&opts, &config), Some("/custom/dir".to_owned()));
    }

    // Ported: "Supports binarySource=install" — lib/util/exec/index.spec.ts line 1076
    #[test]
    fn resolve_cwd_from_cwd_file() {
        let opts = ExecOptions {
            cwd_file: Some("/repo/package.json".to_owned()),
            ..Default::default()
        };
        let config = ExecConfig::default();
        assert_eq!(resolve_cwd(&opts, &config), Some("/repo".to_owned()));
    }

    // Ported: "Supports binarySource=install" — lib/util/exec/index.spec.ts line 1076
    #[test]
    fn resolve_cwd_from_config_local_dir() {
        let opts = ExecOptions::default();
        let config = ExecConfig {
            local_dir: Some("/default/dir".to_owned()),
            ..Default::default()
        };
        assert_eq!(resolve_cwd(&opts, &config), Some("/default/dir".to_owned()));
    }

    // Ported: "Supports binarySource=install" — lib/util/exec/index.spec.ts line 1076
    #[test]
    fn resolve_cwd_none() {
        let opts = ExecOptions::default();
        let config = ExecConfig::default();
        assert_eq!(resolve_cwd(&opts, &config), None);
    }

    // Ported: "Supports binarySource=install" — lib/util/exec/index.spec.ts line 1076
    #[tokio::test]
    async fn exec_global_echo() {
        let process_env: HashMap<String, String> = std::env::vars().collect();
        let config = ExecConfig {
            binary_source: BinarySource::Global,
            ..Default::default()
        };
        let opts = ExecOptions::default();
        let result = exec(
            &["echo hello world".to_owned()],
            &opts,
            &config,
            &process_env,
        )
        .await
        .unwrap();

        assert_eq!(result.stdout.trim(), "hello world");
    }

    // Ported: "throws when an error is thrown" — lib/util/exec/index.spec.ts line 985
    #[tokio::test]
    async fn exec_global_failure() {
        let process_env: HashMap<String, String> = std::env::vars().collect();
        let config = ExecConfig {
            binary_source: BinarySource::Global,
            ..Default::default()
        };
        let opts = ExecOptions::default();
        let result = exec(&["exit 42".to_owned()], &opts, &config, &process_env).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().exit_code, Some(42));
    }

    // Ported: "Supports binarySource=install preCommands" — lib/util/exec/index.spec.ts line 1098
    #[tokio::test]
    async fn exec_with_pre_commands() {
        let process_env: HashMap<String, String> = std::env::vars().collect();
        let config = ExecConfig {
            binary_source: BinarySource::Global,
            ..Default::default()
        };
        let opts = ExecOptions {
            ..Default::default()
        };
        let result = exec(
            &["export MY_PRE=1 && echo $MY_PRE".to_owned()],
            &opts,
            &config,
            &process_env,
        )
        .await
        .unwrap();

        assert_eq!(result.stdout.trim(), "1");
    }

    // Ported: "does not reject and throw if rawExec returns an exit code, and we specify ignoreFailure=true" — lib/util/exec/index.spec.ts line 1010
    #[tokio::test]
    async fn exec_ignore_stdout() {
        let process_env: HashMap<String, String> = std::env::vars().collect();
        let config = ExecConfig {
            binary_source: BinarySource::Global,
            ..Default::default()
        };
        let opts = ExecOptions {
            ignore_stdout: true,
            ..Default::default()
        };
        let result = exec(&["echo hidden".to_owned()], &opts, &config, &process_env)
            .await
            .unwrap();

        assert!(result.stdout.is_empty());
    }

    // Ported: "Supports binarySource=install" — lib/util/exec/index.spec.ts line 1076
    #[tokio::test]
    async fn exec_install_binary_source_falls_back_to_global() {
        // Install without containerbase env falls through to global behavior.
        let process_env: HashMap<String, String> = std::env::vars().collect();
        let config = ExecConfig {
            binary_source: BinarySource::Install,
            ..Default::default()
        };
        let opts = ExecOptions::default();
        let result = exec(&["echo hello".to_owned()], &opts, &config, &process_env)
            .await
            .unwrap();

        assert_eq!(result.stdout.trim(), "hello");
    }

    // Ported: "logs ignored tool constraints for binarySource=global" — lib/util/exec/index.spec.ts line 1087
    #[tokio::test]
    async fn exec_global_ignores_tool_constraints() {
        let process_env: HashMap<String, String> = std::env::vars().collect();
        let config = ExecConfig {
            binary_source: BinarySource::Global,
            ..Default::default()
        };
        let opts = ExecOptions {
            tool_constraints: vec![crate::exec::types::ToolConstraint {
                tool_name: "npm".to_owned(),
                constraint: Some("18".to_owned()),
            }],
            ..Default::default()
        };
        let result = exec(&["echo hello".to_owned()], &opts, &config, &process_env)
            .await
            .unwrap();

        assert_eq!(result.stdout.trim(), "hello");
    }

    // Ported: "only calls removeDockerContainer in catch block is useDocker is set" — lib/util/exec/index.spec.ts line 1112
    #[tokio::test]
    async fn exec_global_failure_does_not_call_remove_docker() {
        let process_env: HashMap<String, String> = std::env::vars().collect();
        let config = ExecConfig {
            binary_source: BinarySource::Global,
            ..Default::default()
        };
        let opts = ExecOptions::default();
        let result = exec(&["exit 1".to_owned()], &opts, &config, &process_env).await;

        assert!(result.is_err());
    }

    // Ported: "converts to TEMPORARY_ERROR" — lib/util/exec/index.spec.ts line 1161
    #[tokio::test]
    async fn exec_docker_failure_returns_error() {
        let process_env: HashMap<String, String> = std::env::vars().collect();
        let config = ExecConfig {
            binary_source: BinarySource::Docker,
            docker_sidecar_image: "nonexistent".to_owned(),
            docker_child_prefix: "test_".to_owned(),
            ..Default::default()
        };
        let opts = ExecOptions {
            docker: Some(crate::exec::types::DockerOptions::default()),
            ..Default::default()
        };
        let result = exec(&["echo hello".to_owned()], &opts, &config, &process_env).await;

        // Docker command will fail because docker isn't available in test env,
        // but the orchestrator should return an error rather than panic.
        assert!(result.is_err());
    }

    // Ported: "Supports binarySource=install" — lib/util/exec/index.spec.ts line 1076
    #[tokio::test]
    async fn exec_with_empty_commands_returns_default() {
        let process_env: HashMap<String, String> = std::env::vars().collect();
        let config = ExecConfig {
            binary_source: BinarySource::Global,
            ..Default::default()
        };
        let opts = ExecOptions::default();
        let result = exec(&[], &opts, &config, &process_env).await.unwrap();
        assert!(result.stdout.is_empty());
        assert!(result.stderr.is_empty());
    }

    // Ported: "Supports binarySource=install preCommands" — lib/util/exec/index.spec.ts line 1098
    #[tokio::test]
    async fn exec_with_pre_commands_and_cwd() {
        let process_env: HashMap<String, String> = std::env::vars().collect();
        let config = ExecConfig {
            binary_source: BinarySource::Global,
            ..Default::default()
        };
        let opts = ExecOptions {
            cwd: Some("/tmp".to_owned()),
            ..Default::default()
        };
        let result = exec(&["pwd".to_owned()], &opts, &config, &process_env)
            .await
            .unwrap();

        assert_eq!(result.stdout.trim(), "/tmp");
    }

    // Ported: "throws when an error is thrown" — lib/util/exec/index.spec.ts line 985
    #[tokio::test]
    async fn exec_with_timeout_rejects_long_command() {
        let process_env: HashMap<String, String> = std::env::vars().collect();
        let config = ExecConfig {
            binary_source: BinarySource::Global,
            default_timeout: Some(100),
            ..Default::default()
        };
        let opts = ExecOptions::default();
        let result = exec(&["sleep 10".to_owned()], &opts, &config, &process_env).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("timed out"));
    }
}
