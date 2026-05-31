//! Onboarding config content generation.
//!
//! Mirrors `lib/workers/repository/onboarding/branch/config.ts`.

use serde_json::Value;

use crate::config::GlobalConfig;

pub fn get_onboarding_config_content(global_config: &GlobalConfig) -> String {
    let config = get_onboarding_config(global_config);
    serde_json::to_string_pretty(&config).unwrap_or_else(|_| "{}".to_owned())
}

pub fn get_onboarding_config(global_config: &GlobalConfig) -> Value {
    if global_config.onboarding_config.is_empty() {
        serde_json::json!({
            "$schema": "https://docs.renovatebot.com/renovate-schema.json",
            "extends": ["config:recommended"]
        })
    } else {
        Value::Object(global_config.onboarding_config.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_onboarding_config_default() {
        let global = GlobalConfig::default();
        let config = get_onboarding_config(&global);
        assert_eq!(config["$schema"], "https://docs.renovatebot.com/renovate-schema.json");
        assert_eq!(config["extends"], serde_json::json!(["config:recommended"]));
    }

    #[test]
    fn get_onboarding_config_custom() {
        let mut global = GlobalConfig::default();
        global.onboarding_config.insert(
            "$schema".to_owned(),
            serde_json::json!("https://docs.renovatebot.com/renovate-schema.json"),
        );
        global.onboarding_config.insert(
            "extends".to_owned(),
            serde_json::json!(["config:base"]),
        );
        let config = get_onboarding_config(&global);
        assert_eq!(config["extends"], serde_json::json!(["config:base"]));
    }

    #[test]
    fn get_onboarding_config_content_valid_json() {
        let global = GlobalConfig::default();
        let content = get_onboarding_config_content(&global);
        let parsed: Value = serde_json::from_str(&content).unwrap();
        assert!(parsed.is_object());
    }

    #[test]
    fn get_onboarding_config_content_custom() {
        let mut global = GlobalConfig::default();
        global.onboarding_config.insert(
            "enabled".to_owned(),
            serde_json::json!(true),
        );
        let content = get_onboarding_config_content(&global);
        let parsed: Value = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed["enabled"], true);
    }
}
