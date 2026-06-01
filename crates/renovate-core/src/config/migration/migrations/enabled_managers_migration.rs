use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct EnabledManagersMigration;

impl Default for EnabledManagersMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl EnabledManagersMigration {
    pub fn new() -> Self {
        Self
    }
}

fn migrate_manager(manager: &str) -> &str {
    match manager {
        "yarn" => "npm",
        "regex" => "custom.regex",
        "renovate-config-presets" => "renovate-config",
        _ => manager,
    }
}

impl Migration for EnabledManagersMigration {
    fn property_name(&self) -> &str {
        "enabledManagers"
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        _original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        let Some(arr) = value.as_array() else {
            return;
        };

        let new_value: Vec<Value> = arr
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| Value::String(migrate_manager(s).into()))
            .collect();

        if !new_value.is_empty() {
            migrated_config.insert("enabledManagers".into(), Value::Array(new_value));
        }
    }

    fn box_clone(&self) -> Box<dyn Migration> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Map;
    use serde_json::json;

    use super::EnabledManagersMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = EnabledManagersMigration::new();
        assert_eq!(m.property_name(), "enabledManagers");
    }

    #[test]
    fn migrates_yarn_to_npm() {
        let m = EnabledManagersMigration::new();
        let mut migrated = Map::new();
        m.run(
            "enabledManagers",
            &json!(["yarn"]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["enabledManagers"], json!(["npm"]));
    }

    #[test]
    fn migrates_regex_to_custom_regex() {
        let m = EnabledManagersMigration::new();
        let mut migrated = Map::new();
        m.run(
            "enabledManagers",
            &json!(["regex"]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["enabledManagers"], json!(["custom.regex"]));
    }

    #[test]
    fn leaves_npm_unchanged() {
        let m = EnabledManagersMigration::new();
        let mut migrated = Map::new();
        m.run(
            "enabledManagers",
            &json!(["npm"]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["enabledManagers"], json!(["npm"]));
    }

    #[test]
    fn ignores_non_array() {
        let m = EnabledManagersMigration::new();
        let mut migrated = Map::new();
        m.run("enabledManagers", &json!("npm"), &Map::new(), &mut migrated);
        assert!(migrated.is_empty());
    }
}
