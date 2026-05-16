# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/filter-map.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/filter-map.spec.ts
**Total tests:** 2 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/filter-map`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return an empty array when given an empty array | 4 | not-applicable | — | — | Renovate's TypeScript in-place array filter/map helper has no Rust API equivalent; Rust uses iterator `filter_map` or `retain` directly. |
| should return an array with only the mapped values that pass the filter | 11 | not-applicable | — | — | Renovate's TypeScript in-place array filter/map helper has no Rust API equivalent; Rust uses iterator `filter_map` or `retain` directly. |

---

