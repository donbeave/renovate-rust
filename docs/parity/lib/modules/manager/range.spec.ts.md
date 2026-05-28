# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/manager/range.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/manager/range.spec.ts
**Total tests:** 5 | **Ported:** 5 | **Actionable:** 5 | **Status:** ported

### `modules/manager/range`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns same if not auto | 5 | ported | `util.rs` | `test_get_range_strategy_not_auto` | — |
| returns manager strategy | 13 | ported | `util.rs` | `test_get_range_strategy_npm_auto_dependencies` | — |
| defaults to update-lockfile if updateLockedDependency() is supported | 22 | ported | `util.rs` | `test_get_range_strategy_bundler_auto` | — |
| defaults to replace | 30 | ported | `util.rs` | `test_get_range_strategy_sbt_auto` | — |
| returns rangeStrategy if not auto | 38 | ported | `util.rs` | `test_get_range_strategy_future` | — |

---

