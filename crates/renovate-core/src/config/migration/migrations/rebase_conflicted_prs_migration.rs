use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct RebaseConflictedPrsMigration;

impl Default for RebaseConflictedPrsMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl RebaseConflictedPrsMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for RebaseConflictedPrsMigration {
    fn property_name(&self) -> &str {
        "rebaseConflictedPrs"
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
        if value.as_bool() == Some(false) && !migrated_config.contains_key("rebaseWhen") {
            migrated_config.insert("rebaseWhen".into(), Value::String("never".into()));
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

    use super::RebaseConflictedPrsMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = RebaseConflictedPrsMigration::new();
        assert_eq!(m.property_name(), "rebaseConflictedPrs");
    }

    #[test]
    fn migrate_false_to_never() {
        let m = RebaseConflictedPrsMigration::new();
        let mut migrated = Map::new();
        m.run("rebaseConflictedPrs", &json!(false), &Map::new(), &mut migrated);
        assert_eq!(migrated["rebaseWhen"], json!("never"));
    }

    #[test]
    fn migrate_true_is_noop() {
        let m = RebaseConflictedPrsMigration::new();
        let mut migrated = Map::new();
        m.run("rebaseConflictedPrs", &json!(true), &Map::new(), &mut migrated);
        assert!(migrated.is_empty());
    }

    #[test]
    fn does_not_overwrite_existing_rebase_when() {
        let m = RebaseConflictedPrsMigration::new();
        let mut migrated = Map::new();
        migrated.insert("rebaseWhen".into(), json!("auto"));
        m.run("rebaseConflictedPrs", &json!(false), &Map::new(), &mut migrated);
        assert_eq!(migrated["rebaseWhen"], json!("auto"));
    }
}
