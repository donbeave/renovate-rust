//! Canonical run-behavior configuration types.

/// Dry-run mode — controls how much work Renovate performs without persisting
/// changes to the platform.
///
/// Source: `dryRun` option in `lib/config/options/index.ts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DryRun {
    /// Perform dependency extraction only; no version lookups.
    Extract,
    /// Perform extraction and version lookups; no branch/PR changes.
    Lookup,
    /// Log all actions that would be taken without creating branches or PRs.
    Full,
}

impl std::fmt::Display for DryRun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Extract => "extract",
            Self::Lookup => "lookup",
            Self::Full => "full",
        };
        f.write_str(s)
    }
}

/// Controls how Renovate responds to missing repository config files.
///
/// Source: `requireConfig` option in `lib/config/options/index.ts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequireConfig {
    /// Onboard repositories that lack a config file; skip those that do.
    Required,
    /// Process repositories with or without a config file.
    Optional,
    /// Ignore the presence or absence of a config file entirely.
    Ignored,
}

/// Whether to process forked repositories.
///
/// Source: `forkProcessing` option.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForkProcessing {
    /// Skip forks in autodiscover mode; process if explicitly listed.
    Auto,
    /// Always process forks.
    Enabled,
    /// Never process forks.
    Disabled,
}

/// When to recreate closed PRs.
///
/// Source: `recreateWhen` option.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecreateWhen {
    /// Only recreate if the PR was closed without merging (Renovate default).
    Auto,
    /// Always recreate closed PRs.
    Always,
    /// Never recreate closed PRs.
    Never,
}
