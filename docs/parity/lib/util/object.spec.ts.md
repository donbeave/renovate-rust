# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/object.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/object.spec.ts
**Total tests:** 5 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/object`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| finds key in regular object | 4 | not-applicable | — | — | Renovate's TypeScript dynamic object key helper is not implemented as a Rust API. |
| detects missing key in regular object | 8 | not-applicable | — | — | Renovate's TypeScript dynamic object key helper is not implemented as a Rust API. |
| returns false for wrong instance type | 12 | not-applicable | — | — | Renovate's TypeScript runtime object/type guard behavior has no Rust API equivalent. |

### `util/object › coerceObject`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty object | 17 | not-applicable | — | — | Renovate's TypeScript nullable object coercion helper is not implemented as a shared Rust API. |
| should return input object | 22 | not-applicable | — | — | Renovate's TypeScript nullable object coercion helper is not implemented as a shared Rust API. |

---

