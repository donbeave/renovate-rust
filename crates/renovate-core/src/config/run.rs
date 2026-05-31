//! Canonical run-behavior configuration types.

/// Controls how Renovate invokes third-party tools.
///
/// Source: `binarySource` option in `lib/config/options/index.ts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BinarySource {
    /// Use globally available tools.
    Global,
    /// Use Docker sidecar containers.
    Docker,
    /// Dynamically install tools.
    Install,
    /// Use Hermit-managed tools.
    Hermit,
}

impl std::fmt::Display for BinarySource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Global => "global",
            Self::Docker => "docker",
            Self::Install => "install",
            Self::Hermit => "hermit",
        };
        f.write_str(s)
    }
}

/// Dry-run mode — controls how much work Renovate performs without persisting
/// changes to the platform.
///
/// Source: `dryRun` option in `lib/config/options/index.ts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RequireConfig {
    /// Onboard repositories that lack a config file; skip those that do.
    Required,
    /// Process repositories with or without a config file.
    Optional,
    /// Ignore the presence or absence of a config file entirely.
    Ignored,
}

impl std::fmt::Display for RequireConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Required => "required",
            Self::Optional => "optional",
            Self::Ignored => "ignored",
        };
        f.write_str(s)
    }
}

/// Whether to process forked repositories.
///
/// Source: `forkProcessing` option.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ForkProcessing {
    /// Skip forks in autodiscover mode; process if explicitly listed.
    Auto,
    /// Always process forks.
    Enabled,
    /// Never process forks.
    Disabled,
}

impl std::fmt::Display for ForkProcessing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Auto => "auto",
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
        };
        f.write_str(s)
    }
}

/// When to recreate closed PRs.
///
/// Source: `recreateWhen` option.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RecreateWhen {
    /// Only recreate if the PR was closed without merging (Renovate default).
    Auto,
    /// Always recreate closed PRs.
    Always,
    /// Never recreate closed PRs.
    Never,
}

impl std::fmt::Display for RecreateWhen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Auto => "auto",
            Self::Always => "always",
            Self::Never => "never",
        };
        f.write_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_source_display() {
        assert_eq!(BinarySource::Global.to_string(), "global");
        assert_eq!(BinarySource::Docker.to_string(), "docker");
        assert_eq!(BinarySource::Install.to_string(), "install");
        assert_eq!(BinarySource::Hermit.to_string(), "hermit");
    }

    #[test]
    fn dry_run_display() {
        assert_eq!(DryRun::Extract.to_string(), "extract");
        assert_eq!(DryRun::Lookup.to_string(), "lookup");
        assert_eq!(DryRun::Full.to_string(), "full");
    }

    #[test]
    fn require_config_display() {
        assert_eq!(RequireConfig::Required.to_string(), "required");
        assert_eq!(RequireConfig::Optional.to_string(), "optional");
        assert_eq!(RequireConfig::Ignored.to_string(), "ignored");
    }

    #[test]
    fn fork_processing_display() {
        assert_eq!(ForkProcessing::Auto.to_string(), "auto");
        assert_eq!(ForkProcessing::Enabled.to_string(), "enabled");
        assert_eq!(ForkProcessing::Disabled.to_string(), "disabled");
    }

    #[test]
    fn recreate_when_display() {
        assert_eq!(RecreateWhen::Auto.to_string(), "auto");
        assert_eq!(RecreateWhen::Always.to_string(), "always");
        assert_eq!(RecreateWhen::Never.to_string(), "never");
    }
}
