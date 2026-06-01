use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct AutomergeTypeMigration;

impl Default for AutomergeTypeMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl AutomergeTypeMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for AutomergeTypeMigration {
    fn property_name(&self) -> &str {
        "automergeType"
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        _original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        if let Some(s) = value.as_str()
            && s.starts_with("branch-")
        {
            migrated_config.insert("automergeType".into(), Value::String("branch".into()));
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

    use super::AutomergeTypeMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = AutomergeTypeMigration::new();
        assert_eq!(m.property_name(), "automergeType");
    }

    #[test]
    fn rewrites_branch_hyphen_to_branch() {
        let m = AutomergeTypeMigration::new();
        let mut migrated = Map::new();
        m.run(
            "automergeType",
            &json!("branch-after-hours"),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["automergeType"], json!("branch"));
    }

    #[test]
    fn leaves_pr_unchanged() {
        let m = AutomergeTypeMigration::new();
        let mut migrated = Map::new();
        m.run("automergeType", &json!("pr"), &Map::new(), &mut migrated);
        assert!(migrated.is_empty());
    }
}
