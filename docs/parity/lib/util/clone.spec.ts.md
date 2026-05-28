# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/clone.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/clone.spec.ts
**Total tests:** 3 | **Ported:** 2 | **Actionable:** 2 | **Status:** ported

### `util/clone`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns $expected when input is $input | 4 | ported | `util.rs` | `test_clone_values` | — |
| maintains same order | 26 | ported | `util.rs` | `test_clone_maintains_order` | — |
| assigns "[Circular]" to circular references | 41 | not-applicable | — | — | TypeScript type-system test; circular references are prevented at compile time in Rust via ownership |

---

