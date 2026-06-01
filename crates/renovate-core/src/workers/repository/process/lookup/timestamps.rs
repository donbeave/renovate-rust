//! Timestamp handling for lookups.
//!
//! Mirrors `lib/workers/repository/process/lookup/timestamps.ts`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TimestampedRelease {
    pub version: String,
    pub release_timestamp: Option<String>,
    pub is_deprecated: Option<bool>,
}

pub fn get_timestamp(releases: &[TimestampedRelease], version: &str) -> Option<String> {
    releases
        .iter()
        .find(|r| r.version == version)
        .and_then(|r| r.release_timestamp.clone())
}

pub fn is_older_than(timestamp: &str, days: u64) -> bool {
    let ts = match DateTime::parse_from_rfc3339(timestamp) {
        Ok(dt) => dt.with_timezone(&Utc),
        Err(_) => match timestamp.parse::<DateTime<Utc>>() {
            Ok(dt) => dt,
            Err(_) => return false,
        },
    };
    let now = Utc::now();
    let duration = now.signed_duration_since(ts);
    duration.num_days() >= days as i64
}

pub fn calculate_most_recent_timestamp(releases: &[TimestampedRelease]) -> Option<String> {
    let mut best: Option<&TimestampedRelease> = None;
    let mut best_ts: Option<DateTime<Utc>> = None;

    for release in releases {
        if release.is_deprecated.unwrap_or(false) {
            continue;
        }
        if let Some(ref ts_str) = release.release_timestamp {
            let ts = parse_flexible_timestamp(ts_str);
            if let Some(dt) = ts {
                match &best_ts {
                    None => {
                        best = Some(release);
                        best_ts = Some(dt);
                    }
                    Some(prev) => {
                        if dt > *prev {
                            best = Some(release);
                            best_ts = Some(dt);
                        }
                    }
                }
            }
        }
    }

    best.and_then(|r| r.release_timestamp.clone())
}

fn parse_flexible_timestamp(ts: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(ts)
        .map(|dt| dt.with_timezone(&Utc))
        .ok()
        .or_else(|| ts.parse::<DateTime<Utc>>().ok())
}

pub fn get_elapsed_days(timestamp: &str) -> Option<u64> {
    let ts = parse_flexible_timestamp(timestamp)?;
    let now = Utc::now();
    let duration = now.signed_duration_since(ts);
    if duration.num_days() >= 0 {
        Some(duration.num_days() as u64)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_release(version: &str, ts: Option<&str>) -> TimestampedRelease {
        TimestampedRelease {
            version: version.into(),
            release_timestamp: ts.map(String::from),
            is_deprecated: None,
        }
    }

    #[test]
    fn get_timestamp_found() {
        let releases = vec![
            make_release("1.0.0", Some("2024-01-01T00:00:00Z")),
            make_release("2.0.0", Some("2024-06-01T00:00:00Z")),
        ];
        assert_eq!(
            get_timestamp(&releases, "2.0.0"),
            Some("2024-06-01T00:00:00Z".into())
        );
    }

    #[test]
    fn get_timestamp_not_found() {
        let releases = vec![make_release("1.0.0", Some("2024-01-01T00:00:00Z"))];
        assert_eq!(get_timestamp(&releases, "3.0.0"), None);
    }

    #[test]
    fn get_timestamp_no_timestamp() {
        let releases = vec![make_release("1.0.0", None)];
        assert_eq!(get_timestamp(&releases, "1.0.0"), None);
    }

    #[test]
    fn is_older_than_true() {
        assert!(is_older_than("2020-01-01T00:00:00Z", 365));
    }

    #[test]
    fn is_older_than_false() {
        assert!(!is_older_than("2099-01-01T00:00:00Z", 365));
    }

    #[test]
    fn is_older_than_invalid() {
        assert!(!is_older_than("not-a-date", 10));
    }

    #[test]
    fn calculate_most_recent_timestamp_basic() {
        let releases = vec![
            make_release("1.0.0", Some("2024-01-01T00:00:00Z")),
            make_release("2.0.0", Some("2024-06-01T00:00:00Z")),
            make_release("3.0.0", Some("2024-03-01T00:00:00Z")),
        ];
        let result = calculate_most_recent_timestamp(&releases);
        assert_eq!(result, Some("2024-06-01T00:00:00Z".into()));
    }

    #[test]
    fn calculate_most_recent_timestamp_skips_deprecated() {
        let mut dep = make_release("2.0.0", Some("2024-06-01T00:00:00Z"));
        dep.is_deprecated = Some(true);
        let releases = vec![
            make_release("1.0.0", Some("2024-01-01T00:00:00Z")),
            dep,
            make_release("3.0.0", Some("2024-03-01T00:00:00Z")),
        ];
        let result = calculate_most_recent_timestamp(&releases);
        assert_eq!(result, Some("2024-03-01T00:00:00Z".into()));
    }

    #[test]
    fn calculate_most_recent_timestamp_empty() {
        let releases: Vec<TimestampedRelease> = vec![];
        let result = calculate_most_recent_timestamp(&releases);
        assert!(result.is_none());
    }

    #[test]
    fn calculate_most_recent_timestamp_no_timestamps() {
        let releases = vec![make_release("1.0.0", None), make_release("2.0.0", None)];
        let result = calculate_most_recent_timestamp(&releases);
        assert!(result.is_none());
    }

    #[test]
    fn get_elapsed_days_some() {
        let ts = "2020-01-01T00:00:00Z";
        let days = get_elapsed_days(ts);
        assert!(days.is_some());
        assert!(days.unwrap() > 1000);
    }

    #[test]
    fn get_elapsed_days_invalid() {
        let ts = "not-a-date";
        let days = get_elapsed_days(ts);
        assert!(days.is_none());
    }

    #[test]
    fn timestamped_release_default() {
        let r = TimestampedRelease::default();
        assert!(r.version.is_empty());
        assert!(r.release_timestamp.is_none());
        assert!(r.is_deprecated.is_none());
    }

    #[test]
    fn timestamped_release_serialization_roundtrip() {
        let r = TimestampedRelease {
            version: "1.0.0".into(),
            release_timestamp: Some("2024-01-01T00:00:00Z".into()),
            is_deprecated: Some(false),
        };
        let json = serde_json::to_string(&r).unwrap();
        let back: TimestampedRelease = serde_json::from_str(&json).unwrap();
        assert_eq!(back.version, "1.0.0");
        assert_eq!(back.release_timestamp, Some("2024-01-01T00:00:00Z".into()));
    }
}
