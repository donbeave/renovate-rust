use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct RecreateClosedMigration;

impl Default for RecreateClosedMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl RecreateClosedMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for RecreateClosedMigration {
    fn property_name(&self) -> &str {
        "recreateClosed"
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
            let new_value = if flag { "always" } else { "auto" };
            if !migrated_config.contains_key("recreateWhen") {
                migrated_config.insert("recreateWhen".into(), Value::String(new_value.into()));
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

    use super::RecreateClosedMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = RecreateClosedMigration::new();
        assert_eq!(m.property_name(), "recreateClosed");
    }

    #[test]
    fn migrate_true_to_always() {
        let m = RecreateClosedMigration::new();
        let mut migrated = Map::new();
        m.run("recreateClosed", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["recreateWhen"], json!("always"));
    }

    #[test]
    fn migrate_false_to_auto() {
        let m = RecreateClosedMigration::new();
        let mut migrated = Map::new();
        m.run("recreateClosed", &json!(false), &Map::new(), &mut migrated);
        assert_eq!(migrated["recreateWhen"], json!("auto"));
    }

    #[test]
    fn does_not_overwrite_existing_recreate_when() {
        let m = RecreateClosedMigration::new();
        let mut migrated = Map::new();
        migrated.insert("recreateWhen".into(), json!("never"));
        m.run("recreateClosed", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["recreateWhen"], json!("never"));
    }
}
