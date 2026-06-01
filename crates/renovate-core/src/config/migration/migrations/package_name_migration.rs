use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct PackageNameMigration;

impl Default for PackageNameMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageNameMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for PackageNameMigration {
    fn property_name(&self) -> &str {
        "packageName"
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
        if !migrated_config.contains_key("packageNames") {
            migrated_config.insert("packageNames".into(), Value::Array(vec![value.clone()]));
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

    use super::PackageNameMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = PackageNameMigration::new();
        assert_eq!(m.property_name(), "packageName");
    }

    #[test]
    fn migrates_to_package_names() {
        let m = PackageNameMigration::new();
        let mut migrated = Map::new();
        m.run("packageName", &json!("foo"), &Map::new(), &mut migrated);
        assert_eq!(migrated["packageNames"], json!(["foo"]));
    }

    #[test]
    fn does_not_overwrite_existing() {
        let m = PackageNameMigration::new();
        let mut migrated = Map::new();
        migrated.insert("packageNames".into(), json!(["bar"]));
        m.run("packageName", &json!("foo"), &Map::new(), &mut migrated);
        assert_eq!(migrated["packageNames"], json!(["bar"]));
    }
}
