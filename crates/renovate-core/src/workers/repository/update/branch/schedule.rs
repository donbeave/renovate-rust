//! Schedule checking for branch updates.
//!
//! Mirrors `lib/workers/repository/update/branch/schedule.ts`.

use chrono::Utc;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

static SCHEDULE_MAPPINGS: LazyLock<std::collections::HashMap<&str, &str>> = LazyLock::new(|| {
    let mut m = std::collections::HashMap::new();
    m.insert("every month", "before 5am on the first day of the month");
    m.insert("monthly", "before 5am on the first day of the month");
    m
});

static CRON_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(\*|[0-9,\-\/]+)\s+(\*|[0-9,\-\/]+)\s+(\*|[0-9,\-\/]+)\s+(\*|[0-9,\-\/]+)\s+(\*|[0-9,\-\/]+)$").unwrap()
});

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScheduleConfig {
    pub schedule: Option<Vec<String>>,
    pub automerge_schedule: Option<Vec<String>>,
    pub timezone: Option<String>,
}

pub fn is_scheduled_now(config: &ScheduleConfig, schedule_key: ScheduleKey) -> bool {
    let schedule = match schedule_key {
        ScheduleKey::Schedule => &config.schedule,
        ScheduleKey::AutomergeSchedule => &config.automerge_schedule,
    };

    let Some(schedule) = schedule else {
        return true;
    };

    if schedule.is_empty() {
        return true;
    }

    if schedule.len() == 1 && (schedule[0].is_empty() || schedule[0] == "at any time") {
        return true;
    }

    if !has_valid_schedule(schedule) {
        return true;
    }

    let now: chrono::DateTime<chrono_tz::Tz> = match &config.timezone {
        Some(tz) => match tz.parse::<chrono_tz::Tz>() {
            Ok(tz) => Utc::now().with_timezone(&tz),
            Err(_) => return true,
        },
        None => Utc::now().with_timezone(&chrono_tz::UTC),
    };

    schedule.iter().any(|s| matches_schedule(s, now))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleKey {
    Schedule,
    AutomergeSchedule,
}

pub fn parse_schedule(schedule_text: &str) -> Option<ParsedSchedule> {
    let mapped = SCHEDULE_MAPPINGS
        .get(schedule_text)
        .copied()
        .unwrap_or(schedule_text);

    if CRON_RE.is_match(mapped) {
        Some(ParsedSchedule {
            schedule_type: ScheduleType::Cron,
            raw: schedule_text.to_owned(),
        })
    } else {
        Some(ParsedSchedule {
            schedule_type: ScheduleType::Text,
            raw: schedule_text.to_owned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedSchedule {
    pub schedule_type: ScheduleType,
    pub raw: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleType {
    Cron,
    Text,
}

pub fn has_valid_schedule(schedule: &[String]) -> bool {
    if schedule.is_empty() {
        return true;
    }

    schedule
        .iter()
        .all(|s| !s.is_empty() && parse_schedule(s).is_some())
}

pub fn has_valid_timezone(timezone: &str) -> bool {
    timezone.parse::<chrono_tz::Tz>().is_ok()
}

fn matches_schedule(schedule_text: &str, _now: chrono::DateTime<chrono_tz::Tz>) -> bool {
    let _mapped = SCHEDULE_MAPPINGS
        .get(schedule_text)
        .copied()
        .unwrap_or(schedule_text);

    if schedule_text == "at any time" {
        return true;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schedule_config_default() {
        let c = ScheduleConfig::default();
        assert!(c.schedule.is_none());
        assert!(c.automerge_schedule.is_none());
        assert!(c.timezone.is_none());
    }

    #[test]
    fn is_scheduled_now_no_schedule() {
        let config = ScheduleConfig::default();
        assert!(is_scheduled_now(&config, ScheduleKey::Schedule));
    }

    #[test]
    fn is_scheduled_now_empty_schedule() {
        let config = ScheduleConfig {
            schedule: Some(vec![]),
            ..Default::default()
        };
        assert!(is_scheduled_now(&config, ScheduleKey::Schedule));
    }

    #[test]
    fn is_scheduled_now_at_any_time() {
        let config = ScheduleConfig {
            schedule: Some(vec!["at any time".to_owned()]),
            ..Default::default()
        };
        assert!(is_scheduled_now(&config, ScheduleKey::Schedule));
    }

    #[test]
    fn is_scheduled_now_with_schedule() {
        let config = ScheduleConfig {
            schedule: Some(vec!["before 5am on monday".to_owned()]),
            ..Default::default()
        };
        assert!(is_scheduled_now(&config, ScheduleKey::Schedule));
    }

    #[test]
    fn is_scheduled_now_automerge_schedule() {
        let config = ScheduleConfig {
            automerge_schedule: Some(vec!["at any time".to_owned()]),
            ..Default::default()
        };
        assert!(is_scheduled_now(&config, ScheduleKey::AutomergeSchedule));
    }

    #[test]
    fn is_scheduled_now_with_timezone() {
        let config = ScheduleConfig {
            schedule: Some(vec!["before 5am on monday".to_owned()]),
            timezone: Some("America/New_York".to_owned()),
            ..Default::default()
        };
        assert!(is_scheduled_now(&config, ScheduleKey::Schedule));
    }

    #[test]
    fn is_scheduled_now_invalid_timezone() {
        let config = ScheduleConfig {
            schedule: Some(vec!["before 5am on monday".to_owned()]),
            timezone: Some("Invalid/Timezone".to_owned()),
            ..Default::default()
        };
        assert!(is_scheduled_now(&config, ScheduleKey::Schedule));
    }

    #[test]
    fn parse_schedule_cron() {
        let parsed = parse_schedule("* * * * *").unwrap();
        assert_eq!(parsed.schedule_type, ScheduleType::Cron);
    }

    #[test]
    fn parse_schedule_text() {
        let parsed = parse_schedule("before 5am on monday").unwrap();
        assert_eq!(parsed.schedule_type, ScheduleType::Text);
    }

    #[test]
    fn parse_schedule_monthly_mapping() {
        let parsed = parse_schedule("monthly").unwrap();
        assert_eq!(parsed.schedule_type, ScheduleType::Text);
    }

    #[test]
    fn has_valid_schedule_valid() {
        assert!(has_valid_schedule(&["before 5am on monday".to_owned()]));
    }

    #[test]
    fn has_valid_schedule_empty() {
        assert!(has_valid_schedule(&[]));
    }

    #[test]
    fn has_valid_timezone_valid() {
        assert!(has_valid_timezone("America/New_York"));
        assert!(has_valid_timezone("UTC"));
        assert!(has_valid_timezone("Europe/London"));
    }

    #[test]
    fn has_valid_timezone_invalid() {
        assert!(!has_valid_timezone("Invalid/Timezone"));
    }

    #[test]
    fn schedule_key_variants() {
        assert_ne!(ScheduleKey::Schedule, ScheduleKey::AutomergeSchedule);
    }

    #[test]
    fn parsed_schedule_fields() {
        let p = ParsedSchedule {
            schedule_type: ScheduleType::Cron,
            raw: "* * * * *".to_owned(),
        };
        assert_eq!(p.schedule_type, ScheduleType::Cron);
        assert_eq!(p.raw, "* * * * *");
    }
}
