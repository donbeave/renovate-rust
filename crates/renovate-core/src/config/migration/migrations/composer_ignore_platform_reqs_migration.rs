use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct ComposerIgnorePlatformReqsMigration;

impl Default for ComposerIgnorePlatformReqsMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl ComposerIgnorePlatformReqsMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for ComposerIgnorePlatformReqsMigration {
    fn property_name(&self) -> &str {
        "composerIgnorePlatformReqs"
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        _original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        if let Some(flag) = value.as_bool() {
            let new_value = if flag {
                Value::Array(Vec::new())
            } else {
                Value::Null
            };
            migrated_config.insert("composerIgnorePlatformReqs".into(), new_value);
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

    use super::ComposerIgnorePlatformReqsMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = ComposerIgnorePlatformReqsMigration::new();
        assert_eq!(m.property_name(), "composerIgnorePlatformReqs");
    }

    #[test]
    fn migrate_true_to_empty_array() {
        let m = ComposerIgnorePlatformReqsMigration::new();
        let mut migrated = Map::new();
        m.run(
            "composerIgnorePlatformReqs",
            &json!(true),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["composerIgnorePlatformReqs"], json!([]));
    }

    #[test]
    fn migrate_false_to_null() {
        let m = ComposerIgnorePlatformReqsMigration::new();
        let mut migrated = Map::new();
        m.run(
            "composerIgnorePlatformReqs",
            &json!(false),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["composerIgnorePlatformReqs"], json!(null));
    }
}
