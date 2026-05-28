# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/util/package-rules/jsonata.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/package-rules/jsonata.spec.ts
**Total tests:** 12 | **Ported:** 0 | **Actionable:** 12 | **Status:** partial

### `util/package-rules/jsonata`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true for a matching JSONata expression | 6 | not-applicable | — | — | JSONata expression evaluation requires jsonata npm library; no Rust JSONata implementation available |
| should return false for a non-matching JSONata expression | 14 | not-applicable | — | — | JSONata expression evaluation requires jsonata npm library; no Rust JSONata implementation available |
| should return false for an invalid JSONata expression | 22 | not-applicable | — | — | JSONata expression evaluation requires jsonata npm library; no Rust JSONata implementation available |
| should return null if matchJsonata is not defined | 30 | not-applicable | — | — | JSONata expression evaluation requires jsonata npm library; no Rust JSONata implementation available |
| should return true for a complex JSONata expression | 35 | not-applicable | — | — | JSONata expression evaluation requires jsonata npm library; no Rust JSONata implementation available |
| should return false for a complex JSONata expression with non-matching version | 44 | not-applicable | — | — | JSONata expression evaluation requires jsonata npm library; no Rust JSONata implementation available |
| should return true for a JSONata expression with nested properties | 53 | not-applicable | — | — | JSONata expression evaluation requires jsonata npm library; no Rust JSONata implementation available |
| should return false for a JSONata expression with nested properties and non-matching version | 62 | not-applicable | — | — | JSONata expression evaluation requires jsonata npm library; no Rust JSONata implementation available |
| should return true if any JSONata expression matches | 71 | not-applicable | — | — | JSONata expression evaluation requires jsonata npm library; no Rust JSONata implementation available |
| should catch evaluate errors | 79 | not-applicable | — | — | JSONata expression evaluation requires jsonata npm library; no Rust JSONata implementation available |

### `util/package-rules/jsonata › $detectPlatform`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return true when sourceUrl matches platform | 88 | not-applicable | — | — | JSONata expression evaluation requires jsonata npm library; no Rust JSONata implementation available |
| should return false when sourceUrl does not match platform | 96 | not-applicable | — | — | JSONata expression evaluation requires jsonata npm library; no Rust JSONata implementation available |

---

