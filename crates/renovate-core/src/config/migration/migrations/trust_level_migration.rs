use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct TrustLevelMigration;

impl Default for TrustLevelMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl TrustLevelMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for TrustLevelMigration {
    fn property_name(&self) -> &str {
        "trustLevel"
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
        if value.as_str() == Some("high") {
            for key in ["allowCustomCrateRegistries", "allowScripts", "exposeAllEnv"] {
                if !migrated_config.contains_key(key) {
                    migrated_config.insert(key.into(), Value::Bool(true));
                }
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

    use super::TrustLevelMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = TrustLevelMigration::new();
        assert_eq!(m.property_name(), "trustLevel");
    }

    #[test]
    fn migrate_high_level() {
        let m = TrustLevelMigration::new();
        let mut migrated = Map::new();
        m.run("trustLevel", &json!("high"), &Map::new(), &mut migrated);
        assert_eq!(migrated["allowCustomCrateRegistries"], json!(true));
        assert_eq!(migrated["allowScripts"], json!(true));
        assert_eq!(migrated["exposeAllEnv"], json!(true));
    }

    #[test]
    fn does_not_rewrite_provided_properties() {
        let m = TrustLevelMigration::new();
        let mut migrated = Map::new();
        migrated.insert("allowCustomCrateRegistries".into(), json!(false));
        migrated.insert("allowScripts".into(), json!(false));
        migrated.insert("exposeAllEnv".into(), json!(false));
        m.run("trustLevel", &json!("high"), &Map::new(), &mut migrated);
        assert_eq!(migrated["allowCustomCrateRegistries"], json!(false));
        assert_eq!(migrated["allowScripts"], json!(false));
        assert_eq!(migrated["exposeAllEnv"], json!(false));
    }

    #[test]
    fn non_high_is_noop() {
        let m = TrustLevelMigration::new();
        let mut migrated = Map::new();
        m.run("trustLevel", &json!("low"), &Map::new(), &mut migrated);
        assert!(migrated.is_empty());
    }
}
