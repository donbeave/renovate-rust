# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/process/lookup/abandonment.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/lookup/abandonment.spec.ts
**Total tests:** 7 | **Ported:** 7 | **Actionable:** 7 | **Status:** ported

### `workers/repository/process/lookup/abandonment › calculateAbandonment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the original release result when no abandonment threshold is provided | 27 | ported | `util.rs` | `test_abandonment_no_threshold` | — |
| returns the original release result when abandonment threshold is invalid | 39 | ported | `util.rs` | `test_abandonment_invalid_threshold` | — |
| returns the original release result when no mostRecentTimestamp timestamp is available | 54 | ported | `util.rs` | `test_abandonment_no_timestamp` | — |
| marks a package as abandoned when mostRecentTimestamp plus threshold is before now | 69 | ported | `util.rs` | `test_abandonment_old_package_is_abandoned` | — |
| does not mark a package as abandoned when mostRecentTimestamp plus threshold is after now | 83 | ported | `util.rs` | `test_abandonment_recent_package_not_abandoned` | — |
| preserves other properties in the release result | 97 | ported | `util.rs` | `test_abandonment_preserves_other_properties` | — |
| handles exactly at the threshold boundary | 117 | ported | `util.rs` | `test_abandonment_boundary` | — |

---

