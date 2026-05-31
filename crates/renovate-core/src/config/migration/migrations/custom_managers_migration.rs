use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct CustomManagersMigration;

impl Default for CustomManagersMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl CustomManagersMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for CustomManagersMigration {
    fn property_name(&self) -> &str {
        "customManagers"
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
        if arr.is_empty() {
            return;
        }

        let custom_managers: Vec<Value> = arr
            .iter()
            .map(|mgr| {
                let Value::Object(map) = mgr else {
                    return mgr.clone();
                };
                if map.contains_key("customType") {
                    return mgr.clone();
                }
                let new_map = map.clone();
                // Insert customType at the top by creating a new ordered map
                let mut ordered = Map::new();
                ordered.insert("customType".into(), Value::String("regex".into()));
                for (k, v) in new_map {
                    ordered.insert(k, v);
                }
                Value::Object(ordered)
            })
            .collect();

        migrated_config.insert("customManagers".into(), Value::Array(custom_managers));
    }

    fn box_clone(&self) -> Box<dyn Migration> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use serde_json::Map;

    use super::CustomManagersMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = CustomManagersMigration::new();
        assert_eq!(m.property_name(), "customManagers");
    }

    #[test]
    fn adds_custom_type_regex_when_missing() {
        let m = CustomManagersMigration::new();
        let mut migrated = Map::new();
        m.run(
            "customManagers",
            &json!([{ "datasourceTemplate": "maven" }]),
            &Map::new(),
            &mut migrated,
        );
        let arr = migrated["customManagers"].as_array().unwrap();
        assert_eq!(arr[0]["customType"], json!("regex"));
        assert_eq!(arr[0]["datasourceTemplate"], json!("maven"));
    }

    #[test]
    fn preserves_existing_custom_type() {
        let m = CustomManagersMigration::new();
        let mut migrated = Map::new();
        m.run(
            "customManagers",
            &json!([{ "customType": "regex", "datasourceTemplate": "maven" }]),
            &Map::new(),
            &mut migrated,
        );
        let arr = migrated["customManagers"].as_array().unwrap();
        assert_eq!(arr[0]["customType"], json!("regex"));
    }
}
