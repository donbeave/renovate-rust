use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone)]
pub struct RenamePropertyMigration {
    old_name: &'static str,
    new_name: &'static str,
}

impl RenamePropertyMigration {
    pub fn new(old_name: &'static str, new_name: &'static str) -> Self {
        Self { old_name, new_name }
    }
}

impl Migration for RenamePropertyMigration {
    fn property_name(&self) -> &str {
        self.old_name
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
        if !migrated_config.contains_key(self.new_name) {
            migrated_config.insert(self.new_name.to_string(), value.clone());
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

    use super::RenamePropertyMigration;
    use crate::config::migration::Migration;

    fn empty_maps() -> (Map<String, serde_json::Value>, Map<String, serde_json::Value>) {
        (Map::new(), Map::new())
    }

    #[test]
    fn property_name_returns_old_name() {
        let m = RenamePropertyMigration::new("oldProp", "newProp");
        assert_eq!(m.property_name(), "oldProp");
    }

    #[test]
    fn is_deprecated() {
        let m = RenamePropertyMigration::new("a", "b");
        assert!(m.is_deprecated());
    }

    #[test]
    fn run_copies_value_to_new_key() {
        let m = RenamePropertyMigration::new("versionScheme", "versioning");
        let (_, mut migrated) = empty_maps();
        m.run(
            "versionScheme",
            &json!("semver"),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["versioning"], json!("semver"));
    }

    #[test]
    fn run_does_not_overwrite_existing_new_key() {
        let m = RenamePropertyMigration::new("versionScheme", "versioning");
        let mut migrated = Map::new();
        migrated.insert("versioning".into(), json!("npm"));
        m.run(
            "versionScheme",
            &json!("semver"),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["versioning"], json!("npm"));
    }
}
