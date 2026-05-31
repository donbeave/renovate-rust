use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct RenovateForkMigration;

impl Default for RenovateForkMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl RenovateForkMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for RenovateForkMigration {
    fn property_name(&self) -> &str {
        "renovateFork"
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
    use serde_json::json;
    use serde_json::Map;

    use super::RenovateForkMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = RenovateForkMigration::new();
        assert_eq!(m.property_name(), "renovateFork");
    }

    #[test]
    fn migrate_true_sets_enabled() {
        let m = RenovateForkMigration::new();
        let mut migrated = Map::new();
        m.run("renovateFork", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["forkProcessing"], json!("enabled"));
    }

    #[test]
    fn migrate_false_sets_disabled() {
        let m = RenovateForkMigration::new();
        let mut migrated = Map::new();
        m.run("renovateFork", &json!(false), &Map::new(), &mut migrated);
        assert_eq!(migrated["forkProcessing"], json!("disabled"));
    }

    #[test]
    fn does_not_overwrite_existing_fork_processing() {
        let m = RenovateForkMigration::new();
        let mut migrated = Map::new();
        migrated.insert("forkProcessing".into(), json!("enabled"));
        m.run("renovateFork", &json!(false), &Map::new(), &mut migrated);
        assert_eq!(migrated["forkProcessing"], json!("enabled"));
    }
}
