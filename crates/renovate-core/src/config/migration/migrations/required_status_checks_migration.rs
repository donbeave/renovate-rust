use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct RequiredStatusChecksMigration;

impl Default for RequiredStatusChecksMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl RequiredStatusChecksMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for RequiredStatusChecksMigration {
    fn property_name(&self) -> &str {
        "requiredStatusChecks"
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
        if value.is_null() {
            migrated_config.insert("ignoreTests".into(), Value::Bool(true));
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

    use super::RequiredStatusChecksMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = RequiredStatusChecksMigration::new();
        assert_eq!(m.property_name(), "requiredStatusChecks");
    }

    #[test]
    fn migrates_null_to_ignore_tests() {
        let m = RequiredStatusChecksMigration::new();
        let mut migrated = Map::new();
        m.run(
            "requiredStatusChecks",
            &json!(null),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["ignoreTests"], json!(true));
    }

    #[test]
    fn ignores_non_null() {
        let m = RequiredStatusChecksMigration::new();
        let mut migrated = Map::new();
        m.run(
            "requiredStatusChecks",
            &json!(["ci"]),
            &Map::new(),
            &mut migrated,
        );
        assert!(migrated.get("ignoreTests").is_none());
    }
}
