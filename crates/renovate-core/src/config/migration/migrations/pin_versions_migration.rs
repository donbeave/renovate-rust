use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct PinVersionsMigration;

impl Default for PinVersionsMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl PinVersionsMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for PinVersionsMigration {
    fn property_name(&self) -> &str {
        "pinVersions"
    }

    fn is_deprecated(&self) -> bool {
        true
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        _original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        if let Some(flag) = value.as_bool() {
            let new_value = if flag { "pin" } else { "replace" };
            if !migrated_config.contains_key("rangeStrategy") {
                migrated_config.insert("rangeStrategy".into(), Value::String(new_value.into()));
            }
        }
    }

    fn box_clone(&self) -> Box<dyn Migration> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use serde_json::Map;

    use super::PinVersionsMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = PinVersionsMigration::new();
        assert_eq!(m.property_name(), "pinVersions");
    }

    #[test]
    fn migrate_true_sets_pin() {
        let m = PinVersionsMigration::new();
        let mut migrated = Map::new();
        m.run("pinVersions", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["rangeStrategy"], json!("pin"));
    }

    #[test]
    fn migrate_false_sets_replace() {
        let m = PinVersionsMigration::new();
        let mut migrated = Map::new();
        m.run("pinVersions", &json!(false), &Map::new(), &mut migrated);
        assert_eq!(migrated["rangeStrategy"], json!("replace"));
    }

    #[test]
    fn does_not_overwrite_existing_range_strategy() {
        let m = PinVersionsMigration::new();
        let mut migrated = Map::new();
        migrated.insert("rangeStrategy".into(), json!("bump"));
        m.run("pinVersions", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["rangeStrategy"], json!("bump"));
    }
}
