use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct AutomergePatchMigration;

impl Default for AutomergePatchMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl AutomergePatchMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for AutomergePatchMigration {
    fn property_name(&self) -> &str {
        "automergePatch"
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
            .entry("patch".to_owned())
            .or_insert_with(|| Value::Object(Map::new()));
        if let Some(Value::Object(obj)) = migrated_config.get_mut("patch") {
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

    use super::AutomergePatchMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = AutomergePatchMigration::new();
        assert_eq!(m.property_name(), "automergePatch");
    }

    #[test]
    fn migrate_true_sets_patch_automerge() {
        let m = AutomergePatchMigration::new();
        let mut migrated = Map::new();
        m.run("automergePatch", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["patch"]["automerge"], json!(true));
    }

    #[test]
    fn migrate_false_sets_patch_automerge_false() {
        let m = AutomergePatchMigration::new();
        let mut migrated = Map::new();
        m.run("automergePatch", &json!(false), &Map::new(), &mut migrated);
        assert_eq!(migrated["patch"]["automerge"], json!(false));
    }
}
