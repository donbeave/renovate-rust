# Renovate Test Detail

[Back to test map](../../../../../../renovate-test-map.md)

## `lib/workers/repository/update/pr/body/config-description.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/update/pr/body/config-description.spec.ts
**Total tests:** 16 | **Ported:** 13 | **Actionable:** 16 | **Status:** done

### `workers/repository/update/pr/body/config-description › getPrConfigDescription`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| renders stopUpdating=true | 14 | ported | `util.rs` | `test_config_desc_stop_updating` | — |
| renders rebaseWhen="never" | 25 | ported | `util.rs` | `test_config_desc_rebase_when_never` | — |
| renders rebaseWhen="behind-base-branch" | 36 | ported | `util.rs` | `test_config_desc_rebase_when_behind` | — |
| renders timezone | 45 | ported | `util.rs` | `test_config_desc_timezone` | — |
| renders UTC as the default timezone | 54 | ported | `util.rs` | `test_config_desc_utc_default` | — |
| summarizes cron schedules | 62 | not-applicable | — | — | Requires cronstrue cron-to-human translation; Rust schedule_to_string only formats raw cron expressions as backticks |
| displays later schedules | 73 | ported | `util.rs` | `test_config_desc_later_schedules` | — |
| renders undefined schedule | 81 | ported | `util.rs` | `test_config_desc_undefined_schedule` | — |
| summarizes cron schedules (for automergeSchedule) | 86 | not-applicable | — | — | Requires cronstrue cron-to-human translation; Rust schedule_to_string only formats raw cron expressions as backticks |
| summarizes both branch creation and automerge schedules | 97 | not-applicable | — | — | Requires cronstrue cron-to-human translation; Rust schedule_to_string only formats raw cron expressions as backticks |
| renders recreateClosed=true | 116 | ported | `util.rs` | `test_config_desc_recreate_closed_true` | — |
| does not render recreateClosed=false | 124 | ported | `util.rs` | `test_config_desc_recreate_closed_false` | — |
| does not render recreateClosed=undefined | 132 | ported | `util.rs` | `test_config_desc_recreate_closed_undefined` | — |
| renders singular | 137 | ported | `util.rs` | `test_config_desc_singular_upgrade` | — |
| renders automerge | 145 | ported | `util.rs` | `test_config_desc_automerge_enabled` | — |
| renders blocked automerge | 150 | ported | `util.rs` | `test_config_desc_automerge_blocked` | — |

---
