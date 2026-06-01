use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct PackageFilesMigration;

impl Default for PackageFilesMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageFilesMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for PackageFilesMigration {
    fn property_name(&self) -> &str {
        "packageFiles"
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

        let mut file_list: Vec<Value> = Vec::new();
        let mut package_rules: Vec<Value> = match migrated_config.get("packageRules") {
            Some(Value::Array(existing)) => existing.clone(),
            _ => Vec::new(),
        };

        for package_file in arr {
            if let Value::Object(map) = package_file {
                if let Some(Value::String(s)) = map.get("packageFile") {
                    file_list.push(Value::String(s.clone()));
                    let mut rule = map.clone();
                    rule.remove("packageFile");
                    rule.insert("paths".into(), Value::Array(vec![Value::String(s.clone())]));
                    if rule.len() > 1 {
                        package_rules.push(Value::Object(rule));
                    }
                }
            } else if let Value::Array(nested) = package_file {
                for item in nested {
                    if let Some(s) = item.as_str() {
                        file_list.push(Value::String(s.into()));
                    }
                }
            } else if let Some(s) = package_file.as_str() {
                file_list.push(Value::String(s.into()));
            }
        }

        if !file_list.is_empty() {
            migrated_config.insert("includePaths".into(), Value::Array(file_list));
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

    use super::PackageFilesMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = PackageFilesMigration::new();
        assert_eq!(m.property_name(), "packageFiles");
    }

    #[test]
    fn migrates_object_value() {
        let m = PackageFilesMigration::new();
        let mut migrated = Map::new();
        m.run(
            "packageFiles",
            &json!([{ "packageFile": "package.json", "packageRules": [] }]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["includePaths"], json!(["package.json"]));
        assert_eq!(
            migrated["packageRules"],
            json!([{ "paths": ["package.json"], "packageRules": [] }])
        );
    }

    #[test]
    fn migrates_nested_arrays() {
        let m = PackageFilesMigration::new();
        let mut migrated = Map::new();
        m.run(
            "packageFiles",
            &json!([["package.json", "Chart.yaml"]]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(
            migrated["includePaths"],
            json!(["package.json", "Chart.yaml"])
        );
    }

    #[test]
    fn migrates_string_values() {
        let m = PackageFilesMigration::new();
        let mut migrated = Map::new();
        m.run(
            "packageFiles",
            &json!(["package.json"]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["includePaths"], json!(["package.json"]));
    }

    #[test]
    fn concats_with_existing_package_rules() {
        let m = PackageFilesMigration::new();
        let mut migrated = Map::new();
        migrated.insert("packageRules".into(), json!([{ "labels": ["lint"] }]));
        m.run(
            "packageFiles",
            &json!([{ "packageFile": "package.json", "packageRules": [{ "labels": ["breaking"] }] }]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(
            migrated["packageRules"],
            json!([
                { "labels": ["lint"] },
                {
                    "paths": ["package.json"],
                    "packageRules": [{ "labels": ["breaking"] }]
                }
            ])
        );
    }

    #[test]
    fn empty_package_files_no_change() {
        let m = PackageFilesMigration::new();
        let mut migrated = Map::new();
        migrated.insert("includePaths".into(), json!(["package.json"]));
        migrated.insert("packageRules".into(), json!([{ "labels": ["linter"] }]));
        m.run("packageFiles", &json!([]), &Map::new(), &mut migrated);
        assert_eq!(migrated["includePaths"], json!(["package.json"]));
        assert_eq!(migrated["packageRules"], json!([{ "labels": ["linter"] }]));
    }
}
