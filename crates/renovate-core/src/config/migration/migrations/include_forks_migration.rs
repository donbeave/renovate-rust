use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct IncludeForksMigration;

impl Default for IncludeForksMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl IncludeForksMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for IncludeForksMigration {
    fn property_name(&self) -> &str {
        "includeForks"
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
        if let Some(flag) = value.as_bool() {
            let new_value = if flag { "enabled" } else { "disabled" };
            if !migrated_config.contains_key("forkProcessing") {
                migrated_config.insert("forkProcessing".into(), Value::String(new_value.into()));
            }
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

    use super::IncludeForksMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = IncludeForksMigration::new();
        assert_eq!(m.property_name(), "includeForks");
    }

    #[test]
    fn migrate_true_sets_enabled() {
        let m = IncludeForksMigration::new();
        let mut migrated = Map::new();
        m.run("includeForks", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["forkProcessing"], json!("enabled"));
    }

    #[test]
    fn migrate_false_sets_disabled() {
        let m = IncludeForksMigration::new();
        let mut migrated = Map::new();
        m.run("includeForks", &json!(false), &Map::new(), &mut migrated);
        assert_eq!(migrated["forkProcessing"], json!("disabled"));
    }

    #[test]
    fn does_not_overwrite_existing_fork_processing() {
        let m = IncludeForksMigration::new();
        let mut migrated = Map::new();
        migrated.insert("forkProcessing".into(), json!("enabled"));
        m.run("includeForks", &json!(false), &Map::new(), &mut migrated);
        assert_eq!(migrated["forkProcessing"], json!("enabled"));
    }
}
