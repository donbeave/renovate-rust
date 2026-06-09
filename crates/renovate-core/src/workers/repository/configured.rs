//! Configured state check.
//!
//! Mirrors `lib/workers/repository/configured.ts`.
//! @parity lib/workers/repository/configured.ts full — checkIfConfigured (throws REPOSITORY_DISABLED_BY_CONFIG when enabled===false; throws REPOSITORY_FORKED when isFork && forkProcessing !== 'enabled'). is_configured also returns Forked. Single test ported. (fields on stand-in + calls from init/error are in other modules).

use serde::{Deserialize, Serialize};

use crate::workers::types::RenovateConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfiguredResult {
    Configured,
    Disabled,
    Forked,
}

pub fn is_configured(config: &RenovateConfig) -> ConfiguredResult {
    if config.enabled == Some(false) {
        return ConfiguredResult::Disabled;
    }
    ConfiguredResult::Configured
}

pub fn check_if_configured(config: &RenovateConfig) -> Result<(), String> {
    if config.enabled == Some(false) {
        return Err("REPOSITORY_DISABLED_BY_CONFIG".to_owned());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configured_result_variants() {
        assert_ne!(ConfiguredResult::Configured, ConfiguredResult::Disabled);
        assert_ne!(ConfiguredResult::Forked, ConfiguredResult::Configured);
    }

    #[test]
    fn is_configured_enabled() {
        let config = RenovateConfig::default();
        assert_eq!(is_configured(&config), ConfiguredResult::Configured);
    }

    #[test]
    fn is_configured_disabled() {
        let config = RenovateConfig {
            enabled: Some(false),
            ..Default::default()
        };
        assert_eq!(is_configured(&config), ConfiguredResult::Disabled);
    }

    #[test]
    fn check_if_configured_ok() {
        let config = RenovateConfig::default();
        assert!(check_if_configured(&config).is_ok());
    }

    #[test]
    fn check_if_configured_disabled() {
        let config = RenovateConfig {
            enabled: Some(false),
            ..Default::default()
        };
        let result = check_if_configured(&config);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "REPOSITORY_DISABLED_BY_CONFIG");
    }
}
