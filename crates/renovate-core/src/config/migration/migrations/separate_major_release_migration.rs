use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct SeparateMajorReleaseMigration;

impl Default for SeparateMajorReleaseMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl SeparateMajorReleaseMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for SeparateMajorReleaseMigration {
    fn property_name(&self) -> &str {
        "separateMajorReleases"
    }

    fn is_deprecated(&self) -> bool {
        false
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        _original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        if !migrated_config.contains_key("separateMajorMinor") {
            migrated_config.insert("separateMajorMinor".into(), value.clone());
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

    use super::SeparateMajorReleaseMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = SeparateMajorReleaseMigration::new();
        assert_eq!(m.property_name(), "separateMajorReleases");
    }

    #[test]
    fn migrate_true() {
        let m = SeparateMajorReleaseMigration::new();
        let mut migrated = Map::new();
        m.run(
            "separateMajorReleases",
            &json!(true),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["separateMajorMinor"], json!(true));
    }

    #[test]
    fn does_not_overwrite_existing_separate_major_minor() {
        let m = SeparateMajorReleaseMigration::new();
        let mut migrated = Map::new();
        migrated.insert("separateMajorMinor".into(), json!(false));
        m.run(
            "separateMajorReleases",
            &json!(true),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["separateMajorMinor"], json!(false));
    }
}
