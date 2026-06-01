use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct FileMatchMigration;

impl Default for FileMatchMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl FileMatchMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for FileMatchMigration {
    fn property_name(&self) -> &str {
        "fileMatch"
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
        let file_match: Vec<String> = if let Some(s) = value.as_str() {
            vec![s.into()]
        } else if let Some(arr) = value.as_array() {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.into())
                .collect()
        } else {
            return;
        };

        let mut manager_file_patterns: Vec<Value> = match migrated_config.get("managerFilePatterns")
        {
            Some(Value::Array(existing)) => existing.clone(),
            _ => Vec::new(),
        };

        for s in file_match {
            manager_file_patterns.push(Value::String(format!("/{s}/")));
        }

        migrated_config.insert(
            "managerFilePatterns".into(),
            Value::Array(manager_file_patterns),
        );
    }

    fn box_clone(&self) -> Box<dyn Migration> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Map;
    use serde_json::json;

    use super::FileMatchMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = FileMatchMigration::new();
        assert_eq!(m.property_name(), "fileMatch");
    }

    #[test]
    fn migrates_string() {
        let m = FileMatchMigration::new();
        let mut migrated = Map::new();
        m.run("fileMatch", &json!("filename"), &Map::new(), &mut migrated);
        assert_eq!(migrated["managerFilePatterns"], json!(["/filename/"]));
    }

    #[test]
    fn migrates_array() {
        let m = FileMatchMigration::new();
        let mut migrated = Map::new();
        m.run(
            "fileMatch",
            &json!(["filename1", "filename2"]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(
            migrated["managerFilePatterns"],
            json!(["/filename1/", "/filename2/"])
        );
    }

    #[test]
    fn concats_to_existing() {
        let m = FileMatchMigration::new();
        let mut migrated = Map::new();
        migrated.insert("managerFilePatterns".into(), json!(["filename3"]));
        m.run(
            "fileMatch",
            &json!(["filename1", "filename2"]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(
            migrated["managerFilePatterns"],
            json!(["filename3", "/filename1/", "/filename2/"])
        );
    }
}
