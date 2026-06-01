use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct BranchPrefixMigration;

impl Default for BranchPrefixMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl BranchPrefixMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for BranchPrefixMigration {
    fn property_name(&self) -> &str {
        "branchPrefix"
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        _original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        let Some(s) = value.as_str() else {
            return;
        };
        if let Some(idx) = s.find("{{") {
            let prefix = &s[..idx];
            let additional = &s[idx..];
            migrated_config.insert("branchPrefix".into(), Value::String(prefix.into()));
            if !additional.is_empty() {
                migrated_config.insert(
                    "additionalBranchPrefix".into(),
                    Value::String(additional.into()),
                );
            }
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

    use super::BranchPrefixMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = BranchPrefixMigration::new();
        assert_eq!(m.property_name(), "branchPrefix");
    }

    #[test]
    fn splits_template_at_first_handlebars() {
        let m = BranchPrefixMigration::new();
        let mut migrated = Map::new();
        m.run(
            "branchPrefix",
            &json!("renovate/{{managerBranchPrefix}}"),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["branchPrefix"], json!("renovate/"));
        assert_eq!(
            migrated["additionalBranchPrefix"],
            json!("{{managerBranchPrefix}}")
        );
    }

    #[test]
    fn noop_when_no_template() {
        let m = BranchPrefixMigration::new();
        let mut migrated = Map::new();
        m.run(
            "branchPrefix",
            &json!("renovate/"),
            &Map::new(),
            &mut migrated,
        );
        assert!(migrated.is_empty());
    }
}
