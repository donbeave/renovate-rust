use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct CompatibilityMigration;

impl Default for CompatibilityMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl CompatibilityMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for CompatibilityMigration {
    fn property_name(&self) -> &str {
        "compatibility"
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
        if value.is_object() && !migrated_config.contains_key("constraints") {
            migrated_config.insert("constraints".into(), value.clone());
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

    use super::CompatibilityMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = CompatibilityMigration::new();
        assert_eq!(m.property_name(), "compatibility");
    }

    #[test]
    fn migrate_object_to_constraints() {
        let m = CompatibilityMigration::new();
        let mut migrated = Map::new();
        m.run(
            "compatibility",
            &json!({"python": "^3.9"}),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["constraints"], json!({"python": "^3.9"}));
    }

    #[test]
    fn does_not_overwrite_existing_constraints() {
        let m = CompatibilityMigration::new();
        let mut migrated = Map::new();
        migrated.insert("constraints".into(), json!({"node": ">=18"}));
        m.run(
            "compatibility",
            &json!({"python": "^3.9"}),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["constraints"], json!({"node": ">=18"}));
    }

    #[test]
    fn ignores_non_object() {
        let m = CompatibilityMigration::new();
        let mut migrated = Map::new();
        m.run("compatibility", &json!("foo"), &Map::new(), &mut migrated);
        assert!(migrated.is_empty());
    }
}
