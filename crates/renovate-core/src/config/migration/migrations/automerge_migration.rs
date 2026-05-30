use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct AutomergeMigration;

impl Default for AutomergeMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl AutomergeMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for AutomergeMigration {
    fn property_name(&self) -> &str {
        "automerge"
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        _original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        let Some(s) = value.as_str() else {
            return;
        };

        fn ensure_object(map: &mut Map<String, Value>, key: &str) {
            map.entry(key.to_owned())
                .or_insert_with(|| Value::Object(Map::new()));
        }

        match s {
            "none" => {
                migrated_config.insert("automerge".into(), Value::Bool(false));
            }
            "patch" => {
                migrated_config.remove("automerge");
                ensure_object(migrated_config, "patch");
                ensure_object(migrated_config, "minor");
                ensure_object(migrated_config, "major");
                if let Some(Value::Object(obj)) = migrated_config.get_mut("patch") {
                    obj.insert("automerge".into(), Value::Bool(true));
                }
                if let Some(Value::Object(obj)) = migrated_config.get_mut("minor") {
                    obj.insert("automerge".into(), Value::Bool(false));
                }
                if let Some(Value::Object(obj)) = migrated_config.get_mut("major") {
                    obj.insert("automerge".into(), Value::Bool(false));
                }
            }
            "minor" => {
                migrated_config.remove("automerge");
                ensure_object(migrated_config, "minor");
                ensure_object(migrated_config, "major");
                if let Some(Value::Object(obj)) = migrated_config.get_mut("minor") {
                    obj.insert("automerge".into(), Value::Bool(true));
                }
                if let Some(Value::Object(obj)) = migrated_config.get_mut("major") {
                    obj.insert("automerge".into(), Value::Bool(false));
                }
            }
            "any" => {
                migrated_config.insert("automerge".into(), Value::Bool(true));
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

    use super::AutomergeMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = AutomergeMigration::new();
        assert_eq!(m.property_name(), "automerge");
    }

    #[test]
    fn migrate_none_rewrites_to_false() {
        let m = AutomergeMigration::new();
        let mut migrated = Map::new();
        m.run("automerge", &json!("none"), &Map::new(), &mut migrated);
        assert_eq!(migrated["automerge"], json!(false));
    }

    #[test]
    fn migrate_any_rewrites_to_true() {
        let m = AutomergeMigration::new();
        let mut migrated = Map::new();
        m.run("automerge", &json!("any"), &Map::new(), &mut migrated);
        assert_eq!(migrated["automerge"], json!(true));
    }

    #[test]
    fn migrate_patch_sets_patch_minor_major() {
        let m = AutomergeMigration::new();
        let mut migrated = Map::new();
        m.run("automerge", &json!("patch"), &Map::new(), &mut migrated);
        assert!(migrated.get("automerge").is_none());
        assert_eq!(migrated["patch"]["automerge"], json!(true));
        assert_eq!(migrated["minor"]["automerge"], json!(false));
        assert_eq!(migrated["major"]["automerge"], json!(false));
    }

    #[test]
    fn migrate_minor_sets_minor_major() {
        let m = AutomergeMigration::new();
        let mut migrated = Map::new();
        m.run("automerge", &json!("minor"), &Map::new(), &mut migrated);
        assert!(migrated.get("automerge").is_none());
        assert_eq!(migrated["minor"]["automerge"], json!(true));
        assert_eq!(migrated["major"]["automerge"], json!(false));
    }

    #[test]
    fn migrate_non_string_value_is_noop() {
        let m = AutomergeMigration::new();
        let mut migrated = Map::new();
        m.run("automerge", &json!(true), &Map::new(), &mut migrated);
        assert!(migrated.is_empty());
    }
}
