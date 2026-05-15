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

use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Timelike, Utc, Weekday};
use chrono_tz::Tz;

/// Parse a Renovate `minimumReleaseAge` string into a [`Duration`].
///
/// Supported units: `second`/`seconds`/`sec`/`secs`, `minute`/`minutes`/`min`/`mins`,
/// `hour`/`hours`/`hr`/`hrs`, `day`/`days`, `week`/`weeks`, `month`/`months`,
/// `year`/`years`.
///
/// A "month" is approximated as 30 days and a "year" as 365 days. Returns
/// `None` for unrecognised strings.
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
        "year" | "years" => Duration::try_days(n * 365),
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
    let ts = if let Ok(date) = NaiveDate::parse_from_str(timestamp, "%Y-%m-%d") {
        date.and_hms_opt(0, 0, 0)
            .map(|naive| DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc))
    } else {
        let ts_str = if timestamp.ends_with('Z') || timestamp.contains('+') {
            timestamp.to_owned()
        } else {
            format!("{timestamp}Z")
        };
        ts_str.parse::<DateTime<Utc>>().ok()
    };
    let Some(ts) = ts else {
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

/// Validate an IANA timezone name, mirroring Renovate's `hasValidTimezone()`.
///
/// Returns `true` if `timezone` is a valid IANA timezone name recognised by
/// `chrono-tz`, `false` otherwise.
pub fn has_valid_timezone(timezone: &str) -> bool {
    timezone.parse::<chrono_tz::Tz>().is_ok()
}

/// Validate a schedule list, mirroring Renovate's `hasValidSchedule()`.
///
/// Returns `true` when the schedule is usable:
/// - Empty schedule or `["at any time"]` → always valid.
/// - Each entry must be either valid cron syntax (5–6 fields, first field is
///   `*`) or a recognisable text schedule (has a time-of-day range and/or a
///   day-of-week/month constraint, and does not specify minutes).
///
/// Returns `false` when any entry fails validation.
pub fn is_valid_schedule(schedule: &[String]) -> bool {
    if schedule.is_empty()
        || schedule
            .iter()
            .any(|s| s.as_str() == "at any time" || s.is_empty())
    {
        return true;
    }
    schedule.iter().all(|entry| is_valid_schedule_entry(entry))
}

/// Validate a single schedule entry.
fn is_valid_schedule_entry(entry: &str) -> bool {
    let fields: Vec<&str> = entry.split_whitespace().collect();
    let n = fields.len();
    // 5-field cron or 6-field extended cron (Croner syntax: "* * * * * 6L")
    if n == 5 || n == 6 {
        let first = fields.first().copied().unwrap_or("");
        if first == "*" {
            return true;
        }
        // Non-wildcard minute in cron → invalid
        if first.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            return false;
        }
    }
    // 4 fields or fewer that look like cron → invalid
    if n <= 4
        && entry.split_whitespace().all(|f| {
            f.chars()
                .all(|c| c.is_ascii_digit() || c == '*' || c == '/' || c == '-' || c == ',')
        })
    {
        return false;
    }

    let raw = entry.trim().to_lowercase();
    // Apply Renovate schedule mappings (scheduleMappings in TypeScript).
    let s: String = match raw.as_str() {
        "every month" | "monthly" => "before 5am on the first day of the month".to_owned(),
        _ => raw.clone(),
    };
    let s = s.as_str();

    // Uses minute keywords → invalid
    if s.contains(" mins") || s.contains(" minutes") || s.contains(" min ") {
        return false;
    }

    // Special recognised text forms that lack explicit time/day but are valid.
    if s.contains("first day of the month") {
        return true;
    }
    if extract_named_month(s).is_some() || extract_every_n_months(s).is_some() {
        return true;
    }

    // Must have at least one recognised time or day constraint.
    let (_, has_time) = evaluate_time_constraint(s, 0);
    let (_, has_day) = evaluate_day_constraint(s, 0);
    has_time || has_day
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

    let (hour, dom, month, weekday, year) = match timezone.and_then(|tz| tz.parse::<Tz>().ok()) {
        Some(tz) => {
            let local = tz.from_utc_datetime(&now.naive_utc());
            (
                local.hour() as u8,
                local.day() as u8,
                local.month() as u8,
                weekday_to_unix(local.weekday()),
                local.year(),
            )
        }
        None => (
            now.hour() as u8,
            now.day() as u8,
            now.month() as u8,
            weekday_to_unix(now.weekday()),
            now.year(),
        ),
    };

    schedule.iter().any(|entry| {
        if looks_like_cron(entry) {
            cron_matches_year(entry, hour, dom, month, weekday, year)
        } else {
            text_schedule_matches_month(entry, hour, dom, weekday, month)
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

/// Number of days in a month (approximate: uses 28 for February without leap check).
fn days_in_month(month: u8, year: i32) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
            if leap { 29 } else { 28 }
        }
        _ => 30,
    }
}

/// Return `true` if `cron_expr` matches the given time components.
///
/// The minute field (first field) is accepted but ignored — Renovate schedule
/// presets always use `*` for minutes and check at any-minute granularity.
/// Extended Croner syntax (`L` for last day, `4L` for last weekday, `1#2` for
/// Nth weekday) is supported when `year` is provided.
pub fn cron_matches(cron_expr: &str, hour: u8, dom: u8, month: u8, weekday: u8) -> bool {
    cron_matches_year(cron_expr, hour, dom, month, weekday, 0)
}

fn cron_matches_year(
    cron_expr: &str,
    hour: u8,
    dom: u8,
    month: u8,
    weekday: u8,
    year: i32,
) -> bool {
    let fields: Vec<&str> = cron_expr.split_whitespace().collect();
    let n = fields.len();
    if n != 5 && n != 6 {
        return false;
    }
    // For 6-field extended cron, treat fields as [sec, min, hour, dom, month, weekday]
    // but we ignore seconds and minutes.
    let (hour_f, dom_f, month_f, weekday_f) = if n == 6 {
        (fields[2], fields[3], fields[4], fields[5])
    } else {
        // fields: [minute, hour, dom, month, weekday]
        (fields[1], fields[2], fields[3], fields[4])
    };

    if !cron_field_matches(hour_f, hour, 0, 23) {
        return false;
    }
    // Month field: handle 'L' (Croner interprets "* * * L *" as "last day of month"
    // even though L appears in the 4th/month field position in the 5-field format).
    if month_f == "L" {
        let last = days_in_month(month, year);
        if dom != last {
            return false;
        }
    } else if !cron_field_matches(month_f, month, 1, 12) {
        return false;
    }
    // DOM field: handle 'L' (last day of month)
    if dom_f == "L" {
        let last = days_in_month(month, year);
        if dom != last {
            return false;
        }
    } else if !cron_field_matches(dom_f, dom, 1, 31) {
        return false;
    }
    // Weekday field: handle `NL` (last Nth weekday) and `N#M` (Mth Nth weekday)
    if !cron_field_matches_weekday_ext(weekday_f, weekday, dom, month, year) {
        return false;
    }
    true
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

/// Extended weekday field matching: handles plain values AND `NL` (last Nth weekday of month)
/// and `N#M` (Mth occurrence of weekday N in month).
fn cron_field_matches_weekday_ext(field: &str, weekday: u8, dom: u8, month: u8, year: i32) -> bool {
    // `NL` — last occurrence of weekday N in the month
    if let Some(wd_str) = field.strip_suffix('L') {
        let wd_num: u8 = wd_str.parse().unwrap_or(255);
        let target_wd = if wd_num == 7 { 0 } else { wd_num };
        if weekday != target_wd {
            return false;
        }
        // Last occurrence: dom + 7 > days_in_month
        let last = days_in_month(month, year);
        return dom + 7 > last;
    }
    // `N#M` — Mth occurrence of weekday N
    if let Some((wd_str, occurrence_str)) = field.split_once('#') {
        let wd_num: u8 = wd_str.parse().unwrap_or(255);
        let occurrence: u8 = occurrence_str.parse().unwrap_or(0);
        let target_wd = if wd_num == 7 { 0 } else { wd_num };
        if weekday != target_wd {
            return false;
        }
        // Mth occurrence: (dom - 1) / 7 + 1 == M (for dom ≥ 1)
        return (dom.saturating_sub(1)) / 7 + 1 == occurrence;
    }
    cron_field_matches_weekday(field, weekday)
}

/// Heuristic: does `entry` look like a cron expression (5 or 6 fields)?
/// Cron entries start with `*` or a digit.
fn looks_like_cron(entry: &str) -> bool {
    let first = entry.chars().next().unwrap_or(' ');
    let count = entry.split_whitespace().count();
    (first == '*' || first.is_ascii_digit()) && (count == 5 || count == 6)
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

/// Extract weekday values from a text schedule string that contains a day name.
fn extract_weekday_from_text(s: &str) -> Option<Vec<u8>> {
    for (name, val) in &[
        ("sunday", 0u8),
        ("monday", 1),
        ("tuesday", 2),
        ("wednesday", 3),
        ("thursday", 4),
        ("friday", 5),
        ("saturday", 6),
    ] {
        if s.contains(name) {
            return Some(vec![*val]);
        }
    }
    None
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
    text_schedule_matches_month(entry, hour, dom, weekday, 0)
}

/// Like [`text_schedule_matches`] but also considers the current month for
/// "of MonthName" and "every N months" patterns.
///
/// Pass `month = 0` to skip month-based filtering (backward compatibility).
pub fn text_schedule_matches_month(entry: &str, hour: u8, dom: u8, weekday: u8, month: u8) -> bool {
    let s = entry.trim().to_lowercase();

    // "at any time" / "" — always match
    if s.is_empty() || s == "at any time" {
        return true;
    }

    // "on [weekday] on the [N]th day instance" — Nth occurrence of weekday in the month
    if s.contains("day instance") {
        let occurrence: u8 = if s.contains("first") {
            1
        } else if s.contains("second") {
            2
        } else if s.contains("third") {
            3
        } else if s.contains("fourth") {
            4
        } else {
            return true; // unknown instance, fail-open
        };
        // Find the weekday name in the schedule
        let day_ok = if let Some(days) = extract_weekday_from_text(&s) {
            days.contains(&weekday)
        } else {
            true
        };
        if !day_ok {
            return false;
        }
        // Check that this is the Nth occurrence: (dom - 1) / 7 + 1 == occurrence
        return (dom.saturating_sub(1)) / 7 + 1 == occurrence;
    }

    // "on the first day of the month"
    if s.contains("first day of the month") {
        let time_ok = if let Some(before_part) = extract_before_time(&s) {
            hour < before_part
        } else {
            true
        };
        // Check "every N months on the first day" — month constraint
        if month > 0
            && let Some(n) = extract_every_n_months(&s)
            && (month as u32) % (n as u32) != 1 % (n as u32)
        {
            return false;
        }
        return dom == 1 && time_ok;
    }

    // "of MonthName" — only in the specified month
    if month > 0 {
        if let Some(named_month) = extract_named_month(&s) {
            if month != named_month {
                return false;
            }
            // No time/day constraint → just month constraint
            let (time_ok, has_time) = evaluate_time_constraint(&s, hour);
            let (day_ok, has_day) = evaluate_day_constraint(&s, weekday);
            return match (has_time, has_day) {
                (true, true) => time_ok && day_ok,
                (true, false) => time_ok,
                (false, true) => day_ok,
                (false, false) => true,
            };
        }

        // "every N months" — month is in the N-month cycle
        if let Some(n) = extract_every_n_months(&s) {
            if (month as u32) % (n as u32) != 1 % (n as u32) {
                return false;
            }
            // No additional time/day constraint
            return true;
        }
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

/// Return the month number (1-12) if `s` contains "of <MonthName>".
fn extract_named_month(s: &str) -> Option<u8> {
    const MONTHS: &[(&str, u8)] = &[
        ("january", 1),
        ("february", 2),
        ("march", 3),
        ("april", 4),
        ("may", 5),
        ("june", 6),
        ("july", 7),
        ("august", 8),
        ("september", 9),
        ("october", 10),
        ("november", 11),
        ("december", 12),
    ];
    for (name, num) in MONTHS {
        if s.contains(&format!("of {name}")) {
            return Some(*num);
        }
    }
    None
}

/// Return N if `s` contains "every N months".
fn extract_every_n_months(s: &str) -> Option<u8> {
    let idx = s.find("every ")?;
    let rest = &s[idx + "every ".len()..];
    let tok = rest.split_whitespace().next()?;
    let n: u8 = tok.parse().ok()?;
    if rest.contains("month") && n > 1 {
        Some(n)
    } else {
        None
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

    fn schedule(entries: &[&str]) -> Vec<String> {
        entries.iter().map(|entry| (*entry).to_owned()).collect()
    }

    fn assert_schedule_cases(entries: &[&str], cases: &[(DateTime<Utc>, bool)]) {
        let sched = schedule(entries);
        for (datetime, expected) in cases {
            assert_eq!(
                is_within_schedule_at(&sched, *datetime),
                *expected,
                "schedule {sched:?} at {datetime}"
            );
        }
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

    // Ported: "returns true if at any time array" — workers/repository/update/branch/schedule.spec.ts line 165
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

    // Ported: "$datetime" — config/presets/internal/schedule.spec.ts line 19
    #[test]
    fn schedule_preset_daily_matches_upstream_cases() {
        assert_schedule_cases(
            &["* 0-3 * * *"],
            &[
                (utc(2017, 6, 30, 0), true),
                (utc(2017, 6, 30, 1), true),
                (utc(2017, 6, 30, 2), true),
                (utc(2017, 6, 30, 3), true),
                (utc(2017, 6, 30, 4), false),
            ],
        );
    }

    // Ported: "$datetime" — config/presets/internal/schedule.spec.ts line 34
    #[test]
    fn schedule_preset_early_mondays_matches_upstream_cases() {
        assert_schedule_cases(
            &["* 0-3 * * 1"],
            &[
                (utc(2017, 6, 26, 0), true),
                (utc(2017, 6, 26, 1), true),
                (utc(2017, 6, 26, 2), true),
                (utc(2017, 6, 26, 3), true),
                (utc(2017, 6, 26, 4), false),
                (utc(2017, 6, 30, 0), false),
            ],
        );
    }

    // Ported: "$datetime" — config/presets/internal/schedule.spec.ts line 50
    #[test]
    fn schedule_preset_monthly_matches_upstream_cases() {
        assert_schedule_cases(
            &["* 0-3 1 * *"],
            &[
                (utc(2017, 6, 1, 0), true),
                (utc(2017, 6, 1, 1), true),
                (utc(2017, 6, 1, 2), true),
                (utc(2017, 6, 1, 3), true),
                (utc(2017, 6, 1, 4), false),
                (utc(2017, 6, 2, 0), false),
            ],
        );
    }

    // Ported: "$datetime" — config/presets/internal/schedule.spec.ts line 66
    #[test]
    fn schedule_preset_non_office_hours_matches_upstream_cases() {
        assert_schedule_cases(
            &["* 0-4,22-23 * * 1-5", "* * * * 0,6"],
            &[
                (utc(2017, 6, 1, 0), true),
                (utc(2017, 6, 1, 1), true),
                (utc(2017, 6, 1, 2), true),
                (utc(2017, 6, 1, 3), true),
                (utc(2017, 6, 1, 4), true),
                (utc(2017, 6, 1, 10), false),
                (utc(2017, 6, 1, 11), false),
                (utc(2017, 6, 1, 22), true),
                (utc(2017, 6, 1, 23), true),
                (utc(2017, 6, 3, 9), true),
            ],
        );
    }

    // Ported: "$datetime" — config/presets/internal/schedule.spec.ts line 86
    #[test]
    fn schedule_preset_office_hours_matches_upstream_cases() {
        assert_schedule_cases(
            &["* 8-17 * * 1-5"],
            &[
                (utc(2017, 6, 1, 7), false),
                (utc(2017, 6, 1, 8), true),
                (utc(2017, 6, 1, 12), true),
                (utc(2017, 6, 1, 17), true),
                (utc(2017, 6, 1, 18), false),
                (utc(2017, 6, 2, 7), false),
                (utc(2017, 6, 2, 8), true),
                (utc(2017, 6, 2, 12), true),
                (utc(2017, 6, 2, 17), true),
                (utc(2017, 6, 2, 18), false),
                (utc(2017, 6, 3, 7), false),
                (utc(2017, 6, 3, 8), false),
                (utc(2017, 6, 3, 12), false),
                (utc(2017, 6, 3, 17), false),
                (utc(2017, 6, 3, 18), false),
                (utc(2017, 6, 4, 7), false),
                (utc(2017, 6, 4, 8), false),
                (utc(2017, 6, 4, 12), false),
                (utc(2017, 6, 4, 17), false),
                (utc(2017, 6, 4, 18), false),
            ],
        );
    }

    // Ported: "$datetime" — config/presets/internal/schedule.spec.ts line 119
    #[test]
    fn schedule_preset_quarterly_matches_upstream_cases() {
        assert_schedule_cases(
            &["* * 1 */3 *"],
            &[
                (utc(2017, 1, 1, 0), true),
                (utc(2017, 1, 2, 0), false),
                (utc(2017, 4, 1, 1), true),
                (utc(2017, 7, 1, 2), true),
                (utc(2017, 10, 1, 3), true),
                (utc(2017, 2, 1, 4), false),
            ],
        );
    }

    // Ported: "$datetime" — config/presets/internal/schedule.spec.ts line 135
    #[test]
    fn schedule_preset_weekdays_matches_upstream_cases() {
        assert_schedule_cases(
            &["* * * * 1-5"],
            &[
                (utc(2017, 6, 1, 0), true),
                (utc(2017, 6, 2, 1), true),
                (utc(2017, 6, 3, 2), false),
                (utc(2017, 6, 4, 3), false),
                (utc(2017, 6, 5, 4), true),
                (utc(2017, 6, 6, 10), true),
                (utc(2017, 6, 7, 11), true),
            ],
        );
    }

    // Ported: "$datetime" — config/presets/internal/schedule.spec.ts line 152
    #[test]
    fn schedule_preset_weekends_matches_upstream_cases() {
        assert_schedule_cases(
            &["* * * * 0,6"],
            &[
                (utc(2017, 6, 1, 0), false),
                (utc(2017, 6, 2, 1), false),
                (utc(2017, 6, 3, 2), true),
                (utc(2017, 6, 4, 3), true),
                (utc(2017, 6, 5, 4), false),
                (utc(2017, 6, 6, 10), false),
                (utc(2017, 6, 7, 11), false),
            ],
        );
    }

    // Ported: "$datetime" — config/presets/internal/schedule.spec.ts line 169
    #[test]
    fn schedule_preset_yearly_matches_upstream_cases() {
        assert_schedule_cases(
            &["* * 1 */12 *"],
            &[
                (utc(2017, 1, 1, 0), true),
                (utc(2017, 2, 2, 1), false),
                (utc(2018, 1, 1, 2), true),
            ],
        );
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

    // ── Ported from schedule.spec.ts (isScheduledNow at 2017-06-30 10:50am, a Friday) ──

    // Ported: "supports before hours true" — workers/repository/update/branch/schedule.spec.ts line 184
    #[test]
    fn spec_supports_before_hours_true() {
        // "returns true if before hours" — at 10am, "before 4:00pm" is true
        let friday_10am = utc(2017, 6, 30, 10);
        let sched = vec!["before 4:00pm".to_owned()];
        assert!(
            is_within_schedule_at(&sched, friday_10am),
            "10am is before 4:00pm"
        );
    }

    // Ported: "supports before hours false" — workers/repository/update/branch/schedule.spec.ts line 190
    #[test]
    fn spec_supports_before_hours_false() {
        // "returns false if before hours" — at 10am, "before 4:00am" is false
        let friday_10am = utc(2017, 6, 30, 10);
        let sched = vec!["before 4:00am".to_owned()];
        assert!(
            !is_within_schedule_at(&sched, friday_10am),
            "10am is NOT before 4:00am"
        );
    }

    // Ported: "supports outside hours" — workers/repository/update/branch/schedule.spec.ts line 202
    #[test]
    fn spec_supports_outside_hours() {
        // "returns false for outside hours" — at 10am, "after 4:00pm" is false
        let friday_10am = utc(2017, 6, 30, 10);
        let sched = vec!["after 4:00pm".to_owned()];
        assert!(
            !is_within_schedule_at(&sched, friday_10am),
            "10am is NOT after 4:00pm"
        );
    }

    // Ported: "supports cron syntax with hours" — workers/repository/update/branch/schedule.spec.ts line 208
    #[test]
    fn spec_cron_with_hours_match() {
        // "supports cron syntax with hours" — * 10 * * * at hour=10 matches
        let friday_10am = utc(2017, 6, 30, 10);
        let sched_match = vec!["* 10 * * *".to_owned()];
        let sched_no_match = vec!["* 11 * * *".to_owned()];
        assert!(is_within_schedule_at(&sched_match, friday_10am));
        assert!(!is_within_schedule_at(&sched_no_match, friday_10am));
    }

    // Ported: "supports cron syntax with days" — workers/repository/update/branch/schedule.spec.ts line 218
    #[test]
    fn spec_cron_with_days_match() {
        // "supports cron syntax with days" — * * 30 * * on day=30 matches
        let friday_10am = utc(2017, 6, 30, 10);
        let sched_match = vec!["* * 30 * *".to_owned()];
        let sched_no_match = vec!["* * 1 * *".to_owned()];
        assert!(is_within_schedule_at(&sched_match, friday_10am));
        assert!(!is_within_schedule_at(&sched_no_match, friday_10am));
    }

    // Ported: "supports cron syntax with months" — workers/repository/update/branch/schedule.spec.ts line 228
    #[test]
    fn spec_cron_with_months_match() {
        // "supports cron syntax with months" — * * * 6 * on month=6 (June) matches
        let friday_10am = utc(2017, 6, 30, 10);
        let sched_match = vec!["* * * 6 *".to_owned()];
        let sched_no_match = vec!["* * * 7 *".to_owned()];
        assert!(is_within_schedule_at(&sched_match, friday_10am));
        assert!(!is_within_schedule_at(&sched_no_match, friday_10am));
    }

    // Ported: "supports cron syntax with weekdays" — workers/repository/update/branch/schedule.spec.ts line 238
    #[test]
    fn spec_cron_with_weekdays_match() {
        // "supports cron syntax with weekdays" — * * * * 5 on weekday=5 (Friday) matches
        let friday_10am = utc(2017, 6, 30, 10);
        let sched_match = vec!["* * * * 5".to_owned()];
        let sched_no_match = vec!["* * * * 6".to_owned()];
        assert!(is_within_schedule_at(&sched_match, friday_10am));
        assert!(!is_within_schedule_at(&sched_no_match, friday_10am));
    }

    // Ported: "returns true if no schedule" — workers/repository/update/branch/schedule.spec.ts line 154
    #[test]
    fn spec_returns_true_if_no_schedule() {
        // "returns true if no schedule" — empty schedule means always scheduled
        let friday_10am = utc(2017, 6, 30, 10);
        assert!(is_within_schedule_at(&[], friday_10am));
    }

    // Ported: "returns true if at any time" — workers/repository/update/branch/schedule.spec.ts line 159
    #[test]
    fn spec_returns_true_for_at_any_time() {
        // "returns true if at any time"
        let friday_10am = utc(2017, 6, 30, 10);
        let sched = vec!["at any time".to_owned()];
        assert!(is_within_schedule_at(&sched, friday_10am));
    }

    // Ported: "approves if the weekday is 0" — workers/repository/update/branch/schedule.spec.ts line 259
    #[test]
    fn spec_cron_on_sunday_weekday_0() {
        // "approves if the weekday is 0" — Sunday matches weekday 0
        let sunday_10am = utc(2023, 1, 8, 10);
        let sched = vec!["* * * * 0".to_owned()];
        assert!(is_within_schedule_at(&sched, sunday_10am));
    }

    // Ported: "rejects if the weekday is 1" — workers/repository/update/branch/schedule.spec.ts line 265
    #[test]
    fn spec_cron_on_sunday_rejects_weekday_1() {
        // "rejects if the weekday is 1" — Sunday does NOT match Monday
        let sunday_10am = utc(2023, 1, 8, 10);
        let sched = vec!["* * * * 1".to_owned()];
        assert!(!is_within_schedule_at(&sched, sunday_10am));
    }

    // Ported: "supports multiple schedules" — workers/repository/update/branch/schedule.spec.ts line 355
    #[test]
    fn spec_supports_multiple_schedules() {
        // "supports multiple schedules" — any one matching is sufficient
        // At 10am: "after 4:00pm" → false, "before 11:00am" → true → overall true
        let friday_10am = utc(2017, 6, 30, 10);
        let sched = vec!["after 4:00pm".to_owned(), "before 11:00am".to_owned()];
        assert!(is_within_schedule_at(&sched, friday_10am));
    }

    // Ported: "supports day match" — workers/repository/update/branch/schedule.spec.ts line 361
    #[test]
    fn spec_supports_day_match_friday() {
        // "supports day match" — on friday and saturday matches at Friday
        let friday_10am = utc(2017, 6, 30, 10);
        let sched = vec!["on friday and saturday".to_owned()];
        assert!(is_within_schedule_at(&sched, friday_10am));
    }

    // Ported: "supports day mismatch" — workers/repository/update/branch/schedule.spec.ts line 367
    #[test]
    fn spec_supports_day_mismatch() {
        // "supports day mismatch" — on monday and tuesday does NOT match Friday
        let friday_10am = utc(2017, 6, 30, 10);
        let sched = vec!["on monday and tuesday".to_owned()];
        assert!(!is_within_schedule_at(&sched, friday_10am));
    }

    // Ported: "supports every weekday" — workers/repository/update/branch/schedule.spec.ts line 373
    #[test]
    fn spec_every_weekday_matches_friday() {
        // "supports every weekday" — Friday is a weekday → matches
        let friday_10am = utc(2017, 6, 30, 10);
        let sched = vec!["every weekday".to_owned()];
        assert!(is_within_schedule_at(&sched, friday_10am));
    }

    // Ported: "supports every weekend" — workers/repository/update/branch/schedule.spec.ts line 379
    #[test]
    fn spec_every_weekend_rejects_friday() {
        // "supports every weekend" — Friday is not a weekend → false
        let friday_10am = utc(2017, 6, 30, 10);
        let sched = vec!["every weekend".to_owned()];
        assert!(!is_within_schedule_at(&sched, friday_10am));
    }

    // Ported: "supports every weekday with time" — workers/repository/update/branch/schedule.spec.ts line 385
    #[test]
    fn spec_before_11am_every_weekday_matches() {
        // "supports every weekday with time" — before 11am on a weekday, at 10am → true
        let friday_10am = utc(2017, 6, 30, 10);
        let sched = vec!["before 11:00am every weekday".to_owned()];
        assert!(is_within_schedule_at(&sched, friday_10am));
    }

    // Ported: "reject if day mismatch" — workers/repository/update/branch/schedule.spec.ts line 337
    #[test]
    fn spec_cron_dom_mismatch_false() {
        // "reject if day mismatch" — * 10 21 * * on dom=30 → false
        let friday_10am = utc(2017, 6, 30, 10);
        let sched = vec!["* 10 21 * *".to_owned()];
        assert!(!is_within_schedule_at(&sched, friday_10am));
    }

    // Ported: "reject if month mismatch" — workers/repository/update/branch/schedule.spec.ts line 343
    #[test]
    fn spec_cron_month_mismatch_false() {
        // "reject if month mismatch" — * 10 30 1 * on month=6 → false
        let friday_10am = utc(2017, 6, 30, 10);
        let sched = vec!["* 10 30 1 *".to_owned()];
        assert!(!is_within_schedule_at(&sched, friday_10am));
    }

    // Ported: "rejects first day of the month" — workers/repository/update/branch/schedule.spec.ts line 397
    #[test]
    fn spec_first_day_of_month_rejects_non_first() {
        // "rejects first day of the month" — June 30 is not the first day
        let friday_10am = utc(2017, 6, 30, 10);
        let sched = vec!["before 11am on the first day of the month".to_owned()];
        assert!(!is_within_schedule_at(&sched, friday_10am));
    }

    // Ported: "approves first day of the month" — workers/repository/update/branch/schedule.spec.ts line 403
    #[test]
    fn spec_first_day_of_month_approves_first() {
        // "approves first day of the month" — October 1, 2017 05:26am
        let oct_1_5am = utc(2017, 10, 1, 5);
        let sched = vec!["before 11am on the first day of the month".to_owned()];
        assert!(is_within_schedule_at(&sched, oct_1_5am));
    }

    // Ported: "approves on months of year" — workers/repository/update/branch/schedule.spec.ts line 424
    #[test]
    fn spec_months_of_year_approves_january() {
        // "approves on months of year" — "of January" in January
        let jan_2_6am = utc(2017, 1, 2, 6);
        let sched = vec!["of January".to_owned()];
        assert!(is_within_schedule_at(&sched, jan_2_6am));
    }

    // Ported: "rejects on months of year" — workers/repository/update/branch/schedule.spec.ts line 431
    #[test]
    fn spec_months_of_year_rejects_february() {
        // "rejects on months of year" — "of January" in February → false
        let feb_2_6am = utc(2017, 2, 2, 6);
        let sched = vec!["of January".to_owned()];
        assert!(!is_within_schedule_at(&sched, feb_2_6am));
    }

    // Ported: "approves schedule longer than 1 month" — workers/repository/update/branch/schedule.spec.ts line 438
    #[test]
    fn spec_every_3_months_approves_july() {
        // "approves schedule longer than 1 month" — "every 3 months" in July 2017
        // July is the 7th month: 7 % 3 != 0 so quarterly (Jan, Apr, Jul, Oct)
        let jul_1_6am = utc(2017, 7, 1, 6);
        let sched = vec!["every 3 months".to_owned()];
        assert!(is_within_schedule_at(&sched, jul_1_6am));
    }

    // Ported: "rejects schedule longer than 1 month" — workers/repository/update/branch/schedule.spec.ts line 445
    #[test]
    fn spec_every_6_months_rejects_february() {
        // "rejects schedule longer than 1 month" — "every 6 months" in Feb (not a 6-month boundary)
        let feb_1_6am = utc(2017, 2, 1, 6);
        let sched = vec!["every 6 months".to_owned()];
        assert!(!is_within_schedule_at(&sched, feb_1_6am));
    }

    // Ported: "approves if the weekday is *" — workers/repository/update/branch/schedule.spec.ts line 253
    #[test]
    fn spec_cron_on_sunday_wildcard_matches() {
        // "* * * * *" on Sunday (2023-01-08) at 10:50am → true
        let sunday_10am = utc(2023, 1, 8, 10);
        let sched = vec!["* * * * *".to_owned()];
        assert!(is_within_schedule_at(&sched, sunday_10am));
    }

    // Ported: "reject if no schedule available" — workers/repository/update/branch/schedule.spec.ts line 349
    #[test]
    fn spec_cron_no_schedule_available_false() {
        // "* * * 1 *" (January only) on June 30, 2017 (month=6) → false
        let friday_10am = utc(2017, 6, 30, 10);
        let sched = vec!["* * * 1 *".to_owned()];
        assert!(!is_within_schedule_at(&sched, friday_10am));
    }

    // Ported: "supports o every weekday" — workers/repository/update/branch/schedule.spec.ts line 391
    #[test]
    fn spec_supports_o_every_weekday() {
        // typo "on inevery weekday" still matches because "every weekday" is a substring
        let friday_10am = utc(2017, 6, 30, 10);
        let sched = vec!["before 11:00am on inevery weekday".to_owned()];
        assert!(is_within_schedule_at(&sched, friday_10am));
    }

    // Ported: "approves schedule longer than 1 month with day of month" — workers/repository/update/branch/schedule.spec.ts line 452
    #[test]
    fn spec_every_3_months_first_day_approves_july_1() {
        let jul_1_6am = utc(2017, 7, 1, 6);
        let sched = vec!["every 3 months on the first day of the month".to_owned()];
        assert!(is_within_schedule_at(&sched, jul_1_6am));
    }

    // Ported: "rejects schedule longer than 1 month with day of month" — workers/repository/update/branch/schedule.spec.ts line 459
    #[test]
    fn spec_every_3_months_first_day_rejects_february() {
        let feb_1_6am = utc(2017, 2, 1, 6);
        let sched = vec!["every 3 months on the first day of the month".to_owned()];
        assert!(!is_within_schedule_at(&sched, feb_1_6am));
    }

    // Ported: "$sched, $tz, $datetime" — workers/repository/update/branch/schedule.spec.ts line 319
    #[test]
    fn spec_timezone_text_after_4pm_singapore() {
        // 2017-06-30T15:59:00+0800 (local Singapore 15:59) → false (not yet after 4pm)
        // UTC equivalent: 2017-06-30T07:59:00Z → with SGT (+8) local = 15:00
        let sched = schedule(&["after 4pm"]);
        assert!(!is_within_schedule_tz_at(
            &sched,
            Some("Asia/Singapore"),
            utc(2017, 6, 30, 7)
        ));
        // 2017-06-30T16:01:00+0800 (local Singapore 16:01) → true (after 4pm)
        // UTC: 2017-06-30T08:01:00Z → local SGT 16:00
        assert!(is_within_schedule_tz_at(
            &sched,
            Some("Asia/Singapore"),
            utc(2017, 6, 30, 8)
        ));
    }

    // Ported: "$sched, $tz, $datetime" — workers/repository/update/branch/schedule.spec.ts line 319
    #[test]
    fn spec_timezone_text_before_4am_monday_tokyo() {
        // 2017-06-26T03:59:00+0900 (local Tokyo 03:59 Monday) → true
        // UTC: 2017-06-25T18:59:00Z → local JST 03:59 next day (Monday June 26)
        let sched = schedule(&["before 4am on Monday"]);
        assert!(is_within_schedule_tz_at(
            &sched,
            Some("Asia/Tokyo"),
            utc(2017, 6, 25, 18)
        ));
        // 2017-06-26T04:01:00+0900 (local Tokyo 04:01 Monday) → false (not before 4am)
        // UTC: 2017-06-25T19:01:00Z → local JST 04:01 Monday
        assert!(!is_within_schedule_tz_at(
            &sched,
            Some("Asia/Tokyo"),
            utc(2017, 6, 25, 19)
        ));
    }

    // Ported: "$sched, $tz, $datetime" — workers/repository/update/branch/schedule.spec.ts line 319
    #[test]
    fn spec_timezone_cron_16_23_singapore() {
        // "* 16-23 * * *" with Singapore: local hours 16-23 → UTC 08-15
        let sched = schedule(&["* 16-23 * * *"]);
        // 2017-06-30T07:59:00Z → local SGT 15:59 → hour=15 NOT in 16-23 → false
        assert!(!is_within_schedule_tz_at(
            &sched,
            Some("Asia/Singapore"),
            utc(2017, 6, 30, 7)
        ));
        // 2017-06-30T08:00:00Z → local SGT 16:00 → hour=16 IN 16-23 → true
        assert!(is_within_schedule_tz_at(
            &sched,
            Some("Asia/Singapore"),
            utc(2017, 6, 30, 8)
        ));
    }

    // Ported: "$sched, $tz, $datetime" — workers/repository/update/branch/schedule.spec.ts line 319
    #[test]
    fn spec_timezone_cron_0_3_monday_tokyo() {
        // "* 0-3 * * 1" with Tokyo: Monday hours 0-3 local → UTC Sunday 15-18
        let sched = schedule(&["* 0-3 * * 1"]);
        // 2017-06-25T18:58:00Z → local JST 03:58 Monday → hour=3 IN 0-3 AND weekday=1 → true
        assert!(is_within_schedule_tz_at(
            &sched,
            Some("Asia/Tokyo"),
            utc(2017, 6, 25, 18)
        ));
        // 2017-06-25T19:01:00Z → local JST 04:01 Monday → hour=4 NOT in 0-3 → false
        assert!(!is_within_schedule_tz_at(
            &sched,
            Some("Asia/Tokyo"),
            utc(2017, 6, 25, 19)
        ));
    }

    // ── is_valid_schedule (hasValidSchedule) ─────────────────────────────────

    fn sched(entries: &[&str]) -> Vec<String> {
        entries.iter().map(|s| s.to_string()).collect()
    }

    // Ported: "returns true for null" — workers/repository/update/branch/schedule.spec.ts line 17
    #[test]
    fn has_valid_schedule_null_returns_true() {
        assert!(is_valid_schedule(&[]));
    }

    // Ported: "returns true for at any time" — workers/repository/update/branch/schedule.spec.ts line 21
    #[test]
    fn has_valid_schedule_at_any_time_returns_true() {
        assert!(is_valid_schedule(&sched(&["at any time"])));
    }

    // Ported: "returns false for invalid schedule" — workers/repository/update/branch/schedule.spec.ts line 25
    #[test]
    fn has_valid_schedule_invalid_returns_false() {
        assert!(!is_valid_schedule(&sched(&["foo"])));
    }

    // Ported: "returns false if any schedule fails to parse" — workers/repository/update/branch/schedule.spec.ts line 29
    #[test]
    fn has_valid_schedule_any_failure_returns_false() {
        assert!(!is_valid_schedule(&sched(&["after 5:00pm", "foo"])));
    }

    // Ported: "returns false if using minutes" — workers/repository/update/branch/schedule.spec.ts line 33
    #[test]
    fn has_valid_schedule_minutes_returns_false() {
        assert!(!is_valid_schedule(&sched(&["every 15 mins every weekday"])));
    }

    // Ported: "returns false for wildcard minutes" — workers/repository/update/branch/schedule.spec.ts line 39
    #[test]
    fn has_valid_schedule_non_wildcard_cron_minute_returns_false() {
        let res = is_valid_schedule(&sched(&["1 * * * *"]));
        assert!(!res);
    }

    // Ported: "returns false if schedules have no days or time range" — workers/repository/update/branch/schedule.spec.ts line 47
    #[test]
    fn has_valid_schedule_no_days_or_time_returns_false() {
        assert!(!is_valid_schedule(&sched(&["at 5:00pm"])));
    }

    // Ported: "returns false if any schedule has no days or time range" — workers/repository/update/branch/schedule.spec.ts line 51
    #[test]
    fn has_valid_schedule_combined_any_failure_returns_false() {
        assert!(!is_valid_schedule(&sched(&["at 5:00pm", "on saturday"])));
    }

    // Ported: "returns false for every xday" — workers/repository/update/branch/schedule.spec.ts line 57
    #[test]
    fn has_valid_schedule_bare_weekday_returns_false() {
        assert!(!is_valid_schedule(&sched(&["every friday"])));
    }

    // Ported: "returns true if schedule has days of week" — workers/repository/update/branch/schedule.spec.ts line 61
    #[test]
    fn has_valid_schedule_days_of_week_returns_true() {
        assert!(is_valid_schedule(&sched(&["on friday and saturday"])));
    }

    // Ported: "returns true for multi day schedules" — workers/repository/update/branch/schedule.spec.ts line 67
    #[test]
    fn has_valid_schedule_multi_day_with_time_returns_true() {
        assert!(is_valid_schedule(&sched(&[
            "after 5:00pm on wednesday and thursday"
        ])));
    }

    // Ported: "returns true if schedule has a start time" — workers/repository/update/branch/schedule.spec.ts line 75
    #[test]
    fn has_valid_schedule_start_time_returns_true() {
        assert!(is_valid_schedule(&sched(&["after 8:00pm"])));
    }

    // Ported: "returns true for first day of the month" — workers/repository/update/branch/schedule.spec.ts line 79
    #[test]
    fn has_valid_schedule_first_day_of_month_returns_true() {
        assert!(is_valid_schedule(&sched(&[
            "on the first day of the month"
        ])));
    }

    // Ported: "returns true for schedules longer than 1 month" — workers/repository/update/branch/schedule.spec.ts line 85
    #[test]
    fn has_valid_schedule_multi_month_returns_true() {
        assert!(is_valid_schedule(&sched(&["every 3 months"])));
        assert!(is_valid_schedule(&sched(&["every 6 months"])));
        assert!(is_valid_schedule(&sched(&["every 12 months"])));
    }

    // Ported: "returns true if schedule has an end time" — workers/repository/update/branch/schedule.spec.ts line 91
    #[test]
    fn has_valid_schedule_end_time_returns_true() {
        assert!(is_valid_schedule(&sched(&["before 6:00am"])));
    }

    // Ported: "returns true if schedule has a start and end time" — workers/repository/update/branch/schedule.spec.ts line 95
    #[test]
    fn has_valid_schedule_start_and_end_time_returns_true() {
        assert!(is_valid_schedule(&sched(&[
            "after 11:00pm and before 6:00am"
        ])));
    }

    // Ported: "returns true if schedule has days and a start and end time" — workers/repository/update/branch/schedule.spec.ts line 101
    #[test]
    fn has_valid_schedule_days_with_start_and_end_time_returns_true() {
        assert!(is_valid_schedule(&sched(&[
            "after 11:00pm and before 6:00am every weekday"
        ])));
    }

    // Ported: "returns true if schedule uses cron syntax" — workers/repository/update/branch/schedule.spec.ts line 109
    #[test]
    fn has_valid_schedule_valid_cron_returns_true() {
        assert!(is_valid_schedule(&sched(&["* 5 * * *"])));
        assert!(is_valid_schedule(&sched(&["* * * * * 6L"])));
        assert!(is_valid_schedule(&sched(&["* * * */2 6#1"])));
        assert!(!is_valid_schedule(&sched(&["2 3 5 11 *"])));
        assert!(!is_valid_schedule(&sched(&["2 3 5 11"])));
    }

    // Ported: "massages schedules" — workers/repository/update/branch/schedule.spec.ts line 117
    #[test]
    fn has_valid_schedule_massaged_forms_return_true() {
        assert!(is_valid_schedule(&sched(&[
            "before 5am on the first day of the month"
        ])));
        assert!(is_valid_schedule(&sched(&["every month"])));
    }

    // Ported: "supports hours shorthand" — workers/repository/update/branch/schedule.spec.ts line 126
    #[test]
    fn has_valid_schedule_hours_shorthand_returns_true() {
        let schedules = sched(&[
            "after 11pm and before 6am every weekend",
            "after 11pm",
            "after 10pm and before 5:00am",
            "after 10pm and before 5am every weekday",
            "after 11pm and before 6am",
            "after 9pm on friday and saturday",
            "before 5am every weekday",
            "every weekend",
        ]);
        assert!(is_valid_schedule(&schedules));
    }

    // ── has_valid_timezone (hasValidTimezone) ────────────────────────────────

    // Ported: "returns false for invalid timezone" — workers/repository/update/branch/schedule.spec.ts line 7
    #[test]
    fn has_valid_timezone_invalid_returns_false() {
        assert!(!has_valid_timezone("Asia"));
    }

    // Ported: "returns true for valid timezone" — workers/repository/update/branch/schedule.spec.ts line 11
    #[test]
    fn has_valid_timezone_valid_returns_true() {
        assert!(has_valid_timezone("Asia/Singapore"));
    }

    // ── L and # cron syntax ──────────────────────────────────────────────────

    // Ported: "supports last day of month" — workers/repository/update/branch/schedule.spec.ts line 277
    #[test]
    fn spec_cron_l_syntax_last_day_of_month() {
        // 2024-10-31T10:50:00 → October 31 (last day of October)
        // "* * * L *" should match
        let oct_31 = utc(2024, 10, 31, 10);
        let sched = schedule(&["* * * L *"]);
        assert!(is_within_schedule_at(&sched, oct_31));
    }

    // Ported: "supports last day of week" — workers/repository/update/branch/schedule.spec.ts line 283
    #[test]
    fn spec_cron_l_syntax_last_day_of_week() {
        // 2024-10-31 is a Thursday (weekday=4). Last Thursday of October 2024.
        let oct_31_thu = utc(2024, 10, 31, 10);
        // "* * * * 4L" → last Thursday → should match
        assert!(is_within_schedule_at(
            &schedule(&["* * * * 4L"]),
            oct_31_thu
        ));
        // "* * * * 5L" → last Friday → Oct 31 is Thursday, not Friday → false
        assert!(!is_within_schedule_at(
            &schedule(&["* * * * 5L"]),
            oct_31_thu
        ));
    }

    // Ported: "supports first Monday of month" — workers/repository/update/branch/schedule.spec.ts line 293
    #[test]
    fn spec_cron_hash_syntax_first_monday_of_month() {
        // 2024-10-07T10:50:00 → October 7, 2024 is the first Monday of October
        let oct_7_mon = utc(2024, 10, 7, 10);
        // "* * * * 1#1" → first Monday → should match
        assert!(is_within_schedule_at(
            &schedule(&["* * * * 1#1"]),
            oct_7_mon
        ));
        // "* * * * 1#2" → second Monday → Oct 7 is first, not second → false
        assert!(!is_within_schedule_at(
            &schedule(&["* * * * 1#2"]),
            oct_7_mon
        ));
    }

    // Ported: "supports weekday instances" — workers/repository/update/branch/schedule.spec.ts line 466
    #[test]
    fn spec_weekday_instances_first_monday() {
        // "on Monday on the first day instance" = first Monday of the month
        let sched = schedule(&["on Monday on the first day instance"]);
        // 2017-02-01 is a Thursday → not Monday → false
        assert!(!is_within_schedule_at(&sched, utc(2017, 2, 1, 6)));
        // 2017-02-06 is the first Monday of February → true
        assert!(is_within_schedule_at(&sched, utc(2017, 2, 6, 6)));
        // 2017-02-13 is the second Monday of February → false
        assert!(!is_within_schedule_at(&sched, utc(2017, 2, 13, 6)));
    }
}
