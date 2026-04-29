//! Renovate schedule evaluation.
//!
//! Parses and evaluates POSIX cron expressions against a point in time,
//! implementing the schedule-checking logic from Renovate's
//! `lib/workers/repository/update/branch/schedule.ts`.
//!
//! ## Supported format
//!
//! Five-field POSIX cron (`minute hour dom month weekday`).
//! Renovate's built-in schedule presets always use `*` as the minute field
//! (e.g. `"* 0-3 * * *"`), so the minute field is parsed but ignored for
//! the `is_within_schedule` check.  Supported field syntax:
//!
//! - `*` — any value
//! - `*/N` — every N-th value (step)
//! - `N` — exact value
//! - `N-M` — inclusive range
//! - `N,M,...` — comma-separated list of values, ranges, and steps
//!
//! ## Weekday numbering
//!
//! Follows Unix convention: Sunday = 0, Monday = 1, ..., Saturday = 6.
//! Both `0` and `7` map to Sunday (ISO week: Monday = 1, Sunday = 7) is
//! normalised to 0.

use chrono::{DateTime, Datelike, Duration, TimeZone, Timelike, Utc, Weekday};
use chrono_tz::Tz;

/// Parse a Renovate `minimumReleaseAge` string into a [`Duration`].
///
/// Supported units: `second`/`seconds`/`sec`/`secs`, `minute`/`minutes`/`min`/`mins`,
/// `hour`/`hours`/`hr`/`hrs`, `day`/`days`, `week`/`weeks`, `month`/`months`.
///
/// A "month" is approximated as 30 days.  Returns `None` for unrecognised strings.
///
/// # Examples
/// ```
/// # use renovate_core::schedule::parse_age_duration;
/// assert!(parse_age_duration("3 days").is_some());
/// assert!(parse_age_duration("1 week").is_some());
/// assert!(parse_age_duration("bogus").is_none());
/// ```
pub fn parse_age_duration(age: &str) -> Option<Duration> {
    let age = age.trim().to_lowercase();
    let mut parts = age.splitn(2, ' ');
    let n: i64 = parts.next()?.parse().ok()?;
    let unit = parts.next()?.trim();

    match unit {
        "second" | "seconds" | "sec" | "secs" => Duration::try_seconds(n),
        "minute" | "minutes" | "min" | "mins" => Duration::try_minutes(n),
        "hour" | "hours" | "hr" | "hrs" => Duration::try_hours(n),
        "day" | "days" => Duration::try_days(n),
        "week" | "weeks" => Duration::try_weeks(n),
        "month" | "months" => Duration::try_days(n * 30),
        _ => None,
    }
}

/// Return `true` when `release_timestamp` satisfies the `minimum_release_age` constraint.
///
/// - If `minimum_release_age` is `None` → always `true` (no restriction).
/// - If the timestamp is unparseable → `true` (fail-open).
/// - Otherwise checks `Utc::now() - timestamp >= minimum_release_age`.
pub fn is_within_release_age(
    release_timestamp: Option<&str>,
    minimum_release_age: Option<&str>,
) -> bool {
    let Some(min_age) = minimum_release_age else {
        return true; // no age restriction
    };
    let Some(min_dur) = parse_age_duration(min_age) else {
        return true; // unparseable constraint → fail-open
    };
    let Some(ts_str) = release_timestamp else {
        return true; // no timestamp available → fail-open
    };
    let Ok(ts) = ts_str.parse::<DateTime<Utc>>() else {
        return true; // unparseable timestamp → fail-open
    };
    Utc::now() - ts >= min_dur
}

