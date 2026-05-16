# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/coerce.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/coerce.spec.ts
**Total tests:** 4 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/coerce › coerceToNull`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return null | 5 | not-applicable | — | — | Renovate's TypeScript null/undefined coercion helper has no Rust API equivalent; Rust uses `Option<T>`. |
| should return original value | 10 | not-applicable | — | — | Renovate's TypeScript null/undefined coercion helper has no Rust API equivalent; Rust uses `Option<T>`. |

### `util/coerce › coerceToUndefined`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return undefined | 18 | not-applicable | — | — | TypeScript undefined coercion has no Rust value-level equivalent; Rust uses `Option<T>`. |
| should return original value | 23 | not-applicable | — | — | TypeScript undefined coercion has no Rust value-level equivalent; Rust uses `Option<T>`. |

---

