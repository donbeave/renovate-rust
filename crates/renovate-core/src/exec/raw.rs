use std::collections::HashMap;
use std::time::Duration;

use crate::exec::env::get_child_env;
use crate::exec::error::ExecError;
use crate::exec::types::ExecOptions;
use crate::exec::types::ExecResult;

pub async fn raw_exec(
    cmd: &str,
    opts: &ExecOptions,
    process_env: &HashMap<String, String>,
) -> Result<ExecResult, ExecError> {
    let env = get_child_env(
        process_env,
        opts.env.as_ref(),
        opts.extra_env.as_ref(),
        &[],
        false,
    );

    let shell = opts.shell.as_deref().unwrap_or("sh");
    let mut cmd_builder = tokio::process::Command::new(shell);
    cmd_builder.arg("-c").arg(cmd).env_clear();

    for (key, val) in &env {
        cmd_builder.env(key, val);
    }

    if let Some(ref cwd) = opts.cwd {
        cmd_builder.current_dir(cwd);
    }

    if let Some(timeout) = opts.timeout {
        cmd_builder
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        let output = tokio::time::timeout(
            Duration::from_millis(timeout),
            cmd_builder.output(),
        )
        .await
        .map_err(|_| {
            ExecError::new(
                format!("command timed out after {}ms", timeout),
                cmd,
            )
        })?
        .map_err(|e| ExecError::new(format!("failed to execute: {}", e), cmd).with_source(Box::new(e)))?;

        let exit_code = output.status.code();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            return Err(
                ExecError::new(format!("command failed with status {}", output.status), cmd)
                    .with_output(stdout, stderr, exit_code),
            );
        }

        Ok(ExecResult {
            stdout,
            stderr,
            exit_code,
        })
    } else if opts.ignore_stdout {
        cmd_builder
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped());

        let output = cmd_builder.output().await.map_err(|e| {
            ExecError::new(format!("failed to execute: {}", e), cmd).with_source(Box::new(e))
        })?;

        let exit_code = output.status.code();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            return Err(
                ExecError::new(format!("command failed with status {}", output.status), cmd)
                    .with_output(String::new(), stderr, exit_code),
            );
        }

        Ok(ExecResult {
            stdout: String::new(),
            stderr,
            exit_code,
        })
    } else {
        cmd_builder
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        let output = cmd_builder.output().await.map_err(|e| {
            ExecError::new(format!("failed to execute: {}", e), cmd).with_source(Box::new(e))
        })?;

        let exit_code = output.status.code();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            return Err(
                ExecError::new(format!("command failed with status {}", output.status), cmd)
                    .with_output(stdout, stderr, exit_code),
            );
        }

        Ok(ExecResult {
            stdout,
            stderr,
            exit_code,
        })
    }
}

pub fn as_raw_command(cmd: &str) -> String {
    cmd.to_owned()
}

pub fn as_raw_commands(cmds: &[String]) -> Vec<String> {
    cmds.iter().map(|c| as_raw_command(c)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Rust-specific: raw behavior test
    #[test]
    fn as_raw_command_returns_input() {
        assert_eq!(as_raw_command("echo hello"), "echo hello");
    }

    // Rust-specific: raw behavior test
    #[test]
    fn as_raw_commands_maps_all() {
        let cmds = vec!["echo a".to_owned(), "echo b".to_owned()];
        let result = as_raw_commands(&cmds);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "echo a");
        assert_eq!(result[1], "echo b");
    }

    #[tokio::test]
    async fn raw_exec_echo() {
        let env = std::env::vars().collect();
        let opts = ExecOptions::default();
        let result = raw_exec("echo hello", &opts, &env).await.unwrap();
        assert_eq!(result.stdout.trim(), "hello");
        assert!(result.exit_code.unwrap() == 0);
    }

    #[tokio::test]
    async fn raw_exec_failure() {
        let env = std::env::vars().collect();
        let opts = ExecOptions::default();
        let result = raw_exec("exit 1", &opts, &env).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.exit_code, Some(1));
    }

    #[tokio::test]
    async fn raw_exec_with_extra_env() {
        let env = std::env::vars().collect::<HashMap<String, String>>();
        let opts = ExecOptions {
            extra_env: Some({
                let mut m = HashMap::new();
                m.insert("MY_TEST_VAR".to_owned(), "test_value".to_owned());
                m
            }),
            ..Default::default()
        };
        let result = raw_exec("echo $MY_TEST_VAR", &opts, &env).await.unwrap();
        assert_eq!(result.stdout.trim(), "test_value");
    }

    #[tokio::test]
    async fn raw_exec_with_cwd() {
        let env = std::env::vars().collect();
        let opts = ExecOptions {
            cwd: Some("/tmp".to_owned()),
            ..Default::default()
        };
        let result = raw_exec("pwd", &opts, &env).await.unwrap();
        assert_eq!(result.stdout.trim(), "/tmp");
    }

    #[tokio::test]
    async fn raw_exec_timeout() {
        let env = std::env::vars().collect();
        let opts = ExecOptions {
            timeout: Some(100),
            ..Default::default()
        };
        let result = raw_exec("sleep 10", &opts, &env).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("timed out"));
    }
}