/// Evaluate a `matchCurrentAge` range expression against a version release timestamp.
///
/// `range` has the form `"<operator> <age>"` where operator is one of
/// `>`, `>=`, `<`, `<=` and `age` is a human duration like `"3 days"`,
/// `"1 month"`, `"2 weeks"`.
///
/// Semantics (mirrors Renovate's `satisfiesDateRange`):
/// - `> age` — version was released **more than** `age` ago
/// - `>= age` — version was released `age` or more ago
/// - `< age` — version was released **less than** `age` ago
/// - `<= age` — version was released `age` or less ago
///
/// Returns `false` when either the range or timestamp is unparseable.
pub fn satisfies_date_range(timestamp: &str, range: &str) -> bool {
    let range = range.trim();
    // Parse "<operator> <age>" — operator is >=, <=, >, or <.
    let (operator, age_str) = if let Some(rest) = range.strip_prefix(">=") {
        (">=", rest.trim())
    } else if let Some(rest) = range.strip_prefix("<=") {
        ("<=", rest.trim())
    } else if let Some(rest) = range.strip_prefix('>') {
        (">", rest.trim())
    } else if let Some(rest) = range.strip_prefix('<') {
        ("<", rest.trim())
    } else {
        return false;
    };

    let Some(age_dur) = parse_age_duration(age_str) else {
        return false;
    };

    // Parse the timestamp — append Z if needed so chrono recognises it as UTC.
    let ts_str = if timestamp.ends_with('Z') || timestamp.contains('+') {
        timestamp.to_owned()
    } else {
        format!("{timestamp}Z")
    };
    let Ok(ts) = ts_str.parse::<DateTime<Utc>>() else {
        return false;
    };

    let date_ms = ts.timestamp_millis();
    let age_ms = age_dur.num_milliseconds();
    let threshold_ms = Utc::now().timestamp_millis() - age_ms;

    match operator {
        ">" => date_ms < threshold_ms,
        ">=" => date_ms <= threshold_ms,
        "<" => date_ms > threshold_ms,
        "<=" => date_ms >= threshold_ms,
        _ => false,
    }
}

/// Return `true` when any entry in `schedule` matches the current UTC time.
///
/// An empty schedule (or `["at any time"]`) always returns `true`.
/// Cron entries are evaluated by checking if the current `(hour, dom, month,
/// weekday)` matches the pattern.  The minute field is ignored.
pub fn is_within_schedule(schedule: &[String]) -> bool {
    is_within_schedule_at(schedule, Utc::now())
}

/// Like [`is_within_schedule`] but takes an explicit `DateTime` for testability.
pub fn is_within_schedule_at(schedule: &[String], now: DateTime<Utc>) -> bool {
    is_within_schedule_tz_at(schedule, None, now)
}

/// Return `true` when any entry in `schedule` matches the current time in
/// `timezone`.  When `timezone` is `None` or unrecognised, falls back to UTC.
///
/// Renovate reference: `lib/workers/repository/update/branch/schedule.ts` —
/// schedule evaluation with `later.schedule()`.  Renovate uses the repository's
/// `timezone` config option (IANA timezone name) to convert the current time
/// before checking.  An unrecognised timezone name is treated as UTC (fail-open)
/// to match Renovate's behaviour of logging a warning and continuing.
pub fn is_within_schedule_tz(schedule: &[String], timezone: Option<&str>) -> bool {
    is_within_schedule_tz_at(schedule, timezone, Utc::now())
}

/// Like [`is_within_schedule_tz`] but takes an explicit `DateTime<Utc>` for
/// testability.
pub fn is_within_schedule_tz_at(
    schedule: &[String],
    timezone: Option<&str>,
    now: DateTime<Utc>,
) -> bool {
    if schedule.is_empty() || schedule.iter().any(|s| s == "at any time" || s.is_empty()) {
        return true;
    }

    let (hour, dom, month, weekday) = match timezone.and_then(|tz| tz.parse::<Tz>().ok()) {
        Some(tz) => {
            let local = tz.from_utc_datetime(&now.naive_utc());
            (
                local.hour() as u8,
                local.day() as u8,
                local.month() as u8,
                weekday_to_unix(local.weekday()),
            )
        }
        None => (
            now.hour() as u8,
            now.day() as u8,
            now.month() as u8,
            weekday_to_unix(now.weekday()),
        ),
    };

    schedule.iter().any(|entry| {
        if looks_like_cron(entry) {
            cron_matches(entry, hour, dom, month, weekday)
        } else {
            text_schedule_matches(entry, hour, dom, weekday)
        }
    })
}

fn weekday_to_unix(w: Weekday) -> u8 {
    match w {
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
        Weekday::Sun => 0,
    }
}

/// Return `true` if `cron_expr` matches the given time components.
///
/// The minute field (first field) is accepted but ignored — Renovate schedule
/// presets always use `*` for minutes and check at any-minute granularity.
pub fn cron_matches(cron_expr: &str, hour: u8, dom: u8, month: u8, weekday: u8) -> bool {
    let fields: Vec<&str> = cron_expr.split_whitespace().collect();
    if fields.len() != 5 {
        return false;
    }
    // fields: [minute, hour, dom, month, weekday]
    // minute (fields[0]) is intentionally ignored.
    cron_field_matches(fields[1], hour, 0, 23)
        && cron_field_matches(fields[2], dom, 1, 31)
        && cron_field_matches(fields[3], month, 1, 12)
        && cron_field_matches_weekday(fields[4], weekday)
}

