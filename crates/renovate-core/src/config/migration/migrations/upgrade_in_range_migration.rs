use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct UpgradeInRangeMigration;

impl Default for UpgradeInRangeMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl UpgradeInRangeMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for UpgradeInRangeMigration {
    fn property_name(&self) -> &str {
        "upgradeInRange"
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
        if value.as_bool() == Some(true) && !migrated_config.contains_key("rangeStrategy") {
            migrated_config.insert("rangeStrategy".into(), Value::String("bump".into()));
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

    use super::UpgradeInRangeMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = UpgradeInRangeMigration::new();
        assert_eq!(m.property_name(), "upgradeInRange");
    }

    #[test]
    fn migrate_true() {
        let m = UpgradeInRangeMigration::new();
        let mut migrated = Map::new();
        m.run("upgradeInRange", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["rangeStrategy"], json!("bump"));
    }

    #[test]
    fn migrate_false_is_noop() {
        let m = UpgradeInRangeMigration::new();
        let mut migrated = Map::new();
        m.run("upgradeInRange", &json!(false), &Map::new(), &mut migrated);
        assert!(migrated.is_empty());
    }

    #[test]
    fn does_not_overwrite_existing_range_strategy() {
        let m = UpgradeInRangeMigration::new();
        let mut migrated = Map::new();
        migrated.insert("rangeStrategy".into(), json!("pin"));
        m.run("upgradeInRange", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["rangeStrategy"], json!("pin"));
    }
}
