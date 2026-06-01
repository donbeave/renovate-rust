pub mod author;
pub mod storage;

pub use storage::GitStorage;

// ---------------------------------------------------------------------------
// git/config — lib/util/git/config.ts
// ---------------------------------------------------------------------------

/// Configuration for git's completion events.
#[derive(Debug, Clone, PartialEq)]
pub struct GitCompletionConfig {
    pub on_close: bool,
    pub on_exit: bool,
}

/// Unsafe git options.
#[derive(Debug, Clone, PartialEq)]
pub struct GitUnsafeConfig {
    pub allow_unsafe_ssh_command: bool,
    pub allow_unsafe_config_env_count: bool,
}

/// Configuration for git operations.
///
/// Mirrors `simpleGitConfig()` from `lib/util/git/config.ts`.
#[derive(Debug, Clone)]
pub struct SimpleGitConfig {
    pub completion: GitCompletionConfig,
    pub config: Vec<String>,
    pub unsafe_opts: GitUnsafeConfig,
    pub timeout_block: Option<u64>,
}

/// Build the default git config, optionally with a timeout.
///
/// Mirrors `simpleGitConfig()` from `lib/util/git/config.ts`.
pub fn simple_git_config(git_timeout: Option<u64>) -> SimpleGitConfig {
    SimpleGitConfig {
        completion: GitCompletionConfig {
            on_close: true,
            on_exit: false,
        },
        config: vec!["core.quotePath=false".to_owned()],
        unsafe_opts: GitUnsafeConfig {
            allow_unsafe_ssh_command: true,
            allow_unsafe_config_env_count: true,
        },
        timeout_block: git_timeout.filter(|&t| t > 0),
    }
}

/// Validate that `value` is an array of strings (for `gitNoVerify` config).
///
/// Returns an error message if invalid, mirrors `setNoVerify()` from
/// `lib/util/git/config.ts`.
pub fn validate_git_no_verify(value: &serde_json::Value) -> Result<Vec<String>, String> {
    match value {
        serde_json::Value::Array(arr) => {
            let mut result = Vec::new();
            for item in arr {
                match item.as_str() {
                    Some(s) => result.push(s.to_owned()),
                    None => {
                        return Err(
                            "config error: gitNoVerify should be an array of strings".to_owned()
                        );
                    }
                }
            }
            Ok(result)
        }
        _ => Err("config error: gitNoVerify should be an array of strings".to_owned()),
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;
    use serde_json::json;

    // Ported: "uses \"close\" events, ignores \"exit\" events from child processes" — util/git/config.spec.ts line 9
    #[test]
    fn simple_git_config_defaults() {
        let cfg = simple_git_config(None);
        assert!(cfg.completion.on_close);
        assert!(!cfg.completion.on_exit);
        assert_eq!(cfg.config, vec!["core.quotePath=false"]);
        assert!(cfg.unsafe_opts.allow_unsafe_ssh_command);
        assert!(cfg.unsafe_opts.allow_unsafe_config_env_count);
        assert!(cfg.timeout_block.is_none());
    }

    // Ported: "uses timeout value from GlobalConfig" — util/git/config.spec.ts line 20
    #[test]
    fn simple_git_config_uses_timeout() {
        let cfg = simple_git_config(Some(50000));
        assert_eq!(cfg.timeout_block, Some(50000));
        // Other fields are the same as defaults
        assert!(cfg.completion.on_close);
        assert!(!cfg.completion.on_exit);
    }

    // Ported: "throws" — util/git/config.spec.ts line 35
    #[test]
    fn set_no_verify_rejects_non_array() {
        let result = validate_git_no_verify(&json!(1));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "config error: gitNoVerify should be an array of strings"
        );
    }
}