/// Parse and evaluate a single cron field against `value`.
///
/// `min`/`max` are the legal bounds for this field (used for `*` and `*/N`).
fn cron_field_matches(field: &str, value: u8, min: u8, max: u8) -> bool {
    // Handle comma-separated alternatives (OR).
    for part in field.split(',') {
        if cron_part_matches(part.trim(), value, min, max) {
            return true;
        }
    }
    false
}

/// Evaluate a single cron part (no comma) against `value`.
fn cron_part_matches(part: &str, value: u8, min: u8, max: u8) -> bool {
    if part == "*" {
        return true;
    }
    // Step: `*/N` or `M-N/S`
    if let Some((base, step_str)) = part.split_once('/') {
        let step: u8 = step_str.parse().unwrap_or(1).max(1);
        let (range_min, range_max) = if base == "*" {
            (min, max)
        } else if let Some((a, b)) = base.split_once('-') {
            let lo: u8 = a.parse().unwrap_or(min);
            let hi: u8 = b.parse().unwrap_or(max);
            (lo, hi)
        } else {
            let n: u8 = base.parse().unwrap_or(min);
            (n, max)
        };
        // value must be in [range_min, range_max] AND congruent to range_min mod step.
        if value < range_min || value > range_max {
            return false;
        }
        return (value - range_min).is_multiple_of(step);
    }
    // Range: `N-M`
    if let Some((a, b)) = part.split_once('-') {
        let lo: u8 = a.parse().unwrap_or(0);
        let hi: u8 = b.parse().unwrap_or(255);
        return value >= lo && value <= hi;
    }
    // Exact: `N`
    if let Ok(n) = part.parse::<u8>() {
        return value == n;
    }
    false
}

/// Weekday field matching with Sunday=0 AND Sunday=7 aliasing.
fn cron_field_matches_weekday(field: &str, weekday: u8) -> bool {
    let normalised = field.replace('7', "0");
    cron_field_matches(&normalised, weekday, 0, 6)
}

/// Heuristic: does `entry` look like a 5-field cron expression?
/// Cron entries start with `*` or a digit and contain at least 4 spaces.
fn looks_like_cron(entry: &str) -> bool {
    let first = entry.chars().next().unwrap_or(' ');
    (first == '*' || first.is_ascii_digit()) && entry.split_whitespace().count() == 5
}

/// Parse an AM/PM time token like `"5am"`, `"4pm"`, `"4:00am"`, `"11:00pm"`.
/// Returns the 24-hour value, or `None` if unparseable.
fn parse_ampm_hour(tok: &str) -> Option<u8> {
    let tok = tok.trim().to_lowercase();
    let (digits, is_pm) = if let Some(s) = tok.strip_suffix("pm") {
        (s, true)
    } else if let Some(s) = tok.strip_suffix("am") {
        (s, false)
    } else {
        return None;
    };
    // Strip optional ":00" or ":30" minute part.
    let hour_part = digits.split(':').next().unwrap_or(digits);
    let h: u8 = hour_part.parse().ok()?;
    if is_pm {
        // 12pm = noon = 12; 1pm = 13; ...
        Some(if h == 12 { 12 } else { h + 12 })
    } else {
        // 12am = midnight = 0; 1am = 1; ...
        Some(if h == 12 { 0 } else { h })
    }
}

/// Return the bitmask of weekday values (0=Sun .. 6=Sat) described by `name`.
/// Handles individual day names and "weekday"/"weekend" keywords.
fn parse_day_names(text: &str) -> Option<Vec<u8>> {
    let t = text.trim().to_lowercase();
    match t.as_str() {
        "sunday" | "sun" => Some(vec![0]),
        "monday" | "mon" => Some(vec![1]),
        "tuesday" | "tue" => Some(vec![2]),
        "wednesday" | "wed" => Some(vec![3]),
        "thursday" | "thu" => Some(vec![4]),
        "friday" | "fri" => Some(vec![5]),
        "saturday" | "sat" => Some(vec![6]),
        "weekday" | "weekdays" => Some(vec![1, 2, 3, 4, 5]),
        "weekend" | "weekends" => Some(vec![0, 6]),
        _ => None,
    }
}

