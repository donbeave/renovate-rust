use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct NodeMigration;

impl Default for NodeMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for NodeMigration {
    fn property_name(&self) -> &str {
        "node"
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        let Value::Object(node) = value else {
            return;
        };

        let enabled = node.get("enabled").and_then(|v| v.as_bool());
        if enabled != Some(true) {
            return;
        }

        let mut new_node = node.clone();
        new_node.remove("enabled");

        let mut travis = match migrated_config
            .get("travis")
            .or_else(|| original_config.get("travis"))
        {
            Some(Value::Object(map)) => map.clone(),
            _ => Map::new(),
        };
        travis.insert("enabled".into(), Value::Bool(true));

        if new_node.is_empty() {
            migrated_config.remove("node");
        } else {
            migrated_config.insert("node".into(), Value::Object(new_node));
        }
        migrated_config.insert("travis".into(), Value::Object(travis));
    }

    fn box_clone(&self) -> Box<dyn Migration> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Map;
    use serde_json::json;

    use super::NodeMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = NodeMigration::new();
        assert_eq!(m.property_name(), "node");
    }

    #[test]
    fn migrates_enabled_true() {
        let m = NodeMigration::new();
        let mut migrated = Map::new();
        m.run(
            "node",
            &json!({ "enabled": true }),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["travis"], json!({ "enabled": true }));
        assert!(migrated.get("node").is_none());
    }

    #[test]
    fn preserves_other_node_props() {
        let m = NodeMigration::new();
        let mut migrated = Map::new();
        m.run(
            "node",
            &json!({ "enabled": true, "automerge": false }),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["travis"], json!({ "enabled": true }));
        assert_eq!(migrated["node"], json!({ "automerge": false }));
    }

    #[test]
    fn does_nothing_when_not_enabled() {
        let m = NodeMigration::new();
        let mut migrated = Map::new();
        m.run(
            "node",
            &json!({ "automerge": false }),
            &Map::new(),
            &mut migrated,
        );
        assert!(migrated.get("travis").is_none());
    }
}
