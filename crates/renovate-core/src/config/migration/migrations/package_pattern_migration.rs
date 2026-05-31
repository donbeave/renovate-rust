use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct PackagePatternMigration;

impl Default for PackagePatternMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl PackagePatternMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for PackagePatternMigration {
    fn property_name(&self) -> &str {
        "packagePattern"
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
        if !migrated_config.contains_key("packagePatterns") {
            migrated_config.insert("packagePatterns".into(), Value::Array(vec![value.clone()]));
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

    use super::PackagePatternMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = PackagePatternMigration::new();
        assert_eq!(m.property_name(), "packagePattern");
    }

    #[test]
    fn migrates_to_package_patterns() {
        let m = PackagePatternMigration::new();
        let mut migrated = Map::new();
        m.run("packagePattern", &json!("foo"), &Map::new(), &mut migrated);
        assert_eq!(migrated["packagePatterns"], json!(["foo"]));
    }

    #[test]
    fn does_not_overwrite_existing() {
        let m = PackagePatternMigration::new();
        let mut migrated = Map::new();
        migrated.insert("packagePatterns".into(), json!(["bar"]));
        m.run("packagePattern", &json!("foo"), &Map::new(), &mut migrated);
        assert_eq!(migrated["packagePatterns"], json!(["bar"]));
    }
}
