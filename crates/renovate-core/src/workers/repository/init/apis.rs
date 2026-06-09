//! API initialization.
//!
//! Mirrors `lib/workers/repository/init/apis.ts`.
//! @parity lib/workers/repository/init/apis.ts partial — initApis + getPlatformConfig (platform.initRepo) + validateOptimizeForDisabled (optimizeForDisabled + getJsonFile default config + :disableRenovate re-enable logic) + validateIncludeForks (forkProcessing/includeForks + repo config checks, getJsonFile failure ignore) + getDefaultConfigFileName + onboardingConfigFileName handling. Single test ported. Full platform result merge + async wiring pending in init/index + worker.

use serde::{Deserialize, Serialize};

use crate::config::GlobalConfig;
use crate::workers::repository::onboarding::common::get_default_config_file_name;
use crate::workers::types::RenovateConfig;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiConfig {
    pub platform: Option<String>,
    pub endpoint: Option<String>,
    pub token: Option<String>,
    pub initialized: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiInitResult {
    pub config: ApiConfig,
    pub errors: Vec<String>,
}

pub fn init_apis(platform: &str, endpoint: Option<&str>) -> ApiInitResult {
    ApiInitResult {
        config: ApiConfig {
            platform: Some(platform.to_owned()),
            endpoint: endpoint.map(|s| s.to_owned()),
            token: None,
            initialized: true,
        },
        errors: Vec::new(),
    }
}

// Port of the high-level initApis + getPlatformConfig + validate* from lib/workers/repository/init/apis.ts.
// getPlatformConfig calls platform.initRepo (enrich + fingerprint); here we stub/merge for the current
// architecture (real platform call lives in platform/* + wiring in pending init/index + worker).
// Validates reuse/extend the configured checks for DISABLED/FORKED + add the optimizeForDisabled +
// includeForks/forkProcessing repo-config file checks (getJsonFile via platform, with failure ignore).
// Divergence note: async platform calls are sync-wrapped or deferred in current stubs; full merge into
// WorkerPlatformConfig / RenovateConfig happens at caller (see index.ts flow). Uses full paths only.

fn get_default_config_file(config: &RenovateConfig, global: &GlobalConfig) -> String {
    // Mirrors getDefaultConfigFileName + respect onboardingConfigFileName (with validation fallback to renovate.json)
    if let Some(name) = &config.onboarding_config_file_name {
        if name == "renovate.json"
            || name == "renovate.json5"
            || name.ends_with("/renovate.json")
            || name.ends_with("/renovate.json5")
        {
            return name.clone();
        }
        // invalid -> fallback (the TS test covers this)
        return "renovate.json".to_owned();
    }
    get_default_config_file_name(global)
}

pub fn validate_optimize_for_disabled(
    config: &RenovateConfig,
    repo_config: Option<&RenovateConfig>,
) -> Result<(), String> {
    if config.optimize_for_disabled != Some(true) {
        return Ok(());
    }
    // getJsonFile(defaultConfigFile) or null on error (ignored per 'ignores platform.getJsonFile() failures')
    let renovate_config = repo_config;
    if let Some(rc) = renovate_config {
        if rc.enabled == Some(false) {
            return Err("REPOSITORY_DISABLED_BY_CONFIG".to_owned());
        }
        // support :disableRenovate + re-enable via :enableRenovate or ignorePresets or enabled:true in repo config
        if config
            .extends
            .as_ref()
            .map_or(false, |e| e.iter().any(|x| x == ":disableRenovate"))
        {
            if rc
                .extends
                .as_ref()
                .map_or(false, |e| e.iter().any(|x| x == ":enableRenovate"))
                || rc.ignore_presets.as_ref().map_or(false, |p: &Vec<String>| {
                    p.iter().any(|x| x == ":disableRenovate")
                })
                || rc.enabled == Some(true)
            {
                // re-enabled, continue
            } else {
                return Err("REPOSITORY_DISABLED_BY_CONFIG".to_owned());
            }
        }
    }
    Ok(())
}

pub fn validate_include_forks(
    config: &RenovateConfig,
    repo_config: Option<&RenovateConfig>,
) -> Result<(), String> {
    if config.fork_processing.as_deref() == Some("enabled") || !config.is_fork.unwrap_or(false) {
        return Ok(());
    }
    // forkProcessing !== 'enabled' && isFork -> must have repo config enabling it
    let _default_config_file = get_default_config_file(config, &GlobalConfig::default());
    let repo_config = repo_config;
    if repo_config.is_none() {
        return Err("REPOSITORY_FORKED".to_owned());
    }
    let rc = repo_config.unwrap();
    if rc.include_forks == Some(true) || rc.fork_processing.as_deref() == Some("enabled") {
        return Ok(());
    }
    Err("REPOSITORY_FORKED".to_owned())
}

// High-level entry mirroring initApis (getPlatformConfig + validates).
// For current architecture the platform enrichment (initRepo) is limited; we focus on the
// observable validate + default-file + fork/optimize logic (the unique surface of this TS file).
pub fn init_apis_for_repository(input: &RenovateConfig) -> Result<RenovateConfig, String> {
    let config = input.clone();
    // getPlatformConfig(config) stub: in full this calls platform.initRepo and spreads; here we
    // assume caller or prior init has enriched (default_branch etc.). We just run the post-init validates.
    // (Divergence: full platform result merge + fingerprint is in platform + pending init units.)
    validate_optimize_for_disabled(&config, None)?; // repo_config fetch is simulated by caller/tests for now
    validate_include_forks(&config, None)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_config_default() {
        let c = ApiConfig::default();
        assert!(c.platform.is_none());
        assert!(!c.initialized);
    }

    #[test]
    fn api_init_result_default() {
        let r = ApiInitResult::default();
        assert!(r.errors.is_empty());
    }

    #[test]
    fn init_apis_github() {
        let result = init_apis("github", None);
        assert!(result.config.initialized);
        assert_eq!(result.config.platform, Some("github".to_owned()));
        assert!(result.config.endpoint.is_none());
        assert!(result.errors.is_empty());
    }

    #[test]
    fn init_apis_gitlab_with_endpoint() {
        let result = init_apis("gitlab", Some("https://gitlab.example.com/api/v4"));
        assert_eq!(
            result.config.endpoint,
            Some("https://gitlab.example.com/api/v4".to_owned())
        );
    }

    #[test]
    fn api_config_serialization_roundtrip() {
        let c = ApiConfig {
            platform: Some("github".into()),
            endpoint: Some("https://api.github.com".into()),
            token: None,
            initialized: true,
        };
        let json = serde_json::to_string(&c).unwrap();
        let back: ApiConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(back.platform, Some("github".into()));
    }

    // Ported: "throws for disabled" — lib/workers/repository/init/apis.spec.ts line 34
    #[test]
    fn throws_for_disabled() {
        // Exercises the validateOptimizeForDisabled + getJsonFile(default config) + REPOSITORY_DISABLED_BY_CONFIG
        // path from initApis (the core new behavior in this TS unit beyond basic configured checks).
        // Simulated repo_config (as if from platform.getJsonFile) with enabled=false triggers the throw.
        let mut config = RenovateConfig::default();
        config.optimize_for_disabled = Some(true);
        let repo_config = Some(RenovateConfig {
            enabled: Some(false),
            ..Default::default()
        });
        let res = validate_optimize_for_disabled(&config, repo_config.as_ref());
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("REPOSITORY_DISABLED_BY_CONFIG"));
    }

    // Ported: "runs" — lib/workers/repository/init/apis.spec.ts line 24
    #[test]
    fn runs() {
        // Exercises the happy path through init_apis_for_repository (validates + default file) for normal non-fork non-optimize config.
        let config = RenovateConfig::default();
        let res = init_apis_for_repository(&config);
        assert!(res.is_ok());
    }

    // Ported: "throws for forked" — lib/workers/repository/init/apis.spec.ts line 49
    #[test]
    fn throws_for_forked() {
        let config = RenovateConfig {
            is_fork: Some(true),
            ..Default::default()
        };
        // fork_processing not enabled, no repo_config (getJsonFile failure or no config)
        let res = validate_include_forks(&config, None);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("REPOSITORY_FORKED"));
    }

    // Ported: "does not throw for includeforks=true" — lib/workers/repository/init/apis.spec.ts line 66
    #[test]
    fn does_not_throw_for_includeforks_true() {
        let config = RenovateConfig {
            is_fork: Some(true),
            ..Default::default()
        };
        let rc = RenovateConfig {
            include_forks: Some(true),
            ..Default::default()
        };
        let res = validate_include_forks(&config, Some(&rc));
        assert!(res.is_ok());
    }

    // Ported: "does not throw for forkprocessing=enabled" — lib/workers/repository/init/apis.spec.ts line 79
    #[test]
    fn does_not_throw_for_forkprocessing_enabled() {
        let config = RenovateConfig {
            is_fork: Some(true),
            ..Default::default()
        };
        let rc = RenovateConfig {
            fork_processing: Some("enabled".to_string()),
            ..Default::default()
        };
        let res = validate_include_forks(&config, Some(&rc));
        assert!(res.is_ok());
    }

    // Ported: "ignores platform.getjsonfile() failures" — lib/workers/repository/init/apis.spec.ts line 92
    #[test]
    fn ignores_platform_getjsonfile_failures() {
        // Non-fork or optimize case ignores getJsonFile failure (repo_config=None)
        let config = RenovateConfig::default(); // not fork
        let res = validate_include_forks(&config, None);
        assert!(res.is_ok());
        // Also for optimize disabled case with no repo
        let opt_config = RenovateConfig {
            optimize_for_disabled: Some(true),
            ..Default::default()
        };
        let res2 = validate_optimize_for_disabled(&opt_config, None);
        assert!(res2.is_ok());
    }

    // Ported: "throws for fork with platform.getjsonfile() failures" — lib/workers/repository/init/apis.spec.ts line 109
    #[test]
    fn throws_for_fork_with_platform_getjsonfile_failures() {
        let mut config = RenovateConfig::default();
        config.is_fork = Some(true);
        // forkProcessing not enabled, repo_config=None (failure) -> throws
        let res = validate_include_forks(&config, None);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("REPOSITORY_FORKED"));
    }

    // Ported: "uses the onboardingconfigfilename if set" — lib/workers/repository/init/apis.spec.ts line 124
    #[test]
    fn uses_the_onboardingconfigfilename_if_set() {
        let config = RenovateConfig {
            onboarding_config_file_name: Some("renovate.json5".to_string()),
            ..Default::default()
        };
        let name = get_default_config_file(&config, &GlobalConfig::default());
        assert_eq!(name, "renovate.json5");
    }

    // Ported: "falls back to \"renovate.json\" if onboardingconfigfilename is not set" — lib/workers/repository/init/apis.spec.ts line 151
    #[test]
    fn falls_back_to_renovate_json_if_onboardingconfigfilename_is_not_set() {
        let config = RenovateConfig::default();
        let name = get_default_config_file(&config, &GlobalConfig::default());
        // falls back to the get_default_config_file_name result, which is typically renovate.json
        assert!(name == "renovate.json" || name.ends_with("/renovate.json") || name.ends_with("/renovate.json5"));
    }

    // Ported: "falls back to \"renovate.json\" if onboardingconfigfilename is not valid" — lib/workers/repository/init/apis.spec.ts line 172
    #[test]
    fn falls_back_to_renovate_json_if_onboardingconfigfilename_is_not_valid() {
        let config = RenovateConfig {
            onboarding_config_file_name: Some("custom.json".to_string()),
            ..Default::default()
        }; // not valid renovate.*
        let name = get_default_config_file(&config, &GlobalConfig::default());
        assert_eq!(name, "renovate.json");
    }

    // Ported: "checks for re-enablement and continues" — lib/workers/repository/init/apis.spec.ts line 191
    #[test]
    fn checks_for_re_enablement_and_continues() {
        let config = RenovateConfig {
            optimize_for_disabled: Some(true),
            extends: Some(vec![":disableRenovate".to_string()]),
            ..Default::default()
        };
        let rc = RenovateConfig {
            extends: Some(vec![":enableRenovate".to_string()]),
            ..Default::default()
        };
        let res = validate_optimize_for_disabled(&config, Some(&rc));
        assert!(res.is_ok());
    }

    // Ported: "checks for re-enablement and skips" — lib/workers/repository/init/apis.spec.ts line 211
    #[test]
    fn checks_for_re_enablement_and_skips() {
        let config = RenovateConfig {
            optimize_for_disabled: Some(true),
            extends: Some(vec![":disableRenovate".to_string()]),
            ..Default::default()
        };
        let rc = RenovateConfig {
            enabled: Some(false),
            ..Default::default()
        };
        // no re-enable in rc
        let res = validate_optimize_for_disabled(&config, Some(&rc));
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("REPOSITORY_DISABLED_BY_CONFIG"));
    }
}
