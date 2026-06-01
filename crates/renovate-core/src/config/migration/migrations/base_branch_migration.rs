use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct BaseBranchMigration;

impl Default for BaseBranchMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl BaseBranchMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for BaseBranchMigration {
    fn property_name(&self) -> &str {
        "baseBranch"
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
        let mut patterns: Vec<Value> = match migrated_config.get("baseBranchPatterns") {
            Some(Value::Array(arr)) => arr.clone(),
            _ => Vec::new(),
        };

        match value {
            Value::Array(arr) => {
                patterns.extend(arr.iter().cloned());
            }
            Value::String(s) => {
                patterns.push(Value::String(s.clone()));
            }
            _ => return,
        }

        if !patterns.is_empty() {
            migrated_config.insert("baseBranchPatterns".into(), Value::Array(patterns));
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

    use super::BaseBranchMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = BaseBranchMigration::new();
        assert_eq!(m.property_name(), "baseBranch");
    }

    #[test]
    fn migrate_string_appends_to_patterns() {
        let m = BaseBranchMigration::new();
        let mut migrated = Map::new();
        m.run("baseBranch", &json!("main"), &Map::new(), &mut migrated);
        assert_eq!(migrated["baseBranchPatterns"], json!(["main"]));
    }

    #[test]
    fn migrate_array_appends_to_patterns() {
        let m = BaseBranchMigration::new();
        let mut migrated = Map::new();
        m.run(
            "baseBranch",
            &json!(["main", "develop"]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["baseBranchPatterns"], json!(["main", "develop"]));
    }

    #[test]
    fn preserves_existing_patterns() {
        let m = BaseBranchMigration::new();
        let mut migrated = Map::new();
        migrated.insert("baseBranchPatterns".into(), json!(["master"]));
        m.run("baseBranch", &json!("main"), &Map::new(), &mut migrated);
        assert_eq!(migrated["baseBranchPatterns"], json!(["master", "main"]));
    }
}
