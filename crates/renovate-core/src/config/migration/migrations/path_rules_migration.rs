use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct PathRulesMigration;

impl Default for PathRulesMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl PathRulesMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for PathRulesMigration {
    fn property_name(&self) -> &str {
        "pathRules"
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
        let Value::Array(arr) = value else {
            return;
        };

        let mut package_rules: Vec<Value> = match migrated_config.get("packageRules") {
            Some(Value::Array(existing)) => existing.clone(),
            _ => Vec::new(),
        };

        package_rules.extend(arr.iter().cloned());
        migrated_config.insert("packageRules".into(), Value::Array(package_rules));
    }

    fn box_clone(&self) -> Box<dyn Migration> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Map;
    use serde_json::json;

    use super::PathRulesMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = PathRulesMigration::new();
        assert_eq!(m.property_name(), "pathRules");
    }

    #[test]
    fn migrates_to_package_rules() {
        let m = PathRulesMigration::new();
        let mut migrated = Map::new();
        m.run(
            "pathRules",
            &json!([{ "paths": ["examples/**"], "extends": ["foo"] }]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(
            migrated["packageRules"],
            json!([{ "paths": ["examples/**"], "extends": ["foo"] }])
        );
    }

    #[test]
    fn concats_with_existing_package_rules() {
        let m = PathRulesMigration::new();
        let mut migrated = Map::new();
        migrated.insert(
            "packageRules".into(),
            json!([{ "packageNames": ["guava"], "versionScheme": "maven" }]),
        );
        m.run(
            "pathRules",
            &json!([{ "paths": ["examples/**"], "extends": ["foo"] }]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(
            migrated["packageRules"],
            json!([
                { "packageNames": ["guava"], "versionScheme": "maven" },
                { "paths": ["examples/**"], "extends": ["foo"] }
            ])
        );
    }

    #[test]
    fn ignores_non_array() {
        let m = PathRulesMigration::new();
        let mut migrated = Map::new();
        m.run("pathRules", &json!("test"), &Map::new(), &mut migrated);
        assert!(migrated.get("packageRules").is_none());
    }
}
