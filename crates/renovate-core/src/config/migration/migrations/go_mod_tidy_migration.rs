use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct GoModTidyMigration;

impl Default for GoModTidyMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl GoModTidyMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for GoModTidyMigration {
    fn property_name(&self) -> &str {
        "gomodTidy"
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
        if value.as_bool() != Some(true) {
            return;
        }

        let mut options: Vec<Value> = match migrated_config.get("postUpdateOptions") {
            Some(Value::Array(arr)) => arr.clone(),
            _ => Vec::new(),
        };

        if !options.iter().any(|v| v.as_str() == Some("gomodTidy")) {
            options.push(Value::String("gomodTidy".into()));
        }

        migrated_config.insert("postUpdateOptions".into(), Value::Array(options));
    }

    fn box_clone(&self) -> Box<dyn Migration> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Map;
    use serde_json::json;

    use super::GoModTidyMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = GoModTidyMigration::new();
        assert_eq!(m.property_name(), "gomodTidy");
    }

    #[test]
    fn migrate_true_adds_gomod_tidy() {
        let m = GoModTidyMigration::new();
        let mut migrated = Map::new();
        m.run("gomodTidy", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["postUpdateOptions"], json!(["gomodTidy"]));
    }

    #[test]
    fn preserves_existing_options() {
        let m = GoModTidyMigration::new();
        let mut migrated = Map::new();
        migrated.insert("postUpdateOptions".into(), json!(["npmDedupe"]));
        m.run("gomodTidy", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(
            migrated["postUpdateOptions"],
            json!(["npmDedupe", "gomodTidy"])
        );
    }

    #[test]
    fn no_duplicate_gomod_tidy() {
        let m = GoModTidyMigration::new();
        let mut migrated = Map::new();
        migrated.insert("postUpdateOptions".into(), json!(["gomodTidy"]));
        m.run("gomodTidy", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["postUpdateOptions"], json!(["gomodTidy"]));
    }

    #[test]
    fn migrate_false_is_noop() {
        let m = GoModTidyMigration::new();
        let mut migrated = Map::new();
        m.run("gomodTidy", &json!(false), &Map::new(), &mut migrated);
        assert!(migrated.is_empty());
    }
}
