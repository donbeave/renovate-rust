use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

fn migrate_datasource(value: &str) -> &str {
    match value {
        "adoptium-java" => "java-version",
        "dotnet" => "dotnet-version",
        "node" => "node-version",
        _ => value,
    }
}

#[derive(Clone, Debug)]
pub struct MatchDatasourcesMigration;

impl Default for MatchDatasourcesMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl MatchDatasourcesMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for MatchDatasourcesMigration {
    fn property_name(&self) -> &str {
        "matchDatasources"
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
            .map(|s| Value::String(migrate_datasource(s).into()))
            .collect();

        migrated_config.insert("matchDatasources".into(), Value::Array(new_value));
    }

    fn box_clone(&self) -> Box<dyn Migration> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Map;
    use serde_json::json;

    use super::MatchDatasourcesMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = MatchDatasourcesMigration::new();
        assert_eq!(m.property_name(), "matchDatasources");
    }

    #[test]
    fn migrates_known_datasources() {
        let m = MatchDatasourcesMigration::new();
        let mut migrated = Map::new();
        m.run(
            "matchDatasources",
            &json!(["adoptium-java", "dotnet", "npm", "node"]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(
            migrated["matchDatasources"],
            json!(["java-version", "dotnet-version", "npm", "node-version"])
        );
    }

    #[test]
    fn filters_empty_and_non_string() {
        let m = MatchDatasourcesMigration::new();
        let mut migrated = Map::new();
        m.run(
            "matchDatasources",
            &json!(["npm", "", null, 123]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["matchDatasources"], json!(["npm"]));
    }
}
