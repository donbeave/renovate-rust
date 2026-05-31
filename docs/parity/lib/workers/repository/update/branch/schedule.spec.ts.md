# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/update/branch/schedule.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/branch/schedule.spec.ts
**Total tests:** 68 | **Ported:** 64 | **Actionable:** 1 | **Status:** pending

### `workers/repository/update/branch/schedule › hasValidTimezone(schedule)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns false for invalid timezone | 7 | ported | `schedule.rs` | `has_valid_timezone_invalid_returns_false` | — |
| returns true for valid timezone | 11 | ported | `schedule.rs` | `has_valid_timezone_valid_returns_true` | — |

### `workers/repository/update/branch/schedule › hasValidSchedule(schedule)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true for null | 17 | ported | `schedule.rs` | `has_valid_schedule_null_returns_true` | — |
| returns true for at any time | 21 | ported | `schedule.rs` | `has_valid_schedule_at_any_time_returns_true` | — |
| returns false for invalid schedule | 25 | ported | `schedule.rs` | `has_valid_schedule_invalid_returns_false` | — |
| returns false if any schedule fails to parse | 29 | ported | `schedule.rs` | `has_valid_schedule_any_failure_returns_false` | — |
| returns false if using minutes | 33 | ported | `schedule.rs` | `has_valid_schedule_minutes_returns_false` | — |
| returns false for wildcard minutes | 39 | ported | `schedule.rs` | `has_valid_schedule_non_wildcard_cron_minute_returns_false` | — |
| returns false if schedules have no days or time range | 47 | ported | `schedule.rs` | `has_valid_schedule_no_days_or_time_returns_false` | — |
| returns false if any schedule has no days or time range | 51 | ported | `schedule.rs` | `has_valid_schedule_combined_any_failure_returns_false` | — |
| returns false for every xday | 57 | ported | `schedule.rs` | `has_valid_schedule_bare_weekday_returns_false` | — |
| returns true if schedule has days of week | 61 | ported | `schedule.rs` | `has_valid_schedule_days_of_week_returns_true` | — |
| returns true for multi day schedules | 67 | ported | `schedule.rs` | `has_valid_schedule_multi_day_with_time_returns_true` | — |
| returns true if schedule has a start time | 75 | ported | `schedule.rs` | `has_valid_schedule_start_time_returns_true` | — |
| returns true for first day of the month | 79 | ported | `schedule.rs` | `has_valid_schedule_first_day_of_month_returns_true` | — |
| returns true for schedules longer than 1 month | 85 | ported | `schedule.rs` | `has_valid_schedule_multi_month_returns_true` | — |
| returns true if schedule has an end time | 91 | ported | `schedule.rs` | `has_valid_schedule_end_time_returns_true` | — |
| returns true if schedule has a start and end time | 95 | ported | `schedule.rs` | `has_valid_schedule_start_and_end_time_returns_true` | — |
| returns true if schedule has days and a start and end time | 101 | ported | `schedule.rs` | `has_valid_schedule_days_with_start_and_end_time_returns_true` | — |
| returns true if schedule uses cron syntax | 109 | ported | `schedule.rs` | `has_valid_schedule_valid_cron_returns_true` | — |
| massages schedules | 117 | ported | `schedule.rs` | `has_valid_schedule_massaged_forms_return_true` | — |
| supports hours shorthand | 126 | ported | `schedule.rs` | `has_valid_schedule_hours_shorthand_returns_true` | — |

### `workers/repository/update/branch/schedule › isScheduledNow(config)`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns true if no schedule | 154 | ported | `schedule.rs` | `spec_returns_true_if_no_schedule` | — |
| returns true if at any time | 159 | ported | `schedule.rs` | `spec_returns_true_for_at_any_time` | — |
| returns true if at any time array | 165 | ported | `schedule.rs` | `at_any_time_always_matches` | — |
| returns true if invalid schedule | 171 | ported | `schedule.rs` | `spec_is_scheduled_now_invalid_schedule_fail_open` | — |
| returns true if invalid timezone | 177 | ported | `schedule.rs` | `spec_is_scheduled_now_invalid_timezone_fail_open` | — |
| supports before hours true | 184 | ported | `schedule.rs` | `spec_supports_before_hours_true` | — |
| supports before hours false | 190 | ported | `schedule.rs` | `spec_supports_before_hours_false` | — |
| massages string | 196 | ported | `schedule.rs` | `spec_is_scheduled_now_massages_string_to_array` | — |
| supports outside hours | 202 | ported | `schedule.rs` | `spec_supports_outside_hours` | — |
| supports cron syntax with hours | 208 | ported | `schedule.rs` | `spec_cron_with_hours_match` | — |
| supports cron syntax with days | 218 | ported | `schedule.rs` | `spec_cron_with_days_match` | — |
| supports cron syntax with months | 228 | ported | `schedule.rs` | `spec_cron_with_months_match` | — |
| supports cron syntax with weekdays | 238 | ported | `schedule.rs` | `spec_cron_with_weekdays_match` | — |

