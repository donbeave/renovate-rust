use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;
use crate::config::presets::common::removed_presets;

#[derive(Clone, Debug)]
pub struct ExtendsMigration;

impl Default for ExtendsMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl ExtendsMigration {
    pub fn new() -> Self {
        Self
    }

    fn normalize_preset(preset: &str) -> Option<String> {
        // Check removedPresets first
        if let Some(removed) = removed_presets().get(preset) {
            return removed.map(|s| s.to_owned());
        }

        // TODO: Check migratePresets from global config when available

        Some(preset.to_owned())
    }

    fn normalize_presets(presets: &[Value]) -> Vec<Value> {
        presets
            .iter()
            .filter_map(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .filter_map(Self::normalize_preset)
            .filter(|s| !s.is_empty())
            .map(Value::String)
            .collect()
    }
}

impl Migration for ExtendsMigration {
    fn property_name(&self) -> &str {
        "extends"
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        _original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        let new_presets = if let Some(s) = value.as_str() {
            Self::normalize_presets(&[Value::String(s.into())])
        } else if let Some(arr) = value.as_array() {
            Self::normalize_presets(arr)
        } else {
            return;
        };

        migrated_config.insert("extends".into(), Value::Array(new_presets));
    }

    fn box_clone(&self) -> Box<dyn Migration> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use serde_json::Map;

    use super::ExtendsMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = ExtendsMigration::new();
        assert_eq!(m.property_name(), "extends");
    }

    #[test]
    fn migrates_string_to_array() {
        let m = ExtendsMigration::new();
        let mut migrated = Map::new();
        m.run("extends", &json!("foo"), &Map::new(), &mut migrated);
        assert_eq!(migrated["extends"], json!(["foo"]));
    }

    #[test]
    fn migrates_preset_renames() {
        let m = ExtendsMigration::new();
        let mut migrated = Map::new();
        m.run("extends", &json!(":js-app"), &Map::new(), &mut migrated);
        assert_eq!(migrated["extends"], json!(["config:js-app"]));
    }

    #[test]
    fn removes_removed_presets() {
        let m = ExtendsMigration::new();
        let mut migrated = Map::new();
        m.run(
            "extends",
            &json!(["helpers:oddIsUnstable"]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["extends"], json!([]));
    }

    #[test]
    fn filters_non_string_values() {
        let m = ExtendsMigration::new();
        let mut migrated = Map::new();
        m.run("extends", &json!([{}]), &Map::new(), &mut migrated);
        assert_eq!(migrated["extends"], json!([]));
    }

    #[test]
    fn migrates_merge_confidence_preset() {
        let m = ExtendsMigration::new();
        let mut migrated = Map::new();
        m.run(
            "extends",
            &json!(["github>whitesource/merge-confidence:beta"]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["extends"], json!(["mergeConfidence:all-badges"]));
    }
}
