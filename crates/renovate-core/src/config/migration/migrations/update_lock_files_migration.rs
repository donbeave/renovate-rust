use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct UpdateLockFilesMigration;

impl Default for UpdateLockFilesMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl UpdateLockFilesMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for UpdateLockFilesMigration {
    fn property_name(&self) -> &str {
        "updateLockFiles"
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
        if value.as_bool() == Some(false) && !migrated_config.contains_key("skipArtifactsUpdate") {
            migrated_config.insert("skipArtifactsUpdate".into(), Value::Bool(true));
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

    use super::UpdateLockFilesMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = UpdateLockFilesMigration::new();
        assert_eq!(m.property_name(), "updateLockFiles");
    }

    #[test]
    fn migrate_false() {
        let m = UpdateLockFilesMigration::new();
        let mut migrated = Map::new();
        m.run("updateLockFiles", &json!(false), &Map::new(), &mut migrated);
        assert_eq!(migrated["skipArtifactsUpdate"], json!(true));
    }

    #[test]
    fn migrate_true_is_noop() {
        let m = UpdateLockFilesMigration::new();
        let mut migrated = Map::new();
        m.run("updateLockFiles", &json!(true), &Map::new(), &mut migrated);
        assert!(migrated.is_empty());
    }

    #[test]
    fn does_not_overwrite_existing_skip_artifacts_update() {
        let m = UpdateLockFilesMigration::new();
        let mut migrated = Map::new();
        migrated.insert("skipArtifactsUpdate".into(), json!(false));
        m.run("updateLockFiles", &json!(false), &Map::new(), &mut migrated);
        assert_eq!(migrated["skipArtifactsUpdate"], json!(false));
    }
}
