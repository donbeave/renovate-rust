use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct AutomergeMinorMigration;

impl Default for AutomergeMinorMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl AutomergeMinorMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for AutomergeMinorMigration {
    fn property_name(&self) -> &str {
        "automergeMinor"
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
        let flag = value.as_bool().unwrap_or(false);
        migrated_config
            .entry("minor".to_owned())
            .or_insert_with(|| Value::Object(Map::new()));
        if let Some(Value::Object(obj)) = migrated_config.get_mut("minor") {
            obj.insert("automerge".into(), Value::Bool(flag));
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

    use super::AutomergeMinorMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = AutomergeMinorMigration::new();
        assert_eq!(m.property_name(), "automergeMinor");
    }

    #[test]
    fn migrate_true_sets_minor_automerge() {
        let m = AutomergeMinorMigration::new();
        let mut migrated = Map::new();
        m.run("automergeMinor", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["minor"]["automerge"], json!(true));
    }

    #[test]
    fn migrate_false_sets_minor_automerge_false() {
        let m = AutomergeMinorMigration::new();
        let mut migrated = Map::new();
        m.run("automergeMinor", &json!(false), &Map::new(), &mut migrated);
        assert_eq!(migrated["minor"]["automerge"], json!(false));
    }
}
