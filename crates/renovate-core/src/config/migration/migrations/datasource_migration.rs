use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct DatasourceMigration;

impl Default for DatasourceMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl DatasourceMigration {
    pub fn new() -> Self {
        Self
    }
}

fn migrate_datasource(value: &str) -> &str {
    match value {
        "adoptium-java" => "java-version",
        "dotnet" => "dotnet-version",
        "node" => "node-version",
        _ => value,
    }
}

impl Migration for DatasourceMigration {
    fn property_name(&self) -> &str {
        "datasource"
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        _original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        if let Some(s) = value.as_str() {
            let new_value = migrate_datasource(s);
            if new_value != s {
                migrated_config.insert("datasource".into(), Value::String(new_value.into()));
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

    use super::DatasourceMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = DatasourceMigration::new();
        assert_eq!(m.property_name(), "datasource");
    }

    #[test]
    fn migrates_adoptium_java() {
        let m = DatasourceMigration::new();
        let mut migrated = Map::new();
        m.run(
            "datasource",
            &json!("adoptium-java"),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["datasource"], json!("java-version"));
    }

    #[test]
    fn migrates_dotnet() {
        let m = DatasourceMigration::new();
        let mut migrated = Map::new();
        m.run("datasource", &json!("dotnet"), &Map::new(), &mut migrated);
        assert_eq!(migrated["datasource"], json!("dotnet-version"));
    }

    #[test]
    fn leaves_npm_unchanged() {
        let m = DatasourceMigration::new();
        let mut migrated = Map::new();
        m.run("datasource", &json!("npm"), &Map::new(), &mut migrated);
        assert!(migrated.is_empty());
    }
}
