//! Global config file discovery and loading.
//!
//! Renovate reference: `lib/workers/global/config/parse/file.ts`.
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
    let Some(explicit) = config_file_env else {
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

    // .renovaterc (no extension) and .json → standard JSON via serde_json.
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
        dry_run: file_config.dry_run.or(base.dry_run),
        // For non-Option fields: file always wins (it was explicitly set in
        // the file or it carries the default — we cannot distinguish, so we
        // always take the file's value, then let CLI override afterwards).
        platform: file_config.platform,
        require_config: file_config.require_config,
        fork_processing: file_config.fork_processing,
        platform_automerge: file_config.platform_automerge,
        recreate_when: file_config.recreate_when,
        allowed_commands: if file_config.allowed_commands.is_empty() {
            base.allowed_commands
        } else {
            file_config.allowed_commands
        },
        allow_command_templating: file_config.allow_command_templating,
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

    #[test]
    fn resolve_returns_none_when_env_not_set() {
        let dir = tempfile::tempdir().unwrap();
        assert!(resolve_config_path(None, dir.path()).unwrap().is_none());
    }

    #[test]
    fn resolve_returns_path_when_file_exists() {
        let (f, path) = write_temp("{}", ".json");
        let dir = f.path().parent().unwrap().to_owned();
        let filename = path.file_name().unwrap().to_str().unwrap().to_owned();
        let resolved = resolve_config_path(Some(&filename), &dir).unwrap();
        assert_eq!(resolved, Some(path));
    }

    #[test]
    fn resolve_errors_when_explicit_file_missing() {
        let dir = tempfile::tempdir().unwrap();
        let err = resolve_config_path(Some("no_such_file.json"), dir.path()).unwrap_err();
        assert!(matches!(err, ConfigFileError::ExplicitPathNotFound(_)));
    }

    // ── load ─────────────────────────────────────────────────────────────────

    #[test]
    fn load_empty_json_returns_defaults() {
        let (_f, path) = write_temp("{}", ".json");
        let config = load(&path).unwrap();
        assert_eq!(config, GlobalConfig::default());
    }

    #[test]
    fn load_json_sets_platform_and_token() {
        let (_f, path) = write_temp(r#"{"platform":"gitlab","token":"mytoken"}"#, ".json");
        let config = load(&path).unwrap();
        assert_eq!(config.platform, Platform::Gitlab);
        assert_eq!(config.token.as_deref(), Some("mytoken"));
    }

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

    #[test]
    fn load_require_config_from_file() {
        let (_f, path) = write_temp(r#"{"requireConfig":"optional"}"#, ".json");
        assert_eq!(load(&path).unwrap().require_config, RequireConfig::Optional);
    }

    #[test]
    fn load_rejects_js_extension() {
        let (_f, path) = write_temp("module.exports = {}", ".js");
        let err = load(&path).unwrap_err();
        assert!(matches!(err, ConfigFileError::UnsupportedFormat(_)));
    }

    #[test]
    fn load_json_parse_error_is_clear() {
        let (_f, path) = write_temp("not json", ".json");
        let err = load(&path).unwrap_err();
        assert!(matches!(err, ConfigFileError::Parse { .. }));
    }

    // ── merge_over_base ──────────────────────────────────────────────────────

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
