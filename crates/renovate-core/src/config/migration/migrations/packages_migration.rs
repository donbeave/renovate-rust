use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct PackagesMigration;

impl Default for PackagesMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl PackagesMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for PackagesMigration {
    fn property_name(&self) -> &str {
        "packages"
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

    use super::PackagesMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = PackagesMigration::new();
        assert_eq!(m.property_name(), "packages");
    }

    #[test]
    fn migrates_to_package_rules() {
        let m = PackagesMigration::new();
        let mut migrated = Map::new();
        m.run(
            "packages",
            &json!([{ "packageNames": ["foo"] }]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(
            migrated["packageRules"],
            json!([{ "packageNames": ["foo"] }])
        );
    }

    #[test]
    fn concats_with_existing_package_rules() {
        let m = PackagesMigration::new();
        let mut migrated = Map::new();
        migrated.insert("packageRules".into(), json!([{ "a": 1 }]));
        m.run("packages", &json!([{ "b": 2 }]), &Map::new(), &mut migrated);
        assert_eq!(migrated["packageRules"], json!([{ "a": 1 }, { "b": 2 }]));
    }
}
