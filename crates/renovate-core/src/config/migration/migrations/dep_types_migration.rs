use regex::Regex;
use serde_json::Map;
use serde_json::Value;
use std::sync::LazyLock;

use crate::config::migration::Migration;

static PROPERTY_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:(?:d|devD|optionalD|peerD)ependencies|engines|depTypes)$").unwrap()
});

#[derive(Clone, Debug)]
pub struct DepTypesMigration;

impl Default for DepTypesMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl DepTypesMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for DepTypesMigration {
    fn property_name(&self) -> &str {
        "depTypes"
    }

    fn is_deprecated(&self) -> bool {
        true
    }

    fn matches(&self, key: &str) -> bool {
        PROPERTY_RE.is_match(key)
    }

    fn run(
        &self,
        key: &str,
        value: &Value,
        _original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        let mut package_rules: Vec<Value> = match migrated_config.get("packageRules") {
            Some(Value::Array(arr)) => arr.clone(),
            _ => Vec::new(),
        };

        match value {
            Value::Object(map) if !map.is_empty() => {
                let mut rule = Map::new();
                rule.insert(
                    "matchDepTypes".into(),
                    Value::Array(vec![Value::String(key.into())]),
                );
                for (k, v) in map {
                    rule.insert(k.clone(), v.clone());
                }
                package_rules.push(Value::Object(rule));
            }
            Value::Array(arr) => {
                for item in arr {
                    let Value::Object(map) = item else {
                        continue;
                    };
                    let dep_type_name = map.get("depType").and_then(|v| v.as_str());
                    let Some(dep_type_name) = dep_type_name else {
                        continue;
                    };
                    let mut rule = map.clone();
                    rule.remove("depType");
                    rule.insert(
                        "matchDepTypes".into(),
                        Value::Array(vec![Value::String(dep_type_name.into())]),
                    );
                    package_rules.push(Value::Object(rule));
                }
            }
            _ => {}
        }

        if !package_rules.is_empty() {
            migrated_config.insert("packageRules".into(), Value::Array(package_rules));
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

    use super::DepTypesMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = DepTypesMigration::new();
        assert_eq!(m.property_name(), "depTypes");
    }

    #[test]
    fn matches_dependencies() {
        let m = DepTypesMigration::new();
        assert!(m.matches("dependencies"));
        assert!(m.matches("devDependencies"));
        assert!(m.matches("optionalDependencies"));
        assert!(m.matches("peerDependencies"));
        assert!(m.matches("engines"));
        assert!(m.matches("depTypes"));
        assert!(!m.matches("other"));
    }

    #[test]
    fn migrate_object_value() {
        let m = DepTypesMigration::new();
        let mut migrated = Map::new();
        m.run(
            "dependencies",
            &json!({ "versionStrategy": "widen" }),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(
            migrated["packageRules"],
            json!([{ "matchDepTypes": ["dependencies"], "versionStrategy": "widen" }])
        );
    }

    #[test]
    fn migrate_array_value() {
        let m = DepTypesMigration::new();
        let mut migrated = Map::new();
        m.run(
            "depTypes",
            &json!([{ "depType": "optionalDependencies", "respectLatest": false }]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(
            migrated["packageRules"],
            json!([{ "matchDepTypes": ["optionalDependencies"], "respectLatest": false }])
        );
    }

    #[test]
    fn ignores_string_elements_in_array() {
        let m = DepTypesMigration::new();
        let mut migrated = Map::new();
        m.run(
            "depTypes",
            &json!(["dependencies"]),
            &Map::new(),
            &mut migrated,
        );
        assert!(migrated.get("packageRules").is_none());
    }

    #[test]
    fn appends_to_existing_package_rules() {
        let m = DepTypesMigration::new();
        let mut migrated = Map::new();
        migrated.insert("packageRules".into(), json!([{ "packageNames": ["foo"] }]));
        m.run(
            "dependencies",
            &json!({ "versionStrategy": "widen" }),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(
            migrated["packageRules"],
            json!([
                { "packageNames": ["foo"] },
                { "matchDepTypes": ["dependencies"], "versionStrategy": "widen" }
            ])
        );
    }
}
