use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct RequireConfigMigration;

impl Default for RequireConfigMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl RequireConfigMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for RequireConfigMigration {
    fn property_name(&self) -> &str {
        "requireConfig"
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        _original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        let new_value = match value {
            Value::Bool(false) => Some("optional"),
            Value::Bool(true) => Some("required"),
            Value::String(s) if s == "false" => Some("optional"),
            Value::String(s) if s == "true" => Some("required"),
            _ => None,
        };
        if let Some(v) = new_value {
            migrated_config.insert("requireConfig".into(), Value::String(v.into()));
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

    use super::RequireConfigMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = RequireConfigMigration::new();
        assert_eq!(m.property_name(), "requireConfig");
    }

    #[test]
    fn migrate_bool_false_to_optional() {
        let m = RequireConfigMigration::new();
        let mut migrated = Map::new();
        m.run("requireConfig", &json!(false), &Map::new(), &mut migrated);
        assert_eq!(migrated["requireConfig"], json!("optional"));
    }

    #[test]
    fn migrate_string_false_to_optional() {
        let m = RequireConfigMigration::new();
        let mut migrated = Map::new();
        m.run("requireConfig", &json!("false"), &Map::new(), &mut migrated);
        assert_eq!(migrated["requireConfig"], json!("optional"));
    }

    #[test]
    fn migrate_bool_true_to_required() {
        let m = RequireConfigMigration::new();
        let mut migrated = Map::new();
        m.run("requireConfig", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["requireConfig"], json!("required"));
    }

    #[test]
    fn migrate_string_true_to_required() {
        let m = RequireConfigMigration::new();
        let mut migrated = Map::new();
        m.run("requireConfig", &json!("true"), &Map::new(), &mut migrated);
        assert_eq!(migrated["requireConfig"], json!("required"));
    }

    #[test]
    fn leaves_string_optional_unchanged() {
        let m = RequireConfigMigration::new();
        let mut migrated = Map::new();
        m.run("requireConfig", &json!("optional"), &Map::new(), &mut migrated);
        assert!(migrated.is_empty());
    }
}
