use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct MatchManagersMigration;

impl Default for MatchManagersMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl MatchManagersMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for MatchManagersMigration {
    fn property_name(&self) -> &str {
        "matchManagers"
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        _original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        let Value::Array(arr) = value else {
            return;
        };

        let new_value: Vec<Value> = arr
            .iter()
            .filter_map(|v| v.as_str())
            .map(|manager| match manager {
                "regex" => "custom.regex",
                "renovate-config-presets" => "renovate-config",
                _ => manager,
            })
            .map(|s| Value::String(s.into()))
            .collect();

        migrated_config.insert("matchManagers".into(), Value::Array(new_value));
    }

    fn box_clone(&self) -> Box<dyn Migration> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use serde_json::Map;

    use super::MatchManagersMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = MatchManagersMigration::new();
        assert_eq!(m.property_name(), "matchManagers");
    }

    #[test]
    fn migrates_custom_managers() {
        let m = MatchManagersMigration::new();
        let mut migrated = Map::new();
        m.run(
            "matchManagers",
            &json!([
                "npm",
                "regex",
                "custom.regex",
                "custom.someMgr",
                "renovate-config-presets"
            ]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(
            migrated["matchManagers"],
            json!([
                "npm",
                "custom.regex",
                "custom.regex",
                "custom.someMgr",
                "renovate-config"
            ])
        );
    }
}
