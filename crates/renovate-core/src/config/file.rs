//! Global config file discovery and loading.
//!
//! Renovate reference: `lib/workers/global/config/parse/file.ts`.
//! @parity lib/workers/global/config/parse/file.ts partial — JSON/JSON5 parser and non-default file cleanup are implemented; CLI/global env integration and some migrate/validation flows are staged elsewhere.
//! @parity lib/workers/global/config/parse/additional-config-file.ts partial — parse-and-load support for `RENOVATE_ADDITIONAL_CONFIG_FILE` is implemented, including `processEnv` export and optional post-load deletion, but JS/yaml configs remain unsupported.
//!
//! ## Supported formats
//!
//! | Extension | Parser | Notes |
//! |---|---|---|
//! | `.json` | `serde_json` | Standard JSON |
//! | `.json5` | `json5` | JSON5 superset (comments, trailing commas) |
//! | `.js` / `.cjs` / `.mjs` | — | **Not supported** — JS execution is out of scope |
//!
//! YAML (`.yaml`, `.yml`) support is deferred pending a stable, maintained
//! `serde_yaml` successor; see compatibility-decisions.md CD-0003.
//!
//! ## Discovery order
//!
//! If `RENOVATE_CONFIG_FILE` is set, that exact path is used (error if absent).
//! Otherwise no global config file is loaded; Renovate's JS default of
//! `config.js` is intentionally not searched (CD-0003).

use std::path::{Path, PathBuf};
use std::env;

use super::GlobalConfig;

/// Errors that can occur while loading the global config file.
#[derive(Debug, thiserror::Error)]
pub enum ConfigFileError {
    /// The path specified in `RENOVATE_CONFIG_FILE` does not exist.
    #[error("RENOVATE_CONFIG_FILE path does not exist: {0}")]
    ExplicitPathNotFound(PathBuf),

    /// The file extension is not supported by the Rust implementation.
    #[error("Unsupported config file format: {0}")]
    UnsupportedFormat(String),

    /// IO error reading the file.
    #[error("Failed to read config file {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Parse error in the config file contents.
    #[error("Failed to parse config file {path}: {message}")]
    Parse { path: PathBuf, message: String },
}

/// Parsed config-file contents or a Renovate-compatible validation error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedFileConfig {
    /// File parsed successfully.
    Success { parsed_contents: serde_json::Value },
    /// File parsing failed.
    Error {
        validation_error: String,
        validation_message: String,
    },
}

/// Resolve which config file path to load, if any.
///
/// Returns `Some(path)` when a file should be loaded, `None` when there is
/// no global config to load.
///
/// Mirrors `file.ts` `getConfig`: if `RENOVATE_CONFIG_FILE` is set the path
/// must exist; otherwise no default is searched (Renovate's `config.js`
/// default is JS-only and not applicable here — see CD-0003).
pub fn resolve_config_path(
    config_file_env: Option<&str>,
    cwd: &Path,
) -> Result<Option<PathBuf>, ConfigFileError> {
    let Some(explicit) = config_file_env.filter(|value| !value.trim().is_empty()) else {
        return Ok(None);
    };

    let path = if Path::new(explicit).is_absolute() {
        PathBuf::from(explicit)
    } else {
        cwd.join(explicit)
    };

    if !path.exists() {
        return Err(ConfigFileError::ExplicitPathNotFound(path));
    }

    Ok(Some(path))
}

/// Delete a non-default global config file after it has been loaded.
///
/// Mirrors `deleteNonDefaultConfig()` in Renovate's global config parser:
/// skip when no explicit config path is set, skip when deletion is disabled,
/// skip when the path does not exist, and ignore deletion failures.
pub fn delete_non_default_config(config_file_env: Option<&str>, delete_config_file: bool) -> bool {
    if !delete_config_file {
        return false;
    }

    let Some(config_file) = config_file_env.filter(|value| !value.trim().is_empty()) else {
        return false;
    };

    let path = Path::new(config_file);
    if !path.exists() {
        return false;
    }

    let result = if path.is_dir() {
        std::fs::remove_dir_all(path)
    } else {
        std::fs::remove_file(path)
    };

    result.is_ok()
}

