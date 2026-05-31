use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct RebaseStalePrsMigration;

impl Default for RebaseStalePrsMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl RebaseStalePrsMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for RebaseStalePrsMigration {
    fn property_name(&self) -> &str {
        "rebaseStalePrs"
    }

    fn is_deprecated(&self) -> bool {
        true
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        let rebase_conflicted_prs = original_config.get("rebaseConflictedPrs");
        if rebase_conflicted_prs == Some(&Value::Bool(false)) {
            return;
        }

        let new_value = match value {
            Value::Bool(true) => Some("behind-base-branch"),
            Value::Bool(false) => Some("conflicted"),
            Value::Null => Some("auto"),
            _ => None,
        };

        if let Some(v) = new_value && !migrated_config.contains_key("rebaseWhen") {
            migrated_config.insert("rebaseWhen".into(), Value::String(v.into()));
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

    use super::RebaseStalePrsMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = RebaseStalePrsMigration::new();
        assert_eq!(m.property_name(), "rebaseStalePrs");
    }

    #[test]
    fn migrate_true() {
        let m = RebaseStalePrsMigration::new();
        let mut migrated = Map::new();
        m.run("rebaseStalePrs", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["rebaseWhen"], json!("behind-base-branch"));
    }

    #[test]
    fn migrate_false() {
        let m = RebaseStalePrsMigration::new();
        let mut migrated = Map::new();
        m.run("rebaseStalePrs", &json!(false), &Map::new(), &mut migrated);
        assert_eq!(migrated["rebaseWhen"], json!("conflicted"));
    }

    #[test]
    fn migrate_null() {
        let m = RebaseStalePrsMigration::new();
        let mut migrated = Map::new();
        m.run("rebaseStalePrs", &json!(null), &Map::new(), &mut migrated);
        assert_eq!(migrated["rebaseWhen"], json!("auto"));
    }

    #[test]
    fn skips_when_rebase_conflicted_prs_is_false() {
        let m = RebaseStalePrsMigration::new();
        let mut original = Map::new();
        original.insert("rebaseConflictedPrs".into(), json!(false));
        let mut migrated = Map::new();
        m.run("rebaseStalePrs", &json!(true), &original, &mut migrated);
        assert!(migrated.is_empty());
    }

    #[test]
    fn does_not_overwrite_existing_rebase_when() {
        let m = RebaseStalePrsMigration::new();
        let mut migrated = Map::new();
        migrated.insert("rebaseWhen".into(), json!("never"));
        m.run("rebaseStalePrs", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["rebaseWhen"], json!("never"));
    }
}
