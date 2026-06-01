use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct PlatformCommitMigration;

impl Default for PlatformCommitMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformCommitMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for PlatformCommitMigration {
    fn property_name(&self) -> &str {
        "platformCommit"
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        _original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        if let Some(flag) = value.as_bool() {
            let new_value = if flag { "enabled" } else { "disabled" };
            migrated_config.insert("platformCommit".into(), Value::String(new_value.into()));
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

    use super::PlatformCommitMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = PlatformCommitMigration::new();
        assert_eq!(m.property_name(), "platformCommit");
    }

    #[test]
    fn migrate_true_sets_enabled() {
        let m = PlatformCommitMigration::new();
        let mut migrated = Map::new();
        m.run("platformCommit", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["platformCommit"], json!("enabled"));
    }

    #[test]
    fn migrate_false_sets_disabled() {
        let m = PlatformCommitMigration::new();
        let mut migrated = Map::new();
        m.run("platformCommit", &json!(false), &Map::new(), &mut migrated);
        assert_eq!(migrated["platformCommit"], json!("disabled"));
    }

    #[test]
    fn leaves_string_unchanged() {
        let m = PlatformCommitMigration::new();
        let mut migrated = Map::new();
        m.run("platformCommit", &json!("auto"), &Map::new(), &mut migrated);
        assert!(migrated.is_empty());
    }
}
