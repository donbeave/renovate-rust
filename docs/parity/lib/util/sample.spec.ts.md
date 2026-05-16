# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/sample.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/sample.spec.ts
**Total tests:** 7 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/sample › sampleSize`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns correct sized array | 7 | not-applicable | — | — | Renovate's TypeScript array sampling helper is not implemented as a Rust API. |
| returns full array for undefined number | 12 | not-applicable | — | — | Renovate's TypeScript array sampling helper includes undefined input handling with no Rust API equivalent. |
| returns full array for null number | 16 | not-applicable | — | — | Renovate's TypeScript array sampling helper includes null input handling with no Rust API equivalent. |
| returns full array for 0 number | 20 | not-applicable | — | — | Renovate's TypeScript array sampling helper is not implemented as a Rust API. |
| returns empty array for null array | 24 | not-applicable | — | — | Renovate's TypeScript array sampling helper includes null input handling with no Rust API equivalent. |
| returns empty array for undefined array | 28 | not-applicable | — | — | Renovate's TypeScript array sampling helper includes undefined input handling with no Rust API equivalent. |
| returns empty array for empty array | 32 | not-applicable | — | — | Renovate's TypeScript array sampling helper is not implemented as a Rust API. |

---

