use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct SemanticCommitsMigration;

impl Default for SemanticCommitsMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticCommitsMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for SemanticCommitsMigration {
    fn property_name(&self) -> &str {
        "semanticCommits"
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        _original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        let new_value = if let Some(b) = value.as_bool() {
            if b { "enabled" } else { "disabled" }
        } else if let Some(s) = value.as_str() {
            if s == "enabled" || s == "disabled" {
                s
            } else {
                "auto"
            }
        } else {
            "auto"
        };

        migrated_config.insert("semanticCommits".into(), Value::String(new_value.into()));
    }

    fn box_clone(&self) -> Box<dyn Migration> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Map;
    use serde_json::json;

    use super::SemanticCommitsMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = SemanticCommitsMigration::new();
        assert_eq!(m.property_name(), "semanticCommits");
    }

    #[test]
    fn migrates_true() {
        let m = SemanticCommitsMigration::new();
        let mut migrated = Map::new();
        m.run("semanticCommits", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(migrated["semanticCommits"], json!("enabled"));
    }

    #[test]
    fn migrates_false() {
        let m = SemanticCommitsMigration::new();
        let mut migrated = Map::new();
        m.run("semanticCommits", &json!(false), &Map::new(), &mut migrated);
        assert_eq!(migrated["semanticCommits"], json!("disabled"));
    }

    #[test]
    fn migrates_null_to_auto() {
        let m = SemanticCommitsMigration::new();
        let mut migrated = Map::new();
        m.run("semanticCommits", &json!(null), &Map::new(), &mut migrated);
        assert_eq!(migrated["semanticCommits"], json!("auto"));
    }

    #[test]
    fn migrates_random_string_to_auto() {
        let m = SemanticCommitsMigration::new();
        let mut migrated = Map::new();
        m.run(
            "semanticCommits",
            &json!("test"),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["semanticCommits"], json!("auto"));
    }

    #[test]
    fn leaves_enabled_unchanged() {
        let m = SemanticCommitsMigration::new();
        let mut migrated = Map::new();
        m.run(
            "semanticCommits",
            &json!("enabled"),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["semanticCommits"], json!("enabled"));
    }

    #[test]
    fn leaves_disabled_unchanged() {
        let m = SemanticCommitsMigration::new();
        let mut migrated = Map::new();
        m.run(
            "semanticCommits",
            &json!("disabled"),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["semanticCommits"], json!("disabled"));
    }
}
