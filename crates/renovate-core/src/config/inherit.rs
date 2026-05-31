//! Config inheritance from parent repositories.
//!
//! Renovate reference: `lib/config/inherit.ts`.

use serde_json::Value;

use super::INHERIT_CONFIG_OPTIONS;

/// Strip inherited config options from a repo config, returning the stripped
/// config and the inherited values.
///
/// Mirrors `InheritConfig.set()` from `lib/config/inherit.ts`.
pub fn strip_inherited_config(config: &Value) -> (Value, Value) {
    let mut result = config.clone();
    let mut inherited = serde_json::Map::new();

    if let Some(obj) = result.as_object_mut() {
        for option in INHERIT_CONFIG_OPTIONS {
            if let Some(value) = obj.remove(*option) {
                inherited.insert(option.to_string(), value);
            }
        }
    }

    (result, Value::Object(inherited))
}

/// Get inherited config value by key.
///
/// Mirrors `InheritConfig.get()` from `lib/config/inherit.ts`.
pub fn get_inherited_config<'a>(inherited: &'a Value, key: &str) -> Option<&'a Value> {
    inherited.get(key)
}

/// Apply inherited config on top of base config for globally-inheritable options.
///
/// Returns a new config with inherited values applied.
pub fn apply_inherited_config(base: &Value, inherited: &Value) -> Value {
    let mut result = base.clone();
    if let (Some(result_obj), Some(inherited_obj)) = (result.as_object_mut(), inherited.as_object())
    {
        for option in INHERIT_CONFIG_OPTIONS {
            if let Some(value) = inherited_obj.get(*option) {
                result_obj.insert(option.to_string(), value.clone());
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn strip_removes_inheritable_keys() {
        let config = json!({
            "onboarding": true,
            "onboardingBranch": "renovate",
            "customField": "value",
            "requireConfig": "required"
        });
        let (stripped, inherited) = strip_inherited_config(&config);
        assert!(stripped.get("onboarding").is_none());
        assert!(stripped.get("requireConfig").is_none());
        assert_eq!(stripped["customField"], "value");
        assert_eq!(inherited["onboarding"], true);
        assert_eq!(inherited["requireConfig"], "required");
    }

    #[test]
    fn get_inherited_config_returns_value() {
        let inherited = json!({"onboarding": false});
        assert_eq!(get_inherited_config(&inherited, "onboarding"), Some(&json!(false)));
        assert_eq!(get_inherited_config(&inherited, "missing"), None);
    }

    #[test]
    fn apply_inherited_config_merges() {
        let base = json!({"customField": "value"});
        let inherited = json!({"onboarding": true, "requireConfig": "optional"});
        let result = apply_inherited_config(&base, &inherited);
        assert_eq!(result["customField"], "value");
        assert_eq!(result["onboarding"], true);
        assert_eq!(result["requireConfig"], "optional");
    }
}
