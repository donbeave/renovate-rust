use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct MatchStringsMigration;

impl Default for MatchStringsMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl MatchStringsMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for MatchStringsMigration {
    fn property_name(&self) -> &str {
        "matchStrings"
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
            .filter(|s| !s.is_empty())
            .map(|s| s.replace("(?<lookupName>", "(?<packageName>"))
            .map(Value::String)
            .collect();

        migrated_config.insert("matchStrings".into(), Value::Array(new_value));
    }

    fn box_clone(&self) -> Box<dyn Migration> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Map;
    use serde_json::json;

    use super::MatchStringsMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = MatchStringsMigration::new();
        assert_eq!(m.property_name(), "matchStrings");
    }

    #[test]
    fn replaces_lookup_name_with_package_name() {
        let m = MatchStringsMigration::new();
        let mut migrated = Map::new();
        m.run(
            "matchStrings",
            &json!(["(?<lookupName>", "(?<lookupName>(?<lookupName>", "", null]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(
            migrated["matchStrings"],
            json!(["(?<packageName>", "(?<packageName>(?<packageName>"])
        );
    }
}
