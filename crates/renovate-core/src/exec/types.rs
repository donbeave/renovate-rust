use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ToolConstraint {
    pub tool_name: String,
    pub constraint: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct DockerOptions {
    pub volumes: Vec<String>,
    pub env_vars: Vec<String>,
    pub cwd: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CommandWithOptions {
    pub command: Vec<String>,
    pub ignore_failure: bool,
    pub shell: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ExecOptions {
    pub cwd: Option<String>,
    pub cwd_file: Option<String>,
    pub env: Option<HashMap<String, String>>,
    pub extra_env: Option<HashMap<String, String>>,
    pub docker: Option<DockerOptions>,
    pub tool_constraints: Vec<ToolConstraint>,
    pub pre_commands: Vec<String>,
    pub ignore_stdout: bool,
    pub max_buffer: Option<usize>,
    pub timeout: Option<u64>,
    pub shell: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ExecResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum BinarySource {
    #[default]
    Global,
    Docker,
    Hermit,
    Containerbase,
    Install,
}
