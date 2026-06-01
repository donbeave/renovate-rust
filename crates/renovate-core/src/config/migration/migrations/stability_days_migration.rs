use serde_json::Map;
use serde_json::Value;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct StabilityDaysMigration;

impl Default for StabilityDaysMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl StabilityDaysMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for StabilityDaysMigration {
    fn property_name(&self) -> &str {
        "stabilityDays"
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
        let Some(days) = value.as_i64() else { return };

        let new_value = match days {
            0 => Value::Null,
            1 => Value::String("1 day".into()),
            _ => Value::String(format!("{} days", days)),
        };

        if !migrated_config.contains_key("minimumReleaseAge") {
            migrated_config.insert("minimumReleaseAge".into(), new_value);
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

    use super::StabilityDaysMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = StabilityDaysMigration::new();
        assert_eq!(m.property_name(), "stabilityDays");
    }

    #[test]
    fn migrate_zero() {
        let m = StabilityDaysMigration::new();
        let mut migrated = Map::new();
        m.run("stabilityDays", &json!(0), &Map::new(), &mut migrated);
        assert_eq!(migrated["minimumReleaseAge"], json!(null));
    }

    #[test]
    fn migrate_one() {
        let m = StabilityDaysMigration::new();
        let mut migrated = Map::new();
        m.run("stabilityDays", &json!(1), &Map::new(), &mut migrated);
        assert_eq!(migrated["minimumReleaseAge"], json!("1 day"));
    }

    #[test]
    fn migrate_two() {
        let m = StabilityDaysMigration::new();
        let mut migrated = Map::new();
        m.run("stabilityDays", &json!(2), &Map::new(), &mut migrated);
        assert_eq!(migrated["minimumReleaseAge"], json!("2 days"));
    }

    #[test]
    fn does_not_overwrite_existing_minimum_release_age() {
        let m = StabilityDaysMigration::new();
        let mut migrated = Map::new();
        migrated.insert("minimumReleaseAge".into(), json!("5 days"));
        m.run("stabilityDays", &json!(2), &Map::new(), &mut migrated);
        assert_eq!(migrated["minimumReleaseAge"], json!("5 days"));
    }
}
