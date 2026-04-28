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

use chrono::{DateTime, Datelike, Timelike, Utc, Weekday};

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
    if schedule.is_empty() || schedule.iter().any(|s| s == "at any time" || s.is_empty()) {
        return true;
    }

    let hour = now.hour() as u8;
    let dom = now.day() as u8;
    let month = now.month() as u8;
    // chrono: Monday=1..Sunday=7; convert to Unix: Sunday=0..Saturday=6
    let weekday: u8 = match now.weekday() {
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
        Weekday::Sun => 0,
    };

    schedule
        .iter()
        .any(|entry| cron_matches(entry, hour, dom, month, weekday))
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
    // Normalise: if any part says "7" treat it as "0" (Sunday).
    let normalised = field.replace('7', "0");
    cron_field_matches(&normalised, weekday, 0, 6)
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
        // First entry: hours 0-3; second entry: Saturdays.
        let sched = vec!["* 0-3 * * *".to_owned(), "* * * * 6".to_owned()];
        // 2024-04-15 10am Monday — neither matches
        assert!(!is_within_schedule_at(&sched, utc(2024, 4, 15, 10)));
        // 2024-04-15 2am Monday — first matches
        assert!(is_within_schedule_at(&sched, utc(2024, 4, 15, 2)));
        // 2024-04-20 Saturday 10am — second matches
        assert!(is_within_schedule_at(&sched, utc(2024, 4, 20, 10)));
    }
}
