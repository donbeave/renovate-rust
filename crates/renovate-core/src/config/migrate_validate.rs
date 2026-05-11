//! Config migration plus validation.
//!
//! Renovate reference: `lib/config/migrate-validate.ts`.

use serde_json::{Map, Value, json};

use super::massage::massage_config;

/// Migrate, massage, and validate a repository config value.
pub fn migrate_and_validate(base_config: &Value, input: &Value) -> Value {
    let migrated = migrate_config(input);
    let massaged = massage_config(&migrated);
    let errors = validate_config(&massaged);

    let mut result = match massaged {
        Value::Object(map) => map,
        _ => Map::new(),
    };
    result.insert("errors".to_owned(), Value::Array(errors));

    if !base_config
        .get("repoIsOnboarded")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        result.insert("warnings".to_owned(), Value::Array(Vec::new()));
    }

    Value::Object(result)
}

fn migrate_config(input: &Value) -> Value {
    let mut migrated = input.clone();
    if let Value::Object(map) = &mut migrated
        && matches!(map.get("automerge"), Some(Value::String(value)) if value == "none")
    {
        map.insert("automerge".to_owned(), Value::Bool(false));
    }
    migrated
}

fn validate_config(config: &Value) -> Vec<Value> {
    let Some(map) = config.as_object() else {
        return vec![
            json!({"topic": "Configuration Error", "message": "Config must be an object"}),
        ];
    };

    map.keys()
        .filter(|key| !is_known_key(key))
        .map(|key| json!({"topic": "Configuration Error", "message": format!("Invalid config option: {key}")}))
        .collect()
}

fn is_known_key(key: &str) -> bool {
    matches!(
        key,
        "automerge" | "errors" | "packageRules" | "repoIsOnboarded" | "schedule" | "warnings"
    )
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::migrate_and_validate;

    // Ported: "handles empty" — config/migrate-validate.spec.ts line 14
    #[test]
    fn migrate_and_validate_handles_empty() {
        assert_eq!(
            migrate_and_validate(&json!({}), &json!({})),
            json!({"errors": [], "warnings": []})
        );
    }

    // Ported: "handles migration" — config/migrate-validate.spec.ts line 22
    #[test]
    fn migrate_and_validate_handles_migration() {
        assert_eq!(
            migrate_and_validate(&json!({}), &json!({"automerge": "none"})),
            json!({"automerge": false, "errors": [], "warnings": []})
        );
    }

    // Ported: "handles invalid" — config/migrate-validate.spec.ts line 32
    #[test]
    fn migrate_and_validate_handles_invalid() {
        let result = migrate_and_validate(&json!({}), &json!({"foo": "none"}));
        assert_eq!(result["errors"].as_array().expect("errors").len(), 1);
    }

    // Ported: "isOnboarded" — config/migrate-validate.spec.ts line 40
    #[test]
    fn migrate_and_validate_omits_warnings_when_onboarded() {
        let result = migrate_and_validate(&json!({"repoIsOnboarded": true}), &json!({}));
        assert!(result.get("warnings").is_none());
        assert_eq!(result["errors"], json!([]));
    }
}
