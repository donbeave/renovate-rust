# Renovate Test Detail

[Back to test map](../../renovate-test-map.md)

## `lib/util/number.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/util/number.spec.ts
**Total tests:** 2 | **Ported:** 0 | **Actionable:** 0 | **Status:** not-applicable

### `util/number`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| coerceNumber($val, $def) = $expected | 4 | not-applicable | — | — | Renovate's TypeScript number coercion helper is not implemented as a shared Rust API; Rust uses typed parsing at call sites. |
| parseInteger($val, $def) = $expected | 13 | not-applicable | — | — | Renovate's TypeScript integer parsing helper is not implemented as a shared Rust API; Rust uses typed parsing at call sites. |

---

