use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct UnpublishSafeMigration;

impl UnpublishSafeMigration {
    const SUPPORTED_VALUES: &[&str] = &[
        ":unpublishSafe",
        "default:unpublishSafe",
        "npm:unpublishSafe",
        "security:minimumReleaseAgeNpm",
    ];

    pub fn new() -> Self {
        Self
    }

    fn is_supported_value(value: &str) -> bool {
        Self::SUPPORTED_VALUES.contains(&value)
    }
}

impl Default for UnpublishSafeMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl Migration for UnpublishSafeMigration {
    fn property_name(&self) -> &str {
        "unpublishSafe"
    }

    fn is_deprecated(&self) -> bool {
        true
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        if value.as_bool() != Some(true) {
            return;
        }

        let mut new_extends: Vec<Value> = match migrated_config
            .get("extends")
            .or_else(|| original_config.get("extends"))
        {
            Some(Value::Array(arr)) => arr.clone(),
            Some(Value::String(s)) => vec![Value::String(s.clone())],
            _ => Vec::new(),
        };

        if new_extends.iter().all(|item| {
            item.as_str()
                .map(|s| !Self::is_supported_value(s))
                .unwrap_or(true)
        }) {
            new_extends.push(Value::String("security:minimumReleaseAgeNpm".into()));
        }

        let new_extends: Vec<Value> = new_extends
            .into_iter()
            .map(|item| {
                if let Some(s) = item.as_str()
                    && Self::is_supported_value(s)
                {
                    return Value::String("security:minimumReleaseAgeNpm".into());
                }
                item
            })
            .collect();

        migrated_config.insert("extends".into(), Value::Array(new_extends));
    }

    fn box_clone(&self) -> Box<dyn Migration> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Map;
    use serde_json::json;

    use super::UnpublishSafeMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = UnpublishSafeMigration::new();
        assert_eq!(m.property_name(), "unpublishSafe");
    }

    #[test]
    fn migrates_true() {
        let m = UnpublishSafeMigration::new();
        let mut migrated = Map::new();
        m.run("unpublishSafe", &json!(true), &Map::new(), &mut migrated);
        assert_eq!(
            migrated["extends"],
            json!(["security:minimumReleaseAgeNpm"])
        );
    }

    #[test]
    fn handles_existing_extends_string() {
        let m = UnpublishSafeMigration::new();
        let mut migrated = Map::new();
        m.run("unpublishSafe", &json!(true), &Map::new(), &mut migrated);
        // Test with original config containing extends
        let mut original = Map::new();
        original.insert("extends".into(), json!("test"));
        let mut migrated2 = Map::new();
        m.run("unpublishSafe", &json!(true), &original, &mut migrated2);
        assert_eq!(
            migrated2["extends"],
            json!(["test", "security:minimumReleaseAgeNpm"])
        );
    }

    #[test]
    fn handles_existing_extends_array() {
        let m = UnpublishSafeMigration::new();
        let mut original = Map::new();
        original.insert("extends".into(), json!([]));
        let mut migrated = Map::new();
        m.run("unpublishSafe", &json!(true), &original, &mut migrated);
        assert_eq!(
            migrated["extends"],
            json!(["security:minimumReleaseAgeNpm"])
        );
    }

    #[test]
    fn replaces_supported_values() {
        let m = UnpublishSafeMigration::new();
        let mut original = Map::new();
        original.insert("extends".into(), json!(["foo", ":unpublishSafe", "bar"]));
        let mut migrated = Map::new();
        m.run("unpublishSafe", &json!(true), &original, &mut migrated);
        assert_eq!(
            migrated["extends"],
            json!(["foo", "security:minimumReleaseAgeNpm", "bar"])
        );
    }

    #[test]
    fn prevents_duplicates() {
        let m = UnpublishSafeMigration::new();
        let mut original = Map::new();
        original.insert("extends".into(), json!(["security:minimumReleaseAgeNpm"]));
        let mut migrated = Map::new();
        m.run("unpublishSafe", &json!(true), &original, &mut migrated);
        assert_eq!(
            migrated["extends"],
            json!(["security:minimumReleaseAgeNpm"])
        );
    }

    #[test]
    fn does_nothing_for_false() {
        let m = UnpublishSafeMigration::new();
        let mut original = Map::new();
        original.insert("extends".into(), json!(["foo", "bar"]));
        let mut migrated = Map::new();
        m.run("unpublishSafe", &json!(false), &original, &mut migrated);
        assert!(migrated.is_empty());
    }
}
