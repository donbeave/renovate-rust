use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct SeparateMultipleMajorMigration;

impl Default for SeparateMultipleMajorMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl SeparateMultipleMajorMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for SeparateMultipleMajorMigration {
    fn property_name(&self) -> &str {
        "separateMultipleMajor"
    }

    fn is_deprecated(&self) -> bool {
        false
    }

    fn run(
        &self,
        _key: &str,
        _value: &Value,
        original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        if original_config.contains_key("separateMajorReleases") {
            migrated_config.remove("separateMultipleMajor");
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

    use super::SeparateMultipleMajorMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = SeparateMultipleMajorMigration::new();
        assert_eq!(m.property_name(), "separateMultipleMajor");
    }

    #[test]
    fn removes_when_separate_major_releases_exists() {
        let m = SeparateMultipleMajorMigration::new();
        let mut original = Map::new();
        original.insert("separateMajorReleases".into(), json!(true));
        let mut migrated = Map::new();
        migrated.insert("separateMultipleMajor".into(), json!(true));
        m.run(
            "separateMultipleMajor",
            &json!(true),
            &original,
            &mut migrated,
        );
        assert!(migrated.get("separateMultipleMajor").is_none());
    }

    #[test]
    fn keeps_when_separate_major_releases_missing() {
        let m = SeparateMultipleMajorMigration::new();
        let mut migrated = Map::new();
        migrated.insert("separateMultipleMajor".into(), json!(true));
        m.run(
            "separateMultipleMajor",
            &json!(true),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["separateMultipleMajor"], json!(true));
    }
}
