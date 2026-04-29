//! Shared pipeline context passed to every manager processing block.

use renovate_core::config::GlobalConfig;
use renovate_core::http::HttpClient;
use renovate_core::managers::DetectedManager;
use renovate_core::platform::AnyPlatformClient;
use renovate_core::repo_config::RepoConfig;

use crate::output;

/// All state shared by the per-manager pipeline blocks inside `process_repo`.
///
/// Immutable fields are borrowed for the lifetime `'a` of the enclosing
/// `process_repo` call.  Mutable state (`report`, `had_error`) is owned so
/// that individual manager functions can push results and flag errors without
/// returning anything.
pub(crate) struct RepoPipelineCtx<'a> {
    pub client: &'a AnyPlatformClient,
    pub http: &'a HttpClient,
    pub config: &'a GlobalConfig,
    pub owner: &'a str,
    pub repo: &'a str,
    pub repo_slug: &'a str,
    pub repo_cfg: &'a RepoConfig,
    pub detected: &'a [DetectedManager],
    /// Full file list after `ignorePaths` filtering — used by managers that
    /// parse file paths directly rather than file content (e.g. Hermit).
    pub filtered_files: &'a [String],
    /// Accumulates per-file dependency reports from every manager.
    pub report: output::RepoReport,
    /// Set to `true` by any manager block that encounters a hard error.
    pub had_error: bool,
}
