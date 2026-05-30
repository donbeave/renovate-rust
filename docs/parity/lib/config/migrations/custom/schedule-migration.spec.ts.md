# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/schedule-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/schedule-migration.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 0 | **Status:** done

### `config/migrations/custom/schedule-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| migrates every friday | 4 | ported | `migrate_validate.rs` | `schedule_every_friday_migrates_to_on_friday` | — |
| does not migrate every weekday | 14 | ported | `migrate_validate.rs` | `schedule_every_weekday_is_unchanged` | — |
| does not migrate multi days | 25 | ported | `migrate_validate.rs` | `schedule_multi_days_is_unchanged` | — |
| does not migrate hour range | 36 | ported | `migrate_validate.rs` | `schedule_hour_range_is_unchanged` | — |
| does not migrate invalid range | 47 | ported | `migrate_validate.rs` | `schedule_invalid_range_is_unchanged` | — |

---

