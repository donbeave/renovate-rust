# Renovate Test Detail

[Back to test map](../../../../../renovate-test-map.md)

## `lib/workers/repository/process/lookup/abandonment.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/workers/repository/process/lookup/abandonment.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 7 | **Status:** not-applicable

### `workers/repository/process/lookup/abandonment › calculateAbandonment`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns the original release result when no abandonment threshold is provided | 27 | not-applicable | — | — | tests package abandonment detection; uses httpMock and TypeScript datasource infrastructure |
| returns the original release result when abandonment threshold is invalid | 39 | not-applicable | — | — | tests package abandonment detection; uses httpMock and TypeScript datasource infrastructure |
| returns the original release result when no mostRecentTimestamp timestamp is available | 54 | not-applicable | — | — | tests package abandonment detection; uses httpMock and TypeScript datasource infrastructure |
| marks a package as abandoned when mostRecentTimestamp plus threshold is before now | 69 | not-applicable | — | — | tests package abandonment detection; uses httpMock and TypeScript datasource infrastructure |
| does not mark a package as abandoned when mostRecentTimestamp plus threshold is after now | 83 | not-applicable | — | — | tests package abandonment detection; uses httpMock and TypeScript datasource infrastructure |
| preserves other properties in the release result | 97 | not-applicable | — | — | tests package abandonment detection; uses httpMock and TypeScript datasource infrastructure |
| handles exactly at the threshold boundary | 117 | not-applicable | — | — | tests package abandonment detection; uses httpMock and TypeScript datasource infrastructure |

---

