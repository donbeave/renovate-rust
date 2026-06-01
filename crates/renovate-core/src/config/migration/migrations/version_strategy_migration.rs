use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct VersionStrategyMigration;

impl Default for VersionStrategyMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl VersionStrategyMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for VersionStrategyMigration {
    fn property_name(&self) -> &str {
        "versionStrategy"
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
        if value.as_str() == Some("widen") && !migrated_config.contains_key("rangeStrategy") {
            migrated_config.insert("rangeStrategy".into(), Value::String("widen".into()));
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

    use super::VersionStrategyMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = VersionStrategyMigration::new();
        assert_eq!(m.property_name(), "versionStrategy");
    }

    #[test]
    fn migrate_widen() {
        let m = VersionStrategyMigration::new();
        let mut migrated = Map::new();
        m.run(
            "versionStrategy",
            &json!("widen"),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["rangeStrategy"], json!("widen"));
    }

    #[test]
    fn non_widen_is_noop() {
        let m = VersionStrategyMigration::new();
        let mut migrated = Map::new();
        m.run(
            "versionStrategy",
            &json!("test"),
            &Map::new(),
            &mut migrated,
        );
        assert!(migrated.is_empty());
    }

    #[test]
    fn does_not_overwrite_existing_range_strategy() {
        let m = VersionStrategyMigration::new();
        let mut migrated = Map::new();
        migrated.insert("rangeStrategy".into(), json!("bump"));
        m.run(
            "versionStrategy",
            &json!("widen"),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["rangeStrategy"], json!("bump"));
    }
}
