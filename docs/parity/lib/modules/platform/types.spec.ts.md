# Renovate Test Detail

[Back to test map](../../../renovate-test-map.md)

## `lib/modules/platform/types.spec.ts`

**Reference:** https://github.com/renovatebot/renovate/blob/main/lib/modules/platform/types.spec.ts
**Total tests:** 1 | **Ported:** 0 | **Actionable:** 1 | **Status:** done

### `modules/platform/types`

| Original test name | Line | Status | Rust file | Rust test name | Reason |
|---|---|---|---|---|---|
| `RepoParams` and `RepoGlobalConfig` types should be incompatible | 5 | not-applicable | — | — | TypeScript compile-time structural type compatibility test (`expectTypeOf`); Rust's type system enforces field separation by construction — no equivalent runtime or compile test needed |

---

