# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/pretty-time.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/pretty-time.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/pretty-time`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| toMs('$input') === $expected | 5 | not-applicable | — | — | Renovate's generic compact pretty-time parser is not implemented as a Rust API; Rust has narrower schedule/release-age parsing where needed. |
| returns null for error | 45 | not-applicable | — | — | Renovate's JavaScript error-swallowing pretty-time helper behavior has no Rust API equivalent. |

### `util/pretty-time › satisfiesDateRange()`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| satisfiesRange('$date', '$range') === $expected | 60 | not-applicable | — | — | Renovate's generic `satisfiesDateRange()` helper is not implemented as a Rust API; Rust release-age checks use feature-specific schedule logic. |

---

