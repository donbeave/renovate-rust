//! Timestamp parsing and normalization.
//!
//! Ports `lib/util/timestamp.ts`.
//!
//! Converts various date/time representations to a normalized ISO 8601 UTC
//! string, or `None` when the input is invalid or falls outside the valid
//! window (2000-01-01 ≤ ts < tomorrow).

use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use serde_json::Value;

const MILLENIUM_MS: i64 = 946_684_800_000; // 2000-01-01T00:00:00.000Z

fn is_valid_ms(ts_ms: i64) -> bool {
    let tomorrow_ms = Utc::now().timestamp_millis() + 86_400_000;
    ts_ms > MILLENIUM_MS && ts_ms < tomorrow_ms
}

fn ms_to_iso(ts_ms: i64) -> Option<String> {
    if !is_valid_ms(ts_ms) {
        return None;
    }
    let dt = Utc.timestamp_millis_opt(ts_ms).single()?;
    Some(format_iso(&dt))
}

fn format_iso(dt: &DateTime<Utc>) -> String {
    dt.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}

fn parse_string(s: &str) -> Option<String> {
    // 1. RFC 3339 / ISO 8601 (with optional milliseconds)
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return ms_to_iso(dt.timestamp_millis());
    }

    // 2. HTTP/RFC 2822 date format: "Wed, 21 Oct 2015 07:28:00 GMT"
    if let Ok(dt) = DateTime::parse_from_rfc2822(s) {
        return ms_to_iso(dt.timestamp_millis());
    }

    // 3. SQL format with timezone offset: "2021-10-11 07:47:24 -0700"
    if let Ok(dt) = DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S %z") {
        return ms_to_iso(dt.timestamp_millis());
    }
    // Without timezone: treat as UTC.
    if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
        let dt = Utc.from_utc_datetime(&ndt);
        return ms_to_iso(dt.timestamp_millis());
    }

    // 4. Compact format: "yyyyMMddHHmmss"
    if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y%m%d%H%M%S") {
        let dt = Utc.from_utc_datetime(&ndt);
        return ms_to_iso(dt.timestamp_millis());
    }

    // 5. Compact with timezone: "yyyyMMddHHmmss+HHMM" or "yyyyMMddHHmmss+HH:MM"
    if let Ok(dt) = DateTime::parse_from_str(s, "%Y%m%d%H%M%S%z") {
        return ms_to_iso(dt.timestamp_millis());
    }

    // 6. ISO date only: "2021-01-01"
    if let Ok(nd) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        let ndt = nd.and_hms_opt(0, 0, 0)?;
        let dt = Utc.from_utc_datetime(&ndt);
        return ms_to_iso(dt.timestamp_millis());
    }

    // 7. Slash-separated date: "2021/01/01"
    if let Ok(nd) = NaiveDate::parse_from_str(s, "%Y/%m/%d") {
        let ndt = nd.and_hms_opt(0, 0, 0)?;
        let dt = Utc.from_utc_datetime(&ndt);
        return ms_to_iso(dt.timestamp_millis());
    }

    // 8. "Jan 1, 2021" and "Jan  1, 2021" (e for padded, %e for space-padded)
    // chrono doesn't support %b %e, %Y directly, so try both %b %d, %Y and %b  %d, %Y
    if let Ok(nd) = NaiveDate::parse_from_str(s.trim(), "%b %e, %Y") {
        let ndt = nd.and_hms_opt(0, 0, 0)?;
        let dt = Utc.from_utc_datetime(&ndt);
        return ms_to_iso(dt.timestamp_millis());
    }
    // Try space-padded single digit day: "Jan  1, 2021" → %b %_d, %Y
    // Use a normalized form
    let normalized = normalize_month_day(s);
    if let Some(ref n) = normalized
        && let Ok(nd) = NaiveDate::parse_from_str(n, "%b %d, %Y") {
            let ndt = nd.and_hms_opt(0, 0, 0)?;
            let dt = Utc.from_utc_datetime(&ndt);
            return ms_to_iso(dt.timestamp_millis());
        }

    None
}

/// Normalize "Jan 1, 2021" → "Jan 01, 2021" for zero-padded parsing.
fn normalize_month_day(s: &str) -> Option<String> {
    // Try to find " M, " pattern and zero-pad M
    let re = regex::Regex::new(r"^([A-Za-z]+ )(\d)(, \d{4})$").ok()?;
    if let Some(caps) = re.captures(s) {
        return Some(format!("{}0{}{}", &caps[1], &caps[2], &caps[3]));
    }
    None
}

