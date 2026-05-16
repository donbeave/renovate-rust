# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/upgrade-in-range-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/upgrade-in-range-migration.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `config/migrations/custom/upgrade-in-range-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate upgradeInRange=true to rangeStrategy="bump" | 4 | ported | `migrate_validate.rs` | `upgrade_in_range_true_migrates_to_range_strategy_bump` | — |
| should just remove property when upgradeInRange not equals to true | 14 | ported | `migrate_validate.rs` | `upgrade_in_range_false_is_removed` | — |

---

