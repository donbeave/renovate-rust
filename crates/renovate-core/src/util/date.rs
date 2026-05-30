//! Date formatting utilities — mirrors `lib/util/date.ts`.

use chrono::{DateTime, NaiveDateTime, Utc};

/// Parse an ISO timestamp string and reformat it to `YYYY-MM-DD HH:MM:SS`.
///
/// Returns `None` if the input cannot be parsed.
pub fn format_timestamp(ts: &str) -> Option<String> {
    if ts.is_empty() {
        return None;
    }

    if let Ok(dt) = DateTime::parse_from_rfc3339(ts) {
        return Some(dt.format("%Y-%m-%d %H:%M:%S").to_string());
    }

    if let Ok(dt) = NaiveDateTime::parse_from_str(ts, "%Y-%m-%dT%H:%M:%S") {
        return Some(dt.format("%Y-%m-%d %H:%M:%S").to_string());
    }

    if let Ok(dt) = NaiveDateTime::parse_from_str(ts, "%Y-%m-%d %H:%M:%S") {
        return Some(dt.format("%Y-%m-%d %H:%M:%S").to_string());
    }

    None
}

/// Return the current UTC date and time as an ISO 8601 string.
pub fn get_current_date() -> String {
    Utc::now().to_rfc3339()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_timestamp_rfc3339_with_z() {
        let result = format_timestamp("2024-01-15T10:30:00Z");
        assert_eq!(result, Some("2024-01-15 10:30:00".to_owned()));
    }

    #[test]
    fn format_timestamp_rfc3339_with_offset() {
        let result = format_timestamp("2024-01-15T10:30:00+05:00");
        assert_eq!(result, Some("2024-01-15 10:30:00".to_owned()));
    }

    #[test]
    fn format_timestamp_naive_datetime() {
        let result = format_timestamp("2024-01-15T10:30:00");
        assert_eq!(result, Some("2024-01-15 10:30:00".to_owned()));
    }

    #[test]
    fn format_timestamp_sql_format() {
        let result = format_timestamp("2024-01-15 10:30:00");
        assert_eq!(result, Some("2024-01-15 10:30:00".to_owned()));
    }

    #[test]
    fn format_timestamp_invalid() {
        assert_eq!(format_timestamp("not-a-date"), None);
    }

    #[test]
    fn format_timestamp_empty() {
        assert_eq!(format_timestamp(""), None);
    }

    #[test]
    fn format_timestamp_with_millis() {
        let result = format_timestamp("2024-06-01T12:34:56.789Z");
        assert_eq!(result, Some("2024-06-01 12:34:56".to_owned()));
    }

    #[test]
    fn get_current_date_returns_valid_iso() {
        let date = get_current_date();
        assert!(DateTime::parse_from_rfc3339(&date).is_ok());
    }

    #[test]
    fn get_current_date_contains_t_separator() {
        let date = get_current_date();
        assert!(date.contains('T'));
    }
}
