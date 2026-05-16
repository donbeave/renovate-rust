# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/clone.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/clone.spec.ts
**Total tests:** 3 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/clone`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| returns $expected when input is $input | 4 | not-applicable | — | — | Renovate's JavaScript dynamic-value clone helper has no Rust API equivalent; Rust values use typed `Clone` implementations. |
| maintains same order | 26 | not-applicable | — | — | Renovate's JavaScript object-order-preserving clone helper has no Rust API equivalent. |
| assigns "[Circular]" to circular references | 41 | not-applicable | — | — | Renovate's JavaScript circular-reference clone behavior has no Rust API equivalent. |

---

