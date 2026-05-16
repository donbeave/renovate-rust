# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/process/lookup/abandonment.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/lookup/abandonment.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 7 | **Status:** pending

### `workers/repository/process/lookup/abandonment › calculateAbandonment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the original release result when no abandonment threshold is provided | 27 | pending | — | — | — |
| returns the original release result when abandonment threshold is invalid | 39 | pending | — | — | — |
| returns the original release result when no mostRecentTimestamp timestamp is available | 54 | pending | — | — | — |
| marks a package as abandoned when mostRecentTimestamp plus threshold is before now | 69 | pending | — | — | — |
| does not mark a package as abandoned when mostRecentTimestamp plus threshold is after now | 83 | pending | — | — | — |
| preserves other properties in the release result | 97 | pending | — | — | — |
| handles exactly at the threshold boundary | 117 | pending | — | — | — |

---

