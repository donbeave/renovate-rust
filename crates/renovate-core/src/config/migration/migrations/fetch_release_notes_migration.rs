use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct FetchReleaseNotesMigration;

impl Default for FetchReleaseNotesMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl FetchReleaseNotesMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for FetchReleaseNotesMigration {
    fn property_name(&self) -> &str {
        "fetchReleaseNotes"
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
        let new_value = match value {
            Value::Bool(true) => Value::String("pr".into()),
            Value::Bool(false) => Value::String("off".into()),
            _ => value.clone(),
        };

        if !migrated_config.contains_key("fetchChangeLogs") {
            migrated_config.insert("fetchChangeLogs".into(), new_value);
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

    use super::FetchReleaseNotesMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = FetchReleaseNotesMigration::new();
        assert_eq!(m.property_name(), "fetchReleaseNotes");
    }

    #[test]
    fn migrate_false_to_off() {
        let m = FetchReleaseNotesMigration::new();
        let mut migrated = Map::new();
        m.run("fetchReleaseNotes", &json!(false), &Map::new(), &mut migrated);
        assert_eq!(migrated["fetchChangeLogs"], json!("off"));
    }

    #[test]
    fn migrate_true_to_pr() {
        let m = FetchReleaseNotesMigration::new();
        let mut migrated = Map::new();
        m.run("fetchReleaseNotes", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["fetchChangeLogs"], json!("pr"));
    }

    #[test]
    fn leaves_string_unchanged() {
        let m = FetchReleaseNotesMigration::new();
        let mut migrated = Map::new();
        m.run("fetchReleaseNotes", &json!("branch"), &Map::new(), &mut migrated);
        assert_eq!(migrated["fetchChangeLogs"], json!("branch"));
    }

    #[test]
    fn does_not_overwrite_existing_fetch_change_logs() {
        let m = FetchReleaseNotesMigration::new();
        let mut migrated = Map::new();
        migrated.insert("fetchChangeLogs".into(), json!("off"));
        m.run("fetchReleaseNotes", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["fetchChangeLogs"], json!("off"));
    }
}
