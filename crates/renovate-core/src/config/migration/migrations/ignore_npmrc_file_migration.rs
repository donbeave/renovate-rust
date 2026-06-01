use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct IgnoreNpmrcFileMigration;

impl Default for IgnoreNpmrcFileMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl IgnoreNpmrcFileMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for IgnoreNpmrcFileMigration {
    fn property_name(&self) -> &str {
        "ignoreNpmrcFile"
    }

    fn is_deprecated(&self) -> bool {
        true
    }

    fn run(
        &self,
        _key: &str,
        _value: &Value,
        original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        let npmrc = migrated_config
            .get("npmrc")
            .or_else(|| original_config.get("npmrc"));
        if !matches!(npmrc, Some(Value::String(_))) {
            migrated_config.insert("npmrc".into(), Value::String("".into()));
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

    use super::IgnoreNpmrcFileMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = IgnoreNpmrcFileMigration::new();
        assert_eq!(m.property_name(), "ignoreNpmrcFile");
    }

    #[test]
    fn init_npmrc_field() {
        let m = IgnoreNpmrcFileMigration::new();
        let mut migrated = Map::new();
        m.run("ignoreNpmrcFile", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["npmrc"], json!(""));
    }

    #[test]
    fn does_not_change_existing_string_npmrc() {
        let m = IgnoreNpmrcFileMigration::new();
        let mut original = Map::new();
        original.insert("npmrc".into(), json!(""));
        let mut migrated = Map::new();
        migrated.insert("npmrc".into(), json!(""));
        m.run("ignoreNpmrcFile", &json!(true), &original, &mut migrated);
        assert_eq!(migrated["npmrc"], json!(""));
    }

    #[test]
    fn changes_non_string_npmrc() {
        let m = IgnoreNpmrcFileMigration::new();
        let mut original = Map::new();
        original.insert("npmrc".into(), json!(true));
        let mut migrated = Map::new();
        migrated.insert("npmrc".into(), json!(true));
        m.run("ignoreNpmrcFile", &json!(true), &original, &mut migrated);
        assert_eq!(migrated["npmrc"], json!(""));
    }
}
