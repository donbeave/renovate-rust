use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct PostUpdateOptionsMigration;

impl Default for PostUpdateOptionsMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl PostUpdateOptionsMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for PostUpdateOptionsMigration {
    fn property_name(&self) -> &str {
        "postUpdateOptions"
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

        let new_value: Vec<Value> = arr
            .iter()
            .filter_map(|v| v.as_str())
            .filter(|s| !s.is_empty() && *s != "gomodNoMassage")
            .map(|s| Value::String(s.into()))
            .collect();

        migrated_config.insert("postUpdateOptions".into(), Value::Array(new_value));
    }

    fn box_clone(&self) -> Box<dyn Migration> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Map;
    use serde_json::json;

    use super::PostUpdateOptionsMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = PostUpdateOptionsMigration::new();
        assert_eq!(m.property_name(), "postUpdateOptions");
    }

    #[test]
    fn filters_gomod_no_massage() {
        let m = PostUpdateOptionsMigration::new();
        let mut migrated = Map::new();
        m.run(
            "postUpdateOptions",
            &json!(["gomodTidy", "gomodNoMassage"]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["postUpdateOptions"], json!(["gomodTidy"]));
    }
}
