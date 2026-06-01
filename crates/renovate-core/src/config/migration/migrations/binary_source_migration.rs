use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct BinarySourceMigration;

impl Default for BinarySourceMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl BinarySourceMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for BinarySourceMigration {
    fn property_name(&self) -> &str {
        "binarySource"
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        _original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        if value.as_str() == Some("auto") {
            migrated_config.insert("binarySource".into(), Value::String("global".into()));
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

    use super::BinarySourceMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = BinarySourceMigration::new();
        assert_eq!(m.property_name(), "binarySource");
    }

    #[test]
    fn migrate_auto_to_global() {
        let m = BinarySourceMigration::new();
        let mut migrated = Map::new();
        m.run("binarySource", &json!("auto"), &Map::new(), &mut migrated);
        assert_eq!(migrated["binarySource"], json!("global"));
    }

    #[test]
    fn non_auto_value_is_noop() {
        let m = BinarySourceMigration::new();
        let mut migrated = Map::new();
        m.run("binarySource", &json!("docker"), &Map::new(), &mut migrated);
        assert!(migrated.get("binarySource").is_none());
    }

    #[test]
    fn non_string_value_is_noop() {
        let m = BinarySourceMigration::new();
        let mut migrated = Map::new();
        m.run("binarySource", &json!(42), &Map::new(), &mut migrated);
        assert!(migrated.is_empty());
    }
}
