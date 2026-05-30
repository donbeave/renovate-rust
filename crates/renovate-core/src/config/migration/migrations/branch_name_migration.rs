use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct BranchNameMigration;

impl Default for BranchNameMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl BranchNameMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for BranchNameMigration {
    fn property_name(&self) -> &str {
        "branchName"
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        _original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        if let Some(s) = value.as_str()
            && s.contains("{{managerBranchPrefix}}") {
                let replaced = s.replace(
                    "{{managerBranchPrefix}}",
                    "{{additionalBranchPrefix}}",
                );
                migrated_config.insert("branchName".into(), Value::String(replaced));
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

    use super::BranchNameMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = BranchNameMigration::new();
        assert_eq!(m.property_name(), "branchName");
    }

    #[test]
    fn replaces_manager_branch_prefix() {
        let m = BranchNameMigration::new();
        let mut migrated = Map::new();
        m.run(
            "branchName",
            &json!("{{managerBranchPrefix}}foo"),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(
            migrated["branchName"],
            json!("{{additionalBranchPrefix}}foo")
        );
    }

    #[test]
    fn no_replacement_when_template_absent() {
        let m = BranchNameMigration::new();
        let mut migrated = Map::new();
        m.run(
            "branchName",
            &json!("renovate/foo"),
            &Map::new(),
            &mut migrated,
        );
        assert!(migrated.get("branchName").is_none());
    }

    #[test]
    fn non_string_value_is_noop() {
        let m = BranchNameMigration::new();
        let mut migrated = Map::new();
        m.run("branchName", &json!(42), &Map::new(), &mut migrated);
        assert!(migrated.is_empty());
    }
}