/// Evaluate a later.js text schedule entry against the given time components.
///
/// Supports the most common Renovate patterns:
/// - `"before Xam"` / `"before X:00am"` — current hour < X
/// - `"after Xpm"` — current hour >= X
/// - `"between Xam and Ypm"` — X <= hour < Y
/// - `"every weekday"` / `"every weekend"` — day of week
/// - `"on Monday"`, `"on friday and saturday"` — specific days
/// - `"before X on weekdays"`, `"after X every weekday"` — combined
/// - `"on the first day of the month"` — dom == 1
///
/// Unrecognised entries return `true` (fail-open: don't block on unknown schedules).
///
/// Renovate reference: `lib/workers/repository/update/branch/schedule.ts` —
/// `later.parse.text()` driven schedule evaluation.
pub fn text_schedule_matches(entry: &str, hour: u8, dom: u8, weekday: u8) -> bool {
    let s = entry.trim().to_lowercase();

    // "at any time" / "" — always match
    if s.is_empty() || s == "at any time" {
        return true;
    }

    // "on the first day of the month"
    if s.contains("first day of the month") {
        let time_ok = if let Some(before_part) = extract_before_time(&s) {
            hour < before_part
        } else {
            true
        };
        return dom == 1 && time_ok;
    }

    // Parse time constraints and day constraints separately.
    let (time_ok, has_time) = evaluate_time_constraint(&s, hour);
    let (day_ok, has_day) = evaluate_day_constraint(&s, weekday);

    if !has_time && !has_day {
        // Unrecognised format → fail-open (treat as always matching)
        return true;
    }

    // Both must pass if both are present; otherwise only the present one matters.
    match (has_time, has_day) {
        (true, true) => time_ok && day_ok,
        (true, false) => time_ok,
        (false, true) => day_ok,
        (false, false) => true,
    }
}

/// Extract the hour limit from a `"before Xam"` clause in `s`, if present.
fn extract_before_time(s: &str) -> Option<u8> {
    let idx = s.find("before ")?;
    let rest = &s[idx + "before ".len()..];
    let tok: &str = rest.split_whitespace().next()?;
    parse_ampm_hour(tok)
}

/// Extract the hour limit from an `"after Xpm"` clause.
fn extract_after_time(s: &str) -> Option<u8> {
    let idx = s.find("after ")?;
    let rest = &s[idx + "after ".len()..];
    let tok: &str = rest.split_whitespace().next()?;
    parse_ampm_hour(tok)
}

/// Evaluate the time part of a text schedule.
/// Returns `(passes, has_time_constraint)`.
fn evaluate_time_constraint(s: &str, hour: u8) -> (bool, bool) {
    let has_before = s.contains("before ");
    let has_after = s.contains("after ");
    let has_between = s.contains("between ");

    if has_between {
        // "between Xam and Ypm"
        if let Some(idx) = s.find("between ") {
            let rest = &s[idx + "between ".len()..];
            let parts: Vec<&str> = rest.split(" and ").collect();
            if parts.len() >= 2 {
                let lo = parts[0].split_whitespace().next().and_then(parse_ampm_hour);
                let hi = parts[1].split_whitespace().next().and_then(parse_ampm_hour);
                if let (Some(lo), Some(hi)) = (lo, hi) {
                    return (hour >= lo && hour < hi, true);
                }
            }
        }
    }

    if has_before && has_after {
        // "after Xpm and before Yam" or vice versa — overnight window
        let before_h = extract_before_time(s);
        let after_h = extract_after_time(s);
        if let (Some(bh), Some(ah)) = (before_h, after_h) {
            // If after > before: e.g. after 22 and before 6 → hour >= 22 OR hour < 6
            if ah > bh {
                return (hour >= ah || hour < bh, true);
            }
            return (hour < bh && hour >= ah, true);
        }
    }

    if let (true, Some(h)) = (has_before, extract_before_time(s)) {
        return (hour < h, true);
    }
    if let (true, Some(h)) = (has_after, extract_after_time(s)) {
        return (hour >= h, true);
    }

    (true, false)
}

