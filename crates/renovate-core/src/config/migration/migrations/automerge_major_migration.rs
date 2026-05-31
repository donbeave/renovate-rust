use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct AutomergeMajorMigration;

impl Default for AutomergeMajorMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl AutomergeMajorMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for AutomergeMajorMigration {
    fn property_name(&self) -> &str {
        "automergeMajor"
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
            .entry("major".to_owned())
            .or_insert_with(|| Value::Object(Map::new()));
        if let Some(Value::Object(obj)) = migrated_config.get_mut("major") {
            obj.insert("automerge".into(), Value::Bool(flag));
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

    use super::AutomergeMajorMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = AutomergeMajorMigration::new();
        assert_eq!(m.property_name(), "automergeMajor");
    }

    #[test]
    fn migrate_true_sets_major_automerge() {
        let m = AutomergeMajorMigration::new();
        let mut migrated = Map::new();
        m.run("automergeMajor", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["major"]["automerge"], json!(true));
    }

    #[test]
    fn migrate_false_sets_major_automerge_false() {
        let m = AutomergeMajorMigration::new();
        let mut migrated = Map::new();
        m.run("automergeMajor", &json!(false), &Map::new(), &mut migrated);
        assert_eq!(migrated["major"]["automerge"], json!(false));
    }

    #[test]
    fn preserves_existing_major_object() {
        let m = AutomergeMajorMigration::new();
        let mut migrated = Map::new();
        migrated.insert("major".into(), json!({"foo": "bar"}));
        m.run("automergeMajor", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["major"]["foo"], json!("bar"));
        assert_eq!(migrated["major"]["automerge"], json!(true));
    }
}