/// Load and parse `RENOVATE_ADDITIONAL_CONFIG_FILE`, including `processEnv`
/// side effects. Mirrors the intent of
/// `lib/workers/global/config/parse/additional-config-file.ts`:
/// - return defaults when unset
/// - fail when the path is missing/invalid
/// - apply process environment variable overrides from `processEnv`
/// - merge into the already-loaded base config with file-style precedence
pub fn load_additional_config(config_file_env: Option<&str>, cwd: &Path) -> Result<GlobalConfig, ConfigFileError> {
    let Some(config_file) = resolve_config_path(config_file_env, cwd)? else {
        return Ok(GlobalConfig::default());
    };

    let file_name = config_file
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    let contents = std::fs::read_to_string(&config_file).map_err(|source| ConfigFileError::Io {
        path: config_file.to_owned(),
        source,
    })?;

    let parsed = parse_file_config(file_name, &contents);
    let mut parsed_contents = match parsed {
        ParsedFileConfig::Success { parsed_contents, .. } => parsed_contents,
        ParsedFileConfig::Error {
            validation_message, ..
        } => {
            return Err(ConfigFileError::Parse {
                path: config_file,
                message: validation_message,
            });
        }
    };

    if let Some(process_env) = parsed_contents
        .get_mut("processEnv")
        .and_then(serde_json::Value::as_object_mut)
    {
        for (key, value) in process_env {
            if let Some(processed) = value.as_str() {
                let _ = env::set_var(key, processed);
            }
        }
        parsed_contents
            .as_object_mut()
            .and_then(|object| object.remove("processEnv"));
    }

    serde_json::from_value(parsed_contents).map_err(|source| ConfigFileError::Parse {
        path: config_file.to_owned(),
        message: source.to_string(),
    })
}

/// Keep behavior parity with `deleteNonDefaultConfig` in
/// `additional-config-file.ts`.
pub fn delete_non_default_additional_config(
    config_file_env: Option<&str>,
    delete_config_file: bool,
) -> bool {
    delete_non_default_config(config_file_env, delete_config_file)
}

/// Load and parse a global config file into a [`GlobalConfig`].
///
/// Supports JSON (`.json`) and JSON5 (`.json5`, `.renovaterc` with no
/// extension is treated as JSON per Renovate's `getParsedContent`).
pub fn load(path: &Path) -> Result<GlobalConfig, ConfigFileError> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    let basename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();

    // .js/.cjs/.mjs: not supported; emit a clear error.
    if matches!(ext.as_str(), "js" | "cjs" | "mjs") {
        return Err(ConfigFileError::UnsupportedFormat(ext));
    }

    let contents = std::fs::read_to_string(path).map_err(|source| ConfigFileError::Io {
        path: path.to_owned(),
        source,
    })?;

    if ext.is_empty() && basename != ".renovaterc" {
        return Err(ConfigFileError::UnsupportedFormat("".to_owned()));
    }

    // .renovaterc (no extension) and .json -> standard JSON via serde_json.
    // .json5 → JSON5 via json5 crate.
    let config: GlobalConfig = match ext.as_str() {
        "json5" => json5::from_str(&contents).map_err(|e| ConfigFileError::Parse {
            path: path.to_owned(),
            message: e.to_string(),
        })?,
        "json" | "" => {
            // .renovaterc with no extension is JSON (upstream behavior).
            let _ = basename; // used implicitly via the "" arm
            serde_json::from_str(&contents).map_err(|e| ConfigFileError::Parse {
                path: path.to_owned(),
                message: e.to_string(),
            })?
        }
        other => return Err(ConfigFileError::UnsupportedFormat(other.to_owned())),
    };

    Ok(config)
}

/// Parse config file contents without loading from disk.
///
/// Mirrors `parseFileConfig()` in `lib/config/parse.ts` for JSON and JSON5
/// config files.
pub fn parse_file_config(file_name: &str, file_contents: &str) -> ParsedFileConfig {
    if file_name.ends_with(".json5") {
        return match json5::from_str(file_contents) {
            Ok(parsed_contents) => ParsedFileConfig::Success { parsed_contents },
            Err(err) => ParsedFileConfig::Error {
                validation_error: "Invalid JSON5 (parsing failed)".to_owned(),
                validation_message: format!(
                    "JSON5.parse error: `{}`",
                    err.to_string().replace('`', "'")
                ),
            },
        };
    }

    match serde_json::from_str(file_contents) {
        Ok(parsed_contents) => ParsedFileConfig::Success { parsed_contents },
        Err(err) => ParsedFileConfig::Error {
            validation_error: "Invalid JSON (parsing failed)".to_owned(),
            validation_message: err.to_string(),
        },
    }
}