/// Convert a JSON value to a normalized UTC timestamp string.
///
/// Accepts:
/// - `Value::Number` — interpreted as milliseconds (large) or seconds (small)
/// - `Value::String` — parsed via multiple format strategies
/// - all other values → `None`
pub fn as_timestamp(input: &Value) -> Option<String> {
    match input {
        Value::Number(n) => {
            let n_f = n.as_f64()?;
            if n_f.is_nan() || n_f < 0.0 {
                return None;
            }
            let n_i = n_f as i64;
            if n_i <= 0 {
                return None;
            }
            // Try as milliseconds first.
            if let Some(s) = ms_to_iso(n_i) {
                return Some(s);
            }
            // Try as seconds.
            let ms = n_i.checked_mul(1000)?;
            ms_to_iso(ms)
        }
        Value::String(s) => parse_string(s),
        _ => None,
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn ts(v: Value) -> Option<String> {
        as_timestamp(&v)
    }

    // Ported: "$input -> $expected" — util/timestamp.spec.ts line 5
    #[test]
    fn timestamp_valid_iso() {
        assert_eq!(
            ts(json!("2021-01-01T00:00:00.000Z")),
            Some("2021-01-01T00:00:00.000Z".to_owned())
        );
    }

    #[test]
    fn timestamp_number_millis() {
        // 1609459200000 ms = 2021-01-01T00:00:00.000Z
        assert_eq!(
            ts(json!(1609459200000i64)),
            Some("2021-01-01T00:00:00.000Z".to_owned())
        );
    }

    #[test]
    fn timestamp_number_seconds() {
        // 1609459200 s = 2021-01-01T00:00:00.000Z
        assert_eq!(
            ts(json!(1609459200i64)),
            Some("2021-01-01T00:00:00.000Z".to_owned())
        );
    }

    #[test]
    fn timestamp_number_too_small() {
        assert_eq!(ts(json!(-1i64)), None);
        assert_eq!(ts(json!(0i64)), None);
        assert_eq!(ts(json!(123i64)), None); // both ms and s before 2000
    }

    #[test]
    fn timestamp_null_and_invalid() {
        assert_eq!(ts(json!(null)), None);
        assert_eq!(ts(json!({})), None);
        assert_eq!(ts(json!([])), None);
        assert_eq!(ts(json!("invalid date")), None);
        assert_eq!(ts(json!("202x0101000000")), None);
    }

    #[test]
    fn timestamp_date_1999() {
        // Before millennium → null
        assert_eq!(ts(json!("1999-01-01T00:00:00.000Z")), None);
    }

    #[test]
    fn timestamp_iso_with_offset() {
        assert_eq!(
            ts(json!("2021-01-02T00:00:00+05:30")),
            Some("2021-01-01T18:30:00.000Z".to_owned())
        );
        assert_eq!(
            ts(json!("2010-05-20T22:43:19-07:00")),
            Some("2010-05-21T05:43:19.000Z".to_owned())
        );
    }

    #[test]
    fn timestamp_date_only() {
        assert_eq!(
            ts(json!("2021-01-01")),
            Some("2021-01-01T00:00:00.000Z".to_owned())
        );
    }

    #[test]
    fn timestamp_compact_format() {
        assert_eq!(
            ts(json!("20210101000000")),
            Some("2021-01-01T00:00:00.000Z".to_owned())
        );
        assert_eq!(
            ts(json!("20211231235959")),
            Some("2021-12-31T23:59:59.000Z".to_owned())
        );
    }

    #[test]
    fn timestamp_compact_with_tz() {
        assert_eq!(
            ts(json!("20210101000000+0000")),
            Some("2021-01-01T00:00:00.000Z".to_owned())
        );
        assert_eq!(
            ts(json!("20211231235959+0000")),
            Some("2021-12-31T23:59:59.000Z".to_owned())
        );
    }

    #[test]
    fn timestamp_sql_with_tz() {
        assert_eq!(
            ts(json!("2021-10-11 07:47:24 -0700")),
            Some("2021-10-11T14:47:24.000Z".to_owned())
        );
    }

    #[test]
    fn timestamp_http_format() {
        assert_eq!(
            ts(json!("Wed, 21 Oct 2015 07:28:00 GMT")),
            Some("2015-10-21T07:28:00.000Z".to_owned())
        );
    }

    #[test]
    fn timestamp_slash_date() {
        assert_eq!(
            ts(json!("2021/01/01")),
            Some("2021-01-01T00:00:00.000Z".to_owned())
        );
    }

    #[test]
    fn timestamp_month_name() {
        assert_eq!(
            ts(json!("Jan 1, 2021")),
            Some("2021-01-01T00:00:00.000Z".to_owned())
        );
    }
}