/// Evaluate the day-of-week part of a text schedule.
/// Returns `(passes, has_day_constraint)`.
fn evaluate_day_constraint(s: &str, weekday: u8) -> (bool, bool) {
    // "every weekday" / "every weekend"
    if s.contains("every weekday") || s.contains("on every weekday") {
        return ((1..=5).contains(&weekday), true);
    }
    if s.contains("every weekend") {
        return (weekday == 0 || weekday == 6, true);
    }

    // "on Monday", "on friday and saturday", "on monday and tuesday"
    let on_pos = s
        .find(" on ")
        .map(|i| i + 4)
        .or_else(|| if s.starts_with("on ") { Some(3) } else { None });

    if let Some(start) = on_pos {
        let day_part = &s[start..];
        // Strip any trailing time-related words.
        let day_part = day_part
            .split(" before ")
            .next()
            .unwrap_or(day_part)
            .split(" after ")
            .next()
            .unwrap_or(day_part);

        // Check special keyword "the first day of the month" — handled elsewhere.
        if day_part.starts_with("the first day") {
            return (true, false); // Handled by DOM check above.
        }

        // Collect allowed weekdays from "day1 and day2" list.
        let mut allowed: Vec<u8> = Vec::new();
        for token in day_part.split(" and ") {
            let token = token.trim();
            if let Some(days) = parse_day_names(token) {
                allowed.extend(days);
            }
        }
        if !allowed.is_empty() {
            return (allowed.contains(&weekday), true);
        }
    }

    (true, false)
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    fn utc(year: i32, month: u32, day: u32, hour: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(year, month, day, hour, 0, 0).unwrap()
    }

    // ── cron_matches ─────────────────────────────────────────────────────────

    #[test]
    fn wildcard_matches_any_time() {
        assert!(cron_matches("* * * * *", 14, 15, 6, 3));
    }

    #[test]
    fn hour_range_matches_within() {
        // "* 0-3 * * *" — hours 0 through 3
        assert!(cron_matches("* 0-3 * * *", 0, 1, 1, 1));
        assert!(cron_matches("* 0-3 * * *", 3, 1, 1, 1));
        assert!(!cron_matches("* 0-3 * * *", 4, 1, 1, 1));
    }

    #[test]
    fn weekday_monday_only() {
        // "* * * * 1" — Mondays only
        assert!(cron_matches("* * * * 1", 12, 1, 1, 1)); // Monday=1
        assert!(!cron_matches("* * * * 1", 12, 1, 1, 2)); // Tuesday=2
    }

    #[test]
    fn weekday_range_mon_to_fri() {
        // "* * * * 1-5" — Mon-Fri
        for dow in 1u8..=5 {
            assert!(cron_matches("* * * * 1-5", 10, 1, 1, dow));
        }
        assert!(!cron_matches("* * * * 1-5", 10, 1, 1, 0)); // Sunday
        assert!(!cron_matches("* * * * 1-5", 10, 1, 1, 6)); // Saturday
    }

    #[test]
    fn weekday_weekend() {
        // "* * * * 0,6" — Sat/Sun
        assert!(cron_matches("* * * * 0,6", 10, 1, 1, 0)); // Sunday
        assert!(cron_matches("* * * * 0,6", 10, 1, 1, 6)); // Saturday
        assert!(!cron_matches("* * * * 0,6", 10, 1, 1, 1)); // Monday
    }

    #[test]
    fn non_office_hours() {
        // "* 0-4,22-23 * * 1-5" — early morning OR late evening on weekdays
        assert!(cron_matches("* 0-4,22-23 * * 1-5", 2, 1, 1, 1)); // 2am Monday
        assert!(cron_matches("* 0-4,22-23 * * 1-5", 23, 1, 1, 5)); // 11pm Friday
        assert!(!cron_matches("* 0-4,22-23 * * 1-5", 10, 1, 1, 3)); // 10am Wednesday
    }

    #[test]
    fn first_of_month() {
        // "* 0-3 1 * *" — first of month, hours 0-3
        assert!(cron_matches("* 0-3 1 * *", 2, 1, 6, 3));
        assert!(!cron_matches("* 0-3 1 * *", 2, 2, 6, 4));
    }

    #[test]
    fn quarterly_schedule() {
        // "* * 1 */3 *" — first day of every 3rd month (starting Jan)
        assert!(cron_matches("* * 1 */3 *", 12, 1, 1, 2)); // Jan 1
        assert!(cron_matches("* * 1 */3 *", 12, 1, 4, 4)); // Apr 1
        assert!(cron_matches("* * 1 */3 *", 12, 1, 7, 6)); // Jul 1
        assert!(cron_matches("* * 1 */3 *", 12, 1, 10, 1)); // Oct 1
        assert!(!cron_matches("* * 1 */3 *", 12, 1, 2, 5)); // Feb 1
        assert!(!cron_matches("* * 1 */3 *", 12, 2, 1, 2)); // Jan 2
    }

    #[test]
    fn sunday_aliased_to_zero() {
        // Cron allows both 0 and 7 for Sunday.
        assert!(cron_matches("* * * * 7", 10, 1, 1, 0)); // Sunday=0 matches "7"
        assert!(cron_matches("* * * * 0", 10, 1, 1, 0));
    }

    // ── is_within_schedule_at ────────────────────────────────────────────────

    #[test]
    fn empty_schedule_always_matches() {
        assert!(is_within_schedule_at(&[], utc(2024, 4, 15, 10)));
    }

    #[test]
    fn at_any_time_always_matches() {
        let sched = vec!["at any time".to_owned()];
        assert!(is_within_schedule_at(&sched, utc(2024, 4, 15, 10)));
    }

    #[test]
    fn daily_preset_matches_midnight() {
        // schedule:daily → "* 0-3 * * *"
        let sched = vec!["* 0-3 * * *".to_owned()];
        assert!(is_within_schedule_at(&sched, utc(2024, 4, 15, 1)));
        assert!(!is_within_schedule_at(&sched, utc(2024, 4, 15, 10)));
    }

    #[test]
    fn weekdays_preset_matches_weekday() {
        // schedule:weekdays → "* * * * 1-5"
        // 2024-04-15 is a Monday
        let sched = vec!["* * * * 1-5".to_owned()];
        assert!(is_within_schedule_at(&sched, utc(2024, 4, 15, 12))); // Monday
        // 2024-04-20 is a Saturday
        assert!(!is_within_schedule_at(&sched, utc(2024, 4, 20, 12)));
    }

    #[test]
    fn weekends_preset_matches_weekend() {
        // schedule:weekends → "* * * * 0,6"
        let sched = vec!["* * * * 0,6".to_owned()];
        // 2024-04-20 is Saturday
        assert!(is_within_schedule_at(&sched, utc(2024, 4, 20, 12)));
        // 2024-04-21 is Sunday
        assert!(is_within_schedule_at(&sched, utc(2024, 4, 21, 12)));
        // 2024-04-15 is Monday
        assert!(!is_within_schedule_at(&sched, utc(2024, 4, 15, 12)));
    }

    #[test]
    fn multiple_entries_any_match_wins() {
        let sched = vec!["* 0-3 * * *".to_owned(), "* * * * 6".to_owned()];
        assert!(!is_within_schedule_at(&sched, utc(2024, 4, 15, 10)));
        assert!(is_within_schedule_at(&sched, utc(2024, 4, 15, 2)));
        assert!(is_within_schedule_at(&sched, utc(2024, 4, 20, 10)));
    }

    // ── text_schedule_matches ─────────────────────────────────────────────────
    // Reference mock time: 2017-06-30 10:50am UTC (Friday, weekday 5)

    fn fri_10am() -> DateTime<Utc> {
        utc(2017, 6, 30, 10)
    }

    #[test]
    fn text_before_4pm_true() {
        // "before 4:00pm" at 10am → true
        let sched = vec!["before 4:00pm".to_owned()];
        assert!(is_within_schedule_at(&sched, fri_10am()));
    }

    #[test]
    fn text_before_4am_false() {
        // "before 4:00am" at 10am → false
        let sched = vec!["before 4:00am".to_owned()];
        assert!(!is_within_schedule_at(&sched, fri_10am()));
    }

    #[test]
    fn text_after_4pm_false() {
        // "after 4:00pm" at 10am → false
        let sched = vec!["after 4:00pm".to_owned()];
        assert!(!is_within_schedule_at(&sched, fri_10am()));
    }

    #[test]
    fn text_every_weekday_true() {
        // "every weekday" on Friday → true
        let sched = vec!["every weekday".to_owned()];
        assert!(is_within_schedule_at(&sched, fri_10am()));
    }

    #[test]
    fn text_every_weekend_false() {
        // "every weekend" on Friday → false
        let sched = vec!["every weekend".to_owned()];
        assert!(!is_within_schedule_at(&sched, fri_10am()));
        // Saturday → true
        assert!(is_within_schedule_at(&sched, utc(2024, 4, 20, 10)));
    }

    #[test]
    fn text_on_friday_and_saturday_true() {
        let sched = vec!["on friday and saturday".to_owned()];
        assert!(is_within_schedule_at(&sched, fri_10am())); // Friday
        assert!(!is_within_schedule_at(&sched, utc(2024, 4, 15, 10))); // Monday
    }

    #[test]
    fn text_on_monday_and_tuesday_false() {
        let sched = vec!["on monday and tuesday".to_owned()];
        assert!(!is_within_schedule_at(&sched, fri_10am())); // Friday
    }

    #[test]
    fn text_before_11am_every_weekday_true() {
        // "before 11:00am every weekday" at 10am Friday → true
        let sched = vec!["before 11:00am every weekday".to_owned()];
        assert!(is_within_schedule_at(&sched, fri_10am()));
        // At noon same day → false (time fails)
        assert!(!is_within_schedule_at(&sched, utc(2017, 6, 30, 12)));
        // Before 11am on Saturday → false (day fails)
        assert!(!is_within_schedule_at(&sched, utc(2024, 4, 20, 9)));
    }

    #[test]
    fn text_first_day_of_month_true() {
        let sched = vec!["before 11am on the first day of the month".to_owned()];
        // First of a month at 9am
        assert!(is_within_schedule_at(&sched, utc(2017, 10, 1, 9)));
        // First of a month but after 11am
        assert!(!is_within_schedule_at(&sched, utc(2017, 10, 1, 12)));
        // Not the first day
        assert!(!is_within_schedule_at(&sched, fri_10am()));
    }

    #[test]
    fn text_after_11pm_and_before_6am_overnight() {
        // "after 11pm and before 6am" — overnight window
        let sched = vec!["after 11pm and before 6am".to_owned()];
        // Midnight → in window
        assert!(is_within_schedule_at(&sched, utc(2024, 4, 15, 0)));
        // 5am → in window
        assert!(is_within_schedule_at(&sched, utc(2024, 4, 15, 5)));
        // 11pm → in window
        assert!(is_within_schedule_at(&sched, utc(2024, 4, 15, 23)));
        // 10am → out of window
        assert!(!is_within_schedule_at(&sched, fri_10am()));
    }

    // ── parse_age_duration ────────────────────────────────────────────────────

    #[test]
    fn parse_age_3_days() {
        let d = parse_age_duration("3 days").unwrap();
        assert_eq!(d.num_days(), 3);
    }

    #[test]
    fn parse_age_1_week() {
        let d = parse_age_duration("1 week").unwrap();
        assert_eq!(d.num_days(), 7);
    }

    #[test]
    fn parse_age_2_hours() {
        let d = parse_age_duration("2 hours").unwrap();
        assert_eq!(d.num_hours(), 2);
    }

    #[test]
    fn parse_age_30_minutes() {
        let d = parse_age_duration("30 minutes").unwrap();
        assert_eq!(d.num_minutes(), 30);
    }

    #[test]
    fn parse_age_1_month_approximated_as_30_days() {
        let d = parse_age_duration("1 month").unwrap();
        assert_eq!(d.num_days(), 30);
    }

    #[test]
    fn parse_age_unknown_returns_none() {
        assert!(parse_age_duration("bogus").is_none());
        assert!(parse_age_duration("3 fortnights").is_none());
    }

    // ── is_within_release_age ────────────────────────────────────────────────

    #[test]
    fn release_age_no_constraint_passes() {
        assert!(is_within_release_age(Some("2020-01-01T00:00:00Z"), None));
    }

    #[test]
    fn release_age_old_release_passes() {
        // Old release (2020) clearly satisfies "3 days" minimum
        assert!(is_within_release_age(
            Some("2020-01-01T00:00:00Z"),
            Some("3 days")
        ));
    }

    #[test]
    fn release_age_missing_timestamp_fails_open() {
        // No timestamp → can't check → fail-open (true)
        assert!(is_within_release_age(None, Some("3 days")));
    }

    #[test]
    fn release_age_future_release_fails() {
        // Far-future release timestamp doesn't satisfy minimum age
        assert!(!is_within_release_age(
            Some("2099-12-31T23:59:59Z"),
            Some("3 days")
        ));
    }

    // ── satisfies_date_range ──────────────────────────────────────────────────

    #[test]
    fn date_range_gt_old_timestamp_is_true() {
        // A timestamp from 2020 is clearly "> 3 days" old
        assert!(satisfies_date_range("2020-01-01T00:00:00Z", "> 3 days"));
    }

    #[test]
    fn date_range_gt_future_timestamp_is_false() {
        // A timestamp from the far future is NOT "> 3 days" old
        assert!(!satisfies_date_range("2099-01-01T00:00:00Z", "> 3 days"));
    }

    #[test]
    fn date_range_lt_recent_timestamp_is_true() {
        // Far-future timestamp is "newer than 5 years" → "< 5 years" is true
        // (only for very old threshold; let's use a near-future timestamp)
        // A timestamp 1 second in the future — not yet "3 days" old
        let just_now = (Utc::now() + chrono::Duration::seconds(5))
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string();
        assert!(satisfies_date_range(&just_now, "< 3 days"));
    }

    #[test]
    fn date_range_lt_old_timestamp_is_false() {
        // A 2020 timestamp is NOT "< 3 days" old
        assert!(!satisfies_date_range("2020-01-01T00:00:00Z", "< 3 days"));
    }

    #[test]
    fn date_range_gte_old_is_true() {
        assert!(satisfies_date_range("2020-01-01T00:00:00Z", ">= 1 week"));
    }

    #[test]
    fn date_range_lte_future_is_true() {
        let just_now = (Utc::now() + chrono::Duration::seconds(5))
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string();
        assert!(satisfies_date_range(&just_now, "<= 1 week"));
    }

    #[test]
    fn date_range_invalid_operator_returns_false() {
        assert!(!satisfies_date_range("2020-01-01T00:00:00Z", "== 3 days"));
    }

    #[test]
    fn date_range_invalid_timestamp_returns_false() {
        assert!(!satisfies_date_range("not-a-date", "> 3 days"));
    }

    #[test]
    fn date_range_naive_timestamp_accepted_with_z_suffix() {
        // PyPI-style naive timestamp gets Z appended internally
        assert!(satisfies_date_range("2020-01-01T00:00:00", "> 3 days"));
    }

    // ── is_within_schedule_tz_at ─────────────────────────────────────────────

    #[test]
    fn timezone_shifts_hour_correctly() {
        // UTC midnight on a Wednesday (weekday=3 in Unix).
        // In America/New_York (UTC-5 during standard time) it's Tuesday 19:00.
        let utc_midnight_wed = utc(2026, 1, 7, 0); // 2026-01-07 is a Wednesday
        // Schedule that only fires Mon-Fri 9am-5pm in UTC: should be false at UTC midnight
        let schedule_utc = vec!["* 9-17 * * 1-5".to_owned()];
        assert!(
            !is_within_schedule_tz_at(&schedule_utc, None, utc_midnight_wed),
            "UTC midnight should be outside 9-17 UTC"
        );
        // With timezone = America/New_York, UTC midnight = NY Tuesday 19:00 — still outside
        assert!(
            !is_within_schedule_tz_at(&schedule_utc, Some("America/New_York"), utc_midnight_wed),
            "NY 19:00 should be outside 9-17"
        );
    }

    #[test]
    fn timezone_fires_during_local_business_hours() {
        // UTC 14:00 on a Wednesday = America/New_York 09:00 (EST, UTC-5)
        let utc_2pm_wed = utc(2026, 1, 7, 14);
        let schedule = vec!["* 9-17 * * 1-5".to_owned()];
        // In UTC 14:00 is within 9-17 window
        assert!(
            is_within_schedule_tz_at(&schedule, None, utc_2pm_wed),
            "UTC 14:00 is within 9-17"
        );
        // In America/New_York (UTC-5 EST) it is 09:00 — also within 9-17
        assert!(
            is_within_schedule_tz_at(&schedule, Some("America/New_York"), utc_2pm_wed),
            "NY 09:00 (UTC 14:00) is within 9-17"
        );
    }

    #[test]
    fn timezone_unknown_tz_falls_back_to_utc() {
        let utc_10am_wed = utc(2026, 1, 7, 10);
        let schedule = vec!["* 9-11 * * *".to_owned()];
        // Unknown timezone → UTC fallback → UTC 10:00 matches 9-11
        assert!(
            is_within_schedule_tz_at(&schedule, Some("Not/AReal_TZ"), utc_10am_wed),
            "unknown timezone falls back to UTC, 10am UTC matches 9-11"
        );
    }

    #[test]
    fn text_schedule_respects_timezone() {
        // "after 9am" = hour >= 9
        // UTC 08:00 on a weekday = NOT within "after 9am" UTC
        let utc_8am = utc(2026, 1, 7, 8); // Wednesday
        let schedule = vec!["after 9am".to_owned()];
        assert!(
            !is_within_schedule_tz_at(&schedule, None, utc_8am),
            "UTC 08:00 is before 9am UTC"
        );
        // With UTC+2 timezone (e.g. Europe/Berlin standard winter) it would be 10:00 → matches
        assert!(
            is_within_schedule_tz_at(&schedule, Some("Europe/Berlin"), utc_8am),
            "Berlin 10:00 (UTC+2 CET) is after 9am"
        );
    }
}
