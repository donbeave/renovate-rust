# Renovate Test Detail

[Back to test map](../../../../renovate-test-map.md)

## `lib/config/migrations/custom/version-strategy-migration.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/config/migrations/custom/version-strategy-migration.spec.ts
**Total tests:** 2 | **Ported:** 2 | **Actionable:** 0 | **Status:** done

### `config/migrations/custom/version-strategy-migration`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should migrate versionStrategy="widen" to rangeStrategy="widen" | 4 | ported | `migrate_validate.rs` | `version_strategy_widen_migrates_to_range_strategy_widen` | — |
| should just remove property when versionStrategy not equals to "widen" | 14 | ported | `migrate_validate.rs` | `version_strategy_other_is_removed` | — |

---

