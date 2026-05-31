use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct DryRunMigration;

impl Default for DryRunMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl DryRunMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for DryRunMigration {
    fn property_name(&self) -> &str {
        "dryRun"
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        _original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        match value {
            Value::Bool(true) => {
                migrated_config.insert("dryRun".into(), Value::String("full".into()));
            }
            Value::Bool(false) => {
                migrated_config.insert("dryRun".into(), Value::Null);
            }
            _ => {}
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

    use super::DryRunMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = DryRunMigration::new();
        assert_eq!(m.property_name(), "dryRun");
    }

    #[test]
    fn migrate_true_to_full() {
        let m = DryRunMigration::new();
        let mut migrated = Map::new();
        m.run("dryRun", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["dryRun"], json!("full"));
    }

    #[test]
    fn migrate_false_to_null() {
        let m = DryRunMigration::new();
        let mut migrated = Map::new();
        m.run("dryRun", &json!(false), &Map::new(), &mut migrated);
        assert_eq!(migrated["dryRun"], json!(null));
    }

    #[test]
    fn leaves_string_unchanged() {
        let m = DryRunMigration::new();
        let mut migrated = Map::new();
        m.run("dryRun", &json!("lookup"), &Map::new(), &mut migrated);
        assert!(migrated.is_empty());
    }
}
