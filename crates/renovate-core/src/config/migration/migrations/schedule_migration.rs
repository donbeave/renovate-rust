use serde_json::Map;
use serde_json::Value;
use std::sync::LazyLock;

use crate::config::migration::Migration;

#[derive(Clone, Debug)]
pub struct ScheduleMigration;

static EVERY_WEEKDAY_RE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"every (mon|tues|wednes|thurs|fri|satur|sun)day$").unwrap());
static EVERY_DAY_RE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"every ([a-z]*day)$").unwrap());

impl Default for ScheduleMigration {
    fn default() -> Self {
        Self::new()
    }
}

impl ScheduleMigration {
    pub fn new() -> Self {
        Self
    }
}

impl Migration for ScheduleMigration {
    fn property_name(&self) -> &str {
        "schedule"
    }

    fn run(
        &self,
        _key: &str,
        value: &Value,
        _original_config: &Map<String, Value>,
        migrated_config: &mut Map<String, Value>,
    ) {
        if value.is_null() {
            return;
        }

        let mut schedules: Vec<String> = Vec::new();
        if let Some(s) = value.as_str() {
            schedules.push(s.to_owned());
        } else if let Some(arr) = value.as_array() {
            for item in arr {
                if let Some(s) = item.as_str() {
                    schedules.push(s.to_owned());
                }
            }
        }

        if schedules.is_empty() {
            return;
        }

        for s in &mut schedules {
            if s.contains("on the last day of the month") {
                *s = s.replace("on the last day of the month", "on the first day of the month");
            }
            if s.contains("on every weekday") {
                *s = s.replace("on every weekday", "every weekday");
            }
            if s.ends_with(" every day") {
                *s = s.replace(" every day", "");
            }
            if s.ends_with("days") {
                *s = s.replace("days", "day");
            }

            if EVERY_WEEKDAY_RE.is_match(s) {
                *s = EVERY_DAY_RE.replace(s, "on $1").to_string();
            }
        }

        if value.is_string() && schedules.len() == 1 {
            migrated_config.insert("schedule".into(), Value::String(schedules.into_iter().next().unwrap()));
        } else {
            migrated_config.insert(
                "schedule".into(),
                Value::Array(schedules.into_iter().map(Value::String).collect()),
            );
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

    use super::ScheduleMigration;
    use crate::config::migration::Migration;

    #[test]
    fn property_name() {
        let m = ScheduleMigration::new();
        assert_eq!(m.property_name(), "schedule");
    }

    #[test]
    fn migrate_last_day_to_first_day() {
        let m = ScheduleMigration::new();
        let mut migrated = Map::new();
        m.run(
            "schedule",
            &json!("on the last day of the month"),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["schedule"], json!("on the first day of the month"));
    }

    #[test]
    fn migrate_every_weekday() {
        let m = ScheduleMigration::new();
        let mut migrated = Map::new();
        m.run(
            "schedule",
            &json!("on every weekday"),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["schedule"], json!("every weekday"));
    }

    #[test]
    fn migrate_every_day_suffix() {
        let m = ScheduleMigration::new();
        let mut migrated = Map::new();
        m.run(
            "schedule",
            &json!("before 5am every day"),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(migrated["schedule"], json!("before 5am"));
    }

    #[test]
    fn migrate_days_to_day() {
        let m = ScheduleMigration::new();
        let mut migrated = Map::new();
        m.run("schedule", &json!("on mondays"), &Map::new(), &mut migrated);
        assert_eq!(migrated["schedule"], json!("on monday"));
    }

    #[test]
    fn migrate_every_xday_to_on_xday() {
        let m = ScheduleMigration::new();
        let mut migrated = Map::new();
        m.run("schedule", &json!("every monday"), &Map::new(), &mut migrated);
        assert_eq!(migrated["schedule"], json!("on monday"));
    }

    #[test]
    fn migrate_array_input() {
        let m = ScheduleMigration::new();
        let mut migrated = Map::new();
        m.run(
            "schedule",
            &json!(["on mondays", "on the last day of the month"]),
            &Map::new(),
            &mut migrated,
        );
        assert_eq!(
            migrated["schedule"],
            json!(["on monday", "on the first day of the month"])
        );
    }

    #[test]
    fn null_value_is_noop() {
        let m = ScheduleMigration::new();
        let mut migrated = Map::new();
        m.run("schedule", &serde_json::Value::Null, &Map::new(), &mut migrated);
        assert!(migrated.is_empty());
    }
}
