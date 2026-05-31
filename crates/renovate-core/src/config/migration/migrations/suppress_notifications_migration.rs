use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct SuppressNotificationsMigration;

impl Default for SuppressNotificationsMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl SuppressNotificationsMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for SuppressNotificationsMigration {
    fn property_name(&self) -> &str {
        "suppressNotifications"
    }

    fn is_deprecated(&self) -> bool {
        false
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
        let new_value: Vec<Value> = arr
            .iter()
            .filter(|v| v.as_str() != Some("prEditNotification"))
            .cloned()
            .collect();
        if new_value.len() != arr.len() {
            migrated_config.insert("suppressNotifications".into(), Value::Array(new_value));
        }
    }

    fn box_clone(&self) -> Box<dyn Migration> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use serde_json::Map;

    use super::SuppressNotificationsMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = SuppressNotificationsMigration::new();
        assert_eq!(m.property_name(), "suppressNotifications");
    }

    #[test]
    fn removes_pr_edit_notification() {
        let m = SuppressNotificationsMigration::new();
        let mut migrated = Map::new();
        m.run(
            "suppressNotifications",
            &json!(["test", "prEditNotification"]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["suppressNotifications"], json!(["test"]));
    }

    #[test]
    fn no_change_without_pr_edit_notification() {
        let m = SuppressNotificationsMigration::new();
        let mut migrated = Map::new();
        m.run(
            "suppressNotifications",
            &json!(["test"]),
            &Map::new(),
            &mut migrated,
        );
        assert!(migrated.is_empty());
    }

    #[test]
    fn no_change_for_empty_array() {
        let m = SuppressNotificationsMigration::new();
        let mut migrated = Map::new();
        m.run(
            "suppressNotifications",
            &json!([]),
            &Map::new(),
            &mut migrated,
        );
        assert!(migrated.is_empty());
    }
}