### `workers/repository/update/branch/schedule › isScheduledNow(config) › supports cron syntax on Sundays`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| approves if the weekday is * | 253 | ported | `schedule.rs` | `spec_cron_on_sunday_wildcard_matches` | — |
| approves if the weekday is 0 | 259 | ported | `schedule.rs` | `spec_cron_on_sunday_weekday_0` | — |
| rejects if the weekday is 1 | 265 | ported | `schedule.rs` | `spec_cron_on_sunday_rejects_weekday_1` | — |

### `workers/repository/update/branch/schedule › isScheduledNow(config) › supports L syntax in cron schedules`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports last day of month | 277 | ported | `schedule.rs` | `spec_cron_l_syntax_last_day_of_month` | — |
| supports last day of week | 283 | ported | `schedule.rs` | `spec_cron_l_syntax_last_day_of_week` | — |

### `workers/repository/update/branch/schedule › isScheduledNow(config) › supports # syntax in cron schedules`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| supports first Monday of month | 293 | ported | `schedule.rs` | `spec_cron_hash_syntax_first_monday_of_month` | — |

### `workers/repository/update/branch/schedule › isScheduledNow(config) › handles schedule with Day Of Month and Day Of Week using AND logic`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $sched, $tz, $datetime | 303 | ported | `schedule.rs` | `spec_dom_and_dow_and_logic` | — |

### `workers/repository/update/branch/schedule › isScheduledNow(config) › supports timezone`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| $sched, $tz, $datetime | 319 | ported | `schedule.rs` | `spec_timezone_text_after_4pm_singapore, spec_timezone_text_before_4am_monday_tokyo, spec_timezone_cron_16_23_singapore, spec_timezone_cron_0_3_monday_tokyo` | — |
| reject if day mismatch | 337 | ported | `schedule.rs` | `spec_cron_dom_mismatch_false` | — |
| reject if month mismatch | 343 | ported | `schedule.rs` | `spec_cron_month_mismatch_false` | — |
| reject if no schedule available | 349 | ported | `schedule.rs` | `spec_cron_no_schedule_available_false` | — |
| supports multiple schedules | 355 | ported | `schedule.rs` | `spec_supports_multiple_schedules` | — |
| supports day match | 361 | ported | `schedule.rs` | `spec_supports_day_match_friday` | — |
| supports day mismatch | 367 | ported | `schedule.rs` | `spec_supports_day_mismatch` | — |
| supports every weekday | 373 | ported | `schedule.rs` | `spec_every_weekday_matches_friday` | — |
| supports every weekend | 379 | ported | `schedule.rs` | `spec_every_weekend_rejects_friday` | — |
| supports every weekday with time | 385 | ported | `schedule.rs` | `spec_before_11am_every_weekday_matches` | — |
| supports o every weekday | 391 | ported | `schedule.rs` | `spec_supports_o_every_weekday` | — |
| rejects first day of the month | 397 | ported | `schedule.rs` | `spec_first_day_of_month_rejects_non_first` | — |
| approves first day of the month | 403 | ported | `schedule.rs` | `spec_first_day_of_month_approves_first` | — |
| approves valid weeks of year | 410 | ported | `schedule.rs` | `spec_weeks_of_year_approves_first_week` | — |
| rejects on weeks of year | 417 | ported | `schedule.rs` | `spec_weeks_of_year_rejects_second_week` | — |
| approves on months of year | 424 | ported | `schedule.rs` | `spec_months_of_year_approves_january` | — |
| rejects on months of year | 431 | ported | `schedule.rs` | `spec_months_of_year_rejects_february` | — |
| approves schedule longer than 1 month | 438 | ported | `schedule.rs` | `spec_every_3_months_approves_july` | — |
| rejects schedule longer than 1 month | 445 | ported | `schedule.rs` | `spec_every_6_months_rejects_february` | — |
| approves schedule longer than 1 month with day of month | 452 | ported | `schedule.rs` | `spec_every_3_months_first_day_approves_july_1` | — |
| rejects schedule longer than 1 month with day of month | 459 | ported | `schedule.rs` | `spec_every_3_months_first_day_rejects_february` | — |
| supports weekday instances | 466 | ported | `schedule.rs` | `spec_weekday_instances_first_monday` | — |

### `workers/repository/update/branch/schedule › log cron schedules`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should correctly convert "* 22 4 * *" to human-readable format  | 483 | pending | — | — | TS-library-specific; tests cronstrue npm package cron-to-human conversion; no Rust equivalent |
| should correctly convert "* */2 * * *" to human-readable format  | 490 | not-applicable | — | — | TS-library-specific; tests cronstrue npm package cron-to-human conversion |
| should correctly convert "* 23 * * *" to human-readable format  | 495 | not-applicable | — | — | TS-library-specific; tests cronstrue npm package cron-to-human conversion |
| should not throw an error for an invalid cron expression "* * */2 6#1"  | 500 | not-applicable | — | — | TS-library-specific; tests cronstrue npm package error handling for invalid cron |

---

