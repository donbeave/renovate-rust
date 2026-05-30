# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/object.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/object.spec.ts
**Total tests:** 5 | **Ported:** 4 | **Actionable:** 0 | **Status:** done

### `util/object`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| finds key in regular object | 4 | ported | `util.rs` | `test_has_key` | — |
| detects missing key in regular object | 8 | ported | `util.rs` | `test_has_key` | — |
| returns false for wrong instance type | 12 | not-applicable | — | — | TypeScript type-system test; Rust's type system prevents passing non-map at compile time |

### `util/object › coerceObject`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| should return empty object | 17 | ported | `util.rs` | `test_coerce_object` | — |
| should return input object | 22 | ported | `util.rs` | `test_coerce_object` | — |

---