/// Apply a file-loaded config on top of the default config.
///
/// Fields present in `file_config` (i.e. non-default deserialized values)
/// override `base`. CLI config is applied on top of this by the caller.
///
/// This is a simple field-by-field merge; `Option<T>` fields from the file
/// override `None` in base but never replace a `Some` that was already set.
pub fn merge_over_base(base: GlobalConfig, file_config: GlobalConfig) -> GlobalConfig {
    GlobalConfig {
        // For Option fields: file wins only if it's Some.
        token: file_config.token.or(base.token),
        endpoint: file_config.endpoint.or(base.endpoint),
        username: file_config.username.or(base.username),
        password: file_config.password.or(base.password),
        git_private_key: file_config.git_private_key.or(base.git_private_key),
        git_private_key_passphrase: file_config
            .git_private_key_passphrase
            .or(base.git_private_key_passphrase),
        user_agent: file_config.user_agent.or(base.user_agent),
        base_dir: file_config.base_dir.or(base.base_dir),
        local_dir: file_config.local_dir.or(base.local_dir),
        cache_dir: file_config.cache_dir.or(base.cache_dir),
        containerbase_dir: file_config.containerbase_dir.or(base.containerbase_dir),
        docker_child_prefix: file_config.docker_child_prefix.or(base.docker_child_prefix),
        docker_cli_options: file_config.docker_cli_options.or(base.docker_cli_options),
        docker_sidecar_image: file_config
            .docker_sidecar_image
            .or(base.docker_sidecar_image),
        docker_user: file_config.docker_user.or(base.docker_user),
        enabled: file_config.enabled.or(base.enabled),
        automerge: file_config.automerge.or(base.automerge),
        dependency_dashboard: file_config
            .dependency_dashboard
            .or(base.dependency_dashboard),
        dependency_dashboard_approval: file_config
            .dependency_dashboard_approval
            .or(base.dependency_dashboard_approval),
        dependency_dashboard_autoclose: file_config
            .dependency_dashboard_autoclose
            .or(base.dependency_dashboard_autoclose),
        dependency_dashboard_title: file_config
            .dependency_dashboard_title
            .or(base.dependency_dashboard_title),
        dependency_dashboard_category: file_config
            .dependency_dashboard_category
            .or(base.dependency_dashboard_category),
        dependency_dashboard_header: file_config
            .dependency_dashboard_header
            .or(base.dependency_dashboard_header),
        dependency_dashboard_footer: file_config
            .dependency_dashboard_footer
            .or(base.dependency_dashboard_footer),
        dependency_dashboard_labels: file_config
            .dependency_dashboard_labels
            .or(base.dependency_dashboard_labels),
        config_warning_reuse_issue: file_config
            .config_warning_reuse_issue
            .or(base.config_warning_reuse_issue),
        dry_run: file_config.dry_run.or(base.dry_run),
        mode: file_config.mode.or(base.mode),
        binary_source: file_config.binary_source.or(base.binary_source),
        // For non-Option fields: file always wins (it was explicitly set in
        // the file or it carries the default — we cannot distinguish, so we
        // always take the file's value, then let CLI override afterwards).
        platform: file_config.platform,
        require_config: file_config.require_config,
        fork_processing: file_config.fork_processing,
        fork_creation: file_config.fork_creation.or(base.fork_creation),
        fork_token: file_config.fork_token.or(base.fork_token),
        fork_org: file_config.fork_org.or(base.fork_org),
        config_migration: file_config.config_migration,
        print_config: file_config.print_config.or(base.print_config),
        onboarding: file_config.onboarding.or(base.onboarding),
        onboarding_branch: file_config.onboarding_branch.or(base.onboarding_branch),
        onboarding_auto_close_age: file_config
            .onboarding_auto_close_age
            .or(base.onboarding_auto_close_age),
        onboarding_commit_message: file_config
            .onboarding_commit_message
            .or(base.onboarding_commit_message),
        config_file_names: file_config.config_file_names.or(base.config_file_names),
        migrate_presets: if file_config.migrate_presets.is_empty() {
            base.migrate_presets
        } else {
            file_config.migrate_presets
        },
        custom_env_variables: if file_config.custom_env_variables.is_empty() {
            base.custom_env_variables
        } else {
            file_config.custom_env_variables
        },
        cache_ttl_override: if file_config.cache_ttl_override.is_empty() {
            base.cache_ttl_override
        } else {
            file_config.cache_ttl_override
        },
        tool_settings: if file_config.tool_settings.is_empty() {
            base.tool_settings
        } else {
            file_config.tool_settings
        },
        onboarding_config_file_name: file_config
            .onboarding_config_file_name
            .or(base.onboarding_config_file_name),
        onboarding_no_deps: file_config.onboarding_no_deps.or(base.onboarding_no_deps),
        onboarding_pr_title: file_config.onboarding_pr_title.or(base.onboarding_pr_title),
        onboarding_rebase_checkbox: file_config
            .onboarding_rebase_checkbox
            .or(base.onboarding_rebase_checkbox),
        pr_commits_per_run_limit: file_config
            .pr_commits_per_run_limit
            .or(base.pr_commits_per_run_limit),
        platform_automerge: file_config.platform_automerge,
        platform_commit: file_config.platform_commit.or(base.platform_commit),
        recreate_when: file_config.recreate_when,
        allowed_commands: if file_config.allowed_commands.is_empty() {
            base.allowed_commands
        } else {
            file_config.allowed_commands
        },
        allow_command_templating: file_config.allow_command_templating,
        allow_plugins: file_config.allow_plugins.or(base.allow_plugins),
        allow_scripts: file_config.allow_scripts.or(base.allow_scripts),
        allow_shell_executor_for_post_upgrade_commands: file_config
            .allow_shell_executor_for_post_upgrade_commands
            .or(base.allow_shell_executor_for_post_upgrade_commands),
        optimize_for_disabled: file_config.optimize_for_disabled,
        allow_custom_crate_registries: file_config
            .allow_custom_crate_registries
            .or(base.allow_custom_crate_registries),
        allowed_headers: file_config.allowed_headers.or(base.allowed_headers),
        allowed_env: file_config.allowed_env.or(base.allowed_env),
        allowed_unsafe_executions: file_config
            .allowed_unsafe_executions
            .or(base.allowed_unsafe_executions),
        expose_all_env: file_config.expose_all_env.or(base.expose_all_env),
        detect_global_manager_config: file_config
            .detect_global_manager_config
            .or(base.detect_global_manager_config),
        detect_host_rules_from_env: file_config
            .detect_host_rules_from_env
            .or(base.detect_host_rules_from_env),
        merge_confidence_endpoint: file_config
            .merge_confidence_endpoint
            .or(base.merge_confidence_endpoint),
        merge_confidence_datasources: if file_config.merge_confidence_datasources.is_empty() {
            base.merge_confidence_datasources
        } else {
            file_config.merge_confidence_datasources
        },
        autodiscover_repo_sort: file_config
            .autodiscover_repo_sort
            .or(base.autodiscover_repo_sort),
        autodiscover_repo_order: file_config
            .autodiscover_repo_order
            .or(base.autodiscover_repo_order),
        autodiscover: file_config.autodiscover.or(base.autodiscover),
        autodiscover_filter: file_config.autodiscover_filter.or(base.autodiscover_filter),
        autodiscover_namespaces: file_config
            .autodiscover_namespaces
            .or(base.autodiscover_namespaces),
        autodiscover_projects: file_config
            .autodiscover_projects
            .or(base.autodiscover_projects),
        autodiscover_topics: file_config.autodiscover_topics.or(base.autodiscover_topics),
        docker_max_pages: file_config.docker_max_pages.or(base.docker_max_pages),
        delete_config_file: file_config.delete_config_file,
        delete_additional_config_file: file_config.delete_additional_config_file,
        config_validation_error: file_config.config_validation_error,
        checked_branches: if file_config.checked_branches.is_empty() {
            base.checked_branches
        } else {
            file_config.checked_branches
        },
        git_no_verify: if file_config.git_no_verify.is_empty() {
            base.git_no_verify
        } else {
            file_config.git_no_verify
        },
        write_discovered_repos: file_config
            .write_discovered_repos
            .or(base.write_discovered_repos),
        s3_endpoint: file_config.s3_endpoint.or(base.s3_endpoint),
        s3_path_style: file_config.s3_path_style,
        repository_cache_force_local: file_config
            .repository_cache_force_local
            .or(base.repository_cache_force_local),
        repository_cache: file_config.repository_cache.or(base.repository_cache),
        repository_cache_type: file_config
            .repository_cache_type
            .or(base.repository_cache_type),
        execution_timeout: file_config.execution_timeout.or(base.execution_timeout),
        git_timeout: file_config.git_timeout.or(base.git_timeout),
        git_url: file_config.git_url,
        http_cache_ttl_days: file_config.http_cache_ttl_days.or(base.http_cache_ttl_days),
        cache_hard_ttl_minutes: file_config
            .cache_hard_ttl_minutes
            .or(base.cache_hard_ttl_minutes),
        cache_private_packages: file_config
            .cache_private_packages
            .or(base.cache_private_packages),
        preset_cache_persistence: file_config
            .preset_cache_persistence
            .or(base.preset_cache_persistence),
        include_mirrors: file_config.include_mirrors.or(base.include_mirrors),
        github_token_warn: file_config.github_token_warn.or(base.github_token_warn),
        encrypted_warning: file_config.encrypted_warning.or(base.encrypted_warning),
        ignore_pr_author: file_config.ignore_pr_author.or(base.ignore_pr_author),
        bb_use_development_branch: file_config
            .bb_use_development_branch
            .or(base.bb_use_development_branch),
        pr_cache_sync_max_pages: file_config
            .pr_cache_sync_max_pages
            .or(base.pr_cache_sync_max_pages),
        report_type: file_config.report_type.or(base.report_type),
        report_path: file_config.report_path.or(base.report_path),
        report_formatting: file_config.report_formatting.or(base.report_formatting),
        unicode_emoji: file_config.unicode_emoji.or(base.unicode_emoji),
        labels: if file_config.labels.is_empty() {
            base.labels
        } else {
            file_config.labels
        },
        host_rules: if file_config.host_rules.is_empty() {
            base.host_rules
        } else {
            file_config.host_rules
        },
        registry_aliases: if file_config.registry_aliases.is_empty() {
            base.registry_aliases
        } else {
            file_config.registry_aliases
        },
        onboarding_config: if file_config.onboarding_config.is_empty() {
            base.onboarding_config
        } else {
            file_config.onboarding_config
        },
        lock_file_maintenance: if file_config.lock_file_maintenance.is_empty() {
            base.lock_file_maintenance
        } else {
            file_config.lock_file_maintenance
        },
        // repositories are CLI-only; the file config never sets them.
        repositories: base.repositories,
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write as _;

    use super::*;
    use crate::config::{DryRun, Platform, RequireConfig};

    fn write_temp(contents: &str, ext: &str) -> (tempfile::NamedTempFile, PathBuf) {
        let mut f = tempfile::Builder::new()
            .suffix(ext)
            .tempfile()
            .expect("tempfile");
        f.write_all(contents.as_bytes()).expect("write");
        let path = f.path().to_owned();
        (f, path)
    }

    // ── resolve_config_path ──────────────────────────────────────────────────

    // Ported: "parse and returns empty config if there is no RENOVATE_CONFIG_FILE in env" — lib/workers/global/config/parse/file.spec.ts line 85
    #[test]
    fn resolve_returns_none_when_env_not_set() {
        let dir = tempfile::tempdir().unwrap();
        assert!(resolve_config_path(None, dir.path()).unwrap().is_none());
    }

    // Ported: "skip when RENOVATE_CONFIG_FILE is not set (\"%s\")" — lib/workers/global/config/parse/file.spec.ts line 220
    #[test]
    fn resolve_returns_none_when_env_is_blank() {
        let dir = tempfile::tempdir().unwrap();
        assert!(
            resolve_config_path(Some(" "), dir.path())
                .unwrap()
                .is_none()
        );
    }

    // Rust-specific: unit test for resolve_config_path when file exists
    #[test]
    fn resolve_returns_path_when_file_exists() {
        let (f, path) = write_temp("{}", ".json");
        let dir = f.path().parent().unwrap().to_owned();
        let filename = path.file_name().unwrap().to_str().unwrap().to_owned();
        let resolved = resolve_config_path(Some(&filename), &dir).unwrap();
        assert_eq!(resolved, Some(path));
    }

    // Ported: "fatal error and exit if custom config file does not exist" — lib/workers/global/config/parse/file.spec.ts line 118
    #[test]
    fn resolve_errors_when_explicit_file_missing() {
        let dir = tempfile::tempdir().unwrap();
        let err = resolve_config_path(Some("no_such_file.json"), dir.path()).unwrap_err();
        assert!(matches!(err, ConfigFileError::ExplicitPathNotFound(_)));
    }

    // ── delete_non_default_config ───────────────────────────────────────────

    // Ported: "skip when RENOVATE_CONFIG_FILE is not set (\"%s\")" — lib/workers/global/config/parse/file.spec.ts line 220
    #[test]
    fn delete_non_default_config_skips_when_env_not_set() {
        assert!(!delete_non_default_config(None, true));
        assert!(!delete_non_default_config(Some(" "), true));
    }

    // Ported: "skip when config file does not exist" — lib/workers/global/config/parse/file.spec.ts line 232
    #[test]
    fn delete_non_default_config_skips_missing_file() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("missing.json");
        assert!(!delete_non_default_config(missing.to_str(), true));
    }

    // Ported: "skip if deleteConfigFile is not set (\"%s\")" — lib/workers/global/config/parse/file.spec.ts line 245
    #[test]
    fn delete_non_default_config_skips_when_flag_is_false() {
        let (_f, path) = write_temp("{}", ".json");
        assert!(!delete_non_default_config(path.to_str(), false));
        assert!(path.exists());
    }

    // Ported: "removes the specified config file" — lib/workers/global/config/parse/file.spec.ts line 261
    #[test]
    fn delete_non_default_config_removes_file() {
        let (_f, path) = write_temp("{}", ".json");
        assert!(delete_non_default_config(path.to_str(), true));
        assert!(!path.exists());
    }

    // Ported: "fails silently when attempting to delete the config file" — lib/workers/global/config/parse/file.spec.ts line 284
    #[cfg(unix)]
    #[test]
    fn delete_non_default_config_fails_silently() {
        use std::os::unix::fs::PermissionsExt as _;

        let dir = tempfile::tempdir().unwrap();
        let parent = dir.path().join("parent");
        std::fs::create_dir(&parent).unwrap();
        let file = parent.join("config.json");
        std::fs::write(&file, "{}").unwrap();

        let original_permissions = std::fs::metadata(&parent).unwrap().permissions();
        std::fs::set_permissions(&parent, std::fs::Permissions::from_mode(0o500)).unwrap();

        assert!(!delete_non_default_config(file.to_str(), true));

        std::fs::set_permissions(&parent, original_permissions).unwrap();
        assert!(file.exists());
    }

    #[test]
    fn load_additional_config_parses_json_and_exports_process_env() {
        let old_key = std::env::var_os("RENOVATE_TEST_ADDENV_KEY");
        std::env::remove_var("RENOVATE_TEST_ADDENV_KEY");

        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("additional.json");
        std::fs::write(
            &path,
            r#"{"processEnv":{"RENOVATE_TEST_ADDENV_KEY":"renovate-test"}, "labels":["renovate"]}"#,
        )
        .unwrap();

        let config = load_additional_config(path.to_str(), dir.path()).unwrap();
        assert_eq!(
            config.labels,
            Vec::from([String::from("renovate")]),
            "processEnv should be stripped and normal config fields preserved"
        );
        assert_eq!(
            std::env::var("RENOVATE_TEST_ADDENV_KEY").ok(),
            Some(String::from("renovate-test"))
        );

        if let Some(value) = old_key {
            std::env::set_var("RENOVATE_TEST_ADDENV_KEY", value);
        } else {
            std::env::remove_var("RENOVATE_TEST_ADDENV_KEY");
        }
    }

    // ── load ─────────────────────────────────────────────────────────────────

    // Rust-specific: unit test for loading empty JSON config
    #[test]
    fn load_empty_json_returns_defaults() {
        let (_f, path) = write_temp("{}", ".json");
        let config = load(&path).unwrap();
        assert_eq!(config, GlobalConfig::default());
    }

    // Rust-specific: unit test for loading JSON config with values
    #[test]
    fn load_json_sets_platform_and_token() {
        let (_f, path) = write_temp(r#"{"platform":"gitlab","token":"mytoken"}"#, ".json");
        let config = load(&path).unwrap();
        assert_eq!(config.platform, Platform::Gitlab);
        assert_eq!(config.token.as_deref(), Some("mytoken"));
    }

    // Rust-specific: unit test for loading JSON5 config with comments
    #[test]
    fn load_json5_with_comment_succeeds() {
        let (_f, path) = write_temp(
            "// renovate config\n{platform: 'github', dryRun: 'full'}",
            ".json5",
        );
        let config = load(&path).unwrap();
        assert_eq!(config.platform, Platform::Github);
        assert_eq!(config.dry_run, Some(DryRun::Full));
    }

    // Rust-specific: unit test for loading .renovaterc without extension
    #[test]
    fn load_renovaterc_without_extension_succeeds() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".renovaterc");
        std::fs::write(&path, r#"{"token":"mytoken"}"#).unwrap();
        let config = load(&path).unwrap();
        assert_eq!(config.token.as_deref(), Some("mytoken"));
    }

    // Rust-specific: unit test for loading requireConfig from file
    #[test]
    fn load_require_config_from_file() {
        let (_f, path) = write_temp(r#"{"requireConfig":"optional"}"#, ".json");
        assert_eq!(load(&path).unwrap().require_config, RequireConfig::Optional);
    }

    // Rust-specific: unit test for rejecting JS extension
    #[test]
    fn load_rejects_js_extension() {
        let (_f, path) = write_temp("module.exports = {}", ".js");
        let err = load(&path).unwrap_err();
        assert!(matches!(err, ConfigFileError::UnsupportedFormat(_)));
    }

    // Ported: "fatal error and exit if %s" — lib/workers/global/config/parse/file.spec.ts line 153
    #[test]
    fn load_rejects_unsupported_or_missing_extension() {
        let (_f, txt_path) = write_temp(r#"{"token":"abc"}"#, ".txt");
        let err = load(&txt_path).unwrap_err();
        assert!(matches!(err, ConfigFileError::UnsupportedFormat(_)));

        let dir = tempfile::tempdir().unwrap();
        let no_extension_path = dir.path().join("file");
        std::fs::write(&no_extension_path, r#"{"token":"abc"}"#).unwrap();
        let err = load(&no_extension_path).unwrap_err();
        assert!(matches!(err, ConfigFileError::UnsupportedFormat(_)));
    }

    // Rust-specific: unit test for JSON parse error handling
    #[test]
    fn load_json_parse_error_is_clear() {
        let (_f, path) = write_temp("not json", ".json");
        let err = load(&path).unwrap_err();
        assert!(matches!(err, ConfigFileError::Parse { .. }));
    }

    // Ported: "parses" — lib/config/parse.spec.ts line 8
    #[test]
    fn parse_file_config_json_parses() {
        assert_eq!(
            parse_file_config("config.json", "{}"),
            ParsedFileConfig::Success {
                parsed_contents: serde_json::json!({})
            }
        );
    }

    // Ported: "returns error" — lib/config/parse.spec.ts line 15
    #[test]
    fn parse_file_config_json_returns_error() {
        let result = parse_file_config("config.json", "{");

        assert!(matches!(
            result,
            ParsedFileConfig::Error {
                validation_error,
                ..
            } if validation_error == "Invalid JSON (parsing failed)"
        ));
    }

    // Ported: "parses" — lib/config/parse.spec.ts line 43
    #[test]
    fn parse_file_config_json5_parses() {
        assert_eq!(
            parse_file_config("config.json5", "{}"),
            ParsedFileConfig::Success {
                parsed_contents: serde_json::json!({})
            }
        );
    }

    // Ported: "returns error" — lib/config/parse.spec.ts line 50
    #[test]
    fn parse_file_config_json5_returns_error() {
        let result = parse_file_config("config.json5", "{");

        assert!(matches!(
            result,
            ParsedFileConfig::Error {
                validation_error,
                ..
            } if validation_error == "Invalid JSON5 (parsing failed)"
        ));
    }

    // ── merge_over_base ──────────────────────────────────────────────────────

    // Rust-specific: unit test for merging file config over base
    #[test]
    fn merge_file_token_over_base() {
        let base = GlobalConfig::default();
        let file = GlobalConfig {
            token: Some("file-token".to_owned()),
            ..GlobalConfig::default()
        };
        assert_eq!(
            merge_over_base(base, file).token.as_deref(),
            Some("file-token")
        );
    }

    // Rust-specific: unit test for base config preservation during merge
    #[test]
    fn merge_base_token_survives_when_file_has_none() {
        let base = GlobalConfig {
            token: Some("base-token".to_owned()),
            ..GlobalConfig::default()
        };
        let file = GlobalConfig::default(); // token = None
        assert_eq!(
            merge_over_base(base, file).token.as_deref(),
            Some("base-token")
        );
    }
}
